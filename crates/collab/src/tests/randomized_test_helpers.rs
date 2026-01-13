use crate::{
    db::{self, NewUserParams, UserId},
    rpc::{CLEANUP_TIMEOUT, RECONNECT_TIMEOUT},
    tests::{TestClient, TestServer},
};
use async_trait::async_trait;
use futures::StreamExt;
use gpui::{BackgroundExecutor, Task, TestAppContext};
use parking_lot::Mutex;
use rand::prelude::*;
use rpc::RECEIVE_TIMEOUT;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use settings::SettingsStore;
use std::sync::OnceLock;
use std::{
    env,
    path::PathBuf,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering::SeqCst},
    },
};

fn plan_load_path() -> &'static Option<PathBuf> {
    static PLAN_LOAD_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();
    PLAN_LOAD_PATH.get_or_init(|| path_env_var("LOAD_PLAN"))
}

fn plan_save_path() -> &'static Option<PathBuf> {
    static PLAN_SAVE_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();
    PLAN_SAVE_PATH.get_or_init(|| path_env_var("SAVE_PLAN"))
}

fn max_peers() -> usize {
    static MAX_PEERS: OnceLock<usize> = OnceLock::new();
    *MAX_PEERS.get_or_init(|| {
        env::var("MAX_PEERS")
            .map(|i| i.parse().expect("invalid `MAX_PEERS` variable"))
            .unwrap_or(3)
    })
}

fn max_operations() -> usize {
    static MAX_OPERATIONS: OnceLock<usize> = OnceLock::new();
    *MAX_OPERATIONS.get_or_init(|| {
        env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10)
    })
}

static LOADED_PLAN_JSON: Mutex<Option<Vec<u8>>> = Mutex::new(None);
static LAST_PLAN: Mutex<Option<Box<dyn Send + FnOnce() -> Vec<u8>>>> = Mutex::new(None);

struct TestPlan<T: RandomizedTest> {
    rng: StdRng,
    replay: bool,
    stored_operations: Vec<(StoredOperation<T::Operation>, Arc<AtomicBool>)>,
    max_operations: usize,
    operation_ix: usize,
    users: Vec<UserTestPlan>,
    next_batch_id: usize,
    allow_server_restarts: bool,
    allow_client_reconnection: bool,
    allow_client_disconnection: bool,
}

