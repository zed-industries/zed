//! Encryption and Decryption using [OAEP padding](https://datatracker.ietf.org/doc/html/rfc8017#section-7.1).
//!
use alloc::string::String;
use alloc::vec::Vec;

use digest::{Digest, DynDigest, FixedOutputReset};
use rand_core::CryptoRngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::Zeroizing;

use super::mgf::{mgf1_xor, mgf1_xor_digest};
use crate::errors::{Error, Result};

// 2**61 -1 (pow is not const yet)
// TODO: This is the maximum for SHA-1, unclear from the RFC what the values are for other hashing functions.
const MAX_LABEL_LEN: u64 = 2_305_843_009_213_693_951;

#[inline]
fn encrypt_internal<R: CryptoRngCore + ?Sized, MGF: FnMut(&mut [u8], &mut [u8])>(
    rng: &mut R,
    msg: &[u8],
    p_hash: &[u8],
    h_size: usize,
    k: usize,
    mut mgf: MGF,
) -> Result<Zeroizing<Vec<u8>>> {
    if msg.len() + 2 * h_size + 2 > k {
        return Err(Error::MessageTooLong);
    }

    let mut em = Zeroizing::new(vec![0u8; k]);

    let (_, payload) = em.split_at_mut(1);
    let (seed, db) = payload.split_at_mut(h_size);
    rng.fill_bytes(seed);

    // Data block DB =  pHash || PS || 01 || M
    let db_len = k - h_size - 1;

    db[0..h_size].copy_from_slice(p_hash);
    db[db_len - msg.len() - 1] = 1;
    db[db_len - msg.len()..].copy_from_slice(msg);

    mgf(seed, db);

    Ok(em)
}

/// Encrypts the given message with RSA and the padding scheme from
/// [PKCS#1 OAEP].
///
/// The message must be no longer than the length of the public modulus minus
/// `2 + (2 * hash.size())`.
///
/// [PKCS#1 OAEP]: https://datatracker.ietf.org/doc/html/rfc8017#section-7.1
#[inline]
pub(crate) fn oaep_encrypt<R: CryptoRngCore + ?Sized>(
    rng: &mut R,
    msg: &[u8],
    digest: &mut dyn DynDigest,
    mgf_digest: &mut dyn DynDigest,
    label: Option<String>,
    k: usize,
) -> Result<Zeroizing<Vec<u8>>> {
    let h_size = digest.output_size();

    let label = label.unwrap_or_default();
    if label.len() as u64 > MAX_LABEL_LEN {
        return Err(Error::LabelTooLong);
    }

    digest.update(label.as_bytes());
    let p_hash = digest.finalize_reset();

    encrypt_internal(rng, msg, &p_hash, h_size, k, |seed, db| {
        mgf1_xor(db, mgf_digest, seed);
        mgf1_xor(seed, mgf_digest, db);
    })
}

/// Encrypts the given message with RSA and the padding scheme from
/// [PKCS#1 OAEP].
///
/// The message must be no longer than the length of the public modulus minus
/// `2 + (2 * hash.size())`.
///
/// [PKCS#1 OAEP]: https://datatracker.ietf.org/doc/html/rfc8017#section-7.1
#[inline]
pub(crate) fn oaep_encrypt_digest<
    R: CryptoRngCore + ?Sized,
    D: Digest,
    MGD: Digest + FixedOutputReset,
>(
    rng: &mut R,
    msg: &[u8],
    label: Option<String>,
    k: usize,
) -> Result<Zeroizing<Vec<u8>>> {
    let h_size = <D as Digest>::output_size();

    let label = label.unwrap_or_default();
    if label.len() as u64 > MAX_LABEL_LEN {
        return Err(Error::LabelTooLong);
    }

    let p_hash = D::digest(label.as_bytes());

    encrypt_internal(rng, msg, &p_hash, h_size, k, |seed, db| {
        let mut mgf_digest = MGD::new();
        mgf1_xor_digest(db, &mut mgf_digest, seed);
        mgf1_xor_digest(seed, &mut mgf_digest, db);
    })
}

