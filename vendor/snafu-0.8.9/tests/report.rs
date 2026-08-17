use snafu::{prelude::*, CleanedErrorText, IntoError, Report};
use std::process::ExitCode;

macro_rules! assert_contains {
    (needle: $needle:expr, haystack: $haystack:expr) => {
        assert!(
            $haystack.contains($needle),
            "Expected {:?} to include {:?}",
            $haystack,
            $needle,
        )
    };
}

macro_rules! assert_not_contains {
    (needle: $needle:expr, haystack: $haystack:expr) => {
        assert!(
            !$haystack.contains($needle),
            "Expected {:?} to not include {:?}",
            $haystack,
            $needle,
        )
    };
}

#[test]
fn includes_the_error_display_text() {
    #[derive(Debug, Snafu)]
    #[snafu(display("This is my Display text!"))]
    struct Error;

    let r = Report::from_error(Error);
    let msg = r.to_string();

    let expected = "This is my Display text!";
    assert_contains!(needle: expected, haystack: msg);
}

#[test]
fn includes_the_source_display_text() {
    #[derive(Debug, Snafu)]
    #[snafu(display("This is my inner Display"))]
    struct InnerError;

    #[derive(Debug, Snafu)]
    #[snafu(display("This is my outer Display"))]
    struct OuterError {
        source: InnerError,
    }

    let e = OuterSnafu.into_error(InnerError);
    let r = Report::from_error(e);
    let msg = r.to_string();

    let expected = "This is my inner Display";
    assert_contains!(needle: expected, haystack: msg);
}

#[test]
fn reduces_duplication_of_the_source_display_text() {
    // Including the source in the Display message is discouraged but
    // quite common.

    #[derive(Debug, Snafu)]
    #[snafu(display("Level 0"))]
    struct Level0Error;

    #[derive(Debug, Snafu)]
    #[snafu(display("Level 1: {source}"))]
    struct Level1Error {
        source: Level0Error,
    }

    #[derive(Debug, Snafu)]
    #[snafu(display("Level 2: {source}"))]
    struct Level2Error {
        source: Level1Error,
    }

    let e = Level2Snafu.into_error(Level1Snafu.into_error(Level0Error));
    let raw_msg = e.to_string();

    let expected = "Level 2: Level 1";
    assert_contains!(needle: expected, haystack: raw_msg);

    let r = Report::from_error(e);
    let msg = r.to_string();

    assert_not_contains!(needle: expected, haystack: msg);
}

#[test]
fn removes_complete_duplication_in_the_source_display_text() {
    // Including **only** the source in the Display message is also
    // discouraged but occurs.

    #[derive(Debug, Snafu)]
    #[snafu(display("Level 0"))]
    struct Level0Error;

    #[derive(Debug, Snafu)]
    #[snafu(display("{source}"))]
    struct Level1Error {
        source: Level0Error,
    }

    #[derive(Debug, Snafu)]
    #[snafu(display("{source}"))]
    struct Level2Error {
        source: Level1Error,
    }

    let e = Level2Snafu.into_error(Level1Snafu.into_error(Level0Error));
    let raw_msg = e.to_string();

    assert_contains!(needle: "Level 0", haystack: raw_msg);

    let r = Report::from_error(e);
    let msg = r.to_string();

    assert_not_contains!(needle: "Caused by", haystack: msg);
}

#[test]
fn debug_and_display_are_the_same() {
    #[derive(Debug, Snafu)]
    #[snafu(display("This is my inner Display"))]
    struct InnerError;

    #[derive(Debug, Snafu)]
    #[snafu(display("This is my outer Display"))]
    struct OuterError {
        source: InnerError,
    }

    let e = OuterSnafu.into_error(InnerError);
    let r = Report::from_error(e);

    let display = format!("{r}",);
    let debug = format!("{r:?}");

    assert_eq!(display, debug);
}

