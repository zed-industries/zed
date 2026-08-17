// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! High level IR and abstract syntax tree (AST) of impls.
//!
//! We compile to this AST and then linearise that to Rust code.

use std::ops::{Add, AddAssign};

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::spanned::Spanned;

use crate::error::{Ctx, DeriveResult};
use crate::use_tracking::UseTracker;
use crate::util::self_ty;

//==============================================================================
// Config
//==============================================================================

/// The `MAX - 1` number of strategies that `TupleUnion` supports.
/// Increase this if the behaviour is changed in `proptest`.
/// Keeping this lower than what `proptest` supports will also work
/// but for optimality this should follow what `proptest` supports.
#[cfg(not(feature = "boxed_union"))]
const UNION_CHUNK_SIZE: usize = 9;

/// The `MAX - 1` tuple length `Arbitrary` is implemented for. After this number,
/// tuples are expanded as nested tuples of up to `MAX` elements. The value should
/// be kept in sync with the largest impl in `proptest/src/arbitrary/tuples.rs`.
const NESTED_TUPLE_CHUNK_SIZE: usize = 9;

/// The name of the top parameter variable name given in `arbitrary_with`.
/// Changing this is not a breaking change because a user is expected not
/// to rely on this (and the user shouldn't be able to..).
const TOP_PARAM_NAME: &str = "_top";

/// The name of the variable name used for user facing parameter types
/// specified in a `#[proptest(params = "<type>")]` attribute.
///
/// Changing the value of this constant constitutes a breaking change!
const API_PARAM_NAME: &str = "params";

//==============================================================================
// AST Root
//==============================================================================

/// Top level AST and everything required to implement `Arbitrary` for any
/// given type. Linearizing this AST gives you the impl wrt. Rust code.
pub struct Impl {
    /// Name of the type.
    typ: syn::Ident,
    /// Tracker for uses of Arbitrary trait for a generic type.
    tracker: UseTracker,
    /// The three main parts, see description of `ImplParts` for details.
    parts: ImplParts,
}

/// The three main parts to deriving `Arbitrary` for a type.
/// That is: the associated items `Parameters` (`Params`),
/// `Strategy` (`Strategy`) as well as the construction of the
/// strategy itself (`Ctor`).
pub type ImplParts = (Params, Strategy, Ctor);

impl Impl {
    /// Constructs a new `Impl` from the parts as described on the type.
    pub fn new(typ: syn::Ident, tracker: UseTracker, parts: ImplParts) -> Self {
        Self {
            typ,
            tracker,
            parts,
        }
    }

    /// Linearises the impl into a sequence of tokens.
    /// This produces the actual Rust code for the impl.
    pub fn into_tokens(self, ctx: Ctx) -> DeriveResult<TokenStream> {
        let Impl {
            typ,
            mut tracker,
            parts: (params, strategy, ctor),
        } = self;

        /// A `Debug` bound on a type variable.
        fn debug_bound() -> syn::TypeParamBound {
            parse_quote!(::std::fmt::Debug)
        }

        /// An `Arbitrary` bound on a type variable.
        fn arbitrary_bound() -> syn::TypeParamBound {
            parse_quote!(_proptest::arbitrary::Arbitrary)
        }

        // Add bounds and get generics for the impl.
        tracker.add_bounds(ctx, &arbitrary_bound(), Some(debug_bound()))?;
        let generics = tracker.consume();
        let (impl_generics, ty_generics, where_clause) =
            generics.split_for_impl();

        let _top = call_site_ident(TOP_PARAM_NAME);

        // Linearise everything. We're done after this.
        //
        // NOTE: The clippy::arc_with_non_send_sync lint is disabled here because the strategies
        // generated are often not Send or Sync, such as BoxedStrategy.
        //
        // The double-curly-braces are not strictly required, but allow the expression to be
        // annotated with an attribute.
        let q = quote! {
            #[allow(non_local_definitions)]
            #[allow(non_upper_case_globals)]
            #[allow(clippy::arc_with_non_send_sync)]
            const _: () = {
            use proptest as _proptest;

            impl #impl_generics _proptest::arbitrary::Arbitrary
            for #typ #ty_generics #where_clause {
                type Parameters = #params;

                type Strategy = #strategy;

                fn arbitrary_with(#_top: Self::Parameters) -> Self::Strategy {
                    #ctor
                }
            }

            };
        };

