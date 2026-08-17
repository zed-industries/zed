use snafu::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Snafu, PartialEq)]
enum LeafError {
    #[snafu(display("User ID {user_id} is invalid"))]
    InvalidUser { user_id: i32 },
    #[snafu(display("no user available"))]
    _MissingUser,
}

#[derive(Debug, Clone, Snafu, PartialEq)]
enum MiddleError {
    #[snafu(display("failed to check the user"))]
    CheckUser { source: LeafError },
}

#[derive(Debug, Clone, Snafu, PartialEq)]
enum Error {
    #[snafu(display("access control failure"))]
    AccessControl { source: MiddleError },
}

#[track_caller]
fn assert_eq_debug(a: impl Debug, b: impl Debug) {
    assert_eq!(format!("{:?}", a), format!("{:?}", b));
}

#[test]
fn chain_compat_iterates() {
    use snafu::{ChainCompat, IntoError};

    let bottom_error = InvalidUserSnafu { user_id: 12 }.build();
    let middle_error = CheckUserSnafu.into_error(bottom_error.clone());
    let error = AccessControlSnafu.into_error(middle_error.clone());

    let errors: Vec<_> = ChainCompat::new(&error).collect();

    assert_eq_debug(&errors[0], &error);
    assert_eq_debug(&errors[1], &middle_error);
    assert_eq_debug(&errors[2], &bottom_error);
}

#[test]
fn errorcompat_chain_iterates() {
    use snafu::{ErrorCompat, IntoError};

    let bottom_error = InvalidUserSnafu { user_id: 12 }.build();
    let middle_error = CheckUserSnafu.into_error(bottom_error.clone());
    let error = AccessControlSnafu.into_error(middle_error.clone());

    let errors: Vec<_> = ErrorCompat::iter_chain(&error).collect();

    assert_eq_debug(&errors[0], &error);
    assert_eq_debug(&errors[1], &middle_error);
    assert_eq_debug(&errors[2], &bottom_error);
}

#[test]
fn can_downcast_chain_values() {
    use snafu::{ChainCompat, IntoError};

    let bottom_error = InvalidUserSnafu { user_id: 12 }.build();
    let middle_error = CheckUserSnafu.into_error(bottom_error.clone());
    let error = AccessControlSnafu.into_error(middle_error.clone());

    let errors: Vec<_> = ChainCompat::new(&error).collect();

    assert_eq!(Some(&error), errors[0].downcast_ref::<Error>());
    assert_eq!(Some(&middle_error), errors[1].downcast_ref::<MiddleError>());
    assert_eq!(Some(&bottom_error), errors[2].downcast_ref::<LeafError>());
}
