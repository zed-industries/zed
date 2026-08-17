use crate::{abort_now, check_correctness, sealed::Sealed, SpanRange};
use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::{quote_spanned, ToTokens};

/// Represents a diagnostic level
///
/// # Warnings
///
/// Warnings are ignored on stable/beta
#[derive(Debug, PartialEq)]
pub enum Level {
    Error,
    Warning,
    #[doc(hidden)]
    NonExhaustive,
}

/// Represents a single diagnostic message
#[derive(Debug)]
pub struct Diagnostic {
    pub(crate) level: Level,
    pub(crate) span_range: SpanRange,
    pub(crate) msg: String,
    pub(crate) suggestions: Vec<(SuggestionKind, String, Option<SpanRange>)>,
    pub(crate) children: Vec<(SpanRange, String)>,
}

/// A collection of methods that do not exist in `proc_macro::Diagnostic`
/// but still useful to have around.
///
/// This trait is sealed and cannot be implemented outside of `proc_macro_error`.
pub trait DiagnosticExt: Sealed {
    /// Create a new diagnostic message that points to the `span_range`.
    ///
    /// This function is the same as `Diagnostic::spanned` but produces considerably
    /// better error messages for multi-token spans on stable.
    fn spanned_range(span_range: SpanRange, level: Level, message: String) -> Self;

    /// Add another error message to self such that it will be emitted right after
    /// the main message.
    ///
    /// This function is the same as `Diagnostic::span_error` but produces considerably
    /// better error messages for multi-token spans on stable.
    fn span_range_error(self, span_range: SpanRange, msg: String) -> Self;

    /// Attach a "help" note to your main message, the note will have it's own span on nightly.
    ///
    /// This function is the same as `Diagnostic::span_help` but produces considerably
    /// better error messages for multi-token spans on stable.
    ///
    /// # Span
    ///
    /// The span is ignored on stable, the note effectively inherits its parent's (main message) span
    fn span_range_help(self, span_range: SpanRange, msg: String) -> Self;

    /// Attach a note to your main message, the note will have it's own span on nightly.
    ///
    /// This function is the same as `Diagnostic::span_note` but produces considerably
    /// better error messages for multi-token spans on stable.
    ///
    /// # Span
    ///
    /// The span is ignored on stable, the note effectively inherits its parent's (main message) span
    fn span_range_note(self, span_range: SpanRange, msg: String) -> Self;
}

impl DiagnosticExt for Diagnostic {
    fn spanned_range(span_range: SpanRange, level: Level, message: String) -> Self {
        Diagnostic {
            level,
            span_range,
            msg: message,
            suggestions: vec![],
            children: vec![],
        }
    }

    fn span_range_error(mut self, span_range: SpanRange, msg: String) -> Self {
        self.children.push((span_range, msg));
        self
    }

    fn span_range_help(mut self, span_range: SpanRange, msg: String) -> Self {
        self.suggestions
            .push((SuggestionKind::Help, msg, Some(span_range)));
        self
    }

    fn span_range_note(mut self, span_range: SpanRange, msg: String) -> Self {
        self.suggestions
            .push((SuggestionKind::Note, msg, Some(span_range)));
        self
    }
}

impl Diagnostic {
    /// Create a new diagnostic message that points to `Span::call_site()`
    pub fn new(level: Level, message: String) -> Self {
        Diagnostic::spanned(Span::call_site(), level, message)
    }

    /// Create a new diagnostic message that points to the `span`
    pub fn spanned(span: Span, level: Level, message: String) -> Self {
        Diagnostic::spanned_range(
            SpanRange {
                first: span,
                last: span,
            },
            level,
            message,
        )
    }

    /// Add another error message to self such that it will be emitted right after
    /// the main message.
    pub fn span_error(self, span: Span, msg: String) -> Self {
        self.span_range_error(
            SpanRange {
                first: span,
                last: span,
            },
            msg,
        )
    }

    /// Attach a "help" note to your main message, the note will have it's own span on nightly.
    ///
    /// # Span
    ///
    /// The span is ignored on stable, the note effectively inherits its parent's (main message) span
    pub fn span_help(self, span: Span, msg: String) -> Self {
        self.span_range_help(
            SpanRange {
                first: span,
                last: span,
            },
            msg,
        )
    }

    /// Attach a "help" note to your main message.
    pub fn help(mut self, msg: String) -> Self {
        self.suggestions.push((SuggestionKind::Help, msg, None));
        self
    }

    /// Attach a note to your main message, the note will have it's own span on nightly.
    ///
    /// # Span
    ///
    /// The span is ignored on stable, the note effectively inherits its parent's (main message) span
    pub fn span_note(self, span: Span, msg: String) -> Self {
        self.span_range_note(
            SpanRange {
                first: span,
                last: span,
            },
            msg,
        )
    }

    /// Attach a note to your main message
    pub fn note(mut self, msg: String) -> Self {
        self.suggestions.push((SuggestionKind::Note, msg, None));
        self
    }

