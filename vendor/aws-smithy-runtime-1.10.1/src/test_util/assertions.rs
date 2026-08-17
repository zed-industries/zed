/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/// Asserts that a given string value `$str` contains a substring `$expected`.
///
/// This macro can also take a custom panic message with formatting.
#[macro_export]
macro_rules! assert_str_contains {
    ($str:expr, $expected:expr) => {
        assert_str_contains!($str, $expected, "")
    };
    ($str:expr, $expected:expr, $($fmt_args:tt)+) => {{
        let s = $str;
        let expected = $expected;
        if !s.contains(&expected) {
            panic!(
                "assertion failed: `str.contains(expected)`\n{:>8}: {expected}\n{:>8}: {s}\n{}",
                "expected",
                "str",
                ::std::fmt::format(::std::format_args!($($fmt_args)+)),
            );
        }
    }};
}

#[cfg(test)]
mod tests {
    use std::panic::{catch_unwind, UnwindSafe};

    fn expect_panic(f: impl FnOnce() + UnwindSafe) -> String {
        *catch_unwind(f)
            .expect_err("it should fail")
            .downcast::<String>()
            .expect("it should be a string")
    }

    #[test]
    fn assert_str_contains() {
        assert_str_contains!("foobar", "bar");
        assert_str_contains!("foobar", "o");

        assert_eq!(
            "assertion failed: `str.contains(expected)`\nexpected: not-in-it\n     str: foobar\n",
            expect_panic(|| assert_str_contains!("foobar", "not-in-it"))
        );
        assert_eq!(
            "assertion failed: `str.contains(expected)`\nexpected: not-in-it\n     str: foobar\nsome custom message",
            expect_panic(|| assert_str_contains!("foobar", "not-in-it", "some custom message"))
        );
        assert_eq!(
            "assertion failed: `str.contains(expected)`\nexpected: not-in-it\n     str: foobar\nsome custom message with formatting",
            expect_panic(|| assert_str_contains!("foobar", "not-in-it", "some custom message with {}", "formatting"))
        );
    }
}
