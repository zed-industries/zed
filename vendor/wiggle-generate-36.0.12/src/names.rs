use escaping::{NamingConvention, escape_id, handle_2big_enum_variant};
use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use witx::{BuiltinType, Id, Type, TypeRef, WasmType};

use crate::UserErrorType;

pub fn type_(id: &Id) -> Ident {
    escape_id(id, NamingConvention::CamelCase)
}

pub fn builtin_type(b: BuiltinType) -> TokenStream {
    match b {
        BuiltinType::U8 { .. } => quote!(u8),
        BuiltinType::U16 => quote!(u16),
        BuiltinType::U32 { .. } => quote!(u32),
        BuiltinType::U64 => quote!(u64),
        BuiltinType::S8 => quote!(i8),
        BuiltinType::S16 => quote!(i16),
        BuiltinType::S32 => quote!(i32),
        BuiltinType::S64 => quote!(i64),
        BuiltinType::F32 => quote!(f32),
        BuiltinType::F64 => quote!(f64),
        BuiltinType::Char => quote!(char),
    }
}

pub fn wasm_type(ty: WasmType) -> TokenStream {
    match ty {
        WasmType::I32 => quote!(i32),
        WasmType::I64 => quote!(i64),
        WasmType::F32 => quote!(f32),
        WasmType::F64 => quote!(f64),
    }
}

