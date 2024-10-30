use crate::headless_project::HeadlessProject;
use client::{Client, UserStore};
use clock::FakeSystemClock;
use fs::{FakeFs, Fs};
use gpui::{Context, Model, TestAppContext};
use http_client::{BlockedHttpClient, FakeHttpClient};
use language::{
    language_settings::{language_settings, AllLanguageSettings},
    Buffer, FakeLspAdapter, LanguageConfig, LanguageMatcher, LanguageRegistry, LanguageServerName,
    LineEnding,
};
use lsp::{CompletionContext, CompletionResponse, CompletionTriggerKind};
use node_runtime::NodeRuntime;
use project::{
    search::{SearchQuery, SearchResult},
    Project, ProjectPath,
};
use remote::SshRemoteClient;
use serde_json::json;
use settings::{initial_server_settings_content, Settings, SettingsLocation, SettingsStore};
use smol::stream::StreamExt;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[gpui::test]
async fn test_basic_remote_editing(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }"
                }
            },
            "project2": {
                "README.md": "# project 2",
            },
        }),
    )
    .await;
    fs.set_index_for_repo(
        Path::new("/code/project1/.git"),
        &[(Path::new("src/lib.rs"), "fn one() -> usize { 0 }".into())],
    );

    let (project, _headless) = init_test(&fs, cx, server_cx).await;
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap();

    // The client sees the worktree's contents.
    cx.executor().run_until_parked();
    let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());
    worktree.update(cx, |worktree, _cx| {
        assert_eq!(
            worktree.paths().map(Arc::as_ref).collect::<Vec<_>>(),
            vec![
                Path::new("README.md"),
                Path::new("src"),
                Path::new("src/lib.rs"),
            ]
        );
    });

    // The user opens a buffer in the remote worktree. The buffer's
    // contents are loaded from the remote filesystem.
    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, Path::new("src/lib.rs")), cx)
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        assert_eq!(buffer.text(), "fn one() -> usize { 1 }");
        assert_eq!(
            buffer.diff_base().unwrap().to_string(),
            "fn one() -> usize { 0 }"
        );
        let ix = buffer.text().find('1').unwrap();
        buffer.edit([(ix..ix + 1, "100")], None, cx);
    });

    // The user saves the buffer. The new contents are written to the
    // remote filesystem.
    project
        .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
        .await
        .unwrap();
    assert_eq!(
        fs.load("/code/project1/src/lib.rs".as_ref()).await.unwrap(),
        "fn one() -> usize { 100 }"
    );

    // A new file is created in the remote filesystem. The user
    // sees the new file.
    fs.save(
        "/code/project1/src/main.rs".as_ref(),
        &"fn main() {}".into(),
        Default::default(),
    )
    .await
    .unwrap();
    cx.executor().run_until_parked();
    worktree.update(cx, |worktree, _cx| {
        assert_eq!(
            worktree.paths().map(Arc::as_ref).collect::<Vec<_>>(),
            vec![
                Path::new("README.md"),
                Path::new("src"),
                Path::new("src/lib.rs"),
                Path::new("src/main.rs"),
            ]
        );
    });

    // A file that is currently open in a buffer is renamed.
    fs.rename(
        "/code/project1/src/lib.rs".as_ref(),
        "/code/project1/src/lib2.rs".as_ref(),
        Default::default(),
    )
    .await
    .unwrap();
    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(&**buffer.file().unwrap().path(), Path::new("src/lib2.rs"));
    });

    fs.set_index_for_repo(
        Path::new("/code/project1/.git"),
        &[(Path::new("src/lib2.rs"), "fn one() -> usize { 100 }".into())],
    );
    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer.diff_base().unwrap().to_string(),
            "fn one() -> usize { 100 }"
        );
    });
}

