use super::{Assert, EStr, Encoder};
use crate::{pct_enc::Encode, utf8::Utf8Chunks};
use alloc::{borrow::ToOwned, string::String};
use core::{borrow::Borrow, cmp::Ordering, fmt, hash, marker::PhantomData, ops::Deref};

/// A percent-encoded, growable string.
///
/// The borrowed counterpart of `EString` is [`EStr`].
/// See its documentation for the meaning of the type parameter `E`.
///
/// # Comparison
///
/// `EString`s are compared [lexicographically](Ord#lexicographical-comparison)
/// by their byte values. Normalization is **not** performed prior to comparison.
///
/// # Examples
///
/// Encode key-value pairs to a query string and use it to build a URI reference:
///
/// ```
/// use fluent_uri::{
///     pct_enc::{
///         encoder::{Data, Query},
///         EStr, EString, Encoder, Table,
///     },
///     UriRef,
/// };
///
/// let pairs = [("name", "张三"), ("speech", "¡Olé!")];
/// let mut buf = EString::<Query>::new();
/// for (k, v) in pairs {
///     if !buf.is_empty() {
///         buf.push('&');
///     }
///
///     // WARNING: Absolutely do not confuse data with delimiters!
///     // Use `Data` (or `IData`) to encode data contained in a URI
///     // (or an IRI) unless you know what you're doing!
///     buf.encode_str::<Data>(k);
///     buf.push('=');
///     buf.encode_str::<Data>(v);
/// }
///
/// assert_eq!(buf, "name=%E5%BC%A0%E4%B8%89&speech=%C2%A1Ol%C3%A9%21");
///
/// let uri_ref = UriRef::builder()
///     .path(EStr::EMPTY)
///     .query(&buf)
///     .build()
///     .unwrap();
/// assert_eq!(uri_ref.as_str(), "?name=%E5%BC%A0%E4%B8%89&speech=%C2%A1Ol%C3%A9%21");
/// ```
///
/// Encode a path whose segments may contain the slash (`'/'`) character
/// by using a custom sub-encoder:
///
/// ```
/// use fluent_uri::pct_enc::{encoder::Path, EString, Encoder, Table};
///
/// struct PathSegment;
///
/// impl Encoder for PathSegment {
///     const TABLE: &'static Table = &Path::TABLE.sub(&Table::new(b"/"));
/// }
///
/// let mut path = EString::<Path>::new();
/// path.push('/');
/// path.encode_str::<PathSegment>("foo/bar");
///
/// assert_eq!(path, "/foo%2Fbar");
/// ```
#[derive(Clone, Default)]
pub struct EString<E: Encoder> {
    pub(crate) buf: String,
    encoder: PhantomData<E>,
}

impl<E: Encoder> Deref for EString<E> {
    type Target = EStr<E>;

    fn deref(&self) -> &EStr<E> {
        EStr::new_validated(&self.buf)
    }
}

impl<E: Encoder> EString<E> {
    pub(crate) fn new_validated(buf: String) -> Self {
        Self {
            buf,
            encoder: PhantomData,
        }
    }

    /// Creates a new empty `EString`.
    #[must_use]
    pub fn new() -> Self {
        Self::new_validated(String::new())
    }

