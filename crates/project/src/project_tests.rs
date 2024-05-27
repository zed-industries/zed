use crate::{Event, *};
use fs::FakeFs;
use futures::{future, StreamExt};
use gpui::{AppContext, UpdateGlobal};
use language::{
    language_settings::{AllLanguageSettings, LanguageSettingsContent},
    tree_sitter_rust, tree_sitter_typescript, Diagnostic, FakeLspAdapter, LanguageConfig,
    LanguageMatcher, LineEnding, OffsetRangeExt, Point, ToPoint,
};
use lsp::Url;
use parking_lot::Mutex;
use pretty_assertions::assert_eq;
use serde_json::json;
#[cfg(not(windows))]
use std::os;
use std::task::Poll;
use task::{ResolvedTask, TaskContext, TaskTemplate, TaskTemplates};
use unindent::Unindent as _;
use util::{assert_set_eq, paths::PathMatcher, test::temp_tree};
use worktree::WorktreeModelHandle as _;

#[gpui::test]
async fn test_block_via_channel(cx: &mut gpui::TestAppContext) {
    cx.executor().allow_parking();

    let (tx, mut rx) = futures::channel::mpsc::unbounded();
    let _thread = std::thread::spawn(move || {
        std::fs::metadata("/Users").unwrap();
        std::thread::sleep(Duration::from_millis(1000));
        tx.unbounded_send(1).unwrap();
    });
    rx.next().await.unwrap();
}

#[gpui::test]
async fn test_block_via_smol(cx: &mut gpui::TestAppContext) {
    cx.executor().allow_parking();

    let io_task = smol::unblock(move || {
        println!("sleeping on thread {:?}", std::thread::current().id());
        std::thread::sleep(Duration::from_millis(10));
        1
    });

    let task = cx.foreground_executor().spawn(async move {
        io_task.await;
    });

    task.await;
}

#[cfg(not(windows))]
#[gpui::test]
async fn test_symlinks(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

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
    os::unix::fs::symlink(&dir.path().join("root"), &root_link_path).unwrap();
    os::unix::fs::symlink(
        &dir.path().join("root/fennel"),
        &dir.path().join("root/finnochio"),
    )
    .unwrap();

    let project = Project::test(Arc::new(RealFs::default()), [root_link_path.as_ref()], cx).await;

    project.update(cx, |project, cx| {
        let tree = project.worktrees().next().unwrap().read(cx);
        assert_eq!(tree.file_count(), 5);
        assert_eq!(
            tree.inode_for_path("fennel/grape"),
            tree.inode_for_path("finnochio/grape")
        );
    });
}

#[gpui::test]
async fn test_managing_project_specific_settings(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/the-root",
        json!({
            ".zed": {
                "settings.json": r#"{ "tab_size": 8 }"#,
                "tasks.json": r#"[{
                    "label": "cargo check",
                    "command": "cargo",
                    "args": ["check", "--all"]
                },]"#,
            },
            "a": {
                "a.rs": "fn a() {\n    A\n}"
            },
            "b": {
                ".zed": {
                    "settings.json": r#"{ "tab_size": 2 }"#,
                    "tasks.json": r#"[{
                        "label": "cargo check",
                        "command": "cargo",
                        "args": ["check"]
                    },]"#,
                },
                "b.rs": "fn b() {\n  B\n}"
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/the-root".as_ref()], cx).await;
    let worktree = project.update(cx, |project, _| project.worktrees().next().unwrap());
    let task_context = TaskContext::default();

    cx.executor().run_until_parked();
    let worktree_id = cx.update(|cx| {
        project.update(cx, |project, cx| {
            project.worktrees().next().unwrap().read(cx).id()
        })
    });
    let global_task_source_kind = TaskSourceKind::Worktree {
        id: worktree_id,
        abs_path: PathBuf::from("/the-root/.zed/tasks.json"),
        id_base: "local_tasks_for_worktree".into(),
    };

    let all_tasks = cx
        .update(|cx| {
            let tree = worktree.read(cx);

            let settings_a = language_settings(
                None,
                Some(
                    &(File::for_entry(
                        tree.entry_for_path("a/a.rs").unwrap().clone(),
                        worktree.clone(),
                    ) as _),
                ),
                cx,
            );
            let settings_b = language_settings(
                None,
                Some(
                    &(File::for_entry(
                        tree.entry_for_path("b/b.rs").unwrap().clone(),
                        worktree.clone(),
                    ) as _),
                ),
                cx,
            );

            assert_eq!(settings_a.tab_size.get(), 8);
            assert_eq!(settings_b.tab_size.get(), 2);

            get_all_tasks(&project, Some(worktree_id), &task_context, cx)
        })
        .await
        .into_iter()
        .map(|(source_kind, task)| {
            let resolved = task.resolved.unwrap();
            (
                source_kind,
                task.resolved_label,
                resolved.args,
                resolved.env,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        all_tasks,
        vec![
            (
                global_task_source_kind.clone(),
                "cargo check".to_string(),
                vec!["check".to_string(), "--all".to_string()],
                HashMap::default(),
            ),
            (
                TaskSourceKind::Worktree {
                    id: worktree_id,
                    abs_path: PathBuf::from("/the-root/b/.zed/tasks.json"),
                    id_base: "local_tasks_for_worktree".into(),
                },
                "cargo check".to_string(),
                vec!["check".to_string()],
                HashMap::default(),
            ),
        ]
    );

    let (_, resolved_task) = cx
        .update(|cx| get_all_tasks(&project, Some(worktree_id), &task_context, cx))
        .await
        .into_iter()
        .find(|(source_kind, _)| source_kind == &global_task_source_kind)
        .expect("should have one global task");
    project.update(cx, |project, cx| {
        project.task_inventory().update(cx, |inventory, _| {
            inventory.task_scheduled(global_task_source_kind.clone(), resolved_task);
        });
    });

    let tasks = serde_json::to_string(&TaskTemplates(vec![TaskTemplate {
        label: "cargo check".to_string(),
        command: "cargo".to_string(),
        args: vec![
            "check".to_string(),
            "--all".to_string(),
            "--all-targets".to_string(),
        ],
        env: HashMap::from_iter(Some((
            "RUSTFLAGS".to_string(),
            "-Zunstable-options".to_string(),
        ))),
        ..TaskTemplate::default()
    }]))
    .unwrap();
    let (tx, rx) = futures::channel::mpsc::unbounded();
    cx.update(|cx| {
        project.update(cx, |project, cx| {
            project.task_inventory().update(cx, |inventory, cx| {
                inventory.remove_local_static_source(Path::new("/the-root/.zed/tasks.json"));
                inventory.add_source(
                    global_task_source_kind.clone(),
                    |tx, cx| StaticSource::new(TrackedFile::new(rx, tx, cx)),
                    cx,
                );
            });
        })
    });
    tx.unbounded_send(tasks).unwrap();

    cx.run_until_parked();
    let all_tasks = cx
        .update(|cx| get_all_tasks(&project, Some(worktree_id), &task_context, cx))
        .await
        .into_iter()
        .map(|(source_kind, task)| {
            let resolved = task.resolved.unwrap();
            (
                source_kind,
                task.resolved_label,
                resolved.args,
                resolved.env,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        all_tasks,
        vec![
            (
                TaskSourceKind::Worktree {
                    id: worktree_id,
                    abs_path: PathBuf::from("/the-root/.zed/tasks.json"),
                    id_base: "local_tasks_for_worktree".into(),
                },
                "cargo check".to_string(),
                vec![
                    "check".to_string(),
                    "--all".to_string(),
                    "--all-targets".to_string()
                ],
                HashMap::from_iter(Some((
                    "RUSTFLAGS".to_string(),
                    "-Zunstable-options".to_string()
                ))),
            ),
            (
                TaskSourceKind::Worktree {
                    id: worktree_id,
                    abs_path: PathBuf::from("/the-root/b/.zed/tasks.json"),
                    id_base: "local_tasks_for_worktree".into(),
                },
                "cargo check".to_string(),
                vec!["check".to_string()],
                HashMap::default(),
            ),
        ]
    );
}

#[gpui::test]
async fn test_managing_language_servers(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
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
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    let mut fake_rust_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            name: "the-rust-language-server",
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), "::".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    let mut fake_json_servers = language_registry.register_fake_lsp_adapter(
        "JSON",
        FakeLspAdapter {
            name: "the-json-language-server",
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![":".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    // Open a buffer without an associated language server.
    let toml_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/the-root/Cargo.toml", cx)
        })
        .await
        .unwrap();

    // Open a buffer with an associated language server before the language for it has been loaded.
    let rust_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/the-root/test.rs", cx)
        })
        .await
        .unwrap();
    rust_buffer.update(cx, |buffer, _| {
        assert_eq!(buffer.language().map(|l| l.name()), None);
    });

    // Now we add the languages to the project, and ensure they get assigned to all
    // the relevant open buffers.
    language_registry.add(json_lang());
    language_registry.add(rust_lang());
    cx.executor().run_until_parked();
    rust_buffer.update(cx, |buffer, _| {
        assert_eq!(buffer.language().map(|l| l.name()), Some("Rust".into()));
    });

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
            language_id: "rust".to_string(),
        }
    );

    // The buffer is configured based on the language server's capabilities.
    rust_buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer.completion_triggers(),
            &[".".to_string(), "::".to_string()]
        );
    });
    toml_buffer.update(cx, |buffer, _| {
        assert!(buffer.completion_triggers().is_empty());
    });

    // Edit a buffer. The changes are reported to the language server.
    rust_buffer.update(cx, |buffer, cx| buffer.edit([(16..16, "2")], None, cx));
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
            language_id: "json".to_string(),
        }
    );

    // This buffer is configured based on the second language server's
    // capabilities.
    json_buffer.update(cx, |buffer, _| {
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
    rust_buffer2.update(cx, |buffer, _| {
        assert_eq!(
            buffer.completion_triggers(),
            &[".".to_string(), "::".to_string()]
        );
    });

    // Changes are reported only to servers matching the buffer's language.
    toml_buffer.update(cx, |buffer, cx| buffer.edit([(5..5, "23")], None, cx));
    rust_buffer2.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "let x = 1;")], None, cx)
    });
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
    project
        .update(cx, |project, cx| project.save_buffer(toml_buffer, cx))
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
            text: rust_buffer2.update(cx, |buffer, _| buffer.text()),
            language_id: "rust".to_string(),
        },
    );

    rust_buffer2.update(cx, |buffer, cx| {
        buffer.update_diagnostics(
            LanguageServerId(0),
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
            text: rust_buffer2.update(cx, |buffer, _| buffer.text()),
            language_id: "json".to_string(),
        },
    );

    // We clear the diagnostics, since the language has changed.
    rust_buffer2.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, usize>(0..buffer.len(), false)
                .count(),
            0
        );
    });

    // The renamed file's version resets after changing language server.
    rust_buffer2.update(cx, |buffer, cx| buffer.edit([(0..0, "// ")], None, cx));
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
            version: 0,
            text: rust_buffer.update(cx, |buffer, _| buffer.text()),
            language_id: "rust".to_string(),
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
                text: json_buffer.update(cx, |buffer, _| buffer.text()),
                language_id: "json".to_string(),
            },
            lsp::TextDocumentItem {
                uri: lsp::Url::from_file_path("/the-root/test3.json").unwrap(),
                version: 0,
                text: rust_buffer2.update(cx, |buffer, _| buffer.text()),
                language_id: "json".to_string(),
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
async fn test_reporting_fs_changes_to_language_servers(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/the-root",
        json!({
            ".gitignore": "target\n",
            "src": {
                "a.rs": "",
                "b.rs": "",
            },
            "target": {
                "x": {
                    "out": {
                        "x.rs": ""
                    }
                },
                "y": {
                    "out": {
                        "y.rs": "",
                    }
                },
                "z": {
                    "out": {
                        "z.rs": ""
                    }
                }
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/the-root".as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            name: "the-language-server",
            ..Default::default()
        },
    );

    cx.executor().run_until_parked();

    // Start the language server by opening a buffer with a compatible file extension.
    let _buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/the-root/src/a.rs", cx)
        })
        .await
        .unwrap();

    // Initially, we don't load ignored files because the language server has not explicitly asked us to watch them.
    project.update(cx, |project, cx| {
        let worktree = project.worktrees().next().unwrap();
        assert_eq!(
            worktree
                .read(cx)
                .snapshot()
                .entries(true)
                .map(|entry| (entry.path.as_ref(), entry.is_ignored))
                .collect::<Vec<_>>(),
            &[
                (Path::new(""), false),
                (Path::new(".gitignore"), false),
                (Path::new("src"), false),
                (Path::new("src/a.rs"), false),
                (Path::new("src/b.rs"), false),
                (Path::new("target"), true),
            ]
        );
    });

    let prev_read_dir_count = fs.read_dir_call_count();

    // Keep track of the FS events reported to the language server.
    let fake_server = fake_servers.next().await.unwrap();
    let file_changes = Arc::new(Mutex::new(Vec::new()));
    fake_server
        .request::<lsp::request::RegisterCapability>(lsp::RegistrationParams {
            registrations: vec![lsp::Registration {
                id: Default::default(),
                method: "workspace/didChangeWatchedFiles".to_string(),
                register_options: serde_json::to_value(
                    lsp::DidChangeWatchedFilesRegistrationOptions {
                        watchers: vec![
                            lsp::FileSystemWatcher {
                                glob_pattern: lsp::GlobPattern::String(
                                    "/the-root/Cargo.toml".to_string(),
                                ),
                                kind: None,
                            },
                            lsp::FileSystemWatcher {
                                glob_pattern: lsp::GlobPattern::String(
                                    "/the-root/src/*.{rs,c}".to_string(),
                                ),
                                kind: None,
                            },
                            lsp::FileSystemWatcher {
                                glob_pattern: lsp::GlobPattern::String(
                                    "/the-root/target/y/**/*.rs".to_string(),
                                ),
                                kind: None,
                            },
                        ],
                    },
                )
                .ok(),
            }],
        })
        .await
        .unwrap();
    fake_server.handle_notification::<lsp::notification::DidChangeWatchedFiles, _>({
        let file_changes = file_changes.clone();
        move |params, _| {
            let mut file_changes = file_changes.lock();
            file_changes.extend(params.changes);
            file_changes.sort_by(|a, b| a.uri.cmp(&b.uri));
        }
    });

    cx.executor().run_until_parked();
    assert_eq!(mem::take(&mut *file_changes.lock()), &[]);
    assert_eq!(fs.read_dir_call_count() - prev_read_dir_count, 4);

    // Now the language server has asked us to watch an ignored directory path,
    // so we recursively load it.
    project.update(cx, |project, cx| {
        let worktree = project.worktrees().next().unwrap();
        assert_eq!(
            worktree
                .read(cx)
                .snapshot()
                .entries(true)
                .map(|entry| (entry.path.as_ref(), entry.is_ignored))
                .collect::<Vec<_>>(),
            &[
                (Path::new(""), false),
                (Path::new(".gitignore"), false),
                (Path::new("src"), false),
                (Path::new("src/a.rs"), false),
                (Path::new("src/b.rs"), false),
                (Path::new("target"), true),
                (Path::new("target/x"), true),
                (Path::new("target/y"), true),
                (Path::new("target/y/out"), true),
                (Path::new("target/y/out/y.rs"), true),
                (Path::new("target/z"), true),
            ]
        );
    });

    // Perform some file system mutations, two of which match the watched patterns,
    // and one of which does not.
    fs.create_file("/the-root/src/c.rs".as_ref(), Default::default())
        .await
        .unwrap();
    fs.create_file("/the-root/src/d.txt".as_ref(), Default::default())
        .await
        .unwrap();
    fs.remove_file("/the-root/src/b.rs".as_ref(), Default::default())
        .await
        .unwrap();
    fs.create_file("/the-root/target/x/out/x2.rs".as_ref(), Default::default())
        .await
        .unwrap();
    fs.create_file("/the-root/target/y/out/y2.rs".as_ref(), Default::default())
        .await
        .unwrap();

    // The language server receives events for the FS mutations that match its watch patterns.
    cx.executor().run_until_parked();
    assert_eq!(
        &*file_changes.lock(),
        &[
            lsp::FileEvent {
                uri: lsp::Url::from_file_path("/the-root/src/b.rs").unwrap(),
                typ: lsp::FileChangeType::DELETED,
            },
            lsp::FileEvent {
                uri: lsp::Url::from_file_path("/the-root/src/c.rs").unwrap(),
                typ: lsp::FileChangeType::CREATED,
            },
            lsp::FileEvent {
                uri: lsp::Url::from_file_path("/the-root/target/y/out/y2.rs").unwrap(),
                typ: lsp::FileChangeType::CREATED,
            },
        ]
    );
}

