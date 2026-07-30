use clippy_utils::diagnostics::span_lint;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};

use crate::render_helpers::{
    is_directly_in_render_method, is_gpui_entity_or_weak, is_unit_or_result_unit,
};

rustc_session::declare_lint! {
    /// ### What it does
    ///
    /// Flags calls to `Entity::update` or `WeakEntity::update` that execute
    /// synchronously inside a `Render::render` or `RenderOnce::render` method
    /// and whose closure returns `()` (indicating mutation rather than reading).
    ///
    /// ### Why is this bad?
    ///
    /// The `render` method should be a pure function of state. Calling
    /// `.update()` mutates an entity during the render pass, which can trigger
    /// re-renders mid-render and lead to inconsistent UI state or infinite
    /// render loops.
    pub ENTITY_UPDATE_IN_RENDER,
    Warn,
    "mutating an entity via `.update()` during render"
}

pub(crate) struct EntityUpdateInRender;

rustc_session::impl_lint_pass!(EntityUpdateInRender => [ENTITY_UPDATE_IN_RENDER]);

impl<'tcx> LateLintPass<'tcx> for EntityUpdateInRender {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }

        let ExprKind::MethodCall(segment, receiver, _args, _span) = &expr.kind else {
            return;
        };

        if segment.ident.name.as_str() != "update" {
            return;
        }

        let receiver_ty = cx.typeck_results().expr_ty(receiver);
        if !is_gpui_entity_or_weak(cx, receiver_ty) {
            return;
        }

        let call_ty = cx.typeck_results().expr_ty(expr);
        if !is_unit_or_result_unit(cx, call_ty) {
            return;
        }

        if !is_directly_in_render_method(cx, expr.hir_id) {
            return;
        }

        span_lint(
            cx,
            ENTITY_UPDATE_IN_RENDER,
            expr.span,
            "entity `.update()` called during render mutates state in the render pass",
        );
    }
}
