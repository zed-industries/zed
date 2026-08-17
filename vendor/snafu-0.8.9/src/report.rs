use crate::ChainCompat;
use core::fmt;

#[cfg(all(feature = "std", feature = "rust_1_61"))]
use std::process::{ExitCode, Termination};

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};

/// Opinionated solution to format an error in a user-friendly
/// way. Useful as the return type from `main` and test functions.
///
/// Most users will use the [`snafu::report`][] procedural macro
/// instead of directly using this type, but you can if you do not
/// wish to use the macro.
///
/// [`snafu::report`]: macro@crate::report
///
/// ## Rust 1.61 and up
///
/// Change the return type of the function to [`Report`][] and wrap
/// the body of your function with [`Report::capture`][].
///
/// ## Rust before 1.61
///
/// Use [`Report`][] as the error type inside of [`Result`][] and then
/// call either [`Report::capture_into_result`][] or
/// [`Report::from_error`][].
///
/// ## Nightly Rust
///
/// Enabling the [`unstable-try-trait` feature flag][try-ff] will
/// allow you to use the `?` operator directly:
///
/// ```rust
/// use snafu::{prelude::*, Report};
///
/// # #[cfg(all(feature = "unstable-try-trait", feature = "rust_1_61"))]
/// fn main() -> Report<PlaceholderError> {
///     let _v = may_fail_with_placeholder_error()?;
///
///     Report::ok()
/// }
/// # #[cfg(not(all(feature = "unstable-try-trait", feature = "rust_1_61")))] fn main() {}
/// # #[derive(Debug, Snafu)]
/// # struct PlaceholderError;
/// # fn may_fail_with_placeholder_error() -> Result<u8, PlaceholderError> { Ok(42) }
/// ```
///
/// [try-ff]: crate::guide::feature_flags#unstable-try-trait
///
/// ## Interaction with the Provider API
///
/// If you return a [`Report`][] from your function and enable the
/// [`unstable-provider-api` feature flag][provider-ff], additional
/// capabilities will be added:
///
/// 1. If provided, a [`Backtrace`][] will be included in the output.
/// 1. If provided, a [`ExitCode`][] will be used as the return value.
///
/// [provider-ff]: crate::guide::feature_flags#unstable-provider-api
/// [`Backtrace`]: crate::Backtrace
/// [`ExitCode`]: std::process::ExitCode
///
/// ## Stability of the output
///
/// The exact content and format of a displayed `Report` are not
/// stable, but this type strives to print the error and as much
/// user-relevant information in an easily-consumable manner
pub struct Report<E>(Result<(), E>);

impl<E> Report<E> {
    /// Convert an error into a [`Report`][].
    ///
    /// Recommended if you support versions of Rust before 1.61.
    ///
    /// ```rust
    /// use snafu::{prelude::*, Report};
    ///
    /// #[derive(Debug, Snafu)]
    /// struct PlaceholderError;
    ///
    /// fn main() -> Result<(), Report<PlaceholderError>> {
    ///     let _v = may_fail_with_placeholder_error().map_err(Report::from_error)?;
    ///     Ok(())
    /// }
    ///
    /// fn may_fail_with_placeholder_error() -> Result<u8, PlaceholderError> {
    ///     Ok(42)
    /// }
    /// ```
    pub fn from_error(error: E) -> Self {
        Self(Err(error))
    }

    /// Executes a closure that returns a [`Result`][], converting the
    /// error variant into a [`Report`][].
    ///
    /// Recommended if you support versions of Rust before 1.61.
    ///
    /// ```rust
    /// use snafu::{prelude::*, Report};
    ///
    /// #[derive(Debug, Snafu)]
    /// struct PlaceholderError;
    ///
    /// fn main() -> Result<(), Report<PlaceholderError>> {
    ///     Report::capture_into_result(|| {
    ///         let _v = may_fail_with_placeholder_error()?;
    ///
    ///         Ok(())
    ///     })
    /// }
    ///
    /// fn may_fail_with_placeholder_error() -> Result<u8, PlaceholderError> {
    ///     Ok(42)
    /// }
    /// ```
    pub fn capture_into_result<T>(body: impl FnOnce() -> Result<T, E>) -> Result<T, Self> {
        body().map_err(Self::from_error)
    }

