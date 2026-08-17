//! Integration tests.

/// Hexadecimal test vectors
struct HexVector {
    /// Raw bytes
    raw: &'static [u8],
    /// Lower hex encoded
    lower_hex: &'static [u8],
    /// Upper hex encoded
    upper_hex: &'static [u8],
}

const HEX_TEST_VECTORS: &[HexVector] = &[
    HexVector {
        raw: b"",
        lower_hex: b"",
        upper_hex: b"",
    },
    HexVector {
        raw: b"\0",
        lower_hex: b"00",
        upper_hex: b"00",
    },
    HexVector {
        raw: b"***",
        lower_hex: b"2a2a2a",
        upper_hex: b"2A2A2A",
    },
    HexVector {
        raw: b"\x01\x02\x03\x04",
        lower_hex: b"01020304",
        upper_hex: b"01020304",
    },
    HexVector {
        raw: b"\xAD\xAD\xAD\xAD\xAD",
        lower_hex: b"adadadadad",
        upper_hex: b"ADADADADAD",
    },
    HexVector {
        raw: b"\xFF\xFF\xFF\xFF\xFF",
        lower_hex: b"ffffffffff",
        upper_hex: b"FFFFFFFFFF",
    },
];

#[test]
fn lower_encode() {
    for vector in HEX_TEST_VECTORS {
        // 10 is the size of the largest encoded test vector
        let mut buf = [0u8; 10];
        let out = base16ct::lower::encode(vector.raw, &mut buf).unwrap();
        assert_eq!(vector.lower_hex, out);
    }
}

#[test]
fn lower_decode() {
    for vector in HEX_TEST_VECTORS {
        // 5 is the size of the largest decoded test vector
        let mut buf = [0u8; 5];
        let out = base16ct::lower::decode(vector.lower_hex, &mut buf).unwrap();
        assert_eq!(vector.raw, out);
    }
}

#[test]
fn lower_reject_odd_size_input() {
    let mut out = [0u8; 3];
    assert_eq!(
        Err(base16ct::Error::InvalidLength),
        base16ct::lower::decode(b"12345", &mut out),
    )
}

#[test]
fn upper_encode() {
    for vector in HEX_TEST_VECTORS {
        // 10 is the size of the largest encoded test vector
        let mut buf = [0u8; 10];
        let out = base16ct::upper::encode(vector.raw, &mut buf).unwrap();
        assert_eq!(vector.upper_hex, out);
    }
}

#[test]
fn upper_decode() {
    for vector in HEX_TEST_VECTORS {
        // 5 is the size of the largest decoded test vector
        let mut buf = [0u8; 5];
        let out = base16ct::upper::decode(vector.upper_hex, &mut buf).unwrap();
        assert_eq!(vector.raw, out);
    }
}

#[test]
fn upper_reject_odd_size_input() {
    let mut out = [0u8; 3];
    assert_eq!(
        Err(base16ct::Error::InvalidLength),
        base16ct::upper::decode(b"12345", &mut out),
    )
}

#[test]
fn mixed_decode() {
    for vector in HEX_TEST_VECTORS {
        // 5 is the size of the largest decoded test vector
        let mut buf = [0u8; 5];
        let out = base16ct::mixed::decode(vector.upper_hex, &mut buf).unwrap();
        assert_eq!(vector.raw, out);
        let out = base16ct::mixed::decode(vector.lower_hex, &mut buf).unwrap();
        assert_eq!(vector.raw, out);
    }
}

#[test]
fn mixed_reject_odd_size_input() {
    let mut out = [0u8; 3];
    assert_eq!(
        Err(base16ct::Error::InvalidLength),
        base16ct::mixed::decode(b"12345", &mut out),
    )
}

#[test]
#[cfg(feature = "alloc")]
fn encode_and_decode_various_lengths() {
    let data = [b'X'; 64];

    for i in 0..data.len() {
        let encoded = base16ct::lower::encode_string(&data[..i]);
        let decoded = base16ct::lower::decode_vec(encoded).unwrap();
        assert_eq!(decoded.as_slice(), &data[..i]);

        let encoded = base16ct::upper::encode_string(&data[..i]);
        let decoded = base16ct::upper::decode_vec(encoded).unwrap();
        assert_eq!(decoded.as_slice(), &data[..i]);

        let encoded = base16ct::lower::encode_string(&data[..i]);
        let decoded = base16ct::mixed::decode_vec(encoded).unwrap();
        assert_eq!(decoded.as_slice(), &data[..i]);

        let encoded = base16ct::upper::encode_string(&data[..i]);
        let decoded = base16ct::mixed::decode_vec(encoded).unwrap();
        assert_eq!(decoded.as_slice(), &data[..i]);
    }
}

#[test]
fn hex_display_upper() {
    for vector in HEX_TEST_VECTORS {
        let hex = format!("{:X}", base16ct::HexDisplay(vector.raw));
        assert_eq!(hex.as_bytes(), vector.upper_hex);
    }
}

#[test]
fn hex_display_lower() {
    for vector in HEX_TEST_VECTORS {
        let hex = format!("{:x}", base16ct::HexDisplay(vector.raw));
        assert_eq!(hex.as_bytes(), vector.lower_hex);
    }
}
