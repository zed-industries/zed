use crate::TestServer;
use call::ActiveCall;
use collections::{HashMap, HashSet};

use dap::{Capabilities, adapters::DebugTaskDefinition, transport::RequestHandling};
use debugger_ui::debugger_panel::DebugPanel;
use editor::{Editor, EditorMode, LSP_REQUEST_DEBOUNCE_TIMEOUT, MultiBuffer};
use extension::ExtensionHostProxy;
use fs::{FakeFs, Fs as _, RemoveOptions};
use futures::{FutureExt as _, StreamExt as _};
use gpui::{
    AppContext as _, BackgroundExecutor, TestAppContext, UpdateGlobal as _, VisualContext as _,
};
use http_client::BlockedHttpClient;
use language::{
    FakeLspAdapter, Language, LanguageConfig, LanguageMatcher, LanguageRegistry,
    language_settings::{
        ConfiguredLanguageServer, Formatter, FormatterList, LanguageSettings, all_language_settings,
    },
    rust_lang, tree_sitter_typescript,
};
use node_runtime::NodeRuntime;
use project::{
    ProjectPath,
    debugger::session::ThreadId,
    lsp_store::{
        FormatTrigger, LspFormatTarget,
        log_store::{self, GlobalLogStore},
    },
    trusted_worktrees::{PathTrust, TrustedWorktrees},
};
use remote::RemoteClient;
use remote_server::{HeadlessAppState, HeadlessProject};
use rpc::proto;
use serde_json::json;
use settings::{
    InlayHintSettingsContent, LanguageServerFormatterSpecifier, PrettierSettingsContent,
    SemanticTokens, SettingsStore,
};
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use task::TcpArgumentsTemplate;
use text::OffsetRangeExt as _;
use util::{path, rel_path::rel_path};

#[gpui::test(iterations = 10)]
async fn test_sharing_an_ssh_remote_project(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    let executor = cx_a.executor();
    cx_a.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    // Set up project on remote FS
    let (opts, server_ssh, _) = RemoteClient::fake_server(cx_a, server_cx);
    let remote_fs = FakeFs::new(server_cx.executor());
    remote_fs
        .insert_tree(
            path!("/code"),
            json!({
                "project1": {
                    ".zed": {
                        "settings.json": r#"{"languages":{"Rust":{"language_servers":["override-rust-analyzer"]}}}"#
                    },
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

    // User A connects to the remote project via SSH.
    server_cx.update(HeadlessProject::init);
    let remote_http_client = Arc::new(BlockedHttpClient);
    let node = NodeRuntime::unavailable();
    let languages = Arc::new(LanguageRegistry::new(server_cx.executor()));
    languages.add(rust_lang());
    let _headless_project = server_cx.new(|cx| {
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: node,
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
                startup_time: std::time::Instant::now(),
            },
            false,
            cx,
        )
    });

    let client_ssh = RemoteClient::connect_mock(opts, cx_a).await;
    let (project_a, worktree_id) = client_a
        .build_ssh_project(path!("/code/project1"), client_ssh, false, cx_a)
        .await;

    // While the SSH worktree is being scanned, user A shares the remote project.
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // User B joins the project.
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    project_b.update(cx_b, |project, _| project.languages().add(rust_lang()));
    let worktree_b = project_b
        .update(cx_b, |project, cx| project.worktree_for_id(worktree_id, cx))
        .unwrap();

    let worktree_a = project_a
        .update(cx_a, |project, cx| project.worktree_for_id(worktree_id, cx))
        .unwrap();

    executor.run_until_parked();

    worktree_a.update(cx_a, |worktree, _cx| {
        assert_eq!(
            worktree.paths().collect::<Vec<_>>(),
            vec![
                rel_path(".zed"),
                rel_path(".zed/settings.json"),
                rel_path("README.md"),
                rel_path("src"),
                rel_path("src/lib.rs"),
            ]
        );
    });

    worktree_b.update(cx_b, |worktree, _cx| {
        assert_eq!(
            worktree.paths().collect::<Vec<_>>(),
            vec![
                rel_path(".zed"),
                rel_path(".zed/settings.json"),
                rel_path("README.md"),
                rel_path("src"),
                rel_path("src/lib.rs"),
            ]
        );
    });

    // User B can open buffers in the remote project.
    let buffer_b = project_b
        .update(cx_b, |project, cx| {
            project.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();
    buffer_b.update(cx_b, |buffer, cx| {
        assert_eq!(buffer.text(), "fn one() -> usize { 1 }");
        let ix = buffer.text().find('1').unwrap();
        buffer.edit([(ix..ix + 1, "100")], None, cx);
    });

    executor.run_until_parked();

    cx_b.read(|cx| {
        assert_eq!(
            LanguageSettings::for_buffer(buffer_b.read(cx), cx).language_servers,
            [ConfiguredLanguageServer::new("override-rust-analyzer")]
        )
    });

    project_b
        .update(cx_b, |project, cx| {
            project.save_buffer_as(
                buffer_b.clone(),
                ProjectPath {
                    worktree_id: worktree_id.to_owned(),
                    path: rel_path("src/renamed.rs").into(),
                },
                cx,
            )
        })
        .await
        .unwrap();
    assert_eq!(
        remote_fs
            .load(path!("/code/project1/src/renamed.rs").as_ref())
            .await
            .unwrap(),
        "fn one() -> usize { 100 }"
    );
    cx_b.run_until_parked();
    cx_b.update(|cx| {
        assert_eq!(
            buffer_b.read(cx).file().unwrap().path().as_ref(),
            rel_path("src/renamed.rs")
        );
    });
}

#[gpui::test]
async fn test_ssh_collaboration_git_branches(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    cx_a.set_name("a");
    cx_b.set_name("b");
    server_cx.set_name("server");

    cx_a.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });

    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    // Set up project on remote FS
    let (opts, server_ssh, _) = RemoteClient::fake_server(cx_a, server_cx);
    let remote_fs = FakeFs::new(server_cx.executor());
    remote_fs
        .insert_tree("/project", serde_json::json!({ ".git":{} }))
        .await;

    let branches = ["main", "dev", "feature-1"];
    let branches_set = branches
        .iter()
        .map(ToString::to_string)
        .collect::<HashSet<_>>();
    remote_fs.insert_branches(Path::new("/project/.git"), &branches);

    // User A connects to the remote project via SSH.
    server_cx.update(HeadlessProject::init);
    let remote_http_client = Arc::new(BlockedHttpClient);
    let node = NodeRuntime::unavailable();
    let languages = Arc::new(LanguageRegistry::new(server_cx.executor()));
    let headless_project = server_cx.new(|cx| {
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: node,
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
                startup_time: std::time::Instant::now(),
            },
            false,
            cx,
        )
    });

    let client_ssh = RemoteClient::connect_mock(opts, cx_a).await;
    let (project_a, _) = client_a
        .build_ssh_project("/project", client_ssh, false, cx_a)
        .await;

    // While the SSH worktree is being scanned, user A shares the remote project.
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // User B joins the project.
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Give client A sometime to see that B has joined, and that the headless server
    // has some git repositories
    executor.run_until_parked();

    let repo_b = cx_b.update(|cx| project_b.read(cx).active_repository(cx).unwrap());

    let branches_b = cx_b
        .update(|cx| repo_b.update(cx, |repo_b, _cx| repo_b.branches()))
        .await
        .unwrap()
        .unwrap();

    let new_branch = branches[2];

    let branches_b = branches_b
        .branches
        .into_iter()
        .map(|branch| branch.name().to_string())
        .collect::<HashSet<_>>();

    assert_eq!(&branches_b, &branches_set);

    cx_b.update(|cx| {
        repo_b.update(cx, |repo_b, _cx| {
            repo_b.change_branch(new_branch.to_string())
        })
    })
    .await
    .unwrap()
    .unwrap();

    executor.run_until_parked();

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
    cx_b.update(|cx| {
        repo_b.update(cx, |repo_b, _cx| {
            repo_b.create_branch("totally-new-branch".to_string(), None)
        })
    })
    .await
    .unwrap()
    .unwrap();

    cx_b.update(|cx| {
        repo_b.update(cx, |repo_b, _cx| {
            repo_b.change_branch("totally-new-branch".to_string())
        })
    })
    .await
    .unwrap()
    .unwrap();

    executor.run_until_parked();

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

    // Remove the git repository and check that all participants get the update.
    remote_fs
        .remove_dir("/project/.git".as_ref(), RemoveOptions::default())
        .await
        .unwrap();
    executor.run_until_parked();

    project_a.update(cx_a, |project, cx| {
        pretty_assertions::assert_eq!(
            project.git_store().read(cx).repo_snapshots(cx),
            HashMap::default()
        );
    });
    project_b.update(cx_b, |project, cx| {
        pretty_assertions::assert_eq!(
            project.git_store().read(cx).repo_snapshots(cx),
            HashMap::default()
        );
    });
}

