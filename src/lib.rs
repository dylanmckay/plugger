extern crate plugger_core;
extern crate rurust;

use plugger_core::Pluggable;

pub extern fn do_something() {
    println!("do_something");
}

#[derive(Debug)]
pub enum ErrorKind
{
    Ruby(rurust::ErrorKind),
}

pub struct Ruby
{
    pub vm: rurust::VM,
}

impl Ruby
{
    pub fn new() -> Result<Self, ErrorKind> {
        match rurust::VM::new() {
            Ok(vm) => Ok(Ruby { vm: vm }),
            Err(e) => return Err(ErrorKind::Ruby(e)),
        }
    }

    pub fn plug(&mut self, object: &mut Pluggable) {
        object.methods().iter().fold(self.vm.class(object.name()), |class, method| {
            class.singleton_method(method.name, do_something as *const u8, 0)
        }).build();
    }

    pub fn eval(&mut self, code: &str) -> Result<String, ErrorKind> {
        match self.vm.eval(code) {
            Ok(val) => Ok(val.inspect_string()),
            Err(e) => Err(ErrorKind::Ruby(e)),
        }
    }
}

