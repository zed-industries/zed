//! [`std::fmt`](https://doc.rust-lang.org/std/fmt/)-like
//! api that can be used at compile-time.
//!
//! # Features
//!
//! This module requires the "fmt" feature to be exported, and Rust 1.83.0,
//! because it uses `&mut` in `const fn`s.
//!
//! # Implementing the formatting methods
//!
//! Users of this library can implement debug and display formatting by
//! defining `const_debug_fmt` and `const_display_fmt` inherent methods
//! with the
//! ```ignore
//! // use const_format::{Formatter, Error};
//! const fn const_debug_fmt(&self, &mut Formatter<'_>) -> Result<(), Error>
//! const fn const_display_fmt(&self, &mut Formatter<'_>) -> Result<(), Error>
//! ```
//! signatures,
//! and implementing the [`FormatMarker`] trait.
//!
//! # Limitations
//!
//! ### Generic impls
//!
//! Because the formatting of custom types is implemented with duck typing,
//! it's not possible to format generic types, instead you must do either of these:
//!
//! - Provide all the implementations ahead of time, what [`impl_fmt`] is for.
//!
//! - Provide a macro that formats the type.
//! The `call_debug_fmt` macro is a version of this that formats generic std types.
//!
//! <span id = "fmtsyntax"></span>
//! # Formatting Syntax
//!
//! The formatting macros all share the formatting syntax,
//! modeled after the syntax of the formatting macros of the standard library.
//!
//! ### Arguments
//!
//! Arguments in the format string can be named and positional in these ways:
//!
//! - Implicitly positional(eg: `formatc!("{}", 20u8)`):<br>
//! Starts at the 0th positional argument and increments with every use.
//!
//! - Explicit positional(eg: `formatc!("{0}", 10u8)`).
//!
//! - Named, passed to the macro as named arguments (eg: `formatc!("{foo}", foo = 10u8)`).
//!
//! - Named, from constant (eg: `formatc!("{FOO}")`):
//! Uses the `FOO` constant from the enclosing scope.
//!
//! - Named, from locals (eg: `writec!(writable, "{foo}")`):
//! Uses the `foo` local variable from the enclosing scope,
//! only usable with the [`writec`] macro.
//!
//! ### Formatters
//!
//! The format arguments can be formatted in these ways:
//!
//! - Debug formatting (eg: `formatc!("{:?}", 0u8)` ):<br>
//! Similar to how Debug formatting in the standard library works,
//! except that it does not escape unicode characters.
//!
//! - Display formatting (eg: `formatc!("{}", 0u8)`, `formatc!("{:}", 0u8)` )
//!
//! - Lowercase hexadecimal formatting (eg: `formatc!("{:x}", 0u8)`):<br>
//! Writes numbers in lowercase hexadecimal.
//! This can be combined with debug formatting with the `"{:x?}"` formatter.
//!
//! - Uppercase hexadecimal formatting (eg: `formatc!("{:X}", 0u8)`):<br>
//! Writes numbers in uppercase hexadecimal.
//! This can be combined with debug formatting with the `"{:X?}"` formatter.
//!
//! - Binary formatting (eg: `formatc!("{:b}", 0u8)`):<br>
//! This can be combined with debug formatting with the `"{:b?}"` formatter.
//!
//! ### Alternate flag
//!
//! The alternate flag allows types to format themselves in an alternate way,
//! written as "#" in a format string argument. eg:`"{:#}", "{:#?}"`.
//!
//! This is the built-in behavior for the alternate flag:
//!
//! - The Debug formater (eg: `formatc!("{:#?}", FOO)`):
//! pretty print structs and enums.
//!
//! - The hexadecimal formater (eg: `formatc!("{:#x}", FOO)`):
//! prefixes numbers with `0x`.
//!
//! - The binary formater (eg: `formatc!("{:#b}", FOO)`):
//! prefixes numbers with `0b`.
//!
//! ### Additional specifiers
//!
//! `const_format` macros don't support width, fill, alignment, sign,
//! or precision specifiers.
//!
//! <span id="custom-formatting-section"></span>
//! ### Custom formatting
//!
//! Arguments can access a [`Formatter`] for custom formatting with an
//! `|identifier|` before the expression.
//!
//! The expression will be evaluated every time it is used in the formatting string.
//!
//! The expression can evaluate to either a `()` or a `Result<(), const_format::Error>`.
//!
//! Note: this doesn't distinguish between debug and display formatting.
//!
//! [Link to full example of custom formatting](#custom-formatting-example)
//!
//! # Examples
//!
//! ### Derive
//!
//! This example demonstrates how you can derive [`ConstDebug`], and use it with the `fmt` API.
//!
//! It uses the "derive" feature
//!
#![cfg_attr(feature = "derive", doc = "```rust")]
#![cfg_attr(not(feature = "derive"), doc = "```ignore")]
//!
//! use const_format::{Error, Formatter, FormattingFlags, PWrapper, StrWriter};
//! use const_format::{ConstDebug, try_, unwrap, writec};
//!
//! use std::ops::Range;
//!
//! #[derive(ConstDebug)]
//! pub struct Foo {
//!     range: Option<Range<usize>>,
//!     point: Point,
//! }
//!
//! #[derive(ConstDebug)]
//! pub struct Point {
//!     x: u32,
//!     y: u32,
//! }
//!
//! const CAP: usize = 90;
//! const fn build_string() -> StrWriter<[u8; CAP]> {
//!     let mut writer = StrWriter::new([0; CAP]);
//!
//!     let foo = Foo {
//!         range: Some(0..10),
//!         point: Point{ x: 13, y: 21 },
//!     };
//!
//!     unwrap!(writec!(writer, "{:X?}", foo));
//!
//!     writer
//! }
//!
//! const STRING: &str = {
//!     const STR: &StrWriter = &build_string();
//!     STR.as_str_alt()
//! };
//!
//! // The formatter
//! assert_eq!(
//!     STRING,
//!     "Foo { range: Some(0..A), point: Point { x: D, y: 15 } }",
//! );
//!
//! ```
//!
//!
//! ### No proc macros
//!
//! This example demonstrates how you can use the `fmt` api without using any proc macros.
//!
//! ```rust
//!
//! use const_format::{Error, Formatter, FormattingFlags, PWrapper, StrWriter};
//! use const_format::{call_debug_fmt, coerce_to_fmt, impl_fmt, try_};
//!
//! use std::cmp::Ordering;
//!
//! pub struct Foo<T, U> {
//!     a: u32,
//!     b: u32,
//!     c: T,
//!     d: [Ordering; 3],
//!     ignored: U,
//! }
//!
//! //
//! impl_fmt!{
//!     // The type parameters of the impl must be written with trailing commas
//!     impl[U,] Foo<u32, U>;
//!     impl[U,] Foo<&str, U>;
//!
//!     pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//!         let mut f = f.debug_struct("Foo");
//!
//!         // PWrapper is a wrapper for std types, which defines the formatter methods for them.
//!         try_!(PWrapper(self.a).const_debug_fmt(f.field("a")));
//!
//!         try_!(PWrapper(self.b).const_debug_fmt(f.field("b")));
//!
//!         // The `coerce_to_fmt` macro automatically wraps std types in `PWrapper`
//!         // and does nothing with non-std types.
//!         try_!(coerce_to_fmt!(self.c).const_debug_fmt(f.field("c")));
//!
//!         // This macro allows debug formatting of some generic types which
//!         // wrap non-std types, including:
//!         // - arrays   - slices    - Option    - newtype wrappers
//!         call_debug_fmt!(array, self.d, f.field("d"));
//!
//!         f.finish()
//!     }
//! }
//!
//! const CAP: usize = 128;
//!
//! const fn build_string() -> StrWriter<[u8; CAP]> {
//!     let flags = FormattingFlags::NEW.set_alternate(true);
//!     let mut writer = StrWriter::new([0; CAP]);
//!
//!     const_format::unwrap!(
//!         Foo {
//!             a: 5,
//!             b: 8,
//!             c: 13,
//!             d: [Ordering::Less, Ordering::Equal, Ordering::Greater],
//!             ignored: (),
//!         }.const_debug_fmt(&mut Formatter::from_sw(&mut writer, flags))
//!     );
//!
//!     writer
//! }
//!
//! const STRING: &str = {
//!     const S: &StrWriter = &build_string();
//!     S.as_str_alt()
//! };
//!
//! assert_eq!(
//!     STRING,
//!     "\
//! Foo {
//!     a: 5,
//!     b: 8,
//!     c: 13,
//!     d: [
//!         Less,
//!         Equal,
//!         Greater,
//!     ],
//! }\
//!     ",
//! );
//!
//!
//! ```
//!
//! <span id="custom-formatting-example"></span>
//! ### Custom formatting
//!
//! This example demonstrates how you can do [custom formatting](#custom-formatting-section),
//! by using a `Formatter` directly.
//!
//! ```rust
//!
//! use const_format::{call_debug_fmt, formatc};
//!
//! // Positional argument
//! assert_eq!(formatc!("{}", |fmt| fmt.write_ascii_repeated(b'a', 4) ), "aaaa");
//!
//! // Named argument
//! assert_eq!(formatc!("{foo}", foo = |fmt| fmt.write_ascii_repeated(b'0', 10) ), "0000000000");
//!
//! // Repeating a positional argument multiple times
//! assert_eq!(formatc!("{0}{0}{0}", |fmt| fmt.write_str("hi") ), "hihihi");
//!
//! // Using debug formatting is no different to display formatting:
//! assert_eq!(formatc!("{0:?}{0:?}{0:?}", |fmt| fmt.write_str("hi") ), "hihihi");
//!
//! // But binary/hex formatting, and the alternate flag, do have an effect:
//! assert_eq!(
//!     formatc!(
//!         "{0}\n{0:x}\n{0:X}\n{0:b}\n{0:#x}\n{0:#X}\n{0:#b}",
//!         |fmt| call_debug_fmt!(array, [3u8, 13], fmt),
//!     ),
//!     "\
//!         [3, 13]\n\
//!         [3, d]\n\
//!         [3, D]\n\
//!         [11, 1101]\n\
//!         [\n    0x3,\n    0xd,\n]\n\
//!         [\n    0x3,\n    0xD,\n]\n\
//!         [\n    0b11,\n    0b1101,\n]\
//!     ",
//! );
//!
//! ```
//!
//!
//! [`writec`]: ../macro.writec.html
//! [`Formatter`]: ./struct.Formatter.html
//! [`FormatMarker`]: ../marker_traits/trait.FormatMarker.html
//! [`ConstDebug`]: ../derive.ConstDebug.html
//!
//!
//!
//!
//!

mod error;
mod formatter;
mod std_type_impls;
mod str_writer;
mod str_writer_mut;

pub use crate::formatting::{FormattingFlags, NumberFormatting};

pub use self::{
    error::{Error, Result, ToResult},
    formatter::{ComputeStrLength, DebugList, DebugSet, DebugStruct, DebugTuple, Formatter},
    str_writer::StrWriter,
    str_writer_mut::{NoEncoding, StrWriterMut, Utf8Encoding},
};