#[gpui::test]
async fn test_single_file_worktrees_diagnostics(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
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
                LanguageServerId(0),
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
                LanguageServerId(0),
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

    buffer_a.update(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(buffer, 0..buffer.len());
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
    buffer_b.update(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(buffer, 0..buffer.len());
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
async fn test_omitted_diagnostics(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "dir": {
                ".git": {
                    "HEAD": "ref: refs/heads/main",
                },
                ".gitignore": "b.rs",
                "a.rs": "let a = 1;",
                "b.rs": "let b = 2;",
            },
            "other.rs": "let b = c;"
        }),
    )
    .await;

    let project = Project::test(fs, ["/root/dir".as_ref()], cx).await;
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_local_worktree("/root/dir", true, cx)
        })
        .await
        .unwrap();
    let main_worktree_id = worktree.read_with(cx, |tree, _| tree.id());

    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_local_worktree("/root/other.rs", false, cx)
        })
        .await
        .unwrap();
    let other_worktree_id = worktree.update(cx, |tree, _| tree.id());

    let server_id = LanguageServerId(0);
    project.update(cx, |project, cx| {
        project
            .update_diagnostics(
                server_id,
                lsp::PublishDiagnosticsParams {
                    uri: Url::from_file_path("/root/dir/b.rs").unwrap(),
                    version: None,
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 5)),
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: "unused variable 'b'".to_string(),
                        ..Default::default()
                    }],
                },
                &[],
                cx,
            )
            .unwrap();
        project
            .update_diagnostics(
                server_id,
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

    let main_ignored_buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((main_worktree_id, "b.rs"), cx)
        })
        .await
        .unwrap();
    main_ignored_buffer.update(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(buffer, 0..buffer.len());
        assert_eq!(
            chunks
                .iter()
                .map(|(s, d)| (s.as_str(), *d))
                .collect::<Vec<_>>(),
            &[
                ("let ", None),
                ("b", Some(DiagnosticSeverity::ERROR)),
                (" = 2;", None),
            ],
            "Gigitnored buffers should still get in-buffer diagnostics",
        );
    });
    let other_buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((other_worktree_id, ""), cx)
        })
        .await
        .unwrap();
    other_buffer.update(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(buffer, 0..buffer.len());
        assert_eq!(
            chunks
                .iter()
                .map(|(s, d)| (s.as_str(), *d))
                .collect::<Vec<_>>(),
            &[
                ("let b = ", None),
                ("c", Some(DiagnosticSeverity::ERROR)),
                (";", None),
            ],
            "Buffers from hidden projects should still get in-buffer diagnostics"
        );
    });

    project.update(cx, |project, cx| {
        assert_eq!(project.diagnostic_summaries(false, cx).next(), None);
        assert_eq!(
            project.diagnostic_summaries(true, cx).collect::<Vec<_>>(),
            vec![(
                ProjectPath {
                    worktree_id: main_worktree_id,
                    path: Arc::from(Path::new("b.rs")),
                },
                server_id,
                DiagnosticSummary {
                    error_count: 1,
                    warning_count: 0,
                }
            )]
        );
        assert_eq!(project.diagnostic_summary(false, cx).error_count, 0);
        assert_eq!(project.diagnostic_summary(true, cx).error_count, 1);
    });
}

#[gpui::test]
async fn test_disk_based_diagnostics_progress(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let progress_token = "the-progress-token";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a.rs": "fn a() { A }",
            "b.rs": "const y: i32 = 1",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            disk_based_diagnostics_progress_token: Some(progress_token.into()),
            disk_based_diagnostics_sources: vec!["disk".into()],
            ..Default::default()
        },
    );

    let worktree_id = project.update(cx, |p, cx| p.worktrees().next().unwrap().read(cx).id());

    // Cause worktree to start the fake language server
    let _buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/b.rs", cx))
        .await
        .unwrap();

    let mut events = cx.events(&project);

    let fake_server = fake_servers.next().await.unwrap();
    assert_eq!(
        events.next().await.unwrap(),
        Event::LanguageServerAdded(LanguageServerId(0)),
    );

    fake_server
        .start_progress(format!("{}/0", progress_token))
        .await;
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiskBasedDiagnosticsStarted {
            language_server_id: LanguageServerId(0),
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
            language_server_id: LanguageServerId(0),
            path: (worktree_id, Path::new("a.rs")).into()
        }
    );

    fake_server.end_progress(format!("{}/0", progress_token));
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiskBasedDiagnosticsFinished {
            language_server_id: LanguageServerId(0)
        }
    );

    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();

    buffer.update(cx, |buffer, _| {
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
            language_server_id: LanguageServerId(0),
            path: (worktree_id, Path::new("a.rs")).into()
        }
    );

    fake_server.notify::<lsp::notification::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
        uri: Url::from_file_path("/dir/a.rs").unwrap(),
        version: None,
        diagnostics: Default::default(),
    });
    cx.executor().run_until_parked();
    assert_eq!(futures::poll!(events.next()), Poll::Pending);
}

