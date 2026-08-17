//! This example shows combining generics with custom errors and validation.
//!
//! Note the use of the type parameter in the `#[builder(...)]` attribute.
#![allow(dead_code)]

#[macro_use]
extern crate derive_builder;

use derive_builder::UninitializedFieldError;

trait Popular {
    fn is_popular(&self) -> bool;
}

impl<'a> Popular for &'a str {
    fn is_popular(&self) -> bool {
        !self.starts_with('b')
    }
}

#[derive(Debug, Builder)]
#[builder(build_fn(validate = "check_person", error = "Error<N>"))]
struct Person<N: Popular + Clone> {
    name: N,
    age: u16,
}

#[derive(Debug)]
enum Error<N> {
    UninitializedField(&'static str),
    UnpopularName(N),
}

impl<N> From<UninitializedFieldError> for Error<N> {
    fn from(error: UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

fn check_person<N: Popular + Clone>(builder: &PersonBuilder<N>) -> Result<(), Error<N>> {
    if let Some(name) = &builder.name {
        if !name.is_popular() {
            return Err(Error::UnpopularName(name.clone()));
        }
    }

    Ok(())
}

fn main() {
    dbg!(PersonBuilder::default()
        .name("bill")
        .age(71)
        .build()
        .unwrap_err());
}
