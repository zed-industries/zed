//! Outputs from password hashing functions.

use crate::{Encoding, Error, Result};
use core::{cmp::PartialEq, fmt, str::FromStr};
use subtle::{Choice, ConstantTimeEq};

/// Output from password hashing functions, i.e. the "hash" or "digest"
/// as raw bytes.
///
/// The [`Output`] type implements the RECOMMENDED best practices described in
/// the [PHC string format specification][1], namely:
///
/// > The hash output, for a verification, must be long enough to make preimage
/// > attacks at least as hard as password guessing. To promote wide acceptance,
/// > a default output size of 256 bits (32 bytes, encoded as 43 characters) is
/// > recommended. Function implementations SHOULD NOT allow outputs of less
/// > than 80 bits to be used for password verification.
///
/// # Recommended length
/// Per the description above, the recommended default length for an [`Output`]
/// of a password hashing function is **32-bytes** (256-bits).
///
/// # Constraints
/// The above guidelines are interpreted into the following constraints:
///
/// - Minimum length: **10**-bytes (80-bits)
/// - Maximum length: **64**-bytes (512-bits)
///
/// The specific recommendation of a 64-byte maximum length is taken as a best
/// practice from the hash output guidelines for [Argon2 Encoding][2] given in
/// the same document:
///
/// > The hash output...length shall be between 12 and 64 bytes (16 and 86
/// > characters, respectively). The default output length is 32 bytes
/// > (43 characters).
///
/// Based on this guidance, this type enforces an upper bound of 64-bytes
/// as a reasonable maximum, and recommends using 32-bytes.
///
/// # Constant-time comparisons
/// The [`Output`] type impls the [`ConstantTimeEq`] trait from the [`subtle`]
/// crate and uses it to perform constant-time comparisons.
///
/// Additionally the [`PartialEq`] and [`Eq`] trait impls for [`Output`] use
/// [`ConstantTimeEq`] when performing comparisons.
///
/// ## Attacks on non-constant-time password hash comparisons
/// Comparing password hashes in constant-time is known to mitigate at least
/// one [poorly understood attack][3] involving an adversary with the following
/// knowledge/capabilities:
///
/// - full knowledge of what password hashing algorithm is being used
///   including any relevant configurable parameters
/// - knowledge of the salt for a particular victim
/// - ability to accurately measure a timing side-channel on comparisons
///   of the password hash over the network
///
/// An attacker with the above is able to perform an offline computation of
/// the hash for any chosen password in such a way that it will match the
/// hash computed by the server.
///
/// As noted above, they also measure timing variability in the server's
/// comparison of the hash it computes for a given password and a target hash
/// the attacker is trying to learn.
///
/// When the attacker observes a hash comparison that takes longer than their
/// previous attempts, they learn that they guessed another byte in the
/// password hash correctly. They can leverage repeated measurements and
/// observations with different candidate passwords to learn the password
/// hash a byte-at-a-time in a manner similar to other such timing side-channel
/// attacks.
///
/// The attack may seem somewhat counterintuitive since learning prefixes of a
/// password hash does not reveal any additional information about the password
/// itself. However, the above can be combined with an offline dictionary
/// attack where the attacker is able to determine candidate passwords to send
/// to the server by performing a brute force search offline and selecting
/// candidate passwords whose hashes match the portion of the prefix they have
/// learned so far.
///
/// As the attacker learns a longer and longer prefix of the password hash,
/// they are able to more effectively eliminate candidate passwords offline as
/// part of a dictionary attack, until they eventually guess the correct
/// password or exhaust their set of candidate passwords.
///
/// ## Mitigations
/// While we have taken care to ensure password hashes are compared in constant
/// time, we would also suggest preventing such attacks by using randomly
/// generated salts and keeping those salts secret.
///
/// The [`SaltString::generate`][`crate::SaltString::generate`] function can be
/// used to generate random high-entropy salt values.
///
/// [1]: https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#function-duties
/// [2]: https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#argon2-encoding
/// [3]: https://web.archive.org/web/20130208100210/http://security-assessment.com/files/documents/presentations/TimingAttackPresentation2012.pdf
#[derive(Copy, Clone, Eq)]
pub struct Output {
    /// Byte array containing a password hashing function output.
    bytes: [u8; Self::MAX_LENGTH],

    /// Length of the password hashing function output in bytes.
    length: u8,

    /// Encoding which output should be serialized with.
    encoding: Encoding,
}

#[allow(clippy::len_without_is_empty)]
impl Output {
    /// Minimum length of a [`Output`] string: 10-bytes.
    pub const MIN_LENGTH: usize = 10;

    /// Maximum length of [`Output`] string: 64-bytes.
    ///
    /// See type-level documentation about [`Output`] for more information.
    pub const MAX_LENGTH: usize = 64;

    /// Maximum length of [`Output`] when encoded as B64 string: 86-bytes
    /// (i.e. 86 ASCII characters)
    pub const B64_MAX_LENGTH: usize = ((Self::MAX_LENGTH * 4) / 3) + 1;

