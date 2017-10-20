#![feature(plugin)]
#![plugin(plugger)]

extern crate plugger_ruby;
extern crate plugger_core;

use std::io::Write;

#[pluggable]
#[derive(Debug)]
pub struct Player {
    x: f32,
    y: f32,
    z: f32,
}

#[pluggable]
#[derive(Debug)]
pub struct Enemy {
    x: f32,
    y: f32,
    z: f32,
}

#[pluggable]
impl Enemy {
}

#[pluggable]
impl Player {
    pub fn new() -> Self {
        Player { x: 61.0, y: 62.0, z: 63.0 }
    }

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

    pub fn check(&self, other: &Player) {
        println!("checking {:?} with {:?}", self, other);
    }

    pub fn other(&self) -> u32 {
        12345
    }

    #[allow(dead_code)]
    fn private_method(&self) -> f32 {
        self.x + self.y
    }
}

fn main() {
    let mut player = Player { x: 1.0, y: 2.0, z: 3.0 };
    let mut enemy = Enemy { x: 5.0, y: 3.0, z: 0.0 };

    let mut ruby = plugger_ruby::Ruby::new().unwrap();
    ruby.plug("player", &mut player);
    ruby.plug("enemy", &mut enemy);

    loop {
        let mut line = String::new();

        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        match line {
            "quit" | "exit" => break,
            _ => match ruby.eval(&line) {
                Ok(val) => println!("=> {:?}", val),
                Err(e) => println!("{:?}", e),
            },
        }
    }
}

