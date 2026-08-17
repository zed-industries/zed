use crate::syntax::Atom::{self, *};
use proc_macro2::{Ident, Span};
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{parenthesized, Expr, LitInt};

pub(crate) enum Repr {
    Align(LitInt),
    Atom(Atom, Span),
}

impl Parse for Repr {
    fn parse(input: ParseStream) -> Result<Self> {
        let begin = input.cursor();
        let ident: Ident = input.parse()?;
        if let Some(atom) = Atom::from(&ident) {
            match atom {
                U8 | U16 | U32 | U64 | Usize | I8 | I16 | I32 | I64 | Isize if input.is_empty() => {
                    return Ok(Repr::Atom(atom, ident.span()));
                }
                _ => {}
            }
        } else if ident == "align" {
            let content;
            parenthesized!(content in input);
            let align_expr: Expr = content.fork().parse()?;
            if !matches!(align_expr, Expr::Lit(_)) {
                return Err(Error::new_spanned(
                    align_expr,
                    "invalid repr(align) attribute: an arithmetic expression is not supported",
                ));
            }
            let align_lit: LitInt = content.parse()?;
            let align: u32 = align_lit.base10_parse()?;
            if !align.is_power_of_two() {
                return Err(Error::new_spanned(
                    align_lit,
                    "invalid repr(align) attribute: not a power of two",
                ));
            }
            if align > 2u32.pow(13) {
                return Err(Error::new_spanned(
                    align_lit,
                    "invalid repr(align) attribute: larger than 2^13",
                ));
            }
            return Ok(Repr::Align(align_lit));
        }
        Err(Error::new_spanned(
            begin.token_stream(),
            "unrecognized repr",
        ))
    }
}
