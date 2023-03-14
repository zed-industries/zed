use crate::{
    db::{NewUserParams, TestDb, UserId},
    executor::Executor,
    rpc::{Server, CLEANUP_TIMEOUT},
    AppState,
};
use anyhow::anyhow;
use call::ActiveCall;
use client::{
    self, proto::PeerId, test::FakeHttpClient, Client, Connection, Credentials,
    EstablishConnectionError, UserStore,
};
use collections::{HashMap, HashSet};
use fs::FakeFs;
use futures::{channel::oneshot, StreamExt as _};
use gpui::{
    executor::Deterministic, test::EmptyView, ModelHandle, Task, TestAppContext, ViewHandle,
};
use language::LanguageRegistry;
use parking_lot::Mutex;
use project::{Project, WorktreeId};
use settings::Settings;
use std::{
    env,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use theme::ThemeRegistry;
use workspace::Workspace;

mod integration_tests;
mod randomized_integration_tests;

struct TestServer {
    app_state: Arc<AppState>,
    server: Arc<Server>,
    connection_killers: Arc<Mutex<HashMap<PeerId, Arc<AtomicBool>>>>,
    forbid_connections: Arc<AtomicBool>,
    _test_db: TestDb,
    test_live_kit_server: Arc<live_kit_client::TestServer>,
}

impl TestServer {
    async fn start(deterministic: &Arc<Deterministic>) -> Self {
        static NEXT_LIVE_KIT_SERVER_ID: AtomicUsize = AtomicUsize::new(0);

        let use_postgres = env::var("USE_POSTGRES").ok();
        let use_postgres = use_postgres.as_deref();
        let test_db = if use_postgres == Some("true") || use_postgres == Some("1") {
            TestDb::postgres(deterministic.build_background())
        } else {
            TestDb::sqlite(deterministic.build_background())
        };
        let live_kit_server_id = NEXT_LIVE_KIT_SERVER_ID.fetch_add(1, SeqCst);
        let live_kit_server = live_kit_client::TestServer::create(
            format!("http://livekit.{}.test", live_kit_server_id),
            format!("devkey-{}", live_kit_server_id),
            format!("secret-{}", live_kit_server_id),
            deterministic.build_background(),
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
            Executor::Deterministic(deterministic.build_background()),
        );
        server.start().await.unwrap();
        // Advance clock to ensure the server's cleanup task is finished.
        deterministic.advance_clock(CLEANUP_TIMEOUT);
        Self {
            app_state,
            server,
            connection_killers: Default::default(),
            forbid_connections: Default::default(),
            _test_db: test_db,
            test_live_kit_server: live_kit_server,
        }
    }

    async fn reset(&self) {
        self.app_state.db.reset();
        let epoch = self
            .app_state
            .db
            .create_server(&self.app_state.config.zed_environment)
            .await
            .unwrap();
        self.server.reset(epoch);
    }

    async fn create_client(&mut self, cx: &mut TestAppContext, name: &str) -> TestClient {
        cx.update(|cx| {
            cx.set_global(Settings::test(cx));
        });

        let http = FakeHttpClient::with_404_response();
        let user_id = if let Ok(Some(user)) = self.app_state.db.get_user_by_github_login(name).await
        {
            user.id
        } else {
            self.app_state
                .db
                .create_user(
                    &format!("{name}@example.com"),
                    false,
                    NewUserParams {
                        github_login: name.into(),
                        github_user_id: 0,
                        invite_count: 0,
                    },
                )
                .await
                .expect("creating user failed")
                .user_id
        };
        let client_name = name.to_string();
        let mut client = cx.read(|cx| Client::new(http.clone(), cx));
        let server = self.server.clone();
        let db = self.app_state.db.clone();
        let connection_killers = self.connection_killers.clone();
        let forbid_connections = self.forbid_connections.clone();

        Arc::get_mut(&mut client)
            .unwrap()
            .set_id(user_id.0 as usize)
            .override_authenticate(move |cx| {
                cx.spawn(|_| async move {
                    let access_token = "the-token".to_string();
                    Ok(Credentials {
                        user_id: user_id.0 as u64,
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
                            Connection::in_memory(cx.background());
                        let (connection_id_tx, connection_id_rx) = oneshot::channel();
                        let user = db
                            .get_user_by_id(user_id)
                            .await
                            .expect("retrieving user failed")
                            .unwrap();
                        cx.background()
                            .spawn(server.handle_connection(
                                server_conn,
                                client_name,
                                user,
                                Some(connection_id_tx),
                                Executor::Deterministic(cx.background()),
                            ))
                            .detach();
                        let connection_id = connection_id_rx.await.unwrap();
                        connection_killers
                            .lock()
                            .insert(connection_id.into(), killed);
                        Ok(client_conn)
                    }
                })
            });

        let fs = FakeFs::new(cx.background());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http, cx));
        let app_state = Arc::new(workspace::AppState {
            client: client.clone(),
            user_store: user_store.clone(),
            languages: Arc::new(LanguageRegistry::new(Task::ready(()))),
            themes: ThemeRegistry::new((), cx.font_cache()),
            fs: fs.clone(),
            build_window_options: |_, _, _| Default::default(),
            initialize_workspace: |_, _, _| unimplemented!(),
            dock_default_item_factory: |_, _| None,
            background_actions: || &[],
        });

        Project::init(&client);
        cx.update(|cx| {
            workspace::init(app_state.clone(), cx);
            call::init(client.clone(), user_store.clone(), cx);
        });

        client
            .authenticate_and_connect(false, &cx.to_async())
            .await
            .unwrap();

        let client = TestClient {
            client,
            username: name.to_string(),
            local_projects: Default::default(),
            remote_projects: Default::default(),
            next_root_dir_id: 0,
            user_store,
            fs,
            language_registry: Arc::new(LanguageRegistry::test()),
            buffers: Default::default(),
        };
        client.wait_for_current_user(cx).await;
        client
    }

    fn disconnect_client(&self, peer_id: PeerId) {
        self.connection_killers
            .lock()
            .remove(&peer_id)
            .unwrap()
            .store(true, SeqCst);
    }

    fn forbid_connections(&self) {
        self.forbid_connections.store(true, SeqCst);
    }

    fn allow_connections(&self) {
        self.forbid_connections.store(false, SeqCst);
    }

    async fn make_contacts(&self, clients: &mut [(&TestClient, &mut TestAppContext)]) {
        for ix in 1..clients.len() {
            let (left, right) = clients.split_at_mut(ix);
            let (client_a, cx_a) = left.last_mut().unwrap();
            for (client_b, cx_b) in right {
                client_a
                    .user_store
                    .update(*cx_a, |store, cx| {
                        store.request_contact(client_b.user_id().unwrap(), cx)
                    })
                    .await
                    .unwrap();
                cx_a.foreground().run_until_parked();
                client_b
                    .user_store
                    .update(*cx_b, |store, cx| {
                        store.respond_to_contact_request(client_a.user_id().unwrap(), true, cx)
                    })
                    .await
                    .unwrap();
            }
        }
    }

    async fn create_room(&self, clients: &mut [(&TestClient, &mut TestAppContext)]) {
        self.make_contacts(clients).await;

        let (left, right) = clients.split_at_mut(1);
        let (_client_a, cx_a) = &mut left[0];
        let active_call_a = cx_a.read(ActiveCall::global);

        for (client_b, cx_b) in right {
            let user_id_b = client_b.current_user_id(*cx_b).to_proto();
            active_call_a
                .update(*cx_a, |call, cx| call.invite(user_id_b, None, cx))
                .await
                .unwrap();

            cx_b.foreground().run_until_parked();
            let active_call_b = cx_b.read(ActiveCall::global);
            active_call_b
                .update(*cx_b, |call, cx| call.accept_incoming(cx))
                .await
                .unwrap();
        }
    }

    async fn build_app_state(
        test_db: &TestDb,
        fake_server: &live_kit_client::TestServer,
    ) -> Arc<AppState> {
        Arc::new(AppState {
            db: test_db.db().clone(),
            live_kit_client: Some(Arc::new(fake_server.create_api_client())),
            config: Default::default(),
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

struct TestClient {
    client: Arc<Client>,
    username: String,
    local_projects: Vec<ModelHandle<Project>>,
    remote_projects: Vec<ModelHandle<Project>>,
    next_root_dir_id: usize,
    pub user_store: ModelHandle<UserStore>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<FakeFs>,
    buffers: HashMap<ModelHandle<Project>, HashSet<ModelHandle<language::Buffer>>>,
}

impl Deref for TestClient {
    type Target = Arc<Client>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

struct ContactsSummary {
    pub current: Vec<String>,
    pub outgoing_requests: Vec<String>,
    pub incoming_requests: Vec<String>,
}

impl TestClient {
    pub fn current_user_id(&self, cx: &TestAppContext) -> UserId {
        UserId::from_proto(
            self.user_store
                .read_with(cx, |user_store, _| user_store.current_user().unwrap().id),
        )
    }

    async fn wait_for_current_user(&self, cx: &TestAppContext) {
        let mut authed_user = self
            .user_store
            .read_with(cx, |user_store, _| user_store.watch_current_user());
        while authed_user.next().await.unwrap().is_none() {}
    }

    async fn clear_contacts(&self, cx: &mut TestAppContext) {
        self.user_store
            .update(cx, |store, _| store.clear_contacts())
            .await;
    }

    fn summarize_contacts(&self, cx: &TestAppContext) -> ContactsSummary {
        self.user_store.read_with(cx, |store, _| ContactsSummary {
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

    async fn build_local_project(
        &self,
        root_path: impl AsRef<Path>,
        cx: &mut TestAppContext,
    ) -> (ModelHandle<Project>, WorktreeId) {
        let project = cx.update(|cx| {
            Project::local(
                self.client.clone(),
                self.user_store.clone(),
                self.language_registry.clone(),
                self.fs.clone(),
                cx,
            )
        });
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

    async fn build_remote_project(
        &self,
        host_project_id: u64,
        guest_cx: &mut TestAppContext,
    ) -> ModelHandle<Project> {
        let active_call = guest_cx.read(ActiveCall::global);
        let room = active_call.read_with(guest_cx, |call, _| call.room().unwrap().clone());
        room.update(guest_cx, |room, cx| {
            room.join_project(
                host_project_id,
                self.language_registry.clone(),
                self.fs.clone(),
                cx,
            )
        })
        .await
        .unwrap()
    }

    fn build_workspace(
        &self,
        project: &ModelHandle<Project>,
        cx: &mut TestAppContext,
    ) -> ViewHandle<Workspace> {
        let (_, root_view) = cx.add_window(|_| EmptyView);
        cx.add_view(&root_view, |cx| Workspace::test_new(project.clone(), cx))
    }

    fn create_new_root_dir(&mut self) -> PathBuf {
        format!(
            "/{}-root-{}",
            self.username,
            util::post_inc(&mut self.next_root_dir_id)
        )
        .into()
    }
}

impl Drop for TestClient {
    fn drop(&mut self) {
        self.client.teardown();
    }
}
