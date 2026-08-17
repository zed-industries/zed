//! `crypt(3)` Base64 tests

#[macro_use]
mod common;

use crate::common::*;
use base64ct::Base64ShaCrypt;

const TEST_VECTORS: &[TestVector] = &[
    TestVector { raw: b"", b64: "" },
    TestVector {
        raw: b"\x55",
        b64: "J/",
    },
    TestVector {
        raw: b"\x55\xaa",
        b64: "Jd8",
    },
    TestVector {
        raw: b"\x55\xaa\x55",
        b64: "JdOJ",
    },
    TestVector {
        raw: b"\x55\xaa\x55\xaa",
        b64: "JdOJe0",
    },
    TestVector {
        raw: b"\x55\xaa\x55\xaa\x55",
        b64: "JdOJeK3",
    },
    TestVector {
        raw: b"\x55\xaa\x55\xaa\x55\xaa",
        b64: "JdOJeKZe",
    },
    TestVector {
        raw: b"\x55\xaa\x55\xaf",
        b64: "JdOJj0",
    },
    TestVector {
        raw: b"\x55\xaa\x55\xaa\x5f",
        b64: "JdOJey3",
    },
    TestVector {
        raw: b"\0",
        b64: "..",
    },
    TestVector {
        raw: b"***",
        b64: "ecW8",
    },
    TestVector {
        raw: b"\x01\x02\x03\x04",
        b64: "/6k.2.",
    },
    TestVector {
        raw: b"\xAD\xAD\xAD\xAD\xAD",
        b64: "hqOfhq8",
    },
    TestVector {
        raw: b"\xFF\xEF\xFE\xFF\xEF\xFE",
        b64: "zzizzziz",
    },
    TestVector {
        raw: b"\xFF\xFF\xFF\xFF\xFF",
        b64: "zzzzzzD",
    },
    TestVector {
        raw: b"\x40\xC1\x3F\xBD\x05\x4C\x72\x2A\xA3\xC2\xF2\x11\x73\xC0\x69\xEA\
               \x49\x7D\x35\x29\x6B\xCC\x24\x65\xF6\xF9\xD0\x41\x08\x7B\xD7\xA9",
        b64: ".3wDxK.Hmdmc09T2n/QOebITpYmOAHGNqbDo/VkSLb8",
    },
    TestVector {
        raw: b"@ \x0cDa\x1cH\xa2,L\xe3<P$MTe]X\xa6m\\\xe7}`(\x8edi\x9eh\xaa\xael\xeb\xbep,\xcf\xfe\x0f\x00",
        b64: "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnyz..",
    },
];

impl_tests!(Base64ShaCrypt);

#[test]
fn reject_trailing_whitespace() {
    let input = "OKC9tOTKagohutGPa6/n4ij7LQjpxAPj7tlOOOf5z4i\n";
    let mut buf = [0u8; 1024];
    assert_eq!(
        Base64ShaCrypt::decode(input, &mut buf),
        Err(Error::InvalidEncoding)
    );
}

#[test]
fn unpadded_reject_trailing_equals() {
    let input = "OKC9tOTKagohutGPa6/n4ij7LQjpxAPj7tlOOOf5z4i=";
    let mut buf = [0u8; 1024];
    assert_eq!(
        Base64ShaCrypt::decode(input, &mut buf),
        Err(Error::InvalidEncoding)
    );
}
