// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type = "proc-macro"]
#![doc(html_root_url = "https://docs.rs/num-derive/0.3")]
#![recursion_limit = "512"]

//! Procedural macros to derive numeric traits in Rust.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! num-traits = "0.2"
//! num-derive = "0.3"
//! ```
//!
//! Then you can derive traits on your own types:
//!
//! ```rust
//! #[macro_use]
//! extern crate num_derive;
//!
//! #[derive(FromPrimitive, ToPrimitive)]
//! enum Color {
//!     Red,
//!     Blue,
//!     Green,
//! }
//! # fn main() {}
//! ```
//!
//! ## Explicit import
//!
//! By default the `num_derive` procedural macros assume that the
//! `num_traits` crate is a direct dependency. If `num_traits` is instead
//! a transitive dependency, the `num_traits` helper attribute can be
//! used to tell `num_derive` to use a specific identifier for its imports.
//!
//! ```rust
//! #[macro_use]
//! extern crate num_derive;
//! // Lets pretend this is a transitive dependency from another crate
//! // reexported as `some_other_ident`.
//! extern crate num_traits as some_other_ident;
//!
//! #[derive(FromPrimitive, ToPrimitive)]
//! #[num_traits = "some_other_ident"]
//! enum Color {
//!     Red,
//!     Blue,
//!     Green,
//! }
//! # fn main() {}
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Data, Fields, Ident};

/// Try to parse the tokens, or else return a compilation error
macro_rules! parse {
    ($tokens:ident as $type:ty) => {
        match syn::parse::<$type>($tokens) {
            Ok(parsed) => parsed,
            Err(error) => {
                return TokenStream::from(error.to_compile_error());
            }
        }
    };
}

// Within `exp`, you can bring things into scope with `extern crate`.
//
// We don't want to assume that `num_traits::` is in scope - the user may have imported it under a
// different name, or may have imported it in a non-toplevel module (common when putting impls
// behind a feature gate).
//
// Solution: let's just generate `extern crate num_traits as _num_traits` and then refer to
// `_num_traits` in the derived code.  However, macros are not allowed to produce `extern crate`
// statements at the toplevel.
//
// Solution: let's generate `mod _impl_foo` and import num_traits within that.  However, now we
// lose access to private members of the surrounding module.  This is a problem if, for example,
// we're deriving for a newtype, where the inner type is defined in the same module, but not
// exported.
//
// Solution: use the anonymous const trick.  For some reason, `extern crate` statements are allowed
// here, but everything from the surrounding module is in scope.  This trick is taken from serde.
fn anon_const_trick(exp: TokenStream2) -> TokenStream2 {
    quote! {
        #[allow(non_upper_case_globals, unused_qualifications)]
        const _: () = {
            #[allow(clippy::useless_attribute)]
            #[allow(rust_2018_idioms)]
            extern crate num_traits as _num_traits;
            #exp
        };
    }
}

// If `data` is a newtype, return the type it's wrapping.
fn newtype_inner(data: &syn::Data) -> Option<syn::Type> {
    match *data {
        Data::Struct(ref s) => {
            match s.fields {
                Fields::Unnamed(ref fs) => {
                    if fs.unnamed.len() == 1 {
                        Some(fs.unnamed[0].ty.clone())
                    } else {
                        None
                    }
                }
                Fields::Named(ref fs) => {
                    if fs.named.len() == 1 {
                        panic!("num-derive doesn't know how to handle newtypes with named fields yet. \
                           Please use a tuple-style newtype, or submit a PR!");
                    }
                    None
                }
                _ => None,
            }
        }
        _ => None,
    }
}

struct NumTraits {
    import: Ident,
    explicit: bool,
}

impl quote::ToTokens for NumTraits {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.import.to_tokens(tokens);
    }
}

impl NumTraits {
    fn new(ast: &syn::DeriveInput) -> Self {
        // If there is a `num_traits` MetaNameValue attribute on the input,
        // retrieve its value, and use it to create an `Ident` to be used
        // to import the `num_traits` crate.
        for attr in &ast.attrs {
            if attr.path().is_ident("num_traits") {
                if let Ok(syn::MetaNameValue {
                    value:
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(ref lit_str),
                            ..
                        }),
                    ..
                }) = attr.meta.require_name_value()
                {
                    return NumTraits {
                        import: syn::Ident::new(&lit_str.value(), lit_str.span()),
                        explicit: true,
                    };
                } else {
                    panic!("#[num_traits] attribute value must be a str");
                }
            }
        }

        // Otherwise, we'll implicitly import our own.
        NumTraits {
            import: Ident::new("_num_traits", Span::call_site()),
            explicit: false,
        }
    }

    fn wrap(&self, output: TokenStream2) -> TokenStream2 {
        if self.explicit {
            output
        } else {
            anon_const_trick(output)
        }
    }
}

