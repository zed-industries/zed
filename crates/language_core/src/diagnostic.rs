use gpui_shared_string::SharedString;
use lsp_types::{DiagnosticSeverity, NumberOrString};
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
    pub code_description: Option<lsp_types::Uri>,
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

pub fn format_diagnostic_for_clipboard(
    diagnostic: &Diagnostic,
    file_path: Option<&str>,
    line: u32,
    column: u32,
) -> String {
    let mut result = String::new();

    let severity_label = match diagnostic.severity {
        DiagnosticSeverity::ERROR => "error",
        DiagnosticSeverity::WARNING => "warning",
        DiagnosticSeverity::INFORMATION => "info",
        DiagnosticSeverity::HINT => "hint",
        _ => "diagnostic",
    };
    result.push_str(severity_label);

    if let Some(code) = &diagnostic.code {
        result.push('[');
        result.push_str(&code.to_string());
        result.push(']');
    }

    if let Some(source) = &diagnostic.source {
        result.push_str(" (");
        result.push_str(source);
        result.push(')');
    }

    result.push_str(": ");
    result.push_str(&diagnostic.message);

    if let Some(path) = file_path {
        result.push_str("\n  --> ");
        result.push_str(path);
        result.push(':');
        result.push_str(&line.to_string());
        result.push(':');
        result.push_str(&column.to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_diagnostic(
        severity: DiagnosticSeverity,
        message: &str,
        source: Option<&str>,
        code: Option<lsp::NumberOrString>,
    ) -> Diagnostic {
        Diagnostic {
            severity,
            message: message.to_string(),
            source: source.map(|s| s.to_string()),
            code,
            ..Default::default()
        }
    }

    #[test]
    fn test_format_all_fields() {
        let diagnostic = make_diagnostic(
            DiagnosticSeverity::ERROR,
            "Type 'string' is not assignable to type 'number'",
            Some("typescript"),
            Some(lsp::NumberOrString::String("ts(2322)".to_string())),
        );
        let result = format_diagnostic_for_clipboard(
            &diagnostic,
            Some("src/components/App.tsx"),
            42,
            5,
        );
        assert_eq!(
            result,
            "error[ts(2322)] (typescript): Type 'string' is not assignable to type 'number'\n  --> src/components/App.tsx:42:5"
        );
    }

    #[test]
    fn test_format_no_code() {
        let diagnostic = make_diagnostic(
            DiagnosticSeverity::WARNING,
            "Unused variable",
            Some("eslint"),
            None,
        );
        let result = format_diagnostic_for_clipboard(&diagnostic, Some("src/main.rs"), 10, 1);
        assert_eq!(
            result,
            "warning (eslint): Unused variable\n  --> src/main.rs:10:1"
        );
    }

    #[test]
    fn test_format_no_source() {
        let diagnostic = make_diagnostic(
            DiagnosticSeverity::ERROR,
            "expected semicolon",
            None,
            Some(lsp::NumberOrString::Number(1002)),
        );
        let result = format_diagnostic_for_clipboard(&diagnostic, Some("lib.rs"), 5, 20);
        assert_eq!(
            result,
            "error[1002]: expected semicolon\n  --> lib.rs:5:20"
        );
    }

    #[test]
    fn test_format_no_code_no_source() {
        let diagnostic = make_diagnostic(
            DiagnosticSeverity::HINT,
            "Consider refactoring",
            None,
            None,
        );
        let result = format_diagnostic_for_clipboard(&diagnostic, Some("app.py"), 1, 1);
        assert_eq!(
            result,
            "hint: Consider refactoring\n  --> app.py:1:1"
        );
    }

    #[test]
    fn test_format_no_file_path() {
        let diagnostic = make_diagnostic(
            DiagnosticSeverity::ERROR,
            "syntax error",
            Some("rustc"),
            None,
        );
        let result = format_diagnostic_for_clipboard(&diagnostic, None, 1, 1);
        assert_eq!(result, "error (rustc): syntax error");
    }

    #[test]
    fn test_format_information_severity() {
        let diagnostic = make_diagnostic(
            DiagnosticSeverity::INFORMATION,
            "Info message",
            None,
            None,
        );
        let result = format_diagnostic_for_clipboard(&diagnostic, Some("test.rs"), 3, 7);
        assert_eq!(
            result,
            "info: Info message\n  --> test.rs:3:7"
        );
    }

}
