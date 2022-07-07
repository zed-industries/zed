use crate::{worktree::WorktreeHandle, Event, *};
use fs::RealFs;
use futures::{future, StreamExt};
use gpui::{executor::Deterministic, test::subscribe};
use language::{
    tree_sitter_rust, tree_sitter_typescript, Diagnostic, FakeLspAdapter, LanguageConfig,
    LineEnding, OffsetRangeExt, Point, ToPoint,
};
use lsp::Url;
use serde_json::json;
use std::{cell::RefCell, os::unix, path::PathBuf, rc::Rc, task::Poll};
use unindent::Unindent as _;
use util::{assert_set_eq, test::temp_tree};

#[gpui::test]
async fn test_populate_and_search(cx: &mut gpui::TestAppContext) {
    let dir = temp_tree(json!({
        "root": {
            "apple": "",
            "banana": {
                "carrot": {
                    "date": "",
                    "endive": "",
                }
            },
            "fennel": {
                "grape": "",
            }
        }
    }));

    let root_link_path = dir.path().join("root_link");
    unix::fs::symlink(&dir.path().join("root"), &root_link_path).unwrap();
    unix::fs::symlink(
        &dir.path().join("root/fennel"),
        &dir.path().join("root/finnochio"),
    )
    .unwrap();

    let project = Project::test(Arc::new(RealFs), [root_link_path.as_ref()], cx).await;

    project.read_with(cx, |project, cx| {
        let tree = project.worktrees(cx).next().unwrap().read(cx);
        assert_eq!(tree.file_count(), 5);
        assert_eq!(
            tree.inode_for_path("fennel/grape"),
            tree.inode_for_path("finnochio/grape")
        );
    });

    let cancel_flag = Default::default();
    let results = project
        .read_with(cx, |project, cx| {
            project.match_paths("bna", false, false, 10, &cancel_flag, cx)
        })
        .await;
    assert_eq!(
        results
            .into_iter()
            .map(|result| result.path)
            .collect::<Vec<Arc<Path>>>(),
        vec![
            PathBuf::from("banana/carrot/date").into(),
            PathBuf::from("banana/carrot/endive").into(),
        ]
    );
}

#[gpui::test]
async fn test_managing_language_servers(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let mut rust_language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut json_language = Language::new(
        LanguageConfig {
            name: "JSON".into(),
            path_suffixes: vec!["json".to_string()],
            ..Default::default()
        },
        None,
    );
    let mut fake_rust_servers = rust_language.set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
        name: "the-rust-language-server",
        capabilities: lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                trigger_characters: Some(vec![".".to_string(), "::".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    }));
    let mut fake_json_servers = json_language.set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
        name: "the-json-language-server",
        capabilities: lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                trigger_characters: Some(vec![":".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    }));

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/the-root",
        json!({
            "test.rs": "const A: i32 = 1;",
            "test2.rs": "",
            "Cargo.toml": "a = 1",
            "package.json": "{\"a\": 1}",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/the-root".as_ref()], cx).await;
    project.update(cx, |project, _| {
        project.languages.add(Arc::new(rust_language));
        project.languages.add(Arc::new(json_language));
    });

    // Open a buffer without an associated language server.
    let toml_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/the-root/Cargo.toml", cx)
        })
        .await
        .unwrap();

    // Open a buffer with an associated language server.
    let rust_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/the-root/test.rs", cx)
        })
        .await
        .unwrap();

    // A server is started up, and it is notified about Rust files.
    let mut fake_rust_server = fake_rust_servers.next().await.unwrap();
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentItem {
            uri: lsp::Url::from_file_path("/the-root/test.rs").unwrap(),
            version: 0,
            text: "const A: i32 = 1;".to_string(),
            language_id: Default::default()
        }
    );

    // The buffer is configured based on the language server's capabilities.
    rust_buffer.read_with(cx, |buffer, _| {
        assert_eq!(
            buffer.completion_triggers(),
            &[".".to_string(), "::".to_string()]
        );
    });
    toml_buffer.read_with(cx, |buffer, _| {
        assert!(buffer.completion_triggers().is_empty());
    });

    // Edit a buffer. The changes are reported to the language server.
    rust_buffer.update(cx, |buffer, cx| buffer.edit([(16..16, "2")], cx));
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidChangeTextDocument>()
            .await
            .text_document,
        lsp::VersionedTextDocumentIdentifier::new(
            lsp::Url::from_file_path("/the-root/test.rs").unwrap(),
            1
        )
    );

    // Open a third buffer with a different associated language server.
    let json_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/the-root/package.json", cx)
        })
        .await
        .unwrap();

    // A json language server is started up and is only notified about the json buffer.
    let mut fake_json_server = fake_json_servers.next().await.unwrap();
    assert_eq!(
        fake_json_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentItem {
            uri: lsp::Url::from_file_path("/the-root/package.json").unwrap(),
            version: 0,
            text: "{\"a\": 1}".to_string(),
            language_id: Default::default()
        }
    );

    // This buffer is configured based on the second language server's
    // capabilities.
    json_buffer.read_with(cx, |buffer, _| {
        assert_eq!(buffer.completion_triggers(), &[":".to_string()]);
    });

    // When opening another buffer whose language server is already running,
    // it is also configured based on the existing language server's capabilities.
    let rust_buffer2 = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/the-root/test2.rs", cx)
        })
        .await
        .unwrap();
    rust_buffer2.read_with(cx, |buffer, _| {
        assert_eq!(
            buffer.completion_triggers(),
            &[".".to_string(), "::".to_string()]
        );
    });

    // Changes are reported only to servers matching the buffer's language.
    toml_buffer.update(cx, |buffer, cx| buffer.edit([(5..5, "23")], cx));
    rust_buffer2.update(cx, |buffer, cx| buffer.edit([(0..0, "let x = 1;")], cx));
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidChangeTextDocument>()
            .await
            .text_document,
        lsp::VersionedTextDocumentIdentifier::new(
            lsp::Url::from_file_path("/the-root/test2.rs").unwrap(),
            1
        )
    );

    // Save notifications are reported to all servers.
    toml_buffer
        .update(cx, |buffer, cx| buffer.save(cx))
        .await
        .unwrap();
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidSaveTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentIdentifier::new(lsp::Url::from_file_path("/the-root/Cargo.toml").unwrap())
    );
    assert_eq!(
        fake_json_server
            .receive_notification::<lsp::notification::DidSaveTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentIdentifier::new(lsp::Url::from_file_path("/the-root/Cargo.toml").unwrap())
    );

    // Renames are reported only to servers matching the buffer's language.
    fs.rename(
        Path::new("/the-root/test2.rs"),
        Path::new("/the-root/test3.rs"),
        Default::default(),
    )
    .await
    .unwrap();
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidCloseTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentIdentifier::new(lsp::Url::from_file_path("/the-root/test2.rs").unwrap()),
    );
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentItem {
            uri: lsp::Url::from_file_path("/the-root/test3.rs").unwrap(),
            version: 0,
            text: rust_buffer2.read_with(cx, |buffer, _| buffer.text()),
            language_id: Default::default()
        },
    );

    rust_buffer2.update(cx, |buffer, cx| {
        buffer.update_diagnostics(
            DiagnosticSet::from_sorted_entries(
                vec![DiagnosticEntry {
                    diagnostic: Default::default(),
                    range: Anchor::MIN..Anchor::MAX,
                }],
                &buffer.snapshot(),
            ),
            cx,
        );
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, usize>(0..buffer.len(), false)
                .count(),
            1
        );
    });

    // When the rename changes the extension of the file, the buffer gets closed on the old
    // language server and gets opened on the new one.
    fs.rename(
        Path::new("/the-root/test3.rs"),
        Path::new("/the-root/test3.json"),
        Default::default(),
    )
    .await
    .unwrap();
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidCloseTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentIdentifier::new(lsp::Url::from_file_path("/the-root/test3.rs").unwrap(),),
    );
    assert_eq!(
        fake_json_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentItem {
            uri: lsp::Url::from_file_path("/the-root/test3.json").unwrap(),
            version: 0,
            text: rust_buffer2.read_with(cx, |buffer, _| buffer.text()),
            language_id: Default::default()
        },
    );

    // We clear the diagnostics, since the language has changed.
    rust_buffer2.read_with(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, usize>(0..buffer.len(), false)
                .count(),
            0
        );
    });

    // The renamed file's version resets after changing language server.
    rust_buffer2.update(cx, |buffer, cx| buffer.edit([(0..0, "// ")], cx));
    assert_eq!(
        fake_json_server
            .receive_notification::<lsp::notification::DidChangeTextDocument>()
            .await
            .text_document,
        lsp::VersionedTextDocumentIdentifier::new(
            lsp::Url::from_file_path("/the-root/test3.json").unwrap(),
            1
        )
    );

    // Restart language servers
    project.update(cx, |project, cx| {
        project.restart_language_servers_for_buffers(
            vec![rust_buffer.clone(), json_buffer.clone()],
            cx,
        );
    });

    let mut rust_shutdown_requests = fake_rust_server
        .handle_request::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));
    let mut json_shutdown_requests = fake_json_server
        .handle_request::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));
    futures::join!(rust_shutdown_requests.next(), json_shutdown_requests.next());

    let mut fake_rust_server = fake_rust_servers.next().await.unwrap();
    let mut fake_json_server = fake_json_servers.next().await.unwrap();

    // Ensure rust document is reopened in new rust language server
    assert_eq!(
        fake_rust_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentItem {
            uri: lsp::Url::from_file_path("/the-root/test.rs").unwrap(),
            version: 1,
            text: rust_buffer.read_with(cx, |buffer, _| buffer.text()),
            language_id: Default::default()
        }
    );

    // Ensure json documents are reopened in new json language server
    assert_set_eq!(
        [
            fake_json_server
                .receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await
                .text_document,
            fake_json_server
                .receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await
                .text_document,
        ],
        [
            lsp::TextDocumentItem {
                uri: lsp::Url::from_file_path("/the-root/package.json").unwrap(),
                version: 0,
                text: json_buffer.read_with(cx, |buffer, _| buffer.text()),
                language_id: Default::default()
            },
            lsp::TextDocumentItem {
                uri: lsp::Url::from_file_path("/the-root/test3.json").unwrap(),
                version: 1,
                text: rust_buffer2.read_with(cx, |buffer, _| buffer.text()),
                language_id: Default::default()
            }
        ]
    );

    // Close notifications are reported only to servers matching the buffer's language.
    cx.update(|_| drop(json_buffer));
    let close_message = lsp::DidCloseTextDocumentParams {
        text_document: lsp::TextDocumentIdentifier::new(
            lsp::Url::from_file_path("/the-root/package.json").unwrap(),
        ),
    };
    assert_eq!(
        fake_json_server
            .receive_notification::<lsp::notification::DidCloseTextDocument>()
            .await,
        close_message,
    );
}

