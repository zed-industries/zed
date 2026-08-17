//! Percent-encoding utilities.

pub mod encoder;
#[cfg(feature = "alloc")]
mod estring;
pub(crate) mod table;

#[cfg(feature = "alloc")]
pub use estring::EString;
pub use table::Table;

use crate::imp::PathEncoder;
use core::{cmp::Ordering, hash, iter::FusedIterator, marker::PhantomData, str};
use ref_cast::{ref_cast_custom, RefCastCustom};

#[cfg(feature = "alloc")]
use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
    vec::Vec,
};

/// A trait used by [`EStr`] and [`EString`] to specify the table used for encoding.
///
/// # Sub-encoders
///
/// A sub-encoder `SubE` of `E` is an encoder such that `SubE::TABLE` is a [subset] of `E::TABLE`.
///
/// [subset]: Table::is_subset
pub trait Encoder: 'static {
    /// The table used for encoding.
    const TABLE: &'static Table;
}

/// Percent-encoded string slices.
///
/// The owned counterpart of `EStr` is [`EString`]. See its documentation
/// if you want to build a percent-encoded string from scratch.
///
/// # Type parameter
///
/// The `EStr<E>` type is parameterized over a type `E` that implements [`Encoder`].
/// The associated constant `E::TABLE` of type [`Table`] specifies the byte patterns
/// allowed in a string. In short, the underlying byte sequence of an `EStr<E>` slice
/// can be formed by joining any number of the following byte sequences:
///
/// - `ch.encode_utf8(&mut [0; 4])` where `E::TABLE.allows(ch)`.
/// - `[b'%', hi, lo]` where `E::TABLE.allows_pct_encoded() && hi.is_ascii_hexdigit() && lo.is_ascii_hexdigit()`.
///
/// # Comparison
///
/// `EStr` slices are compared [lexicographically](Ord#lexicographical-comparison)
/// by their byte values. Normalization is **not** performed prior to comparison.
///
/// # Examples
///
/// Parse key-value pairs from a query string into a hash map:
///
/// ```
/// use fluent_uri::{pct_enc::EStr, UriRef};
/// use std::collections::HashMap;
///
/// let s = "?name=%E5%BC%A0%E4%B8%89&speech=%C2%A1Ol%C3%A9%21";
/// let query = UriRef::parse(s)?.query().unwrap();
/// let map: HashMap<_, _> = query
///     .split('&')
///     .map(|s| s.split_once('=').unwrap_or((s, EStr::EMPTY)))
///     .map(|(k, v)| (k.decode().to_string_lossy(), v.decode().to_string_lossy()))
///     .collect();
/// assert_eq!(map["name"], "张三");
/// assert_eq!(map["speech"], "¡Olé!");
/// # Ok::<_, fluent_uri::ParseError>(())
/// ```
#[derive(RefCastCustom)]
#[repr(transparent)]
pub struct EStr<E: Encoder> {
    encoder: PhantomData<E>,
    inner: str,
}

#[cfg(feature = "alloc")]
struct Assert<L: Encoder, R: Encoder> {
    _marker: PhantomData<(L, R)>,
}

#[cfg(feature = "alloc")]
impl<L: Encoder, R: Encoder> Assert<L, R> {
    const L_IS_SUB_ENCODER_OF_R: () = assert!(L::TABLE.is_subset(R::TABLE), "not a sub-encoder");
}

impl<E: Encoder> EStr<E> {
    const ASSERT_ALLOWS_PCT_ENCODED: () = assert!(
        E::TABLE.allows_pct_encoded(),
        "table does not allow percent-encoded octets"
    );

    /// Converts a string slice to an `EStr` slice assuming validity.
    #[ref_cast_custom]
    pub(crate) const fn new_validated(s: &str) -> &Self;

    /// An empty `EStr` slice.
    pub const EMPTY: &'static Self = Self::new_validated("");