#[gpui::test]
async fn test_ssh_collaboration_git_worktrees(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    cx_a.set_name("a");
    cx_b.set_name("b");
    server_cx.set_name("server");

    cx_a.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });

    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    let (opts, server_ssh, _) = RemoteClient::fake_server(cx_a, server_cx);
    let remote_fs = FakeFs::new(server_cx.executor());
    remote_fs
        .insert_tree("/project", json!({ ".git": {}, "file.txt": "content" }))
        .await;

    server_cx.update(HeadlessProject::init);
    let languages = Arc::new(LanguageRegistry::new(server_cx.executor()));
    let headless_project = server_cx.new(|cx| {
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: Arc::new(BlockedHttpClient),
                node_runtime: NodeRuntime::unavailable(),
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
                startup_time: std::time::Instant::now(),
            },
            false,
            cx,
        )
    });

    let client_ssh = RemoteClient::connect_mock(opts, cx_a).await;
    let (project_a, _) = client_a
        .build_ssh_project("/project", client_ssh, false, cx_a)
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    executor.run_until_parked();

    let repo_b = cx_b.update(|cx| project_b.read(cx).active_repository(cx).unwrap());

    let worktrees = cx_b
        .update(|cx| repo_b.update(cx, |repo, _| repo.worktrees()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(worktrees.len(), 1);

    let worktree_directory = PathBuf::from("/worktrees");
    cx_b.update(|cx| {
        repo_b.update(cx, |repo, _| {
            repo.create_worktree(
                git::repository::CreateWorktreeTarget::NewBranch {
                    branch_name: "feature-branch".to_string(),
                    base_sha: Some("abc123".to_string()),
                },
                worktree_directory.join("feature-branch"),
            )
        })
    })
    .await
    .unwrap()
    .unwrap();

    executor.run_until_parked();

    let worktrees = cx_b
        .update(|cx| repo_b.update(cx, |repo, _| repo.worktrees()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(worktrees.len(), 2);
    assert_eq!(worktrees[1].path, worktree_directory.join("feature-branch"));
    assert_eq!(
        worktrees[1].ref_name,
        Some("refs/heads/feature-branch".into())
    );
    assert_eq!(worktrees[1].sha.as_ref(), "abc123");

    let server_worktrees = {
        let server_repo = server_cx.update(|cx| {
            headless_project.update(cx, |headless_project, cx| {
                headless_project
                    .git_store
                    .read(cx)
                    .repositories()
                    .values()
                    .next()
                    .unwrap()
                    .clone()
            })
        });
        server_cx
            .update(|cx| server_repo.update(cx, |repo, _| repo.worktrees()))
            .await
            .unwrap()
            .unwrap()
    };
    assert_eq!(server_worktrees.len(), 2);
    assert_eq!(
        server_worktrees[1].path,
        worktree_directory.join("feature-branch")
    );

    // Host (client A) renames the worktree via SSH
    let repo_a = cx_a.update(|cx| {
        project_a
            .read(cx)
            .repositories(cx)
            .values()
            .next()
            .unwrap()
            .clone()
    });
    cx_a.update(|cx| {
        repo_a.update(cx, |repository, _| {
            repository.rename_worktree(
                PathBuf::from("/worktrees/feature-branch"),
                PathBuf::from("/worktrees/renamed-branch"),
            )
        })
    })
    .await
    .unwrap()
    .unwrap();

    executor.run_until_parked();

    let host_worktrees = cx_a
        .update(|cx| repo_a.update(cx, |repository, _| repository.worktrees()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        host_worktrees.len(),
        2,
        "Host should still have 2 worktrees after rename"
    );
    assert_eq!(
        host_worktrees[1].path,
        PathBuf::from("/worktrees/renamed-branch")
    );

    let server_worktrees = {
        let server_repo = server_cx.update(|cx| {
            headless_project.update(cx, |headless_project, cx| {
                headless_project
                    .git_store
                    .read(cx)
                    .repositories()
                    .values()
                    .next()
                    .unwrap()
                    .clone()
            })
        });
        server_cx
            .update(|cx| server_repo.update(cx, |repo, _| repo.worktrees()))
            .await
            .unwrap()
            .unwrap()
    };
    assert_eq!(
        server_worktrees.len(),
        2,
        "Server should still have 2 worktrees after rename"
    );
    assert_eq!(
        server_worktrees[1].path,
        PathBuf::from("/worktrees/renamed-branch")
    );

    // Host (client A) removes the renamed worktree via SSH
    cx_a.update(|cx| {
        repo_a.update(cx, |repository, _| {
            repository.remove_worktree(PathBuf::from("/worktrees/renamed-branch"), false)
        })
    })
    .await
    .unwrap()
    .unwrap();

    executor.run_until_parked();

    let host_worktrees = cx_a
        .update(|cx| repo_a.update(cx, |repository, _| repository.worktrees()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        host_worktrees.len(),
        1,
        "Host should only have the main worktree after removal"
    );

    let server_worktrees = {
        let server_repo = server_cx.update(|cx| {
            headless_project.update(cx, |headless_project, cx| {
                headless_project
                    .git_store
                    .read(cx)
                    .repositories()
                    .values()
                    .next()
                    .unwrap()
                    .clone()
            })
        });
        server_cx
            .update(|cx| server_repo.update(cx, |repo, _| repo.worktrees()))
            .await
            .unwrap()
            .unwrap()
    };
    assert_eq!(
        server_worktrees.len(),
        1,
        "Server should only have the main worktree after removal"
    );
}

#[gpui::test]
async fn test_ssh_collaboration_formatting_with_prettier(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    cx_a.set_name("a");
    cx_b.set_name("b");
    server_cx.set_name("server");

    cx_a.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });

    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    let (opts, server_ssh, _) = RemoteClient::fake_server(cx_a, server_cx);
    let remote_fs = FakeFs::new(server_cx.executor());
    let buffer_text = "let one = \"two\"";
    let prettier_format_suffix = project::TEST_PRETTIER_FORMAT_SUFFIX;
    remote_fs
        .insert_tree(
            path!("/project"),
            serde_json::json!({ "a.ts": buffer_text }),
        )
        .await;

    let ts_lang = Arc::new(Language::new(
        LanguageConfig {
            name: "TypeScript".into(),
            matcher: (LanguageMatcher {
                path_suffixes: vec!["ts".into()],
                ..LanguageMatcher::default()
            })
            .into(),
            ..LanguageConfig::default()
        },
        Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
    ));
    client_a.language_registry().add(ts_lang.clone());
    client_b.language_registry().add(ts_lang.clone());

    let languages = Arc::new(LanguageRegistry::new(server_cx.executor()));
    let mut fake_language_servers =
        languages.register_fake_lsp("TypeScript", FakeLspAdapter::default());

    // User A connects to the remote project via SSH.
    server_cx.update(HeadlessProject::init);
    let remote_http_client = Arc::new(BlockedHttpClient);
    let _headless_project = server_cx.new(|cx| {
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: NodeRuntime::unavailable(),
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
                startup_time: std::time::Instant::now(),
            },
            false,
            cx,
        )
    });

    let client_ssh = RemoteClient::connect_mock(opts, cx_a).await;
    let (project_a, worktree_id) = client_a
        .build_ssh_project(path!("/project"), client_ssh, false, cx_a)
        .await;

    // While the SSH worktree is being scanned, user A shares the remote project.
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // User B joins the project.
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    executor.run_until_parked();

    // Opens the buffer and formats it
    let (buffer_b, _handle) = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer_with_lsp((worktree_id, rel_path("a.ts")), cx)
        })
        .await
        .expect("user B opens buffer for formatting");

    cx_a.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |file| {
                file.project.all_languages.defaults.format_on_save =
                    Some(settings::FormatOnSave::On);
                file.project.all_languages.defaults.formatter = Some(FormatterList::default());
                file.project.all_languages.defaults.prettier = Some(PrettierSettingsContent {
                    allowed: Some(true),
                    ..Default::default()
                });
            });
        });
    });
    cx_b.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |file| {
                file.project.all_languages.defaults.format_on_save =
                    Some(settings::FormatOnSave::On);
                file.project.all_languages.defaults.formatter = Some(FormatterList::Single(
                    Formatter::LanguageServer(LanguageServerFormatterSpecifier::Current),
                ));
                file.project.all_languages.defaults.prettier = Some(PrettierSettingsContent {
                    allowed: Some(true),
                    ..Default::default()
                });
            });
        });
    });
    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.set_request_handler::<lsp::request::Formatting, _, _>(|_, _| async move {
        panic!(
            "Unexpected: prettier should be preferred since it's enabled and language supports it"
        )
    });

    project_b
        .update(cx_b, |project, cx| {
            project.format(
                HashSet::from_iter([buffer_b.clone()]),
                LspFormatTarget::Buffers,
                true,
                FormatTrigger::Save,
                cx,
            )
        })
        .await
        .unwrap();

    executor.run_until_parked();
    assert_eq!(
        buffer_b.read_with(cx_b, |buffer, _| buffer.text()),
        buffer_text.to_string() + "\n" + prettier_format_suffix,
        "Prettier formatting was not applied to client buffer after client's request"
    );

    // User A opens and formats the same buffer too
    let buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_buffer((worktree_id, rel_path("a.ts")), cx)
        })
        .await
        .expect("user A opens buffer for formatting");

    cx_a.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |file| {
                file.project.all_languages.defaults.formatter = Some(FormatterList::default());
                file.project.all_languages.defaults.prettier = Some(PrettierSettingsContent {
                    allowed: Some(true),
                    ..Default::default()
                });
            });
        });
    });
    project_a
        .update(cx_a, |project, cx| {
            project.format(
                HashSet::from_iter([buffer_a.clone()]),
                LspFormatTarget::Buffers,
                true,
                FormatTrigger::Manual,
                cx,
            )
        })
        .await
        .unwrap();

    executor.run_until_parked();
    assert_eq!(
        buffer_b.read_with(cx_b, |buffer, _| buffer.text()),
        buffer_text.to_string() + "\n" + prettier_format_suffix + "\n" + prettier_format_suffix,
        "Prettier formatting was not applied to client buffer after host's request"
    );
}

