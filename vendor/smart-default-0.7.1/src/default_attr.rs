use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::Error, MetaNameValue};

use crate::util::find_only;

#[derive(Debug, Clone, Copy)]
pub enum ConversionStrategy {
    NoConversion,
    Into,
}

pub struct DefaultAttr {
    pub code: Option<TokenStream>,
    conversion_strategy: Option<ConversionStrategy>,
}

impl DefaultAttr {
    pub fn find_in_attributes(attrs: &[syn::Attribute]) -> Result<Option<Self>, Error> {
        if let Some(default_attr) =
            find_only(attrs.iter(), |attr| Ok(attr.path().is_ident("default")))?
        {
            match &default_attr.meta {
                syn::Meta::Path(_) => Ok(Some(Self {
                    code: None,
                    conversion_strategy: None,
                })),
                syn::Meta::List(meta) => {
                    // If the meta contains exactly (_code = "...") take the string literal as the
                    // expression
                    if let Ok(ParseCodeHack(code_hack)) = syn::parse(meta.tokens.clone().into()) {
                        Ok(Some(Self {
                            code: Some(code_hack),
                            conversion_strategy: Some(ConversionStrategy::NoConversion),
                        }))
                    } else {
                        Ok(Some(Self {
                            code: Some(meta.tokens.clone()),
                            conversion_strategy: None,
                        }))
                    }
                }
                syn::Meta::NameValue(MetaNameValue { value, .. }) => Ok(Some(Self {
                    code: Some(value.into_token_stream()),
                    conversion_strategy: None,
                })),
            }
        } else {
            Ok(None)
        }
    }

    pub fn conversion_strategy(&self) -> ConversionStrategy {
        if let Some(conversion_strategy) = self.conversion_strategy {
            // Conversion strategy already set
            return conversion_strategy;
        }
        let code = if let Some(code) = &self.code {
            code
        } else {
            // #[default] - so no conversion (`Default::default()` already has the correct type)
            return ConversionStrategy::NoConversion;
        };
        match syn::parse::<syn::Lit>(code.clone().into()) {
            Ok(syn::Lit::Str(_)) | Ok(syn::Lit::ByteStr(_)) => {
                // A string literal - so we need a conversion in case we need to make it a `String`
                return ConversionStrategy::Into;
            }
            _ => {}
        }
        // Not handled by one of the rules, so we don't convert it to avoid causing trouble
        ConversionStrategy::NoConversion
    }
}

struct ParseCodeHack(TokenStream);

impl syn::parse::Parse for ParseCodeHack {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        if ident != "_code" {
            return Err(Error::new(ident.span(), "Expected `_code`"));
        }
        input.parse::<syn::token::Eq>()?;
        let code: syn::LitStr = input.parse()?;
        let code: TokenStream = code.parse()?;
        Ok(ParseCodeHack(code))
    }
}
