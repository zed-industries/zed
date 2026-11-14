use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::visitors::{Visitable, for_each_expr_without_closures};
use clippy_utils::{SpanlessEq, expr_or_init, sym};
use rustc_hir::def_id::DefId;
use rustc_hir::{Expr, ExprKind, LangItem, LetStmt, MatchSource, UnOp};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
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
    /// Keys are compared after stripping borrows and derefs, peeling
    /// conversions that yield an owned key equal to the original (`clone`,
    /// `to_owned`, `to_string` on string types, `to_vec` on slices, and
    /// `String::from` on `&str`), and resolving immutable local aliases to
    /// their initializers. So looking up `&key` and inserting `key.clone()`
    /// is recognized as the same key. Each peeled conversion is resolved by
    /// definition, not by name, so an unrelated user method that happens to
    /// be called `clone` does not count.
    ///
    /// When such a conversion is peeled, the diagnostic carries a note: the
    /// entry API takes the key by value up front, so a conversion the
    /// original code performs only on the miss path would run on every call.
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
                lookup_from_condition(cx, condition).and_then(|lookup| {
                    find_matching_insert(cx, lookup, *then_expression)
                        .or_else(|| {
                            else_expression
                                .and_then(|expression| find_matching_insert(cx, lookup, expression))
                        })
                        .map(|key_converted| (lookup, key_converted))
                })
            }
            ExprKind::Match(scrutinee, arms, MatchSource::Normal) => parse_lookup(cx, scrutinee)
                .and_then(|lookup| {
                    arms.iter()
                        .find_map(|arm| find_matching_insert(cx, lookup, arm.body))
                        .map(|key_converted| (lookup, key_converted))
                }),
            _ => None,
        };

        if let Some((lookup, key_converted)) = repeated_lookup {
            emit_lint(cx, expr.span, lookup, key_converted);
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

        if let Some(key_converted) = find_matching_insert(cx, lookup, else_block) {
            emit_lint(cx, local.span, lookup, key_converted);
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

    // The purity check runs on the peeled key so that borrows and
    // value-preserving conversions do not count as side effects.
    let mut key_converted = false;
    let peeled_key = peel_key_expr(cx, key, &mut key_converted);
    if map.can_have_side_effects() || peeled_key.can_have_side_effects() {
        return None;
    }

    Some(Lookup { map, key })
}

/// Searches `node` for an insertion into `lookup`'s map under `lookup`'s key.
/// Returns `Some(key_converted)` when one is found, where `key_converted`
/// records whether the match required peeling an owned-key conversion.
fn find_matching_insert<'tcx>(
    cx: &LateContext<'tcx>,
    lookup: Lookup<'tcx>,
    node: impl Visitable<'tcx>,
) -> Option<bool> {
    for_each_expr_without_closures(node, |expression| {
        if let Some(insert) = parse_insert(expression)
            && SpanlessEq::new(cx).eq_expr(lookup.map, insert.map)
            && let Some(key_converted) = keys_match(cx, lookup.key, insert.key)
        {
            ControlFlow::Break(key_converted)
        } else {
            ControlFlow::Continue(())
        }
    })
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

/// Checks whether the two key expressions denote the same key value. Returns
/// `Some(key_converted)` on a match, where `key_converted` records whether an
/// owned-key conversion had to be peeled to see it.
///
/// Two forms are recognized:
/// * the keys are equal after peeling borrows, derefs, and value-preserving
///   conversions, e.g. looking up `&entry.key` and inserting
///   `entry.key.clone()`;
/// * after additionally resolving immutable local aliases to their
///   initializers, e.g. `let owned = key.clone();` before a lookup of `&key`
///   and an insertion of `owned`. Both resolved forms must be free of side
///   effects: two structurally equal side-effecting initializers (say, two
///   separate `next_key()` calls) are two different key values.
fn keys_match<'tcx>(
    cx: &LateContext<'tcx>,
    lookup_key: &'tcx Expr<'tcx>,
    insert_key: &'tcx Expr<'tcx>,
) -> Option<bool> {
    let mut key_converted = false;
    let lookup_key = peel_key_expr(cx, lookup_key, &mut key_converted);
    let insert_key = peel_key_expr(cx, insert_key, &mut key_converted);
    if SpanlessEq::new(cx).eq_expr(lookup_key, insert_key) {
        return Some(key_converted);
    }

    let lookup_key = resolve_local_aliases(cx, lookup_key, &mut key_converted);
    let insert_key = resolve_local_aliases(cx, insert_key, &mut key_converted);
    if !lookup_key.can_have_side_effects()
        && !insert_key.can_have_side_effects()
        && SpanlessEq::new(cx).eq_expr(lookup_key, insert_key)
    {
        return Some(key_converted);
    }

    None
}

/// Follows immutable local bindings back to their initializers, peeling each
/// initializer down to its underlying key expression. `expr_or_init` only
/// resolves bindings that cannot be reassigned, so the binding's value is the
/// initializer's value wherever the binding is referenced.
fn resolve_local_aliases<'tcx>(
    cx: &LateContext<'tcx>,
    mut expression: &'tcx Expr<'tcx>,
    key_converted: &mut bool,
) -> &'tcx Expr<'tcx> {
    loop {
        let resolved = expr_or_init(cx, expression);
        if resolved.hir_id == expression.hir_id {
            return expression;
        }
        expression = peel_key_expr(cx, resolved, key_converted);
    }
}

