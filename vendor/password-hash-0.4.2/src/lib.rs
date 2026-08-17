#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_lifetimes)]

//!
//! # Usage
//!
//! This crate represents password hashes using the [`PasswordHash`] type, which
//! represents a parsed "PHC string" with the following format:
//!
//! ```text
//! $<id>[$v=<version>][$<param>=<value>(,<param>=<value>)*][$<salt>[$<hash>]]
//! ```
//!
//! For more information, please see the documentation for [`PasswordHash`].

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "rand_core")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand_core")))]
pub use rand_core;

pub mod errors;

mod encoding;
mod ident;
mod output;
mod params;
mod salt;
mod traits;
mod value;

pub use crate::{
    encoding::Encoding,
    errors::{Error, Result},
    ident::Ident,
    output::Output,
    params::ParamsString,
    salt::{Salt, SaltString},
    traits::{McfHasher, PasswordHasher, PasswordVerifier},
    value::{Decimal, Value},
};

use core::fmt::{self, Debug};

#[cfg(feature = "alloc")]
use alloc::{
    str::FromStr,
    string::{String, ToString},
};

/// Separator character used in password hashes (e.g. `$6$...`).
const PASSWORD_HASH_SEPARATOR: char = '$';

/// Password hash.
///
/// This type corresponds to the parsed representation of a PHC string as
/// described in the [PHC string format specification][1].
///
/// PHC strings have the following format:
///
/// ```text
/// $<id>[$v=<version>][$<param>=<value>(,<param>=<value>)*][$<salt>[$<hash>]]
/// ```
///
/// where:
///
/// - `<id>` is the symbolic name for the function
/// - `<version>` is the algorithm version
/// - `<param>` is a parameter name
/// - `<value>` is a parameter value
/// - `<salt>` is an encoding of the salt
/// - `<hash>` is an encoding of the hash output
///
/// The string is then the concatenation, in that order, of:
///
/// - a `$` sign;
/// - the function symbolic name;
/// - optionally, a `$` sign followed by the algorithm version with a `v=version` format;
/// - optionally, a `$` sign followed by one or several parameters, each with a `name=value` format;
///   the parameters are separated by commas;
/// - optionally, a `$` sign followed by the (encoded) salt value;
/// - optionally, a `$` sign followed by the (encoded) hash output (the hash output may be present
///   only if the salt is present).
///
/// [1]: https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#specification
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PasswordHash<'a> {
    /// Password hashing algorithm identifier.
    ///
    /// This corresponds to the `<id>` field in a PHC string, a.k.a. the
    /// symbolic name for the function.
    pub algorithm: Ident<'a>,

    /// Optional version field.
    ///
    /// This corresponds to the `<version>` field in a PHC string.
    pub version: Option<Decimal>,

    /// Algorithm-specific parameters.
    ///
    /// This corresponds to the set of `$<param>=<value>(,<param>=<value>)*`
    /// name/value pairs in a PHC string.
    pub params: ParamsString,

    /// [`Salt`] string for personalizing a password hash output.
    ///
    /// This corresponds to the `<salt>` value in a PHC string.
    pub salt: Option<Salt<'a>>,

    /// Password hashing function [`Output`], a.k.a. hash/digest.
    ///
    /// This corresponds to the `<hash>` output in a PHC string.
    pub hash: Option<Output>,
}

impl<'a> PasswordHash<'a> {
    /// Parse a password hash from a string in the PHC string format.
    pub fn new(s: &'a str) -> Result<Self> {
        Self::parse(s, Encoding::default())
    }

    /// Parse a password hash from the given [`Encoding`].
    pub fn parse(s: &'a str, encoding: Encoding) -> Result<Self> {
        if s.is_empty() {
            return Err(Error::PhcStringTooShort);
        }

        let mut fields = s.split(PASSWORD_HASH_SEPARATOR);
        let beginning = fields.next().expect("no first field");

        if beginning.chars().next().is_some() {
            return Err(Error::PhcStringInvalid);
        }

        let algorithm = fields
            .next()
            .ok_or(Error::PhcStringTooShort)
            .and_then(Ident::try_from)?;

        let mut version = None;
        let mut params = ParamsString::new();
        let mut salt = None;
        let mut hash = None;

        let mut next_field = fields.next();

        if let Some(field) = next_field {
            // v=<version>
            if field.starts_with("v=") && !field.contains(params::PARAMS_DELIMITER) {
                version = Some(Value::new(&field[2..]).and_then(|value| value.decimal())?);
                next_field = None;
            }
        }

        if next_field.is_none() {
            next_field = fields.next();
        }

        if let Some(field) = next_field {
            // <param>=<value>
            if field.contains(params::PAIR_DELIMITER) {
                params = field.parse()?;
                next_field = None;
            }
        }

        if next_field.is_none() {
            next_field = fields.next();
        }

        if let Some(s) = next_field {
            salt = Some(s.try_into()?);
        }

        if let Some(field) = fields.next() {
            hash = Some(Output::decode(field, encoding)?);
        }

        if fields.next().is_some() {
            return Err(Error::PhcStringTooLong);
        }

        Ok(Self {
            algorithm,
            version,
            params,
            salt,
            hash,
        })
    }

