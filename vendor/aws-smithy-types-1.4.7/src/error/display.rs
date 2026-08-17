/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Error wrapper that displays error context

use std::error::Error;
use std::fmt;

/// Provides a `Display` impl for an `Error` that outputs the full error context
///
/// This utility follows the error cause/source chain and displays every error message
/// in the chain separated by ": ". At the end of the chain, it outputs a debug view
/// of the entire error chain.
///
/// # Example
///
/// ```no_run
/// # let err: &dyn std::error::Error = unimplemented!();
/// # use aws_smithy_types::error::display::DisplayErrorContext;
/// println!("There was an unhandled error: {}", DisplayErrorContext(&err));
/// ```
///
// Internally in the SDK, this is useful for emitting errors with `tracing` in cases
// where the error is not returned back to the customer.
#[derive(Debug)]
pub struct DisplayErrorContext<E: Error>(
    /// The error to display full context for
    pub E,
);

impl<E: Error> fmt::Display for DisplayErrorContext<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_err(f, &self.0)?;
        // Also add a debug version of the error at the end
        write!(f, " ({:?})", self.0)
    }
}

fn write_err(f: &mut fmt::Formatter<'_>, err: &dyn Error) -> fmt::Result {
    write!(f, "{err}")?;
    if let Some(source) = err.source() {
        write!(f, ": ")?;
        write_err(f, source)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    struct TestError {
        what: &'static str,
        source: Option<Box<dyn Error>>,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.what)
        }
    }

    impl Error for TestError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.source.as_deref()
        }
    }

    #[test]
    fn no_sources() {
        assert_eq!(
            "test (TestError { what: \"test\", source: None })",
            format!(
                "{}",
                DisplayErrorContext(TestError {
                    what: "test",
                    source: None
                })
            )
        );
    }

    #[test]
    fn sources() {
        assert_eq!(
            "foo: bar: baz (TestError { what: \"foo\", source: Some(TestError { what: \"bar\", source: Some(TestError { what: \"baz\", source: None }) }) })",
            format!(
                "{}",
                DisplayErrorContext(TestError {
                    what: "foo",
                    source: Some(Box::new(TestError {
                        what: "bar",
                        source: Some(Box::new(TestError {
                            what: "baz",
                            source: None
                        }))
                    }) as Box<_>)
                })
            )
        );
    }
}
