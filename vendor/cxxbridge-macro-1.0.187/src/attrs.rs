use crate::syntax::attrs::OtherAttrs;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Attribute;

impl OtherAttrs {
    pub(crate) fn all(&self) -> PrintOtherAttrs {
        PrintOtherAttrs {
            attrs: self,
            cfg: true,
            lint: true,
            passthrough: true,
        }
    }

    pub(crate) fn cfg(&self) -> PrintOtherAttrs {
        PrintOtherAttrs {
            attrs: self,
            cfg: true,
            lint: false,
            passthrough: false,
        }
    }

    pub(crate) fn cfg_and_lint(&self) -> PrintOtherAttrs {
        PrintOtherAttrs {
            attrs: self,
            cfg: true,
            lint: true,
            passthrough: false,
        }
    }
}

pub(crate) struct PrintOtherAttrs<'a> {
    attrs: &'a OtherAttrs,
    cfg: bool,
    lint: bool,
    passthrough: bool,
}

impl<'a> ToTokens for PrintOtherAttrs<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.cfg {
            print_attrs_as_outer(&self.attrs.cfg, tokens);
        }
        if self.lint {
            print_attrs_as_outer(&self.attrs.lint, tokens);
        }
        if self.passthrough {
            print_attrs_as_outer(&self.attrs.passthrough, tokens);
        }
    }
}

fn print_attrs_as_outer(attrs: &[Attribute], tokens: &mut TokenStream) {
    for attr in attrs {
        let Attribute {
            pound_token,
            style,
            bracket_token,
            meta,
        } = attr;
        pound_token.to_tokens(tokens);
        let _ = style; // ignore; render outer and inner attrs both as outer
        bracket_token.surround(tokens, |tokens| meta.to_tokens(tokens));
    }
}
