//! Functions for encoding into Base58 encoded strings.

use core::fmt;

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

use crate::Check;
#[cfg(any(feature = "check", feature = "cb58"))]
use crate::CHECKSUM_LEN;

use crate::Alphabet;

/// A builder for setting up the alphabet and output of a base58 encode.
#[allow(missing_debug_implementations)]
pub struct EncodeBuilder<'a, I: AsRef<[u8]>> {
    input: I,
    alpha: &'a Alphabet,
    check: Check,
}

/// A specialized [`Result`](core::result::Result) type for [`bs58::encode`](module@crate::encode)
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that could occur when encoding a Base58 encoded string.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// The output buffer was too small to contain the entire input.
    BufferTooSmall,
}

/// Represents a buffer that can be encoded into. See [`EncodeBuilder::onto`] and the provided
/// implementations for more details.
pub trait EncodeTarget {
    /// Encodes into this buffer, provides the maximum length for implementations that wish to
    /// preallocate space, along with a function that will encode ASCII bytes into the buffer and
    /// return the length written to it.
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize>;
}

impl<T: EncodeTarget + ?Sized> EncodeTarget for &mut T {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        T::encode_with(self, max_len, f)
    }
}

#[cfg(feature = "alloc")]
impl EncodeTarget for Vec<u8> {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let original = self.len();
        self.resize(original + max_len, 0);
        let len = f(&mut self[original..])?;
        self.truncate(original + len);
        Ok(len)
    }
}

#[cfg(feature = "smallvec")]
impl<A: smallvec::Array<Item = u8>> EncodeTarget for smallvec::SmallVec<A> {
    /// Encodes data into a [`smallvec::SmallVec`].
    ///
    /// Note that even if the encoded value fits into vector’s inline buffer,
    /// this may result in allocation if `max_len` is greater than vector’s
    /// inline size.  To make sure that the inline buffer is enough for N-byte
    /// buffer encoded in base58, use smallvec with ⌈N*1.5⌉-byte long inline
    /// buffer (or ⌈(N+5)*1.5⌉ if version and checksum are included).
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let original = self.len();
        self.resize(original + max_len, 0);
        let len = f(&mut self[original..])?;
        self.truncate(original + len);
        Ok(len)
    }
}

#[cfg(feature = "tinyvec")]
impl<A: tinyvec::Array<Item = u8>> EncodeTarget for tinyvec::ArrayVec<A> {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let _ = max_len;
        let original = self.len();
        let len = f(self.grab_spare_slice_mut())?;
        self.set_len(original + len);
        Ok(len)
    }
}

#[cfg(feature = "tinyvec")]
impl EncodeTarget for tinyvec::SliceVec<'_, u8> {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let _ = max_len;
        let original = self.len();
        let len = f(self.grab_spare_slice_mut())?;
        self.set_len(original + len);
        Ok(len)
    }
}

#[cfg(all(feature = "tinyvec", feature = "alloc"))]
impl<A: tinyvec::Array<Item = u8>> EncodeTarget for tinyvec::TinyVec<A> {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let original = self.len();
        self.resize(original + max_len, 0);
        let len = f(&mut self[original..])?;
        self.truncate(original + len);
        Ok(len)
    }
}

#[cfg(feature = "alloc")]
impl EncodeTarget for String {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let mut output = core::mem::take(self).into_bytes();
        let len = output.encode_with(max_len, f)?;
        *self = String::from_utf8(output).unwrap();
        Ok(len)
    }
}

impl EncodeTarget for [u8] {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let _ = max_len;
        f(&mut *self)
    }
}

impl EncodeTarget for str {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        struct Guard<'a>(&'a mut [u8]);

        impl Drop for Guard<'_> {
            fn drop(&mut self) {
                let mut index = 0;
                loop {
                    match core::str::from_utf8(&self.0[index..]) {
                        Ok(_) => return,
                        Err(e) => {
                            index += e.valid_up_to();
                            if let Some(len) = e.error_len() {
                                for i in &mut self.0[index..index + len] {
                                    *i = 0;
                                }
                                index += len;
                            } else {
                                for i in &mut self.0[index..] {
                                    *i = 0;
                                }
                                index += self.0[index..].len();
                            }
                        }
                    }
                }
            }
        }

        let _ = max_len;

        #[allow(unsafe_code)]
        // SAFETY: before returning the guard will be dropped and ensure the slice is valid utf-8
        // by replacing invalid bytes with nul-bytes
        let guard = Guard(unsafe { self.as_bytes_mut() });
        f(&mut *guard.0)
    }
}

