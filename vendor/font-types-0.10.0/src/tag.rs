use std::{
    borrow::Borrow,
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};

/// An OpenType tag.
///
/// [Per the spec][spec], a tag is a 4-byte array where each byte is in the
/// printable ASCII range `(0x20..=0x7E)`.
///
/// We do not strictly enforce this constraint as it is possible to encounter
/// invalid tags in existing fonts, and these need to be representable.
///
/// When creating new tags we encourage ensuring that the tag is valid,
/// either by using [`Tag::new_checked`] or by calling [`Tag::validate`] on an
/// existing tag.
///
/// [spec]: https://learn.microsoft.com/en-us/typography/opentype/spec/otff#data-types
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
#[repr(transparent)]
pub struct Tag([u8; 4]);

impl Tag {
    /// Construct a `Tag` from raw bytes.
    ///
    /// This does not perform any validation; use [`Tag::new_checked`] for a
    /// constructor that validates input.
    pub const fn new(src: &[u8; 4]) -> Tag {
        Tag(*src)
    }

    /// Attempt to create a `Tag` from raw bytes.
    ///
    /// The slice must contain between 1 and 4 bytes, each in the printable
    /// ascii range (`0x20..=0x7E`).
    ///
    /// If the input has fewer than four bytes, it will be padded with spaces.
    ///
    /// This method returns an [`InvalidTag`] error if the tag does conform to
    /// the spec.
    pub const fn new_checked(src: &[u8]) -> Result<Self, InvalidTag> {
        if src.is_empty() || src.len() > 4 {
            return Err(InvalidTag::InvalidLength(src.len()));
        }
        let mut raw = [0x20; 4];
        let mut i = 0;
        let mut seen_space = false;
        while i < src.len() {
            let byte = match src[i] {
                byte @ 0x20 if i == 0 => return Err(InvalidTag::InvalidByte { pos: i, byte }),
                byte @ 0..=0x1F | byte @ 0x7f.. => {
                    return Err(InvalidTag::InvalidByte { pos: i, byte })
                }
                byte @ 0x21..=0x7e if seen_space => {
                    return Err(InvalidTag::InvalidByte { pos: i, byte })
                }
                byte => byte,
            };

            seen_space |= byte == 0x20;

            raw[i] = byte;
            i += 1;
        }
        Ok(Tag(raw))
    }

    /// Construct a new `Tag` from a big-endian `u32`, without performing validation.
    ///
    /// This is provided as a convenience method for interop with code that
    /// stores tags as big-endian u32s.
    pub const fn from_u32(src: u32) -> Self {
        Self::from_be_bytes(src.to_be_bytes())
    }

    /// Create a tag from raw big-endian bytes.
    ///
    /// Prefer to use [`Tag::new`] (in const contexts) or [`Tag::new_checked`]
    /// when creating a `Tag`.
    ///
    /// This does not check the input, and is only intended to be used during
    /// parsing, where invalid inputs are accepted.
    pub const fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }

    // for symmetry with integer types / other things we encode/decode
    /// Return the memory representation of this tag.
    pub const fn to_be_bytes(self) -> [u8; 4] {
        self.0
    }

    /// Return the raw byte array representing this tag.
    pub fn into_bytes(self) -> [u8; 4] {
        self.0
    }

    /// Check that the tag conforms with the spec.
    ///
    /// This is intended for use during things like santization or lint passes
    /// on existing fonts; if you are creating a new tag, you should prefer
    /// [`Tag::new_checked`].
    ///
    /// Specifically, this checks the following conditions
    ///
    /// - the tag is not empty
    /// - the tag contains only characters in the printable ascii range (`0x20..=0x1F`)
    /// - the tag does not begin with a space
    /// - the tag does not contain any non-space characters after the first space
    pub fn validate(self) -> Result<(), InvalidTag> {
        if self == Tag::default() {
            return Err(InvalidTag::InvalidLength(0));
        }

        let mut seen_space = false;
        for (i, byte) in self.0.as_slice().iter().copied().enumerate() {
            match byte {
                0x20 if i == 0 => return Err(InvalidTag::InvalidByte { pos: i, byte }),
                0x20 => seen_space = true,
                0..=0x1F | 0x7f.. => return Err(InvalidTag::InvalidByte { pos: i, byte }),
                0x21..=0x7e if seen_space => return Err(InvalidTag::ByteAfterSpace { pos: i }),
                _ => (),
            }
        }
        Ok(())
    }
}

/// An error representing an invalid tag.
///
/// This is returned as an error from [`Tag::new_checked`] and
/// [`Tag::validate`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum InvalidTag {
    /// The tag was not between 1 and 4 bytes in length.
    InvalidLength(usize),
    /// The tag contained an invalid byte, not within the printable
    /// ASCII range `(0x20..=0x7E)`.
    InvalidByte { pos: usize, byte: u8 },
    /// The tag contained one or more non-space characters after a space.
    ByteAfterSpace { pos: usize },
}

impl FromStr for Tag {
    type Err = InvalidTag;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Tag::new_checked(src.as_bytes())
    }
}

impl crate::raw::Scalar for Tag {
    type Raw = [u8; 4];

    fn to_raw(self) -> Self::Raw {
        self.to_be_bytes()
    }

