# plugger

[![Build Status](https://travis-ci.org/dylanmckay/plugger.svg?branch=master)](https://travis-ci.org/dylanmckay/plugger)

*NOTE*: Still some time away from being functional.

Automatic scripting support for Rust.

The idea is that you simply mark a `struct` as `#[pluggable]` and then we should
be able to get a scripting language to easily hook into it.

I'm working on Ruby support off the bat.

As this uses syntax extensions, it can only be used with Rust nightly.

