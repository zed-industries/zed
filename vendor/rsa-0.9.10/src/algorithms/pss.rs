//! Support for the [Probabilistic Signature Scheme] (PSS) a.k.a. RSASSA-PSS.
//!
//! Designed by Mihir Bellare and Phillip Rogaway. Specified in [RFC8017 § 8.1].
//!
//! # Usage
//!
//! See [code example in the toplevel rustdoc](../index.html#pss-signatures).
//!
//! [Probabilistic Signature Scheme]: https://en.wikipedia.org/wiki/Probabilistic_signature_scheme
//! [RFC8017 § 8.1]: https://datatracker.ietf.org/doc/html/rfc8017#section-8.1

use alloc::vec::Vec;
use digest::{Digest, DynDigest, FixedOutputReset};
use subtle::{Choice, ConstantTimeEq};

use super::mgf::{mgf1_xor, mgf1_xor_digest};
use crate::errors::{Error, Result};

pub(crate) fn emsa_pss_encode(
    m_hash: &[u8],
    em_bits: usize,
    salt: &[u8],
    hash: &mut dyn DynDigest,
) -> Result<Vec<u8>> {
    // See [1], section 9.1.1
    let h_len = hash.output_size();
    let s_len = salt.len();
    let em_len = (em_bits + 7) / 8;

    // 1. If the length of M is greater than the input limitation for the
    //     hash function (2^61 - 1 octets for SHA-1), output "message too
    //     long" and stop.
    //
    // 2.  Let mHash = Hash(M), an octet string of length hLen.
    if m_hash.len() != h_len {
        return Err(Error::InputNotHashed);
    }

    // 3. If em_len < h_len + s_len + 2, output "encoding error" and stop.
    if em_len < h_len + s_len + 2 {
        // TODO: Key size too small
        return Err(Error::Internal);
    }

    let mut em = vec![0; em_len];

    let (db, h) = em.split_at_mut(em_len - h_len - 1);
    let h = &mut h[..(em_len - 1) - db.len()];

    // 4. Generate a random octet string salt of length s_len; if s_len = 0,
    //     then salt is the empty string.
    //
    // 5.  Let
    //       M' = (0x)00 00 00 00 00 00 00 00 || m_hash || salt;
    //
    //     M' is an octet string of length 8 + h_len + s_len with eight
    //     initial zero octets.
    //
    // 6.  Let H = Hash(M'), an octet string of length h_len.
    let prefix = [0u8; 8];

    hash.update(&prefix);
    hash.update(m_hash);
    hash.update(salt);

    let hashed = hash.finalize_reset();
    h.copy_from_slice(&hashed);

    // 7.  Generate an octet string PS consisting of em_len - s_len - h_len - 2
    //     zero octets. The length of PS may be 0.
    //
    // 8.  Let DB = PS || 0x01 || salt; DB is an octet string of length
    //     emLen - hLen - 1.
    db[em_len - s_len - h_len - 2] = 0x01;
    db[em_len - s_len - h_len - 1..].copy_from_slice(salt);

    // 9.  Let dbMask = MGF(H, emLen - hLen - 1).
    //
    // 10. Let maskedDB = DB \xor dbMask.
    mgf1_xor(db, hash, h);

    // 11. Set the leftmost 8 * em_len - em_bits bits of the leftmost octet in
    //     maskedDB to zero.
    db[0] &= 0xFF >> (8 * em_len - em_bits);

    // 12. Let EM = maskedDB || H || 0xbc.
    em[em_len - 1] = 0xBC;

    Ok(em)
}

