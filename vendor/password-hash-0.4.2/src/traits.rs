//! Trait definitions.

use crate::{Decimal, Error, Ident, ParamsString, PasswordHash, Result, Salt};
use core::fmt::Debug;

/// Trait for password hashing functions.
pub trait PasswordHasher {
    /// Algorithm-specific parameters.
    type Params: Clone
        + Debug
        + Default
        + for<'a> TryFrom<&'a PasswordHash<'a>, Error = Error>
        + TryInto<ParamsString, Error = Error>;

    /// Compute a [`PasswordHash`] from the provided password using an
    /// explicit set of customized algorithm parameters as opposed to the
    /// defaults.
    ///
    /// When in doubt, use [`PasswordHasher::hash_password`] instead.
    fn hash_password_customized<'a>(
        &self,
        password: &[u8],
        algorithm: Option<Ident<'a>>,
        version: Option<Decimal>,
        params: Self::Params,
        salt: impl Into<Salt<'a>>,
    ) -> Result<PasswordHash<'a>>;

    /// Simple API for computing a [`PasswordHash`] from a password and
    /// salt value.
    ///
    /// Uses the default recommended parameters for a given algorithm.
    fn hash_password<'a, S>(&self, password: &[u8], salt: &'a S) -> Result<PasswordHash<'a>>
    where
        S: AsRef<str> + ?Sized,
    {
        self.hash_password_customized(
            password,
            None,
            None,
            Self::Params::default(),
            Salt::try_from(salt.as_ref())?,
        )
    }
}

/// Trait for password verification.
///
/// Automatically impl'd for any type that impls [`PasswordHasher`].
///
/// This trait is object safe and can be used to implement abstractions over
/// multiple password hashing algorithms. One such abstraction is provided by
/// the [`PasswordHash::verify_password`] method.
pub trait PasswordVerifier {
    /// Compute this password hashing function against the provided password
    /// using the parameters from the provided password hash and see if the
    /// computed output matches.
    fn verify_password(&self, password: &[u8], hash: &PasswordHash<'_>) -> Result<()>;
}

impl<T: PasswordHasher> PasswordVerifier for T {
    fn verify_password(&self, password: &[u8], hash: &PasswordHash<'_>) -> Result<()> {
        if let (Some(salt), Some(expected_output)) = (&hash.salt, &hash.hash) {
            let computed_hash = self.hash_password_customized(
                password,
                Some(hash.algorithm),
                hash.version,
                T::Params::try_from(hash)?,
                *salt,
            )?;

            if let Some(computed_output) = &computed_hash.hash {
                // See notes on `Output` about the use of a constant-time comparison
                if expected_output == computed_output {
                    return Ok(());
                }
            }
        }

        Err(Error::Password)
    }
}

/// Trait for password hashing algorithms which support the legacy
/// [Modular Crypt Format (MCF)][MCF].
///
/// [MCF]: https://passlib.readthedocs.io/en/stable/modular_crypt_format.html
pub trait McfHasher {
    /// Upgrade an MCF hash to a PHC hash. MCF follow this rough format:
    ///
    /// ```text
    /// $<id>$<content>
    /// ```
    ///
    /// MCF hashes are otherwise largely unstructured and parsed according to
    /// algorithm-specific rules so hashers must parse a raw string themselves.
    fn upgrade_mcf_hash<'a>(&self, hash: &'a str) -> Result<PasswordHash<'a>>;

    /// Verify a password hash in MCF format against the provided password.
    fn verify_mcf_hash(&self, password: &[u8], mcf_hash: &str) -> Result<()>
    where
        Self: PasswordVerifier,
    {
        self.verify_password(password, &self.upgrade_mcf_hash(mcf_hash)?)
    }
}
