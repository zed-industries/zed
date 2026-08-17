use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

use crate::info_structures::{ArgType, BuilderType, StructInfo};

pub fn generate_checker_summoner(info: &StructInfo) -> Result<TokenStream, Error> {
    let mut code: Vec<TokenStream> = Vec::new();
    let mut params: Vec<TokenStream> = Vec::new();
    let mut value_consumers: Vec<TokenStream> = Vec::new();
    let mut template_consumers: Vec<TokenStream> = Vec::new();
    for field in &info.fields {
        let field_name = &field.name;

        let arg_type = field.make_constructor_arg_type(info, BuilderType::Sync)?;
        if let ArgType::Plain(plain_type) = arg_type {
            // No fancy builder function, we can just move the value directly into the struct.
            params.push(quote! { #field_name: #plain_type });
        } else if let ArgType::TraitBound(bound_type) = arg_type {
            // Trait bounds are much trickier. We need a special syntax to accept them in the
            // constructor, and generic parameters need to be added to the builder struct to make
            // it work.
            let builder_name = field.builder_name();
            params.push(quote! { #builder_name : impl #bound_type });
            let mut builder_args = Vec::new();
            for (_, borrow) in field.borrows.iter().enumerate() {
                let borrowed_name = &info.fields[borrow.index].name;
                if borrow.mutable {
                    builder_args.push(quote! { &mut #borrowed_name });
                } else {
                    builder_args.push(quote! { &#borrowed_name });
                }
            }
            code.push(quote! { let #field_name = #builder_name (#(#builder_args),*); });
        }
        if field.is_mutably_borrowed() {
            code.push(quote! { let mut #field_name = #field_name; });
        } else {
            code.push(quote! { let #field_name = #field_name; });
            value_consumers.push(quote! { #field_name: &#field_name });
        }
    }
    for (_ty, ident) in info.generic_consumers() {
        template_consumers.push(quote! { #ident: ::core::marker::PhantomData });
    }
    let generic_params = info.generic_params();
    let where_clause = &info.generics.where_clause;
    let borrowed_generic_params_inferred = info.borrowed_generic_params_inferred();
    Ok(quote! {
        fn check_if_okay_according_to_checkers<#generic_params>(
            #(#params,)*
        )
        #where_clause
        {
            #(#code;)*
            BorrowedFields::#borrowed_generic_params_inferred {
                #(#value_consumers,)*
                #(#template_consumers,)*
            };
        }
    })
}