/// Derives [`num_traits::FromPrimitive`][from] for simple enums and newtypes.
///
/// [from]: https://docs.rs/num-traits/0.2/num_traits/cast/trait.FromPrimitive.html
///
/// # Examples
///
/// Simple enums can be derived:
///
/// ```rust
/// # #[macro_use]
/// # extern crate num_derive;
///
/// #[derive(FromPrimitive)]
/// enum Color {
///     Red,
///     Blue,
///     Green = 42,
/// }
/// # fn main() {}
/// ```
///
/// Enums that contain data are not allowed:
///
/// ```compile_fail
/// # #[macro_use]
/// # extern crate num_derive;
///
/// #[derive(FromPrimitive)]
/// enum Color {
///     Rgb(u8, u8, u8),
///     Hsv(u8, u8, u8),
/// }
/// # fn main() {}
/// ```
///
/// Structs are not allowed:
///
/// ```compile_fail
/// # #[macro_use]
/// # extern crate num_derive;
/// #[derive(FromPrimitive)]
/// struct Color {
///     r: u8,
///     g: u8,
///     b: u8,
/// }
/// # fn main() {}
/// ```
#[proc_macro_derive(FromPrimitive, attributes(num_traits))]
pub fn from_primitive(input: TokenStream) -> TokenStream {
    let ast = parse!(input as syn::DeriveInput);
    let name = &ast.ident;

    let import = NumTraits::new(&ast);

    let impl_ = if let Some(inner_ty) = newtype_inner(&ast.data) {
        quote! {
            impl #import::FromPrimitive for #name {
                #[inline]
                fn from_i64(n: i64) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_i64(n).map(#name)
                }
                #[inline]
                fn from_u64(n: u64) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_u64(n).map(#name)
                }
                #[inline]
                fn from_isize(n: isize) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_isize(n).map(#name)
                }
                #[inline]
                fn from_i8(n: i8) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_i8(n).map(#name)
                }
                #[inline]
                fn from_i16(n: i16) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_i16(n).map(#name)
                }
                #[inline]
                fn from_i32(n: i32) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_i32(n).map(#name)
                }
                #[inline]
                fn from_i128(n: i128) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_i128(n).map(#name)
                }
                #[inline]
                fn from_usize(n: usize) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_usize(n).map(#name)
                }
                #[inline]
                fn from_u8(n: u8) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_u8(n).map(#name)
                }
                #[inline]
                fn from_u16(n: u16) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_u16(n).map(#name)
                }
                #[inline]
                fn from_u32(n: u32) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_u32(n).map(#name)
                }
                #[inline]
                fn from_u128(n: u128) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_u128(n).map(#name)
                }
                #[inline]
                fn from_f32(n: f32) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_f32(n).map(#name)
                }
                #[inline]
                fn from_f64(n: f64) -> ::core::option::Option<Self> {
                    <#inner_ty as #import::FromPrimitive>::from_f64(n).map(#name)
                }
            }
        }
    } else {
        let variants = match ast.data {
            Data::Enum(ref data_enum) => &data_enum.variants,
            _ => panic!(
                "`FromPrimitive` can be applied only to enums and newtypes, {} is neither",
                name
            ),
        };

        let from_i64_var = quote! { n };
        let clauses: Vec<_> = variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                match variant.fields {
                    Fields::Unit => (),
                    _ => panic!(
                        "`FromPrimitive` can be applied only to unitary enums and newtypes, \
                         {}::{} is either struct or tuple",
                        name, ident
                    ),
                }

                quote! {
                    if #from_i64_var == #name::#ident as i64 {
                        ::core::option::Option::Some(#name::#ident)
                    }
                }
            })
            .collect();

        let from_i64_var = if clauses.is_empty() {
            quote!(_)
        } else {
            from_i64_var
        };

        quote! {
            impl #import::FromPrimitive for #name {
                #[allow(trivial_numeric_casts)]
                #[inline]
                fn from_i64(#from_i64_var: i64) -> ::core::option::Option<Self> {
                    #(#clauses else)* {
                        ::core::option::Option::None
                    }
                }

                #[inline]
                fn from_u64(n: u64) -> ::core::option::Option<Self> {
                    Self::from_i64(n as i64)
                }
            }
        }
    };

    import.wrap(impl_).into()
}

