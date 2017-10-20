#![feature(plugin)]
#![plugin(plugger)]

#![cfg(test)]

extern crate plugger_ruby;
extern crate plugger_core;

use plugger_ruby::{Ruby, Value};

#[pluggable]
#[derive(Debug)]
pub struct Player {
    name: String,
    x: i32,
    y: i32,
    z: i32,
}

#[pluggable]
impl Player {
    pub fn name(&self) -> String { self.name.clone() }

    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn z(&self) -> i32 { self.z }

    pub fn longest_name(&self, other: &Player) -> String {
        if self.name.len() >= other.name.len() {
            self
        } else {
            other
        }.name.clone()
    }

    pub fn move_left(&mut self) { self.x += 1; }
}

impl Default for Player {
    fn default() -> Player {
        Player {
            name: "Bob".to_owned(),
            x: 453,
            y: -244,
            z: 0xbeef,
        }
    }
}

// We only have one #[test] entry point because we can only have
// one active VM at a time and you can't specify '--test-threads' in
// the Cargo.toml.
//
// This also means we can run tests in this crate by running 'cargo test'
// in the workspace root.
#[test]
fn plugger() {
    let mut ruby = Ruby::new().expect("failed to create Ruby VM");

    can_access_rust_methods_from_ruby(&mut ruby);
    correctly_marshalls_rust_strings(&mut ruby);
    accepts_same_rust_object_as_non_self_argument(&mut ruby);
    returns_nil_if_no_retvalue_in_rust(&mut ruby);
}

/// We should be able to call simple Rust methods from Ruby.
fn can_access_rust_methods_from_ruby(ruby: &mut Ruby) {
    let mut player = Player::default();

    ruby.plug("player", &mut player);

    assert_eq!(Value::integer(player.x as _), ruby.eval("PLAYER.x").unwrap());
    assert_eq!(Value::integer(player.y as _), ruby.eval("PLAYER.y").unwrap());
    assert_eq!(Value::integer(player.z as _), ruby.eval("PLAYER.z").unwrap());
}

/// We should be able to return String objects in Rust and use it from Ruby.
fn correctly_marshalls_rust_strings(ruby: &mut Ruby) {
    let mut player = Player::default();

    ruby.plug("player", &mut player);

    assert_eq!(Value::string(player.name()), ruby.eval("PLAYER.name").unwrap());
}

/// We should be able to pass a Player from Ruby to Rust in an argument.
fn accepts_same_rust_object_as_non_self_argument(ruby: &mut Ruby) {
    let mut long_name_player = Player { name: "long as shit name".to_owned(), ..Player::default() };
    let mut short_name_player = Player { name: "bar".to_owned(), ..Player::default() };

    ruby.plug("long_name_player", &mut long_name_player);
    ruby.plug("short_name_player", &mut short_name_player);

    let longest_name = Value::string(long_name_player.name);
    assert_eq!(longest_name, ruby.eval("SHORT_NAME_PLAYER.longest_name(LONG_NAME_PLAYER)").unwrap());
}

/// If no return value is specified in Rust, the marshall function should automatically
/// return the Ruby value `nil`.
fn returns_nil_if_no_retvalue_in_rust(ruby: &mut Ruby) {
    let mut player = Player::default();

    ruby.plug("player", &mut player);

    assert_eq!(Value::nil(), ruby.eval("PLAYER.move_left").unwrap());
}

