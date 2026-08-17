//! Procedural macros for bytecheck.

#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    unused,
    clippy::all
)]

use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, AttrStyle, Data,
    DeriveInput, Error, Fields, Ident, Index, Lit, LitStr, Meta, NestedMeta, Path, Token,
    WherePredicate,
};

#[derive(Default)]
struct Repr {
    pub transparent: Option<Path>,
    pub packed: Option<Path>,
    pub c: Option<Path>,
    pub int: Option<Path>,
}

#[derive(Default)]
struct Attributes {
    pub repr: Repr,
    pub bound: Option<LitStr>,
    pub bytecheck_crate: Option<Path>,
}

fn parse_check_bytes_attributes(attributes: &mut Attributes, meta: &Meta) -> Result<(), Error> {
    match meta {
        Meta::NameValue(meta) => {
            if meta.path.is_ident("bound") {
                if let Lit::Str(ref lit_str) = meta.lit {
                    if attributes.bound.is_none() {
                        attributes.bound = Some(lit_str.clone());
                        Ok(())
                    } else {
                        Err(Error::new_spanned(
                            meta,
                            "check_bytes bound already specified",
                        ))
                    }
                } else {
                    Err(Error::new_spanned(
                        &meta.lit,
                        "bound arguments must be a string",
                    ))
                }
            } else if meta.path.is_ident("crate") {
                if let Lit::Str(ref lit_str) = meta.lit {
                    if attributes.bytecheck_crate.is_none() {
                        let tokens = respan(syn::parse_str(&lit_str.value())?, lit_str.span());
                        let parsed: Path = syn::parse2(tokens)?;
                        attributes.bytecheck_crate = Some(parsed);
                        Ok(())
                    } else {
                        Err(Error::new_spanned(
                            meta,
                            "check_bytes crate already specified",
                        ))
                    }
                } else {
                    Err(Error::new_spanned(
                        &meta.lit,
                        "crate argument must be a string",
                    ))
                }
            } else {
                Err(Error::new_spanned(
                    &meta.path,
                    "unrecognized check_bytes argument",
                ))
            }
        }
        _ => Err(Error::new_spanned(
            meta,
            "unrecognized check_bytes argument",
        )),
    }
}

