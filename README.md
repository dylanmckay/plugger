# plugger-ruby

[![Build Status](https://travis-ci.org/dylanmckay/plugger.svg)](https://travis-ci.org/dylanmckay/plugger)
[![Crates.io](https://img.shields.io/crates/v/plugger.svg)](https://crates.io/crates/plugger)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Embed Ruby plugins directly into your Rust project!

Requires Rust nightly.

## Purpose

The purpose of this library is to allow scripting in your Rust
projects as easy as possible.

The library itself consists of two main parts - a Ruby VM and a
syntax extension which creates Ruby wrappers over your `struct`s and
`impl`s so they can be used directly from Ruby.

It should be possible to simply annotate a type with `#[pluggable]` and use
it directly from Ruby.

The thing that separates the library from the others is that it allows you to
*share* your Rust code with Ruby, as opposed to writing Ruby objects in Rust.


## Features

- [x] Creation of a Ruby VM and `eval`uating Ruby code
- [x] Calling methods on Rust objects from Ruby
- [ ] Accessing public struct fields from Ruby
- [ ] Creating new Rust objects via Ruby
- [ ] Complicated types such as enums, tuples
- [x] Automatic marshalling of Ruby arguments into Rust types
- [ ] Automatic marshalling of Rust return types into Ruby values
- [ ] Support for Python

## Examples

Check out [`plugger/examples`](plugger/examples).

