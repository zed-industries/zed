// Copyright 2015-2016 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! Testing framework.
//!
//! Unlike the rest of *aws-lc-rs*, this testing framework uses panics pretty
//! liberally. It was originally designed for internal use--it drives most of
//! *aws-lc-rs*'s internal tests, and so it is optimized for getting *aws-lc-rs*'s tests
//! written quickly at the expense of some usability. The documentation is
//! lacking. The best way to learn it is to look at some examples. The digest
//! tests are the most complicated because they use named sections. Other tests
//! avoid named sections and so are easier to understand.
//!
//! # Examples
//!
//! ## Writing Tests
//!
//! Input files look like this:
//!
//! ```text
//! # This is a comment.
//!
//! HMAC = SHA1
//! Input = "My test data"
//! Key = ""
//! Output = 61afdecb95429ef494d61fdee15990cabf0826fc
//!
//! HMAC = SHA256
//! Input = "Sample message for keylen<blocklen"
//! Key = 000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F
//! Output = A28CF43130EE696A98F14A37678B56BCFCBDD9E5CF69717FECF5480F0EBDF790
//! ```
//!
//! Test cases are separated with blank lines. Note how the bytes of the `Key`
//! attribute are specified as a quoted string in the first test case and as
//! hex in the second test case; you can use whichever form is more convenient
//! and you can mix and match within the same file. The empty sequence of bytes
//! can only be represented with the quoted string form (`""`).
//!
//! Here's how you would consume the test data:
//!
//! ```ignore
//! use aws_lc_rs::test;
//!
//! test::run(test::test_file!("hmac_tests.txt"), |section, test_case| {
//!     assert_eq!(section, ""); // This test doesn't use named sections.
//!
//!     let digest_alg = test_case.consume_digest_alg("HMAC");
//!     let input = test_case.consume_bytes("Input");
//!     let key = test_case.consume_bytes("Key");
//!     let output = test_case.consume_bytes("Output");
//!
//!     // Do the actual testing here
//! });
//! ```
//!
//! Note that `consume_digest_alg` automatically maps the string "SHA1" to a
//! reference to `digest::SHA1_FOR_LEGACY_USE_ONLY`, "SHA256" to
//! `digest::SHA256`, etc.
//!
//! ## Output When a Test Fails
//!
//! When a test case fails, the framework automatically prints out the test
//! case. If the test case failed with a panic, then the backtrace of the panic
//! will be printed too. For example, let's say the failing test case looks
//! like this:
//!
//! ```text
//! Curve = P-256
//! a = 2b11cb945c8cf152ffa4c9c2b1c965b019b35d0b7626919ef0ae6cb9d232f8af
//! b = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
//! r = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
//! ```
//! If the test fails, this will be printed (if `$RUST_BACKTRACE` is `1`):
//!
//! ```text
//! src/example_tests.txt: Test panicked.
//! Curve = P-256
//! a = 2b11cb945c8cf152ffa4c9c2b1c965b019b35d0b7626919ef0ae6cb9d232f8af
//! b = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
//! r = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
//! thread 'example_test' panicked at 'Test failed.', src\test.rs:206
//! stack backtrace:
//!    0:     0x7ff654a05c7c - std::rt::lang_start::h61f4934e780b4dfc
//!    1:     0x7ff654a04f32 - std::rt::lang_start::h61f4934e780b4dfc
//!    2:     0x7ff6549f505d - std::panicking::rust_panic_with_hook::hfe203e3083c2b544
//!    3:     0x7ff654a0825b - rust_begin_unwind
//!    4:     0x7ff6549f63af - std::panicking::begin_panic_fmt::h484cd47786497f03
//!    5:     0x7ff654a07e9b - rust_begin_unwind
//!    6:     0x7ff654a0ae95 - core::panicking::panic_fmt::h257ceb0aa351d801
//!    7:     0x7ff654a0b190 - core::panicking::panic::h4bb1497076d04ab9
//!    8:     0x7ff65496dc41 - from_file<closure>
//!                         at C:\Users\Example\example\<core macros>:4
//!    9:     0x7ff65496d49c - example_test
//!                         at C:\Users\Example\example\src\example.rs:652
//!   10:     0x7ff6549d192a - test::stats::Summary::new::ha139494ed2e4e01f
//!   11:     0x7ff6549d51a2 - test::stats::Summary::new::ha139494ed2e4e01f
//!   12:     0x7ff654a0a911 - _rust_maybe_catch_panic
//!   13:     0x7ff6549d56dd - test::stats::Summary::new::ha139494ed2e4e01f
//!   14:     0x7ff654a03783 - std::sys::thread::Thread::new::h2b08da6cd2517f79
//!   15:     0x7ff968518101 - BaseThreadInitThunk
//! ```
//!
//! Notice that the output shows the name of the data file
//! (`src/example_tests.txt`), the test inputs that led to the failure, and the
//! stack trace to the line in the test code that panicked: entry 9 in the
//! stack trace pointing to line 652 of the file `example.rs`.