        Ok(q)
    }
}

//==============================================================================
// Smart construcors, StratPair
//==============================================================================

/// A pair of `Strategy` and `Ctor`. These always come in pairs.
pub type StratPair = (Strategy, Ctor);

/// The type and constructor for `any::<Type>()`.
pub fn pair_any(ty: syn::Type, span: Span) -> StratPair {
    let q = Ctor::Arbitrary(ty.clone(), None, span);
    (Strategy::Arbitrary(ty, span), q)
}

/// The type and constructor for `any_with::<Type>(parameters)`.
pub fn pair_any_with(ty: syn::Type, var: usize, span: Span) -> StratPair {
    let q = Ctor::Arbitrary(ty.clone(), Some(var), span);
    (Strategy::Arbitrary(ty, span), q)
}

/// The type and constructor for a specific strategy value constructed by the
/// given expression. Currently, the type is erased and a `BoxedStrategy<Type>`
/// is given back instead.
///
/// This is a temporary restriction. Once `impl Trait` is stabilized,
/// the boxing and dynamic dispatch can be replaced with a statically
/// dispatched anonymous type instead.
pub fn pair_existential(ty: syn::Type, strat: syn::Expr) -> StratPair {
    (Strategy::Existential(ty), Ctor::Existential(strat))
}

/// The type and constructor for a strategy that always returns the value
/// provided in the expression `val`.
/// This is statically dispatched since no erasure is needed or used.
pub fn pair_value(ty: syn::Type, val: syn::Expr) -> StratPair {
    (Strategy::Value(ty), Ctor::Value(val))
}

/// Same as `pair_existential` for the `Self` type.
pub fn pair_existential_self(strat: syn::Expr) -> StratPair {
    pair_existential(self_ty(), strat)
}

/// Same as `pair_value` for the `Self` type.
pub fn pair_value_self(val: syn::Expr) -> StratPair {
    pair_value(self_ty(), val)
}

/// Erased strategy for a fixed value.
pub fn pair_value_exist(ty: syn::Type, strat: syn::Expr) -> StratPair {
    (Strategy::Existential(ty), Ctor::ValueExistential(strat))
}

/// Erased strategy for a fixed value.
pub fn pair_value_exist_self(strat: syn::Expr) -> StratPair {
    pair_value_exist(self_ty(), strat)
}

