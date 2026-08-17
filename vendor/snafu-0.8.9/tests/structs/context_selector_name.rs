use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(Moo)))]
struct Alpha;

fn alpha_usage() -> Result<(), Alpha> {
    AlphaMoo.fail()
}

#[test]
fn alpha_implements_error() {
    check::<Alpha>();
    alpha_usage().unwrap_err();
}

#[derive(Debug, Snafu)]
#[snafu(context(suffix(Baa)))]
struct TrimsWhenEndingInError;

fn trimming_usage() -> Result<(), TrimsWhenEndingInError> {
    TrimsWhenEndingInBaa.fail()
}

#[test]
fn trimming_implements_error() {
    check::<TrimsWhenEndingInError>();
    trimming_usage().unwrap_err();
}

// `context(suffix(false))` doesn't make sense for structs because the
// struct itself already has that name.

fn check<T: std::error::Error>() {}