    pub(crate) fn cast<F: Encoder>(&self) -> &EStr<F> {
        EStr::new_validated(&self.inner)
    }

    /// Converts a string slice to an `EStr` slice.
    ///
    /// # Panics
    ///
    /// Panics if the string is not properly encoded with `E`.
    /// For a non-panicking variant, use [`new`](Self::new).
    #[must_use]
    pub const fn new_or_panic(s: &str) -> &Self {
        match Self::new(s) {
            Some(s) => s,
            None => panic!("improperly encoded string"),
        }
    }

    /// Converts a string slice to an `EStr` slice, returning `None` if the conversion fails.
    #[must_use]
    pub const fn new(s: &str) -> Option<&Self> {
        if E::TABLE.validate(s.as_bytes()) {
            Some(Self::new_validated(s))
        } else {
            None
        }
    }

    /// Creates an `EStr` slice containing a single percent-encoded octet representing the given byte.
    ///
    /// # Panics
    ///
    /// Panics at compile time if `E::TABLE` does not [allow percent-encoded octets].
    ///
    /// [allow percent-encoded octets]: Table::allows_pct_encoded
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_uri::pct_enc::{encoder::Path, EStr};
    ///
    /// assert_eq!(EStr::<Path>::encode_byte(b'1'), "%31");
    /// ```
    #[must_use]
    pub fn encode_byte(x: u8) -> &'static Self {
        () = Self::ASSERT_ALLOWS_PCT_ENCODED;
        Self::new_validated(encode_byte(x))
    }

    /// Yields the underlying string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Returns the length of the `EStr` slice in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Checks whether the `EStr` slice is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Upcasts the `EStr` slice to associate it with the given super-encoder.
    ///
    /// # Panics
    ///
    /// Panics at compile time if `E` is not a [sub-encoder](Encoder#sub-encoders) of `SuperE`.
    ///
    /// # Example
    ///
    /// ```
    /// use fluent_uri::pct_enc::{encoder::{IPath, Path}, EStr};
    ///
    /// let path = EStr::<Path>::new_or_panic("foo");
    /// let path: &EStr<IPath> = path.upcast();
    /// ```
    #[cfg(fluent_uri_unstable)]
    #[must_use]
    pub fn upcast<SuperE: Encoder>(&self) -> &EStr<SuperE> {
        () = Assert::<E, SuperE>::L_IS_SUB_ENCODER_OF_R;
        EStr::new_validated(self.as_str())
    }

    /// Checks whether the `EStr` slice is unencoded, i.e., does not contain `'%'`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_uri::pct_enc::{encoder::Path, EStr};
    ///
    /// assert!(EStr::<Path>::new_or_panic("Hello!").is_unencoded());
    /// assert!(!EStr::<Path>::new_or_panic("%C2%A1Hola%21").is_unencoded());
    /// ```
    #[cfg(fluent_uri_unstable)]
    #[must_use]
    pub fn is_unencoded(&self) -> bool {
        !(E::TABLE.allows_pct_encoded() && self.inner.contains('%'))
    }

    /// Returns an iterator used to decode the `EStr` slice.
    ///
    /// Always **split before decoding**, as otherwise the data may be
    /// mistaken for component delimiters.
    ///
    /// Note that the iterator will **not** decode `U+002B` (+) as `0x20` (space).
    ///
    /// # Panics
    ///
    /// Panics at compile time if `E::TABLE` does not [allow percent-encoded octets].
    ///
    /// [allow percent-encoded octets]: Table::allows_pct_encoded
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_uri::pct_enc::{encoder::Path, EStr};
    ///
    /// let dec = EStr::<Path>::new_or_panic("%C2%A1Hola%21").decode();
    /// assert_eq!(*dec.clone().to_bytes(), [0xc2, 0xa1, 0x48, 0x6f, 0x6c, 0x61, 0x21]);
    /// assert_eq!(dec.to_string().unwrap(), "¡Hola!");
    /// ```
    pub fn decode(&self) -> Decode<'_> {
        () = Self::ASSERT_ALLOWS_PCT_ENCODED;
        Decode::new(&self.inner)
    }

    /// Returns an iterator over subslices of the `EStr` slice separated by the given delimiter.
    ///
    /// # Panics
    ///
    /// Panics if the delimiter is not a [reserved] character.
    ///
    /// [reserved]: https://datatracker.ietf.org/doc/html/rfc3986#section-2.2
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_uri::pct_enc::{encoder::Path, EStr};
    ///
    /// assert!(EStr::<Path>::new_or_panic("a,b,c").split(',').eq(["a", "b", "c"]));
    /// assert!(EStr::<Path>::new_or_panic(",").split(',').eq(["", ""]));
    /// assert!(EStr::<Path>::EMPTY.split(',').eq([""]));
    /// ```
    pub fn split(&self, delim: char) -> Split<'_, E> {
        assert!(
            delim.is_ascii() && table::RESERVED.allows(delim),
            "splitting with non-reserved character"
        );
        Split {
            inner: self.inner.split(delim),
            encoder: PhantomData,
        }
    }

    /// Splits the `EStr` slice on the first occurrence of the given delimiter and
    /// returns prefix before delimiter and suffix after delimiter.
    ///
    /// Returns `None` if the delimiter is not found.
    ///
    /// # Panics
    ///
    /// Panics if the delimiter is not a [reserved] character.
    ///
    /// [reserved]: https://datatracker.ietf.org/doc/html/rfc3986#section-2.2
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_uri::pct_enc::{encoder::Path, EStr};
    ///
    /// assert_eq!(
    ///     EStr::<Path>::new_or_panic("foo;bar;baz").split_once(';'),
    ///     Some((EStr::new_or_panic("foo"), EStr::new_or_panic("bar;baz")))
    /// );
    ///
    /// assert_eq!(EStr::<Path>::new_or_panic("foo").split_once(';'), None);
    /// ```
    #[must_use]
    pub fn split_once(&self, delim: char) -> Option<(&Self, &Self)> {
        assert!(
            delim.is_ascii() && table::RESERVED.allows(delim),
            "splitting with non-reserved character"
        );
        self.inner
            .split_once(delim)
            .map(|(a, b)| (Self::new_validated(a), Self::new_validated(b)))
    }

    /// Splits the `EStr` slice on the last occurrence of the given delimiter and
    /// returns prefix before delimiter and suffix after delimiter.
    ///
    /// Returns `None` if the delimiter is not found.
    ///
    /// # Panics
    ///
    /// Panics if the delimiter is not a [reserved] character.
    ///
    /// [reserved]: https://datatracker.ietf.org/doc/html/rfc3986#section-2.2
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_uri::pct_enc::{encoder::Path, EStr};
    ///
    /// assert_eq!(
    ///     EStr::<Path>::new_or_panic("foo;bar;baz").rsplit_once(';'),
    ///     Some((EStr::new_or_panic("foo;bar"), EStr::new_or_panic("baz")))
    /// );
    ///
    /// assert_eq!(EStr::<Path>::new_or_panic("foo").rsplit_once(';'), None);
    /// ```
    #[must_use]
    pub fn rsplit_once(&self, delim: char) -> Option<(&Self, &Self)> {
        assert!(
            delim.is_ascii() && table::RESERVED.allows(delim),
            "splitting with non-reserved character"
        );
        self.inner
            .rsplit_once(delim)
            .map(|(a, b)| (Self::new_validated(a), Self::new_validated(b)))
    }
}

