use rurust::Value;
use libc;
use std::mem;

/// A placeholder for any Rust struct.
pub struct Receiver;

/// A method which takes a receiver.
pub type Method = fn(&mut Receiver) -> Value;
pub type Method1 = fn(&mut Receiver, Value) -> Value;
pub type Method2 = fn(&mut Receiver, Value, Value) -> Value;
pub type Method3 = fn(&mut Receiver, Value, Value, Value) -> Value;
pub type Method4 = fn(&mut Receiver, Value, Value, Value, Value) -> Value;
pub type Method5 = fn(&mut Receiver, Value, Value, Value, Value, Value) -> Value;
pub type Method6 = fn(&mut Receiver, Value, Value, Value, Value, Value, Value) -> Value;
pub type Method7 = fn(&mut Receiver, Value, Value, Value, Value, Value, Value, Value) -> Value;
pub type Method8 = fn(&mut Receiver, Value, Value, Value, Value, Value, Value, Value, Value) -> Value;
pub type Method9 = fn(&mut Receiver, Value, Value, Value, Value, Value, Value, Value, Value, Value) -> Value;

/// A function.
pub type Function = fn() -> Value;
pub type Function1 = fn(Value) -> Value;
pub type Function2 = fn(Value, Value) -> Value;
pub type Function3 = fn(Value, Value, Value) -> Value;
pub type Function4 = fn(Value, Value, Value, Value) -> Value;
pub type Function5 = fn(Value, Value, Value, Value, Value) -> Value;
pub type Function6 = fn(Value, Value, Value, Value, Value, Value) -> Value;
pub type Function7 = fn(Value, Value, Value, Value, Value, Value, Value) -> Value;
pub type Function8 = fn(Value, Value, Value, Value, Value, Value, Value, Value) -> Value;
pub type Function9 = fn(Value, Value, Value, Value, Value, Value, Value, Value, Value) -> Value;

/// Dispatches arguments to a method call.
macro_rules! dispatch {
    ( $method:expr => $method_type:ty => ( $( $arg:expr ),* ) ) => {
        {
            let method: $method_type = unsafe { mem::transmute($method) };
            method ( $( $arg ),* )
        }
    }
}

/// Shim to call a Rust method (taking `&self`) from Ruby.
pub extern fn ruby_method(argc: libc::c_int, argv: *const Value, object: Value) -> Value {
    let (method, arguments) = helpers::process_method_arguments(argc, argv);

    let receiver = helpers::reference_to_struct(object);

    let a = arguments;
    match arguments.len() {
        0 => dispatch!(method => Method  => (receiver)),
        1 => dispatch!(method => Method1 => (receiver, a[0])),
        2 => dispatch!(method => Method2 => (receiver, a[0], a[1])),
        3 => dispatch!(method => Method3 => (receiver, a[0], a[1], a[2])),
        4 => dispatch!(method => Method4 => (receiver, a[0], a[1], a[2], a[3])),
        5 => dispatch!(method => Method5 => (receiver, a[0], a[1], a[2], a[3], a[4])),
        6 => dispatch!(method => Method6 => (receiver, a[0], a[1], a[2], a[3], a[4], a[5])),
        7 => dispatch!(method => Method7 => (receiver, a[0], a[1], a[2], a[3], a[4], a[5], a[6])),
        8 => dispatch!(method => Method8 => (receiver, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7])),
        9 => dispatch!(method => Method9 => (receiver, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8])),
        _ => panic!("too many arguments: {}", arguments.len()),
    }
}

/// Shim to call a Rust function (doesn't take `self`) from Ruby.
pub extern fn ruby_singleton_method(argc: libc::c_int, argv: *const Value, _class: Value) -> Value {
    let (function, arguments) = helpers::process_function_arguments(argc, argv);

    let a = arguments;
    match arguments.len() {
        0 => dispatch!(function => Function  => ()),
        1 => dispatch!(function => Function1 => (a[0])),
        2 => dispatch!(function => Function2 => (a[0], a[1])),
        3 => dispatch!(function => Function3 => (a[0], a[1], a[2])),
        4 => dispatch!(function => Function4 => (a[0], a[1], a[2], a[3])),
        5 => dispatch!(function => Function5 => (a[0], a[1], a[2], a[3], a[4])),
        6 => dispatch!(function => Function6 => (a[0], a[1], a[2], a[3], a[4], a[5])),
        7 => dispatch!(function => Function7 => (a[0], a[1], a[2], a[3], a[4], a[5], a[6])),
        8 => dispatch!(function => Function8 => (a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7])),
        9 => dispatch!(function => Function9 => (a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8])),
        _ => panic!("too many arguments: {}", arguments.len()),
    }
}

mod helpers {
    use super::{Receiver, Method, Function};
    use rurust::Value;
    use std::{mem, slice};
    use libc;

    /// Stores the arguments there were passed to the shim.
    struct Arguments<'a> {
        function_ptr: Value,
        full_arguments: &'a [Value],
    }

    /// Processes the arguments given to a method shim.
    pub fn process_method_arguments<'a>(argc: libc::c_int, argv: *const Value) -> (Method, &'a [Value]) {
        let arguments = self::separate_arguments(argc, argv);
        (self::method(arguments.function_ptr), arguments.full_arguments)
    }

    /// Processes the arguments given to a function shim.
    pub fn process_function_arguments<'a>(argc: libc::c_int, argv: *const Value) -> (Function, &'a [Value]) {
        let arguments = self::separate_arguments(argc, argv);
        (self::function(arguments.function_ptr), arguments.full_arguments)
    }

    /// Given an `argc` and `argv` pair, create a new `Arguments` object.
    fn separate_arguments<'a>(argc: libc::c_int, argv: *const Value) -> Arguments<'a> {
        let all_arguments = unsafe { slice::from_raw_parts(argv, argc as usize) };

        Arguments {
            function_ptr: all_arguments[0],
            full_arguments: &all_arguments[1..],
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

