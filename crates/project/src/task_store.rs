use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;
use collections::HashMap;
use fs::Fs;
use gpui::{App, AsyncApp, Context, Entity, EventEmitter, Task, WeakEntity};
use language::{
    ContextLocation, ContextProvider as _, LanguageToolchainStore, Location,
    proto::{deserialize_anchor, serialize_anchor},
};
use rpc::{AnyProtoClient, TypedEnvelope, proto};
use settings::{InvalidSettingsError, SettingsLocation};
use task::{TaskContext, TaskVariables, VariableName};
use text::{BufferId, OffsetRangeExt};
use util::ResultExt;

use crate::{
    BasicContextProvider, Inventory, ProjectEnvironment, buffer_store::BufferStore,
    worktree_store::WorktreeStore,
};

// platform-dependent warning
pub enum TaskStore {
    Functional(StoreState),
    Noop,
}

pub struct StoreState {
    mode: StoreMode,
    task_inventory: Entity<Inventory>,
    buffer_store: WeakEntity<BufferStore>,
    worktree_store: Entity<WorktreeStore>,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
}

enum StoreMode {
    Local {
        downstream_client: Option<(AnyProtoClient, u64)>,
        environment: Entity<ProjectEnvironment>,
    },
    Remote {
        upstream_client: AnyProtoClient,
        project_id: u64,
    },
}

impl EventEmitter<crate::Event> for TaskStore {}