#[gpui::test(iterations = 10)]
async fn test_ssh_restarting_language_server_replaces_remote_status(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    cx_a.set_name("a");
    server_cx.set_name("server");

    cx_a.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });

    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let log_store = cx_a.update(|cx| log_store::init(false, cx));

    let (opts, server_ssh, _) = RemoteClient::fake_server(cx_a, server_cx);
    let remote_fs = FakeFs::new(server_cx.executor());
    remote_fs
        .insert_tree(path!("/project"), json!({ "a.rs": "fn main() {}" }))
        .await;

    client_a.language_registry().add(rust_lang());

    server_cx.update(HeadlessProject::init);
    let languages = Arc::new(LanguageRegistry::new(server_cx.executor()));
    languages.add(rust_lang());
    let mut fake_language_servers = languages.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-language-server",
            ..Default::default()
        },
    );
    let _headless_project = server_cx.new(|cx| {
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: Arc::new(BlockedHttpClient),
                node_runtime: NodeRuntime::unavailable(),
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
                startup_time: std::time::Instant::now(),
            },
            false,
            cx,
        )
    });

    let client_ssh = RemoteClient::connect_mock(opts, cx_a).await;
    let (project_a, worktree_id) = client_a
        .build_ssh_project(path!("/project"), client_ssh, false, cx_a)
        .await;
    log_store.update(cx_a, |log_store, cx| log_store.add_project(&project_a, cx));

    let (buffer, _handle) = project_a
        .update(cx_a, |project, cx| {
            project.open_buffer_with_lsp((worktree_id, rel_path("a.rs")), cx)
        })
        .await
        .unwrap();

    let first_server = fake_language_servers.next().await.unwrap();
    let first_server_id = first_server.server.server_id();
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, cx| {
        let statuses = project.language_server_statuses(cx).collect::<Vec<_>>();
        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].0, first_server_id);
        assert_eq!(statuses[0].1.name.0, "the-language-server");
    });
    cx_a.read_global::<GlobalLogStore, _>(|global, cx| {
        let log_store = global.0.read(cx);
        let matching_server_ids = log_store
            .language_servers
            .iter()
            .filter_map(|(key, state)| {
                state
                    .name
                    .as_ref()
                    .is_some_and(|name| name.0 == "the-language-server")
                    .then_some(key.server_id)
            })
            .collect::<Vec<_>>();
        assert_eq!(matching_server_ids, vec![first_server_id]);
    });

    project_a.update(cx_a, |project, cx| {
        project.restart_language_servers_for_buffers(vec![buffer.clone()], HashSet::default(), cx);
    });

    let restarted_server = fake_language_servers.next().await.unwrap();
    let restarted_server_id = restarted_server.server.server_id();
    assert_ne!(restarted_server_id, first_server_id);
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, cx| {
        let statuses = project.language_server_statuses(cx).collect::<Vec<_>>();
        assert_eq!(
            statuses.len(),
            1,
            "restarting a remote language server should replace the previous status entry"
        );
        assert_eq!(
            statuses[0].0, restarted_server_id,
            "restarting a remote language server should publish the replacement server id"
        );
        assert_eq!(statuses[0].1.name.0, "the-language-server");
    });
    cx_a.read_global::<GlobalLogStore, _>(|global, cx| {
        let log_store = global.0.read(cx);
        let matching_server_ids = log_store
            .language_servers
            .iter()
            .filter_map(|(key, state)| {
                state
                    .name
                    .as_ref()
                    .is_some_and(|name| name.0 == "the-language-server")
                    .then_some(key.server_id)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            matching_server_ids,
            vec![restarted_server_id],
            "restarting a remote language server should replace the old log store entry"
        );
    });

    let lsp_store = project_a.read_with(cx_a, |project, _| project.lsp_store());
    lsp_store.update(cx_a, |lsp_store, cx| {
        lsp_store.stop_all_language_servers(cx);
    });
    executor.run_until_parked();
    lsp_store.read_with(cx_a, |lsp_store, _| {
        assert_eq!(lsp_store.language_server_statuses().count(), 0);
    });

    cx_a.update_global::<SettingsStore, _>(|store, cx| {
        store.update_user_settings(cx, |settings| {
            settings.project.all_languages.defaults.semantic_tokens = Some(SemanticTokens::Full);
        });
    });
    executor.run_until_parked();
    server_cx.read(|cx| {
        assert_eq!(
            all_language_settings(None, cx).defaults.semantic_tokens,
            SemanticTokens::Full
        );
    });
    lsp_store.read_with(cx_a, |lsp_store, _| {
        assert_eq!(lsp_store.language_server_statuses().count(), 0);
    });

    project_a.update(cx_a, |project, cx| {
        project.restart_language_servers_for_buffers(vec![buffer.clone()], HashSet::default(), cx);
    });
    executor.run_until_parked();
    lsp_store.read_with(cx_a, |lsp_store, _| {
        assert_eq!(lsp_store.language_server_statuses().count(), 1);
    });
    let mut resumed_server = fake_language_servers.next().await.unwrap();
    assert_ne!(resumed_server.server.server_id(), restarted_server_id);
    assert_eq!(
        resumed_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document,
        lsp::TextDocumentItem {
            uri: lsp::Uri::from_file_path(path!("/project/a.rs")).unwrap(),
            language_id: "rust".to_owned(),
            version: 0,
            text: "fn main() {}".to_owned(),
        }
    );
}

