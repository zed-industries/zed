use clippy_utils::SpanlessEq;
use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::sym;
use clippy_utils::visitors::{Visitable, for_each_expr_without_closures};
use rustc_hir::def_id::DefId;
use rustc_hir::{Expr, ExprKind, LetStmt, MatchSource};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Symbol;
use std::ops::ControlFlow;

rustc_session::declare_lint! {
    /// ### What it does
    ///
    /// Flags control flow that branches on map value lookup methods and then
    /// inserts into the same map with the same key.
    ///
    /// The lint applies to any map type with an inherent `entry` method: std's
    /// `HashMap` and `BTreeMap`, hasher aliases such as `FxHashMap`, and
    /// third-party maps such as `hashbrown::HashMap` or `indexmap::IndexMap`.
    ///
    /// The lint covers direct `match`, `if let`, `is_some`/`is_none`, and
    /// `let ... else` forms using `get`, `get_mut`, or `get_key_value`.
    ///
    /// The same pattern on `HashSet` and `BTreeSet` is also flagged. Sets have
    /// no stable entry API, and need none: `insert` alone searches once, skips
    /// duplicates, and returns `false` when the value was already present.
    ///
    /// Forms that Clippy already rewrites are excluded. `contains_key` or
    /// `contains` followed by `insert` is `map_entry` or
    /// `set_contains_or_insert`, and `get(..).is_some()`/`is_none()` on the
    /// std collections is `unnecessary_get_then_check`, whose `contains_key`/
    /// `contains` suggestion feeds the former two.
    ///
    /// ### Why is this bad?
    ///
    /// The lookup searches the map once and `insert` searches it again. The
    /// map's entry API can reuse the result of one search.
    pub MAP_LOOKUP_THEN_INSERT,
    Warn,
    "a map value lookup is followed by insertion for the same key"
}

pub(crate) struct MapLookupThenInsert;

rustc_session::impl_lint_pass!(MapLookupThenInsert => [MAP_LOOKUP_THEN_INSERT]);

impl<'tcx> LateLintPass<'tcx> for MapLookupThenInsert {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }

        let repeated_lookup = match &expr.kind {
            ExprKind::If(condition, then_expression, else_expression) => {
                lookup_from_condition(cx, condition).filter(|&lookup| {
                    contains_matching_insert(cx, lookup, *then_expression)
                        || else_expression.is_some_and(|expression| {
                            contains_matching_insert(cx, lookup, expression)
                        })
                })
            }
            ExprKind::Match(scrutinee, arms, MatchSource::Normal) => parse_lookup(cx, scrutinee)
                .filter(|&lookup| {
                    arms.iter()
                        .any(|arm| contains_matching_insert(cx, lookup, arm.body))
                }),
            _ => None,
        };

        if let Some(lookup) = repeated_lookup {
            emit_lint(cx, expr.span, lookup);
        }
    }

    fn check_local(&mut self, cx: &LateContext<'tcx>, local: &'tcx LetStmt<'tcx>) {
        if local.span.from_expansion() {
            return;
        }

        let Some(init) = local.init else {
            return;
        };
        let Some(else_block) = local.els else {
            return;
        };
        let Some(lookup) = parse_lookup(cx, init) else {
            return;
        };

        if contains_matching_insert(cx, lookup, else_block) {
            emit_lint(cx, local.span, lookup);
        }
    }
}

#[derive(Clone, Copy)]
struct Lookup<'tcx> {
    map: &'tcx Expr<'tcx>,
    key: &'tcx Expr<'tcx>,
}

fn lookup_from_condition<'tcx>(
    cx: &LateContext<'tcx>,
    condition: &'tcx Expr<'tcx>,
) -> Option<Lookup<'tcx>> {
    match &condition.kind {
        ExprKind::Let(let_expression) => parse_lookup(cx, let_expression.init),
        ExprKind::MethodCall(segment, receiver, [], _)
            if matches!(segment.ident.name.as_str(), "is_some" | "is_none")
                && !clippy_covers_get_then_check(cx, receiver) =>
        {
            parse_lookup(cx, receiver)
        }
        _ => None,
    }
}

fn parse_lookup<'tcx>(
    cx: &LateContext<'tcx>,
    expression: &'tcx Expr<'tcx>,
) -> Option<Lookup<'tcx>> {
    let expression = match &expression.kind {
        ExprKind::MethodCall(segment, receiver, [], _)
            if matches!(segment.ident.name.as_str(), "copied" | "cloned") =>
        {
            receiver
        }
        _ => expression,
    };

    let ExprKind::MethodCall(segment, map, [key], _) = &expression.kind else {
        return None;
    };
    if !matches!(
        segment.ident.name.as_str(),
        "get" | "get_mut" | "get_key_value"
    ) {
        return None;
    }

    let map_type = cx.typeck_results().expr_ty_adjusted(map).peel_refs();
    let map_definition = map_type.ty_adt_def()?;
    if !is_std_set(cx, map_definition.did()) && !has_inherent_entry_method(cx, map_definition.did())
    {
        return None;
    }

    let key = peel_borrow(key);
    if map.can_have_side_effects() || key.can_have_side_effects() {
        return None;
    }

    Some(Lookup { map, key })
}

