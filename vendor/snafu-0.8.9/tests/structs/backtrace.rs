use snafu::{prelude::*, Backtrace, ErrorCompat};

#[test]
fn can_include_a_backtrace_in_leaf() {
    #[derive(Debug, Snafu)]
    struct Error {
        backtrace: Backtrace,
    }

    let e = Snafu.build();
    let backtrace = ErrorCompat::backtrace(&e);
    assert!(backtrace.is_some());
}

#[test]
fn can_include_a_backtrace_with_source() {
    use snafu::IntoError;

    #[derive(Debug, Snafu)]
    struct InnerError;

    #[derive(Debug, Snafu)]
    struct Error {
        source: InnerError,
        backtrace: Backtrace,
    }

    let i = InnerSnafu.build();
    let e = Snafu.into_error(i);
    let backtrace = ErrorCompat::backtrace(&e);
    assert!(backtrace.is_some());
}

#[test]
fn can_include_a_backtrace_with_no_context() {
    #[derive(Debug, Snafu)]
    struct InnerError;

    #[derive(Debug, Snafu)]
    #[snafu(context(false))]
    struct Error {
        source: InnerError,
        backtrace: Backtrace,
    }

    let i = InnerSnafu.build();
    let e = Error::from(i);
    let backtrace = ErrorCompat::backtrace(&e);
    assert!(backtrace.is_some());
}
