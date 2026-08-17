use crate::builder;
use crate::normalization::{ExpandCfg, Expansion};
use crate::util::prelude::*;
use darling::ast::NestedMeta;
use darling::FromMeta;

pub(crate) fn generate(params: TokenStream, item: TokenStream) -> TokenStream {
    crate::error::handle_errors(item.clone(), || try_generate(params, item))
        .unwrap_or_else(std::convert::identity)
}

pub(crate) fn try_generate(params: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let item = syn::parse2(item)?;

    let ctx = ExpandCfg {
        current_macro: format_ident!("bon"),
        config: params,
        item,
    };

    let input = match ctx.expand_cfg()? {
        Expansion::Expanded(input) => input,
        Expansion::Recurse(output) => return Ok(output),
    };

    let params = NestedMeta::parse_meta_list(input.config)?;
    let params = FromMeta::from_list(&params)?;

    match *input.item {
        syn::Item::Impl(item_impl) => builder::item_impl::generate(params, item_impl),
        _ => bail!(
            &input.item,
            "`#[bon]` attribute is expected to be placed on an `impl` block \
             but it was placed on other syntax instead"
        ),
    }
}
