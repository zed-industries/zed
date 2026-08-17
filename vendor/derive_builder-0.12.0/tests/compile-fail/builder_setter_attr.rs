#![deny(unused_must_use)]

#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
pub struct Lorem {
    ok: String,

    #[builder_setter_attr(must_use)]
    broken: usize,
}

fn main() {
    LoremBuilder::default().broken(42);
}
