//! URL-safe Base64 tests

#[macro_use]
mod common;

/// URL-safe Base64 with `=` padding
mod padded {
    use crate::common::*;
    use base64ct::Base64Url;

    const TEST_VECTORS: &[TestVector] = &[
        TestVector { raw: b"", b64: "" },
        TestVector {
            raw: b"\0",
            b64: "AA==",
        },
        TestVector {
            raw: b"***",
            b64: "Kioq",
        },
        TestVector {
            raw: b"\x01\x02\x03\x04",
            b64: "AQIDBA==",
        },
        TestVector {
            raw: b"\xAD\xAD\xAD\xAD\xAD",
            b64: "ra2tra0=",
        },
        TestVector {
            raw: b"\xFF\xEF\xFE\xFF\xEF\xFE",
            b64: "_-_-_-_-",
        },
        TestVector {
            raw: b"\xFF\xFF\xFF\xFF\xFF",
            b64: "______8=",
        },
        TestVector {
            raw: b"\x40\xC1\x3F\xBD\x05\x4C\x72\x2A\xA3\xC2\xF2\x11\x73\xC0\x69\xEA\
                   \x49\x7D\x35\x29\x6B\xCC\x24\x65\xF6\xF9\xD0\x41\x08\x7B\xD7\xA9",
            b64: "QME_vQVMciqjwvIRc8Bp6kl9NSlrzCRl9vnQQQh716k=",
        },
        TestVector {
            raw: b"\x00\x10\x83\x10Q\x87 \x92\x8B0\xD3\x8FA\x14\x93QU\x97a\x96\x9Bq\
                   \xD7\x9F\x82\x18\xA3\x92Y\xA7\xA2\x9A\xAB\xB2\xDB\xAF\xC3\x1C\xB3\
                   \xFB\xF0\x00",
            b64: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_AA",
        },
    ];

    impl_tests!(Base64Url);

    #[test]
    fn reject_trailing_whitespace() {
        let input = "QME/vQVMciqjwvIRc8Bp6kl9NSlrzCRl9vnQQQh716k\n";
        let mut buf = [0u8; 1024];
        assert_eq!(
            Base64Url::decode(input, &mut buf),
            Err(Error::InvalidEncoding)
        );
    }
}

/// URL-safe Base64 *without* padding
mod unpadded {
    use crate::common::*;
    use base64ct::Base64UrlUnpadded;

    const TEST_VECTORS: &[TestVector] = &[
        TestVector { raw: b"", b64: "" },
        TestVector {
            raw: b"\0",
            b64: "AA",
        },
        TestVector {
            raw: b"***",
            b64: "Kioq",
        },
        TestVector {
            raw: b"\x01\x02\x03\x04",
            b64: "AQIDBA",
        },
        TestVector {
            raw: b"\xAD\xAD\xAD\xAD\xAD",
            b64: "ra2tra0",
        },
        TestVector {
            raw: b"\xFF\xEF\xFE\xFF\xEF\xFE",
            b64: "_-_-_-_-",
        },
        TestVector {
            raw: b"\xFF\xFF\xFF\xFF\xFF",
            b64: "______8",
        },
        TestVector {
            raw: b"\x40\xC1\x3F\xBD\x05\x4C\x72\x2A\xA3\xC2\xF2\x11\x73\xC0\x69\xEA\
                   \x49\x7D\x35\x29\x6B\xCC\x24\x65\xF6\xF9\xD0\x41\x08\x7B\xD7\xA9",
            b64: "QME_vQVMciqjwvIRc8Bp6kl9NSlrzCRl9vnQQQh716k",
        },
        TestVector {
            raw: b"\x00\x10\x83\x10Q\x87 \x92\x8B0\xD3\x8FA\x14\x93QU\x97a\x96\x9Bq\
                   \xD7\x9F\x82\x18\xA3\x92Y\xA7\xA2\x9A\xAB\xB2\xDB\xAF\xC3\x1C\xB3\
                   \xFB\xF0\x00",
            b64: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_AA",
        },
    ];

    impl_tests!(Base64UrlUnpadded);

    #[test]
    fn reject_trailing_whitespace() {
        let input = "QME/vQVMciqjwvIRc8Bp6kl9NSlrzCRl9vnQQQh716k\n";
        let mut buf = [0u8; 1024];
        assert_eq!(
            Base64UrlUnpadded::decode(input, &mut buf),
            Err(Error::InvalidEncoding)
        );
    }

    #[test]
    fn unpadded_reject_trailing_equals() {
        let input = "QME_vQVMciqjwvIRc8Bp6kl9NSlrzCRl9vnQQQh716k=";
        let mut buf = [0u8; 1024];
        assert_eq!(
            Base64UrlUnpadded::decode(input, &mut buf),
            Err(Error::InvalidEncoding)
        );
    }
}
