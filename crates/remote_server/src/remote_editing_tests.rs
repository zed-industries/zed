/// todo(windows)
/// The tests in this file assume that server_cx is running on Windows too.
/// We neead to find a way to test Windows-Non-Windows interactions.
use crate::headless_project::HeadlessProject;
use agent::{AgentTool, ReadFileTool, ReadFileToolInput, Templates, Thread, ToolCallEventStream};
use client::{Client, UserStore};
use clock::FakeSystemClock;
use collections::{HashMap, HashSet};
use language_model::{LanguageModelToolResultContent, fake_provider::FakeLanguageModel};
use prompt_store::ProjectContext;

use extension::ExtensionHostProxy;
use fs::{FakeFs, Fs};
use gpui::{AppContext as _, Entity, SharedString, TestAppContext};
use http_client::{BlockedHttpClient, FakeHttpClient};
use language::{
    Buffer, FakeLspAdapter, LanguageConfig, LanguageMatcher, LanguageRegistry, LineEnding,
    language_settings::{AllLanguageSettings, language_settings},
};
use lsp::{CompletionContext, CompletionResponse, CompletionTriggerKind, LanguageServerName};
use node_runtime::NodeRuntime;
use project::{
    ProgressToken, Project,
    agent_server_store::AgentServerCommand,
    search::{SearchQuery, SearchResult},
};
use remote::RemoteClient;
use serde_json::json;
use settings::{Settings, SettingsLocation, SettingsStore, initial_server_settings_content};
use smol::stream::StreamExt;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use unindent::Unindent as _;
use util::{path, rel_path::rel_path};

#[gpui::test]
async fn test_basic_remote_editing(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        path!("/code"),
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
        Path::new(path!("/code/project1/.git")),
        &[("src/lib.rs", "fn one() -> usize { 0 }".into())],
    );

    let (project, _headless) = init_test(&fs, cx, server_cx).await;
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/code/project1"), true, cx)
        })
        .await
        .unwrap();

    // The client sees the worktree's contents.
    cx.executor().run_until_parked();
    let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());
    worktree.update(cx, |worktree, _cx| {
        assert_eq!(
            worktree.paths().collect::<Vec<_>>(),
            vec![
                rel_path("README.md"),
                rel_path("src"),
                rel_path("src/lib.rs"),
            ]
        );
    });

    // The user opens a buffer in the remote worktree. The buffer's
    // contents are loaded from the remote filesystem.
    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();
    let diff = project
        .update(cx, |project, cx| {
            project.open_unstaged_diff(buffer.clone(), cx)
        })
        .await
        .unwrap();

    diff.update(cx, |diff, cx| {
        assert_eq!(
            diff.base_text_string(cx).unwrap(),
            "fn one() -> usize { 0 }"
        );
    });

    buffer.update(cx, |buffer, cx| {
        assert_eq!(buffer.text(), "fn one() -> usize { 1 }");
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
        path!("/code/project1/src/main.rs").as_ref(),
        &"fn main() {}".into(),
        Default::default(),
    )
    .await
    .unwrap();
    cx.executor().run_until_parked();
    worktree.update(cx, |worktree, _cx| {
        assert_eq!(
            worktree.paths().collect::<Vec<_>>(),
            vec![
                rel_path("README.md"),
                rel_path("src"),
                rel_path("src/lib.rs"),
                rel_path("src/main.rs"),
            ]
        );
    });

    // A file that is currently open in a buffer is renamed.
    fs.rename(
        path!("/code/project1/src/lib.rs").as_ref(),
        path!("/code/project1/src/lib2.rs").as_ref(),
        Default::default(),
    )
    .await
    .unwrap();
    cx.executor().run_until_parked();
    buffer.update(cx, |buffer, _| {
        assert_eq!(&**buffer.file().unwrap().path(), rel_path("src/lib2.rs"));
    });

    fs.set_index_for_repo(
        Path::new(path!("/code/project1/.git")),
        &[("src/lib2.rs", "fn one() -> usize { 100 }".into())],
    );
    cx.executor().run_until_parked();
    diff.update(cx, |diff, cx| {
        assert_eq!(
            diff.base_text_string(cx).unwrap(),
            "fn one() -> usize { 100 }"
        );
    });
}

