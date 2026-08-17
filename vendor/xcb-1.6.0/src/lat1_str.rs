use std::borrow::Cow;
use std::fmt;
use std::str;

/// Error that can produce Latin-1 string operations
#[derive(Debug, Copy, Clone)]
pub enum Lat1Error {
    /// Some non-ASCII characters were encountered.
    /// This error is generated when attempting to borrow
    /// a latin-1 string out of a UTF-8 string.
    /// For such borrow, only the ASCII character set is allowed.
    /// See [Lat1Str::try_from_ascii].
    NonAscii,
}

/// A slice to a Latin-1 (aka. ISO 8859-1) string.
///
/// It is usually seen in its borrowed form, `&Lat1Str`.
/// Lat1Str contains a slice of bytes and is by definition always
/// valid Latin-1.
///
/// This type is useful for XCB because strings in the X protocol are
/// expected to be Latin-1 encoded.
/// Although the X strings are Latin-1, in reality ASCII can be
/// expected without too much risk, hence all the ASCII related functions.
///
/// This does not account for strings passed as raw bytes
/// to [x::ChangeProperty](crate::x::ChangeProperty) (e.g. to set a window title).
/// These strings are passed as-is by the X server to the window compositor and
/// encoding is implied by the property itself
/// (e.g. UTF-8 for `_NET_WM_NAME` aka. window title).
pub struct Lat1Str {
    data: [u8],
}

impl Lat1Str {
    /// Returns a reference to a Lat1Str that borrows the passed bytes
    pub fn from_bytes(bytes: &[u8]) -> &Self {
        unsafe { &*(bytes as *const [u8] as *const Self) }
    }

    /// Returns a reference to a `Lat1Str` that borrows the passed string bytes
    /// only if `str` is pure ASCII.
    /// Otherwise, a `Lat1Error::NonAscii` is returned.
    pub fn try_from_ascii(str: &str) -> Result<&Self, Lat1Error> {
        if str.is_ascii() {
            Ok(Self::from_bytes(str.as_bytes()))
        } else {
            Err(Lat1Error::NonAscii)
        }
    }

    /// Returns a reference to a `Lat1Str` that borrows the passed string bytes
    /// only if `str` is pure ASCII.
    ///
    /// # Panics
    /// This function panics if `str` contains non-ASCII chars.
    pub fn from_ascii(str: &str) -> &Self {
        Self::try_from_ascii(str).unwrap()
    }

    /// Returns a reference to a `Lat1Str` that borrows the passed string bytes.
    ///
    /// # Safety
    /// If `str` contains non-ASCII characters, the returned string will not correspond
    /// to the passed string (the latin-1 will contain utf-8 encoding).
    pub unsafe fn from_ascii_unchecked(str: &str) -> &Self {
        Self::from_bytes(str.as_bytes())
    }

    /// Returns a Latin-1 string built from a UTF-8 string
    ///
    /// `Cow::Borrowed` is returned if `str` contains only ASCII,
    /// otherwise, a conversion from UTF-8 is performed and `Cow::Owned` is returned.
    pub fn from_utf8(str: &str) -> Cow<'_, Lat1Str> {
        if str.is_ascii() {
            Cow::Borrowed(Lat1Str::from_bytes(str.as_bytes()))
        } else {
            Cow::Owned(Lat1String::from_utf8(str))
        }
    }

    /// Checks whether the slice only contains ASCII characters.
    pub fn is_ascii(&self) -> bool {
        self.data.is_ascii()
    }

    /// Returns the number of characters in the string.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the string as slice of bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Returns the string in UTF-8 encoding, only if the string is pure ASCII.
    /// Otherwise, a `Lat1Error::NonAscii` is returned.
    pub fn try_as_ascii(&self) -> Result<&str, Lat1Error> {
        if self.is_ascii() {
            Ok(unsafe { str::from_utf8_unchecked(&self.data) })
        } else {
            Err(Lat1Error::NonAscii)
        }
    }

    /// Returns the string in UTF-8 encoding, only if the string is pure ASCII.
    ///
    /// # Panics
    /// This function panics if the string contains non-ASCII chars.
    pub fn as_ascii(&self) -> &str {
        self.try_as_ascii().unwrap()
    }

    /// Returns the string in UTF-8 encoding.
    ///
    /// # Safety
    /// If the string contains non-ASCII characters, the returned string will be
    /// invalid UTF-8.
    pub unsafe fn as_ascii_unchecked(&self) -> &str {
        str::from_utf8_unchecked(&self.data)
    }

    /// Returns the string converted to UTF-8.
    ///
    /// `Cow::Borrowed` is returned if the string is pure ASCII,
    /// otherwise a conversion to UTF-8 is performed and `Cow::Owned` is returned.
    pub fn to_utf8(&self) -> Cow<'_, str> {
        if self.is_ascii() {
            Cow::Borrowed(unsafe { self.as_ascii_unchecked() })
        } else {
            Cow::Owned(self.data.iter().map(|c| *c as char).collect())
        }
    }
}

