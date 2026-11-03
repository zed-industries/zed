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
        message: lsp::DiagnosticMessage::from("Test error message"),
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
        markup_message_kind: None,
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
    assert_eq!(
        lsp_diagnostic.message,
        lsp::DiagnosticMessage::from("assigned but unused variable - a")
    );
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
        message: lsp::DiagnosticMessage::from("assigned but unused variable - a"),
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
fn test_serialize_lsp_diagnostic_markup_message() {
    let message = lsp::DiagnosticMessage::MarkupContent(lsp::MarkupContent {
        kind: lsp::MarkupKind::Markdown,
        value: "unused variable `a`\n\nsee [docs](https://example.com)".to_string(),
    });
    let lsp_diagnostic = lsp::Diagnostic {
        range: lsp::Range {
            start: lsp::Position::new(0, 1),
            end: lsp::Position::new(2, 3),
        },
        severity: Some(DiagnosticSeverity::WARNING),
        message: message.clone(),
        ..lsp::Diagnostic::default()
    };

    let proto_diagnostic = GetDocumentDiagnostics::serialize_lsp_diagnostic(lsp_diagnostic)
        .expect("Failed to serialize diagnostic");
    assert_eq!(
        proto_diagnostic.message,
        "unused variable `a`\n\nsee [docs](https://example.com)"
    );
    assert_eq!(
        proto_diagnostic.markup_message_kind,
        Some(proto::MarkupKind::Markdown as i32)
    );

    let lsp_diagnostic = GetDocumentDiagnostics::deserialize_lsp_diagnostic(proto_diagnostic)
        .expect("Failed to deserialize diagnostic");
    assert_eq!(lsp_diagnostic.message, message);
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
        markup_message_kind: None,
    };

    let result = GetDocumentDiagnostics::deserialize_lsp_diagnostic(proto_diagnostic);
    assert!(result.is_err());
}

#[gpui::test(iterations = 10)]
async fn test_definitions_load_independent_targets_concurrently(cx: &mut gpui::TestAppContext) {
    check_navigation_loads_independent_targets_concurrently(false, cx).await;
}

#[gpui::test(iterations = 10)]
async fn test_references_load_independent_targets_concurrently(cx: &mut gpui::TestAppContext) {
    check_navigation_loads_independent_targets_concurrently(true, cx).await;
}

