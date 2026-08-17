//! Types for parsing arguments, shared by many of the macros

use crate::{
    parse_utils::{MyParse, ParseBuffer, ParseStream},
    spanned::Spans,
};

use proc_macro2::TokenStream as TokenStream2;

use quote::ToTokens;

////////////////////////////////////////////////

// An expression inside `(...)`
pub(crate) struct ExprArg {
    pub(crate) span: Spans,
    /// Using a TokenStream2 because it is validated to be a valid expression in
    /// the macro_rules! macros that call these proc macros.
    pub(crate) expr: TokenStream2,
}

impl ToTokens for ExprArg {
    fn to_tokens(&self, ts: &mut TokenStream2) {
        self.expr.to_tokens(ts);
    }
}

/// A sequence of comma separated expressions wrapped in parentheses (with a trailing comma)
pub(crate) struct ExprArgs {
    pub(crate) args: Vec<ExprArg>,
}

////////////////////////////////////////////////

impl MyParse for ExprArg {
    fn parse(input: ParseStream<'_>) -> Result<Self, crate::Error> {
        let paren = input.parse_paren()?;

        let mut content = ParseBuffer::new(paren.contents);

        content.parse_unwrap_tt(|content| {
            let (expr, span) = content.parse_token_stream_and_span();

            Ok(Self { span, expr })
        })
    }
}

////////////////////////////////////////////////

impl MyParse for ExprArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self, crate::Error> {
        let mut args = Vec::new();

        while !input.is_empty() {
            args.push(ExprArg::parse(input)?);

            if !input.is_empty() {
                input.parse_punct(',')?;
            }
        }

        Ok(Self { args })
    }
}
