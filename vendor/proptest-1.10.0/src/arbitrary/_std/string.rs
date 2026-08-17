//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::string`.

use crate::std_facade::{Box, String, Vec};
use std::iter;
use std::rc::Rc;
use std::slice;
use std::sync::Arc;

multiplex_alloc! {
    alloc::string::FromUtf8Error, ::std::string::FromUtf8Error,
    alloc::string::FromUtf16Error, ::std::string::FromUtf16Error
}

use crate::arbitrary::*;
use crate::collection;
use crate::strategy::statics::static_map;
use crate::strategy::*;
use crate::string::StringParam;

impl Arbitrary for String {
    type Parameters = StringParam;
    type Strategy = &'static str;

    /// ## Panics
    ///
    /// This implementation panics if the input is not a valid regex proptest
    /// can handle.
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        args.into()
    }
}

macro_rules! dst_wrapped {
    ($($w: ident),*) => {
        $(arbitrary!($w<str>, MapInto<StrategyFor<String>, Self>, StringParam;
            a => any_with::<String>(a).prop_map_into()
        );)*
    };
}

dst_wrapped!(Box, Rc, Arc);

lazy_just!(FromUtf16Error, || String::from_utf16(&[0xD800])
    .unwrap_err());

// This is a void-like type, it needs to be handled by the user of
// the type by simply never constructing the variant in an enum or for
// structs by inductively not generating the struct.
// The same applies to ! and Infallible.
// generator!(ParseError, || panic!());

arbitrary!(FromUtf8Error, SFnPtrMap<BoxedStrategy<Vec<u8>>, Self>;
    static_map(not_utf8_bytes(true).boxed(),
        |bs| String::from_utf8(bs).unwrap_err())
);

/// This strategy produces sequences of bytes that are guaranteed to be illegal
/// wrt. UTF-8 with the goal of producing a suffix of bytes in the end of
/// an otherwise legal UTF-8 string that causes the string to be illegal.
/// This is used primarily to generate the `Utf8Error` type and similar.
pub(crate) fn not_utf8_bytes(
    allow_null: bool,
) -> impl Strategy<Value = Vec<u8>> {
    let prefix = collection::vec(any::<char>(), ..::std::u16::MAX as usize);
    let suffix = gen_el_bytes(allow_null);
    (prefix, suffix).prop_map(move |(prefix_bytes, el_bytes)| {
        let iter = prefix_bytes.iter();
        let string: String = if allow_null {
            iter.collect()
        } else {
            iter.filter(|&&x| x != '\u{0}').collect()
        };
        let mut bytes = string.into_bytes();
        bytes.extend(el_bytes.into_iter());
        bytes
    })
}

/// Stands for "error_length" bytes and contains a suffix of bytes that
/// will cause the whole string to become invalid UTF-8.
/// See `gen_el_bytes` for more details.
#[derive(Debug)]
enum ELBytes {
    B1([u8; 1]),
    B2([u8; 2]),
    B3([u8; 3]),
    B4([u8; 4]),
}

impl<'a> IntoIterator for &'a ELBytes {
    type Item = u8;
    type IntoIter = iter::Cloned<slice::Iter<'a, u8>>;
    fn into_iter(self) -> Self::IntoIter {
        use self::ELBytes::*;
        (match *self {
            B1(ref a) => a.iter(),
            B2(ref a) => a.iter(),
            B3(ref a) => a.iter(),
            B4(ref a) => a.iter(),
        })
        .cloned()
    }
}

