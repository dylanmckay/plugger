#![feature(plugin)]
#![plugin(plugger)]

extern crate plugger_ruby;
extern crate plugger_core;

use std::io::Write;

#[pluggable]
pub struct Player {
    x: f32,
    y: f32,
    z: f32,
}

#[pluggable]
impl Player {
    pub fn info(&self) -> String {
        println!("Player at ({},{},{})", self.x, self.y, self.z);
        "meooooooow".to_owned()
    }

    pub fn set_health(&self, health: u8) {
        println!("setting health to '{}'", health);
    }

    pub fn set_foobar(foobar: String) {
        println!("setting foobar to '{}'", foobar);
    }

    pub fn other(&self) -> u32 {
        12345
    }

    fn private_method(&self) -> f32 {
        self.x + self.y
    }
}

fn main() {
    let mut player = Player { x: 1.0, y: 2.0, z: 3.0 };

    let mut ruby = plugger_ruby::Ruby::new().unwrap();
    ruby.plug("player", &mut player);

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