#[gpui::test]
async fn test_single_file_worktrees_diagnostics(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "a.rs": "let a = 1;",
            "b.rs": "let b = 2;"
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir/a.rs".as_ref(), "/dir/b.rs".as_ref()], cx).await;

    let buffer_a = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();
    let buffer_b = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/b.rs", cx))
        .await
        .unwrap();

    project.update(cx, |project, cx| {
        project
            .update_diagnostics(
                0,
                lsp::PublishDiagnosticsParams {
                    uri: Url::from_file_path("/dir/a.rs").unwrap(),
                    version: None,
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 5)),
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: "error 1".to_string(),
                        ..Default::default()
                    }],
                },
                &[],
                cx,
            )
            .unwrap();
        project
            .update_diagnostics(
                0,
                lsp::PublishDiagnosticsParams {
                    uri: Url::from_file_path("/dir/b.rs").unwrap(),
                    version: None,
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 5)),
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        message: "error 2".to_string(),
                        ..Default::default()
                    }],
                },
                &[],
                cx,
            )
            .unwrap();
    });

    buffer_a.read_with(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(&buffer, 0..buffer.len());
        assert_eq!(
            chunks
                .iter()
                .map(|(s, d)| (s.as_str(), *d))
                .collect::<Vec<_>>(),
            &[
                ("let ", None),
                ("a", Some(DiagnosticSeverity::ERROR)),
                (" = 1;", None),
            ]
        );
    });
    buffer_b.read_with(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(&buffer, 0..buffer.len());
        assert_eq!(
            chunks
                .iter()
                .map(|(s, d)| (s.as_str(), *d))
                .collect::<Vec<_>>(),
            &[
                ("let ", None),
                ("b", Some(DiagnosticSeverity::WARNING)),
                (" = 2;", None),
            ]
        );
    });
}

#[gpui::test]
async fn test_hidden_worktrees_diagnostics(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/root",
        json!({
            "dir": {
                "a.rs": "let a = 1;",
            },
            "other.rs": "let b = c;"
        }),
    )
    .await;

    let project = Project::test(fs, ["/root/dir".as_ref()], cx).await;

    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_local_worktree("/root/other.rs", false, cx)
        })
        .await
        .unwrap();
    let worktree_id = worktree.read_with(cx, |tree, _| tree.id());

    project.update(cx, |project, cx| {
        project
            .update_diagnostics(
                0,
                lsp::PublishDiagnosticsParams {
                    uri: Url::from_file_path("/root/other.rs").unwrap(),
                    version: None,
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 8), lsp::Position::new(0, 9)),
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: "unknown variable 'c'".to_string(),
                        ..Default::default()
                    }],
                },
                &[],
                cx,
            )
            .unwrap();
    });

    let buffer = project
        .update(cx, |project, cx| project.open_buffer((worktree_id, ""), cx))
        .await
        .unwrap();
    buffer.read_with(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(&buffer, 0..buffer.len());
        assert_eq!(
            chunks
                .iter()
                .map(|(s, d)| (s.as_str(), *d))
                .collect::<Vec<_>>(),
            &[
                ("let b = ", None),
                ("c", Some(DiagnosticSeverity::ERROR)),
                (";", None),
            ]
        );
    });

    project.read_with(cx, |project, cx| {
        assert_eq!(project.diagnostic_summaries(cx).next(), None);
        assert_eq!(project.diagnostic_summary(cx).error_count, 0);
    });
}

#[gpui::test]
async fn test_disk_based_diagnostics_progress(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let progress_token = "the-progress-token";
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_servers = language.set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
        disk_based_diagnostics_progress_token: Some(progress_token.into()),
        disk_based_diagnostics_sources: vec!["disk".into()],
        ..Default::default()
    }));

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "a.rs": "fn a() { A }",
            "b.rs": "const y: i32 = 1",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    project.update(cx, |project, _| project.languages.add(Arc::new(language)));
    let worktree_id = project.read_with(cx, |p, cx| p.worktrees(cx).next().unwrap().read(cx).id());

    // Cause worktree to start the fake language server
    let _buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/b.rs", cx))
        .await
        .unwrap();

    let mut events = subscribe(&project, cx);

    let fake_server = fake_servers.next().await.unwrap();
    fake_server.start_progress(progress_token).await;
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiskBasedDiagnosticsStarted {
            language_server_id: 0,
        }
    );

    fake_server.notify::<lsp::notification::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
        uri: Url::from_file_path("/dir/a.rs").unwrap(),
        version: None,
        diagnostics: vec![lsp::Diagnostic {
            range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
            severity: Some(lsp::DiagnosticSeverity::ERROR),
            message: "undefined variable 'A'".to_string(),
            ..Default::default()
        }],
    });
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiagnosticsUpdated {
            language_server_id: 0,
            path: (worktree_id, Path::new("a.rs")).into()
        }
    );

    fake_server.end_progress(progress_token);
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiskBasedDiagnosticsFinished {
            language_server_id: 0
        }
    );

    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();

    buffer.read_with(cx, |buffer, _| {
        let snapshot = buffer.snapshot();
        let diagnostics = snapshot
            .diagnostics_in_range::<_, Point>(0..buffer.len(), false)
            .collect::<Vec<_>>();
        assert_eq!(
            diagnostics,
            &[DiagnosticEntry {
                range: Point::new(0, 9)..Point::new(0, 10),
                diagnostic: Diagnostic {
                    severity: lsp::DiagnosticSeverity::ERROR,
                    message: "undefined variable 'A'".to_string(),
                    group_id: 0,
                    is_primary: true,
                    ..Default::default()
                }
            }]
        )
    });

    // Ensure publishing empty diagnostics twice only results in one update event.
    fake_server.notify::<lsp::notification::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
        uri: Url::from_file_path("/dir/a.rs").unwrap(),
        version: None,
        diagnostics: Default::default(),
    });
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiagnosticsUpdated {
            language_server_id: 0,
            path: (worktree_id, Path::new("a.rs")).into()
        }
    );

    fake_server.notify::<lsp::notification::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
        uri: Url::from_file_path("/dir/a.rs").unwrap(),
        version: None,
        diagnostics: Default::default(),
    });
    cx.foreground().run_until_parked();
    assert_eq!(futures::poll!(events.next()), Poll::Pending);
}

