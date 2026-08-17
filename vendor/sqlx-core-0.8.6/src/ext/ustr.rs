use std::borrow::Borrow;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

// U meaning micro
// a micro-string is either a reference-counted string or a static string
// this guarantees these are cheap to clone everywhere
#[derive(Clone, Eq)]
pub enum UStr {
    Static(&'static str),
    Shared(Arc<str>),
}

impl UStr {
    pub fn new(s: &str) -> Self {
        UStr::Shared(Arc::from(s.to_owned()))
    }

    /// Apply [str::strip_prefix], without copying if possible.
    pub fn strip_prefix(this: &Self, prefix: &str) -> Option<Self> {
        match this {
            UStr::Static(s) => s.strip_prefix(prefix).map(Self::Static),
            UStr::Shared(s) => s.strip_prefix(prefix).map(|s| Self::Shared(s.into())),
        }
    }
}

impl Deref for UStr {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        match self {
            UStr::Static(s) => s,
            UStr::Shared(s) => s,
        }
    }
}

impl Hash for UStr {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Forward the hash to the string representation of this
        // A derive(Hash) encodes the enum discriminant
        (**self).hash(state);
    }
}

impl Borrow<str> for UStr {
    #[inline]
    fn borrow(&self) -> &str {
        self
    }
}

impl PartialEq<UStr> for UStr {
    fn eq(&self, other: &UStr) -> bool {
        (**self).eq(&**other)
    }
}

impl From<&'static str> for UStr {
    #[inline]
    fn from(s: &'static str) -> Self {
        UStr::Static(s)
    }
}

impl<'a> From<&'a UStr> for UStr {
    fn from(value: &'a UStr) -> Self {
        value.clone()
    }
}

impl From<String> for UStr {
    #[inline]
    fn from(s: String) -> Self {
        UStr::Shared(s.into())
    }
}

impl Debug for UStr {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self)
    }
}

impl Display for UStr {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self)
    }
}

// manual impls because otherwise things get a little screwy with lifetimes

#[cfg(feature = "offline")]
impl<'de> serde::Deserialize<'de> for UStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[cfg(feature = "offline")]
impl serde::Serialize for UStr {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self)
    }
}
