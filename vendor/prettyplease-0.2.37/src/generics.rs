use crate::algorithm::Printer;
use crate::iter::IterDelimited;
use crate::path::PathKind;
use crate::INDENT;
use proc_macro2::TokenStream;
use std::ptr;
use syn::{
    BoundLifetimes, CapturedParam, ConstParam, Expr, GenericParam, Generics, LifetimeParam,
    PreciseCapture, PredicateLifetime, PredicateType, TraitBound, TraitBoundModifier, TypeParam,
    TypeParamBound, WhereClause, WherePredicate,
};

impl Printer {
    pub fn generics(&mut self, generics: &Generics) {
        if generics.params.is_empty() {
            return;
        }

        self.word("<");
        self.cbox(0);
        self.zerobreak();

        // Print lifetimes before types and consts, regardless of their
        // order in self.params.
        #[derive(Ord, PartialOrd, Eq, PartialEq)]
        enum Group {
            First,
            Second,
        }
        fn group(param: &GenericParam) -> Group {
            match param {
                GenericParam::Lifetime(_) => Group::First,
                GenericParam::Type(_) | GenericParam::Const(_) => Group::Second,
            }
        }
        let last = generics.params.iter().max_by_key(|param| group(param));
        for current_group in [Group::First, Group::Second] {
            for param in &generics.params {
                if group(param) == current_group {
                    self.generic_param(param);
                    self.trailing_comma(ptr::eq(param, last.unwrap()));
                }
            }
        }

        self.offset(-INDENT);
        self.end();
        self.word(">");
    }

    fn generic_param(&mut self, generic_param: &GenericParam) {
        match generic_param {
            GenericParam::Type(type_param) => self.type_param(type_param),
            GenericParam::Lifetime(lifetime_param) => self.lifetime_param(lifetime_param),
            GenericParam::Const(const_param) => self.const_param(const_param),
        }
    }

    pub fn bound_lifetimes(&mut self, bound_lifetimes: &BoundLifetimes) {
        self.word("for<");
        for param in bound_lifetimes.lifetimes.iter().delimited() {
            self.generic_param(&param);
            if !param.is_last {
                self.word(", ");
            }
        }
        self.word("> ");
    }

    fn lifetime_param(&mut self, lifetime_param: &LifetimeParam) {
        self.outer_attrs(&lifetime_param.attrs);
        self.lifetime(&lifetime_param.lifetime);
        for lifetime in lifetime_param.bounds.iter().delimited() {
            if lifetime.is_first {
                self.word(": ");
            } else {
                self.word(" + ");
            }
            self.lifetime(&lifetime);
        }
    }

    fn type_param(&mut self, type_param: &TypeParam) {
        self.outer_attrs(&type_param.attrs);
        self.ident(&type_param.ident);
        self.ibox(INDENT);
        for type_param_bound in type_param.bounds.iter().delimited() {
            if type_param_bound.is_first {
                self.word(": ");
            } else {
                self.space();
                self.word("+ ");
            }
            self.type_param_bound(&type_param_bound);
        }
        if let Some(default) = &type_param.default {
            self.space();
            self.word("= ");
            self.ty(default);
        }
        self.end();
    }

    pub fn type_param_bound(&mut self, type_param_bound: &TypeParamBound) {
        match type_param_bound {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            TypeParamBound::Trait(trait_bound) => {
                self.trait_bound(trait_bound, TraitBoundConst::None);
            }
            TypeParamBound::Lifetime(lifetime) => self.lifetime(lifetime),
            TypeParamBound::PreciseCapture(precise_capture) => {
                self.precise_capture(precise_capture);
            }
            TypeParamBound::Verbatim(bound) => self.type_param_bound_verbatim(bound),
            _ => unimplemented!("unknown TypeParamBound"),
        }
    }

    fn trait_bound(&mut self, trait_bound: &TraitBound, constness: TraitBoundConst) {
        if trait_bound.paren_token.is_some() {
            self.word("(");
        }
        if let Some(bound_lifetimes) = &trait_bound.lifetimes {
            self.bound_lifetimes(bound_lifetimes);
        }
        match constness {
            TraitBoundConst::None => {}
            #[cfg(feature = "verbatim")]
            TraitBoundConst::Conditional => self.word("[const] "),
            #[cfg(feature = "verbatim")]
            TraitBoundConst::Unconditional => self.word("const "),
        }
        self.trait_bound_modifier(&trait_bound.modifier);
        for segment in trait_bound.path.segments.iter().delimited() {
            if !segment.is_first || trait_bound.path.leading_colon.is_some() {
                self.word("::");
            }
            self.path_segment(&segment, PathKind::Type);
        }
        if trait_bound.paren_token.is_some() {
            self.word(")");
        }
    }