    /// Executes a closure that returns a [`Result`][], converting any
    /// error to a [`Report`][].
    ///
    /// Recommended if you only support Rust version 1.61 or above.
    ///
    /// ```rust
    /// use snafu::{prelude::*, Report};
    ///
    /// #[derive(Debug, Snafu)]
    /// struct PlaceholderError;
    ///
    /// # #[cfg(feature = "rust_1_61")]
    /// fn main() -> Report<PlaceholderError> {
    ///     Report::capture(|| {
    ///         let _v = may_fail_with_placeholder_error()?;
    ///
    ///         Ok(())
    ///     })
    /// }
    /// # #[cfg(not(feature = "rust_1_61"))] fn main() {}
    ///
    /// fn may_fail_with_placeholder_error() -> Result<u8, PlaceholderError> {
    ///     Ok(42)
    /// }
    /// ```
    pub fn capture(body: impl FnOnce() -> Result<(), E>) -> Self {
        Self(body())
    }

    /// A [`Report`][] that indicates no error occurred.
    pub const fn ok() -> Self {
        Self(Ok(()))
    }
}

impl<E> From<Result<(), E>> for Report<E> {
    fn from(other: Result<(), E>) -> Self {
        Self(other)
    }
}

impl<E> fmt::Debug for Report<E>
where
    E: crate::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<E> fmt::Display for Report<E>
where
    E: crate::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Err(e) => fmt::Display::fmt(&ReportFormatter(e), f),
            _ => Ok(()),
        }
    }
}

#[cfg(all(feature = "std", feature = "rust_1_61"))]
impl<E> Termination for Report<E>
where
    E: crate::Error,
{
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                std::eprintln!("Error: {}", ReportFormatter(&e));

                #[cfg(feature = "unstable-provider-api")]
                {
                    use crate::error;

                    // Broken. https://github.com/rust-lang/rust/pull/114973
                    // error::request_value::<ExitCode>(&e)
                    //     .or_else(|| error::request_ref::<ExitCode>(&e).copied())

                    error::request_ref::<ExitCode>(&e)
                        .copied()
                        .unwrap_or(ExitCode::FAILURE)
                }

                #[cfg(not(feature = "unstable-provider-api"))]
                {
                    ExitCode::FAILURE
                }
            }
        }
    }
}

#[cfg(feature = "unstable-try-trait")]
impl<T, E> core::ops::FromResidual<Result<T, E>> for Report<E> {
    fn from_residual(residual: Result<T, E>) -> Self {
        Self(residual.map(drop))
    }
}

struct ReportFormatter<'a>(&'a dyn crate::Error);

impl<'a> fmt::Display for ReportFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "std")]
        {
            if trace_cleaning_enabled() {
                self.cleaned_error_trace(f)?;
            } else {
                self.error_trace(f)?;
            }
        }

        #[cfg(not(feature = "std"))]
        {
            self.error_trace(f)?;
        }

        #[cfg(feature = "unstable-provider-api")]
        {
            use crate::error;

            if let Some(bt) = error::request_ref::<crate::Backtrace>(self.0) {
                writeln!(f, "\nBacktrace:\n{}", bt)?;
            }
        }

        Ok(())
    }
}

impl<'a> ReportFormatter<'a> {
    fn error_trace(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "{}", self.0)?;

        let sources = ChainCompat::new(self.0).skip(1);
        let plurality = sources.clone().take(2).count();

        match plurality {
            0 => {}
            1 => writeln!(f, "\nCaused by this error:")?,
            _ => writeln!(f, "\nCaused by these errors (recent errors listed first):")?,
        }

        for (i, source) in sources.enumerate() {
            // Let's use 1-based indexing for presentation
            let i = i + 1;
            writeln!(f, "{:3}: {}", i, source)?;
        }

