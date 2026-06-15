use gpui_shared_string::SharedString;
use lsp::{DiagnosticSeverity, NumberOrString};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
