/// Concatenates constants of primitive types into a `&'static str`.
///
/// Each argument is stringified after evaluating it, so `concatcp!(1u8 + 3) == "4"`
///
/// [For **examples** look here](#examples)
///
/// `concatcp` stands for "concatenate constants (of) primitives"
///
/// # Limitations
///
/// This macro can only take constants of these types as inputs:
///
/// - `&str`
///
/// - `i*`/`u*` (all the primitive integer types).
///
/// - `char`
///
/// - `bool`
///
/// This macro also shares
/// [the limitations described in here](./index.html#macro-limitations)
/// as well.
///
/// # Examples
///
/// ### Literal arguments
///
///
/// ```rust
/// use const_format::concatcp;
///
/// const MSG: &str = concatcp!(2u8, "+", 2u8, '=', 2u8 + 2);
///
/// assert_eq!(MSG, "2+2=4");
///
/// ```
///
/// ### `const` arguments
///
/// ```rust
/// use const_format::concatcp;
///
/// const PASSWORD: &str = "password";
///
/// const fn times() -> u64 { 10 }
///
/// const MSG: &str =
///     concatcp!("The password is \"", PASSWORD, "\", you can only guess ", times(), " times.");
///
/// assert_eq!(MSG, r#"The password is "password", you can only guess 10 times."#);
///
/// ```
///
#[macro_export]
macro_rules! concatcp {
    ()=>{""};
    ($($arg: expr),* $(,)?)=>(
        $crate::__str_const! {{
            use $crate::__cf_osRcTFl4A;
            $crate::pmr::__concatcp_impl!{
                $( ( $arg ), )*
            }
        }}
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __concatcp_inner {
    ($variables:expr) => {{
        #[doc(hidden)]
        const ARR_LEN: usize = $crate::pmr::PArgument::calc_len($variables);

        #[doc(hidden)]
        const CONCAT_ARR: &$crate::pmr::LenAndArray<[u8; ARR_LEN]> =
            &$crate::pmr::__priv_concatenate($variables);

        #[doc(hidden)]
        #[allow(clippy::transmute_ptr_to_ptr)]
        const CONCAT_STR: &str = unsafe {
            // This transmute truncates the length of the array to the amound of written bytes.
            let slice =
                $crate::pmr::transmute::<&[u8; ARR_LEN], &[u8; CONCAT_ARR.len]>(&CONCAT_ARR.array);

            $crate::__priv_transmute_bytes_to_str!(slice)
        };
        CONCAT_STR
    }};
}

////////////////////////////////////////////////////////////////////////////////

/// Formats constants of primitive types into a `&'static str`
///
/// [For **examples** look here](#examples)
///
/// `formatcp` stands for "format constants (of) primitives"
///
/// # Syntax
///
/// This macro uses a limited version of the syntax from the standard library [`format`] macro,
/// it can do these things:
///
/// - Take positional arguments: `formatcp!("{}{0}", "hello" )`
///
/// - Take named arguments: `formatcp!("{a}{a}", a = "hello" )`
///
/// - Use constants from scope as arguments: `formatcp!("{FOO}")`<br>
/// equivalent to the [`format_args_implicits` RFC]
///
/// - Use Debug-like formatting (eg: `formatcp!("{:?}", "hello" )`:<br>
/// Similar to how `Debug` formatting in the standard library works,
/// except that it does not escape unicode characters.
///
/// - Use LowerHex formatting (eg: `formatcp!("{:x}", "hello" )`):<br>
/// Formats numbers as lowercase hexadecimal.
/// The alternate version (written as `"{:#x}"`) prefixes the number with `0x`
///
/// - Use UpperHex formatting (eg: `formatcp!("{:X}", "hello" )`):<br>
/// Formats numbers as capitalized hexadecimal.
/// The alternate version (written as `"{:#X}"`) prefixes the number with `0x`
///
/// - Use Binary formatting (eg: `formatcp!("{:b}", "hello" )`)<br>
/// The alternate version (written as `"{:#b}"`) prefixes the number with `0b`
///
/// - Use Display formatting: `formatcp!("{}", "hello" )`
///
///
/// # Limitations
///
/// This macro can only take constants of these types as inputs:
///
/// - `&str`
///
/// - `i*`/`u*` (all the primitive integer types).
///
/// - `char`
///
/// - `bool`
///
/// This macro also shares
/// [the limitations described in here](./index.html#macro-limitations)
/// as well.
///
/// # Formating behavior
///
/// ### Debug-like
///
/// The `{:?}` formatter formats things similarly to how Debug does it.
///
/// For `&'static str` it does these things:
/// - Prepend and append the double quote character (`"`).
/// - Escape the `'\t'`,`'\n'`,`'\r'`,`'\\'`, `'\''`, and`'\"'` characters.
/// - Escape control characters with `\xYY`,
/// where `YY` is the hexadecimal value of the control character.
///
/// Example:
/// ```
/// use const_format::formatcp;
///
/// assert_eq!(formatcp!("{:?}", r#" \ " ó "#), r#"" \\ \" ó ""#);
/// ```
///
/// For `char` it does these things:
/// - Prepend and append the single quote character (`'`).
/// - Uses the same escapes as `&'static str`.
///
/// ### Display
///
/// The `{}`/`{:}` formatter produces the same output as in [`format`].
///
///
/// # Examples
///
/// ### Implicit argument
///
/// ```rust
/// use const_format::formatcp;
///
/// const NAME: &str = "John";
///
/// const MSG: &str = formatcp!("Hello {NAME}, your name is {} bytes long", NAME.len());
///
/// assert_eq!(MSG, "Hello John, your name is 4 bytes long");
///
/// ```
///
/// ### Repeating arguments
///
/// ```rust
/// use const_format::formatcp;
///
/// const MSG: &str = formatcp!("{0}{S}{0}{S}{0}", "SPAM", S = "   ");
///
/// assert_eq!(MSG, "SPAM   SPAM   SPAM");
///
/// ```
///
/// ### Debug-like and Display formatting
///
/// ```rust
/// use const_format::formatcp;
///
/// {
///     const TEXT: &str = r#"hello " \ world"#;
///     const MSG: &str = formatcp!("{TEXT}____{TEXT:?}");
///    
///     assert_eq!(MSG, r#"hello " \ world____"hello \" \\ world""#);
/// }
/// {
///     const CHARS: &str = formatcp!("{0:?} - {0} - {1} - {1:?}", '"', '👀');
///    
///     assert_eq!(CHARS, r#"'\"' - " - 👀 - '👀'"#);
/// }
/// ```
///
/// ### Additional specifiers
///
/// `const_format` macros don't support width, fill, alignment, sign,
/// or precision specifiers.
///
/// [`format`]: https://doc.rust-lang.org/std/macro.format.html
///
/// [`format_args_implicits` RFC]:
/// https://github.com/rust-lang/rfcs/blob/master/text/2795-format-args-implicit-identifiers.md
///
///
#[macro_export]
macro_rules! formatcp {
    ($format_string:expr $( $(, $expr:expr )+ )? $(,)? ) => (
        $crate::__str_const! {{
            use $crate::__cf_osRcTFl4A;

            $crate::pmr::__formatcp_impl!(
                ($format_string)
                $(, $($expr,)+)?
            )
        }}
    );
}

////////////////////////////////////////////////////////////////////////////////

/// Concatenates constants of standard library and/or user-defined types into a `&'static str`.
///
/// User defined types must implement the [`FormatMarker`] trait and
/// and have a `const_display_fmt` method (as described in the trait) to be concatenated.
///
/// # Stable equivalent
///
/// For an equivalent macro which can be used in stable Rust,
/// but can only concatenate primitive types,
/// you can use the [`concatcp`](crate::concatcp) macro.
///
/// # Limitations
///
/// This macro has [the limitations described in here](./index.html#macro-limitations).
///
/// # Examples
///
/// ### With standard library types
///
/// ```rust
///
/// use const_format::concatc;
///
/// assert_eq!(concatc!("There is ", 99u8, " monkeys!"), "There is 99 monkeys!");
///
/// ```
///
/// ### With user-defined types
///
/// ```rust
///
/// use const_format::{Formatter, Sliced, concatc, impl_fmt};
///
/// const STRING: &str = "foo bar baz";
///
/// assert_eq!(concatc!(Sliced(STRING, 4..7), ' ', Foo), "bar table");
///
/// struct Foo;
///
/// impl_fmt!{
///     impl Foo;
///     const fn const_display_fmt(&self, fmt: &mut Formatter<'_>) -> const_format::Result {
///         fmt.write_str("table")
///     }
/// }
/// ```
///
///
/// [`FormatMarker`]: ./marker_traits/trait.FormatMarker.html
///
#[cfg_attr(feature = "__docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! concatc {
    ()=>{""};
    ($($anything:tt)*)=>(
        $crate::__str_const! {{
            use $crate::__cf_osRcTFl4A;

            $crate::__concatc_expr!(($($anything)*) ($($anything)*))
            as &'static $crate::pmr::str
        }}
    )
}

#[doc(hidden)]
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! __concatc_expr {
    (($($arg: expr),* $(,)?) ($($span:tt)*) )=>({
        const fn fmt_NHPMWYD3NJA(
            mut fmt: $crate::fmt::Formatter<'_>,
        ) -> $crate::Result {
            use $crate::coerce_to_fmt as __cf_coerce_to_fmt;
            use $crate::pmr::respan_to as __cf_respan_to;
            use $crate::try_ as __cf_try;

            $({
                let __cf_respan_to!(($arg) fmt) = &mut fmt;
                __cf_respan_to!(($arg)
                    __cf_try!(__cf_coerce_to_fmt!($arg).const_display_fmt(fmt))
                );
            })*

            $crate::pmr::Ok(())
        }

        $crate::__concatc_inner!(fmt_NHPMWYD3NJA, true, $($span)*)
    })
}

#[doc(hidden)]
#[macro_export]
macro_rules! __concatc_inner {
    ($debug_fmt_fn:ident, $cond:expr, $($span:tt)*) => {{
        const fn len_nhpmwyd3nj() -> usize {
            if $cond {
                let mut strlen = __cf_osRcTFl4A::pmr::ComputeStrLength::new();
                let fmt = strlen.make_formatter(__cf_osRcTFl4A::FormattingFlags::NEW);
                match $debug_fmt_fn(fmt) {
                    __cf_osRcTFl4A::pmr::Ok(()) => strlen.len(),
                    __cf_osRcTFl4A::pmr::Err(_) => 0,
                }
            } else {
                0
            }
        }

        const LEN_NHPMWYD3NJA: usize = len_nhpmwyd3nj();

        const fn str_writer_nhpmwyd3nja(
        ) -> __cf_osRcTFl4A::msg::ErrorTupleAndStrWriter<[u8; LEN_NHPMWYD3NJA]> {
            let mut writer = __cf_osRcTFl4A::pmr::StrWriter::new([0; LEN_NHPMWYD3NJA]);
            let error = if $cond {
                $debug_fmt_fn(__cf_osRcTFl4A::pmr::Formatter::from_sw(
                    &mut writer,
                    __cf_osRcTFl4A::FormattingFlags::NEW,
                ))
            } else {
                __cf_osRcTFl4A::pmr::Ok(())
            };

            __cf_osRcTFl4A::msg::ErrorTupleAndStrWriter {
                error: __cf_osRcTFl4A::msg::ErrorTuple::new(error, &writer),
                writer,
            }
        }

        const STR_WRITER_NHPMWYD3NJA: &__cf_osRcTFl4A::msg::ErrorTupleAndStrWriter<
            [u8; LEN_NHPMWYD3NJA],
        > = &str_writer_nhpmwyd3nja();

        const _: __cf_osRcTFl4A::msg::Ok = <<__cf_osRcTFl4A::msg::ErrorPicker<
            [(); STR_WRITER_NHPMWYD3NJA.error.error_variant],
            [(); STR_WRITER_NHPMWYD3NJA.error.capacity],
        > as __cf_osRcTFl4A::msg::ErrorAsType>::Type>::NEW;

        const STR_NHPMWYD3NJA: &str = STR_WRITER_NHPMWYD3NJA.writer.unsize().as_str_alt();

        STR_NHPMWYD3NJA
    }};
}

////////////////////////////////////////////////////////////////////////////////

/// Formats constants of standard library and/or user-defined types into a `&'static str`.
///
/// User-defined types must implement the [`FormatMarker`] trait
/// (as described in the docs for that trait) to be usable with this macro.
///
/// # Stable equivalent
///
/// For an equivalent macro which can be used in stable Rust,
/// but can only format primitive types,
/// you can use the [`formatcp`](crate::formatcp) macro.
///
/// # Syntax
///
/// This macro uses the syntax described in
/// [the const_format::fmt module](./fmt/index.html#fmtsyntax)
///
/// # Limitations
///
/// This macro has [the limitations described in here](./index.html#macro-limitations).
///
/// # Example
///
/// ```rust
///
/// use const_format::for_examples::Point3;
/// use const_format::formatc;
///
/// // Formatting a non-std struct.
/// const POINT: &str = formatc!("{:?}", Point3{x: 8, y: 13, z: 21});
///
/// // Formatting a number as decimal, hexadecimal, and binary
/// const NUMBER: &str = formatc!("{0},{0:x},{0:b}", 10u8);
///
/// // Formatting the numbers in an array as decimal, hexadecimal, and binary.
/// // You can use the name of cnstants from scope, as well as named arguments.
/// const ARR: &[u32] = &[9, 25];
/// const ARRAY: &str = formatc!("{ARR:?},{ARR:X},{ARR:b}");
///
///
/// assert_eq!(POINT, "Point3 { x: 8, y: 13, z: 21 }");
/// assert_eq!(NUMBER, "10,a,1010");
/// assert_eq!(ARRAY, "[9, 25],[9, 19],[1001, 11001]");
///
/// ```
///
/// ### Custom formatting.
///
/// This example demonstrates how you can access the [`Formatter`] in arguments
/// to do custom formatting.
///
/// For more details on this you can look
/// [in the fmt module](./fmt/index.html#custom-formatting-section).
///
/// ```rust
///
/// use const_format::for_examples::Point3;
/// use const_format::{formatc, try_};
///
/// const P: Point3 = Point3{x: 5, y: 13, z: 21};
///
/// const STR: &str = formatc!("{0};{0:#X};{0:#b}", |fmt|{
///     try_!(fmt.write_u32_debug(P.x));
///     try_!(fmt.write_str(" "));
///     try_!(fmt.write_u32_debug(P.y));
///     try_!(fmt.write_char('.'));
/// });
///
/// assert_eq!(STR, "5 13.;0x5 0xD.;0b101 0b1101.");
///
/// ```
/// [`Formatter`]: crate::fmt::Formatter
/// [`FormatMarker`]: crate::marker_traits::FormatMarker
///
///
#[macro_export]
#[cfg_attr(feature = "__docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
macro_rules! formatc {
    ($format_string:expr $( $(, $expr:expr )+ )? $(,)? ) => (
        $crate::__str_const! {{
            use $crate::__cf_osRcTFl4A;

            $crate::pmr::__formatc_impl!{
                ($format_string)
                $(, $($expr,)+)?
            }
        }}
    );
}

/// Writes some formatted standard library and/or user-defined types into a buffer.
///
/// This macro evaluates to a `Result<(), const_format::Error>` which must be handled.
///
/// # Syntax
///
/// The syntax is similar to that of other formatting macros in this crate:
///
/// ```ignore
/// ẁritec!(
///     writer_expression,
///     "formatting literal",
///     positional_arg_0_expression,
///     positional_arg_1_expression,
///     named_arg_foo = expression,
///     named_arg_bar = expression,
/// )
/// ```
///
/// The syntax is otherwise the same as described in
/// [the `const_format::fmt` module](./fmt/index.html#fmtsyntax).
///
/// # Writers
///
/// The first argument must be a type that implements the [`WriteMarker`] trait,
/// and has these inherent methods:
/// ```ignore
/// const fn borrow_mutably(&mut self) -> &mut Self
/// const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_>
/// ```
///
/// [This example](#custom-writable-example) below shows how to use this macro
/// with a custom type.
///
/// # Limitations
///
/// Integer arguments must have a type inferrable from context,
/// [more details in the Integer arguments section](./index.html#integer-args).
///
/// # Examples
///
/// ### Ẁriting a Display impl.
///
/// ```
///
/// use const_format::{Error, Formatter, StrWriter};
/// use const_format::{impl_fmt, try_, writec};
///
/// pub struct Foo(u32, &'static str);
///
/// impl_fmt!{
///     impl Foo;
///     pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
///         try_!(writec!(f, "{},", self.0));
///         try_!(writec!(f, "{:?};", self.1));
///         Ok(())
///     }
/// }
///
/// // Coerces the `&mut StrWriter<[u8; 128]>` to `&mut StrWriter<[u8]>`.
/// // This is necessary because the `as_str` method is defined for `StrWriter<[u8]>`.
/// let writer: &mut StrWriter = &mut StrWriter::new([0; 128]);
/// writec!(writer, "{}", Foo(100, "bar"))?;
///
/// assert_eq!(writer.as_str(), r#"100,"bar";"#);
///
/// # Ok::<(), const_format::Error>(())
/// ```
///
/// <span id="custom-writable-example"></span>
/// ### Writing to a custom type
///
/// This example demonstrates how you can use the `ẁritec` macro with a custom type,
/// in this case it's a buffer that is cleared every time it's written.
///
/// ```rust
///
/// use const_format::marker_traits::{IsNotAStrWriter, WriteMarker};
/// use const_format::{Formatter, FormattingFlags};
/// use const_format::writec;
///
/// const ARRAY_CAP: usize = 20;
/// struct Array {
///     len: usize,
///     arr: [u8; ARRAY_CAP],
/// }
///
/// impl WriteMarker for Array{
///     type Kind = IsNotAStrWriter;
///     type This = Self;
/// }
///
/// impl Array {
///     // Gets the part of the array that has been written to.
///     pub const fn as_bytes(&self) -> &[u8] {
///         const_format::utils::slice_up_to_len_alt(&self.arr, self.len)
///     }
///
///     pub const fn borrow_mutably(&mut self) -> &mut Self {
///         self
///     }
///
///     pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
///         Formatter::from_custom_cleared(&mut self.arr, &mut self.len, flags)
///     }
/// }
///
///
/// let mut buffer = Array{ arr: [0; ARRAY_CAP], len: 0 };
///
/// writec!(buffer, "{:?}", [3u8, 5, 8, 13, 21])?;
/// assert_eq!(buffer.as_bytes(), b"[3, 5, 8, 13, 21]");
///
/// writec!(buffer, "{}{}", "Hello, world!", 100u16)?;
/// assert_eq!(buffer.as_bytes(), b"Hello, world!100");
///
/// # Ok::<(), const_format::Error>(())
/// ```
///
/// ### Custom formatting.
///
/// This example demonstrates how you can access the [`Formatter`] in arguments
/// to do custom formatting.
///
/// Note that `return` inside arguments returns from the function around the `writec`.
///
/// For more details on this you can look
/// [in the fmt module](./fmt/index.html#custom-formatting-section).
///
/// ```rust
///
/// use const_format::for_examples::Point3;
/// use const_format::{StrWriter, call_debug_fmt, try_, writec};
///
/// const P: Point3 = Point3{x: 5, y: 13, z: 21};
///
/// let writer: &mut StrWriter = &mut StrWriter::new([0; 128]);
///
/// writec!(
///     writer,
///     "The options are: {}, and {}",
///     |fmt| call_debug_fmt!(Option, Some(P), fmt),
///     |fmt| call_debug_fmt!(Option, None::<Point3>, fmt),
/// )?;
///
/// assert_eq!(writer.as_str(), "The options are: Some(Point3 { x: 5, y: 13, z: 21 }), and None");
///
/// # Ok::<(), const_format::Error>(())
/// ```
///
/// ### Locals in the format string
///
/// This example demonstrates how you can format local variables,
/// by using their identifiers in the format string.
///
/// ```rust
///
/// use const_format::{Formatter, FormattingFlags, StrWriter, try_, writec};
///
/// const fn writeit(mut fmt: Formatter<'_>, foo: u32, bar: &str) -> const_format::Result {
///     try_!(writec!(fmt, "{foo},{foo:?},{foo:#x},{foo:#b};"));
///     try_!(writec!(fmt, "{bar},{bar:?}"));
///     Ok(())
/// }
///
/// let writer: &mut StrWriter = &mut StrWriter::new([0; 128]);
///
/// writeit(writer.make_formatter(FormattingFlags::NEW), 100, "hello")?;
///
/// assert_eq!(writer.as_str(), r#"100,100,0x64,0b1100100;hello,"hello""#);
///
/// # Ok::<(), const_format::Error>(())
/// ```
///
/// [`Formatter`]: ./fmt/struct.Formatter.html
/// [`WriteMarker`]: ./marker_traits/trait.WriteMarker.html
///
///
///
///
#[macro_export]
#[cfg_attr(feature = "__docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
macro_rules! writec {
    ( $writer:expr, $format_string:expr $( $(, $expr:expr )+ )? $(,)? ) => ({
        use $crate::__cf_osRcTFl4A;

        $crate::pmr::__writec_impl!{
            ($writer)
            ($format_string)
            $(, $($expr,)+)?
        }
    });
}
