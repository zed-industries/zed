use clippy_utils::diagnostics::span_lint;
use clippy_utils::source::snippet_opt;
use rustc_hir::{Attribute, FieldDef, ImplItem, Item, TraitItem, Variant};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;

rustc_session::declare_lint! {
    /// ### What it does
    ///
    /// Flags doc comments whose paragraphs are wrapped across multiple
    /// `///` lines.
    ///
    /// ### Why is this bad?
    ///
    /// Doc comments on agent tool input types (and the tool struct itself)
    /// are serialized into JSON tool descriptions sent verbatim to language
    /// models. Mid-paragraph line wraps add newlines that have no semantic
    /// meaning, fragment the prose for the model, and waste tokens. Each
    /// paragraph should be a single physical line; paragraphs are separated
    /// by an empty `///` line.
    ///
    /// ### Example
    ///
    /// ```ignore
    /// /// Returns a list of references including file paths,
    /// /// line numbers, and code snippets.
    /// ```
    ///
    /// Use instead:
    ///
    /// ```ignore
    /// /// Returns a list of references including file paths, line numbers, and code snippets.
    /// ```
    pub WRAPPED_DOC_COMMENT,
    Warn,
    "doc-comment paragraph is wrapped across multiple `///` lines"
}

pub(crate) struct WrappedDocComment;

rustc_session::impl_lint_pass!(WrappedDocComment => [WRAPPED_DOC_COMMENT]);

impl<'tcx> LateLintPass<'tcx> for WrappedDocComment {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        check_attrs(cx, cx.tcx.hir_attrs(item.hir_id()));
    }

    fn check_field_def(&mut self, cx: &LateContext<'tcx>, field: &'tcx FieldDef<'tcx>) {
        check_attrs(cx, cx.tcx.hir_attrs(field.hir_id));
    }

    fn check_variant(&mut self, cx: &LateContext<'tcx>, variant: &'tcx Variant<'tcx>) {
        check_attrs(cx, cx.tcx.hir_attrs(variant.hir_id));
    }

    fn check_trait_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx TraitItem<'tcx>) {
        check_attrs(cx, cx.tcx.hir_attrs(item.hir_id()));
    }

    fn check_impl_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx ImplItem<'tcx>) {
        check_attrs(cx, cx.tcx.hir_attrs(item.hir_id()));
    }
}

fn check_attrs(cx: &LateContext<'_>, attrs: &[Attribute]) {
    let mut paragraph_lines: u32 = 0;
    let mut wrap_span: Option<Span> = None;

    for attr in attrs {
        // `Attribute::span` panics for some parsed attribute variants
        // (e.g. `AutomaticallyDerived`). Filter to doc-comment attributes
        // before touching the span.
        let Some(doc_sym) = attr.doc_str() else {
            flush(cx, &mut wrap_span, &mut paragraph_lines);
            continue;
        };

        let span = attr.span();
        if span.from_expansion() {
            flush(cx, &mut wrap_span, &mut paragraph_lines);
            continue;
        }

        // Only consider line-style `///` (or `//!`) doc comments. Block
        // (`/** … */`) and explicit `#[doc = "…"]` attributes are written
        // deliberately and aren't the wrap pattern this lint targets.
        let Some(snippet) = snippet_opt(cx, span) else {
            flush(cx, &mut wrap_span, &mut paragraph_lines);
            continue;
        };
        let snippet_trimmed = snippet.trim_start();
        if !(snippet_trimmed.starts_with("///") || snippet_trimmed.starts_with("//!")) {
            flush(cx, &mut wrap_span, &mut paragraph_lines);
            continue;
        }

        let text = doc_sym.as_str().trim();
        if text.is_empty() {
            flush(cx, &mut wrap_span, &mut paragraph_lines);
            continue;
        }

        if is_structural_line(text) {
            flush(cx, &mut wrap_span, &mut paragraph_lines);
            continue;
        }

        paragraph_lines += 1;
        if paragraph_lines == 2 {
            wrap_span = Some(span);
        }
    }
    flush(cx, &mut wrap_span, &mut paragraph_lines);
}

fn flush(cx: &LateContext<'_>, wrap_span: &mut Option<Span>, paragraph_lines: &mut u32) {
    if *paragraph_lines >= 2
        && let Some(span) = *wrap_span
    {
        span_lint(
            cx,
            WRAPPED_DOC_COMMENT,
            span,
            "doc-comment paragraph is wrapped across multiple `///` lines; \
             keep each paragraph on a single line",
        );
    }
    *wrap_span = None;
    *paragraph_lines = 0;
}

/// Returns `true` for doc-comment lines that act as paragraph boundaries:
/// list items, headings, block quotes, tables, fenced code, and HTML/XML
/// tags such as `<example>`. The wrap heuristic only inspects sequences of
/// plain prose lines.
fn is_structural_line(text: &str) -> bool {
    if let Some(c) = text.chars().next()
        && matches!(c, '-' | '*' | '+' | '#' | '>' | '|' | '<')
    {
        return true;
    }
    if text.starts_with("```") {
        return true;
    }
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i > 0 && i < bytes.len() && (bytes[i] == b'.' || bytes[i] == b')') {
        return true;
    }
    false
}