impl<E: Encoder> AsRef<Self> for EStr<E> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<E: Encoder> AsRef<str> for EStr<E> {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl<E: Encoder> PartialEq for EStr<E> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<E: Encoder> PartialEq<str> for EStr<E> {
    fn eq(&self, other: &str) -> bool {
        &self.inner == other
    }
}

impl<E: Encoder> PartialEq<EStr<E>> for str {
    fn eq(&self, other: &EStr<E>) -> bool {
        self == &other.inner
    }
}

impl<E: Encoder> Eq for EStr<E> {}

impl<E: Encoder> hash::Hash for EStr<E> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<E: Encoder> PartialOrd for EStr<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<E: Encoder> Ord for EStr<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<E: Encoder> Default for &EStr<E> {
    /// Creates an empty `EStr` slice.
    fn default() -> Self {
        EStr::EMPTY
    }
}

#[cfg(feature = "alloc")]
impl<E: Encoder> ToOwned for EStr<E> {
    type Owned = EString<E>;

    fn to_owned(&self) -> EString<E> {
        EString::new_validated(self.inner.to_owned())
    }

    fn clone_into(&self, target: &mut EString<E>) {
        self.inner.clone_into(&mut target.buf);
    }
}

/// Extension methods for the [path] component.
///
/// [path]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.3
impl<E: PathEncoder> EStr<E> {
    /// Checks whether the path is absolute, i.e., starting with `'/'`.
    #[inline]
    #[must_use]
    pub fn is_absolute(&self) -> bool {
        self.inner.starts_with('/')
    }