pub struct UserTestPlan {
    pub user_id: UserId,
    pub username: String,
    pub allow_client_disconnection: bool,
    next_root_id: usize,
    operation_ix: usize,
    online: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum StoredOperation<T> {
    Server(ServerOperation),
    Client {
        user_id: UserId,
        batch_id: usize,
        operation: T,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum ServerOperation {
    AddConnection {
        user_id: UserId,
    },
    RemoveConnection {
        user_id: UserId,
    },
    BounceConnection {
        user_id: UserId,
    },
    RestartServer,
    MutateClients {
        batch_id: usize,
        #[serde(skip_serializing)]
        #[serde(skip_deserializing)]
        user_ids: Vec<UserId>,
        quiesce: bool,
    },
}

pub enum TestError {
    Inapplicable,
    Other(anyhow::Error),
}

#[async_trait(?Send)]
pub trait RandomizedTest: 'static + Sized {
    type Operation: Send + Clone + Serialize + DeserializeOwned;

    fn generate_operation(
        client: &TestClient,
        rng: &mut StdRng,
        plan: &mut UserTestPlan,
        cx: &TestAppContext,
    ) -> Self::Operation;

    async fn apply_operation(
        client: &TestClient,
        operation: Self::Operation,
        cx: &mut TestAppContext,
    ) -> Result<(), TestError>;

    async fn initialize(server: &mut TestServer, users: &[UserTestPlan]);

    async fn on_client_added(_client: &Rc<TestClient>, _cx: &mut TestAppContext) {}

    async fn on_quiesce(server: &mut TestServer, client: &mut [(Rc<TestClient>, TestAppContext)]);
}

pub async fn run_randomized_test<T: RandomizedTest>(
    cx: &mut TestAppContext,
    executor: BackgroundExecutor,
    rng: StdRng,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let plan = TestPlan::<T>::new(&mut server, rng).await;

    LAST_PLAN.lock().replace({
        let plan = plan.clone();
        Box::new(move || plan.lock().serialize())
    });

    let mut clients = Vec::new();
    let mut client_tasks = Vec::new();
    let mut operation_channels = Vec::new();
    loop {
        let Some((next_operation, applied)) = plan.lock().next_server_operation(&clients) else {
            break;
        };
        applied.store(true, SeqCst);
        let did_apply = TestPlan::apply_server_operation(
            plan.clone(),
            executor.clone(),
            &mut server,
            &mut clients,
            &mut client_tasks,
            &mut operation_channels,
            next_operation,
            cx,
        )
        .await;
        if !did_apply {
            applied.store(false, SeqCst);
        }
    }

    drop(operation_channels);
    executor.start_waiting();
    futures::future::join_all(client_tasks).await;
    executor.finish_waiting();

    executor.run_until_parked();
    T::on_quiesce(&mut server, &mut clients).await;

    for (client, cx) in clients {
        cx.update(|cx| {
            let settings = cx.remove_global::<SettingsStore>();
            cx.clear_globals();
            cx.set_global(settings);
            theme::init(theme::LoadThemes::JustBase, cx);
            drop(client);
        });
    }
    executor.run_until_parked();

    if let Some(path) = plan_save_path() {
        eprintln!("saved test plan to path {:?}", path);
        std::fs::write(path, plan.lock().serialize()).unwrap();
    }
}

pub fn save_randomized_test_plan() {
    if let Some(serialize_plan) = LAST_PLAN.lock().take()
        && let Some(path) = plan_save_path()
    {
        eprintln!("saved test plan to path {:?}", path);
        std::fs::write(path, serialize_plan()).unwrap();
    }
}

impl<T: RandomizedTest> TestPlan<T> {
    pub async fn new(server: &mut TestServer, mut rng: StdRng) -> Arc<Mutex<Self>> {
        let allow_server_restarts = rng.random_bool(0.7);
        let allow_client_reconnection = rng.random_bool(0.7);
        let allow_client_disconnection = rng.random_bool(0.1);

        let mut users = Vec::new();
        for ix in 0..max_peers() {
            let username = format!("user-{}", ix + 1);
            let user_id = server
                .app_state
                .db
                .create_user(
                    &format!("{username}@example.com"),
                    None,
                    false,
                    NewUserParams {
                        github_login: username.clone(),
                        github_user_id: ix as i32,
                    },
                )
                .await
                .unwrap()
                .user_id;
            users.push(UserTestPlan {
                user_id,
                username,
                online: false,
                next_root_id: 0,
                operation_ix: 0,
                allow_client_disconnection,
            });
        }

        T::initialize(server, &users).await;

        let plan = Arc::new(Mutex::new(Self {
            replay: false,
            allow_server_restarts,
            allow_client_reconnection,
            allow_client_disconnection,
            stored_operations: Vec::new(),
            operation_ix: 0,
            next_batch_id: 0,
            max_operations: max_operations(),
            users,
            rng,
        }));

        if let Some(path) = plan_load_path() {
            let json = LOADED_PLAN_JSON
                .lock()
                .get_or_insert_with(|| {
                    eprintln!("loaded test plan from path {:?}", path);
                    std::fs::read(path).unwrap()
                })
                .clone();
            plan.lock().deserialize(json);
        }

        plan
    }

    fn deserialize(&mut self, json: Vec<u8>) {
        let stored_operations: Vec<StoredOperation<T::Operation>> =
            serde_json::from_slice(&json).unwrap();
        self.replay = true;
        self.stored_operations = stored_operations
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, mut operation)| {
                let did_apply = Arc::new(AtomicBool::new(false));
                if let StoredOperation::Server(ServerOperation::MutateClients {
                    batch_id: current_batch_id,
                    user_ids,
                    ..
                }) = &mut operation
                {
                    assert!(user_ids.is_empty());
                    user_ids.extend(stored_operations[i + 1..].iter().filter_map(|operation| {
                        if let StoredOperation::Client {
                            user_id, batch_id, ..
                        } = operation
                            && batch_id == current_batch_id
                        {
                            return Some(user_id);
                        }
                        None
                    }));
                    user_ids.sort_unstable();
                }
                (operation, did_apply)
            })
            .collect()
    }

