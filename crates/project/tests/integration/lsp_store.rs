use std::{
    borrow::Cow,
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

use collections::HashMap;
use fs::{FakeFs, Fs};
use futures::{FutureExt, StreamExt, future::LocalBoxFuture};
use gpui::{AsyncApp, Entity, Task, TestAppContext, UpdateGlobal as _};
use http_client::FakeHttpClient;
use language::{
    Buffer, CachedLspAdapter, CodeLabel, DiagnosticSourceKind, FakeLspAdapter, HighlightId,
    LanguageRegistry, LocalFile, LspAdapter, LspAdapterDelegate, LspInstaller, rust_lang,
};
use lsp::{
    LanguageServerBinary, LanguageServerBinaryOptions, LanguageServerId, LanguageServerName, Uri,
};
use parking_lot::Mutex;
use project::{
    DiagnosticSummary, Event, Project,
    binary_downloads::{self, BinaryDownload, BinaryDownloads},
    lsp_store::{
        log_store::{TestRpcLogHeaderState, TestRpcRequestTracker},
        *,
    },
};
use serde_json::json;
use settings::{ScanSymlinksSetting, SettingsStore};
use unindent::Unindent;
use util::{path, rel_path::rel_path};

use crate::init_test;

#[gpui::test]
async fn test_diagnostic_batches_skip_paths_without_worktrees(cx: &mut TestAppContext) {
    init_test(cx);

    for skipped_index in 0..=2 {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), json!({ "a.rs": "one", "b.rs": "two" }))
            .await;
        let project = Project::test(fs, [Path::new(path!("/dir"))], cx).await;
        let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
        let buffer_a = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/a.rs"), cx)
            })
            .await
            .unwrap();
        let worktree_id =
            buffer_a.read_with(cx, |buffer, cx| buffer.file().unwrap().worktree_id(cx));
        let server_id = LanguageServerId(0);

        for message in [Some("error"), None] {
            cx.run_until_parked();
            project.read_with(cx, |project, cx| {
                assert_eq!(
                    project.get_open_buffer(&(worktree_id, rel_path("b.rs")).into(), cx),
                    None
                );
            });
            let mut events = cx.events(&project);
            let mut paths = vec![path!("/dir/a.rs"), path!("/dir/b.rs")];
            paths.insert(skipped_index, path!("/outside.rs"));
            let updates = paths
                .into_iter()
                .map(|path| DocumentDiagnosticsUpdate {
                    diagnostics: lsp::PublishDiagnosticsParams {
                        uri: Uri::from_file_path(path).unwrap(),
                        version: None,
                        diagnostics: message
                            .into_iter()
                            .map(|message| lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 0),
                                    lsp::Position::new(0, 3),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::ERROR),
                                message: lsp::DiagnosticMessage::from(message),
                                ..lsp::Diagnostic::default()
                            })
                            .collect(),
                    },
                    result_id: None,
                    registration_id: None,
                    server_id,
                    disk_based_sources: Cow::Borrowed(&[]),
                })
                .collect();
            lsp_store.update(cx, |lsp_store, cx| {
                lsp_store
                    .merge_lsp_diagnostics(
                        DiagnosticSourceKind::Pushed,
                        updates,
                        |_, _, _| false,
                        cx,
                    )
                    .unwrap();
            });
            cx.run_until_parked();

            project.read_with(cx, |project, cx| {
                assert_eq!(
                    project.diagnostic_summary(false, cx),
                    DiagnosticSummary {
                        error_count: if message.is_some() { 2 } else { 0 },
                        warning_count: 0,
                    },
                    "skipped update at index {skipped_index}, message {message:?}"
                );
            });
            let diagnostic_events = std::iter::from_fn(|| events.next().now_or_never().flatten())
                .filter_map(|event| match event {
                    Event::DiagnosticsUpdated {
                        language_server_id,
                        paths,
                    } => Some((language_server_id, paths)),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(
                diagnostic_events,
                vec![(
                    server_id,
                    vec![
                        (worktree_id, rel_path("a.rs")).into(),
                        (worktree_id, rel_path("b.rs")).into(),
                    ],
                )],
                "skipped update at index {skipped_index}, message {message:?}"
            );

            let buffer_b = project
                .update(cx, |project, cx| {
                    project.open_local_buffer(path!("/dir/b.rs"), cx)
                })
                .await
                .unwrap();
            for buffer in [&buffer_a, &buffer_b] {
                buffer.read_with(cx, |buffer, _| {
                    assert_eq!(
                        buffer
                            .buffer_diagnostics(Some(server_id))
                            .iter()
                            .map(|entry| entry.diagnostic.message.to_string())
                            .collect::<Vec<_>>(),
                        message.into_iter().collect::<Vec<_>>()
                    );
                });
            }
        }
    }
}

