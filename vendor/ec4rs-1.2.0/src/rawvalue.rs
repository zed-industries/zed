//! Types and utilities related to unparsed EditorConfig values.

use crate::PropertyValue;

use std::borrow::Cow;

/// An unset `RawValue`.
///
/// Not all unset `&RawValues` returned by this library are referentially equal to this one.
/// This exists to provide an unset raw value for whenever a reference to one is necessary.
pub static UNSET: RawValue = RawValue {
    value: Cow::Borrowed(""),
    #[cfg(feature = "track-source")]
    source: None,
};

/// An unparsed property value.
///
/// This is conceptually an optional non-empty string with some convenience methods.
/// With the `track-source` feature,
/// objects of this type can also track the file and line number they originate from.
#[derive(Clone, Debug, Default)]
pub struct RawValue {
    value: Cow<'static, str>,
    #[cfg(feature = "track-source")]
    source: Option<(std::sync::Arc<std::path::Path>, usize)>,
}

// Manual-impl (Partial)Eq, (Partial)Ord, and Hash so that the source isn't considered.

impl PartialEq for RawValue {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for RawValue {}
impl PartialOrd for RawValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}
impl Ord for RawValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
impl std::hash::Hash for RawValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.value.as_bytes());
        state.write_u8(0);
    }
}

impl RawValue {
    #[must_use]
    fn detect_unset(&self) -> Option<bool> {
        if self.is_unset() {
            Some(false)
        } else if "unset".eq_ignore_ascii_case(self.value.as_ref()) {
            Some(true)
        } else {
            None
        }
    }

    #[cfg(feature = "track-source")]
    /// Returns the path to the file and the line number that this value originates from.
    ///
    /// The line number is 1-indexed to match convention;
    /// the first line will have a line number of 1 rather than 0.
    pub fn source(&self) -> Option<(&std::path::Path, usize)> {
        self.source
            .as_ref()
            .map(|(path, line)| (std::sync::Arc::as_ref(path), *line))
    }

    #[cfg(feature = "track-source")]
    /// Sets the path and line number from which this value originated.
    ///
    /// The line number should be 1-indexed to match convention;
    /// the first line should have a line number of 1 rather than 0.
    pub fn set_source(&mut self, path: impl Into<std::sync::Arc<std::path::Path>>, line: usize) {
        self.source = Some((path.into(), line))
    }

    #[cfg(feature = "track-source")]
    /// Clears the path and line number from which this value originated.
    pub fn clear_source(&mut self) {
        self.source = None;
    }

    /// Returns true if the value is unset.
    ///
    /// Does not handle values of "unset".
    /// See [`RawValue::filter_unset`].
    pub fn is_unset(&self) -> bool {
        self.value.is_empty()
    }

    /// Returns a reference to.an unset `RawValue`
    /// if the value case-insensitively matches `"unset"`,
    /// otherwise returns `self`.
    #[must_use]
    pub fn filter_unset(&self) -> &Self {
        if let Some(true) = self.detect_unset() {
            &UNSET
        } else {
            self
        }
    }

    /// Changes `self` to unset
    /// if the value case-insensitively matches `"unset"`.
    pub fn filter_unset_mut(&mut self) -> &mut Self {
        if let Some(true) = self.detect_unset() {
            *self = UNSET.clone();
        }
        self
    }

    /// Converts this `RawValue` into a [`Result`].
    ///
    /// This function filters out values of "unset".
    /// The `bool` in the `Err` variant will be false
    /// if and only if the value was not set.
    pub fn into_result(&self) -> Result<&str, bool> {
        if let Some(v) = self.detect_unset() {
            Err(v)
        } else {
            Ok(self.value.as_ref())
        }
    }

    /// Converts this `RawValue` into an [`Option`].
    pub fn into_option(&self) -> Option<&str> {
        Some(self.value.as_ref()).filter(|v| !v.is_empty())
    }

    /// Converts this `RawValue` into `&str`.
    ///
    /// If the value was not set, returns "unset".
    pub fn into_str(&self) -> &str {
        if self.is_unset() {
            "unset"
        } else {
            self.value.as_ref()
        }
    }

    /// Sets the contained string value.
    pub fn set<T: Into<RawValue>>(&mut self, val: T) -> &mut Self {
        *self = val.into();
        self
    }

    /// Attempts to parse the contained value.
    ///
    /// If the value is unset, returns `Err(None)`.
    pub fn parse<T: PropertyValue>(&self) -> Result<T, Option<T::Err>> {
        let this = if T::MAYBE_UNSET {
            self.filter_unset()
        } else {
            self
        };
        if this.is_unset() {
            Err(None)
        } else {
            T::parse(this).map_err(Some)
        }
    }

    /// Returns a lowercased version of `self`.
    #[must_use]
    pub fn to_lowercase(&self) -> Self {
        Self {
            value: Cow::Owned(self.value.to_lowercase()),
            #[cfg(feature = "track-source")]
            source: self.source.clone(),
        }
    }
}

impl std::fmt::Display for RawValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.as_ref())
    }
}

impl From<String> for RawValue {
    fn from(value: String) -> Self {
        RawValue {
            value: Cow::Owned(value),
            #[cfg(feature = "track-source")]
            source: None,
        }
    }
}

impl From<&'static str> for RawValue {
    fn from(value: &'static str) -> Self {
        if value.is_empty() {
            UNSET.clone()
        } else {
            RawValue {
                value: Cow::Borrowed(value),
                #[cfg(feature = "track-source")]
                source: None,
            }
        }
    }
}