#[gpui::test]
async fn test_remote_project_search(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }"
                }
            },
        }),
    )
    .await;

    let (project, headless) = init_test(&fs, cx, server_cx).await;

    project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();

    async fn do_search(project: &Model<Project>, mut cx: TestAppContext) -> Model<Buffer> {
        let mut receiver = project.update(&mut cx, |project, cx| {
            project.search(
                SearchQuery::text(
                    "project",
                    false,
                    true,
                    false,
                    Default::default(),
                    Default::default(),
                    None,
                )
                .unwrap(),
                cx,
            )
        });

        let first_response = receiver.next().await.unwrap();
        let SearchResult::Buffer { buffer, .. } = first_response else {
            panic!("incorrect result");
        };
        buffer.update(&mut cx, |buffer, cx| {
            assert_eq!(
                buffer.file().unwrap().full_path(cx).to_string_lossy(),
                "project1/README.md"
            )
        });

        assert!(receiver.next().await.is_none());
        buffer
    }

    let buffer = do_search(&project, cx.clone()).await;

    // test that the headless server is tracking which buffers we have open correctly.
    cx.run_until_parked();
    headless.update(server_cx, |headless, cx| {
        assert!(!headless.buffer_store.read(cx).shared_buffers().is_empty())
    });
    do_search(&project, cx.clone()).await;

    cx.update(|_| {
        drop(buffer);
    });
    cx.run_until_parked();
    headless.update(server_cx, |headless, cx| {
        assert!(headless.buffer_store.read(cx).shared_buffers().is_empty())
    });

    do_search(&project, cx.clone()).await;
}

#[gpui::test]
async fn test_remote_settings(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }"
                }
            },
        }),
    )
    .await;

    let (project, headless) = init_test(&fs, cx, server_cx).await;

    cx.update_global(|settings_store: &mut SettingsStore, cx| {
        settings_store.set_user_settings(
            r#"{"languages":{"Rust":{"language_servers":["from-local-settings"]}}}"#,
            cx,
        )
    })
    .unwrap();

    cx.run_until_parked();

    server_cx.read(|cx| {
        assert_eq!(
            AllLanguageSettings::get_global(cx)
                .language(None, Some(&"Rust".into()), cx)
                .language_servers,
            ["..."] // local settings are ignored
        )
    });

    server_cx
        .update_global(|settings_store: &mut SettingsStore, cx| {
            settings_store.set_server_settings(
                r#"{"languages":{"Rust":{"language_servers":["from-server-settings"]}}}"#,
                cx,
            )
        })
        .unwrap();

    cx.run_until_parked();

    server_cx.read(|cx| {
        assert_eq!(
            AllLanguageSettings::get_global(cx)
                .language(None, Some(&"Rust".into()), cx)
                .language_servers,
            ["from-server-settings".to_string()]
        )
    });

    fs.insert_tree(
        "/code/project1/.zed",
        json!({
            "settings.json": r#"
                  {
                    "languages": {"Rust":{"language_servers":["override-rust-analyzer"]}},
                    "lsp": {
                      "override-rust-analyzer": {
                        "binary": {
                          "path": "~/.cargo/bin/rust-analyzer"
                        }
                      }
                    }
                  }"#
        }),
    )
    .await;

    let worktree_id = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap()
        .0
        .read_with(cx, |worktree, _| worktree.id());

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, Path::new("src/lib.rs")), cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();

    server_cx.read(|cx| {
        let worktree_id = headless
            .read(cx)
            .worktree_store
            .read(cx)
            .worktrees()
            .next()
            .unwrap()
            .read(cx)
            .id();
        assert_eq!(
            AllLanguageSettings::get(
                Some(SettingsLocation {
                    worktree_id,
                    path: Path::new("src/lib.rs")
                }),
                cx
            )
            .language(None, Some(&"Rust".into()), cx)
            .language_servers,
            ["override-rust-analyzer".to_string()]
        )
    });

    cx.read(|cx| {
        let file = buffer.read(cx).file();
        assert_eq!(
            language_settings(Some("Rust".into()), file, cx).language_servers,
            ["override-rust-analyzer".to_string()]
        )
    });
}

