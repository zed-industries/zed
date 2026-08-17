use std::ops::Deref;

use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[cfg(any(feature = "gtk4_wayland", feature = "gtk4_x11"))]
mod gtk4;

#[cfg(feature = "wayland")]
mod wayland;

/// A token that can be used to activate an application.
///
/// No guarantees are made for the token structure.
#[derive(Debug, Deserialize, Serialize, Type, PartialEq, Eq, Hash, Clone)]
pub struct ActivationToken(String);

impl From<String> for ActivationToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ActivationToken {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<ActivationToken> for String {
    fn from(value: ActivationToken) -> String {
        value.0
    }
}

impl Deref for ActivationToken {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl AsRef<str> for ActivationToken {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for ActivationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
