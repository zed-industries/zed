// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides a parser from syn attributes to our logical model.

use quote::ToTokens;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{self, Attribute, Expr, Ident, Lit, Meta, Type};

use crate::error::{self, Ctx, DeriveResult};
use crate::interp;
use crate::util;

//==============================================================================
// Public API
//==============================================================================

/// Parsed attributes in our logical model.
#[derive(Clone)]
pub struct ParsedAttributes {
    /// If we've been ordered to skip this item.
    /// This is only valid for enum variants.
    pub skip: bool,
    /// The potential weight assigned to an enum variant.
    /// This must be `None` for things that are not enum variants.
    pub weight: Option<u32>,
    /// The mode for `Parameters` to use. See that type for more.
    pub params: ParamsMode,
    /// The mode for `Strategy` to use. See that type for more.
    pub strategy: StratMode,
    /// Filter expressions if any.
    pub filter: Vec<syn::Expr>,
    /// True if no_bound was specified.
    pub no_bound: bool,
}

/// The mode for the associated item `Strategy` to use.
#[derive(Clone)]
pub enum StratMode {
    /// This means that no explicit strategy was specified
    /// and that we thus should use `Arbitrary` for whatever
    /// it is that needs a strategy.
    Arbitrary,
    /// This means that an explicit value has been provided.
    /// The result of this is to use a strategy that always
    /// returns the given value.
    Value(Expr),
    /// This means that an explicit strategy has been provided.
    /// This strategy will be used to generate whatever it
    /// is that the attribute was set on.
    Strategy(Expr),
    /// This means that an explicit *regex* strategy has been provided.
    /// We don't reuse `Strategy(..)` so that we can produce better and
    /// more tailored error messages.
    Regex(Expr),
}

/// The mode for the associated item `Parameters` to use.
#[derive(Clone)]
pub enum ParamsMode {
    /// Nothing has been specified. The children are now free to
    /// specify their parameters, and if nothing is specified, then
    /// `<X as Arbitrary>::Parameters` will be used for a type `X`.
    Passthrough,
    /// We've been ordered to use the Default value of
    /// `<X as Arbitrary>::Parameters` for some field where applicable.
    /// For the top level item, this means that `Parameters` will be
    /// the unit type. For children, it means that this child should
    /// not count towards the product type that is being built up.
    Default,
    /// An explicit type has been specified on some item.
    /// If the top level item has this specified on it, this means
    /// that `Parameters` will have the given type.
    /// If it is specified on a child of the top level item, this
    /// entails that the given type will be added to the resultant
    /// product type.
    Specified(Type),
}

impl ParamsMode {
    /// Returns `true` iff the mode was explicitly set.
    pub fn is_set(&self) -> bool {
        if let ParamsMode::Passthrough = *self {
            false
        } else {
            true
        }
    }

    /// Converts the mode to an `Option` of an `Option` of a type
    /// where the outer `Option` is `None` iff the mode wasn't set
    /// and the inner `Option` is `None` iff the mode was `Default`.
    pub fn into_option(self) -> Option<Option<Type>> {
        use self::ParamsMode::*;
        match self {
            Passthrough => None,
            Specified(ty) => Some(Some(ty)),
            Default => Some(None),
        }
    }
}

impl StratMode {
    /// Returns `true` iff the mode was explicitly set.
    pub fn is_set(&self) -> bool {
        if let StratMode::Arbitrary = self {
            false
        } else {
            true
        }
    }
}

/// Parse the attributes specified on an item and parsed by syn
/// into our logical model that we work with.
pub fn parse_attributes(
    ctx: Ctx,
    attrs: &[Attribute],
) -> DeriveResult<ParsedAttributes> {
    let attrs = parse_attributes_base(ctx, attrs)?;
    if attrs.no_bound {
        error::no_bound_set_on_non_tyvar(ctx);
    }
    Ok(attrs)
}

/// Parse the attributes specified on a type definition...
pub fn parse_top_attributes(
    ctx: Ctx,
    attrs: &[Attribute],
) -> DeriveResult<ParsedAttributes> {
    parse_attributes_base(ctx, attrs)
}

