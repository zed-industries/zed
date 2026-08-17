//! This test ensures custom errors don't need a conversion from `UninitializedFieldError`
//! if uninitialized fields are impossible.

#[macro_use]
extern crate derive_builder;

#[derive(Default, Builder)]
#[builder(default, build_fn(validate = "check_person", error = "Error"))]
struct Person {
    name: String,
    age: u16,
}

/// An error that deliberately doesn't have `impl From<UninitializedFieldError>`; as long
/// as `PersonBuilder` uses `Person::default` then missing field errors are never possible.
enum Error {
    UnpopularName(String),
    UnrealisticAge(u16),
}

fn check_age_realistic(age: u16) -> Result<(), Error> {
    if age > 150 {
        Err(Error::UnrealisticAge(age))
    } else {
        Ok(())
    }
}

fn check_name_popular(name: &str) -> Result<(), Error> {
    if name.starts_with('B') {
        Err(Error::UnpopularName(name.to_string()))
    } else {
        Ok(())
    }
}

fn check_person(builder: &PersonBuilder) -> Result<(), Error> {
    if let Some(age) = &builder.age {
        check_age_realistic(*age)?;
    }

    if let Some(name) = &builder.name {
        check_name_popular(name)?;
    }

    Ok(())
}

fn main() {}
