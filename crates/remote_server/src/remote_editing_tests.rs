use crate::headless_project::HeadlessProject;
use client::{Client, UserStore};
use clock::FakeSystemClock;
use fs::{FakeFs, Fs};
use gpui::{Context, Model, TestAppContext};
use http_client::FakeHttpClient;
use language::{Buffer, LanguageRegistry};
use node_runtime::FakeNodeRuntime;
use project::{
    search::{SearchQuery, SearchResult},
    Project,
};
use remote::SshSession;
use serde_json::json;
use settings::SettingsStore;
use smol::stream::StreamExt;
use std::{path::Path, sync::Arc};

#[gpui::test]
async fn test_basic_remote_editing(cx: &mut TestAppContext, server_cx: &mut TestAppContext) {
    let (project, _headless, fs) = init_test(cx, server_cx).await;
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
                Path::new(".git"),
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
                Path::new(".git"),
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
    let (project, headless, _) = init_test(cx, server_cx).await;

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

fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::try_init().ok();
    }
}

async fn init_test(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) -> (Model<Project>, Model<HeadlessProject>, Arc<FakeFs>) {
    let (client_ssh, server_ssh) = SshSession::fake(cx, server_cx);
    init_logger();

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

    server_cx.update(HeadlessProject::init);
    let headless = server_cx.new_model(|cx| HeadlessProject::new(server_ssh, fs.clone(), cx));
    let project = build_project(client_ssh, cx);

    project
        .update(cx, {
            let headless = headless.clone();
            |_, cx| cx.on_release(|_, _| drop(headless))
        })
        .detach();
    (project, headless, fs)
}

fn build_project(ssh: Arc<SshSession>, cx: &mut TestAppContext) -> Model<Project> {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
    });

    let client = cx.update(|cx| {
        Client::new(
            Arc::new(FakeSystemClock::default()),
            FakeHttpClient::with_404_response(),
            cx,
        )
    });

    let node = FakeNodeRuntime::new();
    let user_store = cx.new_model(|cx| UserStore::new(client.clone(), cx));
    let languages = Arc::new(LanguageRegistry::test(cx.executor()));
    let fs = FakeFs::new(cx.executor());
    cx.update(|cx| {
        Project::init(&client, cx);
        language::init(cx);
    });

    cx.update(|cx| Project::ssh(ssh, client, node, user_store, languages, fs, cx))
}
