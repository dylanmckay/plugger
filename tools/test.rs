#![feature(plugin)]

#![plugin(plugger_macros)]

extern crate plugger_ruby;
extern crate plugger_core;
extern crate rurust;

use std::io::Write;
use rurust::Value;

#[pluggable]
struct Animal {
    pub a: u32,
}

#[pluggable]
impl Animal
{
    #[plug]
    pub fn woof() -> Value {
        Value::nil()
    }

    #[plug]
    pub fn meow() -> Value {
        println!("meow!");
        Value::string("meooooooow")
    }

    #[plug]
    pub fn moo(&self) -> Value {
        println!("moo!! = {}", self.a);
        Value::fixnum(self.a as _)
    }
}

fn main() {
    let mut animal = Animal { a: 512 };

    let mut ruby = plugger_ruby::Ruby::new().unwrap();
    ruby.plug("animal", &mut animal);

    loop {
        let mut line = String::new();

        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut line).unwrap();

        match ruby.eval(&line) {
            Ok(val) => println!("=> {}", val),
            Err(e) => println!("{:?}", e),
        }
    }
}