#[gpui::test]
async fn test_restarting_server_with_diagnostics_running(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let progress_token = "the-progress-token";
    let mut language = Language::new(
        LanguageConfig {
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        None,
    );
    let mut fake_servers = language.set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
        disk_based_diagnostics_sources: vec!["disk".into()],
        disk_based_diagnostics_progress_token: Some(progress_token.into()),
        ..Default::default()
    }));

    let fs = FakeFs::new(cx.background());
    fs.insert_tree("/dir", json!({ "a.rs": "" })).await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    project.update(cx, |project, _| project.languages.add(Arc::new(language)));

    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();

    // Simulate diagnostics starting to update.
    let fake_server = fake_servers.next().await.unwrap();
    fake_server.start_progress(progress_token).await;

    // Restart the server before the diagnostics finish updating.
    project.update(cx, |project, cx| {
        project.restart_language_servers_for_buffers([buffer], cx);
    });
    let mut events = subscribe(&project, cx);

    // Simulate the newly started server sending more diagnostics.
    let fake_server = fake_servers.next().await.unwrap();
    fake_server.start_progress(progress_token).await;
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiskBasedDiagnosticsStarted {
            language_server_id: 1
        }
    );
    project.read_with(cx, |project, _| {
        assert_eq!(
            project
                .language_servers_running_disk_based_diagnostics()
                .collect::<Vec<_>>(),
            [1]
        );
    });

    // All diagnostics are considered done, despite the old server's diagnostic
    // task never completing.
    fake_server.end_progress(progress_token);
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiskBasedDiagnosticsFinished {
            language_server_id: 1
        }
    );
    project.read_with(cx, |project, _| {
        assert_eq!(
            project
                .language_servers_running_disk_based_diagnostics()
                .collect::<Vec<_>>(),
            [0; 0]
        );
    });
}

#[gpui::test]
async fn test_toggling_enable_language_server(
    deterministic: Arc<Deterministic>,
    cx: &mut gpui::TestAppContext,
) {
    deterministic.forbid_parking();

    let mut rust = Language::new(
        LanguageConfig {
            name: Arc::from("Rust"),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        None,
    );
    let mut fake_rust_servers = rust.set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
        name: "rust-lsp",
        ..Default::default()
    }));
    let mut js = Language::new(
        LanguageConfig {
            name: Arc::from("JavaScript"),
            path_suffixes: vec!["js".to_string()],
            ..Default::default()
        },
        None,
    );
    let mut fake_js_servers = js.set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
        name: "js-lsp",
        ..Default::default()
    }));

    let fs = FakeFs::new(cx.background());
    fs.insert_tree("/dir", json!({ "a.rs": "", "b.js": "" }))
        .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    project.update(cx, |project, _| {
        project.languages.add(Arc::new(rust));
        project.languages.add(Arc::new(js));
    });

    let _rs_buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();
    let _js_buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/b.js", cx))
        .await
        .unwrap();

    let mut fake_rust_server_1 = fake_rust_servers.next().await.unwrap();
    assert_eq!(
        fake_rust_server_1
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document
            .uri
            .as_str(),
        "file:///dir/a.rs"
    );

    let mut fake_js_server = fake_js_servers.next().await.unwrap();
    assert_eq!(
        fake_js_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document
            .uri
            .as_str(),
        "file:///dir/b.js"
    );

    // Disable Rust language server, ensuring only that server gets stopped.
    cx.update(|cx| {
        cx.update_global(|settings: &mut Settings, _| {
            settings.language_overrides.insert(
                Arc::from("Rust"),
                settings::LanguageSettings {
                    enable_language_server: Some(false),
                    ..Default::default()
                },
            );
        })
    });
    fake_rust_server_1
        .receive_notification::<lsp::notification::Exit>()
        .await;

    // Enable Rust and disable JavaScript language servers, ensuring that the
    // former gets started again and that the latter stops.
    cx.update(|cx| {
        cx.update_global(|settings: &mut Settings, _| {
            settings.language_overrides.insert(
                Arc::from("Rust"),
                settings::LanguageSettings {
                    enable_language_server: Some(true),
                    ..Default::default()
                },
            );
            settings.language_overrides.insert(
                Arc::from("JavaScript"),
                settings::LanguageSettings {
                    enable_language_server: Some(false),
                    ..Default::default()
                },
            );
        })
    });
    let mut fake_rust_server_2 = fake_rust_servers.next().await.unwrap();
    assert_eq!(
        fake_rust_server_2
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document
            .uri
            .as_str(),
        "file:///dir/a.rs"
    );
    fake_js_server
        .receive_notification::<lsp::notification::Exit>()
        .await;
}

#[gpui::test]
async fn test_transforming_diagnostics(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_servers = language.set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
        disk_based_diagnostics_sources: vec!["disk".into()],
        ..Default::default()
    }));

    let text = "
        fn a() { A }
        fn b() { BB }
        fn c() { CCC }
    "
    .unindent();

    let fs = FakeFs::new(cx.background());
    fs.insert_tree("/dir", json!({ "a.rs": text })).await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    project.update(cx, |project, _| project.languages.add(Arc::new(language)));

    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();

    let mut fake_server = fake_servers.next().await.unwrap();
    let open_notification = fake_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;

    // Edit the buffer, moving the content down
    buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "\n\n")], cx));
    let change_notification_1 = fake_server
        .receive_notification::<lsp::notification::DidChangeTextDocument>()
        .await;
    assert!(change_notification_1.text_document.version > open_notification.text_document.version);

    // Report some diagnostics for the initial version of the buffer
    fake_server.notify::<lsp::notification::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
        uri: lsp::Url::from_file_path("/dir/a.rs").unwrap(),
        version: Some(open_notification.text_document.version),
        diagnostics: vec![
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: "undefined variable 'A'".to_string(),
                source: Some("disk".to_string()),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(1, 9), lsp::Position::new(1, 11)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: "undefined variable 'BB'".to_string(),
                source: Some("disk".to_string()),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(2, 9), lsp::Position::new(2, 12)),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("disk".to_string()),
                message: "undefined variable 'CCC'".to_string(),
                ..Default::default()
            },
        ],
    });

    // The diagnostics have moved down since they were created.
    buffer.next_notification(cx).await;
    buffer.read_with(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(Point::new(3, 0)..Point::new(5, 0), false)
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(3, 9)..Point::new(3, 11),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'BB'".to_string(),
                        is_disk_based: true,
                        group_id: 1,
                        is_primary: true,
                        ..Default::default()
                    },
                },
                DiagnosticEntry {
                    range: Point::new(4, 9)..Point::new(4, 12),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'CCC'".to_string(),
                        is_disk_based: true,
                        group_id: 2,
                        is_primary: true,
                        ..Default::default()
                    }
                }
            ]
        );
        assert_eq!(
            chunks_with_diagnostics(buffer, 0..buffer.len()),
            [
                ("\n\nfn a() { ".to_string(), None),
                ("A".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }\nfn b() { ".to_string(), None),
                ("BB".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }\nfn c() { ".to_string(), None),
                ("CCC".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }\n".to_string(), None),
            ]
        );
        assert_eq!(
            chunks_with_diagnostics(buffer, Point::new(3, 10)..Point::new(4, 11)),
            [
                ("B".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }\nfn c() { ".to_string(), None),
                ("CC".to_string(), Some(DiagnosticSeverity::ERROR)),
            ]
        );
    });

    // Ensure overlapping diagnostics are highlighted correctly.
    fake_server.notify::<lsp::notification::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
        uri: lsp::Url::from_file_path("/dir/a.rs").unwrap(),
        version: Some(open_notification.text_document.version),
        diagnostics: vec![
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: "undefined variable 'A'".to_string(),
                source: Some("disk".to_string()),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 12)),
                severity: Some(DiagnosticSeverity::WARNING),
                message: "unreachable statement".to_string(),
                source: Some("disk".to_string()),
                ..Default::default()
            },
        ],
    });

    buffer.next_notification(cx).await;
    buffer.read_with(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(Point::new(2, 0)..Point::new(3, 0), false)
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(2, 9)..Point::new(2, 12),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::WARNING,
                        message: "unreachable statement".to_string(),
                        is_disk_based: true,
                        group_id: 4,
                        is_primary: true,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(2, 9)..Point::new(2, 10),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'A'".to_string(),
                        is_disk_based: true,
                        group_id: 3,
                        is_primary: true,
                        ..Default::default()
                    },
                }
            ]
        );
        assert_eq!(
            chunks_with_diagnostics(buffer, Point::new(2, 0)..Point::new(3, 0)),
            [
                ("fn a() { ".to_string(), None),
                ("A".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }".to_string(), Some(DiagnosticSeverity::WARNING)),
                ("\n".to_string(), None),
            ]
        );
        assert_eq!(
            chunks_with_diagnostics(buffer, Point::new(2, 10)..Point::new(3, 0)),
            [
                (" }".to_string(), Some(DiagnosticSeverity::WARNING)),
                ("\n".to_string(), None),
            ]
        );
    });

    // Keep editing the buffer and ensure disk-based diagnostics get translated according to the
    // changes since the last save.
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(Point::new(2, 0)..Point::new(2, 0), "    ")], cx);
        buffer.edit([(Point::new(2, 8)..Point::new(2, 10), "(x: usize)")], cx);
        buffer.edit([(Point::new(3, 10)..Point::new(3, 10), "xxx")], cx);
    });
    let change_notification_2 = fake_server
        .receive_notification::<lsp::notification::DidChangeTextDocument>()
        .await;
    assert!(
        change_notification_2.text_document.version > change_notification_1.text_document.version
    );

    // Handle out-of-order diagnostics
    fake_server.notify::<lsp::notification::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
        uri: lsp::Url::from_file_path("/dir/a.rs").unwrap(),
        version: Some(change_notification_2.text_document.version),
        diagnostics: vec![
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(1, 9), lsp::Position::new(1, 11)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: "undefined variable 'BB'".to_string(),
                source: Some("disk".to_string()),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                severity: Some(DiagnosticSeverity::WARNING),
                message: "undefined variable 'A'".to_string(),
                source: Some("disk".to_string()),
                ..Default::default()
            },
        ],
    });

    buffer.next_notification(cx).await;
    buffer.read_with(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(0..buffer.len(), false)
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(2, 21)..Point::new(2, 22),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::WARNING,
                        message: "undefined variable 'A'".to_string(),
                        is_disk_based: true,
                        group_id: 6,
                        is_primary: true,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(3, 9)..Point::new(3, 14),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'BB'".to_string(),
                        is_disk_based: true,
                        group_id: 5,
                        is_primary: true,
                        ..Default::default()
                    },
                }
            ]
        );
    });
}

