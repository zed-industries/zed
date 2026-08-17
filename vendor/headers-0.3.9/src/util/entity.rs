use std::fmt;

use super::{FlatCsv, IterExt};
use HeaderValue;

/// An entity tag, defined in [RFC7232](https://tools.ietf.org/html/rfc7232#section-2.3)
///
/// An entity tag consists of a string enclosed by two literal double quotes.
/// Preceding the first double quote is an optional weakness indicator,
/// which always looks like `W/`. Examples for valid tags are `"xyzzy"` and `W/"xyzzy"`.
///
/// # ABNF
///
/// ```text
/// entity-tag = [ weak ] opaque-tag
/// weak       = %x57.2F ; "W/", case-sensitive
/// opaque-tag = DQUOTE *etagc DQUOTE
/// etagc      = %x21 / %x23-7E / obs-text
///            ; VCHAR except double quotes, plus obs-text
/// ```
///
/// # Comparison
/// To check if two entity tags are equivalent in an application always use the `strong_eq` or
/// `weak_eq` methods based on the context of the Tag. Only use `==` to check if two tags are
/// identical.
///
/// The example below shows the results for a set of entity-tag pairs and
/// both the weak and strong comparison function results:
///
/// | ETag 1  | ETag 2  | Strong Comparison | Weak Comparison |
/// |---------|---------|-------------------|-----------------|
/// | `W/"1"` | `W/"1"` | no match          | match           |
/// | `W/"1"` | `W/"2"` | no match          | no match        |
/// | `W/"1"` | `"1"`   | no match          | match           |
/// | `"1"`   | `"1"`   | match             | match           |
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct EntityTag<T = HeaderValue>(T);

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EntityTagRange {
    Any,
    Tags(FlatCsv),
}

// ===== impl EntityTag =====

impl<T: AsRef<[u8]>> EntityTag<T> {
    /// Get the tag.
    pub(crate) fn tag(&self) -> &[u8] {
        let bytes = self.0.as_ref();
        let end = bytes.len() - 1;
        if bytes[0] == b'W' {
            // W/"<tag>"
            &bytes[3..end]
        } else {
            // "<tag>"
            &bytes[1..end]
        }
    }

    /// Return if this is a "weak" tag.
    pub(crate) fn is_weak(&self) -> bool {
        self.0.as_ref()[0] == b'W'
    }

    /// For strong comparison two entity-tags are equivalent if both are not weak and their
    /// opaque-tags match character-by-character.
    pub(crate) fn strong_eq<R>(&self, other: &EntityTag<R>) -> bool
    where
        R: AsRef<[u8]>,
    {
        !self.is_weak() && !other.is_weak() && self.tag() == other.tag()
    }

    /// For weak comparison two entity-tags are equivalent if their
    /// opaque-tags match character-by-character, regardless of either or
    /// both being tagged as "weak".
    pub(crate) fn weak_eq<R>(&self, other: &EntityTag<R>) -> bool
    where
        R: AsRef<[u8]>,
    {
        self.tag() == other.tag()
    }

    /// The inverse of `EntityTag.strong_eq()`.
    #[cfg(test)]
    pub(crate) fn strong_ne(&self, other: &EntityTag) -> bool {
        !self.strong_eq(other)
    }

    /// The inverse of `EntityTag.weak_eq()`.
    #[cfg(test)]
    pub(crate) fn weak_ne(&self, other: &EntityTag) -> bool {
        !self.weak_eq(other)
    }

    pub(crate) fn parse(src: T) -> Option<Self> {
        let slice = src.as_ref();
        let length = slice.len();

        // Early exits if it doesn't terminate in a DQUOTE.
        if length < 2 || slice[length - 1] != b'"' {
            return None;
        }

        let start = match slice[0] {
            // "<tag>"
            b'"' => 1,
            // W/"<tag>"
            b'W' => {
                if length >= 4 && slice[1] == b'/' && slice[2] == b'"' {
                    3
                } else {
                    return None;
                }
            }
            _ => return None,
        };

        if check_slice_validity(&slice[start..length - 1]) {
            Some(EntityTag(src))
        } else {
            None
        }
    }
}

impl EntityTag {
    /*
    /// Constructs a new EntityTag.
    /// # Panics
    /// If the tag contains invalid characters.
    pub fn new(weak: bool, tag: String) -> EntityTag {
        assert!(check_slice_validity(&tag), "Invalid tag: {:?}", tag);
        EntityTag { weak: weak, tag: tag }
    }

    /// Constructs a new weak EntityTag.
    /// # Panics
    /// If the tag contains invalid characters.
    pub fn weak(tag: String) -> EntityTag {
        EntityTag::new(true, tag)
    }

    /// Constructs a new strong EntityTag.
    /// # Panics
    /// If the tag contains invalid characters.
    pub fn strong(tag: String) -> EntityTag {
        EntityTag::new(false, tag)
    }
    */

    #[cfg(test)]
    pub fn from_static(bytes: &'static str) -> EntityTag {
        let val = HeaderValue::from_static(bytes);
        match EntityTag::from_val(&val) {
            Some(tag) => tag,
            None => {
                panic!("invalid static string for EntityTag: {:?}", bytes);
            }
        }
    }

    pub(crate) fn from_owned(val: HeaderValue) -> Option<EntityTag> {
        EntityTag::parse(val.as_bytes())?;
        Some(EntityTag(val))
    }

    pub(crate) fn from_val(val: &HeaderValue) -> Option<EntityTag> {
        EntityTag::parse(val.as_bytes()).map(|_entity| EntityTag(val.clone()))
    }
}

impl<T: fmt::Debug> fmt::Debug for EntityTag<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl super::TryFromValues for EntityTag {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .just_one()
            .and_then(EntityTag::from_val)
            .ok_or_else(::Error::invalid)
    }
}

