//! Traits related to the key components

use num_bigint::{BigInt, BigUint};
use zeroize::Zeroize;

/// Components of an RSA public key.
pub trait PublicKeyParts {
    /// Returns the modulus of the key.
    fn n(&self) -> &BigUint;

    /// Returns the public exponent of the key.
    fn e(&self) -> &BigUint;

    /// Returns the modulus size in bytes. Raw signatures and ciphertexts for
    /// or by this public key will have the same size.
    fn size(&self) -> usize {
        (self.n().bits() + 7) / 8
    }
}

/// Components of an RSA private key.
pub trait PrivateKeyParts: PublicKeyParts {
    /// Returns the private exponent of the key.
    fn d(&self) -> &BigUint;

    /// Returns the prime factors.
    fn primes(&self) -> &[BigUint];

    /// Returns the precomputed dp value, D mod (P-1)
    fn dp(&self) -> Option<&BigUint>;

    /// Returns the precomputed dq value, D mod (Q-1)
    fn dq(&self) -> Option<&BigUint>;

    /// Returns the precomputed qinv value, Q^-1 mod P
    fn qinv(&self) -> Option<&BigInt>;

    /// Returns an iterator over the CRT Values
    fn crt_values(&self) -> Option<&[CrtValue]>;
}

/// Contains the precomputed Chinese remainder theorem values.
#[derive(Debug, Clone)]
pub struct CrtValue {
    /// D mod (prime - 1)
    pub(crate) exp: BigInt,
    /// R·Coeff ≡ 1 mod Prime.
    pub(crate) coeff: BigInt,
    /// product of primes prior to this (inc p and q)
    pub(crate) r: BigInt,
}

impl Zeroize for CrtValue {
    fn zeroize(&mut self) {
        self.exp.zeroize();
        self.coeff.zeroize();
        self.r.zeroize();
    }
}

impl Drop for CrtValue {
    fn drop(&mut self) {
        self.zeroize();
    }
}
