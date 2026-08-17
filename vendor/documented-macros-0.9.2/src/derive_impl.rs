//! Shared implementation for non-opt and opt variants of the derive macros.
//!
//! All functions in this module use the dependency injection pattern to
//! generate the correct trait implementation for both macro variants.

use convert_case::Casing;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    spanned::Spanned, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Expr, Fields,
    Ident,
};

#[cfg(feature = "customise")]
use crate::config::customise_core::get_customisations_from_attrs;
use crate::{
    config::{
        derive::DeriveConfig,
        derive_fields::{DeriveFieldsConfig, RenameMode},
    },
    util::{crate_module_path, get_docs},
};

/// The type of the doc comment.
#[derive(Copy, Clone, Debug)]
pub enum DocType {
    /// &'static str
    Str,
    /// Option<&'static str>
    OptStr,
}
impl ToTokens for DocType {
    fn to_tokens(&self, ts: &mut TokenStream) {
        let tokens = match self {
            Self::Str => quote! { &'static str },
            Self::OptStr => quote! { Option<&'static str> },
        };
        ts.append_all([tokens]);
    }
}
impl DocType {
    /// Get the closure that determines how to handle optional docs.
    /// The closure takes three arguments:
    ///
    /// 1. The optional doc comments on an item
    /// 2. A default value optionally set by the user
    /// 3. The token span on which to report any errors
    ///
    /// And fallibly returns the tokenised doc comments.
    #[allow(clippy::type_complexity)]
    fn docs_handler_opt<S>(
        &self,
    ) -> Box<dyn Fn(Option<String>, Option<Expr>, S) -> syn::Result<TokenStream>>
    where
        S: ToTokens,
    {
        match self {
            Self::Str => Box::new(
                |docs_opt, default_opt, span| match (docs_opt, default_opt) {
                    (Some(docs), _) => Ok(quote! { #docs }),
                    (None, Some(default)) => Ok(quote! { #default }),
                    (None, None) => Err(Error::new_spanned(span, "Missing doc comments")),
                },
            ),
            Self::OptStr => Box::new(|docs_opt, default_opt, _span| {
                let tokens = match (docs_opt, default_opt) {
                    (Some(docs), _) => quote! { Some(#docs) },
                    (None, Some(default)) => quote! { #default },
                    (None, None) => quote! { None },
                };
                Ok(tokens)
            }),
        }
    }

    /// Get the trait identifier, given a prefix.
    fn trait_ident_for(&self, prefix: &str) -> Ident {
        let name = match self {
            Self::Str => prefix.to_string(),
            Self::OptStr => format!("{prefix}Opt"),
        };
        Ident::new(&name, Span::call_site())
    }
}

/// Shared implementation of `Documented` & `DocumentedOpt`.
pub fn documented_impl(input: DeriveInput, docs_ty: DocType) -> syn::Result<TokenStream> {
    let trait_ident = docs_ty.trait_ident_for("Documented");
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    #[cfg(not(feature = "customise"))]
    let config = DeriveConfig::default();
    #[cfg(feature = "customise")]
    let config = get_customisations_from_attrs(&input.attrs, "documented")
        .map(|c| DeriveConfig::default().with_customisations(c))?;

    let docs = get_docs(&input.attrs, config.trim)
        .and_then(|docs_opt| docs_ty.docs_handler_opt()(docs_opt, config.default_value, &input))?;

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics documented::#trait_ident for #ident #ty_generics #where_clause {
            const DOCS: #docs_ty = #docs;
        }
    })
}

/// Shared implementation of `DocumentedFields` & `DocumentedFieldsOpt`.
pub fn documented_fields_impl(input: DeriveInput, docs_ty: DocType) -> syn::Result<TokenStream> {
    let trait_ident = docs_ty.trait_ident_for("DocumentedFields");
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // `#[documented_fields(...)]` on container type
    #[cfg(not(feature = "customise"))]
    let base_config = DeriveFieldsConfig::default();
    #[cfg(feature = "customise")]
    let base_config = get_customisations_from_attrs(&input.attrs, "documented_fields")
        .map(|c| DeriveFieldsConfig::default().with_base_customisations(c))?;

    let fields_attrs: Vec<_> = match input.data.clone() {
        Data::Enum(DataEnum { variants, .. }) => variants
            .into_iter()
            .map(|v| (v.to_token_stream(), Some(v.ident), v.attrs))
            .collect(),
        Data::Struct(DataStruct { fields, .. }) => fields
            .into_iter()
            .map(|f| (f.to_token_stream(), f.ident, f.attrs))
            .collect(),
        Data::Union(DataUnion { fields, .. }) => fields
            .named
            .into_iter()
            .map(|f| (f.to_token_stream(), f.ident, f.attrs))
            .collect(),
    };

    let (field_names, field_docs) = fields_attrs
        .into_iter()
        .map(|(span, ident, attrs)| {
            #[cfg(not(feature = "customise"))]
            let config = base_config.clone();
            #[cfg(feature = "customise")]
            let config = get_customisations_from_attrs(&attrs, "documented_fields")
                .map(|c| base_config.with_field_customisations(c))?;
            let name = match config.rename_mode {
                None => ident.map(|ident| ident.to_string()),
                Some(RenameMode::ToCase(case)) => {
                    ident.map(|ident| ident.to_string().to_case(case))
                }
                Some(RenameMode::Custom(name)) => Some(name),
            };
            get_docs(&attrs, config.trim)
                .and_then(|docs_opt| {
                    docs_ty.docs_handler_opt()(docs_opt, config.default_value, span)
                })
                .map(|docs| (name, docs))
        })
        .collect::<syn::Result<Vec<_>>>()?
        .into_iter()
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let (field_names, phf_match_arms) = field_names
        .into_iter()
        .enumerate()
        .filter_map(|(i, field)| field.map(|field| (i, field.as_str().to_owned())))
        .map(|(i, name)| (name.clone(), quote! { #name => #i, }))
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let documented_module_path = crate_module_path();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics documented::#trait_ident for #ident #ty_generics #where_clause {
            const FIELD_NAMES: &'static [&'static str] = &[#(#field_names),*];
            const FIELD_DOCS: &'static [#docs_ty] = &[#(#field_docs),*];

            fn __documented_get_index<__Documented_T: AsRef<str>>(field_name: __Documented_T) -> Option<usize> {
                use #documented_module_path::_private_phf_reexport_for_macro as phf;

                static PHF: phf::Map<&'static str, usize> = phf::phf_map! {
                    #(#phf_match_arms)*
                };
                PHF.get(field_name.as_ref()).copied()
            }
        }
    })
}

/// Shared implementation of `DocumentedVariants` & `DocumentedVariantsOpt`.
pub fn documented_variants_impl(input: DeriveInput, docs_ty: DocType) -> syn::Result<TokenStream> {
    let trait_ident = docs_ty.trait_ident_for("DocumentedVariants");
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // `#[documented_variants(...)]` on container type
    #[cfg(not(feature = "customise"))]
    let base_config = DeriveConfig::default();
    #[cfg(feature = "customise")]
    let base_config = get_customisations_from_attrs(&input.attrs, "documented_variants")
        .map(|c| DeriveConfig::default().with_customisations(c))?;

    let variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => Ok(variants),
        Data::Struct(DataStruct { struct_token, .. }) => Err(struct_token.span()),
        Data::Union(DataUnion { union_token, .. }) => Err(union_token.span()),
    }
    .map_err(|span| {
        Error::new(
            span,
            "DocumentedVariants can only be used on enums.\n\
            For structs and unions, use DocumentedFields instead.",
        )
    })?;

    let variants_docs = variants
        .into_iter()
        .map(|v| {
            #[cfg(not(feature = "customise"))]
            let config = base_config.clone();
            #[cfg(feature = "customise")]
            let config = get_customisations_from_attrs(&v.attrs, "documented_variants")
                .map(|c| base_config.with_customisations(c))?;
            get_docs(&v.attrs, config.trim)
                .and_then(|docs_opt| docs_ty.docs_handler_opt()(docs_opt, config.default_value, &v))
                .map(|docs| (v.ident, v.fields, docs))
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let match_arms = variants_docs
        .into_iter()
        .map(|(ident, fields, docs)| {
            let pat = match fields {
                Fields::Unit => quote! { Self::#ident },
                Fields::Unnamed(_) => quote! { Self::#ident(..) },
                Fields::Named(_) => quote! { Self::#ident{..} },
            };
            quote! { #pat => #docs, }
        })
        .collect::<Vec<_>>();

    // IDEA: I'd like to use phf here, but it doesn't seem to be possible at the moment,
    // because there isn't a way to get an enum's discriminant at compile time
    // if this becomes possible in the future, or alternatively you have a good workaround,
    // improvement suggestions are more than welcomed
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics documented::#trait_ident for #ident #ty_generics #where_clause {
            fn get_variant_docs(&self) -> #docs_ty {
                match self {
                    #(#match_arms)*
                }
            }
        }
    })
}