    /// Checks whether the path is rootless, i.e., not starting with `'/'`.
    #[inline]
    #[must_use]
    pub fn is_rootless(&self) -> bool {
        !self.inner.starts_with('/')
    }

    /// Returns an iterator over the path segments, separated by `'/'`.
    ///
    /// Returns `None` if the path is [rootless]. Use [`split`]
    /// instead if you need to split a rootless path on occurrences of `'/'`.
    ///
    /// Note that the path can be [empty] when authority is present,
    /// in which case this method will return `None`.
    ///
    /// [rootless]: Self::is_rootless
    /// [`split`]: Self::split
    /// [empty]: Self::is_empty
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_uri::Uri;
    ///
    /// // Segments are separated by '/'.
    /// // The empty string before a leading '/' is not a segment.
    /// // However, segments can be empty in the other cases.
    /// let path = Uri::parse("file:///path/to//dir/")?.path();
    /// assert_eq!(path, "/path/to//dir/");
    /// assert!(path.segments_if_absolute().unwrap().eq(["path", "to", "", "dir", ""]));
    ///
    /// let path = Uri::parse("foo:bar/baz")?.path();
    /// assert_eq!(path, "bar/baz");
    /// assert!(path.segments_if_absolute().is_none());
    ///
    /// let path = Uri::parse("http://example.com")?.path();
    /// assert!(path.is_empty());
    /// assert!(path.segments_if_absolute().is_none());
    /// # Ok::<_, fluent_uri::ParseError>(())
    /// ```
    #[inline]
    #[must_use]
    pub fn segments_if_absolute(&self) -> Option<Split<'_, E>> {
        self.inner
            .strip_prefix('/')
            .map(|s| Self::new_validated(s).split('/'))
    }
}

const fn gen_octet_table(hi: bool) -> [u8; 256] {
    let mut out = [0xff; 256];
    let shift = if hi { 4 } else { 0 };

    let mut i = 0;
    while i < 10 {
        out[(i + b'0') as usize] = i << shift;
        i += 1;
    }
    while i < 16 {
        out[(i - 10 + b'A') as usize] = i << shift;
        out[(i - 10 + b'a') as usize] = i << shift;
        i += 1;
    }
    out
}

const OCTET_TABLE_HI: &[u8; 256] = &gen_octet_table(true);
pub(crate) const OCTET_TABLE_LO: &[u8; 256] = &gen_octet_table(false);

/// Decodes a percent-encoded octet, assuming that the bytes are hexadecimal.
pub(crate) fn decode_octet(hi: u8, lo: u8) -> u8 {
    debug_assert!(hi.is_ascii_hexdigit() && lo.is_ascii_hexdigit());
    OCTET_TABLE_HI[hi as usize] | OCTET_TABLE_LO[lo as usize]
}

