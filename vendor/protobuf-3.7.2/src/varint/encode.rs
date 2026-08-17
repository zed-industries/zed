use std::mem::MaybeUninit;

use crate::varint::MAX_VARINT_ENCODED_LEN;

/// Encode u64 as varint.
/// Panics if buffer length is less than 10.
#[inline]
pub(crate) fn encode_varint64(mut value: u64, buf: &mut [MaybeUninit<u8>]) -> usize {
    assert!(buf.len() >= MAX_VARINT_ENCODED_LEN);

    fn iter(value: &mut u64, byte: &mut MaybeUninit<u8>) -> bool {
        if (*value & !0x7F) > 0 {
            byte.write(((*value & 0x7F) | 0x80) as u8);
            *value >>= 7;
            true
        } else {
            byte.write(*value as u8);
            false
        }
    }

    // Explicitly unroll loop to avoid either
    // unsafe code or bound checking when writing to `buf`

    if !iter(&mut value, &mut buf[0]) {
        return 1;
    };
    if !iter(&mut value, &mut buf[1]) {
        return 2;
    };
    if !iter(&mut value, &mut buf[2]) {
        return 3;
    };
    if !iter(&mut value, &mut buf[3]) {
        return 4;
    };
    if !iter(&mut value, &mut buf[4]) {
        return 5;
    };
    if !iter(&mut value, &mut buf[5]) {
        return 6;
    };
    if !iter(&mut value, &mut buf[6]) {
        return 7;
    };
    if !iter(&mut value, &mut buf[7]) {
        return 8;
    };
    if !iter(&mut value, &mut buf[8]) {
        return 9;
    };
    buf[9].write(value as u8);
    10
}

/// Encode u32 value as varint.
/// Panics if buffer length is less than 5.
#[inline]
pub(crate) fn encode_varint32(mut value: u32, buf: &mut [MaybeUninit<u8>]) -> usize {
    assert!(buf.len() >= 5);

    fn iter(value: &mut u32, byte: &mut MaybeUninit<u8>) -> bool {
        if (*value & !0x7F) > 0 {
            byte.write(((*value & 0x7F) | 0x80) as u8);
            *value >>= 7;
            true
        } else {
            byte.write(*value as u8);
            false
        }
    }

    // Explicitly unroll loop to avoid either
    // unsafe code or bound checking when writing to `buf`

    if !iter(&mut value, &mut buf[0]) {
        return 1;
    };
    if !iter(&mut value, &mut buf[1]) {
        return 2;
    };
    if !iter(&mut value, &mut buf[2]) {
        return 3;
    };
    if !iter(&mut value, &mut buf[3]) {
        return 4;
    };
    buf[4].write(value as u8);
    5
}

/// Encoded size of u64 value.
#[inline]
pub(crate) fn encoded_varint64_len(value: u64) -> usize {
    // Bitwise-or'ing by 1 allows the `value = zero` case to work without
    // affecting other cases.
    let significant_bits = 64 - (value | 1).leading_zeros();
    (significant_bits + 6) as usize / 7
}

#[cfg(test)]
mod test {
    use std::mem::MaybeUninit;

    use crate::varint::encode::encode_varint64;
    use crate::varint::encode::encoded_varint64_len;

    #[test]
    fn test_encoded_varint64_len() {
        fn test(n: u64) {
            let mut buf = [MaybeUninit::uninit(); 10];
            let expected = encode_varint64(n, &mut buf);
            assert_eq!(expected, encoded_varint64_len(n), "n={}", n);
        }

        for n in 0..1000 {
            test(n);
        }

        for p in 0.. {
            match 2u64.checked_pow(p) {
                Some(n) => test(n),
                None => break,
            }
        }

        for p in 0.. {
            match 3u64.checked_pow(p) {
                Some(n) => test(n),
                None => break,
            }
        }

        test(u64::MAX);
        test(u64::MAX - 1);
        test((i64::MAX as u64) + 1);
        test(i64::MAX as u64);
        test((i64::MAX as u64) - 1);
        test((u32::MAX as u64) + 1);
        test(u32::MAX as u64);
        test((u32::MAX as u64) - 1);
        test((i32::MAX as u64) + 1);
        test(i32::MAX as u64);
        test((i32::MAX as u64) - 1);
    }
}
