use crate::algorithm::Printer;
use crate::fixup::FixupContext;
use crate::iter::IterDelimited;
use crate::path::PathKind;
use crate::INDENT;
use proc_macro2::TokenStream;
use syn::{
    FieldPat, Pat, PatIdent, PatOr, PatParen, PatReference, PatRest, PatSlice, PatStruct, PatTuple,
    PatTupleStruct, PatType, PatWild,
};

impl Printer {
    pub fn pat(&mut self, pat: &Pat) {
        match pat {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            Pat::Const(pat) => self.expr_const(pat),
            Pat::Ident(pat) => self.pat_ident(pat),
            Pat::Lit(pat) => self.expr_lit(pat),
            Pat::Macro(pat) => self.expr_macro(pat),
            Pat::Or(pat) => self.pat_or(pat),
            Pat::Paren(pat) => self.pat_paren(pat),
            Pat::Path(pat) => self.expr_path(pat),
            Pat::Range(pat) => self.expr_range(pat, FixupContext::NONE),
            Pat::Reference(pat) => self.pat_reference(pat),
            Pat::Rest(pat) => self.pat_rest(pat),
            Pat::Slice(pat) => self.pat_slice(pat),
            Pat::Struct(pat) => self.pat_struct(pat),
            Pat::Tuple(pat) => self.pat_tuple(pat),
            Pat::TupleStruct(pat) => self.pat_tuple_struct(pat),
            Pat::Type(pat) => self.pat_type(pat),
            Pat::Verbatim(pat) => self.pat_verbatim(pat),
            Pat::Wild(pat) => self.pat_wild(pat),
            _ => unimplemented!("unknown Pat"),
        }
    }

    fn pat_ident(&mut self, pat: &PatIdent) {
        self.outer_attrs(&pat.attrs);
        if pat.by_ref.is_some() {
            self.word("ref ");
        }
        if pat.mutability.is_some() {
            self.word("mut ");
        }
        self.ident(&pat.ident);
        if let Some((_at_token, subpat)) = &pat.subpat {
            self.word(" @ ");
            self.pat(subpat);
        }
    }

    fn pat_or(&mut self, pat: &PatOr) {
        self.outer_attrs(&pat.attrs);
        let mut consistent_break = false;
        for case in &pat.cases {
            match case {
                Pat::Lit(_) | Pat::Wild(_) => {}
                _ => {
                    consistent_break = true;
                    break;
                }
            }
        }
        if consistent_break {
            self.cbox(0);
        } else {
            self.ibox(0);
        }
        for case in pat.cases.iter().delimited() {
            if !case.is_first {
                self.space();
                self.word("| ");
            }
            self.pat(&case);
        }
        self.end();
    }

    fn pat_paren(&mut self, pat: &PatParen) {
        self.outer_attrs(&pat.attrs);
        self.word("(");
        self.pat(&pat.pat);
        self.word(")");
    }

    fn pat_reference(&mut self, pat: &PatReference) {
        self.outer_attrs(&pat.attrs);
        self.word("&");
        if pat.mutability.is_some() {
            self.word("mut ");
        }
        self.pat(&pat.pat);
    }

    fn pat_rest(&mut self, pat: &PatRest) {
        self.outer_attrs(&pat.attrs);
        self.word("..");
    }

    fn pat_slice(&mut self, pat: &PatSlice) {
        self.outer_attrs(&pat.attrs);
        self.word("[");
        for elem in pat.elems.iter().delimited() {
            self.pat(&elem);
            self.trailing_comma(elem.is_last);
        }
        self.word("]");
    }

    fn pat_struct(&mut self, pat: &PatStruct) {
        self.outer_attrs(&pat.attrs);
        self.cbox(INDENT);
        self.path(&pat.path, PathKind::Expr);
        self.word(" {");
        self.space_if_nonempty();
        for field in pat.fields.iter().delimited() {
            self.field_pat(&field);
            self.trailing_comma_or_space(field.is_last && pat.rest.is_none());
        }
        if let Some(rest) = &pat.rest {
            self.pat_rest(rest);
            self.space();
        }
        self.offset(-INDENT);
        self.end();
        self.word("}");
    }

