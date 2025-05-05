pub mod extension;
pub mod registry;

use std::{path::Path, sync::Arc};

use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet};
use context_server::{ContextServer, ContextServerId};
use gpui::{App, AsyncApp, Context, Entity, EventEmitter, Subscription, Task, WeakEntity, actions};
use registry::ContextServerDescriptorRegistry;
use settings::{Settings as _, SettingsStore};
use util::ResultExt as _;

use crate::{
    project_settings::{ContextServerConfiguration, ProjectSettings},
    worktree_store::WorktreeStore,
};

pub fn init(cx: &mut App) {
    extension::init(cx);
}

actions!(context_server, [Restart]);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextServerStatus {
    Starting,
    Running,
    Stopped,
    Error(Arc<str>),
}

impl ContextServerStatus {
    fn from_state(state: &ContextServerState) -> Self {
        match state {
            ContextServerState::Starting { .. } => ContextServerStatus::Starting,
            ContextServerState::Running { .. } => ContextServerStatus::Running,
            ContextServerState::Stopped { error, .. } => {
                if let Some(error) = error {
                    ContextServerStatus::Error(error.clone())
                } else {
                    ContextServerStatus::Stopped
                }
            }
        }
    }
}

enum ContextServerState {
    Starting {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        _task: Task<()>,
    },
    Running {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
    },
    Stopped {
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        error: Option<Arc<str>>,
    },
}

impl ContextServerState {
    pub fn server(&self) -> Arc<ContextServer> {
        match self {
            ContextServerState::Starting { server, .. } => server.clone(),
            ContextServerState::Running { server, .. } => server.clone(),
            ContextServerState::Stopped { server, .. } => server.clone(),
        }
    }

    pub fn configuration(&self) -> Arc<ContextServerConfiguration> {
        match self {
            ContextServerState::Starting { configuration, .. } => configuration.clone(),
            ContextServerState::Running { configuration, .. } => configuration.clone(),
            ContextServerState::Stopped { configuration, .. } => configuration.clone(),
        }
    }
}

pub type ContextServerFactory =
    Box<dyn Fn(ContextServerId, Arc<ContextServerConfiguration>) -> Arc<ContextServer>>;

pub struct ContextServerStore {
    servers: HashMap<ContextServerId, ContextServerState>,
    worktree_store: Entity<WorktreeStore>,
    registry: Entity<ContextServerDescriptorRegistry>,
    update_servers_task: Option<Task<Result<()>>>,
    context_server_factory: Option<ContextServerFactory>,
    needs_server_update: bool,
    _subscriptions: Vec<Subscription>,
}

pub enum Event {
    ServerStatusChanged {
        server_id: ContextServerId,
        status: ContextServerStatus,
    },
}

impl EventEmitter<Event> for ContextServerStore {}

