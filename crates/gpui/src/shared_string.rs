use derive_more::{Deref, DerefMut};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, sync::Arc};
use util::arc_cow::ArcCow;

/// A shared string is an immutable string that can be cheaply cloned in GPUI
/// tasks. Essentially an abstraction over an `Arc<str>` and `&'static str`,
#[derive(Deref, DerefMut, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub struct SharedString(ArcCow<'static, str>);

impl SharedString {
    /// Creates a static [`SharedString`] from a `&'static str`.
    pub const fn new_static(str: &'static str) -> Self {
        Self(ArcCow::Borrowed(str))
    }

    /// Creates a [`SharedString`] from anything that can become an `Arc<str>`
    pub fn new(str: impl Into<Arc<str>>) -> Self {
        SharedString(ArcCow::Owned(str.into()))
    }
}

impl JsonSchema for SharedString {
    fn schema_name() -> String {
        String::schema_name()
    }

    fn json_schema(r#gen: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(r#gen)
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

impl std::fmt::Debug for SharedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for SharedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            ArcCow::Owned(owned) => owned.clone(),
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
