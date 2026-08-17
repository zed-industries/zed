//! This crate can parse a C++ “mangled” linker symbol name into a Rust value
//! describing what the name refers to: a variable, a function, a virtual table,
//! etc. The description type implements `Display`, producing human-readable
//! text describing the mangled name. Debuggers and profilers can use this crate
//! to provide more meaningful output.
//!
//! C++ requires the compiler to choose names for linker symbols consistently
//! across compilation units, so that two compilation units that have seen the
//! same declarations can pair up definitions in one unit with references in
//! another.  Almost all platforms other than Microsoft Windows follow the
//! [Itanium C++ ABI][itanium]'s rules for this.
//!
//! [itanium]: https://itanium-cxx-abi.github.io/cxx-abi/abi.html#mangling
//!
//! For example, suppose a C++ compilation unit has the definition:
//!
//! ```c++
//! namespace space {
//!   int foo(int x, int y) { return x+y; }
//! }
//! ```
//!
//! The Itanium C++ ABI specifies that the linker symbol for that function must
//! be named `_ZN5space3fooEii`. This crate can parse that name into a Rust
//! value representing its structure. Formatting the value with the `format!`
//! macro or the `std::string::ToString::to_string` trait method yields the
//! string `space::foo(int, int)`, which is more meaningful to the C++
//! developer.

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unsafe_code)]
// Clippy stuff.
#![allow(unknown_lints)]
#![allow(clippy::inline_always)]
#![allow(clippy::redundant_field_names)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

#[cfg(not(feature = "alloc"))]
compile_error!("`alloc` or `std` feature is required for this crate");

#[macro_use]
mod logging;

pub mod ast;
pub mod error;
mod index_str;
mod subs;

use alloc::string::String;
use alloc::vec::Vec;
use ast::{Demangle, Parse, ParseContext};
use core::fmt;
use core::num::NonZeroU32;
use error::{Error, Result};
use index_str::IndexStr;

/// Options to control the parsing process.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct ParseOptions {
    recursion_limit: Option<NonZeroU32>,
}

impl ParseOptions {
    /// Set the limit on recursion depth during the parsing phase. A low
    /// limit will cause valid symbols to be rejected, but a high limit may
    /// allow pathological symbols to overflow the stack during parsing.
    /// The default value is 96, which will not overflow the stack even in
    /// a debug build.
    pub fn recursion_limit(mut self, limit: u32) -> Self {
        self.recursion_limit = Some(NonZeroU32::new(limit).expect("Recursion limit must be > 0"));
        self
    }
}

/// Options to control the demangling process.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct DemangleOptions {
    no_params: bool,
    no_return_type: bool,
    hide_expression_literal_types: bool,
    recursion_limit: Option<NonZeroU32>,
}

impl DemangleOptions {
    /// Construct a new `DemangleOptions` with the default values.
    pub fn new() -> Self {
        Default::default()
    }

    /// Do not display function arguments.
    pub fn no_params(mut self) -> Self {
        self.no_params = true;
        self
    }

    /// Do not display the function return type.
    pub fn no_return_type(mut self) -> Self {
        self.no_return_type = true;
        self
    }

    /// Hide type annotations in template value parameters.
    /// These are not needed to distinguish template instances
    /// so this can make it easier to match user-provided
    /// template instance names.
    pub fn hide_expression_literal_types(mut self) -> Self {
        self.hide_expression_literal_types = true;
        self
    }

    /// Set the limit on recursion depth during the demangling phase. A low
    /// limit will cause valid symbols to be rejected, but a high limit may
    /// allow pathological symbols to overflow the stack during demangling.
    /// The default value is 128.
    pub fn recursion_limit(mut self, limit: u32) -> Self {
        self.recursion_limit = Some(NonZeroU32::new(limit).expect("Recursion limit must be > 0"));
        self
    }
}

/// A `Symbol` which owns the underlying storage for the mangled name.
pub type OwnedSymbol = Symbol<Vec<u8>>;

/// A `Symbol` which borrows the underlying storage for the mangled name.
pub type BorrowedSymbol<'a> = Symbol<&'a [u8]>;

/// A mangled symbol that has been parsed into an AST.
///
/// This is generic over some storage type `T` which can be either owned or
/// borrowed. See the `OwnedSymbol` and `BorrowedSymbol` type aliases.
#[derive(Clone, Debug, PartialEq)]
pub struct Symbol<T> {
    raw: T,
    substitutions: subs::SubstitutionTable,
    parsed: ast::MangledName,
}