/// Parses the attributes specified on an item and parsed by syn
/// and returns true if we've been ordered to not set an `Arbitrary`
/// bound on the given type variable the attributes are from,
/// no matter what.
pub fn has_no_bound(ctx: Ctx, attrs: &[Attribute]) -> DeriveResult<bool> {
    let attrs = parse_attributes_base(ctx, attrs)?;
    error::if_anything_specified(ctx, &attrs, error::TY_VAR);
    Ok(attrs.no_bound)
}

/// Parse the attributes specified on an item and parsed by syn
/// into our logical model that we work with.
fn parse_attributes_base(
    ctx: Ctx,
    attrs: &[Attribute],
) -> DeriveResult<ParsedAttributes> {
    let acc = parse_accumulate(ctx, attrs);

    Ok(ParsedAttributes {
        skip: acc.skip.is_some(),
        weight: acc.weight,
        filter: acc.filter,
        // Process params and no_params together to see which one to use.
        params: parse_params_mode(ctx, acc.no_params, acc.params)?,
        // Process strategy and value together to see which one to use.
        strategy: parse_strat_mode(ctx, acc.strategy, acc.value, acc.regex)?,
        no_bound: acc.no_bound.is_some(),
    })
}

//==============================================================================
// Internals: Initialization
//==============================================================================

/// The internal state of the attribute parser.
#[derive(Default)]
struct ParseAcc {
    skip: Option<()>,
    weight: Option<u32>,
    no_params: Option<()>,
    params: Option<Type>,
    strategy: Option<Expr>,
    value: Option<Expr>,
    regex: Option<Expr>,
    filter: Vec<Expr>,
    no_bound: Option<()>,
}

//==============================================================================
// Internals: Extraction & Filtering
//==============================================================================

fn parse_accumulate(ctx: Ctx, attrs: &[Attribute]) -> ParseAcc {
    let mut state = ParseAcc::default();

    // Get rid of attributes we don't care about:
    for attr in attrs {
        if is_proptest_attr(&attr) {
            // Flatten attributes so we deal with them uniformly.
            state = extract_modifiers(ctx, &attr)
                .into_iter()
                // Accumulate attributes into a form for final processing.
                .fold(state, |state, meta| dispatch_attribute(ctx, state, meta))
        }
    }

    state
}

/// Returns `true` iff the attribute has to do with proptest.
/// Otherwise, the attribute is irrevant to us and we will simply
/// ignore it in our processing.
fn is_proptest_attr(attr: &Attribute) -> bool {
    util::eq_simple_path("proptest", attr.path())
}

/// Extract all individual attributes inside one `#[proptest(..)]`.
/// We do this to treat all pieces uniformly whether a single
/// `#[proptest(..)]` was used or many. This simplifies the
/// logic somewhat.
fn extract_modifiers(ctx: Ctx, attr: &Attribute) -> Vec<Meta> {
    // Ensure we've been given an outer attribute form.
    if !is_outer_attr(&attr) {
        error::inner_attr(ctx);
    }

    match &attr.meta {
        Meta::List(list) => {
            if syn::parse2::<Lit>(list.tokens.clone()).is_ok() {
                error::immediate_literals(ctx);
            } else {
                let parser = Punctuated::<Meta, Token![,]>::parse_separated_nonempty;
                let metas = parser.parse2(list.tokens.clone()).unwrap();
                return metas.into_iter().collect();
            }
        }
        Meta::Path(_) => error::bare_proptest_attr(ctx),
        Meta::NameValue(_) => error::literal_set_proptest(ctx),
    }

    vec![]
}

/// Returns true iff the given attribute is an outer one, i.e: `#[<attr>]`.
/// An inner attribute is the other possibility and has the syntax `#![<attr>]`.
/// Note that `<attr>` is a meta-variable for the contents inside.
fn is_outer_attr(attr: &Attribute) -> bool {
    syn::AttrStyle::Outer == attr.style
}

//==============================================================================
// Internals: Dispatch
//==============================================================================