#[gpui::test]
async fn test_restarting_server_with_diagnostics_running(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let progress_token = "the-progress-token";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/dir", json!({ "a.rs": "" })).await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            name: "the-language-server",
            disk_based_diagnostics_sources: vec!["disk".into()],
            disk_based_diagnostics_progress_token: Some(progress_token.into()),
            ..Default::default()
        },
    );

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
    let mut events = cx.events(&project);

    // Simulate the newly started server sending more diagnostics.
    let fake_server = fake_servers.next().await.unwrap();
    assert_eq!(
        events.next().await.unwrap(),
        Event::LanguageServerAdded(LanguageServerId(1))
    );
    fake_server.start_progress(progress_token).await;
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiskBasedDiagnosticsStarted {
            language_server_id: LanguageServerId(1)
        }
    );
    project.update(cx, |project, _| {
        assert_eq!(
            project
                .language_servers_running_disk_based_diagnostics()
                .collect::<Vec<_>>(),
            [LanguageServerId(1)]
        );
    });

    // All diagnostics are considered done, despite the old server's diagnostic
    // task never completing.
    fake_server.end_progress(progress_token);
    assert_eq!(
        events.next().await.unwrap(),
        Event::DiskBasedDiagnosticsFinished {
            language_server_id: LanguageServerId(1)
        }
    );
    project.update(cx, |project, _| {
        assert_eq!(
            project
                .language_servers_running_disk_based_diagnostics()
                .collect::<Vec<_>>(),
            [LanguageServerId(0); 0]
        );
    });
}

#[gpui::test]
async fn test_restarting_server_with_diagnostics_published(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/dir", json!({ "a.rs": "x" })).await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers =
        language_registry.register_fake_lsp_adapter("Rust", FakeLspAdapter::default());

    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();

    // Publish diagnostics
    let fake_server = fake_servers.next().await.unwrap();
    fake_server.notify::<lsp::notification::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
        uri: Url::from_file_path("/dir/a.rs").unwrap(),
        version: None,
        diagnostics: vec![lsp::Diagnostic {
            range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
            severity: Some(lsp::DiagnosticSeverity::ERROR),
            message: "the message".to_string(),
            ..Default::default()
        }],
    });

    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, usize>(0..1, false)
                .map(|entry| entry.diagnostic.message.clone())
                .collect::<Vec<_>>(),
            ["the message".to_string()]
        );
    });
    project.update(cx, |project, cx| {
        assert_eq!(
            project.diagnostic_summary(false, cx),
            DiagnosticSummary {
                error_count: 1,
                warning_count: 0,
            }
        );
    });

    project.update(cx, |project, cx| {
        project.restart_language_servers_for_buffers([buffer.clone()], cx);
    });

    // The diagnostics are cleared.
    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, usize>(0..1, false)
                .map(|entry| entry.diagnostic.message.clone())
                .collect::<Vec<_>>(),
            Vec::<String>::new(),
        );
    });
    project.update(cx, |project, cx| {
        assert_eq!(
            project.diagnostic_summary(false, cx),
            DiagnosticSummary {
                error_count: 0,
                warning_count: 0,
            }
        );
    });
}

#[gpui::test]
async fn test_restarted_server_reporting_invalid_buffer_version(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/dir", json!({ "a.rs": "" })).await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    language_registry.add(rust_lang());
    let mut fake_servers =
        language_registry.register_fake_lsp_adapter("Rust", FakeLspAdapter::default());

    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();

    // Before restarting the server, report diagnostics with an unknown buffer version.
    let fake_server = fake_servers.next().await.unwrap();
    fake_server.notify::<lsp::notification::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
        uri: lsp::Url::from_file_path("/dir/a.rs").unwrap(),
        version: Some(10000),
        diagnostics: Vec::new(),
    });
    cx.executor().run_until_parked();

    project.update(cx, |project, cx| {
        project.restart_language_servers_for_buffers([buffer.clone()], cx);
    });
    let mut fake_server = fake_servers.next().await.unwrap();
    let notification = fake_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await
        .text_document;
    assert_eq!(notification.version, 0);
}

#[gpui::test]
async fn test_toggling_enable_language_server(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/dir", json!({ "a.rs": "", "b.js": "" }))
        .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    let mut fake_rust_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            name: "rust-lsp",
            ..Default::default()
        },
    );
    let mut fake_js_servers = language_registry.register_fake_lsp_adapter(
        "JavaScript",
        FakeLspAdapter {
            name: "js-lsp",
            ..Default::default()
        },
    );
    language_registry.add(rust_lang());
    language_registry.add(js_lang());

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
        SettingsStore::update_global(cx, |settings, cx| {
            settings.update_user_settings::<AllLanguageSettings>(cx, |settings| {
                settings.languages.insert(
                    Arc::from("Rust"),
                    LanguageSettingsContent {
                        enable_language_server: Some(false),
                        ..Default::default()
                    },
                );
            });
        })
    });
    fake_rust_server_1
        .receive_notification::<lsp::notification::Exit>()
        .await;

    // Enable Rust and disable JavaScript language servers, ensuring that the
    // former gets started again and that the latter stops.
    cx.update(|cx| {
        SettingsStore::update_global(cx, |settings, cx| {
            settings.update_user_settings::<AllLanguageSettings>(cx, |settings| {
                settings.languages.insert(
                    Arc::from("Rust"),
                    LanguageSettingsContent {
                        enable_language_server: Some(true),
                        ..Default::default()
                    },
                );
                settings.languages.insert(
                    Arc::from("JavaScript"),
                    LanguageSettingsContent {
                        enable_language_server: Some(false),
                        ..Default::default()
                    },
                );
            });
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

#[gpui::test(iterations = 3)]
async fn test_transforming_diagnostics(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let text = "
        fn a() { A }
        fn b() { BB }
        fn c() { CCC }
    "
    .unindent();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/dir", json!({ "a.rs": text })).await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            disk_based_diagnostics_sources: vec!["disk".into()],
            ..Default::default()
        },
    );

    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
        .await
        .unwrap();

    let mut fake_server = fake_servers.next().await.unwrap();
    let open_notification = fake_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;

    // Edit the buffer, moving the content down
    buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "\n\n")], None, cx));
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
    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(Point::new(3, 0)..Point::new(5, 0), false)
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(3, 9)..Point::new(3, 11),
                    diagnostic: Diagnostic {
                        source: Some("disk".into()),
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
                        source: Some("disk".into()),
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

    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(Point::new(2, 0)..Point::new(3, 0), false)
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(2, 9)..Point::new(2, 12),
                    diagnostic: Diagnostic {
                        source: Some("disk".into()),
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
                        source: Some("disk".into()),
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
        buffer.edit([(Point::new(2, 0)..Point::new(2, 0), "    ")], None, cx);
        buffer.edit(
            [(Point::new(2, 8)..Point::new(2, 10), "(x: usize)")],
            None,
            cx,
        );
        buffer.edit([(Point::new(3, 10)..Point::new(3, 10), "xxx")], None, cx);
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

    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(0..buffer.len(), false)
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(2, 21)..Point::new(2, 22),
                    diagnostic: Diagnostic {
                        source: Some("disk".into()),
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
                        source: Some("disk".into()),
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
    init_test(cx);

    let text = concat!(
        "let one = ;\n", //
        "let two = \n",
        "let three = 3;\n",
    );

    let fs = FakeFs::new(cx.executor());
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
                LanguageServerId(0),
                None,
                vec![
                    DiagnosticEntry {
                        range: Unclipped(PointUtf16::new(0, 10))..Unclipped(PointUtf16::new(0, 10)),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "syntax error 1".to_string(),
                            ..Default::default()
                        },
                    },
                    DiagnosticEntry {
                        range: Unclipped(PointUtf16::new(1, 10))..Unclipped(PointUtf16::new(1, 10)),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "syntax error 2".to_string(),
                            ..Default::default()
                        },
                    },
                ],
                cx,
            )
            .unwrap();
    });

    // An empty range is extended forward to include the following character.
    // At the end of a line, an empty range is extended backward to include
    // the preceding character.
    buffer.update(cx, |buffer, _| {
        let chunks = chunks_with_diagnostics(buffer, 0..buffer.len());
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
async fn test_diagnostics_from_multiple_language_servers(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/dir", json!({ "a.rs": "one two three" }))
        .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;

    project.update(cx, |project, cx| {
        project
            .update_diagnostic_entries(
                LanguageServerId(0),
                Path::new("/dir/a.rs").to_owned(),
                None,
                vec![DiagnosticEntry {
                    range: Unclipped(PointUtf16::new(0, 0))..Unclipped(PointUtf16::new(0, 3)),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        is_primary: true,
                        message: "syntax error a1".to_string(),
                        ..Default::default()
                    },
                }],
                cx,
            )
            .unwrap();
        project
            .update_diagnostic_entries(
                LanguageServerId(1),
                Path::new("/dir/a.rs").to_owned(),
                None,
                vec![DiagnosticEntry {
                    range: Unclipped(PointUtf16::new(0, 0))..Unclipped(PointUtf16::new(0, 3)),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        is_primary: true,
                        message: "syntax error b1".to_string(),
                        ..Default::default()
                    },
                }],
                cx,
            )
            .unwrap();

        assert_eq!(
            project.diagnostic_summary(false, cx),
            DiagnosticSummary {
                error_count: 2,
                warning_count: 0,
            }
        );
    });
}