impl<T> Symbol<T>
where
    T: AsRef<[u8]>,
{
    /// Given some raw storage, parse the mangled symbol from it with the default
    /// options.
    ///
    /// ```
    /// use cpp_demangle::Symbol;
    /// use std::string::ToString;
    ///
    /// // First, something easy :)
    ///
    /// let mangled = b"_ZN5space3fooEibc";
    ///
    /// let sym = Symbol::new(&mangled[..])
    ///     .expect("Could not parse mangled symbol!");
    ///
    /// let demangled = sym.to_string();
    /// assert_eq!(demangled, "space::foo(int, bool, char)");
    ///
    /// // Now let's try something a little more complicated!
    ///
    /// let mangled =
    ///     b"__Z28JS_GetPropertyDescriptorByIdP9JSContextN2JS6HandleIP8JSObjectEENS2_I4jsidEENS1_13MutableHandleINS1_18PropertyDescriptorEEE";
    ///
    /// let sym = Symbol::new(&mangled[..])
    ///     .expect("Could not parse mangled symbol!");
    ///
    /// let demangled = sym.to_string();
    /// assert_eq!(
    ///     demangled,
    ///     "JS_GetPropertyDescriptorById(JSContext*, JS::Handle<JSObject*>, JS::Handle<jsid>, JS::MutableHandle<JS::PropertyDescriptor>)"
    /// );
    /// ```
    #[inline]
    pub fn new(raw: T) -> Result<Symbol<T>> {
        Self::new_with_options(raw, &Default::default())
    }

    /// Given some raw storage, parse the mangled symbol from it.
    ///
    /// ```
    /// use cpp_demangle::{ParseOptions, Symbol};
    /// use std::string::ToString;
    ///
    /// // First, something easy :)
    ///
    /// let mangled = b"_ZN5space3fooEibc";
    ///
    /// let parse_options = ParseOptions::default()
    ///     .recursion_limit(1024);
    ///
    /// let sym = Symbol::new_with_options(&mangled[..], &parse_options)
    ///     .expect("Could not parse mangled symbol!");
    ///
    /// let demangled = sym.to_string();
    /// assert_eq!(demangled, "space::foo(int, bool, char)");
    ///
    /// // Now let's try something a little more complicated!
    ///
    /// let mangled =
    ///     b"__Z28JS_GetPropertyDescriptorByIdP9JSContextN2JS6HandleIP8JSObjectEENS2_I4jsidEENS1_13MutableHandleINS1_18PropertyDescriptorEEE";
    ///
    /// let sym = Symbol::new(&mangled[..])
    ///     .expect("Could not parse mangled symbol!");
    ///
    /// let demangled = sym.to_string();
    /// assert_eq!(
    ///     demangled,
    ///     "JS_GetPropertyDescriptorById(JSContext*, JS::Handle<JSObject*>, JS::Handle<jsid>, JS::MutableHandle<JS::PropertyDescriptor>)"
    /// );
    /// ```
    pub fn new_with_options(raw: T, options: &ParseOptions) -> Result<Symbol<T>> {
        let mut substitutions = subs::SubstitutionTable::new();

        let parsed = {
            let ctx = ParseContext::new(*options);
            let input = IndexStr::new(raw.as_ref());

            let (parsed, tail) = ast::MangledName::parse(&ctx, &mut substitutions, input)?;
            debug_assert!(ctx.recursion_level() == 0);

            if tail.is_empty() {
                parsed
            } else {
                return Err(Error::UnexpectedText);
            }
        };

        let symbol = Symbol {
            raw: raw,
            substitutions: substitutions,
            parsed: parsed,
        };

        log!(
            "Successfully parsed '{}' as

AST = {:#?}

substitutions = {:#?}",
            String::from_utf8_lossy(symbol.raw.as_ref()),
            symbol.parsed,
            symbol.substitutions
        );

        Ok(symbol)
    }

    /// Demangle the symbol and return it as a String.
    ///
    /// Unlike the `ToString` implementation, this function allows options to
    /// be specified.
    ///
    /// ```
    /// use cpp_demangle::{DemangleOptions, Symbol};
    /// use std::string::ToString;
    ///
    /// let mangled = b"_ZN5space3fooEibc";
    ///
    /// let sym = Symbol::new(&mangled[..])
    ///     .expect("Could not parse mangled symbol!");
    ///
    /// let demangled = sym.to_string();
    /// let options = DemangleOptions::default();
    /// let demangled_again = sym.demangle(&options).unwrap();
    /// assert_eq!(demangled_again, demangled);
    /// ```
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn demangle(
        &self,
        options: &DemangleOptions,
    ) -> ::core::result::Result<String, fmt::Error> {
        let mut out = String::new();
        {
            let mut ctx = ast::DemangleContext::new(
                &self.substitutions,
                self.raw.as_ref(),
                *options,
                &mut out,
            );
            self.parsed.demangle(&mut ctx, None)?;
        }

        Ok(out)
    }

    /// Demangle the symbol to a DemangleWrite, which lets the consumer be informed about
    /// syntactic structure.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn structured_demangle<W: DemangleWrite>(
        &self,
        out: &mut W,
        options: &DemangleOptions,
    ) -> fmt::Result {
        let mut ctx =
            ast::DemangleContext::new(&self.substitutions, self.raw.as_ref(), *options, out);
        self.parsed.demangle(&mut ctx, None)
    }
}

/// The type of a demangled AST node.
/// This is only partial, not all nodes are represented.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DemangleNodeType {
    /// Entering a <prefix> production
    Prefix,
    /// Entering a <template-prefix> production
    TemplatePrefix,
    /// Entering a <template-args> production
    TemplateArgs,
    /// Entering a <unqualified-name> production
    UnqualifiedName,
    /// Entering a <template-param> production
    TemplateParam,
    /// Entering a <decltype> production
    Decltype,
    /// Entering a <data-member-prefix> production
    DataMemberPrefix,
    /// Entering a <nested-name> production
    NestedName,
    /// Entering a <special-name> production that is a vtable.
    VirtualTable,
    /// Additional values may be added in the future. Use a
    /// _ pattern for compatibility.
    __NonExhaustive,
}

