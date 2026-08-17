//! This example shows using custom validation with a non-string error type.
//!
//! This relies on how the generated build function is constructed; the validator
//! is invoked in conjunction with the `?` operator, so anything that converts to
//! the generated `FooBuilderError` type is valid.
#![allow(dead_code)]

#[macro_use]
extern crate derive_builder;

use derive_builder::UninitializedFieldError;
use std::fmt;

fn validate_age(builder: &ExampleBuilder) -> Result<(), Error> {
    match builder.age {
        Some(age) if age > 150 => Err(Error::UnrealisticAge(age)),
        _ => Ok(()),
    }
}

#[derive(Debug, Builder)]
#[builder(setter(into), build_fn(validate = "validate_age", error = "Error"))]
struct Example {
    name: String,
    age: usize,
}

enum Error {
    UninitializedField(&'static str),
    UnrealisticAge(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnrealisticAge(age) => write!(f, "Nobody is {} years old", age),
            Self::UninitializedField(field) => write!(f, "Required field '{}' not set", field),
        }
    }
}

impl From<UninitializedFieldError> for Error {
    fn from(e: UninitializedFieldError) -> Self {
        Self::UninitializedField(e.field_name())
    }
}

fn main() {
    let person_err = ExampleBuilder::default()
        .name("Jane Doe")
        .age(200usize)
        .build()
        .unwrap_err();
    println!("{}", person_err);
}
