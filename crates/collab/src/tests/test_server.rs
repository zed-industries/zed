use crate::{
    db::{tests::TestDb, NewUserParams, UserId},
    executor::Executor,
    rpc::{Server, ZedVersion, CLEANUP_TIMEOUT, RECONNECT_TIMEOUT},
    AppState, Config,
};
use anyhow::anyhow;
use call::ActiveCall;
use channel::{ChannelBuffer, ChannelStore};
use client::{
    self, proto::PeerId, ChannelId, Client, Connection, Credentials, EstablishConnectionError,
    UserStore,
};
use clock::FakeSystemClock;
use collab_ui::channel_view::ChannelView;
use collections::{HashMap, HashSet};
use fs::FakeFs;
use futures::{channel::oneshot, StreamExt as _};
use gpui::{BackgroundExecutor, Context, Model, Task, TestAppContext, View, VisualTestContext};
use language::LanguageRegistry;
use node_runtime::FakeNodeRuntime;

use notifications::NotificationStore;
use parking_lot::Mutex;
use project::{Project, WorktreeId};
use rpc::{
    proto::{self, ChannelRole},
    RECEIVE_TIMEOUT,
};
use serde_json::json;
use settings::SettingsStore;
use std::{
    cell::{Ref, RefCell, RefMut},
    env,
    ops::{Deref, DerefMut},
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use util::{http::FakeHttpClient, SemanticVersion};
use workspace::{Workspace, WorkspaceStore};

pub struct TestServer {
    pub app_state: Arc<AppState>,
    pub test_live_kit_server: Arc<live_kit_client::TestServer>,
    server: Arc<Server>,
    next_github_user_id: i32,
    connection_killers: Arc<Mutex<HashMap<PeerId, Arc<AtomicBool>>>>,
    forbid_connections: Arc<AtomicBool>,
    _test_db: TestDb,
}

pub struct TestClient {
    pub username: String,
    pub app_state: Arc<workspace::AppState>,
    channel_store: Model<ChannelStore>,
    notification_store: Model<NotificationStore>,
    state: RefCell<TestClientState>,
}

#[derive(Default)]
struct TestClientState {
    local_projects: Vec<Model<Project>>,
    remote_projects: Vec<Model<Project>>,
    buffers: HashMap<Model<Project>, HashSet<Model<language::Buffer>>>,
    channel_buffers: HashSet<Model<ChannelBuffer>>,
}

pub struct ContactsSummary {
    pub current: Vec<String>,
    pub outgoing_requests: Vec<String>,
    pub incoming_requests: Vec<String>,
}

impl TestServer {
    pub async fn start(deterministic: BackgroundExecutor) -> Self {
        static NEXT_LIVE_KIT_SERVER_ID: AtomicUsize = AtomicUsize::new(0);

        let use_postgres = env::var("USE_POSTGRES").ok();
        let use_postgres = use_postgres.as_deref();
        let test_db = if use_postgres == Some("true") || use_postgres == Some("1") {
            TestDb::postgres(deterministic.clone())
        } else {
            TestDb::sqlite(deterministic.clone())
        };
        let live_kit_server_id = NEXT_LIVE_KIT_SERVER_ID.fetch_add(1, SeqCst);
        let live_kit_server = live_kit_client::TestServer::create(
            format!("http://livekit.{}.test", live_kit_server_id),
            format!("devkey-{}", live_kit_server_id),
            format!("secret-{}", live_kit_server_id),
            deterministic.clone(),
        )
        .unwrap();
        let app_state = Self::build_app_state(&test_db, &live_kit_server).await;
        let epoch = app_state
            .db
            .create_server(&app_state.config.zed_environment)
            .await
            .unwrap();
        let server = Server::new(
            epoch,
            app_state.clone(),
            Executor::Deterministic(deterministic.clone()),
        );
        server.start().await.unwrap();
        // Advance clock to ensure the server's cleanup task is finished.
        deterministic.advance_clock(CLEANUP_TIMEOUT);
        Self {
            app_state,
            server,
            connection_killers: Default::default(),
            forbid_connections: Default::default(),
            next_github_user_id: 0,
            _test_db: test_db,
            test_live_kit_server: live_kit_server,
        }
    }

    pub async fn start2(
        cx_a: &mut TestAppContext,
        cx_b: &mut TestAppContext,
    ) -> (TestServer, TestClient, TestClient, ChannelId) {
        let mut server = Self::start(cx_a.executor()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;
        let channel_id = server
            .make_channel(
                "test-channel",
                None,
                (&client_a, cx_a),
                &mut [(&client_b, cx_b)],
            )
            .await;
        cx_a.run_until_parked();

        (server, client_a, client_b, channel_id)
    }

    pub async fn start1(cx: &mut TestAppContext) -> TestClient {
        let mut server = Self::start(cx.executor().clone()).await;
        server.create_client(cx, "user_a").await
    }

    pub async fn reset(&self) {
        self.app_state.db.reset();
        let epoch = self
            .app_state
            .db
            .create_server(&self.app_state.config.zed_environment)
            .await
            .unwrap();
        self.server.reset(epoch);
    }

    pub async fn create_client(&mut self, cx: &mut TestAppContext, name: &str) -> TestClient {
        cx.update(|cx| {
            if cx.has_global::<SettingsStore>() {
                panic!("Same cx used to create two test clients")
            }
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            release_channel::init("0.0.0", cx);
            client::init_settings(cx);
        });

        let clock = Arc::new(FakeSystemClock::default());
        let http = FakeHttpClient::with_404_response();
        let user_id = if let Ok(Some(user)) = self.app_state.db.get_user_by_github_login(name).await
        {
            user.id
        } else {
            let github_user_id = self.next_github_user_id;
            self.next_github_user_id += 1;
            self.app_state
                .db
                .create_user(
                    &format!("{name}@example.com"),
                    false,
                    NewUserParams {
                        github_login: name.into(),
                        github_user_id,
                    },
                )
                .await
                .expect("creating user failed")
                .user_id
        };
        let client_name = name.to_string();
        let mut client = cx.update(|cx| Client::new(clock, http.clone(), cx));
        let server = self.server.clone();
        let db = self.app_state.db.clone();
        let connection_killers = self.connection_killers.clone();
        let forbid_connections = self.forbid_connections.clone();

        Arc::get_mut(&mut client)
            .unwrap()
            .set_id(user_id.to_proto())
            .override_authenticate(move |cx| {
                cx.spawn(|_| async move {
                    let access_token = "the-token".to_string();
                    Ok(Credentials {
                        user_id: user_id.to_proto(),
                        access_token,
                    })
                })
            })
            .override_establish_connection(move |credentials, cx| {
                assert_eq!(credentials.user_id, user_id.0 as u64);
                assert_eq!(credentials.access_token, "the-token");

                let server = server.clone();
                let db = db.clone();
                let connection_killers = connection_killers.clone();
                let forbid_connections = forbid_connections.clone();
                let client_name = client_name.clone();
                cx.spawn(move |cx| async move {
                    if forbid_connections.load(SeqCst) {
                        Err(EstablishConnectionError::other(anyhow!(
                            "server is forbidding connections"
                        )))
                    } else {
                        let (client_conn, server_conn, killed) =
                            Connection::in_memory(cx.background_executor().clone());
                        let (connection_id_tx, connection_id_rx) = oneshot::channel();
                        let user = db
                            .get_user_by_id(user_id)
                            .await
                            .expect("retrieving user failed")
                            .unwrap();
                        cx.background_executor()
                            .spawn(server.handle_connection(
                                server_conn,
                                client_name,
                                user,
                                ZedVersion(SemanticVersion::new(1, 0, 0)),
                                None,
                                Some(connection_id_tx),
                                Executor::Deterministic(cx.background_executor().clone()),
                            ))
                            .detach();
                        let connection_id = connection_id_rx.await.map_err(|e| {
                            EstablishConnectionError::Other(anyhow!(
                                "{} (is server shutting down?)",
                                e
                            ))
                        })?;
                        connection_killers
                            .lock()
                            .insert(connection_id.into(), killed);
                        Ok(client_conn)
                    }
                })
            });

        let fs = FakeFs::new(cx.executor());
        let user_store = cx.new_model(|cx| UserStore::new(client.clone(), cx));
        let workspace_store = cx.new_model(|cx| WorkspaceStore::new(client.clone(), cx));
        let mut language_registry = LanguageRegistry::test();
        language_registry.set_executor(cx.executor());
        let app_state = Arc::new(workspace::AppState {
            client: client.clone(),
            user_store: user_store.clone(),
            workspace_store,
            languages: Arc::new(language_registry),
            fs: fs.clone(),
            build_window_options: |_, _, _| Default::default(),
            node_runtime: FakeNodeRuntime::new(),
        });

        cx.update(|cx| {
            theme::init(theme::LoadThemes::JustBase, cx);
            Project::init(&client, cx);
            client::init(&client, cx);
            language::init(cx);
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
            call::init(client.clone(), user_store.clone(), cx);
            channel::init(&client, user_store.clone(), cx);
            notifications::init(client.clone(), user_store, cx);
            collab_ui::init(&app_state, cx);
            file_finder::init(cx);
            menu::init();
            settings::KeymapFile::load_asset("keymaps/default-macos.json", cx).unwrap();
        });

        client
            .authenticate_and_connect(false, &cx.to_async())
            .await
            .unwrap();

        let client = TestClient {
            app_state,
            username: name.to_string(),
            channel_store: cx.read(ChannelStore::global).clone(),
            notification_store: cx.read(NotificationStore::global).clone(),
            state: Default::default(),
        };
        client.wait_for_current_user(cx).await;
        client
    }

    pub fn disconnect_client(&self, peer_id: PeerId) {
        self.connection_killers
            .lock()
            .remove(&peer_id)
            .unwrap()
            .store(true, SeqCst);
    }

    pub fn simulate_long_connection_interruption(
        &self,
        peer_id: PeerId,
        deterministic: BackgroundExecutor,
    ) {
        self.forbid_connections();
        self.disconnect_client(peer_id);
        deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
        self.allow_connections();
        deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
        deterministic.run_until_parked();
    }

    pub fn forbid_connections(&self) {
        self.forbid_connections.store(true, SeqCst);
    }

    pub fn allow_connections(&self) {
        self.forbid_connections.store(false, SeqCst);
    }

    pub async fn make_contacts(&self, clients: &mut [(&TestClient, &mut TestAppContext)]) {
        for ix in 1..clients.len() {
            let (left, right) = clients.split_at_mut(ix);
            let (client_a, cx_a) = left.last_mut().unwrap();
            for (client_b, cx_b) in right {
                client_a
                    .app_state
                    .user_store
                    .update(*cx_a, |store, cx| {
                        store.request_contact(client_b.user_id().unwrap(), cx)
                    })
                    .await
                    .unwrap();
                cx_a.executor().run_until_parked();
                client_b
                    .app_state
                    .user_store
                    .update(*cx_b, |store, cx| {
                        store.respond_to_contact_request(client_a.user_id().unwrap(), true, cx)
                    })
                    .await
                    .unwrap();
            }
        }
    }

    pub async fn make_channel(
        &self,
        channel: &str,
        parent: Option<ChannelId>,
        admin: (&TestClient, &mut TestAppContext),
        members: &mut [(&TestClient, &mut TestAppContext)],
    ) -> ChannelId {
        let (_, admin_cx) = admin;
        let channel_id = admin_cx
            .read(ChannelStore::global)
            .update(admin_cx, |channel_store, cx| {
                channel_store.create_channel(channel, parent, cx)
            })
            .await
            .unwrap();

        for (member_client, member_cx) in members {
            admin_cx
                .read(ChannelStore::global)
                .update(admin_cx, |channel_store, cx| {
                    channel_store.invite_member(
                        channel_id,
                        member_client.user_id().unwrap(),
                        ChannelRole::Member,
                        cx,
                    )
                })
                .await
                .unwrap();

            admin_cx.executor().run_until_parked();

            member_cx
                .read(ChannelStore::global)
                .update(*member_cx, |channels, cx| {
                    channels.respond_to_channel_invite(channel_id, true, cx)
                })
                .await
                .unwrap();
        }

        channel_id
    }

    pub async fn make_public_channel(
        &self,
        channel: &str,
        client: &TestClient,
        cx: &mut TestAppContext,
    ) -> ChannelId {
        let channel_id = self
            .make_channel(channel, None, (client, cx), &mut [])
            .await;

        client
            .channel_store()
            .update(cx, |channel_store, cx| {
                channel_store.set_channel_visibility(
                    channel_id,
                    proto::ChannelVisibility::Public,
                    cx,
                )
            })
            .await
            .unwrap();

        channel_id
    }

    pub async fn make_channel_tree(
        &self,
        channels: &[(&str, Option<&str>)],
        creator: (&TestClient, &mut TestAppContext),
    ) -> Vec<ChannelId> {
        let mut observed_channels = HashMap::default();
        let mut result = Vec::new();
        for (channel, parent) in channels {
            let id;
            if let Some(parent) = parent {
                if let Some(parent_id) = observed_channels.get(parent) {
                    id = self
                        .make_channel(channel, Some(*parent_id), (creator.0, creator.1), &mut [])
                        .await;
                } else {
                    panic!(
                        "Edge {}->{} referenced before {} was created",
                        parent, channel, parent
                    )
                }
            } else {
                id = self
                    .make_channel(channel, None, (creator.0, creator.1), &mut [])
                    .await;
            }

            observed_channels.insert(channel, id);
            result.push(id);
        }

        result
    }

    pub async fn create_room(&self, clients: &mut [(&TestClient, &mut TestAppContext)]) {
        self.make_contacts(clients).await;

        let (left, right) = clients.split_at_mut(1);
        let (_client_a, cx_a) = &mut left[0];
        let active_call_a = cx_a.read(ActiveCall::global);

        for (client_b, cx_b) in right {
            let user_id_b = client_b.current_user_id(cx_b).to_proto();
            active_call_a
                .update(*cx_a, |call, cx| call.invite(user_id_b, None, cx))
                .await
                .unwrap();

            cx_b.executor().run_until_parked();
            let active_call_b = cx_b.read(ActiveCall::global);
            active_call_b
                .update(*cx_b, |call, cx| call.accept_incoming(cx))
                .await
                .unwrap();
        }
    }

    pub async fn build_app_state(
        test_db: &TestDb,
        fake_server: &live_kit_client::TestServer,
    ) -> Arc<AppState> {
        Arc::new(AppState {
            db: test_db.db().clone(),
            live_kit_client: Some(Arc::new(fake_server.create_api_client())),
            blob_store_client: None,
            clickhouse_client: None,
            config: Config {
                http_port: 0,
                database_url: "".into(),
                database_max_connections: 0,
                api_token: "".into(),
                invite_link_prefix: "".into(),
                live_kit_server: None,
                live_kit_key: None,
                live_kit_secret: None,
                rust_log: None,
                log_json: None,
                zed_environment: "test".into(),
                blob_store_url: None,
                blob_store_region: None,
                blob_store_access_key: None,
                blob_store_secret_key: None,
                blob_store_bucket: None,
                clickhouse_url: None,
                clickhouse_user: None,
                clickhouse_password: None,
                clickhouse_database: None,
                zed_client_checksum_seed: None,
                slack_panics_webhook: None,
            },
        })
    }
}

impl Deref for TestServer {
    type Target = Server;

    fn deref(&self) -> &Self::Target {
        &self.server
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.server.teardown();
        self.test_live_kit_server.teardown().unwrap();
    }
}

impl Deref for TestClient {
    type Target = Arc<Client>;

    fn deref(&self) -> &Self::Target {
        &self.app_state.client
    }
}

impl TestClient {
    pub fn fs(&self) -> &FakeFs {
        self.app_state.fs.as_fake()
    }

    pub fn channel_store(&self) -> &Model<ChannelStore> {
        &self.channel_store
    }

    pub fn notification_store(&self) -> &Model<NotificationStore> {
        &self.notification_store
    }

    pub fn user_store(&self) -> &Model<UserStore> {
        &self.app_state.user_store
    }

    pub fn language_registry(&self) -> &Arc<LanguageRegistry> {
        &self.app_state.languages
    }

    pub fn client(&self) -> &Arc<Client> {
        &self.app_state.client
    }

    pub fn current_user_id(&self, cx: &TestAppContext) -> UserId {
        UserId::from_proto(
            self.app_state
                .user_store
                .read_with(cx, |user_store, _| user_store.current_user().unwrap().id),
        )
    }

    pub async fn wait_for_current_user(&self, cx: &TestAppContext) {
        let mut authed_user = self
            .app_state
            .user_store
            .read_with(cx, |user_store, _| user_store.watch_current_user());
        while authed_user.next().await.unwrap().is_none() {}
    }

    pub async fn clear_contacts(&self, cx: &mut TestAppContext) {
        self.app_state
            .user_store
            .update(cx, |store, _| store.clear_contacts())
            .await;
    }

    pub fn local_projects(&self) -> impl Deref<Target = Vec<Model<Project>>> + '_ {
        Ref::map(self.state.borrow(), |state| &state.local_projects)
    }

    pub fn remote_projects(&self) -> impl Deref<Target = Vec<Model<Project>>> + '_ {
        Ref::map(self.state.borrow(), |state| &state.remote_projects)
    }

    pub fn local_projects_mut(&self) -> impl DerefMut<Target = Vec<Model<Project>>> + '_ {
        RefMut::map(self.state.borrow_mut(), |state| &mut state.local_projects)
    }

    pub fn remote_projects_mut(&self) -> impl DerefMut<Target = Vec<Model<Project>>> + '_ {
        RefMut::map(self.state.borrow_mut(), |state| &mut state.remote_projects)
    }

    pub fn buffers_for_project<'a>(
        &'a self,
        project: &Model<Project>,
    ) -> impl DerefMut<Target = HashSet<Model<language::Buffer>>> + 'a {
        RefMut::map(self.state.borrow_mut(), |state| {
            state.buffers.entry(project.clone()).or_default()
        })
    }

    pub fn buffers(
        &self,
    ) -> impl DerefMut<Target = HashMap<Model<Project>, HashSet<Model<language::Buffer>>>> + '_
    {
        RefMut::map(self.state.borrow_mut(), |state| &mut state.buffers)
    }

    pub fn channel_buffers(&self) -> impl DerefMut<Target = HashSet<Model<ChannelBuffer>>> + '_ {
        RefMut::map(self.state.borrow_mut(), |state| &mut state.channel_buffers)
    }

    pub fn summarize_contacts(&self, cx: &TestAppContext) -> ContactsSummary {
        self.app_state
            .user_store
            .read_with(cx, |store, _| ContactsSummary {
                current: store
                    .contacts()
                    .iter()
                    .map(|contact| contact.user.github_login.clone())
                    .collect(),
                outgoing_requests: store
                    .outgoing_contact_requests()
                    .iter()
                    .map(|user| user.github_login.clone())
                    .collect(),
                incoming_requests: store
                    .incoming_contact_requests()
                    .iter()
                    .map(|user| user.github_login.clone())
                    .collect(),
            })
    }

    pub async fn build_local_project(
        &self,
        root_path: impl AsRef<Path>,
        cx: &mut TestAppContext,
    ) -> (Model<Project>, WorktreeId) {
        let project = self.build_empty_local_project(cx);
        let (worktree, _) = project
            .update(cx, |p, cx| {
                p.find_or_create_local_worktree(root_path, true, cx)
            })
            .await
            .unwrap();
        worktree
            .read_with(cx, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        (project, worktree.read_with(cx, |tree, _| tree.id()))
    }

    pub async fn build_test_project(&self, cx: &mut TestAppContext) -> Model<Project> {
        self.fs()
            .insert_tree(
                "/a",
                json!({
                    "1.txt": "one\none\none",
                    "2.js": "function two() { return 2; }",
                    "3.rs": "mod test",
                }),
            )
            .await;
        self.build_local_project("/a", cx).await.0
    }

    pub async fn host_workspace(
        &self,
        workspace: &View<Workspace>,
        channel_id: ChannelId,
        cx: &mut VisualTestContext,
    ) {
        cx.update(|cx| {
            let active_call = ActiveCall::global(cx);
            active_call.update(cx, |call, cx| call.join_channel(channel_id, cx))
        })
        .await
        .unwrap();
        cx.update(|cx| {
            let active_call = ActiveCall::global(cx);
            let project = workspace.read(cx).project().clone();
            active_call.update(cx, |call, cx| call.share_project(project, cx))
        })
        .await
        .unwrap();
        cx.executor().run_until_parked();
    }

    pub async fn join_workspace<'a>(
        &'a self,
        channel_id: ChannelId,
        cx: &'a mut TestAppContext,
    ) -> (View<Workspace>, &'a mut VisualTestContext) {
        cx.update(|cx| workspace::join_channel(channel_id, self.app_state.clone(), None, cx))
            .await
            .unwrap();
        cx.run_until_parked();

        self.active_workspace(cx)
    }

    pub fn build_empty_local_project(&self, cx: &mut TestAppContext) -> Model<Project> {
        cx.update(|cx| {
            Project::local(
                self.client().clone(),
                self.app_state.node_runtime.clone(),
                self.app_state.user_store.clone(),
                self.app_state.languages.clone(),
                self.app_state.fs.clone(),
                cx,
            )
        })
    }

    pub async fn build_remote_project(
        &self,
        host_project_id: u64,
        guest_cx: &mut TestAppContext,
    ) -> Model<Project> {
        let active_call = guest_cx.read(ActiveCall::global);
        let room = active_call.read_with(guest_cx, |call, _| call.room().unwrap().clone());
        room.update(guest_cx, |room, cx| {
            room.join_project(
                host_project_id,
                self.app_state.languages.clone(),
                self.app_state.fs.clone(),
                cx,
            )
        })
        .await
        .unwrap()
    }

    pub fn build_workspace<'a>(
        &'a self,
        project: &Model<Project>,
        cx: &'a mut TestAppContext,
    ) -> (View<Workspace>, &'a mut VisualTestContext) {
        cx.add_window_view(|cx| {
            cx.activate_window();
            Workspace::new(0, project.clone(), self.app_state.clone(), cx)
        })
    }

    pub async fn build_test_workspace<'a>(
        &'a self,
        cx: &'a mut TestAppContext,
    ) -> (View<Workspace>, &'a mut VisualTestContext) {
        let project = self.build_test_project(cx).await;
        cx.add_window_view(|cx| {
            cx.activate_window();
            Workspace::new(0, project.clone(), self.app_state.clone(), cx)
        })
    }

    pub fn active_workspace<'a>(
        &'a self,
        cx: &'a mut TestAppContext,
    ) -> (View<Workspace>, &'a mut VisualTestContext) {
        let window = cx.update(|cx| cx.active_window().unwrap().downcast::<Workspace>().unwrap());

        let view = window.root_view(cx).unwrap();
        let cx = VisualTestContext::from_window(*window.deref(), cx).as_mut();
        // it might be nice to try and cleanup these at the end of each test.
        (view, cx)
    }
}

pub fn open_channel_notes(
    channel_id: ChannelId,
    cx: &mut VisualTestContext,
) -> Task<anyhow::Result<View<ChannelView>>> {
    let window = cx.update(|cx| cx.active_window().unwrap().downcast::<Workspace>().unwrap());
    let view = window.root_view(cx).unwrap();

    cx.update(|cx| ChannelView::open(channel_id, None, view.clone(), cx))
}

impl Drop for TestClient {
    fn drop(&mut self) {
        self.app_state.client.teardown();
    }
}
