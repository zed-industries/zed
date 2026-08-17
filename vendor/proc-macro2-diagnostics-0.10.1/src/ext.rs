use proc_macro2::Span;

use crate::diagnostic::{Level, Diagnostic};

macro_rules! diagnostic_def {
    ($name:ident) => (
        /// Create a new `Diagnostic` of the kind of this method's name with the
        /// span `self`.
        fn $name<T: Into<String>>(self, message: T) -> Diagnostic;
    )
}

/// Extension trait for `proc_macro2::Span` emulating the proc-macro diagnostic
/// API on stable and nightly.
///
/// # Example
///
/// ```rust
/// use proc_macro2::Span;
/// use proc_macro2_diagnostics::SpanDiagnosticExt;
///
/// let span = Span::call_site();
/// let diag = span.error("there's a problem here...");
///
/// // emit into an expression context.
/// # let diag = span.error("there's a problem here...");
/// let tokens = diag.emit_as_expr_tokens();
///
/// // or emit into an item context.
/// # let diag = span.error("there's a problem here...");
/// let tokens = diag.emit_as_item_tokens();
/// ```
pub trait SpanDiagnosticExt {
    diagnostic_def!(error);
    diagnostic_def!(warning);
    diagnostic_def!(note);
    diagnostic_def!(help);
}

macro_rules! diagnostic_method {
    ($name:ident, $level:expr) => (
        fn $name<T: Into<String>>(self, message: T) -> Diagnostic {
            Diagnostic::spanned(self, $level, message)
        }
    )
}

impl SpanDiagnosticExt for Span {
    diagnostic_method!(error, Level::Error);
    diagnostic_method!(warning, Level::Warning);
    diagnostic_method!(note, Level::Note);
    diagnostic_method!(help, Level::Help);
}
