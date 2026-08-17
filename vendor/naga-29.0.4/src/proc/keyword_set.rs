use core::{fmt, hash};

use crate::racy_lock::RacyLock;
use crate::FastHashSet;

/// A case-sensitive set of strings,
/// for use with [`Namer`][crate::proc::Namer] to avoid collisions with keywords and other reserved
/// identifiers.
///
/// This is currently implemented as a hash table.
/// Future versions of Naga may change the implementation based on speed and code size
/// considerations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct KeywordSet(FastHashSet<&'static str>);

impl KeywordSet {
    /// Returns a new mutable empty set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the empty set.
    pub fn empty() -> &'static Self {
        static EMPTY: RacyLock<KeywordSet> = RacyLock::new(Default::default);
        &EMPTY
    }

    /// Returns whether the set contains the given string.
    #[inline]
    pub fn contains(&self, identifier: &str) -> bool {
        self.0.contains(identifier)
    }
}

impl Default for &'static KeywordSet {
    fn default() -> Self {
        KeywordSet::empty()
    }
}

impl FromIterator<&'static str> for KeywordSet {
    fn from_iter<T: IntoIterator<Item = &'static str>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

/// Accepts double references so that `KeywordSet::from_iter(&["foo"])` works.
impl<'a> FromIterator<&'a &'static str> for KeywordSet {
    fn from_iter<T: IntoIterator<Item = &'a &'static str>>(iter: T) -> Self {
        Self::from_iter(iter.into_iter().copied())
    }
}

impl Extend<&'static str> for KeywordSet {
    #[expect(
        clippy::useless_conversion,
        reason = "doing .into_iter() sooner reduces distinct monomorphizations"
    )]
    fn extend<T: IntoIterator<Item = &'static str>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter())
    }
}

/// Accepts double references so that `.extend(&["foo"])` works.
impl<'a> Extend<&'a &'static str> for KeywordSet {
    fn extend<T: IntoIterator<Item = &'a &'static str>>(&mut self, iter: T) {
        self.extend(iter.into_iter().copied())
    }
}

/// A case-insensitive, ASCII-only set of strings,
/// for use with [`Namer`][crate::proc::Namer] to avoid collisions with keywords and other reserved
/// identifiers.
///
/// This is currently implemented as a hash table.
/// Future versions of Naga may change the implementation based on speed and code size
/// considerations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CaseInsensitiveKeywordSet(FastHashSet<AsciiUniCase<&'static str>>);

impl CaseInsensitiveKeywordSet {
    /// Returns a new mutable empty set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the empty set.
    pub fn empty() -> &'static Self {
        static EMPTY: RacyLock<CaseInsensitiveKeywordSet> = RacyLock::new(Default::default);
        &EMPTY
    }

    /// Returns whether the set contains the given string, with comparison
    /// by [`str::eq_ignore_ascii_case()`].
    #[inline]
    pub fn contains(&self, identifier: &str) -> bool {
        self.0.contains(&AsciiUniCase(identifier))
    }
}

impl Default for &'static CaseInsensitiveKeywordSet {
    fn default() -> Self {
        CaseInsensitiveKeywordSet::empty()
    }
}

impl FromIterator<&'static str> for CaseInsensitiveKeywordSet {
    fn from_iter<T: IntoIterator<Item = &'static str>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .inspect(debug_assert_ascii)
                .map(AsciiUniCase)
                .collect(),
        )
    }
}

/// Accepts double references so that `CaseInsensitiveKeywordSet::from_iter(&["foo"])` works.
impl<'a> FromIterator<&'a &'static str> for CaseInsensitiveKeywordSet {
    fn from_iter<T: IntoIterator<Item = &'a &'static str>>(iter: T) -> Self {
        Self::from_iter(iter.into_iter().copied())
    }
}

impl Extend<&'static str> for CaseInsensitiveKeywordSet {
    fn extend<T: IntoIterator<Item = &'static str>>(&mut self, iter: T) {
        self.0.extend(
            iter.into_iter()
                .inspect(debug_assert_ascii)
                .map(AsciiUniCase),
        )
    }
}

/// Accepts double references so that `.extend(&["foo"])` works.
impl<'a> Extend<&'a &'static str> for CaseInsensitiveKeywordSet {
    fn extend<T: IntoIterator<Item = &'a &'static str>>(&mut self, iter: T) {
        self.extend(iter.into_iter().copied())
    }
}

/// A string wrapper type with an ascii case insensitive Eq and Hash impl
#[derive(Clone, Copy)]
struct AsciiUniCase<S: AsRef<str> + ?Sized>(S);

impl<S: ?Sized + AsRef<str>> fmt::Debug for AsciiUniCase<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.as_ref().fmt(f)
    }
}

impl<S: AsRef<str>> PartialEq<Self> for AsciiUniCase<S> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref().eq_ignore_ascii_case(other.0.as_ref())
    }
}

impl<S: AsRef<str>> Eq for AsciiUniCase<S> {}

impl<S: AsRef<str>> hash::Hash for AsciiUniCase<S> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        for byte in self
            .0
            .as_ref()
            .as_bytes()
            .iter()
            .map(|b| b.to_ascii_lowercase())
        {
            hasher.write_u8(byte);
        }
    }
}

fn debug_assert_ascii(s: &&'static str) {
    debug_assert!(s.is_ascii(), "{s:?} not ASCII")
}
