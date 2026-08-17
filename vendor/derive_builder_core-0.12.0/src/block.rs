use std::convert::TryFrom;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{self, spanned::Spanned, Block, LitStr};

/// A wrapper for expressions/blocks which automatically adds the start and end
/// braces.
///
/// - **full access** to variables environment.
/// - **full access** to control-flow of the environment via `return`, `?` etc.
#[derive(Debug, Clone)]
pub struct BlockContents(Block);

impl BlockContents {
    pub fn is_empty(&self) -> bool {
        self.0.stmts.is_empty()
    }
}

impl ToTokens for BlockContents {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl TryFrom<&'_ LitStr> for BlockContents {
    type Error = syn::Error;

    fn try_from(s: &LitStr) -> Result<Self, Self::Error> {
        let mut block_str = s.value();
        block_str.insert(0, '{');
        block_str.push('}');
        LitStr::new(&block_str, s.span()).parse().map(Self)
    }
}

impl From<syn::Expr> for BlockContents {
    fn from(v: syn::Expr) -> Self {
        Self(Block {
            brace_token: syn::token::Brace(v.span()),
            stmts: vec![syn::Stmt::Expr(v)],
        })
    }
}

impl darling::FromMeta for BlockContents {
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        if let syn::Lit::Str(s) = value {
            let contents = BlockContents::try_from(s)?;
            if contents.is_empty() {
                Err(darling::Error::unknown_value("").with_span(s))
            } else {
                Ok(contents)
            }
        } else {
            Err(darling::Error::unexpected_lit_type(value))
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use super::*;
    use proc_macro2::Span;

    fn parse(s: &str) -> Result<BlockContents, syn::Error> {
        (&LitStr::new(s, Span::call_site())).try_into()
    }

    #[test]
    #[should_panic(expected = r#"lex error"#)]
    fn block_invalid_token_trees() {
        parse("let x = 2; { x+1").unwrap();
    }

    #[test]
    fn block_delimited_token_tree() {
        let expr = parse("let x = 2; { x+1 }").unwrap();
        assert_eq!(
            quote!(#expr).to_string(),
            quote!({
                let x = 2;
                {
                    x + 1
                }
            })
            .to_string()
        );
    }

    #[test]
    fn block_single_token_tree() {
        let expr = parse("42").unwrap();
        assert_eq!(quote!(#expr).to_string(), quote!({ 42 }).to_string());
    }
}
