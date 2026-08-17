use crate::prelude::*;

// This used to trigger the `unused_parens` lint
#[test]
fn func_with_skipped_generic_arg() {
    #[builder]
    fn sut(arg: &(impl Clone + Default)) -> impl Clone {
        arg.clone()
    }

    sut().arg(&32).call();
}

// Test for https://github.com/elastio/bon/pull/349
#[test]
fn clippy_wrong_self_convention() {
    // This used to trigger the `clippy::wrong_self_convention` lint
    #[derive(Builder)]
    struct Sut {
        is_unique: bool,
    }

    let value = Sut::builder().is_unique(true).build();
    let _ = value.is_unique;
}