#[gpui::test]
async fn test_empty_diagnostic_ranges(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let text = concat!(
        "let one = ;\n", //
        "let two = \n",
        "let three = 3;\n",
    );

    let fs = FakeFs::new(cx.background());
    fs.insert_tree("/dir", json!({ "a.rs": text })).await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();

    project.update(cx, |project, cx| {
        project
            .update_buffer_diagnostics(
                &buffer,
                vec![
                    DiagnosticEntry {
                        range: PointUtf16::new(0, 10)..PointUtf16::new(0, 10),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "syntax error 1".to_string(),
                            ..Default::default()
                        },
                    },
                    DiagnosticEntry {
                        range: PointUtf16::new(1, 10)..PointUtf16::new(1, 10),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "syntax error 2".to_string(),
                            ..Default::default()
                        },
                    },
                ],
                None,
                cx,
            )
            .unwrap();
    });

    // An empty range is extended forward to include the following character.
    // At the end of a line, an empty range is extended backward to include
    // the preceding character.
    buffer.read_with(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(&buffer, 0..buffer.len());
        assert_eq!(
            chunks
                .iter()
                .map(|(s, d)| (s.as_str(), *d))
                .collect::<Vec<_>>(),
            &[
                ("let one = ", None),
                (";", Some(DiagnosticSeverity::ERROR)),
                ("\nlet two =", None),
                (" ", Some(DiagnosticSeverity::ERROR)),
                ("\nlet three = 3;\n", None)
            ]
        );
    });
}

#[gpui::test]
async fn test_edits_from_lsp_with_past_version(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_servers = language.set_fake_lsp_adapter(Default::default());

    let text = "
        fn a() {
            f1();
        }
        fn b() {
            f2();
        }
        fn c() {
            f3();
        }
    "
    .unindent();

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "a.rs": text.clone(),
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    project.update(cx, |project, _| project.languages.add(Arc::new(language)));
    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();

    let mut fake_server = fake_servers.next().await.unwrap();
    let lsp_document_version = fake_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await
        .text_document
        .version;

    // Simulate editing the buffer after the language server computes some edits.
    buffer.update(cx, |buffer, cx| {
        buffer.edit(
            [(
                Point::new(0, 0)..Point::new(0, 0),
                "// above first function\n",
            )],
            cx,
        );
        buffer.edit(
            [(
                Point::new(2, 0)..Point::new(2, 0),
                "    // inside first function\n",
            )],
            cx,
        );
        buffer.edit(
            [(
                Point::new(6, 4)..Point::new(6, 4),
                "// inside second function ",
            )],
            cx,
        );

        assert_eq!(
            buffer.text(),
            "
                // above first function
                fn a() {
                    // inside first function
                    f1();
                }
                fn b() {
                    // inside second function f2();
                }
                fn c() {
                    f3();
                }
            "
            .unindent()
        );
    });

    let edits = project
        .update(cx, |project, cx| {
            project.edits_from_lsp(
                &buffer,
                vec![
                    // replace body of first function
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(3, 0)),
                        new_text: "
                            fn a() {
                                f10();
                            }
                            "
                        .unindent(),
                    },
                    // edit inside second function
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(4, 6), lsp::Position::new(4, 6)),
                        new_text: "00".into(),
                    },
                    // edit inside third function via two distinct edits
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(7, 5), lsp::Position::new(7, 5)),
                        new_text: "4000".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(7, 5), lsp::Position::new(7, 6)),
                        new_text: "".into(),
                    },
                ],
                Some(lsp_document_version),
                cx,
            )
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        for (range, new_text) in edits {
            buffer.edit([(range, new_text)], cx);
        }
        assert_eq!(
            buffer.text(),
            "
                // above first function
                fn a() {
                    // inside first function
                    f10();
                }
                fn b() {
                    // inside second function f200();
                }
                fn c() {
                    f4000();
                }
                "
            .unindent()
        );
    });
}

#[gpui::test]
async fn test_edits_from_lsp_with_edits_on_adjacent_lines(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let text = "
        use a::b;
        use a::c;

        fn f() {
            b();
            c();
        }
    "
    .unindent();

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "a.rs": text.clone(),
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();

    // Simulate the language server sending us a small edit in the form of a very large diff.
    // Rust-analyzer does this when performing a merge-imports code action.
    let edits = project
        .update(cx, |project, cx| {
            project.edits_from_lsp(
                &buffer,
                [
                    // Replace the first use statement without editing the semicolon.
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 8)),
                        new_text: "a::{b, c}".into(),
                    },
                    // Reinsert the remainder of the file between the semicolon and the final
                    // newline of the file.
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 9)),
                        new_text: "\n\n".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 9)),
                        new_text: "
                            fn f() {
                                b();
                                c();
                            }"
                        .unindent(),
                    },
                    // Delete everything after the first newline of the file.
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(7, 0)),
                        new_text: "".into(),
                    },
                ],
                None,
                cx,
            )
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        let edits = edits
            .into_iter()
            .map(|(range, text)| {
                (
                    range.start.to_point(&buffer)..range.end.to_point(&buffer),
                    text,
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            edits,
            [
                (Point::new(0, 4)..Point::new(0, 8), "a::{b, c}".into()),
                (Point::new(1, 0)..Point::new(2, 0), "".into())
            ]
        );

        for (range, new_text) in edits {
            buffer.edit([(range, new_text)], cx);
        }
        assert_eq!(
            buffer.text(),
            "
                use a::{b, c};
                
                fn f() {
                    b();
                    c();
                }
            "
            .unindent()
        );
    });
}

#[gpui::test]
async fn test_invalid_edits_from_lsp(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let text = "
        use a::b;
        use a::c;
        
        fn f() {
            b();
            c();
        }
    "
    .unindent();

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "a.rs": text.clone(),
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();

    // Simulate the language server sending us edits in a non-ordered fashion,
    // with ranges sometimes being inverted.
    let edits = project
        .update(cx, |project, cx| {
            project.edits_from_lsp(
                &buffer,
                [
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 9)),
                        new_text: "\n\n".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 8), lsp::Position::new(0, 4)),
                        new_text: "a::{b, c}".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(7, 0)),
                        new_text: "".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 9)),
                        new_text: "
                            fn f() {
                                b();
                                c();
                            }"
                        .unindent(),
                    },
                ],
                None,
                cx,
            )
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        let edits = edits
            .into_iter()
            .map(|(range, text)| {
                (
                    range.start.to_point(&buffer)..range.end.to_point(&buffer),
                    text,
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            edits,
            [
                (Point::new(0, 4)..Point::new(0, 8), "a::{b, c}".into()),
                (Point::new(1, 0)..Point::new(2, 0), "".into())
            ]
        );

        for (range, new_text) in edits {
            buffer.edit([(range, new_text)], cx);
        }
        assert_eq!(
            buffer.text(),
            "
                use a::{b, c};
                
                fn f() {
                    b();
                    c();
                }
            "
            .unindent()
        );
    });
}

fn chunks_with_diagnostics<T: ToOffset + ToPoint>(
    buffer: &Buffer,
    range: Range<T>,
) -> Vec<(String, Option<DiagnosticSeverity>)> {
    let mut chunks: Vec<(String, Option<DiagnosticSeverity>)> = Vec::new();
    for chunk in buffer.snapshot().chunks(range, true) {
        if chunks.last().map_or(false, |prev_chunk| {
            prev_chunk.1 == chunk.diagnostic_severity
        }) {
            chunks.last_mut().unwrap().0.push_str(chunk.text);
        } else {
            chunks.push((chunk.text.to_string(), chunk.diagnostic_severity));
        }
    }
    chunks
}