#[gpui::test]
async fn test_remote_project_search(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        path!("/code"),
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
            project.find_or_create_worktree(path!("/code/project1"), true, cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();

    async fn do_search(project: &Entity<Project>, mut cx: TestAppContext) -> Entity<Buffer> {
        let receiver = project.update(&mut cx, |project, cx| {
            project.search(
                SearchQuery::text(
                    "project",
                    false,
                    true,
                    false,
                    Default::default(),
                    Default::default(),
                    false,
                    None,
                )
                .unwrap(),
                cx,
            )
        });

        let first_response = receiver.rx.recv().await.unwrap();
        let SearchResult::Buffer { buffer, .. } = first_response else {
            panic!("incorrect result");
        };
        buffer.update(&mut cx, |buffer, cx| {
            assert_eq!(
                buffer.file().unwrap().full_path(cx).to_string_lossy(),
                path!("project1/README.md")
            )
        });

        assert!(receiver.rx.recv().await.is_err());
        buffer
    }

    let buffer = do_search(&project, cx.clone()).await;

    // test that the headless server is tracking which buffers we have open correctly.
    cx.run_until_parked();
    headless.update(server_cx, |headless, cx| {
        assert!(headless.buffer_store.read(cx).has_shared_buffers())
    });
    do_search(&project, cx.clone()).await;
    server_cx.run_until_parked();
    cx.update(|_| {
        drop(buffer);
    });
    cx.run_until_parked();
    server_cx.run_until_parked();
    headless.update(server_cx, |headless, cx| {
        assert!(!headless.buffer_store.read(cx).has_shared_buffers())
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
            ["from-local-settings"],
            "User language settings should be synchronized with the server settings"
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
            ["from-server-settings".to_string()],
            "Server language settings should take precedence over the user settings"
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
            project.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
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
                    path: rel_path("src/lib.rs")
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
        path!("/code"),
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
        path!("/code/project1/.zed"),
        json!({
            "settings.json": r#"
          {
            "languages": {"Rust":{"language_servers":["rust-analyzer", "fake-analyzer"]}},
            "lsp": {
              "rust-analyzer": {
                "binary": {
                  "path": "~/.cargo/bin/rust-analyzer"
                }
              },
              "fake-analyzer": {
               "binary": {
                "path": "~/.cargo/bin/rust-analyzer"
               }
              }
            }
          }"#
        }),
    )
    .await;

    cx.update_entity(&project, |project, _| {
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
                capabilities: lsp::ServerCapabilities {
                    completion_provider: Some(lsp::CompletionOptions::default()),
                    rename_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        );
        project.languages().register_fake_lsp_adapter(
            "Rust",
            FakeLspAdapter {
                name: "fake-analyzer",
                capabilities: lsp::ServerCapabilities {
                    completion_provider: Some(lsp::CompletionOptions::default()),
                    rename_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        )
    });

    let mut fake_lsp = server_cx.update(|cx| {
        headless.read(cx).languages.register_fake_lsp_server(
            LanguageServerName("rust-analyzer".into()),
            lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions::default()),
                rename_provider: Some(lsp::OneOf::Left(true)),
                ..lsp::ServerCapabilities::default()
            },
            None,
        )
    });

    let mut fake_second_lsp = server_cx.update(|cx| {
        headless.read(cx).languages.register_fake_lsp_adapter(
            "Rust",
            FakeLspAdapter {
                name: "fake-analyzer",
                capabilities: lsp::ServerCapabilities {
                    completion_provider: Some(lsp::CompletionOptions::default()),
                    rename_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        );
        headless.read(cx).languages.register_fake_lsp_server(
            LanguageServerName("fake-analyzer".into()),
            lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions::default()),
                rename_provider: Some(lsp::OneOf::Left(true)),
                ..lsp::ServerCapabilities::default()
            },
            None,
        )
    });

    cx.run_until_parked();

    let worktree_id = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/code/project1"), true, cx)
        })
        .await
        .unwrap()
        .0
        .read_with(cx, |worktree, _| worktree.id());

    // Wait for the settings to synchronize
    cx.run_until_parked();

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_buffer_with_lsp((worktree_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();

    let fake_lsp = fake_lsp.next().await.unwrap();
    let fake_second_lsp = fake_second_lsp.next().await.unwrap();

    cx.read(|cx| {
        let file = buffer.read(cx).file();
        assert_eq!(
            language_settings(Some("Rust".into()), file, cx).language_servers,
            ["rust-analyzer".to_string(), "fake-analyzer".to_string()]
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
        assert_eq!(lsp_store.as_local().unwrap().language_servers.len(), 2);
    });

    fake_lsp.set_request_handler::<lsp::request::Completion, _, _>(|_, _| async move {
        Ok(Some(CompletionResponse::Array(vec![lsp::CompletionItem {
            label: "boop".to_string(),
            ..Default::default()
        }])))
    });

    fake_second_lsp.set_request_handler::<lsp::request::Completion, _, _>(|_, _| async move {
        Ok(Some(CompletionResponse::Array(vec![lsp::CompletionItem {
            label: "beep".to_string(),
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
        result
            .into_iter()
            .flat_map(|response| response.completions)
            .map(|c| c.label.text)
            .collect::<Vec<_>>(),
        vec!["boop".to_string(), "beep".to_string()]
    );

    fake_lsp.set_request_handler::<lsp::request::Rename, _, _>(|_, _| async move {
        Ok(Some(lsp::WorkspaceEdit {
            changes: Some(
                [(
                    lsp::Uri::from_file_path(path!("/code/project1/src/lib.rs")).unwrap(),
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
async fn test_remote_cancel_language_server_work(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        path!("/code"),
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
        path!("/code/project1/.zed"),
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

    cx.update_entity(&project, |project, _| {
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
        headless.read(cx).languages.register_fake_lsp_server(
            LanguageServerName("rust-analyzer".into()),
            Default::default(),
            None,
        )
    });

    cx.run_until_parked();

    let worktree_id = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/code/project1"), true, cx)
        })
        .await
        .unwrap()
        .0
        .read_with(cx, |worktree, _| worktree.id());

    cx.run_until_parked();

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_buffer_with_lsp((worktree_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();

    let mut fake_lsp = fake_lsp.next().await.unwrap();

    // Cancelling all language server work for a given buffer
    {
        // Two operations, one cancellable and one not.
        fake_lsp
            .start_progress_with(
                "another-token",
                lsp::WorkDoneProgressBegin {
                    cancellable: Some(false),
                    ..Default::default()
                },
            )
            .await;

        let progress_token = "the-progress-token";
        fake_lsp
            .start_progress_with(
                progress_token,
                lsp::WorkDoneProgressBegin {
                    cancellable: Some(true),
                    ..Default::default()
                },
            )
            .await;

        cx.executor().run_until_parked();

        project.update(cx, |project, cx| {
            project.cancel_language_server_work_for_buffers([buffer.clone()], cx)
        });

        cx.executor().run_until_parked();

        // Verify the cancellation was received on the server side
        let cancel_notification = fake_lsp
            .receive_notification::<lsp::notification::WorkDoneProgressCancel>()
            .await;
        assert_eq!(
            cancel_notification.token,
            lsp::NumberOrString::String(progress_token.into())
        );
    }

    // Cancelling work by server_id and token
    {
        let server_id = fake_lsp.server.server_id();
        let progress_token = "the-progress-token";

        fake_lsp
            .start_progress_with(
                progress_token,
                lsp::WorkDoneProgressBegin {
                    cancellable: Some(true),
                    ..Default::default()
                },
            )
            .await;

        cx.executor().run_until_parked();

        project.update(cx, |project, cx| {
            project.cancel_language_server_work(
                server_id,
                Some(ProgressToken::String(SharedString::from(progress_token))),
                cx,
            )
        });

        cx.executor().run_until_parked();

        // Verify the cancellation was received on the server side
        let cancel_notification = fake_lsp
            .receive_notification::<lsp::notification::WorkDoneProgressCancel>()
            .await;
        assert_eq!(
            cancel_notification.token,
            lsp::NumberOrString::String(progress_token.to_owned())
        );
    }
}

#[gpui::test]
async fn test_remote_reload(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        path!("/code"),
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
            project.find_or_create_worktree(path!("/code/project1"), true, cx)
        })
        .await
        .unwrap();

    let worktree_id = cx.update(|cx| worktree.read(cx).id());

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();

    fs.save(
        &PathBuf::from(path!("/code/project1/src/lib.rs")),
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
        &PathBuf::from(path!("/code/project1/src/lib.rs")),
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
    // Even though we are not testing anything from project1, it is necessary to test if project2 is picking up correct worktree
    fs.insert_tree(
        path!("/code"),
        json!({
            "project1": {
                ".git": {},
                "README.md": "# project 1",
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }"
                }
            },
            "project2": {
                ".git": {},
                "README.md": "# project 2",
                "src": {
                    "lib.rs": "fn two() -> usize { 2 }"
                }
            }
        }),
    )
    .await;

    let (project, _headless) = init_test(&fs, cx, server_cx).await;

    let _ = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/code/project1"), true, cx)
        })
        .await
        .unwrap();

    let (worktree2, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/code/project2"), true, cx)
        })
        .await
        .unwrap();

    let worktree2_id = cx.update(|cx| worktree2.read(cx).id());

    cx.run_until_parked();

    let buffer2 = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree2_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();

    let path = project
        .update(cx, |project, cx| {
            project.resolve_path_in_buffer(path!("/code/project2/README.md"), &buffer2, cx)
        })
        .await
        .unwrap();
    assert!(path.is_file());
    assert_eq!(path.abs_path().unwrap(), path!("/code/project2/README.md"));

    let path = project
        .update(cx, |project, cx| {
            project.resolve_path_in_buffer("../README.md", &buffer2, cx)
        })
        .await
        .unwrap();
    assert!(path.is_file());
    assert_eq!(
        path.project_path().unwrap().clone(),
        (worktree2_id, rel_path("README.md")).into()
    );

    let path = project
        .update(cx, |project, cx| {
            project.resolve_path_in_buffer("../src", &buffer2, cx)
        })
        .await
        .unwrap();
    assert_eq!(
        path.project_path().unwrap().clone(),
        (worktree2_id, rel_path("src")).into()
    );
    assert!(path.is_dir());
}

#[gpui::test]
async fn test_remote_resolve_abs_path(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        path!("/code"),
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
            project.resolve_abs_path(path!("/code/project1/README.md"), cx)
        })
        .await
        .unwrap();

    assert!(path.is_file());
    assert_eq!(path.abs_path().unwrap(), path!("/code/project1/README.md"));

    let path = project
        .update(cx, |project, cx| {
            project.resolve_abs_path(path!("/code/project1/src"), cx)
        })
        .await
        .unwrap();

    assert!(path.is_dir());
    assert_eq!(path.abs_path().unwrap(), path!("/code/project1/src"));

    let path = project
        .update(cx, |project, cx| {
            project.resolve_abs_path(path!("/code/project1/DOESNOTEXIST"), cx)
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
    let buffer = project.update(cx, |p, cx| {
        p.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
    });
    cx.executor().simulate_random_delay().await;
    drop(buffer);

    // Try opening the same buffer again as the client, and ensure we can
    // still do it despite the cancellation above.
    let buffer = project
        .update(cx, |p, cx| {
            p.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
        })
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
        path!("/code"),
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
            project.find_or_create_worktree(path!("/code/project1"), true, cx)
        })
        .await
        .unwrap();

    let (worktree_2, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/code/project2"), true, cx)
        })
        .await
        .unwrap();
    let worktree_id_2 = worktree_2.read_with(cx, |tree, _| tree.id());

    project.update(cx, |project, cx| project.remove_worktree(worktree_id_2, cx));

    let (worktree_2, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/code/project2"), true, cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();
    worktree_2.update(cx, |worktree, _cx| {
        assert!(worktree.is_visible());
        let entries = worktree.entries(true, 0).collect::<Vec<_>>();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[1].path.as_unix_str(), "README.md")
    })
}

#[gpui::test]
async fn test_open_server_settings(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        path!("/code"),
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
            initial_server_settings_content()
                .to_string()
                .replace("\r\n", "\n")
        )
    })
}

#[gpui::test(iterations = 20)]
async fn test_reconnect(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        path!("/code"),
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
            project.find_or_create_worktree(path!("/code/project1"), true, cx)
        })
        .await
        .unwrap();

    let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());
    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        assert_eq!(buffer.text(), "fn one() -> usize { 1 }");
        let ix = buffer.text().find('1').unwrap();
        buffer.edit([(ix..ix + 1, "100")], None, cx);
    });

    let client = cx.read(|cx| project.read(cx).remote_client().unwrap());
    client
        .update(cx, |client, cx| client.simulate_disconnect(cx))
        .detach();

    project
        .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
        .await
        .unwrap();

    assert_eq!(
        fs.load(path!("/code/project1/src/lib.rs").as_ref())
            .await
            .unwrap(),
        "fn one() -> usize { 100 }"
    );
}

#[gpui::test]
async fn test_remote_root_rename(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
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

    let (project, _) = init_test(&fs, cx, server_cx).await;

    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();

    fs.rename(
        &PathBuf::from("/code/project1"),
        &PathBuf::from("/code/project2"),
        Default::default(),
    )
    .await
    .unwrap();

    cx.run_until_parked();
    worktree.update(cx, |worktree, _| {
        assert_eq!(worktree.root_name(), "project2")
    })
}

#[gpui::test]
async fn test_remote_rename_entry(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
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

    let (project, _) = init_test(&fs, cx, server_cx).await;
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();

    let entry = project
        .update(cx, |project, cx| {
            let worktree = worktree.read(cx);
            let entry = worktree.entry_for_path(rel_path("README.md")).unwrap();
            project.rename_entry(entry.id, (worktree.id(), rel_path("README.rst")).into(), cx)
        })
        .await
        .unwrap()
        .into_included()
        .unwrap();

    cx.run_until_parked();

    worktree.update(cx, |worktree, _| {
        assert_eq!(
            worktree.entry_for_path(rel_path("README.rst")).unwrap().id,
            entry.id
        )
    });
}

#[gpui::test]
async fn test_copy_file_into_remote_project(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    let remote_fs = FakeFs::new(server_cx.executor());
    remote_fs
        .insert_tree(
            path!("/code"),
            json!({
                "project1": {
                    ".git": {},
                    "README.md": "# project 1",
                    "src": {
                        "main.rs": ""
                    }
                },
            }),
        )
        .await;

    let (project, _) = init_test(&remote_fs, cx, server_cx).await;
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/code/project1"), true, cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();

    let local_fs = project
        .read_with(cx, |project, _| project.fs().clone())
        .as_fake();
    local_fs
        .insert_tree(
            path!("/local-code"),
            json!({
                "dir1": {
                    "file1": "file 1 content",
                    "dir2": {
                        "file2": "file 2 content",
                        "dir3": {
                            "file3": ""
                        },
                        "dir4": {}
                    },
                    "dir5": {}
                },
                "file4": "file 4 content"
            }),
        )
        .await;

    worktree
        .update(cx, |worktree, cx| {
            worktree.copy_external_entries(
                rel_path("src").into(),
                vec![
                    Path::new(path!("/local-code/dir1/file1")).into(),
                    Path::new(path!("/local-code/dir1/dir2")).into(),
                ],
                local_fs.clone(),
                cx,
            )
        })
        .await
        .unwrap();

    assert_eq!(
        remote_fs.paths(true),
        vec![
            PathBuf::from(path!("/")),
            PathBuf::from(path!("/code")),
            PathBuf::from(path!("/code/project1")),
            PathBuf::from(path!("/code/project1/.git")),
            PathBuf::from(path!("/code/project1/README.md")),
            PathBuf::from(path!("/code/project1/src")),
            PathBuf::from(path!("/code/project1/src/dir2")),
            PathBuf::from(path!("/code/project1/src/file1")),
            PathBuf::from(path!("/code/project1/src/main.rs")),
            PathBuf::from(path!("/code/project1/src/dir2/dir3")),
            PathBuf::from(path!("/code/project1/src/dir2/dir4")),
            PathBuf::from(path!("/code/project1/src/dir2/file2")),
            PathBuf::from(path!("/code/project1/src/dir2/dir3/file3")),
        ]
    );
    assert_eq!(
        remote_fs
            .load(path!("/code/project1/src/file1").as_ref())
            .await
            .unwrap(),
        "file 1 content"
    );
    assert_eq!(
        remote_fs
            .load(path!("/code/project1/src/dir2/file2").as_ref())
            .await
            .unwrap(),
        "file 2 content"
    );
    assert_eq!(
        remote_fs
            .load(path!("/code/project1/src/dir2/dir3/file3").as_ref())
            .await
            .unwrap(),
        ""
    );
}

#[gpui::test]
async fn test_remote_git_diffs(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let text_2 = "
        fn one() -> usize {
            1
        }
    "
    .unindent();
    let text_1 = "
        fn one() -> usize {
            0
        }
    "
    .unindent();

    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        "/code",
        json!({
            "project1": {
                ".git": {},
                "src": {
                    "lib.rs": text_2
                },
                "README.md": "# project 1",
            },
        }),
    )
    .await;
    fs.set_index_for_repo(
        Path::new("/code/project1/.git"),
        &[("src/lib.rs", text_1.clone())],
    );
    fs.set_head_for_repo(
        Path::new("/code/project1/.git"),
        &[("src/lib.rs", text_1.clone())],
        "deadbeef",
    );

    let (project, _headless) = init_test(&fs, cx, server_cx).await;
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/code/project1", true, cx)
        })
        .await
        .unwrap();
    let worktree_id = cx.update(|cx| worktree.read(cx).id());
    cx.executor().run_until_parked();

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();
    let diff = project
        .update(cx, |project, cx| {
            project.open_uncommitted_diff(buffer.clone(), cx)
        })
        .await
        .unwrap();

    diff.read_with(cx, |diff, cx| {
        assert_eq!(diff.base_text_string(cx).unwrap(), text_1);
        assert_eq!(
            diff.secondary_diff()
                .unwrap()
                .read(cx)
                .base_text_string(cx)
                .unwrap(),
            text_1
        );
    });

    // stage the current buffer's contents
    fs.set_index_for_repo(
        Path::new("/code/project1/.git"),
        &[("src/lib.rs", text_2.clone())],
    );

    cx.executor().run_until_parked();
    diff.read_with(cx, |diff, cx| {
        assert_eq!(diff.base_text_string(cx).unwrap(), text_1);
        assert_eq!(
            diff.secondary_diff()
                .unwrap()
                .read(cx)
                .base_text_string(cx)
                .unwrap(),
            text_2
        );
    });

    // commit the current buffer's contents
    fs.set_head_for_repo(
        Path::new("/code/project1/.git"),
        &[("src/lib.rs", text_2.clone())],
        "deadbeef",
    );

    cx.executor().run_until_parked();
    diff.read_with(cx, |diff, cx| {
        assert_eq!(diff.base_text_string(cx).unwrap(), text_2);
        assert_eq!(
            diff.secondary_diff()
                .unwrap()
                .read(cx)
                .base_text_string(cx)
                .unwrap(),
            text_2
        );
    });
}

