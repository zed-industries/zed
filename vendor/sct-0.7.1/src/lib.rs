//! # SCT.rs: SCT verification library
//! This library implements verification of Signed Certificate Timestamps.
//! These are third-party assurances that a particular certificate has
//! been included in a Certificate Transparency log.
//!
//! See RFC6962 for the details of the formats implemented here.
//!
//! It is intended to be useful to libraries which perform certificate
//! validation, OCSP libraries, and TLS libraries.

#![forbid(unsafe_code, unstable_features)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    missing_docs,
    unused_import_braces,
    unused_extern_crates,
    unused_qualifications
)]
#![no_std]

extern crate alloc;

use alloc::{vec, vec::Vec};

/// Describes a CT log
///
/// This structure contains some metadata fields not used by the library.
/// Rationale: it makes sense to keep this metadata with the other
/// values for review purposes.
#[derive(Debug)]
pub struct Log<'a> {
    /// The operator's name/description of the log.
    /// This field is not used by the library.
    pub description: &'a str,

    /// The certificate submission url.
    /// This field is not used by the library.
    pub url: &'a str,

    /// Which entity operates the log.
    /// This field is not used by the library.
    pub operated_by: &'a str,

    /// Public key usable for verifying certificates.
    /// TODO: fixme format of this; should be a SPKI
    /// so the `id` is verifiable, but currently is a
    /// raw public key (like, an ECPoint or RSAPublicKey).
    pub key: &'a [u8],

    /// Key hash, which is SHA256 applied to the SPKI
    /// encoding.
    pub id: [u8; 32],

    /// The log's maximum merge delay.
    /// This field is not used by the library.
    pub max_merge_delay: usize,
}

/// How sct.rs reports errors.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error {
    /// The SCT was somehow misencoded, truncated or otherwise corrupt.
    MalformedSct,

    /// The SCT contained an invalid signature.
    InvalidSignature,

    /// The SCT was signed in the future.  Clock skew?
    TimestampInFuture,

    /// The SCT had a version that this library does not handle.
    UnsupportedSctVersion,

    /// The SCT was refers to an unknown log.
    UnknownLog,
}

impl Error {
    /// Applies a suggested policy for error handling:
    ///
    /// Returns `true` if the error should end processing
    /// for whatever the SCT is attached to (like, abort a TLS
    /// handshake).
    ///
    /// Returns `false` if this error should be a 'soft failure'
    /// -- the SCT is unverifiable with this library and set of
    /// logs.
    pub fn should_be_fatal(&self) -> bool {
        !matches!(self, Error::UnknownLog | Error::UnsupportedSctVersion)
    }
}

fn lookup(logs: &[&Log], id: &[u8]) -> Result<usize, Error> {
    for (i, l) in logs.iter().enumerate() {
        if id == l.id {
            return Ok(i);
        }
    }

    Err(Error::UnknownLog)
}

fn decode_u64(inp: untrusted::Input) -> u64 {
    let b = inp.as_slice_less_safe();
    assert_eq!(b.len(), 8);
    (b[0] as u64) << 56
        | (b[1] as u64) << 48
        | (b[2] as u64) << 40
        | (b[3] as u64) << 32
        | (b[4] as u64) << 24
        | (b[5] as u64) << 16
        | (b[6] as u64) << 8
        | (b[7] as u64)
}

fn decode_u16(inp: untrusted::Input) -> u16 {
    let b = inp.as_slice_less_safe();
    assert_eq!(b.len(), 2);
    (b[0] as u16) << 8 | (b[1] as u16)
}

fn write_u64(v: u64, out: &mut Vec<u8>) {
    out.push((v >> 56) as u8);
    out.push((v >> 48) as u8);
    out.push((v >> 40) as u8);
    out.push((v >> 32) as u8);
    out.push((v >> 24) as u8);
    out.push((v >> 16) as u8);
    out.push((v >> 8) as u8);
    out.push(v as u8);
}

fn write_u24(v: u32, out: &mut Vec<u8>) {
    out.push((v >> 16) as u8);
    out.push((v >> 8) as u8);
    out.push(v as u8);
}

fn write_u16(v: u16, out: &mut Vec<u8>) {
    out.push((v >> 8) as u8);
    out.push(v as u8);
}

