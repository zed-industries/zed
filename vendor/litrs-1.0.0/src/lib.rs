//! Parsing and inspecting Rust literal tokens.
//!
//! This library offers functionality to parse Rust literals, i.e. tokens in the
//! Rust programming language that represent fixed values. The grammar for
//! those is defined [here][ref].
//!
//! This kind of functionality already exists in the crate `syn`. However, as
//! you oftentimes don't need (nor want) the full power of `syn`, `litrs` was
//! built. This crate also offers a bit more flexibility compared to `syn`
//! (only regarding literals, of course).
//!
//!
//! # Quick start
//!
//! | **`StringLit::try_from(tt)?.value()`** |
//! | - |
//!
//! ... where `tt` is a `proc_macro::TokenTree` and where [`StringLit`] can be
//! replaced with [`Literal`] or other types of literals (e.g. [`FloatLit`]).
//! Calling `value()` returns the value that is represented by the literal.
//!
//! **Mini Example**
//!
//! ```ignore
//! use proc_macro::TokenStream;
//!
//! #[proc_macro]
//! pub fn foo(input: TokenStream) -> TokenStream {
//!      let first_token = input.into_iter().next().unwrap(); // Do proper error handling!
//!      let string_value = match litrs::StringLit::try_from(first_token) {
//!          Ok(string_lit) => string_lit.value(),
//!          Err(e) => return e.to_compile_error(),
//!      };
//!
//!      // `string_value` is the string value with all escapes resolved.
//!      todo!()
//! }
//! ```
//!
//! # Overview
//!
//! The main types of this library are [`Literal`], representing any kind of
//! literal, and `*Lit`, like [`StringLit`] or [`FloatLit`], representing a
//! specific kind of literal.
//!
//! There are different ways to obtain such a literal type:
//!
//! - **`parse`**: parses a `&str` or `String` and returns `Result<_,
//!     ParseError>`. For example: [`Literal::parse`] and
//!     [`IntegerLit::parse`].
//!
//! - **`From<proc_macro::Literal> for Literal`**: turns a `Literal` value from
//!     the `proc_macro` crate into a `Literal` from this crate.
//!
//! - **`TryFrom<proc_macro::Literal> for *Lit`**: tries to turn a
//!     `proc_macro::Literal` into a specific literal type of this crate. If
//!     the input is a literal of a different kind, `Err(InvalidToken)` is
//!     returned.
//!
//! - **`TryFrom<proc_macro::TokenTree>`**: attempts to turn a token tree into a
//!     literal type of this crate. An error is returned if the token tree is
//!     not a literal, or if you are trying to turn it into a specific kind of
//!     literal and the token tree is a different kind of literal.
//!
//! All of the `From` and `TryFrom` conversions also work for reference to
//! `proc_macro` types. Additionally, if the crate feature `proc-macro2` is
//! enabled, all these `From` and `TryFrom` impls also exist for the
//! corresponding `proc_macro2` types.
//!
//! **Note**: `true` and `false` are `Ident`s when passed to your proc macro.
//! The `TryFrom<TokenTree>` impls check for those two special idents and
//! return a [`BoolLit`] appropriately. For that reason, there is also no
//! `TryFrom<proc_macro::Literal>` impl for [`BoolLit`]. The `proc_macro::Literal`
//! simply cannot represent bool literals.
//!
//!
//! # Examples
//!
//! In a proc-macro:
//!
//! ```ignore
//! use std::convert::TryFrom;
//! use proc_macro::TokenStream;
//! use litrs::FloatLit;
//!
//! #[proc_macro]
//! pub fn foo(input: TokenStream) -> TokenStream {
//!      let mut input = input.into_iter().collect::<Vec<_>>();
//!      if input.len() != 1 {
//!          // Please do proper error handling in your real code!
//!          panic!("expected exactly one token as input");
//!      }
//!      let token = input.remove(0);
//!
//!      match FloatLit::try_from(token) {
//!          Ok(float_lit) => { /* do something */ }
//!          Err(e) => return e.to_compile_error(),
//!      }
//!
//!      // Dummy output
//!      TokenStream::new()
//! }
//! ```
//!
//! Parsing from string:
//!
//! ```
//! use litrs::{FloatLit, Literal};
//!
//! // Parse a specific kind of literal (float in this case):
//! let float_lit = FloatLit::parse("3.14f32");
//! assert!(float_lit.is_ok());
//! assert_eq!(float_lit.unwrap().suffix(), "f32");
//! assert!(FloatLit::parse("'c'").is_err());
//!
//! // Parse any kind of literal. After parsing, you can inspect the literal
//! // and decide what to do in each case.
//! let lit = Literal::parse("0xff80").expect("failed to parse literal");
//! match lit {
//!     Literal::Integer(lit) => { /* ... */ }
//!     Literal::Float(lit) => { /* ... */ }
//!     Literal::Bool(lit) => { /* ... */ }
//!     Literal::Char(lit) => { /* ... */ }
//!     Literal::String(lit) => { /* ... */ }
//!     Literal::Byte(lit) => { /* ... */ }
//!     Literal::ByteString(lit) => { /* ... */ }
//!     Literal::CString(lit) => { /* ... */ }
//!     _ => { /* ... */ }
//! }
//! ```
//!
//! # SemVer/Versioning guarantees
//!
//! Some technically breaking changes might be released as a minor/patch version
//! in some situations, for example:
//! - Bugs in this library (e.g. behavior different from rustc)
//! - Rust making breaking changes, likely via new edition
//!
//! In all cases, releasing these changes as a minor/patch version is only done
//! if it is expected that breakage is minimal or non-existent.
//!
//!
//! # Crate features
//!
//! - `proc-macro2`: adds the dependency `proc_macro2`, a bunch of `From` and
//!   `TryFrom` impls, and [`InvalidToken::to_compile_error2`].
//! - `check_suffix`: if enabled, `parse` functions will exactly verify that the
//!   literal suffix is valid. Adds the dependency `unicode-xid`. If disabled,
//!   only an approximate check (only in ASCII range) is done. If you are
//!   writing a proc macro, you don't need to enable this as the suffix is
//!   already checked by the compiler.
//!
//!
//! [ref]: https://doc.rust-lang.org/reference/tokens.html#literals
//!