/// An iterator used to decode an [`EStr`] slice.
///
/// This struct is created by [`EStr::decode`]. Normally you'll use the methods below
/// instead of iterating over a `Decode` manually, unless you need precise control
/// over allocation.
///
/// See the [`DecodedChunk`] type for documentation of the items yielded by this iterator.
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Decode<'a> {
    source: &'a str,
}

/// An item returned by the [`Decode`] iterator.
#[derive(Clone, Copy, Debug)]
pub enum DecodedChunk<'a> {
    /// An unencoded subslice.
    Unencoded(&'a str),
    /// A percent-encoded octet, decoded (for example, `"%20"` decoded as `0x20`).
    PctDecoded(u8),
}

impl<'a> Decode<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self { source }
    }

    fn next_if_unencoded(&mut self) -> Option<&'a str> {
        let i = self
            .source
            .bytes()
            .position(|x| x == b'%')
            .unwrap_or(self.source.len());

        if i == 0 {
            None
        } else {
            let s;
            (s, self.source) = self.source.split_at(i);
            Some(s)
        }
    }
}

impl<'a> Iterator for Decode<'a> {
    type Item = DecodedChunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source.is_empty() {
            None
        } else if let Some(s) = self.next_if_unencoded() {
            Some(DecodedChunk::Unencoded(s))
        } else {
            let s;
            (s, self.source) = self.source.split_at(3);
            let x = decode_octet(s.as_bytes()[1], s.as_bytes()[2]);
            Some(DecodedChunk::PctDecoded(x))
        }
    }
}

impl FusedIterator for Decode<'_> {}

#[cfg(feature = "alloc")]
pub(crate) enum DecodedUtf8Chunk<'a, 'b> {
    Unencoded(&'a str),
    Decoded { valid: &'b str, invalid: &'b [u8] },
}

#[cfg(feature = "alloc")]
impl<'a> Decode<'a> {
    pub(crate) fn decode_utf8(self, mut handle_chunk: impl FnMut(DecodedUtf8Chunk<'a, '_>)) {
        use crate::utf8::Utf8Chunks;

        let mut buf = [0; 32];
        let mut len = 0;

        'decode: for chunk in self {
            match chunk {
                DecodedChunk::Unencoded(s) => {
                    if len > 0 {
                        for chunk in Utf8Chunks::new(&buf[..len]) {
                            handle_chunk(DecodedUtf8Chunk::Decoded {
                                valid: chunk.valid(),
                                invalid: chunk.invalid(),
                            });
                        }
                        len = 0;
                    }
                    handle_chunk(DecodedUtf8Chunk::Unencoded(s));
                }
                DecodedChunk::PctDecoded(x) => {
                    buf[len] = x;
                    len += 1;

                    if len >= buf.len() {
                        for chunk in Utf8Chunks::new(&buf[..len]) {
                            if chunk.incomplete() {
                                handle_chunk(DecodedUtf8Chunk::Decoded {
                                    valid: chunk.valid(),
                                    invalid: &[],
                                });

                                let invalid_len = chunk.invalid().len();
                                buf.copy_within(len - invalid_len..len, 0);

                                len = invalid_len;
                                continue 'decode;
                            }
                            handle_chunk(DecodedUtf8Chunk::Decoded {
                                valid: chunk.valid(),
                                invalid: chunk.invalid(),
                            });
                        }
                        len = 0;
                    }
                }
            }
        }

        for chunk in Utf8Chunks::new(&buf[..len]) {
            handle_chunk(DecodedUtf8Chunk::Decoded {
                valid: chunk.valid(),
                invalid: chunk.invalid(),
            });
        }
    }

    fn decoded_len(&self) -> usize {
        self.source.len() - self.source.bytes().filter(|&x| x == b'%').count() * 2
    }