#[gpui::test]
async fn test_remote_git_diffs_when_recv_update_repository_delay(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme::init(theme::LoadThemes::JustBase, cx);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        editor::init(cx);
    });

    use editor::Editor;
    use gpui::VisualContext;
    let text_2 = "
        fn one() -> usize {
            1
        }
    "
    .unindent();
    let text_1 = "
        fn one() -> usize {
            0
        }
    "
    .unindent();

    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        path!("/code"),
        json!({
            "project1": {
                "src": {
                    "lib.rs": text_2
                },
                "README.md": "# project 1",
            },
        }),
    )
    .await;

    let (project, _headless) = init_test(&fs, cx, server_cx).await;
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/code/project1"), true, cx)
        })
        .await
        .unwrap();
    let worktree_id = cx.update(|cx| worktree.read(cx).id());
    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();
    let buffer_id = cx.update(|cx| buffer.read(cx).remote_id());

    let cx = cx.add_empty_window();
    let editor = cx.new_window_entity(|window, cx| {
        Editor::for_buffer(buffer, Some(project.clone()), window, cx)
    });

    // Remote server will send proto::UpdateRepository after the instance of Editor create.
    fs.insert_tree(
        path!("/code"),
        json!({
            "project1": {
                ".git": {},
            },
        }),
    )
    .await;

    fs.set_index_for_repo(
        Path::new(path!("/code/project1/.git")),
        &[("src/lib.rs", text_1.clone())],
    );
    fs.set_head_for_repo(
        Path::new(path!("/code/project1/.git")),
        &[("src/lib.rs", text_1.clone())],
        "sha",
    );

    cx.executor().run_until_parked();
    let diff = editor
        .read_with(cx, |editor, cx| {
            editor
                .buffer()
                .read_with(cx, |buffer, _| buffer.diff_for(buffer_id))
        })
        .unwrap();

    diff.read_with(cx, |diff, cx| {
        assert_eq!(diff.base_text_string(cx).unwrap(), text_1);
        assert_eq!(
            diff.secondary_diff()
                .unwrap()
                .read(cx)
                .base_text_string(cx)
                .unwrap(),
            text_1
        );
    });

    // stage the current buffer's contents
    fs.set_index_for_repo(
        Path::new(path!("/code/project1/.git")),
        &[("src/lib.rs", text_2.clone())],
    );

    cx.executor().run_until_parked();
    diff.read_with(cx, |diff, cx| {
        assert_eq!(diff.base_text_string(cx).unwrap(), text_1);
        assert_eq!(
            diff.secondary_diff()
                .unwrap()
                .read(cx)
                .base_text_string(cx)
                .unwrap(),
            text_2
        );
    });

    // commit the current buffer's contents
    fs.set_head_for_repo(
        Path::new(path!("/code/project1/.git")),
        &[("src/lib.rs", text_2.clone())],
        "sha",
    );

    cx.executor().run_until_parked();
    diff.read_with(cx, |diff, cx| {
        assert_eq!(diff.base_text_string(cx).unwrap(), text_2);
        assert_eq!(
            diff.secondary_diff()
                .unwrap()
                .read(cx)
                .base_text_string(cx)
                .unwrap(),
            text_2
        );
    });
}

