use std::collections::HashSet;

use clippy_utils::diagnostics::span_lint;
use clippy_utils::paths::{PathNS, lookup_path_str};
use clippy_utils::ty::implements_trait;
use rustc_hir::attrs::CfgEntry;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_hir::{Expr, ExprKind, HirId};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{self, Ty};
use rustc_span::sym;

rustc_session::declare_lint! {
    /// ### What it does
    ///
    /// Flags `gpui::AppContext::background_spawn` calls whose spawned future
    /// contains state that does not implement `gpui::WorkerSend`.
    ///
    /// Closures and futures are treated like auto traits: they satisfy
    /// `WorkerSend` when all data stored in them satisfies `WorkerSend`.
    /// Calls in code that is provably excluded from `wasm32` are ignored.
    pub EXTERN_FN_IN_BACKGROUND_SPAWN,
    Warn,
    "non-WorkerSend data captured by a GPUI background task that can compile for wasm"
}

pub(crate) struct ExternFnInBackgroundSpawn;

rustc_session::impl_lint_pass!(ExternFnInBackgroundSpawn => [EXTERN_FN_IN_BACKGROUND_SPAWN]);

impl<'tcx> LateLintPass<'tcx> for ExternFnInBackgroundSpawn {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let ExprKind::MethodCall(_, _, [spawned], _) = expr.kind else {
            return;
        };
        let Some(def_id) = cx.typeck_results().type_dependent_def_id(expr.hir_id) else {
            return;
        };
        if cx.tcx.crate_name(def_id.krate).as_str() != "gpui"
            || cx.tcx.item_name(def_id).as_str() != "background_spawn"
            || is_excluded_from_wasm(cx, expr.hir_id)
        {
            return;
        }
        let Some(worker_send_trait) = worker_send_trait(cx) else {
            return;
        };

        let spawned_type = cx
            .tcx
            .erase_and_anonymize_regions(cx.typeck_results().expr_ty(spawned));
        if !is_worker_send(cx, worker_send_trait, spawned_type, &mut HashSet::new()) {
            span_lint(
                cx,
                EXTERN_FN_IN_BACKGROUND_SPAWN,
                expr.span,
                "GPUI background task contains data that does not implement `WorkerSend` for wasm",
            );
        }
    }
}

fn worker_send_trait(cx: &LateContext<'_>) -> Option<DefId> {
    lookup_path_str(cx.tcx, PathNS::Type, "gpui::WorkerSend")
        .into_iter()
        .find(|def_id| cx.tcx.def_kind(*def_id) == DefKind::Trait)
}

fn is_worker_send<'tcx>(
    cx: &LateContext<'tcx>,
    worker_send_trait: DefId,
    ty: Ty<'tcx>,
    checking: &mut HashSet<Ty<'tcx>>,
) -> bool {
    let ty = cx.tcx.erase_and_anonymize_regions(ty);
    if implements_trait(cx, ty, worker_send_trait, &[]) {
        return true;
    }
    if !checking.insert(ty) {
        return true;
    }

    let result = match ty.kind() {
        ty::Closure(_, arguments) => arguments
            .as_closure()
            .upvar_tys()
            .iter()
            .all(|ty| is_worker_send(cx, worker_send_trait, ty, checking)),
        ty::CoroutineClosure(_, arguments) => arguments
            .as_coroutine_closure()
            .upvar_tys()
            .iter()
            .all(|ty| is_worker_send(cx, worker_send_trait, ty, checking)),
        ty::Coroutine(def_id, arguments) => {
            let upvars_are_worker_send = arguments
                .as_coroutine()
                .upvar_tys()
                .iter()
                .all(|ty| is_worker_send(cx, worker_send_trait, ty, checking));
            let hidden_types = cx
                .tcx
                .coroutine_hidden_types(*def_id)
                .instantiate(cx.tcx, arguments);
            let hidden_types = cx.tcx.instantiate_bound_regions_with_erased(hidden_types);
            upvars_are_worker_send
                && hidden_types
                    .types
                    .iter()
                    .all(|ty| is_worker_send(cx, worker_send_trait, ty, checking))
        }
        ty::Alias(ty::Opaque, alias) => {
            let hidden_type = cx.tcx.type_of(alias.def_id).instantiate(cx.tcx, alias.args);
            is_worker_send(cx, worker_send_trait, hidden_type, checking)
        }
        _ => false,
    };

    checking.remove(&ty);
    result
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WasmValue {
    True,
    False,
    Unknown,
}

fn is_excluded_from_wasm(cx: &LateContext<'_>, hir_id: HirId) -> bool {
    std::iter::once(hir_id)
        .chain(cx.tcx.hir_parent_id_iter(hir_id))
        .filter_map(
            |hir_id| rustc_hir::find_attr!(cx.tcx.hir_attrs(hir_id), CfgTrace(entries) => entries),
        )
        .flatten()
        .any(|(entry, _)| cfg_value_on_wasm(entry) == WasmValue::False)
}

fn cfg_value_on_wasm(entry: &CfgEntry) -> WasmValue {
    match entry {
        CfgEntry::All(entries, _) => {
            if entries
                .iter()
                .any(|entry| cfg_value_on_wasm(entry) == WasmValue::False)
            {
                WasmValue::False
            } else if entries
                .iter()
                .all(|entry| cfg_value_on_wasm(entry) == WasmValue::True)
            {
                WasmValue::True
            } else {
                WasmValue::Unknown
            }
        }
        CfgEntry::Any(entries, _) => {
            if entries
                .iter()
                .any(|entry| cfg_value_on_wasm(entry) == WasmValue::True)
            {
                WasmValue::True
            } else if entries
                .iter()
                .all(|entry| cfg_value_on_wasm(entry) == WasmValue::False)
            {
                WasmValue::False
            } else {
                WasmValue::Unknown
            }
        }
        CfgEntry::Not(entry, _) => match cfg_value_on_wasm(entry) {
            WasmValue::True => WasmValue::False,
            WasmValue::False => WasmValue::True,
            WasmValue::Unknown => WasmValue::Unknown,
        },
        CfgEntry::Bool(value, _) => {
            if *value {
                WasmValue::True
            } else {
                WasmValue::False
            }
        }
        CfgEntry::NameValue { name, value, .. } if *name == sym::target_arch => {
            if *value == Some(sym::wasm32) {
                WasmValue::True
            } else {
                WasmValue::False
            }
        }
        CfgEntry::NameValue { .. } | CfgEntry::Version(_, _) => WasmValue::Unknown,
    }
}
