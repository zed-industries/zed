use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::codegen::FromMetaImpl;
use crate::options::{Core, ParseAttribute, ParseData};
use crate::Result;

pub struct FromMetaOptions {
    base: Core,
}

impl FromMetaOptions {
    pub fn new(di: &syn::DeriveInput) -> Result<Self> {
        (FromMetaOptions {
            base: Core::start(di)?,
        })
        .parse_attributes(&di.attrs)?
        .parse_body(&di.data)
    }
}

impl ParseAttribute for FromMetaOptions {
    fn parse_nested(&mut self, mi: &syn::Meta) -> Result<()> {
        self.base.parse_nested(mi)
    }
}

impl ParseData for FromMetaOptions {
    fn parse_variant(&mut self, variant: &syn::Variant) -> Result<()> {
        self.base.parse_variant(variant)
    }

    fn parse_field(&mut self, field: &syn::Field) -> Result<()> {
        self.base.parse_field(field)
    }
}

impl<'a> From<&'a FromMetaOptions> for FromMetaImpl<'a> {
    fn from(v: &'a FromMetaOptions) -> Self {
        FromMetaImpl {
            base: (&v.base).into(),
        }
    }
}

impl ToTokens for FromMetaOptions {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        FromMetaImpl::from(self).to_tokens(tokens)
    }
}