/// Derives [`num_traits::ToPrimitive`][to] for simple enums and newtypes.
///
/// [to]: https://docs.rs/num-traits/0.2/num_traits/cast/trait.ToPrimitive.html
///
/// # Examples
///
/// Simple enums can be derived:
///
/// ```rust
/// # #[macro_use]
/// # extern crate num_derive;
///
/// #[derive(ToPrimitive)]
/// enum Color {
///     Red,
///     Blue,
///     Green = 42,
/// }
/// # fn main() {}
/// ```
///
/// Enums that contain data are not allowed:
///
/// ```compile_fail
/// # #[macro_use]
/// # extern crate num_derive;
///
/// #[derive(ToPrimitive)]
/// enum Color {
///     Rgb(u8, u8, u8),
///     Hsv(u8, u8, u8),
/// }
/// # fn main() {}
/// ```
///
/// Structs are not allowed:
///
/// ```compile_fail
/// # #[macro_use]
/// # extern crate num_derive;
/// #[derive(ToPrimitive)]
/// struct Color {
///     r: u8,
///     g: u8,
///     b: u8,
/// }
/// # fn main() {}
/// ```
#[proc_macro_derive(ToPrimitive, attributes(num_traits))]
pub fn to_primitive(input: TokenStream) -> TokenStream {
    let ast = parse!(input as syn::DeriveInput);
    let name = &ast.ident;

    let import = NumTraits::new(&ast);

    let impl_ = if let Some(inner_ty) = newtype_inner(&ast.data) {
        quote! {
            impl #import::ToPrimitive for #name {
                #[inline]
                fn to_i64(&self) -> ::core::option::Option<i64> {
                    <#inner_ty as #import::ToPrimitive>::to_i64(&self.0)
                }
                #[inline]
                fn to_u64(&self) -> ::core::option::Option<u64> {
                    <#inner_ty as #import::ToPrimitive>::to_u64(&self.0)
                }
                #[inline]
                fn to_isize(&self) -> ::core::option::Option<isize> {
                    <#inner_ty as #import::ToPrimitive>::to_isize(&self.0)
                }
                #[inline]
                fn to_i8(&self) -> ::core::option::Option<i8> {
                    <#inner_ty as #import::ToPrimitive>::to_i8(&self.0)
                }
                #[inline]
                fn to_i16(&self) -> ::core::option::Option<i16> {
                    <#inner_ty as #import::ToPrimitive>::to_i16(&self.0)
                }
                #[inline]
                fn to_i32(&self) -> ::core::option::Option<i32> {
                    <#inner_ty as #import::ToPrimitive>::to_i32(&self.0)
                }
                #[inline]
                fn to_i128(&self) -> ::core::option::Option<i128> {
                    <#inner_ty as #import::ToPrimitive>::to_i128(&self.0)
                }
                #[inline]
                fn to_usize(&self) -> ::core::option::Option<usize> {
                    <#inner_ty as #import::ToPrimitive>::to_usize(&self.0)
                }
                #[inline]
                fn to_u8(&self) -> ::core::option::Option<u8> {
                    <#inner_ty as #import::ToPrimitive>::to_u8(&self.0)
                }
                #[inline]
                fn to_u16(&self) -> ::core::option::Option<u16> {
                    <#inner_ty as #import::ToPrimitive>::to_u16(&self.0)
                }
                #[inline]
                fn to_u32(&self) -> ::core::option::Option<u32> {
                    <#inner_ty as #import::ToPrimitive>::to_u32(&self.0)
                }
                #[inline]
                fn to_u128(&self) -> ::core::option::Option<u128> {
                    <#inner_ty as #import::ToPrimitive>::to_u128(&self.0)
                }
                #[inline]
                fn to_f32(&self) -> ::core::option::Option<f32> {
                    <#inner_ty as #import::ToPrimitive>::to_f32(&self.0)
                }
                #[inline]
                fn to_f64(&self) -> ::core::option::Option<f64> {
                    <#inner_ty as #import::ToPrimitive>::to_f64(&self.0)
                }
            }
        }
    } else {
        let variants = match ast.data {
            Data::Enum(ref data_enum) => &data_enum.variants,
            _ => panic!(
                "`ToPrimitive` can be applied only to enums and newtypes, {} is neither",
                name
            ),
        };

        let variants: Vec<_> = variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                match variant.fields {
                    Fields::Unit => (),
                    _ => {
                        panic!("`ToPrimitive` can be applied only to unitary enums and newtypes, {}::{} is either struct or tuple", name, ident)
                    },
                }

                // NB: We have to check each variant individually, because we'll only have `&self`
                // for the input.  We can't move from that, and it might not be `Clone` or `Copy`.
                // (Otherwise we could just do `*self as i64` without a `match` at all.)
                quote!(#name::#ident => #name::#ident as i64)
            })
            .collect();

        let match_expr = if variants.is_empty() {
            // No variants found, so do not use Some to not to trigger `unreachable_code` lint
            quote! {
                match *self {}
            }
        } else {
            quote! {
                ::core::option::Option::Some(match *self {
                    #(#variants,)*
                })
            }
        };

        quote! {
            impl #import::ToPrimitive for #name {
                #[inline]
                #[allow(trivial_numeric_casts)]
                fn to_i64(&self) -> ::core::option::Option<i64> {
                    #match_expr
                }

                #[inline]
                fn to_u64(&self) -> ::core::option::Option<u64> {
                    self.to_i64().map(|x| x as u64)
                }
            }
        }
    };

    import.wrap(impl_).into()
}