fn peel_borrow<'tcx>(expression: &'tcx Expr<'tcx>) -> &'tcx Expr<'tcx> {
    match &expression.kind {
        ExprKind::AddrOf(_, _, inner) => inner,
        _ => expression,
    }
}

fn contains_matching_insert<'tcx>(
    cx: &LateContext<'tcx>,
    lookup: Lookup<'tcx>,
    node: impl Visitable<'tcx>,
) -> bool {
    let mut equality = SpanlessEq::new(cx);
    for_each_expr_without_closures(node, |expression| {
        if parse_insert(expression).is_some_and(|insert| {
            equality.eq_expr(lookup.map, insert.map) && equality.eq_expr(lookup.key, insert.key)
        }) {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    })
    .is_some()
}

fn parse_insert<'tcx>(expression: &'tcx Expr<'tcx>) -> Option<Lookup<'tcx>> {
    // Maps insert `(key, value)`; sets insert only the value.
    let ExprKind::MethodCall(segment, map, [key] | [key, _], _) = &expression.kind else {
        return None;
    };
    if segment.ident.name.as_str() != "insert" {
        return None;
    }

    Some(Lookup { map, key })
}

/// Clippy already rewrites `get(..).is_some()`/`is_none()` on the std maps
/// and sets: `unnecessary_get_then_check` turns the condition into
/// `contains_key`/`contains`, which `map_entry`/`set_contains_or_insert` then
/// turn into the final form. Defer to that chain instead of duplicating it.
/// Conditions using `get_mut` or `get_key_value`, and any `get` on a non-std
/// map, are not covered by Clippy and stay ours.
fn clippy_covers_get_then_check(cx: &LateContext<'_>, receiver: &Expr<'_>) -> bool {
    let ExprKind::MethodCall(segment, map, [_], _) = &receiver.kind else {
        return false;
    };
    if segment.ident.name.as_str() != "get" {
        return false;
    }
    cx.typeck_results()
        .expr_ty_adjusted(map)
        .peel_refs()
        .ty_adt_def()
        .is_some_and(|definition| {
            matches!(
                cx.tcx.get_diagnostic_name(definition.did()),
                Some(sym::HashMap | sym::BTreeMap | sym::HashSet | sym::BTreeSet)
            )
        })
}

/// Sets are gated by diagnostic name rather than by an `entry` method: their
/// entry API is unstable (`hash_set_entry`), and the suggested rewrite (plain
/// `insert`) relies on std's no-op-when-present insertion semantics.
fn is_std_set(cx: &LateContext<'_>, definition: DefId) -> bool {
    matches!(
        cx.tcx.get_diagnostic_name(definition),
        Some(sym::HashSet | sym::BTreeSet)
    )
}

/// A lookup-then-insert pattern is only worth reporting when the map offers an
/// entry API to rewrite it with. Rather than hard-coding `HashMap` and
/// `BTreeMap`, accept any type whose inherent impls provide an `entry` method;
/// this also covers maps like `hashbrown::HashMap` and `indexmap::IndexMap`.
/// Hasher aliases such as `FxHashMap` need no special case because they
/// resolve to the underlying map type.
fn has_inherent_entry_method(cx: &LateContext<'_>, map_definition: DefId) -> bool {
    let entry = Symbol::intern("entry");
    cx.tcx.inherent_impls(map_definition).iter().any(|&imp| {
        cx.tcx
            .associated_items(imp)
            .filter_by_name_unhygienic(entry)
            .next()
            .is_some()
    })
}

fn emit_lint(cx: &LateContext<'_>, span: rustc_span::Span, lookup: Lookup<'_>) {
    let is_set = cx
        .typeck_results()
        .expr_ty_adjusted(lookup.map)
        .peel_refs()
        .ty_adt_def()
        .is_some_and(|definition| is_std_set(cx, definition.did()));

    let (message, help) = if is_set {
        (
            "this set lookup and insertion search for the same value twice",
            "`insert` alone searches once, skips duplicates, and returns `false` \
             when the value was already present",
        )
    } else {
        (
            "this map lookup and insertion search for the same key twice",
            "to search the map only once, use the entry API: \
             https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html",
        )
    };
    span_lint_and_help(cx, MAP_LOOKUP_THEN_INSERT, span, message, None, help);
}
