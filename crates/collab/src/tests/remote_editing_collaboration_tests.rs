use crate::tests::TestServer;
use call::ActiveCall;
use collections::{HashMap, HashSet};

use debugger_ui::debugger_panel::DebugPanel;
use extension::ExtensionHostProxy;
use fs::{FakeFs, Fs as _, RemoveOptions};
use futures::StreamExt as _;
use gpui::{
    AppContext as _, BackgroundExecutor, SemanticVersion, TestAppContext, UpdateGlobal as _,
    VisualContext,
};
use http_client::BlockedHttpClient;
use language::{
    FakeLspAdapter, Language, LanguageConfig, LanguageMatcher, LanguageRegistry,
    language_settings::{
        AllLanguageSettings, Formatter, FormatterList, PrettierSettings, SelectedFormatter,
        language_settings,
    },
    tree_sitter_typescript,
};
use node_runtime::NodeRuntime;
use project::{
    ProjectPath,
    lsp_store::{FormatTrigger, LspFormatTarget},
};
use remote::SshRemoteClient;
use remote_server::{HeadlessAppState, HeadlessProject};
use serde_json::json;
use settings::SettingsStore;
use std::{path::Path, sync::Arc};
use util::{path, separator};

#[gpui::test(iterations = 10)]
async fn test_sharing_an_ssh_remote_project(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    let executor = cx_a.executor();
    cx_a.update(|cx| {
        release_channel::init(SemanticVersion::default(), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(SemanticVersion::default(), cx);
    });
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    // Set up project on remote FS
    let (opts, server_ssh) = SshRemoteClient::fake_server(cx_a, server_cx);
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
    let _headless_project = server_cx.new(|cx| {
        client::init_settings(cx);
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: node,
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
            },
            cx,
        )
    });

    let client_ssh = SshRemoteClient::fake_client(opts, cx_a).await;
    let (project_a, worktree_id) = client_a
        .build_ssh_project(path!("/code/project1"), client_ssh, cx_a)
        .await;

    // While the SSH worktree is being scanned, user A shares the remote project.
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // User B joins the project.
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let worktree_b = project_b
        .update(cx_b, |project, cx| project.worktree_for_id(worktree_id, cx))
        .unwrap();

    let worktree_a = project_a
        .update(cx_a, |project, cx| project.worktree_for_id(worktree_id, cx))
        .unwrap();

    executor.run_until_parked();

    worktree_a.update(cx_a, |worktree, _cx| {
        assert_eq!(
            worktree.paths().map(Arc::as_ref).collect::<Vec<_>>(),
            vec![
                Path::new(".zed"),
                Path::new(".zed/settings.json"),
                Path::new("README.md"),
                Path::new("src"),
                Path::new("src/lib.rs"),
            ]
        );
    });

    worktree_b.update(cx_b, |worktree, _cx| {
        assert_eq!(
            worktree.paths().map(Arc::as_ref).collect::<Vec<_>>(),
            vec![
                Path::new(".zed"),
                Path::new(".zed/settings.json"),
                Path::new("README.md"),
                Path::new("src"),
                Path::new("src/lib.rs"),
            ]
        );
    });

    // User B can open buffers in the remote project.
    let buffer_b = project_b
        .update(cx_b, |project, cx| {
            project.open_buffer((worktree_id, "src/lib.rs"), cx)
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
        let file = buffer_b.read(cx).file();
        assert_eq!(
            language_settings(Some("Rust".into()), file, cx).language_servers,
            ["override-rust-analyzer".to_string()]
        )
    });

    project_b
        .update(cx_b, |project, cx| {
            project.save_buffer_as(
                buffer_b.clone(),
                ProjectPath {
                    worktree_id: worktree_id.to_owned(),
                    path: Arc::from(Path::new("src/renamed.rs")),
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
            buffer_b
                .read(cx)
                .file()
                .unwrap()
                .path()
                .to_string_lossy()
                .to_string(),
            separator!("src/renamed.rs").to_string()
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
        release_channel::init(SemanticVersion::default(), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(SemanticVersion::default(), cx);
    });

    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    // Set up project on remote FS
    let (opts, server_ssh) = SshRemoteClient::fake_server(cx_a, server_cx);
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
        client::init_settings(cx);
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: node,
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
            },
            cx,
        )
    });

    let client_ssh = SshRemoteClient::fake_client(opts, cx_a).await;
    let (project_a, _) = client_a
        .build_ssh_project("/project", client_ssh, cx_a)
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
        .into_iter()
        .map(|branch| branch.name.to_string())
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

    assert_eq!(server_branch.name, branches[2]);

    // Also try creating a new branch
    cx_b.update(|cx| {
        repo_b.update(cx, |repo_b, _cx| {
            repo_b.create_branch("totally-new-branch".to_string())
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

    assert_eq!(server_branch.name, "totally-new-branch");

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
        release_channel::init(SemanticVersion::default(), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(SemanticVersion::default(), cx);
    });

    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    let (opts, server_ssh) = SshRemoteClient::fake_server(cx_a, server_cx);
    let remote_fs = FakeFs::new(server_cx.executor());
    let buffer_text = "let one = \"two\"";
    let prettier_format_suffix = project::TEST_PRETTIER_FORMAT_SUFFIX;
    remote_fs
        .insert_tree(
            path!("/project"),
            serde_json::json!({ "a.ts": buffer_text }),
        )
        .await;

    let test_plugin = "test_plugin";
    let ts_lang = Arc::new(Language::new(
        LanguageConfig {
            name: "TypeScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["ts".to_string()],
                ..LanguageMatcher::default()
            },
            ..LanguageConfig::default()
        },
        Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
    ));
    client_a.language_registry().add(ts_lang.clone());
    client_b.language_registry().add(ts_lang.clone());

    let languages = Arc::new(LanguageRegistry::new(server_cx.executor()));
    let mut fake_language_servers = languages.register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            prettier_plugins: vec![test_plugin],
            ..Default::default()
        },
    );

    // User A connects to the remote project via SSH.
    server_cx.update(HeadlessProject::init);
    let remote_http_client = Arc::new(BlockedHttpClient);
    let _headless_project = server_cx.new(|cx| {
        client::init_settings(cx);
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: NodeRuntime::unavailable(),
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
            },
            cx,
        )
    });

    let client_ssh = SshRemoteClient::fake_client(opts, cx_a).await;
    let (project_a, worktree_id) = client_a
        .build_ssh_project(path!("/project"), client_ssh, cx_a)
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
            p.open_buffer_with_lsp((worktree_id, "a.ts"), cx)
        })
        .await
        .expect("user B opens buffer for formatting");

    cx_a.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |file| {
                file.defaults.formatter = Some(SelectedFormatter::Auto);
                file.defaults.prettier = Some(PrettierSettings {
                    allowed: true,
                    ..PrettierSettings::default()
                });
            });
        });
    });
    cx_b.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |file| {
                file.defaults.formatter = Some(SelectedFormatter::List(FormatterList(
                    vec![Formatter::LanguageServer { name: None }].into(),
                )));
                file.defaults.prettier = Some(PrettierSettings {
                    allowed: true,
                    ..PrettierSettings::default()
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
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.ts"), cx))
        .await
        .expect("user A opens buffer for formatting");

    cx_a.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |file| {
                file.defaults.formatter = Some(SelectedFormatter::Auto);
                file.defaults.prettier = Some(PrettierSettings {
                    allowed: true,
                    ..PrettierSettings::default()
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

#[gpui::test]
async fn test_remote_server_debugger(cx_a: &mut TestAppContext, server_cx: &mut TestAppContext) {
    cx_a.update(|cx| {
        release_channel::init(SemanticVersion::default(), cx);
        command_palette_hooks::init(cx);
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::try_init().ok();
        }
    });
    server_cx.update(|cx| {
        release_channel::init(SemanticVersion::default(), cx);
    });
    let (opts, server_ssh) = SshRemoteClient::fake_server(cx_a, server_cx);
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
        client::init_settings(cx);
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: node,
                languages,
                extension_host_proxy: Arc::new(ExtensionHostProxy::new()),
            },
            cx,
        )
    });

    let client_ssh = SshRemoteClient::fake_client(opts, cx_a).await;
    let mut server = TestServer::start(server_cx.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    cx_a.update(|cx| {
        debugger_ui::init(cx);
        command_palette_hooks::init(cx);
    });
    let (project_a, _) = client_a
        .build_ssh_project(path!("/code"), client_ssh, cx_a)
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
        .downcast::<workspace::Workspace>()
        .unwrap();

    let session = debugger_ui::tests::start_debug_session(&workspace_window, cx_a, |_| {}).unwrap();
    cx_a.run_until_parked();
    debug_panel.update(cx_a, |debug_panel, cx| {
        assert_eq!(
            debug_panel.active_session().unwrap().read(cx).session(cx),
            session
        )
    });
    session
        .update(cx_a, |session, cx| {
            assert_eq!(session.binary().command, "ssh");
            session.shutdown(cx)
        })
        .await;
}
