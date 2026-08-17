use crate::syntax::cfg::CfgExpr;
use crate::syntax::namespace::Namespace;
use quote::quote;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{
    braced, token, Abi, Attribute, ForeignItem, Ident, Item as RustItem, ItemEnum, ItemImpl,
    ItemStruct, ItemUse, LitStr, Token, Visibility,
};

pub(crate) struct Module {
    #[expect(dead_code)]
    pub cfg: CfgExpr,
    pub namespace: Namespace,
    pub attrs: Vec<Attribute>,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub vis: Visibility,
    pub unsafety: Option<Token![unsafe]>,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub mod_token: Token![mod],
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub ident: Ident,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub brace_token: token::Brace,
    pub content: Vec<Item>,
}

pub(crate) enum Item {
    Struct(ItemStruct),
    Enum(ItemEnum),
    ForeignMod(ItemForeignMod),
    Use(ItemUse),
    Impl(ItemImpl),
    Other(RustItem),
}

pub(crate) struct ItemForeignMod {
    pub attrs: Vec<Attribute>,
    pub unsafety: Option<Token![unsafe]>,
    pub abi: Abi,
    #[expect(dead_code)]
    pub brace_token: token::Brace,
    pub items: Vec<ForeignItem>,
}

impl Parse for Module {
    fn parse(input: ParseStream) -> Result<Self> {
        let cfg = CfgExpr::Unconditional;
        let namespace = Namespace::ROOT;
        let mut attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let unsafety: Option<Token![unsafe]> = input.parse()?;
        let mod_token: Token![mod] = input.parse()?;
        let ident: Ident = input.parse()?;

        let semi: Option<Token![;]> = input.parse()?;
        if let Some(semi) = semi {
            let span = quote!(#vis #mod_token #semi);
            return Err(Error::new_spanned(
                span,
                "#[cxx::bridge] module must have inline contents",
            ));
        }

        let content;
        let brace_token = braced!(content in input);
        attrs.extend(content.call(Attribute::parse_inner)?);

        let mut items = Vec::new();
        while !content.is_empty() {
            items.push(content.parse()?);
        }

        Ok(Module {
            cfg,
            namespace,
            attrs,
            vis,
            unsafety,
            mod_token,
            ident,
            brace_token,
            content: items,
        })
    }
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let ahead = input.fork();
        let unsafety = if ahead.parse::<Option<Token![unsafe]>>()?.is_some()
            && ahead.parse::<Option<Token![extern]>>()?.is_some()
            && ahead.parse::<Option<LitStr>>().is_ok()
            && ahead.peek(token::Brace)
        {
            Some(input.parse()?)
        } else {
            None
        };

        let item = input.parse()?;
        match item {
            RustItem::Struct(mut item) => {
                item.attrs.splice(..0, attrs);
                Ok(Item::Struct(item))
            }
            RustItem::Enum(mut item) => {
                item.attrs.splice(..0, attrs);
                Ok(Item::Enum(item))
            }
            RustItem::ForeignMod(mut item) => {
                item.attrs.splice(..0, attrs);
                Ok(Item::ForeignMod(ItemForeignMod {
                    attrs: item.attrs,
                    unsafety,
                    abi: item.abi,
                    brace_token: item.brace_token,
                    items: item.items,
                }))
            }
            RustItem::Impl(mut item) => {
                item.attrs.splice(..0, attrs);
                Ok(Item::Impl(item))
            }
            RustItem::Use(mut item) => {
                item.attrs.splice(..0, attrs);
                Ok(Item::Use(item))
            }
            other => Ok(Item::Other(other)),
        }
    }
}
