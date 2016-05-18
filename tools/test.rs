#![feature(plugin)]

#![plugin(plugger_macros)]

extern crate plugger_ruby;
extern crate plugger_core;

use std::io::Write;

#[pluggable]
struct Animal {
    pub a: u32,
}

#[pluggable]
impl Animal
{
    #[plug]
    pub fn woof() { println!("woof!"); }

    #[plug]
    pub fn meow() { println!("meow!"); }

    #[plug]
    pub fn moo(&self) {
        println!("moo!! = {}", self.a);
    }
}

fn main() {
    let mut animal = Animal { a: 512 };

    let mut ruby = plugger_ruby::Ruby::new().unwrap();
    ruby.plug(&mut animal);

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