/// Dispatches an attribute modifier to handlers and
/// let's them add stuff into our accumulartor.
fn dispatch_attribute(ctx: Ctx, mut acc: ParseAcc, meta: Meta) -> ParseAcc {
    // Dispatch table for attributes:
    let path = meta.path();
    if let Some(name) = path.get_ident().map(ToString::to_string) {
        match name.as_ref() {
            // Valid modifiers:
            "skip" => parse_skip(ctx, &mut acc, meta),
            "w" | "weight" => parse_weight(ctx, &mut acc, &meta),
            "no_params" => parse_no_params(ctx, &mut acc, meta),
            "params" => parse_params(ctx, &mut acc, meta),
            "strategy" => parse_strategy(ctx, &mut acc, &meta),
            "value" => parse_value(ctx, &mut acc, &meta),
            "regex" => parse_regex(ctx, &mut acc, &meta),
            "filter" => parse_filter(ctx, &mut acc, &meta),
            "no_bound" => parse_no_bound(ctx, &mut acc, meta),
            // Invalid modifiers:
            name => dispatch_unknown_mod(ctx, name),
        }
    } else {
        // Occurs when passed path is something other than a single ident
        error::unkown_modifier(ctx, &path.into_token_stream().to_string());
    }
    acc
}

fn dispatch_unknown_mod(ctx: Ctx, name: &str) {
    match name {
        "no_bounds" => error::did_you_mean(ctx, name, "no_bound"),
        "weights" | "weighted" => error::did_you_mean(ctx, name, "weight"),
        "strat" | "strategies" => error::did_you_mean(ctx, name, "strategy"),
        "values" | "valued" | "fix" | "fixed" => {
            error::did_you_mean(ctx, name, "value")
        }
        "regexes" | "regexp" | "re" => error::did_you_mean(ctx, name, "regex"),
        "param" | "parameters" => error::did_you_mean(ctx, name, "params"),
        "no_param" | "no_parameters" => {
            error::did_you_mean(ctx, name, "no_params")
        }
        name => error::unkown_modifier(ctx, name),
        // TODO: consider levenshtein distance.
    }
}

//==============================================================================
// Internals: no_bound
//==============================================================================

/// Parse a no_bound attribute.
/// Valid forms are:
/// + `#[proptest(no_bound)]`
fn parse_no_bound(ctx: Ctx, acc: &mut ParseAcc, meta: Meta) {
    parse_bare_modifier(ctx, &mut acc.no_bound, meta, error::no_bound_malformed)
}

//==============================================================================
// Internals: Skip
//==============================================================================

/// Parse a skip attribute.
/// Valid forms are:
/// + `#[proptest(skip)]`
fn parse_skip(ctx: Ctx, acc: &mut ParseAcc, meta: Meta) {
    parse_bare_modifier(ctx, &mut acc.skip, meta, error::skip_malformed)
}

//==============================================================================
// Internals: Weight
//==============================================================================

/// Parses a weight.
/// Valid forms are:
/// + `#[proptest(weight = <integer>)]`
/// + `#[proptest(weight = "<expr>")]`
/// + `#[proptest(weight(<integer>))]`
/// + `#[proptest(weight("<expr>""))]`
///
/// The `<integer>` must also fit within an `u32` and be unsigned.
fn parse_weight(ctx: Ctx, acc: &mut ParseAcc, meta: &Meta) {
    use std::u32;
    error_if_set(ctx, &acc.weight, &meta);

    // Convert to value if possible:
    let value = normalize_meta(meta.clone())
        .and_then(extract_lit)
        .and_then(extract_expr)
        // Evaluate the expression into a value:
        .as_ref()
        .and_then(interp::eval_expr)
        // Ensure that `val` fits within an `u32` as proptest requires that:
        .filter(|&value| value <= u128::from(u32::MAX))
        .map(|value| value as u32);

    if let v @ Some(_) = value {
        acc.weight = v;
    } else {
        error::weight_malformed(ctx, meta)
    }
}

//==============================================================================
// Internals: Filter
//==============================================================================