#![deny(missing_debug_implementations)]

extern crate proc_macro;

#[cfg(test)]
#[macro_use]
mod test_util;

#[cfg(test)]
mod tests;

mod bool;
mod byte;
mod bytestr;
mod char;
mod cstr;
mod err;
mod escape;
mod float;
mod impls;
mod integer;
mod parse;
mod string;


use std::{
    borrow::{Borrow, Cow},
    fmt,
    ops::{Deref, Range},
};

pub use self::{
    bool::BoolLit,
    byte::ByteLit,
    bytestr::ByteStringLit,
    char::CharLit,
    cstr::CStringLit,
    err::{InvalidToken, ParseError},
    float::{FloatLit, FloatType},
    integer::{FromIntegerLiteral, IntegerBase, IntegerLit, IntegerType},
    string::StringLit,
};


// ==============================================================================================
// ===== `Literal` and type defs
// ==============================================================================================

/// A literal. This is the main type of this library.
///
/// This type is generic over the underlying buffer `B`, which can be `&str` or
/// `String`.
///
/// To create this type, you have to either call [`Literal::parse`] with an
/// input string or use the `From<_>` impls of this type. The impls are only
/// available of the corresponding crate features are enabled (they are enabled
/// by default).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Literal<B: Buffer> {
    Bool(BoolLit),
    Integer(IntegerLit<B>),
    Float(FloatLit<B>),
    Char(CharLit<B>),
    String(StringLit<B>),
    Byte(ByteLit<B>),
    ByteString(ByteStringLit<B>),
    CString(CStringLit<B>),
}

impl<B: Buffer> Literal<B> {
    /// Parses the given input as a Rust literal.
    pub fn parse(input: B) -> Result<Self, ParseError> {
        parse::parse(input)
    }

    /// Returns the suffix of this literal or `""` if it doesn't have one.
    ///
    /// Rust token grammar actually allows suffixes for all kinds of tokens.
    /// Most Rust programmer only know the type suffixes for integer and
    /// floats, e.g. `0u32`. And in normal Rust code, everything else causes an
    /// error. But it is possible to pass literals with arbitrary suffixes to
    /// proc macros, for example:
    ///
    /// ```ignore
    /// some_macro!(3.14f33  16px  '🦊'good_boy  "toph"beifong);
    /// ```
    ///
    /// Boolean literals, not actually being literals, but idents, cannot have
    /// suffixes and this method always returns `""` for those.
    ///
    /// There are some edge cases to be aware of:
    /// - Integer suffixes must not start with `e` or `E` as that conflicts with
    ///   the exponent grammar for floats. `0e1` is a float; `0eel` is also
    ///   parsed as a float and results in an error.
    /// - Hexadecimal integers eagerly parse digits, so `0x5abcdefgh` has a
    ///   suffix von `gh`.
    /// - Suffixes can contain and start with `_`, but for integer and number
    ///   literals, `_` is eagerly parsed as part of the number, so `1_x` has
    ///   the suffix `x`.
    /// - The input `55f32` is regarded as integer literal with suffix `f32`.
    ///
    /// # Example
    ///
    /// ```
    /// use litrs::Literal;
    ///
    /// assert_eq!(Literal::parse(r##"3.14f33"##).unwrap().suffix(), "f33");
    /// assert_eq!(Literal::parse(r##"123hackerman"##).unwrap().suffix(), "hackerman");
    /// assert_eq!(Literal::parse(r##"0x0fuck"##).unwrap().suffix(), "uck");
    /// assert_eq!(Literal::parse(r##"'🦊'good_boy"##).unwrap().suffix(), "good_boy");
    /// assert_eq!(Literal::parse(r##""toph"beifong"##).unwrap().suffix(), "beifong");
    /// ```
    pub fn suffix(&self) -> &str {
        match self {
            Literal::Bool(_) => "",
            Literal::Integer(l) => l.suffix(),
            Literal::Float(l) => l.suffix(),
            Literal::Char(l) => l.suffix(),
            Literal::String(l) => l.suffix(),
            Literal::Byte(l) => l.suffix(),
            Literal::ByteString(l) => l.suffix(),
            Literal::CString(l) => l.suffix(),
        }
    }