#[gpui::test]
async fn test_search_worktree_without_files(cx: &mut gpui::TestAppContext) {
    let dir = temp_tree(json!({
        "root": {
            "dir1": {},
            "dir2": {
                "dir3": {}
            }
        }
    }));

    let project = Project::test(Arc::new(RealFs), [dir.path()], cx).await;
    let cancel_flag = Default::default();
    let results = project
        .read_with(cx, |project, cx| {
            project.match_paths("dir", false, false, 10, &cancel_flag, cx)
        })
        .await;

    assert!(results.is_empty());
}

#[gpui::test(iterations = 10)]
async fn test_definition(cx: &mut gpui::TestAppContext) {
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_servers = language.set_fake_lsp_adapter(Default::default());

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "a.rs": "const fn a() { A }",
            "b.rs": "const y: i32 = crate::a()",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir/b.rs".as_ref()], cx).await;
    project.update(cx, |project, _| project.languages.add(Arc::new(language)));

    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/b.rs", cx))
        .await
        .unwrap();

    let fake_server = fake_servers.next().await.unwrap();
    fake_server.handle_request::<lsp::request::GotoDefinition, _, _>(|params, _| async move {
        let params = params.text_document_position_params;
        assert_eq!(
            params.text_document.uri.to_file_path().unwrap(),
            Path::new("/dir/b.rs"),
        );
        assert_eq!(params.position, lsp::Position::new(0, 22));

        Ok(Some(lsp::GotoDefinitionResponse::Scalar(
            lsp::Location::new(
                lsp::Url::from_file_path("/dir/a.rs").unwrap(),
                lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
            ),
        )))
    });

    let mut definitions = project
        .update(cx, |project, cx| project.definition(&buffer, 22, cx))
        .await
        .unwrap();

    // Assert no new language server started
    cx.foreground().run_until_parked();
    assert!(fake_servers.try_next().is_err());

    assert_eq!(definitions.len(), 1);
    let definition = definitions.pop().unwrap();
    cx.update(|cx| {
        let target_buffer = definition.target.buffer.read(cx);
        assert_eq!(
            target_buffer
                .file()
                .unwrap()
                .as_local()
                .unwrap()
                .abs_path(cx),
            Path::new("/dir/a.rs"),
        );
        assert_eq!(definition.target.range.to_offset(target_buffer), 9..10);
        assert_eq!(
            list_worktrees(&project, cx),
            [("/dir/b.rs".as_ref(), true), ("/dir/a.rs".as_ref(), false)]
        );

        drop(definition);
    });
    cx.read(|cx| {
        assert_eq!(list_worktrees(&project, cx), [("/dir/b.rs".as_ref(), true)]);
    });

    fn list_worktrees<'a>(
        project: &'a ModelHandle<Project>,
        cx: &'a AppContext,
    ) -> Vec<(&'a Path, bool)> {
        project
            .read(cx)
            .worktrees(cx)
            .map(|worktree| {
                let worktree = worktree.read(cx);
                (
                    worktree.as_local().unwrap().abs_path().as_ref(),
                    worktree.is_visible(),
                )
            })
            .collect::<Vec<_>>()
    }
}

#[gpui::test]
async fn test_completions_without_edit_ranges(cx: &mut gpui::TestAppContext) {
    let mut language = Language::new(
        LanguageConfig {
            name: "TypeScript".into(),
            path_suffixes: vec!["ts".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_typescript::language_typescript()),
    );
    let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "a.ts": "",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    project.update(cx, |project, _| project.languages.add(Arc::new(language)));
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/a.ts", cx))
        .await
        .unwrap();

    let fake_server = fake_language_servers.next().await.unwrap();

    let text = "let a = b.fqn";
    buffer.update(cx, |buffer, cx| buffer.set_text(text, cx));
    let completions = project.update(cx, |project, cx| {
        project.completions(&buffer, text.len(), cx)
    });

    fake_server
        .handle_request::<lsp::request::Completion, _, _>(|_, _| async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "fullyQualifiedName?".into(),
                    insert_text: Some("fullyQualifiedName".into()),
                    ..Default::default()
                },
            ])))
        })
        .next()
        .await;
    let completions = completions.await.unwrap();
    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
    assert_eq!(completions.len(), 1);
    assert_eq!(completions[0].new_text, "fullyQualifiedName");
    assert_eq!(
        completions[0].old_range.to_offset(&snapshot),
        text.len() - 3..text.len()
    );

    let text = "let a = \"atoms/cmp\"";
    buffer.update(cx, |buffer, cx| buffer.set_text(text, cx));
    let completions = project.update(cx, |project, cx| {
        project.completions(&buffer, text.len() - 1, cx)
    });

    fake_server
        .handle_request::<lsp::request::Completion, _, _>(|_, _| async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "component".into(),
                    ..Default::default()
                },
            ])))
        })
        .next()
        .await;
    let completions = completions.await.unwrap();
    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
    assert_eq!(completions.len(), 1);
    assert_eq!(completions[0].new_text, "component");
    assert_eq!(
        completions[0].old_range.to_offset(&snapshot),
        text.len() - 4..text.len() - 1
    );
}

#[gpui::test]
async fn test_completions_with_carriage_returns(cx: &mut gpui::TestAppContext) {
    let mut language = Language::new(
        LanguageConfig {
            name: "TypeScript".into(),
            path_suffixes: vec!["ts".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_typescript::language_typescript()),
    );
    let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "a.ts": "",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    project.update(cx, |project, _| project.languages.add(Arc::new(language)));
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/a.ts", cx))
        .await
        .unwrap();

    let fake_server = fake_language_servers.next().await.unwrap();

    let text = "let a = b.fqn";
    buffer.update(cx, |buffer, cx| buffer.set_text(text, cx));
    let completions = project.update(cx, |project, cx| {
        project.completions(&buffer, text.len(), cx)
    });

    fake_server
        .handle_request::<lsp::request::Completion, _, _>(|_, _| async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "fullyQualifiedName?".into(),
                    insert_text: Some("fully\rQualified\r\nName".into()),
                    ..Default::default()
                },
            ])))
        })
        .next()
        .await;
    let completions = completions.await.unwrap();
    assert_eq!(completions.len(), 1);
    assert_eq!(completions[0].new_text, "fully\nQualified\nName");
}

#[gpui::test(iterations = 10)]
async fn test_apply_code_actions_with_commands(cx: &mut gpui::TestAppContext) {
    let mut language = Language::new(
        LanguageConfig {
            name: "TypeScript".into(),
            path_suffixes: vec!["ts".to_string()],
            ..Default::default()
        },
        None,
    );
    let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "a.ts": "a",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    project.update(cx, |project, _| project.languages.add(Arc::new(language)));
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/a.ts", cx))
        .await
        .unwrap();

    let fake_server = fake_language_servers.next().await.unwrap();

    // Language server returns code actions that contain commands, and not edits.
    let actions = project.update(cx, |project, cx| project.code_actions(&buffer, 0..0, cx));
    fake_server
        .handle_request::<lsp::request::CodeActionRequest, _, _>(|_, _| async move {
            Ok(Some(vec![
                lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
                    title: "The code action".into(),
                    command: Some(lsp::Command {
                        title: "The command".into(),
                        command: "_the/command".into(),
                        arguments: Some(vec![json!("the-argument")]),
                    }),
                    ..Default::default()
                }),
                lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
                    title: "two".into(),
                    ..Default::default()
                }),
            ]))
        })
        .next()
        .await;

    let action = actions.await.unwrap()[0].clone();
    let apply = project.update(cx, |project, cx| {
        project.apply_code_action(buffer.clone(), action, true, cx)
    });

    // Resolving the code action does not populate its edits. In absence of
    // edits, we must execute the given command.
    fake_server.handle_request::<lsp::request::CodeActionResolveRequest, _, _>(
        |action, _| async move { Ok(action) },
    );

    // While executing the command, the language server sends the editor
    // a `workspaceEdit` request.
    fake_server
        .handle_request::<lsp::request::ExecuteCommand, _, _>({
            let fake = fake_server.clone();
            move |params, _| {
                assert_eq!(params.command, "_the/command");
                let fake = fake.clone();
                async move {
                    fake.server
                        .request::<lsp::request::ApplyWorkspaceEdit>(
                            lsp::ApplyWorkspaceEditParams {
                                label: None,
                                edit: lsp::WorkspaceEdit {
                                    changes: Some(
                                        [(
                                            lsp::Url::from_file_path("/dir/a.ts").unwrap(),
                                            vec![lsp::TextEdit {
                                                range: lsp::Range::new(
                                                    lsp::Position::new(0, 0),
                                                    lsp::Position::new(0, 0),
                                                ),
                                                new_text: "X".into(),
                                            }],
                                        )]
                                        .into_iter()
                                        .collect(),
                                    ),
                                    ..Default::default()
                                },
                            },
                        )
                        .await
                        .unwrap();
                    Ok(Some(json!(null)))
                }
            }
        })
        .next()
        .await;

    // Applying the code action returns a project transaction containing the edits
    // sent by the language server in its `workspaceEdit` request.
    let transaction = apply.await.unwrap();
    assert!(transaction.0.contains_key(&buffer));
    buffer.update(cx, |buffer, cx| {
        assert_eq!(buffer.text(), "Xa");
        buffer.undo(cx);
        assert_eq!(buffer.text(), "a");
    });
}

