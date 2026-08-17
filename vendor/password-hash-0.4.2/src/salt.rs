//! Salt string support.

use crate::{Encoding, Error, Result, Value};
use core::{fmt, str};

use crate::errors::InvalidValue;
#[cfg(feature = "rand_core")]
use rand_core::{CryptoRng, RngCore};

/// Error message used with `expect` for when internal invariants are violated
/// (i.e. the contents of a [`Salt`] should always be valid)
const INVARIANT_VIOLATED_MSG: &str = "salt string invariant violated";

/// Salt string.
///
/// In password hashing, a "salt" is an additional value used to
/// personalize/tweak the output of a password hashing function for a given
/// input password.
///
/// Salts help defend against attacks based on precomputed tables of hashed
/// passwords, i.e. "[rainbow tables][1]".
///
/// The [`Salt`] type implements the RECOMMENDED best practices for salts
/// described in the [PHC string format specification][2], namely:
///
/// > - Maximum lengths for salt, output and parameter values are meant to help
/// >   consumer implementations, in particular written in C and using
/// >   stack-allocated buffers. These buffers must account for the worst case,
/// >   i.e. the maximum defined length. Therefore, keep these lengths low.
/// > - The role of salts is to achieve uniqueness. A random salt is fine for
/// >   that as long as its length is sufficient; a 16-byte salt would work well
/// >   (by definition, UUID are very good salts, and they encode over exactly
/// >   16 bytes). 16 bytes encode as 22 characters in B64. Functions should
/// >   disallow salt values that are too small for security (4 bytes should be
/// >   viewed as an absolute minimum).
///
/// # Recommended length
/// The recommended default length for a salt string is **16-bytes** (128-bits).
///
/// See [`Salt::RECOMMENDED_LENGTH`] for more information.
///
/// # Constraints
/// Salt strings are constrained to the following set of characters per the
/// PHC spec:
///
/// > The salt consists in a sequence of characters in: `[a-zA-Z0-9/+.-]`
/// > (lowercase letters, uppercase letters, digits, /, +, . and -).
///
/// Additionally the following length restrictions are enforced based on the
/// guidelines from the spec:
///
/// - Minimum length: **4**-bytes
/// - Maximum length: **64**-bytes
///
/// A maximum length is enforced based on the above recommendation for
/// supporting stack-allocated buffers (which this library uses), and the
/// specific determination of 64-bytes is taken as a best practice from the
/// [Argon2 Encoding][3] specification in the same document:
///
/// > The length in bytes of the salt is between 8 and 64 bytes<sup>†</sup>, thus
/// > yielding a length in characters between 11 and 64 characters (and that
/// > length is never equal to 1 modulo 4). The default byte length of the salt
/// > is 16 bytes (22 characters in B64 encoding). An encoded UUID, or a
/// > sequence of 16 bytes produced with a cryptographically strong PRNG, are
/// > appropriate salt values.
/// >
/// > <sup>†</sup>The Argon2 specification states that the salt can be much longer, up
/// > to 2^32-1 bytes, but this makes little sense for password hashing.
/// > Specifying a relatively small maximum length allows for parsing with a
/// > stack allocated buffer.)
///
/// Based on this guidance, this type enforces an upper bound of 64-bytes
/// as a reasonable maximum, and recommends using 16-bytes.
///
/// [1]: https://en.wikipedia.org/wiki/Rainbow_table
/// [2]: https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#function-duties
/// [3]: https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#argon2-encoding
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Salt<'a>(Value<'a>);

#[allow(clippy::len_without_is_empty)]
impl<'a> Salt<'a> {
    /// Minimum length of a [`Salt`] string: 4-bytes.
    pub const MIN_LENGTH: usize = 4;

    /// Maximum length of a [`Salt`] string: 64-bytes.
    ///
    /// See type-level documentation about [`Salt`] for more information.
    pub const MAX_LENGTH: usize = 64;

    /// Recommended length of a salt: 16-bytes.
    ///
    /// This recommendation comes from the [PHC string format specification]:
    ///
    /// > The role of salts is to achieve uniqueness. A *random* salt is fine
    /// > for that as long as its length is sufficient; a 16-byte salt would
    /// > work well (by definition, UUID are very good salts, and they encode
    /// > over exactly 16 bytes). 16 bytes encode as 22 characters in B64.
    ///
    /// [PHC string format specification]: https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#function-duties
    pub const RECOMMENDED_LENGTH: usize = 16;

    /// Create a [`Salt`] from the given `str`, validating it according to
    /// [`Salt::MIN_LENGTH`] and [`Salt::MAX_LENGTH`] length restrictions.
    pub fn new(input: &'a str) -> Result<Self> {
        let length = input.as_bytes().len();

        if length < Self::MIN_LENGTH {
            return Err(Error::SaltInvalid(InvalidValue::TooShort));
        }

        if length > Self::MAX_LENGTH {
            return Err(Error::SaltInvalid(InvalidValue::TooLong));
        }

        input.try_into().map(Self).map_err(|e| match e {
            Error::ParamValueInvalid(value_err) => Error::SaltInvalid(value_err),
            err => err,
        })
    }

