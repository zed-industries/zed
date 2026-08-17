use std::{fmt, str::FromStr};

use crate::{Error, Result};

/// Authentication mechanisms
///
/// Note that the `DBUS_COOKIE_SHA1` mechanism is not supported by zbus since version 5.0. The
/// reasons are:
///
/// * It drags the `sha1` crate as a dependency, which can be [problematic for some users].
/// * It makes the handshake more complex, now allowing use to pipeline all the commands.
/// * It's not widely used. If `EXTERNAL` is not an option, you might as well just use `ANONYMOUS`.
///
/// See <https://dbus.freedesktop.org/doc/dbus-specification.html#auth-mechanisms>
///
/// [problematic for some users]: https://github.com/z-galaxy/zbus/issues/543
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthMechanism {
    /// This is the recommended authentication mechanism on platforms where credentials can be
    /// transferred out-of-band, in particular Unix platforms that can perform credentials-passing
    /// over the `unix:` transport.
    External,

    /// Does not perform any authentication at all, and should not be accepted by message buses.
    /// However, it might sometimes be useful for non-message-bus uses of D-Bus.
    Anonymous,
}

impl AuthMechanism {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuthMechanism::External => "EXTERNAL",
            AuthMechanism::Anonymous => "ANONYMOUS",
        }
    }
}

impl fmt::Display for AuthMechanism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mech = self.as_str();
        write!(f, "{mech}")
    }
}

impl FromStr for AuthMechanism {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "EXTERNAL" => Ok(AuthMechanism::External),
            "ANONYMOUS" => Ok(AuthMechanism::Anonymous),
            _ => Err(Error::Handshake(format!("Unsupported mechanism: {s}"))),
        }
    }
}