    /// Create a [`Output`] from the given byte slice, validating it according
    /// to [`Output::MIN_LENGTH`] and [`Output::MAX_LENGTH`] restrictions.
    pub fn new(input: &[u8]) -> Result<Self> {
        Self::init_with(input.len(), |bytes| {
            bytes.copy_from_slice(input);
            Ok(())
        })
    }

    /// Create a [`Output`] from the given byte slice and [`Encoding`],
    /// validating it according to [`Output::MIN_LENGTH`] and
    /// [`Output::MAX_LENGTH`] restrictions.
    pub fn new_with_encoding(input: &[u8], encoding: Encoding) -> Result<Self> {
        let mut result = Self::new(input)?;
        result.encoding = encoding;
        Ok(result)
    }

    /// Initialize an [`Output`] using the provided method, which is given
    /// a mutable byte slice into which it should write the output.
    ///
    /// The `output_size` (in bytes) must be known in advance, as well as at
    /// least [`Output::MIN_LENGTH`] bytes and at most [`Output::MAX_LENGTH`]
    /// bytes.
    pub fn init_with<F>(output_size: usize, f: F) -> Result<Self>
    where
        F: FnOnce(&mut [u8]) -> Result<()>,
    {
        if output_size < Self::MIN_LENGTH {
            return Err(Error::OutputTooShort);
        }

        if output_size > Self::MAX_LENGTH {
            return Err(Error::OutputTooLong);
        }

        let mut bytes = [0u8; Self::MAX_LENGTH];
        f(&mut bytes[..output_size])?;

        Ok(Self {
            bytes,
            length: output_size as u8,
            encoding: Encoding::default(),
        })
    }

    /// Borrow the output value as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len()]
    }

    /// Get the [`Encoding`] that this [`Output`] is serialized with.
    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    /// Get the length of the output value as a byte slice.
    pub fn len(&self) -> usize {
        usize::from(self.length)
    }

    /// Parse B64-encoded [`Output`], i.e. using the PHC string
    /// specification's restricted interpretation of Base64.
    pub fn b64_decode(input: &str) -> Result<Self> {
        Self::decode(input, Encoding::B64)
    }

    /// Write B64-encoded [`Output`] to the provided buffer, returning
    /// a sub-slice containing the encoded data.
    ///
    /// Returns an error if the buffer is too short to contain the output.
    pub fn b64_encode<'a>(&self, out: &'a mut [u8]) -> Result<&'a str> {
        self.encode(out, Encoding::B64)
    }

    /// Decode the given input string using the specified [`Encoding`].
    pub fn decode(input: &str, encoding: Encoding) -> Result<Self> {
        let mut bytes = [0u8; Self::MAX_LENGTH];
        encoding
            .decode(input, &mut bytes)
            .map_err(Into::into)
            .and_then(|decoded| Self::new_with_encoding(decoded, encoding))
    }

    /// Encode this [`Output`] using the specified [`Encoding`].
    pub fn encode<'a>(&self, out: &'a mut [u8], encoding: Encoding) -> Result<&'a str> {
        Ok(encoding.encode(self.as_ref(), out)?)
    }

    /// Get the length of this [`Output`] when encoded as B64.
    pub fn b64_len(&self) -> usize {
        Encoding::B64.encoded_len(self.as_ref())
    }
}

impl AsRef<[u8]> for Output {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ConstantTimeEq for Output {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.as_ref().ct_eq(other.as_ref())
    }
}

impl FromStr for Output {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::b64_decode(s)
    }
}

impl PartialEq for Output {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

impl TryFrom<&[u8]> for Output {
    type Error = Error;

    fn try_from(input: &[u8]) -> Result<Output> {
        Self::new(input)
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = [0u8; Self::B64_MAX_LENGTH];
        self.encode(&mut buffer, self.encoding)
            .map_err(|_| fmt::Error)
            .and_then(|encoded| f.write_str(encoded))
    }
}

impl fmt::Debug for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Output(\"{}\")", self)
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Output};

    #[test]
    fn new_with_valid_min_length_input() {
        let bytes = [10u8; 10];
        let output = Output::new(&bytes).unwrap();
        assert_eq!(output.as_ref(), &bytes);
    }

    #[test]
    fn new_with_valid_max_length_input() {
        let bytes = [64u8; 64];
        let output = Output::new(&bytes).unwrap();
        assert_eq!(output.as_ref(), &bytes);
    }

    #[test]
    fn reject_new_too_short() {
        let bytes = [9u8; 9];
        let err = Output::new(&bytes).err().unwrap();
        assert_eq!(err, Error::OutputTooShort);
    }

    #[test]
    fn reject_new_too_long() {
        let bytes = [65u8; 65];
        let err = Output::new(&bytes).err().unwrap();
        assert_eq!(err, Error::OutputTooLong);
    }

    #[test]
    fn partialeq_true() {
        let a = Output::new(&[1u8; 32]).unwrap();
        let b = Output::new(&[1u8; 32]).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn partialeq_false() {
        let a = Output::new(&[1u8; 32]).unwrap();
        let b = Output::new(&[2u8; 32]).unwrap();
        assert_ne!(a, b);
    }
}