// By analysis of run_utf8_validation defined at:
// https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1429
// we know that .error_len() \in {None, Some(1), Some(2), Some(3)}.
// We represent this with the range [0..4) and generate a valid
// sequence from that.
fn gen_el_bytes(allow_null: bool) -> impl Strategy<Value = ELBytes> {
    fn b1(a: u8) -> ELBytes {
        ELBytes::B1([a])
    }
    fn b2(a: (u8, u8)) -> ELBytes {
        ELBytes::B2([a.0, a.1])
    }
    fn b3(a: ((u8, u8), u8)) -> ELBytes {
        ELBytes::B3([(a.0).0, (a.0).1, a.1])
    }
    fn b4(a: ((u8, u8), u8, u8)) -> ELBytes {
        ELBytes::B4([(a.0).0, (a.0).1, a.1, a.2])
    }

    /*
    // https://tools.ietf.org/html/rfc3629
    static UTF8_CHAR_WIDTH: [u8; 256] = [
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
    0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
    2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
    3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
    4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
    ];

    /// Mask of the value bits of a continuation byte.
    const CONT_MASK: u8 = 0b0011_1111;
    /// Value of the tag bits (tag mask is !CONT_MASK) of a continuation byte.
    const TAG_CONT_U8: u8 = 0b1000_0000;
    */

    // Continuation byte:
    let succ_byte = 0x80u8..0xC0u8;

    // Do we allow the nul byte or not?
    let start_byte = if allow_null { 0x00u8 } else { 0x01u8 };

    // Invalid continuation byte:
    let fail_byte = prop_oneof![start_byte..0x7Fu8, 0xC1u8..];

    // Matches zero in the UTF8_CHAR_WIDTH table above.
    let byte0_w0 = prop_oneof![0x80u8..0xC0u8, 0xF5u8..];

    // Start of a 3 (width) byte sequence:
    // Leads here: https://doc.rust-lang.org/1.23.0/src/core/str/mod.rs.html#1479
    let byte0_w2 = 0xC2u8..0xE0u8;

    // Start of a 3 (width) byte sequence:
    // https://doc.rust-lang.org/1.23.0/src/core/str/mod.rs.html#1484
    // See the left column in the match.
    let byte0_w3 = 0xE0u8..0xF0u8;

    // Start of a 4 (width) byte sequence:
    // https://doc.rust-lang.org/1.23.0/src/core/str/mod.rs.html#1495
    // See the left column in the match.
    let byte0_w4 = 0xF0u8..0xF5u8;

    // The 2 first (valid) bytes of a 3 (width) byte sequence:
    // The first byte is byte0_w3. The second is the ones produced on the right.
    let byte01_w3 = byte0_w3.clone().prop_flat_map(|x| {
        (
            Just(x),
            match x {
                0xE0u8 => 0xA0u8..0xC0u8,
                0xE1u8..=0xECu8 => 0x80u8..0xC0u8,
                0xEDu8 => 0x80u8..0xA0u8,
                0xEEu8..=0xEFu8 => 0x80u8..0xA0u8,
                _ => panic!(),
            },
        )
    });

    // In a 3 (width) byte sequence, an invalid second byte is chosen such that
    // it will yield an error length of Some(1). The second byte is on
    // the right of the match arms.
    let byte01_w3_e1 = byte0_w3.clone().prop_flat_map(move |x| {
        (
            Just(x),
            match x {
                0xE0u8 => prop_oneof![start_byte..0xA0u8, 0xC0u8..],
                0xE1u8..=0xECu8 => prop_oneof![start_byte..0x80u8, 0xC0u8..],
                0xEDu8 => prop_oneof![start_byte..0x80u8, 0xA0u8..],
                0xEEu8..=0xEFu8 => prop_oneof![start_byte..0x80u8, 0xA0u8..],
                _ => panic!(),
            },
        )
    });

    // In a 4 (width) byte sequence, an invalid second byte is chosen such that
    // it will yield an error length of Some(1). The second byte is on
    // the right of the match arms.
    let byte01_w4_e1 = byte0_w4.clone().prop_flat_map(move |x| {
        (
            Just(x),
            match x {
                0xF0u8 => prop_oneof![start_byte..0x90u8, 0xA0u8..],
                0xF1u8..=0xF3u8 => prop_oneof![start_byte..0x80u8, 0xA0u8..],
                0xF4u8 => prop_oneof![start_byte..0x80u8, 0x90u8..],
                _ => panic!(),
            },
        )
    });

    // The 2 first (valid) bytes of a 4 (width) byte sequence:
    // The first byte is byte0_w4. The second is the ones produced on the right.
    let byte01_w4 = byte0_w4.clone().prop_flat_map(|x| {
        (
            Just(x),
            match x {
                0xF0u8 => 0x90u8..0xA0u8,
                0xF1u8..=0xF3u8 => 0x80u8..0xA0u8,
                0xF4u8 => 0x80u8..0x90u8,
                _ => panic!(),
            },
        )
    });

    prop_oneof![
        // error_len = None
        // These are all happen when next!() fails to provide a byte.
        prop_oneof![
            // width = 2
            // lacking 1 bytes:
            static_map(byte0_w2.clone(), b1),
            // width = 3
            // lacking 2 bytes:
            static_map(byte0_w3, b1),
            // lacking 1 bytes:
            static_map(byte01_w3.clone(), b2),
            // width = 4
            // lacking 3 bytes:
            static_map(byte0_w4, b1),
            // lacking 2 bytes:
            static_map(byte01_w4.clone(), b2),
            // lacking 1 byte:
            static_map((byte01_w4.clone(), succ_byte.clone()), b3),
        ],
        // error_len = Some(1)
        prop_oneof![
            // width = 1 is not represented.
            // width = 0
            // path taken:
            // https://doc.rust-lang.org/1.23.0/src/core/str/mod.rs.html#1508
            static_map(byte0_w0, b1),
            // width = 2
            // path taken:
            // https://doc.rust-lang.org/1.23.0/src/core/str/mod.rs.html#1480
            static_map((byte0_w2, fail_byte.clone()), b2),
            // width = 3
            // path taken:
            // https://doc.rust-lang.org/1.23.0/src/core/str/mod.rs.html#1488
            static_map(byte01_w3_e1, b2),
            // width = 4
            // path taken:
            // https://doc.rust-lang.org/1.23.0/src/core/str/mod.rs.html#1499
            static_map(byte01_w4_e1, b2),
        ],
        // error_len = Some(2)
        static_map(
            prop_oneof![
                // width = 3
                // path taken:
                // https://doc.rust-lang.org/1.23.0/src/core/str/mod.rs.html#1491
                (byte01_w3, fail_byte.clone()),
                // width = 4
                // path taken:
                // https://doc.rust-lang.org/1.23.0/src/core/str/mod.rs.html#1502
                (byte01_w4.clone(), fail_byte.clone())
            ],
            b3
        ),
        // error_len = Some(3), width = 4
        // path taken:
        // https://doc.rust-lang.org/1.23.0/src/core/str/mod.rs.html#1505
        static_map((byte01_w4, succ_byte, fail_byte), b4),
    ]
    .boxed()
}

#[cfg(test)]
mod test {
    no_panic_test!(
        string  => String,
        str_box => Box<str>,
        str_rc  => Rc<str>,
        str_arc => Arc<str>,
        from_utf16_error => FromUtf16Error,
        from_utf8_error => FromUtf8Error
    );
}