    fn pat_tuple(&mut self, pat: &PatTuple) {
        self.outer_attrs(&pat.attrs);
        self.word("(");
        self.cbox(INDENT);
        self.zerobreak();
        for elem in pat.elems.iter().delimited() {
            self.pat(&elem);
            if pat.elems.len() == 1 {
                if pat.elems.trailing_punct() {
                    self.word(",");
                }
                self.zerobreak();
            } else {
                self.trailing_comma(elem.is_last);
            }
        }
        self.offset(-INDENT);
        self.end();
        self.word(")");
    }

    fn pat_tuple_struct(&mut self, pat: &PatTupleStruct) {
        self.outer_attrs(&pat.attrs);
        self.path(&pat.path, PathKind::Expr);
        self.word("(");
        self.cbox(INDENT);
        self.zerobreak();
        for elem in pat.elems.iter().delimited() {
            self.pat(&elem);
            self.trailing_comma(elem.is_last);
        }
        self.offset(-INDENT);
        self.end();
        self.word(")");
    }

    pub fn pat_type(&mut self, pat: &PatType) {
        self.outer_attrs(&pat.attrs);
        self.pat(&pat.pat);
        self.word(": ");
        self.ty(&pat.ty);
    }

    #[cfg(not(feature = "verbatim"))]
    fn pat_verbatim(&mut self, pat: &TokenStream) {
        unimplemented!("Pat::Verbatim `{}`", pat);
    }

    #[cfg(feature = "verbatim")]
    fn pat_verbatim(&mut self, tokens: &TokenStream) {
        use syn::parse::{Parse, ParseStream, Result};
        use syn::{braced, Attribute, Block, Token};

        enum PatVerbatim {
            Ellipsis,
            Box(Pat),
            Const(PatConst),
        }

        struct PatConst {
            attrs: Vec<Attribute>,
            block: Block,
        }

        impl Parse for PatVerbatim {
            fn parse(input: ParseStream) -> Result<Self> {
                let lookahead = input.lookahead1();
                if lookahead.peek(Token![box]) {
                    input.parse::<Token![box]>()?;
                    let inner = Pat::parse_single(input)?;
                    Ok(PatVerbatim::Box(inner))
                } else if lookahead.peek(Token![const]) {
                    input.parse::<Token![const]>()?;
                    let content;
                    let brace_token = braced!(content in input);
                    let attrs = content.call(Attribute::parse_inner)?;
                    let stmts = content.call(Block::parse_within)?;
                    Ok(PatVerbatim::Const(PatConst {
                        attrs,
                        block: Block { brace_token, stmts },
                    }))
                } else if lookahead.peek(Token![...]) {
                    input.parse::<Token![...]>()?;
                    Ok(PatVerbatim::Ellipsis)
                } else {
                    Err(lookahead.error())
                }
            }
        }

        let pat: PatVerbatim = match syn::parse2(tokens.clone()) {
            Ok(pat) => pat,
            Err(_) => unimplemented!("Pat::Verbatim `{}`", tokens),
        };

        match pat {
            PatVerbatim::Ellipsis => {
                self.word("...");
            }
            PatVerbatim::Box(pat) => {
                self.word("box ");
                self.pat(&pat);
            }
            PatVerbatim::Const(pat) => {
                self.word("const ");
                self.cbox(INDENT);
                self.small_block(&pat.block, &pat.attrs);
                self.end();
            }
        }
    }

    fn pat_wild(&mut self, pat: &PatWild) {
        self.outer_attrs(&pat.attrs);
        self.word("_");
    }

    fn field_pat(&mut self, field_pat: &FieldPat) {
        self.outer_attrs(&field_pat.attrs);
        if field_pat.colon_token.is_some() {
            self.member(&field_pat.member);
            self.word(": ");
        }
        self.pat(&field_pat.pat);
    }
}
