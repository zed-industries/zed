/// Replaces all the instances of `$pattern` in `$input`
/// (a `&'static str` constant) with `$replace_with` (a `&'static str` constant).
///
/// # Signature
///
/// This macro acts like a function of this signature:
/// ```rust
/// # trait Pattern {}
/// fn str_replace(
///     input: &'static str,
///     pattern: impl Pattern,
///     replace_with: &'static str,
/// ) -> &'static str
/// # {""}
/// ```
/// and is evaluated at compile-time.
///
/// Where `pattern` can be any of these types:
///
/// - `&'static str`
///
/// - `char`
///
/// - `u8`: required to be ascii (`0` up to `127` inclusive).
///
/// # Example
///
///
/// ```rust
/// use const_format::str_replace;
///
/// // Passing a string pattern
/// assert_eq!(
///     str_replace!("The incredible shrinking man.", "i", "eee"),
///     "The eeencredeeeble shreeenkeeeng man.",
/// );
///
/// // Passing a char pattern
/// assert_eq!(
///     str_replace!("The incredible shrinking man.", ' ', "---"),
///     "The---incredible---shrinking---man.",
/// );
///
/// // Passing an ascii u8 pattern.
/// assert_eq!(
///     str_replace!("The incredible shrinking man.", b'i', "eee"),
///     "The eeencredeeeble shreeenkeeeng man.",
/// );
///
/// // Removing all instances of the pattern
/// assert_eq!(
///     str_replace!("remove haire", "re", ""),
///     "move hai",
/// );
///
/// // This shows that all the arguments can be `const`s, they don't have to be literals.
/// {
///     const IN: &str = "Foo Boo Patoo";
///     const REPLACING: &str = "oo";
///     const REPLACE_WITH: &str = "uh";
///     assert_eq!(str_replace!(IN, REPLACING, REPLACE_WITH), "Fuh Buh Patuh");
/// }
/// ```
///
/// [`str::replace`]: https://doc.rust-lang.org/std/primitive.str.html#method.replace
#[macro_export]
macro_rules! str_replace {
    ($input:expr, $pattern:expr, $replace_with:expr $(,)*) => {
        $crate::__str_const! {{
            const ARGS_OSRCTFL4A: $crate::__str_methods::ReplaceInput =
                $crate::__str_methods::ReplaceInputConv($input, $pattern, $replace_with).conv();

            {
                const OB: &[$crate::pmr::u8; ARGS_OSRCTFL4A.replace_length()] =
                    &ARGS_OSRCTFL4A.replace();

                const OS: &$crate::pmr::str = unsafe { $crate::__priv_transmute_bytes_to_str!(OB) };

                OS
            }
        }}
    };
}

/// Creates a `&'static str` by repeating a `&'static str` constant `times` times
///
/// This is evaluated at compile-time.
///
/// # Example
///
/// ```rust
/// use const_format::str_repeat;
///
/// {
///     const OUT: &str = str_repeat!("hi ", 4);
///     assert_eq!(OUT, "hi hi hi hi ")
/// }
/// {
///     const IN: &str = "bye ";
///     const REPEAT: usize = 5;
///     const OUT: &str = str_repeat!(IN, REPEAT);
///     assert_eq!(OUT, "bye bye bye bye bye ")
/// }
///
/// ```
///
/// ### Failing
///
/// If this macro would produce too large a string,
/// it causes a compile-time error.
///
/// ```compile_fail
/// const_format::str_repeat!("hello", usize::MAX / 4);
/// ```
///
#[cfg_attr(
    feature = "__test",
    doc = r##"
```rust
const_format::str_repeat!("hello", usize::MAX.wrapping_add(4));
```
"##
)]
#[macro_export]
macro_rules! str_repeat {
    ($string:expr, $times:expr  $(,)*) => {
        $crate::__str_const! {{
            const P_OSRCTFL4A: &$crate::__str_methods::StrRepeatArgs =
                &$crate::__str_methods::StrRepeatArgs($string, $times);

            {
                use $crate::__hidden_utils::PtrToRef;
                use $crate::pmr::{str, transmute, u8};

                const P: &$crate::__str_methods::StrRepeatArgs = P_OSRCTFL4A;

                $crate::pmr::respan_to! {
                    ($string)
                    const _ASSERT_VALID_LEN: () = P.assert_valid();
                }

                const OUT_B: &[u8; P.out_len] = &unsafe {
                    let ptr = P.str.as_ptr() as *const [u8; P.str_len];
                    transmute::<[[u8; P.str_len]; P.repeat], [u8; P.out_len]>(
                        [*PtrToRef { ptr }.reff; P.repeat],
                    )
                };
                const OUT_S: &str = unsafe { $crate::__priv_transmute_bytes_to_str!(OUT_B) };
                OUT_S
            }
        }}
    };
}