#[gpui::test]
async fn test_remote_lsp(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }"
                }
            },
        }),
    )
    .await;

    let (project, headless) = init_test(&fs, cx, server_cx).await;

    fs.insert_tree(
        "/code/project1/.zed",
        json!({
            "settings.json": r#"
          {
            "languages": {"Rust":{"language_servers":["rust-analyzer"]}},
            "lsp": {
              "rust-analyzer": {
                "binary": {
                  "path": "~/.cargo/bin/rust-analyzer"
                }
              }
            }
          }"#
        }),
    )
    .await;

    cx.update_model(&project, |project, _| {
        project.languages().register_test_language(LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".into()],
                ..Default::default()
            },
            ..Default::default()
        });
        project.languages().register_fake_lsp_adapter(
            "Rust",
            FakeLspAdapter {
                name: "rust-analyzer",
                ..Default::default()
            },
        )
    });

    let mut fake_lsp = server_cx.update(|cx| {
        headless.read(cx).languages.register_fake_language_server(
            LanguageServerName("rust-analyzer".into()),
            Default::default(),
            None,
        )
    });

    cx.run_until_parked();

    let worktree_id = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap()
        .0
        .read_with(cx, |worktree, _| worktree.id());

    // Wait for the settings to synchronize
    cx.run_until_parked();

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, Path::new("src/lib.rs")), cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();

    let fake_lsp = fake_lsp.next().await.unwrap();

    cx.read(|cx| {
        let file = buffer.read(cx).file();
        assert_eq!(
            language_settings(Some("Rust".into()), file, cx).language_servers,
            ["rust-analyzer".to_string()]
        )
    });

    let buffer_id = cx.read(|cx| {
        let buffer = buffer.read(cx);
        assert_eq!(buffer.language().unwrap().name(), "Rust".into());
        buffer.remote_id()
    });

    server_cx.read(|cx| {
        let buffer = headless
            .read(cx)
            .buffer_store
            .read(cx)
            .get(buffer_id)
            .unwrap();

        assert_eq!(buffer.read(cx).language().unwrap().name(), "Rust".into());
    });

    server_cx.read(|cx| {
        let lsp_store = headless.read(cx).lsp_store.read(cx);
        assert_eq!(lsp_store.as_local().unwrap().language_servers.len(), 1);
    });

    fake_lsp.handle_request::<lsp::request::Completion, _, _>(|_, _| async move {
        Ok(Some(CompletionResponse::Array(vec![lsp::CompletionItem {
            label: "boop".to_string(),
            ..Default::default()
        }])))
    });

    let result = project
        .update(cx, |project, cx| {
            project.completions(
                &buffer,
                0,
                CompletionContext {
                    trigger_kind: CompletionTriggerKind::INVOKED,
                    trigger_character: None,
                },
                cx,
            )
        })
        .await
        .unwrap();

    assert_eq!(
        result.into_iter().map(|c| c.label.text).collect::<Vec<_>>(),
        vec!["boop".to_string()]
    );

    fake_lsp.handle_request::<lsp::request::Rename, _, _>(|_, _| async move {
        Ok(Some(lsp::WorkspaceEdit {
            changes: Some(
                [(
                    lsp::Url::from_file_path("/code/project1/src/lib.rs").unwrap(),
                    vec![lsp::TextEdit::new(
                        lsp::Range::new(lsp::Position::new(0, 3), lsp::Position::new(0, 6)),
                        "two".to_string(),
                    )],
                )]
                .into_iter()
                .collect(),
            ),
            ..Default::default()
        }))
    });

    project
        .update(cx, |project, cx| {
            project.perform_rename(buffer.clone(), 3, "two".to_string(), cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), "fn two() -> usize { 1 }")
    })
}

#[gpui::test]
async fn test_remote_reload(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }"
                }
            },
        }),
    )
    .await;

    let (project, _headless) = init_test(&fs, cx, server_cx).await;
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap();

    let worktree_id = cx.update(|cx| worktree.read(cx).id());

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, Path::new("src/lib.rs")), cx)
        })
        .await
        .unwrap();

    fs.save(
        &PathBuf::from("/code/project1/src/lib.rs"),
        &("bangles".to_string().into()),
        LineEnding::Unix,
    )
    .await
    .unwrap();

    cx.run_until_parked();

    buffer.update(cx, |buffer, cx| {
        assert_eq!(buffer.text(), "bangles");
        buffer.edit([(0..0, "a")], None, cx);
    });

    fs.save(
        &PathBuf::from("/code/project1/src/lib.rs"),
        &("bloop".to_string().into()),
        LineEnding::Unix,
    )
    .await
    .unwrap();

    cx.run_until_parked();
    cx.update(|cx| {
        assert!(buffer.read(cx).has_conflict());
    });

    project
        .update(cx, |project, cx| {
            project.reload_buffers([buffer.clone()].into_iter().collect(), false, cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();

    cx.update(|cx| {
        assert!(!buffer.read(cx).has_conflict());
    });
}

#[gpui::test]
async fn test_remote_resolve_path_in_buffer(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }"
                }
            },
        }),
    )
    .await;

    let (project, _headless) = init_test(&fs, cx, server_cx).await;
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap();

    let worktree_id = cx.update(|cx| worktree.read(cx).id());

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, Path::new("src/lib.rs")), cx)
        })
        .await
        .unwrap();

    let path = project
        .update(cx, |project, cx| {
            project.resolve_path_in_buffer("/code/project1/README.md", &buffer, cx)
        })
        .await
        .unwrap();
    assert!(path.is_file());
    assert_eq!(
        path.abs_path().unwrap().to_string_lossy(),
        "/code/project1/README.md"
    );

    let path = project
        .update(cx, |project, cx| {
            project.resolve_path_in_buffer("../README.md", &buffer, cx)
        })
        .await
        .unwrap();
    assert!(path.is_file());
    assert_eq!(
        path.project_path().unwrap().clone(),
        ProjectPath::from((worktree_id, "README.md"))
    );

    let path = project
        .update(cx, |project, cx| {
            project.resolve_path_in_buffer("../src", &buffer, cx)
        })
        .await
        .unwrap();
    assert_eq!(
        path.project_path().unwrap().clone(),
        ProjectPath::from((worktree_id, "src"))
    );
    assert!(path.is_dir());
}