#[gpui::test]
async fn test_remote_server_debugger(
    cx_a: &mut TestAppContext,
    server_cx: &mut TestAppContext,
    executor: BackgroundExecutor,
) {
    cx_a.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        command_palette_hooks::init(cx);
        zlog::init_test();
        dap_adapters::init(cx);
    });
    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        dap_adapters::init(cx);
    });
    let (opts, server_ssh, _) = RemoteClient::fake_server(cx_a, server_cx);
    let remote_fs = FakeFs::new(server_cx.executor());
    remote_fs
        .insert_tree(
            path!("/code"),
            json!({
                "lib.rs": "fn one() -> usize { 1 }"
            }),
        )
        .await;

    // User A connects to the remote project via SSH.
    server_cx.update(HeadlessProject::init);
    let remote_http_client = Arc::new(BlockedHttpClient);
    let node = NodeRuntime::unavailable();
    let languages = Arc::new(LanguageRegistry::new(server_cx.executor()));
    let _headless_project = server_cx.new(|cx| {
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: node,
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
                startup_time: std::time::Instant::now(),
            },
            false,
            cx,
        )
    });

    let client_ssh = RemoteClient::connect_mock(opts, cx_a).await;
    let mut server = TestServer::start(server_cx.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    cx_a.update(|cx| {
        debugger_ui::init(cx);
        command_palette_hooks::init(cx);
    });
    let (project_a, _) = client_a
        .build_ssh_project(path!("/code"), client_ssh.clone(), false, cx_a)
        .await;

    let (workspace, cx_a) = client_a.build_workspace(&project_a, cx_a);

    let debugger_panel = workspace
        .update_in(cx_a, |_workspace, window, cx| {
            cx.spawn_in(window, DebugPanel::load)
        })
        .await
        .unwrap();

    workspace.update_in(cx_a, |workspace, window, cx| {
        workspace.add_panel(debugger_panel, window, cx);
    });

    cx_a.run_until_parked();
    let debug_panel = workspace
        .update(cx_a, |workspace, cx| workspace.panel::<DebugPanel>(cx))
        .unwrap();

    let workspace_window = cx_a
        .window_handle()
        .downcast::<workspace::MultiWorkspace>()
        .unwrap();

    let session = debugger_ui::tests::start_debug_session(&workspace_window, cx_a, |_| {}).unwrap();
    cx_a.run_until_parked();
    debug_panel.update(cx_a, |debug_panel, cx| {
        assert_eq!(
            debug_panel.active_session().unwrap().read(cx).session(cx),
            session.clone()
        )
    });

    session.update(
        cx_a,
        |session: &mut project::debugger::session::Session, _| {
            assert_eq!(session.binary().unwrap().command.as_deref(), Some("mock"));
        },
    );

    let shutdown_session = workspace.update(cx_a, |workspace, cx| {
        workspace.project().update(cx, |project, cx| {
            project.dap_store().update(cx, |dap_store, cx| {
                dap_store.shutdown_session(session.read(cx).session_id(), cx)
            })
        })
    });

    client_ssh.update(cx_a, |a, _| {
        a.shutdown_processes(Some(proto::ShutdownRemoteServer {}), executor)
    });

    shutdown_session.await.unwrap();
}