    fn trait_bound_modifier(&mut self, trait_bound_modifier: &TraitBoundModifier) {
        match trait_bound_modifier {
            TraitBoundModifier::None => {}
            TraitBoundModifier::Maybe(_question_mark) => self.word("?"),
        }
    }

    #[cfg(not(feature = "verbatim"))]
    fn type_param_bound_verbatim(&mut self, bound: &TokenStream) {
        unimplemented!("TypeParamBound::Verbatim `{}`", bound);
    }

    #[cfg(feature = "verbatim")]
    fn type_param_bound_verbatim(&mut self, tokens: &TokenStream) {
        use syn::parse::{Parse, ParseStream, Result};
        use syn::{
            bracketed, parenthesized, token, ParenthesizedGenericArguments, Path, PathArguments,
            Token,
        };

        enum TypeParamBoundVerbatim {
            Ellipsis,
            Const(TraitBound, TraitBoundConst),
        }

        impl Parse for TypeParamBoundVerbatim {
            fn parse(input: ParseStream) -> Result<Self> {
                if input.peek(Token![...]) {
                    input.parse::<Token![...]>()?;
                    return Ok(TypeParamBoundVerbatim::Ellipsis);
                }

                let content;
                let content = if input.peek(token::Paren) {
                    parenthesized!(content in input);
                    &content
                } else {
                    input
                };

                let lifetimes: Option<BoundLifetimes> = content.parse()?;

                let constness = if content.peek(token::Bracket) {
                    let conditionally_const;
                    bracketed!(conditionally_const in content);
                    conditionally_const.parse::<Token![const]>()?;
                    TraitBoundConst::Conditional
                } else if content.peek(Token![const]) {
                    content.parse::<Token![const]>()?;
                    TraitBoundConst::Unconditional
                } else {
                    TraitBoundConst::None
                };

                let modifier: TraitBoundModifier = content.parse()?;

                let mut path: Path = content.parse()?;
                if path.segments.last().unwrap().arguments.is_empty()
                    && (content.peek(token::Paren)
                        || content.peek(Token![::]) && content.peek3(token::Paren))
                {
                    content.parse::<Option<Token![::]>>()?;
                    let args: ParenthesizedGenericArguments = content.parse()?;
                    let parenthesized = PathArguments::Parenthesized(args);
                    path.segments.last_mut().unwrap().arguments = parenthesized;
                }

                Ok(TypeParamBoundVerbatim::Const(
                    TraitBound {
                        paren_token: None,
                        modifier,
                        lifetimes,
                        path,
                    },
                    constness,
                ))
            }
        }

        let bound: TypeParamBoundVerbatim = match syn::parse2(tokens.clone()) {
            Ok(bound) => bound,
            Err(_) => unimplemented!("TypeParamBound::Verbatim `{}`", tokens),
        };

        match bound {
            TypeParamBoundVerbatim::Ellipsis => {
                self.word("...");
            }
            TypeParamBoundVerbatim::Const(trait_bound, constness) => {
                self.trait_bound(&trait_bound, constness);
            }
        }
    }

    fn const_param(&mut self, const_param: &ConstParam) {
        self.outer_attrs(&const_param.attrs);
        self.word("const ");
        self.ident(&const_param.ident);
        self.word(": ");
        self.ty(&const_param.ty);
        if let Some(default) = &const_param.default {
            self.word(" = ");
            self.const_argument(default);
        }
    }

    pub fn where_clause_for_body(&mut self, where_clause: &Option<WhereClause>) {
        let hardbreaks = true;
        let semi = false;
        self.where_clause_impl(where_clause, hardbreaks, semi);
    }

    pub fn where_clause_semi(&mut self, where_clause: &Option<WhereClause>) {
        let hardbreaks = true;
        let semi = true;
        self.where_clause_impl(where_clause, hardbreaks, semi);
    }

    pub fn where_clause_oneline(&mut self, where_clause: &Option<WhereClause>) {
        let hardbreaks = false;
        let semi = false;
        self.where_clause_impl(where_clause, hardbreaks, semi);
    }