    /// Generate a password hash using the supplied algorithm.
    pub fn generate(
        phf: impl PasswordHasher,
        password: impl AsRef<[u8]>,
        salt: &'a str,
    ) -> Result<Self> {
        phf.hash_password(password.as_ref(), salt)
    }

    /// Verify this password hash using the specified set of supported
    /// [`PasswordHasher`] trait objects.
    pub fn verify_password(
        &self,
        phfs: &[&dyn PasswordVerifier],
        password: impl AsRef<[u8]>,
    ) -> Result<()> {
        for &phf in phfs {
            if phf.verify_password(password.as_ref(), self).is_ok() {
                return Ok(());
            }
        }

        Err(Error::Password)
    }

    /// Get the [`Encoding`] that this [`PasswordHash`] is serialized with.
    pub fn encoding(&self) -> Encoding {
        self.hash.map(|h| h.encoding()).unwrap_or_default()
    }

    /// Serialize this [`PasswordHash`] as a [`PasswordHashString`].
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn serialize(&self) -> PasswordHashString {
        self.into()
    }
}

// Note: this uses `TryFrom` instead of `FromStr` to support a lifetime on
// the `str` the value is being parsed from.
impl<'a> TryFrom<&'a str> for PasswordHash<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self> {
        Self::new(s)
    }
}

impl<'a> fmt::Display for PasswordHash<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", PASSWORD_HASH_SEPARATOR, self.algorithm)?;

        if let Some(version) = self.version {
            write!(f, "{}v={}", PASSWORD_HASH_SEPARATOR, version)?;
        }

        if !self.params.is_empty() {
            write!(f, "{}{}", PASSWORD_HASH_SEPARATOR, self.params)?;
        }

        if let Some(salt) = &self.salt {
            write!(f, "{}{}", PASSWORD_HASH_SEPARATOR, salt)?;

            if let Some(hash) = &self.hash {
                write!(f, "{}{}", PASSWORD_HASH_SEPARATOR, hash)?;
            }
        }

        Ok(())
    }
}

/// Serialized [`PasswordHash`].
///
/// This type contains a serialized password hash string which is ensured to
/// parse successfully.
// TODO(tarcieri): cached parsed representations? or at least structural data
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PasswordHashString {
    /// String value
    string: String,

    /// String encoding
    encoding: Encoding,
}

#[cfg(feature = "alloc")]
#[allow(clippy::len_without_is_empty)]
impl PasswordHashString {
    /// Parse a password hash from a string in the PHC string format.
    pub fn new(s: &str) -> Result<Self> {
        Self::parse(s, Encoding::default())
    }

    /// Parse a password hash from the given [`Encoding`].
    pub fn parse(s: &str, encoding: Encoding) -> Result<Self> {
        Ok(PasswordHash::parse(s, encoding)?.into())
    }

    /// Parse this owned string as a [`PasswordHash`].
    pub fn password_hash(&self) -> PasswordHash<'_> {
        PasswordHash::parse(&self.string, self.encoding).expect("malformed password hash")
    }

    /// Get the [`Encoding`] that this [`PasswordHashString`] is serialized with.
    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    /// Borrow this value as a `str`.
    pub fn as_str(&self) -> &str {
        self.string.as_str()
    }

    /// Borrow this value as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    /// Get the length of this value in ASCII characters.
    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    /// Password hashing algorithm identifier.
    pub fn algorithm(&self) -> Ident<'_> {
        self.password_hash().algorithm
    }

    /// Optional version field.
    pub fn version(&self) -> Option<Decimal> {
        self.password_hash().version
    }

    /// Algorithm-specific parameters.
    pub fn params(&self) -> ParamsString {
        self.password_hash().params
    }

    /// [`Salt`] string for personalizing a password hash output.
    pub fn salt(&self) -> Option<Salt<'_>> {
        self.password_hash().salt
    }

    /// Password hashing function [`Output`], a.k.a. hash/digest.
    pub fn hash(&self) -> Option<Output> {
        self.password_hash().hash
    }
}

#[cfg(feature = "alloc")]
impl AsRef<str> for PasswordHashString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(feature = "alloc")]
impl From<PasswordHash<'_>> for PasswordHashString {
    fn from(hash: PasswordHash<'_>) -> PasswordHashString {
        PasswordHashString::from(&hash)
    }
}

#[cfg(feature = "alloc")]
impl From<&PasswordHash<'_>> for PasswordHashString {
    fn from(hash: &PasswordHash<'_>) -> PasswordHashString {
        PasswordHashString {
            string: hash.to_string(),
            encoding: hash.encoding(),
        }
    }
}

#[cfg(feature = "alloc")]
impl FromStr for PasswordHashString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

#[cfg(feature = "alloc")]
impl fmt::Display for PasswordHashString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