/// Sink for demangled text that reports syntactic structure.
pub trait DemangleWrite {
    /// Called when we are entering the scope of some AST node.
    fn push_demangle_node(&mut self, _: DemangleNodeType) {}
    /// Same as `fmt::Write::write_str`.
    fn write_string(&mut self, s: &str) -> fmt::Result;
    /// Called when we are exiting the scope of some AST node for
    /// which `push_demangle_node` was called.
    fn pop_demangle_node(&mut self) {}
}

impl<W: fmt::Write> DemangleWrite for W {
    fn write_string(&mut self, s: &str) -> fmt::Result {
        fmt::Write::write_str(self, s)
    }
}

impl<'a, T> Symbol<&'a T>
where
    T: AsRef<[u8]> + ?Sized,
{
    /// Parse a mangled symbol from input and return it and the trailing tail of
    /// bytes that come after the symbol, with the default options.
    ///
    /// While `Symbol::new` will return an error if there is unexpected trailing
    /// bytes, `with_tail` simply returns the trailing bytes along with the
    /// parsed symbol.
    ///
    /// ```
    /// use cpp_demangle::BorrowedSymbol;
    /// use std::string::ToString;
    ///
    /// let mangled = b"_ZN5space3fooEibc and some trailing junk";
    ///
    /// let (sym, tail) = BorrowedSymbol::with_tail(&mangled[..])
    ///     .expect("Could not parse mangled symbol!");
    ///
    /// assert_eq!(tail, b" and some trailing junk");
    ///
    /// let demangled = sym.to_string();
    /// assert_eq!(demangled, "space::foo(int, bool, char)");
    /// ```
    #[inline]
    pub fn with_tail(input: &'a T) -> Result<(BorrowedSymbol<'a>, &'a [u8])> {
        Self::with_tail_and_options(input, &Default::default())
    }

    /// Parse a mangled symbol from input and return it and the trailing tail of
    /// bytes that come after the symbol.
    ///
    /// While `Symbol::new_with_options` will return an error if there is
    /// unexpected trailing bytes, `with_tail_and_options` simply returns the
    /// trailing bytes along with the parsed symbol.
    ///
    /// ```
    /// use cpp_demangle::{BorrowedSymbol, ParseOptions};
    /// use std::string::ToString;
    ///
    /// let mangled = b"_ZN5space3fooEibc and some trailing junk";
    ///
    /// let parse_options = ParseOptions::default()
    ///     .recursion_limit(1024);
    ///
    /// let (sym, tail) = BorrowedSymbol::with_tail_and_options(&mangled[..], &parse_options)
    ///     .expect("Could not parse mangled symbol!");
    ///
    /// assert_eq!(tail, b" and some trailing junk");
    ///
    /// let demangled = sym.to_string();
    /// assert_eq!(demangled, "space::foo(int, bool, char)");
    /// ```
    pub fn with_tail_and_options(
        input: &'a T,
        options: &ParseOptions,
    ) -> Result<(BorrowedSymbol<'a>, &'a [u8])> {
        let mut substitutions = subs::SubstitutionTable::new();

        let ctx = ParseContext::new(*options);
        let idx_str = IndexStr::new(input.as_ref());
        let (parsed, tail) = ast::MangledName::parse(&ctx, &mut substitutions, idx_str)?;
        debug_assert!(ctx.recursion_level() == 0);

        let symbol = Symbol {
            raw: input.as_ref(),
            substitutions: substitutions,
            parsed: parsed,
        };

        log!(
            "Successfully parsed '{}' as

AST = {:#?}

substitutions = {:#?}",
            String::from_utf8_lossy(symbol.raw),
            symbol.parsed,
            symbol.substitutions
        );

        Ok((symbol, tail.into()))
    }
}

impl<T> fmt::Display for Symbol<T>
where
    T: AsRef<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        {
            let options = DemangleOptions::default();
            let mut ctx = ast::DemangleContext::new(
                &self.substitutions,
                self.raw.as_ref(),
                options,
                &mut out,
            );
            self.parsed.demangle(&mut ctx, None).map_err(|err| {
                log!("Demangling error: {:#?}", err);
                fmt::Error
            })?;
        }
        write!(f, "{}", &out)
    }
}
