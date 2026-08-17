//! Elliptic Curve Diffie-Hellman (Ephemeral) Support.
//!
//! This module contains a high-level interface for performing ephemeral
//! Diffie-Hellman key exchanges using the secp256k1 elliptic curve.
//!
//! # Usage
//!
//! This usage example is from the perspective of two participants in the
//! exchange, nicknamed "Alice" and "Bob".
//!
//! ```
//! use p256::{EncodedPoint, PublicKey, ecdh::EphemeralSecret};
//! use rand_core::OsRng; // requires 'getrandom' feature
//!
//! // Alice
//! let alice_secret = EphemeralSecret::random(&mut OsRng);
//! let alice_pk_bytes = EncodedPoint::from(alice_secret.public_key());
//!
//! // Bob
//! let bob_secret = EphemeralSecret::random(&mut OsRng);
//! let bob_pk_bytes = EncodedPoint::from(bob_secret.public_key());
//!
//! // Alice decodes Bob's serialized public key and computes a shared secret from it
//! let bob_public = PublicKey::from_sec1_bytes(bob_pk_bytes.as_ref())
//!     .expect("bob's public key is invalid!"); // In real usage, don't panic, handle this!
//!
//! let alice_shared = alice_secret.diffie_hellman(&bob_public);
//!
//! // Bob decodes Alice's serialized public key and computes the same shared secret
//! let alice_public = PublicKey::from_sec1_bytes(alice_pk_bytes.as_ref())
//!     .expect("alice's public key is invalid!"); // In real usage, don't panic, handle this!
//!
//! let bob_shared = bob_secret.diffie_hellman(&alice_public);
//!
//! // Both participants arrive on the same shared secret
//! assert_eq!(alice_shared.raw_secret_bytes(), bob_shared.raw_secret_bytes());
//! ```

pub use elliptic_curve::ecdh::diffie_hellman;

use crate::{AffinePoint, NistP256};

/// NIST P-256 Ephemeral Diffie-Hellman Secret.
pub type EphemeralSecret = elliptic_curve::ecdh::EphemeralSecret<NistP256>;

/// Shared secret value computed via ECDH key agreement.
pub type SharedSecret = elliptic_curve::ecdh::SharedSecret<NistP256>;

impl From<&AffinePoint> for SharedSecret {
    fn from(affine: &AffinePoint) -> SharedSecret {
        affine.x.to_bytes().into()
    }
}
