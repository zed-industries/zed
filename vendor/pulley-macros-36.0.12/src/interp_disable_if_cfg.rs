use proc_macro::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{
    Attribute, Result, Signature, Visibility, braced,
    parse::{Parse, ParseStream},
    parse_macro_input, token,
};

pub fn run(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let mut cfg = None;

    let config_parser = syn::meta::parser(|meta| {
        cfg = Some(meta.path.require_ident()?.clone());
        Ok(())
    });

    parse_macro_input!(attrs with config_parser);

    match expand(cfg.unwrap(), parse_macro_input!(item as Fn)) {
        Ok(tok) => tok,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Custom function parser.
/// Parses the function's attributes, visibility and signature, leaving the
/// block as an opaque [`TokenStream`].
struct Fn {
    attrs: Vec<Attribute>,
    visibility: Visibility,
    sig: Signature,
    body: Block,
}

impl Parse for Fn {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let visibility: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;
        let body: Block = input.parse()?;

        Ok(Self {
            attrs,
            visibility,
            sig,
            body,
        })
    }
}

impl ToTokens for Fn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.visibility.to_tokens(tokens);
        self.sig.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

/// A generic function body represented as a braced [`TokenStream`].
struct Block {
    brace: token::Brace,
    rest: proc_macro2::TokenStream,
}

impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            brace: braced!(content in input),
            rest: content.parse()?,
        })
    }
}

impl ToTokens for Block {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.brace.surround(tokens, |tokens| {
            tokens.append_all(self.rest.clone());
        });
    }
}

fn expand(cfg: syn::Ident, func: Fn) -> Result<TokenStream> {
    let Fn {
        attrs,
        visibility,
        sig,
        body: _,
    } = &func;
    let name = &sig.ident;
    Ok(quote! {
        #[cfg(#cfg)]
        #(#attrs)*
        #[allow(unused_variables)]
        #visibility #sig {
            self.done_trap_kind::<crate::#name>(Some(TrapKind::DisabledOpcode))
        }

        #[cfg(not(#cfg))]
        #func
    }
    .into())
}