const NEWTYPE_ONLY: &str = "This trait can only be derived for newtypes";

/// Derives [`num_traits::NumOps`][num_ops] for newtypes.  The inner type must already implement
/// `NumOps`.
///
/// [num_ops]: https://docs.rs/num-traits/0.2/num_traits/trait.NumOps.html
///
/// Note that, since `NumOps` is really a trait alias for `Add + Sub + Mul + Div + Rem`, this macro
/// generates impls for _those_ traits.  Furthermore, in all generated impls, `RHS=Self` and
/// `Output=Self`.
#[proc_macro_derive(NumOps)]
pub fn num_ops(input: TokenStream) -> TokenStream {
    let ast = parse!(input as syn::DeriveInput);
    let name = &ast.ident;
    let inner_ty = newtype_inner(&ast.data).expect(NEWTYPE_ONLY);
    let impl_ = quote! {
        impl ::core::ops::Add for #name {
            type Output = Self;
            #[inline]
            fn add(self, other: Self) -> Self {
                #name(<#inner_ty as ::core::ops::Add>::add(self.0, other.0))
            }
        }
        impl ::core::ops::Sub for #name {
            type Output = Self;
            #[inline]
            fn sub(self, other: Self) -> Self {
                #name(<#inner_ty as ::core::ops::Sub>::sub(self.0, other.0))
            }
        }
        impl ::core::ops::Mul for #name {
            type Output = Self;
            #[inline]
            fn mul(self, other: Self) -> Self {
                #name(<#inner_ty as ::core::ops::Mul>::mul(self.0, other.0))
            }
        }
        impl ::core::ops::Div for #name {
            type Output = Self;
            #[inline]
            fn div(self, other: Self) -> Self {
                #name(<#inner_ty as ::core::ops::Div>::div(self.0, other.0))
            }
        }
        impl ::core::ops::Rem for #name {
            type Output = Self;
            #[inline]
            fn rem(self, other: Self) -> Self {
                #name(<#inner_ty as ::core::ops::Rem>::rem(self.0, other.0))
            }
        }
    };
    impl_.into()
}

/// Derives [`num_traits::NumCast`][num_cast] for newtypes.  The inner type must already implement
/// `NumCast`.
///
/// [num_cast]: https://docs.rs/num-traits/0.2/num_traits/cast/trait.NumCast.html
#[proc_macro_derive(NumCast, attributes(num_traits))]
pub fn num_cast(input: TokenStream) -> TokenStream {
    let ast = parse!(input as syn::DeriveInput);
    let name = &ast.ident;
    let inner_ty = newtype_inner(&ast.data).expect(NEWTYPE_ONLY);

    let import = NumTraits::new(&ast);

    let impl_ = quote! {
        impl #import::NumCast for #name {
            #[inline]
            fn from<T: #import::ToPrimitive>(n: T) -> ::core::option::Option<Self> {
                <#inner_ty as #import::NumCast>::from(n).map(#name)
            }
        }
    };

    import.wrap(impl_).into()
}

