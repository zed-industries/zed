//! This crate provides a macro for automatically generating setter methods for a struct's fields.
//! This can be used to add setters to a plain data struct, or to help in implementing builders.
//!
//! For detailed usage, see [`#[derive(Setters)]`](macro@Setters)
//!
//! # Example
//!
//! ```rust
//! use derive_setters::*;
//!
//! #[derive(Default, Setters, Debug, PartialEq, Eq)]
//! struct BasicStruct {
//!     #[setters(rename = "test")]
//!     a: u32,
//!     b: u32,
//!     c: u32,
//! }
//!
//! assert_eq!(
//!     BasicStruct::default().test(30).b(10).c(20),
//!     BasicStruct { a: 30, b: 10, c: 20 },
//! );
//! ```

#![cfg_attr(feature = "nightly", feature(proc_macro_diagnostic))]

extern crate proc_macro;

use darling::util::Flag;
use darling::*;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as SynTokenStream};
use quote::*;
use std::result::Result;
use syn::spanned::Spanned;
use syn::*;

#[cfg(feature = "nightly")]
fn error(span: Span, data: &str) -> SynTokenStream {
    span.unstable().error(data).emit();
    SynTokenStream::new()
}

#[cfg(not(feature = "nightly"))]
fn error(_: Span, data: &str) -> SynTokenStream {
    quote! { compile_error!(#data); }
}

fn parse_generics(input: String) -> Result<Generics, darling::Error> {
    parse_str(&input).map_err(|parse_err| {
        darling::Error::custom(format!("Could not parse provided generics: {}", parse_err))
    })
}

#[derive(Debug, Clone, FromMeta)]
struct ExternalDelegate {
    /// The type to generate a delegate for.
    ty: Path,
    /// The generics for the type to generate a delegate for, if any.
    #[darling(and_then = parse_generics, default)]
    generics: Generics,
    /// The field to delegate the methods to.
    #[darling(default)]
    field: Option<Ident>,
    /// The method to delegate the methods to.
    #[darling(default)]
    method: Option<Ident>,
    /// A prefix for the delegated setter methods.
    #[darling(default)]
    prefix: Option<String>,
}

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(setters), supports(struct_named))]
struct ContainerAttrs {
    ident: Ident,
    generics: Generics,

    /// Use the core library rather than the std library.
    #[darling(default)]
    no_std: Flag,

    /// Whether to accept any field that converts via `Into` by default.
    #[darling(default)]
    into: bool,

    /// Whether to strip `Option<T>` into `T` in the parameters for the setter method by default.
    #[darling(default)]
    strip_option: bool,

    /// Whether to borrow or take ownership of `self` in the setter method by default.
    #[darling(default)]
    borrow_self: bool,

    /// Whether to generate a method that sets a boolean by default.
    #[darling(default)]
    bool: bool,

    /// Whether to generate setters for this struct's fields by default. If set to false,
    /// `generate_public` and `generate_private` are ignored.
    #[darling(default)]
    generate: Option<bool>,

    /// Whether to generate setters for this struct's public fields by default.
    #[darling(default)]
    generate_public: Option<bool>,

    /// Whether to generate setters for this struct's private fields by default.
    #[darling(default)]
    generate_private: Option<bool>,

    /// A prefix for the generated setter methods.
    #[darling(default)]
    prefix: Option<String>,

    /// Other types to generate delegates to this type for.
    #[darling(multiple)]
    generate_delegates: Vec<ExternalDelegate>,
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(setters), forward_attrs(doc))]
struct FieldAttrs {
    attrs: Vec<Attribute>,

    /// The name of the generated setter method.
    #[darling(default)]
    rename: Option<Ident>,

    /// Whether to accept any field that converts via `Into` to this field's type.
    #[darling(default)]
    into: Option<bool>,

    /// Whether to strip `Option<T>` into `T` in the parameters for the setter method.
    #[darling(default)]
    strip_option: Option<bool>,

    /// Whether to borrow or take ownership of `self` for the setter method.
    #[darling(default)]
    borrow_self: Option<bool>,

    /// Whether to generate a method that sets a boolean.
    #[darling(default)]
    bool: Option<bool>,

    /// Whether to generate a setter for this field regardless of global settings.
    #[darling(default)]
    generate: bool,