#[gpui::test]
async fn test_remote_git_branches(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        path!("/code"),
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
    let branches_set = branches
        .iter()
        .map(ToString::to_string)
        .collect::<HashSet<_>>();
    fs.insert_branches(Path::new(path!("/code/project1/.git")), &branches);

    let (_worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/code/project1"), true, cx)
        })
        .await
        .unwrap();
    // Give the worktree a bit of time to index the file system
    cx.run_until_parked();

    let repository = project.update(cx, |project, cx| project.active_repository(cx).unwrap());

    let remote_branches = repository
        .update(cx, |repository, _| repository.branches())
        .await
        .unwrap()
        .unwrap();

    let new_branch = branches[2];

    let remote_branches = remote_branches
        .into_iter()
        .map(|branch| branch.name().to_string())
        .collect::<HashSet<_>>();

    assert_eq!(&remote_branches, &branches_set);

    cx.update(|cx| {
        repository.update(cx, |repository, _cx| {
            repository.change_branch(new_branch.to_string())
        })
    })
    .await
    .unwrap()
    .unwrap();

    cx.run_until_parked();

    let server_branch = server_cx.update(|cx| {
        headless_project.update(cx, |headless_project, cx| {
            headless_project.git_store.update(cx, |git_store, cx| {
                git_store
                    .repositories()
                    .values()
                    .next()
                    .unwrap()
                    .read(cx)
                    .branch
                    .as_ref()
                    .unwrap()
                    .clone()
            })
        })
    });

    assert_eq!(server_branch.name(), branches[2]);

    // Also try creating a new branch
    cx.update(|cx| {
        repository.update(cx, |repo, _cx| {
            repo.create_branch("totally-new-branch".to_string(), None)
        })
    })
    .await
    .unwrap()
    .unwrap();

    cx.update(|cx| {
        repository.update(cx, |repo, _cx| {
            repo.change_branch("totally-new-branch".to_string())
        })
    })
    .await
    .unwrap()
    .unwrap();

    cx.run_until_parked();

    let server_branch = server_cx.update(|cx| {
        headless_project.update(cx, |headless_project, cx| {
            headless_project.git_store.update(cx, |git_store, cx| {
                git_store
                    .repositories()
                    .values()
                    .next()
                    .unwrap()
                    .read(cx)
                    .branch
                    .as_ref()
                    .unwrap()
                    .clone()
            })
        })
    });

    assert_eq!(server_branch.name(), "totally-new-branch");
}

