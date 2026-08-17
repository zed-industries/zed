//! Error types.

/// Alias for [`core::result::Result`] with the `rsa` crate's [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

/// Error types
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Invalid padding scheme.
    InvalidPaddingScheme,

    /// Decryption error.
    Decryption,

    /// Verification error.
    Verification,

    /// Message too long.
    MessageTooLong,

    /// Input must be hashed.
    InputNotHashed,

    /// Number of primes must be 2 or greater.
    NprimesTooSmall,

    /// Too few primes of a given length to generate an RSA key.
    TooFewPrimes,

    /// Invalid prime value.
    InvalidPrime,

    /// Invalid modulus.
    InvalidModulus,

    /// Invalid exponent.
    InvalidExponent,

    /// Invalid coefficient.
    InvalidCoefficient,

    /// Modulus too large.
    ModulusTooLarge,

    /// Public exponent too small.
    PublicExponentTooSmall,

    /// Public exponent too large.
    PublicExponentTooLarge,

    /// PKCS#1 error.
    Pkcs1(pkcs1::Error),

    /// PKCS#8 error.
    Pkcs8(pkcs8::Error),

    /// Internal error.
    Internal,

    /// Label too long.
    LabelTooLong,

    /// Invalid padding length.
    InvalidPadLen,

    /// Invalid arguments.
    InvalidArguments,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::InvalidPaddingScheme => write!(f, "invalid padding scheme"),
            Error::Decryption => write!(f, "decryption error"),
            Error::Verification => write!(f, "verification error"),
            Error::MessageTooLong => write!(f, "message too long"),
            Error::InputNotHashed => write!(f, "input must be hashed"),
            Error::NprimesTooSmall => write!(f, "nprimes must be >= 2"),
            Error::TooFewPrimes => {
                write!(f, "too few primes of given length to generate an RSA key")
            }
            Error::InvalidPrime => write!(f, "invalid prime value"),
            Error::InvalidModulus => write!(f, "invalid modulus"),
            Error::InvalidExponent => write!(f, "invalid exponent"),
            Error::InvalidCoefficient => write!(f, "invalid coefficient"),
            Error::ModulusTooLarge => write!(f, "modulus too large"),
            Error::PublicExponentTooSmall => write!(f, "public exponent too small"),
            Error::PublicExponentTooLarge => write!(f, "public exponent too large"),
            Error::Pkcs1(err) => write!(f, "{}", err),
            Error::Pkcs8(err) => write!(f, "{}", err),
            Error::Internal => write!(f, "internal error"),
            Error::LabelTooLong => write!(f, "label too long"),
            Error::InvalidPadLen => write!(f, "invalid padding length"),
            Error::InvalidArguments => write!(f, "invalid arguments"),
        }
    }
}

impl From<pkcs1::Error> for Error {
    fn from(err: pkcs1::Error) -> Error {
        Error::Pkcs1(err)
    }
}

impl From<pkcs8::Error> for Error {
    fn from(err: pkcs8::Error) -> Error {
        Error::Pkcs8(err)
    }
}

#[cfg(feature = "std")]
impl From<Error> for signature::Error {
    fn from(err: Error) -> Self {
        Self::from_source(err)
    }
}

#[cfg(not(feature = "std"))]
impl From<Error> for signature::Error {
    fn from(_err: Error) -> Self {
        Self::new()
    }
}