#![doc(hidden)]

extern crate alloc;
extern crate std;

use std::error::Error;

use crate::{digest, error};

pub use crate::hex::{
    decode as from_hex, decode_dirty as from_dirty_hex, encode as to_hex,
    encode_upper as to_hex_upper,
};

/// `compile_time_assert_clone::<T>();` fails to compile if `T` doesn't
/// implement `Clone`.
#[allow(clippy::extra_unused_type_parameters)]
pub fn compile_time_assert_clone<T: Clone>() {}

/// `compile_time_assert_copy::<T>();` fails to compile if `T` doesn't
/// implement `Copy`.
#[allow(clippy::extra_unused_type_parameters)]
pub fn compile_time_assert_copy<T: Copy>() {}

/// `compile_time_assert_eq::<T>();` fails to compile if `T` doesn't
/// implement `Eq`.
#[allow(clippy::extra_unused_type_parameters)]
pub fn compile_time_assert_eq<T: Eq>() {}

/// `compile_time_assert_send::<T>();` fails to compile if `T` doesn't
/// implement `Send`.
#[allow(clippy::extra_unused_type_parameters)]
pub fn compile_time_assert_send<T: Send>() {}

/// `compile_time_assert_sync::<T>();` fails to compile if `T` doesn't
/// implement `Sync`.
#[allow(clippy::extra_unused_type_parameters)]
pub fn compile_time_assert_sync<T: Sync>() {}

/// `compile_time_assert_std_error_error::<T>();` fails to compile if `T`
/// doesn't implement `std::error::Error`.
#[allow(clippy::extra_unused_type_parameters)]
pub fn compile_time_assert_std_error_error<T: Error>() {}

/// A test case. A test case consists of a set of named attributes. Every
/// attribute in the test case must be consumed exactly once; this helps catch
/// typos and omissions.
///
/// Requires the `alloc` default feature to be enabled.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct TestCase {
    attributes: Vec<(String, String, bool)>,
}