#[gpui::test]
async fn test_remote_agent_fs_tool_calls(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(
        path!("/project"),
        json!({
            "a.txt": "A",
            "b.txt": "B",
        }),
    )
    .await;

    let (project, _headless_project) = init_test(&fs, cx, server_cx).await;
    project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/project"), true, cx)
        })
        .await
        .unwrap();

    let action_log = cx.new(|_| action_log::ActionLog::new(project.clone()));

    // Create a minimal thread for the ReadFileTool
    let context_server_registry =
        cx.new(|cx| agent::ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
    let model = Arc::new(FakeLanguageModel::default());
    let thread = cx.new(|cx| {
        Thread::new(
            project.clone(),
            cx.new(|_cx| ProjectContext::default()),
            context_server_registry,
            Templates::new(),
            Some(model),
            cx,
        )
    });

    let input = ReadFileToolInput {
        path: "project/b.txt".into(),
        start_line: None,
        end_line: None,
    };
    let read_tool = Arc::new(ReadFileTool::new(thread.downgrade(), project, action_log));
    let (event_stream, _) = ToolCallEventStream::test();

    let exists_result = cx.update(|cx| read_tool.clone().run(input, event_stream.clone(), cx));
    let output = exists_result.await.unwrap();
    assert_eq!(output, LanguageModelToolResultContent::Text("B".into()));

    let input = ReadFileToolInput {
        path: "project/c.txt".into(),
        start_line: None,
        end_line: None,
    };
    let does_not_exist_result = cx.update(|cx| read_tool.run(input, event_stream, cx));
    does_not_exist_result.await.unwrap_err();
}

