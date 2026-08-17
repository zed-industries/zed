#![cfg_attr(not(test), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo_small.png")]
#![warn(missing_docs)]

//! # Supported algorithms
//!
//! This crate supports several schemes described in [RFC8017]:
//!
//! - [OAEP encryption scheme](#oaep-encryption)
//! - [PKCS#1 v1.5 encryption scheme](#pkcs1-v15-encryption)
//! - [PKCS#1 v1.5 signature scheme](#pkcs1-v15-signatures)
//! - [PSS signature scheme](#pss-signatures)
//!
//! These schemes are described below.
//!
//! # Usage
//!
//! ## OAEP encryption
//!
//! Note: requires `sha2` feature of `rsa` crate is enabled.
//!
#![cfg_attr(feature = "sha2", doc = "```")]
#![cfg_attr(not(feature = "sha2"), doc = "```ignore")]
//! use rsa::{RsaPrivateKey, RsaPublicKey, Oaep, sha2::Sha256};
//!
//! let mut rng = rand::thread_rng(); // rand@0.8
//!
//! let bits = 2048;
//! let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
//! let public_key = RsaPublicKey::from(&private_key);
//!
//! // Encrypt
//! let data = b"hello world";
//! let padding = Oaep::new::<Sha256>();
//! let enc_data = public_key.encrypt(&mut rng, padding, &data[..]).expect("failed to encrypt");
//! assert_ne!(&data[..], &enc_data[..]);
//!
//! // Decrypt
//! let padding = Oaep::new::<Sha256>();
//! let dec_data = private_key.decrypt(padding, &enc_data).expect("failed to decrypt");
//! assert_eq!(&data[..], &dec_data[..]);
//! ```
//!
//! ## PKCS#1 v1.5 encryption
//! ```
//! use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
//!
//! let mut rng = rand::thread_rng(); // rand@0.8
//!
//! let bits = 2048;
//! let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
//! let public_key = RsaPublicKey::from(&private_key);
//!
//! // Encrypt
//! let data = b"hello world";
//! let enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
//! assert_ne!(&data[..], &enc_data[..]);
//!
//! // Decrypt
//! let dec_data = private_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
//! assert_eq!(&data[..], &dec_data[..]);
//! ```
//!
//! ## PKCS#1 v1.5 signatures
//!
//! Note: requires `sha2` feature of `rsa` crate is enabled.
//!
#![cfg_attr(feature = "sha2", doc = "```")]
#![cfg_attr(not(feature = "sha2"), doc = "```ignore")]
//! use rsa::RsaPrivateKey;
//! use rsa::pkcs1v15::{SigningKey, VerifyingKey};
//! use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
//! use rsa::sha2::{Digest, Sha256};
//!
//! let mut rng = rand::thread_rng(); // rand@0.8
//!
//! let bits = 2048;
//! let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
//! let signing_key = SigningKey::<Sha256>::new(private_key);
//! let verifying_key = signing_key.verifying_key();
//!
//! // Sign
//! let data = b"hello world";
//! let signature = signing_key.sign_with_rng(&mut rng, data);
//! assert_ne!(signature.to_bytes().as_ref(), data.as_slice());
//!
//! // Verify
//! verifying_key.verify(data, &signature).expect("failed to verify");
//! ```
//!
//! ## PSS signatures
//!
//! Note: requires `sha2` feature of `rsa` crate is enabled.
//!
#![cfg_attr(feature = "sha2", doc = "```")]
#![cfg_attr(not(feature = "sha2"), doc = "```ignore")]
//! use rsa::RsaPrivateKey;
//! use rsa::pss::{BlindedSigningKey, VerifyingKey};
//! use rsa::signature::{Keypair,RandomizedSigner, SignatureEncoding, Verifier};
//! use rsa::sha2::{Digest, Sha256};
//!
//! let mut rng = rand::thread_rng(); // rand@0.8
//!
//! let bits = 2048;
//! let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
//! let signing_key = BlindedSigningKey::<Sha256>::new(private_key);
//! let verifying_key = signing_key.verifying_key();
//!
//! // Sign
//! let data = b"hello world";
//! let signature = signing_key.sign_with_rng(&mut rng, data);
//! assert_ne!(signature.to_bytes().as_ref(), data);
//!
//! // Verify
//! verifying_key.verify(data, &signature).expect("failed to verify");
//! ```
//!
//! ## PKCS#1 RSA Key Encoding
//!
//! PKCS#1 supports a legacy format for encoding RSA keys as binary (DER) or
//! text (PEM) data.
//!
//! You can recognize PEM encoded PKCS#1 keys because they have "RSA * KEY" in
//! the type label, e.g.:
//!
//! ```text
//! -----BEGIN RSA PRIVATE KEY-----
//! ```
//!
//! Most modern applications use the newer PKCS#8 format instead (see below).
//!
//! The following traits can be used to decode/encode [`RsaPrivateKey`] and
//! [`RsaPublicKey`] as PKCS#1. Note that [`pkcs1`] is re-exported from the
//! toplevel of the `rsa` crate:
//!
//! - [`pkcs1::DecodeRsaPrivateKey`]: decode RSA private keys from PKCS#1
//! - [`pkcs1::EncodeRsaPrivateKey`]: encode RSA private keys to PKCS#1
//! - [`pkcs1::DecodeRsaPublicKey`]: decode RSA public keys from PKCS#1
//! - [`pkcs1::EncodeRsaPublicKey`]: encode RSA public keys to PKCS#1
//!
//! ### Example
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # #[cfg(all(feature = "pem", feature = "std"))]
//! # {
//! use rsa::{RsaPublicKey, pkcs1::DecodeRsaPublicKey};
//!
//! let pem = "-----BEGIN RSA PUBLIC KEY-----
//! MIIBCgKCAQEAtsQsUV8QpqrygsY+2+JCQ6Fw8/omM71IM2N/R8pPbzbgOl0p78MZ
//! GsgPOQ2HSznjD0FPzsH8oO2B5Uftws04LHb2HJAYlz25+lN5cqfHAfa3fgmC38Ff
//! wBkn7l582UtPWZ/wcBOnyCgb3yLcvJrXyrt8QxHJgvWO23ITrUVYszImbXQ67YGS
//! 0YhMrbixRzmo2tpm3JcIBtnHrEUMsT0NfFdfsZhTT8YbxBvA8FdODgEwx7u/vf3J
//! 9qbi4+Kv8cvqyJuleIRSjVXPsIMnoejIn04APPKIjpMyQdnWlby7rNyQtE4+CV+j
//! cFjqJbE/Xilcvqxt6DirjFCvYeKYl1uHLwIDAQAB
//! -----END RSA PUBLIC KEY-----";
//!
//! let public_key = RsaPublicKey::from_pkcs1_pem(pem)?;
//! # }
//! # Ok(())
//! # }
//! ```
//!
//! ## PKCS#8 RSA Key Encoding
//!
//! PKCS#8 is a private key format with support for multiple algorithms.
//! Like PKCS#1, it can be encoded as binary (DER) or text (PEM).
//!
//! You can recognize PEM encoded PKCS#8 keys because they *don't* have
//! an algorithm name in the type label, e.g.:
//!
//! ```text
//! -----BEGIN PRIVATE KEY-----
//! ```
//!
//! The following traits can be used to decode/encode [`RsaPrivateKey`] and
//! [`RsaPublicKey`] as PKCS#8. Note that [`pkcs8`] is re-exported from the
//! toplevel of the `rsa` crate:
//!
//! - [`pkcs8::DecodePrivateKey`]: decode private keys from PKCS#8
//! - [`pkcs8::EncodePrivateKey`]: encode private keys to PKCS#8
//! - [`pkcs8::DecodePublicKey`]: decode public keys from PKCS#8
//! - [`pkcs8::EncodePublicKey`]: encode public keys to PKCS#8
//!
//! ### Example
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # #[cfg(all(feature = "pem", feature = "std"))]
//! # {
//! use rsa::{RsaPublicKey, pkcs8::DecodePublicKey};
//!
//! let pem = "-----BEGIN PUBLIC KEY-----
//! MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAtsQsUV8QpqrygsY+2+JC
//! Q6Fw8/omM71IM2N/R8pPbzbgOl0p78MZGsgPOQ2HSznjD0FPzsH8oO2B5Uftws04
//! LHb2HJAYlz25+lN5cqfHAfa3fgmC38FfwBkn7l582UtPWZ/wcBOnyCgb3yLcvJrX
//! yrt8QxHJgvWO23ITrUVYszImbXQ67YGS0YhMrbixRzmo2tpm3JcIBtnHrEUMsT0N
//! fFdfsZhTT8YbxBvA8FdODgEwx7u/vf3J9qbi4+Kv8cvqyJuleIRSjVXPsIMnoejI
//! n04APPKIjpMyQdnWlby7rNyQtE4+CV+jcFjqJbE/Xilcvqxt6DirjFCvYeKYl1uH
//! LwIDAQAB
//! -----END PUBLIC KEY-----";
//!
//! let public_key = RsaPublicKey::from_public_key_pem(pem)?;
//! # }
//! # Ok(())
//! # }
//! ```
//!
//! [RFC8017]: https://datatracker.ietf.org/doc/html/rfc8017#section-8.1
//!
// TODO(tarcieri): figure out why rustdoc isn't rendering these links correctly
//! [`pkcs8::DecodePublicKey`]: https://docs.rs/pkcs8/latest/pkcs8/trait.DecodePublicKey.html
//! [`pkcs8::EncodePublicKey`]: https://docs.rs/pkcs8/latest/pkcs8/trait.EncodePublicKey.html

#[cfg(doctest)]
pub struct ReadmeDoctests;

#[macro_use]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub use num_bigint::BigUint;
pub use rand_core;
pub use signature;

mod algorithms;
pub mod errors;
pub mod oaep;
pub mod pkcs1v15;
pub mod pss;
pub mod traits;

mod dummy_rng;
mod encoding;
mod key;

pub use pkcs1;
pub use pkcs8;
#[cfg(feature = "sha2")]
pub use sha2;

pub use crate::{
    errors::{Error, Result},
    key::{RsaPrivateKey, RsaPublicKey},
    oaep::Oaep,
    pkcs1v15::{Pkcs1v15Encrypt, Pkcs1v15Sign},
    pss::Pss,
    traits::keys::CrtValue,
};

#[cfg(feature = "hazmat")]
pub mod hazmat;