impl TestCase {
    /// Maps the strings "SHA1", "SHA256", "SHA384", and "SHA512" to digest
    /// algorithms, maps "SHA224" to `None`, and panics on other (erroneous)
    /// inputs. "SHA224" is mapped to None because *ring* intentionally does
    /// not support SHA224, but we need to consume test vectors from NIST that
    /// have SHA224 vectors in them.
    pub fn consume_digest_alg(&mut self, key: &str) -> Option<&'static digest::Algorithm> {
        let name = self.consume_string(key);
        match name.as_ref() {
            "SHA1" => Some(&digest::SHA1_FOR_LEGACY_USE_ONLY),
            "SHA224" => Some(&digest::SHA224),
            "SHA256" => Some(&digest::SHA256),
            "SHA384" => Some(&digest::SHA384),
            "SHA512" => Some(&digest::SHA512),
            "SHA512_256" => Some(&digest::SHA512_256),
            "SHA3_256" => Some(&digest::SHA3_256),
            "SHA3_384" => Some(&digest::SHA3_384),
            "SHA3_512" => Some(&digest::SHA3_512),
            _ => unreachable!("Unsupported digest algorithm: {}", name),
        }
    }

    /// Returns the value of an attribute that is encoded as a sequence of an
    /// even number of hex digits, or as a double-quoted UTF-8 string. The
    /// empty (zero-length) value is represented as "".
    pub fn consume_bytes(&mut self, key: &str) -> Vec<u8> {
        self.consume_optional_bytes(key)
            .unwrap_or_else(|| panic!("No attribute named \"{key}\""))
    }

    /// Like `consume_bytes()` except it returns `None` if the test case
    /// doesn't have the attribute.
    pub fn consume_optional_bytes(&mut self, key: &str) -> Option<Vec<u8>> {
        let s = self.consume_optional_string(key)?;
        let result = if s.starts_with('\"') {
            // The value is a quoted UTF-8 string.
            let s = s.as_bytes();
            let mut bytes = Vec::with_capacity(s.len());
            let mut s = s.iter().skip(1);
            loop {
                let b = match s.next() {
                    Some(b'\\') => {
                        match s.next() {
                            // We don't allow all octal escape sequences, only "\0" for null.
                            Some(b'0') => 0u8,
                            Some(b't') => b'\t',
                            Some(b'n') => b'\n',
                            _ => {
                                panic!("Invalid hex escape sequence in string.");
                            }
                        }
                    }
                    Some(b'"') => {
                        assert!(
                            s.next().is_none(),
                            "characters after the closing quote of a quoted string."
                        );
                        break;
                    }
                    Some(b) => *b,
                    None => panic!("Missing terminating '\"' in string literal."),
                };
                bytes.push(b);
            }
            bytes
        } else {
            // The value is hex encoded.
            match from_hex(&s) {
                Ok(s) => s,
                Err(err_str) => {
                    panic!("{err_str} in {s}");
                }
            }
        };
        Some(result)
    }

    /// Returns the value of an attribute that is an integer, in decimal
    /// notation.
    pub fn consume_usize(&mut self, key: &str) -> usize {
        let s = self.consume_string(key);
        s.parse::<usize>().unwrap()
    }

    /// Returns the value of an attribute that is an integer, in decimal
    /// notation.
    pub fn consume_bool(&mut self, key: &str) -> bool {
        let value_str = self
            .consume_optional_string(key)
            .unwrap_or_else(|| panic!("No attribute named \"{key}\""))
            .to_ascii_lowercase();
        value_str.starts_with('t') || value_str.starts_with('y')
    }

    /// Returns the raw value of an attribute, without any unquoting or
    /// other interpretation.
    pub fn consume_string(&mut self, key: &str) -> String {
        self.consume_optional_string(key)
            .unwrap_or_else(|| panic!("No attribute named \"{key}\""))
    }

    /// Like `consume_string()` except it returns `None` if the test case
    /// doesn't have the attribute.
    pub fn consume_optional_string(&mut self, key: &str) -> Option<String> {
        for (name, value, consumed) in &mut self.attributes {
            if key == name {
                assert!(!(*consumed), "Attribute {key} was already consumed");
                *consumed = true;
                return Some(value.clone());
            }
        }
        None
    }
}

/// References a test input file.
#[macro_export]
#[allow(clippy::module_name_repetitions)]
macro_rules! test_file {
    ($file_name: expr) => {
        $crate::test::File {
            file_name: $file_name,
            contents: include_str!($file_name),
        }
    };
}

/// A test input file.
#[derive(Clone, Copy)]
pub struct File<'a> {
    /// The name (path) of the file.
    pub file_name: &'a str,

    /// The contents of the file.
    pub contents: &'a str,
}

/// Parses test cases out of the given file, calling `f` on each vector until
/// `f` fails or until all the test vectors have been read. `f` can indicate
/// failure either by returning `Err()` or by panicking.
///
/// # Panics
/// Panics on test failure.
#[allow(clippy::needless_pass_by_value)]
pub fn run<F>(test_file: File, mut f: F)
where
    F: FnMut(&str, &mut TestCase) -> Result<(), error::Unspecified>,
{
    let lines = &mut test_file.contents.lines();

    let mut current_section = String::new();
    let mut failed = false;

    while let Some(mut test_case) = parse_test_case(&mut current_section, lines) {
        let result = match f(&current_section, &mut test_case) {
            Ok(()) => {
                if test_case
                    .attributes
                    .iter()
                    .any(|&(_, _, consumed)| !consumed)
                {
                    failed = true;
                    Err("Test didn't consume all attributes.")
                } else {
                    Ok(())
                }
            }
            Err(error::Unspecified) => Err("Test returned Err(error::Unspecified)."),
        };

        if result.is_err() {
            failed = true;
        }

        #[cfg(feature = "test_logging")]
        {
            if let Err(msg) = result {
                println!("{}: {}", test_file.file_name, msg);

                for (name, value, consumed) in test_case.attributes {
                    let consumed_str = if consumed { "" } else { " (unconsumed)" };
                    println!("{}{} = {}", name, consumed_str, value);
                }
            };
        }
    }

    assert!(!failed, "Test failed.");
}