impl ContextServerStore {
    pub fn new(worktree_store: Entity<WorktreeStore>, cx: &mut Context<Self>) -> Self {
        Self::new_internal(
            true,
            None,
            ContextServerDescriptorRegistry::default_global(cx),
            worktree_store,
            cx,
        )
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(
        registry: Entity<ContextServerDescriptorRegistry>,
        worktree_store: Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_internal(false, None, registry, worktree_store, cx)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_maintain_server_loop(
        context_server_factory: ContextServerFactory,
        registry: Entity<ContextServerDescriptorRegistry>,
        worktree_store: Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_internal(
            true,
            Some(context_server_factory),
            registry,
            worktree_store,
            cx,
        )
    }

    fn new_internal(
        maintain_server_loop: bool,
        context_server_factory: Option<ContextServerFactory>,
        registry: Entity<ContextServerDescriptorRegistry>,
        worktree_store: Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = if maintain_server_loop {
            vec![
                cx.observe(&registry, |this, _registry, cx| {
                    this.available_context_servers_changed(cx);
                }),
                cx.observe_global::<SettingsStore>(|this, cx| {
                    this.available_context_servers_changed(cx);
                }),
            ]
        } else {
            Vec::new()
        };

        let mut this = Self {
            _subscriptions: subscriptions,
            worktree_store,
            registry,
            needs_server_update: false,
            servers: HashMap::default(),
            update_servers_task: None,
            context_server_factory,
        };
        if maintain_server_loop {
            this.available_context_servers_changed(cx);
        }
        this
    }

    pub fn get_server(&self, id: &ContextServerId) -> Option<Arc<ContextServer>> {
        self.servers.get(id).map(|state| state.server())
    }

    pub fn get_running_server(&self, id: &ContextServerId) -> Option<Arc<ContextServer>> {
        if let Some(ContextServerState::Running { server, .. }) = self.servers.get(id) {
            Some(server.clone())
        } else {
            None
        }
    }

    pub fn status_for_server(&self, id: &ContextServerId) -> Option<ContextServerStatus> {
        self.servers.get(id).map(ContextServerStatus::from_state)
    }

    pub fn all_server_ids(&self) -> Vec<ContextServerId> {
        self.servers.keys().cloned().collect()
    }

    pub fn running_servers(&self) -> Vec<Arc<ContextServer>> {
        self.servers
            .values()
            .filter_map(|state| {
                if let ContextServerState::Running { server, .. } = state {
                    Some(server.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn start_server(
        &mut self,
        server: Arc<ContextServer>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let location = self
            .worktree_store
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .map(|worktree| settings::SettingsLocation {
                worktree_id: worktree.read(cx).id(),
                path: Path::new(""),
            });
        let settings = ProjectSettings::get(location, cx);
        let configuration = settings
            .context_servers
            .get(&server.id().0)
            .context("Failed to load context server configuration from settings")?
            .clone();

        self.run_server(server, Arc::new(configuration), cx);
        Ok(())
    }

    pub fn stop_server(&mut self, id: &ContextServerId, cx: &mut Context<Self>) -> Result<()> {
        let Some(state) = self.servers.remove(id) else {
            return Err(anyhow::anyhow!("Context server not found"));
        };

        let server = state.server();
        let configuration = state.configuration();
        let mut result = Ok(());
        if let ContextServerState::Running { server, .. } = &state {
            result = server.stop();
        }
        drop(state);

        self.update_server_state(
            id.clone(),
            ContextServerState::Stopped {
                configuration,
                server,
                error: None,
            },
            cx,
        );

        result
    }

    pub fn restart_server(&mut self, id: &ContextServerId, cx: &mut Context<Self>) -> Result<()> {
        if let Some(state) = self.servers.get(&id) {
            let configuration = state.configuration();

            self.stop_server(&state.server().id(), cx)?;
            let new_server = self.create_context_server(id.clone(), configuration.clone())?;
            self.run_server(new_server, configuration, cx);
        }
        Ok(())
    }

    fn run_server(
        &mut self,
        server: Arc<ContextServer>,
        configuration: Arc<ContextServerConfiguration>,
        cx: &mut Context<Self>,
    ) {
        let id = server.id();
        if matches!(
            self.servers.get(&id),
            Some(ContextServerState::Starting { .. } | ContextServerState::Running { .. })
        ) {
            self.stop_server(&id, cx).log_err();
        }

        let task = cx.spawn({
            let id = server.id();
            let server = server.clone();
            let configuration = configuration.clone();
            async move |this, cx| {
                match server.clone().start(&cx).await {
                    Ok(_) => {
                        log::info!("Started {} context server", id);
                        debug_assert!(server.client().is_some());

                        this.update(cx, |this, cx| {
                            this.update_server_state(
                                id.clone(),
                                ContextServerState::Running {
                                    server,
                                    configuration,
                                },
                                cx,
                            )
                        })
                        .log_err()
                    }
                    Err(err) => {
                        log::error!("{} context server failed to start: {}", id, err);
                        this.update(cx, |this, cx| {
                            this.update_server_state(
                                id.clone(),
                                ContextServerState::Stopped {
                                    configuration,
                                    server,
                                    error: Some(err.to_string().into()),
                                },
                                cx,
                            )
                        })
                        .log_err()
                    }
                };
            }
        });

        self.update_server_state(
            id.clone(),
            ContextServerState::Starting {
                configuration,
                _task: task,
                server,
            },
            cx,
        );
    }

    fn create_context_server(
        &self,
        id: ContextServerId,
        configuration: Arc<ContextServerConfiguration>,
    ) -> Result<Arc<ContextServer>> {
        if let Some(factory) = self.context_server_factory.as_ref() {
            Ok(factory(id, configuration))
        } else {
            let command = configuration
                .command
                .clone()
                .context("Missing command to run context server")?;
            Ok(Arc::new(ContextServer::stdio(id, command)))
        }
    }

    fn update_server_state(
        &mut self,
        id: ContextServerId,
        state: ContextServerState,
        cx: &mut Context<Self>,
    ) {
        let status = ContextServerStatus::from_state(&state);
        self.servers.insert(id.clone(), state);
        cx.emit(Event::ServerStatusChanged {
            server_id: id,
            status,
        });
    }

    fn available_context_servers_changed(&mut self, cx: &mut Context<Self>) {
        if self.update_servers_task.is_some() {
            self.needs_server_update = true;
        } else {
            self.update_servers_task = Some(cx.spawn(async move |this, cx| {
                this.update(cx, |this, _| {
                    this.needs_server_update = false;
                })?;

                if let Err(err) = Self::maintain_servers(this.clone(), cx).await {
                    log::error!("Error maintaining context servers: {}", err);
                }

                this.update(cx, |this, cx| {
                    this.update_servers_task.take();
                    if this.needs_server_update {
                        this.available_context_servers_changed(cx);
                    }
                })?;

                Ok(())
            }));
        }
    }

    async fn maintain_servers(this: WeakEntity<Self>, cx: &mut AsyncApp) -> Result<()> {
        let mut desired_servers = HashMap::default();

        let (registry, worktree_store) = this.update(cx, |this, cx| {
            let location = this
                .worktree_store
                .read(cx)
                .visible_worktrees(cx)
                .next()
                .map(|worktree| settings::SettingsLocation {
                    worktree_id: worktree.read(cx).id(),
                    path: Path::new(""),
                });
            let settings = ProjectSettings::get(location, cx);
            desired_servers = settings.context_servers.clone();

            (this.registry.clone(), this.worktree_store.clone())
        })?;

        for (id, descriptor) in
            registry.read_with(cx, |registry, _| registry.context_server_descriptors())?
        {
            let config = desired_servers.entry(id).or_default();
            if config.command.is_none() {
                if let Some(extension_command) = descriptor
                    .command(worktree_store.clone(), &cx)
                    .await
                    .log_err()
                {
                    config.command = Some(extension_command);
                }
            }
        }

        let mut servers_to_start = Vec::new();
        let mut servers_to_stop = HashSet::default();

        this.update(cx, |this, _cx| {
            for server_id in this.servers.keys() {
                if !desired_servers.contains_key(&server_id.0) {
                    servers_to_stop.insert(server_id.clone());
                }
            }

            for (id, config) in desired_servers {
                let id = ContextServerId(id.clone());

                let existing_config = this.servers.get(&id).map(|state| state.configuration());
                if existing_config.as_deref() != Some(&config) {
                    let config = Arc::new(config);
                    if let Some(server) = this
                        .create_context_server(id.clone(), config.clone())
                        .log_err()
                    {
                        servers_to_start.push((server, config));
                        if this.servers.contains_key(&id) {
                            servers_to_stop.insert(id);
                        }
                    }
                }
            }
        })?;

        for id in servers_to_stop {
            this.update(cx, |this, cx| this.stop_server(&id, cx).ok())?;
        }

        for (server, config) in servers_to_start {
            this.update(cx, |this, cx| this.run_server(server, config, cx))
                .log_err();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FakeFs, Project, project_settings::ProjectSettings};
    use context_server::{
        transport::Transport,
        types::{
            self, Implementation, InitializeResponse, ProtocolVersion, RequestType,
            ServerCapabilities,
        },
    };
    use futures::{Stream, StreamExt as _, lock::Mutex};
    use gpui::{AppContext, BackgroundExecutor, TestAppContext, UpdateGlobal as _};
    use serde_json::json;
    use std::{cell::RefCell, pin::Pin, rc::Rc};
    use util::path;

    #[gpui::test]
    async fn test_context_server_status(cx: &mut TestAppContext) {
        const SERVER_1_ID: &'static str = "mcp-1";
        const SERVER_2_ID: &'static str = "mcp-2";

        let (_fs, project) = setup_context_server_test(
            cx,
            json!({"code.rs": ""}),
            vec![
                (SERVER_1_ID.into(), ContextServerConfiguration::default()),
                (SERVER_2_ID.into(), ContextServerConfiguration::default()),
            ],
        )
        .await;

        let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
        let store = cx.new(|cx| {
            ContextServerStore::test(registry.clone(), project.read(cx).worktree_store(), cx)
        });

        let server_1_id = ContextServerId("mcp-1".into());
        let server_2_id = ContextServerId("mcp-2".into());

        let transport_1 =
            Arc::new(FakeTransport::new(
                cx.executor(),
                |_, request_type, _| match request_type {
                    Some(RequestType::Initialize) => {
                        Some(create_initialize_response("mcp-1".to_string()))
                    }
                    _ => None,
                },
            ));

        let transport_2 =
            Arc::new(FakeTransport::new(
                cx.executor(),
                |_, request_type, _| match request_type {
                    Some(RequestType::Initialize) => {
                        Some(create_initialize_response("mcp-2".to_string()))
                    }
                    _ => None,
                },
            ));

        let server_1 = Arc::new(ContextServer::new(server_1_id.clone(), transport_1.clone()));
        let server_2 = Arc::new(ContextServer::new(server_2_id.clone(), transport_2.clone()));

        store
            .update(cx, |store, cx| store.start_server(server_1, cx))
            .unwrap();

        cx.run_until_parked();

        cx.update(|cx| {
            assert_eq!(
                store.read(cx).status_for_server(&server_1_id),
                Some(ContextServerStatus::Running)
            );
            assert_eq!(store.read(cx).status_for_server(&server_2_id), None);
        });

        store
            .update(cx, |store, cx| store.start_server(server_2.clone(), cx))
            .unwrap();

        cx.run_until_parked();

        cx.update(|cx| {
            assert_eq!(
                store.read(cx).status_for_server(&server_1_id),
                Some(ContextServerStatus::Running)
            );
            assert_eq!(
                store.read(cx).status_for_server(&server_2_id),
                Some(ContextServerStatus::Running)
            );
        });

        store
            .update(cx, |store, cx| store.stop_server(&server_2_id, cx))
            .unwrap();

        cx.update(|cx| {
            assert_eq!(
                store.read(cx).status_for_server(&server_1_id),
                Some(ContextServerStatus::Running)
            );
            assert_eq!(
                store.read(cx).status_for_server(&server_2_id),
                Some(ContextServerStatus::Stopped)
            );
        });
    }

    #[gpui::test]
    async fn test_context_server_status_events(cx: &mut TestAppContext) {
        const SERVER_1_ID: &'static str = "mcp-1";
        const SERVER_2_ID: &'static str = "mcp-2";

        let (_fs, project) = setup_context_server_test(
            cx,
            json!({"code.rs": ""}),
            vec![
                (SERVER_1_ID.into(), ContextServerConfiguration::default()),
                (SERVER_2_ID.into(), ContextServerConfiguration::default()),
            ],
        )
        .await;

        let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
        let store = cx.new(|cx| {
            ContextServerStore::test(registry.clone(), project.read(cx).worktree_store(), cx)
        });

        let server_1_id = ContextServerId("mcp-1".into());
        let server_2_id = ContextServerId("mcp-2".into());

        let transport_1 =
            Arc::new(FakeTransport::new(
                cx.executor(),
                |_, request_type, _| match request_type {
                    Some(RequestType::Initialize) => {
                        Some(create_initialize_response("mcp-1".to_string()))
                    }
                    _ => None,
                },
            ));

        let transport_2 =
            Arc::new(FakeTransport::new(
                cx.executor(),
                |_, request_type, _| match request_type {
                    Some(RequestType::Initialize) => {
                        Some(create_initialize_response("mcp-2".to_string()))
                    }
                    _ => None,
                },
            ));

        let server_1 = Arc::new(ContextServer::new(server_1_id.clone(), transport_1.clone()));
        let server_2 = Arc::new(ContextServer::new(server_2_id.clone(), transport_2.clone()));

        let _server_events = assert_server_events(
            &store,
            vec![
                (server_1_id.clone(), ContextServerStatus::Starting),
                (server_1_id.clone(), ContextServerStatus::Running),
                (server_2_id.clone(), ContextServerStatus::Starting),
                (server_2_id.clone(), ContextServerStatus::Running),
                (server_2_id.clone(), ContextServerStatus::Stopped),
            ],
            cx,
        );

        store
            .update(cx, |store, cx| store.start_server(server_1, cx))
            .unwrap();

        cx.run_until_parked();

        store
            .update(cx, |store, cx| store.start_server(server_2.clone(), cx))
            .unwrap();

        cx.run_until_parked();

        store
            .update(cx, |store, cx| store.stop_server(&server_2_id, cx))
            .unwrap();
    }

    #[gpui::test(iterations = 25)]
    async fn test_context_server_concurrent_starts(cx: &mut TestAppContext) {
        const SERVER_1_ID: &'static str = "mcp-1";

        let (_fs, project) = setup_context_server_test(
            cx,
            json!({"code.rs": ""}),
            vec![(SERVER_1_ID.into(), ContextServerConfiguration::default())],
        )
        .await;

        let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
        let store = cx.new(|cx| {
            ContextServerStore::test(registry.clone(), project.read(cx).worktree_store(), cx)
        });

        let server_id = ContextServerId(SERVER_1_ID.into());

        let transport_1 =
            Arc::new(FakeTransport::new(
                cx.executor(),
                |_, request_type, _| match request_type {
                    Some(RequestType::Initialize) => {
                        Some(create_initialize_response(SERVER_1_ID.to_string()))
                    }
                    _ => None,
                },
            ));

        let transport_2 =
            Arc::new(FakeTransport::new(
                cx.executor(),
                |_, request_type, _| match request_type {
                    Some(RequestType::Initialize) => {
                        Some(create_initialize_response(SERVER_1_ID.to_string()))
                    }
                    _ => None,
                },
            ));

        let server_with_same_id_1 = Arc::new(ContextServer::new(server_id.clone(), transport_1));
        let server_with_same_id_2 = Arc::new(ContextServer::new(server_id.clone(), transport_2));

        // If we start another server with the same id, we should report that we stopped the previous one
        let _server_events = assert_server_events(
            &store,
            vec![
                (server_id.clone(), ContextServerStatus::Starting),
                (server_id.clone(), ContextServerStatus::Stopped),
                (server_id.clone(), ContextServerStatus::Starting),
                (server_id.clone(), ContextServerStatus::Running),
            ],
            cx,
        );

        store
            .update(cx, |store, cx| {
                store.start_server(server_with_same_id_1.clone(), cx)
            })
            .unwrap();
        store
            .update(cx, |store, cx| {
                store.start_server(server_with_same_id_2.clone(), cx)
            })
            .unwrap();
        cx.update(|cx| {
            assert_eq!(
                store.read(cx).status_for_server(&server_id),
                Some(ContextServerStatus::Starting)
            );
        });

        cx.run_until_parked();

        cx.update(|cx| {
            assert_eq!(
                store.read(cx).status_for_server(&server_id),
                Some(ContextServerStatus::Running)
            );
        });
    }

    #[gpui::test(iterations = 25)]
    async fn test_context_server_maintain_servers_loop(cx: &mut TestAppContext) {
        const SERVER_1_ID: &'static str = "mcp-1";
        const SERVER_2_ID: &'static str = "mcp-2";

        let server_1_id = ContextServerId(SERVER_1_ID.into());
        let server_2_id = ContextServerId(SERVER_2_ID.into());

        let (_fs, project) = setup_context_server_test(
            cx,
            json!({"code.rs": ""}),
            vec![(
                SERVER_1_ID.into(),
                ContextServerConfiguration {
                    command: None,
                    settings: Some(json!({
                        "somevalue": true
                    })),
                },
            )],
        )
        .await;

        let executor = cx.executor();
        let registry = cx.new(|_| ContextServerDescriptorRegistry::new());
        let store = cx.new(|cx| {
            ContextServerStore::test_maintain_server_loop(
                Box::new(move |id, _| {
                    let transport = FakeTransport::new(executor.clone(), {
                        let id = id.0.clone();
                        move |_, request_type, _| match request_type {
                            Some(RequestType::Initialize) => {
                                Some(create_initialize_response(id.clone().to_string()))
                            }
                            _ => None,
                        }
                    });
                    Arc::new(ContextServer::new(id.clone(), Arc::new(transport)))
                }),
                registry.clone(),
                project.read(cx).worktree_store(),
                cx,
            )
        });

        // Ensure that mcp-1 starts up
        {
            let _server_events = assert_server_events(
                &store,
                vec![
                    (server_1_id.clone(), ContextServerStatus::Starting),
                    (server_1_id.clone(), ContextServerStatus::Running),
                ],
                cx,
            );
            cx.run_until_parked();
        }

        // Ensure that mcp-1 is restarted when the configuration was changed
        {
            let _server_events = assert_server_events(
                &store,
                vec![
                    (server_1_id.clone(), ContextServerStatus::Stopped),
                    (server_1_id.clone(), ContextServerStatus::Starting),
                    (server_1_id.clone(), ContextServerStatus::Running),
                ],
                cx,
            );
            update_context_server_configuration(cx, {
                let server_1_id = server_1_id.0.clone();
                |context_servers| {
                    context_servers.insert(
                        server_1_id,
                        ContextServerConfiguration {
                            command: None,
                            settings: Some(json!({
                                "somevalue": false
                            })),
                        },
                    );
                }
            });

            cx.run_until_parked();
        }

        // Ensure that mcp-1 is not restarted when the configuration was not changed
        {
            let _server_events = assert_server_events(&store, vec![], cx);
            update_context_server_configuration(cx, {
                let server_1_id = server_1_id.0.clone();
                |context_servers| {
                    context_servers.insert(
                        server_1_id,
                        ContextServerConfiguration {
                            command: None,
                            settings: Some(json!({
                                "somevalue": false
                            })),
                        },
                    );
                }
            });

            cx.run_until_parked();
        }

        // Ensure that mcp-2 is started once it is added to the settings
        {
            let _server_events = assert_server_events(
                &store,
                vec![
                    (server_2_id.clone(), ContextServerStatus::Starting),
                    (server_2_id.clone(), ContextServerStatus::Running),
                ],
                cx,
            );
            update_context_server_configuration(cx, {
                let server_2_id = server_2_id.0.clone();
                |context_servers| {
                    context_servers.insert(
                        server_2_id,
                        ContextServerConfiguration {
                            command: None,
                            settings: Some(json!({
                                "somevalue": true
                            })),
                        },
                    );
                }
            });

            cx.run_until_parked();
        }
    }

    fn update_context_server_configuration(
        cx: &mut TestAppContext,
        update_configurations: impl FnOnce(&mut HashMap<Arc<str>, ContextServerConfiguration>),
    ) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<ProjectSettings>(cx, |settings| {
                    update_configurations(&mut settings.context_servers);
                });
            })
        });
    }

    struct ServerEvents {
        received_event_count: Rc<RefCell<usize>>,
        expected_event_count: usize,
        _subscription: Subscription,
    }

    impl Drop for ServerEvents {
        fn drop(&mut self) {
            let actual_event_count = *self.received_event_count.borrow();
            assert_eq!(
                actual_event_count, self.expected_event_count,
                "
                Expected to receive {} context server store events, but received {} events",
                self.expected_event_count, actual_event_count
            );
        }
    }

    fn assert_server_events(
        store: &Entity<ContextServerStore>,
        expected_events: Vec<(ContextServerId, ContextServerStatus)>,
        cx: &mut TestAppContext,
    ) -> ServerEvents {
        cx.update(|cx| {
            let mut ix = 0;
            let received_event_count = Rc::new(RefCell::new(0));
            let expected_event_count = expected_events.len();
            let subscription = cx.subscribe(store, {
                let received_event_count = received_event_count.clone();
                move |_, event, _| match event {
                    Event::ServerStatusChanged {
                        server_id: actual_server_id,
                        status: actual_status,
                    } => {
                        let (expected_server_id, expected_status) = &expected_events[ix];

                        assert_eq!(
                            actual_server_id, expected_server_id,
                            "Expected different server id at index {}",
                            ix
                        );
                        assert_eq!(
                            actual_status, expected_status,
                            "Expected different status at index {}",
                            ix
                        );
                        ix += 1;
                        *received_event_count.borrow_mut() += 1;
                    }
                }
            });
            ServerEvents {
                expected_event_count,
                received_event_count,
                _subscription: subscription,
            }
        })
    }

    async fn setup_context_server_test(
        cx: &mut TestAppContext,
        files: serde_json::Value,
        context_server_configurations: Vec<(Arc<str>, ContextServerConfiguration)>,
    ) -> (Arc<FakeFs>, Entity<Project>) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            let mut settings = ProjectSettings::get_global(cx).clone();
            for (id, config) in context_server_configurations {
                settings.context_servers.insert(id, config);
            }
            ProjectSettings::override_global(settings, cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), files).await;
        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        (fs, project)
    }

    fn create_initialize_response(server_name: String) -> serde_json::Value {
        serde_json::to_value(&InitializeResponse {
            protocol_version: ProtocolVersion(types::LATEST_PROTOCOL_VERSION.to_string()),
            server_info: Implementation {
                name: server_name,
                version: "1.0.0".to_string(),
            },
            capabilities: ServerCapabilities::default(),
            meta: None,
        })
        .unwrap()
    }

    struct FakeTransport {
        on_request: Arc<
            dyn Fn(u64, Option<RequestType>, serde_json::Value) -> Option<serde_json::Value>
                + Send
                + Sync,
        >,
        tx: futures::channel::mpsc::UnboundedSender<String>,
        rx: Arc<Mutex<futures::channel::mpsc::UnboundedReceiver<String>>>,
        executor: BackgroundExecutor,
    }

    impl FakeTransport {
        fn new(
            executor: BackgroundExecutor,
            on_request: impl Fn(
                u64,
                Option<RequestType>,
                serde_json::Value,
            ) -> Option<serde_json::Value>
            + 'static
            + Send
            + Sync,
        ) -> Self {
            let (tx, rx) = futures::channel::mpsc::unbounded();
            Self {
                on_request: Arc::new(on_request),
                tx,
                rx: Arc::new(Mutex::new(rx)),
                executor,
            }
        }
    }

    #[async_trait::async_trait]
    impl Transport for FakeTransport {
        async fn send(&self, message: String) -> Result<()> {
            if let Ok(msg) = serde_json::from_str::<serde_json::Value>(&message) {
                let id = msg.get("id").and_then(|id| id.as_u64()).unwrap_or(0);

                if let Some(method) = msg.get("method") {
                    let request_type = method
                        .as_str()
                        .and_then(|method| types::RequestType::try_from(method).ok());
                    if let Some(payload) = (self.on_request.as_ref())(id, request_type, msg) {
                        let response = serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": payload
                        });

                        self.tx
                            .unbounded_send(response.to_string())
                            .map_err(|e| anyhow::anyhow!("Failed to send message: {}", e))?;
                    }
                }
            }
            Ok(())
        }

        fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
            let rx = self.rx.clone();
            let executor = self.executor.clone();
            Box::pin(futures::stream::unfold(rx, move |rx| {
                let executor = executor.clone();
                async move {
                    let mut rx_guard = rx.lock().await;
                    executor.simulate_random_delay().await;
                    if let Some(message) = rx_guard.next().await {
                        drop(rx_guard);
                        Some((message, rx))
                    } else {
                        None
                    }
                }
            }))
        }

        fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
            Box::pin(futures::stream::empty())
        }
    }
}