#[gpui::test]
async fn test_slow_adapter_startup_retries(
    cx_a: &mut TestAppContext,
    server_cx: &mut TestAppContext,
    executor: BackgroundExecutor,
) {
    cx_a.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        command_palette_hooks::init(cx);
        zlog::init_test();
        dap_adapters::init(cx);
    });
    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        dap_adapters::init(cx);
    });
    let (opts, server_ssh, _) = RemoteClient::fake_server(cx_a, server_cx);
    let remote_fs = FakeFs::new(server_cx.executor());
    remote_fs
        .insert_tree(
            path!("/code"),
            json!({
                "lib.rs": "fn one() -> usize { 1 }"
            }),
        )
        .await;

    // User A connects to the remote project via SSH.
    server_cx.update(HeadlessProject::init);
    let remote_http_client = Arc::new(BlockedHttpClient);
    let node = NodeRuntime::unavailable();
    let languages = Arc::new(LanguageRegistry::new(server_cx.executor()));
    let _headless_project = server_cx.new(|cx| {
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: node,
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
                startup_time: std::time::Instant::now(),
            },
            false,
            cx,
        )
    });

    let client_ssh = RemoteClient::connect_mock(opts, cx_a).await;
    let mut server = TestServer::start(server_cx.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    cx_a.update(|cx| {
        debugger_ui::init(cx);
        command_palette_hooks::init(cx);
    });
    let (project_a, _) = client_a
        .build_ssh_project(path!("/code"), client_ssh.clone(), false, cx_a)
        .await;

    let (workspace, cx_a) = client_a.build_workspace(&project_a, cx_a);

    let debugger_panel = workspace
        .update_in(cx_a, |_workspace, window, cx| {
            cx.spawn_in(window, DebugPanel::load)
        })
        .await
        .unwrap();

    workspace.update_in(cx_a, |workspace, window, cx| {
        workspace.add_panel(debugger_panel, window, cx);
    });

    cx_a.run_until_parked();
    let debug_panel = workspace
        .update(cx_a, |workspace, cx| workspace.panel::<DebugPanel>(cx))
        .unwrap();

    let workspace_window = cx_a
        .window_handle()
        .downcast::<workspace::MultiWorkspace>()
        .unwrap();

    let count = Arc::new(AtomicUsize::new(0));
    let session = debugger_ui::tests::start_debug_session_with(
        &workspace_window,
        cx_a,
        DebugTaskDefinition {
            adapter: "fake-adapter".into(),
            label: "test".into(),
            config: json!({
                "request": "launch"
            }),
            tcp_connection: Some(TcpArgumentsTemplate {
                port: None,
                host: None,
                timeout: None,
            }),
        },
        move |client| {
            let count = count.clone();
            client.on_request_ext::<dap::requests::Initialize, _>(move |_seq, _request| {
                if count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) < 5 {
                    return RequestHandling::Exit;
                }
                RequestHandling::Respond(Ok(Capabilities::default()))
            });
        },
    )
    .unwrap();
    cx_a.run_until_parked();

    let client = session.update(
        cx_a,
        |session: &mut project::debugger::session::Session, _| session.adapter_client().unwrap(),
    );
    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Pause,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx_a.run_until_parked();

    let active_session = debug_panel
        .update(cx_a, |this, _| this.active_session())
        .unwrap();

    let running_state = active_session.update(cx_a, |active_session, _| {
        active_session.running_state().clone()
    });

    assert_eq!(
        client.id(),
        running_state.read_with(cx_a, |running_state, _| running_state.session_id())
    );
    assert_eq!(
        ThreadId(1),
        running_state.read_with(cx_a, |running_state, _| running_state
            .selected_thread_id()
            .unwrap())
    );

    let shutdown_session = workspace.update(cx_a, |workspace, cx| {
        workspace.project().update(cx, |project, cx| {
            project.dap_store().update(cx, |dap_store, cx| {
                dap_store.shutdown_session(session.read(cx).session_id(), cx)
            })
        })
    });

    client_ssh.update(cx_a, |a, _| {
        a.shutdown_processes(Some(proto::ShutdownRemoteServer {}), executor)
    });

    shutdown_session.await.unwrap();
}

#[gpui::test]
async fn test_ssh_remote_worktree_trust(cx_a: &mut TestAppContext, server_cx: &mut TestAppContext) {
    cx_a.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        project::trusted_worktrees::init(HashMap::default(), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        project::trusted_worktrees::init(HashMap::default(), cx);
    });

    let mut server = TestServer::start(cx_a.executor().clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;

    let server_name = "override-rust-analyzer";
    let lsp_inlay_hint_request_count = Arc::new(AtomicUsize::new(0));

    let (opts, server_ssh, _) = RemoteClient::fake_server(cx_a, server_cx);
    let remote_fs = FakeFs::new(server_cx.executor());
    remote_fs
        .insert_tree(
            path!("/projects"),
            json!({
                "project_a": {
                    ".zed": {
                        "settings.json": r#"{"languages":{"Rust":{"language_servers":["override-rust-analyzer"]}}}"#
                    },
                    "main.rs": "fn main() {}"
                },
                "project_b": { "lib.rs": "pub fn lib() {}" }
            }),
        )
        .await;

    server_cx.update(HeadlessProject::init);
    let remote_http_client = Arc::new(BlockedHttpClient);
    let node = NodeRuntime::unavailable();
    let languages = Arc::new(LanguageRegistry::new(server_cx.executor()));
    languages.add(rust_lang());

    let capabilities = lsp::ServerCapabilities {
        inlay_hint_provider: Some(lsp::OneOf::Left(true)),
        ..lsp::ServerCapabilities::default()
    };
    let mut fake_language_servers = languages.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: server_name,
            capabilities: capabilities.clone(),
            initializer: Some(Box::new({
                let lsp_inlay_hint_request_count = lsp_inlay_hint_request_count.clone();
                move |fake_server| {
                    let lsp_inlay_hint_request_count = lsp_inlay_hint_request_count.clone();
                    fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                        move |_params, _| {
                            lsp_inlay_hint_request_count.fetch_add(1, Ordering::Release);
                            async move {
                                Ok(Some(vec![lsp::InlayHint {
                                    position: lsp::Position::new(0, 0),
                                    label: lsp::InlayHintLabel::String("hint".to_string()),
                                    kind: None,
                                    text_edits: None,
                                    tooltip: None,
                                    padding_left: None,
                                    padding_right: None,
                                    data: None,
                                }]))
                            }
                        },
                    );
                }
            })),
            ..FakeLspAdapter::default()
        },
    );

    let _headless_project = server_cx.new(|cx| {
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: node,
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
                startup_time: std::time::Instant::now(),
            },
            true,
            cx,
        )
    });

    let client_ssh = RemoteClient::connect_mock(opts, cx_a).await;
    let (project_a, worktree_id_a) = client_a
        .build_ssh_project(path!("/projects/project_a"), client_ssh.clone(), true, cx_a)
        .await;

    cx_a.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);

        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                let language_settings = &mut settings.project.all_languages.defaults;
                language_settings.inlay_hints = Some(InlayHintSettingsContent {
                    enabled: Some(true),
                    ..InlayHintSettingsContent::default()
                })
            });
        });
    });

    project_a
        .update(cx_a, |project, cx| {
            project.languages().add(rust_lang());
            project.languages().register_fake_lsp_adapter(
                "Rust",
                FakeLspAdapter {
                    name: server_name,
                    capabilities,
                    ..FakeLspAdapter::default()
                },
            );
            project.find_or_create_worktree(path!("/projects/project_b"), true, cx)
        })
        .await
        .unwrap();

    cx_a.run_until_parked();

    let worktree_ids = project_a.read_with(cx_a, |project, cx| {
        project
            .worktrees(cx)
            .map(|wt| wt.read(cx).id())
            .collect::<Vec<_>>()
    });
    assert_eq!(worktree_ids.len(), 2);

    let trusted_worktrees =
        cx_a.update(|cx| TrustedWorktrees::try_get_global(cx).expect("trust global should exist"));
    let worktree_store = project_a.read_with(cx_a, |project, _| project.worktree_store());

    let can_trust_a = trusted_worktrees.update(cx_a, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[0], cx)
    });
    let can_trust_b = trusted_worktrees.update(cx_a, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[1], cx)
    });
    assert!(!can_trust_a, "project_a should be restricted initially");
    assert!(!can_trust_b, "project_b should be restricted initially");

    let has_restricted = trusted_worktrees.read_with(cx_a, |store, cx| {
        store.has_restricted_worktrees(&worktree_store, cx)
    });
    assert!(has_restricted, "should have restricted worktrees");

    let buffer_before_approval = project_a
        .update(cx_a, |project, cx| {
            project.open_buffer((worktree_id_a, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();

    let (editor, cx_a) = cx_a.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            cx.new(|cx| MultiBuffer::singleton(buffer_before_approval.clone(), cx)),
            Some(project_a.clone()),
            window,
            cx,
        )
    });
    cx_a.run_until_parked();
    let fake_language_server = fake_language_servers.next();

    cx_a.read(|cx| {
        assert_eq!(
            LanguageSettings::for_buffer(buffer_before_approval.read(cx), cx).language_servers,
            [ConfiguredLanguageServer::new("...")],
            "remote .zed/settings.json must not sync before trust approval"
        )
    });

    editor.update_in(cx_a, |editor, window, cx| {
        editor.handle_input("1", window, cx);
    });
    cx_a.run_until_parked();
    cx_a.executor().advance_clock(Duration::from_secs(1));
    assert_eq!(
        lsp_inlay_hint_request_count.load(Ordering::Acquire),
        0,
        "inlay hints must not be queried before trust approval"
    );

    trusted_worktrees.update(cx_a, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::Worktree(worktree_ids[0])]),
            cx,
        );
    });
    cx_a.run_until_parked();

    cx_a.read(|cx| {
        assert_eq!(
            LanguageSettings::for_buffer(buffer_before_approval.read(cx), cx).language_servers,
            [ConfiguredLanguageServer::new("override-rust-analyzer")],
            "remote .zed/settings.json should sync after trust approval"
        )
    });
    let _fake_language_server = fake_language_server.await.unwrap();
    editor.update_in(cx_a, |editor, window, cx| {
        editor.handle_input("1", window, cx);
    });
    cx_a.run_until_parked();
    cx_a.executor().advance_clock(Duration::from_secs(1));
    assert!(
        lsp_inlay_hint_request_count.load(Ordering::Acquire) > 0,
        "inlay hints should be queried after trust approval"
    );

    let can_trust_a = trusted_worktrees.update(cx_a, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[0], cx)
    });
    let can_trust_b = trusted_worktrees.update(cx_a, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[1], cx)
    });
    assert!(can_trust_a, "project_a should be trusted after trust()");
    assert!(!can_trust_b, "project_b should still be restricted");

    trusted_worktrees.update(cx_a, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::Worktree(worktree_ids[1])]),
            cx,
        );
    });

    let can_trust_a = trusted_worktrees.update(cx_a, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[0], cx)
    });
    let can_trust_b = trusted_worktrees.update(cx_a, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[1], cx)
    });
    assert!(can_trust_a, "project_a should remain trusted");
    assert!(can_trust_b, "project_b should now be trusted");

    let has_restricted_after = trusted_worktrees.read_with(cx_a, |store, cx| {
        store.has_restricted_worktrees(&worktree_store, cx)
    });
    assert!(
        !has_restricted_after,
        "should have no restricted worktrees after trusting both"
    );
}

