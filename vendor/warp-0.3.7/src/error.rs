use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// Errors that can happen inside warp.
pub struct Error {
    inner: BoxError,
}

impl Error {
    pub(crate) fn new<E: Into<BoxError>>(err: E) -> Error {
        Error { inner: err.into() }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Skip showing worthless `Error { .. }` wrapper.
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.inner.as_ref())
    }
}

impl From<Infallible> for Error {
    fn from(infallible: Infallible) -> Error {
        match infallible {}
    }
}

#[test]
fn error_size_of() {
    assert_eq!(
        ::std::mem::size_of::<Error>(),
        ::std::mem::size_of::<usize>() * 2
    );
}

#[test]
fn error_source() {
    let e = Error::new(std::fmt::Error {});
    assert!(e.source().unwrap().is::<std::fmt::Error>());
}

macro_rules! unit_error {
    (
        $(#[$docs:meta])*
        $pub:vis $typ:ident: $display:literal
    ) => (
        $(#[$docs])*
        $pub struct $typ { _p: (), }

        impl ::std::fmt::Debug for $typ {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct(stringify!($typ)).finish()
            }
        }

        impl ::std::fmt::Display for $typ {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str($display)
            }
        }

        impl ::std::error::Error for $typ {}
    )
}