#[derive(Debug)]
pub enum TaskSettingsLocation<'a> {
    Global(&'a Path),
    Worktree(SettingsLocation<'a>),
}

impl TaskStore {
    pub fn init(client: Option<&AnyProtoClient>) {
        if let Some(client) = client {
            client.add_entity_request_handler(Self::handle_task_context_for_location);
        }
    }

    async fn handle_task_context_for_location(
        store: Entity<Self>,
        envelope: TypedEnvelope<proto::TaskContextForLocation>,
        mut cx: AsyncApp,
    ) -> anyhow::Result<proto::TaskContext> {
        let location = envelope
            .payload
            .location
            .context("no location given for task context handling")?;
        let (buffer_store, is_remote) = store.read_with(&cx, |store, _| {
            Ok(match store {
                TaskStore::Functional(state) => (
                    state.buffer_store.clone(),
                    match &state.mode {
                        StoreMode::Local { .. } => false,
                        StoreMode::Remote { .. } => true,
                    },
                ),
                TaskStore::Noop => {
                    anyhow::bail!("empty task store cannot handle task context requests")
                }
            })
        })??;
        let buffer_store = buffer_store
            .upgrade()
            .context("no buffer store when handling task context request")?;

        let buffer_id = BufferId::new(location.buffer_id).with_context(|| {
            format!(
                "cannot handle task context request for invalid buffer id: {}",
                location.buffer_id
            )
        })?;

        let start = location
            .start
            .and_then(deserialize_anchor)
            .context("missing task context location start")?;
        let end = location
            .end
            .and_then(deserialize_anchor)
            .context("missing task context location end")?;
        let buffer = buffer_store
            .update(&mut cx, |buffer_store, cx| {
                if is_remote {
                    buffer_store.wait_for_remote_buffer(buffer_id, cx)
                } else {
                    Task::ready(
                        buffer_store
                            .get(buffer_id)
                            .with_context(|| format!("no local buffer with id {buffer_id}")),
                    )
                }
            })?
            .await?;

        let location = Location {
            buffer,
            range: start..end,
        };
        let context_task = store.update(&mut cx, |store, cx| {
            let captured_variables = {
                let mut variables = TaskVariables::from_iter(
                    envelope
                        .payload
                        .task_variables
                        .into_iter()
                        .filter_map(|(k, v)| Some((k.parse().log_err()?, v))),
                );

                let snapshot = location.buffer.read(cx).snapshot();
                let range = location.range.to_offset(&snapshot);

                for range in snapshot.runnable_ranges(range) {
                    for (capture_name, value) in range.extra_captures {
                        variables.insert(VariableName::Custom(capture_name.into()), value);
                    }
                }
                variables
            };
            store.task_context_for_location(captured_variables, location, cx)
        })?;
        let task_context = context_task.await.unwrap_or_default();
        Ok(proto::TaskContext {
            project_env: task_context.project_env.into_iter().collect(),
            cwd: task_context
                .cwd
                .map(|cwd| cwd.to_string_lossy().to_string()),
            task_variables: task_context
                .task_variables
                .into_iter()
                .map(|(variable_name, variable_value)| (variable_name.to_string(), variable_value))
                .collect(),
        })
    }

    pub fn local(
        fs: Arc<dyn Fs>,
        buffer_store: WeakEntity<BufferStore>,
        worktree_store: Entity<WorktreeStore>,
        toolchain_store: Arc<dyn LanguageToolchainStore>,
        environment: Entity<ProjectEnvironment>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::Functional(StoreState {
            mode: StoreMode::Local {
                downstream_client: None,
                environment,
            },
            task_inventory: Inventory::new(fs, cx),
            buffer_store,
            toolchain_store,
            worktree_store,
        })
    }

    pub fn remote(
        fs: Arc<dyn Fs>,
        buffer_store: WeakEntity<BufferStore>,
        worktree_store: Entity<WorktreeStore>,
        toolchain_store: Arc<dyn LanguageToolchainStore>,
        upstream_client: AnyProtoClient,
        project_id: u64,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::Functional(StoreState {
            mode: StoreMode::Remote {
                upstream_client,
                project_id,
            },
            task_inventory: Inventory::new(fs, cx),
            buffer_store,
            toolchain_store,
            worktree_store,
        })
    }

    pub fn task_context_for_location(
        &self,
        captured_variables: TaskVariables,
        location: Location,
        cx: &mut App,
    ) -> Task<Option<TaskContext>> {
        match self {
            TaskStore::Functional(state) => match &state.mode {
                StoreMode::Local { environment, .. } => local_task_context_for_location(
                    state.worktree_store.clone(),
                    state.toolchain_store.clone(),
                    environment.clone(),
                    captured_variables,
                    location,
                    cx,
                ),
                StoreMode::Remote {
                    upstream_client,
                    project_id,
                } => remote_task_context_for_location(
                    *project_id,
                    upstream_client.clone(),
                    state.worktree_store.clone(),
                    captured_variables,
                    location,
                    state.toolchain_store.clone(),
                    cx,
                ),
            },
            TaskStore::Noop => Task::ready(None),
        }
    }

    pub fn task_inventory(&self) -> Option<&Entity<Inventory>> {
        match self {
            TaskStore::Functional(state) => Some(&state.task_inventory),
            TaskStore::Noop => None,
        }
    }

    pub fn shared(&mut self, remote_id: u64, new_downstream_client: AnyProtoClient, _cx: &mut App) {
        if let Self::Functional(StoreState {
            mode: StoreMode::Local {
                downstream_client, ..
            },
            ..
        }) = self
        {
            *downstream_client = Some((new_downstream_client, remote_id));
        }
    }

    pub fn unshared(&mut self, _: &mut Context<Self>) {
        if let Self::Functional(StoreState {
            mode: StoreMode::Local {
                downstream_client, ..
            },
            ..
        }) = self
        {
            *downstream_client = None;
        }
    }

    pub(super) fn update_user_tasks(
        &self,
        location: TaskSettingsLocation<'_>,
        raw_tasks_json: Option<&str>,
        cx: &mut Context<Self>,
    ) -> Result<(), InvalidSettingsError> {
        let task_inventory = match self {
            TaskStore::Functional(state) => &state.task_inventory,
            TaskStore::Noop => return Ok(()),
        };
        let raw_tasks_json = raw_tasks_json
            .map(|json| json.trim())
            .filter(|json| !json.is_empty());

        task_inventory.update(cx, |inventory, _| {
            inventory.update_file_based_tasks(location, raw_tasks_json)
        })
    }

    pub(super) fn update_user_debug_scenarios(
        &self,
        location: TaskSettingsLocation<'_>,
        raw_tasks_json: Option<&str>,
        cx: &mut Context<Self>,
    ) -> Result<(), InvalidSettingsError> {
        let task_inventory = match self {
            TaskStore::Functional(state) => &state.task_inventory,
            TaskStore::Noop => return Ok(()),
        };
        let raw_tasks_json = raw_tasks_json
            .map(|json| json.trim())
            .filter(|json| !json.is_empty());

        task_inventory.update(cx, |inventory, _| {
            inventory.update_file_based_scenarios(location, raw_tasks_json)
        })
    }
}

fn local_task_context_for_location(
    worktree_store: Entity<WorktreeStore>,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
    environment: Entity<ProjectEnvironment>,
    captured_variables: TaskVariables,
    location: Location,
    cx: &App,
) -> Task<Option<TaskContext>> {
    let worktree_id = location.buffer.read(cx).file().map(|f| f.worktree_id(cx));
    let worktree_abs_path = worktree_id
        .and_then(|worktree_id| worktree_store.read(cx).worktree_for_id(worktree_id, cx))
        .and_then(|worktree| worktree.read(cx).root_dir());
    let fs = worktree_store.read(cx).fs();

    cx.spawn(async move |cx| {
        let project_env = environment
            .update(cx, |environment, cx| {
                environment.get_buffer_environment(&location.buffer, &worktree_store, cx)
            })
            .ok()?
            .await;

        let mut task_variables = cx
            .update(|cx| {
                combine_task_variables(
                    captured_variables,
                    fs,
                    worktree_store.clone(),
                    location,
                    project_env.clone(),
                    BasicContextProvider::new(worktree_store),
                    toolchain_store,
                    cx,
                )
            })
            .ok()?
            .await
            .log_err()?;
        // Remove all custom entries starting with _, as they're not intended for use by the end user.
        task_variables.sweep();

        Some(TaskContext {
            project_env: project_env.unwrap_or_default(),
            cwd: worktree_abs_path.map(|p| p.to_path_buf()),
            task_variables,
        })
    })
}

fn remote_task_context_for_location(
    project_id: u64,
    upstream_client: AnyProtoClient,
    worktree_store: Entity<WorktreeStore>,
    captured_variables: TaskVariables,
    location: Location,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
    cx: &mut App,
) -> Task<Option<TaskContext>> {
    cx.spawn(async move |cx| {
        // We need to gather a client context, as the headless one may lack certain information (e.g. tree-sitter parsing is disabled there, so symbols are not available).
        let mut remote_context = cx
            .update(|cx| {
                let worktree_root = worktree_root(&worktree_store, &location, cx);

                BasicContextProvider::new(worktree_store).build_context(
                    &TaskVariables::default(),
                    ContextLocation {
                        fs: None,
                        worktree_root,
                        file_location: &location,
                    },
                    None,
                    toolchain_store,
                    cx,
                )
            })
            .ok()?
            .await
            .log_err()
            .unwrap_or_default();
        remote_context.extend(captured_variables);

        let buffer_id = cx
            .update(|cx| location.buffer.read(cx).remote_id().to_proto())
            .ok()?;
        let context_task = upstream_client.request(proto::TaskContextForLocation {
            project_id,
            location: Some(proto::Location {
                buffer_id,
                start: Some(serialize_anchor(&location.range.start)),
                end: Some(serialize_anchor(&location.range.end)),
            }),
            task_variables: remote_context
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        });
        let task_context = context_task.await.log_err()?;
        Some(TaskContext {
            cwd: task_context.cwd.map(PathBuf::from),
            task_variables: task_context
                .task_variables
                .into_iter()
                .filter_map(
                    |(variable_name, variable_value)| match variable_name.parse() {
                        Ok(variable_name) => Some((variable_name, variable_value)),
                        Err(()) => {
                            log::error!("Unknown variable name: {variable_name}");
                            None
                        }
                    },
                )
                .collect(),
            project_env: task_context.project_env.into_iter().collect(),
        })
    })
}

fn worktree_root(
    worktree_store: &Entity<WorktreeStore>,
    location: &Location,
    cx: &mut App,
) -> Option<PathBuf> {
    location
        .buffer
        .read(cx)
        .file()
        .map(|f| f.worktree_id(cx))
        .and_then(|worktree_id| worktree_store.read(cx).worktree_for_id(worktree_id, cx))
        .and_then(|worktree| {
            let worktree = worktree.read(cx);
            if !worktree.is_visible() {
                return None;
            }
            let root_entry = worktree.root_entry()?;
            if !root_entry.is_dir() {
                return None;
            }
            worktree.absolutize(&root_entry.path).ok()
        })
}

fn combine_task_variables(
    mut captured_variables: TaskVariables,
    fs: Option<Arc<dyn Fs>>,
    worktree_store: Entity<WorktreeStore>,
    location: Location,
    project_env: Option<HashMap<String, String>>,
    baseline: BasicContextProvider,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
    cx: &mut App,
) -> Task<anyhow::Result<TaskVariables>> {
    let language_context_provider = location
        .buffer
        .read(cx)
        .language()
        .and_then(|language| language.context_provider());
    cx.spawn(async move |cx| {
        let baseline = cx
            .update(|cx| {
                let worktree_root = worktree_root(&worktree_store, &location, cx);
                baseline.build_context(
                    &captured_variables,
                    ContextLocation {
                        fs: fs.clone(),
                        worktree_root,
                        file_location: &location,
                    },
                    project_env.clone(),
                    toolchain_store.clone(),
                    cx,
                )
            })?
            .await
            .context("building basic default context")?;
        captured_variables.extend(baseline);
        if let Some(provider) = language_context_provider {
            captured_variables.extend(
                cx.update(|cx| {
                    let worktree_root = worktree_root(&worktree_store, &location, cx);
                    provider.build_context(
                        &captured_variables,
                        ContextLocation {
                            fs,
                            worktree_root,
                            file_location: &location,
                        },
                        project_env,
                        toolchain_store,
                        cx,
                    )
                })?
                .await
                .context("building provider context")?,
            );
        }
        Ok(captured_variables)
    })
}
