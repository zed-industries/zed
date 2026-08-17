use proc_macro2::{Span, TokenStream};

use crate::SpanDiagnosticExt;
use crate::line::Line;

/// Trait implemented by types that can be converted into a set of `Span`s.
pub trait MultiSpan {
    /// Converts `self` into a `Vec<Span>`.
    fn into_spans(self) -> Vec<Span>;
}

impl MultiSpan for Span {
    fn into_spans(self) -> Vec<Span> { vec![self] }
}

impl MultiSpan for Vec<Span> {
    fn into_spans(self) -> Vec<Span> { self }
}

impl<'a> MultiSpan for &'a [Span] {
    fn into_spans(self) -> Vec<Span> {
        self.to_vec()
    }
}

/// An enum representing a diagnostic level.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Level {
    /// An error.
    Error,
    /// A warning.
    Warning,
    /// A note.
    Note,
    /// A help message.
    Help,
}

impl std::str::FromStr for Level {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(Level::Error.as_str()) {
            Ok(Level::Error)
        } else if s.contains(Level::Warning.as_str()) {
            Ok(Level::Warning)
        } else if s.contains(Level::Note.as_str()) {
            Ok(Level::Note)
        } else if s.contains(Level::Help.as_str()) {
            Ok(Level::Help)
        } else {
            Err(())
        }
    }
}

impl Level {
    fn as_str(self) -> &'static str {
        match self {
            Level::Error => "error",
            Level::Warning => "warning",
            Level::Note => "note",
            Level::Help => "help",
        }
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// A structure representing a diagnostic message and associated children
/// messages.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    level: Level,
    message: String,
    spans: Vec<Span>,
    children: Vec<Diagnostic>
}

macro_rules! diagnostic_child_methods {
    ($spanned:ident, $regular:ident, $level:expr) => (
        /// Adds a new child diagnostic message to `self` with the level
        /// identified by this method's name with the given `spans` and
        /// `message`.
        pub fn $spanned<S, T>(self, spans: S, message: T) -> Diagnostic
            where S: MultiSpan, T: Into<String>
        {
            self.spanned_child(spans, $level, message)
        }

        /// Adds a new child diagnostic message to `self` with the level
        /// identified by this method's name with the given `message`.
        pub fn $regular<T: Into<String>>(self, message: T) -> Diagnostic {
            self.child($level, message)
        }
    )
}

impl Diagnostic {
    /// Creates a new diagnostic with the given `level` and `message`.
    pub fn new<T: Into<String>>(level: Level, message: T) -> Diagnostic {
        Diagnostic {
            level,
            message: message.into(),
            spans: vec![],
            children: vec![]
        }
    }

    /// Creates a new diagnostic with the given `level` and `message` pointing
    /// to the given set of `spans`.
    pub fn spanned<S, T>(spans: S, level: Level, message: T) -> Diagnostic
        where S: MultiSpan, T: Into<String>
    {
        Diagnostic {
            level,
            message: message.into(),
            spans: spans.into_spans(),
            children: vec![]
        }
    }

    /// Adds a new child diagnostic message to `self` with the `level` and the
    /// given `spans` and `message`.
    pub fn spanned_child<S, T>(mut self, spans: S, level: Level, message: T) -> Diagnostic
        where S: MultiSpan, T: Into<String>
    {
        self.children.push(Diagnostic::spanned(spans, level, message));
        self
    }

    /// Adds a new child diagnostic message to `self` with `level` and the given
    /// `message`.
    pub fn child<T: Into<String>>(mut self, level: Level, message: T) -> Diagnostic {
        self.children.push(Diagnostic::new(level, message));
        self
    }

    diagnostic_child_methods!(span_error, error, Level::Error);
    diagnostic_child_methods!(span_warning, warning, Level::Warning);
    diagnostic_child_methods!(span_note, note, Level::Note);
    diagnostic_child_methods!(span_help, help, Level::Help);

    /// Return the children diagnostics of `self`.
    pub fn children(&self) -> impl Iterator<Item=&Diagnostic> {
        self.children.iter()
    }

    /// Return the `level` of `self`.
    pub fn level(&self) -> Level {
        self.level
    }

