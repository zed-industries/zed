use crate::{
    info_structures::{FieldType, Options, StructInfo},
    utils::{replace_this_with_lifetime, uses_this_lifetime},
};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Error, Lifetime, WhereClause};

pub fn make_with_all_mut_function(
    info: &StructInfo,
    options: Options,
) -> Result<(TokenStream, TokenStream), Error> {
    let visibility = if options.do_pub_extras {
        info.vis.clone()
    } else {
        syn::parse_quote! { pub(super) }
    };
    let mut mut_fields = Vec::new();
    let mut mut_field_assignments = Vec::new();
    let mut lifetime_idents = Vec::new();
    // I don't think the reverse is necessary but it does make the expanded code more uniform.
    for (index, field) in info.fields.iter().rev().enumerate() {
        let field_name = &field.name;
        let original_field_type = &field.typ;
        let lifetime = format_ident!("this{}", index);
        let field_type = replace_this_with_lifetime(quote! { #original_field_type }, lifetime.clone());
        if field.field_type == FieldType::Tail {
            mut_fields.push(quote! { #visibility #field_name: &'outer_borrow mut #field_type });
            mut_field_assignments.push(quote! { #field_name: &mut this.#field_name });
            if uses_this_lifetime(quote! { #original_field_type }) {
                lifetime_idents.push(lifetime.clone());
            }
        } else if field.field_type == FieldType::Borrowed {
            let ass = quote! { #field_name: unsafe {
                ::ouroboros::macro_help::change_lifetime(
                    &*this.#field_name
                )
            } };
            let lt = Lifetime::new(&format!("'{}", lifetime), Span::call_site());
            mut_fields.push(quote! { #visibility #field_name: &#lt #field_type });
            mut_field_assignments.push(ass);
            lifetime_idents.push(lifetime.clone());
        } else if field.field_type == FieldType::BorrowedMut {
            // Add nothing because we cannot borrow something that has already been mutably
            // borrowed.
        }
    }

    for (ty, ident) in info.generic_consumers() {
        mut_fields.push(quote! { #ident: ::core::marker::PhantomData<#ty> });
        mut_field_assignments.push(quote! { #ident: ::core::marker::PhantomData });
    }

    let mut new_generic_params = info.generic_params().clone();
    for lt in &lifetime_idents {
        let lt = Lifetime::new(&format!("'{}", lt), Span::call_site());
        new_generic_params.insert(0, syn::parse_quote! { #lt });
    }
    new_generic_params.insert(0, syn::parse_quote! { 'outer_borrow });
    let mut new_generic_args = info.generic_arguments();
    let mut lifetimes = Vec::new();
    for lt in &lifetime_idents {
        let lt = Lifetime::new(&format!("'{}", lt), Span::call_site());
        lifetimes.push(lt.clone());
        new_generic_args.insert(0, quote! { #lt });
    }
    new_generic_args.insert(0, quote! { 'outer_borrow });

    let mut_struct_documentation = format!(
        concat!(
            "A struct for holding mutable references to all ",
            "[tail fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) in an instance of ",
            "[`{0}`]({0})."
        ),
        info.ident.to_string()
    );
    let fake_lifetime = Lifetime::new(&format!("'{}", info.fake_lifetime()), Span::call_site());
    let mut generic_where = if let Some(clause) = &info.generics.where_clause {
        clause.clone()
    } else {
        syn::parse_quote! { where }
    };
    for lt in &lifetime_idents {
        let lt = Lifetime::new(&format!("'{}", lt), Span::call_site());
        let extra: WhereClause = syn::parse_quote! { where #fake_lifetime: #lt };
        generic_where
            .predicates
            .extend(extra.predicates.into_iter());
    }
    for idents in lifetime_idents.windows(2) {
        let lt = Lifetime::new(&format!("'{}", idents[1]), Span::call_site());
        let outlives = Lifetime::new(&format!("'{}", idents[0]), Span::call_site());
        let extra: WhereClause = syn::parse_quote! { where #lt: #outlives };
        generic_where
            .predicates
            .extend(extra.predicates.into_iter());
    }
    let struct_defs = quote! {
        #[doc=#mut_struct_documentation]
        #visibility struct BorrowedMutFields <#new_generic_params> #generic_where { #(#mut_fields),* }
    };
    let borrowed_mut_fields_type = quote! { BorrowedMutFields<#(#new_generic_args),*> };
    let mut_documentation = concat!(
        "This method provides mutable references to all ",
        "[tail fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions).",
    );
    let mut_documentation = if !options.do_no_doc {
        quote! {
            #[doc=#mut_documentation]
        }
    } else {
        quote! { #[doc(hidden)] }
    };
    let fn_defs = quote! {
        #mut_documentation
        #[inline(always)]
        #visibility fn with_mut <'outer_borrow, ReturnType>(
            &'outer_borrow mut self,
            user: impl for<#(#lifetimes),*> ::core::ops::FnOnce(#borrowed_mut_fields_type) -> ReturnType
        ) -> ReturnType {
            let this = unsafe { self.actual_data.assume_init_mut() };
            user(BorrowedMutFields {
                #(#mut_field_assignments),*
            })
        }
    };
    Ok((struct_defs, fn_defs))
}