impl From<EntityTag> for HeaderValue {
    fn from(tag: EntityTag) -> HeaderValue {
        tag.0
    }
}

impl<'a> From<&'a EntityTag> for HeaderValue {
    fn from(tag: &'a EntityTag) -> HeaderValue {
        tag.0.clone()
    }
}

/// check that each char in the slice is either:
/// 1. `%x21`, or
/// 2. in the range `%x23` to `%x7E`, or
/// 3. above `%x80`
fn check_slice_validity(slice: &[u8]) -> bool {
    slice.iter().all(|&c| {
        // HeaderValue already validates that this doesnt contain control
        // characters, so we only need to look for DQUOTE (`"`).
        //
        // The debug_assert is just in case we use check_slice_validity in
        // some new context that didnt come from a HeaderValue.
        debug_assert!(
            (c >= b'\x21' && c <= b'\x7e') | (c >= b'\x80'),
            "EntityTag expects HeaderValue to have check for control characters"
        );
        c != b'"'
    })
}

// ===== impl EntityTagRange =====

impl EntityTagRange {
    pub(crate) fn matches_strong(&self, entity: &EntityTag) -> bool {
        self.matches_if(entity, |a, b| a.strong_eq(b))
    }

    pub(crate) fn matches_weak(&self, entity: &EntityTag) -> bool {
        self.matches_if(entity, |a, b| a.weak_eq(b))
    }

    fn matches_if<F>(&self, entity: &EntityTag, func: F) -> bool
    where
        F: Fn(&EntityTag<&str>, &EntityTag) -> bool,
    {
        match *self {
            EntityTagRange::Any => true,
            EntityTagRange::Tags(ref tags) => tags
                .iter()
                .flat_map(EntityTag::<&str>::parse)
                .any(|tag| func(&tag, entity)),
        }
    }
}

impl super::TryFromValues for EntityTagRange {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let flat = FlatCsv::try_from_values(values)?;
        if flat.value == "*" {
            Ok(EntityTagRange::Any)
        } else {
            Ok(EntityTagRange::Tags(flat))
        }
    }
}

impl<'a> From<&'a EntityTagRange> for HeaderValue {
    fn from(tag: &'a EntityTagRange) -> HeaderValue {
        match *tag {
            EntityTagRange::Any => HeaderValue::from_static("*"),
            EntityTagRange::Tags(ref tags) => tags.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(slice: &[u8]) -> Option<EntityTag> {
        let val = HeaderValue::from_bytes(slice).ok()?;
        EntityTag::from_val(&val)
    }

    #[test]
    fn test_etag_parse_success() {
        // Expected success
        let tag = parse(b"\"foobar\"").unwrap();
        assert!(!tag.is_weak());
        assert_eq!(tag.tag(), b"foobar");

        let weak = parse(b"W/\"weaktag\"").unwrap();
        assert!(weak.is_weak());
        assert_eq!(weak.tag(), b"weaktag");
    }

    #[test]
    fn test_etag_parse_failures() {
        // Expected failures
        macro_rules! fails {
            ($slice:expr) => {
                assert_eq!(parse($slice), None);
            };
        }

        fails!(b"no-dquote");
        fails!(b"w/\"the-first-w-is-case sensitive\"");
        fails!(b"W/\"");
        fails!(b"");
        fails!(b"\"unmatched-dquotes1");
        fails!(b"unmatched-dquotes2\"");
        fails!(b"\"inner\"quotes\"");
    }

    /*
    #[test]
    fn test_etag_fmt() {
        assert_eq!(format!("{}", EntityTag::strong("foobar".to_owned())), "\"foobar\"");
        assert_eq!(format!("{}", EntityTag::strong("".to_owned())), "\"\"");
        assert_eq!(format!("{}", EntityTag::weak("weak-etag".to_owned())), "W/\"weak-etag\"");
        assert_eq!(format!("{}", EntityTag::weak("\u{0065}".to_owned())), "W/\"\x65\"");
        assert_eq!(format!("{}", EntityTag::weak("".to_owned())), "W/\"\"");
    }
    */

    #[test]
    fn test_cmp() {
        // | ETag 1  | ETag 2  | Strong Comparison | Weak Comparison |
        // |---------|---------|-------------------|-----------------|
        // | `W/"1"` | `W/"1"` | no match          | match           |
        // | `W/"1"` | `W/"2"` | no match          | no match        |
        // | `W/"1"` | `"1"`   | no match          | match           |
        // | `"1"`   | `"1"`   | match             | match           |
        let mut etag1 = EntityTag::from_static("W/\"1\"");
        let mut etag2 = etag1.clone();
        assert!(!etag1.strong_eq(&etag2));
        assert!(etag1.weak_eq(&etag2));
        assert!(etag1.strong_ne(&etag2));
        assert!(!etag1.weak_ne(&etag2));

        etag2 = EntityTag::from_static("W/\"2\"");
        assert!(!etag1.strong_eq(&etag2));
        assert!(!etag1.weak_eq(&etag2));
        assert!(etag1.strong_ne(&etag2));
        assert!(etag1.weak_ne(&etag2));

        etag2 = EntityTag::from_static("\"1\"");
        assert!(!etag1.strong_eq(&etag2));
        assert!(etag1.weak_eq(&etag2));
        assert!(etag1.strong_ne(&etag2));
        assert!(!etag1.weak_ne(&etag2));

        etag1 = EntityTag::from_static("\"1\"");
        assert!(etag1.strong_eq(&etag2));
        assert!(etag1.weak_eq(&etag2));
        assert!(!etag1.strong_ne(&etag2));
        assert!(!etag1.weak_ne(&etag2));
    }
}