pub(crate) fn emsa_pss_encode_digest<D>(
    m_hash: &[u8],
    em_bits: usize,
    salt: &[u8],
) -> Result<Vec<u8>>
where
    D: Digest + FixedOutputReset,
{
    // See [1], section 9.1.1
    let h_len = <D as Digest>::output_size();
    let s_len = salt.len();
    let em_len = (em_bits + 7) / 8;

    // 1. If the length of M is greater than the input limitation for the
    //     hash function (2^61 - 1 octets for SHA-1), output "message too
    //     long" and stop.
    //
    // 2.  Let mHash = Hash(M), an octet string of length hLen.
    if m_hash.len() != h_len {
        return Err(Error::InputNotHashed);
    }

    // 3. If em_len < h_len + s_len + 2, output "encoding error" and stop.
    if em_len < h_len + s_len + 2 {
        // TODO: Key size too small
        return Err(Error::Internal);
    }

    let mut em = vec![0; em_len];

    let (db, h) = em.split_at_mut(em_len - h_len - 1);
    let h = &mut h[..(em_len - 1) - db.len()];

    // 4. Generate a random octet string salt of length s_len; if s_len = 0,
    //     then salt is the empty string.
    //
    // 5.  Let
    //       M' = (0x)00 00 00 00 00 00 00 00 || m_hash || salt;
    //
    //     M' is an octet string of length 8 + h_len + s_len with eight
    //     initial zero octets.
    //
    // 6.  Let H = Hash(M'), an octet string of length h_len.
    let prefix = [0u8; 8];

    let mut hash = D::new();

    Digest::update(&mut hash, prefix);
    Digest::update(&mut hash, m_hash);
    Digest::update(&mut hash, salt);

    let hashed = hash.finalize_reset();
    h.copy_from_slice(&hashed);

    // 7.  Generate an octet string PS consisting of em_len - s_len - h_len - 2
    //     zero octets. The length of PS may be 0.
    //
    // 8.  Let DB = PS || 0x01 || salt; DB is an octet string of length
    //     emLen - hLen - 1.
    db[em_len - s_len - h_len - 2] = 0x01;
    db[em_len - s_len - h_len - 1..].copy_from_slice(salt);

    // 9.  Let dbMask = MGF(H, emLen - hLen - 1).
    //
    // 10. Let maskedDB = DB \xor dbMask.
    mgf1_xor_digest(db, &mut hash, h);

    // 11. Set the leftmost 8 * em_len - em_bits bits of the leftmost octet in
    //     maskedDB to zero.
    db[0] &= 0xFF >> (8 * em_len - em_bits);

    // 12. Let EM = maskedDB || H || 0xbc.
    em[em_len - 1] = 0xBC;

    Ok(em)
}

fn emsa_pss_verify_pre<'a>(
    m_hash: &[u8],
    em: &'a mut [u8],
    em_bits: usize,
    s_len: usize,
    h_len: usize,
) -> Result<(&'a mut [u8], &'a mut [u8])> {
    // 1. If the length of M is greater than the input limitation for the
    //    hash function (2^61 - 1 octets for SHA-1), output "inconsistent"
    //    and stop.
    //
    // 2. Let mHash = Hash(M), an octet string of length hLen
    if m_hash.len() != h_len {
        return Err(Error::Verification);
    }

    // 3. If emLen < hLen + sLen + 2, output "inconsistent" and stop.
    let em_len = em.len(); //(em_bits + 7) / 8;
    if em_len < h_len + s_len + 2 {
        return Err(Error::Verification);
    }

    // 4. If the rightmost octet of EM does not have hexadecimal value
    //    0xbc, output "inconsistent" and stop.
    if em[em.len() - 1] != 0xBC {
        return Err(Error::Verification);
    }

    // 5. Let maskedDB be the leftmost emLen - hLen - 1 octets of EM, and
    //    let H be the next hLen octets.
    let (db, h) = em.split_at_mut(em_len - h_len - 1);
    let h = &mut h[..h_len];

    // 6. If the leftmost 8 * em_len - em_bits bits of the leftmost octet in
    //    maskedDB are not all equal to zero, output "inconsistent" and
    //    stop.
    if db[0]
        & (0xFF_u8
            .checked_shl(8 - (8 * em_len - em_bits) as u32)
            .unwrap_or(0))
        != 0
    {
        return Err(Error::Verification);
    }

    Ok((db, h))
}

