use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, parse_quote, DeriveInput, Field, FieldsNamed, PredicateType, TraitBound,
    Type, TypeParamBound, WhereClause, WherePredicate,
};

#[proc_macro_derive(Refineable, attributes(refineable))]
pub fn derive_refineable(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input);

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

    // Create trait bound that each wrapped type must implement Clone & Default
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
                    punctuated.push_punct(syn::token::Add::default());
                    punctuated.push_value(TypeParamBound::Trait(TraitBound {
                        paren_token: None,
                        modifier: syn::TraitBoundModifier::None,
                        lifetimes: None,
                        path: parse_quote!(Default),
                    }));
                    punctuated
                },
            })
        })
        .collect();

    // Append to where_clause or create a new one if it doesn't exist
    let where_clause = match where_clause.cloned() {
        Some(mut where_clause) => {
            where_clause
                .predicates
                .extend(type_param_bounds.into_iter());
            where_clause.clone()
        }
        None => WhereClause {
            where_token: Default::default(),
            predicates: type_param_bounds.into_iter().collect(),
        },
    };

    let field_assignments: Vec<TokenStream2> = fields
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
                    if let Some(ref value) = &refinement.#name {
                        self.#name = Some(value.clone());
                    }
                }
            } else {
                quote! {
                    if let Some(ref value) = &refinement.#name {
                        self.#name = value.clone();
                    }
                }
            }
        })
        .collect();

    let refinement_field_assignments: Vec<TokenStream2> = fields
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
                    if let Some(ref value) = &refinement.#name {
                        self.#name = Some(value.clone());
                    }
                }
            }
        })
        .collect();

    let gen = quote! {
        #[derive(Default, Clone)]
        pub struct #refinement_ident #impl_generics {
            #( #field_visibilities #field_names: #wrapped_types ),*
        }

        impl #impl_generics Refineable for #ident #ty_generics
            #where_clause
        {
            type Refinement = #refinement_ident #ty_generics;

            fn refine(&mut self, refinement: &Self::Refinement) {
                #( #field_assignments )*
            }
        }

        impl #impl_generics Refineable for #refinement_ident #ty_generics
            #where_clause
        {
            type Refinement = #refinement_ident #ty_generics;

            fn refine(&mut self, refinement: &Self::Refinement) {
                #( #refinement_field_assignments )*
            }
        }
    };

    println!("{}", gen);

    gen.into()
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
