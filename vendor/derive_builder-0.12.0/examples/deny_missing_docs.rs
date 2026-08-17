//! Some people may have `#![deny(missing_docs)]` in their crate.
//!
//! NOTE: This can only be tested in examples, but not integration tests.
#![deny(missing_docs)]

#[macro_use]
extern crate derive_builder;

/// Traditional form of communication.
#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Letter {
    /// Be creative.
    pub message: String,
}

fn main() {
    let x = LetterBuilder::default()
        .message("Hello World!")
        .build()
        .unwrap();
    println!("{}", x.message);
}
