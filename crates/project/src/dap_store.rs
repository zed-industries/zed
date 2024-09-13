use anyhow::Context as _;
use collections::{HashMap, HashSet};
use dap::SourceBreakpoint;
use dap::{
    client::{DebugAdapterClient, DebugAdapterClientId},
    transport::Payload,
};
use gpui::{EventEmitter, ModelContext, Task};
use language::{Buffer, BufferSnapshot};
use settings::WorktreeId;
use std::{
    collections::BTreeMap,
    future::Future,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use task::DebugAdapterConfig;
use text::Point;
use util::ResultExt as _;

use crate::ProjectPath;

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
    breakpoints: BTreeMap<ProjectPath, HashSet<Breakpoint>>,
}

impl EventEmitter<DapStoreEvent> for DapStore {}

impl DapStore {
    pub fn new(cx: &mut ModelContext<Self>) -> Self {
        cx.on_app_quit(Self::shutdown_clients).detach();

        Self {
            next_client_id: Default::default(),
            clients: Default::default(),
            breakpoints: Default::default(),
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

    pub fn breakpoints(&self) -> &BTreeMap<ProjectPath, HashSet<Breakpoint>> {
        &self.breakpoints
    }

    pub fn set_active_breakpoints(&mut self, project_path: &ProjectPath, buffer: &Buffer) {
        let entry = self.breakpoints.remove(project_path).unwrap_or_default();
        let mut set_bp: HashSet<Breakpoint> = HashSet::default();

        for mut bp in entry.into_iter() {
            bp.set_active_position(&buffer);
            set_bp.insert(bp);
        }

        self.breakpoints.insert(project_path.clone(), set_bp);
    }

    pub fn deserialize_breakpoints(
        &mut self,
        worktree_id: WorktreeId,
        serialize_breakpoints: Vec<SerializedBreakpoint>,
    ) {
        for serialize_breakpoint in serialize_breakpoints {
            self.breakpoints
                .entry(ProjectPath {
                    worktree_id,
                    path: serialize_breakpoint.path.clone(),
                })
                .or_default()
                .insert(Breakpoint {
                    active_position: None,
                    cache_position: serialize_breakpoint.position.saturating_sub(1u32),
                });
        }
    }

    pub fn sync_open_breakpoints_to_closed_breakpoints(
        &mut self,
        project_path: &ProjectPath,
        buffer: &mut Buffer,
    ) {
        if let Some(breakpoint_set) = self.breakpoints.remove(project_path) {
            let breakpoint_iter = breakpoint_set.into_iter().map(|mut bp| {
                bp.cache_position = bp.point_for_buffer(&buffer).row;
                bp.active_position = None;
                bp
            });

            self.breakpoints.insert(
                project_path.clone(),
                breakpoint_iter.collect::<HashSet<_>>(),
            );
        }
    }

    pub fn start_client(&mut self, config: DebugAdapterConfig, cx: &mut ModelContext<Self>) {
        let client_id = self.next_client_id();

        let start_client_task = cx.spawn(|this, mut cx| async move {
            let dap_store = this.clone();
            let client = DebugAdapterClient::new(
                client_id,
                config,
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
                    DebugAdapterClientState::Starting(task) => task.await?.shutdown().await.ok(),
                    DebugAdapterClientState::Running(client) => client.shutdown().await.ok(),
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
        cx: &mut ModelContext<Self>,
    ) {
        let Some(debug_client) = self.clients.remove(&client_id) else {
            return;
        };

        cx.emit(DapStoreEvent::DebugClientStopped(client_id));

        cx.background_executor()
            .spawn(async move {
                match debug_client {
                    DebugAdapterClientState::Starting(task) => task.await?.shutdown().await.ok(),
                    DebugAdapterClientState::Running(client) => client.shutdown().await.ok(),
                }
            })
            .detach();
    }

    pub fn toggle_breakpoint_for_buffer(
        &mut self,
        project_path: &ProjectPath,
        breakpoint: Breakpoint,
        buffer_path: PathBuf,
        buffer_snapshot: BufferSnapshot,
        cx: &mut ModelContext<Self>,
    ) {
        let breakpoint_set = self.breakpoints.entry(project_path.clone()).or_default();

        if !breakpoint_set.remove(&breakpoint) {
            breakpoint_set.insert(breakpoint);
        }

        self.send_changed_breakpoints(project_path, buffer_path, buffer_snapshot, cx);
    }

    pub fn send_changed_breakpoints(
        &self,
        project_path: &ProjectPath,
        buffer_path: PathBuf,
        buffer_snapshot: BufferSnapshot,
        cx: &mut ModelContext<Self>,
    ) {
        let clients = self.running_clients().collect::<Vec<_>>();

        if clients.is_empty() {
            return;
        }

        let Some(breakpoints) = self.breakpoints.get(project_path) else {
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Breakpoint {
    pub active_position: Option<text::Anchor>,
    pub cache_position: u32,
}

impl Breakpoint {
    pub fn to_source_breakpoint(&self, buffer: &Buffer) -> SourceBreakpoint {
        let line = self
            .active_position
            .map(|position| buffer.summary_for_anchor::<Point>(&position).row)
            .unwrap_or(self.cache_position) as u64
            + 1u64;

        SourceBreakpoint {
            line,
            condition: None,
            hit_condition: None,
            log_message: None,
            column: None,
            mode: None,
        }
    }

    pub fn set_active_position(&mut self, buffer: &Buffer) {
        if self.active_position.is_none() {
            self.active_position =
                Some(buffer.anchor_at(Point::new(self.cache_position, 0), text::Bias::Left));
        }
    }

    pub fn point_for_buffer(&self, buffer: &Buffer) -> Point {
        self.active_position
            .map(|position| buffer.summary_for_anchor::<Point>(&position))
            .unwrap_or(Point::new(self.cache_position, 0))
    }

    pub fn point_for_buffer_snapshot(&self, buffer_snapshot: &BufferSnapshot) -> Point {
        self.active_position
            .map(|position| buffer_snapshot.summary_for_anchor::<Point>(&position))
            .unwrap_or(Point::new(self.cache_position, 0))
    }

    pub fn source_for_snapshot(&self, snapshot: &BufferSnapshot) -> SourceBreakpoint {
        let line = self
            .active_position
            .map(|position| snapshot.summary_for_anchor::<Point>(&position).row)
            .unwrap_or(self.cache_position) as u64
            + 1u64;

        SourceBreakpoint {
            line,
            condition: None,
            hit_condition: None,
            log_message: None,
            column: None,
            mode: None,
        }
    }

    pub fn to_serialized(&self, buffer: Option<&Buffer>, path: Arc<Path>) -> SerializedBreakpoint {
        match buffer {
            Some(buffer) => SerializedBreakpoint {
                position: self
                    .active_position
                    .map(|position| buffer.summary_for_anchor::<Point>(&position).row + 1u32)
                    .unwrap_or(self.cache_position + 1u32),
                path,
            },
            None => SerializedBreakpoint {
                position: self.cache_position + 1u32,
                path,
            },
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SerializedBreakpoint {
    pub position: u32,
    pub path: Arc<Path>,
}

impl SerializedBreakpoint {
    pub fn to_source_breakpoint(&self) -> SourceBreakpoint {
        SourceBreakpoint {
            line: self.position as u64,
            condition: None,
            hit_condition: None,
            log_message: None,
            column: None,
            mode: None,
        }
    }
}