fn emsa_pss_verify_salt(db: &[u8], em_len: usize, s_len: usize, h_len: usize) -> Choice {
    // 10. If the emLen - hLen - sLen - 2 leftmost octets of DB are not zero
    //     or if the octet at position emLen - hLen - sLen - 1 (the leftmost
    //     position is "position 1") does not have hexadecimal value 0x01,
    //     output "inconsistent" and stop.
    let (zeroes, rest) = db.split_at(em_len - h_len - s_len - 2);
    let valid: Choice = zeroes
        .iter()
        .fold(Choice::from(1u8), |a, e| a & e.ct_eq(&0x00));

    valid & rest[0].ct_eq(&0x01)
}

pub(crate) fn emsa_pss_verify(
    m_hash: &[u8],
    em: &mut [u8],
    s_len: usize,
    hash: &mut dyn DynDigest,
    key_bits: usize,
) -> Result<()> {
    let em_bits = key_bits - 1;
    let em_len = (em_bits + 7) / 8;
    let key_len = (key_bits + 7) / 8;
    let h_len = hash.output_size();

    let em = &mut em[key_len - em_len..];

    let (db, h) = emsa_pss_verify_pre(m_hash, em, em_bits, s_len, h_len)?;

    // 7. Let dbMask = MGF(H, em_len - h_len - 1)
    //
    // 8. Let DB = maskedDB \xor dbMask
    mgf1_xor(db, hash, &*h);

    // 9.  Set the leftmost 8 * emLen - emBits bits of the leftmost octet in DB
    //     to zero.
    db[0] &= 0xFF >> /*uint*/(8 * em_len - em_bits);

    let salt_valid = emsa_pss_verify_salt(db, em_len, s_len, h_len);

    // 11. Let salt be the last s_len octets of DB.
    let salt = &db[db.len() - s_len..];

    // 12. Let
    //          M' = (0x)00 00 00 00 00 00 00 00 || mHash || salt ;
    //     M' is an octet string of length 8 + hLen + sLen with eight
    //     initial zero octets.
    //
    // 13. Let H' = Hash(M'), an octet string of length hLen.
    let prefix = [0u8; 8];

    hash.update(&prefix[..]);
    hash.update(m_hash);
    hash.update(salt);
    let h0 = hash.finalize_reset();

    // 14. If H = H', output "consistent." Otherwise, output "inconsistent."
    if (salt_valid & h0.ct_eq(h)).into() {
        Ok(())
    } else {
        Err(Error::Verification)
    }
}

pub(crate) fn emsa_pss_verify_digest<D>(
    m_hash: &[u8],
    em: &mut [u8],
    s_len: usize,
    key_bits: usize,
) -> Result<()>
where
    D: Digest + FixedOutputReset,
{
    let em_bits = key_bits - 1;
    let em_len = (em_bits + 7) / 8;
    let key_len = (key_bits + 7) / 8;
    let h_len = <D as Digest>::output_size();

    let em = &mut em[key_len - em_len..];

    let (db, h) = emsa_pss_verify_pre(m_hash, em, em_bits, s_len, h_len)?;

    let mut hash = D::new();

    // 7. Let dbMask = MGF(H, em_len - h_len - 1)
    //
    // 8. Let DB = maskedDB \xor dbMask
    mgf1_xor_digest::<D>(db, &mut hash, &*h);

    // 9.  Set the leftmost 8 * emLen - emBits bits of the leftmost octet in DB
    //     to zero.
    db[0] &= 0xFF >> /*uint*/(8 * em_len - em_bits);

    let salt_valid = emsa_pss_verify_salt(db, em_len, s_len, h_len);

    // 11. Let salt be the last s_len octets of DB.
    let salt = &db[db.len() - s_len..];

    // 12. Let
    //          M' = (0x)00 00 00 00 00 00 00 00 || mHash || salt ;
    //     M' is an octet string of length 8 + hLen + sLen with eight
    //     initial zero octets.
    //
    // 13. Let H' = Hash(M'), an octet string of length hLen.
    let prefix = [0u8; 8];

    Digest::update(&mut hash, &prefix[..]);
    Digest::update(&mut hash, m_hash);
    Digest::update(&mut hash, salt);
    let h0 = hash.finalize_reset();

    // 14. If H = H', output "consistent." Otherwise, output "inconsistent."
    if (salt_valid & h0.ct_eq(h)).into() {
        Ok(())
    } else {
        Err(Error::Verification)
    }
}
