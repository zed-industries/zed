//! Recursive Length Prefix (RLP) encoding support.

use crate::{Encoding, Uint};
use rlp::{DecoderError, Rlp, RlpStream};

impl<const LIMBS: usize> rlp::Encodable for Uint<LIMBS>
where
    Self: Encoding,
{
    fn rlp_append(&self, stream: &mut RlpStream) {
        let bytes = self.to_be_bytes();
        let mut bytes_stripped = bytes.as_ref();

        while bytes_stripped.first().cloned() == Some(0) {
            bytes_stripped = &bytes_stripped[1..];
        }

        stream.encoder().encode_value(bytes_stripped);
    }
}

impl<const LIMBS: usize> rlp::Decodable for Uint<LIMBS>
where
    Self: Encoding,
    <Self as Encoding>::Repr: Default,
{
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        rlp.decoder().decode_value(|bytes| {
            if bytes.first().cloned() == Some(0) {
                Err(rlp::DecoderError::RlpInvalidIndirection)
            } else {
                let mut repr = <Self as Encoding>::Repr::default();
                let offset = repr
                    .as_ref()
                    .len()
                    .checked_sub(bytes.len())
                    .ok_or(DecoderError::RlpIsTooBig)?;

                repr.as_mut()[offset..].copy_from_slice(bytes);
                Ok(Self::from_be_bytes(repr))
            }
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::U256;
    use hex_literal::hex;

    /// U256 test vectors from the `rlp` crate.
    ///
    /// <https://github.com/paritytech/parity-common/blob/faad8b6/rlp/tests/tests.rs#L216-L222>
    const U256_VECTORS: &[(U256, &[u8])] = &[
        (U256::ZERO, &hex!("80")),
        (
            U256::from_be_hex("0000000000000000000000000000000000000000000000000000000001000000"),
            &hex!("8401000000"),
        ),
        (
            U256::from_be_hex("00000000000000000000000000000000000000000000000000000000ffffffff"),
            &hex!("84ffffffff"),
        ),
        (
            U256::from_be_hex("8090a0b0c0d0e0f00910203040506077000000000000000100000000000012f0"),
            &hex!("a08090a0b0c0d0e0f00910203040506077000000000000000100000000000012f0"),
        ),
    ];

    #[test]
    fn round_trip() {
        for &(uint, expected_bytes) in U256_VECTORS {
            assert_eq!(rlp::encode(&uint), expected_bytes);
            assert_eq!(rlp::decode::<U256>(expected_bytes).unwrap(), uint);
        }
    }
}
