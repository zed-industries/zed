//! # rustls-pemfile
//! A basic parser for .pem files containing cryptographic keys and certificates.
//!
//! The input to this crate is a .pem file containing potentially many sections,
//! and the output is those sections as alleged DER-encodings.  This crate does
//! not decode the actual DER-encoded keys/certificates.
//!
//! ## Quick start
//! Starting with an `io::BufRead` containing the file to be read:
//! - Use `read_all()` to ingest the whole file, then work through the contents in-memory, or,
//! - Use `read_one()` to stream through the file, processing the items as found, or,
//! - Use `certs()` to extract just the certificates (silently discarding other sections), and
//!   similarly for `rsa_private_keys()` and `pkcs8_private_keys()`.
//!
//! ## Example code
//! ```
//! use std::iter;
//! use rustls_pemfile::{Item, read_one};
//! # let mut reader = std::io::BufReader::new(&b"junk\n-----BEGIN RSA PRIVATE KEY-----\nqw\n-----END RSA PRIVATE KEY-----\n"[..]);
//! // Assume `reader` is any std::io::BufRead implementor
//! for item in iter::from_fn(|| read_one(&mut reader).transpose()) {
//!     match item.unwrap() {
//!         Item::X509Certificate(cert) => println!("certificate {:?}", cert),
//!         Item::Crl(crl) => println!("certificate revocation list: {:?}", crl),
//!         Item::RSAKey(key) => println!("rsa pkcs1 key {:?}", key),
//!         Item::PKCS8Key(key) => println!("pkcs8 key {:?}", key),
//!         Item::ECKey(key) => println!("sec1 ec key {:?}", key),
//!         _ => println!("unhandled item"),
//!     }
//! }
//! ```

// Require docs for public APIs, deny unsafe code, etc.
#![forbid(unsafe_code, unused_must_use, unstable_features)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    missing_docs,
    unused_import_braces,
    unused_extern_crates,
    unused_qualifications
)]

#[cfg(test)]
mod tests;

/// --- Main crate APIs:
mod pemfile;
pub use pemfile::{read_all, read_one, Item};

/// --- Legacy APIs:
use std::io;

/// Extract all the certificates from `rd`, and return a vec of byte vecs
/// containing the der-format contents.
///
/// This function does not fail if there are no certificates in the file --
/// it returns an empty vector.
pub fn certs(rd: &mut dyn io::BufRead) -> Result<Vec<Vec<u8>>, io::Error> {
    let mut certs = Vec::new();

    loop {
        match read_one(rd)? {
            None => return Ok(certs),
            Some(Item::X509Certificate(cert)) => certs.push(cert),
            _ => {}
        };
    }
}

/// Extract all the certificate revocation lists (CRLs) from `rd`, and return a vec of byte vecs
/// containing the der-format contents.
///
/// This function does not fail if there are no CRLs in the file --
/// it returns an empty vector.
pub fn crls(rd: &mut dyn io::BufRead) -> Result<Vec<Vec<u8>>, io::Error> {
    let mut crls = Vec::new();

    loop {
        match read_one(rd)? {
            None => return Ok(crls),
            Some(Item::Crl(crl)) => crls.push(crl),
            _ => {}
        };
    }
}

/// Extract all RSA private keys from `rd`, and return a vec of byte vecs
/// containing the der-format contents.
///
/// This function does not fail if there are no keys in the file -- it returns an
/// empty vector.
pub fn rsa_private_keys(rd: &mut dyn io::BufRead) -> Result<Vec<Vec<u8>>, io::Error> {
    let mut keys = Vec::new();

    loop {
        match read_one(rd)? {
            None => return Ok(keys),
            Some(Item::RSAKey(key)) => keys.push(key),
            _ => {}
        };
    }
}

/// Extract all PKCS8-encoded private keys from `rd`, and return a vec of
/// byte vecs containing the der-format contents.
///
/// This function does not fail if there are no keys in the file -- it returns an
/// empty vector.
pub fn pkcs8_private_keys(rd: &mut dyn io::BufRead) -> Result<Vec<Vec<u8>>, io::Error> {
    let mut keys = Vec::new();

    loop {
        match read_one(rd)? {
            None => return Ok(keys),
            Some(Item::PKCS8Key(key)) => keys.push(key),
            _ => {}
        };
    }
}

/// Extract all SEC1-encoded EC private keys from `rd`, and return a vec of
/// byte vecs containing the der-format contents.
///
/// This function does not fail if there are no keys in the file -- it returns an
/// empty vector.
pub fn ec_private_keys(rd: &mut dyn io::BufRead) -> Result<Vec<Vec<u8>>, io::Error> {
    let mut keys = Vec::new();

    loop {
        match read_one(rd)? {
            None => return Ok(keys),
            Some(Item::ECKey(key)) => keys.push(key),
            _ => {}
        };
    }
}
