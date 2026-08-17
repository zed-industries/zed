#![deny(single_use_lifetimes)]

use snafu::prelude::*;

#[test]
fn an_error_with_generic_lifetimes_does_not_trigger_the_lint() {
    #[derive(Debug, Snafu)]
    struct _Error<'id> {
        to: &'id u32,
    }
}