    /// Creates a new empty `EString` with at least the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::new_validated(String::with_capacity(capacity))
    }

    /// Coerces to an `EStr` slice.
    #[must_use]
    pub fn as_estr(&self) -> &EStr<E> {
        self
    }

    /// Returns this `EString`'s capacity, in bytes.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    /// Encodes a string with a sub-encoder and appends the result onto the end of this `EString`.
    ///
    /// A character will be preserved if `SubE::TABLE` [allows] it and percent-encoded otherwise.
    ///
    /// In most cases, use [`Data`] (for URI) or [`IData`] (for IRI) as the sub-encoder.
    /// When using other sub-encoders, make sure that `SubE::TABLE` does not [allow][allows]
    /// the component delimiters that delimit the data.
    ///
    /// Note that this method will **not** encode `U+0020` (space) as `U+002B` (+).
    ///
    /// [allows]: super::Table::allows
    /// [`Data`]: super::encoder::Data
    /// [`IData`]: super::encoder::IData
    ///
    /// # Panics
    ///
    /// Panics at compile time if `SubE` is not a [sub-encoder](Encoder#sub-encoders) of `E`,
    /// or if `SubE::TABLE` does not [allow percent-encoded octets].
    ///
    /// [allow percent-encoded octets]: super::Table::allows_pct_encoded
    pub fn encode_str<SubE: Encoder>(&mut self, s: &str) {
        () = Assert::<SubE, E>::L_IS_SUB_ENCODER_OF_R;
        () = EStr::<SubE>::ASSERT_ALLOWS_PCT_ENCODED;

        for chunk in Encode::new(SubE::TABLE, s) {
            self.buf.push_str(chunk.as_str());
        }
    }

    /// Encodes a byte sequence with a sub-encoder and appends the result onto the end of this `EString`.
    ///
    /// A byte will be preserved if it is part of a UTF-8-encoded character
    /// that `SubE::TABLE` [allows] and percent-encoded otherwise.
    ///
    /// In most cases, use [`Data`] (for URI) or [`IData`] (for IRI) as the sub-encoder.
    /// When using other sub-encoders, make sure that `SubE::TABLE` does not [allow][allows]
    /// the component delimiters that delimit the data.
    ///
    /// Note that this method will **not** encode `0x20` (space) as `U+002B` (+).
    ///
    /// If you need to encode a string, use [`encode_str`][Self::encode_str] instead.
    ///
    /// [allows]: super::Table::allows
    /// [`Data`]: super::encoder::Data
    /// [`IData`]: super::encoder::IData
    ///
    /// # Deprecation
    ///
    /// This method is deprecated because percent-encoding non-UTF-8 bytes is
    /// a non-standard operation. If you're developing a new protocol, use
    /// other encodings such as Base64 instead. If you absolutely must, here's
    /// a workaround:
    ///
    /// ```
    /// use fluent_uri::pct_enc::{encoder::Path, EStr, EString};
    ///
    /// let mut buf = EString::<Path>::new();
    ///
    /// for chunk in b"D\xFCrst".utf8_chunks() {
    ///     buf.encode_str::<Path>(chunk.valid());
    ///     for &x in chunk.invalid() {
    ///         buf.push_estr(EStr::encode_byte(x));
    ///     }
    /// }
    ///
    /// assert_eq!(buf, "D%FCrst");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics at compile time if `SubE` is not a [sub-encoder](Encoder#sub-encoders) of `E`,
    /// or if `SubE::TABLE` does not [allow percent-encoded octets].
    ///
    /// [allow percent-encoded octets]: super::Table::allows_pct_encoded
    #[deprecated = "use `<[u8]>::utf8_chunks`, `EString::encode_str`, `EStr::encode_byte`, and `EString::push_estr` instead"]
    pub fn encode_bytes<SubE: Encoder>(&mut self, bytes: &[u8]) {
        () = Assert::<SubE, E>::L_IS_SUB_ENCODER_OF_R;
        () = EStr::<SubE>::ASSERT_ALLOWS_PCT_ENCODED;

        for chunk in Utf8Chunks::new(bytes) {
            for chunk in Encode::new(SubE::TABLE, chunk.valid()) {
                self.buf.push_str(chunk.as_str());
            }
            for &x in chunk.invalid() {
                self.buf.push_str(super::encode_byte(x));
            }
        }
    }

    /// Appends an unencoded character onto the end of this `EString`.
    ///
    /// # Panics
    ///
    /// Panics if `E::TABLE` does not [allow] the character.
    ///
    /// [allow]: super::Table::allows
    pub fn push(&mut self, ch: char) {
        assert!(E::TABLE.allows(ch), "table does not allow the char");
        self.buf.push(ch);
    }

    /// Appends an `EStr` slice onto the end of this `EString`.
    pub fn push_estr(&mut self, s: &EStr<E>) {
        self.buf.push_str(s.as_str());
    }

    /// Truncates this `EString`, removing all contents.
    pub fn clear(&mut self) {
        self.buf.clear();
    }

    /// Consumes this `EString` and yields the underlying `String`.
    #[must_use]
    pub fn into_string(self) -> String {
        self.buf
    }
}

impl<E: Encoder> AsRef<EStr<E>> for EString<E> {
    fn as_ref(&self) -> &EStr<E> {
        self
    }
}

impl<E: Encoder> AsRef<str> for EString<E> {
    fn as_ref(&self) -> &str {
        &self.buf
    }
}

impl<E: Encoder> Borrow<EStr<E>> for EString<E> {
    fn borrow(&self) -> &EStr<E> {
        self
    }
}

impl<E: Encoder> From<&EStr<E>> for EString<E> {
    fn from(s: &EStr<E>) -> Self {
        s.to_owned()
    }
}

impl<E: Encoder> PartialEq for EString<E> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<E: Encoder> PartialEq<EStr<E>> for EString<E> {
    fn eq(&self, other: &EStr<E>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<E: Encoder> PartialEq<EString<E>> for EStr<E> {
    fn eq(&self, other: &EString<E>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<E: Encoder> PartialEq<&EStr<E>> for EString<E> {
    fn eq(&self, other: &&EStr<E>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<E: Encoder> PartialEq<EString<E>> for &EStr<E> {
    fn eq(&self, other: &EString<E>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<E: Encoder> PartialEq<str> for EString<E> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<E: Encoder> PartialEq<EString<E>> for str {
    fn eq(&self, other: &EString<E>) -> bool {
        self == other.as_str()
    }
}

impl<E: Encoder> PartialEq<&str> for EString<E> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<E: Encoder> PartialEq<EString<E>> for &str {
    fn eq(&self, other: &EString<E>) -> bool {
        *self == other.as_str()
    }
}

impl<E: Encoder> Eq for EString<E> {}

impl<E: Encoder> hash::Hash for EString<E> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.buf.hash(state);
    }
}

impl<E: Encoder> PartialOrd for EString<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<E: Encoder> Ord for EString<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<E: Encoder> fmt::Debug for EString<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<E: Encoder> fmt::Display for EString<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
