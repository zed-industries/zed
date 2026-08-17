use core::fmt;

#[cfg(not(feature = "std"))]
use arrayvec::ArrayString;

#[cfg(not(feature = "std"))]
use crate::strings::MAX_ERR_LEN;

/// This crate's error kind.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[allow(missing_copy_implementations)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum ErrorKind {
    /// Input exceeds buffer capacity.
    Capacity {
        /// Length of the input in bytes.
        len: usize,
        /// Capacity of the buffer in bytes.
        cap: usize,
    },

    #[cfg(all(feature = "with-system-locale", any(unix, windows)))]
    /// Locale name contains an interior nul byte, which is not allowed.
    InteriorNulByte(String),

    #[cfg(feature = "std")]
    /// Other miscellaneous error.
    Other(String),

    #[cfg(not(feature = "std"))]
    /// Other miscellaneous error.
    Other(ArrayString<MAX_ERR_LEN>),

    #[cfg(feature = "std")]
    /// Failed to parse input into a valid locale.
    ParseLocale(String),

    #[cfg(not(feature = "std"))]
    /// Failed to parse input into a valid locale.
    ParseLocale(ArrayString<MAX_ERR_LEN>),

    #[cfg(feature = "std")]
    /// Failed to parse input into a number.
    ParseNumber(String),

    #[cfg(not(feature = "std"))]
    /// Failed to parse input into a number.
    ParseNumber(ArrayString<MAX_ERR_LEN>),

    #[cfg(all(feature = "with-system-locale", any(unix, windows)))]
    /// Call to C standard library or Windows API unexpectedly returned invalid data.
    SystemInvalidReturn {
        /// The name of the C standard library or Windows API function called.
        function_name: String,
        /// Details about the invalid data returned.
        message: String,
    },

    #[cfg(all(feature = "with-system-locale", unix))]
    /// Attempted to use a system locale that relies on an encoding that is not currently supported
    /// by num-format.
    SystemUnsupportedEncoding(String),

    #[cfg(all(feature = "with-system-locale", any(unix, windows)))]
    /// The operating system returned grouping data that is currently unsuppported by num-format.
    SystemUnsupportedGrouping(Vec<u8>),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ErrorKind::*;
        match self {
            Capacity { len, cap } => write!(
                f,
                "Attempted to write input of length {} bytes into a buffer with \
                 capacity {} bytes.",
                len, cap
            ),

            #[cfg(all(feature = "with-system-locale", any(unix, windows)))]
            InteriorNulByte(ref locale_name) => write!(
                f,
                "Locale name {} contains an interior nul byte, which is not allowed.",
                locale_name
            ),

            Other(ref message) => write!(f, "{}", message),

            ParseLocale(ref input) => write!(f, "Failed to parse {} into a valid locale.", input),

            ParseNumber(ref input) => write!(f, "Failed to parse {} into a number.", input),

            #[cfg(all(feature = "with-system-locale", any(unix, windows)))]
            SystemInvalidReturn { message, .. } => write!(f, "{}", message),

            #[cfg(all(feature = "with-system-locale", unix))]
            SystemUnsupportedEncoding(ref encoding_name) => write!(
                f,
                "Attempted to use a system locale that relies on an encoding that is not \
                 currently supported by num-format. The unsupported encoding is {}.",
                encoding_name
            ),

            #[cfg(all(feature = "with-system-locale", any(unix, windows)))]
            SystemUnsupportedGrouping(ref bytes) => write!(
                f,
                "The operating system returned grouping data of {:?}, which is not currently \
                 suppported by num-format.",
                bytes
            ),
        }
    }
}