fn parse_attributes(input: &DeriveInput) -> Result<Attributes, Error> {
    let mut result = Attributes::default();
    for a in input.attrs.iter() {
        if let AttrStyle::Outer = a.style {
            if let Ok(Meta::List(meta)) = a.parse_meta() {
                if meta.path.is_ident("check_bytes") {
                    for nested in meta.nested.iter() {
                        if let NestedMeta::Meta(meta) = nested {
                            parse_check_bytes_attributes(&mut result, meta)?;
                        } else {
                            return Err(Error::new_spanned(
                                nested,
                                "check_bytes parameters must be metas",
                            ));
                        }
                    }
                } else if meta.path.is_ident("repr") {
                    for n in meta.nested.iter() {
                        if let NestedMeta::Meta(Meta::Path(path)) = n {
                            if path.is_ident("transparent") {
                                result.repr.transparent = Some(path.clone());
                            } else if path.is_ident("packed") {
                                result.repr.packed = Some(path.clone());
                            } else if path.is_ident("C") {
                                result.repr.c = Some(path.clone());
                            } else if path.is_ident("align") {
                                // Ignore alignment modifiers
                            } else {
                                let is_int_repr = path.is_ident("i8")
                                    || path.is_ident("i16")
                                    || path.is_ident("i32")
                                    || path.is_ident("i64")
                                    || path.is_ident("i128")
                                    || path.is_ident("u8")
                                    || path.is_ident("u16")
                                    || path.is_ident("u32")
                                    || path.is_ident("u64")
                                    || path.is_ident("u128");

                                if is_int_repr {
                                    result.repr.int = Some(path.clone());
                                } else {
                                    return Err(Error::new_spanned(
                                        path,
                                        "invalid repr, available reprs are transparent, C, i* and u*",
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(result)
}

/// Derives `CheckBytes` for the labeled type.
///
/// Additional arguments can be specified using the `#[check_bytes(...)]` attribute:
///
/// - `bound = "..."`: Adds additional bounds to the `CheckBytes` implementation. This can be
///   especially useful when dealing with recursive structures, where bounds may need to be omitted
///   to prevent recursive type definitions.
///
/// This derive macro automatically adds a type bound `field: CheckBytes<__C>` for each field type.
/// This can cause an overflow while evaluating trait bounds if the structure eventually references
/// its own type, as the implementation of `CheckBytes` for a struct depends on each field type
/// implementing it as well. Adding the attribute `#[omit_bounds]` to a field will suppress this
/// trait bound and allow recursive structures. This may be too coarse for some types, in which case
/// additional type bounds may be required with `bound = "..."`.
#[proc_macro_derive(CheckBytes, attributes(check_bytes, omit_bounds))]
pub fn check_bytes_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match derive_check_bytes(parse_macro_input!(input as DeriveInput)) {
        Ok(result) => result.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_check_bytes(mut input: DeriveInput) -> Result<TokenStream, Error> {
    let attributes = parse_attributes(&input)?;

    let mut impl_input_generics = input.generics.clone();
    let impl_where_clause = impl_input_generics.make_where_clause();
    if let Some(ref bounds) = attributes.bound {
        let clauses =
            bounds.parse_with(Punctuated::<WherePredicate, Token![,]>::parse_terminated)?;
        for clause in clauses {
            impl_where_clause.predicates.push(clause);
        }
    }
    impl_input_generics
        .params
        .insert(0, parse_quote! { __C: ?Sized });

    let name = &input.ident;

    let (impl_generics, _, impl_where_clause) = impl_input_generics.split_for_impl();
    let impl_where_clause = impl_where_clause.unwrap();

    input.generics.make_where_clause();
    let (struct_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let where_clause = where_clause.unwrap();

    let check_bytes_impl = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let mut check_where = impl_where_clause.clone();
                for field in fields
                    .named
                    .iter()
                    .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("omit_bounds")))
                {
                    let ty = &field.ty;
                    check_where
                        .predicates
                        .push(parse_quote! { #ty: CheckBytes<__C> });
                }

                let field_checks = fields.named.iter().map(|f| {
                    let field = &f.ident;
                    let ty = &f.ty;
                    quote_spanned! { ty.span() =>
                        <#ty as CheckBytes<__C>>::check_bytes(
                            ::core::ptr::addr_of!((*value).#field),
                            context
                        ).map_err(|e| StructCheckError {
                            field_name: stringify!(#field),
                            inner: ErrorBox::new(e),
                        })?;
                    }
                });

                quote! {
                    #[automatically_derived]
                    impl #impl_generics CheckBytes<__C> for #name #ty_generics #check_where {
                        type Error = StructCheckError;

                        unsafe fn check_bytes<'__bytecheck>(
                            value: *const Self,
                            context: &mut __C,
                        ) -> ::core::result::Result<&'__bytecheck Self, StructCheckError> {
                            let bytes = value.cast::<u8>();
                            #(#field_checks)*
                            Ok(&*value)
                        }
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let mut check_where = impl_where_clause.clone();
                for field in fields
                    .unnamed
                    .iter()
                    .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("omit_bounds")))
                {
                    let ty = &field.ty;
                    check_where
                        .predicates
                        .push(parse_quote! { #ty: CheckBytes<__C> });
                }

                let field_checks = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let ty = &f.ty;
                    let index = Index::from(i);
                    quote_spanned! { ty.span() =>
                        <#ty as CheckBytes<__C>>::check_bytes(
                            ::core::ptr::addr_of!((*value).#index),
                            context
                        ).map_err(|e| TupleStructCheckError {
                            field_index: #i,
                            inner: ErrorBox::new(e),
                        })?;
                    }
                });

                quote! {
                    #[automatically_derived]
                    impl #impl_generics CheckBytes<__C> for #name #ty_generics #check_where {
                        type Error = TupleStructCheckError;

                        unsafe fn check_bytes<'__bytecheck>(
                            value: *const Self,
                            context: &mut __C,
                        ) -> ::core::result::Result<&'__bytecheck Self, TupleStructCheckError> {
                            let bytes = value.cast::<u8>();
                            #(#field_checks)*
                            Ok(&*value)
                        }
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    #[automatically_derived]
                    impl #impl_generics CheckBytes<__C> for #name #ty_generics #impl_where_clause {
                        type Error = Infallible;

                        unsafe fn check_bytes<'__bytecheck>(
                            value: *const Self,
                            context: &mut __C,
                        ) -> ::core::result::Result<&'__bytecheck Self, Infallible> {
                            Ok(&*value)
                        }
                    }
                }
            }
        },
        Data::Enum(ref data) => {
            if let Some(path) = attributes.repr.transparent.or(attributes.repr.packed) {
                return Err(Error::new_spanned(
                    path,
                    "enums implementing CheckBytes cannot be repr(transparent) or repr(packed)",
                ));
            }

            let repr = match attributes.repr.int {
                None => {
                    return Err(Error::new(
                        input.span(),
                        "enums implementing CheckBytes must be repr(Int)",
                    ));
                }
                Some(ref repr) => repr,
            };

            let mut check_where = impl_where_clause.clone();
            for v in data.variants.iter() {
                match v.fields {
                    Fields::Named(ref fields) => {
                        for field in fields
                            .named
                            .iter()
                            .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("omit_bounds")))
                        {
                            let ty = &field.ty;
                            check_where
                                .predicates
                                .push(parse_quote! { #ty: CheckBytes<__C> });
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        for field in fields
                            .unnamed
                            .iter()
                            .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("omit_bounds")))
                        {
                            let ty = &field.ty;
                            check_where
                                .predicates
                                .push(parse_quote! { #ty: CheckBytes<__C> });
                        }
                    }
                    Fields::Unit => (),
                }
            }

            let tag_variant_defs = data.variants.iter().map(|v| {
                let variant = &v.ident;
                if let Some((_, expr)) = &v.discriminant {
                    quote_spanned! { variant.span() => #variant = #expr }
                } else {
                    quote_spanned! { variant.span() => #variant }
                }
            });

            let discriminant_const_defs = data.variants.iter().map(|v| {
                let variant = &v.ident;
                quote! {
                    #[allow(non_upper_case_globals)]
                    const #variant: #repr = Tag::#variant as #repr;
                }
            });

            let tag_variant_values = data.variants.iter().map(|v| {
                let name = &v.ident;
                quote_spanned! { name.span() => Discriminant::#name }
            });

            let variant_structs = data.variants.iter().map(|v| {
                let variant = &v.ident;
                let variant_name = Ident::new(&format!("Variant{}", variant), v.span());
                match v.fields {
                    Fields::Named(ref fields) => {
                        let fields = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            let ty = &f.ty;
                            quote_spanned! { f.span() => #name: #ty }
                        });
                        quote_spanned! { name.span() =>
                            #[repr(C)]
                            struct #variant_name #struct_generics #where_clause {
                                __tag: Tag,
                                #(#fields,)*
                                __phantom: PhantomData<#name #ty_generics>,
                            }
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        let fields = fields.unnamed.iter().map(|f| {
                            let ty = &f.ty;
                            quote_spanned! { f.span() => #ty }
                        });
                        quote_spanned! { name.span() =>
                            #[repr(C)]
                            struct #variant_name #struct_generics (
                                Tag,
                                #(#fields,)*
                                PhantomData<#name #ty_generics>
                            ) #where_clause;
                        }
                    }
                    Fields::Unit => quote! {},
                }
            });

            let check_arms = data.variants.iter().map(|v| {
                let variant = &v.ident;
                let variant_name = Ident::new(&format!("Variant{}", variant), v.span());
                match v.fields {
                    Fields::Named(ref fields) => {
                        let checks = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            let ty = &f.ty;
                            quote! {
                                <#ty as CheckBytes<__C>>::check_bytes(
                                    ::core::ptr::addr_of!((*value).#name),
                                    context
                                ).map_err(|e| EnumCheckError::InvalidStruct {
                                    variant_name: stringify!(#variant),
                                    inner: StructCheckError {
                                        field_name: stringify!(#name),
                                        inner: ErrorBox::new(e),
                                    },
                                })?;
                            }
                        });
                        quote_spanned! { variant.span() => {
                            let value = value.cast::<#variant_name #ty_generics>();
                            #(#checks)*
                        } }
                    }
                    Fields::Unnamed(ref fields) => {
                        let checks = fields.unnamed.iter().enumerate().map(|(i, f)| {
                            let ty = &f.ty;
                            let index = Index::from(i + 1);
                            quote! {
                                <#ty as CheckBytes<__C>>::check_bytes(
                                    ::core::ptr::addr_of!((*value).#index),
                                    context
                                ).map_err(|e| EnumCheckError::InvalidTuple {
                                    variant_name: stringify!(#variant),
                                    inner: TupleStructCheckError {
                                        field_index: #i,
                                        inner: ErrorBox::new(e),
                                    },
                                })?;
                            }
                        });
                        quote_spanned! { variant.span() => {
                            let value = value.cast::<#variant_name #ty_generics>();
                            #(#checks)*
                        } }
                    }
                    Fields::Unit => quote_spanned! { name.span() => (), },
                }
            });

            quote! {
                #[repr(#repr)]
                enum Tag {
                    #(#tag_variant_defs,)*
                }

                struct Discriminant;

                #[automatically_derived]
                impl Discriminant {
                    #(#discriminant_const_defs)*
                }

                #(#variant_structs)*

                #[automatically_derived]
                impl #impl_generics CheckBytes<__C> for #name #ty_generics #check_where {
                    type Error = EnumCheckError<#repr>;

                    unsafe fn check_bytes<'__bytecheck>(
                        value: *const Self,
                        context: &mut __C,
                    ) -> ::core::result::Result<&'__bytecheck Self, EnumCheckError<#repr>> {
                        let tag = *value.cast::<#repr>();
                        match tag {
                            #(#tag_variant_values => #check_arms)*
                            _ => return Err(EnumCheckError::InvalidTag(tag)),
                        }
                        Ok(&*value)
                    }
                }
            }
        }
        Data::Union(_) => {
            return Err(Error::new(
                input.span(),
                "CheckBytes cannot be derived for unions",
            ));
        }
    };

    // Default to `bytecheck`, rather than `::bytecheck`,
    // to allow providing it from a reexport, e.g. `use rkyv::bytecheck;`.
    let bytecheck_crate = attributes
        .bytecheck_crate
        .unwrap_or(parse_quote!(bytecheck));

    Ok(quote! {
        #[allow(unused_results)]
        const _: () = {
            use ::core::{convert::Infallible, marker::PhantomData};
            use #bytecheck_crate::{
                CheckBytes,
                EnumCheckError,
                ErrorBox,
                StructCheckError,
                TupleStructCheckError,
            };

            #check_bytes_impl
        };
    })
}

fn respan(stream: TokenStream, span: Span) -> TokenStream {
    stream
        .into_iter()
        .map(|token| respan_token(token, span))
        .collect()
}

fn respan_token(mut token: TokenTree, span: Span) -> TokenTree {
    if let TokenTree::Group(g) = &mut token {
        *g = Group::new(g.delimiter(), respan(g.stream(), span));
    }
    token.set_span(span);
    token
}