#[gpui::test]
async fn test_removing_invisible_worktree_cleans_reused_lsp_bookkeeping(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;
    fs.insert_tree(
        path!("/the-registry"),
        json!({ "dep": { "src": { "dep.rs": "pub fn dep() {}" } } }),
    )
    .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp("Rust", FakeLspAdapter::default());

    let (_visible_buffer, _visible_handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();
    fake_servers.next().await.unwrap();
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
    let external_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_via_lsp(
                Uri::from_file_path(path!("/the-registry/dep/src/dep.rs")).unwrap(),
                server_id,
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();

    let invisible_worktree_id =
        external_buffer.read_with(cx, |buffer, cx| buffer.file().unwrap().worktree_id(cx));
    project.read_with(cx, |project, cx| {
        let worktree = project.worktree_for_id(invisible_worktree_id, cx).unwrap();
        assert!(!worktree.read(cx).is_visible());
        assert!(
            project
                .lsp_store()
                .read(cx)
                .has_language_server_seed_for_worktree(invisible_worktree_id)
        );
    });

    project.update(cx, |project, cx| {
        project.remove_worktree(invisible_worktree_id, cx);
    });
    cx.run_until_parked();

    project.read_with(cx, |project, cx| {
        let lsp_store = project.lsp_store();
        let lsp_store = lsp_store.read(cx);
        assert!(
            lsp_store
                .language_server_statuses()
                .any(|(status_server_id, _)| status_server_id == server_id)
        );
        assert!(!lsp_store.has_language_server_seed_for_worktree(invisible_worktree_id));
    });
}

#[gpui::test]
async fn test_open_buffer_via_lsp_case_variant_no_duplicate(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.set_case_sensitive(false);
    fs.insert_tree(
        path!("/root"),
        json!({ "src": { "main.rs": "fn main() {}" } }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp("Rust", FakeLspAdapter::default());

    project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/root/src/main.rs"), cx)
        })
        .await
        .unwrap();
    fake_servers.next().await.unwrap();
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

    project
        .update(cx, |project, cx| {
            project.open_local_buffer_via_lsp(
                Uri::from_file_path(path!("/root/SRC/main.rs")).unwrap(),
                server_id,
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();

    project.read_with(cx, |project, cx| {
        let worktree = project.worktrees(cx).next().unwrap();
        let entries: Vec<_> = worktree
            .read(cx)
            .snapshot()
            .entries(true, 0)
            .map(|entry| entry.path.as_unix_str().to_string())
            .collect();
        assert_eq!(entries, vec!["", "src", "src/main.rs"]);
    });
}

#[gpui::test]
async fn test_open_buffer_via_lsp_preserves_external_symlink_path(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/shared"),
        json!({ "pkg": { "def.rs": "pub fn def() {}" } }),
    )
    .await;
    fs.insert_tree(
        path!("/project"),
        json!({ "src": { "main.rs": "fn main() {}" } }),
    )
    .await;
    fs.create_symlink(
        path!("/project/pkg").as_ref(),
        PathBuf::from(path!("/shared/pkg")),
    )
    .await
    .unwrap();

    let (project, server_id) =
        project_with_rust_server(fs, path!("/project"), path!("/project/src/main.rs"), cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_via_lsp(
                Uri::from_file_path(path!("/project/pkg/def.rs")).unwrap(),
                server_id,
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();

    assert_eq!(
        buffer_paths(&buffer, cx),
        (
            "pkg/def.rs".to_string(),
            PathBuf::from(path!("/project/pkg/def.rs"))
        )
    );
    assert_eq!(
        worktree_roots(&project, cx),
        vec![PathBuf::from(path!("/project"))]
    );
}

#[gpui::test]
async fn test_open_buffer_via_lsp_maps_external_symlink_target(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/shared"),
        json!({ "pkg": { "def.rs": "pub fn def() {}", "other.rs": "pub fn other() {}" } }),
    )
    .await;
    fs.insert_tree(
        path!("/project"),
        json!({ "src": { "main.rs": "fn main() {}" } }),
    )
    .await;
    fs.create_symlink(
        path!("/project/pkg").as_ref(),
        PathBuf::from(path!("/shared/pkg")),
    )
    .await
    .unwrap();

    let (project, server_id) =
        project_with_rust_server(fs, path!("/project"), path!("/project/src/main.rs"), cx).await;

    project
        .update(cx, |project, cx| {
            project.open_local_buffer_via_lsp(
                Uri::from_file_path(path!("/project/pkg/def.rs")).unwrap(),
                server_id,
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_via_lsp(
                Uri::from_file_path(path!("/shared/pkg/other.rs")).unwrap(),
                server_id,
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();

    assert_eq!(
        buffer_paths(&buffer, cx),
        (
            "pkg/other.rs".to_string(),
            PathBuf::from(path!("/project/pkg/other.rs"))
        )
    );
    assert_eq!(
        worktree_roots(&project, cx),
        vec![PathBuf::from(path!("/project"))]
    );
}

#[gpui::test]
async fn test_open_buffer_via_lsp_maps_external_symlink_target_with_scan_symlinks_always(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    cx.executor().allow_parking();
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.scan_symlinks = Some(ScanSymlinksSetting::Always);
            });
        });
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/shared"),
        json!({ "pkg": { "other.rs": "pub fn other() {}" } }),
    )
    .await;
    fs.insert_tree(
        path!("/project"),
        json!({ "src": { "main.rs": "fn main() {}" } }),
    )
    .await;
    fs.create_symlink(
        path!("/project/pkg").as_ref(),
        PathBuf::from(path!("/shared/pkg")),
    )
    .await
    .unwrap();

    let (project, server_id) =
        project_with_rust_server(fs, path!("/project"), path!("/project/src/main.rs"), cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_via_lsp(
                Uri::from_file_path(path!("/shared/pkg/other.rs")).unwrap(),
                server_id,
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();

    assert_eq!(
        buffer_paths(&buffer, cx),
        (
            "pkg/other.rs".to_string(),
            PathBuf::from(path!("/project/pkg/other.rs"))
        )
    );
    assert_eq!(
        worktree_roots(&project, cx),
        vec![PathBuf::from(path!("/project"))]
    );
}

#[gpui::test]
async fn test_open_buffer_via_lsp_case_variant_in_unscanned_dir(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.set_case_sensitive(false);
    fs.insert_tree(
        path!("/root"),
        json!({
            ".gitignore": "ignored\n",
            "src": { "main.rs": "fn main() {}" },
            "ignored": { "lib.rs": "pub fn lib() {}" },
        }),
    )
    .await;

    let (project, server_id) =
        project_with_rust_server(fs, path!("/root"), path!("/root/src/main.rs"), cx).await;
    assert_eq!(
        worktree_entries(&project, cx),
        vec!["", ".gitignore", "ignored", "src", "src/main.rs"]
    );

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_via_lsp(
                Uri::from_file_path(path!("/root/IGNORED/LIB.rs")).unwrap(),
                server_id,
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();

    assert_eq!(
        buffer_paths(&buffer, cx),
        (
            "ignored/lib.rs".to_string(),
            PathBuf::from(path!("/root/ignored/lib.rs"))
        )
    );
    assert_eq!(
        worktree_roots(&project, cx),
        vec![PathBuf::from(path!("/root"))]
    );
    assert_eq!(
        worktree_entries(&project, cx),
        vec![
            "",
            ".gitignore",
            "ignored",
            "ignored/lib.rs",
            "src",
            "src/main.rs"
        ]
    );
}

#[test]
fn test_rpc_log_grouping_separates_timed_messages() {
    for (received, direction) in [(false, "Send"), (true, "Receive")] {
        let mut header_state = TestRpcLogHeaderState::new();

        assert_eq!(
            header_state.header_for_message(received, None),
            Some(format!("\n// {direction}:"))
        );
        assert_eq!(header_state.header_for_message(received, None), None);
        assert_eq!(
            header_state.header_for_message(received, Some(Duration::from_millis(53))),
            Some(format!("\n// {direction} (took 53.0ms):"))
        );
        assert_eq!(
            header_state.header_for_message(received, None),
            Some(format!("\n// {direction}:"))
        );
        assert_eq!(header_state.header_for_message(received, None), None);
    }
}

#[test]
fn test_rpc_request_tracker_distinguishes_request_directions() {
    let mut tracker = TestRpcRequestTracker::new();
    let started_at = Instant::now();

    assert_eq!(
        tracker.observe(
            false,
            r#"{"jsonrpc":"2.0","id":1,"method":"textDocument/hover"}"#,
            started_at,
        ),
        None
    );
    assert_eq!(
        tracker.observe(
            true,
            r#"{"jsonrpc":"2.0","id":1,"method":"workspace/configuration"}"#,
            started_at + Duration::from_millis(10),
        ),
        None
    );
    assert_eq!(
        tracker.observe(
            false,
            r#"{"jsonrpc":"2.0","id":1,"result":[]}"#,
            started_at + Duration::from_millis(30),
        ),
        Some(Duration::from_millis(20))
    );
    assert_eq!(
        tracker.observe(
            true,
            r#"{"jsonrpc":"2.0","id":1,"result":null}"#,
            started_at + Duration::from_millis(50),
        ),
        Some(Duration::from_millis(50))
    );
}

#[test]
fn test_rpc_request_tracker_decodes_ids_and_times_cancelled_requests() {
    let mut tracker = TestRpcRequestTracker::new();
    let started_at = Instant::now();

    tracker.observe(
        true,
        r#"{"jsonrpc":"2.0","id":"foo\u002fbar","method":"workspace/configuration"}"#,
        started_at,
    );
    assert_eq!(
        tracker.observe(
            false,
            r#"{"jsonrpc":"2.0","id":"foo/bar","result":[]}"#,
            started_at + Duration::from_millis(25),
        ),
        Some(Duration::from_millis(25))
    );

    tracker.observe(
        false,
        r#"{"jsonrpc":"2.0","id":7,"method":"textDocument/hover"}"#,
        started_at,
    );
    tracker.observe(
        false,
        r#"{"jsonrpc":"2.0","method":"$/cancelRequest","params":{"id":7}}"#,
        started_at + Duration::from_millis(1),
    );
    assert_eq!(tracker.pending_request_count(), 1);
    assert_eq!(
        tracker.observe(
            true,
            r#"{"jsonrpc":"2.0","id":7,"error":{"code":-32800,"message":"Request was cancelled"}}"#,
            started_at + Duration::from_millis(10),
        ),
        Some(Duration::from_millis(10))
    );
    assert_eq!(tracker.pending_request_count(), 0);
}

#[test]
fn test_rpc_request_tracker_bounds_unanswered_requests() {
    let mut tracker = TestRpcRequestTracker::new();
    let started_at = Instant::now();
    let max_pending_requests = TestRpcRequestTracker::max_pending_requests();

    for id in 0..=max_pending_requests {
        tracker.observe(
            false,
            &format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"textDocument/hover"}}"#),
            started_at + Duration::from_nanos(id as u64),
        );
    }

    assert_eq!(tracker.pending_request_count(), max_pending_requests);
    assert_eq!(
        tracker.observe(
            true,
            r#"{"jsonrpc":"2.0","id":0,"result":null}"#,
            started_at + Duration::from_secs(1),
        ),
        None
    );
    assert!(
        tracker
            .observe(
                true,
                r#"{"jsonrpc":"2.0","id":1,"result":null}"#,
                started_at + Duration::from_secs(1),
            )
            .is_some()
    );
}

#[test]
fn test_rpc_log_duration_proto_roundtrip() {
    let log_type = LanguageServerLogType::Rpc {
        received: true,
        elapsed: Some(Duration::from_micros(1234)),
    };

    assert_eq!(
        LanguageServerLogType::from_proto(log_type.to_proto()),
        log_type
    );
}

#[test]
fn test_glob_literal_prefix() {
    assert_eq!(glob_literal_prefix(Path::new("**/*.js")), Path::new(""));
    assert_eq!(
        glob_literal_prefix(Path::new("node_modules/**/*.js")),
        Path::new("node_modules")
    );
    assert_eq!(
        glob_literal_prefix(Path::new("foo/{bar,baz}.js")),
        Path::new("foo")
    );
    assert_eq!(
        glob_literal_prefix(Path::new("foo/bar/baz.js")),
        Path::new("foo/bar/baz.js")
    );

    #[cfg(target_os = "windows")]
    {
        assert_eq!(glob_literal_prefix(Path::new("**\\*.js")), Path::new(""));
        assert_eq!(
            glob_literal_prefix(Path::new("node_modules\\**/*.js")),
            Path::new("node_modules")
        );
        assert_eq!(
            glob_literal_prefix(Path::new("foo/{bar,baz}.js")),
            Path::new("foo")
        );
        assert_eq!(
            glob_literal_prefix(Path::new("foo\\bar\\baz.js")),
            Path::new("foo/bar/baz.js")
        );
    }
}

#[test]
fn test_multi_len_chars_normalization() {
    let mut label = CodeLabel::new(
        "myElˇ (parameter) myElˇ: {\n    foo: string;\n}".to_string(),
        0..6,
        vec![(0..6, HighlightId::new(1))],
    );
    ensure_uniform_list_compatible_label(&mut label);
    assert_eq!(
        label,
        CodeLabel::new(
            "myElˇ (parameter) myElˇ: { foo: string; }".to_string(),
            0..6,
            vec![(0..6, HighlightId::new(1))],
        )
    );
}

#[test]
fn test_completion_label_snippet_normalization() {
    for line_ending in ["\n", "\r\n", "\r"] {
        let text = "
            #[cfg(test)]
            mod tests {
                use super::*;

                #[test]
                fn test_name() {

                }
            }"
        .unindent()
        .replace('\n', line_ending);
        let name_start = text.find("test_name").expect("snippet has a test name");
        let text_len = text.len();
        let mut label = CodeLabel::new(
            text,
            0..text_len,
            vec![
                (0..12, HighlightId::new(1)),
                (name_start..name_start + 9, HighlightId::TABSTOP_REPLACE_ID),
            ],
        );

        ensure_uniform_list_compatible_label(&mut label);

        assert_eq!(
            label,
            CodeLabel::new(
                "#[cfg(test)] mod tests { use super::*; #[test] fn test_name() { } }".to_string(),
                0..67,
                vec![
                    (0..12, HighlightId::new(1)),
                    (50..59, HighlightId::TABSTOP_REPLACE_ID),
                ],
            ),
            "line ending: {line_ending:?}",
        );
    }
}

#[test]
fn test_completion_label_unicode_normalization() {
    for line_ending in ["\n", "\r\n", "\r"] {
        let text = "
            héllo {
                🦀value: 世界,
            }"
        .unindent()
        .replace('\n', line_ending);
        let value_start = text.find("🦀value").expect("label has a value");
        let type_start = text.find("世界").expect("label has a type");
        let text_len = text.len();
        let mut label = CodeLabel::new(
            text,
            value_start..value_start + 9,
            vec![
                (0..text_len, HighlightId::new(0)),
                (0..6, HighlightId::new(1)),
                (
                    value_start..value_start + 9,
                    HighlightId::TABSTOP_REPLACE_ID,
                ),
                (type_start..type_start + 6, HighlightId::new(2)),
            ],
        );

        ensure_uniform_list_compatible_label(&mut label);

        assert_eq!(
            label,
            CodeLabel::new(
                "héllo { 🦀value: 世界, }".to_string(),
                9..18,
                vec![
                    (0..29, HighlightId::new(0)),
                    (0..6, HighlightId::new(1)),
                    (9..18, HighlightId::TABSTOP_REPLACE_ID),
                    (20..26, HighlightId::new(2)),
                ],
            ),
            "line ending: {line_ending:?}",
        );
    }
}

#[test]
fn test_completion_label_whitespace_normalization() {
    for line_ending in ["\n", "\r\n", "\r", "\n\r", "\r\r\n"] {
        for before in ["", " ", " \t "] {
            for after in ["", " ", " \t "] {
                let mut label = CodeLabel::plain(format!("{before}{line_ending}{after}"), None);
                ensure_uniform_list_compatible_label(&mut label);
                assert_eq!(label, CodeLabel::plain(" ".to_string(), None));
            }
        }
    }

    for text in ["", " ", " \t ", "héllo  世界", "héllo\t世界"] {
        let mut label = CodeLabel::plain(text.to_string(), None);
        let expected = label.clone();
        ensure_uniform_list_compatible_label(&mut label);
        assert_eq!(label, expected);
    }
}

#[test]
fn test_trailing_newline_in_completion_documentation() {
    let doc =
        lsp::Documentation::String("Inappropriate argument value (of correct type).\n".to_string());
    let completion_doc: CompletionDocumentation = doc.into();
    assert!(
        matches!(completion_doc, CompletionDocumentation::SingleLine(s) if s == "Inappropriate argument value (of correct type).")
    );

    let doc = lsp::Documentation::String("  some value  \n".to_string());
    let completion_doc: CompletionDocumentation = doc.into();
    assert!(matches!(
        completion_doc,
        CompletionDocumentation::SingleLine(s) if s == "some value"
    ));
}

#[gpui::test]
async fn test_user_initialization_options_override_adapter_arrays(cx: &mut TestAppContext) {
    init_test(cx);

    let user_settings = serde_json::json!({
        "lsp": {
            "the-fake-language-server": {
                "initialization_options": {
                    "preview": {
                        "background": {
                            "enabled": true,
                            "args": ["--data-plane-host=127.0.0.1:23635", "--invert-colors=never"],
                        },
                    },
                    "plugins": ["user-plugin"],
                    "userOnly": ["user"],
                },
            },
        },
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/the-root"),
        json!({
            ".zed": {
                "settings.json": user_settings.to_string(),
            },
            "main.rs": "fn main() {}",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());

    let sent_initialization_options = Arc::new(Mutex::new(None));
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-fake-language-server",
            initialization_options: Some(json!({
                "preview": {
                    "background": {
                        "args": ["--data-plane-host=127.0.0.1:23635", "--invert-colors=never"],
                        "partialRendering": true,
                    },
                },
                "plugins": ["default-plugin", "user-plugin"],
                "adapterOnly": [1, 2],
            })),
            initializer: Some(Box::new({
                let sent_initialization_options = sent_initialization_options.clone();
                move |fake_server| {
                    let sent_initialization_options = sent_initialization_options.clone();
                    fake_server.set_request_handler::<lsp::request::Initialize, _, _>(
                        move |params, _| {
                            *sent_initialization_options.lock() = params.initialization_options;
                            async move { Ok(lsp::InitializeResult::default()) }
                        },
                    );
                }
            })),
            ..FakeLspAdapter::default()
        },
    );
    cx.run_until_parked();

    project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();
    fake_servers.next().await.unwrap();
    cx.run_until_parked();

    assert_eq!(
        sent_initialization_options.lock().take(),
        Some(json!({
            "preview": {
                "background": {
                    "enabled": true,
                    "args": ["--data-plane-host=127.0.0.1:23635", "--invert-colors=never"],
                    "partialRendering": true,
                },
            },
            "plugins": ["user-plugin"],
            "adapterOnly": [1, 2],
            "userOnly": ["user"],
        })),
    );
}

#[gpui::test]
async fn test_other_adapters_lsp_configuration_contributions_are_unioned(cx: &mut TestAppContext) {
    init_test(cx);

    let user_settings = serde_json::json!({
        "lsp": {
            "the-fake-language-server": {
                "initialization_options": {
                    "languages": ["user-lang"],
                    "userOnly": true,
                },
            },
        },
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/the-root"),
        json!({
            ".zed": {
                "settings.json": user_settings.to_string(),
            },
            "main.rs": "fn main() {}",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());

    let main_server_name = LanguageServerName("the-fake-language-server".into());
    for (language, server_name, plugin, lang, memory) in [
        ("Vue", "vue-language-server", "vue-plugin", "vue", 4096),
        (
            "Astro",
            "astro-language-server",
            "astro-plugin",
            "astro",
            2048,
        ),
    ] {
        let contribution = json!({
            "tsserver": {
                "globalPlugins": ["shared-plugin", plugin],
                "maxMemory": memory,
            },
            "languages": [lang],
        });
        language_registry.register_fake_lsp_adapter(
            language,
            FakeLspAdapter {
                name: server_name,
                additional_initialization_options: HashMap::from_iter([(
                    main_server_name.clone(),
                    contribution.clone(),
                )]),
                additional_workspace_configuration: HashMap::from_iter([(
                    main_server_name.clone(),
                    contribution,
                )]),
                ..FakeLspAdapter::default()
            },
        );
    }

    let sent_initialization_options = Arc::new(Mutex::new(None));
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-fake-language-server",
            initialization_options: Some(json!({
                "tsserver": {
                    "globalPlugins": ["default-plugin"],
                },
                "languages": ["default-lang"],
            })),
            initializer: Some(Box::new({
                let sent_initialization_options = sent_initialization_options.clone();
                move |fake_server| {
                    let sent_initialization_options = sent_initialization_options.clone();
                    fake_server.set_request_handler::<lsp::request::Initialize, _, _>(
                        move |params, _| {
                            *sent_initialization_options.lock() = params.initialization_options;
                            async move { Ok(lsp::InitializeResult::default()) }
                        },
                    );
                }
            })),
            ..FakeLspAdapter::default()
        },
    );
    cx.run_until_parked();

    project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();
    let mut fake_server = fake_servers.next().await.unwrap();
    let workspace_configuration = fake_server
        .receive_notification::<lsp::notification::DidChangeConfiguration>()
        .await
        .settings;
    cx.run_until_parked();

    assert_eq!(
        sent_initialization_options.lock().take(),
        Some(json!({
            "tsserver": {
                "globalPlugins": ["default-plugin", "shared-plugin", "astro-plugin", "vue-plugin"],
                "maxMemory": 4096,
            },
            "languages": ["user-lang"],
            "userOnly": true,
        })),
    );
    assert_eq!(
        workspace_configuration,
        json!({
            "tsserver": {
                "globalPlugins": ["shared-plugin", "astro-plugin", "vue-plugin"],
                "maxMemory": 4096,
            },
            "languages": ["astro", "vue"],
        }),
    );
}

#[gpui::test]
async fn test_initialization_options_contributions_without_own_options(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());

    let contribution = json!({
        "tsserver": {
            "globalPlugins": ["vue-plugin"],
        },
    });
    language_registry.register_fake_lsp_adapter(
        "Vue",
        FakeLspAdapter {
            name: "vue-language-server",
            additional_initialization_options: HashMap::from_iter([(
                LanguageServerName("the-fake-language-server".into()),
                contribution.clone(),
            )]),
            ..FakeLspAdapter::default()
        },
    );

    let sent_initialization_options = Arc::new(Mutex::new(None));
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-fake-language-server",
            initialization_options: None,
            initializer: Some(Box::new({
                let sent_initialization_options = sent_initialization_options.clone();
                move |fake_server| {
                    let sent_initialization_options = sent_initialization_options.clone();
                    fake_server.set_request_handler::<lsp::request::Initialize, _, _>(
                        move |params, _| {
                            *sent_initialization_options.lock() =
                                Some(params.initialization_options);
                            async move { Ok(lsp::InitializeResult::default()) }
                        },
                    );
                }
            })),
            ..FakeLspAdapter::default()
        },
    );
    cx.run_until_parked();

    project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();
    fake_servers.next().await.unwrap();
    cx.run_until_parked();

    assert_eq!(
        sent_initialization_options.lock().take(),
        Some(Some(contribution)),
    );
}

#[gpui::test]
async fn test_language_server_download_waits_for_consent(cx: &mut TestAppContext) {
    let (project, delegate, _download_dir) = download_consent_test_setup(cx).await;

    let (cached, cached_fetches) = DownloadableLspAdapter::new(Some("/cached/server"));
    let download = BinaryDownload::new(cached.name().0);
    let (existing_binary, download_task) =
        language_server_command(&cached, delegate.clone(), cx).await;
    assert_eq!(
        existing_binary.unwrap().path,
        PathBuf::from("/cached/server")
    );
    assert!(download_task.is_none());
    assert_eq!(cached_fetches.load(Ordering::SeqCst), 0);
    assert_eq!(
        cx.update(|cx| binary_downloads::pending_downloads(project.read(cx), cx)),
        vec![download.clone()]
    );

    let (missing, missing_fetches) = DownloadableLspAdapter::new(None);
    let (existing_binary, download_task) = language_server_command(&missing, delegate, cx).await;
    assert!(existing_binary.is_err());
    let download_task = cx.spawn(async move |_| download_task.unwrap().await);
    cx.run_until_parked();
    assert_eq!(missing_fetches.load(Ordering::SeqCst), 0);
    assert_eq!(
        cx.update(|cx| binary_downloads::pending_downloads(project.read(cx), cx)),
        vec![download.clone()]
    );
    let persisted = Rc::new(RefCell::new(None));

    cx.update(|cx| {
        BinaryDownloads::try_get_global(cx)
            .unwrap()
            .update(cx, |binary_downloads, cx| {
                binary_downloads.allow_download(&project, download.clone(), cx);
                binary_downloads.schedule_serialization(cx, {
                    let persisted = persisted.clone();
                    move |allowed_downloads, _| {
                        *persisted.borrow_mut() = Some(allowed_downloads);
                        Task::ready(())
                    }
                });
            });
    });
    cx.run_until_parked();
    assert_eq!(
        download_task
            .now_or_never()
            .map(|binary| binary.map(|binary| binary.path).ok()),
        Some(Some(PathBuf::from("/downloaded/server")))
    );
    assert_eq!(missing_fetches.load(Ordering::SeqCst), 1);
    assert_eq!(
        cx.update(|cx| binary_downloads::pending_downloads(project.read(cx), cx)),
        Vec::new()
    );
    assert_eq!(
        persisted.take(),
        Some(HashMap::from_iter([(
            None,
            collections::HashSet::from_iter([download])
        )]))
    );
}

#[gpui::test]
async fn test_denied_language_server_download_fails(cx: &mut TestAppContext) {
    let (project, delegate, _download_dir) = download_consent_test_setup(cx).await;
    let (adapter, fetches) = DownloadableLspAdapter::new(None);
    let download = BinaryDownload::new(adapter.name().0);

    let (existing_binary, download_task) =
        language_server_command(&adapter, delegate.clone(), cx).await;
    assert!(existing_binary.is_err());
    let download_task = cx.spawn(async move |_| download_task.unwrap().await);
    cx.run_until_parked();
    assert_eq!(
        cx.update(|cx| binary_downloads::pending_downloads(project.read(cx), cx)),
        vec![download.clone()]
    );

    cx.update(|cx| {
        BinaryDownloads::try_get_global(cx)
            .unwrap()
            .update(cx, |binary_downloads, cx| {
                binary_downloads.deny_download(&project, download.clone(), cx);
            });
    });
    cx.run_until_parked();
    assert_eq!(
        download_task
            .now_or_never()
            .map(|binary| binary.map(|binary| binary.path).ok()),
        Some(None)
    );
    assert_eq!(
        cx.update(|cx| binary_downloads::pending_downloads(project.read(cx), cx)),
        Vec::new()
    );

    let (_, retry_task) = language_server_command(&adapter, delegate, cx).await;
    assert_eq!(
        retry_task
            .unwrap()
            .now_or_never()
            .map(|binary| binary.map(|binary| binary.path).ok()),
        Some(None)
    );
    assert_eq!(fetches.load(Ordering::SeqCst), 0);
    assert_eq!(
        cx.update(|cx| binary_downloads::pending_downloads(project.read(cx), cx)),
        Vec::new()
    );
}

async fn project_with_rust_server(
    fs: Arc<FakeFs>,
    root: &str,
    first_file: &str,
    cx: &mut TestAppContext,
) -> (Entity<Project>, LanguageServerId) {
    let project = Project::test(fs, [root.as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp("Rust", FakeLspAdapter::default());

    project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(first_file, cx)
        })
        .await
        .unwrap();
    fake_servers.next().await.unwrap();
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
    (project, server_id)
}

fn buffer_paths(buffer: &Entity<Buffer>, cx: &TestAppContext) -> (String, PathBuf) {
    buffer.read_with(cx, |buffer, cx| {
        let file = File::from_dyn(buffer.file()).unwrap();
        (file.path.as_unix_str().to_string(), file.abs_path(cx))
    })
}

fn worktree_roots(project: &Entity<Project>, cx: &TestAppContext) -> Vec<PathBuf> {
    project.read_with(cx, |project, cx| {
        project
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
            .collect()
    })
}

fn worktree_entries(project: &Entity<Project>, cx: &TestAppContext) -> Vec<String> {
    project.read_with(cx, |project, cx| {
        let worktree = project.worktrees(cx).next().unwrap();
        worktree
            .read(cx)
            .snapshot()
            .entries(true, 0)
            .map(|entry| entry.path.as_unix_str().to_string())
            .collect()
    })
}

async fn language_server_command(
    adapter: &Arc<CachedLspAdapter>,
    delegate: Arc<dyn LspAdapterDelegate>,
    cx: &mut TestAppContext,
) -> (
    anyhow::Result<LanguageServerBinary>,
    Option<LocalBoxFuture<'static, anyhow::Result<LanguageServerBinary>>>,
) {
    let download_gate = cx.update(|cx| binary_downloads::download_gate(cx));
    let mut async_cx = cx.to_async();
    adapter
        .clone()
        .get_language_server_command(
            delegate,
            None,
            LanguageServerBinaryOptions {
                allow_path_lookup: true,
                allow_binary_download: true,
                pre_release: false,
            },
            download_gate,
            &mut async_cx,
        )
        .await
        .await
}

struct DownloadableLspAdapter {
    cached_binary: Option<&'static str>,
    fetches: Arc<AtomicUsize>,
}

impl DownloadableLspAdapter {
    fn new(cached_binary: Option<&'static str>) -> (Arc<CachedLspAdapter>, Arc<AtomicUsize>) {
        let fetches = Arc::new(AtomicUsize::new(0));
        let adapter = CachedLspAdapter::new(Arc::new(Self {
            cached_binary,
            fetches: fetches.clone(),
        }));
        (adapter, fetches)
    }
}

impl LspInstaller for DownloadableLspAdapter {
    type BinaryVersion = ();

    async fn fetch_latest_server_version(
        &self,
        _: &Arc<dyn LspAdapterDelegate>,
        _: bool,
        _: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn fetch_server_binary(
        &self,
        _: (),
        _: PathBuf,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> impl Send + Future<Output = anyhow::Result<LanguageServerBinary>> + use<> {
        self.fetches.fetch_add(1, Ordering::SeqCst);
        async { Ok(test_binary("/downloaded/server")) }
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        self.cached_binary.map(test_binary)
    }
}

impl LspAdapter for DownloadableLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName::new_static("downloadable-language-server")
    }
}

fn test_binary(path: &str) -> LanguageServerBinary {
    LanguageServerBinary {
        path: PathBuf::from(path),
        arguments: Vec::new(),
        env: None,
    }
}

async fn download_consent_test_setup(
    cx: &mut TestAppContext,
) -> (
    Entity<Project>,
    Arc<dyn LspAdapterDelegate>,
    tempfile::TempDir,
) {
    init_test(cx);
    cx.executor().allow_parking();
    cx.update(|cx| {
        binary_downloads::init(HashMap::default(), cx);
    });
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "" }))
        .await;
    let project = Project::test(fs.clone(), [path!("/the-root").as_ref()], cx).await;
    let download_dir = tempfile::tempdir().unwrap();
    let delegate = project.update(cx, |project, cx| {
        let mut languages = LanguageRegistry::test(cx.background_executor().clone());
        languages.set_language_server_download_dir(download_dir.path());
        let worktree = project.worktrees(cx).next().unwrap();
        LocalLspAdapterDelegate::new(
            Arc::new(languages),
            project.environment(),
            project.lsp_store().downgrade(),
            &worktree,
            FakeHttpClient::with_404_response(),
            fs.clone(),
            cx,
        ) as Arc<dyn LspAdapterDelegate>
    });
    (project, delegate, download_dir)
}