/// Strips layers that do not change which slot of the map the key selects:
/// borrows, derefs, and owned-key conversions of the underlying expression.
fn peel_key_expr<'tcx>(
    cx: &LateContext<'tcx>,
    mut expression: &'tcx Expr<'tcx>,
    key_converted: &mut bool,
) -> &'tcx Expr<'tcx> {
    loop {
        match &expression.kind {
            ExprKind::AddrOf(_, _, inner) | ExprKind::Unary(UnOp::Deref, inner) => {
                expression = inner;
            }
            _ => match peel_value_preserving_conversion(cx, expression) {
                Some(inner) => {
                    *key_converted = true;
                    expression = inner;
                }
                None => return expression,
            },
        }
    }
}

/// If `expression` is a conversion that yields an owned key equal to its
/// input — under the same `Eq`/`Hash`/`Ord` contract the map lookup itself
/// relies on — returns the input.
///
/// Each conversion is resolved by definition, not by name, so a user-defined
/// method that merely happens to be called `clone` or `to_owned` does not
/// count. For the `ToOwned` family, the `Borrow` contract (the owned and
/// borrowed forms hash and compare identically) is the same contract that
/// makes the map lookup itself well-defined. For `Clone`, the original code
/// already inserts the clone under the probed key, so the lint assumes
/// nothing the code did not.
fn peel_value_preserving_conversion<'tcx>(
    cx: &LateContext<'tcx>,
    expression: &'tcx Expr<'tcx>,
) -> Option<&'tcx Expr<'tcx>> {
    match &expression.kind {
        ExprKind::MethodCall(segment, receiver, [], _) => {
            let method = cx
                .typeck_results()
                .type_dependent_def_id(expression.hir_id)?;
            let preserves_value = match segment.ident.name.as_str() {
                "clone" => is_assoc_of_diag_trait(cx, method, sym::Clone),
                "to_owned" => is_assoc_of_diag_trait(cx, method, sym::ToOwned),
                // `ToString` runs an arbitrary `Display` impl; only the
                // string types are guaranteed to format as themselves.
                "to_string" => {
                    is_assoc_of_diag_trait(cx, method, sym::ToString) && {
                        let receiver_type =
                            cx.typeck_results().expr_ty_adjusted(receiver).peel_refs();
                        receiver_type.is_str() || is_string(cx, receiver_type)
                    }
                }
                "to_vec" => cx.tcx.impl_of_assoc(method).is_some_and(|impl_id| {
                    cx.tcx.type_of(impl_id).instantiate_identity().is_slice()
                }),
                _ => false,
            };
            preserves_value.then_some(receiver)
        }
        // UFCS forms: `Clone::clone(&key)`, `ToOwned::to_owned(&key)`, and
        // `String::from(key)` where `key` is a `&str`.
        ExprKind::Call(callee, [argument]) => {
            let ExprKind::Path(qpath) = &callee.kind else {
                return None;
            };
            let function = cx.qpath_res(qpath, callee.hir_id).opt_def_id()?;
            let preserves_value = is_assoc_of_diag_trait(cx, function, sym::Clone)
                || is_assoc_of_diag_trait(cx, function, sym::ToOwned)
                || (is_assoc_of_diag_trait(cx, function, sym::From)
                    && is_string(cx, cx.typeck_results().expr_ty(expression))
                    && cx.typeck_results().expr_ty(argument).peel_refs().is_str());
            preserves_value.then_some(argument)
        }
        _ => None,
    }
}

/// Whether `item` is an associated item of the trait named by the diagnostic
/// item `trait_name`. Qualified paths can resolve to a trait impl's item
/// rather than the trait's, so impl items are first mapped back to the trait
/// item they implement.
fn is_assoc_of_diag_trait(cx: &LateContext<'_>, item: DefId, trait_name: Symbol) -> bool {
    let item = cx
        .tcx
        .opt_associated_item(item)
        .and_then(|associated| associated.trait_item_def_id())
        .unwrap_or(item);
    cx.tcx
        .trait_of_assoc(item)
        .is_some_and(|trait_id| cx.tcx.is_diagnostic_item(trait_name, trait_id))
}

fn is_string(cx: &LateContext<'_>, ty: Ty<'_>) -> bool {
    ty.ty_adt_def()
        .is_some_and(|definition| cx.tcx.is_lang_item(definition.did(), LangItem::String))
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

fn emit_lint(
    cx: &LateContext<'_>,
    span: rustc_span::Span,
    lookup: Lookup<'_>,
    key_converted: bool,
) {
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
    span_lint_and_then(cx, MAP_LOOKUP_THEN_INSERT, span, message, |diagnostic| {
        diagnostic.help(help);
        if key_converted {
            diagnostic.note(
                "the inserted key is an owned copy of the looked-up key; a rewrite that \
                 passes the key by value up front would make that copy on hits as well \
                 as misses",
            );
        }
    });
}