impl<'a, I: AsRef<[u8]>> EncodeBuilder<'a, I> {
    /// Setup encoder for the given string using the given alphabet.
    /// Preferably use [`bs58::encode`](crate::encode()) instead of this
    /// directly.
    pub fn new(input: I, alpha: &'a Alphabet) -> EncodeBuilder<'a, I> {
        EncodeBuilder {
            input,
            alpha,
            check: Check::Disabled,
        }
    }

    /// Setup encoder for the given string using default prepared alphabet.
    pub(crate) fn from_input(input: I) -> EncodeBuilder<'static, I> {
        EncodeBuilder {
            input,
            alpha: Alphabet::DEFAULT,
            check: Check::Disabled,
        }
    }

    /// Change the alphabet that will be used for encoding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let input = [0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78];
    /// assert_eq!(
    ///     "he11owor1d",
    ///     bs58::encode(input)
    ///         .with_alphabet(bs58::Alphabet::RIPPLE)
    ///         .into_string());
    /// ```
    pub fn with_alphabet(self, alpha: &'a Alphabet) -> EncodeBuilder<'a, I> {
        EncodeBuilder { alpha, ..self }
    }

    /// Include checksum calculated using the [Base58Check][] algorithm when
    /// encoding.
    ///
    /// [Base58Check]: https://en.bitcoin.it/wiki/Base58Check_encoding
    ///
    /// # Examples
    ///
    /// ```rust
    /// let input = [0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78];
    /// assert_eq!(
    ///     "QuT57JNzzWTu7mW",
    ///     bs58::encode(input)
    ///         .with_check()
    ///         .into_string());
    /// ```
    #[cfg(feature = "check")]
    pub fn with_check(self) -> EncodeBuilder<'a, I> {
        let check = Check::Enabled(None);
        EncodeBuilder { check, ..self }
    }

    /// Include checksum calculated using the [Base58Check][] algorithm and
    /// version when encoding.
    ///
    /// [Base58Check]: https://en.bitcoin.it/wiki/Base58Check_encoding
    ///
    /// # Examples
    ///
    /// ```rust
    /// let input = [0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78];
    /// assert_eq!(
    ///     "oP8aA4HEEyFxxYhp",
    ///     bs58::encode(input)
    ///         .with_check_version(42)
    ///         .into_string());
    /// ```
    #[cfg(feature = "check")]
    pub fn with_check_version(self, expected_ver: u8) -> EncodeBuilder<'a, I> {
        let check = Check::Enabled(Some(expected_ver));
        EncodeBuilder { check, ..self }
    }

    /// Include checksum calculated using the [CB58][] algorithm and
    /// version (if specified) when encoding.
    ///
    /// [CB58]: https://support.avax.network/en/articles/4587395-what-is-cb58
    ///
    /// # Examples
    ///
    /// ```rust
    /// let input = [0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78];
    /// assert_eq!(
    ///     "oP8aA4HEEyChXhM2",
    ///     bs58::encode(input)
    ///         .as_cb58(Some(42))
    ///         .into_string());
    /// ```
    #[cfg(feature = "cb58")]
    pub fn as_cb58(self, expected_ver: Option<u8>) -> EncodeBuilder<'a, I> {
        let check = Check::CB58(expected_ver);
        EncodeBuilder { check, ..self }
    }

    /// Encode into a new owned string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// assert_eq!("he11owor1d", bs58::encode(input).into_string());
    /// ```
    #[cfg(feature = "alloc")]
    pub fn into_string(self) -> String {
        let mut output = String::new();
        self.onto(&mut output).unwrap();
        output
    }

    /// Encode into a new owned vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// assert_eq!(b"he11owor1d", &*bs58::encode(input).into_vec());
    /// ```
    #[cfg(feature = "alloc")]
    pub fn into_vec(self) -> Vec<u8> {
        let mut output = Vec::new();
        self.onto(&mut output).unwrap();
        output
    }

    /// Encode onto the given buffer.
    ///
    /// Returns the length written onto the buffer.
    ///
    /// If the buffer is resizeable it will be extended and the new data will be written to the end
    /// of it, otherwise the data will be overwritten from the start.
    ///
    /// If the buffer is not resizeable bytes after the final character will be left alone, except
    /// up to 3 null bytes may be written to an `&mut str` to overwrite remaining characters of a
    /// partially overwritten multi-byte character.
    ///
    /// See the documentation for [`bs58::encode`](crate::encode()) for an
    /// explanation of the errors that may occur.
    ///
    /// # Examples
    ///
    /// ## `Vec<u8>`
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// let mut output = b"goodbye world ".to_vec();
    /// bs58::encode(input).onto(&mut output)?;
    /// assert_eq!(b"goodbye world he11owor1d", output.as_slice());
    /// # Ok::<(), bs58::encode::Error>(())
    /// ```
    ///
    /// ## `&mut [u8]`
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// let mut output = b"goodbye world".to_owned();
    /// bs58::encode(input).onto(&mut output[..])?;
    /// assert_eq!(b"he11owor1drld", output.as_ref());
    /// # Ok::<(), bs58::encode::Error>(())
    /// ```
    ///
    /// ## `String`
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// let mut output = "goodbye world ".to_owned();
    /// bs58::encode(input).onto(&mut output)?;
    /// assert_eq!("goodbye world he11owor1d", output);
    /// # Ok::<(), bs58::encode::Error>(())
    /// ```
    ///
    /// ## `&mut str`
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// let mut output = "goodbye world".to_owned();
    /// bs58::encode(input).onto(output.as_mut_str())?;
    /// assert_eq!("he11owor1drld", output);
    /// # Ok::<(), bs58::encode::Error>(())
    /// ```
    ///
    /// ### Clearing partially overwritten characters
    ///
    /// ```rust
    /// let input = [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58];
    /// let mut output = "goodbye w®ld".to_owned();
    /// bs58::encode(input).onto(output.as_mut_str())?;
    /// assert_eq!("he11owor1d\0ld", output);
    /// # Ok::<(), bs58::encode::Error>(())
    /// ```
    pub fn onto(self, mut output: impl EncodeTarget) -> Result<usize> {
        let input = self.input.as_ref();
        match self.check {
            Check::Disabled => output.encode_with(max_encoded_len(input.len()), |output| {
                encode_into(input, output, self.alpha)
            }),
            #[cfg(feature = "check")]
            Check::Enabled(version) => {
                let input_len = input.len() + CHECKSUM_LEN + version.map_or(0, |_| 1);
                output.encode_with(max_encoded_len(input_len), |output| {
                    encode_check_into(self.input.as_ref(), output, self.alpha, version)
                })
            }
            #[cfg(feature = "cb58")]
            Check::CB58(version) => {
                let input_len = input.len() + CHECKSUM_LEN + version.map_or(0, |_| 1);
                output.encode_with(max_encoded_len(input_len), |output| {
                    encode_cb58_into(self.input.as_ref(), output, self.alpha, version)
                })
            }
        }
    }
}