    /// The message of main warning/error (no notes attached)
    pub fn message(&self) -> &str {
        &self.msg
    }

    /// Abort the proc-macro's execution and display the diagnostic.
    ///
    /// # Warnings
    ///
    /// Warnings are not emitted on stable and beta, but this function will abort anyway.
    pub fn abort(self) -> ! {
        self.emit();
        abort_now()
    }

    /// Display the diagnostic while not aborting macro execution.
    ///
    /// # Warnings
    ///
    /// Warnings are ignored on stable/beta
    pub fn emit(self) {
        check_correctness();
        crate::imp::emit_diagnostic(self);
    }
}

/// **NOT PUBLIC API! NOTHING TO SEE HERE!!!**
#[doc(hidden)]
impl Diagnostic {
    pub fn span_suggestion(self, span: Span, suggestion: &str, msg: String) -> Self {
        match suggestion {
            "help" | "hint" => self.span_help(span, msg),
            _ => self.span_note(span, msg),
        }
    }

    pub fn suggestion(self, suggestion: &str, msg: String) -> Self {
        match suggestion {
            "help" | "hint" => self.help(msg),
            _ => self.note(msg),
        }
    }
}

impl ToTokens for Diagnostic {
    fn to_tokens(&self, ts: &mut TokenStream) {
        use std::borrow::Cow;

        fn ensure_lf(buf: &mut String, s: &str) {
            if s.ends_with('\n') {
                buf.push_str(s);
            } else {
                buf.push_str(s);
                buf.push('\n');
            }
        }

        fn diag_to_tokens(
            span_range: SpanRange,
            level: &Level,
            msg: &str,
            suggestions: &[(SuggestionKind, String, Option<SpanRange>)],
        ) -> TokenStream {
            if *level == Level::Warning {
                return TokenStream::new();
            }

            let message = if suggestions.is_empty() {
                Cow::Borrowed(msg)
            } else {
                let mut message = String::new();
                ensure_lf(&mut message, msg);
                message.push('\n');

                for (kind, note, _span) in suggestions {
                    message.push_str("  = ");
                    message.push_str(kind.name());
                    message.push_str(": ");
                    ensure_lf(&mut message, note);
                }
                message.push('\n');

                Cow::Owned(message)
            };

            let mut msg = proc_macro2::Literal::string(&message);
            msg.set_span(span_range.last);
            let group = quote_spanned!(span_range.last=> { #msg } );
            quote_spanned!(span_range.first=> compile_error!#group)
        }

        ts.extend(diag_to_tokens(
            self.span_range,
            &self.level,
            &self.msg,
            &self.suggestions,
        ));
        ts.extend(
            self.children
                .iter()
                .map(|(span_range, msg)| diag_to_tokens(*span_range, &Level::Error, &msg, &[])),
        );
    }
}

#[derive(Debug)]
pub(crate) enum SuggestionKind {
    Help,
    Note,
}

impl SuggestionKind {
    fn name(&self) -> &'static str {
        match self {
            SuggestionKind::Note => "note",
            SuggestionKind::Help => "help",
        }
    }
}

#[cfg(feature = "syn-error")]
impl From<syn::Error> for Diagnostic {
    fn from(err: syn::Error) -> Self {
        use proc_macro2::{Delimiter, TokenTree};

        fn gut_error(ts: &mut impl Iterator<Item = TokenTree>) -> Option<(SpanRange, String)> {
            let first = match ts.next() {
                // compile_error
                None => return None,
                Some(tt) => tt.span(),
            };
            ts.next().unwrap(); // !

            let lit = match ts.next().unwrap() {
                TokenTree::Group(group) => {
                    // Currently `syn` builds `compile_error!` invocations
                    // exclusively in `ident{"..."}` (braced) form which is not
                    // followed by `;` (semicolon).
                    //
                    // But if it changes to `ident("...");` (parenthesized)
                    // or `ident["..."];` (bracketed) form,
                    // we will need to skip the `;` as well.
                    // Highly unlikely, but better safe than sorry.

                    if group.delimiter() == Delimiter::Parenthesis
                        || group.delimiter() == Delimiter::Bracket
                    {
                        ts.next().unwrap(); // ;
                    }

                    match group.stream().into_iter().next().unwrap() {
                        TokenTree::Literal(lit) => lit,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            };

            let last = lit.span();
            let mut msg = lit.to_string();

            // "abc" => abc
            msg.pop();
            msg.remove(0);

            Some((SpanRange { first, last }, msg))
        }

        let mut ts = err.to_compile_error().into_iter();

        let (span_range, msg) = gut_error(&mut ts).unwrap();
        let mut res = Diagnostic::spanned_range(span_range, Level::Error, msg);

        while let Some((span_range, msg)) = gut_error(&mut ts) {
            res = res.span_range_error(span_range, msg);
        }

        res
    }
}