    pub fn where_clause_oneline_semi(&mut self, where_clause: &Option<WhereClause>) {
        let hardbreaks = false;
        let semi = true;
        self.where_clause_impl(where_clause, hardbreaks, semi);
    }

    fn where_clause_impl(
        &mut self,
        where_clause: &Option<WhereClause>,
        hardbreaks: bool,
        semi: bool,
    ) {
        let where_clause = match where_clause {
            Some(where_clause) if !where_clause.predicates.is_empty() => where_clause,
            _ => {
                if semi {
                    self.word(";");
                } else {
                    self.nbsp();
                }
                return;
            }
        };
        if hardbreaks {
            self.hardbreak();
            self.offset(-INDENT);
            self.word("where");
            self.hardbreak();
            for predicate in where_clause.predicates.iter().delimited() {
                self.where_predicate(&predicate);
                if predicate.is_last && semi {
                    self.word(";");
                } else {
                    self.word(",");
                    self.hardbreak();
                }
            }
            if !semi {
                self.offset(-INDENT);
            }
        } else {
            self.space();
            self.offset(-INDENT);
            self.word("where");
            self.space();
            for predicate in where_clause.predicates.iter().delimited() {
                self.where_predicate(&predicate);
                if predicate.is_last && semi {
                    self.word(";");
                } else {
                    self.trailing_comma_or_space(predicate.is_last);
                }
            }
            if !semi {
                self.offset(-INDENT);
            }
        }
    }

    fn where_predicate(&mut self, predicate: &WherePredicate) {
        match predicate {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            WherePredicate::Type(predicate) => self.predicate_type(predicate),
            WherePredicate::Lifetime(predicate) => self.predicate_lifetime(predicate),
            _ => unimplemented!("unknown WherePredicate"),
        }
    }

    fn predicate_type(&mut self, predicate: &PredicateType) {
        if let Some(bound_lifetimes) = &predicate.lifetimes {
            self.bound_lifetimes(bound_lifetimes);
        }
        self.ty(&predicate.bounded_ty);
        self.word(":");
        if predicate.bounds.len() == 1 {
            self.ibox(0);
        } else {
            self.ibox(INDENT);
        }
        for type_param_bound in predicate.bounds.iter().delimited() {
            if type_param_bound.is_first {
                self.nbsp();
            } else {
                self.space();
                self.word("+ ");
            }
            self.type_param_bound(&type_param_bound);
        }
        self.end();
    }

    fn predicate_lifetime(&mut self, predicate: &PredicateLifetime) {
        self.lifetime(&predicate.lifetime);
        self.word(":");
        self.ibox(INDENT);
        for lifetime in predicate.bounds.iter().delimited() {
            if lifetime.is_first {
                self.nbsp();
            } else {
                self.space();
                self.word("+ ");
            }
            self.lifetime(&lifetime);
        }
        self.end();
    }

    fn precise_capture(&mut self, precise_capture: &PreciseCapture) {
        self.word("use<");
        for capture in precise_capture.params.iter().delimited() {
            self.captured_param(&capture);
            if !capture.is_last {
                self.word(", ");
            }
        }
        self.word(">");
    }

    fn captured_param(&mut self, capture: &CapturedParam) {
        match capture {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            CapturedParam::Lifetime(lifetime) => self.lifetime(lifetime),
            CapturedParam::Ident(ident) => self.ident(ident),
            _ => unimplemented!("unknown CapturedParam"),
        }
    }

    pub fn const_argument(&mut self, expr: &Expr) {
        match expr {
            #![cfg_attr(all(test, exhaustive), allow(non_exhaustive_omitted_patterns))]
            Expr::Lit(expr) => self.expr_lit(expr),

            Expr::Path(expr)
                if expr.attrs.is_empty()
                    && expr.qself.is_none()
                    && expr.path.get_ident().is_some() =>
            {
                self.expr_path(expr);
            }

            Expr::Block(expr) => self.expr_block(expr),

            _ => {
                self.cbox(INDENT);
                self.expr_as_small_block(expr, 0);
                self.end();
            }
        }
    }
}

enum TraitBoundConst {
    None,
    #[cfg(feature = "verbatim")]
    Conditional,
    #[cfg(feature = "verbatim")]
    Unconditional,
}
