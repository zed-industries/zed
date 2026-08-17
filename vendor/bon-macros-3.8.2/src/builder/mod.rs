mod builder_gen;

pub(crate) mod item_impl;

mod item_fn;
mod item_struct;

use crate::normalization::{ExpandCfg, Expansion, GenericsNamespace};
use crate::util;
use crate::util::prelude::*;
use builder_gen::TopLevelConfig;
use syn::parse::Parser;
use syn::visit::Visit;

pub(crate) fn generate_from_derive(item: TokenStream) -> TokenStream {
    try_generate_from_derive(item).unwrap_or_else(Error::write_errors)
}

fn try_generate_from_derive(item: TokenStream) -> Result<TokenStream> {
    match syn::parse2(item)? {
        syn::Item::Struct(item_struct) => item_struct::generate(item_struct),
        _ => bail!(
            &Span::call_site(),
            "only `struct` items are supported by the `#[derive(bon::Builder)]` attribute"
        ),
    }
}

pub(crate) fn generate_from_attr(params: TokenStream, item: TokenStream) -> TokenStream {
    crate::error::handle_errors(item.clone(), || {
        try_generate_from_attr(params.clone(), item)
    })
    .unwrap_or_else(|fallback| [generate_completion_triggers(params), fallback].concat())
}

fn try_generate_from_attr(params: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let item = syn::parse2(item)?;

    let ctx = ExpandCfg {
        current_macro: format_ident!("builder"),
        config: params,
        item,
    };

    let input = match ctx.expand_cfg()? {
        Expansion::Expanded(input) => input,
        Expansion::Recurse(output) => return Ok(output),
    };

    let main_output = match *input.item {
        syn::Item::Fn(item_fn) => {
            let mut namespace = GenericsNamespace::default();

            namespace.visit_token_stream(input.config.clone());
            namespace.visit_item_fn(&item_fn);

            let config = TopLevelConfig::parse_for_fn(&item_fn, Some(input.config.clone()))?;

            item_fn::generate(config, item_fn, &namespace)?
        }
        syn::Item::Struct(struct_item) => {
            bail!(
                &struct_item.struct_token,
                "to generate a builder for a struct, use `#[derive(bon::Builder)]` instead; \
                 `#[bon::builder]` syntax is supported only for functions starting with bon v3"
            )
        }
        _ => bail!(
            &Span::call_site(),
            "only `fn` items are supported by the `#[bon::builder]` attribute"
        ),
    };

    let output = [generate_completion_triggers(input.config), main_output].concat();

    Ok(output)
}

fn generate_completion_triggers(params: TokenStream) -> TokenStream {
    let meta = util::ide::parse_comma_separated_meta
        .parse2(params)
        .unwrap_or_default();

    util::ide::generate_completion_triggers(meta)
}