struct Sct<'a> {
    log_id: &'a [u8],
    timestamp: u64,
    sig_alg: u16,
    sig: &'a [u8],
    exts: &'a [u8],
}

const ECDSA_SHA256: u16 = 0x0403;
const ECDSA_SHA384: u16 = 0x0503;
const RSA_PKCS1_SHA256: u16 = 0x0401;
const RSA_PKCS1_SHA384: u16 = 0x0501;
const SCT_V1: u8 = 0u8;
const SCT_TIMESTAMP: u8 = 0u8;
const SCT_X509_ENTRY: [u8; 2] = [0, 0];

impl<'a> Sct<'a> {
    fn verify(&self, key: &[u8], cert: &[u8]) -> Result<(), Error> {
        let alg: &dyn ring::signature::VerificationAlgorithm = match self.sig_alg {
            ECDSA_SHA256 => &ring::signature::ECDSA_P256_SHA256_ASN1,
            ECDSA_SHA384 => &ring::signature::ECDSA_P384_SHA384_ASN1,
            RSA_PKCS1_SHA256 => &ring::signature::RSA_PKCS1_2048_8192_SHA256,
            RSA_PKCS1_SHA384 => &ring::signature::RSA_PKCS1_2048_8192_SHA384,
            _ => return Err(Error::InvalidSignature),
        };

        let mut data = vec![SCT_V1, SCT_TIMESTAMP];
        write_u64(self.timestamp, &mut data);
        data.extend_from_slice(&SCT_X509_ENTRY);
        write_u24(cert.len() as u32, &mut data);
        data.extend_from_slice(cert);
        write_u16(self.exts.len() as u16, &mut data);
        data.extend_from_slice(self.exts);

        let key = ring::signature::UnparsedPublicKey::new(alg, key);

        key.verify(&data, self.sig)
            .map_err(|_| Error::InvalidSignature)
    }

    fn parse(enc: &'a [u8]) -> Result<Sct<'a>, Error> {
        let inp = untrusted::Input::from(enc);

        inp.read_all(Error::MalformedSct, |rd| {
            let version = rd.read_byte().map_err(|_| Error::MalformedSct)?;
            if version != 0 {
                return Err(Error::UnsupportedSctVersion);
            }

            let id = rd.read_bytes(32).map_err(|_| Error::MalformedSct)?;
            let timestamp = rd
                .read_bytes(8)
                .map_err(|_| Error::MalformedSct)
                .map(decode_u64)?;

            let ext_len = rd
                .read_bytes(2)
                .map_err(|_| Error::MalformedSct)
                .map(decode_u16)?;
            let exts = rd
                .read_bytes(ext_len as usize)
                .map_err(|_| Error::MalformedSct)?;

            let sig_alg = rd
                .read_bytes(2)
                .map_err(|_| Error::MalformedSct)
                .map(decode_u16)?;
            let sig_len = rd
                .read_bytes(2)
                .map_err(|_| Error::MalformedSct)
                .map(decode_u16)?;
            let sig = rd
                .read_bytes(sig_len as usize)
                .map_err(|_| Error::MalformedSct)?;

            let ret = Sct {
                log_id: id.as_slice_less_safe(),
                timestamp,
                sig_alg,
                sig: sig.as_slice_less_safe(),
                exts: exts.as_slice_less_safe(),
            };

            Ok(ret)
        })
    }
}

/// Verifies that the SCT `sct` (a `SignedCertificateTimestamp` encoding)
/// is a correctly signed timestamp for `cert` (a DER-encoded X.509 end-entity
/// certificate) valid `at_time`.  `logs` describe the CT logs trusted by
/// the caller to sign such an SCT.
///
/// On success, this function returns the log used as an index into `logs`.
/// Otherwise, it returns an `Error`.
pub fn verify_sct(cert: &[u8], sct: &[u8], at_time: u64, logs: &[&Log]) -> Result<usize, Error> {
    let sct = Sct::parse(sct)?;
    let i = lookup(logs, sct.log_id)?;
    let log = logs[i];
    sct.verify(log.key, cert)?;

    if sct.timestamp > at_time {
        return Err(Error::TimestampInFuture);
    }

    Ok(i)
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;
#[cfg(test)]
mod tests_google;
