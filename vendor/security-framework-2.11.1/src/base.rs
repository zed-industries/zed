//! Support types for other modules.

use core_foundation::string::CFString;
use core_foundation_sys::base::OSStatus;
use std::error;
use std::fmt;
use std::num::NonZeroI32;
use std::result;

/// A `Result` type commonly returned by functions.
pub type Result<T, E = Error> = result::Result<T, E>;

/// A Security Framework error.
#[derive(Copy, Clone)]
pub struct Error(NonZeroI32);

impl fmt::Debug for Error {
    #[cold]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = fmt.debug_struct("Error");
        builder.field("code", &self.0);
        if let Some(message) = self.message() {
            builder.field("message", &message);
        }
        builder.finish()
    }
}

impl Error {
    /// Creates a new `Error` from a status code.
    /// The code must not be zero
    #[inline]
    #[must_use]
    pub fn from_code(code: OSStatus) -> Self {
        Self(NonZeroI32::new(code).unwrap_or_else(|| NonZeroI32::new(1).unwrap()))
    }

    /// Returns a string describing the current error, if available.
    #[inline(always)]
    #[must_use]
    pub fn message(self) -> Option<String> {
        self.inner_message()
    }

    #[cold]
    fn inner_message(self) -> Option<String> {
        use core_foundation::base::TCFType;
        use security_framework_sys::base::SecCopyErrorMessageString;
        use std::ptr;

        unsafe {
            let s = SecCopyErrorMessageString(self.code(), ptr::null_mut());
            if s.is_null() {
                None
            } else {
                Some(CFString::wrap_under_create_rule(s).to_string())
            }
        }
    }

    /// Returns the code of the current error.
    #[inline(always)]
    #[must_use]
    pub fn code(self) -> OSStatus {
        self.0.get() as _
    }
}

impl From<OSStatus> for Error {
    #[inline(always)]
    #[must_use]
    fn from(code: OSStatus) -> Self {
        Self::from_code(code)
    }
}

impl fmt::Display for Error {
    #[cold]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(message) = self.message() {
            write!(fmt, "{message}")
        } else {
            write!(fmt, "error code {}", self.code())
        }
    }
}

impl error::Error for Error {}
