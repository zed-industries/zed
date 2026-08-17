// Copyright 2015-2021 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! Error reporting.

extern crate std;

use core::num::TryFromIntError;
// The Error trait is not in core: https://github.com/rust-lang/rust/issues/103765
use std::error::Error;

/// An error with absolutely no details.
///
/// *aws-lc-rs* uses this unit type as the error type in most of its results
/// because (a) usually the specific reasons for a failure are obvious or are
/// not useful to know, and/or (b) providing more details about a failure might
/// provide a dangerous side channel, and/or (c) it greatly simplifies the
/// error handling logic.
///
/// `Result<T, aws_lc_rs::error::Unspecified>` is mostly equivalent to
/// `Result<T, ()>`. However, `aws_lc_rs::error::Unspecified` implements
/// [`std::error::Error`] and users can implement
/// `From<error::Unspecified>` to map this to their own error types, as
/// described in [“Error Handling” in the Rust Book](https://doc.rust-lang.org/book/ch09-00-error-handling.html):
///
/// ```
/// use aws_lc_rs::rand::{self, SecureRandom};
///
/// enum Error {
///     CryptoError,
///
///     IOError(std::io::Error),
///     // [...]
/// }
///
/// impl From<aws_lc_rs::error::Unspecified> for Error {
///     fn from(_: aws_lc_rs::error::Unspecified) -> Self {
///         Error::CryptoError
///     }
/// }
///
/// fn eight_random_bytes() -> Result<[u8; 8], Error> {
///     let rng = rand::SystemRandom::new();
///     let mut bytes = [0; 8];
///
///     // The `From<aws_lc_rs::error::Unspecified>` implementation above makes this
///     // equivalent to
///     // `rng.fill(&mut bytes).map_err(|_| Error::CryptoError)?`.
///     rng.fill(&mut bytes)?;
///
///     Ok(bytes)
/// }
///
/// assert!(eight_random_bytes().is_ok());
/// ```
///
/// Experience with using and implementing other crypto libraries like has
/// shown that sophisticated error reporting facilities often cause significant
/// bugs themselves, both within the crypto library and within users of the
/// crypto library. This approach attempts to minimize complexity in the hopes
/// of avoiding such problems. In some cases, this approach may be too extreme,
/// and it may be important for an operation to provide some details about the
/// cause of a failure. Users of *aws-lc-rs* are encouraged to report such cases so
/// that they can be addressed individually.
///
/// [`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
/// [“Error Handling” in the Rust Book]:
///     https://doc.rust-lang.org/book/first-edition/error-handling.html#the-from-trait
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unspecified;

// This is required for the implementation of `std::error::Error`.
impl core::fmt::Display for Unspecified {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("Unspecified")
    }
}

impl From<core::array::TryFromSliceError> for Unspecified {
    fn from(_: core::array::TryFromSliceError) -> Self {
        Self
    }
}

/// An error parsing or validating a key.
///
/// The `Display` implementation and `<KeyRejected as Error>::description()`
/// will return a string that will help you better understand why a key was
/// rejected change which errors are reported in which situations while
/// minimizing the likelihood that any applications will be broken.
///
/// Here is an incomplete list of reasons a key may be unsupported:
///
/// * Invalid or Inconsistent Components: A component of the key has an invalid
///   value, or the mathematical relationship between two (or more) components
///   required for a valid key does not hold.
///
/// * The encoding of the key is invalid. Perhaps the key isn't in the correct
///   format; e.g. it may be Base64 ("PEM") encoded, in which case   the Base64
///   encoding needs to be undone first.
///
/// * The encoding includes a versioning mechanism and that mechanism indicates
///   that the key is encoded in a version of the encoding that isn't supported.
///   This might happen for multi-prime RSA keys (keys with more than two
///   private   prime factors), which aren't supported, for example.
///
/// * Too small or too Large: One of the primary components of the key is too
///   small or two large. Too-small keys are rejected for security reasons. Some
///   unnecessarily large keys are rejected for performance reasons.
///
///  * Wrong algorithm: The key is not valid for the algorithm in which it was
///    being used.
///
///  * Unexpected errors: Report this as a bug.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyRejected(&'static str);

impl KeyRejected {
    /// The value returned from `<Self as std::error::Error>::description()`
    #[must_use]
    pub fn description_(&self) -> &'static str {
        self.0
    }

    pub(crate) fn inconsistent_components() -> Self {
        KeyRejected("InconsistentComponents")
    }

    #[inline]
    pub(crate) fn invalid_encoding() -> Self {
        KeyRejected("InvalidEncoding")
    }

    pub(crate) fn too_small() -> Self {
        KeyRejected("TooSmall")
    }

    pub(crate) fn too_large() -> Self {
        KeyRejected("TooLarge")
    }

    pub(crate) fn wrong_algorithm() -> Self {
        KeyRejected("WrongAlgorithm")
    }

    pub(crate) fn unexpected_error() -> Self {
        KeyRejected("UnexpectedError")
    }

    pub(crate) fn unspecified() -> Self {
        KeyRejected("Unspecified")
    }
}

impl Error for KeyRejected {
    fn description(&self) -> &str {
        self.description_()
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl Error for Unspecified {
    #[allow(clippy::unnecessary_literal_bound)]
    fn description(&self) -> &str {
        "Unspecified"
    }

    #[inline]
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl core::fmt::Display for KeyRejected {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.description_())
    }
}

impl From<KeyRejected> for Unspecified {
    fn from(_: KeyRejected) -> Self {
        Unspecified
    }
}

impl From<()> for Unspecified {
    fn from((): ()) -> Self {
        Unspecified
    }
}

impl From<Unspecified> for () {
    fn from(_: Unspecified) -> Self {}
}

impl From<()> for KeyRejected {
    fn from((): ()) -> Self {
        KeyRejected::unexpected_error()
    }
}

#[cfg(any(feature = "ring-sig-verify", feature = "ring-io"))]
impl From<untrusted::EndOfInput> for Unspecified {
    fn from(_: untrusted::EndOfInput) -> Self {
        Unspecified
    }
}

impl From<TryFromIntError> for Unspecified {
    fn from(_: TryFromIntError) -> Self {
        Unspecified
    }
}

impl From<TryFromIntError> for KeyRejected {
    fn from(_: TryFromIntError) -> Self {
        KeyRejected::unexpected_error()
    }
}

impl From<Unspecified> for KeyRejected {
    fn from(_: Unspecified) -> Self {
        Self::unspecified()
    }
}

#[allow(deprecated, unused_imports)]
#[cfg(test)]
mod tests {
    use crate::error::KeyRejected;
    use crate::test;
    use std::error::Error;

    #[test]
    fn display_unspecified() {
        let output = format!("{}", super::Unspecified);
        assert_eq!("Unspecified", output);
    }

    #[test]
    fn unexpected_error() {
        let key_rejected = super::KeyRejected::from(());
        assert_eq!("UnexpectedError", key_rejected.description());

        let unspecified = super::Unspecified::from(key_rejected);
        assert_eq!("Unspecified", unspecified.description());

        #[allow(clippy::redundant_locals)]
        let unspecified = unspecified;
        assert_eq!("Unspecified", unspecified.description());
    }

    #[test]
    fn std_error() {
        let key_rejected = KeyRejected::wrong_algorithm();
        assert!(key_rejected.cause().is_none());
        assert_eq!("WrongAlgorithm", key_rejected.description());

        let unspecified = super::Unspecified;
        assert!(unspecified.cause().is_none());
        assert_eq!("Unspecified", unspecified.description());

        test::compile_time_assert_std_error_error::<KeyRejected>();
    }
}
