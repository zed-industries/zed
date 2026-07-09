#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use clippy_utils::diagnostics::{span_lint_and_help, span_lint_and_then};
use clippy_utils::is_def_id_trait_method;
use clippy_utils::source::snippet_opt;
use rustc_ast::ast::LitKind;
use rustc_errors::Applicability;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::DefId;
use rustc_hir::intravisit::{Visitor, walk_expr};
use rustc_hir::{
    Closure, ClosureKind, CoroutineDesugaring, CoroutineKind, CoroutineSource, Expr, ExprKind,
    YieldSource,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::hir::nested_filter;
use rustc_middle::ty::Ty;
use rustc_span::Span;

mod blocking_io_on_foreground;
mod entity_update_in_render;
mod notify_in_render;
mod owned_string_into_shared;
mod render_helpers;

use blocking_io_on_foreground::BLOCKING_IO_ON_FOREGROUND;
use entity_update_in_render::ENTITY_UPDATE_IN_RENDER;
use notify_in_render::NOTIFY_IN_RENDER;
use owned_string_into_shared::OWNED_STRING_INTO_SHARED;

// ---------------------------------------------------------------------------
// Boilerplate: export the dylint ABI version symbol.
// ---------------------------------------------------------------------------
dylint_linting::dylint_library!();

// ---------------------------------------------------------------------------
// Registration: a single entry point that hands both lints to the compiler.
// ---------------------------------------------------------------------------
#[allow(clippy::no_mangle_with_rust_abi)]
#[unsafe(no_mangle)]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    dylint_linting::init_config(sess);
    lint_store.register_lints(&[
        SHARED_STRING_FROM_STR_LITERAL,
        ASYNC_BLOCK_WITHOUT_AWAIT,
        BLOCKING_IO_ON_FOREGROUND,
        ENTITY_UPDATE_IN_RENDER,
        NOTIFY_IN_RENDER,
        OWNED_STRING_INTO_SHARED,
    ]);
    lint_store.register_late_pass(|_| Box::new(SharedStringFromStrLiteral));
    lint_store.register_late_pass(|_| Box::new(AsyncBlockWithoutAwait));
    lint_store.register_late_pass(|_| Box::new(blocking_io_on_foreground::BlockingIoOnForeground));
    lint_store.register_late_pass(|_| Box::new(entity_update_in_render::EntityUpdateInRender));
    lint_store.register_late_pass(|_| Box::new(notify_in_render::NotifyInRender));
    lint_store.register_late_pass(|_| Box::new(owned_string_into_shared::OwnedStringIntoShared));
}

// ===========================================================================
// Lint A — SHARED_STRING_FROM_STR_LITERAL
// ===========================================================================

rustc_session::declare_lint! {
    /// ### What it does
    ///
    /// Flags `gpui::SharedString` values constructed from a string literal by
    /// any path other than `SharedString::new_static`.
    ///
    /// ### Why is this bad?
    ///
    /// `SharedString` wraps a `SmolStr`. `SmolStr::from` either copies the
    /// bytes into inline storage (literals ≤ 23 bytes) or allocates a fresh
    /// `Arc<str>` on the heap (literals > 23 bytes). `SharedString::new_static`
    /// does neither: it stores the `'static` pointer directly. For a string
    /// literal the constant-pointer path is always available and strictly
    /// cheaper.
    ///
    /// This lint fires on `SharedString::from("…")`, `SharedString::new("…")`,
    /// `<SharedString as From<_>>::from("…")`, and `"…".into()` whose inferred
    /// target type is `SharedString`. It does not fire on
    /// `SharedString::new_static(…)`.
    ///
    /// The lint distinguishes two tiers of wastefulness:
    /// * Literals > 23 bytes trigger a heap allocation per call site, flagged
    ///   at full severity.
    /// * Literals ≤ 23 bytes "only" pay a memcpy; still strictly worse than
    ///   `new_static`, but cheaper to leave alone.
    ///
    /// ### Example
    ///
    /// ```ignore
    /// let s: SharedString = SharedString::from("Right-click for more options");
    /// let t: SharedString = "hello".into();
    /// ```
    ///
    /// Use instead:
    ///
    /// ```ignore
    /// let s = SharedString::new_static("Right-click for more options");
    /// let t = SharedString::new_static("hello");
    /// ```
    pub SHARED_STRING_FROM_STR_LITERAL,
    Warn,
    "constructing a `SharedString` from a string literal via a copying/allocating path"
}

rustc_session::declare_lint_pass!(SharedStringFromStrLiteral => [SHARED_STRING_FROM_STR_LITERAL]);

