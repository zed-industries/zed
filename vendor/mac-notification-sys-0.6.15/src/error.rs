//! Custom errors for mac-notification-sys.

use std::error;
use std::fmt;

/// Custom Result type for mac-notification-sys.
pub type NotificationResult<T> = Result<T, Error>;

mod application {
    use super::*;
    /// Errors that can occur setting the Bundle Identifier.
    #[derive(Debug)]
    pub enum ApplicationError {
        /// The application name is already set.
        AlreadySet(String),

        /// The application name could not be set.
        CouldNotSet(String),
    }

    impl fmt::Display for ApplicationError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ApplicationError::AlreadySet(e) => {
                    write!(f, "Application '{e}' can only be set once.")
                }
                ApplicationError::CouldNotSet(e) => write!(
                    f,
                    "Could not set application '{e}', using default \"com.apple.Terminal\"",
                ),
            }
        }
    }

    impl error::Error for ApplicationError {}
}

mod notification {
    use super::*;

    /// Errors that can occur while interacting with the NSUserNotificationCenter.
    #[derive(Debug)]
    pub enum NotificationError {
        /// Notifications can not be scheduled in the past.
        ScheduleInThePast,

        /// Scheduling a notification caused an error.
        UnableToSchedule,

        /// Delivering a notification caused an error.
        UnableToDeliver,
    }
    impl fmt::Display for NotificationError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                NotificationError::ScheduleInThePast => {
                    write!(f, "Can not schedule notification in the past")
                }
                NotificationError::UnableToSchedule => write!(f, "Could not schedule notification"),
                NotificationError::UnableToDeliver => write!(f, "Could not deliver notification"),
            }
        }
    }

    impl error::Error for NotificationError {}
}

pub use self::application::ApplicationError;
pub use self::notification::NotificationError;

/// Our local error Type
#[derive(Debug)]
pub enum Error {
    /// Application related Error
    Application(ApplicationError),
    /// Notification related Error
    Notification(NotificationError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Application(e) => write!(f, "{e}"),
            Error::Notification(e) => write!(f, "{e}"),
        }
    }
}

impl error::Error for Error {}

impl From<ApplicationError> for Error {
    fn from(e: ApplicationError) -> Error {
        Error::Application(e)
    }
}

impl From<NotificationError> for Error {
    fn from(e: NotificationError) -> Error {
        Error::Notification(e)
    }
}

/// Just the usual bail macro
#[macro_export]
#[doc(hidden)]
macro_rules! bail {
    ($e:expr_2021) => {
        return Err($e.into());
    };
    ($fmt:expr_2021, $($arg:tt)+) => {
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
    ($cond:expr_2021, $e:expr_2021) => {
        if ($cond) != true {
            bail!($e);
        }
    };
    ($cond:expr_2021, $fmt:expr_2021, $($arg:tt)*) => {
        if !($cond) {
            bail!($fmt, $($arg)*);
        }
    };
}
