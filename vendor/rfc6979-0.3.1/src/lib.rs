#![doc = include_str!("../README.md")]

//! ## Usage
//!
//! See also: the documentation for the [`generate_k`] function.
//!
//! ```
//! use crypto_bigint::{ArrayEncoding, U256};
//! use sha2::{Digest, Sha256};
//!
//! // NIST P-256 field modulus
//! const NIST_P256_MODULUS: U256 =
//!     U256::from_be_hex("FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551");
//!
//! // Public key for RFC6979 NIST P256/SHA256 test case
//! const RFC6979_KEY: U256 =
//!     U256::from_be_hex("C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721");
//!
//! // Test message for RFC6979 NIST P256/SHA256 test case
//! const RFC6979_MSG: &[u8; 6] = b"sample";
//!
//! // Expected K for RFC6979 NIST P256/SHA256 test case
//! const RFC6979_EXPECTED_K: U256 =
//!     U256::from_be_hex("A6E3C57DD01ABE90086538398355DD4C3B17AA873382B0F24D6129493D8AAD60");
//!
//! let h = Sha256::digest(RFC6979_MSG);
//! let aad = b"";
//! let k = rfc6979::generate_k::<Sha256, U256>(&RFC6979_KEY, &NIST_P256_MODULUS, &h, aad);
//! assert_eq!(&k.to_be_byte_array(), &RFC6979_EXPECTED_K.to_be_byte_array());
//! ```

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg"
)]

use crypto_bigint::{ArrayEncoding, ByteArray, Integer};
use hmac::{
    digest::{
        core_api::BlockSizeUser, generic_array::GenericArray, Digest, FixedOutput,
        FixedOutputReset, Mac,
    },
    SimpleHmac,
};
use zeroize::{Zeroize, Zeroizing};

/// Deterministically generate ephemeral scalar `k`.
///
/// Accepts the following parameters and inputs:
///
/// - `x`: secret key
/// - `n`: field modulus
/// - `h`: hash/digest of input message: must be reduced modulo `n` in advance
/// - `data`: additional associated data, e.g. CSRNG output used as added entropy
#[inline]
pub fn generate_k<D, I>(x: &I, n: &I, h: &ByteArray<I>, data: &[u8]) -> Zeroizing<I>
where
    D: Digest + BlockSizeUser + FixedOutput<OutputSize = I::ByteSize> + FixedOutputReset,
    I: ArrayEncoding + Integer + Zeroize,
{
    let mut x = x.to_be_byte_array();
    let mut hmac_drbg = HmacDrbg::<D>::new(&x, h, data);
    x.zeroize();

    loop {
        let mut bytes = ByteArray::<I>::default();
        hmac_drbg.fill_bytes(&mut bytes);
        let k = I::from_be_byte_array(bytes);

        if (!k.is_zero() & k.ct_lt(n)).into() {
            return Zeroizing::new(k);
        }
    }
}

/// Internal implementation of `HMAC_DRBG` as described in NIST SP800-90A.
///
/// <https://csrc.nist.gov/publications/detail/sp/800-90a/rev-1/final>
///
/// This is a HMAC-based deterministic random bit generator used compute a
/// deterministic ephemeral scalar `k`.
pub struct HmacDrbg<D>
where
    D: Digest + BlockSizeUser + FixedOutputReset,
{
    /// HMAC key `K` (see RFC 6979 Section 3.2.c)
    k: SimpleHmac<D>,

    /// Chaining value `V` (see RFC 6979 Section 3.2.c)
    v: GenericArray<u8, D::OutputSize>,
}

impl<D> HmacDrbg<D>
where
    D: Digest + BlockSizeUser + FixedOutputReset,
{
    /// Initialize `HMAC_DRBG`
    pub fn new(entropy_input: &[u8], nonce: &[u8], additional_data: &[u8]) -> Self {
        let mut k = SimpleHmac::new(&Default::default());
        let mut v = GenericArray::default();

        for b in &mut v {
            *b = 0x01;
        }

        for i in 0..=1 {
            k.update(&v);
            k.update(&[i]);
            k.update(entropy_input);
            k.update(nonce);
            k.update(additional_data);
            k = SimpleHmac::new_from_slice(&k.finalize().into_bytes()).expect("HMAC error");

            // Steps 3.2.e,g: v = HMAC_k(v)
            k.update(&v);
            v = k.finalize_reset().into_bytes();
        }

        Self { k, v }
    }

    /// Write the next `HMAC_DRBG` output to the given byte slice.
    pub fn fill_bytes(&mut self, out: &mut [u8]) {
        for out_chunk in out.chunks_mut(self.v.len()) {
            self.k.update(&self.v);
            self.v = self.k.finalize_reset().into_bytes();
            out_chunk.copy_from_slice(&self.v[..out_chunk.len()]);
        }

        self.k.update(&self.v);
        self.k.update(&[0x00]);
        self.k =
            SimpleHmac::new_from_slice(&self.k.finalize_reset().into_bytes()).expect("HMAC error");
        self.k.update(&self.v);
        self.v = self.k.finalize_reset().into_bytes();
    }
}