#[gpui::test]
async fn test_edits_from_lsp2_with_past_version(cx: &mut gpui::TestAppContext) {
    init_test(cx);

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

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a.rs": text.clone(),
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers =
        language_registry.register_fake_lsp_adapter("Rust", FakeLspAdapter::default());

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
            None,
            cx,
        );
        buffer.edit(
            [(
                Point::new(2, 0)..Point::new(2, 0),
                "    // inside first function\n",
            )],
            None,
            cx,
        );
        buffer.edit(
            [(
                Point::new(6, 4)..Point::new(6, 4),
                "// inside second function ",
            )],
            None,
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
                LanguageServerId(0),
                Some(lsp_document_version),
                cx,
            )
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        for (range, new_text) in edits {
            buffer.edit([(range, new_text)], None, cx);
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
async fn test_edits_from_lsp2_with_edits_on_adjacent_lines(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let text = "
        use a::b;
        use a::c;

        fn f() {
            b();
            c();
        }
    "
    .unindent();

    let fs = FakeFs::new(cx.executor());
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
                LanguageServerId(0),
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
                    range.start.to_point(buffer)..range.end.to_point(buffer),
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
            buffer.edit([(range, new_text)], None, cx);
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
async fn test_invalid_edits_from_lsp2(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let text = "
        use a::b;
        use a::c;

        fn f() {
            b();
            c();
        }
    "
    .unindent();

    let fs = FakeFs::new(cx.executor());
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
    // with ranges sometimes being inverted or pointing to invalid locations.
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
                        range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(99, 0)),
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
                LanguageServerId(0),
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
                    range.start.to_point(buffer)..range.end.to_point(buffer),
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
            buffer.edit([(range, new_text)], None, cx);
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

#[gpui::test(iterations = 10)]
async fn test_definition(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a.rs": "const fn a() { A }",
            "b.rs": "const y: i32 = crate::a()",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir/b.rs".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers =
        language_registry.register_fake_lsp_adapter("Rust", FakeLspAdapter::default());

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
    cx.executor().run_until_parked();
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
            [("/dir/a.rs".as_ref(), false), ("/dir/b.rs".as_ref(), true)],
        );

        drop(definition);
    });
    cx.update(|cx| {
        assert_eq!(list_worktrees(&project, cx), [("/dir/b.rs".as_ref(), true)]);
    });

    fn list_worktrees<'a>(
        project: &'a Model<Project>,
        cx: &'a AppContext,
    ) -> Vec<(&'a Path, bool)> {
        project
            .read(cx)
            .worktrees()
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
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a.ts": "",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(typescript_lang());
    let mut fake_language_servers = language_registry.register_fake_lsp_adapter(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![":".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        },
    );

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
    let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());
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
    let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());
    assert_eq!(completions.len(), 1);
    assert_eq!(completions[0].new_text, "component");
    assert_eq!(
        completions[0].old_range.to_offset(&snapshot),
        text.len() - 4..text.len() - 1
    );
}

#[gpui::test]
async fn test_completions_with_carriage_returns(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a.ts": "",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(typescript_lang());
    let mut fake_language_servers = language_registry.register_fake_lsp_adapter(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![":".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        },
    );

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
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a.ts": "a",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(typescript_lang());
    let mut fake_language_servers = language_registry.register_fake_lsp_adapter(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                code_action_provider: Some(lsp::CodeActionProviderCapability::Options(
                    lsp::CodeActionOptions {
                        resolve_provider: Some(true),
                        ..lsp::CodeActionOptions::default()
                    },
                )),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );

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
                    data: Some(serde_json::json!({
                        "command": "_the/command",
                    })),
                    ..lsp::CodeAction::default()
                }),
                lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
                    title: "two".into(),
                    ..lsp::CodeAction::default()
                }),
            ]))
        })
        .next()
        .await;

    let action = actions.await[0].clone();
    let apply = project.update(cx, |project, cx| {
        project.apply_code_action(buffer.clone(), action, true, cx)
    });

    // Resolving the code action does not populate its edits. In absence of
    // edits, we must execute the given command.
    fake_server.handle_request::<lsp::request::CodeActionResolveRequest, _, _>(
        |mut action, _| async move {
            if action.data.is_some() {
                action.command = Some(lsp::Command {
                    title: "The command".into(),
                    command: "_the/command".into(),
                    arguments: Some(vec![json!("the-argument")]),
                });
            }
            Ok(action)
        },
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

#[gpui::test(iterations = 10)]
async fn test_save_file(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
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
    buffer.update(cx, |buffer, cx| {
        assert_eq!(buffer.text(), "the old contents");
        buffer.edit([(0..0, "a line of text.\n".repeat(10 * 1024))], None, cx);
    });

    project
        .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
        .await
        .unwrap();

    let new_text = fs.load(Path::new("/dir/file1")).await.unwrap();
    assert_eq!(new_text, buffer.update(cx, |buffer, _| buffer.text()));
}

#[gpui::test(iterations = 30)]
async fn test_file_changes_multiple_times_on_disk(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
    fs.insert_tree(
        "/dir",
        json!({
            "file1": "the original contents",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
    let worktree = project.read_with(cx, |project, _| project.worktrees().next().unwrap());
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/file1", cx))
        .await
        .unwrap();

    // Simulate buffer diffs being slow, so that they don't complete before
    // the next file change occurs.
    cx.executor().deprioritize(*language::BUFFER_DIFF_TASK);

    // Change the buffer's file on disk, and then wait for the file change
    // to be detected by the worktree, so that the buffer starts reloading.
    fs.save(
        "/dir/file1".as_ref(),
        &"the first contents".into(),
        Default::default(),
    )
    .await
    .unwrap();
    worktree.next_event(cx).await;

    // Change the buffer's file again. Depending on the random seed, the
    // previous file change may still be in progress.
    fs.save(
        "/dir/file1".as_ref(),
        &"the second contents".into(),
        Default::default(),
    )
    .await
    .unwrap();
    worktree.next_event(cx).await;

    cx.executor().run_until_parked();
    let on_disk_text = fs.load(Path::new("/dir/file1")).await.unwrap();
    buffer.read_with(cx, |buffer, _| {
        assert_eq!(buffer.text(), on_disk_text);
        assert!(!buffer.is_dirty(), "buffer should not be dirty");
        assert!(!buffer.has_conflict(), "buffer should not be dirty");
    });
}

#[gpui::test(iterations = 30)]
async fn test_edit_buffer_while_it_reloads(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
    fs.insert_tree(
        "/dir",
        json!({
            "file1": "the original contents",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
    let worktree = project.read_with(cx, |project, _| project.worktrees().next().unwrap());
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/file1", cx))
        .await
        .unwrap();

    // Simulate buffer diffs being slow, so that they don't complete before
    // the next file change occurs.
    cx.executor().deprioritize(*language::BUFFER_DIFF_TASK);

    // Change the buffer's file on disk, and then wait for the file change
    // to be detected by the worktree, so that the buffer starts reloading.
    fs.save(
        "/dir/file1".as_ref(),
        &"the first contents".into(),
        Default::default(),
    )
    .await
    .unwrap();
    worktree.next_event(cx).await;

    cx.executor()
        .spawn(cx.executor().simulate_random_delay())
        .await;

    // Perform a noop edit, causing the buffer's version to increase.
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, " ")], None, cx);
        buffer.undo(cx);
    });

    cx.executor().run_until_parked();
    let on_disk_text = fs.load(Path::new("/dir/file1")).await.unwrap();
    buffer.read_with(cx, |buffer, _| {
        let buffer_text = buffer.text();
        if buffer_text == on_disk_text {
            assert!(
                !buffer.is_dirty() && !buffer.has_conflict(),
                "buffer shouldn't be dirty. text: {buffer_text:?}, disk text: {on_disk_text:?}",
            );
        }
        // If the file change occurred while the buffer was processing the first
        // change, the buffer will be in a conflicting state.
        else {
            assert!(buffer.is_dirty(), "buffer should report that it is dirty. text: {buffer_text:?}, disk text: {on_disk_text:?}");
            assert!(buffer.has_conflict(), "buffer should report that it is dirty. text: {buffer_text:?}, disk text: {on_disk_text:?}");
        }
    });
}

#[gpui::test]
async fn test_save_in_single_file_worktree(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
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
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "a line of text.\n".repeat(10 * 1024))], None, cx);
    });

    project
        .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
        .await
        .unwrap();

    let new_text = fs.load(Path::new("/dir/file1")).await.unwrap();
    assert_eq!(new_text, buffer.update(cx, |buffer, _| buffer.text()));
}

#[gpui::test]
async fn test_save_as(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/dir", json!({})).await;

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    let languages = project.update(cx, |project, _| project.languages().clone());
    languages.add(rust_lang());

    let buffer = project.update(cx, |project, cx| project.create_local_buffer("", None, cx));
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "abc")], None, cx);
        assert!(buffer.is_dirty());
        assert!(!buffer.has_conflict());
        assert_eq!(buffer.language().unwrap().name().as_ref(), "Plain Text");
    });
    project
        .update(cx, |project, cx| {
            let worktree_id = project.worktrees().next().unwrap().read(cx).id();
            let path = ProjectPath {
                worktree_id,
                path: Arc::from(Path::new("file1.rs")),
            };
            project.save_buffer_as(buffer.clone(), path, cx)
        })
        .await
        .unwrap();
    assert_eq!(fs.load(Path::new("/dir/file1.rs")).await.unwrap(), "abc");

    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, cx| {
        assert_eq!(
            buffer.file().unwrap().full_path(cx),
            Path::new("dir/file1.rs")
        );
        assert!(!buffer.is_dirty());
        assert!(!buffer.has_conflict());
        assert_eq!(buffer.language().unwrap().name().as_ref(), "Rust");
    });

    let opened_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/dir/file1.rs", cx)
        })
        .await
        .unwrap();
    assert_eq!(opened_buffer, buffer);
}