    fn borrow_all_or_prep_buf(&mut self) -> Result<&'a str, String> {
        if let Some(s) = self.next_if_unencoded() {
            if self.source.is_empty() {
                return Ok(s);
            }
            let mut buf = String::with_capacity(s.len() + self.decoded_len());
            buf.push_str(s);
            Err(buf)
        } else {
            Err(String::with_capacity(self.decoded_len()))
        }
    }

    /// Decodes the slice to bytes.
    ///
    /// This method allocates only when the slice contains any percent-encoded octet.
    #[must_use]
    pub fn to_bytes(mut self) -> Cow<'a, [u8]> {
        if self.source.is_empty() {
            return Cow::Borrowed(&[]);
        }

        let mut buf = match self.borrow_all_or_prep_buf() {
            Ok(s) => return Cow::Borrowed(s.as_bytes()),
            Err(buf) => buf.into_bytes(),
        };

        for chunk in self {
            match chunk {
                DecodedChunk::Unencoded(s) => buf.extend_from_slice(s.as_bytes()),
                DecodedChunk::PctDecoded(s) => buf.push(s),
            }
        }
        Cow::Owned(buf)
    }

    /// Attempts to decode the slice to a string.
    ///
    /// This method allocates only when the slice contains any percent-encoded octet.
    ///
    /// # Errors
    ///
    /// Returns `Err` containing the decoded bytes if they are not valid UTF-8.
    pub fn to_string(mut self) -> Result<Cow<'a, str>, Vec<u8>> {
        if self.source.is_empty() {
            return Ok(Cow::Borrowed(""));
        }

        let mut buf = match self.borrow_all_or_prep_buf() {
            Ok(s) => return Ok(Cow::Borrowed(s)),
            Err(buf) => Ok::<_, Vec<u8>>(buf),
        };

        self.decode_utf8(|chunk| match chunk {
            DecodedUtf8Chunk::Unencoded(s) => match &mut buf {
                Ok(string) => string.push_str(s),
                Err(vec) => vec.extend_from_slice(s.as_bytes()),
            },
            DecodedUtf8Chunk::Decoded { valid, invalid } => match &mut buf {
                Ok(string) => {
                    string.push_str(valid);
                    if !invalid.is_empty() {
                        let mut vec = core::mem::take(string).into_bytes();
                        vec.extend_from_slice(invalid);
                        buf = Err(vec);
                    }
                }
                Err(vec) => {
                    vec.extend_from_slice(valid.as_bytes());
                    vec.extend_from_slice(invalid);
                }
            },
        });

        match buf {
            Ok(buf) => Ok(Cow::Owned(buf)),
            Err(buf) => Err(buf),
        }
    }

    /// Decodes the slice to a string, replacing any invalid UTF-8 sequences with
    /// [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD].
    ///
    /// [U+FFFD]: char::REPLACEMENT_CHARACTER
    ///
    /// This method allocates only when the slice contains any percent-encoded octet.
    #[must_use]
    pub fn to_string_lossy(mut self) -> Cow<'a, str> {
        if self.source.is_empty() {
            return Cow::Borrowed("");
        }

        let mut buf = match self.borrow_all_or_prep_buf() {
            Ok(s) => return Cow::Borrowed(s),
            Err(buf) => buf,
        };

        self.decode_utf8(|chunk| match chunk {
            DecodedUtf8Chunk::Unencoded(s) => buf.push_str(s),
            DecodedUtf8Chunk::Decoded { valid, invalid } => {
                buf.push_str(valid);
                if !invalid.is_empty() {
                    buf.push(char::REPLACEMENT_CHARACTER);
                }
            }
        });
        Cow::Owned(buf)
    }
}