#[gpui::test]
async fn test_remote_resolve_abs_path(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }"
                }
            },
        }),
    )
    .await;

    let (project, _headless) = init_test(&fs, cx, server_cx).await;

    let path = project
        .update(cx, |project, cx| {
            project.resolve_abs_path("/code/project1/README.md", cx)
        })
        .await
        .unwrap();

    assert!(path.is_file());
    assert_eq!(
        path.abs_path().unwrap().to_string_lossy(),
        "/code/project1/README.md"
    );

    let path = project
        .update(cx, |project, cx| {
            project.resolve_abs_path("/code/project1/src", cx)
        })
        .await
        .unwrap();

    assert!(path.is_dir());
    assert_eq!(
        path.abs_path().unwrap().to_string_lossy(),
        "/code/project1/src"
    );

    let path = project
        .update(cx, |project, cx| {
            project.resolve_abs_path("/code/project1/DOESNOTEXIST", cx)
        })
        .await;
    assert!(path.is_none());
}

#[gpui::test(iterations = 10)]
async fn test_canceling_buffer_opening(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }"
                }
            },
        }),
    )
    .await;

    let (project, _headless) = init_test(&fs, cx, server_cx).await;
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap();
    let worktree_id = worktree.read_with(cx, |tree, _| tree.id());

    // Open a buffer on the client but cancel after a random amount of time.
    let buffer = project.update(cx, |p, cx| p.open_buffer((worktree_id, "src/lib.rs"), cx));
    cx.executor().simulate_random_delay().await;
    drop(buffer);

    // Try opening the same buffer again as the client, and ensure we can
    // still do it despite the cancellation above.
    let buffer = project
        .update(cx, |p, cx| p.open_buffer((worktree_id, "src/lib.rs"), cx))
        .await
        .unwrap();

    buffer.read_with(cx, |buf, _| {
        assert_eq!(buf.text(), "fn one() -> usize { 1 }")
    });
}

#[gpui::test]
async fn test_adding_then_removing_then_adding_worktrees(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }"
                }
            },
            "project2": {
                "README.md": "# project 2",
            },
        }),
    )
    .await;

    let (project, _headless) = init_test(&fs, cx, server_cx).await;
    let (_worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap();

    let (worktree_2, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project2", true, cx)
        })
        .await
        .unwrap();
    let worktree_id_2 = worktree_2.read_with(cx, |tree, _| tree.id());

    project.update(cx, |project, cx| project.remove_worktree(worktree_id_2, cx));

    let (worktree_2, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project2", true, cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();
    worktree_2.update(cx, |worktree, _cx| {
        assert!(worktree.is_visible());
        let entries = worktree.entries(true, 0).collect::<Vec<_>>();
        assert_eq!(entries.len(), 2);
        assert_eq!(
            entries[1].path.to_string_lossy().to_string(),
            "README.md".to_string()
        )
    })
}

#[gpui::test]
async fn test_open_server_settings(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }"
                }
            },
        }),
    )
    .await;

    let (project, _headless) = init_test(&fs, cx, server_cx).await;
    let buffer = project.update(cx, |project, cx| project.open_server_settings(cx));
    cx.executor().run_until_parked();

    let buffer = buffer.await.unwrap();

    cx.update(|cx| {
        assert_eq!(
            buffer.read(cx).text(),
            initial_server_settings_content().to_string()
        )
    })
}

#[gpui::test(iterations = 20)]
async fn test_reconnect(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }"
                }
            },
        }),
    )
    .await;

    let (project, _headless) = init_test(&fs, cx, server_cx).await;

    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap();

    let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());
    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, Path::new("src/lib.rs")), cx)
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        assert_eq!(buffer.text(), "fn one() -> usize { 1 }");
        let ix = buffer.text().find('1').unwrap();
        buffer.edit([(ix..ix + 1, "100")], None, cx);
    });

    let client = cx.read(|cx| project.read(cx).ssh_client().unwrap());
    client
        .update(cx, |client, cx| client.simulate_disconnect(cx))
        .detach();

    project
        .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
        .await
        .unwrap();

    assert_eq!(
        fs.load("/code/project1/src/lib.rs".as_ref()).await.unwrap(),
        "fn one() -> usize { 100 }"
    );
}

