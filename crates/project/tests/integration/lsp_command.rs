use std::str::FromStr;

use lsp::{DiagnosticSeverity, DiagnosticTag};
use project::lsp_command::*;
use rpc::proto::{self};
use serde_json::json;

#[test]
fn test_serialize_lsp_diagnostic() {
    let lsp_diagnostic = lsp::Diagnostic {
        range: lsp::Range {
            start: lsp::Position::new(0, 1),
            end: lsp::Position::new(2, 3),
        },
        severity: Some(DiagnosticSeverity::ERROR),
        code: Some(lsp::NumberOrString::String("E001".to_string())),
        source: Some("test-source".to_string()),
        message: "Test error message".to_string(),
        related_information: None,
        tags: Some(vec![DiagnosticTag::DEPRECATED]),
        code_description: None,
        data: Some(json!({"detail": "test detail"})),
    };

    let proto_diagnostic = GetDocumentDiagnostics::serialize_lsp_diagnostic(lsp_diagnostic)
        .expect("Failed to serialize diagnostic");

    let start = proto_diagnostic.start.unwrap();
    let end = proto_diagnostic.end.unwrap();
    assert_eq!(start.row, 0);
    assert_eq!(start.column, 1);
    assert_eq!(end.row, 2);
    assert_eq!(end.column, 3);
    assert_eq!(
        proto_diagnostic.severity,
        proto::lsp_diagnostic::Severity::Error as i32
    );
    assert_eq!(proto_diagnostic.code, Some("E001".to_string()));
    assert_eq!(proto_diagnostic.source, Some("test-source".to_string()));
    assert_eq!(proto_diagnostic.message, "Test error message");
}

#[test]
fn test_deserialize_lsp_diagnostic() {
    let proto_diagnostic = proto::LspDiagnostic {
        start: Some(proto::PointUtf16 { row: 0, column: 1 }),
        end: Some(proto::PointUtf16 { row: 2, column: 3 }),
        severity: proto::lsp_diagnostic::Severity::Warning as i32,
        code: Some("ERR".to_string()),
        source: Some("Prism".to_string()),
        message: "assigned but unused variable - a".to_string(),
        related_information: vec![],
        tags: vec![],
        code_description: None,
        data: None,
    };

    let lsp_diagnostic = GetDocumentDiagnostics::deserialize_lsp_diagnostic(proto_diagnostic)
        .expect("Failed to deserialize diagnostic");

    assert_eq!(lsp_diagnostic.range.start.line, 0);
    assert_eq!(lsp_diagnostic.range.start.character, 1);
    assert_eq!(lsp_diagnostic.range.end.line, 2);
    assert_eq!(lsp_diagnostic.range.end.character, 3);
    assert_eq!(lsp_diagnostic.severity, Some(DiagnosticSeverity::WARNING));
    assert_eq!(
        lsp_diagnostic.code,
        Some(lsp::NumberOrString::String("ERR".to_string()))
    );
    assert_eq!(lsp_diagnostic.source, Some("Prism".to_string()));
    assert_eq!(lsp_diagnostic.message, "assigned but unused variable - a");
}

#[test]
fn test_related_information() {
    let related_info = lsp::DiagnosticRelatedInformation {
        location: lsp::Location {
            uri: lsp::Uri::from_str("file:///test.rs").unwrap(),
            range: lsp::Range {
                start: lsp::Position::new(1, 1),
                end: lsp::Position::new(1, 5),
            },
        },
        message: "Related info message".to_string(),
    };

    let lsp_diagnostic = lsp::Diagnostic {
        range: lsp::Range {
            start: lsp::Position::new(0, 0),
            end: lsp::Position::new(0, 1),
        },
        severity: Some(DiagnosticSeverity::INFORMATION),
        code: None,
        source: Some("Prism".to_string()),
        message: "assigned but unused variable - a".to_string(),
        related_information: Some(vec![related_info]),
        tags: None,
        code_description: None,
        data: None,
    };

    let proto_diagnostic = GetDocumentDiagnostics::serialize_lsp_diagnostic(lsp_diagnostic)
        .expect("Failed to serialize diagnostic");

    assert_eq!(proto_diagnostic.related_information.len(), 1);
    let related = &proto_diagnostic.related_information[0];
    assert_eq!(related.location_url, Some("file:///test.rs".to_string()));
    assert_eq!(related.message, "Related info message");
}

#[test]
fn test_invalid_ranges() {
    let proto_diagnostic = proto::LspDiagnostic {
        start: None,
        end: Some(proto::PointUtf16 { row: 2, column: 3 }),
        severity: proto::lsp_diagnostic::Severity::Error as i32,
        code: None,
        source: None,
        message: "Test message".to_string(),
        related_information: vec![],
        tags: vec![],
        code_description: None,
        data: None,
    };

    let result = GetDocumentDiagnostics::deserialize_lsp_diagnostic(proto_diagnostic);
    assert!(result.is_err());
}