/// Maximum number of bytes that `SmolStr` (and therefore `SharedString`) can
/// store inline on 64-bit targets. Literals larger than this trigger an
/// `Arc<str>` allocation on every conversion.
///
/// Source: `smol_str` v0.3's `INLINE_CAP`. See
/// <https://docs.rs/smol_str/0.3.6/smol_str/>.
const SMOL_STR_INLINE_CAP: usize = 23;

impl<'tcx> LateLintPass<'tcx> for SharedStringFromStrLiteral {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        // Do not descend into macro-expanded code: we'd be suggesting edits to
        // spans the user cannot actually touch.
        if expr.span.from_expansion() {
            return;
        }

        let ty = cx.typeck_results().expr_ty(expr);
        if !is_shared_string(cx, ty) {
            return;
        }

        let Some(literal) = extract_literal_source(cx, expr) else {
            return;
        };

        emit_shared_string(cx, expr.span, literal);
    }
}

/// A string literal the user wrote and that we are confident we can replace.
struct LiteralSource {
    /// The literal's decoded contents.
    contents: String,
    /// The span of the full expression that should be replaced (e.g. the whole
    /// `SharedString::from("x")` call), not just the literal token.
    replace_span: Span,
}

fn extract_literal_source<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
) -> Option<LiteralSource> {
    match &expr.kind {
        // `SharedString::from(lit)`, `SharedString::new(lit)`, or any other
        // associated/trait function resolving onto `SharedString` with a
        // single string-literal argument.
        ExprKind::Call(func, [arg]) => {
            let def_id = call_def_id(cx, func)?;
            if !is_interesting_shared_string_constructor(cx, def_id) {
                return None;
            }
            let contents = str_literal_contents(arg)?;
            Some(LiteralSource {
                contents,
                replace_span: expr.span,
            })
        }

        // `lit.into()` where the target type is `SharedString`.
        ExprKind::MethodCall(path_seg, receiver, [], _)
            if path_seg.ident.name.as_str() == "into" =>
        {
            let contents = str_literal_contents(receiver)?;
            Some(LiteralSource {
                contents,
                replace_span: expr.span,
            })
        }

        _ => None,
    }
}

/// Extract the contents of a string literal expression, peeling through a
/// single layer of reference if the user wrote `&"lit"`.
fn str_literal_contents<'tcx>(expr: &'tcx Expr<'tcx>) -> Option<String> {
    let inner = match &expr.kind {
        ExprKind::AddrOf(_, _, inner) => *inner,
        _ => expr,
    };
    if let ExprKind::Lit(lit) = &inner.kind
        && let LitKind::Str(sym, _) = lit.node
    {
        Some(sym.as_str().to_owned())
    } else {
        None
    }
}

/// Returns the `DefId` of the function being called, if `func` is a direct
/// path to a function or associated function. This handles both
/// `Type::method(...)` syntax (including type-relative paths that resolve
/// through `typeck_results`) and free-function paths.
fn call_def_id<'tcx>(cx: &LateContext<'tcx>, func: &'tcx Expr<'tcx>) -> Option<DefId> {
    let ExprKind::Path(qpath) = &func.kind else {
        return None;
    };
    match cx.qpath_res(qpath, func.hir_id) {
        Res::Def(DefKind::Fn | DefKind::AssocFn, def_id) => Some(def_id),
        _ => None,
    }
}

/// True if `def_id` names a `SharedString` constructor that we treat as a
/// wasteful alternative to `SharedString::new_static` when passed a string
/// literal.
///
/// This covers two distinct resolutions rustc produces for these call sites:
///
/// * `SharedString::new("x")` resolves to the inherent associated function
///   on `impl SharedString`. The impl's `Self` type is `SharedString`.
/// * `SharedString::from("x")` resolves to the trait method
///   `core::convert::From::from`. The impl is not recorded on the `def_id`
///   itself; we instead verify the enclosing trait is `From` and rely on the
///   caller having already checked that the call's result type is
///   `SharedString`.
///
/// `SharedString::new_static` is explicitly exempted because it is the
/// preferred alternative.
fn is_interesting_shared_string_constructor(cx: &LateContext<'_>, def_id: DefId) -> bool {
    let tcx = cx.tcx;
    let name = tcx.item_name(def_id);
    if name.as_str() == "new_static" {
        return false;
    }
    if !matches!(name.as_str(), "from" | "new") {
        return false;
    }
    if let Some(impl_id) = tcx.impl_of_assoc(def_id) {
        let self_ty = tcx.type_of(impl_id).skip_binder();
        return is_shared_string(cx, self_ty);
    }
    if let Some(trait_id) = tcx.trait_of_assoc(def_id) {
        // The caller has already asserted that the call's result type is
        // `SharedString`, so a `From::from` call resolving here is
        // equivalent to `<SharedString as From<_>>::from`.
        let path = tcx.def_path_str(trait_id);
        return path == "core::convert::From" || path == "std::convert::From";
    }
    false
}