    fn from_raw(raw: Self::Raw) -> Self {
        Self::from_be_bytes(raw)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidTag {}

impl Borrow<[u8; 4]> for Tag {
    fn borrow(&self) -> &[u8; 4] {
        &self.0
    }
}

impl PartialEq<[u8; 4]> for Tag {
    fn eq(&self, other: &[u8; 4]) -> bool {
        &self.0 == other
    }
}

impl PartialEq<str> for Tag {
    fn eq(&self, other: &str) -> bool {
        self.0 == other.as_bytes()
    }
}

impl PartialEq<&str> for Tag {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<&[u8]> for Tag {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0.as_ref() == *other
    }
}

impl AsRef<[u8]> for Tag {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for byte in self.0 {
            if (0x20..=0x7E).contains(&byte) {
                write!(f, "{}", byte as char)?;
            } else {
                write!(f, "{{0x{:02X}}}", byte)?;
            }
        }
        Ok(())
    }
}

impl Display for InvalidTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidTag::InvalidByte { pos, byte } => {
                write!(f, "Invalid byte 0x{byte:X} at index {pos}")
            }
            InvalidTag::InvalidLength(len) => write!(f, "Invalid length ({len})"),
            InvalidTag::ByteAfterSpace { .. } => write!(f, "Non-space character after first space"),
        }
    }
}

impl Debug for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Tag({})", self)
    }
}

// a meaningless placeholder value.
impl Default for Tag {
    fn default() -> Self {
        Tag([b' '; 4])
    }
}

// fancy impls: these will serialize to a string if the target format is
// human-readable, but to bytes otherwise.
//
// NOTE: this means that tags which are not utf-8 will fail to serialize to
// json/yaml.
#[cfg(feature = "serde")]
impl serde::Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            std::str::from_utf8(&self.0)
                .map_err(serde::ser::Error::custom)?
                .serialize(serializer)
        } else {
            self.0.serialize(serializer)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TagStrVisitor;
        impl serde::de::Visitor<'_> for TagStrVisitor {
            type Value = Tag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a four-byte ascii string")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse().map_err(serde::de::Error::custom)
            }
        }
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(TagStrVisitor)
        } else {
            <[u8; 4]>::deserialize(deserializer).map(|raw| Tag::new(&raw))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        Tag::new(b"head");
        assert!(Tag::new_checked(b"").is_err());
        assert!(Tag::new_checked(b" ").is_err());
        assert!(Tag::new_checked(b"a").is_ok());
        assert!(Tag::new_checked(b"ab").is_ok());
        assert!(Tag::new_checked(b"abc").is_ok());
        assert!(Tag::new_checked(b"abcd").is_ok());
        assert!(Tag::new_checked(b"abcde").is_err());
        assert!(Tag::new_checked(b" bc").is_err()); // space invalid in first position
        assert!(Tag::new_checked(b"b c").is_err()); // non-space cannot follow space
        assert_eq!(Tag::new_checked(b"bc"), Ok(Tag::new(b"bc  ")));

        // ascii only:
        assert!(Tag::new_checked(&[0x19]).is_err());
        assert!(Tag::new_checked(&[0x21]).is_ok());
        assert!(Tag::new_checked(&[0x7E]).is_ok());
        assert!(Tag::new_checked(&[0x7F]).is_err());
    }

    #[test]
    fn validate_test() {
        assert!(Tag::new(b"    ").validate().is_err());
        assert!(Tag::new(b"a   ").validate().is_ok());
        assert!(Tag::new(b"ab  ").validate().is_ok());
        assert!(Tag::new(b"abc ").validate().is_ok());
        assert!(Tag::new(b"abcd").validate().is_ok());
        assert!(Tag::new(b" bcc").validate().is_err()); // space invalid in first position
        assert!(Tag::new(b"b cc").validate().is_err()); // non-space cannot follow space

        // ascii only:
        assert!(Tag::new(&[0x19, 0x33, 0x33, 0x33]).validate().is_err());
        assert!(Tag::new(&[0x21, 0x33, 0x33, 0x33]).validate().is_ok());
        assert!(Tag::new(&[0x7E, 0x33, 0x33, 0x33]).validate().is_ok());
        assert!(Tag::new(&[0x7F, 0x33, 0x33, 0x33]).validate().is_err());
    }

    #[test]
    #[cfg(feature = "std")]
    fn display() {
        let bad_tag = Tag::new(&[0x19, b'z', b'@', 0x7F]);
        assert_eq!(bad_tag.to_string(), "{0x19}z@{0x7F}");
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    #[derive(PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
    struct TestMe {
        tag: Tag,
    }

    #[test]
    fn serde_json_good() {
        let ser_me = TestMe {
            tag: Tag::new(b"yolo"),
        };

        let json_str = serde_json::to_string(&ser_me).unwrap();
        assert_eq!(json_str, r#"{"tag":"yolo"}"#);

        let de_me: TestMe = serde_json::from_str(&json_str).unwrap();
        assert_eq!(de_me, ser_me);
    }

    #[test]
    #[should_panic(expected = "invalid utf-8")]
    fn serde_json_bad() {
        let ser_me = TestMe {
            tag: Tag::new(&[3, 244, 0, 221]),
        };

        serde_json::to_string(&ser_me).unwrap();
    }

    // ensure that we impl DeserializeOwned
    #[test]
    fn deser_json_owned() {
        let json = r#"{"tag":"yolo"}"#;
        let de_me: TestMe = serde_json::from_reader(json.as_bytes()).unwrap();
        assert_eq!(de_me.tag, Tag::new(b"yolo"));
    }
}