/// Same as `pair_value` but for a unit variant or unit struct.
pub fn pair_unit_self(path: &syn::Path) -> StratPair {
    pair_value_self(parse_quote!( #path {} ))
}

/// The type and constructor for `#[proptest(regex(..))]`.
pub fn pair_regex(ty: syn::Type, regex: syn::Expr) -> StratPair {
    (Strategy::Regex(ty.clone()), Ctor::Regex(ty, regex))
}

/// Same as `pair_regex` for the `Self` type.
pub fn pair_regex_self(regex: syn::Expr) -> StratPair {
    pair_regex(self_ty(), regex)
}

/// The type and constructor for .prop_map:ing a set of strategies
/// into the type we are implementing for. The closure for the
/// `.prop_map(<closure>)` must also be given.
pub fn pair_map(
    (strats, ctors): (Vec<Strategy>, Vec<Ctor>),
    closure: MapClosure,
) -> StratPair {
    (
        Strategy::Map(strats.into()),
        Ctor::Map(ctors.into(), closure),
    )
}

/// The type and constructor for a union of strategies which produces a new
/// strategy that used the given strategies with probabilities based on the
/// assigned relative weights for each strategy.
pub fn pair_oneof(
    (strats, ctors): (Vec<Strategy>, Vec<(u32, Ctor)>),
) -> StratPair {
    (Strategy::Union(strats.into()), Ctor::Union(ctors.into()))
}

/// Potentially apply a filter to a strategy type and its constructor.
pub fn pair_filter(
    filter: Vec<syn::Expr>,
    ty: syn::Type,
    pair: StratPair,
) -> StratPair {
    filter.into_iter().fold(pair, |(strat, ctor), filter| {
        (
            Strategy::Filter(Box::new(strat), ty.clone()),
            Ctor::Filter(Box::new(ctor), filter),
        )
    })
}

//==============================================================================
// Parameters
//==============================================================================

/// Represents the associated item of `Parameters` of an `Arbitrary` impl.
pub struct Params(Vec<syn::Type>);

impl Params {
    /// Construct an `empty` list of parameters.
    /// This is equivalent to the unit type `()`.
    pub fn empty() -> Self {
        Params(Vec::new())
    }

    /// Computes and returns the number of parameter types.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Params> for syn::Type {
    fn from(x: Params) -> Self {
        let tys = x.0;
        parse_quote!( (#(#tys),*) )
    }
}

impl Add<syn::Type> for Params {
    type Output = Params;

    fn add(mut self, rhs: syn::Type) -> Self::Output {
        self.0.push(rhs);
        self
    }
}

impl AddAssign<syn::Type> for Params {
    fn add_assign(&mut self, rhs: syn::Type) {
        self.0.push(rhs);
    }
}

impl ToTokens for Params {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        NestedTuple(self.0.as_slice()).to_tokens(tokens)
    }
}

/// Returns for a given type `ty` the associated item `Parameters` of the
/// type's `Arbitrary` implementation.
pub fn arbitrary_param(ty: &syn::Type) -> syn::Type {
    parse_quote!(<#ty as _proptest::arbitrary::Arbitrary>::Parameters)
}

//==============================================================================
// Strategy
//==============================================================================

/// The type of a given `Strategy`.
pub enum Strategy {
    /// Assuming the metavariable `$ty` for a given type, this models the
    /// strategy type `<$ty as Arbitrary>::Strategy`.
    Arbitrary(syn::Type, Span),
    /// This models <$ty as StrategyFromRegex>::Strategy.
    Regex(syn::Type),
    /// Assuming the metavariable `$ty` for a given type, this models the
    /// strategy type `BoxedStrategy<$ty>`, i.e: an existentially typed strategy.
    ///
    /// The dynamic dispatch used here is an implementation detail that may be
    /// changed. Such a change does not count as a breakage semver wise.
    Existential(syn::Type),
    /// Assuming the metavariable `$ty` for a given type, this models a
    /// non-shrinking strategy that simply always returns a value of the
    /// given type.
    Value(syn::Type),
    /// Assuming a sequence of strategies, this models a mapping from that
    /// sequence to `Self`.
    Map(Box<[Strategy]>),
    /// Assuming a sequence of relative-weighted strategies, this models a
    /// weighted choice of those strategies. The resultant strategy will in
    /// other words randomly pick one strategy with probabilities based on the
    /// specified weights.
    Union(Box<[Strategy]>),
    /// A filtered strategy with `.prop_filter`.
    Filter(Box<Strategy>, syn::Type),
}

macro_rules! quote_append {
    ($tokens: expr, $($quasi: tt)*) => {
        $tokens.append_all(quote!($($quasi)*))
    };
}

impl Strategy {
    fn types(&self) -> Vec<syn::Type> {
        use self::Strategy::*;
        match self {
            Arbitrary(ty, _) => vec![ty.clone()],
            Regex(ty) => vec![ty.clone()],
            Existential(ty) => vec![ty.clone()],
            Value(ty) => vec![ty.clone()],
            Map(strats) => strats.iter().flat_map(|s| s.types()).collect(),
            Union(strats) => strats.iter().flat_map(|s| s.types()).collect(),
            Filter(_, ty) => vec![ty.clone()],
        }
    }
}

impl ToTokens for Strategy {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // The logic of each of these are pretty straight forward save for
        // union which is described separately.
        use self::Strategy::*;
        match self {
            Arbitrary(ty, span) => tokens.append_all(quote_spanned!(*span=>
                <#ty as _proptest::arbitrary::Arbitrary>::Strategy
            )),
            Regex(ty) => quote_append!(tokens,
                <#ty as _proptest::string::StrategyFromRegex>::Strategy
            ),
            Existential(ty) => quote_append!(tokens,
                _proptest::strategy::BoxedStrategy<#ty>
            ),
            Value(ty) => quote_append!(tokens, fn() -> #ty ),
            Map(strats) => {
                let types = self.types();
                let field_tys = NestedTuple(&types);
                let strats = NestedTuple(&strats);
                quote_append!(tokens,
                    _proptest::strategy::Map< ( #strats ),
                        fn( #field_tys ) -> Self
                    >
                )
            }
            #[cfg(not(feature = "boxed_union"))]
            Union(strats) => union_strat_to_tokens(tokens, strats),
            #[cfg(feature = "boxed_union")]
            Union(strats) => union_strat_to_tokens_boxed(tokens, strats),
            Filter(strat, ty) => quote_append!(tokens,
                _proptest::strategy::Filter<#strat, fn(&#ty) -> bool>
            ),
        }
    }
}

//==============================================================================
// Constructor
//==============================================================================

/// The right hand side (RHS) of a let binding of parameters.
pub enum FromReg {
    /// Denotes a move from the top parameter given in the arguments of
    /// `arbitrary_with`.
    Top,
    /// Denotes a move from a variable `params_<x>` where `<x>` is the given
    /// number.
    Num(usize),
}

/// The left hand side (LHS) of a let binding of parameters.
pub enum ToReg {
    /// Denotes a move and declaration to a sequence of variables from
    /// `params_0` to `params_x`.
    Range(usize),
    /// Denotes a move and declaration of a special variable `params` that is
    /// user facing and is ALWAYS named `params`.
    ///
    /// To change the name this linearises to is considered a breaking change
    /// wrt. semver.
    API,
}

/// Models an expression that generates a proptest `Strategy`.
pub enum Ctor {
    /// A strategy generated by using the `Arbitrary` impl for the given `Ty´.
    /// If `Some(idx)` is specified, then a parameter at `params_<idx>` is used
    /// and provided to `any_with::<Ty>(params_<idx>)`.
    Arbitrary(syn::Type, Option<usize>, Span),
    /// A strategy that is generated by a mapping a regex in the form of a
    /// string slice to the actual regex.
    Regex(syn::Type, syn::Expr),
    /// An exact strategy value given by the expression.
    Existential(syn::Expr),
    /// A strategy that always produces the given expression.
    Value(syn::Expr),
    /// A strategy that always produces the given expression but which is erased.
    ValueExistential(syn::Expr),
    /// A strategy that maps from a sequence of strategies into `Self`.
    Map(Box<[Ctor]>, MapClosure),
    /// A strategy that randomly selects one of the given relative-weighted
    /// strategies.
    Union(Box<[(u32, Ctor)]>),
    /// A let binding that moves to and declares the `ToReg` from the `FromReg`
    /// as well as the strategy that uses the `ToReg`.
    Extract(Box<Ctor>, ToReg, FromReg),
    /// A filtered strategy with `.prop_filter`.
    Filter(Box<Ctor>, syn::Expr),
}

/// Wraps the given strategy producing expression with a move into
/// `params_<to>` from `FromReg`. This is used when the given `c` expects
/// `params_<to>` to be there.
pub fn extract_all(c: Ctor, to: usize, from: FromReg) -> Ctor {
    extract(c, ToReg::Range(to), from)
}

/// Wraps the given strategy producing expression with a move into `params`
/// (literally named like that) from `FromReg`. This is used when the given
/// `c` expects `params` to be there.
pub fn extract_api(c: Ctor, from: FromReg) -> Ctor {
    extract(c, ToReg::API, from)
}

impl ToTokens for FromReg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FromReg::Top => call_site_ident(TOP_PARAM_NAME).to_tokens(tokens),
            FromReg::Num(reg) => param(*reg).to_tokens(tokens),
        }
    }
}

impl ToTokens for ToReg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            ToReg::Range(to) if to == 1 => param(0).to_tokens(tokens),
            ToReg::Range(to) => {
                let params: Vec<_> = (0..to).map(param).collect();
                NestedTuple(&params).to_tokens(tokens)
            }
            ToReg::API => call_site_ident(API_PARAM_NAME).to_tokens(tokens),
        }
    }
}