#[gpui::test]
async fn test_remote_external_agent_server(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    let fs = FakeFs::new(server_cx.executor());
    fs.insert_tree(path!("/project"), json!({})).await;

    let (project, _headless_project) = init_test(&fs, cx, server_cx).await;
    project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/project"), true, cx)
        })
        .await
        .unwrap();
    let names = project.update(cx, |project, cx| {
        project
            .agent_server_store()
            .read(cx)
            .external_agents()
            .map(|name| name.to_string())
            .collect::<Vec<_>>()
    });
    pretty_assertions::assert_eq!(names, ["codex", "gemini", "claude"]);
    server_cx.update_global::<SettingsStore, _>(|settings_store, cx| {
        settings_store
            .set_server_settings(
                &json!({
                    "agent_servers": {
                        "foo": {
                            "type": "custom",
                            "command": "foo-cli",
                            "args": ["--flag"],
                            "env": {
                                "VAR": "val"
                            }
                        }
                    }
                })
                .to_string(),
                cx,
            )
            .unwrap();
    });
    server_cx.run_until_parked();
    cx.run_until_parked();
    let names = project.update(cx, |project, cx| {
        project
            .agent_server_store()
            .read(cx)
            .external_agents()
            .map(|name| name.to_string())
            .collect::<Vec<_>>()
    });
    pretty_assertions::assert_eq!(names, ["gemini", "codex", "claude", "foo"]);
    let (command, root, login) = project
        .update(cx, |project, cx| {
            project.agent_server_store().update(cx, |store, cx| {
                store
                    .get_external_agent(&"foo".into())
                    .unwrap()
                    .get_command(
                        None,
                        HashMap::from_iter([("OTHER_VAR".into(), "other-val".into())]),
                        None,
                        None,
                        &mut cx.to_async(),
                    )
            })
        })
        .await
        .unwrap();
    assert_eq!(
        command,
        AgentServerCommand {
            path: "mock".into(),
            args: vec!["foo-cli".into(), "--flag".into()],
            env: Some(HashMap::from_iter([
                ("VAR".into(), "val".into()),
                ("OTHER_VAR".into(), "other-val".into())
            ]))
        }
    );
    assert_eq!(&PathBuf::from(root), paths::home_dir());
    assert!(login.is_none());
}

