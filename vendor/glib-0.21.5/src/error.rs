// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! `Error` binding and helper trait.

use std::{borrow::Cow, convert::Infallible, error, ffi::CStr, fmt, str};

use crate::{ffi, translate::*, Quark};

wrapper! {
    // rustdoc-stripper-ignore-next
    /// A generic error capable of representing various error domains (types).
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GError")]
    pub struct Error(Boxed<ffi::GError>);

    match fn {
        copy => |ptr| ffi::g_error_copy(ptr),
        free => |ptr| ffi::g_error_free(ptr),
        type_ => || ffi::g_error_get_type(),
    }
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl Error {
    // rustdoc-stripper-ignore-next
    /// Creates an error with supplied error enum variant and message.
    #[doc(alias = "g_error_new_literal")]
    #[doc(alias = "g_error_new")]
    pub fn new<T: ErrorDomain>(error: T, message: &str) -> Error {
        unsafe {
            from_glib_full(ffi::g_error_new_literal(
                T::domain().into_glib(),
                error.code(),
                message.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates an error with supplied error domain quark, code and message.
    ///
    /// This is useful when you need to create an error with the same domain and code
    /// as an existing error but with a different message.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let original = Error::new(FileError::Failed, "Original message");
    /// let modified = Error::with_domain(
    ///     original.domain(),
    ///     original.code(),
    ///     "Modified message"
    /// );
    /// ```
    #[doc(alias = "g_error_new_literal")]
    pub fn with_domain(domain: Quark, code: i32, message: &str) -> Error {
        unsafe {
            from_glib_full(ffi::g_error_new_literal(
                domain.into_glib(),
                code,
                message.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Checks if the error domain matches `T`.
    pub fn is<T: ErrorDomain>(&self) -> bool {
        self.inner.domain == T::domain().into_glib()
    }

    // rustdoc-stripper-ignore-next
    /// Returns the error domain quark
    pub fn domain(&self) -> Quark {
        unsafe { from_glib(self.inner.domain) }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the error code
    pub fn code(&self) -> i32 {
        self.inner.code
    }

    // rustdoc-stripper-ignore-next
    /// Checks if the error matches the specified domain and error code.
    #[doc(alias = "g_error_matches")]
    pub fn matches<T: ErrorDomain>(&self, err: T) -> bool {
        self.is::<T>() && self.inner.code == err.code()
    }

    // rustdoc-stripper-ignore-next
    /// Tries to convert to a specific error enum.
    ///
    /// Returns `Some` if the error belongs to the enum's error domain and
    /// `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// if let Some(file_error) = error.kind::<FileError>() {
    ///     match file_error {
    ///         FileError::Exist => ...
    ///         FileError::Isdir => ...
    ///         ...
    ///     }
    /// }
    /// ```
    pub fn kind<T: ErrorDomain>(&self) -> Option<T> {
        if self.is::<T>() {
            T::from(self.inner.code)
        } else {
            None
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the error message
    ///
    /// Most of the time you can simply print the error since it implements the `Display`
    /// trait, but you can use this method if you need to have the message as a `&str`.
    pub fn message(&self) -> &str {
        unsafe {
            let bytes = CStr::from_ptr(self.inner.message).to_bytes();
            str::from_utf8(bytes)
                .unwrap_or_else(|err| str::from_utf8(&bytes[..err.valid_up_to()]).unwrap())
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("domain", unsafe {
                &crate::Quark::from_glib(self.inner.domain)
            })
            .field("code", &self.inner.code)
            .field("message", &self.message())
            .finish()
    }
}

impl error::Error for Error {}

impl From<Infallible> for Error {
    fn from(e: Infallible) -> Self {
        match e {}
    }
}

// rustdoc-stripper-ignore-next
/// `GLib` error domain.
///
/// This trait is implemented by error enums that represent error domains (types).
pub trait ErrorDomain: Copy {
    // rustdoc-stripper-ignore-next
    /// Returns the quark identifying the error domain.
    ///
    /// As returned from `g_some_error_quark`.
    fn domain() -> Quark;

    // rustdoc-stripper-ignore-next
    /// Gets the integer representation of the variant.
    fn code(self) -> i32;

    // rustdoc-stripper-ignore-next
    /// Tries to convert an integer code to an enum variant.
    ///
    /// By convention, the `Failed` variant, if present, is a catch-all,
    /// i.e. any unrecognized codes map to it.
    fn from(code: i32) -> Option<Self>
    where
        Self: Sized;
}

// rustdoc-stripper-ignore-next
/// Generic error used for functions that fail without any further information
#[macro_export]
macro_rules! bool_error(
    ($($msg:tt)*) =>  {{
        match ::std::format_args!($($msg)*) {
            formatted => {
                if let Some(s) = formatted.as_str() {
                    $crate::BoolError::new(
                        s,
                        file!(),
                        $crate::function_name!(),
                        line!()
                    )
                } else {
                    $crate::BoolError::new(
                        formatted.to_string(),
                        file!(),
                        $crate::function_name!(),
                        line!(),
                    )
                }
            }
        }
    }};
);

#[macro_export]
macro_rules! result_from_gboolean(
    ($ffi_bool:expr, $($msg:tt)*) =>  {{
        match ::std::format_args!($($msg)*) {
            formatted => {
                if let Some(s) = formatted.as_str() {
                    $crate::BoolError::from_glib(
                        $ffi_bool,
                        s,
                        file!(),
                        $crate::function_name!(),
                        line!(),
                    )
                } else {
                    $crate::BoolError::from_glib(
                        $ffi_bool,
                        formatted.to_string(),
                        file!(),
                        $crate::function_name!(),
                        line!(),
                    )
                }
            }
        }


    }};
);

#[derive(Debug, Clone)]
pub struct BoolError {
    pub message: Cow<'static, str>,
    #[doc(hidden)]
    pub filename: &'static str,
    #[doc(hidden)]
    pub function: &'static str,
    #[doc(hidden)]
    pub line: u32,
}

impl BoolError {
    pub fn new(
        message: impl Into<Cow<'static, str>>,
        filename: &'static str,
        function: &'static str,
        line: u32,
    ) -> Self {
        Self {
            message: message.into(),
            filename,
            function,
            line,
        }
    }

    pub fn from_glib(
        b: ffi::gboolean,
        message: impl Into<Cow<'static, str>>,
        filename: &'static str,
        function: &'static str,
        line: u32,
    ) -> Result<(), Self> {
        match b {
            ffi::GFALSE => Err(BoolError::new(message, filename, function, line)),
            _ => Ok(()),
        }
    }
}

impl fmt::Display for BoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl error::Error for BoolError {}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_error_matches() {
        let e = Error::new(crate::FileError::Failed, "Failed");
        assert!(e.matches(crate::FileError::Failed));
        assert!(!e.matches(crate::FileError::Again));
        assert!(!e.matches(crate::KeyFileError::NotFound));
    }

    #[test]
    fn test_error_kind() {
        let e = Error::new(crate::FileError::Failed, "Failed");
        assert_eq!(e.kind::<crate::FileError>(), Some(crate::FileError::Failed));
        assert_eq!(e.kind::<crate::KeyFileError>(), None);
    }

    #[test]
    fn test_into_raw() {
        unsafe {
            let e: *mut ffi::GError =
                Error::new(crate::FileError::Failed, "Failed").into_glib_ptr();
            assert_eq!((*e).domain, ffi::g_file_error_quark());
            assert_eq!((*e).code, ffi::G_FILE_ERROR_FAILED);
            assert_eq!(
                CStr::from_ptr((*e).message),
                CString::new("Failed").unwrap().as_c_str()
            );

            ffi::g_error_free(e);
        }
    }

    #[test]
    fn test_bool_error() {
        let from_static_msg = bool_error!("Static message");
        assert_eq!(from_static_msg.to_string(), "Static message");

        let from_dynamic_msg = bool_error!("{} message", "Dynamic");
        assert_eq!(from_dynamic_msg.to_string(), "Dynamic message");

        let false_static_res = result_from_gboolean!(ffi::GFALSE, "Static message");
        assert!(false_static_res.is_err());
        let static_err = false_static_res.err().unwrap();
        assert_eq!(static_err.to_string(), "Static message");

        let true_static_res = result_from_gboolean!(ffi::GTRUE, "Static message");
        assert!(true_static_res.is_ok());

        let false_dynamic_res = result_from_gboolean!(ffi::GFALSE, "{} message", "Dynamic");
        assert!(false_dynamic_res.is_err());
        let dynamic_err = false_dynamic_res.err().unwrap();
        assert_eq!(dynamic_err.to_string(), "Dynamic message");

        let true_dynamic_res = result_from_gboolean!(ffi::GTRUE, "{} message", "Dynamic");
        assert!(true_dynamic_res.is_ok());
    }

    #[test]
    fn test_value() {
        let e1 = Error::new(crate::FileError::Failed, "Failed");
        // This creates a copy ...
        let v = e1.to_value();
        // ... so we have to get the raw pointer from inside the value to check for equality.
        let ptr = unsafe {
            crate::gobject_ffi::g_value_get_boxed(v.to_glib_none().0) as *const ffi::GError
        };

        let e2 = v.get::<&Error>().unwrap();

        assert_eq!(ptr, e2.to_glib_none().0);
    }

    #[test]
    fn test_from_quark() {
        let original = Error::new(crate::FileError::Failed, "Original message");
        let modified = Error::with_domain(original.domain(), original.code(), "Modified message");

        // Should have same domain and code
        assert_eq!(original.domain(), modified.domain());
        assert_eq!(original.code(), modified.code());
        assert!(modified.matches(crate::FileError::Failed));

        // But different message
        assert_eq!(modified.message(), "Modified message");
        assert_ne!(original.message(), modified.message());
    }
}