/// Derives [`num_traits::Zero`][zero] for newtypes.  The inner type must already implement `Zero`.
///
/// [zero]: https://docs.rs/num-traits/0.2/num_traits/identities/trait.Zero.html
#[proc_macro_derive(Zero, attributes(num_traits))]
pub fn zero(input: TokenStream) -> TokenStream {
    let ast = parse!(input as syn::DeriveInput);
    let name = &ast.ident;
    let inner_ty = newtype_inner(&ast.data).expect(NEWTYPE_ONLY);

    let import = NumTraits::new(&ast);

    let impl_ = quote! {
        impl #import::Zero for #name {
            #[inline]
            fn zero() -> Self {
                #name(<#inner_ty as #import::Zero>::zero())
            }
            #[inline]
            fn is_zero(&self) -> bool {
                <#inner_ty as #import::Zero>::is_zero(&self.0)
            }
        }
    };

    import.wrap(impl_).into()
}

/// Derives [`num_traits::One`][one] for newtypes.  The inner type must already implement `One`.
///
/// [one]: https://docs.rs/num-traits/0.2/num_traits/identities/trait.One.html
#[proc_macro_derive(One, attributes(num_traits))]
pub fn one(input: TokenStream) -> TokenStream {
    let ast = parse!(input as syn::DeriveInput);
    let name = &ast.ident;
    let inner_ty = newtype_inner(&ast.data).expect(NEWTYPE_ONLY);

    let import = NumTraits::new(&ast);

    let impl_ = quote! {
        impl #import::One for #name {
            #[inline]
            fn one() -> Self {
                #name(<#inner_ty as #import::One>::one())
            }
            #[inline]
            fn is_one(&self) -> bool {
                <#inner_ty as #import::One>::is_one(&self.0)
            }
        }
    };

    import.wrap(impl_).into()
}

/// Derives [`num_traits::Num`][num] for newtypes.  The inner type must already implement `Num`.
///
/// [num]: https://docs.rs/num-traits/0.2/num_traits/trait.Num.html
#[proc_macro_derive(Num, attributes(num_traits))]
pub fn num(input: TokenStream) -> TokenStream {
    let ast = parse!(input as syn::DeriveInput);
    let name = &ast.ident;
    let inner_ty = newtype_inner(&ast.data).expect(NEWTYPE_ONLY);

    let import = NumTraits::new(&ast);

    let impl_ = quote! {
        impl #import::Num for #name {
            type FromStrRadixErr = <#inner_ty as #import::Num>::FromStrRadixErr;
            #[inline]
            fn from_str_radix(s: &str, radix: u32) -> ::core::result::Result<Self, Self::FromStrRadixErr> {
                <#inner_ty as #import::Num>::from_str_radix(s, radix).map(#name)
            }
        }
    };

    import.wrap(impl_).into()
}

