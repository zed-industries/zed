//! bcrypt Base64 tests

#[macro_use]
mod common;

use crate::common::*;
use base64ct::Base64Bcrypt;

const TEST_VECTORS: &[TestVector] = &[
    TestVector { raw: b"", b64: "" },
    TestVector {
        raw: b"\0",
        b64: "..",
    },
    TestVector {
        raw: b"***",
        b64: "Igmo",
    },
    TestVector {
        raw: b"\x01\x02\x03\x04",
        b64: ".OGB/.",
    },
    TestVector {
        raw: b"\xAD\xAD\xAD\xAD\xAD",
        b64: "pY0rpYy",
    },
    TestVector {
        raw: b"\xFF\xEF\xFE\xFF\xEF\xFE",
        b64: "98989898",
    },
    TestVector {
        raw: b"\xFF\xFF\xFF\xFF\xFF",
        b64: "9999996",
    },
    TestVector {
        raw: b"\x40\xC1\x3F\xBD\x05\x4C\x72\x2A\xA3\xC2\xF2\x11\x73\xC0\x69\xEA\
               \x49\x7D\x35\x29\x6B\xCC\x24\x65\xF6\xF9\xD0\x41\x08\x7B\xD7\xA9",
        b64: "OKC9tOTKagohutGPa6/n4ij7LQjpxAPj7tlOOOf5z4i",
    },
    TestVector {
        raw: b"\x00\x10\x83\x10Q\x87 \x92\x8B0\xD3\x8FA\x14\x93QU\x97a\x96\x9Bq\
               \xD7\x9F\x82\x18\xA3\x92Y\xA7\xA2\x9A\xAB\xB2\xDB\xAF\xC3\x1C\xB3\
               \xFB\xF0\x00",
        b64: "./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwx89..",
    },
];

impl_tests!(Base64Bcrypt);

#[test]
fn reject_trailing_whitespace() {
    let input = "OKC9tOTKagohutGPa6/n4ij7LQjpxAPj7tlOOOf5z4i\n";
    let mut buf = [0u8; 1024];
    assert_eq!(
        Base64Bcrypt::decode(input, &mut buf),
        Err(Error::InvalidEncoding)
    );
}

#[test]
fn unpadded_reject_trailing_equals() {
    let input = "OKC9tOTKagohutGPa6/n4ij7LQjpxAPj7tlOOOf5z4i=";
    let mut buf = [0u8; 1024];
    assert_eq!(
        Base64Bcrypt::decode(input, &mut buf),
        Err(Error::InvalidEncoding)
    );
}
