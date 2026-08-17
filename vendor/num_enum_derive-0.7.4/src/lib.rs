// Not supported by MSRV
#![allow(clippy::uninlined_format_args)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Expr, Ident};

mod enum_attributes;
mod parsing;
use parsing::{get_crate_name, EnumInfo};
mod utils;
mod variant_attributes;

/// Implements `Into<Primitive>` for a `#[repr(Primitive)] enum`.
///
/// (It actually implements `From<Enum> for Primitive`)
///
/// ## Allows turning an enum into a primitive.
///
/// ```rust
/// use num_enum::IntoPrimitive;
///
/// #[derive(IntoPrimitive)]
/// #[repr(u8)]
/// enum Number {
///     Zero,
///     One,
/// }
///
/// let zero: u8 = Number::Zero.into();
/// assert_eq!(zero, 0u8);
/// ```
#[proc_macro_derive(IntoPrimitive, attributes(num_enum, catch_all))]
pub fn derive_into_primitive(input: TokenStream) -> TokenStream {
    let enum_info = parse_macro_input!(input as EnumInfo);
    let catch_all = enum_info.catch_all();
    let name = &enum_info.name;
    let repr = &enum_info.repr;

    let body = if let Some(catch_all_ident) = catch_all {
        quote! {
            match enum_value {
                #name::#catch_all_ident(raw) => raw,
                rest => unsafe { *(&rest as *const #name as *const Self) }
            }
        }
    } else {
        quote! { enum_value as Self }
    };

    TokenStream::from(quote! {
        impl From<#name> for #repr {
            #[inline]
            fn from (enum_value: #name) -> Self
            {
                #body
            }
        }
    })
}

/// Implements `From<Primitive>` for a `#[repr(Primitive)] enum`.
///
/// Turning a primitive into an enum with `from`.
/// ----------------------------------------------
///
/// ```rust
/// use num_enum::FromPrimitive;
///
/// #[derive(Debug, Eq, PartialEq, FromPrimitive)]
/// #[repr(u8)]
/// enum Number {
///     Zero,
///     #[num_enum(default)]
///     NonZero,
/// }
///
/// let zero = Number::from(0u8);
/// assert_eq!(zero, Number::Zero);
///
/// let one = Number::from(1u8);
/// assert_eq!(one, Number::NonZero);
///
/// let two = Number::from(2u8);
/// assert_eq!(two, Number::NonZero);
/// ```
#[proc_macro_derive(FromPrimitive, attributes(num_enum, default, catch_all))]
pub fn derive_from_primitive(input: TokenStream) -> TokenStream {
    let enum_info: EnumInfo = parse_macro_input!(input);
    let krate = Ident::new(&get_crate_name(), Span::call_site());

    let is_naturally_exhaustive = enum_info.is_naturally_exhaustive();
    let catch_all_body = match is_naturally_exhaustive {
        Ok(is_naturally_exhaustive) => {
            if is_naturally_exhaustive {
                quote! { unreachable!("exhaustive enum") }
            } else if let Some(default_ident) = enum_info.default() {
                quote! { Self::#default_ident }
            } else if let Some(catch_all_ident) = enum_info.catch_all() {
                quote! { Self::#catch_all_ident(number) }
            } else {
                let span = Span::call_site();
                let message =
                    "#[derive(num_enum::FromPrimitive)] requires enum to be exhaustive, or a variant marked with `#[default]`, `#[num_enum(default)]`, or `#[num_enum(catch_all)`";
                return syn::Error::new(span, message).to_compile_error().into();
            }
        }
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let EnumInfo {
        ref name, ref repr, ..
    } = enum_info;

    let variant_idents: Vec<Ident> = enum_info.variant_idents();
    let expression_idents: Vec<Vec<Ident>> = enum_info.expression_idents();
    let variant_expressions: Vec<Vec<Expr>> = enum_info.variant_expressions();

    debug_assert_eq!(variant_idents.len(), variant_expressions.len());

    TokenStream::from(quote! {
        impl ::#krate::FromPrimitive for #name {
            type Primitive = #repr;

            fn from_primitive(number: Self::Primitive) -> Self {
                // Use intermediate const(s) so that enums defined like
                // `Two = ONE + 1u8` work properly.
                #![allow(non_upper_case_globals)]
                #(
                    #(
                        const #expression_idents: #repr = #variant_expressions;
                    )*
                )*
                #[deny(unreachable_patterns)]
                match number {
                    #(
                        #( #expression_idents )|*
                        => Self::#variant_idents,
                    )*
                    #[allow(unreachable_patterns)]
                    _ => #catch_all_body,
                }
            }
        }

        impl ::core::convert::From<#repr> for #name {
            #[inline]
            fn from (
                number: #repr,
            ) -> Self {
                ::#krate::FromPrimitive::from_primitive(number)
            }
        }

        #[doc(hidden)]
        impl ::#krate::CannotDeriveBothFromPrimitiveAndTryFromPrimitive for #name {}
    })
}

