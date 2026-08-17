use crate::info_structures::{FieldType, Options, StructInfo};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Lifetime, WhereClause};

pub fn make_with_all_function(
    info: &StructInfo,
    options: Options,
) -> Result<(TokenStream, TokenStream), Error> {
    let visibility = if options.do_pub_extras {
        info.vis.clone()
    } else {
        syn::parse_quote! { pub(super) }
    };
    let mut fields = Vec::new();
    let mut field_assignments = Vec::new();
    // I don't think the reverse is necessary but it does make the expanded code more uniform.
    for field in info.fields.iter().rev() {
        let field_name = &field.name;
        let field_type = &field.typ;
        if field.field_type == FieldType::Tail {
            fields.push(quote! { #visibility #field_name: &'outer_borrow #field_type });
            field_assignments.push(quote! { #field_name: &this.#field_name });
        } else if field.field_type == FieldType::Borrowed {
            let ass = quote! { #field_name: unsafe {
                ::ouroboros::macro_help::change_lifetime(
                    &*this.#field_name
                )
            } };
            fields.push(quote! { #visibility #field_name: &'this #field_type });
            field_assignments.push(ass.clone());
        } else if field.field_type == FieldType::BorrowedMut {
            // Add nothing because we cannot borrow something that has already been mutably
            // borrowed.
        }
    }

    for (ty, ident) in info.generic_consumers() {
        fields.push(quote! { #ident: ::core::marker::PhantomData<#ty> });
        field_assignments.push(quote! { #ident: ::core::marker::PhantomData });
    }
    let new_generic_params = info.borrowed_generic_params();
    let new_generic_args = info.borrowed_generic_arguments();

    let struct_documentation = format!(
        concat!(
            "A struct for holding immutable references to all ",
            "[tail and immutably borrowed fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) in an instance of ",
            "[`{0}`]({0})."
        ),
        info.ident.to_string()
    );
    let ltname = format!("'{}", info.fake_lifetime());
    let lifetime = Lifetime::new(&ltname, Span::call_site());
    let generic_where = if let Some(clause) = &info.generics.where_clause {
        let mut clause = clause.clone();
        let extra: WhereClause = syn::parse_quote! { where #lifetime: 'this };
        clause
            .predicates
            .push(extra.predicates.first().unwrap().clone());
        let extra: WhereClause = syn::parse_quote! { where 'this: 'outer_borrow };
        clause
            .predicates
            .push(extra.predicates.first().unwrap().clone());
        clause
    } else {
        syn::parse_quote! { where #lifetime: 'this, 'this: 'outer_borrow }
    };
    let struct_defs = quote! {
        #[doc=#struct_documentation]
        #visibility struct BorrowedFields #new_generic_params #generic_where { #(#fields),* }
    };
    let borrowed_fields_type = quote! { BorrowedFields<#(#new_generic_args),*> };
    let documentation = concat!(
        "This method provides immutable references to all ",
        "[tail and immutably borrowed fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions).",
    );
    let documentation = if !options.do_no_doc {
        quote! {
            #[doc=#documentation]
        }
    } else {
        quote! { #[doc(hidden)] }
    };
    let fn_defs = quote! {
        #documentation
        #[inline(always)]
        #visibility fn with <'outer_borrow, ReturnType>(
            &'outer_borrow self,
            user: impl for<'this> ::core::ops::FnOnce(#borrowed_fields_type) -> ReturnType
        ) -> ReturnType {
            let this = unsafe { self.actual_data.assume_init_ref() };
            user(BorrowedFields {
                #(#field_assignments),*
            })
        }
    };
    Ok((struct_defs, fn_defs))
}