/// Parses an explicit value as a strategy.
/// Valid forms are:
/// + `#[proptest(filter(<ident>))]`
/// + `#[proptest(filter = "<expr>")]`
/// + `#[proptest(filter("<expr>")]`
fn parse_filter(ctx: Ctx, acc: &mut ParseAcc, meta: &Meta) {
    if let Some(filter) = match normalize_meta(meta.clone()) {
        Some(NormMeta::Lit(Lit::Str(lit))) => lit.parse().ok(),
        Some(NormMeta::Word(ident)) => Some(parse_quote!( #ident )),
        _ => None,
    } {
        acc.filter.push(filter);
    } else {
        error::filter_malformed(ctx, meta)
    }
}

//==============================================================================
// Internals: Strategy
//==============================================================================

/// Parses an explicit value as a strategy.
/// Valid forms are:
/// + `#[proptest(regex = "<string>")]`
/// + `#[proptest(regex("<string>")]`
/// + `#[proptest(regex(<ident>)]`
fn parse_regex(ctx: Ctx, acc: &mut ParseAcc, meta: &Meta) {
    error_if_set(ctx, &acc.regex, &meta);

    if let expr @ Some(_) = match normalize_meta(meta.clone()) {
        Some(NormMeta::Word(fun)) => Some(function_call(fun)),
        Some(NormMeta::Lit(lit @ Lit::Str(_))) => Some(lit_to_expr(lit)),
        _ => None,
    } {
        acc.regex = expr;
    } else {
        error::regex_malformed(ctx)
    }
}

/// Parses an explicit value as a strategy.
/// Valid forms are:
/// + `#[proptest(value = <literal>)]`
/// + `#[proptest(value = "<expr>")]`
/// + `#[proptest(value("<expr>")]`
/// + `#[proptest(value(<literal>)]`
/// + `#[proptest(value(<ident>)]`
fn parse_value(ctx: Ctx, acc: &mut ParseAcc, meta: &Meta) {
    parse_strategy_base(ctx, &mut acc.value, meta)
}

/// Parses an explicit strategy.
/// Valid forms are:
/// + `#[proptest(strategy = <literal>)]`
/// + `#[proptest(strategy = "<expr>")]`
/// + `#[proptest(strategy("<expr>")]`
/// + `#[proptest(strategy(<literal>)]`
/// + `#[proptest(strategy(<ident>)]`
fn parse_strategy(ctx: Ctx, acc: &mut ParseAcc, meta: &Meta) {
    parse_strategy_base(ctx, &mut acc.strategy, meta)
}

/// Parses an explicit strategy. This is a helper.
/// Valid forms are:
/// + `#[proptest(<meta.name()> = <literal>)]`
/// + `#[proptest(<meta.name()> = "<expr>")]`
/// + `#[proptest(<meta.name()>("<expr>")]`
/// + `#[proptest(<meta.name()>(<literal>)]`
/// + `#[proptest(<meta.name()>(<ident>)]`
fn parse_strategy_base(ctx: Ctx, loc: &mut Option<Expr>, meta: &Meta) {
    error_if_set(ctx, &loc, &meta);

    if let expr @ Some(_) = match normalize_meta(meta.clone()) {
        Some(NormMeta::Word(fun)) => Some(function_call(fun)),
        Some(NormMeta::Lit(lit)) => extract_expr(lit),
        _ => None,
    } {
        *loc = expr;
    } else {
        error::strategy_malformed(ctx, meta)
    }
}

/// Combines any parsed explicit strategy, value, and regex into a single
/// value and fails if both an explicit strategy / value / regex was set.
/// Only one of them can be set, or none.
fn parse_strat_mode(
    ctx: Ctx,
    strat: Option<Expr>,
    value: Option<Expr>,
    regex: Option<Expr>,
) -> DeriveResult<StratMode> {
    Ok(match (strat, value, regex) {
        (None, None, None) => StratMode::Arbitrary,
        (None, None, Some(re)) => StratMode::Regex(re),
        (None, Some(vl), None) => StratMode::Value(vl),
        (Some(st), None, None) => StratMode::Strategy(st),
        _ => error::overspecified_strat(ctx)?,
    })
}

//==============================================================================
// Internals: Parameters
//==============================================================================

/// Combines a potentially set `params` and `no_params` into a single value
/// and fails if both have been set. Only one of them can be set, or none.
fn parse_params_mode(
    ctx: Ctx,
    no_params: Option<()>,
    ty_params: Option<Type>,
) -> DeriveResult<ParamsMode> {
    Ok(match (no_params, ty_params) {
        (None, None) => ParamsMode::Passthrough,
        (None, Some(ty)) => ParamsMode::Specified(ty),
        (Some(_), None) => ParamsMode::Default,
        (Some(_), Some(_)) => error::overspecified_param(ctx)?,
    })
}

/// Parses an explicit Parameters type.
///
/// Valid forms are:
/// + `#[proptest(params(<type>)]`
/// + `#[proptest(params("<type>")]`
/// + `#[proptest(params = "<type>"]`
///
/// The latter form is required for more complex types.
fn parse_params(ctx: Ctx, acc: &mut ParseAcc, meta: Meta) {
    error_if_set(ctx, &acc.params, &meta);

    let typ = match normalize_meta(meta) {
        // Form is: `#[proptest(params(<type>)]`.
        Some(NormMeta::Word(ident)) => Some(ident_to_type(ident)),
        // Form is: `#[proptest(params = "<type>"]` or,
        // Form is: `#[proptest(params("<type>")]`..
        Some(NormMeta::Lit(Lit::Str(lit))) => lit.parse().ok(),
        _ => None,
    };

    if let typ @ Some(_) = typ {
        acc.params = typ;
    } else {
        error::param_malformed(ctx)
    }
}

/// Parses an order to use the default Parameters type and value.
/// Valid forms are:
/// + `#[proptest(no_params)]`
fn parse_no_params(ctx: Ctx, acc: &mut ParseAcc, meta: Meta) {
    parse_bare_modifier(
        ctx,
        &mut acc.no_params,
        meta,
        error::no_params_malformed,
    )
}

//==============================================================================
// Internals: Utilities
//==============================================================================

/// Parses a bare attribute of the form `#[proptest(<attr>)]` and sets `loc`.
fn parse_bare_modifier(
    ctx: Ctx,
    loc: &mut Option<()>,
    meta: Meta,
    malformed: fn(Ctx),
) {
    error_if_set(ctx, loc, &meta);

    if let Some(NormMeta::Plain) = normalize_meta(meta) {
        *loc = Some(());
    } else {
        malformed(ctx);
    }
}

/// Emits a "set again" error iff the given option `.is_some()`.
fn error_if_set<T>(ctx: Ctx, loc: &Option<T>, meta: &Meta) {
    if loc.is_some() {
        error::set_again(ctx, meta)
    }
}

/// Constructs a type out of an identifier.
fn ident_to_type(ident: Ident) -> Type {
    Type::Path(syn::TypePath {
        qself: None,
        path: ident.into(),
    })
}

/// Extract a `lit` in `NormMeta::Lit(<lit>)`.
fn extract_lit(meta: NormMeta) -> Option<Lit> {
    if let NormMeta::Lit(lit) = meta {
        Some(lit)
    } else {
        None
    }
}

/// Extract expression out of literal if possible.
fn extract_expr(lit: Lit) -> Option<Expr> {
    match lit {
        Lit::Str(lit) => lit.parse().ok(),
        lit @ Lit::Int(_) => Some(lit_to_expr(lit)),
        // TODO(centril): generalize to other literals, e.g. floats
        _ => None,
    }
}

/// Construct an expression from a literal.
fn lit_to_expr(lit: Lit) -> Expr {
    syn::ExprLit { attrs: vec![], lit }.into()
}

/// Construct a function call expression for an identifier.
fn function_call(fun: Ident) -> Expr {
    parse_quote!( #fun() )
}

/// Normalized `Meta` into all the forms we will possibly accept.
#[derive(Debug)]
enum NormMeta {
    /// Accepts: `#[proptest(<word>)]`
    Plain,
    /// Accepts: `#[proptest(<word> = <lit>)]` and `#[proptest(<word>(<lit>))]`
    Lit(Lit),
    /// Accepts: `#[proptest(<word>(<word>))`.
    Word(Ident),
}

/// Normalize a `meta: Meta` into the forms accepted in `#[proptest(<meta>)]`.
fn normalize_meta(meta: Meta) -> Option<NormMeta> {
    match meta {
        Meta::Path(_) => Some(NormMeta::Plain),
        Meta::NameValue(nv) => match nv.value {
            Expr::Lit(elit) => Some(NormMeta::Lit(elit.lit)),
            _ => None,
        }
        Meta::List(ml) => {
            let mut output: Option<NormMeta> = None;

            if let Ok(lit) = syn::parse2(ml.tokens.clone()) {
                output = Some(NormMeta::Lit(lit));
            } else if let Ok(ident) = syn::parse2(ml.tokens.clone()) {
                output = Some(NormMeta::Word(ident));
            }

            output
        }
    }
}
