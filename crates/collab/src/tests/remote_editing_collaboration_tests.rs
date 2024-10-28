use crate::tests::TestServer;
use call::ActiveCall;
use fs::{FakeFs, Fs as _};
use gpui::{BackgroundExecutor, Context as _, TestAppContext};
use http_client::BlockedHttpClient;
use language::{language_settings::language_settings, LanguageRegistry};
use node_runtime::NodeRuntime;
use project::ProjectPath;
use remote::SshRemoteClient;
use remote_server::{HeadlessAppState, HeadlessProject};
use serde_json::json;
use std::{path::Path, sync::Arc};

#[gpui::test(iterations = 10)]
async fn test_sharing_an_ssh_remote_project(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    let executor = cx_a.executor();
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
            "/code",
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
    let _headless_project = server_cx.new_model(|cx| {
        client::init_settings(cx);
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: node,
                languages,
            },
            cx,
        )
    });

    let client_ssh = SshRemoteClient::fake_client(opts, cx_a).await;
    let (project_a, worktree_id) = client_a
        .build_ssh_project("/code/project1", client_ssh, cx_a)
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
            .load("/code/project1/src/renamed.rs".as_ref())
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
            "src/renamed.rs".to_string()
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
    remote_fs.insert_branches(Path::new("/project/.git"), &branches);

    // User A connects to the remote project via SSH.
    server_cx.update(HeadlessProject::init);
    let remote_http_client = Arc::new(BlockedHttpClient);
    let node = NodeRuntime::unavailable();
    let languages = Arc::new(LanguageRegistry::new(server_cx.executor()));
    let headless_project = server_cx.new_model(|cx| {
        client::init_settings(cx);
        HeadlessProject::new(
            HeadlessAppState {
                session: server_ssh,
                fs: remote_fs.clone(),
                http_client: remote_http_client,
                node_runtime: node,
                languages,
            },
            cx,
        )
    });

    let client_ssh = SshRemoteClient::fake_client(opts, cx_a).await;
    let (project_a, worktree_id) = client_a
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

    let root_path = ProjectPath::root_path(worktree_id);

    let branches_b = cx_b
        .update(|cx| project_b.update(cx, |project, cx| project.branches(root_path.clone(), cx)))
        .await
        .unwrap();

    let new_branch = branches[2];

    let branches_b = branches_b
        .into_iter()
        .map(|branch| branch.name)
        .collect::<Vec<_>>();

    assert_eq!(&branches_b, &branches);

    cx_b.update(|cx| {
        project_b.update(cx, |project, cx| {
            project.update_or_create_branch(root_path.clone(), new_branch.to_string(), cx)
        })
    })
    .await
    .unwrap();

    executor.run_until_parked();

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
    cx_b.update(|cx| {
        project_b.update(cx, |project, cx| {
            project.update_or_create_branch(root_path.clone(), "totally-new-branch".to_string(), cx)
        })
    })
    .await
    .unwrap();

    executor.run_until_parked();

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