fn parse_test_case(
    current_section: &mut String,
    lines: &mut dyn Iterator<Item = &str>,
) -> Option<TestCase> {
    let mut attributes = Vec::new();

    let mut is_first_line = true;
    loop {
        let line = lines.next();

        #[cfg(feature = "test_logging")]
        {
            if let Some(text) = &line {
                println!("Line: {}", text);
            }
        }

        match line {
            // If we get to EOF when we're not in the middle of a test case,
            // then we're done.
            None if is_first_line => {
                return None;
            }

            // End of the file on a non-empty test cases ends the test case.
            None => {
                return Some(TestCase { attributes });
            }

            // A blank line ends a test case if the test case isn't empty.
            Some("") => {
                if !is_first_line {
                    return Some(TestCase { attributes });
                }
                // Ignore leading blank lines.
            }

            // Comments start with '#'; ignore them.
            Some(line) if line.starts_with('#') => (),

            Some(line) if line.starts_with('[') => {
                assert!(is_first_line);
                assert!(line.ends_with(']'));
                current_section.clear();
                current_section.push_str(line);
                let _: Option<char> = current_section.pop();
                let _: char = current_section.remove(0);
            }

            Some(line) => {
                is_first_line = false;

                let parts: Vec<&str> = line.splitn(2, " = ").collect();
                assert_eq!(parts.len(), 2, "Syntax error: Expected Key = Value.");

                let key = parts[0].trim();
                let value = parts[1].trim();

                // Don't allow the value to be ommitted. An empty value can be
                // represented as an empty quoted string.
                assert_ne!(value.len(), 0);

                // Checking is_none() ensures we don't accept duplicate keys.
                attributes.push((String::from(key), String::from(value), false));
            }
        }
    }
}

/// Deterministic implementations of `ring::rand::SecureRandom`.
///
/// These are only used for testing KATs where a random number should be generated.
pub mod rand {
    use crate::error;

    /// An implementation of `SecureRandom` that always fills the output slice
    /// with the given byte.
    #[derive(Debug)]
    pub struct FixedByteRandom {
        pub byte: u8,
    }

    impl crate::rand::sealed::SecureRandom for FixedByteRandom {
        fn fill_impl(&self, dest: &mut [u8]) -> Result<(), error::Unspecified> {
            dest.fill(self.byte);
            Ok(())
        }
    }

