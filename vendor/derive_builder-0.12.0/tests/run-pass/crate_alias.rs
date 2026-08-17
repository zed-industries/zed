//! Test that an alias of derive_builder is valid in #[builder(crate = "...")]
//!
//! This test is imperfect, as it still passes without setting `crate = "..."`.
//! This is likely because `derive_builder` is automatically present in 2018
//! without needing the extern crate line.
//!
//! The test will fail if an incorrect alias is used, so it adds limited value.

#[macro_use]
extern crate derive_builder as db;

#[derive(Builder)]
#[builder(crate = "db")]
struct AliasedCrate {
    #[builder(setter(into))]
    lorem: String,
}

fn main() {
    AliasedCrateBuilder::default()
        .lorem("hello")
        .build()
        .unwrap();
}
