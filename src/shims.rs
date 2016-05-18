use rurust::Value;
use plugger_core::Pluggable;

struct Abc;

#[no_mangle]
pub extern fn ruby_method_arg0(receiver: Value, function_address: Value) -> Value {
    let method_ptr_str = function_address.display_string();
    let method_ptr = usize::from_str_radix(&method_ptr_str, 10).unwrap();
    let object_ptr_str = receiver.call_no_args("object_pointer").display_string();
    let object_ptr = usize::from_str_radix(&object_ptr_str, 10).unwrap();

    let obj_pointer: *mut Abc = unsafe { ::std::mem::transmute(object_ptr) };
    let obj: &Abc = unsafe { ::std::mem::transmute(obj_pointer) };

    let func: fn(&Abc) -> Value = unsafe { ::std::mem::transmute(method_ptr) };
    func(obj);

    Value::nil()
}