#[gpui::test]
async fn test_save_file(cx: &mut gpui::TestAppContext) {
    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "file1": "the old contents",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/file1", cx))
        .await
        .unwrap();
    buffer
        .update(cx, |buffer, cx| {
            assert_eq!(buffer.text(), "the old contents");
            buffer.edit([(0..0, "a line of text.\n".repeat(10 * 1024))], cx);
            buffer.save(cx)
        })
        .await
        .unwrap();

    let new_text = fs.load(Path::new("/dir/file1")).await.unwrap();
    assert_eq!(new_text, buffer.read_with(cx, |buffer, _| buffer.text()));
}

#[gpui::test]
async fn test_save_in_single_file_worktree(cx: &mut gpui::TestAppContext) {
    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "file1": "the old contents",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/dir/file1".as_ref()], cx).await;
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/file1", cx))
        .await
        .unwrap();
    buffer
        .update(cx, |buffer, cx| {
            buffer.edit([(0..0, "a line of text.\n".repeat(10 * 1024))], cx);
            buffer.save(cx)
        })
        .await
        .unwrap();

    let new_text = fs.load(Path::new("/dir/file1")).await.unwrap();
    assert_eq!(new_text, buffer.read_with(cx, |buffer, _| buffer.text()));
}

#[gpui::test]
async fn test_save_as(cx: &mut gpui::TestAppContext) {
    let fs = FakeFs::new(cx.background());
    fs.insert_tree("/dir", json!({})).await;

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
    let buffer = project.update(cx, |project, cx| {
        project.create_buffer("", None, cx).unwrap()
    });
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "abc")], cx);
        assert!(buffer.is_dirty());
        assert!(!buffer.has_conflict());
    });
    project
        .update(cx, |project, cx| {
            project.save_buffer_as(buffer.clone(), "/dir/file1".into(), cx)
        })
        .await
        .unwrap();
    assert_eq!(fs.load(Path::new("/dir/file1")).await.unwrap(), "abc");
    buffer.read_with(cx, |buffer, cx| {
        assert_eq!(buffer.file().unwrap().full_path(cx), Path::new("dir/file1"));
        assert!(!buffer.is_dirty());
        assert!(!buffer.has_conflict());
    });

    let opened_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/dir/file1", cx)
        })
        .await
        .unwrap();
    assert_eq!(opened_buffer, buffer);
}

#[gpui::test(retries = 5)]
async fn test_rescan_and_remote_updates(
    deterministic: Arc<Deterministic>,
    cx: &mut gpui::TestAppContext,
) {
    let dir = temp_tree(json!({
        "a": {
            "file1": "",
            "file2": "",
            "file3": "",
        },
        "b": {
            "c": {
                "file4": "",
                "file5": "",
            }
        }
    }));

    let project = Project::test(Arc::new(RealFs), [dir.path()], cx).await;
    let rpc = project.read_with(cx, |p, _| p.client.clone());

    let buffer_for_path = |path: &'static str, cx: &mut gpui::TestAppContext| {
        let buffer = project.update(cx, |p, cx| p.open_local_buffer(dir.path().join(path), cx));
        async move { buffer.await.unwrap() }
    };
    let id_for_path = |path: &'static str, cx: &gpui::TestAppContext| {
        project.read_with(cx, |project, cx| {
            let tree = project.worktrees(cx).next().unwrap();
            tree.read(cx)
                .entry_for_path(path)
                .expect(&format!("no entry for path {}", path))
                .id
        })
    };

    let buffer2 = buffer_for_path("a/file2", cx).await;
    let buffer3 = buffer_for_path("a/file3", cx).await;
    let buffer4 = buffer_for_path("b/c/file4", cx).await;
    let buffer5 = buffer_for_path("b/c/file5", cx).await;

    let file2_id = id_for_path("a/file2", &cx);
    let file3_id = id_for_path("a/file3", &cx);
    let file4_id = id_for_path("b/c/file4", &cx);

    // Create a remote copy of this worktree.
    let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
    let initial_snapshot = tree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot());
    let remote = cx.update(|cx| {
        Worktree::remote(
            1,
            1,
            proto::WorktreeMetadata {
                id: initial_snapshot.id().to_proto(),
                root_name: initial_snapshot.root_name().into(),
                visible: true,
            },
            rpc.clone(),
            cx,
        )
    });
    remote.update(cx, |remote, _| {
        let update = initial_snapshot.build_initial_update(1);
        remote.as_remote_mut().unwrap().update_from_remote(update);
    });
    deterministic.run_until_parked();

    cx.read(|cx| {
        assert!(!buffer2.read(cx).is_dirty());
        assert!(!buffer3.read(cx).is_dirty());
        assert!(!buffer4.read(cx).is_dirty());
        assert!(!buffer5.read(cx).is_dirty());
    });

    // Rename and delete files and directories.
    tree.flush_fs_events(&cx).await;
    std::fs::rename(dir.path().join("a/file3"), dir.path().join("b/c/file3")).unwrap();
    std::fs::remove_file(dir.path().join("b/c/file5")).unwrap();
    std::fs::rename(dir.path().join("b/c"), dir.path().join("d")).unwrap();
    std::fs::rename(dir.path().join("a/file2"), dir.path().join("a/file2.new")).unwrap();
    tree.flush_fs_events(&cx).await;

    let expected_paths = vec![
        "a",
        "a/file1",
        "a/file2.new",
        "b",
        "d",
        "d/file3",
        "d/file4",
    ];

    cx.read(|app| {
        assert_eq!(
            tree.read(app)
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            expected_paths
        );

        assert_eq!(id_for_path("a/file2.new", &cx), file2_id);
        assert_eq!(id_for_path("d/file3", &cx), file3_id);
        assert_eq!(id_for_path("d/file4", &cx), file4_id);

        assert_eq!(
            buffer2.read(app).file().unwrap().path().as_ref(),
            Path::new("a/file2.new")
        );
        assert_eq!(
            buffer3.read(app).file().unwrap().path().as_ref(),
            Path::new("d/file3")
        );
        assert_eq!(
            buffer4.read(app).file().unwrap().path().as_ref(),
            Path::new("d/file4")
        );
        assert_eq!(
            buffer5.read(app).file().unwrap().path().as_ref(),
            Path::new("b/c/file5")
        );

        assert!(!buffer2.read(app).file().unwrap().is_deleted());
        assert!(!buffer3.read(app).file().unwrap().is_deleted());
        assert!(!buffer4.read(app).file().unwrap().is_deleted());
        assert!(buffer5.read(app).file().unwrap().is_deleted());
    });

    // Update the remote worktree. Check that it becomes consistent with the
    // local worktree.
    remote.update(cx, |remote, cx| {
        let update = tree.read(cx).as_local().unwrap().snapshot().build_update(
            &initial_snapshot,
            1,
            1,
            true,
        );
        remote.as_remote_mut().unwrap().update_from_remote(update);
    });
    deterministic.run_until_parked();
    remote.read_with(cx, |remote, _| {
        assert_eq!(
            remote
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            expected_paths
        );
    });
}

#[gpui::test]
async fn test_buffer_deduping(cx: &mut gpui::TestAppContext) {
    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "a.txt": "a-contents",
            "b.txt": "b-contents",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    // Spawn multiple tasks to open paths, repeating some paths.
    let (buffer_a_1, buffer_b, buffer_a_2) = project.update(cx, |p, cx| {
        (
            p.open_local_buffer("/dir/a.txt", cx),
            p.open_local_buffer("/dir/b.txt", cx),
            p.open_local_buffer("/dir/a.txt", cx),
        )
    });

    let buffer_a_1 = buffer_a_1.await.unwrap();
    let buffer_a_2 = buffer_a_2.await.unwrap();
    let buffer_b = buffer_b.await.unwrap();
    assert_eq!(buffer_a_1.read_with(cx, |b, _| b.text()), "a-contents");
    assert_eq!(buffer_b.read_with(cx, |b, _| b.text()), "b-contents");

    // There is only one buffer per path.
    let buffer_a_id = buffer_a_1.id();
    assert_eq!(buffer_a_2.id(), buffer_a_id);

    // Open the same path again while it is still open.
    drop(buffer_a_1);
    let buffer_a_3 = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/a.txt", cx))
        .await
        .unwrap();

    // There's still only one buffer per path.
    assert_eq!(buffer_a_3.id(), buffer_a_id);
}

