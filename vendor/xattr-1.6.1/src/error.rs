use std::error::Error;
use std::fmt;

/// The error type returned on unsupported platforms.
///
/// On unsupported platforms, all operations will fail with an `io::Error` with
/// a kind `io::ErrorKind::Unsupported` and an `UnsupportedPlatformError` error as the inner error.
/// While you *could* check the inner error, it's probably simpler just to check
/// `xattr::SUPPORTED_PLATFORM`.
///
/// This error mostly exists for pretty error messages.
#[derive(Copy, Clone, Debug)]
pub struct UnsupportedPlatformError;

impl Error for UnsupportedPlatformError {
    fn description(&self) -> &str {
        "unsupported platform"
    }
}

impl fmt::Display for UnsupportedPlatformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "unsupported platform, please file a bug at `https://github.com/Stebalien/xattr'"
        )
    }
}