#[gpui::test(retries = 5)]
async fn test_rescan_and_remote_updates(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

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

    let project = Project::test(Arc::new(RealFs::default()), [dir.path()], cx).await;
    let rpc = project.update(cx, |p, _| p.client.clone());

    let buffer_for_path = |path: &'static str, cx: &mut gpui::TestAppContext| {
        let buffer = project.update(cx, |p, cx| p.open_local_buffer(dir.path().join(path), cx));
        async move { buffer.await.unwrap() }
    };
    let id_for_path = |path: &'static str, cx: &mut gpui::TestAppContext| {
        project.update(cx, |project, cx| {
            let tree = project.worktrees().next().unwrap();
            tree.read(cx)
                .entry_for_path(path)
                .unwrap_or_else(|| panic!("no entry for path {}", path))
                .id
        })
    };

    let buffer2 = buffer_for_path("a/file2", cx).await;
    let buffer3 = buffer_for_path("a/file3", cx).await;
    let buffer4 = buffer_for_path("b/c/file4", cx).await;
    let buffer5 = buffer_for_path("b/c/file5", cx).await;

    let file2_id = id_for_path("a/file2", cx);
    let file3_id = id_for_path("a/file3", cx);
    let file4_id = id_for_path("b/c/file4", cx);

    // Create a remote copy of this worktree.
    let tree = project.update(cx, |project, _| project.worktrees().next().unwrap());

    let metadata = tree.update(cx, |tree, _| tree.as_local().unwrap().metadata_proto());

    let updates = Arc::new(Mutex::new(Vec::new()));
    tree.update(cx, |tree, cx| {
        let _ = tree.as_local_mut().unwrap().observe_updates(0, cx, {
            let updates = updates.clone();
            move |update| {
                updates.lock().push(update);
                async { true }
            }
        });
    });

    let remote = cx.update(|cx| Worktree::remote(1, 1, metadata, rpc.clone(), cx));

    cx.executor().run_until_parked();

    cx.update(|cx| {
        assert!(!buffer2.read(cx).is_dirty());
        assert!(!buffer3.read(cx).is_dirty());
        assert!(!buffer4.read(cx).is_dirty());
        assert!(!buffer5.read(cx).is_dirty());
    });

    // Rename and delete files and directories.
    tree.flush_fs_events(cx).await;
    std::fs::rename(dir.path().join("a/file3"), dir.path().join("b/c/file3")).unwrap();
    std::fs::remove_file(dir.path().join("b/c/file5")).unwrap();
    std::fs::rename(dir.path().join("b/c"), dir.path().join("d")).unwrap();
    std::fs::rename(dir.path().join("a/file2"), dir.path().join("a/file2.new")).unwrap();
    tree.flush_fs_events(cx).await;

    let expected_paths = vec![
        "a",
        "a/file1",
        "a/file2.new",
        "b",
        "d",
        "d/file3",
        "d/file4",
    ];

    cx.update(|app| {
        assert_eq!(
            tree.read(app)
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            expected_paths
        );
    });

    assert_eq!(id_for_path("a/file2.new", cx), file2_id);
    assert_eq!(id_for_path("d/file3", cx), file3_id);
    assert_eq!(id_for_path("d/file4", cx), file4_id);

    cx.update(|cx| {
        assert_eq!(
            buffer2.read(cx).file().unwrap().path().as_ref(),
            Path::new("a/file2.new")
        );
        assert_eq!(
            buffer3.read(cx).file().unwrap().path().as_ref(),
            Path::new("d/file3")
        );
        assert_eq!(
            buffer4.read(cx).file().unwrap().path().as_ref(),
            Path::new("d/file4")
        );
        assert_eq!(
            buffer5.read(cx).file().unwrap().path().as_ref(),
            Path::new("b/c/file5")
        );

        assert!(!buffer2.read(cx).file().unwrap().is_deleted());
        assert!(!buffer3.read(cx).file().unwrap().is_deleted());
        assert!(!buffer4.read(cx).file().unwrap().is_deleted());
        assert!(buffer5.read(cx).file().unwrap().is_deleted());
    });

    // Update the remote worktree. Check that it becomes consistent with the
    // local worktree.
    cx.executor().run_until_parked();

    remote.update(cx, |remote, _| {
        for update in updates.lock().drain(..) {
            remote.as_remote_mut().unwrap().update_from_remote(update);
        }
    });
    cx.executor().run_until_parked();
    remote.update(cx, |remote, _| {
        assert_eq!(
            remote
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            expected_paths
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_buffer_identity_across_renames(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a": {
                "file1": "",
            }
        }),
    )
    .await;

    let project = Project::test(fs, [Path::new("/dir")], cx).await;
    let tree = project.update(cx, |project, _| project.worktrees().next().unwrap());
    let tree_id = tree.update(cx, |tree, _| tree.id());

    let id_for_path = |path: &'static str, cx: &mut gpui::TestAppContext| {
        project.update(cx, |project, cx| {
            let tree = project.worktrees().next().unwrap();
            tree.read(cx)
                .entry_for_path(path)
                .unwrap_or_else(|| panic!("no entry for path {}", path))
                .id
        })
    };

    let dir_id = id_for_path("a", cx);
    let file_id = id_for_path("a/file1", cx);
    let buffer = project
        .update(cx, |p, cx| p.open_buffer((tree_id, "a/file1"), cx))
        .await
        .unwrap();
    buffer.update(cx, |buffer, _| assert!(!buffer.is_dirty()));

    project
        .update(cx, |project, cx| {
            project.rename_entry(dir_id, Path::new("b"), cx)
        })
        .unwrap()
        .await
        .unwrap();
    cx.executor().run_until_parked();

    assert_eq!(id_for_path("b", cx), dir_id);
    assert_eq!(id_for_path("b/file1", cx), file_id);
    buffer.update(cx, |buffer, _| assert!(!buffer.is_dirty()));
}

#[gpui::test]
async fn test_buffer_deduping(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
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
    assert_eq!(buffer_a_1.update(cx, |b, _| b.text()), "a-contents");
    assert_eq!(buffer_b.update(cx, |b, _| b.text()), "b-contents");

    // There is only one buffer per path.
    let buffer_a_id = buffer_a_1.entity_id();
    assert_eq!(buffer_a_2.entity_id(), buffer_a_id);

    // Open the same path again while it is still open.
    drop(buffer_a_1);
    let buffer_a_3 = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/a.txt", cx))
        .await
        .unwrap();

    // There's still only one buffer per path.
    assert_eq!(buffer_a_3.entity_id(), buffer_a_id);
}

#[gpui::test]
async fn test_buffer_is_dirty(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
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
    let events = Arc::new(Mutex::new(Vec::new()));

    // initially, the buffer isn't dirty.
    buffer1.update(cx, |buffer, cx| {
        cx.subscribe(&buffer1, {
            let events = events.clone();
            move |_, _, event, _| match event {
                BufferEvent::Operation(_) => {}
                _ => events.lock().push(event.clone()),
            }
        })
        .detach();

        assert!(!buffer.is_dirty());
        assert!(events.lock().is_empty());

        buffer.edit([(1..2, "")], None, cx);
    });

    // after the first edit, the buffer is dirty, and emits a dirtied event.
    buffer1.update(cx, |buffer, cx| {
        assert!(buffer.text() == "ac");
        assert!(buffer.is_dirty());
        assert_eq!(
            *events.lock(),
            &[language::Event::Edited, language::Event::DirtyChanged]
        );
        events.lock().clear();
        buffer.did_save(buffer.version(), buffer.file().unwrap().mtime(), cx);
    });

    // after saving, the buffer is not dirty, and emits a saved event.
    buffer1.update(cx, |buffer, cx| {
        assert!(!buffer.is_dirty());
        assert_eq!(*events.lock(), &[language::Event::Saved]);
        events.lock().clear();

        buffer.edit([(1..1, "B")], None, cx);
        buffer.edit([(2..2, "D")], None, cx);
    });

    // after editing again, the buffer is dirty, and emits another dirty event.
    buffer1.update(cx, |buffer, cx| {
        assert!(buffer.text() == "aBDc");
        assert!(buffer.is_dirty());
        assert_eq!(
            *events.lock(),
            &[
                language::Event::Edited,
                language::Event::DirtyChanged,
                language::Event::Edited,
            ],
        );
        events.lock().clear();

        // After restoring the buffer to its previously-saved state,
        // the buffer is not considered dirty anymore.
        buffer.edit([(1..3, "")], None, cx);
        assert!(buffer.text() == "ac");
        assert!(!buffer.is_dirty());
    });

    assert_eq!(
        *events.lock(),
        &[language::Event::Edited, language::Event::DirtyChanged]
    );

    // When a file is deleted, the buffer is considered dirty.
    let events = Arc::new(Mutex::new(Vec::new()));
    let buffer2 = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/file2", cx))
        .await
        .unwrap();
    buffer2.update(cx, |_, cx| {
        cx.subscribe(&buffer2, {
            let events = events.clone();
            move |_, _, event, _| events.lock().push(event.clone())
        })
        .detach();
    });

    fs.remove_file("/dir/file2".as_ref(), Default::default())
        .await
        .unwrap();
    cx.executor().run_until_parked();
    buffer2.update(cx, |buffer, _| assert!(buffer.is_dirty()));
    assert_eq!(
        *events.lock(),
        &[
            language::Event::DirtyChanged,
            language::Event::FileHandleChanged
        ]
    );

    // When a file is already dirty when deleted, we don't emit a Dirtied event.
    let events = Arc::new(Mutex::new(Vec::new()));
    let buffer3 = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/file3", cx))
        .await
        .unwrap();
    buffer3.update(cx, |_, cx| {
        cx.subscribe(&buffer3, {
            let events = events.clone();
            move |_, _, event, _| events.lock().push(event.clone())
        })
        .detach();
    });

    buffer3.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "x")], None, cx);
    });
    events.lock().clear();
    fs.remove_file("/dir/file3".as_ref(), Default::default())
        .await
        .unwrap();
    cx.executor().run_until_parked();
    assert_eq!(*events.lock(), &[language::Event::FileHandleChanged]);
    cx.update(|cx| assert!(buffer3.read(cx).is_dirty()));
}

#[gpui::test]
async fn test_buffer_file_changes_on_disk(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let initial_contents = "aaa\nbbbbb\nc\n";
    let fs = FakeFs::new(cx.executor());
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
        .map(|row| buffer.update(cx, |b, _| b.anchor_before(Point::new(row, 1))))
        .collect::<Vec<_>>();

    // Change the file on disk, adding two new lines of text, and removing
    // one line.
    buffer.update(cx, |buffer, _| {
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
    cx.executor().run_until_parked();
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
            [Point::new(1, 1), Point::new(3, 1), Point::new(3, 5)]
        );
    });

    // Modify the buffer
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, " ")], None, cx);
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
    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert!(buffer.has_conflict());
    });
}

#[gpui::test]
async fn test_buffer_line_endings(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
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

    buffer1.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), "a\nb\nc\n");
        assert_eq!(buffer.line_ending(), LineEnding::Unix);
    });
    buffer2.update(cx, |buffer, _| {
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
    cx.executor().run_until_parked();
    buffer1.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), "aaa\nb\nc\n");
        assert_eq!(buffer.line_ending(), LineEnding::Windows);
    });

    // Save a file with windows line endings. The file is written correctly.
    buffer2.update(cx, |buffer, cx| {
        buffer.set_text("one\ntwo\nthree\nfour\n", cx);
    });
    project
        .update(cx, |project, cx| project.save_buffer(buffer2, cx))
        .await
        .unwrap();
    assert_eq!(
        fs.load("/dir/file2".as_ref()).await.unwrap(),
        "one\r\ntwo\r\nthree\r\nfour\r\n",
    );
}