/// Return maximum possible encoded length of a buffer with given length.
///
/// Assumes that the `len` already includes version and checksum bytes if those
/// are
fn max_encoded_len(len: usize) -> usize {
    // log_2(256) / log_2(58) ≈ 1.37.  Assume 1.5 for easier calculation.
    len + (len + 1) / 2
}

fn encode_into<'a, I>(input: I, output: &mut [u8], alpha: &Alphabet) -> Result<usize>
where
    I: Clone + IntoIterator<Item = &'a u8>,
{
    let mut index = 0;
    for &val in input.clone() {
        let mut carry = val as usize;
        for byte in &mut output[..index] {
            carry += (*byte as usize) << 8;
            *byte = (carry % 58) as u8;
            carry /= 58;
        }
        while carry > 0 {
            if index == output.len() {
                return Err(Error::BufferTooSmall);
            }
            output[index] = (carry % 58) as u8;
            index += 1;
            carry /= 58;
        }
    }

    for _ in input.into_iter().take_while(|v| **v == 0) {
        if index == output.len() {
            return Err(Error::BufferTooSmall);
        }
        output[index] = 0;
        index += 1;
    }

    for val in &mut output[..index] {
        *val = alpha.encode[*val as usize];
    }

    output[..index].reverse();
    Ok(index)
}

#[cfg(feature = "check")]
fn encode_check_into(
    input: &[u8],
    output: &mut [u8],
    alpha: &Alphabet,
    version: Option<u8>,
) -> Result<usize> {
    use sha2::{Digest, Sha256};

    let mut first_hash = Sha256::new();
    if let Some(version) = version {
        first_hash.update([version; 1]);
    }
    let first_hash = first_hash.chain_update(input).finalize();
    let second_hash = Sha256::digest(first_hash);

    let checksum = &second_hash[0..CHECKSUM_LEN];

    encode_into(
        version.iter().chain(input.iter()).chain(checksum.iter()),
        output,
        alpha,
    )
}

#[cfg(feature = "cb58")]
fn encode_cb58_into(
    input: &[u8],
    output: &mut [u8],
    alpha: &Alphabet,
    version: Option<u8>,
) -> Result<usize> {
    use sha2::{Digest, Sha256};

    let mut hash = Sha256::new();
    if let Some(version) = version {
        hash.update([version; 1]);
    }
    let hash = hash.chain_update(input).finalize();

    let checksum = &hash[hash.len() - CHECKSUM_LEN..];

    encode_into(
        version.iter().chain(input.iter()).chain(checksum.iter()),
        output,
        alpha,
    )
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BufferTooSmall => write!(
                f,
                "buffer provided to encode base58 string into was too small"
            ),
        }
    }
}
