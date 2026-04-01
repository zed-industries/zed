use derive_more::{Deref, DerefMut};
use gpui_util::arc_cow::ArcCow;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, fmt, sync::Arc};

/// A shared immutable string, backed by either a `&'static str` or an `Arc<str>`.
///
/// This mirrors `gpui::SharedString` but lives outside the `gpui` crate so that
/// lower-level crates can use it without pulling in the full UI framework.
#[derive(Deref, DerefMut, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub struct SharedString(ArcCow<'static, str>);

impl SharedString {
    pub const fn new_static(s: &'static str) -> Self {
        Self(ArcCow::Borrowed(s))
    }

    pub fn new(s: impl Into<Arc<str>>) -> Self {
        Self(ArcCow::Owned(s.into()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SharedString {
    fn default() -> Self {
        Self(ArcCow::Owned(Arc::default()))
    }
}

impl AsRef<str> for SharedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for SharedString {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl fmt::Debug for SharedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for SharedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_ref())
    }
}

impl PartialEq<String> for SharedString {
    fn eq(&self, other: &String) -> bool {
        self.as_ref() == other
    }
}

impl PartialEq<SharedString> for String {
    fn eq(&self, other: &SharedString) -> bool {
        self == other.as_ref()
    }
}

impl PartialEq<str> for SharedString {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

impl<'a> PartialEq<&'a str> for SharedString {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl From<&SharedString> for SharedString {
    fn from(value: &SharedString) -> Self {
        value.clone()
    }
}

impl From<SharedString> for Arc<str> {
    fn from(val: SharedString) -> Self {
        match val.0 {
            ArcCow::Borrowed(borrowed) => Arc::from(borrowed),
            ArcCow::Owned(owned) => owned,
        }
    }
}

impl<T: Into<ArcCow<'static, str>>> From<T> for SharedString {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl From<SharedString> for String {
    fn from(val: SharedString) -> Self {
        val.0.to_string()
    }
}

impl Serialize for SharedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for SharedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SharedString::from(s))
    }
}
