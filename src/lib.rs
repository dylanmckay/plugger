extern crate plugger_core;
extern crate rurust;

/// Shim functions which act as middlemen between C and Ruby.
pub mod shims;

/// The Ruby support code.
const RUBY_SUPPORT: &'static str = include_str!("../support/ruby.rb");
/// The base class all Ruby plugger objects derive from.
const PLUGGER_BASE_CLASS: &'static str = "PluggerObject";

use plugger_core::Pluggable;
use rurust::Value;

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
            Ok(vm) => {
                let mut ruby = Ruby { vm: vm };
                ruby.eval(RUBY_SUPPORT).expect("the support module crashed");

                Ok(ruby)
            }
            Err(e) => return Err(ErrorKind::Ruby(e)),
        }
    }

    pub fn plug<P>(&mut self, object: &mut P) where P: Pluggable {
        let base_class = self.vm.eval(PLUGGER_BASE_CLASS).expect("could not find the plugger base class");

        let class_builder = object.methods().iter().fold(self.vm.class(object.name()).extend(base_class), |class, method| {
            let ptr = method.method_pointer as usize;
            let ptr_value = self.vm.eval(&format!("{}", ptr)).unwrap();

            let name = format!("{}_internal", method.name);

            if method.is_static() {
                class.singleton_method(name, shims::ruby_method_arg0 as *mut _, 1)
            } else {
                class.method(name, shims::ruby_method_arg0 as *mut _, 1)
            }.constant(method.name.to_uppercase(), ptr_value)
        });

        class_builder.build();
        
        println!("ptr: {}", object as *mut _ as usize);
    }

    pub fn eval(&mut self, code: &str) -> Result<String, ErrorKind> {
        match self.vm.eval(code) {
            Ok(val) => Ok(val.inspect_string()),
            Err(e) => Err(ErrorKind::Ruby(e)),
        }
    }
}

