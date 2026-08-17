extern crate proc_macro;

mod covariance_detection;
mod generate;
mod info_structures;
mod parse;
mod utils;

use crate::{
    generate::{
        constructor::create_builder_and_constructor, derives::create_derives,
        into_heads::make_into_heads, struc::create_internal_struct_def,
        summon_checker::generate_checker_summoner,
        try_constructor::create_try_builder_and_constructor, type_asserts::make_type_asserts,
        with::make_with_all_function, with_each::make_with_functions,
    },
    info_structures::Options,
    parse::parse_struct,
};
use generate::{
    drop::create_drop_impl, struc::create_actual_struct_def, with_mut::make_with_all_mut_function,
};
use heck::ToSnakeCase;
use info_structures::BuilderType;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree;
use quote::{format_ident, quote};
use syn::{Error, ItemStruct};

fn self_referencing_impl(
    original_struct_def: &ItemStruct,
    options: Options,
) -> Result<TokenStream, Error> {
    let struct_name = &original_struct_def.ident;
    let mod_name = format_ident!("ouroboros_impl_{}", struct_name.to_string().to_snake_case());
    let visibility = &original_struct_def.vis;

    let info = parse_struct(original_struct_def)?;

    let actual_struct_def = create_actual_struct_def(&info)?;
    let internal_struct_def = create_internal_struct_def(&info)?;
    let drop_impl = create_drop_impl(&info)?;

    let borrowchk_summoner = generate_checker_summoner(&info)?;

    let (builder_struct_name, builder_def, constructor_def) =
        create_builder_and_constructor(&info, options, BuilderType::Sync)?;
    let (async_builder_struct_name, async_builder_def, async_constructor_def) =
        create_builder_and_constructor(&info, options, BuilderType::Async)?;
    let (async_send_builder_struct_name, async_send_builder_def, async_send_constructor_def) =
        create_builder_and_constructor(&info, options, BuilderType::AsyncSend)?;
    let (try_builder_struct_name, try_builder_def, try_constructor_def) =
        create_try_builder_and_constructor(&info, options, BuilderType::Sync)?;
    let (async_try_builder_struct_name, async_try_builder_def, async_try_constructor_def) =
        create_try_builder_and_constructor(&info, options, BuilderType::Async)?;
    let (
        async_send_try_builder_struct_name,
        async_send_try_builder_def,
        async_send_try_constructor_def,
    ) = create_try_builder_and_constructor(&info, options, BuilderType::AsyncSend)?;

    let (with_defs, with_errors) = make_with_functions(&info, options);
    let with_errors = with_errors
        .into_iter()
        .map(|err| err.emit_as_item_tokens())
        .collect::<Vec<_>>();
    let (with_all_struct_def, with_all_fn_def) = make_with_all_function(&info, options)?;
    let (with_all_mut_struct_def, with_all_mut_fn_def) =
        make_with_all_mut_function(&info, options)?;
    let (heads_struct_def, into_heads_fn) = make_into_heads(&info, options);

    let impls = create_derives(&info)?;

    // These check that types like Box, Arc, and Rc refer to those types in the std lib and have not
    // been overridden.
    let type_asserts_def = make_type_asserts(&info);

    let extra_visibility = if options.do_pub_extras {
        visibility.clone()
    } else {
        syn::Visibility::Inherited
    };

    let generic_params = info.generic_params();
    let generic_args = info.generic_arguments();
    let generic_where = &info.generics.where_clause;
    Ok(TokenStream::from(quote! {
        #[doc="Encapsulates implementation details for a self-referencing struct. This module is only visible when using --document-private-items."]
        mod #mod_name {
            use super::*;
            #[doc="The self-referencing struct."]
            #actual_struct_def
            #internal_struct_def
            #drop_impl
            #borrowchk_summoner
            #builder_def
            #async_builder_def
            #async_send_builder_def
            #try_builder_def
            #async_try_builder_def
            #async_send_try_builder_def
            #with_all_struct_def
            #with_all_mut_struct_def
            #(#with_errors)*
            #heads_struct_def
            #impls
            impl <#generic_params> #struct_name <#(#generic_args),*> #generic_where {
                #constructor_def
                #async_constructor_def
                #async_send_constructor_def
                #try_constructor_def
                #async_try_constructor_def
                #async_send_try_constructor_def
                #(#with_defs)*
                #with_all_fn_def
                #with_all_mut_fn_def
                #into_heads_fn
            }
            #type_asserts_def
        }
        #visibility use #mod_name :: #struct_name;
        #extra_visibility use #mod_name :: #builder_struct_name;
        #extra_visibility use #mod_name :: #async_builder_struct_name;
        #extra_visibility use #mod_name :: #async_send_builder_struct_name;
        #extra_visibility use #mod_name :: #try_builder_struct_name;
        #extra_visibility use #mod_name :: #async_try_builder_struct_name;
        #extra_visibility use #mod_name :: #async_send_try_builder_struct_name;
    }))
}

#[proc_macro_attribute]
pub fn self_referencing(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut options = Options {
        do_no_doc: false,
        do_pub_extras: false,
    };
    let mut expecting_comma = false;
    for token in <TokenStream as std::convert::Into<TokenStream2>>::into(attr).into_iter() {
        if let TokenTree::Ident(ident) = &token {
            if expecting_comma {
                return Error::new(token.span(), "Unexpected identifier, expected comma.")
                    .to_compile_error()
                    .into();
            }
            match &ident.to_string()[..] {
                "no_doc" => options.do_no_doc = true,
                "pub_extras" => options.do_pub_extras = true,
                _ => {
                    return Error::new_spanned(
                        ident,
                        "Unknown identifier, expected 'no_doc' or 'pub_extras'.",
                    )
                    .to_compile_error()
                    .into()
                }
            }
            expecting_comma = true;
        } else if let TokenTree::Punct(punct) = &token {
            if !expecting_comma {
                return Error::new(token.span(), "Unexpected punctuation, expected identifier.")
                    .to_compile_error()
                    .into();
            }
            if punct.as_char() != ',' {
                return Error::new(token.span(), "Unknown punctuation, expected comma.")
                    .to_compile_error()
                    .into();
            }
            expecting_comma = false;
        } else {
            return Error::new(token.span(), "Unknown syntax, expected identifier.")
                .to_compile_error()
                .into();
        }
    }
    let original_struct_def: ItemStruct = syn::parse_macro_input!(item);
    match self_referencing_impl(&original_struct_def, options) {
        Ok(content) => content,
        Err(err) => err.to_compile_error().into(),
    }
}
