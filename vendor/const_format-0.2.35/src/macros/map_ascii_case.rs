/// Converts the casing style of a `&'static str` constant,
/// ignoring non-ascii unicode characters.
///
/// This nacro is equivalent to a function with this signature:
///
/// ```rust
/// const fn map_ascii_case(case: const_format::Case, input: &'static str) -> &'static str
/// # {""}
/// ```
///
/// The [`Case`](enum.Case.html) parameter determines the casing style of the returned string.
///
/// # Ascii
///
/// This only transforms ascii characters because broader unicode case conversion,
/// while possible, is much harder to implement purely with `const fn`s.
///
/// Non-ascii characters are treated as though they're alphabetic ascii characters.
///
/// # Ignored characters
///
/// These casing styles treat non-alphanumeric ascii characters as spaces,
/// removing them from the returned string:
///
/// - `Case::Pascal`
/// - `Case::Camel`
/// - `Case::Snake`
/// - `Case::UpperSnake`
/// - `Case::Kebab`
/// - `Case::UpperKebab`
///
/// # Example
///
/// ```rust
/// use const_format::{Case, map_ascii_case};
///
/// {
///     const LOW: &str = map_ascii_case!(Case::Lower, "hello WORLD");
///     assert_eq!(LOW, "hello world");
/// }
/// {
///     const IN: &str = "hello WORLD каждому";
///     const OUT: &str = map_ascii_case!(Case::Upper, IN);
///     // non-ascii characters are ignored by map_ascii_case.
///     assert_eq!(OUT, "HELLO WORLD каждому");
/// }
///
/// const IN2: &str = "hello fooкаждому100Bar#qux";
/// {
///     const OUT: &str = map_ascii_case!(Case::Pascal, IN2);
///     assert_eq!(OUT, "HelloFooкаждому100BarQux");
/// }
/// {
///     const OUT: &str = map_ascii_case!(Case::Camel, IN2);
///     assert_eq!(OUT, "helloFooкаждому100BarQux");
/// }
/// {
///     const OUT: &str = map_ascii_case!(Case::Snake, IN2);
///     assert_eq!(OUT, "hello_fooкаждому_100_bar_qux");
/// }
/// {
///     const OUT: &str = map_ascii_case!(Case::UpperSnake, IN2);
///     assert_eq!(OUT, "HELLO_FOOкаждому_100_BAR_QUX");
/// }
/// {
///     const OUT: &str = map_ascii_case!(Case::Kebab, IN2);
///     assert_eq!(OUT, "hello-fooкаждому-100-bar-qux");
/// }
/// {
///     const OUT: &str = map_ascii_case!(Case::UpperKebab, IN2);
///     assert_eq!(OUT, "HELLO-FOOкаждому-100-BAR-QUX");
/// }
///
///
/// ```
#[macro_export]
macro_rules! map_ascii_case {
    ($case:expr, $str:expr) => {
        $crate::__str_const! {{
            const S_OSRCTFL4A: &$crate::pmr::str = $str;
            const CASE_OSRCTFL4A: $crate::Case = $case;
            {
                const L: $crate::pmr::usize =
                    $crate::__ascii_case_conv::size_after_conversion(CASE_OSRCTFL4A, S_OSRCTFL4A);

                const OB: &[$crate::pmr::u8; L] =
                    &$crate::__ascii_case_conv::convert_str::<L>(CASE_OSRCTFL4A, S_OSRCTFL4A);

                const OS: &$crate::pmr::str = unsafe { $crate::__priv_transmute_bytes_to_str!(OB) };

                OS
            }
        }}
    };
}
