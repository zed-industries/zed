//! Mask generation function common to both PSS and OAEP padding

use digest::{Digest, DynDigest, FixedOutputReset};

/// Mask generation function.
///
/// Panics if out is larger than 2**32. This is in accordance with RFC 8017 - PKCS #1 B.2.1
pub(crate) fn mgf1_xor(out: &mut [u8], digest: &mut dyn DynDigest, seed: &[u8]) {
    let mut counter = [0u8; 4];
    let mut i = 0;

    const MAX_LEN: u64 = core::u32::MAX as u64 + 1;
    assert!(out.len() as u64 <= MAX_LEN);

    while i < out.len() {
        let mut digest_input = vec![0u8; seed.len() + 4];
        digest_input[0..seed.len()].copy_from_slice(seed);
        digest_input[seed.len()..].copy_from_slice(&counter);

        digest.update(digest_input.as_slice());
        let digest_output = &*digest.finalize_reset();
        let mut j = 0;
        loop {
            if j >= digest_output.len() || i >= out.len() {
                break;
            }

            out[i] ^= digest_output[j];
            j += 1;
            i += 1;
        }
        inc_counter(&mut counter);
    }
}

/// Mask generation function.
///
/// Panics if out is larger than 2**32. This is in accordance with RFC 8017 - PKCS #1 B.2.1
pub(crate) fn mgf1_xor_digest<D>(out: &mut [u8], digest: &mut D, seed: &[u8])
where
    D: Digest + FixedOutputReset,
{
    let mut counter = [0u8; 4];
    let mut i = 0;

    const MAX_LEN: u64 = core::u32::MAX as u64 + 1;
    assert!(out.len() as u64 <= MAX_LEN);

    while i < out.len() {
        Digest::update(digest, seed);
        Digest::update(digest, counter);

        let digest_output = digest.finalize_reset();
        let mut j = 0;
        loop {
            if j >= digest_output.len() || i >= out.len() {
                break;
            }

            out[i] ^= digest_output[j];
            j += 1;
            i += 1;
        }
        inc_counter(&mut counter);
    }
}
fn inc_counter(counter: &mut [u8; 4]) {
    for i in (0..4).rev() {
        counter[i] = counter[i].wrapping_add(1);
        if counter[i] != 0 {
            // No overflow
            return;
        }
    }
}
