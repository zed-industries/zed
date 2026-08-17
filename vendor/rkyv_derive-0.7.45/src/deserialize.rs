use crate::{
    attributes::{parse_attributes, Attributes},
    util::add_bounds,
    with::{make_with_ty, with_inner},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, Data, DeriveInput, Error, Fields,
    Generics, Ident, Index,
};

pub fn derive(input: DeriveInput) -> Result<TokenStream, Error> {
    let attributes = parse_attributes(&input)?;
    derive_deserialize_impl(input, &attributes)
}

fn derive_deserialize_impl(
    mut input: DeriveInput,
    attributes: &Attributes,
) -> Result<TokenStream, Error> {
    let where_clause = input.generics.make_where_clause();
    if let Some(ref bounds) = attributes.archive_bound {
        add_bounds(bounds, where_clause)?;
    }
    if let Some(ref bounds) = attributes.deserialize_bound {
        add_bounds(bounds, where_clause)?;
    }

    let mut impl_input_params = Punctuated::default();
    impl_input_params.push(parse_quote! { __D: Fallible + ?Sized });
    for param in input.generics.params.iter() {
        impl_input_params.push(param.clone());
    }
    let impl_input_generics = Generics {
        lt_token: Some(Default::default()),
        params: impl_input_params,
        gt_token: Some(Default::default()),
        where_clause: input.generics.where_clause.clone(),
    };

    let default_rkyv_path = parse_quote! { ::rkyv };
    let rkyv_path = attributes.rkyv_path.as_ref().unwrap_or(&default_rkyv_path);
    let with_ty = make_with_ty(rkyv_path);

    let name = &input.ident;
    let (impl_generics, _, _) = impl_input_generics.split_for_impl();
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();
    let where_clause = where_clause.unwrap();

    let deserialize_impl = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let mut deserialize_where = where_clause.clone();
                for field in fields
                    .named
                    .iter()
                    .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("omit_bounds")))
                {
                    let ty = with_ty(field)?;
                    deserialize_where
                        .predicates
                        .push(parse_quote! { #ty: Archive });
                    deserialize_where
                        .predicates
                        .push(parse_quote! { Archived<#ty>: Deserialize<#ty, __D> });
                }

                let deserialize_fields = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = with_ty(f).unwrap();
                    let value = with_inner(
                        f,
                        parse_quote! {
                            Deserialize::<#ty, __D>::deserialize(
                                &self.#name,
                                deserializer,
                            )?
                        },
                    )
                    .unwrap();
                    quote! { #name: #value }
                });

                quote! {
                    impl #impl_generics Deserialize<#name #ty_generics, __D> for Archived<#name #ty_generics> #deserialize_where {
                        #[inline]
                        fn deserialize(&self, deserializer: &mut __D) -> ::core::result::Result<#name #ty_generics, __D::Error> {
                            Ok(#name {
                                #(#deserialize_fields,)*
                            })
                        }
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let mut deserialize_where = where_clause.clone();
                for field in fields
                    .unnamed
                    .iter()
                    .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("omit_bounds")))
                {
                    let ty = with_ty(field)?;
                    deserialize_where
                        .predicates
                        .push(parse_quote! { #ty: Archive });
                    deserialize_where
                        .predicates
                        .push(parse_quote! { Archived<#ty>: Deserialize<#ty, __D> });
                }

                let deserialize_fields = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    let ty = with_ty(f).unwrap();
                    let value = with_inner(
                        f,
                        parse_quote! {
                            Deserialize::<#ty, __D>::deserialize(
                                &self.#index,
                                deserializer,
                            )?
                        },
                    )
                    .unwrap();
                    quote! { #value }
                });

                quote! {
                    impl #impl_generics Deserialize<#name #ty_generics, __D> for Archived<#name #ty_generics> #deserialize_where {
                        #[inline]
                        fn deserialize(&self, deserializer: &mut __D) -> ::core::result::Result<#name #ty_generics, __D::Error> {
                            Ok(#name(
                                #(#deserialize_fields,)*
                            ))
                        }
                    }
                }
            }
            Fields::Unit => quote! {
                impl #impl_generics Deserialize<#name #ty_generics, __D> for Archived<#name #ty_generics> #where_clause {
                    #[inline]
                    fn deserialize(&self, _: &mut __D) -> ::core::result::Result<#name #ty_generics, __D::Error> {
                        Ok(#name)
                    }
                }
            },
        },
        Data::Enum(ref data) => {
            let mut deserialize_where = where_clause.clone();
            for variant in data.variants.iter() {
                match variant.fields {
                    Fields::Named(ref fields) => {
                        for field in fields
                            .named
                            .iter()
                            .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("omit_bounds")))
                        {
                            let ty = with_ty(field)?;
                            deserialize_where
                                .predicates
                                .push(parse_quote! { #ty: Archive });
                            deserialize_where
                                .predicates
                                .push(parse_quote! { Archived<#ty>: Deserialize<#ty, __D> });
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        for field in fields
                            .unnamed
                            .iter()
                            .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("omit_bounds")))
                        {
                            let ty = with_ty(field)?;
                            deserialize_where
                                .predicates
                                .push(parse_quote! { #ty: Archive });
                            deserialize_where
                                .predicates
                                .push(parse_quote! { Archived<#ty>: Deserialize<#ty, __D> });
                        }
                    }
                    Fields::Unit => (),
                }
            }

            let deserialize_variants = data.variants.iter().map(|v| {
                let variant = &v.ident;
                match v.fields {
                    Fields::Named(ref fields) => {
                        let bindings = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            quote! { #name }
                        });
                        let fields = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            let ty = with_ty(f).unwrap();
                            let value = with_inner(
                                f,
                                parse_quote! {
                                    Deserialize::<#ty, __D>::deserialize(
                                        #name,
                                        deserializer,
                                    )?
                                },
                            )
                            .unwrap();
                            quote! { #name: #value }
                        });
                        quote! {
                            Self::#variant { #(#bindings,)* } => #name::#variant { #(#fields,)* }
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        let bindings = fields.unnamed.iter().enumerate().map(|(i, f)| {
                            let name = Ident::new(&format!("_{}", i), f.span());
                            quote! { #name }
                        });
                        let fields = fields.unnamed.iter().enumerate().map(|(i, f)| {
                            let binding = Ident::new(&format!("_{}", i), f.span());
                            let ty = with_ty(f).unwrap();
                            let value = with_inner(
                                f,
                                parse_quote! {
                                    Deserialize::<#ty, __D>::deserialize(
                                        #binding,
                                        deserializer,
                                    )?
                                },
                            )
                            .unwrap();
                            quote! { #value }
                        });
                        quote! {
                            Self::#variant( #(#bindings,)* ) => #name::#variant(#(#fields,)*)
                        }
                    }
                    Fields::Unit => {
                        quote! { Self::#variant => #name::#variant }
                    }
                }
            });

            quote! {
                impl #impl_generics Deserialize<#name #ty_generics, __D> for Archived<#name #ty_generics> #deserialize_where {
                    #[inline]
                    fn deserialize(&self, deserializer: &mut __D) -> ::core::result::Result<#name #ty_generics, __D::Error> {
                        Ok(match self {
                            #(#deserialize_variants,)*
                        })
                    }
                }
            }
        }
        Data::Union(_) => {
            return Err(Error::new_spanned(
                input,
                "Deserialize cannot be derived for unions",
            ))
        }
    };

    Ok(quote! {
        #[automatically_derived]
        const _: () = {
            use #rkyv_path::{Archive, Archived, Deserialize, Fallible};
            #deserialize_impl
        };
    })
}