/// Implements `TryFrom<Primitive>` for a `#[repr(Primitive)] enum`.
///
/// Attempting to turn a primitive into an enum with `try_from`.
/// ----------------------------------------------
///
/// ```rust
/// use num_enum::TryFromPrimitive;
/// use std::convert::TryFrom;
///
/// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
/// #[repr(u8)]
/// enum Number {
///     Zero,
///     One,
/// }
///
/// let zero = Number::try_from(0u8);
/// assert_eq!(zero, Ok(Number::Zero));
///
/// let three = Number::try_from(3u8);
/// assert_eq!(
///     three.unwrap_err().to_string(),
///     "No discriminant in enum `Number` matches the value `3`",
/// );
/// ```
#[proc_macro_derive(TryFromPrimitive, attributes(num_enum))]
pub fn derive_try_from_primitive(input: TokenStream) -> TokenStream {
    let enum_info: EnumInfo = parse_macro_input!(input);
    let krate = Ident::new(&get_crate_name(), Span::call_site());

    let EnumInfo {
        ref name,
        ref repr,
        ref error_type_info,
        ..
    } = enum_info;

    let variant_idents: Vec<Ident> = enum_info.variant_idents();
    let expression_idents: Vec<Vec<Ident>> = enum_info.expression_idents();
    let variant_expressions: Vec<Vec<Expr>> = enum_info.variant_expressions();

    debug_assert_eq!(variant_idents.len(), variant_expressions.len());

    let error_type = &error_type_info.name;
    let error_constructor = &error_type_info.constructor;

    TokenStream::from(quote! {
        impl ::#krate::TryFromPrimitive for #name {
            type Primitive = #repr;
            type Error = #error_type;

            const NAME: &'static str = stringify!(#name);

            fn try_from_primitive (
                number: Self::Primitive,
            ) -> ::core::result::Result<
                Self,
                #error_type
            > {
                // Use intermediate const(s) so that enums defined like
                // `Two = ONE + 1u8` work properly.
                #![allow(non_upper_case_globals)]
                #(
                    #(
                        const #expression_idents: #repr = #variant_expressions;
                    )*
                )*
                #[deny(unreachable_patterns)]
                match number {
                    #(
                        #( #expression_idents )|*
                        => ::core::result::Result::Ok(Self::#variant_idents),
                    )*
                    #[allow(unreachable_patterns)]
                    _ => ::core::result::Result::Err(
                        #error_constructor ( number )
                    ),
                }
            }
        }

        impl ::core::convert::TryFrom<#repr> for #name {
            type Error = #error_type;

            #[inline]
            fn try_from (
                number: #repr,
            ) -> ::core::result::Result<Self, #error_type>
            {
                ::#krate::TryFromPrimitive::try_from_primitive(number)
            }
        }

        #[doc(hidden)]
        impl ::#krate::CannotDeriveBothFromPrimitiveAndTryFromPrimitive for #name {}
    })
}

/// Generates a `unsafe fn unchecked_transmute_from(number: Primitive) -> Self`
/// associated function.
///
/// Allows unsafely turning a primitive into an enum with unchecked_transmute_from
/// ------------------------------------------------------------------------------
///
/// If you're really certain a conversion will succeed, and want to avoid a small amount of overhead, you can use unsafe
/// code to do this conversion. Unless you have data showing that the match statement generated in the `try_from` above is a
/// bottleneck for you, you should avoid doing this, as the unsafe code has potential to cause serious memory issues in
/// your program.
///
/// Note that this derive ignores any `default`, `catch_all`, and `alternatives` attributes on the enum.
/// If you need support for conversions from these values, you should use `TryFromPrimitive` or `FromPrimitive`.
///
/// ```rust
/// use num_enum::UnsafeFromPrimitive;
///
/// #[derive(Debug, Eq, PartialEq, UnsafeFromPrimitive)]
/// #[repr(u8)]
/// enum Number {
///     Zero,
///     One,
/// }
///
/// fn main() {
///     assert_eq!(
///         Number::Zero,
///         unsafe { Number::unchecked_transmute_from(0_u8) },
///     );
///     assert_eq!(
///         Number::One,
///         unsafe { Number::unchecked_transmute_from(1_u8) },
///     );
/// }
///
/// unsafe fn undefined_behavior() {
///     let _ = Number::unchecked_transmute_from(2); // 2 is not a valid discriminant!
/// }
/// ```
#[proc_macro_derive(UnsafeFromPrimitive, attributes(num_enum))]
pub fn derive_unsafe_from_primitive(stream: TokenStream) -> TokenStream {
    let enum_info = parse_macro_input!(stream as EnumInfo);
    let krate = Ident::new(&get_crate_name(), Span::call_site());

    let EnumInfo {
        ref name, ref repr, ..
    } = enum_info;

    TokenStream::from(quote! {
        impl ::#krate::UnsafeFromPrimitive for #name {
            type Primitive = #repr;

            unsafe fn unchecked_transmute_from(number: Self::Primitive) -> Self {
                ::core::mem::transmute(number)
            }
        }
    })
}

/// Implements `core::default::Default` for a `#[repr(Primitive)] enum`.
///
/// Whichever variant has the `#[default]` or `#[num_enum(default)]` attribute will be returned.
/// ----------------------------------------------
///
/// ```rust
/// #[derive(Debug, Eq, PartialEq, num_enum::Default)]
/// #[repr(u8)]
/// enum Number {
///     Zero,
///     #[default]
///     One,
/// }
///
/// assert_eq!(Number::One, Number::default());
/// assert_eq!(Number::One, <Number as ::core::default::Default>::default());
/// ```
#[proc_macro_derive(Default, attributes(num_enum, default))]
pub fn derive_default(stream: TokenStream) -> TokenStream {
    let enum_info = parse_macro_input!(stream as EnumInfo);

    let default_ident = match enum_info.default() {
        Some(ident) => ident,
        None => {
            let span = Span::call_site();
            let message =
                "#[derive(num_enum::Default)] requires enum to be exhaustive, or a variant marked with `#[default]` or `#[num_enum(default)]`";
            return syn::Error::new(span, message).to_compile_error().into();
        }
    };

    let EnumInfo { ref name, .. } = enum_info;

    TokenStream::from(quote! {
        impl ::core::default::Default for #name {
            #[inline]
            fn default() -> Self {
                Self::#default_ident
            }
        }
    })
}
