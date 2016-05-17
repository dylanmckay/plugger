extern crate plugger_core;
extern crate rurust;

/// Shim functions which act as middlemen between C and Ruby.
pub mod shims;

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
        let object = object.methods().iter().fold(self.vm.class(object.name()), |class, method| {
            class.method(method.name, shims::ruby_method_arg0 as *mut _, 0)
        });

        object.build();
    }

    pub fn eval(&mut self, code: &str) -> Result<String, ErrorKind> {
        match self.vm.eval(code) {
            Ok(val) => Ok(val.inspect_string()),
            Err(e) => Err(ErrorKind::Ruby(e)),
        }
    }
}

