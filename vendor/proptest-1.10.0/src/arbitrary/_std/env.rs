//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::env`.

use std::env::*;
use std::ffi::OsString;
use std::iter::once;

use crate::arbitrary::*;
use crate::strategy::statics::static_map;
use crate::strategy::*;

// FIXME: SplitPaths when lifetimes in strategies are possible.

lazy_just!(
    Args, args;
    ArgsOs, args_os;
    Vars, vars;
    VarsOs, vars_os;
    JoinPathsError, jpe
);

#[cfg(not(target_os = "windows"))]
fn jpe() -> JoinPathsError {
    join_paths(once(":")).unwrap_err()
}

#[cfg(target_os = "windows")]
fn jpe() -> JoinPathsError {
    join_paths(once("\"")).unwrap_err()
}

// Algorithm from: https://stackoverflow.com/questions/47749164
#[cfg(any(target_os = "windows", test))]
fn make_utf16_invalid(buf: &mut [u16], p: usize) {
    // Verify that length is non-empty.
    // An empty string is always valid UTF-16.
    assert!(buf.len() > 0);

    // If first elem or previous entry is not a leading surrogate.
    let gen_trail = 0 == p || 0xd800 != (buf[p - 1] & 0xfc00);
    // If last element or succeeding entry is not a traililng surrogate.
    let gen_lead = p == buf.len() - 1 || 0xdc00 != (buf[p + 1] & 0xfc00);
    let (force_bits_mask, force_bits_value) = if gen_trail {
        if gen_lead {
            // Trailing or leading surrogate.
            (0xf800, 0xd800)
        } else {
            // Trailing surrogate.
            (0xfc00, 0xdc00)
        }
    } else {
        // Leading surrogate.
        // Note that `gen_lead` and `gen_trail` could both be false here if `p`
        // lies exactly between a leading and a trailing surrogate. In this
        // case, it doesn't matter what we do because the UTF-16 will be
        // invalid regardless, so just always force a leading surrogate.
        (0xfc00, 0xd800)
    };
    debug_assert_eq!(0, (force_bits_value & !force_bits_mask));
    buf[p] = (buf[p] & !force_bits_mask) | force_bits_value;
}

#[cfg(not(target_arch = "wasm32"))]
mod var_error {
    use super::*;

    /// Generates the set of `WTF-16 \ UTF-16` and makes
    /// an `OsString` that is not a valid String from it.
    #[cfg(target_os = "windows")]
    fn osstring_invalid_string() -> impl Strategy<Value = OsString> {
        use std::os::windows::ffi::OsStringExt;
        let size = 1..::std::u16::MAX as usize;
        let vec_gen = crate::collection::vec(..::std::u16::MAX, size.clone());
        (size, vec_gen).prop_map(|(p, mut sbuf)| {
            // Not quite a uniform distribution due to clamping,
            // but probably good enough
            let p = ::std::cmp::min(p, sbuf.len() - 1);
            make_utf16_invalid(&mut sbuf, p);
            OsString::from_wide(sbuf.as_slice())
                .into_string()
                .unwrap_err()
        })
    }

    #[cfg(not(target_os = "windows"))]
    fn osstring_invalid_string() -> impl Strategy<Value = OsString> {
        use crate::arbitrary::_std::string::not_utf8_bytes;
        use std::os::unix::ffi::OsStringExt;
        static_map(not_utf8_bytes(true), OsString::from_vec)
    }

    arbitrary!(VarError,
        TupleUnion<(
            WA<Just<Self>>,
            WA<SFnPtrMap<BoxedStrategy<OsString>, Self>>
        )>;
        prop_oneof![
            Just(VarError::NotPresent),
            static_map(osstring_invalid_string().boxed(), VarError::NotUnicode)
        ]
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::num;
    use crate::test_runner::Config;

    no_panic_test!(
        args => Args,
        args_os => ArgsOs,
        vars => Vars,
        vars_os => VarsOs,
        join_paths_error => JoinPathsError,
        var_error => VarError
    );

    proptest! {
        #![proptest_config(Config {
            cases: 65536,
            .. Config::default()
        })]

        #[test]
        fn make_utf16_invalid_doesnt_panic(
            mut buf in [num::u16::ANY; 3],
            p in 0usize..3
        ) {
            make_utf16_invalid(&mut buf, p);
        }
    }
}
