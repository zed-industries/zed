use gpui::SharedString;
use lsp::{DiagnosticSeverity, NumberOrString};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Range;

/// A diagnostic associated with a certain range of a buffer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// The name of the service that produced this diagnostic.
    pub source: Option<String>,
    /// The ID provided by the dynamic registration that produced this diagnostic.
    pub registration_id: Option<SharedString>,
    /// A machine-readable code that identifies this diagnostic.
    pub code: Option<NumberOrString>,
    pub code_description: Option<lsp::Uri>,
    /// Whether this diagnostic is a hint, warning, or error.
    pub severity: DiagnosticSeverity,
    /// The human-readable message associated with this diagnostic.
    pub message: String,
    /// The human-readable message (in markdown format)
    pub markdown: Option<String>,
    /// An id that identifies the group to which this diagnostic belongs.
    ///
    /// When a language server produces a diagnostic with
    /// one or more associated diagnostics, those diagnostics are all
    /// assigned a single group ID.
    pub group_id: usize,
    /// Whether this diagnostic is the primary diagnostic for its group.
    ///
    /// In a given group, the primary diagnostic is the top-level diagnostic
    /// returned by the language server. The non-primary diagnostics are the
    /// associated diagnostics.
    pub is_primary: bool,
    /// Whether this diagnostic is considered to originate from an analysis of
    /// files on disk, as opposed to any unsaved buffer contents. This is a
    /// property of a given diagnostic source, and is configured for a given
    /// language server via the `LspAdapter::disk_based_diagnostic_sources` method
    /// for the language server.
    pub is_disk_based: bool,
    /// Whether this diagnostic marks unnecessary code.
    pub is_unnecessary: bool,
    /// Quick separation of diagnostics groups based by their source.
    pub source_kind: DiagnosticSourceKind,
    /// Data from language server that produced this diagnostic. Passed back to the LS when we request code actions for this diagnostic.
    pub data: Option<Value>,
    /// Whether to underline the corresponding text range in the editor.
    pub underline: bool,
}

/// A location and message the language server attached to a diagnostic.
///
/// Kept as the server sent it, since flattening it into non-primary entries is lossy,
/// and passed back to the LS when we request code actions for the diagnostic.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RelatedInformation<T> {
    /// The location the diagnostic points at.
    pub location: RelatedLocation<T>,
    /// The message as the language server sent it.
    pub message: String,
}

/// Where a [`RelatedInformation`] points.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum RelatedLocation<T> {
    /// A range of the buffer the diagnostic belongs to, in the same coordinates as the
    /// diagnostic's own range, so that it follows edits the same way.
    InBuffer(Range<T>),
    /// A location in another file, as the language server published it. There is
    /// nothing in this buffer to anchor it to.
    InAnotherFile(lsp::Location),
}

impl<T: Clone> RelatedInformation<T> {
    /// Converts the coordinates of this related information to a different type.
    pub(crate) fn map_location<O>(
        &self,
        map: impl FnOnce(&Range<T>) -> Range<O>,
    ) -> RelatedInformation<O> {
        RelatedInformation {
            location: match &self.location {
                RelatedLocation::InBuffer(range) => RelatedLocation::InBuffer(map(range)),
                RelatedLocation::InAnotherFile(location) => {
                    RelatedLocation::InAnotherFile(location.clone())
                }
            },
            message: self.message.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSourceKind {
    Pulled,
    Pushed,
    Other,
}

impl Default for Diagnostic {
    fn default() -> Self {
        Self {
            source: Default::default(),
            source_kind: DiagnosticSourceKind::Other,
            code: None,
            code_description: None,
            severity: DiagnosticSeverity::ERROR,
            message: Default::default(),
            markdown: None,
            group_id: 0,
            is_primary: false,
            is_disk_based: false,
            is_unnecessary: false,
            underline: true,
            data: None,
            registration_id: None,
        }
    }
}