/// `Report as Termination` prints-out the "Error:" prefix.  Ensure that `Report as Display` does
/// not also add such a prefix, to avoid printing-out "Error: Error: ...".
#[test]
fn display_not_prefixed() {
    #[derive(Debug, Snafu)]
    #[snafu(display("This is my Display text!"))]
    struct Error;

    let r = Report::from_error(Error);
    let msg = r.to_string();
    let msg = msg.trim_start();

    assert!(!msg.starts_with("Err"));
    assert!(!msg.starts_with("err"));
}

#[test]
fn procedural_macro_works_with_result_return_type() {
    #[derive(Debug, Snafu)]
    struct Error;

    #[snafu::report]
    fn mainlike_result() -> Result<(), Error> {
        Ok(())
    }

    let _: Report<Error> = mainlike_result();
}

#[test]
fn procedural_macro_works_with_tough_inference() {
    #[derive(Debug, Snafu)]
    struct InnerError;

    #[derive(Debug, Snafu)]
    struct OuterError {
        source: InnerError,
    }

    fn inner() -> Result<(), InnerError> {
        InnerSnafu.fail()
    }

    #[snafu::report]
    fn mainlike_result() -> Result<(), OuterError> {
        loop {
            inner().context(OuterSnafu)?;
        }
    }

    let _: Report<_> = mainlike_result();
}

#[test]
fn termination_returns_failure_code() {
    use std::process::Termination;

    #[derive(Debug, Snafu)]
    struct Error;

    let r = Report::from_error(Error);
    let code: ExitCode = r.report();

    assert!(
        nasty_hack_exit_code_eq(code, ExitCode::FAILURE),
        "Wanted {:?} but got {:?}",
        ExitCode::FAILURE,
        code,
    );
}

fn nasty_hack_exit_code_eq(left: ExitCode, right: ExitCode) -> bool {
    use std::mem;

    #[cfg(target_os = "windows")]
    type ExitCodeSize = u32;
    #[cfg(not(target_os = "windows"))]
    type ExitCodeSize = u8;

    let (left, right): (ExitCodeSize, ExitCodeSize) = unsafe {
        assert_eq!(mem::size_of::<ExitCodeSize>(), mem::size_of::<ExitCode>());
        (mem::transmute(left), mem::transmute(right))
    };

    left == right
}

#[derive(Debug, Snafu)]
struct TestFunctionError;

#[test]
#[snafu::report]
fn procedural_macro_works_with_test_functions() -> Result<(), TestFunctionError> {
    Ok(())
}

#[track_caller]
fn assert_cleaning_step(iter: &mut CleanedErrorText, text: &str, removed_text: &str) {
    let (error, actual_text, actual_cleaned) =
        iter.next().expect("Iterator unexpectedly exhausted");
    let actual_original_text = error.to_string();

    let original_text = [text, removed_text].concat();
    let cleaned = !removed_text.is_empty();

    assert_eq!(original_text, actual_original_text);
    assert_eq!(text, actual_text);
    assert_eq!(cleaned, actual_cleaned);
}

#[test]
fn cleaning_a_leaf_error_changes_nothing() {
    #[derive(Debug, Snafu)]
    #[snafu(display("But I am only C"))]
    struct C;

    let c = C;
    let mut iter = CleanedErrorText::new(&c);

    assert_cleaning_step(&mut iter, "But I am only C", "");
    assert!(iter.next().is_none());
}

#[test]
fn cleaning_nested_errors_removes_duplication() {
    #[derive(Debug, Snafu)]
    #[snafu(display("This is A: {source}"))]
    struct A {
        source: B,
    }

    #[derive(Debug, Snafu)]
    #[snafu(display("And this is B: {source}"))]
    struct B {
        source: C,
    }

    #[derive(Debug, Snafu)]
    #[snafu(display("But I am only C"))]
    struct C;

    let a = A {
        source: B { source: C },
    };
    let mut iter = CleanedErrorText::new(&a);

    assert_cleaning_step(&mut iter, "This is A", ": And this is B: But I am only C");
    assert_cleaning_step(&mut iter, "And this is B", ": But I am only C");
    assert_cleaning_step(&mut iter, "But I am only C", "");
    assert!(iter.next().is_none());
}
