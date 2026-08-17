use crate::{
    attributes::{parse_attributes, Attributes},
    util::{add_bounds, strip_raw},
    with::{make_with_cast, make_with_ty},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, Data, DeriveInput, Error, Fields,
    Generics, Ident, Index,
};

pub fn derive(input: DeriveInput) -> Result<TokenStream, Error> {
    let attributes = parse_attributes(&input)?;
    derive_serialize_impl(input, &attributes)
}

fn derive_serialize_impl(
    mut input: DeriveInput,
    attributes: &Attributes,
) -> Result<TokenStream, Error> {
    let where_clause = input.generics.make_where_clause();
    if let Some(ref bounds) = attributes.archive_bound {
        add_bounds(bounds, where_clause)?;
    }
    if let Some(ref bounds) = attributes.serialize_bound {
        add_bounds(bounds, where_clause)?;
    }

    let mut impl_input_params = Punctuated::default();
    impl_input_params.push(parse_quote! { __S: Fallible + ?Sized });
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
    let with_cast = make_with_cast(rkyv_path);

    let name = &input.ident;
    let (impl_generics, _, _) = impl_input_generics.split_for_impl();
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();
    let where_clause = where_clause.unwrap();

    let resolver = attributes.resolver.as_ref().map_or_else(
        || Ident::new(&format!("{}Resolver", strip_raw(name)), name.span()),
        |value| value.clone(),
    );

    let serialize_impl = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let mut serialize_where = where_clause.clone();
                for field in fields
                    .named
                    .iter()
                    .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("omit_bounds")))
                {
                    let ty = with_ty(field)?;
                    serialize_where
                        .predicates
                        .push(parse_quote! { #ty: Serialize<__S> });
                }

                let resolver_values = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let field = with_cast(f, parse_quote! { &self.#name }).unwrap();
                    quote! { #name: Serialize::<__S>::serialize(#field, serializer)? }
                });

                quote! {
                    impl #impl_generics Serialize<__S> for #name #ty_generics #serialize_where {
                        #[inline]
                        fn serialize(&self, serializer: &mut __S) -> ::core::result::Result<Self::Resolver, __S::Error> {
                            Ok(#resolver {
                                #(#resolver_values,)*
                            })
                        }
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let mut serialize_where = where_clause.clone();
                for field in fields
                    .unnamed
                    .iter()
                    .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("omit_bounds")))
                {
                    let ty = with_ty(field)?;
                    serialize_where
                        .predicates
                        .push(parse_quote! { #ty: Serialize<__S> });
                }

                let resolver_values = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    let field = with_cast(f, parse_quote! { &self.#index }).unwrap();
                    quote! { Serialize::<__S>::serialize(#field, serializer)? }
                });

                quote! {
                    impl #impl_generics Serialize<__S> for #name #ty_generics #serialize_where {
                        #[inline]
                        fn serialize(&self, serializer: &mut __S) -> ::core::result::Result<Self::Resolver, __S::Error> {
                            Ok(#resolver(
                                #(#resolver_values,)*
                            ))
                        }
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    impl #impl_generics Serialize<__S> for #name #ty_generics #where_clause {
                        #[inline]
                        fn serialize(&self, serializer: &mut __S) -> ::core::result::Result<Self::Resolver, __S::Error> {
                            Ok(#resolver)
                        }
                    }
                }
            }
        },
        Data::Enum(ref data) => {
            let mut serialize_where = where_clause.clone();
            for variant in data.variants.iter() {
                match variant.fields {
                    Fields::Named(ref fields) => {
                        for field in fields
                            .named
                            .iter()
                            .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("omit_bounds")))
                        {
                            let ty = with_ty(field)?;
                            serialize_where
                                .predicates
                                .push(parse_quote! { #ty: Serialize<__S> });
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        for field in fields
                            .unnamed
                            .iter()
                            .filter(|f| !f.attrs.iter().any(|a| a.path.is_ident("omit_bounds")))
                        {
                            let ty = with_ty(field)?;
                            serialize_where
                                .predicates
                                .push(parse_quote! { #ty: Serialize<__S> });
                        }
                    }
                    Fields::Unit => (),
                }
            }

            let serialize_arms = data.variants.iter().map(|v| {
                let variant = &v.ident;
                match v.fields {
                    Fields::Named(ref fields) => {
                        let bindings = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            quote! { #name }
                        });
                        let fields = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            let field = with_cast(f, parse_quote! { #name }).unwrap();
                            quote! {
                                #name: Serialize::<__S>::serialize(#field, serializer)?
                            }
                        });
                        quote! {
                            Self::#variant { #(#bindings,)* } => #resolver::#variant {
                                #(#fields,)*
                            }
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        let bindings = fields.unnamed.iter().enumerate().map(|(i, f)| {
                            let name = Ident::new(&format!("_{}", i), f.span());
                            quote! { #name }
                        });
                        let fields = fields.unnamed.iter().enumerate().map(|(i, f)| {
                            let binding = Ident::new(&format!("_{}", i), f.span());
                            let field = with_cast(f, parse_quote! { #binding }).unwrap();
                            quote! {
                                Serialize::<__S>::serialize(#field, serializer)?
                            }
                        });
                        quote! {
                            Self::#variant( #(#bindings,)* ) => #resolver::#variant(#(#fields,)*)
                        }
                    }
                    Fields::Unit => {
                        quote! { Self::#variant => #resolver::#variant }
                    }
                }
            });

            quote! {
                impl #impl_generics Serialize<__S> for #name #ty_generics #serialize_where {
                    #[inline]
                    fn serialize(&self, serializer: &mut __S) -> ::core::result::Result<<Self as Archive>::Resolver, __S::Error> {
                        Ok(match self {
                            #(#serialize_arms,)*
                        })
                    }
                }
            }
        }
        Data::Union(_) => {
            return Err(Error::new_spanned(
                input,
                "Serialize cannot be derived for unions",
            ))
        }
    };

    Ok(quote! {
        #[automatically_derived]
        const _: () = {
            use #rkyv_path::{Archive, Fallible, Serialize};
            #serialize_impl
        };
    })
}
