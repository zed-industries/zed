use crate::BlockContents;
use proc_macro2::Span;
use quote::ToTokens;

/// A `DefaultExpression` can be either explicit or refer to the canonical trait.
#[derive(Debug, Clone)]
pub enum DefaultExpression {
    Explicit(BlockContents),
    Trait,
}

impl DefaultExpression {
    /// Add the crate root path so the default expression can be emitted
    /// to a `TokenStream`.
    ///
    /// This function is needed because the crate root is inherited from the container, so it cannot
    /// be provided at parse time to [`darling::FromMeta::from_word`] when reading, and [`ToTokens`] does not
    /// accept any additional parameters, so it annot be provided at emit time.
    pub fn with_crate_root<'a>(&'a self, crate_root: &'a syn::Path) -> impl 'a + ToTokens {
        DefaultExpressionWithCrateRoot {
            crate_root,
            expr: self,
        }
    }

    #[cfg(test)]
    pub fn explicit<I: Into<BlockContents>>(content: I) -> Self {
        DefaultExpression::Explicit(content.into())
    }
}

impl darling::FromMeta for DefaultExpression {
    fn from_word() -> darling::Result<Self> {
        Ok(DefaultExpression::Trait)
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        Ok(Self::Explicit(BlockContents::from_value(value)?))
    }
}

impl syn::spanned::Spanned for DefaultExpression {
    fn span(&self) -> Span {
        match self {
            DefaultExpression::Explicit(block) => block.span(),
            DefaultExpression::Trait => Span::call_site(),
        }
    }
}

/// Wrapper for `DefaultExpression`
struct DefaultExpressionWithCrateRoot<'a> {
    crate_root: &'a syn::Path,
    expr: &'a DefaultExpression,
}

impl<'a> ToTokens for DefaultExpressionWithCrateRoot<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_root = self.crate_root;
        match self.expr {
            DefaultExpression::Explicit(ref block) => block.to_tokens(tokens),
            DefaultExpression::Trait => quote!(
                #crate_root::export::core::default::Default::default()
            )
            .to_tokens(tokens),
        }
    }
}