#[gpui::test]
async fn test_grouped_diagnostics(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
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
                        uri: buffer_uri,
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
        .update(cx, |p, cx| {
            p.update_diagnostics(LanguageServerId(0), message, &[], cx)
        })
        .unwrap();
    let buffer = buffer.update(cx, |buffer, _| buffer.snapshot());

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
                    group_id: 1,
                    is_primary: true,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 8)..Point::new(1, 9),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 1 hint 1".to_string(),
                    group_id: 1,
                    is_primary: false,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 13)..Point::new(1, 15),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 2 hint 1".to_string(),
                    group_id: 0,
                    is_primary: false,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 13)..Point::new(1, 15),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 2 hint 2".to_string(),
                    group_id: 0,
                    is_primary: false,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(2, 8)..Point::new(2, 17),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::ERROR,
                    message: "error 2".to_string(),
                    group_id: 0,
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
                range: Point::new(1, 13)..Point::new(1, 15),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 2 hint 1".to_string(),
                    group_id: 0,
                    is_primary: false,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 13)..Point::new(1, 15),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 2 hint 2".to_string(),
                    group_id: 0,
                    is_primary: false,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(2, 8)..Point::new(2, 17),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::ERROR,
                    message: "error 2".to_string(),
                    group_id: 0,
                    is_primary: true,
                    ..Default::default()
                }
            }
        ]
    );

    assert_eq!(
        buffer.diagnostic_group::<Point>(1).collect::<Vec<_>>(),
        &[
            DiagnosticEntry {
                range: Point::new(1, 8)..Point::new(1, 9),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::WARNING,
                    message: "error 1".to_string(),
                    group_id: 1,
                    is_primary: true,
                    ..Default::default()
                }
            },
            DiagnosticEntry {
                range: Point::new(1, 8)..Point::new(1, 9),
                diagnostic: Diagnostic {
                    severity: DiagnosticSeverity::HINT,
                    message: "error 1 hint 1".to_string(),
                    group_id: 1,
                    is_primary: false,
                    ..Default::default()
                }
            },
        ]
    );
}

#[gpui::test]
async fn test_rename(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "one.rs": "const ONE: usize = 1;",
            "two.rs": "const TWO: usize = one::ONE + one::ONE;"
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                rename_provider: Some(lsp::OneOf::Right(lsp::RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                ..Default::default()
            },
            ..Default::default()
        },
    );

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
    let range = buffer.update(cx, |buffer, _| range.to_offset(buffer));
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
            .update(cx, |buffer, _| buffer.text()),
        "const THREE: usize = 1;"
    );
    assert_eq!(
        transaction
            .into_keys()
            .next()
            .unwrap()
            .update(cx, |buffer, _| buffer.text()),
        "const TWO: usize = one::THREE + one::THREE;"
    );
}

#[gpui::test]
async fn test_search(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
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
        search(
            &project,
            SearchQuery::text("TWO", false, true, false, Vec::new(), Vec::new()).unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            ("dir/two.rs".to_string(), vec![6..9]),
            ("dir/three.rs".to_string(), vec![37..40])
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
        buffer.edit([(20..28, text), (31..43, text)], None, cx);
    });

    assert_eq!(
        search(
            &project,
            SearchQuery::text("TWO", false, true, false, Vec::new(), Vec::new()).unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            ("dir/two.rs".to_string(), vec![6..9]),
            ("dir/three.rs".to_string(), vec![37..40]),
            ("dir/four.rs".to_string(), vec![25..28, 36..39])
        ])
    );
}

#[gpui::test]
async fn test_search_with_inclusions(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let search_query = "file";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "one.rs": r#"// Rust file one"#,
            "one.ts": r#"// TypeScript file one"#,
            "two.rs": r#"// Rust file two"#,
            "two.ts": r#"// TypeScript file two"#,
        }),
    )
    .await;
    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    assert!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                vec![PathMatcher::new("*.odd").unwrap()],
                Vec::new()
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap()
        .is_empty(),
        "If no inclusions match, no files should be returned"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                vec![PathMatcher::new("*.rs").unwrap()],
                Vec::new()
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            ("dir/one.rs".to_string(), vec![8..12]),
            ("dir/two.rs".to_string(), vec![8..12]),
        ]),
        "Rust only search should give only Rust files"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                vec![
                    PathMatcher::new("*.ts").unwrap(),
                    PathMatcher::new("*.odd").unwrap(),
                ],
                Vec::new()
            ).unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            ("dir/one.ts".to_string(), vec![14..18]),
            ("dir/two.ts".to_string(), vec![14..18]),
        ]),
        "TypeScript only search should give only TypeScript files, even if other inclusions don't match anything"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                vec![
                    PathMatcher::new("*.rs").unwrap(),
                    PathMatcher::new("*.ts").unwrap(),
                    PathMatcher::new("*.odd").unwrap(),
                ],
                Vec::new()
            ).unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            ("dir/two.ts".to_string(), vec![14..18]),
            ("dir/one.rs".to_string(), vec![8..12]),
            ("dir/one.ts".to_string(), vec![14..18]),
            ("dir/two.rs".to_string(), vec![8..12]),
        ]),
        "Rust and typescript search should give both Rust and TypeScript files, even if other inclusions don't match anything"
    );
}

#[gpui::test]
async fn test_search_with_exclusions(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let search_query = "file";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "one.rs": r#"// Rust file one"#,
            "one.ts": r#"// TypeScript file one"#,
            "two.rs": r#"// Rust file two"#,
            "two.ts": r#"// TypeScript file two"#,
        }),
    )
    .await;
    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                Vec::new(),
                vec![PathMatcher::new("*.odd").unwrap()],
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            ("dir/one.rs".to_string(), vec![8..12]),
            ("dir/one.ts".to_string(), vec![14..18]),
            ("dir/two.rs".to_string(), vec![8..12]),
            ("dir/two.ts".to_string(), vec![14..18]),
        ]),
        "If no exclusions match, all files should be returned"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                Vec::new(),
                vec![PathMatcher::new("*.rs").unwrap()],
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            ("dir/one.ts".to_string(), vec![14..18]),
            ("dir/two.ts".to_string(), vec![14..18]),
        ]),
        "Rust exclusion search should give only TypeScript files"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                Vec::new(),
                vec![
                    PathMatcher::new("*.ts").unwrap(),
                    PathMatcher::new("*.odd").unwrap(),
                ],
            ).unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            ("dir/one.rs".to_string(), vec![8..12]),
            ("dir/two.rs".to_string(), vec![8..12]),
        ]),
        "TypeScript exclusion search should give only Rust files, even if other exclusions don't match anything"
    );

    assert!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                Vec::new(),
                vec![
                    PathMatcher::new("*.rs").unwrap(),
                    PathMatcher::new("*.ts").unwrap(),
                    PathMatcher::new("*.odd").unwrap(),
                ],
            ).unwrap(),
            cx
        )
        .await
        .unwrap().is_empty(),
        "Rust and typescript exclusion should give no files, even if other exclusions don't match anything"
    );
}

#[gpui::test]
async fn test_search_with_exclusions_and_inclusions(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let search_query = "file";

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "one.rs": r#"// Rust file one"#,
            "one.ts": r#"// TypeScript file one"#,
            "two.rs": r#"// Rust file two"#,
            "two.ts": r#"// TypeScript file two"#,
        }),
    )
    .await;
    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    assert!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                vec![PathMatcher::new("*.odd").unwrap()],
                vec![PathMatcher::new("*.odd").unwrap()],
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap()
        .is_empty(),
        "If both no exclusions and inclusions match, exclusions should win and return nothing"
    );

    assert!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                vec![PathMatcher::new("*.ts").unwrap()],
                vec![PathMatcher::new("*.ts").unwrap()],
            ).unwrap(),
            cx
        )
        .await
        .unwrap()
        .is_empty(),
        "If both TypeScript exclusions and inclusions match, exclusions should win and return nothing files."
    );

    assert!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                vec![
                    PathMatcher::new("*.ts").unwrap(),
                    PathMatcher::new("*.odd").unwrap()
                ],
                vec![
                    PathMatcher::new("*.ts").unwrap(),
                    PathMatcher::new("*.odd").unwrap()
                ],
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap()
        .is_empty(),
        "Non-matching inclusions and exclusions should not change that."
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                search_query,
                false,
                true,
                false,
                vec![
                    PathMatcher::new("*.ts").unwrap(),
                    PathMatcher::new("*.odd").unwrap()
                ],
                vec![
                    PathMatcher::new("*.rs").unwrap(),
                    PathMatcher::new("*.odd").unwrap()
                ],
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            ("dir/one.ts".to_string(), vec![14..18]),
            ("dir/two.ts".to_string(), vec![14..18]),
        ]),
        "Non-intersecting TypeScript inclusions and Rust exclusions should return TypeScript files"
    );
}

#[gpui::test]
async fn test_search_multiple_worktrees_with_inclusions(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/worktree-a",
        json!({
            "haystack.rs": r#"// NEEDLE"#,
            "haystack.ts": r#"// NEEDLE"#,
        }),
    )
    .await;
    fs.insert_tree(
        "/worktree-b",
        json!({
            "haystack.rs": r#"// NEEDLE"#,
            "haystack.ts": r#"// NEEDLE"#,
        }),
    )
    .await;

    let project = Project::test(
        fs.clone(),
        ["/worktree-a".as_ref(), "/worktree-b".as_ref()],
        cx,
    )
    .await;

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                "NEEDLE",
                false,
                true,
                false,
                vec![PathMatcher::new("worktree-a/*.rs").unwrap()],
                Vec::new()
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([("worktree-a/haystack.rs".to_string(), vec![3..9])]),
        "should only return results from included worktree"
    );
    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                "NEEDLE",
                false,
                true,
                false,
                vec![PathMatcher::new("worktree-b/*.rs").unwrap()],
                Vec::new()
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([("worktree-b/haystack.rs".to_string(), vec![3..9])]),
        "should only return results from included worktree"
    );

    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                "NEEDLE",
                false,
                true,
                false,
                vec![PathMatcher::new("*.ts").unwrap()],
                Vec::new()
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            ("worktree-a/haystack.ts".to_string(), vec![3..9]),
            ("worktree-b/haystack.ts".to_string(), vec![3..9])
        ]),
        "should return results from both worktrees"
    );
}

