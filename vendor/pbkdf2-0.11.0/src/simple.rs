//! Implementation of the `password-hash` crate API.

use crate::pbkdf2;
use core::{cmp::Ordering, fmt, str::FromStr};
use hmac::Hmac;
use password_hash::{
    errors::InvalidValue, Decimal, Error, Ident, Output, ParamsString, PasswordHash,
    PasswordHasher, Result, Salt,
};
use sha2::{Sha256, Sha512};

#[cfg(feature = "sha1")]
use sha1::Sha1;

/// PBKDF2 (SHA-1)
#[cfg(feature = "sha1")]
pub const PBKDF2_SHA1: Ident = Ident::new_unwrap("pbkdf2");

/// PBKDF2 (SHA-256)
pub const PBKDF2_SHA256: Ident = Ident::new_unwrap("pbkdf2-sha256");

/// PBKDF2 (SHA-512)
pub const PBKDF2_SHA512: Ident = Ident::new_unwrap("pbkdf2-sha512");

/// PBKDF2 type for use with [`PasswordHasher`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(docsrs, doc(cfg(feature = "simple")))]
pub struct Pbkdf2;

impl PasswordHasher for Pbkdf2 {
    type Params = Params;

    fn hash_password_customized<'a>(
        &self,
        password: &[u8],
        alg_id: Option<Ident<'a>>,
        version: Option<Decimal>,
        params: Params,
        salt: impl Into<Salt<'a>>,
    ) -> Result<PasswordHash<'a>> {
        let algorithm = Algorithm::try_from(alg_id.unwrap_or(PBKDF2_SHA256))?;

        // Versions unsupported
        if version.is_some() {
            return Err(Error::Version);
        }

        let salt = salt.into();
        let mut salt_arr = [0u8; 64];
        let salt_bytes = salt.b64_decode(&mut salt_arr)?;

        let output = Output::init_with(params.output_length, |out| {
            let f = match algorithm {
                #[cfg(feature = "sha1")]
                Algorithm::Pbkdf2Sha1 => pbkdf2::<Hmac<Sha1>>,
                Algorithm::Pbkdf2Sha256 => pbkdf2::<Hmac<Sha256>>,
                Algorithm::Pbkdf2Sha512 => pbkdf2::<Hmac<Sha512>>,
            };

            f(password, salt_bytes, params.rounds, out);
            Ok(())
        })?;

        Ok(PasswordHash {
            algorithm: algorithm.ident(),
            version: None,
            params: params.try_into()?,
            salt: Some(salt),
            hash: Some(output),
        })
    }
}

/// PBKDF2 variants.
///
/// <https://en.wikipedia.org/wiki/PBKDF2>
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
#[cfg_attr(docsrs, doc(cfg(feature = "simple")))]
pub enum Algorithm {
    /// PBKDF2 SHA1
    #[cfg(feature = "sha1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sha1")))]
    Pbkdf2Sha1,

    /// PBKDF2 SHA-256
    Pbkdf2Sha256,

    /// PBKDF2 SHA-512
    Pbkdf2Sha512,
}

impl Algorithm {
    /// Parse an [`Algorithm`] from the provided string.
    pub fn new(id: impl AsRef<str>) -> Result<Self> {
        id.as_ref().parse()
    }

    /// Get the [`Ident`] that corresponds to this PBKDF2 [`Algorithm`].
    pub fn ident(&self) -> Ident<'static> {
        match self {
            #[cfg(feature = "sha1")]
            Algorithm::Pbkdf2Sha1 => PBKDF2_SHA1,
            Algorithm::Pbkdf2Sha256 => PBKDF2_SHA256,
            Algorithm::Pbkdf2Sha512 => PBKDF2_SHA512,
        }
    }

    /// Get the identifier string for this PBKDF2 [`Algorithm`].
    pub fn as_str(&self) -> &str {
        self.ident().as_str()
    }
}

impl AsRef<str> for Algorithm {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Algorithm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Algorithm> {
        Ident::try_from(s)?.try_into()
    }
}

impl From<Algorithm> for Ident<'static> {
    fn from(alg: Algorithm) -> Ident<'static> {
        alg.ident()
    }
}

impl<'a> TryFrom<Ident<'a>> for Algorithm {
    type Error = Error;

    fn try_from(ident: Ident<'a>) -> Result<Algorithm> {
        match ident {
            #[cfg(feature = "sha1")]
            PBKDF2_SHA1 => Ok(Algorithm::Pbkdf2Sha1),
            PBKDF2_SHA256 => Ok(Algorithm::Pbkdf2Sha256),
            PBKDF2_SHA512 => Ok(Algorithm::Pbkdf2Sha512),
            _ => Err(Error::Algorithm),
        }
    }
}

/// PBKDF2 params
#[cfg_attr(docsrs, doc(cfg(feature = "simple")))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Params {
    /// Number of rounds
    pub rounds: u32,

    /// Size of the output (in bytes)
    pub output_length: usize,
}

impl Default for Params {
    fn default() -> Params {
        Params {
            rounds: 10_000,
            output_length: 32,
        }
    }
}

impl<'a> TryFrom<&'a PasswordHash<'a>> for Params {
    type Error = Error;

    fn try_from(hash: &'a PasswordHash<'a>) -> Result<Self> {
        let mut params = Params::default();
        let mut output_length = None;

        if hash.version.is_some() {
            return Err(Error::Version);
        }

        for (ident, value) in hash.params.iter() {
            match ident.as_str() {
                "i" => params.rounds = value.decimal()?,
                "l" => {
                    output_length = Some(
                        value
                            .decimal()?
                            .try_into()
                            .map_err(|_| InvalidValue::Malformed.param_error())?,
                    )
                }
                _ => return Err(Error::ParamNameInvalid),
            }
        }

        if let Some(len) = output_length {
            if let Some(hash) = &hash.hash {
                match hash.len().cmp(&len) {
                    Ordering::Less => return Err(InvalidValue::TooShort.param_error()),
                    Ordering::Greater => return Err(InvalidValue::TooLong.param_error()),
                    Ordering::Equal => (),
                }
            }

            params.output_length = len;
        }

        Ok(params)
    }
}

impl<'a> TryFrom<Params> for ParamsString {
    type Error = Error;

    fn try_from(input: Params) -> Result<ParamsString> {
        let mut output = ParamsString::new();
        output.add_decimal("i", input.rounds)?;
        output.add_decimal("l", input.output_length as u32)?;
        Ok(output)
    }
}