///Decrypts OAEP padding.
///
/// Note that whether this function returns an error or not discloses secret
/// information. If an attacker can cause this function to run repeatedly and
/// learn whether each instance returned an error then they can decrypt and
/// forge signatures as if they had the private key.
///
/// See `decrypt_session_key` for a way of solving this problem.
///
/// [PKCS#1 OAEP]: https://datatracker.ietf.org/doc/html/rfc8017#section-7.1
#[inline]
pub(crate) fn oaep_decrypt(
    em: &mut [u8],
    digest: &mut dyn DynDigest,
    mgf_digest: &mut dyn DynDigest,
    label: Option<String>,
    k: usize,
) -> Result<Vec<u8>> {
    let h_size = digest.output_size();

    let label = label.unwrap_or_default();
    if label.len() as u64 > MAX_LABEL_LEN {
        return Err(Error::Decryption);
    }

    digest.update(label.as_bytes());

    let expected_p_hash = digest.finalize_reset();

    let res = decrypt_inner(em, h_size, &expected_p_hash, k, |seed, db| {
        mgf1_xor(seed, mgf_digest, db);
        mgf1_xor(db, mgf_digest, seed);
    })?;
    if res.is_none().into() {
        return Err(Error::Decryption);
    }

    let (out, index) = res.unwrap();

    Ok(out[index as usize..].to_vec())
}

///Decrypts OAEP padding.
///
/// Note that whether this function returns an error or not discloses secret
/// information. If an attacker can cause this function to run repeatedly and
/// learn whether each instance returned an error then they can decrypt and
/// forge signatures as if they had the private key.
///
/// See `decrypt_session_key` for a way of solving this problem.
///
/// [PKCS#1 OAEP]: https://datatracker.ietf.org/doc/html/rfc8017#section-7.1
#[inline]
pub(crate) fn oaep_decrypt_digest<D: Digest, MGD: Digest + FixedOutputReset>(
    em: &mut [u8],
    label: Option<String>,
    k: usize,
) -> Result<Vec<u8>> {
    let h_size = <D as Digest>::output_size();

    let label = label.unwrap_or_default();
    if label.len() as u64 > MAX_LABEL_LEN {
        return Err(Error::LabelTooLong);
    }

    let expected_p_hash = D::digest(label.as_bytes());

    let res = decrypt_inner(em, h_size, &expected_p_hash, k, |seed, db| {
        let mut mgf_digest = MGD::new();
        mgf1_xor_digest(seed, &mut mgf_digest, db);
        mgf1_xor_digest(db, &mut mgf_digest, seed);
    })?;
    if res.is_none().into() {
        return Err(Error::Decryption);
    }

    let (out, index) = res.unwrap();

    Ok(out[index as usize..].to_vec())
}

/// Decrypts OAEP padding. It returns one or zero in valid that indicates whether the
/// plaintext was correctly structured.
#[inline]
fn decrypt_inner<MGF: FnMut(&mut [u8], &mut [u8])>(
    em: &mut [u8],
    h_size: usize,
    expected_p_hash: &[u8],
    k: usize,
    mut mgf: MGF,
) -> Result<CtOption<(Vec<u8>, u32)>> {
    if k < 11 {
        return Err(Error::Decryption);
    }

    if k < h_size * 2 + 2 {
        return Err(Error::Decryption);
    }

    let first_byte_is_zero = em[0].ct_eq(&0u8);

    let (_, payload) = em.split_at_mut(1);
    let (seed, db) = payload.split_at_mut(h_size);

    mgf(seed, db);

    let hash_are_equal = db[0..h_size].ct_eq(expected_p_hash);

    // The remainder of the plaintext must be zero or more 0x00, followed
    // by 0x01, followed by the message.
    //   looking_for_index: 1 if we are still looking for the 0x01
    //   index: the offset of the first 0x01 byte
    //   zero_before_one: 1 if we saw a non-zero byte before the 1
    let mut looking_for_index = Choice::from(1u8);
    let mut index = 0u32;
    let mut nonzero_before_one = Choice::from(0u8);

    for (i, el) in db.iter().skip(h_size).enumerate() {
        let equals0 = el.ct_eq(&0u8);
        let equals1 = el.ct_eq(&1u8);
        index.conditional_assign(&(i as u32), looking_for_index & equals1);
        looking_for_index &= !equals1;
        nonzero_before_one |= looking_for_index & !equals0;
    }

    let valid = first_byte_is_zero & hash_are_equal & !nonzero_before_one & !looking_for_index;

    Ok(CtOption::new(
        (em.to_vec(), index + 2 + (h_size * 2) as u32),
        valid,
    ))
}