    fn stable_emit_as_tokens(self, item: bool) -> TokenStream {
        let error: syn::parse::Error = self.into();
        if item {
            error.to_compile_error()
        } else {
            let compile_error_calls = error.into_iter().map(|e| {
                let compile_error = e.to_compile_error();
                quote::quote_spanned!(e.span() => #compile_error;)
            });

            quote::quote!({ #(#compile_error_calls)* })
        }
    }

    /// Emit the diagnostic as tokens.
    #[cfg(not(nightly_diagnostics))]
    fn emit_as_tokens(self, item: bool, _: TokenStream) -> TokenStream {
        self.stable_emit_as_tokens(item)
    }

    /// Emit the diagnostic as tokens.
    #[cfg(nightly_diagnostics)]
    fn emit_as_tokens(self, item: bool, default: TokenStream) -> TokenStream {
        if !crate::nightly_works() {
            return self.stable_emit_as_tokens(item);
        }

        proc_macro::Diagnostic::from(self).emit();
        default
    }

    /// Emit tokens, suitable for item contexts, to generate a comple-time
    /// diagnostic corresponding to `self`. On nightly, this directly emits the
    /// error and returns an empty token stream.
    pub fn emit_as_item_tokens(self) -> TokenStream {
        self.emit_as_tokens(true, TokenStream::new())
    }

    /// Emit tokens, suitable for item contexts, to generate a comple-time
    /// diagnostic corresponding to `self`. On nightly, this directly emits the
    /// error and returns `default`.
    pub fn emit_as_item_tokens_or(self, default: TokenStream) -> TokenStream {
        self.emit_as_tokens(true, default)
    }

    /// Emit tokens, suitable for expression contexts, to generate a comple-time
    /// diagnostic corresponding to `self`. On nightly, this directly emits the
    /// error and returns a `()` token stream.
    pub fn emit_as_expr_tokens(self) -> TokenStream {
        self.emit_as_tokens(false, quote::quote!({}))
    }

    /// Emit tokens, suitable for expressioon contexts, to generate a
    /// comple-time diagnostic corresponding to `self`. On nightly, this
    /// directly emits the error and returns `default`.
    pub fn emit_as_expr_tokens_or(self, default: TokenStream) -> TokenStream {
        self.emit_as_tokens(false, default)
    }
}

impl From<Diagnostic> for syn::parse::Error {
    fn from(diag: Diagnostic) -> syn::parse::Error {
        fn diag_to_msg(diag: &Diagnostic) -> String {
            let (spans, level, msg) = (&diag.spans, diag.level, &diag.message);
            if spans.is_empty() {
                Line::joined(level, msg).to_string()
            } else {
                if level == Level::Error {
                    return msg.into();
                }

                Line::new(level, msg).to_string()
            }
        }

        fn diag_to_span(diag: &Diagnostic) -> Span {
            diag.spans.get(0).cloned().unwrap_or_else(|| Span::call_site())
        }

        let mut msg = diag_to_msg(&diag);
        let mut span = diag_to_span(&diag);
        let mut error: Option<syn::Error> = None;
        for child in diag.children {
            if child.spans.is_empty() {
                // Join to the current error we're building up.
                msg.push_str(&format!("\n{}", diag_to_msg(&child)));
            } else {
                // This creates a new error with all of the diagnostic messages
                // that have been joined thus far in `msg`.
                let new_error = syn::parse::Error::new(span, &msg);
                if let Some(ref mut error) = error {
                    error.combine(new_error);
                } else {
                    error = Some(new_error);
                }

                // Start a new error to be built from `child`.
                span = diag_to_span(&child);
                msg = diag_to_msg(&child);
            }
        }

        if let Some(mut error) = error {
            error.combine(syn::parse::Error::new(span, &msg));
            error
        } else {
            syn::parse::Error::new(span, &msg)
        }
    }
}

impl From<syn::parse::Error> for Diagnostic {
    fn from(error: syn::parse::Error) -> Diagnostic {
        let mut diag: Option<Diagnostic> = None;
        for e in &error {
            for line in e.to_string().lines() {
                if let Some(line) = Line::parse(line) {
                    if line.is_new() {
                        diag = diag.map(|d| d.spanned_child(e.span(), line.level, line.msg))
                            .or_else(|| Some(Diagnostic::spanned(e.span(), line.level, line.msg)));
                    } else {
                        diag = diag.map(|d| d.child(line.level, line.msg));
                    }
                } else {
                    diag = diag.map(|d| d.span_error(e.span(), line))
                        .or_else(|| Some(e.span().error(line)));
                }
            }
        }

        diag.unwrap_or_else(|| error.span().error(error.to_string()))
    }
}

#[cfg(nightly_diagnostics)]
impl From<Diagnostic> for proc_macro::Diagnostic {
    fn from(diag: Diagnostic) -> proc_macro::Diagnostic {
        fn spans_to_proc_macro_spans(spans: Vec<Span>) -> Vec<proc_macro::Span> {
            spans.into_iter()
                .map(|s| s.unstable())
                .collect::<Vec<proc_macro::Span>>()
        }

        let spans = spans_to_proc_macro_spans(diag.spans);

        let level = match diag.level {
            Level::Error => proc_macro::Level::Error,
            Level::Warning => proc_macro::Level::Warning,
            Level::Note => proc_macro::Level::Note,
            Level::Help => proc_macro::Level::Help,
        };

        let mut proc_diag = proc_macro::Diagnostic::spanned(spans, level, diag.message);
        for child in diag.children {
            // FIXME: proc_macro::Diagnostic needs a `push` method.
            let spans = spans_to_proc_macro_spans(child.spans);
            proc_diag = match child.level {
                Level::Error => proc_diag.span_error(spans, child.message),
                Level::Warning => proc_diag.span_warning(spans, child.message),
                Level::Note => proc_diag.span_note(spans, child.message),
                Level::Help => proc_diag.span_help(spans, child.message),
            };
        }

        proc_diag
    }
}
