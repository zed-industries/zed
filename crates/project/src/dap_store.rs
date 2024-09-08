use anyhow::Context as _;
use collections::{HashMap, HashSet};
use dap::{
    client::{Breakpoint, DebugAdapterClient, DebugAdapterClientId, SerializedBreakpoint},
    transport::Payload,
};
use gpui::{EventEmitter, ModelContext, Subscription, Task};
use language::{Buffer, BufferSnapshot};
use multi_buffer::MultiBufferSnapshot;
use std::{
    collections::BTreeMap,
    future::Future,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use task::DebugAdapterConfig;
use text::{Bias, BufferId, Point};
use util::ResultExt as _;

use crate::{Item, ProjectPath};

pub enum DapStoreEvent {
    DebugClientStarted(DebugAdapterClientId),
    DebugClientStopped(DebugAdapterClientId),
    DebugClientEvent {
        client_id: DebugAdapterClientId,
        payload: Payload,
    },
}

pub enum DebugAdapterClientState {
    Starting(Task<Option<Arc<DebugAdapterClient>>>),
    Running(Arc<DebugAdapterClient>),
}

pub struct DapStore {
    next_client_id: AtomicUsize,
    clients: HashMap<DebugAdapterClientId, DebugAdapterClientState>,
    open_breakpoints: BTreeMap<BufferId, HashSet<Breakpoint>>,
    /// All breakpoints that belong to this project but are in closed files
    pub closed_breakpoints: BTreeMap<ProjectPath, Vec<SerializedBreakpoint>>,
    _subscription: Vec<Subscription>,
}

impl EventEmitter<DapStoreEvent> for DapStore {}

impl DapStore {
    pub fn new(cx: &mut ModelContext<Self>) -> Self {
        Self {
            next_client_id: Default::default(),
            clients: Default::default(),
            open_breakpoints: Default::default(),
            closed_breakpoints: Default::default(),
            _subscription: vec![cx.on_app_quit(Self::shutdown_clients)],
        }
    }

    pub fn next_client_id(&self) -> DebugAdapterClientId {
        DebugAdapterClientId(self.next_client_id.fetch_add(1, SeqCst))
    }

    pub fn running_clients(&self) -> impl Iterator<Item = Arc<DebugAdapterClient>> + '_ {
        self.clients.values().filter_map(|state| match state {
            DebugAdapterClientState::Starting(_) => None,
            DebugAdapterClientState::Running(client) => Some(client.clone()),
        })
    }

    pub fn client_by_id(&self, id: DebugAdapterClientId) -> Option<Arc<DebugAdapterClient>> {
        self.clients.get(&id).and_then(|state| match state {
            DebugAdapterClientState::Starting(_) => None,
            DebugAdapterClientState::Running(client) => Some(client.clone()),
        })
    }

    pub fn open_breakpoints(&self) -> &BTreeMap<BufferId, HashSet<Breakpoint>> {
        &self.open_breakpoints
    }

    pub fn closed_breakpoints(&self) -> &BTreeMap<ProjectPath, Vec<SerializedBreakpoint>> {
        &self.closed_breakpoints
    }

    pub fn sync_open_breakpoints_to_closed_breakpoints(
        &mut self,
        buffer_id: &BufferId,
        buffer: &mut Buffer,
        cx: &mut ModelContext<Self>,
    ) {
        let Some(breakpoints) = self.open_breakpoints.remove(&buffer_id) else {
            return;
        };

        if let Some(project_path) = buffer.project_path(cx) {
            self.closed_breakpoints
                .entry(project_path.clone())
                .or_default()
                .extend(
                    breakpoints
                        .into_iter()
                        .map(|bp| bp.to_serialized(buffer, project_path.path.clone())),
                );
        }
    }

    pub fn sync_closed_breakpoint_to_open_breakpoint(
        &mut self,
        buffer_id: &BufferId,
        project_path: &ProjectPath,
        snapshot: MultiBufferSnapshot,
    ) {
        let Some(closed_breakpoints) = self.closed_breakpoints.remove(project_path) else {
            return;
        };

        let open_breakpoints = self.open_breakpoints.entry(*buffer_id).or_default();

        for closed_breakpoint in closed_breakpoints {
            // serialized breakpoints start at index one and need to converted
            // to index zero in order to display/work properly with open breakpoints
            let position = snapshot.anchor_at(
                Point::new(closed_breakpoint.position.saturating_sub(1), 0),
                Bias::Left,
            );

            open_breakpoints.insert(Breakpoint { position });
        }
    }

    pub fn start_client(
        &mut self,
        config: DebugAdapterConfig,
        command: String,
        args: Vec<String>,
        cwd: PathBuf,
        request_args: Option<serde_json::Value>,
        cx: &mut ModelContext<Self>,
    ) {
        let client_id = self.next_client_id();

        let start_client_task = cx.spawn(|this, mut cx| async move {
            let dap_store = this.clone();
            let client = DebugAdapterClient::new(
                client_id,
                config,
                &command,
                &args,
                &cwd,
                request_args,
                move |payload, cx| {
                    dap_store
                        .update(cx, |_, cx| {
                            cx.emit(DapStoreEvent::DebugClientEvent { client_id, payload })
                        })
                        .log_err();
                },
                &mut cx,
            )
            .await
            .log_err()?;

            this.update(&mut cx, |store, cx| {
                let handle = store
                    .clients
                    .get_mut(&client_id)
                    .with_context(|| "Failed to find starting debug client")?;

                *handle = DebugAdapterClientState::Running(client.clone());

                cx.emit(DapStoreEvent::DebugClientStarted(client_id));

                anyhow::Ok(())
            })
            .log_err();

            Some(client)
        });

        self.clients.insert(
            client_id,
            DebugAdapterClientState::Starting(start_client_task),
        );
    }

    fn shutdown_clients(&mut self, _: &mut ModelContext<Self>) -> impl Future<Output = ()> {
        let shutdown_futures = self
            .clients
            .drain()
            .map(|(_, client_state)| async {
                match client_state {
                    DebugAdapterClientState::Starting(task) => {
                        task.await?.shutdown(true).await.ok()
                    }
                    DebugAdapterClientState::Running(client) => client.shutdown(true).await.ok(),
                }
            })
            .collect::<Vec<_>>();

        async move {
            futures::future::join_all(shutdown_futures).await;
        }
    }

    pub fn shutdown_client(
        &mut self,
        client_id: DebugAdapterClientId,
        should_terminate: bool,
        cx: &mut ModelContext<Self>,
    ) {
        let Some(debug_client) = self.clients.remove(&client_id) else {
            return;
        };

        cx.emit(DapStoreEvent::DebugClientStopped(client_id));

        cx.background_executor()
            .spawn(async move {
                match debug_client {
                    DebugAdapterClientState::Starting(task) => {
                        task.await?.shutdown(should_terminate).await.ok()
                    }
                    DebugAdapterClientState::Running(client) => {
                        client.shutdown(should_terminate).await.ok()
                    }
                }
            })
            .detach();
    }

    pub fn toggle_breakpoint_for_buffer(
        &mut self,
        buffer_id: &BufferId,
        breakpoint: Breakpoint,
        buffer_path: PathBuf,
        buffer_snapshot: BufferSnapshot,
        cx: &mut ModelContext<Self>,
    ) {
        let breakpoint_set = self.open_breakpoints.entry(*buffer_id).or_default();

        if !breakpoint_set.remove(&breakpoint) {
            breakpoint_set.insert(breakpoint);
        }

        self.send_changed_breakpoints(buffer_id, buffer_path, buffer_snapshot, cx);
    }

    pub fn send_changed_breakpoints(
        &self,
        buffer_id: &BufferId,
        buffer_path: PathBuf,
        buffer_snapshot: BufferSnapshot,
        cx: &mut ModelContext<Self>,
    ) {
        let clients = self.running_clients().collect::<Vec<_>>();

        if clients.is_empty() {
            return;
        }

        let Some(breakpoints) = self.open_breakpoints.get(buffer_id) else {
            return;
        };

        let source_breakpoints = breakpoints
            .iter()
            .map(|bp| bp.source_for_snapshot(&buffer_snapshot))
            .collect::<Vec<_>>();

        let mut tasks = Vec::new();
        for client in clients {
            let buffer_path = buffer_path.clone();
            let source_breakpoints = source_breakpoints.clone();
            tasks.push(async move {
                client
                    .set_breakpoints(Arc::from(buffer_path), source_breakpoints)
                    .await
            });
        }

        cx.background_executor()
            .spawn(async move { futures::future::join_all(tasks).await })
            .detach()
    }
}