impl std::borrow::ToOwned for Lat1Str {
    type Owned = Lat1String;
    fn to_owned(&self) -> Self::Owned {
        Lat1String {
            data: self.as_bytes().to_vec(),
        }
    }
}

impl fmt::Display for Lat1Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.to_utf8();
        f.write_str(&s)
    }
}

impl fmt::Debug for Lat1Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.to_utf8();
        f.write_fmt(format_args!("Lat1(\"{}\")", s))
    }
}

/// A struct owning a Latin-1 (aka. ISO 8859-1) string.
///
/// See [Lat1Str] for details.
#[derive(Clone)]
pub struct Lat1String {
    data: Vec<u8>,
}

impl Lat1String {
    /// Construct a [Lat1String] from a slice of bytes.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Lat1String {
            data: bytes.to_vec(),
        }
    }

    /// Construct a [Lat1String] from UTF-8 (a conversion to Latin-1 is performed).
    pub fn from_utf8(str: &str) -> Self {
        Lat1String {
            data: str.chars().map(|c| c as u8).collect(),
        }
    }
}

impl std::ops::Deref for Lat1String {
    type Target = Lat1Str;
    fn deref(&self) -> &Self::Target {
        Lat1Str::from_bytes(self.data.as_slice())
    }
}

impl std::borrow::Borrow<Lat1Str> for Lat1String {
    fn borrow(&self) -> &Lat1Str {
        Lat1Str::from_bytes(self.data.as_slice())
    }
}

impl fmt::Display for Lat1String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.to_utf8();
        f.write_str(&s)
    }
}

impl fmt::Debug for Lat1String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.to_utf8();
        f.write_fmt(format_args!("Lat1(\"{}\")", s))
    }
}

#[derive(Copy, Clone)]
/// Latin-1 (aka. ISO 8859-1) of fixed size
pub struct Lat1StrF<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Lat1StrF<N> {
    pub fn from_bytes(bytes: [u8; N]) -> Self {
        Self { data: bytes }
    }
}

impl<const N: usize> std::ops::Deref for Lat1StrF<N> {
    type Target = Lat1Str;
    fn deref(&self) -> &Self::Target {
        Lat1Str::from_bytes(self.data.as_slice())
    }
}

impl<const N: usize> fmt::Display for Lat1StrF<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.to_utf8();
        f.write_str(&s)
    }
}

impl<const N: usize> fmt::Debug for Lat1StrF<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.to_utf8();
        f.write_fmt(format_args!("Lat1(\"{}\")", s))
    }
}

#[test]
fn test_latin_str() {
    let utf8 = "Mon frère est là.";
    let latin1: &[u8] = &[
        0x4D, 0x6F, 0x6E, 0x20, 0x66, 0x72, 0xE8, 0x72, 0x65, 0x20, 0x65, 0x73, 0x74, 0x20, 0x6C,
        0xE0, 0x2E,
    ];

    let ls = Lat1String::from_utf8(utf8);
    assert_eq!(ls.as_bytes(), latin1);

    let ls = Lat1Str::from_bytes(latin1);
    assert_eq!(ls.to_utf8(), utf8);
}
