#[macro_use]
extern crate derive_builder;

fn validate_age(age: usize) -> Result<(), Error> {
    if age > 200 {
        Err(Error::UnrealisticAge(age))
    } else {
        Ok(())
    }
}

fn check_person(builder: &PersonBuilder) -> Result<(), Error> {
    if let Some(age) = builder.age {
        validate_age(age)
    } else {
        Ok(())
    }
}

#[derive(Builder)]
#[builder(build_fn(validate = "check_person", error = "Error"))]
struct Person {
    name: String,
    age: usize,
}

// NOTE: This enum has a variant for the uninitialized field case (called MissingData)
// but has forgotten `impl From<derive_builder::UninitializedFieldError>`, which is a
// compile-blocking mistake.
#[derive(Debug)]
enum Error {
    /// A required field is not filled out.
    MissingData(&'static str),
    UnrealisticAge(usize),
}

fn main() {}
