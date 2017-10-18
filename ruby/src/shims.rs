//! Shim methods for calling Rust methods from Ruby.
//!
//! These methods are hooked into the Ruby VM, which then
//! dispatch to actual Rust functions.

use rurust::Value;
use std::mem;

/// A placeholder for any Rust struct.
pub struct Receiver;

pub type FunctionPointer = fn() -> Value;

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

macro_rules! dispatch_method {
    ( $func_ptr:expr , $receiver:expr => $method_type:ty => ( $( $arg:expr ),* ) ) => {
        {
            let method = helpers::function_pointer($func_ptr);
            let receiver = helpers::reference_to_struct($receiver);
            dispatch!( method => $method_type => ( receiver $(, $arg )* ) )
        }
    }
}

macro_rules! dispatch_function {
    ( $func_ptr:expr => $method_type:ty => ( $( $arg:expr ),* ) ) => {
        {
            let method = helpers::function_pointer($func_ptr);
            dispatch!( method => $method_type => ( $( $arg ),* ) )
        }
    }
}

/// Gets the shim function that receives a receiver object and `arg_count` arguments.
pub fn ruby_method(arg_count: usize) -> *mut extern fn() -> Value {
    match arg_count {
        0 => self::ruby_method0 as *mut _,
        1 => self::ruby_method1 as *mut _,
        2 => self::ruby_method2 as *mut _,
        3 => self::ruby_method3 as *mut _,
        4 => self::ruby_method4 as *mut _,
        5 => self::ruby_method5 as *mut _,
        6 => self::ruby_method6 as *mut _,
        7 => self::ruby_method7 as *mut _,
        8 => self::ruby_method8 as *mut _,
        9 => self::ruby_method9 as *mut _,
        _ => panic!("too many arguments: {}", arg_count),
    }
}

/// Gets the ruby function that receives `arg_count` arguments.
pub fn ruby_function(arg_count: usize) -> *mut extern fn() -> Value {
    match arg_count {
        0 => self::ruby_function0 as *mut _,
        1 => self::ruby_function1 as *mut _,
        2 => self::ruby_function2 as *mut _,
        3 => self::ruby_function3 as *mut _,
        4 => self::ruby_function4 as *mut _,
        5 => self::ruby_function5 as *mut _,
        6 => self::ruby_function6 as *mut _,
        7 => self::ruby_function7 as *mut _,
        8 => self::ruby_function8 as *mut _,
        9 => self::ruby_function9 as *mut _,
        _ => panic!("too many arguments: {}", arg_count),
    }
}

pub extern fn ruby_method0(receiver: Value, func_ptr: Value) -> Value {
    dispatch_method!(func_ptr, receiver => Method => ())
}

pub extern fn ruby_method1(receiver: Value, func_ptr: Value, a1: Value) -> Value {
    dispatch_method!(func_ptr, receiver => Method1 => (a1))
}

pub extern fn ruby_method2(receiver: Value, func_ptr: Value, a1: Value, a2: Value) -> Value {
    dispatch_method!(func_ptr, receiver => Method2 => (a1, a2))
}

pub extern fn ruby_method3(receiver: Value, func_ptr: Value,
                           a1: Value, a2: Value, a3: Value) -> Value {
    dispatch_method!(func_ptr, receiver => Method3 => (a1, a2, a3))
}

pub extern fn ruby_method4(receiver: Value, func_ptr: Value,
                           a1: Value, a2: Value, a3: Value, a4: Value) -> Value {
    dispatch_method!(func_ptr, receiver => Method4 => (a1, a2, a3, a4))
}

pub extern fn ruby_method5(receiver: Value, func_ptr: Value,
                           a1: Value, a2: Value, a3: Value, a4: Value,
                           a5: Value) -> Value {
    dispatch_method!(func_ptr, receiver => Method5 => (a1, a2, a3, a4, a5))
}

pub extern fn ruby_method6(receiver: Value, func_ptr: Value,
                           a1: Value, a2: Value, a3: Value, a4: Value,
                           a5: Value, a6: Value) -> Value {
    dispatch_method!(func_ptr, receiver => Method6 => (a1, a2, a3, a4, a5, a6))
}