/// Replaces a substring in a `&'static str` constant.
/// Returns both the new resulting `&'static str`, and the replaced substring.
///
/// # Alternatives
///
/// For an alternative which only returns the string with the applied replacement,
/// you can use [`str_splice_out`].
///
/// # Signature
///
/// This macro acts like a function of this signature:
/// ```rust
/// # trait SomeIndex {}
/// fn str_splice(
///     input: &'static str,
///     range: impl SomeIndex,
///     replace_with: &'static str,
/// ) -> const_format::SplicedStr
/// # {unimplemented!()}
/// ```
/// and is evaluated at compile-time.
///
/// ### `range` argument
///
/// The `range` parameter determines what part of `input` is replaced,
/// and can be any of these types:
///
/// - `usize`: the starting index of a char, only includes that char.
/// - `Range<usize>`
/// - `RangeTo<usize>`
/// - `RangeFrom<usize>`
/// - `RangeInclusive<usize>`
/// - `RangeToInclusive<usize>`
/// - `RangeFull`
///
/// [`SplicedStr`] contains:
/// - `output`: a `&'static str` with the substring at `range` in `input` replaced with
/// `replace_with`.
/// - `removed`: the substring at `range` in `input`.
///
/// # Example
///
/// ```rust
/// use const_format::{str_splice, SplicedStr};
///
/// const OUT: SplicedStr = str_splice!("foo bar baz", 4..=6, "is");
/// assert_eq!(OUT , SplicedStr{output: "foo is baz", removed: "bar"});
///
/// // You can pass `const`ants to this macro, not just literals
/// {
///     const IN: &str = "this is bad";
///     const INDEX: std::ops::RangeFrom<usize> = 8..;
///     const REPLACE_WITH: &str = "... fine";
///     const OUT: SplicedStr = str_splice!(IN, INDEX, REPLACE_WITH);
///     assert_eq!(OUT , SplicedStr{output: "this is ... fine", removed: "bad"});
/// }
/// {
///     const OUT: SplicedStr = str_splice!("ABC豆-", 3, "DEFGH");
///     assert_eq!(OUT , SplicedStr{output: "ABCDEFGH-", removed: "豆"});
/// }
/// ```
///
/// ### Invalid index
///
/// Invalid indices cause compilation errors.
///
/// ```compile_fail
/// const_format::str_splice!("foo", 0..10, "");
/// ```
#[cfg_attr(
    feature = "__test",
    doc = r#"
```rust
const_format::str_splice!("foo", 0..3, "");
```

```compile_fail
const_format::str_splice!("foo", 0..usize::MAX, "");
```

```rust
assert_eq!(
    const_format::str_splice!("効率的", 3..6, "A"),
    const_format::SplicedStr{output: "効A的", removed: "率"} ,
);
```

```compile_fail
assert_eq!(
    const_format::str_splice!("効率的", 1..6, "A"),
    const_format::SplicedStr{output: "効A的", removed: "率"} ,
);
```

```compile_fail
assert_eq!(
    const_format::str_splice!("効率的", 3..5, "A"),
    const_format::SplicedStr{output: "効A的", removed: "率"} ,
);
```

"#
)]
///
///
/// [`SplicedStr`]: ./struct.SplicedStr.html
/// [`str_splice_out`]: ./macro.str_splice_out.html
#[macro_export]
macro_rules! str_splice {
    ($string:expr, $index:expr, $insert:expr $(,)*) => {
        $crate::__const! {
            $crate::__str_methods::SplicedStr =>
            $crate::__str_splice!($string, $index, $insert)
        }
    };
}