    /// An implementation of `SecureRandom` that always fills the output slice
    /// with the slice in `bytes`. The length of the slice given to `slice`
    /// must match exactly.
    #[derive(Debug)]
    pub struct FixedSliceRandom<'a> {
        pub bytes: &'a [u8],
    }

    impl crate::rand::sealed::SecureRandom for FixedSliceRandom<'_> {
        #[inline]
        fn fill_impl(&self, dest: &mut [u8]) -> Result<(), error::Unspecified> {
            dest.copy_from_slice(self.bytes);
            Ok(())
        }
    }

    /// An implementation of `SecureRandom` where each slice in `bytes` is a
    /// test vector for one call to `fill()`. *Not thread-safe.*
    ///
    /// The first slice in `bytes` is the output for the first call to
    /// `fill()`, the second slice is the output for the second call to
    /// `fill()`, etc. The output slice passed to `fill()` must have exactly
    /// the length of the corresponding entry in `bytes`. `current` must be
    /// initialized to zero. `fill()` must be called exactly once for each
    /// entry in `bytes`.
    #[derive(Debug)]
    pub struct FixedSliceSequenceRandom<'a> {
        /// The value.
        pub bytes: &'a [&'a [u8]],
        pub current: core::cell::UnsafeCell<usize>,
    }

    impl crate::rand::sealed::SecureRandom for FixedSliceSequenceRandom<'_> {
        fn fill_impl(&self, dest: &mut [u8]) -> Result<(), error::Unspecified> {
            let current = unsafe { *self.current.get() };
            let bytes = self.bytes[current];
            dest.copy_from_slice(bytes);
            // Remember that we returned this slice and prepare to return
            // the next one, if any.
            unsafe { *self.current.get() += 1 };
            Ok(())
        }
    }

    impl Drop for FixedSliceSequenceRandom<'_> {
        fn drop(&mut self) {
            // Ensure that `fill()` was called exactly the right number of
            // times.
            assert_eq!(unsafe { *self.current.get() }, self.bytes.len());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rand::sealed::SecureRandom;
    use crate::test::rand::{FixedByteRandom, FixedSliceRandom, FixedSliceSequenceRandom};
    use crate::test::{from_dirty_hex, to_hex_upper};
    use crate::{error, test};
    use core::cell::UnsafeCell;

    #[test]
    fn fixed_byte_random() {
        let fbr = FixedByteRandom { byte: 42 };
        let mut bs = [0u8; 42];
        fbr.fill_impl(&mut bs).expect("filled");
        assert_eq!([42u8; 42], bs);
    }

    #[test]
    fn fixed_slice_random() {
        let fbr = FixedSliceRandom { bytes: &[42u8; 42] };
        let mut bs = [0u8; 42];
        fbr.fill_impl(&mut bs).expect("fill");
    }

    #[test]
    #[should_panic(
        expected = "source slice length (42) does not match destination slice length (0)"
    )]
    fn fixed_slice_random_length_mismatch() {
        let fbr = FixedSliceRandom { bytes: &[42u8; 42] };
        let _: Result<(), error::Unspecified> = fbr.fill_impl(&mut []);
    }

    #[test]
    fn fixed_slice_sequence_random() {
        let fbr = FixedSliceSequenceRandom {
            bytes: &[&[7u8; 7], &[42u8; 42]],
            current: UnsafeCell::new(0),
        };
        let mut bs_one = [0u8; 7];
        fbr.fill_impl(&mut bs_one).expect("fill");
        assert_eq!([7u8; 7], bs_one);
        let mut bs_two = [42u8; 42];
        fbr.fill_impl(&mut bs_two).expect("filled");
        assert_eq!([42u8; 42], bs_two);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn fixed_slice_sequence_random_no_remaining() {
        let fbr = FixedSliceSequenceRandom {
            bytes: &[],
            current: UnsafeCell::new(0),
        };
        let mut bs_one = [0u8; 7];
        let _: Result<(), error::Unspecified> = fbr.fill_impl(&mut bs_one);
    }

    // TODO: This test is causing a thread panic which prevents capture with should_panic
    // #[test]
    // #[should_panic]
    // fn fixed_slice_sequence_random_length_mismatch() {
    //     let fbr = FixedSliceSequenceRandom {
    //         bytes: &[&[42u8; 42]],
    //         current: UnsafeCell::new(0),
    //     };
    //     let _: Result<(), error::Unspecified> = fbr.fill_impl(&mut []);
    // }

    #[test]
    fn one_ok() {
        test::run(test_file!("test/test_1_tests.txt"), |_, test_case| {
            test_case.consume_string("Key");
            Ok(())
        });
    }

    #[test]
    #[should_panic(expected = "Test failed.")]
    fn one_err() {
        test::run(test_file!("test/test_1_tests.txt"), |_, test_case| {
            test_case.consume_string("Key");
            Err(error::Unspecified)
        });
    }

    #[test]
    #[should_panic(expected = "Oh noes!")]
    fn one_panics() {
        test::run(test_file!("test/test_1_tests.txt"), |_, test_case| {
            test_case.consume_string("Key");
            panic!("Oh noes!");
        });
    }

    #[test]
    #[should_panic(expected = "Test failed.")]
    fn first_err() {
        err_one(0);
    }

    #[test]
    #[should_panic(expected = "Test failed.")]
    fn middle_err() {
        err_one(1);
    }

    #[test]
    #[should_panic(expected = "Test failed.")]
    fn last_err() {
        err_one(2);
    }

    fn err_one(test_to_fail: usize) {
        let mut n = 0;
        test::run(test_file!("test/test_3_tests.txt"), |_, test_case| {
            test_case.consume_string("Key");
            let result = if n == test_to_fail {
                Err(error::Unspecified)
            } else {
                Ok(())
            };
            n += 1;
            result
        });
    }

    #[test]
    #[should_panic(expected = "Oh Noes!")]
    fn first_panic() {
        panic_one(0);
    }

    #[test]
    #[should_panic(expected = "Oh Noes!")]
    fn middle_panic() {
        panic_one(1);
    }

    #[test]
    #[should_panic(expected = "Oh Noes!")]
    fn last_panic() {
        panic_one(2);
    }

    fn panic_one(test_to_fail: usize) {
        let mut n = 0;
        test::run(test_file!("test/test_3_tests.txt"), |_, test_case| {
            test_case.consume_string("Key");
            assert_ne!(n, test_to_fail, "Oh Noes!");
            n += 1;
            Ok(())
        });
    }

    #[test]
    #[should_panic(expected = "Syntax error: Expected Key = Value.")]
    fn syntax_error() {
        test::run(test_file!("test/test_1_syntax_error_tests.txt"), |_, _| {
            Ok(())
        });
    }

    #[test]
    fn test_to_hex_upper() {
        let hex = "abcdef0123";
        let bytes = from_dirty_hex(hex);
        assert_eq!(hex.to_ascii_uppercase(), to_hex_upper(bytes));
    }
}