/// Derives [`num_traits::Float`][float] for newtypes.  The inner type must already implement
/// `Float`.
///
/// [float]: https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html
#[proc_macro_derive(Float, attributes(num_traits))]
pub fn float(input: TokenStream) -> TokenStream {
    let ast = parse!(input as syn::DeriveInput);
    let name = &ast.ident;
    let inner_ty = newtype_inner(&ast.data).expect(NEWTYPE_ONLY);

    let import = NumTraits::new(&ast);

    let impl_ = quote! {
        impl #import::Float for #name {
            #[inline]
            fn nan() -> Self {
                #name(<#inner_ty as #import::Float>::nan())
            }
            #[inline]
            fn infinity() -> Self {
                #name(<#inner_ty as #import::Float>::infinity())
            }
            #[inline]
            fn neg_infinity() -> Self {
                #name(<#inner_ty as #import::Float>::neg_infinity())
            }
            #[inline]
            fn neg_zero() -> Self {
                #name(<#inner_ty as #import::Float>::neg_zero())
            }
            #[inline]
            fn min_value() -> Self {
                #name(<#inner_ty as #import::Float>::min_value())
            }
            #[inline]
            fn min_positive_value() -> Self {
                #name(<#inner_ty as #import::Float>::min_positive_value())
            }
            #[inline]
            fn max_value() -> Self {
                #name(<#inner_ty as #import::Float>::max_value())
            }
            #[inline]
            fn is_nan(self) -> bool {
                <#inner_ty as #import::Float>::is_nan(self.0)
            }
            #[inline]
            fn is_infinite(self) -> bool {
                <#inner_ty as #import::Float>::is_infinite(self.0)
            }
            #[inline]
            fn is_finite(self) -> bool {
                <#inner_ty as #import::Float>::is_finite(self.0)
            }
            #[inline]
            fn is_normal(self) -> bool {
                <#inner_ty as #import::Float>::is_normal(self.0)
            }
            #[inline]
            fn classify(self) -> ::core::num::FpCategory {
                <#inner_ty as #import::Float>::classify(self.0)
            }
            #[inline]
            fn floor(self) -> Self {
                #name(<#inner_ty as #import::Float>::floor(self.0))
            }
            #[inline]
            fn ceil(self) -> Self {
                #name(<#inner_ty as #import::Float>::ceil(self.0))
            }
            #[inline]
            fn round(self) -> Self {
                #name(<#inner_ty as #import::Float>::round(self.0))
            }
            #[inline]
            fn trunc(self) -> Self {
                #name(<#inner_ty as #import::Float>::trunc(self.0))
            }
            #[inline]
            fn fract(self) -> Self {
                #name(<#inner_ty as #import::Float>::fract(self.0))
            }
            #[inline]
            fn abs(self) -> Self {
                #name(<#inner_ty as #import::Float>::abs(self.0))
            }
            #[inline]
            fn signum(self) -> Self {
                #name(<#inner_ty as #import::Float>::signum(self.0))
            }
            #[inline]
            fn is_sign_positive(self) -> bool {
                <#inner_ty as #import::Float>::is_sign_positive(self.0)
            }
            #[inline]
            fn is_sign_negative(self) -> bool {
                <#inner_ty as #import::Float>::is_sign_negative(self.0)
            }
            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self {
                #name(<#inner_ty as #import::Float>::mul_add(self.0, a.0, b.0))
            }
            #[inline]
            fn recip(self) -> Self {
                #name(<#inner_ty as #import::Float>::recip(self.0))
            }
            #[inline]
            fn powi(self, n: i32) -> Self {
                #name(<#inner_ty as #import::Float>::powi(self.0, n))
            }
            #[inline]
            fn powf(self, n: Self) -> Self {
                #name(<#inner_ty as #import::Float>::powf(self.0, n.0))
            }
            #[inline]
            fn sqrt(self) -> Self {
                #name(<#inner_ty as #import::Float>::sqrt(self.0))
            }
            #[inline]
            fn exp(self) -> Self {
                #name(<#inner_ty as #import::Float>::exp(self.0))
            }
            #[inline]
            fn exp2(self) -> Self {
                #name(<#inner_ty as #import::Float>::exp2(self.0))
            }
            #[inline]
            fn ln(self) -> Self {
                #name(<#inner_ty as #import::Float>::ln(self.0))
            }
            #[inline]
            fn log(self, base: Self) -> Self {
                #name(<#inner_ty as #import::Float>::log(self.0, base.0))
            }
            #[inline]
            fn log2(self) -> Self {
                #name(<#inner_ty as #import::Float>::log2(self.0))
            }
            #[inline]
            fn log10(self) -> Self {
                #name(<#inner_ty as #import::Float>::log10(self.0))
            }
            #[inline]
            fn max(self, other: Self) -> Self {
                #name(<#inner_ty as #import::Float>::max(self.0, other.0))
            }
            #[inline]
            fn min(self, other: Self) -> Self {
                #name(<#inner_ty as #import::Float>::min(self.0, other.0))
            }
            #[inline]
            fn abs_sub(self, other: Self) -> Self {
                #name(<#inner_ty as #import::Float>::abs_sub(self.0, other.0))
            }
            #[inline]
            fn cbrt(self) -> Self {
                #name(<#inner_ty as #import::Float>::cbrt(self.0))
            }
            #[inline]
            fn hypot(self, other: Self) -> Self {
                #name(<#inner_ty as #import::Float>::hypot(self.0, other.0))
            }
            #[inline]
            fn sin(self) -> Self {
                #name(<#inner_ty as #import::Float>::sin(self.0))
            }
            #[inline]
            fn cos(self) -> Self {
                #name(<#inner_ty as #import::Float>::cos(self.0))
            }
            #[inline]
            fn tan(self) -> Self {
                #name(<#inner_ty as #import::Float>::tan(self.0))
            }
            #[inline]
            fn asin(self) -> Self {
                #name(<#inner_ty as #import::Float>::asin(self.0))
            }
            #[inline]
            fn acos(self) -> Self {
                #name(<#inner_ty as #import::Float>::acos(self.0))
            }
            #[inline]
            fn atan(self) -> Self {
                #name(<#inner_ty as #import::Float>::atan(self.0))
            }
            #[inline]
            fn atan2(self, other: Self) -> Self {
                #name(<#inner_ty as #import::Float>::atan2(self.0, other.0))
            }
            #[inline]
            fn sin_cos(self) -> (Self, Self) {
                let (x, y) = <#inner_ty as #import::Float>::sin_cos(self.0);
                (#name(x), #name(y))
            }
            #[inline]
            fn exp_m1(self) -> Self {
                #name(<#inner_ty as #import::Float>::exp_m1(self.0))
            }
            #[inline]
            fn ln_1p(self) -> Self {
                #name(<#inner_ty as #import::Float>::ln_1p(self.0))
            }
            #[inline]
            fn sinh(self) -> Self {
                #name(<#inner_ty as #import::Float>::sinh(self.0))
            }
            #[inline]
            fn cosh(self) -> Self {
                #name(<#inner_ty as #import::Float>::cosh(self.0))
            }
            #[inline]
            fn tanh(self) -> Self {
                #name(<#inner_ty as #import::Float>::tanh(self.0))
            }
            #[inline]
            fn asinh(self) -> Self {
                #name(<#inner_ty as #import::Float>::asinh(self.0))
            }
            #[inline]
            fn acosh(self) -> Self {
                #name(<#inner_ty as #import::Float>::acosh(self.0))
            }
            #[inline]
            fn atanh(self) -> Self {
                #name(<#inner_ty as #import::Float>::atanh(self.0))
            }
            #[inline]
            fn integer_decode(self) -> (u64, i16, i8) {
                <#inner_ty as #import::Float>::integer_decode(self.0)
            }
            #[inline]
            fn epsilon() -> Self {
                #name(<#inner_ty as #import::Float>::epsilon())
            }
            #[inline]
            fn to_degrees(self) -> Self {
                #name(<#inner_ty as #import::Float>::to_degrees(self.0))
            }
            #[inline]
            fn to_radians(self) -> Self {
                #name(<#inner_ty as #import::Float>::to_radians(self.0))
            }
        }
    };

    import.wrap(impl_).into()
}

