use std::borrow::Borrow;

use gpui_shared_string::SharedString;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ManifestName(SharedString);

impl Borrow<SharedString> for ManifestName {
    fn borrow(&self) -> &SharedString {
        &self.0
    }
}

impl Borrow<str> for ManifestName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl From<SharedString> for ManifestName {
    fn from(value: SharedString) -> Self {
        Self(value)
    }
}

impl From<ManifestName> for SharedString {
    fn from(value: ManifestName) -> Self {
        value.0
    }
}

impl AsRef<SharedString> for ManifestName {
    fn as_ref(&self) -> &SharedString {
        &self.0
    }
}