#[gpui::test]
async fn test_ssh_document_links_resolve(
    cx_a: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    cx_a.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        project::trusted_worktrees::init(HashMap::default(), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        project::trusted_worktrees::init(HashMap::default(), cx);
    });

    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;

    let document_link_count = Arc::new(AtomicUsize::new(0));
    let resolve_count = Arc::new(AtomicUsize::new(0));

    let (opts, server_ssh, _) = RemoteClient::fake_server(cx_a, server_cx);
    let remote_fs = FakeFs::new(server_cx.executor());
    remote_fs
        .insert_tree(
            path!("/code"),
            json!({
                "main.rs": "// see LICENSE for details\nfn main() {}",
                "other.rs": "fn other() {}\n",
            }),
        )
        .await;

    server_cx.update(HeadlessProject::init);
    let remote_http_client = Arc::new(BlockedHttpClient);
    let node = NodeRuntime::unavailable();
    let languages = Arc::new(LanguageRegistry::new(server_cx.executor()));
    languages.add(rust_lang());

    let capabilities = lsp::ServerCapabilities {
        document_link_provider: Some(lsp::DocumentLinkOptions {
            resolve_provider: Some(true),
            work_done_progress_options: lsp::WorkDoneProgressOptions::default(),
        }),
        ..lsp::ServerCapabilities::default()
    };
    let other_path_for_remote = path!("/code/other.rs");
    let mut fake_language_servers = languages.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: capabilities.clone(),
            initializer: Some(Box::new({
                let document_link_count = document_link_count.clone();
                let resolve_count = resolve_count.clone();
                move |fake_server| {
                    let document_link_count = document_link_count.clone();
                    fake_server.set_request_handler::<lsp::request::DocumentLinkRequest, _, _>({
                        move |_params, _| {
                            let document_link_count = document_link_count.clone();
                            async move {
                                document_link_count.fetch_add(1, Ordering::Release);
                                Ok(Some(vec![lsp::DocumentLink {
                                    range: lsp::Range {
                                        start: lsp::Position {
                                            line: 0,
                                            character: 7,
                                        },
                                        end: lsp::Position {
                                            line: 0,
                                            character: 14,
                                        },
                                    },
                                    target: None,
                                    tooltip: None,
                                    data: Some(serde_json::json!({"id": 7})),
                                }]))
                            }
                        }
                    });
                    let resolve_count = resolve_count.clone();
                    fake_server.set_request_handler::<lsp::request::DocumentLinkResolve, _, _>({
                        move |link, _| {
                            let resolve_count = resolve_count.clone();
                            async move {
                                resolve_count.fetch_add(1, Ordering::Release);
                                Ok(lsp::DocumentLink {
                                    range: link.range,
                                    target: Some(
                                        lsp::Uri::from_file_path(other_path_for_remote).unwrap(),
                                    ),
                                    tooltip: Some("Open other.rs".into()),
                                    data: None,
                                })
                            }
                        }
                    });
                }
            })),
            ..FakeLspAdapter::default()
        },
    );

    let _headless_project = server_cx.new(|cx| {
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: node,
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
                startup_time: std::time::Instant::now(),
            },
            true,
            cx,
        )
    });

    let client_ssh = RemoteClient::connect_mock(opts, cx_a).await;
    let (project_a, worktree_id) = client_a
        .build_ssh_project(path!("/code"), client_ssh.clone(), true, cx_a)
        .await;

    cx_a.run_until_parked();
    let trusted_worktrees =
        cx_a.update(|cx| TrustedWorktrees::try_get_global(cx).expect("trust global"));
    let worktree_store = project_a.read_with(cx_a, |project, _| project.worktree_store());
    trusted_worktrees.update(cx_a, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
            cx,
        );
    });
    cx_a.run_until_parked();

    cx_a.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.editor.lsp_document_links = Some(true);
            });
        });
    });

    project_a.update(cx_a, |project, _| {
        project.languages().add(rust_lang());
        project.languages().register_fake_lsp_adapter(
            "Rust",
            FakeLspAdapter {
                capabilities,
                ..FakeLspAdapter::default()
            },
        );
    });

    let (buffer, _registration) = project_a
        .update(cx_a, |project, cx| {
            project.open_buffer_with_lsp((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();
    let buffer_id = buffer.read_with(cx_a, |buffer, _| buffer.remote_id());
    cx_a.run_until_parked();
    let _fake_language_server = fake_language_servers.next().await.unwrap();
    cx_a.run_until_parked();

    let (editor, cx_a) = cx_a.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx)),
            Some(project_a.clone()),
            window,
            cx,
        )
    });
    cx_a.executor()
        .advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT + Duration::from_millis(100));
    cx_a.run_until_parked();

    let fetched = project_a.read_with(cx_a, |project, cx| {
        project
            .lsp_store()
            .read(cx)
            .document_links_for_buffer(buffer_id)
            .unwrap_or_default()
    });
    assert_eq!(
        fetched.values().map(|links| links.len()).sum::<usize>(),
        1,
        "Editor should auto-pull a single document link via SSH"
    );
    assert!(
        document_link_count.load(Ordering::Acquire) >= 1,
        "Remote LSP should have served the fetch request"
    );

    let unresolved = fetched
        .values()
        .flat_map(|per_server| per_server.values())
        .next()
        .expect("local cache should mirror the remote document link");
    assert!(
        !unresolved.resolved,
        "freshly fetched links must come back unresolved"
    );

    let anchor = buffer.read_with(cx_a, |buffer, _| buffer.anchor_after(10));
    let resolved = editor
        .update(cx_a, |editor, cx| {
            editor.document_links_at(buffer.clone(), anchor, cx)
        })
        .expect("editor should expose the cached document link at the cursor")
        .await;
    cx_a.run_until_parked();

    assert_eq!(
        resolved.len(),
        1,
        "Editor should surface exactly one resolved link at the cursor"
    );
    assert!(
        resolve_count.load(Ordering::Acquire) >= 1,
        "Local resolve should be forwarded over SSH and run on the remote LSP"
    );

    let other_uri = lsp::Uri::from_file_path(path!("/code/other.rs"))
        .unwrap()
        .to_string();
    let links = project_a.read_with(cx_a, |project, cx| {
        project
            .lsp_store()
            .read(cx)
            .document_links_for_buffer(buffer_id)
            .unwrap_or_default()
    });
    assert_eq!(
        1,
        links.values().map(|m| m.len()).sum::<usize>(),
        "Local cache should mirror the single document link"
    );
    let link = links
        .values()
        .flat_map(|per_server| per_server.values())
        .next()
        .expect("local cache should contain the mirrored link");
    assert_eq!(
        link.target.as_deref(),
        Some(other_uri.as_str()),
        "Local should see the file:// target resolved on the remote"
    );
    assert_eq!(link.tooltip.as_deref(), Some("Open other.rs"));

    let executor = cx_a.executor();
    client_ssh.update(cx_a, |a, _| {
        a.shutdown_processes(Some(proto::ShutdownRemoteServer {}), executor)
    });
}