/// Match the canonical definition path of `gpui_shared_string::SharedString`.
/// Re-exports through `gpui` resolve back to the same `DefId`.
fn is_shared_string(cx: &LateContext<'_>, ty: Ty<'_>) -> bool {
    let Some(adt) = ty.ty_adt_def() else {
        return false;
    };
    let did = adt.did();
    let krate = cx.tcx.crate_name(did.krate);
    if krate.as_str() != "gpui_shared_string" {
        return false;
    }
    cx.tcx.item_name(did).as_str() == "SharedString"
}

fn emit_shared_string(cx: &LateContext<'_>, call_span: Span, literal: LiteralSource) {
    let LiteralSource {
        contents,
        replace_span,
    } = literal;
    let byte_len = contents.len();
    let over_inline = byte_len > SMOL_STR_INLINE_CAP;

    // Use the original source text for the replacement where possible so we
    // preserve raw-string syntax, escapes, etc. Fall back to debug-formatting
    // the decoded contents if the source is unavailable (e.g. macro-generated).
    let replacement_lit = snippet_opt(cx, replace_span)
        .and_then(extract_embedded_string_literal)
        .unwrap_or_else(|| format!("{contents:?}"));

    let suggestion = format!("SharedString::new_static({replacement_lit})");

    let primary_msg = if over_inline {
        "this `SharedString` construction heap-allocates on every call"
    } else {
        "this `SharedString` construction copies the literal on every call"
    };

    span_lint_and_then(
        cx,
        SHARED_STRING_FROM_STR_LITERAL,
        call_span,
        primary_msg,
        |diag| {
            if over_inline {
                diag.note(format!(
                "the literal is {byte_len} bytes, which exceeds `SmolStr`'s {SMOL_STR_INLINE_CAP}-byte inline capacity, so `SmolStr::from` allocates an `Arc<str>` here",
            ));
            } else {
                diag.note(format!(
                "the literal is {byte_len} bytes (≤ {SMOL_STR_INLINE_CAP}) so it stays inline, but the copy is still avoidable",
            ));
            }
            diag.note("`SharedString::new_static` stores the `'static` pointer directly and performs no allocation or copy");
            diag.span_suggestion(
                replace_span,
                "use the zero-cost static constructor",
                suggestion,
                Applicability::MachineApplicable,
            );
        },
    );
}

/// Given a snippet like `SharedString::from("hi")` or `"hi".into()`, extract
/// the first embedded string literal token (including any `r#"..."#` prefix)
/// so we can paste it back unchanged. This is a best-effort scanner and
/// returns `None` when the snippet has no literal or an unterminated one.
fn extract_embedded_string_literal(snippet: String) -> Option<String> {
    let bytes = snippet.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        // Raw string: optional `b`, then `r`, then `#`*, then `"`.
        let raw_start = i;
        let mut j = i;
        if j < bytes.len() && bytes[j] == b'b' {
            j += 1;
        }
        if j < bytes.len() && bytes[j] == b'r' {
            let mut hashes = 0;
            let mut k = j + 1;
            while k < bytes.len() && bytes[k] == b'#' {
                hashes += 1;
                k += 1;
            }
            if k < bytes.len() && bytes[k] == b'"' {
                // Scan for closing `"` followed by the same number of `#`.
                let mut m = k + 1;
                while m < bytes.len() {
                    if bytes[m] == b'"' {
                        let mut close_hashes = 0;
                        let mut n = m + 1;
                        while close_hashes < hashes && n < bytes.len() && bytes[n] == b'#' {
                            close_hashes += 1;
                            n += 1;
                        }
                        if close_hashes == hashes {
                            return Some(snippet[raw_start..n].to_owned());
                        }
                    }
                    m += 1;
                }
                return None;
            }
        }
        // Regular string: optional `b`, then `"`, until matching unescaped `"`.
        let mut j = i;
        if j < bytes.len() && bytes[j] == b'b' {
            j += 1;
        }
        if j < bytes.len() && bytes[j] == b'"' {
            let mut m = j + 1;
            while m < bytes.len() {
                match bytes[m] {
                    b'\\' => {
                        m += 2;
                        continue;
                    }
                    b'"' => return Some(snippet[raw_start..=m].to_owned()),
                    _ => m += 1,
                }
            }
            return None;
        }
        i += 1;
    }
    None
}