    fn serialize(&mut self) -> Vec<u8> {
        // Format each operation as one line
        let mut json = Vec::new();
        json.push(b'[');
        for (operation, applied) in &self.stored_operations {
            if !applied.load(SeqCst) {
                continue;
            }
            if json.len() > 1 {
                json.push(b',');
            }
            json.extend_from_slice(b"\n  ");
            serde_json::to_writer(&mut json, operation).unwrap();
        }
        json.extend_from_slice(b"\n]\n");
        json
    }

    fn next_server_operation(
        &mut self,
        clients: &[(Rc<TestClient>, TestAppContext)],
    ) -> Option<(ServerOperation, Arc<AtomicBool>)> {
        if self.replay {
            while let Some(stored_operation) = self.stored_operations.get(self.operation_ix) {
                self.operation_ix += 1;
                if let (StoredOperation::Server(operation), applied) = stored_operation {
                    return Some((operation.clone(), applied.clone()));
                }
            }
            None
        } else {
            let operation = self.generate_server_operation(clients)?;
            let applied = Arc::new(AtomicBool::new(false));
            self.stored_operations
                .push((StoredOperation::Server(operation.clone()), applied.clone()));
            Some((operation, applied))
        }
    }

    fn next_client_operation(
        &mut self,
        client: &TestClient,
        current_batch_id: usize,
        cx: &TestAppContext,
    ) -> Option<(T::Operation, Arc<AtomicBool>)> {
        let current_user_id = client.current_user_id(cx);
        let user_ix = self
            .users
            .iter()
            .position(|user| user.user_id == current_user_id)
            .unwrap();
        let user_plan = &mut self.users[user_ix];

        if self.replay {
            while let Some(stored_operation) = self.stored_operations.get(user_plan.operation_ix) {
                user_plan.operation_ix += 1;
                if let (
                    StoredOperation::Client {
                        user_id, operation, ..
                    },
                    applied,
                ) = stored_operation
                    && user_id == &current_user_id
                {
                    return Some((operation.clone(), applied.clone()));
                }
            }
            None
        } else {
            if self.operation_ix == self.max_operations {
                return None;
            }
            self.operation_ix += 1;
            let operation = T::generate_operation(
                client,
                &mut self.rng,
                self.users
                    .iter_mut()
                    .find(|user| user.user_id == current_user_id)
                    .unwrap(),
                cx,
            );
            let applied = Arc::new(AtomicBool::new(false));
            self.stored_operations.push((
                StoredOperation::Client {
                    user_id: current_user_id,
                    batch_id: current_batch_id,
                    operation: operation.clone(),
                },
                applied.clone(),
            ));
            Some((operation, applied))
        }
    }