/// Alternative version of [`str_splice`] which only returns the string
/// with the applied replacement.
///
/// # Example
///
/// ```rust
/// use const_format::{str_splice_out, SplicedStr};
///
/// const OUT: &str = str_splice_out!("foo bar baz", 4..=6, "is");
/// assert_eq!(OUT , "foo is baz");
///
/// // You can pass `const`ants to this macro, not just literals
/// {
///     const IN: &str = "this is bad";
///     const INDEX: std::ops::RangeFrom<usize> = 8..;
///     const REPLACE_WITH: &str = "... fine";
///     const OUT: &str = str_splice_out!(IN, INDEX, REPLACE_WITH);
///     assert_eq!(OUT , "this is ... fine");
/// }
/// {
///     const OUT: &str = str_splice_out!("ABC豆-", 3, "DEFGH");
///     assert_eq!(OUT , "ABCDEFGH-");
/// }
/// ```
///
/// [`str_splice`]: ./macro.str_splice.html
#[macro_export]
macro_rules! str_splice_out {
    ($string:expr, $index:expr, $insert:expr $(,)*) => {
        $crate::__str_const! {
            $crate::__str_splice!($string, $index, $insert).output
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __str_splice {
    ($string:expr, $index:expr, $insert:expr) => {{
        const P_OSRCTFL4A: $crate::__str_methods::StrSpliceArgs =
            $crate::__str_methods::StrSplceArgsConv($string, $index, $insert).conv();
        {
            use $crate::__hidden_utils::PtrToRef;
            use $crate::__str_methods::{DecomposedString, SplicedStr, StrSpliceArgs};
            use $crate::pmr::{str, u8};

            const P: &StrSpliceArgs = &P_OSRCTFL4A;

            type DecompIn =
                DecomposedString<[u8; P.used_rstart], [u8; P.used_rlen], [u8; P.suffix_len]>;

            type DecompOut =
                DecomposedString<[u8; P.used_rstart], [u8; P.insert_len], [u8; P.suffix_len]>;

            $crate::pmr::respan_to! {
                ($string)
                const _ASSERT_VALID_INDEX: () = P.index_validity.assert_valid();
            }

            const OUT_A: (&DecompOut, &str) = unsafe {
                let input = PtrToRef {
                    ptr: P.str.as_ptr() as *const DecompIn,
                }
                .reff;
                let insert = PtrToRef {
                    ptr: P.insert.as_ptr() as *const [u8; P.insert_len],
                }
                .reff;

                (
                    &DecomposedString {
                        prefix: input.prefix,
                        middle: *insert,
                        suffix: input.suffix,
                    },
                    $crate::__priv_transmute_bytes_to_str!(&input.middle),
                )
            };

            const OUT: SplicedStr = unsafe {
                let output = OUT_A.0 as *const DecompOut as *const [u8; P.out_len];
                SplicedStr {
                    output: $crate::__priv_transmute_raw_bytes_to_str!(output),
                    removed: OUT_A.1,
                }
            };

            OUT
        }
    }};
}

/// Indexes a `&'static str` constant.
///
///
/// # Signature
///
/// This macro acts like a function of this signature:
/// ```rust
/// # trait SomeIndex {}
/// fn str_index(input: &'static str, range: impl SomeIndex) -> &'static str
/// # {unimplemented!()}
/// ```
/// and is evaluated at compile-time.
///
/// This accepts
/// [the same `range` arguments as `str_splice`](macro.str_splice.html#range-argument)
///
/// # Example
///
/// ```
/// use const_format::str_index;
///
/// use std::ops::RangeFrom;
///
/// assert_eq!(str_index!("foo bar baz", ..7), "foo bar");
/// assert_eq!(str_index!("foo bar baz", 4..7), "bar");
/// assert_eq!(str_index!("foo bar baz", 4..), "bar baz");
///
/// {
///     const IN: &str = "hello world";
///     const INDEX: RangeFrom<usize> = 6..;
///     // You can pass `const`ants to this macro, not just literals
///     const OUT_0: &str = str_index!(IN, INDEX);
///     assert_eq!(OUT_0, "world");
/// }
/// {
///     const OUT: &str = str_index!("hello world", 4);
///     assert_eq!(OUT, "o");
/// }
///
/// ```
///
/// ### Invalid index
///
/// Invalid indices cause compilation errors.
///
/// ```compile_fail
/// const_format::str_index!("foo", 0..10);
/// ```
#[cfg_attr(
    feature = "__test",
    doc = r#"
```rust
assert_eq!(const_format::str_index!("効率的", 3..6), "率");
```

```compile_fail
assert_eq!(const_format::str_index!("効率的", 3..5), "率");
```
```compile_fail
assert_eq!(const_format::str_index!("効率的", 4..6), "率");
```
"#
)]
///
///
#[macro_export]
macro_rules! str_index {
    ($string:expr, $index:expr $(,)*) => {
        $crate::__str_const! {{
            const P_OSRCTFL4A: $crate::__str_methods::StrIndexArgs =
                $crate::__str_methods::StrIndexArgsConv($string, $index).conv();

            {
                $crate::pmr::respan_to! {
                    ($string)
                    const _ASSERT_VALID_INDEX: () =
                        P_OSRCTFL4A.index_validity.assert_valid();
                }

                use $crate::__hidden_utils::PtrToRef;
                use $crate::__str_methods::DecomposedString;
                type DecompIn = DecomposedString<
                    [u8; P_OSRCTFL4A.used_rstart],
                    [u8; P_OSRCTFL4A.used_rlen],
                    [u8; 0],
                >;

                const OUT: &'static str = unsafe {
                    let input = PtrToRef {
                        ptr: P_OSRCTFL4A.str.as_ptr() as *const DecompIn,
                    }
                    .reff;
                    $crate::__priv_transmute_raw_bytes_to_str!(&input.middle)
                };

                OUT
            }
        }}
    };
}