#[gpui::test]
async fn test_remote_git_branches(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
            },
        }),
    )
    .await;

    let (project, headless_project) = init_test(&fs, cx, server_cx).await;
    let branches = ["main", "dev", "feature-1"];
    fs.insert_branches(Path::new("/code/project1/.git"), &branches);

    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap();

    let worktree_id = cx.update(|cx| worktree.read(cx).id());
    let root_path = ProjectPath::root_path(worktree_id);
    // Give the worktree a bit of time to index the file system
    cx.run_until_parked();

    let remote_branches = project
        .update(cx, |project, cx| project.branches(root_path.clone(), cx))
        .await
        .unwrap();

    let new_branch = branches[2];

    let remote_branches = remote_branches
        .into_iter()
        .map(|branch| branch.name)
        .collect::<Vec<_>>();

    assert_eq!(&remote_branches, &branches);

    cx.update(|cx| {
        project.update(cx, |project, cx| {
            project.update_or_create_branch(root_path.clone(), new_branch.to_string(), cx)
        })
    })
    .await
    .unwrap();

    cx.run_until_parked();

    let server_branch = server_cx.update(|cx| {
        headless_project.update(cx, |headless_project, cx| {
            headless_project
                .worktree_store
                .update(cx, |worktree_store, cx| {
                    worktree_store
                        .current_branch(root_path.clone(), cx)
                        .unwrap()
                })
        })
    });

    assert_eq!(server_branch.as_ref(), branches[2]);

    // Also try creating a new branch
    cx.update(|cx| {
        project.update(cx, |project, cx| {
            project.update_or_create_branch(root_path.clone(), "totally-new-branch".to_string(), cx)
        })
    })
    .await
    .unwrap();

    cx.run_until_parked();

    let server_branch = server_cx.update(|cx| {
        headless_project.update(cx, |headless_project, cx| {
            headless_project
                .worktree_store
                .update(cx, |worktree_store, cx| {
                    worktree_store.current_branch(root_path, cx).unwrap()
                })
        })
    });

    assert_eq!(server_branch.as_ref(), "totally-new-branch");
}

pub async fn init_test(
    server_fs: &Arc<FakeFs>,
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) -> (Model<Project>, Model<HeadlessProject>) {
    let server_fs = server_fs.clone();
    init_logger();

    let (opts, ssh_server_client) = SshRemoteClient::fake_server(cx, server_cx);
    let http_client = Arc::new(BlockedHttpClient);
    let node_runtime = NodeRuntime::unavailable();
    let languages = Arc::new(LanguageRegistry::new(cx.executor()));
    server_cx.update(HeadlessProject::init);
    let headless = server_cx.new_model(|cx| {
        client::init_settings(cx);

        HeadlessProject::new(
            crate::HeadlessAppState {
                session: ssh_server_client,
                fs: server_fs.clone(),
                http_client,
                node_runtime,
                languages,
            },
            cx,
        )
    });

    let ssh = SshRemoteClient::fake_client(opts, cx).await;
    let project = build_project(ssh, cx);
    project
        .update(cx, {
            let headless = headless.clone();
            |_, cx| cx.on_release(|_, _| drop(headless))
        })
        .detach();
    (project, headless)
}

fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::try_init().ok();
    }
}

fn build_project(ssh: Model<SshRemoteClient>, cx: &mut TestAppContext) -> Model<Project> {
    cx.update(|cx| {
        if !cx.has_global::<SettingsStore>() {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        }
    });

    let client = cx.update(|cx| {
        Client::new(
            Arc::new(FakeSystemClock::default()),
            FakeHttpClient::with_404_response(),
            cx,
        )
    });

    let node = NodeRuntime::unavailable();
    let user_store = cx.new_model(|cx| UserStore::new(client.clone(), cx));
    let languages = Arc::new(LanguageRegistry::test(cx.executor()));
    let fs = FakeFs::new(cx.executor());

    cx.update(|cx| {
        Project::init(&client, cx);
        language::init(cx);
    });

    cx.update(|cx| Project::ssh(ssh, client, node, user_store, languages, fs, cx))
}
