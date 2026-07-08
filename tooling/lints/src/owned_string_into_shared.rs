use clippy_utils::diagnostics::span_lint;
use rustc_ast::ast::LitKind;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;

rustc_session::declare_lint! {
    /// ### What it does
    ///
    /// Flags expressions that build an owned `String` from a string literal
    /// and then immediately convert it with `.into()` into one of the
    /// refcounted/shared string types: `gpui::SharedString`, `Arc<str>`,
    /// `Rc<str>`, or `Cow<'_, str>`.
    ///
    /// The flagged shapes are:
    ///
    /// ```ignore
    /// let label: SharedString = String::from("foo").into();
    /// let key:   Arc<str>     = "foo".to_string().into();
    /// let value: Rc<str>      = "foo".to_owned().into();
    /// ```
    ///
    /// ### Why is this bad?
    ///
    /// Two heap allocations and two copies of the literal happen where one is
    /// enough: `String::from` (or `to_string`/`to_owned`) allocates a `String`
    /// and copies the bytes; the `.into()` conversion into the refcounted
    /// destination then allocates an `Arc<str>`-like buffer and copies the
    /// bytes a second time. For string literals the destination can be built
    /// directly from `'static` data with no allocation at all.
    pub OWNED_STRING_INTO_SHARED,
    Warn,
    "an owned `String` is built from a string literal only to be converted into a refcounted string"
}

pub(crate) struct OwnedStringIntoShared;

rustc_session::impl_lint_pass!(OwnedStringIntoShared => [OWNED_STRING_INTO_SHARED]);

impl<'tcx> LateLintPass<'tcx> for OwnedStringIntoShared {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }

        let ExprKind::MethodCall(segment, receiver, [], _) = &expr.kind else {
            return;
        };
        if segment.ident.name.as_str() != "into" {
            return;
        }

        let dest_ty = cx.typeck_results().expr_ty(expr);
        if !is_refcounted_string_destination(cx, dest_ty) {
            return;
        }

        // The receiver must produce an owned `String`. Confirming this rules
        // out custom `into` impls on unrelated types that just happen to look
        // similar.
        let receiver_ty = cx.typeck_results().expr_ty(receiver);
        if !is_std_string(cx, receiver_ty) {
            return;
        }

        if !is_owned_string_built_from_literal(cx, receiver) {
            return;
        }

        span_lint(
            cx,
            OWNED_STRING_INTO_SHARED,
            expr.span,
            "this allocates an owned `String` from a string literal only to convert it into a refcounted string",
        );
    }
}

/// Returns `true` when `ty` is one of the refcounted/shared string types this
/// lint targets: `gpui::SharedString`, `Arc<str>`, `Rc<str>`, or
/// `Cow<'_, str>`.
fn is_refcounted_string_destination<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> bool {
    let Some(adt) = ty.ty_adt_def() else {
        return false;
    };
    let did = adt.did();

    if cx.tcx.crate_name(did.krate).as_str() == "gpui_shared_string"
        && cx.tcx.item_name(did).as_str() == "SharedString"
    {
        return true;
    }

    let path = cx.tcx.def_path_str(did);
    let is_str_wrapper = matches!(
        path.as_str(),
        "alloc::sync::Arc"
            | "std::sync::Arc"
            | "alloc::rc::Rc"
            | "std::rc::Rc"
            | "alloc::borrow::Cow"
            | "std::borrow::Cow"
    );
    if !is_str_wrapper {
        return false;
    }

    let rustc_middle::ty::TyKind::Adt(_, args) = ty.kind() else {
        return false;
    };
    args.iter()
        .find_map(|arg| arg.as_type())
        .is_some_and(|inner| inner.is_str())
}

/// Returns `true` when `ty` is `alloc::string::String`.
fn is_std_string(cx: &LateContext<'_>, ty: Ty<'_>) -> bool {
    let Some(adt) = ty.ty_adt_def() else {
        return false;
    };
    let path = cx.tcx.def_path_str(adt.did());
    path == "alloc::string::String" || path == "std::string::String"
}

/// Returns `true` if `expr` matches one of:
///
/// * `String::from(<string literal>)`
/// * `<string literal>.to_string()`
/// * `<string literal>.to_owned()`
fn is_owned_string_built_from_literal<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
) -> bool {
    match &expr.kind {
        ExprKind::Call(func, [arg]) => {
            let ExprKind::Path(qpath) = &func.kind else {
                return false;
            };
            let Res::Def(DefKind::Fn | DefKind::AssocFn, def_id) = cx.qpath_res(qpath, func.hir_id)
            else {
                return false;
            };
            if cx.tcx.item_name(def_id).as_str() != "from" {
                return false;
            }
            is_string_literal(arg)
        }
        ExprKind::MethodCall(segment, receiver, [], _) => {
            let name = segment.ident.name.as_str();
            if name != "to_string" && name != "to_owned" {
                return false;
            }
            is_string_literal(receiver)
        }
        _ => false,
    }
}

/// Returns `true` if `expr` is a string-literal expression, optionally wrapped
/// in a single layer of reference (`&"lit"`).
fn is_string_literal<'tcx>(expr: &'tcx Expr<'tcx>) -> bool {
    let inner = match &expr.kind {
        ExprKind::AddrOf(_, _, inner) => *inner,
        _ => expr,
    };
    matches!(
        &inner.kind,
        ExprKind::Lit(lit) if matches!(lit.node, LitKind::Str(..))
    )
}
