use snafu::{prelude::*, Whatever};

type Result<T, E = Whatever> = std::result::Result<T, E>;

// Using fully-qualified paths here to ensure the module's imports are
// minimal
#[derive(Debug, snafu::Snafu)]
pub struct UnderlyingError;
pub fn underlying(success: bool) -> Result<i32, UnderlyingError> {
    snafu::ensure!(success, UnderlyingSnafu);
    Ok(1)
}

#[test]
fn implements_error() {
    fn check<T: std::error::Error>() {}
    check::<Whatever>();
}

#[test]
fn does_not_need_a_cause() {
    use std::error::Error as _;

    fn exercise(success: bool) -> Result<i32> {
        if !success {
            whatever!("I caused a problem {}", 42);
        }
        Ok(1)
    }

    let e = exercise(false).unwrap_err();
    assert!(e.source().is_none());
}

#[test]
fn can_wrap_cause_with_a_formatted_string_via_macro() {
    use std::error::Error as _;

    fn exercise(success: bool) -> Result<i32> {
        let v = whatever!(underlying(success), "Something else happened {}", 42);
        Ok(v + 1)
    }

    assert!(matches!(exercise(true), Ok(2)));
    let e = exercise(false).unwrap_err();
    assert_eq!("Something else happened 42", e.to_string());

    let src = e.source().expect("Must have a source");
    let src = src.downcast_ref::<UnderlyingError>();
    assert!(src.is_some());
}

#[test]
fn can_wrap_cause_with_a_formatted_string_via_trait() {
    use std::error::Error as _;

    fn exercise(success: bool) -> Result<i32> {
        let v = underlying(success)
            .with_whatever_context(|_| format!("Something else happened {}", 42))?;
        Ok(v + 1)
    }

    assert!(matches!(exercise(true), Ok(2)));
    let e = exercise(false).unwrap_err();
    assert_eq!("Something else happened 42", e.to_string());

    let src = e.source().expect("Must have a source");
    let src = src.downcast_ref::<UnderlyingError>();
    assert!(src.is_some());
}

#[test]
fn can_be_recursive() {
    use std::error::Error as _;

    fn inner(success: bool) -> Result<i32> {
        if !success {
            whatever!("Inner error");
        }
        Ok(1)
    }

    fn outer(success: bool) -> Result<i32> {
        let v = whatever!(inner(success), "Outer error");
        Ok(v + 1)
    }

    assert!(matches!(outer(true), Ok(2)));
    let outer_error = outer(false).unwrap_err();
    let inner_error = outer_error.source().expect("Must have a source");
    assert!(inner_error.downcast_ref::<Whatever>().is_some());
    assert!(inner_error.source().is_none());
}

#[test]
fn has_a_backtrace() {
    use snafu::ErrorCompat;

    fn exercise(success: bool) -> Result<i32> {
        let v = whatever!(underlying(success), "Something else happened {}", 42);
        Ok(v + 1)
    }

    let e = exercise(false).unwrap_err();
    let bt = ErrorCompat::backtrace(&e).expect("Must have a backtrace");
    assert!(bt.to_string().contains("has_a_backtrace"));
}
