#[macro_use]
extern crate derive_builder;

#[allow(dead_code)]
#[derive(Builder)]
struct Foo {
    lorem: bool,
}

#[allow(dead_code)]
#[derive(Builder)]
struct Bar {
    ipsum: bool,
}

fn main() { }