/// Derives [`num_traits::Signed`][signed] for newtypes.  The inner type must already implement
/// `Signed`.
///
/// [signed]: https://docs.rs/num-traits/0.2/num_traits/sign/trait.Signed.html
#[proc_macro_derive(Signed, attributes(num_traits))]
pub fn signed(input: TokenStream) -> TokenStream {
    let ast = parse!(input as syn::DeriveInput);
    let name = &ast.ident;
    let inner_ty = newtype_inner(&ast.data).expect(NEWTYPE_ONLY);

    let import = NumTraits::new(&ast);

    let impl_ = quote! {
        impl #import::Signed for #name {
            #[inline]
            fn abs(&self) -> Self {
                #name(<#inner_ty as #import::Signed>::abs(&self.0))
            }
            #[inline]
            fn abs_sub(&self, other: &Self) -> Self {
                #name(<#inner_ty as #import::Signed>::abs_sub(&self.0, &other.0))
            }
            #[inline]
            fn signum(&self) -> Self {
                #name(<#inner_ty as #import::Signed>::signum(&self.0))
            }
            #[inline]
            fn is_positive(&self) -> bool {
                <#inner_ty as #import::Signed>::is_positive(&self.0)
            }
            #[inline]
            fn is_negative(&self) -> bool {
                <#inner_ty as #import::Signed>::is_negative(&self.0)
            }
        }
    };

    import.wrap(impl_).into()
}

/// Derives [`num_traits::Unsigned`][unsigned].  The inner type must already implement
/// `Unsigned`.
///
/// [unsigned]: https://docs.rs/num/latest/num/traits/trait.Unsigned.html
#[proc_macro_derive(Unsigned, attributes(num_traits))]
pub fn unsigned(input: TokenStream) -> TokenStream {
    let ast = parse!(input as syn::DeriveInput);
    let name = &ast.ident;

    let import = NumTraits::new(&ast);

    let impl_ = quote! {
        impl #import::Unsigned for #name {}
    };

    import.wrap(impl_).into()
}
