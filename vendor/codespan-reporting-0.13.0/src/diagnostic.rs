//! Diagnostic data structures.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use core::fmt::Display;
use core::ops::Range;

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

/// A severity level for diagnostic messages.
///
/// These are ordered in the following way:
///
/// ```rust
/// use codespan_reporting::diagnostic::Severity;
///
/// assert!(Severity::Bug > Severity::Error);
/// assert!(Severity::Error > Severity::Warning);
/// assert!(Severity::Warning > Severity::Note);
/// assert!(Severity::Note > Severity::Help);
/// ```
#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum Severity {
    /// A help message.
    Help,
    /// A note.
    Note,
    /// A warning.
    Warning,
    /// An error.
    Error,
    /// An unexpected bug.
    Bug,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum LabelStyle {
    /// Labels that describe the primary cause of a diagnostic.
    Primary,
    /// Labels that provide additional context for a diagnostic.
    Secondary,
}

/// A label describing an underlined region of code associated with a diagnostic.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Label<FileId> {
    /// The style of the label.
    pub style: LabelStyle,
    /// The file that we are labelling.
    pub file_id: FileId,
    /// The range in bytes we are going to include in the final snippet.
    pub range: Range<usize>,
    /// An optional message to provide some additional information for the
    /// underlined code. These should not include line breaks.
    pub message: String,
}

impl<FileId> Label<FileId> {
    /// Create a new label.
    pub fn new(
        style: LabelStyle,
        file_id: FileId,
        range: impl Into<Range<usize>>,
    ) -> Label<FileId> {
        Label {
            style,
            file_id,
            range: range.into(),
            message: String::new(),
        }
    }

    /// Create a new label with a style of [`LabelStyle::Primary`].
    ///
    /// [`LabelStyle::Primary`]: LabelStyle::Primary
    pub fn primary(file_id: FileId, range: impl Into<Range<usize>>) -> Label<FileId> {
        Label::new(LabelStyle::Primary, file_id, range)
    }

    /// Create a new label with a style of [`LabelStyle::Secondary`].
    ///
    /// [`LabelStyle::Secondary`]: LabelStyle::Secondary
    pub fn secondary(file_id: FileId, range: impl Into<Range<usize>>) -> Label<FileId> {
        Label::new(LabelStyle::Secondary, file_id, range)
    }

    /// Add a message to the diagnostic.
    pub fn with_message(mut self, message: impl Display) -> Label<FileId> {
        self.message = message.to_string();
        self
    }
}

/// Represents a diagnostic message that can provide information like errors and
/// warnings to the user.
///
/// The position of a Diagnostic is considered to be the position of the [`Label`] that has the earliest starting position and has the highest style which appears in all the labels of the diagnostic.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Diagnostic<FileId> {
    /// The overall severity of the diagnostic
    pub severity: Severity,
    /// An optional code that identifies this diagnostic.
    pub code: Option<String>,
    /// The main message associated with this diagnostic.
    ///
    /// These should not include line breaks, and in order support the 'short'
    /// diagnostic display mod, the message should be specific enough to make
    /// sense on its own, without additional context provided by labels and notes.
    pub message: String,
    /// Source labels that describe the cause of the diagnostic.
    /// The order of the labels inside the vector does not have any meaning.
    /// The labels are always arranged in the order they appear in the source code.
    pub labels: Vec<Label<FileId>>,
    /// Notes that are associated with the primary cause of the diagnostic.
    /// These can include line breaks for improved formatting.
    pub notes: Vec<String>,
}

impl<FileId> Diagnostic<FileId> {
    /// Create a new diagnostic.
    pub fn new(severity: Severity) -> Diagnostic<FileId> {
        Diagnostic {
            severity,
            code: None,
            message: String::new(),
            labels: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Create a new diagnostic with a severity of [`Severity::Bug`].
    ///
    /// [`Severity::Bug`]: Severity::Bug
    pub fn bug() -> Diagnostic<FileId> {
        Diagnostic::new(Severity::Bug)
    }

    /// Create a new diagnostic with a severity of [`Severity::Error`].
    ///
    /// [`Severity::Error`]: Severity::Error
    pub fn error() -> Diagnostic<FileId> {
        Diagnostic::new(Severity::Error)
    }

    /// Create a new diagnostic with a severity of [`Severity::Warning`].
    ///
    /// [`Severity::Warning`]: Severity::Warning
    pub fn warning() -> Diagnostic<FileId> {
        Diagnostic::new(Severity::Warning)
    }

    /// Create a new diagnostic with a severity of [`Severity::Note`].
    ///
    /// [`Severity::Note`]: Severity::Note
    pub fn note() -> Diagnostic<FileId> {
        Diagnostic::new(Severity::Note)
    }

    /// Create a new diagnostic with a severity of [`Severity::Help`].
    ///
    /// [`Severity::Help`]: Severity::Help
    pub fn help() -> Diagnostic<FileId> {
        Diagnostic::new(Severity::Help)
    }

    /// Set the error code of the diagnostic.
    pub fn with_code(mut self, code: impl Display) -> Diagnostic<FileId> {
        self.code = Some(code.to_string());
        self
    }

    /// Set the message of the diagnostic.
    pub fn with_message(mut self, message: impl Display) -> Diagnostic<FileId> {
        self.message = message.to_string();
        self
    }

    /// Add a label to the diagnostic.
    pub fn with_label(mut self, label: Label<FileId>) -> Diagnostic<FileId> {
        self.labels.push(label);
        self
    }

    /// Add some labels to the diagnostic.
    pub fn with_labels(mut self, mut labels: Vec<Label<FileId>>) -> Diagnostic<FileId> {
        self.labels.append(&mut labels);
        self
    }

    /// Add some labels to the diagnostic.
    pub fn with_labels_iter(
        mut self,
        labels: impl IntoIterator<Item = Label<FileId>>,
    ) -> Diagnostic<FileId> {
        self.labels.extend(labels);
        self
    }

    /// Add a note to the diagnostic.
    pub fn with_note(mut self, note: &impl ToString) -> Diagnostic<FileId> {
        self.notes.push(note.to_string());
        self
    }

    /// Add some notes to the diagnostic.
    pub fn with_notes(mut self, mut notes: Vec<String>) -> Diagnostic<FileId> {
        self.notes.append(&mut notes);
        self
    }

    /// Add some notes to the diagnostic.
    pub fn with_notes_iter(
        mut self,
        notes: impl IntoIterator<Item = String>,
    ) -> Diagnostic<FileId> {
        self.notes.extend(notes);
        self
    }
}