// ===========================================================================
// Lint B — ASYNC_BLOCK_WITHOUT_AWAIT
// ===========================================================================

rustc_session::declare_lint! {
    /// ### What it does
    ///
    /// Flags `async { … }` and `async move { … }` blocks whose body contains
    /// no `.await` expression at their own nesting level.
    ///
    /// ### Why is this bad?
    ///
    /// An async block without an `.await` wraps synchronous code in a `Future`
    /// state machine for no benefit. The state machine adds binary size, and
    /// the indirection may hide the fact that the code never actually yields.
    /// Either the `async` should be removed (the code is synchronous), or a
    /// missing `.await` is a bug.
    ///
    /// ### Example
    ///
    /// ```ignore
    /// let future = async { compute_something() };
    /// ```
    ///
    /// Use instead:
    ///
    /// ```ignore
    /// let value = compute_something();
    /// ```
    pub ASYNC_BLOCK_WITHOUT_AWAIT,
    Warn,
    "`async` block that contains no `.await` expression"
}

rustc_session::declare_lint_pass!(AsyncBlockWithoutAwait => [ASYNC_BLOCK_WITHOUT_AWAIT]);

impl<'tcx> LateLintPass<'tcx> for AsyncBlockWithoutAwait {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }

        // Match only async blocks — not async function bodies or async closures.
        let ExprKind::Closure(Closure {
            kind:
                ClosureKind::Coroutine(CoroutineKind::Desugared(
                    CoroutineDesugaring::Async,
                    CoroutineSource::Block,
                )),
            body,
            ..
        }) = &expr.kind
        else {
            return;
        };

        // Trait impls are constrained by the trait's signature. If the trait
        // requires a method that returns a future, the implementor must produce
        // an async block even when their implementation has nothing to await.
        let enclosing_body_owner = cx.tcx.hir_enclosing_body_owner(expr.hir_id);
        if is_def_id_trait_method(cx, enclosing_body_owner) {
            return;
        }

        let body = cx.tcx.hir_body(*body);
        let mut visitor = AwaitVisitor {
            cx,
            found_await: false,
            async_depth: 0,
        };
        walk_expr(&mut visitor, body.value);

        if !visitor.found_await {
            span_lint_and_help(
                cx,
                ASYNC_BLOCK_WITHOUT_AWAIT,
                expr.span,
                "this `async` block contains no `.await`",
                None,
                "consider removing the `async` block or adding the missing `.await`",
            );
        }
    }
}

/// Walks the body of an async block looking for `.await` expressions. Tracks
/// nesting depth so that an `.await` inside a *nested* async block is not
/// attributed to the *outer* block.
struct AwaitVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    found_await: bool,
    async_depth: usize,
}

impl<'tcx> Visitor<'tcx> for AwaitVisitor<'_, 'tcx> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.cx.tcx
    }

    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Yield(_, YieldSource::Await { .. }) = expr.kind {
            if self.async_depth == 0 {
                self.found_await = true;
                return;
            }
        }

        let is_nested_async_block = matches!(
            expr.kind,
            ExprKind::Closure(Closure {
                kind: ClosureKind::Coroutine(CoroutineKind::Desugared(
                    CoroutineDesugaring::Async,
                    _
                )),
                ..
            })
        );

        if is_nested_async_block {
            self.async_depth += 1;
        }

        walk_expr(self, expr);

        if is_nested_async_block {
            self.async_depth -= 1;
        }
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::process::Command;

    /// Build the test-fixture `gpui` crate and return rustc flags that make
    /// it available to standalone UI test files via `extern crate gpui`.
    fn gpui_fixture_rustc_flags() -> Vec<String> {
        let fixture_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test_fixture"]
            .iter()
            .collect();

        let status = Command::new("cargo")
            .args(["build", "--package", "gpui"])
            .current_dir(&fixture_dir)
            .status()
            .expect("failed to run cargo build for gpui fixture");
        assert!(status.success(), "gpui fixture build failed");

        let rlib: PathBuf = fixture_dir.join("target/debug/libgpui.rlib");
        let deps: PathBuf = fixture_dir.join("target/debug/deps");

        vec![
            "--edition=2021".to_string(),
            format!("--extern=gpui={}", rlib.display()),
            format!("-Ldependency={}", deps.display()),
        ]
    }

    #[test]
    fn ui() {
        let flags = gpui_fixture_rustc_flags();
        dylint_testing::ui::Test::src_base(env!("CARGO_PKG_NAME"), "ui")
            .rustc_flags(flags)
            .run();
    }

    #[test]
    fn ui_shared_string() {
        dylint_testing::ui_test_examples(env!("CARGO_PKG_NAME"));
    }
}
