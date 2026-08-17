use super::Tag;
use crate::base::{Bytes, BytesCow, HasReplacementsError, Range};
use encoding_rs::Encoding;
use std::borrow::Cow;
use std::fmt;

// NOTE: All standard tag names contain only ASCII alpha characters
// and digits from 1 to 6 (in numbered header tags, i.e. <h1> - <h6>).
// Considering that tag names are case insensitive we have only
// 26 + 6 = 32 characters. Thus, single character can be encoded in
// 5 bits and we can fit up to 64 / 5 ≈ 12 characters in a 64-bit
// integer. This is enough to encode all standard tag names, so
// we can just compare integers instead of expensive string
// comparison for tag names.
//
// The original idea of this tag hash-like thing belongs to Ingvar
// Stepanyan and was implemented in lazyhtml. So, kudos to him for
// comming up with this cool optimisation. This implementation differs
// from the original one as it adds ability to encode digits from 1
// to 6 which allows us to encode numbered header tags.
//
// In this implementation we reserve numbers from 0 to 5 for digits
// from 1 to 6 and numbers from 6 to 31 for ASCII alphas. Otherwise,
// if we use numbers from 0 to 25 for ASCII alphas we'll have an
// ambiguity for repetitative `a` characters: both `a`,
// `aaa` and even `aaaaa` will give us 0 as a hash. It's still a case
// for digits, but considering that tag name can't start with a digit
// we are safe here, since we'll just get first character shifted left
// by zeroes as repetitave 1 digits get added to the hash.
//
// LocalNameHash is built incrementally as tags are parsed, so it needs
// to be able to invalidate itself if parsing an unrepresentable name.
// `EMPTY_HASH` is used as a sentinel value.
//
// Pub only for integration tests
#[derive(PartialEq, Eq, Copy, Clone, Default, Hash)]
pub struct LocalNameHash(u64);

const EMPTY_HASH: u64 = !0;

impl LocalNameHash {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == EMPTY_HASH
    }

    #[inline]
    pub fn update(&mut self, ch: u8) {
        let h = self.0;

        // NOTE: check if we still have space for yet another
        // character and if not then invalidate the hash.
        // Note, that we can't have `1` (which is encoded as 0b00000) as
        // a first character of a tag name, so it's safe to perform
        // check this way.
        // EMPTY_HASH has all bits set, so it will fail this check.
        self.0 = if h >> (64 - 5) == 0 {
            match ch {
                // NOTE: apply 0x1F mask on ASCII alpha to convert it to the
                // number from 1 to 26 (character case is controlled by one of
                // upper bits which we eliminate with the mask). Then add
                // 5, since numbers from 0 to 5 are reserved for digits.
                // Aftwerards put result as 5 lower bits of the hash.
                b'a'..=b'z' | b'A'..=b'Z' => (h << 5) | ((u64::from(ch) & 0x1F) + 5),

                // NOTE: apply 0x0F mask on ASCII digit to convert it to number
                // from 1 to 6. Then subtract 1 to make it zero-based.
                // Afterwards, put result as lower bits of the hash.
                b'1'..=b'6' => (h << 5) | ((u64::from(ch) & 0x0F) - 1),

                // NOTE: for any other characters hash function is not
                // applicable, so we completely invalidate the hash.
                _ => EMPTY_HASH,
            }
        } else {
            EMPTY_HASH
        };
    }
}

impl fmt::Debug for LocalNameHash {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return f.write_str("N/A");
        }

        let mut reverse_buf = [0u8; 12];
        let mut pos = 11;
        let mut h = self.0;
        loop {
            reverse_buf[pos] = match (h & 31) as u8 {
                v @ 6.. => v + (b'a' - 6),
                v => v + b'1',
            };
            h >>= 5;
            if h == 0 || pos == 0 {
                break;
            }
            pos -= 1;
        }
        std::str::from_utf8(&reverse_buf[pos..])
            .unwrap_or_default()
            .fmt(f)
    }
}

impl From<&str> for LocalNameHash {
    #[inline]
    fn from(string: &str) -> Self {
        let mut hash = Self::new();

        for ch in string.bytes() {
            hash.update(ch);
        }

        hash
    }
}

impl PartialEq<Tag> for LocalNameHash {
    #[inline]
    fn eq(&self, tag: &Tag) -> bool {
        self.0 == *tag as u64
    }
}

/// `LocalName` is used for the comparison of tag names.
/// In the majority of cases it will be represented as a hash, however for long
/// non-standard tag names it fallsback to the Name representation.
#[derive(Clone, Debug, Eq, Hash)]
pub enum LocalName<'i> {
    Hash(LocalNameHash),
    Bytes(BytesCow<'i>),
}

impl<'i> LocalName<'i> {
    #[inline]
    #[must_use]
    pub(crate) fn new(input: Bytes<'i>, range: Range, hash: LocalNameHash) -> Self {
        if hash.is_empty() {
            LocalName::Bytes(input.slice(range).into())
        } else {
            LocalName::Hash(hash)
        }
    }

    #[inline]
    #[must_use]
    pub fn into_owned(self) -> LocalName<'static> {
        match self {
            LocalName::Bytes(b) => LocalName::Bytes(b.into_owned()),
            LocalName::Hash(h) => LocalName::Hash(h),
        }
    }

    #[inline]
    pub(crate) fn from_str_without_replacements<'s>(
        string: impl Into<Cow<'s, str>>,
        encoding: &'static Encoding,
    ) -> Result<LocalName<'s>, HasReplacementsError> {
        let string = string.into();
        let hash = LocalNameHash::from(&*string);

        if hash.is_empty() {
            BytesCow::borrow_from_str_without_replacements(string, encoding).map(LocalName::Bytes)
        } else {
            Ok(LocalName::Hash(hash))
        }
    }
}

impl PartialEq<Tag> for LocalName<'_> {
    #[inline]
    fn eq(&self, tag: &Tag) -> bool {
        match self {
            LocalName::Hash(h) => h == tag,
            LocalName::Bytes(_) => false,
        }
    }
}

impl PartialEq<LocalName<'_>> for LocalName<'_> {
    #[inline]
    fn eq(&self, other: &LocalName<'_>) -> bool {
        use LocalName::{Bytes, Hash};

        match (self, other) {
            (Hash(s), Hash(o)) => {
                debug_assert!(!s.is_empty());
                s == o
            }
            (Bytes(s), Bytes(o)) => s.eq_ignore_ascii_case(o),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(LocalNameHash::from("div"), LocalNameHash(9691u64));
    }

    #[test]
    fn hash_invalidation_for_non_ascii_chars() {
        assert!(LocalNameHash::from("div@&").is_empty());
    }

    #[test]
    fn hash_invalidation_for_long_values() {
        assert!(LocalNameHash::from("aaaaaaaaaaaaaa").is_empty());
    }
}
