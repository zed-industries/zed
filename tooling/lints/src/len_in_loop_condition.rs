use clippy_utils::diagnostics::span_lint;
use rustc_hir::{BinOpKind, Expr, ExprKind, LoopSource, MatchSource, Node};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;

rustc_session::declare_lint! {
    /// ### What it does
    ///
    /// Flags loops whose bound calls `.len()` on a collection each iteration.
    ///
    /// The flagged shapes are:
    ///
    /// ```ignore
    /// for i in 0..v.len() { /* … */ }
    /// while i < v.len()   { /* … */ }
    /// ```
    ///
    /// ### Why is this bad?
    ///
    /// In a `while` loop the `.len()` call is re-evaluated every iteration.
    /// On types whose `len` is not O(1) this is wasteful. Even when `len` is
    /// cheap, hoisting it into a local communicates that the bound is fixed
    /// and makes the code easier to reason about.
    pub LEN_IN_LOOP_CONDITION,
    Warn,
    "`.len()` called in a loop bound instead of being hoisted into a local"
}

pub(crate) struct LenInLoopCondition;

rustc_session::impl_lint_pass!(LenInLoopCondition => [LEN_IN_LOOP_CONDITION]);

impl<'tcx> LateLintPass<'tcx> for LenInLoopCondition {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }

        if !is_len_method_call(expr) {
            return;
        }

        // Case 1: `.len()` appears as the upper bound of a range that feeds
        // a for loop.  Walk up the parent chain from the `.len()` call.
        if is_range_bound_in_for_loop(cx, expr) {
            emit(cx, expr.span);
            return;
        }

        // Case 2: `.len()` appears in a comparison inside a while-loop
        // condition.
        if is_comparison_operand_in_while_condition(cx, expr) {
            emit(cx, expr.span);
        }
    }
}

fn is_len_method_call(expr: &Expr<'_>) -> bool {
    if let ExprKind::MethodCall(segment, _receiver, args, _span) = &expr.kind {
        return segment.ident.name.as_str() == "len" && args.is_empty();
    }
    false
}

/// Returns `true` when `len_expr` is the upper bound of a range struct that is
/// the iterator argument of a desugared for loop.
///
/// After desugaring `for _ in 0..v.len() { .. }`, the parent chain from the
/// `.len()` call is:
///
///   ExprField ("end")  →  Struct (Range)  →  Call (into_iter)
///     →  Match (ForLoopDesugar)  →  …  →  Loop (ForLoop)
fn is_range_bound_in_for_loop(cx: &LateContext<'_>, len_expr: &Expr<'_>) -> bool {
    let mut saw_range = false;
    let mut saw_into_iter = false;
    for (_id, node) in cx.tcx.hir_parent_iter(len_expr.hir_id) {
        match node {
            // The `.len()` value is used as a struct field (`end:`) in the
            // half-open range literal (`0..v.len()`).
            Node::ExprField(_) => continue,
            // The Range struct literal itself (half-open ranges).
            Node::Expr(expr) if !saw_range && matches!(expr.kind, ExprKind::Struct(..)) => {
                saw_range = true;
                continue;
            }
            // `RangeInclusive::new(start, end)` for inclusive ranges
            // (`0..=v.len()`).  The `.len()` is a direct argument to this
            // Call, which also serves as the range constructor.
            Node::Expr(expr) if !saw_range && matches!(expr.kind, ExprKind::Call(..)) => {
                saw_range = true;
                continue;
            }
            // The `IntoIterator::into_iter(range)` call.
            Node::Expr(expr)
                if saw_range && !saw_into_iter && matches!(expr.kind, ExprKind::Call(..)) =>
            {
                saw_into_iter = true;
                continue;
            }
            // Match with ForLoopDesugar source — this confirms the range
            // feeds a for loop.  The `loop` node is inside the match arm,
            // not above it, so finding this match is sufficient.
            Node::Expr(expr)
                if matches!(
                    expr.kind,
                    ExprKind::Match(_, _, MatchSource::ForLoopDesugar)
                ) =>
            {
                return true;
            }
            Node::Block(_) | Node::Arm(_) | Node::Stmt(_) => continue,
            _ => return false,
        }
    }
    false
}

/// Returns `true` when `len_expr` is an operand of a comparison expression
/// that serves as the condition of a `while` loop.
fn is_comparison_operand_in_while_condition(cx: &LateContext<'_>, len_expr: &Expr<'_>) -> bool {
    for (_id, node) in cx.tcx.hir_parent_iter(len_expr.hir_id) {
        match node {
            // Found a comparison operator — the `.len()` is an operand.
            Node::Expr(expr)
                if matches!(
                    &expr.kind,
                    ExprKind::Binary(op, _, _) if matches!(
                        op.node,
                        BinOpKind::Lt | BinOpKind::Le | BinOpKind::Gt | BinOpKind::Ge | BinOpKind::Ne
                    )
                ) =>
            {
                continue;
            }
            // The comparison may be the condition of an `if` inside a
            // `while` desugar.
            Node::Expr(expr) if matches!(expr.kind, ExprKind::If(..)) => continue,
            // The `while` desugar wraps the condition in a Block.
            Node::Block(_) | Node::Stmt(_) => continue,
            // Found the while loop.
            Node::Expr(expr) if matches!(expr.kind, ExprKind::Loop(_, _, LoopSource::While, _)) => {
                return true;
            }
            _ => return false,
        }
    }
    false
}

fn emit(cx: &LateContext<'_>, span: Span) {
    span_lint(
        cx,
        LEN_IN_LOOP_CONDITION,
        span,
        "`.len()` called in a loop bound; consider hoisting into a local variable",
    );
}