        Ok(())
    }

    #[cfg(feature = "std")]
    fn cleaned_error_trace(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use alloc::vec::Vec;

        const NOTE: char = '*';

        let mut any_cleaned = false;
        let mut any_removed = false;
        let cleaned_messages: Vec<_> = CleanedErrorText::new(self.0)
            .flat_map(|(_, mut msg, cleaned)| {
                if msg.is_empty() {
                    any_removed = true;
                    None
                } else {
                    if cleaned {
                        any_cleaned = true;
                        msg.push(' ');
                        msg.push(NOTE);
                    }
                    Some(msg)
                }
            })
            .collect();

        let mut visible_messages = cleaned_messages.iter();

        let head = match visible_messages.next() {
            Some(v) => v,
            None => return Ok(()),
        };

        writeln!(f, "{}", head)?;

        match cleaned_messages.len() {
            0 | 1 => {}
            2 => writeln!(f, "\nCaused by this error:")?,
            _ => writeln!(f, "\nCaused by these errors (recent errors listed first):")?,
        }

        for (i, msg) in visible_messages.enumerate() {
            // Let's use 1-based indexing for presentation
            let i = i + 1;
            writeln!(f, "{:3}: {}", i, msg)?;
        }

        if any_cleaned || any_removed {
            write!(f, "\nNOTE: ")?;

            if any_cleaned {
                write!(
                    f,
                    "Some redundant information has been removed from the lines marked with {}. ",
                    NOTE,
                )?;
            } else {
                write!(f, "Some redundant information has been removed. ")?;
            }

            writeln!(
                f,
                "Set {}=1 to disable this behavior.",
                SNAFU_RAW_ERROR_MESSAGES,
            )?;
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
const SNAFU_RAW_ERROR_MESSAGES: &str = "SNAFU_RAW_ERROR_MESSAGES";

#[cfg(feature = "std")]
fn trace_cleaning_enabled() -> bool {
    use crate::once_bool::OnceBool;
    use std::env;

    static DISABLED: OnceBool = OnceBool::new();
    !DISABLED.get(|| env::var_os(SNAFU_RAW_ERROR_MESSAGES).map_or(false, |v| v == "1"))
}

/// An iterator over an Error and its sources that removes duplicated
/// text from the error display strings.
///
/// It's common for errors with a `source` to have a `Display`
/// implementation that includes their source text as well:
///
/// ```text
/// Outer error text: Middle error text: Inner error text
/// ```
///
/// This works for smaller errors without much detail, but can be
/// annoying when trying to format the error in a more structured way,
/// such as line-by-line:
///
/// ```text
/// 1. Outer error text: Middle error text: Inner error text
/// 2. Middle error text: Inner error text
/// 3. Inner error text
/// ```
///
/// This iterator compares each pair of errors in the source chain,
/// removing the source error's text from the containing error's text:
///
/// ```text
/// 1. Outer error text
/// 2. Middle error text
/// 3. Inner error text
/// ```
#[cfg(feature = "alloc")]
pub struct CleanedErrorText<'a>(Option<CleanedErrorTextStep<'a>>);

#[cfg(feature = "alloc")]
impl<'a> CleanedErrorText<'a> {
    /// Constructs the iterator.
    pub fn new(error: &'a dyn crate::Error) -> Self {
        Self(Some(CleanedErrorTextStep::new(error)))
    }
}

#[cfg(feature = "alloc")]
impl<'a> Iterator for CleanedErrorText<'a> {
    /// The original error, the display string and if it has been cleaned
    type Item = (&'a dyn crate::Error, String, bool);

    fn next(&mut self) -> Option<Self::Item> {
        use core::mem;

        let mut step = self.0.take()?;
        let mut error_text = mem::take(&mut step.error_text);

        match step.error.source() {
            Some(next_error) => {
                let next_error_text = next_error.to_string();

                let cleaned_text = error_text
                    .trim_end_matches(&next_error_text)
                    .trim_end()
                    .trim_end_matches(':');
                let cleaned = cleaned_text.len() != error_text.len();
                let cleaned_len = cleaned_text.len();
                error_text.truncate(cleaned_len);

                self.0 = Some(CleanedErrorTextStep {
                    error: next_error,
                    error_text: next_error_text,
                });

                Some((step.error, error_text, cleaned))
            }
            None => Some((step.error, error_text, false)),
        }
    }
}

#[cfg(feature = "alloc")]
struct CleanedErrorTextStep<'a> {
    error: &'a dyn crate::Error,
    error_text: String,
}

#[cfg(feature = "alloc")]
impl<'a> CleanedErrorTextStep<'a> {
    fn new(error: &'a dyn crate::Error) -> Self {
        let error_text = error.to_string();
        Self { error, error_text }
    }
}

#[doc(hidden)]
pub trait __InternalExtractErrorType {
    type Err;
}

impl<T, E> __InternalExtractErrorType for core::result::Result<T, E> {
    type Err = E;
}