#[gpui::test]
async fn test_buffer_is_dirty(cx: &mut gpui::TestAppContext) {
    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "file1": "abc",
            "file2": "def",
            "file3": "ghi",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    let buffer1 = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/file1", cx))
        .await
        .unwrap();
    let events = Rc::new(RefCell::new(Vec::new()));

    // initially, the buffer isn't dirty.
    buffer1.update(cx, |buffer, cx| {
        cx.subscribe(&buffer1, {
            let events = events.clone();
            move |_, _, event, _| match event {
                BufferEvent::Operation(_) => {}
                _ => events.borrow_mut().push(event.clone()),
            }
        })
        .detach();

        assert!(!buffer.is_dirty());
        assert!(events.borrow().is_empty());

        buffer.edit([(1..2, "")], cx);
    });

    // after the first edit, the buffer is dirty, and emits a dirtied event.
    buffer1.update(cx, |buffer, cx| {
        assert!(buffer.text() == "ac");
        assert!(buffer.is_dirty());
        assert_eq!(
            *events.borrow(),
            &[language::Event::Edited, language::Event::DirtyChanged]
        );
        events.borrow_mut().clear();
        buffer.did_save(
            buffer.version(),
            buffer.as_rope().fingerprint(),
            buffer.file().unwrap().mtime(),
            None,
            cx,
        );
    });

    // after saving, the buffer is not dirty, and emits a saved event.
    buffer1.update(cx, |buffer, cx| {
        assert!(!buffer.is_dirty());
        assert_eq!(*events.borrow(), &[language::Event::Saved]);
        events.borrow_mut().clear();

        buffer.edit([(1..1, "B")], cx);
        buffer.edit([(2..2, "D")], cx);
    });

    // after editing again, the buffer is dirty, and emits another dirty event.
    buffer1.update(cx, |buffer, cx| {
        assert!(buffer.text() == "aBDc");
        assert!(buffer.is_dirty());
        assert_eq!(
            *events.borrow(),
            &[
                language::Event::Edited,
                language::Event::DirtyChanged,
                language::Event::Edited,
            ],
        );
        events.borrow_mut().clear();

        // After restoring the buffer to its previously-saved state,
        // the buffer is not considered dirty anymore.
        buffer.edit([(1..3, "")], cx);
        assert!(buffer.text() == "ac");
        assert!(!buffer.is_dirty());
    });

    assert_eq!(
        *events.borrow(),
        &[language::Event::Edited, language::Event::DirtyChanged]
    );

    // When a file is deleted, the buffer is considered dirty.
    let events = Rc::new(RefCell::new(Vec::new()));
    let buffer2 = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/file2", cx))
        .await
        .unwrap();
    buffer2.update(cx, |_, cx| {
        cx.subscribe(&buffer2, {
            let events = events.clone();
            move |_, _, event, _| events.borrow_mut().push(event.clone())
        })
        .detach();
    });

    fs.remove_file("/dir/file2".as_ref(), Default::default())
        .await
        .unwrap();
    cx.foreground().run_until_parked();
    assert_eq!(
        *events.borrow(),
        &[
            language::Event::DirtyChanged,
            language::Event::FileHandleChanged
        ]
    );

    // When a file is already dirty when deleted, we don't emit a Dirtied event.
    let events = Rc::new(RefCell::new(Vec::new()));
    let buffer3 = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/file3", cx))
        .await
        .unwrap();
    buffer3.update(cx, |_, cx| {
        cx.subscribe(&buffer3, {
            let events = events.clone();
            move |_, _, event, _| events.borrow_mut().push(event.clone())
        })
        .detach();
    });

    buffer3.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "x")], cx);
    });
    events.borrow_mut().clear();
    fs.remove_file("/dir/file3".as_ref(), Default::default())
        .await
        .unwrap();
    cx.foreground().run_until_parked();
    assert_eq!(*events.borrow(), &[language::Event::FileHandleChanged]);
    cx.read(|cx| assert!(buffer3.read(cx).is_dirty()));
}

#[gpui::test]
async fn test_buffer_file_changes_on_disk(cx: &mut gpui::TestAppContext) {
    let initial_contents = "aaa\nbbbbb\nc\n";
    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "the-file": initial_contents,
        }),
    )
    .await;
    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/the-file", cx))
        .await
        .unwrap();

    let anchors = (0..3)
        .map(|row| buffer.read_with(cx, |b, _| b.anchor_before(Point::new(row, 1))))
        .collect::<Vec<_>>();

    // Change the file on disk, adding two new lines of text, and removing
    // one line.
    buffer.read_with(cx, |buffer, _| {
        assert!(!buffer.is_dirty());
        assert!(!buffer.has_conflict());
    });
    let new_contents = "AAAA\naaa\nBB\nbbbbb\n";
    fs.save(
        "/dir/the-file".as_ref(),
        &new_contents.into(),
        LineEnding::Unix,
    )
    .await
    .unwrap();

    // Because the buffer was not modified, it is reloaded from disk. Its
    // contents are edited according to the diff between the old and new
    // file contents.
    cx.foreground().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), new_contents);
        assert!(!buffer.is_dirty());
        assert!(!buffer.has_conflict());

        let anchor_positions = anchors
            .iter()
            .map(|anchor| anchor.to_point(&*buffer))
            .collect::<Vec<_>>();
        assert_eq!(
            anchor_positions,
            [Point::new(1, 1), Point::new(3, 1), Point::new(4, 0)]
        );
    });

    // Modify the buffer
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, " ")], cx);
        assert!(buffer.is_dirty());
        assert!(!buffer.has_conflict());
    });

    // Change the file on disk again, adding blank lines to the beginning.
    fs.save(
        "/dir/the-file".as_ref(),
        &"\n\n\nAAAA\naaa\nBB\nbbbbb\n".into(),
        LineEnding::Unix,
    )
    .await
    .unwrap();

    // Because the buffer is modified, it doesn't reload from disk, but is
    // marked as having a conflict.
    cx.foreground().run_until_parked();
    buffer.read_with(cx, |buffer, _| {
        assert!(buffer.has_conflict());
    });
}

#[gpui::test]
async fn test_buffer_line_endings(cx: &mut gpui::TestAppContext) {
    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "file1": "a\nb\nc\n",
            "file2": "one\r\ntwo\r\nthree\r\n",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
    let buffer1 = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/file1", cx))
        .await
        .unwrap();
    let buffer2 = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/file2", cx))
        .await
        .unwrap();

    buffer1.read_with(cx, |buffer, _| {
        assert_eq!(buffer.text(), "a\nb\nc\n");
        assert_eq!(buffer.line_ending(), LineEnding::Unix);
    });
    buffer2.read_with(cx, |buffer, _| {
        assert_eq!(buffer.text(), "one\ntwo\nthree\n");
        assert_eq!(buffer.line_ending(), LineEnding::Windows);
    });

    // Change a file's line endings on disk from unix to windows. The buffer's
    // state updates correctly.
    fs.save(
        "/dir/file1".as_ref(),
        &"aaa\nb\nc\n".into(),
        LineEnding::Windows,
    )
    .await
    .unwrap();
    cx.foreground().run_until_parked();
    buffer1.read_with(cx, |buffer, _| {
        assert_eq!(buffer.text(), "aaa\nb\nc\n");
        assert_eq!(buffer.line_ending(), LineEnding::Windows);
    });

    // Save a file with windows line endings. The file is written correctly.
    buffer2
        .update(cx, |buffer, cx| {
            buffer.set_text("one\ntwo\nthree\nfour\n", cx);
            buffer.save(cx)
        })
        .await
        .unwrap();
    assert_eq!(
        fs.load("/dir/file2".as_ref()).await.unwrap(),
        "one\r\ntwo\r\nthree\r\nfour\r\n",
    );
}