pub(crate) fn encode_byte(x: u8) -> &'static str {
    const TABLE: &[u8; 256 * 3] = &{
        const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

        let mut i = 0;
        let mut table = [0; 256 * 3];
        while i < 256 {
            table[i * 3] = b'%';
            table[i * 3 + 1] = HEX_DIGITS[i >> 4];
            table[i * 3 + 2] = HEX_DIGITS[i & 0b1111];
            i += 1;
        }
        table
    };

    const TABLE_STR: &str = match str::from_utf8(TABLE) {
        Ok(s) => s,
        Err(_) => unreachable!(),
    };

    &TABLE_STR[x as usize * 3..x as usize * 3 + 3]
}

/// An iterator used to percent-encode a string slice.
///
/// This struct is created by [`Table::encode`]. Normally you'll use [`EString::encode_str`]
/// instead, unless you need precise control over allocation.
///
/// See the [`EncodedChunk`] type for documentation of the items yielded by this iterator.
#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub(crate) struct Encode<'t, 's> {
    table: &'t Table,
    source: &'s str,
    enc_len: usize,
    enc_i: usize,
}

#[cfg(feature = "alloc")]
impl<'t, 's> Encode<'t, 's> {
    pub(crate) fn new(table: &'t Table, source: &'s str) -> Self {
        Self {
            table,
            source,
            enc_len: 0,
            enc_i: 0,
        }
    }
}

/// An item returned by the [`Encode`] iterator.
#[cfg(feature = "alloc")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EncodedChunk<'a> {
    /// An unencoded subslice.
    Unencoded(&'a str),
    /// A byte, percent-encoded (for example, `0x20` encoded as `"%20"`).
    PctEncoded(&'static str),
}

#[cfg(feature = "alloc")]
impl<'a> EncodedChunk<'a> {
    /// Returns the chunk as a string slice.
    #[must_use]
    pub fn as_str(self) -> &'a str {
        match self {
            Self::Unencoded(s) | Self::PctEncoded(s) => s,
        }
    }
}

#[cfg(feature = "alloc")]
impl<'t, 's> Iterator for Encode<'t, 's> {
    type Item = EncodedChunk<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.enc_i < self.enc_len {
            let s = encode_byte(self.source.as_bytes()[self.enc_i]);
            self.enc_i += 1;
            return Some(EncodedChunk::PctEncoded(s));
        }

        self.source = &self.source[self.enc_len..];
        self.enc_len = 0;

        if self.source.is_empty() {
            return None;
        }

        let mut iter = self.source.char_indices();
        let i = iter
            .find_map(|(i, ch)| (!self.table.allows(ch)).then_some(i))
            .unwrap_or(self.source.len());

        // `CharIndices::offset` sadly requires an MSRV of 1.82,
        // so we do pointer math to get the offset for now.
        if i == 0 {
            self.enc_len = iter.as_str().as_ptr() as usize - self.source.as_ptr() as usize;
            self.enc_i = 1;

            let s = encode_byte(self.source.as_bytes()[0]);
            Some(EncodedChunk::PctEncoded(s))
        } else {
            let s;
            (s, self.source) = self.source.split_at(i);

            self.enc_len = iter.as_str().as_ptr() as usize - self.source.as_ptr() as usize;
            self.enc_i = 0;

            Some(EncodedChunk::Unencoded(s))
        }
    }
}

#[cfg(feature = "alloc")]
impl FusedIterator for Encode<'_, '_> {}

/// An iterator over subslices of an [`EStr`] slice separated by a delimiter.
///
/// This struct is created by [`EStr::split`].
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Split<'a, E: Encoder> {
    inner: str::Split<'a, char>,
    encoder: PhantomData<E>,
}

impl<'a, E: Encoder> Iterator for Split<'a, E> {
    type Item = &'a EStr<E>;

    fn next(&mut self) -> Option<&'a EStr<E>> {
        self.inner.next().map(EStr::new_validated)
    }
}

impl<'a, E: Encoder> DoubleEndedIterator for Split<'a, E> {
    fn next_back(&mut self) -> Option<&'a EStr<E>> {
        self.inner.next_back().map(EStr::new_validated)
    }
}

impl<E: Encoder> FusedIterator for Split<'_, E> {}
