use crate::{
    admin, auth, github,
    rpc::{self, add_rpc_routes},
    AppState, Config,
};
use async_std::task;
use gpui::TestAppContext;
use rand::prelude::*;
use serde_json::json;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    postgres::PgPoolOptions,
    Executor as _, Postgres,
};
use std::{fs, path::Path, sync::Arc};
use zed::{
    editor::Editor,
    language::LanguageRegistry,
    rpc::Client,
    settings,
    test::{temp_tree, Channel},
    worktree::{FakeFs, Fs, RealFs, Worktree},
};
use zed_rpc::{ForegroundRouter, Peer, Router};

#[gpui::test]
async fn test_share_worktree(mut cx_a: TestAppContext, mut cx_b: TestAppContext) {
    let (window_b, _) = cx_b.add_window(|_| EmptyView);
    let settings = settings::channel(&cx_b.font_cache()).unwrap().1;
    let lang_registry = Arc::new(LanguageRegistry::new());

    // Connect to a server as 2 clients.
    let mut server = TestServer::start().await;
    let client_a = server.create_client(&mut cx_a, "user_a").await;
    let client_b = server.create_client(&mut cx_b, "user_b").await;

    // Share a local worktree as client A
    let dir = temp_tree(json!({
        "a.txt": "a-contents",
        "b.txt": "b-contents",
    }));
    let worktree_a = Worktree::open_local(
        dir.path(),
        lang_registry.clone(),
        Arc::new(RealFs),
        &mut cx_a.to_async(),
    )
    .await
    .unwrap();
    worktree_a
        .read_with(&cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
        .await;
    let (worktree_id, worktree_token) = worktree_a
        .update(&mut cx_a, |tree, cx| {
            tree.as_local_mut().unwrap().share(client_a.clone(), cx)
        })
        .await
        .unwrap();

    // Join that worktree as client B, and see that a guest has joined as client A.
    let worktree_b = Worktree::open_remote(
        client_b.clone(),
        worktree_id,
        worktree_token,
        lang_registry.clone(),
        &mut cx_b.to_async(),
    )
    .await
    .unwrap();
    let replica_id_b = worktree_b.read_with(&cx_b, |tree, _| tree.replica_id());
    worktree_a
        .condition(&cx_a, |tree, _| {
            tree.peers()
                .values()
                .any(|replica_id| *replica_id == replica_id_b)
        })
        .await;

    // Open the same file as client B and client A.
    let buffer_b = worktree_b
        .update(&mut cx_b, |worktree, cx| worktree.open_buffer("b.txt", cx))
        .await
        .unwrap();
    buffer_b.read_with(&cx_b, |buf, _| assert_eq!(buf.text(), "b-contents"));
    worktree_a.read_with(&cx_a, |tree, cx| assert!(tree.has_open_buffer("b.txt", cx)));
    let buffer_a = worktree_a
        .update(&mut cx_a, |tree, cx| tree.open_buffer("b.txt", cx))
        .await
        .unwrap();

    // Create a selection set as client B and see that selection set as client A.
    let editor_b = cx_b.add_view(window_b, |cx| Editor::for_buffer(buffer_b, settings, cx));
    buffer_a
        .condition(&cx_a, |buffer, _| buffer.selection_sets().count() == 1)
        .await;

    // Edit the buffer as client B and see that edit as client A.
    editor_b.update(&mut cx_b, |editor, cx| {
        editor.insert(&"ok, ".to_string(), cx)
    });
    buffer_a
        .condition(&cx_a, |buffer, _| buffer.text() == "ok, b-contents")
        .await;

    // Remove the selection set as client B, see those selections disappear as client A.
    cx_b.update(move |_| drop(editor_b));
    buffer_a
        .condition(&cx_a, |buffer, _| buffer.selection_sets().count() == 0)
        .await;

    // Close the buffer as client A, see that the buffer is closed.
    drop(buffer_a);
    worktree_a
        .condition(&cx_a, |tree, cx| !tree.has_open_buffer("b.txt", cx))
        .await;

    // Dropping the worktree removes client B from client A's peers.
    cx_b.update(move |_| drop(worktree_b));
    worktree_a
        .condition(&cx_a, |tree, _| tree.peers().is_empty())
        .await;
}

#[gpui::test]
async fn test_propagate_saves_and_fs_changes_in_shared_worktree(
    mut cx_a: TestAppContext,
    mut cx_b: TestAppContext,
    mut cx_c: TestAppContext,
) {
    let lang_registry = Arc::new(LanguageRegistry::new());

    // Connect to a server as 3 clients.
    let mut server = TestServer::start().await;
    let client_a = server.create_client(&mut cx_a, "user_a").await;
    let client_b = server.create_client(&mut cx_b, "user_b").await;
    let client_c = server.create_client(&mut cx_c, "user_c").await;

    // Share a worktree as client A.
    let dir = temp_tree(json!({
        "file1": "",
        "file2": ""
    }));
    let worktree_a = Worktree::open_local(
        dir.path(),
        lang_registry.clone(),
        Arc::new(RealFs),
        &mut cx_a.to_async(),
    )
    .await
    .unwrap();
    worktree_a
        .read_with(&cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
        .await;
    let (worktree_id, worktree_token) = worktree_a
        .update(&mut cx_a, |tree, cx| {
            tree.as_local_mut().unwrap().share(client_a.clone(), cx)
        })
        .await
        .unwrap();

    // Join that worktree as clients B and C.
    let worktree_b = Worktree::open_remote(
        client_b.clone(),
        worktree_id,
        worktree_token.clone(),
        lang_registry.clone(),
        &mut cx_b.to_async(),
    )
    .await
    .unwrap();
    let worktree_c = Worktree::open_remote(
        client_c.clone(),
        worktree_id,
        worktree_token,
        lang_registry.clone(),
        &mut cx_c.to_async(),
    )
    .await
    .unwrap();

    // Open and edit a buffer as both guests B and C.
    let buffer_b = worktree_b
        .update(&mut cx_b, |tree, cx| tree.open_buffer("file1", cx))
        .await
        .unwrap();
    let buffer_c = worktree_c
        .update(&mut cx_c, |tree, cx| tree.open_buffer("file1", cx))
        .await
        .unwrap();
    buffer_b.update(&mut cx_b, |buf, cx| buf.edit([0..0], "i-am-b, ", cx));
    buffer_c.update(&mut cx_c, |buf, cx| buf.edit([0..0], "i-am-c, ", cx));

    // Open and edit that buffer as the host.
    let buffer_a = worktree_a
        .update(&mut cx_a, |tree, cx| tree.open_buffer("file1", cx))
        .await
        .unwrap();
    buffer_a.update(&mut cx_a, |buf, cx| buf.edit([0..0], "i-am-a", cx));

    // Wait for edits to propagate
    buffer_a
        .condition(&mut cx_a, |buf, _| buf.text() == "i-am-c, i-am-b, i-am-a")
        .await;
    buffer_b
        .condition(&mut cx_b, |buf, _| buf.text() == "i-am-c, i-am-b, i-am-a")
        .await;
    buffer_c
        .condition(&mut cx_c, |buf, _| buf.text() == "i-am-c, i-am-b, i-am-a")
        .await;

    // Edit the buffer as the host and concurrently save as guest B.
    let save_b = buffer_b.update(&mut cx_b, |buf, cx| buf.save(cx).unwrap());
    buffer_a.update(&mut cx_a, |buf, cx| buf.edit([0..0], "hi-a, ", cx));
    save_b.await.unwrap();
    assert_eq!(
        fs::read_to_string(dir.path().join("file1")).unwrap(),
        "hi-a, i-am-c, i-am-b, i-am-a"
    );
    buffer_a.read_with(&cx_a, |buf, _| assert!(!buf.is_dirty()));
    buffer_b.read_with(&cx_b, |buf, _| assert!(!buf.is_dirty()));
    buffer_c.condition(&cx_c, |buf, _| !buf.is_dirty()).await;

    // Make changes on host's file system, see those changes on the guests.
    fs::rename(dir.path().join("file2"), dir.path().join("file3")).unwrap();
    fs::write(dir.path().join("file4"), "4").unwrap();
    worktree_b
        .condition(&cx_b, |tree, _| tree.file_count() == 3)
        .await;
    worktree_c
        .condition(&cx_c, |tree, _| tree.file_count() == 3)
        .await;
    worktree_b.read_with(&cx_b, |tree, _| {
        assert_eq!(
            tree.paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            &["file1", "file3", "file4"]
        )
    });
    worktree_c.read_with(&cx_c, |tree, _| {
        assert_eq!(
            tree.paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            &["file1", "file3", "file4"]
        )
    });
}

#[gpui::test]
async fn test_buffer_conflict_after_save(mut cx_a: TestAppContext, mut cx_b: TestAppContext) {
    let lang_registry = Arc::new(LanguageRegistry::new());

    // Connect to a server as 2 clients.
    let mut server = TestServer::start().await;
    let client_a = server.create_client(&mut cx_a, "user_a").await;
    let client_b = server.create_client(&mut cx_b, "user_b").await;

    // Share a local worktree as client A
    let fs = Arc::new(FakeFs::new());
    fs.save(Path::new("/a.txt"), &"a-contents".into())
        .await
        .unwrap();
    let worktree_a = Worktree::open_local(
        "/".as_ref(),
        lang_registry.clone(),
        Arc::new(RealFs),
        &mut cx_a.to_async(),
    )
    .await
    .unwrap();
    worktree_a
        .read_with(&cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
        .await;
    let (worktree_id, worktree_token) = worktree_a
        .update(&mut cx_a, |tree, cx| {
            tree.as_local_mut().unwrap().share(client_a.clone(), cx)
        })
        .await
        .unwrap();

    // Join that worktree as client B, and see that a guest has joined as client A.
    let worktree_b = Worktree::open_remote(
        client_b.clone(),
        worktree_id,
        worktree_token,
        lang_registry.clone(),
        &mut cx_b.to_async(),
    )
    .await
    .unwrap();

    let buffer_b = worktree_b
        .update(&mut cx_b, |worktree, cx| worktree.open_buffer("a.txt", cx))
        .await
        .unwrap();
    let mtime = buffer_b.read_with(&cx_b, |buf, _| buf.file().unwrap().mtime);

    buffer_b.update(&mut cx_b, |buf, cx| buf.edit([0..0], "world ", cx));
    buffer_b.read_with(&cx_b, |buf, _| {
        assert!(buf.is_dirty());
        assert!(!buf.has_conflict());
    });

    buffer_b
        .update(&mut cx_b, |buf, cx| buf.save(cx))
        .unwrap()
        .await
        .unwrap();
    worktree_b
        .condition(&cx_b, |_, cx| {
            buffer_b.read(cx).file().unwrap().mtime != mtime
        })
        .await;
    buffer_b.read_with(&cx_b, |buf, _| {
        assert!(!buf.is_dirty());
        assert!(!buf.has_conflict());
    });

    buffer_b.update(&mut cx_b, |buf, cx| buf.edit([0..0], "hello ", cx));
    buffer_b.read_with(&cx_b, |buf, _| {
        assert!(buf.is_dirty());
        assert!(!buf.has_conflict());
    });
}

#[gpui::test]
async fn test_editing_while_guest_opens_buffer(mut cx_a: TestAppContext, mut cx_b: TestAppContext) {
    let lang_registry = Arc::new(LanguageRegistry::new());

    // Connect to a server as 2 clients.
    let mut server = TestServer::start().await;
    let client_a = server.create_client(&mut cx_a, "user_a").await;
    let client_b = server.create_client(&mut cx_b, "user_b").await;

    // Share a local worktree as client A
    let fs = Arc::new(FakeFs::new());
    fs.save(Path::new("/a.txt"), &"a-contents".into())
        .await
        .unwrap();
    let worktree_a = Worktree::open_local(
        "/".as_ref(),
        lang_registry.clone(),
        Arc::new(RealFs),
        &mut cx_a.to_async(),
    )
    .await
    .unwrap();
    worktree_a
        .read_with(&cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
        .await;
    let (worktree_id, worktree_token) = worktree_a
        .update(&mut cx_a, |tree, cx| {
            tree.as_local_mut().unwrap().share(client_a.clone(), cx)
        })
        .await
        .unwrap();

    // Join that worktree as client B, and see that a guest has joined as client A.
    let worktree_b = Worktree::open_remote(
        client_b.clone(),
        worktree_id,
        worktree_token,
        lang_registry.clone(),
        &mut cx_b.to_async(),
    )
    .await
    .unwrap();

    let buffer_a = worktree_a
        .update(&mut cx_a, |tree, cx| tree.open_buffer("a.txt", cx))
        .await
        .unwrap();
    let buffer_b = cx_b
        .background()
        .spawn(worktree_b.update(&mut cx_b, |worktree, cx| worktree.open_buffer("a.txt", cx)));

    task::yield_now().await;
    buffer_a.update(&mut cx_a, |buf, cx| buf.edit([0..0], "z", cx));

    let text = buffer_a.read_with(&cx_a, |buf, _| buf.text());
    let buffer_b = buffer_b.await.unwrap();
    buffer_b.condition(&cx_b, |buf, _| buf.text() == text).await;
}

#[gpui::test]
async fn test_peer_disconnection(mut cx_a: TestAppContext, cx_b: TestAppContext) {
    let lang_registry = Arc::new(LanguageRegistry::new());

    // Connect to a server as 2 clients.
    let mut server = TestServer::start().await;
    let client_a = server.create_client(&mut cx_a, "user_a").await;
    let client_b = server.create_client(&mut cx_a, "user_b").await;

    // Share a local worktree as client A
    let dir = temp_tree(json!({
        "a.txt": "a-contents",
        "b.txt": "b-contents",
    }));
    let worktree_a = Worktree::open_local(
        dir.path(),
        lang_registry.clone(),
        Arc::new(RealFs),
        &mut cx_a.to_async(),
    )
    .await
    .unwrap();
    worktree_a
        .read_with(&cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
        .await;
    let (worktree_id, worktree_token) = worktree_a
        .update(&mut cx_a, |tree, cx| {
            tree.as_local_mut().unwrap().share(client_a.clone(), cx)
        })
        .await
        .unwrap();

    // Join that worktree as client B, and see that a guest has joined as client A.
    let _worktree_b = Worktree::open_remote(
        client_b.clone(),
        worktree_id,
        worktree_token,
        lang_registry.clone(),
        &mut cx_b.to_async(),
    )
    .await
    .unwrap();
    worktree_a
        .condition(&cx_a, |tree, _| tree.peers().len() == 1)
        .await;

    // Drop client B's connection and ensure client A observes client B leaving the worktree.
    client_b.disconnect().await.unwrap();
    worktree_a
        .condition(&cx_a, |tree, _| tree.peers().len() == 0)
        .await;
}

struct TestServer {
    peer: Arc<Peer>,
    app_state: Arc<AppState>,
    db_name: String,
    router: Arc<Router>,
}

impl TestServer {
    async fn start() -> Self {
        let mut rng = StdRng::from_entropy();
        let db_name = format!("zed-test-{}", rng.gen::<u128>());
        let app_state = Self::build_app_state(&db_name).await;
        let peer = Peer::new();
        let mut router = Router::new();
        add_rpc_routes(&mut router, &app_state, &peer);
        Self {
            peer,
            router: Arc::new(router),
            app_state,
            db_name,
        }
    }

    async fn create_client(&mut self, cx: &mut TestAppContext, name: &str) -> Client {
        let user_id = admin::create_user(&self.app_state.db, name, false)
            .await
            .unwrap();
        let lang_registry = Arc::new(LanguageRegistry::new());
        let client = Client::new(lang_registry.clone());
        let mut client_router = ForegroundRouter::new();
        cx.update(|cx| zed::worktree::init(cx, &client, &mut client_router));

        let (client_conn, server_conn) = Channel::bidirectional();
        cx.background()
            .spawn(rpc::handle_connection(
                self.peer.clone(),
                self.router.clone(),
                self.app_state.clone(),
                name.to_string(),
                server_conn,
                user_id,
            ))
            .detach();
        client
            .add_connection(client_conn, Arc::new(client_router), cx.to_async())
            .await
            .unwrap();

        // Reset the executor because running SQL queries has a non-deterministic impact on it.
        cx.foreground().reset();
        client
    }

    async fn build_app_state(db_name: &str) -> Arc<AppState> {
        let mut config = Config::default();
        config.session_secret = "a".repeat(32);
        config.database_url = format!("postgres://postgres@localhost/{}", db_name);

        Self::create_db(&config.database_url).await;
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await
            .expect("failed to connect to postgres database");
        let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();
        migrator.run(&db).await.unwrap();

        let github_client = github::AppClient::test();
        Arc::new(AppState {
            db,
            handlebars: Default::default(),
            auth_client: auth::build_client("", ""),
            repo_client: github::RepoClient::test(&github_client),
            github_client,
            rpc: Default::default(),
            config,
        })
    }

    async fn create_db(url: &str) {
        // Enable tests to run in parallel by serializing the creation of each test database.
        lazy_static::lazy_static! {
            static ref DB_CREATION: async_std::sync::Mutex<()> = async_std::sync::Mutex::new(());
        }

        let _lock = DB_CREATION.lock().await;
        Postgres::create_database(url)
            .await
            .expect("failed to create test database");
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        task::block_on(async {
            self.peer.reset().await;
            self.app_state
                .db
                .execute(
                    format!(
                        "
                        SELECT pg_terminate_backend(pg_stat_activity.pid)
                        FROM pg_stat_activity
                        WHERE pg_stat_activity.datname = '{}' AND pid <> pg_backend_pid();",
                        self.db_name,
                    )
                    .as_str(),
                )
                .await
                .unwrap();
            self.app_state.db.close().await;
            Postgres::drop_database(&self.app_state.config.database_url)
                .await
                .unwrap();
        });
    }
}

struct EmptyView;

impl gpui::Entity for EmptyView {
    type Event = ();
}

impl gpui::View for EmptyView {
    fn ui_name() -> &'static str {
        "empty view"
    }

    fn render<'a>(&self, _: &gpui::AppContext) -> gpui::ElementBox {
        gpui::Element::boxed(gpui::elements::Empty)
    }
}
