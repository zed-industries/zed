use clippy_utils::diagnostics::span_lint;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};

use crate::render_helpers::{is_directly_in_render_method, is_gpui_context};

rustc_session::declare_lint! {
    /// ### What it does
    ///
    /// Flags calls to `Context::notify()` that execute synchronously inside a
    /// `Render::render` method.
    ///
    /// ### Why is this bad?
    ///
    /// `notify()` tells the framework that the entity's state has changed and
    /// it should be re-rendered. Calling it during render means every render
    /// pass schedules another render pass — either an infinite loop or wasted
    /// work.
    pub NOTIFY_IN_RENDER,
    Warn,
    "calling `cx.notify()` during render schedules a redundant re-render"
}

pub(crate) struct NotifyInRender;

rustc_session::impl_lint_pass!(NotifyInRender => [NOTIFY_IN_RENDER]);

impl<'tcx> LateLintPass<'tcx> for NotifyInRender {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }

        let ExprKind::MethodCall(segment, receiver, _args, _span) = &expr.kind else {
            return;
        };

        if segment.ident.name.as_str() != "notify" {
            return;
        }

        let receiver_ty = cx.typeck_results().expr_ty(receiver);
        if !is_gpui_context(cx, receiver_ty) {
            return;
        }

        if !is_directly_in_render_method(cx, expr.hir_id) {
            return;
        }

        span_lint(
            cx,
            NOTIFY_IN_RENDER,
            expr.span,
            "`cx.notify()` called during render schedules a re-render every render pass",
        );
    }
}
