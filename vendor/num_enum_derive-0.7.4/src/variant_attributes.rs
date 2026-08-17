use syn::{
    parse::{Parse, ParseStream},
    Expr, Result,
};

mod kw {
    syn::custom_keyword!(default);
    syn::custom_keyword!(catch_all);
    syn::custom_keyword!(alternatives);
}

pub(crate) struct NumEnumVariantAttributes {
    pub(crate) items: syn::punctuated::Punctuated<NumEnumVariantAttributeItem, syn::Token![,]>,
}

impl Parse for NumEnumVariantAttributes {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            items: input.parse_terminated(NumEnumVariantAttributeItem::parse, syn::Token![,])?,
        })
    }
}

pub(crate) enum NumEnumVariantAttributeItem {
    Default(VariantDefaultAttribute),
    CatchAll(VariantCatchAllAttribute),
    Alternatives(VariantAlternativesAttribute),
}

impl Parse for NumEnumVariantAttributeItem {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::default) {
            input.parse().map(Self::Default)
        } else if lookahead.peek(kw::catch_all) {
            input.parse().map(Self::CatchAll)
        } else if lookahead.peek(kw::alternatives) {
            input.parse().map(Self::Alternatives)
        } else {
            Err(lookahead.error())
        }
    }
}

pub(crate) struct VariantDefaultAttribute {
    pub(crate) keyword: kw::default,
}

impl Parse for VariantDefaultAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            keyword: input.parse()?,
        })
    }
}

pub(crate) struct VariantCatchAllAttribute {
    pub(crate) keyword: kw::catch_all,
}

impl Parse for VariantCatchAllAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            keyword: input.parse()?,
        })
    }
}

pub(crate) struct VariantAlternativesAttribute {
    _keyword: kw::alternatives,
    _eq_token: syn::Token![=],
    _bracket_token: syn::token::Bracket,
    pub(crate) expressions: syn::punctuated::Punctuated<Expr, syn::Token![,]>,
}

impl Parse for VariantAlternativesAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let keyword = input.parse()?;
        let _eq_token = input.parse()?;
        let _bracket_token = syn::bracketed!(content in input);
        let expressions = content.parse_terminated(Expr::parse, syn::Token![,])?;
        Ok(Self {
            _keyword: keyword,
            _eq_token,
            _bracket_token,
            expressions,
        })
    }
}
