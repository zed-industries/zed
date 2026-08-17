use std::{num::ParseIntError, str::FromStr, time::Duration};

/// Describes the timeout of a notification
///
/// # `FromStr`
/// You can also parse a `Timeout` from a `&str`.
/// ```
/// # use notify_rust::Timeout;
/// assert_eq!("default".parse(), Ok(Timeout::Default));
/// assert_eq!("never".parse(), Ok(Timeout::Never));
/// assert_eq!("42".parse(), Ok(Timeout::Milliseconds(42)));
/// ```
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Timeout {
    /// Expires according to server default.
    ///
    /// Whatever that might be...
    #[default]
    Default,

    /// Do not expire, user will have to close this manually.
    Never,

    /// Expire after n milliseconds.
    Milliseconds(u32),
}

#[test]
fn timeout_from_i32() {
    assert_eq!(Timeout::from(234), Timeout::Milliseconds(234));
    assert_eq!(Timeout::from(-234), Timeout::Default);
    assert_eq!(Timeout::from(0), Timeout::Never);
}

impl From<i32> for Timeout {
    fn from(int: i32) -> Timeout {
        use std::cmp::Ordering::*;
        match int.cmp(&0) {
            Greater => Timeout::Milliseconds(int as u32),
            Less => Timeout::Default,
            Equal => Timeout::Never,
        }
    }
}

impl From<Duration> for Timeout {
    fn from(duration: Duration) -> Timeout {
        if duration.is_zero() {
            Timeout::Never
        } else if duration.as_millis() > u32::MAX as u128 {
            Timeout::Default
        } else {
            Timeout::Milliseconds(duration.as_millis().try_into().unwrap_or(u32::MAX))
        }
    }
}

impl From<Timeout> for i32 {
    fn from(timeout: Timeout) -> Self {
        match timeout {
            Timeout::Default => -1,
            Timeout::Never => 0,
            Timeout::Milliseconds(ms) => ms as i32,
        }
    }
}

impl FromStr for Timeout {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Timeout::Default),
            "never" => Ok(Timeout::Never),
            milliseconds => Ok(Timeout::Milliseconds(u32::from_str(milliseconds)?)),
        }
    }
}

#[cfg(all(feature = "dbus", unix, not(target_os = "macos")))]
pub struct TimeoutMessage(Timeout);

#[cfg(all(feature = "dbus", unix, not(target_os = "macos")))]
impl From<Timeout> for TimeoutMessage {
    fn from(hint: Timeout) -> Self {
        TimeoutMessage(hint)
    }
}

#[cfg(all(feature = "dbus", unix, not(target_os = "macos")))]
impl std::ops::Deref for TimeoutMessage {
    type Target = Timeout;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(all(feature = "dbus", unix, not(target_os = "macos")))]
impl TryFrom<&dbus::arg::messageitem::MessageItem> for TimeoutMessage {
    type Error = ();

    fn try_from(mi: &dbus::arg::messageitem::MessageItem) -> Result<TimeoutMessage, ()> {
        mi.inner::<i32>().map(|i| TimeoutMessage(i.into()))
    }
}
