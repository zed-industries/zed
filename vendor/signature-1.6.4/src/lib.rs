#![no_std]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

//! # Design
//!
//! This crate provides a common set of traits for signing and verifying
//! digital signatures intended to be implemented by libraries which produce
//! or contain implementations of digital signature algorithms, and used by
//! libraries which want to produce or verify digital signatures while
//! generically supporting any compatible backend.
//!
//! ## Goals
//!
//! The traits provided by this crate were designed with the following goals
//! in mind:
//!
//! - Provide an easy-to-use, misuse resistant API optimized for consumers
//!   (as opposed to implementers) of its traits.
//! - Support common type-safe wrappers around "bag-of-bytes" representations
//!   which can be directly parsed from or written to the "wire".
//! - Expose a trait/object-safe API where signers/verifiers spanning multiple
//!   homogeneous provider implementations can be seamlessly leveraged together
//!   in the same logical "keyring" so long as they operate on the same
//!   underlying signature type.
//! - Allow one provider type to potentially implement support (including
//!   being generic over) several signature types.
//! - Keep signature algorithm customizations / "knobs" out-of-band from the
//!   signing/verification APIs, ideally pushing such concerns into the type
//!   system so that algorithm mismatches are caught as type errors.
//! - Opaque error type which minimizes information leaked from cryptographic
//!   failures, as "rich" error types in these scenarios are often a source
//!   of sidechannel information for attackers (e.g. [BB'06])
//!
//! [BB'06]: https://en.wikipedia.org/wiki/Daniel_Bleichenbacher
//!
//! ## Implementation
//!
//! To accomplish the above goals, the [`Signer`] and [`Verifier`] traits
//! provided by this are generic over a [`Signature`] return value, and use
//! generic parameters rather than associated types. Notably, they use such
//! a parameter for the return value, allowing it to be inferred by the type
//! checker based on the desired signature type.
//!
//! The [`Signature`] trait is bounded on `AsRef<[u8]>`, enforcing that
//! signature types are thin wrappers around a "bag-of-bytes"
//! serialization. Inspiration for this approach comes from the Ed25519
//! signature system, which was based on the observation that past
//! systems were not prescriptive about how signatures should be represented
//! on-the-wire, and that lead to a proliferation of different wire formats
//! and confusion about which ones should be used. This crate aims to provide
//! similar simplicity by minimizing the number of steps involved to obtain
//! a serializable signature.
//!
//! ## Alternatives considered
//!
//! This crate is based on over two years of exploration of how to encapsulate
//! digital signature systems in the most flexible, developer-friendly way.
//! During that time many design alternatives were explored, tradeoffs
//! compared, and ultimately the provided API was selected.
//!
//! The tradeoffs made in this API have all been to improve simplicity,
//! ergonomics, type safety, and flexibility for *consumers* of the traits.
//! At times, this has come at a cost to implementers. Below are some concerns
//! we are cognizant of which were considered in the design of the API:
//!
//! - "Bag-of-bytes" serialization precludes signature providers from using
//!   their own internal representation of a signature, which can be helpful
//!   for many reasons (e.g. advanced signature system features like batch
//!   verification). Alternatively each provider could define its own signature
//!   type, using a marker trait to identify the particular signature algorithm,
//!   have `From` impls for converting to/from `[u8; N]`, and a marker trait
//!   for identifying a specific signature algorithm.
//! - Associated types, rather than generic parameters of traits, could allow
//!   more customization of the types used by a particular signature system,
//!   e.g. using custom error types.
//!
//! It may still make sense to continue to explore the above tradeoffs, but
//! with a *new* set of traits which are intended to be implementor-friendly,
//! rather than consumer friendly. The existing [`Signer`] and [`Verifier`]
//! traits could have blanket impls for the "provider-friendly" traits.
//! However, as noted above this is a design space easily explored after
//! stabilizing the consumer-oriented traits, and thus we consider these
//! more important.
//!
//! That said, below are some caveats of trying to design such traits, and
//! why we haven't actively pursued them:
//!
//! - Generics in the return position are already used to select which trait
//!   impl to use, i.e. for a particular signature algorithm/system. Avoiding
//!   a unified, concrete signature type adds another dimension to complexity
//!   and compiler errors, and in our experience makes them unsuitable for this
//!   sort of API. We believe such an API is the natural one for signature
//!   systems, reflecting the natural way they are written absent a trait.
//! - Associated types preclude multiple (or generic) implementations of the
//!   same trait. These parameters are common in signature systems, notably
//!   ones which support different digest algorithms.
//! - Digital signatures are almost always larger than the present 32-entry
//!   trait impl limitation on array types, which complicates trait signatures
//!   for these types (particularly things like `From` or `Borrow` bounds).
//!   This may be more interesting to explore after const generics.
//!
//! ## Unstable features
//!
//! Despite being post-1.0, this crate includes a number of off-by-default
//! unstable features named `*-preview`, each of which depends on a pre-1.0
//! crate.
//!
//! These features are considered exempt from SemVer. See the
//! [SemVer policy](#semver-policy) above for more information.
//!
//! The following unstable features are presently supported:
//!
//! - `derive-preview`: for implementers of signature systems using
//!   [`DigestSigner`] and [`DigestVerifier`], the `derive-preview` feature
//!   can be used to derive [`Signer`] and [`Verifier`] traits which prehash
//!   the input message using the [`PrehashSignature::Digest`] algorithm for
//!   a given [`Signature`] type. When the `derive-preview` feature is enabled
//!   import the proc macros with `use signature::{Signer, Verifier}` and then
//!   add a `derive(Signer)` or `derive(Verifier)` attribute to the given
//!   digest signer/verifier type. Enabling this feature also enables `digest`
//!   support (see immediately below).
//! - `digest-preview`: enables the [`DigestSigner`] and [`DigestVerifier`]
//!   traits which are based on the [`Digest`] trait from the [`digest`] crate.
//!   These traits are used for representing signature systems based on the
//!   [Fiat-Shamir heuristic] which compute a random challenge value to sign
//!   by computing a cryptographically secure digest of the input message.
//! - `rand-preview`: enables the [`RandomizedSigner`] trait for signature
//!   systems which rely on a cryptographically secure random number generator
//!   for security.
//!
//! NOTE: the [`async-signature`] crate contains experimental `async` support
//! for [`Signer`] and [`DigestSigner`].
//!
//! [`async-signature`]: https://docs.rs/async-signature
//! [`digest`]: https://docs.rs/digest/
//! [`Digest`]: https://docs.rs/digest/latest/digest/trait.Digest.html
//! [Fiat-Shamir heuristic]: https://en.wikipedia.org/wiki/Fiat%E2%80%93Shamir_heuristic

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "signature_derive", not(feature = "derive-preview")))]
compile_error!(
    "The `signature_derive` feature should not be enabled directly. \
    Use the `derive-preview` feature instead."
);

#[cfg(all(feature = "digest", not(feature = "digest-preview")))]
compile_error!(
    "The `digest` feature should not be enabled directly. \
    Use the `digest-preview` feature instead."
);

#[cfg(all(feature = "rand_core", not(feature = "rand-preview")))]
compile_error!(
    "The `rand_core` feature should not be enabled directly. \
    Use the `rand-preview` feature instead."
);

#[cfg(feature = "hazmat-preview")]
#[cfg_attr(docsrs, doc(cfg(feature = "hazmat-preview")))]
pub mod hazmat;

mod error;
mod keypair;
mod signature;
mod signer;
mod verifier;

#[cfg(feature = "derive-preview")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive-preview")))]
pub use signature_derive::{Signer, Verifier};

#[cfg(all(feature = "derive-preview", feature = "digest-preview"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "derive-preview", feature = "digest-preview")))
)]
pub use signature_derive::{DigestSigner, DigestVerifier};

#[cfg(feature = "digest-preview")]
pub use digest;

#[cfg(feature = "rand-preview")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand-preview")))]
pub use rand_core;

pub use crate::{error::*, keypair::*, signature::*, signer::*, verifier::*};