pub fn type_ref(tref: &TypeRef, lifetime: TokenStream) -> TokenStream {
    match tref {
        TypeRef::Name(nt) => {
            let ident = type_(&nt.name);
            quote!(#ident)
        }
        TypeRef::Value(ty) => match &**ty {
            Type::Builtin(builtin) => builtin_type(*builtin),
            Type::Pointer(pointee) | Type::ConstPointer(pointee) => {
                let pointee_type = type_ref(&pointee, lifetime.clone());
                quote!(wiggle::GuestPtr<#pointee_type>)
            }
            Type::List(pointee) => match &**pointee.type_() {
                Type::Builtin(BuiltinType::Char) => {
                    quote!(wiggle::GuestPtr<str>)
                }
                _ => {
                    let pointee_type = type_ref(&pointee, lifetime.clone());
                    quote!(wiggle::GuestPtr<[#pointee_type]>)
                }
            },
            Type::Variant(v) => match v.as_expected() {
                Some((ok, err)) => {
                    let ok = match ok {
                        Some(ty) => type_ref(ty, lifetime.clone()),
                        None => quote!(()),
                    };
                    let err = match err {
                        Some(ty) => type_ref(ty, lifetime.clone()),
                        None => quote!(()),
                    };
                    quote!(Result<#ok, #err>)
                }
                None => unimplemented!("anonymous variant ref {:?}", tref),
            },
            Type::Record(r) if r.is_tuple() => {
                let types = r
                    .members
                    .iter()
                    .map(|m| type_ref(&m.tref, lifetime.clone()))
                    .collect::<Vec<_>>();
                quote!((#(#types,)*))
            }
            _ => unimplemented!("anonymous type ref {:?}", tref),
        },
    }
}

/// Convert an enum variant from its [`Id`][witx] name to its Rust [`Ident`][id] representation.
///
/// [id]: https://docs.rs/proc-macro2/*/proc_macro2/struct.Ident.html
/// [witx]: https://docs.rs/witx/*/witx/struct.Id.html
pub fn enum_variant(id: &Id) -> Ident {
    handle_2big_enum_variant(id).unwrap_or_else(|| escape_id(id, NamingConvention::CamelCase))
}

pub fn flag_member(id: &Id) -> Ident {
    format_ident!("{}", id.as_str().to_shouty_snake_case())
}

pub fn int_member(id: &Id) -> Ident {
    format_ident!("{}", id.as_str().to_shouty_snake_case())
}

/// Convert a struct member from its [`Id`][witx] name to its Rust [`Ident`][id] representation.
///
/// [id]: https://docs.rs/proc-macro2/*/proc_macro2/struct.Ident.html
/// [witx]: https://docs.rs/witx/*/witx/struct.Id.html
pub fn struct_member(id: &Id) -> Ident {
    escape_id(id, NamingConvention::SnakeCase)
}

/// Convert a module name from its [`Id`][witx] name to its Rust [`Ident`][id] representation.
///
/// [id]: https://docs.rs/proc-macro2/*/proc_macro2/struct.Ident.html
/// [witx]: https://docs.rs/witx/*/witx/struct.Id.html
pub fn module(id: &Id) -> Ident {
    escape_id(id, NamingConvention::SnakeCase)
}

/// Convert a trait name from its [`Id`][witx] name to its Rust [`Ident`][id] representation.
///
/// [id]: https://docs.rs/proc-macro2/*/proc_macro2/struct.Ident.html
/// [witx]: https://docs.rs/witx/*/witx/struct.Id.html
pub fn trait_name(id: &Id) -> Ident {
    escape_id(id, NamingConvention::CamelCase)
}

/// Convert a function name from its [`Id`][witx] name to its Rust [`Ident`][id] representation.
///
/// [id]: https://docs.rs/proc-macro2/*/proc_macro2/struct.Ident.html
/// [witx]: https://docs.rs/witx/*/witx/struct.Id.html
pub fn func(id: &Id) -> Ident {
    escape_id(id, NamingConvention::SnakeCase)
}

/// Convert a parameter name from its [`Id`][witx] name to its Rust [`Ident`][id] representation.
///
/// [id]: https://docs.rs/proc-macro2/*/proc_macro2/struct.Ident.html
/// [witx]: https://docs.rs/witx/*/witx/struct.Id.html
pub fn func_param(id: &Id) -> Ident {
    escape_id(id, NamingConvention::SnakeCase)
}

/// For when you need a {name}_ptr binding for passing a value by reference:
pub fn func_ptr_binding(id: &Id) -> Ident {
    format_ident!("{}_ptr", id.as_str().to_snake_case())
}

/// For when you need a {name}_len binding for passing an array:
pub fn func_len_binding(id: &Id) -> Ident {
    format_ident!("{}_len", id.as_str().to_snake_case())
}

fn builtin_name(b: &BuiltinType) -> &'static str {
    match b {
        BuiltinType::U8 { .. } => "u8",
        BuiltinType::U16 => "u16",
        BuiltinType::U32 { .. } => "u32",
        BuiltinType::U64 => "u64",
        BuiltinType::S8 => "i8",
        BuiltinType::S16 => "i16",
        BuiltinType::S32 => "i32",
        BuiltinType::S64 => "i64",
        BuiltinType::F32 => "f32",
        BuiltinType::F64 => "f64",
        BuiltinType::Char => "char",
    }
}

fn snake_typename(tref: &TypeRef) -> String {
    match tref {
        TypeRef::Name(nt) => nt.name.as_str().to_snake_case(),
        TypeRef::Value(ty) => match &**ty {
            Type::Builtin(b) => builtin_name(&b).to_owned(),
            _ => panic!("unexpected anonymous type: {ty:?}"),
        },
    }
}

pub fn user_error_conversion_method(user_type: &UserErrorType) -> Ident {
    let abi_type = snake_typename(&user_type.abi_type());
    format_ident!(
        "{}_from_{}",
        abi_type,
        user_type.method_fragment().to_snake_case()
    )
}

/// Identifier escaping utilities.
///
/// This module most importantly exports an `escape_id` function that can be used to properly
/// escape tokens that conflict with strict and reserved keywords, as of Rust's 2018 edition.
///
/// Weak keywords are not included as their semantic rules do not have the same implications as
/// those of strict and reserved keywords. `union` for example, is permitted as the name of a
/// variable. `dyn` was promoted to a strict keyword beginning in the 2018 edition.
mod escaping {
    use {
        heck::{ToSnakeCase, ToUpperCamelCase},
        proc_macro2::Ident,
        quote::format_ident,
        witx::Id,
    };

    /// Identifier naming convention.
    ///
    /// Because shouty snake case values (identifiers that look `LIKE_THIS`) cannot potentially
    /// conflict with any Rust keywords, this enum only include snake and camel case variants.
    pub enum NamingConvention {
        /// Snake case. Used to denote values `LikeThis`.
        CamelCase,
        /// Snake case. Used to denote values `like_this`.
        SnakeCase,
    }

    /// Given a witx [`Id`][witx] and a [`NamingConvention`][naming], return a [`Ident`] word of
    /// Rust syntax that accounts for escaping both strict and reserved keywords. If an identifier
    /// would have conflicted with a keyword, a trailing underscode will be appended.
    ///
    /// [id]: https://docs.rs/proc-macro2/*/proc_macro2/struct.Ident.html
    /// [naming]: enum.NamingConvention.html
    /// [witx]: https://docs.rs/witx/*/witx/struct.Id.html
    pub fn escape_id(id: &Id, conv: NamingConvention) -> Ident {
        use NamingConvention::{CamelCase, SnakeCase};
        match (conv, id.as_str()) {
            // For camel-cased identifiers, `Self` is the only potential keyword conflict.
            (CamelCase, "self") => format_ident!("Self_"),
            (CamelCase, s) => format_ident!("{}", s.to_upper_camel_case()),
            // Snake-cased identifiers are where the bulk of conflicts can occur.
            (SnakeCase, s) => {
                let s = s.to_snake_case();
                if STRICT.iter().chain(RESERVED).any(|k| *k == s) {
                    // If the camel-cased string matched any strict or reserved keywords, then
                    // append a trailing underscore to the identifier we generate.
                    format_ident!("{}_", s)
                } else {
                    format_ident!("{}", s) // Otherwise, use the string as is.
                }
            }
        }
    }

    /// Strict keywords.
    ///
    /// >  Strict keywords cannot be used as the names of:
    /// >    * Items
    /// >    * Variables and function parameters
    /// >    * Fields and variants
    /// >    * Type parameters
    /// >    * Lifetime parameters or loop labels
    /// >    * Macros or attributes
    /// >    * Macro placeholders
    /// >    * Crates
    /// >
    /// > - <cite>[The Rust Reference][ref]</cite>
    ///
    /// This list also includes keywords that were introduced in the 2018 edition of Rust.
    ///
    /// [ref]: https://doc.rust-lang.org/reference/keywords.html#strict-keywords
    const STRICT: &[&str] = &[
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
        "true", "type", "unsafe", "use", "where", "while",
    ];

    /// Reserved keywords.
    ///
    /// > These keywords aren't used yet, but they are reserved for future use. They have the same
    /// > restrictions as strict keywords. The reasoning behind this is to make current programs
    /// > forward compatible with future versions of Rust by forbidding them to use these keywords.
    /// >
    /// > - <cite>[The Rust Reference][ref]</cite>
    ///
    /// This list also includes keywords that were introduced in the 2018 edition of Rust.
    ///
    /// [ref]: https://doc.rust-lang.org/reference/keywords.html#reserved-keywords
    const RESERVED: &[&str] = &[
        "abstract", "become", "box", "do", "final", "macro", "override", "priv", "try", "typeof",
        "unsized", "virtual", "yield",
    ];

    /// Handle WASI's [`errno::2big`][err] variant.
    ///
    /// This is an unfortunate edge case that must account for when generating `enum` variants.
    /// This will only return `Some(_)` if the given witx identifier *is* `2big`, otherwise this
    /// function will return `None`.
    ///
    /// This functionality is a short-term fix that keeps WASI working. Instead of expanding these sort of special cases,
    /// we should replace this function by having the user provide a mapping of witx identifiers to Rust identifiers in the
    /// arguments to the macro.
    ///
    /// [err]: https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#-errno-enumu16
    pub fn handle_2big_enum_variant(id: &Id) -> Option<Ident> {
        if id.as_str() == "2big" {
            Some(format_ident!("TooBig"))
        } else {
            None
        }
    }
}
