use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::token::Brace;
use syn::{Attribute, Generics, ImplItem, ItemImpl, Path, Token, Type};

#[derive(Clone)]
pub struct TraitImpl {
    pub attrs: Vec<Attribute>,
    pub defaultness: Option<Token![default]>,
    pub unsafety: Option<Token![unsafe]>,
    pub impl_token: Token![impl],
    pub generics: Generics,
    pub trait_: Path,
    pub for_token: Token![for],
    pub self_ty: Type,
    pub brace_token: Brace,
    pub items: Vec<ImplItem>,
}

impl Parse for TraitImpl {
    fn parse(input: ParseStream) -> Result<Self> {
        let imp: ItemImpl = input.parse()?;

        let (trait_, for_token) = match imp.trait_ {
            Some((_bang_token, trait_, for_token)) => (trait_, for_token),
            None => {
                return Err(Error::new(
                    Span::call_site(),
                    "must be placed on a trait impl",
                ))
            }
        };

        Ok(TraitImpl {
            attrs: imp.attrs,
            defaultness: imp.defaultness,
            unsafety: imp.unsafety,
            impl_token: imp.impl_token,
            generics: imp.generics,
            trait_,
            for_token,
            self_ty: *imp.self_ty,
            brace_token: imp.brace_token,
            items: imp.items,
        })
    }
}

impl ToTokens for TraitImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let imp = self.clone();

        ItemImpl::to_tokens(
            &ItemImpl {
                attrs: imp.attrs,
                defaultness: imp.defaultness,
                unsafety: imp.unsafety,
                impl_token: imp.impl_token,
                generics: imp.generics,
                trait_: Some((None, imp.trait_, imp.for_token)),
                self_ty: Box::new(imp.self_ty),
                brace_token: imp.brace_token,
                items: imp.items,
            },
            tokens,
        );
    }
}
