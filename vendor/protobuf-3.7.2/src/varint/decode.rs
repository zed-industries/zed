//! Varint decode utilities.

use crate::error::WireError;
use crate::varint::MAX_VARINT32_ENCODED_LEN;
use crate::varint::MAX_VARINT_ENCODED_LEN;

trait DecodeVarint {
    const MAX_ENCODED_LEN: usize;
    const LAST_BYTE_MAX_VALUE: u8;

    fn from_u64(value: u64) -> Self;
}

impl DecodeVarint for u64 {
    const MAX_ENCODED_LEN: usize = MAX_VARINT_ENCODED_LEN;
    const LAST_BYTE_MAX_VALUE: u8 = 0x01;

    fn from_u64(value: u64) -> Self {
        value
    }
}

impl DecodeVarint for u32 {
    const MAX_ENCODED_LEN: usize = MAX_VARINT32_ENCODED_LEN;
    const LAST_BYTE_MAX_VALUE: u8 = 0x0f;

    fn from_u64(value: u64) -> Self {
        value as u32
    }
}

/// Decode a varint, and return decoded value and decoded byte count.
#[inline]
fn decode_varint_full<D: DecodeVarint>(rem: &[u8]) -> crate::Result<Option<(D, usize)>> {
    let mut r: u64 = 0;
    for (i, &b) in rem.iter().enumerate() {
        if i == D::MAX_ENCODED_LEN - 1 {
            if b > D::LAST_BYTE_MAX_VALUE {
                return Err(WireError::IncorrectVarint.into());
            }
            let r = r | ((b as u64) << (i as u64 * 7));
            return Ok(Some((D::from_u64(r), i + 1)));
        }

        r = r | (((b & 0x7f) as u64) << (i as u64 * 7));
        if b < 0x80 {
            return Ok(Some((D::from_u64(r), i + 1)));
        }
    }
    Ok(None)
}

#[inline]
fn decode_varint_impl<D: DecodeVarint>(buf: &[u8]) -> crate::Result<Option<(D, usize)>> {
    if buf.len() >= 1 && buf[0] < 0x80 {
        // The the most common case.
        let ret = buf[0] as u64;
        let consume = 1;
        Ok(Some((D::from_u64(ret), consume)))
    } else if buf.len() >= 2 && buf[1] < 0x80 {
        // Handle the case of two bytes too.
        let ret = (buf[0] & 0x7f) as u64 | (buf[1] as u64) << 7;
        let consume = 2;
        Ok(Some((D::from_u64(ret), consume)))
    } else {
        // Read from array when buf at at least 10 bytes,
        // max len for varint.
        decode_varint_full(buf)
    }
}

/// Try decode a varint. Return `None` if the buffer does not contain complete varint.
#[inline]
pub(crate) fn decode_varint64(buf: &[u8]) -> crate::Result<Option<(u64, usize)>> {
    decode_varint_impl(buf)
}

/// Try decode a varint. Return `None` if the buffer does not contain complete varint.
#[inline]
pub(crate) fn decode_varint32(buf: &[u8]) -> crate::Result<Option<(u32, usize)>> {
    decode_varint_impl(buf)
}

#[cfg(test)]
mod tests {
    use crate::hex::decode_hex;
    use crate::varint::decode::decode_varint32;
    use crate::varint::decode::decode_varint64;

    #[test]
    fn test_decode_varint64() {
        assert_eq!((0, 1), decode_varint64(&decode_hex("00")).unwrap().unwrap());
        assert_eq!(
            (u64::MAX, 10),
            decode_varint64(&decode_hex("ff ff ff ff ff ff ff ff ff 01"))
                .unwrap()
                .unwrap()
        );
        assert!(decode_varint64(&decode_hex("ff ff ff ff ff ff ff ff ff 02")).is_err());
    }

    #[test]
    fn test_decode_varint32() {
        assert_eq!((0, 1), decode_varint32(&decode_hex("00")).unwrap().unwrap());
        assert_eq!(
            (u32::MAX, 5),
            decode_varint32(&decode_hex("ff ff ff ff 0f"))
                .unwrap()
                .unwrap()
        );
        assert!(decode_varint32(&decode_hex("ff ff ff ff 10")).is_err());
    }
}