    /// Whether to skip this field regardless of global settings. Overwrites `generate`.
    #[darling(default)]
    skip: bool,

    /// The documentation used for this field's setter.
    #[darling(default)]
    doc: Option<String>,
}

struct ContainerDef {
    ty: Type,
    std: Ident,
    generics: Generics,

    prefix: String,
    uses_into: bool,
    strip_option: bool,
    borrow_self: bool,
    bool: bool,
    generate_public: bool,
    generate_private: bool,

    generate_delegates: Vec<ExternalDelegate>,
}

struct FieldDef {
    field_name: Ident,
    field_ty: Type,
    field_doc: SynTokenStream,
    setter_name: Ident,
    uses_into: bool,
    strip_option: bool,
    borrow_self: bool,
    must_use: bool,
    bool: bool,
}

fn init_container_def(input: &DeriveInput) -> Result<ContainerDef, SynTokenStream> {
    let darling_attrs: ContainerAttrs = match FromDeriveInput::from_derive_input(input) {
        Ok(v) => v,
        Err(e) => return Err(e.write_errors()),
    };

    let ident = &darling_attrs.ident;
    let (_, generics_ty, _) = darling_attrs.generics.split_for_impl();
    let ty = quote! { #ident #generics_ty };

    let generate = darling_attrs.generate.unwrap_or(true);
    Ok(ContainerDef {
        ty: parse2(ty).expect("Internal error: failed to parse internally generated type."),
        std: if darling_attrs.no_std.is_present() {
            Ident::new("core", Span::call_site())
        } else {
            Ident::new("std", Span::call_site())
        },
        borrow_self: darling_attrs.borrow_self,
        generics: darling_attrs.generics,
        prefix: darling_attrs.prefix.unwrap_or(String::new()),
        uses_into: darling_attrs.into,
        strip_option: darling_attrs.strip_option,
        bool: darling_attrs.bool,
        generate_public: generate && darling_attrs.generate_public.unwrap_or(true),
        generate_private: generate && darling_attrs.generate_private.unwrap_or(true),
        generate_delegates: darling_attrs.generate_delegates,
    })
}

fn init_field_def(
    container: &ContainerDef,
    field: &Field,
) -> Result<Option<FieldDef>, SynTokenStream> {
    // Decode the various attribute options.
    let darling_attrs: FieldAttrs = match FromField::from_field(field) {
        Ok(v) => v,
        Err(e) => return Err(e.write_errors()),
    };

    // Check whether this field should generate a setter.
    if darling_attrs.skip {
        return Ok(None);
    }
    let generates = match field.vis {
        Visibility::Public(_) => container.generate_public,
        _ => container.generate_private,
    };
    if !(darling_attrs.generate || generates) {
        return Ok(None);
    }

    // Returns a definition for this field.
    let ident = match &field.ident {
        Some(i) => i.clone(),
        None => panic!("Internal error: init_field_def on wrong item."),
    };
    Ok(Some(FieldDef {
        field_name: ident.clone(),
        field_ty: field.ty.clone(),
        field_doc: if let Visibility::Public(_) = field.vis {
            let doc_str =
                format!("Sets the [`{}`](#structfield.{}) field of this struct.", ident, ident);
            quote! { #[doc = #doc_str] }
        } else if let Some(x) = darling_attrs.doc {
            quote! { #[doc = #x] }
        } else {
            let attrs = darling_attrs.attrs;
            quote! { #( #attrs )* }
        },
        setter_name: darling_attrs
            .rename
            .unwrap_or_else(|| Ident::new(&format!("{}{}", container.prefix, ident), ident.span())),
        uses_into: darling_attrs.into.unwrap_or(container.uses_into),
        strip_option: darling_attrs.strip_option.unwrap_or(container.strip_option),
        borrow_self: darling_attrs.borrow_self.unwrap_or(container.borrow_self),
        must_use: false == darling_attrs.borrow_self.unwrap_or(container.borrow_self),
        bool: darling_attrs.bool.unwrap_or(container.bool),
    }))
}

fn generate_setter_method(
    container: &ContainerDef,
    def: FieldDef,
    additional_prefix: &Option<String>,
    delegate_toks: &Option<SynTokenStream>,
) -> Result<SynTokenStream, SynTokenStream> {
    let FieldDef { field_name, mut field_ty, field_doc, setter_name, .. } = def;
    let std = &container.std;
    let setter_name = if let Some(additional_prefix) = additional_prefix {
        Ident::new(&format!("{additional_prefix}{}", setter_name), setter_name.span())
    } else {
        setter_name
    };

    // Strips `Option<T>` into `T` if the `strip_option` property is set.
    let mut stripped_option = false;
    if def.strip_option {
        if let Type::Path(path) = &field_ty {
            let segment = path.path.segments.last().unwrap();
            if segment.ident.to_string() == "Option" {
                if let PathArguments::AngleBracketed(path) = &segment.arguments {
                    if let GenericArgument::Type(tp) = path.args.first().unwrap() {
                        field_ty = tp.clone();
                        stripped_option = true;
                    }
                }
            }
        }
    }

    // The type the setter accepts.
    let value_ty = if def.uses_into {
        quote! { impl ::#std::convert::Into<#field_ty> }
    } else {
        quote! { #field_ty }
    };

    // The expression actually stored into the field.
    let mut expr = quote! { value };
    if def.uses_into {
        expr = quote! { #expr.into() };
    }
    if def.bool {
        expr = quote! { true };
    }
    if stripped_option {
        expr = quote! { Some(#expr) };
    }

    // Handle the parameters when bool is enabled.
    let params = if def.bool {
        SynTokenStream::new()
    } else {
        quote! { value: #value_ty }
    };

    // Add extra attributes
    let field_attrs = if def.must_use {
        quote! { #[must_use] }
    } else {
        SynTokenStream::new()
    };

    // Generates the setter method itself.
    if let Some(delegate) = delegate_toks {
        let _self = if def.borrow_self {
            quote! { &mut self }
        } else {
            quote! { mut self }
        };

        let return_self = if def.borrow_self {
            quote! { &mut Self }
        } else {
            quote! { Self }
        };

        Ok(quote! {
            #field_doc
            #field_attrs
            pub fn #setter_name (#_self, #params) -> #return_self {
                self.#delegate.#field_name = #expr;
                self
            }
        })
    } else {
        if def.borrow_self {
            Ok(quote! {
                #field_doc
                #field_attrs
                pub fn #setter_name (&mut self, #params) -> &mut Self {
                    self.#field_name = #expr;
                    self
                }
            })
        } else {
            Ok(quote! {
                #field_doc
                #field_attrs
                pub fn #setter_name (mut self, #params) -> Self {
                    self.#field_name = #expr;
                    self
                }
            })
        }
    }
}

fn generate_setters_for(
    input: &DeriveInput,
    data: &DataStruct,
    generics: &Generics,
    ty: SynTokenStream,
    additional_prefix: Option<String>,
    delegate_toks: Option<SynTokenStream>,
) -> Result<SynTokenStream, SynTokenStream> {
    let container_def = init_container_def(&input)?;
    let mut toks = SynTokenStream::new();
    for field in &data.fields {
        if let Some(field_def) = init_field_def(&container_def, field)? {
            let method = generate_setter_method(
                &container_def,
                field_def,
                &additional_prefix,
                &delegate_toks,
            )?;
            toks.extend(method);
        }
    }

    let (generics_bound, _, generics_where) = generics.split_for_impl();
    Ok(quote! {
        impl #generics_bound #ty #generics_where {
            #toks
        }
    })
}

fn generate_setters(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream, TokenStream> {
    let container_def = init_container_def(&input)?;
    let mut toks = SynTokenStream::new();
    let container_ty = &container_def.ty;
    toks.extend(generate_setters_for(
        input,
        data,
        &container_def.generics,
        quote! { #container_ty },
        None,
        None,
    ));
    for delegate in container_def.generate_delegates {
        let delegate_ty = delegate.ty;
        toks.extend(generate_setters_for(
            input,
            data,
            &delegate.generics,
            quote! { #delegate_ty },
            delegate.prefix,
            if delegate.field.is_some() && delegate.method.is_some() {
                return Err(error(
                    input.span(),
                    "Cannot set both `method` and `field` on a delegate.",
                )
                .into());
            } else if let Some(field) = &delegate.field {
                Some(quote! { #field })
            } else if let Some(method) = &delegate.method {
                Some(quote! { #method() })
            } else {
                return Err(error(
                    input.span(),
                    "Must set either `method` or `field` on a delegate.",
                )
                .into());
            },
        ));
    }
    Ok(toks.into())
}

/// This macro can be used on a struct to generate setter methods for its fields.
///
/// By default, it generates a single method with the same name as each field, that takes self by
/// value and returns a new struct with the field modified.
///
/// The way these methods are generated can be additionally customized with the `#[setters(…)]`
/// attribute.
///
/// # Example
///
/// A basic usage example:
///
/// ```rust
/// # use derive_setters::*;
/// #[derive(Default, Setters, Debug, PartialEq, Eq)]
/// struct BasicStruct {
///     a: u32,
///     b: u32,
///     c: u32,
/// }
/// ```
///
/// This would generate the following implementations:
///
/// ```rust
/// # struct BasicStruct { a: u32, b: u32, c: u32 }
/// impl BasicStruct {
///     #[must_use]
///     pub fn a(mut self, value: u32) -> Self {
///         self.a = value;
///         self
///     }
///
///     #[must_use]
///     pub fn b(mut self, value: u32) -> Self {
///         self.b = value;
///         self
///     }
///
///     #[must_use]
///     pub fn c(mut self, value: u32) -> Self {
///         self.c = value;
///         self
///     }
/// }
/// ```
///
/// # Struct-level options
///
/// * The following options control which fields are actually generated:
///     * `#[setters(generate = false)]` causes setter methods to not be generated by default. This
///       can be overwritten with `#[setters(generate)]` on the fields you want setters for.
///     * `#[setters(generate_private = false)]` causes setter methods to not be generated by
///       default for private fields.
///     * `#[setters(generate_public = false)]` causes setter methods to not be generated by
///       default for public fields.
/// * `#[setters(no_std)]` causes the generated code to use `core` instead of `std`.
/// * `#[setters(prefix = "…")]` causes the setter method on all fields to be prefixed with the
///   given string.
///
/// # Field-level options
///
/// The following options can be set on a field.
///
/// * `#[setters(generate)]` causes a setter method to be generated, overriding struct-level
///   settings.
/// * `#[setters(skip)]` causes no setter method to be generated.
/// * `#[setters(rename = "…")]` causes the setter method to have a different name from the field.
///   When used, the `prefix` option is ignored.
///
/// # Common options
///
/// The following options can be set on either the entire struct, or on an individual field. You
/// can disable the features for a particular field with `#[setters(option = false)]`
///
/// * `#[setters(into)]` causes the setter method to accept any type that can be converted into the
///   field's type via the [`Into`] trait.
/// * `#[setters(strip_option)]` causes the setter method to accept `T` instead of `Option<T>`. If
///   applied to a field  that isn't wrapped in an `Option`, it does nothing.
/// * `#[setters(bool)]` causes the setter method to take no arguments, and to always set the value
///   of the field to `true`.
/// * `#[setters(borrow_self)]` causes the generated setter method to borrow `&mut self` instead of
///   taking `self`, and return `&mut Self` rather than `Self`.
///
/// # Delegation options
///
/// The following options allow additional setters to be generated for wrappers around the type
/// this macro is used on.
///
/// * `#[setters(generate_delegates(ty = "OtherTy", field = "some_field"))]` causes all setter
///   methods on this struct to be duplicated on the target struct, instead modifying
///   `self.some_field` instead of `self`.
/// * `#[setters(generate_delegates(ty = "OtherTy", method = "get_field"))]` does the same thing as
///   above, except calling `get_field` with no arguments instead of directly accessing a field.
/// * `#[setters(generate_delegates(ty = "OtherTy", field = "some_field", prefix = "set_"))]` adds
///   the prefix `set_` to all delegated fields.
/// * `#[setters(generate_delegates(ty = "OtherTy<T>", generics = "<T>", method = "get_field"))]`
///   does the same thing as above, but it supports generic types.

#[proc_macro_derive(Setters, attributes(setters))]
pub fn derive_setters(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    if let Data::Struct(data) = &input.data {
        generate_setters(&input, data).unwrap_or_else(|toks| toks)
    } else {
        error(input.span(), "`#[derive(Setters)] may only be used on structs.").into()
    }
}
