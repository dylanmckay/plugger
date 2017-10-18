#![feature(plugin)]

#![plugin(plugger_macros)]
#![allow(dead_code, unused_variables)]

extern crate plugger_core;
extern crate plugger_ruby;
#[derive(Debug)]
pub struct Value(u32);

#[pluggable]
#[derive(Debug)]
struct Abc
{
    pub num: Value,
    pub foo: Value,
    pub val: Value,
}

#[pluggable]
impl Abc
{
    #[plug]
    pub fn abc() { println!("abc!!!") }

    #[plug]
    pub fn foo(success: bool) -> String {
        println!("foo!!!");
        "foo result".to_owned()
    }

    #[plug]
    pub fn bar(val: u16) { println!("bar!!!"); }

    #[plug]
    pub fn biz(&self, bop: bool) {
        println!("value: {}", bop);
    }

    fn hidden() { }
}

pub struct TestMarshall;

impl plugger_core::Marshall for TestMarshall {
    type Value = String;

    fn to_bool(value: Self::Value) -> bool { value.parse().unwrap_or(false) }

    fn to_u8(value: Self::Value) -> u8 { value.parse().unwrap_or(0) }
    fn to_u16(value: Self::Value) -> u16 { value.parse().unwrap_or(0) }
    fn to_u32(value: Self::Value) -> u32 { value.parse().unwrap_or(0) }
    fn to_u64(value: Self::Value) -> u64 { value.parse().unwrap_or(0) }
    fn to_i8(value: Self::Value) -> i8 { value.parse().unwrap_or(0) }
    fn to_i16(value: Self::Value) -> i16 { value.parse().unwrap_or(0) }
    fn to_i32(value: Self::Value) -> i32 { value.parse().unwrap_or(0) }
    fn to_i64(value: Self::Value) -> i64 { value.parse().unwrap_or(0) }

    fn to_f32(value: Self::Value) -> f32 { value.parse().unwrap_or(0.0) }
    fn to_f64(value: Self::Value) -> f64 { value.parse().unwrap_or(0.0) }

    fn to_string(value: Self::Value) -> String { value }
}

fn main() {
    let thing = Abc { num: Value(64113), foo: Value(2), val: Value(3) };

    thing.biz_marshall::<TestMarshall>("false".to_owned());
}

