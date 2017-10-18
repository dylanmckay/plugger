extern crate plugger_core;
extern crate rurust;
extern crate libc;

pub use rurust::Value as Value;

/// Shim functions which act as middlemen between C and Ruby.
pub mod shims;

// Must be public so that the plugger crate can use.
#[doc(hidden)]
pub use self::marshall::Marshall;

mod marshall;

/// The Ruby support code.
const RUBY_SUPPORT: &'static str = include_str!("../support/ruby.rb");
/// The base class all Ruby plugger objects derive from.
const PLUGGER_BASE_CLASS: &'static str = "PluggerObject";

use plugger_core::Pluggable;

static mut VM_INITIALISED: bool = false;

pub extern fn do_something() {
    println!("do_something");
}

#[derive(Debug)]
pub enum ErrorKind
{
    Ruby(rurust::ErrorKind),
}

pub struct Ruby;

impl Ruby
{
    pub fn new() -> Result<Self, ErrorKind> {
        Ok(Ruby)
    }

    pub fn plug<P>(&mut self, name: &str, object: &mut P) where P: Pluggable {
        let mut vm = vm();
        let base_class = vm.eval(PLUGGER_BASE_CLASS).expect("could not find the plugger base class");

        let class_builder = object.methods().iter().fold(vm.class(object.name()).extend(base_class), |class, method| {
            let ptr = method.marshall("ruby") as usize;
            let ptr_value = vm.eval(&format!("{}", ptr)).unwrap();
            let realised_param_count = method.parameters.len();
            let actual_param_count = realised_param_count + 1; // Account for the hidden func ptr parameter.

            let name = format!("{}_internal", method.name);

            if method.is_static {
                class.singleton_method(name, shims::ruby_function(realised_param_count) as *mut _, actual_param_count as i8)
            } else {
                class.method(name, shims::ruby_method(realised_param_count) as *mut _, actual_param_count as i8)
            }.constant(method.name.to_uppercase(), ptr_value)
        });

        class_builder.build();
        let ptr = object as *mut _ as usize;

        let constant_name = name.to_uppercase();

        let ruby_val = vm.eval(&format!("{}.new({})", object.name(), ptr)).unwrap();
        vm.set_global_const(&constant_name, ruby_val);

        println!("Plugging in {} {}", object.name(), constant_name);
    }

    pub fn eval(&mut self, code: &str) -> Result<String, ErrorKind> {
        match vm().eval(code) {
            Ok(val) => Ok(val.inspect_string()),
            Err(e) => Err(ErrorKind::Ruby(e)),
        }
    }

}

fn vm() -> ::std::sync::MutexGuard<'static, rurust::VM> {
    let mut vm = rurust::VM::get().lock().unwrap();

    unsafe {
        // FIXME: use a mutex or something
        if !VM_INITIALISED {
            vm.eval(RUBY_SUPPORT).expect("the support module crashed");
            VM_INITIALISED = true;
        }
    }
    vm
}