    fn generate_server_operation(
        &mut self,
        clients: &[(Rc<TestClient>, TestAppContext)],
    ) -> Option<ServerOperation> {
        if self.operation_ix == self.max_operations {
            return None;
        }

        Some(loop {
            break match self.rng.random_range(0..100) {
                0..=29 if clients.len() < self.users.len() => {
                    let user = self
                        .users
                        .iter()
                        .filter(|u| !u.online)
                        .choose(&mut self.rng)
                        .unwrap();
                    self.operation_ix += 1;
                    ServerOperation::AddConnection {
                        user_id: user.user_id,
                    }
                }
                30..=34 if clients.len() > 1 && self.allow_client_disconnection => {
                    let (client, cx) = &clients[self.rng.random_range(0..clients.len())];
                    let user_id = client.current_user_id(cx);
                    self.operation_ix += 1;
                    ServerOperation::RemoveConnection { user_id }
                }
                35..=39 if clients.len() > 1 && self.allow_client_reconnection => {
                    let (client, cx) = &clients[self.rng.random_range(0..clients.len())];
                    let user_id = client.current_user_id(cx);
                    self.operation_ix += 1;
                    ServerOperation::BounceConnection { user_id }
                }
                40..=44 if self.allow_server_restarts && clients.len() > 1 => {
                    self.operation_ix += 1;
                    ServerOperation::RestartServer
                }
                _ if !clients.is_empty() => {
                    let count = self
                        .rng
                        .random_range(1..10)
                        .min(self.max_operations - self.operation_ix);
                    let batch_id = util::post_inc(&mut self.next_batch_id);
                    let mut user_ids = (0..count)
                        .map(|_| {
                            let ix = self.rng.random_range(0..clients.len());
                            let (client, cx) = &clients[ix];
                            client.current_user_id(cx)
                        })
                        .collect::<Vec<_>>();
                    user_ids.sort_unstable();
                    ServerOperation::MutateClients {
                        user_ids,
                        batch_id,
                        quiesce: self.rng.random_bool(0.7),
                    }
                }
                _ => continue,
            };
        })
    }

    async fn apply_server_operation(
        plan: Arc<Mutex<Self>>,
        deterministic: BackgroundExecutor,
        server: &mut TestServer,
        clients: &mut Vec<(Rc<TestClient>, TestAppContext)>,
        client_tasks: &mut Vec<Task<()>>,
        operation_channels: &mut Vec<futures::channel::mpsc::UnboundedSender<usize>>,
        operation: ServerOperation,
        cx: &mut TestAppContext,
    ) -> bool {
        match operation {
            ServerOperation::AddConnection { user_id } => {
                let username;
                {
                    let mut plan = plan.lock();
                    let user = plan.user(user_id);
                    if user.online {
                        return false;
                    }
                    user.online = true;
                    username = user.username.clone();
                };
                log::info!("adding new connection for {}", username);

                let mut client_cx = cx.new_app();

                let (operation_tx, operation_rx) = futures::channel::mpsc::unbounded();
                let client = Rc::new(server.create_client(&mut client_cx, &username).await);
                operation_channels.push(operation_tx);
                clients.push((client.clone(), client_cx.clone()));

                let foreground_executor = client_cx.foreground_executor().clone();
                let simulate_client =
                    Self::simulate_client(plan.clone(), client, operation_rx, client_cx);
                client_tasks.push(foreground_executor.spawn(simulate_client));

                log::info!("added connection for {}", username);
            }

            ServerOperation::RemoveConnection {
                user_id: removed_user_id,
            } => {
                log::info!("simulating full disconnection of user {}", removed_user_id);
                let client_ix = clients
                    .iter()
                    .position(|(client, cx)| client.current_user_id(cx) == removed_user_id);
                let Some(client_ix) = client_ix else {
                    return false;
                };
                let user_connection_ids = server
                    .connection_pool
                    .lock()
                    .user_connection_ids(removed_user_id)
                    .collect::<Vec<_>>();
                assert_eq!(user_connection_ids.len(), 1);
                let removed_peer_id = user_connection_ids[0].into();
                let (client, client_cx) = clients.remove(client_ix);
                let client_task = client_tasks.remove(client_ix);
                operation_channels.remove(client_ix);
                server.forbid_connections();
                server.disconnect_client(removed_peer_id);
                deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
                deterministic.start_waiting();
                log::info!("waiting for user {} to exit...", removed_user_id);
                client_task.await;
                deterministic.finish_waiting();
                server.allow_connections();

                for project in client.dev_server_projects().iter() {
                    project.read_with(&client_cx, |project, cx| {
                        assert!(
                            project.is_disconnected(cx),
                            "project {:?} should be read only",
                            project.remote_id()
                        )
                    });
                }

                for (client, cx) in clients {
                    let contacts = server
                        .app_state
                        .db
                        .get_contacts(client.current_user_id(cx))
                        .await
                        .unwrap();
                    let pool = server.connection_pool.lock();
                    for contact in contacts {
                        if let db::Contact::Accepted { user_id, busy, .. } = contact
                            && user_id == removed_user_id
                        {
                            assert!(!pool.is_user_online(user_id));
                            assert!(!busy);
                        }
                    }
                }

                log::info!("{} removed", client.username);
                plan.lock().user(removed_user_id).online = false;
                client_cx.update(|cx| {
                    cx.clear_globals();
                    drop(client);
                });
            }

            ServerOperation::BounceConnection { user_id } => {
                log::info!("simulating temporary disconnection of user {}", user_id);
                let user_connection_ids = server
                    .connection_pool
                    .lock()
                    .user_connection_ids(user_id)
                    .collect::<Vec<_>>();
                if user_connection_ids.is_empty() {
                    return false;
                }
                assert_eq!(user_connection_ids.len(), 1);
                let peer_id = user_connection_ids[0].into();
                server.disconnect_client(peer_id);
                deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
            }

            ServerOperation::RestartServer => {
                log::info!("simulating server restart");
                server.reset().await;
                deterministic.advance_clock(RECEIVE_TIMEOUT);
                server.start().await.unwrap();
                deterministic.advance_clock(CLEANUP_TIMEOUT);
                let environment = &server.app_state.config.zed_environment;
                let (stale_room_ids, _) = server
                    .app_state
                    .db
                    .stale_server_resource_ids(environment, server.id())
                    .await
                    .unwrap();
                assert_eq!(stale_room_ids, vec![]);
            }

            ServerOperation::MutateClients {
                user_ids,
                batch_id,
                quiesce,
            } => {
                let mut applied = false;
                for user_id in user_ids {
                    let client_ix = clients
                        .iter()
                        .position(|(client, cx)| client.current_user_id(cx) == user_id);
                    let Some(client_ix) = client_ix else { continue };
                    applied = true;
                    if let Err(err) = operation_channels[client_ix].unbounded_send(batch_id) {
                        log::error!("error signaling user {user_id}: {err}");
                    }
                }

                if quiesce && applied {
                    deterministic.run_until_parked();
                    T::on_quiesce(server, clients).await;
                }

                return applied;
            }
        }
        true
    }

