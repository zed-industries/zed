//! Special handling for converting the BigUint to u8 vectors

use alloc::vec::Vec;
use num_bigint::BigUint;
use zeroize::Zeroizing;

use crate::errors::{Error, Result};

/// Returns a new vector of the given length, with 0s left padded.
#[inline]
fn left_pad(input: &[u8], padded_len: usize) -> Result<Vec<u8>> {
    if input.len() > padded_len {
        return Err(Error::InvalidPadLen);
    }

    let mut out = vec![0u8; padded_len];
    out[padded_len - input.len()..].copy_from_slice(input);
    Ok(out)
}

/// Converts input to the new vector of the given length, using BE and with 0s left padded.
#[inline]
pub(crate) fn uint_to_be_pad(input: BigUint, padded_len: usize) -> Result<Vec<u8>> {
    left_pad(&input.to_bytes_be(), padded_len)
}

/// Converts input to the new vector of the given length, using BE and with 0s left padded.
#[inline]
pub(crate) fn uint_to_zeroizing_be_pad(input: BigUint, padded_len: usize) -> Result<Vec<u8>> {
    let m = Zeroizing::new(input);
    let m = Zeroizing::new(m.to_bytes_be());
    left_pad(&m, padded_len)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_left_pad() {
        const INPUT_LEN: usize = 3;
        let input = vec![0u8; INPUT_LEN];

        // input len < padded len
        let padded = left_pad(&input, INPUT_LEN + 1).unwrap();
        assert_eq!(padded.len(), INPUT_LEN + 1);

        // input len == padded len
        let padded = left_pad(&input, INPUT_LEN).unwrap();
        assert_eq!(padded.len(), INPUT_LEN);

        // input len > padded len
        let padded = left_pad(&input, INPUT_LEN - 1);
        assert!(padded.is_err());
    }
}
