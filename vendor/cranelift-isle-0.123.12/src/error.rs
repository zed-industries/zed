//! Error types.

use std::sync::Arc;

use crate::{files::Files, lexer::Pos};

/// A collection of errors from attempting to compile some ISLE source files.
pub struct Errors {
    /// The individual errors.
    pub errors: Vec<Error>,
    pub(crate) files: Arc<Files>,
}

impl std::fmt::Debug for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.errors.is_empty() {
            return Ok(());
        }
        let diagnostics = Vec::from_iter(self.errors.iter().map(|e| {
            let message = match e {
                Error::IoError { context, .. } => context.clone(),
                Error::ParseError { msg, .. } => format!("parse error: {msg}"),
                Error::TypeError { msg, .. } => format!("type error: {msg}"),
                Error::UnreachableError { msg, .. } => format!("unreachable rule: {msg}"),
                Error::OverlapError { msg, .. } => format!("overlap error: {msg}"),
                Error::ShadowedError { .. } => {
                    "more general higher-priority rule shadows other rules".to_string()
                }
            };

            let labels = match e {
                Error::IoError { .. } => vec![],

                Error::ParseError { span, .. }
                | Error::TypeError { span, .. }
                | Error::UnreachableError { span, .. } => {
                    vec![Label::primary(span.from.file, span)]
                }

                Error::OverlapError { rules, .. } => {
                    let mut labels = vec![Label::primary(rules[0].from.file, &rules[0])];
                    labels.extend(
                        rules[1..]
                            .iter()
                            .map(|span| Label::secondary(span.from.file, span)),
                    );
                    labels
                }

                Error::ShadowedError { shadowed, mask } => {
                    let mut labels = vec![Label::primary(mask.from.file, mask)];
                    labels.extend(
                        shadowed
                            .iter()
                            .map(|span| Label::secondary(span.from.file, span)),
                    );
                    labels
                }
            };

            let mut sources = Vec::new();
            let mut source = e.source();
            while let Some(e) = source {
                sources.push(format!("{e:?}"));
                source = std::error::Error::source(e);
            }

            Diagnostic::error()
                .with_message(message)
                .with_labels(labels)
                .with_notes(sources)
        }));
        self.emit(f, diagnostics)?;
        if self.errors.len() > 1 {
            writeln!(f, "found {} errors", self.errors.len())?;
        }
        Ok(())
    }
}

/// Errors produced by ISLE.
#[derive(Debug)]
pub enum Error {
    /// An I/O error.
    IoError {
        /// The underlying I/O error.
        error: std::io::Error,
        /// The context explaining what caused the I/O error.
        context: String,
    },

    /// The input ISLE source has a parse error.
    ParseError {
        /// The error message.
        msg: String,

        /// The location of the parse error.
        span: Span,
    },

    /// The input ISLE source has a type error.
    TypeError {
        /// The error message.
        msg: String,

        /// The location of the type error.
        span: Span,
    },

    /// The rule can never match any input.
    UnreachableError {
        /// The error message.
        msg: String,

        /// The location of the unreachable rule.
        span: Span,
    },

    /// The rules mentioned overlap in the input they accept.
    OverlapError {
        /// The error message.
        msg: String,

        /// The locations of all the rules that overlap. When there are more than two rules
        /// present, the first rule is the one with the most overlaps (likely a fall-through
        /// wildcard case).
        rules: Vec<Span>,
    },

    /// The rules can never match because another rule will always match first.
    ShadowedError {
        /// The locations of the unmatchable rules.
        shadowed: Vec<Span>,

        /// The location of the rule that shadows them.
        mask: Span,
    },
}

impl Errors {
    /// Create new Errors
    pub fn new(errors: Vec<Error>, files: Arc<Files>) -> Self {
        Self { errors, files }
    }

    /// Create `isle::Errors` from the given I/O error and context.
    pub fn from_io(error: std::io::Error, context: impl Into<String>) -> Self {
        Errors {
            errors: vec![Error::IoError {
                error,
                context: context.into(),
            }],
            files: Arc::new(Files::default()),
        }
    }

