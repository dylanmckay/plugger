use plugger_core;
use rurust::Value;

pub struct Marshall;

macro_rules! to_uint {
    ($ty:ident, $value:expr) => {
        {
            let v = $value.to_u64();

            if v > ($ty::max_value() as u64) {
                panic!("out of bounds");
            }
            v as $ty
        }
    }
}

macro_rules! to_int {
    ($ty:ident, $value:expr) => {
        {
            let v = $value.to_i64();

            if v > ($ty::max_value() as i64) ||
                v < ($ty::min_value() as i64) {
                panic!("out of bounds");
            }
            v as $ty
        }
    }
}

impl plugger_core::Marshall for Marshall {
    type Value = Value;

    fn to_bool(value: Value) -> bool {
        // TODO: we might want to do a truthy check.
        value.is_true()
    }

    fn to_u8(value: Value) -> u8 { to_uint!(u8, value) }
    fn to_u16(value: Value) -> u16 { to_uint!(u16, value) }
    fn to_u32(value: Value) -> u32 { to_uint!(u32, value) }
    fn to_u64(value: Value) -> u64 { value.to_u64() }
    fn to_i8(value: Value) -> i8 { to_int!(i8, value) }
    fn to_i16(value: Value) -> i16 { to_int!(i16, value) }
    fn to_i32(value: Value) -> i32 { to_int!(i32, value) }
    fn to_i64(value: Value) -> i64 { value.to_i64() }

    fn to_f32(value: Value) -> f32 {
        value.to_f64() as f32
    }

    fn to_f64(value: Value) -> f64 {
        value.to_f64()
    }

    fn to_string(value: Value) -> String {
        if let Some(s) = value.as_string() {
            s
        } else {
            panic!("Ruby value is not a String");
        }
    }

    fn from_bool(value: bool) -> Value { Value::boolean(value) }
    fn from_u8(value: u8) -> Value { Value::integer(value as i64) }
    fn from_u16(value: u16) -> Value { Value::integer(value as i64) }
    fn from_u32(value: u32) -> Value { Value::integer(value as i64) }
    fn from_u64(value: u64) -> Value { Value::integer(value as i64) } // FIXME: this may overflow
    fn from_i8(value: i8) -> Value { Value::integer(value as i64) }
    fn from_i16(value: i16) -> Value { Value::integer(value as i64) }
    fn from_i32(value: i32) -> Value { Value::integer(value as i64) }
    fn from_i64(value: i64) -> Value { Value::integer(value) }
    fn from_string(value: String) -> Value {
        Value::string(value)
    }
}

