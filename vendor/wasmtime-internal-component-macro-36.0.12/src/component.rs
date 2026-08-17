use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashSet;
use std::fmt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Data, DeriveInput, Error, Ident, Result, Token, braced, parse_quote};
use wasmtime_component_util::{DiscriminantSize, FlagsSize};

mod kw {
    syn::custom_keyword!(record);
    syn::custom_keyword!(variant);
    syn::custom_keyword!(flags);
    syn::custom_keyword!(name);
    syn::custom_keyword!(wasmtime_crate);
}

#[derive(Debug, Copy, Clone)]
enum Style {
    Record,
    Enum,
    Variant,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Style::Record => f.write_str("record"),
            Style::Enum => f.write_str("enum"),
            Style::Variant => f.write_str("variant"),
        }
    }
}

#[derive(Debug, Clone)]
enum ComponentAttr {
    Style(Style),
    WasmtimeCrate(syn::Path),
}

impl Parse for ComponentAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::record) {
            input.parse::<kw::record>()?;
            Ok(ComponentAttr::Style(Style::Record))
        } else if lookahead.peek(kw::variant) {
            input.parse::<kw::variant>()?;
            Ok(ComponentAttr::Style(Style::Variant))
        } else if lookahead.peek(Token![enum]) {
            input.parse::<Token![enum]>()?;
            Ok(ComponentAttr::Style(Style::Enum))
        } else if lookahead.peek(kw::wasmtime_crate) {
            input.parse::<kw::wasmtime_crate>()?;
            input.parse::<Token![=]>()?;
            Ok(ComponentAttr::WasmtimeCrate(input.parse()?))
        } else if input.peek(kw::flags) {
            Err(input.error(
                "`flags` not allowed here; \
                 use `wasmtime::component::flags!` macro to define `flags` types",
            ))
        } else {
            Err(lookahead.error())
        }
    }
}

fn find_rename(attributes: &[syn::Attribute]) -> Result<Option<syn::LitStr>> {
    let mut name = None;

    for attribute in attributes {
        if !attribute.path().is_ident("component") {
            continue;
        }
        let name_literal = attribute.parse_args_with(|parser: ParseStream<'_>| {
            parser.parse::<kw::name>()?;
            parser.parse::<Token![=]>()?;
            parser.parse::<syn::LitStr>()
        })?;

        if name.is_some() {
            return Err(Error::new_spanned(
                attribute,
                "duplicate field rename attribute",
            ));
        }

        name = Some(name_literal);
    }

    Ok(name)
}

fn add_trait_bounds(generics: &syn::Generics, bound: syn::TypeParamBound) -> syn::Generics {
    let mut generics = generics.clone();
    for param in &mut generics.params {
        if let syn::GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(bound.clone());
        }
    }
    generics
}

pub struct VariantCase<'a> {
    attrs: &'a [syn::Attribute],
    ident: &'a syn::Ident,
    ty: Option<&'a syn::Type>,
}

pub trait Expander {
    fn expand_record(
        &self,
        name: &syn::Ident,
        generics: &syn::Generics,
        fields: &[&syn::Field],
        wasmtime_crate: &syn::Path,
    ) -> Result<TokenStream>;

    fn expand_variant(
        &self,
        name: &syn::Ident,
        generics: &syn::Generics,
        discriminant_size: DiscriminantSize,
        cases: &[VariantCase],
        wasmtime_crate: &syn::Path,
    ) -> Result<TokenStream>;

    fn expand_enum(
        &self,
        name: &syn::Ident,
        discriminant_size: DiscriminantSize,
        cases: &[VariantCase],
        wasmtime_crate: &syn::Path,
    ) -> Result<TokenStream>;
}

pub fn expand(expander: &dyn Expander, input: &DeriveInput) -> Result<TokenStream> {
    let mut wasmtime_crate = None;
    let mut style = None;

    for attribute in &input.attrs {
        if !attribute.path().is_ident("component") {
            continue;
        }
        match attribute.parse_args()? {
            ComponentAttr::WasmtimeCrate(c) => wasmtime_crate = Some(c),
            ComponentAttr::Style(attr_style) => {
                if style.is_some() {
                    return Err(Error::new_spanned(
                        attribute,
                        "duplicate `component` attribute",
                    ));
                }
                style = Some(attr_style);
            }
        }
    }

    let style = style.ok_or_else(|| Error::new_spanned(input, "missing `component` attribute"))?;
    let wasmtime_crate = wasmtime_crate.unwrap_or_else(default_wasmtime_crate);
    match style {
        Style::Record => expand_record(expander, input, &wasmtime_crate),
        Style::Enum | Style::Variant => expand_variant(expander, input, style, &wasmtime_crate),
    }
}

fn default_wasmtime_crate() -> syn::Path {
    Ident::new("wasmtime", Span::call_site()).into()
}

fn expand_record(
    expander: &dyn Expander,
    input: &DeriveInput,
    wasmtime_crate: &syn::Path,
) -> Result<TokenStream> {
    let name = &input.ident;

    let body = if let Data::Struct(body) = &input.data {
        body
    } else {
        return Err(Error::new(
            name.span(),
            "`record` component types can only be derived for Rust `struct`s",
        ));
    };

    match &body.fields {
        syn::Fields::Named(fields) => expander.expand_record(
            &input.ident,
            &input.generics,
            &fields.named.iter().collect::<Vec<_>>(),
            wasmtime_crate,
        ),

        syn::Fields::Unnamed(_) | syn::Fields::Unit => Err(Error::new(
            name.span(),
            "`record` component types can only be derived for `struct`s with named fields",
        )),
    }
}