#[gpui::test]
async fn test_grouped_diagnostics(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/the-dir",
        json!({
            "a.rs": "
                fn foo(mut v: Vec<usize>) {
                    for x in &v {
                        v.push(1);
                    }
                }
            "
            .unindent(),
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/the-dir".as_ref()], cx).await;
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/the-dir/a.rs", cx))
        .await
        .unwrap();

    let buffer_uri = Url::from_file_path("/the-dir/a.rs").unwrap();
    let message = lsp::PublishDiagnosticsParams {
        uri: buffer_uri.clone(),
        diagnostics: vec![
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9)),
                severity: Some(DiagnosticSeverity::WARNING),
                message: "error 1".to_string(),
                related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                    location: lsp::Location {
                        uri: buffer_uri.clone(),
                        range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9)),
                    },
                    message: "error 1 hint 1".to_string(),
                }]),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9)),
                severity: Some(DiagnosticSeverity::HINT),
                message: "error 1 hint 1".to_string(),
                related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                    location: lsp::Location {
                        uri: buffer_uri.clone(),
                        range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9)),
                    },
                    message: "original diagnostic".to_string(),
                }]),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(2, 8), lsp::Position::new(2, 17)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: "error 2".to_string(),
                related_information: Some(vec![
                    lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: buffer_uri.clone(),
                            range: lsp::Range::new(
                                lsp::Position::new(1, 13),
                                lsp::Position::new(1, 15),
                            ),
                        },
                        message: "error 2 hint 1".to_string(),
                    },
                    lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: buffer_uri.clone(),
                            range: lsp::Range::new(
                                lsp::Position::new(1, 13),
                                lsp::Position::new(1, 15),
                            ),
                        },
                        message: "error 2 hint 2".to_string(),
                    },
                ]),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(1, 13), lsp::Position::new(1, 15)),
                severity: Some(DiagnosticSeverity::HINT),
                message: "error 2 hint 1".to_string(),
                related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                    location: lsp::Location {
                        uri: buffer_uri.clone(),
                        range: lsp::Range::new(lsp::Position::new(2, 8), lsp::Position::new(2, 17)),
                    },
                    message: "original diagnostic".to_string(),
                }]),
                ..Default::default()
            },
            lsp::Diagnostic {
                range: lsp::Range::new(lsp::Position::new(1, 13), lsp::Position::new(1, 15)),
                severity: Some(DiagnosticSeverity::HINT),
                message: "error 2 hint 2".to_string(),
                related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                    location: lsp::Location {
                        uri: buffer_uri.clone(),
                        range: lsp::Range::new(lsp::Position::new(2, 8), lsp::Position::new(2, 17)),
                    },
                    message: "original diagnostic".to_string(),
                }]),
                ..Default::default()
            },
        ],
        version: None,
    };

    project
        .update(cx, |p, cx| p.update_diagnostics(0, message, &[], cx))
        .unwrap();
    let buffer = buffer.read_with(cx, |buffer, _| buffer.snapshot());

    assert_eq!(
        buffer
            .diagnostics_in_range::<_, Point>(0..buffer.len(), false)
            .collect::<Vec<_>>(),
        &[
            DiagnosticEntry {
                range: Point::new(1, 8)..Point::new(1, 9),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::WARNING,
                    message: "error 1".to_string(),
                    group_id: 0,
                    is_primary: true,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 8)..Point::new(1, 9),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 1 hint 1".to_string(),
                    group_id: 0,
                    is_primary: false,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 13)..Point::new(1, 15),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 2 hint 1".to_string(),
                    group_id: 1,
                    is_primary: false,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 13)..Point::new(1, 15),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 2 hint 2".to_string(),
                    group_id: 1,
                    is_primary: false,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(2, 8)..Point::new(2, 17),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::ERROR,
                    message: "error 2".to_string(),
                    group_id: 1,
                    is_primary: true,
                    ..Default::default()
                }
            }
        ]
    );

    assert_eq!(
        buffer.diagnostic_group::<Point>(0).collect::<Vec<_>>(),
        &[
            DiagnosticEntry {
                range: Point::new(1, 8)..Point::new(1, 9),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::WARNING,
                    message: "error 1".to_string(),
                    group_id: 0,
                    is_primary: true,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 8)..Point::new(1, 9),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 1 hint 1".to_string(),
                    group_id: 0,
                    is_primary: false,
                    ..Default::default()
                }
            },
        ]
    );
    assert_eq!(
        buffer.diagnostic_group::<Point>(1).collect::<Vec<_>>(),
        &[
            DiagnosticEntry {
                range: Point::new(1, 13)..Point::new(1, 15),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 2 hint 1".to_string(),
                    group_id: 1,
                    is_primary: false,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 13)..Point::new(1, 15),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 2 hint 2".to_string(),
                    group_id: 1,
                    is_primary: false,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(2, 8)..Point::new(2, 17),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::ERROR,
                    message: "error 2".to_string(),
                    group_id: 1,
                    is_primary: true,
                    ..Default::default()
                }
            }
        ]
    );
}

#[gpui::test]
async fn test_rename(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_servers = language.set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
        capabilities: lsp::ServerCapabilities {
            rename_provider: Some(lsp::OneOf::Right(lsp::RenameOptions {
                prepare_provider: Some(true),
                work_done_progress_options: Default::default(),
            })),
            ..Default::default()
        },
        ..Default::default()
    }));

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "one.rs": "const ONE: usize = 1;",
            "two.rs": "const TWO: usize = one::ONE + one::ONE;"
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
    project.update(cx, |project, _| project.languages.add(Arc::new(language)));
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/dir/one.rs", cx)
        })
        .await
        .unwrap();

    let fake_server = fake_servers.next().await.unwrap();

    let response = project.update(cx, |project, cx| {
        project.prepare_rename(buffer.clone(), 7, cx)
    });
    fake_server
        .handle_request::<lsp::request::PrepareRenameRequest, _, _>(|params, _| async move {
            assert_eq!(params.text_document.uri.as_str(), "file:///dir/one.rs");
            assert_eq!(params.position, lsp::Position::new(0, 7));
            Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
                lsp::Position::new(0, 6),
                lsp::Position::new(0, 9),
            ))))
        })
        .next()
        .await
        .unwrap();
    let range = response.await.unwrap().unwrap();
    let range = buffer.read_with(cx, |buffer, _| range.to_offset(buffer));
    assert_eq!(range, 6..9);

    let response = project.update(cx, |project, cx| {
        project.perform_rename(buffer.clone(), 7, "THREE".to_string(), true, cx)
    });
    fake_server
        .handle_request::<lsp::request::Rename, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri.as_str(),
                "file:///dir/one.rs"
            );
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(0, 7)
            );
            assert_eq!(params.new_name, "THREE");
            Ok(Some(lsp::WorkspaceEdit {
                changes: Some(
                    [
                        (
                            lsp::Url::from_file_path("/dir/one.rs").unwrap(),
                            vec![lsp::TextEdit::new(
                                lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                                "THREE".to_string(),
                            )],
                        ),
                        (
                            lsp::Url::from_file_path("/dir/two.rs").unwrap(),
                            vec![
                                lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(0, 24),
                                        lsp::Position::new(0, 27),
                                    ),
                                    "THREE".to_string(),
                                ),
                                lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(0, 35),
                                        lsp::Position::new(0, 38),
                                    ),
                                    "THREE".to_string(),
                                ),
                            ],
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ),
                ..Default::default()
            }))
        })
        .next()
        .await
        .unwrap();
    let mut transaction = response.await.unwrap().0;
    assert_eq!(transaction.len(), 2);
    assert_eq!(
        transaction
            .remove_entry(&buffer)
            .unwrap()
            .0
            .read_with(cx, |buffer, _| buffer.text()),
        "const THREE: usize = 1;"
    );
    assert_eq!(
        transaction
            .into_keys()
            .next()
            .unwrap()
            .read_with(cx, |buffer, _| buffer.text()),
        "const TWO: usize = one::THREE + one::THREE;"
    );
}

#[gpui::test]
async fn test_search(cx: &mut gpui::TestAppContext) {
    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/dir",
        json!({
            "one.rs": "const ONE: usize = 1;",
            "two.rs": "const TWO: usize = one::ONE + one::ONE;",
            "three.rs": "const THREE: usize = one::ONE + two::TWO;",
            "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
        }),
    )
    .await;
    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
    assert_eq!(
        search(&project, SearchQuery::text("TWO", false, true), cx)
            .await
            .unwrap(),
        HashMap::from_iter([
            ("two.rs".to_string(), vec![6..9]),
            ("three.rs".to_string(), vec![37..40])
        ])
    );

    let buffer_4 = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/dir/four.rs", cx)
        })
        .await
        .unwrap();
    buffer_4.update(cx, |buffer, cx| {
        let text = "two::TWO";
        buffer.edit([(20..28, text), (31..43, text)], cx);
    });

    assert_eq!(
        search(&project, SearchQuery::text("TWO", false, true), cx)
            .await
            .unwrap(),
        HashMap::from_iter([
            ("two.rs".to_string(), vec![6..9]),
            ("three.rs".to_string(), vec![37..40]),
            ("four.rs".to_string(), vec![25..28, 36..39])
        ])
    );

    async fn search(
        project: &ModelHandle<Project>,
        query: SearchQuery,
        cx: &mut gpui::TestAppContext,
    ) -> Result<HashMap<String, Vec<Range<usize>>>> {
        let results = project
            .update(cx, |project, cx| project.search(query, cx))
            .await?;

        Ok(results
            .into_iter()
            .map(|(buffer, ranges)| {
                buffer.read_with(cx, |buffer, _| {
                    let path = buffer.file().unwrap().path().to_string_lossy().to_string();
                    let ranges = ranges
                        .into_iter()
                        .map(|range| range.to_offset(buffer))
                        .collect::<Vec<_>>();
                    (path, ranges)
                })
            })
            .collect())
    }
}