    #[cfg(feature = "fancy-errors")]
    fn emit(
        &self,
        f: &mut std::fmt::Formatter,
        diagnostics: Vec<Diagnostic<usize>>,
    ) -> std::fmt::Result {
        use codespan_reporting::term::termcolor;
        let w = termcolor::BufferWriter::stderr(termcolor::ColorChoice::Auto);
        let mut b = w.buffer();
        let mut files = codespan_reporting::files::SimpleFiles::new();
        for (name, source) in self
            .files
            .file_names
            .iter()
            .zip(self.files.file_texts.iter())
        {
            files.add(name, source);
        }
        for diagnostic in diagnostics {
            codespan_reporting::term::emit(&mut b, &Default::default(), &files, &diagnostic)
                .map_err(|_| std::fmt::Error)?;
        }
        let b = b.into_inner();
        let b = std::str::from_utf8(&b).map_err(|_| std::fmt::Error)?;
        f.write_str(b)
    }

    #[cfg(not(feature = "fancy-errors"))]
    fn emit(
        &self,
        f: &mut std::fmt::Formatter,
        diagnostics: Vec<Diagnostic<usize>>,
    ) -> std::fmt::Result {
        let pos = |file_id: usize, offset| {
            let ends = self.files.file_line_map(file_id).unwrap();
            let line0 = ends.line(offset);
            let text = &self.files.file_texts[file_id];
            let start = line0.checked_sub(1).map_or(0, |prev| ends[prev]);
            let end = ends.get(line0).copied().unwrap_or(text.len());
            let col = offset - start + 1;
            format!(
                "{}:{}:{}: {}",
                self.files.file_names[file_id],
                line0 + 1,
                col,
                &text[start..end]
            )
        };
        for diagnostic in diagnostics {
            writeln!(f, "{}", diagnostic.message)?;
            for label in diagnostic.labels {
                f.write_str(&pos(label.file_id, label.range.start))?;
            }
            for note in diagnostic.notes {
                writeln!(f, "{note}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError { error, .. } => Some(error),
            _ => None,
        }
    }
}

/// A span in a given source.
#[derive(Clone, Debug)]
pub struct Span {
    /// The byte offset of the start of the span.
    pub from: Pos,
    /// The byte offset of the end of the span.
    pub to: Pos,
}

impl Span {
    /// Create a new span that covers one character at the given offset.
    pub fn new_single(pos: Pos) -> Span {
        Span {
            from: pos,
            // This is a slight hack (we don't actually look at the
            // file to find line/col of next char); but the `to`
            // position only matters for pretty-printed errors and only
            // the offset is used in that case.
            to: Pos {
                file: pos.file,
                offset: pos.offset + 1,
            },
        }
    }
}

impl From<&Span> for std::ops::Range<usize> {
    fn from(span: &Span) -> Self {
        span.from.offset..span.to.offset
    }
}

use diagnostic::{Diagnostic, Label};

#[cfg(feature = "fancy-errors")]
use codespan_reporting::diagnostic;

#[cfg(not(feature = "fancy-errors"))]
/// Minimal versions of types from codespan-reporting.
mod diagnostic {
    use std::ops::Range;

    pub struct Diagnostic<FileId> {
        pub message: String,
        pub labels: Vec<Label<FileId>>,
        pub notes: Vec<String>,
    }

    impl<FileId> Diagnostic<FileId> {
        pub fn error() -> Self {
            Self {
                message: String::new(),
                labels: Vec::new(),
                notes: Vec::new(),
            }
        }

        pub fn with_message(mut self, message: impl Into<String>) -> Self {
            self.message = message.into();
            self
        }

        pub fn with_labels(mut self, labels: Vec<Label<FileId>>) -> Self {
            self.labels = labels;
            self
        }

        pub fn with_notes(mut self, notes: Vec<String>) -> Self {
            self.notes = notes;
            self
        }
    }

    pub struct Label<FileId> {
        pub file_id: FileId,
        pub range: Range<usize>,
    }

    impl<FileId> Label<FileId> {
        pub fn primary(file_id: FileId, range: impl Into<Range<usize>>) -> Self {
            Self {
                file_id,
                range: range.into(),
            }
        }

        pub fn secondary(file_id: FileId, range: impl Into<Range<usize>>) -> Self {
            Self::primary(file_id, range)
        }
    }
}