    /// Returns the raw input that was passed to `parse`.
    ///
    /// This can be used to compare literals with different `Buffer` types.
    /// Note: this does not necessarily point to the same string buffer, in
    /// particular, bool literals just return a `&'static str`.
    pub fn raw_input(&self) -> &str {
        match self {
            Literal::Bool(l) => l.as_str(),
            Literal::Integer(l) => l.raw_input(),
            Literal::Float(l) => l.raw_input(),
            Literal::Char(l) => l.raw_input(),
            Literal::String(l) => l.raw_input(),
            Literal::Byte(l) => l.raw_input(),
            Literal::ByteString(l) => l.raw_input(),
            Literal::CString(l) => l.raw_input(),
        }
    }
}

impl Literal<&str> {
    /// Makes a copy of the underlying buffer and returns the owned version of
    /// `Self`.
    pub fn into_owned(self) -> Literal<String> {
        match self {
            Literal::Bool(l) => Literal::Bool(l.to_owned()),
            Literal::Integer(l) => Literal::Integer(l.to_owned()),
            Literal::Float(l) => Literal::Float(l.to_owned()),
            Literal::Char(l) => Literal::Char(l.to_owned()),
            Literal::String(l) => Literal::String(l.into_owned()),
            Literal::Byte(l) => Literal::Byte(l.to_owned()),
            Literal::ByteString(l) => Literal::ByteString(l.into_owned()),
            Literal::CString(l) => Literal::CString(l.into_owned()),
        }
    }
}

impl<B: Buffer> fmt::Display for Literal<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Bool(l) => l.fmt(f),
            Literal::Integer(l) => l.fmt(f),
            Literal::Float(l) => l.fmt(f),
            Literal::Char(l) => l.fmt(f),
            Literal::String(l) => l.fmt(f),
            Literal::Byte(l) => l.fmt(f),
            Literal::ByteString(l) => l.fmt(f),
            Literal::CString(l) => l.fmt(f),
        }
    }
}


// ==============================================================================================
// ===== Buffer
// ==============================================================================================

/// A shared or owned string buffer. Implemented for `String` and `&str`. *Implementation detail*.
///
/// This is trait is implementation detail of this library, cannot be
/// implemented in other crates and is not subject to semantic versioning.
/// `litrs` only guarantees that this trait is implemented for `String` and
/// `for<'a> &'a str`.
pub trait Buffer: sealed::Sealed + Deref<Target = str> {
    /// This is `String` for `String`, and `Cow<'a, str>` for `&'a str`.
    type Cow: From<String> + AsRef<str> + Borrow<str> + Deref<Target = str>;

    #[doc(hidden)]
    fn into_cow(self) -> Self::Cow;

    /// This is `Vec<u8>` for `String`, and `Cow<'a, [u8]>` for `&'a str`.
    type ByteCow: From<Vec<u8>> + AsRef<[u8]> + Borrow<[u8]> + Deref<Target = [u8]>;

    #[doc(hidden)]
    fn into_byte_cow(self) -> Self::ByteCow;

    /// Cuts away some characters at the beginning and some at the end. Given
    /// range has to be in bounds.
    #[doc(hidden)]
    fn cut(self, range: Range<usize>) -> Self;
}

mod sealed {
    pub trait Sealed {}
}

impl sealed::Sealed for &'_ str {}
impl<'a> Buffer for &'a str {
    #[doc(hidden)]
    fn cut(self, range: Range<usize>) -> Self {
        &self[range]
    }

    type Cow = Cow<'a, str>;
    #[doc(hidden)]
    fn into_cow(self) -> Self::Cow {
        self.into()
    }
    type ByteCow = Cow<'a, [u8]>;
    #[doc(hidden)]
    fn into_byte_cow(self) -> Self::ByteCow {
        self.as_bytes().into()
    }
}

impl sealed::Sealed for String {}
impl Buffer for String {
    #[doc(hidden)]
    fn cut(mut self, range: Range<usize>) -> Self {
        // This is not the most efficient way, but it works. First we cut the
        // end, then the beginning. Note that `drain` also removes the range if
        // the iterator is not consumed.
        self.truncate(range.end);
        self.drain(..range.start);
        self
    }

    type Cow = String;
    #[doc(hidden)]
    fn into_cow(self) -> Self::Cow {
        self
    }

    type ByteCow = Vec<u8>;
    #[doc(hidden)]
    fn into_byte_cow(self) -> Self::ByteCow {
        self.into_bytes()
    }
}