#[gpui::test]
async fn test_ssh_collaboration_language_server_requests(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    cx_a.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });

    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    let capabilities = lsp::ServerCapabilities {
        hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
        definition_provider: Some(lsp::OneOf::Left(true)),
        call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(true)),
        rename_provider: Some(lsp::OneOf::Left(true)),
        semantic_tokens_provider: Some(
            lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                lsp::SemanticTokensOptions {
                    legend: lsp::SemanticTokensLegend {
                        token_types: vec!["function".into()],
                        token_modifiers: Vec::new(),
                    },
                    full: Some(lsp::SemanticTokensFullOptions::Bool(true)),
                    ..lsp::SemanticTokensOptions::default()
                },
            ),
        ),
        ..lsp::ServerCapabilities::default()
    };

    let (opts, server_ssh, _) = RemoteClient::fake_server(cx_a, server_cx);
    let remote_fs = FakeFs::new(server_cx.executor());
    remote_fs
        .insert_tree(
            path!("/code"),
            json!({
                "main.rs": "fn main() { other::one(); }",
                "other.rs": "pub fn one() {}\n",
                "reexport.rs": "pub use other::one;\n",
            }),
        )
        .await;

    for client in [&client_a, &client_b] {
        client.language_registry().add(rust_lang());
        client.language_registry().register_fake_lsp_adapter(
            "Rust",
            FakeLspAdapter {
                capabilities: capabilities.clone(),
                ..FakeLspAdapter::default()
            },
        );
    }

    server_cx.update(HeadlessProject::init);
    let languages = Arc::new(LanguageRegistry::new(server_cx.executor()));
    languages.add(rust_lang());
    let mut fake_language_servers = languages.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities,
            ..FakeLspAdapter::default()
        },
    );
    let _headless_project = server_cx.new(|cx| {
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: Arc::new(BlockedHttpClient),
                node_runtime: NodeRuntime::unavailable(),
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
                startup_time: std::time::Instant::now(),
            },
            false,
            cx,
        )
    });

    let client_ssh = RemoteClient::connect_mock(opts, cx_a).await;
    let (project_a, worktree_id) = client_a
        .build_ssh_project(path!("/code"), client_ssh, false, cx_a)
        .await;
    let (buffer_a, _handle) = project_a
        .update(cx_a, |project, cx| {
            project.open_buffer_with_lsp((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();
    let fake_language_server = fake_language_servers.next().await.unwrap();

    let hover_gate = Arc::new(Mutex::new(
        None::<futures::future::Shared<futures::channel::oneshot::Receiver<()>>>,
    ));
    let hover_requests = Arc::new(AtomicUsize::new(0));
    fake_language_server.set_request_handler::<lsp::request::HoverRequest, _, _>({
        let hover_gate = hover_gate.clone();
        let hover_requests = hover_requests.clone();
        move |_, _| {
            hover_requests.fetch_add(1, Ordering::SeqCst);
            let gate = hover_gate.lock().unwrap().clone();
            async move {
                if let Some(gate) = gate {
                    gate.await.ok();
                }
                Ok(Some(lsp::Hover {
                    contents: lsp::HoverContents::Scalar(lsp::MarkedString::String(
                        "remote hover".to_string(),
                    )),
                    range: None,
                }))
            }
        }
    });
    fake_language_server.set_request_handler::<lsp::request::GotoDefinition, _, _>(
        |_, _| async move {
            Ok(Some(lsp::GotoDefinitionResponse::Scalar(
                lsp::Location::new(
                    lsp::Uri::from_file_path(path!("/code/other.rs")).unwrap(),
                    lsp::Range::new(lsp::Position::new(0, 7), lsp::Position::new(0, 10)),
                ),
            )))
        },
    );
    let item = lsp::CallHierarchyItem {
        name: "one".to_string(),
        kind: lsp::SymbolKind::FUNCTION,
        tags: None,
        detail: None,
        uri: lsp::Uri::from_file_path(path!("/code/other.rs")).unwrap(),
        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 15)),
        selection_range: lsp::Range::new(lsp::Position::new(0, 7), lsp::Position::new(0, 10)),
        data: None,
    };
    fake_language_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
        let item = item.clone();
        move |_, _| {
            let item = item.clone();
            async move { Ok(Some(vec![item])) }
        }
    });
    fake_language_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
        let item = item.clone();
        move |_, _| {
            let from = item.clone();
            async move {
                Ok(Some(vec![lsp::CallHierarchyIncomingCall {
                    from,
                    from_ranges: vec![lsp::Range::new(
                        lsp::Position::new(0, 12),
                        lsp::Position::new(0, 15),
                    )],
                }]))
            }
        }
    });
    executor.run_until_parked();

    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let buffer_b = project_b
        .update(cx_b, |project, cx| {
            project.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();

    project_b.read_with(cx_b, |project, cx| {
        assert_eq!(
            project
                .language_server_statuses(cx)
                .map(|(_, status)| status.name.0.to_string())
                .collect::<Vec<_>>(),
            ["the-fake-language-server"],
            "the guest should learn about the language servers of the remote project",
        );
    });

    let hovers = project_b
        .update(cx_b, |project, cx| project.hover(&buffer_b, 14, cx))
        .await
        .unwrap();
    assert_eq!(hovers.len(), 1, "unexpected hovers: {hovers:?}");
    assert_eq!(hovers[0].contents[0].text, "remote hover");

    // The definition lies in a file that the host has not opened.
    let definitions = project_b
        .update(cx_b, |project, cx| project.definitions(&buffer_b, 14, cx))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        definitions.len(),
        1,
        "unexpected definitions: {definitions:?}"
    );
    cx_b.read(|cx| {
        let target_buffer = definitions[0].target.buffer.read(cx);
        assert_eq!(target_buffer.text(), "pub fn one() {}\n");
        assert_eq!(
            definitions[0].target.range.to_point(target_buffer),
            text::Point::new(0, 7)..text::Point::new(0, 10)
        );
    });

    let items = project_b
        .update(cx_b, |project, cx| {
            project.prepare_call_hierarchy(&buffer_b, 14, cx)
        })
        .await
        .unwrap()
        .unwrap();
    assert_eq!(items.len(), 1);
    let incoming_calls = project_b
        .update(cx_b, |project, cx| {
            project.incoming_calls(items[0].clone(), cx)
        })
        .await
        .unwrap()
        .unwrap();
    assert_eq!(incoming_calls.len(), 1);
    assert_eq!(incoming_calls[0].from.name, "one");

    // Requests that the host and the guest make for the same buffer at the same time
    // must not supersede each other on the remote server.
    let (release_hover, hover_gate_receiver) = futures::channel::oneshot::channel::<()>();
    *hover_gate.lock().unwrap() = Some(hover_gate_receiver.shared());
    let hover_requests_before = hover_requests.load(Ordering::SeqCst);
    let host_hover = project_a.update(cx_a, |project, cx| project.hover(&buffer_a, 14, cx));
    let guest_hover = project_b.update(cx_b, |project, cx| project.hover(&buffer_b, 14, cx));
    executor.run_until_parked();
    assert_eq!(
        hover_requests.load(Ordering::SeqCst) - hover_requests_before,
        2
    );
    release_hover.send(()).unwrap();
    executor.run_until_parked();
    assert_eq!(host_hover.await.unwrap().len(), 1);
    assert_eq!(guest_hover.await.unwrap().len(), 1);

    let (release_semantic_tokens, semantic_tokens_gate) =
        futures::channel::oneshot::channel::<()>();
    let semantic_tokens_gate = semantic_tokens_gate.shared();
    let semantic_tokens_requests = Arc::new(AtomicUsize::new(0));
    fake_language_server.set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>({
        let semantic_tokens_requests = semantic_tokens_requests.clone();
        move |_, _| {
            semantic_tokens_requests.fetch_add(1, Ordering::SeqCst);
            let semantic_tokens_gate = semantic_tokens_gate.clone();
            async move {
                semantic_tokens_gate.await.ok();
                Ok(Some(lsp::SemanticTokensResult::Tokens(
                    lsp::SemanticTokens {
                        data: vec![0, 3, 4, 0, 0],
                        result_id: None,
                    },
                )))
            }
        }
    });
    let lsp_store_a = project_a.read_with(cx_a, |project, _| project.lsp_store());
    let lsp_store_b = project_b.read_with(cx_b, |project, _| project.lsp_store());
    let host_tokens = lsp_store_a.update(cx_a, |lsp_store, cx| {
        lsp_store.semantic_tokens(buffer_a.clone(), cx)
    });
    let guest_tokens = lsp_store_b.update(cx_b, |lsp_store, cx| {
        lsp_store.semantic_tokens(buffer_b.clone(), cx)
    });
    executor.run_until_parked();
    assert_eq!(semantic_tokens_requests.load(Ordering::SeqCst), 2);
    release_semantic_tokens.send(()).unwrap();
    executor.run_until_parked();
    for tokens in [host_tokens.await.unwrap(), guest_tokens.await.unwrap()] {
        assert_eq!(tokens.tokens.as_ref().map_or(0, HashMap::len), 1);
    }

    // The rename edits a file that neither the host nor the guest has opened.
    fake_language_server.set_request_handler::<lsp::request::Rename, _, _>(|_, _| async move {
        Ok(Some(lsp::WorkspaceEdit {
            changes: Some(
                [
                    (
                        lsp::Uri::from_file_path(path!("/code/main.rs")).unwrap(),
                        vec![lsp::TextEdit::new(
                            lsp::Range::new(lsp::Position::new(0, 19), lsp::Position::new(0, 22)),
                            "two".to_string(),
                        )],
                    ),
                    (
                        lsp::Uri::from_file_path(path!("/code/reexport.rs")).unwrap(),
                        vec![lsp::TextEdit::new(
                            lsp::Range::new(lsp::Position::new(0, 15), lsp::Position::new(0, 18)),
                            "two".to_string(),
                        )],
                    ),
                ]
                .into_iter()
                .collect(),
            ),
            ..lsp::WorkspaceEdit::default()
        }))
    });
    let transaction = project_b
        .update(cx_b, |project, cx| {
            project.perform_rename(buffer_b.clone(), 14, "two".to_string(), None, cx)
        })
        .await
        .unwrap();
    cx_b.read(|cx| {
        let mut texts = transaction
            .0
            .keys()
            .map(|buffer| buffer.read(cx).text())
            .collect::<Vec<_>>();
        texts.sort();
        assert_eq!(
            texts,
            ["fn main() { other::two(); }", "pub use other::two;\n"]
        );
    });

    // Servers that the remote project starts and stops after sharing reach the guest too.
    let first_server_id = fake_language_server.server.server_id();
    project_a.update(cx_a, |project, cx| {
        project.restart_language_servers_for_buffers(
            vec![buffer_a.clone()],
            HashSet::default(),
            cx,
        );
    });
    let restarted_server_id = fake_language_servers
        .next()
        .await
        .unwrap()
        .server
        .server_id();
    assert_ne!(restarted_server_id, first_server_id);
    executor.run_until_parked();

    project_b.read_with(cx_b, |project, cx| {
        assert_eq!(
            project
                .language_server_statuses(cx)
                .map(|(server_id, _)| server_id)
                .collect::<Vec<_>>(),
            [restarted_server_id],
            "the guest should see the restarted server in place of the stopped one",
        );
    });
}