pub extern fn ruby_method7(receiver: Value, func_ptr: Value,
                           a1: Value, a2: Value, a3: Value, a4: Value,
                           a5: Value, a6: Value, a7: Value) -> Value {
    dispatch_method!(func_ptr, receiver => Method7 => (a1, a2, a3, a4, a5, a6, a7))
}

pub extern fn ruby_method8(receiver: Value, func_ptr: Value,
                           a1: Value, a2: Value, a3: Value, a4: Value,
                           a5: Value, a6: Value, a7: Value, a8: Value) -> Value {
    dispatch_method!(func_ptr, receiver => Method8 => (a1, a2, a3, a4, a5, a6, a7, a8))
}

pub extern fn ruby_method9(receiver: Value, func_ptr: Value,
                           a1: Value, a2: Value, a3: Value, a4: Value,
                           a5: Value, a6: Value, a7: Value, a8: Value, a9: Value) -> Value {
    dispatch_method!(func_ptr, receiver => Method9 => (a1, a2, a3, a4, a5, a6, a7, a8, a9))
}

pub extern fn ruby_function0(_receiver: Value, func_ptr: Value) -> Value {
    dispatch_function!(func_ptr => Function => ())
}

pub extern fn ruby_function1(_receiver: Value, func_ptr: Value, a1: Value) -> Value {
    dispatch_function!(func_ptr => Function1 => (a1))
}

pub extern fn ruby_function2(_receiver: Value, func_ptr: Value, a1: Value, a2: Value) -> Value {
    dispatch_function!(func_ptr => Function2 => (a1, a2))
}

pub extern fn ruby_function3(_receiver: Value, func_ptr: Value, a1: Value, a2: Value, a3: Value) -> Value {
    dispatch_function!(func_ptr => Function3 => (a1, a2, a3))
}

pub extern fn ruby_function4(_receiver: Value, func_ptr: Value, a1: Value, a2: Value, a3: Value,
                             a4: Value) -> Value {
    dispatch_function!(func_ptr => Function4 => (a1, a2, a3, a4))
}

pub extern fn ruby_function5(_receiver: Value, func_ptr: Value, a1: Value, a2: Value, a3: Value,
                             a4: Value, a5: Value) -> Value {
    dispatch_function!(func_ptr => Function5 => (a1, a2, a3, a4, a5))
}

pub extern fn ruby_function6(_receiver: Value, func_ptr: Value, a1: Value, a2: Value, a3: Value,
                             a4: Value, a5: Value, a6: Value) -> Value {
    dispatch_function!(func_ptr => Function6 => (a1, a2, a3, a4, a5, a6))
}

pub extern fn ruby_function7(_receiver: Value, func_ptr: Value, a1: Value, a2: Value, a3: Value,
                             a4: Value, a5: Value, a6: Value, a7: Value) -> Value {
    dispatch_function!(func_ptr => Function7 => (a1, a2, a3, a4, a5, a6, a7))
}

pub extern fn ruby_function8(_receiver: Value, func_ptr: Value, a1: Value, a2: Value, a3: Value,
                             a4: Value, a5: Value, a6: Value, a7: Value,
                             a8: Value) -> Value {
    dispatch_function!(func_ptr => Function8 => (a1, a2, a3, a4, a5, a6, a7, a8))
}

pub extern fn ruby_function9(_receiver: Value, func_ptr: Value, a1: Value, a2: Value, a3: Value,
                             a4: Value, a5: Value, a6: Value, a7: Value,
                             a8: Value, a9: Value) -> Value {
    dispatch_function!(func_ptr => Function9 => (a1, a2, a3, a4, a5, a6, a7, a8, a9))
}

mod helpers {
    use super::Receiver;
    use rurust::Value;
    use std::mem;
    use super::FunctionPointer;

    /// Gets a reference to the Rust struct from the associated Ruby object.
    pub fn reference_to_struct<'a>(ruby_object: Value) -> &'a mut Receiver {
        let ptr = ruby_object.call_no_args("object_pointer").to_u64() as usize;
        let receiver: &mut Receiver = unsafe { mem::transmute(ptr) };
        receiver
    }

    /// Creates a function from a function pointer given from Ruby.
    pub fn function_pointer(function_pointer: Value) -> FunctionPointer {
        let ptr = function_pointer.to_u64() as usize;

        unsafe { mem::transmute(ptr) }
    }
}

