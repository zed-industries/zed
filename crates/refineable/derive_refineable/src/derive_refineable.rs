use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    DeriveInput, Field, FieldsNamed, PredicateType, TraitBound, Type, TypeParamBound, WhereClause,
    WherePredicate, parse_macro_input, parse_quote,
};

#[proc_macro_derive(Refineable, attributes(refineable))]
pub fn derive_refineable(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        attrs,
        ..
    } = parse_macro_input!(input);

    let refineable_attr = attrs.iter().find(|attr| attr.path.is_ident("refineable"));

    let mut impl_debug_on_refinement = false;
    let mut refinement_traits_to_derive = vec![];

    if let Some(refineable_attr) = refineable_attr {
        if let Ok(syn::Meta::List(meta_list)) = refineable_attr.parse_meta() {
            for nested in meta_list.nested {
                let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested else {
                    continue;
                };

                if path.is_ident("Debug") {
                    impl_debug_on_refinement = true;
                } else {
                    refinement_traits_to_derive.push(path);
                }
            }
        }
    }

    let refinement_ident = format_ident!("{}Refinement", ident);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named.into_iter().collect::<Vec<Field>>(),
        _ => panic!("This derive macro only supports structs with named fields"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let field_visibilities: Vec<_> = fields.iter().map(|f| &f.vis).collect();
    let wrapped_types: Vec<_> = fields.iter().map(|f| get_wrapper_type(f, &f.ty)).collect();

    // Create trait bound that each wrapped type must implement Clone // & Default
    let type_param_bounds: Vec<_> = wrapped_types
        .iter()
        .map(|ty| {
            WherePredicate::Type(PredicateType {
                lifetimes: None,
                bounded_ty: ty.clone(),
                colon_token: Default::default(),
                bounds: {
                    let mut punctuated = syn::punctuated::Punctuated::new();
                    punctuated.push_value(TypeParamBound::Trait(TraitBound {
                        paren_token: None,
                        modifier: syn::TraitBoundModifier::None,
                        lifetimes: None,
                        path: parse_quote!(Clone),
                    }));

                    punctuated
                },
            })
        })
        .collect();

    // Append to where_clause or create a new one if it doesn't exist
    let where_clause = match where_clause.cloned() {
        Some(mut where_clause) => {
            where_clause.predicates.extend(type_param_bounds);
            where_clause.clone()
        }
        None => WhereClause {
            where_token: Default::default(),
            predicates: type_param_bounds.into_iter().collect(),
        },
    };

    let refineable_refine_assignments: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let name = &field.ident;
            let is_refineable = is_refineable_field(field);
            let is_optional = is_optional_field(field);

            if is_refineable {
                quote! {
                    self.#name.refine(&refinement.#name);
                }
            } else if is_optional {
                quote! {
                    if let Some(value) = &refinement.#name {
                        self.#name = Some(value.clone());
                    }
                }
            } else {
                quote! {
                    if let Some(value) = &refinement.#name {
                        self.#name = value.clone();
                    }
                }
            }
        })
        .collect();

    let refineable_refined_assignments: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let name = &field.ident;
            let is_refineable = is_refineable_field(field);
            let is_optional = is_optional_field(field);

            if is_refineable {
                quote! {
                    self.#name = self.#name.refined(refinement.#name);
                }
            } else if is_optional {
                quote! {
                    if let Some(value) = refinement.#name {
                        self.#name = Some(value);
                    }
                }
            } else {
                quote! {
                    if let Some(value) = refinement.#name {
                        self.#name = value;
                    }
                }
            }
        })
        .collect();

    let refinement_refine_assignments: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let name = &field.ident;
            let is_refineable = is_refineable_field(field);

            if is_refineable {
                quote! {
                    self.#name.refine(&refinement.#name);
                }
            } else {
                quote! {
                    if let Some(value) = &refinement.#name {
                        self.#name = Some(value.clone());
                    }
                }
            }
        })
        .collect();

    let refinement_refined_assignments: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let name = &field.ident;
            let is_refineable = is_refineable_field(field);

            if is_refineable {
                quote! {
                    self.#name = self.#name.refined(refinement.#name);
                }
            } else {
                quote! {
                    if let Some(value) = refinement.#name {
                        self.#name = Some(value);
                    }
                }
            }
        })
        .collect();

    let from_refinement_assignments: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let name = &field.ident;
            let is_refineable = is_refineable_field(field);
            let is_optional = is_optional_field(field);

            if is_refineable {
                quote! {
                    #name: value.#name.into(),
                }
            } else if is_optional {
                quote! {
                    #name: value.#name.map(|v| v.into()),
                }
            } else {
                quote! {
                    #name: value.#name.map(|v| v.into()).unwrap_or_default(),
                }
            }
        })
        .collect();

    let debug_impl = if impl_debug_on_refinement {
        let refinement_field_debugs: Vec<TokenStream2> = fields
            .iter()
            .map(|field| {
                let name = &field.ident;
                quote! {
                    if self.#name.is_some() {
                        debug_struct.field(stringify!(#name), &self.#name);
                    } else {
                        all_some = false;
                    }
                }
            })
            .collect();

        quote! {
            impl #impl_generics std::fmt::Debug for #refinement_ident #ty_generics
                #where_clause
            {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let mut debug_struct = f.debug_struct(stringify!(#refinement_ident));
                    let mut all_some = true;
                    #( #refinement_field_debugs )*
                    if all_some {
                        debug_struct.finish()
                    } else {
                        debug_struct.finish_non_exhaustive()
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let mut derive_stream = quote! {};
    for trait_to_derive in refinement_traits_to_derive {
        derive_stream.extend(quote! { #[derive(#trait_to_derive)] })
    }

    let r#gen = quote! {
        /// A refinable version of [`#ident`], see that documentation for details.
        #[derive(Clone)]
        #derive_stream
        pub struct #refinement_ident #impl_generics {
            #(
                #[allow(missing_docs)]
                #field_visibilities #field_names: #wrapped_types
            ),*
        }

        impl #impl_generics Refineable for #ident #ty_generics
            #where_clause
        {
            type Refinement = #refinement_ident #ty_generics;

            fn refine(&mut self, refinement: &Self::Refinement) {
                #( #refineable_refine_assignments )*
            }

            fn refined(mut self, refinement: Self::Refinement) -> Self {
                #( #refineable_refined_assignments )*
                self
            }
        }

        impl #impl_generics Refineable for #refinement_ident #ty_generics
            #where_clause
        {
            type Refinement = #refinement_ident #ty_generics;

            fn refine(&mut self, refinement: &Self::Refinement) {
                #( #refinement_refine_assignments )*
            }

            fn refined(mut self, refinement: Self::Refinement) -> Self {
                #( #refinement_refined_assignments )*
                self
            }
        }

        impl #impl_generics From<#refinement_ident #ty_generics> for #ident #ty_generics
            #where_clause
        {
            fn from(value: #refinement_ident #ty_generics) -> Self {
                Self {
                    #( #from_refinement_assignments )*
                }
            }
        }

        impl #impl_generics ::core::default::Default for #refinement_ident #ty_generics
            #where_clause
        {
            fn default() -> Self {
                #refinement_ident {
                    #( #field_names: Default::default() ),*
                }
            }
        }

        impl #impl_generics #refinement_ident #ty_generics
            #where_clause
        {
            /// Returns `true` if all fields are `Some`
            pub fn is_some(&self) -> bool {
                #(
                    if self.#field_names.is_some() {
                        return true;
                    }
                )*
                false
            }
        }

        #debug_impl
    };
    r#gen.into()
}

fn is_refineable_field(f: &Field) -> bool {
    f.attrs.iter().any(|attr| attr.path.is_ident("refineable"))
}

fn is_optional_field(f: &Field) -> bool {
    if let Type::Path(typepath) = &f.ty {
        if typepath.qself.is_none() {
            let segments = &typepath.path.segments;
            if segments.len() == 1 && segments.iter().any(|s| s.ident == "Option") {
                return true;
            }
        }
    }
    false
}

fn get_wrapper_type(field: &Field, ty: &Type) -> syn::Type {
    if is_refineable_field(field) {
        let struct_name = if let Type::Path(tp) = ty {
            tp.path.segments.last().unwrap().ident.clone()
        } else {
            panic!("Expected struct type for a refineable field");
        };
        let refinement_struct_name = format_ident!("{}Refinement", struct_name);
        let generics = if let Type::Path(tp) = ty {
            &tp.path.segments.last().unwrap().arguments
        } else {
            &syn::PathArguments::None
        };
        parse_quote!(#refinement_struct_name #generics)
    } else if is_optional_field(field) {
        ty.clone()
    } else {
        parse_quote!(Option<#ty>)
    }
}
