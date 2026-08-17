//! Signature traits

use crate::error::Error;
use core::fmt::Debug;

/// For intra-doc link resolution
#[cfg(feature = "digest-preview")]
#[allow(unused_imports)]
use crate::{
    signer::{DigestSigner, Signer},
    verifier::{DigestVerifier, Verifier},
};

/// Trait impl'd by concrete types that represent digital signatures.
///
/// Signature types *must* (as mandated by the `AsRef<[u8]>` bound) be a thin
/// wrapper around the "bag-of-bytes" serialized form of a signature which can
/// be directly parsed from or written to the "wire".
///
/// Inspiration for this approach comes from the Ed25519 signature system,
/// which adopted it based on the observation that past signature systems
/// were not prescriptive about how signatures should be represented
/// on-the-wire, and that lead to a proliferation of different wire formats and
/// confusion about which ones should be used.
///
/// The [`Signature`] trait aims to provide similar simplicity by minimizing
/// the number of steps involved to obtain a serializable signature and
/// ideally ensuring there is one signature type for any given signature system
/// shared by all "provider" crates.
///
/// For signature systems which require a more advanced internal representation
/// (e.g. involving decoded scalars or decompressed elliptic curve points) it's
/// recommended that "provider" libraries maintain their own internal signature
/// type and use `From` bounds to provide automatic conversions.
pub trait Signature: AsRef<[u8]> + Debug + Sized {
    /// Parse a signature from its byte representation
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error>;

    /// Borrow a byte slice representing the serialized form of this signature
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

/// Marker trait for `Signature` types computable as `𝐒(𝐇(𝒎))`
/// i.e. ones which prehash a message to be signed as `𝐇(𝒎)`
///
/// Where:
///
/// - `𝐒`: signature algorithm
/// - `𝐇`: hash (a.k.a. digest) function
/// - `𝒎`: message
///
/// This approach is relatively common in signature schemes based on the
/// [Fiat-Shamir heuristic].
///
/// For signature types that implement this trait, when the `derive-preview`
/// Cargo feature is enabled a custom derive for [`Signer`] is available for any
/// types that impl [`DigestSigner`], and likewise for deriving [`Verifier`] for
/// types which impl [`DigestVerifier`].
///
/// [Fiat-Shamir heuristic]: https://en.wikipedia.org/wiki/Fiat%E2%80%93Shamir_heuristic
#[cfg(feature = "digest-preview")]
#[cfg_attr(docsrs, doc(cfg(feature = "digest-preview")))]
pub trait PrehashSignature: Signature {
    /// Preferred `Digest` algorithm to use when computing this signature type.
    type Digest: digest::Digest;
}
