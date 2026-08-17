use core::fmt;

/**
An error encountered buffering data.
*/
#[derive(Debug)]
pub struct Error(ErrorKind);

#[derive(Debug)]
enum ErrorKind {
    Buffer(sval_buffer::Error),
    InvalidValue {
        reason: &'static str,
    },
    #[cfg(not(feature = "alloc"))]
    #[allow(dead_code)]
    NoAlloc {
        method: &'static str,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ErrorKind::Buffer(ref _e) => {
                write!(f, "failed to buffer a value")
            }
            ErrorKind::InvalidValue { reason } => {
                write!(f, "the value is invalid: {}", reason)
            }
            #[cfg(not(feature = "alloc"))]
            ErrorKind::NoAlloc { method } => write!(f, "cannot allocate for {}", method),
        }
    }
}

impl Error {
    pub(crate) fn buffer(err: sval_buffer::Error) -> Self {
        Error(ErrorKind::Buffer(err))
    }

    /**
    The given value is invalid.
    */
    pub fn invalid_value(reason: &'static str) -> Self {
        Error(ErrorKind::InvalidValue { reason })
    }

    #[cfg(not(feature = "alloc"))]
    #[track_caller]
    pub(crate) fn no_alloc(method: &'static str) -> Self {
        /*
        The pattern here is the same as what's used in `sval_buffer`.
        */

        #[cfg(all(debug_assertions, not(no_debug_assertions), not(test)))]
        {
            panic!("attempt to allocate for {} would fail; add the `alloc` feature of `sval_nested` or the depdendent `sval_*` library to support allocation. This call will error instead of panicking in release builds. Add the `no_debug_assertions` feature of `sval_nested` if this error is expected.", method);
        }
        #[cfg(not(all(debug_assertions, not(no_debug_assertions), not(test))))]
        {
            Error(ErrorKind::NoAlloc { method })
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use std::error;

    impl error::Error for Error {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match self.0 {
                ErrorKind::Buffer(ref err) => Some(err),
                ErrorKind::InvalidValue { .. } => None,
                #[cfg(not(feature = "alloc"))]
                ErrorKind::NoAlloc { .. } => None,
            }
        }
    }
}
