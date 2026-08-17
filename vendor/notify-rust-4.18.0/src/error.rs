#![allow(missing_docs)]

#[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
use crate::image::ImageError;
use std::{fmt, num};
/// Convenient wrapper around `std::Result`.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(all(target_os = "macos", not(feature = "preview-macos-un")))]
pub use crate::macos::{ApplicationError, MacOsError, NotificationError};

#[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
pub use crate::macos::MacOsError;

/// The Error type.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

/// The kind of an error used by [`Error`].
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Only here for backwards compatibility.
    Msg(String),

    #[cfg(all(feature = "dbus", unix, not(target_os = "macos")))]
    Dbus(dbus::Error),

    #[cfg(all(feature = "zbus", unix, not(target_os = "macos")))]
    Zbus(zbus::Error),

    #[cfg(target_os = "macos")]
    MacNotificationSys(mac_notification_sys::error::Error),

    #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
    MacUserNotifications(mac_usernotifications::Error),

    Parse(num::ParseIntError),

    SpecVersion(String),

    Conversion(String),

    #[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
    Image(ImageError),

    ImplementationMissing,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            #[cfg(all(feature = "dbus", unix, not(target_os = "macos")))]
            ErrorKind::Dbus(ref e) => write!(f, "{}", e),

            #[cfg(all(feature = "zbus", unix, not(target_os = "macos")))]
            ErrorKind::Zbus(ref e) => write!(f, "{}", e),

            #[cfg(target_os = "macos")]
            ErrorKind::MacNotificationSys(ref e) => write!(f, "{e}"),

            #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
            ErrorKind::MacUserNotifications(ref e) => write!(f, "{e}"),

            ErrorKind::Parse(ref e) => write!(f, "Parsing Error: {e}"),
            ErrorKind::Conversion(ref e) => write!(f, "Conversion Error: {e}"),
            ErrorKind::SpecVersion(ref e) | ErrorKind::Msg(ref e) => write!(f, "{e}"),
            #[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
            ErrorKind::Image(ref e) => write!(f, "{}", e),
            ErrorKind::ImplementationMissing => write!(
                f,
                r#"No Dbus implementation available, please compile with either feature ="z" or feature="d""#
            ),
        }
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(e: &str) -> Error {
        Error {
            kind: ErrorKind::Msg(e.into()),
        }
    }
}

#[cfg(all(feature = "dbus", unix, not(target_os = "macos")))]
impl From<dbus::Error> for Error {
    fn from(e: dbus::Error) -> Error {
        Error {
            kind: ErrorKind::Dbus(e),
        }
    }
}

#[cfg(all(feature = "zbus", unix, not(target_os = "macos")))]
impl From<zbus::Error> for Error {
    fn from(e: zbus::Error) -> Error {
        Error {
            kind: ErrorKind::Zbus(e),
        }
    }
}

#[cfg(target_os = "macos")]
impl From<mac_notification_sys::error::Error> for Error {
    fn from(e: mac_notification_sys::error::Error) -> Error {
        Error {
            kind: ErrorKind::MacNotificationSys(e),
        }
    }
}

#[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
impl From<ImageError> for Error {
    fn from(e: ImageError) -> Error {
        Error {
            kind: ErrorKind::Image(e),
        }
    }
}

#[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
impl From<mac_usernotifications::Error> for Error {
    fn from(e: mac_usernotifications::Error) -> Error {
        Error {
            kind: ErrorKind::MacUserNotifications(e),
        }
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Error {
        Error {
            kind: ErrorKind::Parse(e),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

/// Just the usual bail macro
#[macro_export]
#[doc(hidden)]
macro_rules! bail {
    ($e:expr) => {
        return Err($e.into());
    };
    ($fmt:expr, $($arg:tt)+) => {
        return Err(format!($fmt, $($arg)+).into());
    };
}

/// Exits a function early with an `Error` if the condition is not satisfied.
///
/// Similar to `assert!`, `ensure!` takes a condition and exits the function
/// if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`,
/// it does not panic.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            bail!($e);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !($cond) {
            bail!($fmt, $($arg)*);
        }
    };
}