    async fn simulate_client(
        plan: Arc<Mutex<Self>>,
        client: Rc<TestClient>,
        mut operation_rx: futures::channel::mpsc::UnboundedReceiver<usize>,
        mut cx: TestAppContext,
    ) {
        T::on_client_added(&client, &mut cx).await;

        while let Some(batch_id) = operation_rx.next().await {
            let Some((operation, applied)) =
                plan.lock().next_client_operation(&client, batch_id, &cx)
            else {
                break;
            };
            applied.store(true, SeqCst);
            match T::apply_operation(&client, operation, &mut cx).await {
                Ok(()) => {}
                Err(TestError::Inapplicable) => {
                    applied.store(false, SeqCst);
                    log::info!("skipped operation");
                }
                Err(TestError::Other(error)) => {
                    log::error!("{} error: {}", client.username, error);
                }
            }
            cx.executor().simulate_random_delay().await;
        }
        log::info!("{}: done", client.username);
    }

    fn user(&mut self, user_id: UserId) -> &mut UserTestPlan {
        self.users
            .iter_mut()
            .find(|user| user.user_id == user_id)
            .unwrap()
    }
}

impl UserTestPlan {
    pub fn next_root_dir_name(&mut self) -> String {
        let user_id = self.user_id;
        let root_id = util::post_inc(&mut self.next_root_id);
        format!("dir-{user_id}-{root_id}")
    }
}

impl From<anyhow::Error> for TestError {
    fn from(value: anyhow::Error) -> Self {
        Self::Other(value)
    }
}

fn path_env_var(name: &str) -> Option<PathBuf> {
    let value = env::var(name).ok()?;
    let mut path = PathBuf::from(value);
    if path.is_relative() {
        let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        abs_path.pop();
        abs_path.pop();
        abs_path.push(path);
        path = abs_path
    }
    Some(path)
}