pub async fn init_test(
    server_fs: &Arc<FakeFs>,
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) -> (Entity<Project>, Entity<HeadlessProject>) {
    let server_fs = server_fs.clone();
    cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
    init_logger();

    let (opts, ssh_server_client, _) = RemoteClient::fake_server(cx, server_cx);
    let http_client = Arc::new(BlockedHttpClient);
    let node_runtime = NodeRuntime::unavailable();
    let languages = Arc::new(LanguageRegistry::new(cx.executor()));
    let proxy = Arc::new(ExtensionHostProxy::new());
    server_cx.update(HeadlessProject::init);
    let headless = server_cx.new(|cx| {
        HeadlessProject::new(
            crate::HeadlessAppState {
                session: ssh_server_client,
                fs: server_fs.clone(),
                http_client,
                node_runtime,
                languages,
                extension_host_proxy: proxy,
            },
            false,
            cx,
        )
    });

    let ssh = RemoteClient::connect_mock(opts, cx).await;
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
    zlog::init_test();
}

fn build_project(ssh: Entity<RemoteClient>, cx: &mut TestAppContext) -> Entity<Project> {
    cx.update(|cx| {
        if !cx.has_global::<SettingsStore>() {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        }
    });

    let client = cx.update(|cx| {
        Client::new(
            Arc::new(FakeSystemClock::new()),
            FakeHttpClient::with_404_response(),
            cx,
        )
    });

    let node = NodeRuntime::unavailable();
    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
    let languages = Arc::new(LanguageRegistry::test(cx.executor()));
    let fs = FakeFs::new(cx.executor());

    cx.update(|cx| {
        Project::init(&client, cx);
    });

    cx.update(|cx| Project::remote(ssh, client, node, user_store, languages, fs, false, cx))
}