fn expand_variant(
    expander: &dyn Expander,
    input: &DeriveInput,
    style: Style,
    wasmtime_crate: &syn::Path,
) -> Result<TokenStream> {
    let name = &input.ident;

    let body = if let Data::Enum(body) = &input.data {
        body
    } else {
        return Err(Error::new(
            name.span(),
            format!("`{style}` component types can only be derived for Rust `enum`s"),
        ));
    };

    if body.variants.is_empty() {
        return Err(Error::new(
            name.span(),
            format!(
                "`{style}` component types can only be derived for Rust `enum`s with at least one variant"
            ),
        ));
    }

    let discriminant_size = DiscriminantSize::from_count(body.variants.len()).ok_or_else(|| {
        Error::new(
            input.ident.span(),
            "`enum`s with more than 2^32 variants are not supported",
        )
    })?;

    let cases = body
        .variants
        .iter()
        .map(
            |syn::Variant {
                 attrs,
                 ident,
                 fields,
                 ..
             }| {
                Ok(VariantCase {
                    attrs,
                    ident,
                    ty: match fields {
                        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                            Some(&fields.unnamed[0].ty)
                        }
                        syn::Fields::Unit => None,
                        _ => {
                            return Err(Error::new(
                                name.span(),
                                format!(
                                    "`{}` component types can only be derived for Rust `enum`s \
                                     containing variants with {}",
                                    style,
                                    match style {
                                        Style::Variant => "at most one unnamed field each",
                                        Style::Enum => "no fields",
                                        Style::Record => unreachable!(),
                                    }
                                ),
                            ));
                        }
                    },
                })
            },
        )
        .collect::<Result<Vec<_>>>()?;

    match style {
        Style::Variant => expander.expand_variant(
            &input.ident,
            &input.generics,
            discriminant_size,
            &cases,
            wasmtime_crate,
        ),
        Style::Enum => {
            validate_enum(input, &body, discriminant_size)?;
            expander.expand_enum(&input.ident, discriminant_size, &cases, wasmtime_crate)
        }
        Style::Record => unreachable!(),
    }
}

/// Validates component model `enum` definitions are accompanied with
/// appropriate `#[repr]` tags. Additionally requires that no discriminants are
/// listed to ensure that unsafe transmutes in lift are valid.
fn validate_enum(input: &DeriveInput, body: &syn::DataEnum, size: DiscriminantSize) -> Result<()> {
    if !input.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &input.generics.params,
            "cannot have generics on an `enum`",
        ));
    }
    if let Some(clause) = &input.generics.where_clause {
        return Err(Error::new_spanned(
            clause,
            "cannot have a where clause on an `enum`",
        ));
    }
    let expected_discr = match size {
        DiscriminantSize::Size1 => "u8",
        DiscriminantSize::Size2 => "u16",
        DiscriminantSize::Size4 => "u32",
    };
    let mut found_repr = false;
    for attr in input.attrs.iter() {
        if !attr.meta.path().is_ident("repr") {
            continue;
        }
        let list = attr.meta.require_list()?;
        found_repr = true;
        if list.tokens.to_string() != expected_discr {
            return Err(Error::new_spanned(
                &list.tokens,
                format!(
                    "expected `repr({expected_discr})`, found `repr({})`",
                    list.tokens
                ),
            ));
        }
    }
    if !found_repr {
        return Err(Error::new_spanned(
            &body.enum_token,
            format!("missing required `#[repr({expected_discr})]`"),
        ));
    }

    for case in body.variants.iter() {
        if let Some((_, expr)) = &case.discriminant {
            return Err(Error::new_spanned(
                expr,
                "cannot have an explicit discriminant",
            ));
        }
    }

    Ok(())
}