    /// Attempt to decode a B64-encoded [`Salt`], writing the decoded result
    /// into the provided buffer, and returning a slice of the buffer
    /// containing the decoded result on success.
    ///
    /// [1]: https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#argon2-encoding
    pub fn b64_decode<'b>(&self, buf: &'b mut [u8]) -> Result<&'b [u8]> {
        self.0.b64_decode(buf)
    }

    /// Borrow this value as a `str`.
    pub fn as_str(&self) -> &'a str {
        self.0.as_str()
    }

    /// Borrow this value as bytes.
    pub fn as_bytes(&self) -> &'a [u8] {
        self.as_str().as_bytes()
    }

    /// Get the length of this value in ASCII characters.
    pub fn len(&self) -> usize {
        self.as_str().len()
    }
}

impl<'a> AsRef<str> for Salt<'a> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> TryFrom<&'a str> for Salt<'a> {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self> {
        Self::new(input)
    }
}

impl<'a> fmt::Display for Salt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a> fmt::Debug for Salt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Salt({:?})", self.as_str())
    }
}

/// Owned stack-allocated equivalent of [`Salt`].
#[derive(Clone, Eq)]
pub struct SaltString {
    /// Byte array containing an ASCiI-encoded string.
    bytes: [u8; Salt::MAX_LENGTH],

    /// Length of the string in ASCII characters (i.e. bytes).
    length: u8,
}

#[allow(clippy::len_without_is_empty)]
impl SaltString {
    /// Generate a random B64-encoded [`SaltString`].
    #[cfg(feature = "rand_core")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rand_core")))]
    pub fn generate(mut rng: impl CryptoRng + RngCore) -> Self {
        let mut bytes = [0u8; Salt::RECOMMENDED_LENGTH];
        rng.fill_bytes(&mut bytes);
        Self::b64_encode(&bytes).expect(INVARIANT_VIOLATED_MSG)
    }

    /// Create a new [`SaltString`].
    pub fn new(s: &str) -> Result<Self> {
        // Assert `s` parses successfully as a `Salt`
        Salt::new(s)?;

        let length = s.as_bytes().len();

        if length < Salt::MAX_LENGTH {
            let mut bytes = [0u8; Salt::MAX_LENGTH];
            bytes[..length].copy_from_slice(s.as_bytes());
            Ok(SaltString {
                bytes,
                length: length as u8,
            })
        } else {
            Err(Error::SaltInvalid(InvalidValue::TooLong))
        }
    }

    /// Encode the given byte slice as B64 into a new [`SaltString`].
    ///
    /// Returns `None` if the slice is too long.
    pub fn b64_encode(input: &[u8]) -> Result<Self> {
        let mut bytes = [0u8; Salt::MAX_LENGTH];
        let length = Encoding::B64.encode(input, &mut bytes)?.len() as u8;
        Ok(Self { bytes, length })
    }

    /// Decode this [`SaltString`] from B64 into the provided output buffer.
    pub fn b64_decode<'a>(&self, buf: &'a mut [u8]) -> Result<&'a [u8]> {
        self.as_salt().b64_decode(buf)
    }

    /// Borrow the contents of a [`SaltString`] as a [`Salt`].
    pub fn as_salt(&self) -> Salt<'_> {
        Salt::new(self.as_str()).expect(INVARIANT_VIOLATED_MSG)
    }

    /// Borrow the contents of a [`SaltString`] as a `str`.
    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.bytes[..(self.length as usize)]).expect(INVARIANT_VIOLATED_MSG)
    }

    /// Borrow this value as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    /// Get the length of this value in ASCII characters.
    pub fn len(&self) -> usize {
        self.as_str().len()
    }
}

impl AsRef<str> for SaltString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq for SaltString {
    fn eq(&self, other: &Self) -> bool {
        // Ensure comparisons always honor the initialized portion of the buffer
        self.as_ref().eq(other.as_ref())
    }
}

impl<'a> From<&'a SaltString> for Salt<'a> {
    fn from(salt_string: &'a SaltString) -> Salt<'a> {
        salt_string.as_salt()
    }
}

impl fmt::Display for SaltString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Debug for SaltString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SaltString({:?})", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Salt};
    use crate::errors::InvalidValue;

    #[test]
    fn new_with_valid_min_length_input() {
        let s = "abcd";
        let salt = Salt::new(s).unwrap();
        assert_eq!(salt.as_ref(), s);
    }

    #[test]
    fn new_with_valid_max_length_input() {
        let s = "012345678911234567892123456789312345678941234567";
        let salt = Salt::new(s).unwrap();
        assert_eq!(salt.as_ref(), s);
    }

    #[test]
    fn reject_new_too_short() {
        for &too_short in &["", "a", "ab", "abc"] {
            let err = Salt::new(too_short).err().unwrap();
            assert_eq!(err, Error::SaltInvalid(InvalidValue::TooShort));
        }
    }

    #[test]
    fn reject_new_too_long() {
        let s = "01234567891123456789212345678931234567894123456785234567896234567";
        let err = Salt::new(s).err().unwrap();
        assert_eq!(err, Error::SaltInvalid(InvalidValue::TooLong));
    }

    #[test]
    fn reject_new_invalid_char() {
        let s = "01234_abcd";
        let err = Salt::new(s).err().unwrap();
        assert_eq!(err, Error::SaltInvalid(InvalidValue::InvalidChar('_')));
    }
}