/// Indexes a `&'static str` constant,
/// returning `None` when the index is not on a character boundary.
///
///
/// # Signature
///
/// This macro acts like a function of this signature:
/// ```rust
/// # trait SomeIndex {}
/// fn str_get(input: &'static str, range: impl SomeIndex) -> Option<&'static str>
/// # {unimplemented!()}
/// ```
/// and is evaluated at compile-time.
///
/// This accepts
/// [the same `range` arguments as `str_splice`](macro.str_splice.html#range-argument)
///
/// # Example
///
/// ```
/// use const_format::str_get;
///
/// use std::ops::RangeFrom;
///
/// assert_eq!(str_get!("foo 鉄 baz", ..7), Some("foo 鉄"));
/// assert_eq!(str_get!("foo 鉄 baz", 4..7), Some("鉄"));
/// assert_eq!(str_get!("foo 鉄 baz", 4..100), None);
///
///
/// {
///     const IN: &str = "hello 鉄";
///     const INDEX: RangeFrom<usize> = 6..;
///     // You can pass `const`ants to this macro, not just literals
///     const OUT: Option<&str> = str_get!(IN, INDEX);
///     assert_eq!(OUT, Some("鉄"));
/// }
/// {
///     const OUT: Option<&str> = str_get!("hello 鉄", 4);
///     assert_eq!(OUT, Some("o"));
/// }
/// {
///     // End index not on a character boundary
///     const OUT: Option<&str> = str_get!("hello 鉄", 0..7);
///     assert_eq!(OUT, None);
/// }
/// {
///     // Out of bounds indexing
///     const OUT: Option<&str> = str_get!("hello 鉄", 0..1000 );
///     assert_eq!(OUT, None);
/// }
///
/// ```
#[cfg_attr(
    feature = "__test",
    doc = r#"
```rust
assert_eq!(const_format::str_get!("効率的", 3..6), Some("率"));
assert_eq!(const_format::str_get!("効率的", 3..5), None);
assert_eq!(const_format::str_get!("効率的", 4..6), None);
```
"#
)]
///
#[macro_export]
macro_rules! str_get {
    ($string:expr, $index:expr $(,)*) => {
        $crate::__const! {
            $crate::pmr::Option<&'static $crate::pmr::str> => {
            const P_OSRCTFL4A: $crate::__str_methods::StrIndexArgs =
                $crate::__str_methods::StrIndexArgsConv($string, $index).conv();

            {
                use $crate::__hidden_utils::PtrToRef;
                use $crate::__str_methods::DecomposedString;
                type DecompIn = DecomposedString<
                    [u8; P_OSRCTFL4A.used_rstart],
                    [u8; P_OSRCTFL4A.used_rlen],
                    [u8; 0],
                >;

                const OUT: $crate::pmr::Option<&'static $crate::pmr::str> = unsafe {
                    if P_OSRCTFL4A.index_validity.is_valid() {
                        let input = PtrToRef {
                            ptr: P_OSRCTFL4A.str.as_ptr() as *const DecompIn,
                        }
                        .reff;

                        $crate::pmr::Some($crate::__priv_transmute_raw_bytes_to_str!(&input.middle))
                    } else {
                        $crate::pmr::None
                    }
                };

                OUT
            }
        }}
    };
}