async fn check_navigation_loads_independent_targets_concurrently(
    references: bool,
    cx: &mut gpui::TestAppContext,
) {
    use crate::init_test;
    use fs::FakeFs;
    use futures::{FutureExt as _, StreamExt as _};
    use language::{FakeLspAdapter, OffsetRangeExt as _, rust_lang};
    use project::Project;
    use std::time::Duration;
    use util::path;

    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "main.rs": "fn main() {}",
            "first.rs": "pub fn first() {}",
            "second.rs": "pub fn second() {}",
            "third.rs": "pub fn third() {}",
        }),
    )
    .await;
    let root = path!("/dir/main.rs");
    let project = Project::test(fs.clone(), [root.as_ref()], cx).await;
    let registry = project.read_with(cx, |project, _| project.languages().clone());
    registry.add(rust_lang());
    let mut servers = registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                references_provider: Some(lsp::OneOf::Left(true)),
                ..lsp::LanguageServer::full_capabilities()
            },
            ..Default::default()
        },
    );
    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/dir/main.rs"), cx)
        })
        .await
        .unwrap();
    let server = servers.next().await.unwrap();
    cx.run_until_parked();
    let server_id = project.read_with(cx, |project, cx| {
        project
            .lsp_store()
            .read(cx)
            .language_server_statuses()
            .next()
            .unwrap()
            .0
    });

    let names = ["third.rs", "first.rs", "second.rs", "first.rs"];
    for name in names {
        fs.set_load_delay(
            std::path::Path::new(path!("/dir")).join(name),
            Duration::from_millis(40),
        );
    }
    let locations = names
        .into_iter()
        .map(|name| {
            lsp::Location::new(
                lsp::Uri::from_file_path(std::path::Path::new(path!("/dir")).join(name)).unwrap(),
                lsp::Range::new(lsp::Position::new(0, 7), lsp::Position::new(0, 9)),
            )
        })
        .collect::<Vec<_>>();
    let store = project.read_with(cx, |project, _| project.lsp_store());
    server.set_request_handler::<lsp::request::References, _, _>({
        let locations = locations.clone();
        move |_, _| {
            let locations = locations.clone();
            async move { Ok(Some(locations)) }
        }
    });
    let mut task = cx.spawn({
        let buffer = buffer.clone();
        let project = project.clone();
        async move |mut cx| {
            if references {
                project
                    .update(&mut cx, |project, cx| project.references(&buffer, 0, cx))
                    .await
                    .map(|locations| locations.unwrap())
            } else {
                location_links_from_lsp(
                    Some(lsp::GotoDefinitionResponse::Link(
                        locations
                            .into_iter()
                            .map(|location| lsp::LocationLink {
                                origin_selection_range: Some(lsp::Range::new(
                                    lsp::Position::new(0, 3),
                                    lsp::Position::new(0, 5),
                                )),
                                target_uri: location.uri,
                                target_range: lsp::Range::new(
                                    lsp::Position::new(0, 0),
                                    lsp::Position::new(0, 16),
                                ),
                                target_selection_range: location.range,
                            })
                            .collect(),
                    )),
                    store,
                    buffer,
                    server_id,
                    cx.clone(),
                )
                .await
                .map(|links| {
                    for link in &links {
                        let origin = link.origin.as_ref().unwrap();
                        origin.buffer.read_with(&cx, |buffer, _| {
                            assert_eq!(origin.range.to_offset(buffer), 3..5);
                        });
                    }
                    links.into_iter().map(|link| link.target).collect()
                })
            }
        }
    });
    cx.run_until_parked();
    let mut elapsed_ms = 0;
    let locations = loop {
        cx.executor().advance_clock(Duration::from_millis(40));
        elapsed_ms += 40;
        cx.run_until_parked();
        if let Some(locations) = (&mut task).now_or_never() {
            break locations.unwrap();
        }
        assert!(elapsed_ms < 400, "navigation did not finish");
    };
    eprintln!("navigation references={references}: {elapsed_ms} ms of controlled I/O delay");
    assert_eq!(elapsed_ms, 40, "independent target reads must overlap");
    assert_eq!(locations.len(), names.len());
    for (location, name) in locations.iter().zip(names) {
        location.buffer.read_with(cx, |buffer, cx| {
            assert_eq!(
                buffer
                    .file()
                    .unwrap()
                    .as_local()
                    .unwrap()
                    .abs_path(cx)
                    .file_name()
                    .unwrap(),
                name
            );
            assert_eq!(location.range.to_offset(buffer), 7..9);
        });
    }
    assert_eq!(locations[1].buffer, locations[3].buffer);
    cx.run_until_parked();
    assert!(
        servers.try_recv().is_err(),
        "target files must reuse the language server"
    );
    drop(task);
    cx.update(|_| drop(locations));
    cx.run_until_parked();
    assert_eq!(
        project.read_with(cx, |project, cx| project.worktrees(cx).count()),
        1
    );
    if !references {
        let store = project.read_with(cx, |project, _| project.lsp_store());
        let result = cx
            .spawn(async move |cx| {
                location_links_from_lsp(
                    Some(lsp::GotoDefinitionResponse::Array(vec![
                        lsp::Location::new(
                            lsp::Uri::from_str("https://example.invalid/unsupported.rs").unwrap(),
                            lsp::Range::default(),
                        ),
                    ])),
                    store,
                    buffer,
                    server_id,
                    cx,
                )
                .await
            })
            .await;
        assert!(
            result.is_err(),
            "invalid target errors must reach the caller"
        );
    }
    drop(server);
}
