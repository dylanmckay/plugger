use rurust::Value;

pub extern fn ruby_method_arg0(self_obj: Value) -> Value {
    println!("shim: self is {:?}", self_obj);
    Value::nil()
    // let internal_pointer = self_obj.call_no_args("object_pointer");
}