#[gpui::test]
async fn test_search_in_gitignored_dirs(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/dir",
        json!({
            ".git": {},
            ".gitignore": "**/target\n/node_modules\n",
            "target": {
                "index.txt": "index_key:index_value"
            },
            "node_modules": {
                "eslint": {
                    "index.ts": "const eslint_key = 'eslint value'",
                    "package.json": r#"{ "some_key": "some value" }"#,
                },
                "prettier": {
                    "index.ts": "const prettier_key = 'prettier value'",
                    "package.json": r#"{ "other_key": "other value" }"#,
                },
            },
            "package.json": r#"{ "main_key": "main value" }"#,
        }),
    )
    .await;
    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

    let query = "key";
    assert_eq!(
        search(
            &project,
            SearchQuery::text(query, false, false, false, Vec::new(), Vec::new()).unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([("dir/package.json".to_string(), vec![8..11])]),
        "Only one non-ignored file should have the query"
    );

    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
    assert_eq!(
        search(
            &project,
            SearchQuery::text(query, false, false, true, Vec::new(), Vec::new()).unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([
            ("dir/package.json".to_string(), vec![8..11]),
            ("dir/target/index.txt".to_string(), vec![6..9]),
            (
                "dir/node_modules/prettier/package.json".to_string(),
                vec![9..12]
            ),
            (
                "dir/node_modules/prettier/index.ts".to_string(),
                vec![15..18]
            ),
            ("dir/node_modules/eslint/index.ts".to_string(), vec![13..16]),
            (
                "dir/node_modules/eslint/package.json".to_string(),
                vec![8..11]
            ),
        ]),
        "Unrestricted search with ignored directories should find every file with the query"
    );

    let files_to_include = vec![PathMatcher::new("/dir/node_modules/prettier/**").unwrap()];
    let files_to_exclude = vec![PathMatcher::new("*.ts").unwrap()];
    let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
    assert_eq!(
        search(
            &project,
            SearchQuery::text(
                query,
                false,
                false,
                true,
                files_to_include,
                files_to_exclude,
            )
            .unwrap(),
            cx
        )
        .await
        .unwrap(),
        HashMap::from_iter([(
            "dir/node_modules/prettier/package.json".to_string(),
            vec![9..12]
        )]),
        "With search including ignored prettier directory and excluding TS files, only one file should be found"
    );
}

#[test]
fn test_glob_literal_prefix() {
    assert_eq!(glob_literal_prefix("**/*.js"), "");
    assert_eq!(glob_literal_prefix("node_modules/**/*.js"), "node_modules");
    assert_eq!(glob_literal_prefix("foo/{bar,baz}.js"), "foo");
    assert_eq!(glob_literal_prefix("foo/bar/baz.js"), "foo/bar/baz.js");
}

#[gpui::test]
async fn test_create_entry(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
    fs.insert_tree(
        "/one/two",
        json!({
            "three": {
                "a.txt": "",
                "four": {}
            },
            "c.rs": ""
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/one/two/three".as_ref()], cx).await;
    project
        .update(cx, |project, cx| {
            let id = project.worktrees().next().unwrap().read(cx).id();
            project.create_entry((id, "b.."), true, cx)
        })
        .unwrap()
        .await
        .unwrap();

    // Can't create paths outside the project
    let result = project
        .update(cx, |project, cx| {
            let id = project.worktrees().next().unwrap().read(cx).id();
            project.create_entry((id, "../../boop"), true, cx)
        })
        .await;
    assert!(result.is_err());

    // Can't create paths with '..'
    let result = project
        .update(cx, |project, cx| {
            let id = project.worktrees().next().unwrap().read(cx).id();
            project.create_entry((id, "four/../beep"), true, cx)
        })
        .await;
    assert!(result.is_err());

    assert_eq!(
        fs.paths(true),
        vec![
            PathBuf::from("/"),
            PathBuf::from("/one"),
            PathBuf::from("/one/two"),
            PathBuf::from("/one/two/c.rs"),
            PathBuf::from("/one/two/three"),
            PathBuf::from("/one/two/three/a.txt"),
            PathBuf::from("/one/two/three/b.."),
            PathBuf::from("/one/two/three/four"),
        ]
    );

    // And we cannot open buffers with '..'
    let result = project
        .update(cx, |project, cx| {
            let id = project.worktrees().next().unwrap().read(cx).id();
            project.open_buffer((id, "../c.rs"), cx)
        })
        .await;
    assert!(result.is_err())
}

#[gpui::test]
async fn test_multiple_language_server_hovers(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a.tsx": "a",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(tsx_lang());
    let language_server_names = [
        "TypeScriptServer",
        "TailwindServer",
        "ESLintServer",
        "NoHoverCapabilitiesServer",
    ];
    let mut fake_tsx_language_servers = language_registry.register_specific_fake_lsp_adapter(
        "tsx",
        true,
        FakeLspAdapter {
            name: &language_server_names[0],
            capabilities: lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );
    let _a = language_registry.register_specific_fake_lsp_adapter(
        "tsx",
        false,
        FakeLspAdapter {
            name: &language_server_names[1],
            capabilities: lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );
    let _b = language_registry.register_specific_fake_lsp_adapter(
        "tsx",
        false,
        FakeLspAdapter {
            name: &language_server_names[2],
            capabilities: lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );
    let _c = language_registry.register_specific_fake_lsp_adapter(
        "tsx",
        false,
        FakeLspAdapter {
            name: &language_server_names[3],
            capabilities: lsp::ServerCapabilities {
                hover_provider: None,
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );

    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/a.tsx", cx))
        .await
        .unwrap();
    cx.executor().run_until_parked();

    let mut servers_with_hover_requests = HashMap::default();
    for i in 0..language_server_names.len() {
        let new_server = fake_tsx_language_servers.next().await.unwrap_or_else(|| {
            panic!(
                "Failed to get language server #{i} with name {}",
                &language_server_names[i]
            )
        });
        let new_server_name = new_server.server.name();
        assert!(
            !servers_with_hover_requests.contains_key(new_server_name),
            "Unexpected: initialized server with the same name twice. Name: `{new_server_name}`"
        );
        let new_server_name = new_server_name.to_string();
        match new_server_name.as_str() {
            "TailwindServer" | "TypeScriptServer" => {
                servers_with_hover_requests.insert(
                    new_server_name.clone(),
                    new_server.handle_request::<lsp::request::HoverRequest, _, _>(move |_, _| {
                        let name = new_server_name.clone();
                        async move {
                            Ok(Some(lsp::Hover {
                                contents: lsp::HoverContents::Scalar(lsp::MarkedString::String(
                                    format!("{name} hover"),
                                )),
                                range: None,
                            }))
                        }
                    }),
                );
            }
            "ESLintServer" => {
                servers_with_hover_requests.insert(
                    new_server_name,
                    new_server.handle_request::<lsp::request::HoverRequest, _, _>(
                        |_, _| async move { Ok(None) },
                    ),
                );
            }
            "NoHoverCapabilitiesServer" => {
                let _never_handled = new_server.handle_request::<lsp::request::HoverRequest, _, _>(
                    |_, _| async move {
                        panic!(
                            "Should not call for hovers server with no corresponding capabilities"
                        )
                    },
                );
            }
            unexpected => panic!("Unexpected server name: {unexpected}"),
        }
    }

    let hover_task = project.update(cx, |project, cx| {
        project.hover(&buffer, Point::new(0, 0), cx)
    });
    let _: Vec<()> = futures::future::join_all(servers_with_hover_requests.into_values().map(
        |mut hover_request| async move {
            hover_request
                .next()
                .await
                .expect("All hover requests should have been triggered")
        },
    ))
    .await;
    assert_eq!(
        vec!["TailwindServer hover", "TypeScriptServer hover"],
        hover_task
            .await
            .into_iter()
            .map(|hover| hover.contents.iter().map(|block| &block.text).join("|"))
            .sorted()
            .collect::<Vec<_>>(),
        "Should receive hover responses from all related servers with hover capabilities"
    );
}

#[gpui::test]
async fn test_hovers_with_empty_parts(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a.ts": "a",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(typescript_lang());
    let mut fake_language_servers = language_registry.register_fake_lsp_adapter(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );

    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/a.ts", cx))
        .await
        .unwrap();
    cx.executor().run_until_parked();

    let fake_server = fake_language_servers
        .next()
        .await
        .expect("failed to get the language server");

    let mut request_handled =
        fake_server.handle_request::<lsp::request::HoverRequest, _, _>(move |_, _| async move {
            Ok(Some(lsp::Hover {
                contents: lsp::HoverContents::Array(vec![
                    lsp::MarkedString::String("".to_string()),
                    lsp::MarkedString::String("      ".to_string()),
                    lsp::MarkedString::String("\n\n\n".to_string()),
                ]),
                range: None,
            }))
        });

    let hover_task = project.update(cx, |project, cx| {
        project.hover(&buffer, Point::new(0, 0), cx)
    });
    let () = request_handled
        .next()
        .await
        .expect("All hover requests should have been triggered");
    assert_eq!(
        Vec::<String>::new(),
        hover_task
            .await
            .into_iter()
            .map(|hover| hover.contents.iter().map(|block| &block.text).join("|"))
            .sorted()
            .collect::<Vec<_>>(),
        "Empty hover parts should be ignored"
    );
}

#[gpui::test]
async fn test_multiple_language_server_actions(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a.tsx": "a",
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(tsx_lang());
    let language_server_names = [
        "TypeScriptServer",
        "TailwindServer",
        "ESLintServer",
        "NoActionsCapabilitiesServer",
    ];
    let mut fake_tsx_language_servers = language_registry.register_specific_fake_lsp_adapter(
        "tsx",
        true,
        FakeLspAdapter {
            name: &language_server_names[0],
            capabilities: lsp::ServerCapabilities {
                code_action_provider: Some(lsp::CodeActionProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );
    let _a = language_registry.register_specific_fake_lsp_adapter(
        "tsx",
        false,
        FakeLspAdapter {
            name: &language_server_names[1],
            capabilities: lsp::ServerCapabilities {
                code_action_provider: Some(lsp::CodeActionProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );
    let _b = language_registry.register_specific_fake_lsp_adapter(
        "tsx",
        false,
        FakeLspAdapter {
            name: &language_server_names[2],
            capabilities: lsp::ServerCapabilities {
                code_action_provider: Some(lsp::CodeActionProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );
    let _c = language_registry.register_specific_fake_lsp_adapter(
        "tsx",
        false,
        FakeLspAdapter {
            name: &language_server_names[3],
            capabilities: lsp::ServerCapabilities {
                code_action_provider: None,
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );

    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer("/dir/a.tsx", cx))
        .await
        .unwrap();
    cx.executor().run_until_parked();

    let mut servers_with_actions_requests = HashMap::default();
    for i in 0..language_server_names.len() {
        let new_server = fake_tsx_language_servers.next().await.unwrap_or_else(|| {
            panic!(
                "Failed to get language server #{i} with name {}",
                &language_server_names[i]
            )
        });
        let new_server_name = new_server.server.name();
        assert!(
            !servers_with_actions_requests.contains_key(new_server_name),
            "Unexpected: initialized server with the same name twice. Name: `{new_server_name}`"
        );
        let new_server_name = new_server_name.to_string();
        match new_server_name.as_str() {
            "TailwindServer" | "TypeScriptServer" => {
                servers_with_actions_requests.insert(
                    new_server_name.clone(),
                    new_server.handle_request::<lsp::request::CodeActionRequest, _, _>(
                        move |_, _| {
                            let name = new_server_name.clone();
                            async move {
                                Ok(Some(vec![lsp::CodeActionOrCommand::CodeAction(
                                    lsp::CodeAction {
                                        title: format!("{name} code action"),
                                        ..lsp::CodeAction::default()
                                    },
                                )]))
                            }
                        },
                    ),
                );
            }
            "ESLintServer" => {
                servers_with_actions_requests.insert(
                    new_server_name,
                    new_server.handle_request::<lsp::request::CodeActionRequest, _, _>(
                        |_, _| async move { Ok(None) },
                    ),
                );
            }
            "NoActionsCapabilitiesServer" => {
                let _never_handled = new_server
                    .handle_request::<lsp::request::CodeActionRequest, _, _>(|_, _| async move {
                        panic!(
                            "Should not call for code actions server with no corresponding capabilities"
                        )
                    });
            }
            unexpected => panic!("Unexpected server name: {unexpected}"),
        }
    }

    let code_actions_task = project.update(cx, |project, cx| {
        project.code_actions(&buffer, 0..buffer.read(cx).len(), cx)
    });
    let _: Vec<()> = futures::future::join_all(servers_with_actions_requests.into_values().map(
        |mut code_actions_request| async move {
            code_actions_request
                .next()
                .await
                .expect("All code actions requests should have been triggered")
        },
    ))
    .await;
    assert_eq!(
        vec!["TailwindServer code action", "TypeScriptServer code action"],
        code_actions_task
            .await
            .into_iter()
            .map(|code_action| code_action.lsp_action.title)
            .sorted()
            .collect::<Vec<_>>(),
        "Should receive code actions responses from all related servers with hover capabilities"
    );
}

#[gpui::test]
async fn test_reordering_worktrees(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/dir",
        json!({
            "a.rs": "let a = 1;",
            "b.rs": "let b = 2;",
            "c.rs": "let c = 2;",
        }),
    )
    .await;

    let project = Project::test(
        fs,
        [
            "/dir/a.rs".as_ref(),
            "/dir/b.rs".as_ref(),
            "/dir/c.rs".as_ref(),
        ],
        cx,
    )
    .await;

    // check the initial state and get the worktrees
    let (worktree_a, worktree_b, worktree_c) = project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let worktree_a = worktrees[0].read(cx);
        let worktree_b = worktrees[1].read(cx);
        let worktree_c = worktrees[2].read(cx);

        // check they start in the right order
        assert_eq!(worktree_a.abs_path().to_str().unwrap(), "/dir/a.rs");
        assert_eq!(worktree_b.abs_path().to_str().unwrap(), "/dir/b.rs");
        assert_eq!(worktree_c.abs_path().to_str().unwrap(), "/dir/c.rs");

        (
            worktrees[0].clone(),
            worktrees[1].clone(),
            worktrees[2].clone(),
        )
    });

    // move first worktree to after the second
    // [a, b, c] -> [b, a, c]
    project
        .update(cx, |project, cx| {
            let first = worktree_a.read(cx);
            let second = worktree_b.read(cx);
            project.move_worktree(first.id(), second.id(), cx)
        })
        .expect("moving first after second");

    // check the state after moving
    project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let first = worktrees[0].read(cx);
        let second = worktrees[1].read(cx);
        let third = worktrees[2].read(cx);

        // check they are now in the right order
        assert_eq!(first.abs_path().to_str().unwrap(), "/dir/b.rs");
        assert_eq!(second.abs_path().to_str().unwrap(), "/dir/a.rs");
        assert_eq!(third.abs_path().to_str().unwrap(), "/dir/c.rs");
    });

    // move the second worktree to before the first
    // [b, a, c] -> [a, b, c]
    project
        .update(cx, |project, cx| {
            let second = worktree_a.read(cx);
            let first = worktree_b.read(cx);
            project.move_worktree(first.id(), second.id(), cx)
        })
        .expect("moving second before first");

    // check the state after moving
    project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let first = worktrees[0].read(cx);
        let second = worktrees[1].read(cx);
        let third = worktrees[2].read(cx);

        // check they are now in the right order
        assert_eq!(first.abs_path().to_str().unwrap(), "/dir/a.rs");
        assert_eq!(second.abs_path().to_str().unwrap(), "/dir/b.rs");
        assert_eq!(third.abs_path().to_str().unwrap(), "/dir/c.rs");
    });

    // move the second worktree to after the third
    // [a, b, c] -> [a, c, b]
    project
        .update(cx, |project, cx| {
            let second = worktree_b.read(cx);
            let third = worktree_c.read(cx);
            project.move_worktree(second.id(), third.id(), cx)
        })
        .expect("moving second after third");

    // check the state after moving
    project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let first = worktrees[0].read(cx);
        let second = worktrees[1].read(cx);
        let third = worktrees[2].read(cx);

        // check they are now in the right order
        assert_eq!(first.abs_path().to_str().unwrap(), "/dir/a.rs");
        assert_eq!(second.abs_path().to_str().unwrap(), "/dir/c.rs");
        assert_eq!(third.abs_path().to_str().unwrap(), "/dir/b.rs");
    });

    // move the third worktree to before the second
    // [a, c, b] -> [a, b, c]
    project
        .update(cx, |project, cx| {
            let third = worktree_c.read(cx);
            let second = worktree_b.read(cx);
            project.move_worktree(third.id(), second.id(), cx)
        })
        .expect("moving third before second");

    // check the state after moving
    project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let first = worktrees[0].read(cx);
        let second = worktrees[1].read(cx);
        let third = worktrees[2].read(cx);

        // check they are now in the right order
        assert_eq!(first.abs_path().to_str().unwrap(), "/dir/a.rs");
        assert_eq!(second.abs_path().to_str().unwrap(), "/dir/b.rs");
        assert_eq!(third.abs_path().to_str().unwrap(), "/dir/c.rs");
    });

    // move the first worktree to after the third
    // [a, b, c] -> [b, c, a]
    project
        .update(cx, |project, cx| {
            let first = worktree_a.read(cx);
            let third = worktree_c.read(cx);
            project.move_worktree(first.id(), third.id(), cx)
        })
        .expect("moving first after third");

    // check the state after moving
    project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let first = worktrees[0].read(cx);
        let second = worktrees[1].read(cx);
        let third = worktrees[2].read(cx);

        // check they are now in the right order
        assert_eq!(first.abs_path().to_str().unwrap(), "/dir/b.rs");
        assert_eq!(second.abs_path().to_str().unwrap(), "/dir/c.rs");
        assert_eq!(third.abs_path().to_str().unwrap(), "/dir/a.rs");
    });

    // move the third worktree to before the first
    // [b, c, a] -> [a, b, c]
    project
        .update(cx, |project, cx| {
            let third = worktree_a.read(cx);
            let first = worktree_b.read(cx);
            project.move_worktree(third.id(), first.id(), cx)
        })
        .expect("moving third before first");

    // check the state after moving
    project.update(cx, |project, cx| {
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 3);

        let first = worktrees[0].read(cx);
        let second = worktrees[1].read(cx);
        let third = worktrees[2].read(cx);

        // check they are now in the right order
        assert_eq!(first.abs_path().to_str().unwrap(), "/dir/a.rs");
        assert_eq!(second.abs_path().to_str().unwrap(), "/dir/b.rs");
        assert_eq!(third.abs_path().to_str().unwrap(), "/dir/c.rs");
    });
}

async fn search(
    project: &Model<Project>,
    query: SearchQuery,
    cx: &mut gpui::TestAppContext,
) -> Result<HashMap<String, Vec<Range<usize>>>> {
    let mut search_rx = project.update(cx, |project, cx| project.search(query, cx));
    let mut results = HashMap::default();
    while let Some(search_result) = search_rx.next().await {
        match search_result {
            SearchResult::Buffer { buffer, ranges } => {
                results.entry(buffer).or_insert(ranges);
            }
            SearchResult::LimitReached => {}
        }
    }
    Ok(results
        .into_iter()
        .map(|(buffer, ranges)| {
            buffer.update(cx, |buffer, cx| {
                let path = buffer
                    .file()
                    .unwrap()
                    .full_path(cx)
                    .to_string_lossy()
                    .to_string();
                let ranges = ranges
                    .into_iter()
                    .map(|range| range.to_offset(buffer))
                    .collect::<Vec<_>>();
                (path, ranges)
            })
        })
        .collect())
}

fn init_test(cx: &mut gpui::TestAppContext) {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::try_init().ok();
    }

    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        release_channel::init("0.0.0", cx);
        language::init(cx);
        Project::init_settings(cx);
    });
}

fn json_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "JSON".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["json".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        None,
    ))
}

fn js_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: Arc::from("JavaScript"),
            matcher: LanguageMatcher {
                path_suffixes: vec!["js".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        None,
    ))
}

fn rust_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    ))
}

fn typescript_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "TypeScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["ts".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_typescript::language_typescript()),
    ))
}

fn tsx_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "tsx".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["tsx".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_typescript::language_tsx()),
    ))
}

fn get_all_tasks(
    project: &Model<Project>,
    worktree_id: Option<WorktreeId>,
    task_context: &TaskContext,
    cx: &mut AppContext,
) -> Task<Vec<(TaskSourceKind, ResolvedTask)>> {
    let resolved_tasks = project.update(cx, |project, cx| {
        project
            .task_inventory()
            .read(cx)
            .used_and_current_resolved_tasks(None, worktree_id, None, task_context, cx)
    });

    cx.spawn(|_| async move {
        let (mut old, new) = resolved_tasks.await;
        old.extend(new);
        old
    })
}