fn expand_record_for_component_type(
    name: &syn::Ident,
    generics: &syn::Generics,
    fields: &[&syn::Field],
    typecheck: TokenStream,
    typecheck_argument: TokenStream,
    wt: &syn::Path,
) -> Result<TokenStream> {
    let internal = quote!(#wt::component::__internal);

    let mut lower_generic_params = TokenStream::new();
    let mut lower_generic_args = TokenStream::new();
    let mut lower_field_declarations = TokenStream::new();
    let mut abi_list = TokenStream::new();
    let mut unique_types = HashSet::new();

    for (index, syn::Field { ident, ty, .. }) in fields.iter().enumerate() {
        let generic = format_ident!("T{}", index);

        lower_generic_params.extend(quote!(#generic: Copy,));
        lower_generic_args.extend(quote!(<#ty as #wt::component::ComponentType>::Lower,));

        lower_field_declarations.extend(quote!(#ident: #generic,));

        abi_list.extend(quote!(
            <#ty as #wt::component::ComponentType>::ABI,
        ));

        unique_types.insert(ty);
    }

    let generics = add_trait_bounds(generics, parse_quote!(#wt::component::ComponentType));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let lower = format_ident!("Lower{}", name);

    // You may wonder why we make the types of all the fields of the #lower struct generic.  This is to work
    // around the lack of [perfect derive support in
    // rustc](https://smallcultfollowing.com/babysteps//blog/2022/04/12/implied-bounds-and-perfect-derive/#what-is-perfect-derive)
    // as of this writing.
    //
    // If the struct we're deriving a `ComponentType` impl for has any generic parameters, then #lower needs
    // generic parameters too.  And if we just copy the parameters and bounds from the impl to #lower, then the
    // `#[derive(Clone, Copy)]` will fail unless the original generics were declared with those bounds, which
    // we don't want to require.
    //
    // Alternatively, we could just pass the `Lower` associated type of each generic type as arguments to
    // #lower, but that would require distinguishing between generic and concrete types when generating
    // #lower_field_declarations, which would require some form of symbol resolution.  That doesn't seem worth
    // the trouble.

    let expanded = quote! {
        #[doc(hidden)]
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct #lower <#lower_generic_params> {
            #lower_field_declarations
            _align: [#wt::ValRaw; 0],
        }

        unsafe impl #impl_generics #wt::component::ComponentType for #name #ty_generics #where_clause {
            type Lower = #lower <#lower_generic_args>;

            const ABI: #internal::CanonicalAbiInfo =
                #internal::CanonicalAbiInfo::record_static(&[#abi_list]);

            #[inline]
            fn typecheck(
                ty: &#internal::InterfaceType,
                types: &#internal::InstanceType<'_>,
            ) -> #internal::anyhow::Result<()> {
                #internal::#typecheck(ty, types, &[#typecheck_argument])
            }
        }
    };

    Ok(quote!(const _: () = { #expanded };))
}

fn quote(size: DiscriminantSize, discriminant: usize) -> TokenStream {
    match size {
        DiscriminantSize::Size1 => {
            let discriminant = u8::try_from(discriminant).unwrap();
            quote!(#discriminant)
        }
        DiscriminantSize::Size2 => {
            let discriminant = u16::try_from(discriminant).unwrap();
            quote!(#discriminant)
        }
        DiscriminantSize::Size4 => {
            let discriminant = u32::try_from(discriminant).unwrap();
            quote!(#discriminant)
        }
    }
}

pub struct LiftExpander;

impl Expander for LiftExpander {
    fn expand_record(
        &self,
        name: &syn::Ident,
        generics: &syn::Generics,
        fields: &[&syn::Field],
        wt: &syn::Path,
    ) -> Result<TokenStream> {
        let internal = quote!(#wt::component::__internal);

        let mut lifts = TokenStream::new();
        let mut loads = TokenStream::new();

        for (i, syn::Field { ident, ty, .. }) in fields.iter().enumerate() {
            let field_ty = quote!(ty.fields[#i].ty);
            lifts.extend(
                quote!(#ident: <#ty as #wt::component::Lift>::linear_lift_from_flat(
                cx, #field_ty, &src.#ident
            )?,),
            );

            loads.extend(
                quote!(#ident: <#ty as #wt::component::Lift>::linear_lift_from_memory(
                cx, #field_ty,
                &bytes
                    [<#ty as #wt::component::ComponentType>::ABI.next_field32_size(&mut offset)..]
                    [..<#ty as #wt::component::ComponentType>::SIZE32]
            )?,),
            );
        }

        let generics = add_trait_bounds(generics, parse_quote!(#wt::component::Lift));
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let extract_ty = quote! {
            let ty = match ty {
                #internal::InterfaceType::Record(i) => &cx.types[i],
                _ => #internal::bad_type_info(),
            };
        };

        let expanded = quote! {
            unsafe impl #impl_generics #wt::component::Lift for #name #ty_generics #where_clause {
                #[inline]
                fn linear_lift_from_flat(
                    cx: &mut #internal::LiftContext<'_>,
                    ty: #internal::InterfaceType,
                    src: &Self::Lower,
                ) -> #internal::anyhow::Result<Self> {
                    #extract_ty
                    Ok(Self {
                        #lifts
                    })
                }

                #[inline]
                fn linear_lift_from_memory(
                    cx: &mut #internal::LiftContext<'_>,
                    ty: #internal::InterfaceType,
                    bytes: &[u8],
                ) -> #internal::anyhow::Result<Self> {
                    #extract_ty
                    debug_assert!(
                        (bytes.as_ptr() as usize)
                            % (<Self as #wt::component::ComponentType>::ALIGN32 as usize)
                            == 0
                    );
                    let mut offset = 0;
                    Ok(Self {
                        #loads
                    })
                }
            }
        };

        Ok(expanded)
    }

    fn expand_variant(
        &self,
        name: &syn::Ident,
        generics: &syn::Generics,
        discriminant_size: DiscriminantSize,
        cases: &[VariantCase],
        wt: &syn::Path,
    ) -> Result<TokenStream> {
        let internal = quote!(#wt::component::__internal);

        let mut lifts = TokenStream::new();
        let mut loads = TokenStream::new();

        for (index, VariantCase { ident, ty, .. }) in cases.iter().enumerate() {
            let index_u32 = u32::try_from(index).unwrap();

            let index_quoted = quote(discriminant_size, index);

            if let Some(ty) = ty {
                let payload_ty = quote!(ty.cases[#index].unwrap_or_else(#internal::bad_type_info));
                lifts.extend(
                    quote!(#index_u32 => Self::#ident(<#ty as #wt::component::Lift>::linear_lift_from_flat(
                        cx, #payload_ty, unsafe { &src.payload.#ident }
                    )?),),
                );

                loads.extend(
                    quote!(#index_quoted => Self::#ident(<#ty as #wt::component::Lift>::linear_lift_from_memory(
                        cx, #payload_ty, &payload[..<#ty as #wt::component::ComponentType>::SIZE32]
                    )?),),
                );
            } else {
                lifts.extend(quote!(#index_u32 => Self::#ident,));

                loads.extend(quote!(#index_quoted => Self::#ident,));
            }
        }

        let generics = add_trait_bounds(generics, parse_quote!(#wt::component::Lift));
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let from_bytes = match discriminant_size {
            DiscriminantSize::Size1 => quote!(bytes[0]),
            DiscriminantSize::Size2 => quote!(u16::from_le_bytes(bytes[0..2].try_into()?)),
            DiscriminantSize::Size4 => quote!(u32::from_le_bytes(bytes[0..4].try_into()?)),
        };

        let extract_ty = quote! {
            let ty = match ty {
                #internal::InterfaceType::Variant(i) => &cx.types[i],
                _ => #internal::bad_type_info(),
            };
        };

        let expanded = quote! {
            unsafe impl #impl_generics #wt::component::Lift for #name #ty_generics #where_clause {
                #[inline]
                fn linear_lift_from_flat(
                    cx: &mut #internal::LiftContext<'_>,
                    ty: #internal::InterfaceType,
                    src: &Self::Lower,
                ) -> #internal::anyhow::Result<Self> {
                    #extract_ty
                    Ok(match src.tag.get_u32() {
                        #lifts
                        discrim => #internal::anyhow::bail!("unexpected discriminant: {}", discrim),
                    })
                }

                #[inline]
                fn linear_lift_from_memory(
                    cx: &mut #internal::LiftContext<'_>,
                    ty: #internal::InterfaceType,
                    bytes: &[u8],
                ) -> #internal::anyhow::Result<Self> {
                    let align = <Self as #wt::component::ComponentType>::ALIGN32;
                    debug_assert!((bytes.as_ptr() as usize) % (align as usize) == 0);
                    let discrim = #from_bytes;
                    let payload_offset = <Self as #internal::ComponentVariant>::PAYLOAD_OFFSET32;
                    let payload = &bytes[payload_offset..];
                    #extract_ty
                    Ok(match discrim {
                        #loads
                        discrim => #internal::anyhow::bail!("unexpected discriminant: {}", discrim),
                    })
                }
            }
        };

        Ok(expanded)
    }

    fn expand_enum(
        &self,
        name: &syn::Ident,
        discriminant_size: DiscriminantSize,
        cases: &[VariantCase],
        wt: &syn::Path,
    ) -> Result<TokenStream> {
        let internal = quote!(#wt::component::__internal);

        let (from_bytes, discrim_ty) = match discriminant_size {
            DiscriminantSize::Size1 => (quote!(bytes[0]), quote!(u8)),
            DiscriminantSize::Size2 => (
                quote!(u16::from_le_bytes(bytes[0..2].try_into()?)),
                quote!(u16),
            ),
            DiscriminantSize::Size4 => (
                quote!(u32::from_le_bytes(bytes[0..4].try_into()?)),
                quote!(u32),
            ),
        };
        let discrim_limit = proc_macro2::Literal::usize_unsuffixed(cases.len());

        let extract_ty = quote! {
            let ty = match ty {
                #internal::InterfaceType::Enum(i) => &cx.types[i],
                _ => #internal::bad_type_info(),
            };
        };

        let expanded = quote! {
            unsafe impl #wt::component::Lift for #name {
                #[inline]
                fn linear_lift_from_flat(
                    cx: &mut #internal::LiftContext<'_>,
                    ty: #internal::InterfaceType,
                    src: &Self::Lower,
                ) -> #internal::anyhow::Result<Self> {
                    #extract_ty
                    let discrim = src.tag.get_u32();
                    if discrim >= #discrim_limit {
                        #internal::anyhow::bail!("unexpected discriminant: {discrim}");
                    }
                    Ok(unsafe {
                        #internal::transmute::<#discrim_ty, #name>(discrim as #discrim_ty)
                    })
                }

                #[inline]
                fn linear_lift_from_memory(
                    cx: &mut #internal::LiftContext<'_>,
                    ty: #internal::InterfaceType,
                    bytes: &[u8],
                ) -> #internal::anyhow::Result<Self> {
                    let align = <Self as #wt::component::ComponentType>::ALIGN32;
                    debug_assert!((bytes.as_ptr() as usize) % (align as usize) == 0);
                    let discrim = #from_bytes;
                    if discrim >= #discrim_limit {
                        #internal::anyhow::bail!("unexpected discriminant: {discrim}");
                    }
                    Ok(unsafe {
                        #internal::transmute::<#discrim_ty, #name>(discrim)
                    })
                }
            }
        };

        Ok(expanded)
    }
}

pub struct LowerExpander;

impl Expander for LowerExpander {
    fn expand_record(
        &self,
        name: &syn::Ident,
        generics: &syn::Generics,
        fields: &[&syn::Field],
        wt: &syn::Path,
    ) -> Result<TokenStream> {
        let internal = quote!(#wt::component::__internal);

        let mut lowers = TokenStream::new();
        let mut stores = TokenStream::new();

        for (i, syn::Field { ident, ty, .. }) in fields.iter().enumerate() {
            let field_ty = quote!(ty.fields[#i].ty);
            lowers.extend(quote!(#wt::component::Lower::linear_lower_to_flat(
                &self.#ident, cx, #field_ty, #internal::map_maybe_uninit!(dst.#ident)
            )?;));

            stores.extend(quote!(#wt::component::Lower::linear_lower_to_memory(
                &self.#ident,
                cx,
                #field_ty,
                <#ty as #wt::component::ComponentType>::ABI.next_field32_size(&mut offset),
            )?;));
        }

        let generics = add_trait_bounds(generics, parse_quote!(#wt::component::Lower));
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let extract_ty = quote! {
            let ty = match ty {
                #internal::InterfaceType::Record(i) => &cx.types[i],
                _ => #internal::bad_type_info(),
            };
        };

        let expanded = quote! {
            unsafe impl #impl_generics #wt::component::Lower for #name #ty_generics #where_clause {
                #[inline]
                fn linear_lower_to_flat<T>(
                    &self,
                    cx: &mut #internal::LowerContext<'_, T>,
                    ty: #internal::InterfaceType,
                    dst: &mut core::mem::MaybeUninit<Self::Lower>,
                ) -> #internal::anyhow::Result<()> {
                    #extract_ty
                    #lowers
                    Ok(())
                }

                #[inline]
                fn linear_lower_to_memory<T>(
                    &self,
                    cx: &mut #internal::LowerContext<'_, T>,
                    ty: #internal::InterfaceType,
                    mut offset: usize
                ) -> #internal::anyhow::Result<()> {
                    debug_assert!(offset % (<Self as #wt::component::ComponentType>::ALIGN32 as usize) == 0);
                    #extract_ty
                    #stores
                    Ok(())
                }
            }
        };

        Ok(expanded)
    }

    fn expand_variant(
        &self,
        name: &syn::Ident,
        generics: &syn::Generics,
        discriminant_size: DiscriminantSize,
        cases: &[VariantCase],
        wt: &syn::Path,
    ) -> Result<TokenStream> {
        let internal = quote!(#wt::component::__internal);

        let mut lowers = TokenStream::new();
        let mut stores = TokenStream::new();

        for (index, VariantCase { ident, ty, .. }) in cases.iter().enumerate() {
            let index_u32 = u32::try_from(index).unwrap();

            let index_quoted = quote(discriminant_size, index);

            let discriminant_size = usize::from(discriminant_size);

            let pattern;
            let lower;
            let store;

            if ty.is_some() {
                let ty = quote!(ty.cases[#index].unwrap_or_else(#internal::bad_type_info));
                pattern = quote!(Self::#ident(value));
                lower = quote!(value.linear_lower_to_flat(cx, #ty, dst));
                store = quote!(value.linear_lower_to_memory(
                    cx,
                    #ty,
                    offset + <Self as #internal::ComponentVariant>::PAYLOAD_OFFSET32,
                ));
            } else {
                pattern = quote!(Self::#ident);
                lower = quote!(Ok(()));
                store = quote!(Ok(()));
            }

            lowers.extend(quote!(#pattern => {
                #internal::map_maybe_uninit!(dst.tag).write(#wt::ValRaw::u32(#index_u32));
                unsafe {
                    #internal::lower_payload(
                        #internal::map_maybe_uninit!(dst.payload),
                        |payload| #internal::map_maybe_uninit!(payload.#ident),
                        |dst| #lower,
                    )
                }
            }));

            stores.extend(quote!(#pattern => {
                *cx.get::<#discriminant_size>(offset) = #index_quoted.to_le_bytes();
                #store
            }));
        }

        let generics = add_trait_bounds(generics, parse_quote!(#wt::component::Lower));
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let extract_ty = quote! {
            let ty = match ty {
                #internal::InterfaceType::Variant(i) => &cx.types[i],
                _ => #internal::bad_type_info(),
            };
        };

        let expanded = quote! {
            unsafe impl #impl_generics #wt::component::Lower for #name #ty_generics #where_clause {
                #[inline]
                fn linear_lower_to_flat<T>(
                    &self,
                    cx: &mut #internal::LowerContext<'_, T>,
                    ty: #internal::InterfaceType,
                    dst: &mut core::mem::MaybeUninit<Self::Lower>,
                ) -> #internal::anyhow::Result<()> {
                    #extract_ty
                    match self {
                        #lowers
                    }
                }

                #[inline]
                fn linear_lower_to_memory<T>(
                    &self,
                    cx: &mut #internal::LowerContext<'_, T>,
                    ty: #internal::InterfaceType,
                    mut offset: usize
                ) -> #internal::anyhow::Result<()> {
                    #extract_ty
                    debug_assert!(offset % (<Self as #wt::component::ComponentType>::ALIGN32 as usize) == 0);
                    match self {
                        #stores
                    }
                }
            }
        };

        Ok(expanded)
    }

    fn expand_enum(
        &self,
        name: &syn::Ident,
        discriminant_size: DiscriminantSize,
        _cases: &[VariantCase],
        wt: &syn::Path,
    ) -> Result<TokenStream> {
        let internal = quote!(#wt::component::__internal);

        let extract_ty = quote! {
            let ty = match ty {
                #internal::InterfaceType::Enum(i) => &cx.types[i],
                _ => #internal::bad_type_info(),
            };
        };

        let (size, ty) = match discriminant_size {
            DiscriminantSize::Size1 => (1, quote!(u8)),
            DiscriminantSize::Size2 => (2, quote!(u16)),
            DiscriminantSize::Size4 => (4, quote!(u32)),
        };
        let size = proc_macro2::Literal::usize_unsuffixed(size);

        let expanded = quote! {
            unsafe impl #wt::component::Lower for #name {
                #[inline]
                fn linear_lower_to_flat<T>(
                    &self,
                    cx: &mut #internal::LowerContext<'_, T>,
                    ty: #internal::InterfaceType,
                    dst: &mut core::mem::MaybeUninit<Self::Lower>,
                ) -> #internal::anyhow::Result<()> {
                    #extract_ty
                    #internal::map_maybe_uninit!(dst.tag)
                        .write(#wt::ValRaw::u32(*self as u32));
                    Ok(())
                }

                #[inline]
                fn linear_lower_to_memory<T>(
                    &self,
                    cx: &mut #internal::LowerContext<'_, T>,
                    ty: #internal::InterfaceType,
                    mut offset: usize
                ) -> #internal::anyhow::Result<()> {
                    #extract_ty
                    debug_assert!(offset % (<Self as #wt::component::ComponentType>::ALIGN32 as usize) == 0);
                    let discrim = *self as #ty;
                    *cx.get::<#size>(offset) = discrim.to_le_bytes();
                    Ok(())
                }
            }
        };

        Ok(expanded)
    }
}

pub struct ComponentTypeExpander;

impl Expander for ComponentTypeExpander {
    fn expand_record(
        &self,
        name: &syn::Ident,
        generics: &syn::Generics,
        fields: &[&syn::Field],
        wt: &syn::Path,
    ) -> Result<TokenStream> {
        expand_record_for_component_type(
            name,
            generics,
            fields,
            quote!(typecheck_record),
            fields
                .iter()
                .map(
                    |syn::Field {
                         attrs, ident, ty, ..
                     }| {
                        let name = find_rename(attrs)?.unwrap_or_else(|| {
                            let ident = ident.as_ref().unwrap();
                            syn::LitStr::new(&ident.to_string(), ident.span())
                        });

                        Ok(quote!((#name, <#ty as #wt::component::ComponentType>::typecheck),))
                    },
                )
                .collect::<Result<_>>()?,
            wt,
        )
    }

    fn expand_variant(
        &self,
        name: &syn::Ident,
        generics: &syn::Generics,
        _discriminant_size: DiscriminantSize,
        cases: &[VariantCase],
        wt: &syn::Path,
    ) -> Result<TokenStream> {
        let internal = quote!(#wt::component::__internal);

        let mut case_names_and_checks = TokenStream::new();
        let mut lower_payload_generic_params = TokenStream::new();
        let mut lower_payload_generic_args = TokenStream::new();
        let mut lower_payload_case_declarations = TokenStream::new();
        let mut lower_generic_args = TokenStream::new();
        let mut abi_list = TokenStream::new();
        let mut unique_types = HashSet::new();

        for (index, VariantCase { attrs, ident, ty }) in cases.iter().enumerate() {
            let rename = find_rename(attrs)?;

            let name = rename.unwrap_or_else(|| syn::LitStr::new(&ident.to_string(), ident.span()));

            if let Some(ty) = ty {
                abi_list.extend(quote!(Some(<#ty as #wt::component::ComponentType>::ABI),));

                case_names_and_checks.extend(
                    quote!((#name, Some(<#ty as #wt::component::ComponentType>::typecheck)),),
                );

                let generic = format_ident!("T{}", index);

                lower_payload_generic_params.extend(quote!(#generic: Copy,));
                lower_payload_generic_args.extend(quote!(#generic,));
                lower_payload_case_declarations.extend(quote!(#ident: #generic,));
                lower_generic_args.extend(quote!(<#ty as #wt::component::ComponentType>::Lower,));

                unique_types.insert(ty);
            } else {
                abi_list.extend(quote!(None,));
                case_names_and_checks.extend(quote!((#name, None),));
                lower_payload_case_declarations.extend(quote!(#ident: [#wt::ValRaw; 0],));
            }
        }

        let generics = add_trait_bounds(generics, parse_quote!(#wt::component::ComponentType));
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let lower = format_ident!("Lower{}", name);
        let lower_payload = format_ident!("LowerPayload{}", name);

        // You may wonder why we make the types of all the fields of the #lower struct and #lower_payload union
        // generic.  This is to work around a [normalization bug in
        // rustc](https://github.com/rust-lang/rust/issues/90903) such that the compiler does not understand that
        // e.g. `<i32 as ComponentType>::Lower` is `Copy` despite the bound specified in `ComponentType`'s
        // definition.
        //
        // See also the comment in `Self::expand_record` above for another reason why we do this.

        let expanded = quote! {
            #[doc(hidden)]
            #[derive(Clone, Copy)]
            #[repr(C)]
            pub struct #lower<#lower_payload_generic_params> {
                tag: #wt::ValRaw,
                payload: #lower_payload<#lower_payload_generic_args>
            }

            #[doc(hidden)]
            #[allow(non_snake_case)]
            #[derive(Clone, Copy)]
            #[repr(C)]
            union #lower_payload<#lower_payload_generic_params> {
                #lower_payload_case_declarations
            }

            unsafe impl #impl_generics #wt::component::ComponentType for #name #ty_generics #where_clause {
                type Lower = #lower<#lower_generic_args>;

                #[inline]
                fn typecheck(
                    ty: &#internal::InterfaceType,
                    types: &#internal::InstanceType<'_>,
                ) -> #internal::anyhow::Result<()> {
                    #internal::typecheck_variant(ty, types, &[#case_names_and_checks])
                }

                const ABI: #internal::CanonicalAbiInfo =
                    #internal::CanonicalAbiInfo::variant_static(&[#abi_list]);
            }

            unsafe impl #impl_generics #internal::ComponentVariant for #name #ty_generics #where_clause {
                const CASES: &'static [Option<#internal::CanonicalAbiInfo>] = &[#abi_list];
            }
        };

        Ok(quote!(const _: () = { #expanded };))
    }

    fn expand_enum(
        &self,
        name: &syn::Ident,
        _discriminant_size: DiscriminantSize,
        cases: &[VariantCase],
        wt: &syn::Path,
    ) -> Result<TokenStream> {
        let internal = quote!(#wt::component::__internal);

        let mut case_names = TokenStream::new();
        let mut abi_list = TokenStream::new();

        for VariantCase { attrs, ident, ty } in cases.iter() {
            let rename = find_rename(attrs)?;

            let name = rename.unwrap_or_else(|| syn::LitStr::new(&ident.to_string(), ident.span()));

            if ty.is_some() {
                return Err(Error::new(
                    ident.span(),
                    "payloads are not permitted for `enum` cases",
                ));
            }
            abi_list.extend(quote!(None,));
            case_names.extend(quote!(#name,));
        }

        let lower = format_ident!("Lower{}", name);

        let cases_len = cases.len();
        let expanded = quote! {
            #[doc(hidden)]
            #[derive(Clone, Copy)]
            #[repr(C)]
            pub struct #lower {
                tag: #wt::ValRaw,
            }

            unsafe impl #wt::component::ComponentType for #name {
                type Lower = #lower;

                #[inline]
                fn typecheck(
                    ty: &#internal::InterfaceType,
                    types: &#internal::InstanceType<'_>,
                ) -> #internal::anyhow::Result<()> {
                    #internal::typecheck_enum(ty, types, &[#case_names])
                }

                const ABI: #internal::CanonicalAbiInfo =
                    #internal::CanonicalAbiInfo::enum_(#cases_len);
            }

            unsafe impl #internal::ComponentVariant for #name {
                const CASES: &'static [Option<#internal::CanonicalAbiInfo>] = &[#abi_list];
            }
        };

        Ok(quote!(const _: () = { #expanded };))
    }
}

#[derive(Debug)]
struct Flag {
    rename: Option<String>,
    name: String,
}

impl Parse for Flag {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = syn::Attribute::parse_outer(input)?;

        let rename = find_rename(&attributes)?.map(|literal| literal.value());

        input.parse::<Token![const]>()?;
        let name = input.parse::<syn::Ident>()?.to_string();

        Ok(Self { rename, name })
    }
}

#[derive(Debug)]
pub struct Flags {
    name: String,
    flags: Vec<Flag>,
}

impl Parse for Flags {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();

        let content;
        braced!(content in input);

        let flags = content
            .parse_terminated(Flag::parse, Token![;])?
            .into_iter()
            .collect();

        Ok(Self { name, flags })
    }
}

pub fn expand_flags(flags: &Flags) -> Result<TokenStream> {
    let wt = default_wasmtime_crate();
    let size = FlagsSize::from_count(flags.flags.len());

    let ty;
    let eq;

    let count = flags.flags.len();

    match size {
        FlagsSize::Size0 => {
            ty = quote!(());
            eq = quote!(true);
        }
        FlagsSize::Size1 => {
            ty = quote!(u8);

            eq = if count == 8 {
                quote!(self.__inner0.eq(&rhs.__inner0))
            } else {
                let mask = !(0xFF_u8 << count);

                quote!((self.__inner0 & #mask).eq(&(rhs.__inner0 & #mask)))
            };
        }
        FlagsSize::Size2 => {
            ty = quote!(u16);

            eq = if count == 16 {
                quote!(self.__inner0.eq(&rhs.__inner0))
            } else {
                let mask = !(0xFFFF_u16 << count);

                quote!((self.__inner0 & #mask).eq(&(rhs.__inner0 & #mask)))
            };
        }
        FlagsSize::Size4Plus(n) => {
            ty = quote!(u32);

            let comparisons = (0..(n - 1))
                .map(|index| {
                    let field = format_ident!("__inner{}", index);

                    quote!(self.#field.eq(&rhs.#field) &&)
                })
                .collect::<TokenStream>();

            let field = format_ident!("__inner{}", n - 1);

            eq = if count % 32 == 0 {
                quote!(#comparisons self.#field.eq(&rhs.#field))
            } else {
                let mask = !(0xFFFF_FFFF_u32 << (count % 32));

                quote!(#comparisons (self.#field & #mask).eq(&(rhs.#field & #mask)))
            }
        }
    }

    let count;
    let mut as_array;
    let mut bitor;
    let mut bitor_assign;
    let mut bitand;
    let mut bitand_assign;
    let mut bitxor;
    let mut bitxor_assign;
    let mut not;

    match size {
        FlagsSize::Size0 => {
            count = 0;
            as_array = quote!([]);
            bitor = quote!(Self {});
            bitor_assign = quote!();
            bitand = quote!(Self {});
            bitand_assign = quote!();
            bitxor = quote!(Self {});
            bitxor_assign = quote!();
            not = quote!(Self {});
        }
        FlagsSize::Size1 | FlagsSize::Size2 => {
            count = 1;
            as_array = quote!([self.__inner0 as u32]);
            bitor = quote!(Self {
                __inner0: self.__inner0.bitor(rhs.__inner0)
            });
            bitor_assign = quote!(self.__inner0.bitor_assign(rhs.__inner0));
            bitand = quote!(Self {
                __inner0: self.__inner0.bitand(rhs.__inner0)
            });
            bitand_assign = quote!(self.__inner0.bitand_assign(rhs.__inner0));
            bitxor = quote!(Self {
                __inner0: self.__inner0.bitxor(rhs.__inner0)
            });
            bitxor_assign = quote!(self.__inner0.bitxor_assign(rhs.__inner0));
            not = quote!(Self {
                __inner0: self.__inner0.not()
            });
        }
        FlagsSize::Size4Plus(n) => {
            count = usize::from(n);
            as_array = TokenStream::new();
            bitor = TokenStream::new();
            bitor_assign = TokenStream::new();
            bitand = TokenStream::new();
            bitand_assign = TokenStream::new();
            bitxor = TokenStream::new();
            bitxor_assign = TokenStream::new();
            not = TokenStream::new();

            for index in 0..n {
                let field = format_ident!("__inner{}", index);

                as_array.extend(quote!(self.#field,));
                bitor.extend(quote!(#field: self.#field.bitor(rhs.#field),));
                bitor_assign.extend(quote!(self.#field.bitor_assign(rhs.#field);));
                bitand.extend(quote!(#field: self.#field.bitand(rhs.#field),));
                bitand_assign.extend(quote!(self.#field.bitand_assign(rhs.#field);));
                bitxor.extend(quote!(#field: self.#field.bitxor(rhs.#field),));
                bitxor_assign.extend(quote!(self.#field.bitxor_assign(rhs.#field);));
                not.extend(quote!(#field: self.#field.not(),));
            }

            as_array = quote!([#as_array]);
            bitor = quote!(Self { #bitor });
            bitand = quote!(Self { #bitand });
            bitxor = quote!(Self { #bitxor });
            not = quote!(Self { #not });
        }
    };

    let name = format_ident!("{}", flags.name);

    let mut constants = TokenStream::new();
    let mut rust_names = TokenStream::new();
    let mut component_names = TokenStream::new();

    for (index, Flag { name, rename }) in flags.flags.iter().enumerate() {
        rust_names.extend(quote!(#name,));

        let component_name = rename.as_ref().unwrap_or(name);
        component_names.extend(quote!(#component_name,));

        let fields = match size {
            FlagsSize::Size0 => quote!(),
            FlagsSize::Size1 => {
                let init = 1_u8 << index;
                quote!(__inner0: #init)
            }
            FlagsSize::Size2 => {
                let init = 1_u16 << index;
                quote!(__inner0: #init)
            }
            FlagsSize::Size4Plus(n) => (0..n)
                .map(|i| {
                    let field = format_ident!("__inner{}", i);

                    let init = if index / 32 == usize::from(i) {
                        1_u32 << (index % 32)
                    } else {
                        0
                    };

                    quote!(#field: #init,)
                })
                .collect::<TokenStream>(),
        };

        let name = format_ident!("{}", name);

        constants.extend(quote!(pub const #name: Self = Self { #fields };));
    }

    let generics = syn::Generics {
        lt_token: None,
        params: Punctuated::new(),
        gt_token: None,
        where_clause: None,
    };

    let fields = {
        let ty = syn::parse2::<syn::Type>(ty.clone())?;

        (0..count)
            .map(|index| syn::Field {
                attrs: Vec::new(),
                vis: syn::Visibility::Inherited,
                ident: Some(format_ident!("__inner{}", index)),
                colon_token: None,
                ty: ty.clone(),
                mutability: syn::FieldMutability::None,
            })
            .collect::<Vec<_>>()
    };

    let fields = fields.iter().collect::<Vec<_>>();

    let component_type_impl = expand_record_for_component_type(
        &name,
        &generics,
        &fields,
        quote!(typecheck_flags),
        component_names,
        &wt,
    )?;

    let internal = quote!(#wt::component::__internal);

    let field_names = fields
        .iter()
        .map(|syn::Field { ident, .. }| ident)
        .collect::<Vec<_>>();

    let fields = fields
        .iter()
        .map(|syn::Field { ident, .. }| quote!(#[doc(hidden)] #ident: #ty,))
        .collect::<TokenStream>();

    let (field_interface_type, field_size) = match size {
        FlagsSize::Size0 => (quote!(NOT USED), 0usize),
        FlagsSize::Size1 => (quote!(#internal::InterfaceType::U8), 1),
        FlagsSize::Size2 => (quote!(#internal::InterfaceType::U16), 2),
        FlagsSize::Size4Plus(_) => (quote!(#internal::InterfaceType::U32), 4),
    };

    let expanded = quote! {
        #[derive(Copy, Clone, Default)]
        pub struct #name { #fields }

        impl #name {
            #constants

            pub fn as_array(&self) -> [u32; #count] {
                #as_array
            }

            pub fn empty() -> Self {
                Self::default()
            }

            pub fn all() -> Self {
                use core::ops::Not;
                Self::default().not()
            }

            pub fn contains(&self, other: Self) -> bool {
                *self & other == other
            }

            pub fn intersects(&self, other: Self) -> bool {
                *self & other != Self::empty()
            }
        }

        impl core::cmp::PartialEq for #name {
            fn eq(&self, rhs: &#name) -> bool {
                #eq
            }
        }

        impl core::cmp::Eq for #name { }

        impl core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                #internal::format_flags(&self.as_array(), &[#rust_names], f)
            }
        }

        impl core::ops::BitOr for #name {
            type Output = #name;

            fn bitor(self, rhs: #name) -> #name {
                #bitor
            }
        }

        impl core::ops::BitOrAssign for #name {
            fn bitor_assign(&mut self, rhs: #name) {
                #bitor_assign
            }
        }

        impl core::ops::BitAnd for #name {
            type Output = #name;

            fn bitand(self, rhs: #name) -> #name {
                #bitand
            }
        }

        impl core::ops::BitAndAssign for #name {
            fn bitand_assign(&mut self, rhs: #name) {
                #bitand_assign
            }
        }

        impl core::ops::BitXor for #name {
            type Output = #name;

            fn bitxor(self, rhs: #name) -> #name {
                #bitxor
            }
        }

        impl core::ops::BitXorAssign for #name {
            fn bitxor_assign(&mut self, rhs: #name) {
                #bitxor_assign
            }
        }

        impl core::ops::Not for #name {
            type Output = #name;

            fn not(self) -> #name {
                #not
            }
        }

        #component_type_impl

        unsafe impl #wt::component::Lower for #name {
            fn linear_lower_to_flat<T>(
                &self,
                cx: &mut #internal::LowerContext<'_, T>,
                _ty: #internal::InterfaceType,
                dst: &mut core::mem::MaybeUninit<Self::Lower>,
            ) -> #internal::anyhow::Result<()> {
                #(
                    self.#field_names.linear_lower_to_flat(
                        cx,
                        #field_interface_type,
                        #internal::map_maybe_uninit!(dst.#field_names),
                    )?;
                )*
                Ok(())
            }

            fn linear_lower_to_memory<T>(
                &self,
                cx: &mut #internal::LowerContext<'_, T>,
                _ty: #internal::InterfaceType,
                mut offset: usize
            ) -> #internal::anyhow::Result<()> {
                debug_assert!(offset % (<Self as #wt::component::ComponentType>::ALIGN32 as usize) == 0);
                #(
                    self.#field_names.linear_lower_to_memory(
                        cx,
                        #field_interface_type,
                        offset,
                    )?;
                    offset += core::mem::size_of_val(&self.#field_names);
                )*
                Ok(())
            }
        }

        unsafe impl #wt::component::Lift for #name {
            fn linear_lift_from_flat(
                cx: &mut #internal::LiftContext<'_>,
                _ty: #internal::InterfaceType,
                src: &Self::Lower,
            ) -> #internal::anyhow::Result<Self> {
                Ok(Self {
                    #(
                        #field_names: #wt::component::Lift::linear_lift_from_flat(
                            cx,
                            #field_interface_type,
                            &src.#field_names,
                        )?,
                    )*
                })
            }

            fn linear_lift_from_memory(
                cx: &mut #internal::LiftContext<'_>,
                _ty: #internal::InterfaceType,
                bytes: &[u8],
            ) -> #internal::anyhow::Result<Self> {
                debug_assert!(
                    (bytes.as_ptr() as usize)
                        % (<Self as #wt::component::ComponentType>::ALIGN32 as usize)
                        == 0
                );
                #(
                    let (field, bytes) = bytes.split_at(#field_size);
                    let #field_names = #wt::component::Lift::linear_lift_from_memory(
                        cx,
                        #field_interface_type,
                        field,
                    )?;
                )*
                Ok(Self { #(#field_names,)* })
            }
        }
    };

    Ok(expanded)
}