impl ToTokens for Ctor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // The logic of each of these are pretty straight forward save for
        // union which is described separately.
        use self::Ctor::*;
        match self {
            Filter(ctor, filter) => quote_append!(tokens,
                _proptest::strategy::Strategy::prop_filter(
                    #ctor, stringify!(#filter), #filter)
            ),
            Extract(ctor, to, from) => quote_append!(tokens, {
                let #to = #from; #ctor
            }),
            Arbitrary(ty, fv, span) => {
                tokens.append_all(if let Some(fv) = fv {
                    let args = param(*fv);
                    quote_spanned!(*span=>
                        _proptest::arbitrary::any_with::<#ty>(#args)
                    )
                } else {
                    quote_spanned!(*span=>
                        _proptest::arbitrary::any::<#ty>()
                    )
                })
            }
            Regex(ty, regex) => quote_append!(tokens,
                <#ty as _proptest::string::StrategyFromRegex>::from_regex(#regex)
            ),
            Existential(expr) => quote_append!(tokens,
                _proptest::strategy::Strategy::boxed( #expr ) ),
            Value(expr) => quote_append!(tokens, (|| #expr) as fn() -> _),
            ValueExistential(expr) => quote_append!(tokens,
                _proptest::strategy::Strategy::boxed(
                    _proptest::strategy::LazyJust::new(move || #expr)
                )
            ),
            Map(ctors, closure) => map_ctor_to_tokens(tokens, &ctors, closure),
            #[cfg(not(feature = "boxed_union"))]
            Union(ctors) => union_ctor_to_tokens(tokens, ctors),
            #[cfg(feature = "boxed_union")]
            Union(ctors) => union_ctor_to_tokens_boxed(tokens, ctors),
        }
    }
}

struct NestedTuple<'a, T>(&'a [T]);

impl<'a, T: ToTokens> ToTokens for NestedTuple<'a, T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let NestedTuple(elems) = self;
        if elems.is_empty() {
            quote_append!(tokens, ());
        } else if let [x] = elems {
            x.to_tokens(tokens);
        } else {
            let chunks = elems.chunks(NESTED_TUPLE_CHUNK_SIZE);
            Recurse(&chunks).to_tokens(tokens);
        }

        struct Recurse<'a, T: ToTokens>(&'a ::std::slice::Chunks<'a, T>);

        impl<'a, T: ToTokens> ToTokens for Recurse<'a, T> {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                let mut chunks = self.0.clone();
                if let Some(head) = chunks.next() {
                    if let [c] = head {
                        // Only one element left - no need to nest.
                        quote_append!(tokens, #c);
                    } else {
                        let tail = Recurse(&chunks);
                        quote_append!(tokens, (#(#head,)* #tail));
                    }
                }
            }
        }
    }
}

fn map_ctor_to_tokens(
    tokens: &mut TokenStream,
    ctors: &[Ctor],
    closure: &MapClosure,
) {
    let ctors = NestedTuple(ctors);

    quote_append!(tokens,
        _proptest::strategy::Strategy::prop_map(
            #ctors,
            #closure
        )
    );
}

/// Tokenizes a weighted list of `Ctor`.
///
/// The logic is that the output should be as linear as possible while still
/// supporting enums with an unbounded number of variants without any boxing
/// (erasure) or dynamic dispatch.
///
/// As `TupleUnion` is (currently) limited to 10 summands in the coproduct
/// we can't just emit the entire thing linearly as this will fail on the 11:th
/// variant.
///
/// A naive approach to solve might be to simply use a cons-list like so:
///
/// ```ignore
/// TupleUnion::new(
///     (w_1, s_1),
///     (w_2 + w_3 + w_4 + w_5,
///      TupleUnion::new(
///         (w_2, s_2),
///         (w_3 + w_4 + w_5,
///          TupleUnion::new(
///             (w_3, s_3),
///             (w_4 + w_5,
///              TupleUnion::new(
///                 (w_4, s_4),
///                 (w_5, s_5),
///             ))
///         ))
///     ))
/// )
/// ```
///
/// However, we can do better by being linear for the `10 - 1` first
/// strategies and then switch to nesting like so:
///
/// ```ignore
/// (1, 2, 3, 4, 5, 6, 7, 8, 9,
///     (10, 11, 12, 13, 14, 15, 16, 17, 18,
///         (19, ..)))
/// ```
#[cfg(not(feature = "boxed_union"))]
fn union_ctor_to_tokens(tokens: &mut TokenStream, ctors: &[(u32, Ctor)]) {
    if ctors.is_empty() {
        return;
    }

    if let [(_, ctor)] = ctors {
        // This is not a union at all - user provided an enum with one variant.
        ctor.to_tokens(tokens);
        return;
    }

    let mut chunks = ctors.chunks(UNION_CHUNK_SIZE);
    let chunk = chunks.next().unwrap();
    let head = chunk.iter().map(wrap_arc);
    let tail = Recurse(weight_sum(ctors) - weight_sum(chunk), chunks);

    quote_append!(tokens,
        _proptest::strategy::TupleUnion::new(( #(#head,)* #tail ))
    );

    struct Recurse<'a>(u32, ::std::slice::Chunks<'a, (u32, Ctor)>);

    impl<'a> ToTokens for Recurse<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let (tweight, mut chunks) = (self.0, self.1.clone());

            if let Some(chunk) = chunks.next() {
                if let [(w, c)] = chunk {
                    // Only one element left - no need to nest.
                    quote_append!(tokens, (#w, ::std::sync::Arc::new(#c)) );
                } else {
                    let head = chunk.iter().map(wrap_arc);
                    let tail = Recurse(tweight - weight_sum(chunk), chunks);
                    quote_append!(tokens,
                        (#tweight, ::std::sync::Arc::new(
                            _proptest::strategy::TupleUnion::new((
                                #(#head,)* #tail
                            ))))
                    );
                }
            }
        }
    }

    fn weight_sum(ctors: &[(u32, Ctor)]) -> u32 {
        use std::num::Wrapping;
        let Wrapping(x) = ctors.iter().map(|&(w, _)| Wrapping(w)).sum();
        x
    }

    fn wrap_arc(arg: &(u32, Ctor)) -> TokenStream {
        let (w, c) = arg;
        quote!( (#w, ::std::sync::Arc::new(#c)) )
    }
}

/// Tokenizes a weighted list of `Strategy`.
/// For details, see `union_ctor_to_tokens`.
#[cfg(not(feature = "boxed_union"))]
fn union_strat_to_tokens(tokens: &mut TokenStream, strats: &[Strategy]) {
    if strats.is_empty() {
        return;
    }

    if let [strat] = strats {
        // This is not a union at all - user provided an enum with one variant.
        strat.to_tokens(tokens);
        return;
    }

    let mut chunks = strats.chunks(UNION_CHUNK_SIZE);
    let chunk = chunks.next().unwrap();
    let head = chunk.iter().map(wrap_arc);
    let tail = Recurse(chunks);

    quote_append!(tokens,
        _proptest::strategy::TupleUnion<( #(#head,)* #tail )>
    );

    struct Recurse<'a>(::std::slice::Chunks<'a, Strategy>);

    impl<'a> ToTokens for Recurse<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let mut chunks = self.0.clone();

            if let Some(chunk) = chunks.next() {
                if let [s] = chunk {
                    // Only one element left - no need to nest.
                    quote_append!(tokens, (u32, ::std::sync::Arc<#s>) );
                } else {
                    let head = chunk.iter().map(wrap_arc);
                    let tail = Recurse(chunks);
                    quote_append!(tokens,
                        (u32,
                         ::std::sync::Arc<_proptest::strategy::TupleUnion<(
                             #(#head,)* #tail
                         )>>)
                    );
                }
            }
        }
    }

    fn wrap_arc(s: &Strategy) -> TokenStream {
        quote!( (u32, ::std::sync::Arc<#s>) )
    }
}

/// Tokenizes a weighted list of `Ctor`.
///
/// This can be used instead of `union_ctor_to_tokens` to generate a boxing
/// macro.
#[cfg(feature = "boxed_union")]
fn union_ctor_to_tokens_boxed(tokens: &mut TokenStream, ctors: &[(u32, Ctor)]) {
    if ctors.is_empty() {
        return;
    }

    if let [(_, ctor)] = ctors {
        // This is not a union at all - user provided an enum with one variant.
        ctor.to_tokens(tokens);
        return;
    }

    let ctors_boxed = ctors.iter().map(wrap_boxed);

    quote_append!(
        tokens,
        _proptest::strategy::Union::new_weighted(vec![ #(#ctors_boxed,)* ])
    );

    fn wrap_boxed(arg: &(u32, Ctor)) -> TokenStream {
        let (w, c) = arg;
        quote!( (#w, _proptest::strategy::Strategy::boxed(#c)) )
    }
}

/// Tokenizes a weighted list of `Strategy`.
/// For details, see `union_ctor_to_tokens_boxed`.
#[cfg(feature = "boxed_union")]
fn union_strat_to_tokens_boxed(tokens: &mut TokenStream, strats: &[Strategy]) {
    if strats.is_empty() {
        return;
    }

    if let [strat] = strats {
        // This is not a union at all - user provided an enum with one variant.
        strat.to_tokens(tokens);
        return;
    }

    quote_append!(
        tokens,
        _proptest::strategy::Union<_proptest::strategy::BoxedStrategy<Self>>
    );
}

/// Wraps a `Ctor` that expects the `to` "register" to be filled with
/// contents of the `from` register. The correctness of this wrt. the
/// generated Rust code has to be verified externally by checking the
/// construction of the particular `Ctor`.
fn extract(c: Ctor, to: ToReg, from: FromReg) -> Ctor {
    Ctor::Extract(Box::new(c), to, from)
}

/// Construct a `FreshVar` prefixed by `param_`.
fn param<'a>(fv: usize) -> FreshVar<'a> {
    fresh_var("param", fv)
}

//==============================================================================
// MapClosure
//==============================================================================

/// Constructs a `MapClosure` for the given `path` and a list of fields.
pub fn map_closure(path: syn::Path, fs: &[syn::Field]) -> MapClosure {
    MapClosure(path, fs.to_owned())
}

/// A `MapClosure` models the closure part inside a `.prop_map(..)` call.
#[derive(Debug)]
pub struct MapClosure(syn::Path, Vec<syn::Field>);

impl ToTokens for MapClosure {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        fn tmp_var<'a>(idx: usize) -> FreshVar<'a> {
            fresh_var("tmp", idx)
        }

        let MapClosure(path, fields) = self;
        let count = fields.len();
        let tmps: Vec<_> = (0..count).map(tmp_var).collect();
        let inits = fields.iter().enumerate().map(|(idx, field)| {
            let tv = tmp_var(idx);
            if let Some(name) = &field.ident {
                quote_spanned!(field.span()=> #name: #tv )
            } else {
                let name = syn::Member::Unnamed(syn::Index::from(idx));
                quote_spanned!(field.span()=> #name: #tv )
            }
        });
        let tmps = NestedTuple(&tmps);
        quote_append!(tokens, | #tmps | #path { #(#inits),* } );
    }
}

//==============================================================================
// FreshVar
//==============================================================================

/// Construct a `FreshVar` with the given `prefix` and the number it has in the
/// count of temporaries for that prefix.
fn fresh_var(prefix: &str, count: usize) -> FreshVar<'_> {
    FreshVar { prefix, count }
}

/// A `FreshVar` is an internal implementation detail and models a temporary
/// variable on the stack.
struct FreshVar<'a> {
    prefix: &'a str,
    count: usize,
}

impl<'a> ToTokens for FreshVar<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format!("{}_{}", self.prefix, self.count);
        call_site_ident(&ident).to_tokens(tokens)
    }
}

fn call_site_ident(ident: &str) -> syn::Ident {
    syn::Ident::new(ident, Span::call_site())
}
