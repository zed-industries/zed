#![allow(missing_docs, reason = "may be removed in the future")]
#![allow(
    clippy::missing_const_for_fn,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::alloc_instead_of_core,
    reason = "irrelevant for tests"
)]

#[cfg(not(all(
    feature = "default",
    feature = "alloc",
    feature = "formatting",
    feature = "large-dates",
    feature = "local-offset",
    feature = "macros",
    feature = "parsing",
    feature = "quickcheck",
    feature = "serde-human-readable",
    feature = "serde-well-known",
    feature = "std",
    feature = "rand",
    feature = "serde",
)))]
#[test]
fn run_with_all_features() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug)]
    struct Error(std::process::ExitStatus);

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl std::error::Error for Error {}

    let status = std::process::Command::new("cargo")
        .args(["test", "--all-features"])
        .status()?;

    return if status.success() {
        Ok(())
    } else {
        Err(Box::new(Error(status)))
    };

    // Intentionally unreachable. This is to show the user a warning when they don't provide
    // `--all-features`.
    "Tests must be run with `--all-features`. Because the flag was not provided, `cargo test \
     --all-features` is run.";
}

macro_rules! require_all_features {
    ($($x:item)*) => {$(
        #[cfg(all(
            feature = "default",
            feature = "alloc",
            feature = "formatting",
            feature = "large-dates",
            feature = "local-offset",
            feature = "macros",
            feature = "parsing",
            feature = "quickcheck",
            feature = "serde-human-readable",
            feature = "serde-well-known",
            feature = "std",
            feature = "rand",
            feature = "serde",
        ))]
        $x
    )*};
}

require_all_features! {
    /// Assert that the given expression panics.
    macro_rules! assert_panic {
        ($($x:tt)*) => {
            assert!(std::panic::catch_unwind(|| {
                $($x)*
            })
            .is_err())
        }
    }

    /// `assert_eq!` or `assert_ne!` depending on the value of `$is_eq`.
    ///
    /// This provides better diagnostics than `assert_eq!($left == $right, $is_eq)`.
    macro_rules! assert_eq_ne {
        ($left:expr, $right:expr, $is_eq:expr $(, $($rest:tt)*)?) => {{
            if $is_eq {
                assert_eq!($left, $right $(, $($rest)*)?);
            } else {
                assert_ne!($left, $right $(, $($rest)*)?);
            }
        }}
    }

    mod convert;
    mod date;
    mod derives;
    mod duration;
    mod error;
    mod ext;
    mod format_description;
    mod formatting;
    mod instant;
    mod macros;
    mod meta;
    mod month;
    mod offset_date_time;
    mod parse_format_description;
    mod parsed;
    mod parsing;
    mod primitive_date_time;
    #[path = "quickcheck.rs"]
    mod quickcheck_mod;
    mod rand;
    mod serde;
    mod serde_helpers;
    mod time;
    mod utc_date_time;
    mod utc_offset;
    mod util;
    mod weekday;

    #[cfg(__ui_tests)]
    #[test]
    fn compile_fail() {
        let tests = trybuild::TestCases::new();
        // Path is relative from `time/Cargo.toml`.
        tests.compile_fail("./tests/integration/compile-fail/*.rs");
    }
}