/// Splits `$string` (a `&'static str` constant) with `$splitter`,
/// returning an array of `&'static str`s.
///
/// # Alternatives
///
/// For an alternative macro which will be usable in slice patterns
/// (once [`inline_const_pat`] is stabilized)
/// you can use [`str_split_pat`].
///
/// # Signature
///
/// This macro acts like a function of this signature:
/// ```rust
/// # const LEN: usize = 0;
/// # trait Splitter {}
/// fn str_split(string: &'static str, splitter: impl Splitter) -> [&'static str; LEN]
/// # { [] }
/// ```
/// and is evaluated at compile-time.
///
/// `impl Splitter` is any of these types:
///
/// - `&'static str`
///
/// - `char`
///
/// - `u8`: only ascii values (0 up to 127 inclusive) are allowed
///
/// The value of `LEN` depends on the `string` and `splitter` arguments.
///
///
/// # Example
///
/// ```rust
/// use const_format::str_split;
///
/// assert_eq!(str_split!("this is nice", ' '), ["this", "is", "nice"]);
///
/// assert_eq!(str_split!("Hello, world!", ", "), ["Hello", "world!"]);
///
/// // A `""` splitter outputs all chars individually (`str::split` does the same)
/// assert_eq!(str_split!("🧡BAR🧠", ""), ["", "🧡", "B", "A", "R", "🧠", ""]);
///
/// // Splitting the string with an ascii byte
/// assert_eq!(str_split!("dash-separated-string", b'-'), ["dash", "separated", "string"]);
///
/// {
///     const STR: &str = "foo bar baz";
///     const SPLITTER: &str = " ";
///
///     // both arguments to the `str_aplit` macro can be non-literal constants
///     const SPLIT: [&str; 3] = str_split!(STR, SPLITTER);
///
///     assert_eq!(SPLIT, ["foo", "bar", "baz"]);
/// }
///
/// ```
///
/// [`inline_const_pat`]: https://doc.rust-lang.org/1.83.0/unstable-book/language-features/inline-const-pat.html
/// [`str_split_pat`]: crate::str_split_pat
#[macro_export]
#[cfg(feature = "rust_1_64")]
#[cfg_attr(feature = "__docsrs", doc(cfg(feature = "rust_1_64")))]
macro_rules! str_split {
    ($string:expr, $splitter:expr $(,)?) => {{
        const ARGS_OSRCTFL4A: $crate::__str_methods::SplitInput =
            $crate::__str_methods::SplitInputConv($string, $splitter).conv();

        {
            const OB: [&$crate::pmr::str; ARGS_OSRCTFL4A.length()] = ARGS_OSRCTFL4A.split_it();
            OB
        }
    }};
}

/// Version of [`str_split`] which evaluates to a `&'static [&'static str]`.
///
/// # Example
///
/// Using the currently unstable [`inline_const_pat`] feature to use this macro in patterns:
///
/// ```rust
/// # /*
/// #![feature(inline_const_pat)]
/// # */
///
/// use const_format::str_split_pat;
///
/// assert!(is_it(&["foo", "bar", "baz"]));
/// assert!(!is_it(&["foo", "bar"]));
///
/// fn is_it(slice: &[&str]) -> bool {
///     const SEP: char = ' ';
/// # /*
///     matches!(slice, str_split_pat!("foo bar baz", SEP))
/// # */
/// #   slice == str_split_pat!("foo bar baz", SEP)
/// }
/// ```
///
/// [`inline_const_pat`]: https://doc.rust-lang.org/1.83.0/unstable-book/language-features/inline-const-pat.html
#[macro_export]
#[cfg(feature = "rust_1_64")]
#[cfg_attr(feature = "__docsrs", doc(cfg(feature = "rust_1_64")))]
macro_rules! str_split_pat {
    ($string:expr, $splitter:expr $(,)?) => {
        $crate::__const! {&'static [&'static $crate::pmr::str] => {
            const ARGS_OSRCTFL4A: $crate::__str_methods::SplitInput =
                $crate::__str_methods::SplitInputConv($string, $splitter).conv();

            {
                const OB: [&$crate::pmr::str; ARGS_OSRCTFL4A.length()] = ARGS_OSRCTFL4A.split_it();
                &OB
            }
        }}
    };
}
