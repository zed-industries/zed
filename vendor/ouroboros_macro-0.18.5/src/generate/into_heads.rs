use proc_macro2::TokenStream;
use quote::quote;

use crate::info_structures::{Options, StructInfo};

/// Returns the Heads struct and a function to convert the original struct into a Heads instance.
pub fn make_into_heads(info: &StructInfo, options: Options) -> (TokenStream, TokenStream) {
    let visibility = if options.do_pub_extras {
        info.vis.clone()
    } else {
        syn::parse_quote! { pub(super) }
    };
    let mut code = Vec::new();
    let mut field_initializers = Vec::new();
    let mut head_fields = Vec::new();
    let internal_struct = &info.internal_ident;
    // Drop everything in the reverse order of what it was declared in. Fields that come later
    // are only dependent on fields that came before them.
    for field in info.fields.iter().rev() {
        let field_name = &field.name;
        if field.self_referencing {
            // Heads are fields that do not borrow anything.
            code.push(quote! { ::core::mem::drop(this.#field_name); });
        } else {
            code.push(quote! { let #field_name = this.#field_name; });
            if field.is_borrowed() {
                field_initializers
                    .push(quote! { #field_name: ::ouroboros::macro_help::unbox(#field_name) });
            } else {
                field_initializers.push(quote! { #field_name });
            }
            let field_type = &field.typ;
            head_fields.push(quote! { #visibility #field_name: #field_type });
        }
    }
    for (ty, ident) in info.generic_consumers() {
        head_fields.push(quote! { #ident: ::core::marker::PhantomData<#ty> });
        field_initializers.push(quote! { #ident: ::core::marker::PhantomData });
    }
    let documentation = format!(
        concat!(
            "A struct which contains only the ",
            "[head fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) of [`{0}`]({0})."
        ),
        info.ident.to_string()
    );
    let generic_params = info.generic_params();
    let generic_where = &info.generics.where_clause;
    let heads_struct_def = quote! {
        #[doc=#documentation]
        #visibility struct Heads <#generic_params> #generic_where {
            #(#head_fields),*
        }
    };
    let documentation = concat!(
        "This function drops all internally referencing fields and returns only the ",
        "[head fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) of this struct."
    ).to_owned();

    let documentation = if !options.do_no_doc {
        quote! {
            #[doc=#documentation]
        }
    } else {
        quote! { #[doc(hidden)] }
    };

    let generic_args = info.generic_arguments();
    let into_heads_fn = quote! {
        #documentation
        #[allow(clippy::drop_ref)]
        #[allow(clippy::drop_copy)]
        #[allow(clippy::drop_non_drop)]
        #visibility fn into_heads(self) -> Heads<#(#generic_args),*> {
            let this_ptr = &self as *const _;
            let this: #internal_struct<#(#generic_args),*> = unsafe { ::core::mem::transmute_copy(&*this_ptr) };
            ::core::mem::forget(self);
            #(#code)*
            Heads {
                #(#field_initializers),*
            }
        }
    };
    (heads_struct_def, into_heads_fn)
}
