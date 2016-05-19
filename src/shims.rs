use rurust::Value;
use plugger_core::Pluggable;
use libc;

/// A placeholder for any Rust struct.
pub struct Receiver;

/// A method which takes a receiver.
pub type Method = fn(&mut Receiver) -> Value;
/// A function.
pub type Function = fn() -> Value;

/// Shim to call a Rust method (taking `&self`) from Ruby.
pub extern fn ruby_method(argc: libc::c_int, argv: *const Value, object: Value) -> Value {
    let (method, arguments) = helpers::process_method_arguments(argc, argv);

    println!("sent from object : {:?}", object);

    for argument in arguments.iter() {
        println!("received argument: {:?}", argument);
    }

    let receiver = helpers::reference_to_struct(object);

    method(receiver)
}

/// Shim to call a Rust function (doesn't take `self`) from Ruby.
pub extern fn ruby_singleton_method(argc: libc::c_int, argv: *const Value, class: Value) -> Value {
    let (function, arguments) = helpers::process_function_arguments(argc, argv);

    println!("class: {:?}", class);

    for argument in arguments.iter() {
        println!("received argument: {:?}", argument);
    }

    function()
}

mod helpers {
    use super::{Receiver, Method, Function};
    use rurust::Value;
    use std::{mem, slice};
    use libc;

    /// Stores the arguments there were passed to the shim.
    struct Arguments<'a> {
        function_ptr: Value,
        normal_arguments: &'a [Value],
    }

    /// Processes the arguments given to a method shim.
    pub fn process_method_arguments<'a>(argc: libc::c_int, argv: *const Value) -> (Method, &'a [Value]) {
        let arguments = self::separate_arguments(argc, argv);
        (self::method(arguments.function_ptr), arguments.normal_arguments)
    }

    /// Processes the arguments given to a function shim.
    pub fn process_function_arguments<'a>(argc: libc::c_int, argv: *const Value) -> (Function, &'a [Value]) {
        let arguments = self::separate_arguments(argc, argv);
        (self::function(arguments.function_ptr), arguments.normal_arguments)
    }

    /// Given an `argc` and `argv` pair, create a new `Arguments` object.
    fn separate_arguments<'a>(argc: libc::c_int, argv: *const Value) -> Arguments<'a> {
        let all_arguments = unsafe { slice::from_raw_parts(argv, argc as usize) };

        Arguments {
            function_ptr: all_arguments[0],
            normal_arguments: &all_arguments[1..],
        }
    }

    /// Gets a reference to the Rust struct from the associated Ruby object.
    pub fn reference_to_struct<'a>(ruby_object: Value) -> &'a mut Receiver {
        let ptr = ruby_object.call_no_args("object_pointer").to_u64() as usize;
        let receiver: &mut Receiver = unsafe { mem::transmute(ptr) };
        receiver
    }

    /// Creates a method from a function pointer given from Ruby.
    fn method(function_pointer: Value) -> Method {
        unsafe { mem::transmute(self::function(function_pointer)) }
    }

    /// Creates a function from a function pointer given from Ruby.
    fn function(function_pointer: Value) -> Function {
        let ptr = function_pointer.to_u64() as usize;

        unsafe { mem::transmute(ptr) }
    }
}

