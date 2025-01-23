use std::{path::PathBuf, sync::Arc};

use anyhow::Context as _;
use collections::HashMap;
use fs::Fs;
use futures::StreamExt as _;
use gpui::{AppContext, AsyncAppContext, EventEmitter, Model, ModelContext, Task, WeakModel};
use language::{
    proto::{deserialize_anchor, serialize_anchor},
    ContextProvider as _, LanguageToolchainStore, Location,
};
use rpc::{proto, AnyProtoClient, TypedEnvelope};
use settings::{watch_config_file, SettingsLocation};
use task::{TaskContext, TaskVariables, VariableName};
use text::BufferId;
use util::ResultExt;

use crate::{
    buffer_store::BufferStore, worktree_store::WorktreeStore, BasicContextProvider, Inventory,
    ProjectEnvironment,
};

#[allow(clippy::large_enum_variant)] // platform-dependent warning
pub enum TaskStore {
    Functional(StoreState),
    Noop,
}

pub struct StoreState {
    mode: StoreMode,
    task_inventory: Model<Inventory>,
    buffer_store: WeakModel<BufferStore>,
    worktree_store: Model<WorktreeStore>,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
    _global_task_config_watcher: Task<()>,
}

enum StoreMode {
    Local {
        downstream_client: Option<(AnyProtoClient, u64)>,
        environment: Model<ProjectEnvironment>,
    },
    Remote {
        upstream_client: AnyProtoClient,
        project_id: u64,
    },
}

impl EventEmitter<crate::Event> for TaskStore {}

impl TaskStore {
    pub fn init(client: Option<&AnyProtoClient>) {
        if let Some(client) = client {
            client.add_model_request_handler(Self::handle_task_context_for_location);
        }
    }

    async fn handle_task_context_for_location(
        store: Model<Self>,
        envelope: TypedEnvelope<proto::TaskContextForLocation>,
        mut cx: AsyncAppContext,
    ) -> anyhow::Result<proto::TaskContext> {
        let location = envelope
            .payload
            .location
            .context("no location given for task context handling")?;
        let (buffer_store, is_remote) = store.update(&mut cx, |store, _| {
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

                for range in location
                    .buffer
                    .read(cx)
                    .snapshot()
                    .runnable_ranges(location.range.clone())
                {
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
        buffer_store: WeakModel<BufferStore>,
        worktree_store: Model<WorktreeStore>,
        toolchain_store: Arc<dyn LanguageToolchainStore>,
        environment: Model<ProjectEnvironment>,
        cx: &mut ModelContext<'_, Self>,
    ) -> Self {
        Self::Functional(StoreState {
            mode: StoreMode::Local {
                downstream_client: None,
                environment,
            },
            task_inventory: Inventory::new(cx),
            buffer_store,
            toolchain_store,
            worktree_store,
            _global_task_config_watcher: Self::subscribe_to_global_task_file_changes(fs, cx),
        })
    }

    pub fn remote(
        fs: Arc<dyn Fs>,
        buffer_store: WeakModel<BufferStore>,
        worktree_store: Model<WorktreeStore>,
        toolchain_store: Arc<dyn LanguageToolchainStore>,
        upstream_client: AnyProtoClient,
        project_id: u64,
        cx: &mut ModelContext<'_, Self>,
    ) -> Self {
        Self::Functional(StoreState {
            mode: StoreMode::Remote {
                upstream_client,
                project_id,
            },
            task_inventory: Inventory::new(cx),
            buffer_store,
            toolchain_store,
            worktree_store,
            _global_task_config_watcher: Self::subscribe_to_global_task_file_changes(fs, cx),
        })
    }

    pub fn task_context_for_location(
        &self,
        captured_variables: TaskVariables,
        location: Location,
        cx: &mut AppContext,
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

    pub fn task_inventory(&self) -> Option<&Model<Inventory>> {
        match self {
            TaskStore::Functional(state) => Some(&state.task_inventory),
            TaskStore::Noop => None,
        }
    }

    pub fn shared(
        &mut self,
        remote_id: u64,
        new_downstream_client: AnyProtoClient,
        _cx: &mut AppContext,
    ) {
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

    pub fn unshared(&mut self, _: &mut ModelContext<Self>) {
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
        location: Option<SettingsLocation<'_>>,
        raw_tasks_json: Option<&str>,
        cx: &mut ModelContext<'_, Self>,
    ) -> anyhow::Result<()> {
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

    fn subscribe_to_global_task_file_changes(
        fs: Arc<dyn Fs>,
        cx: &mut ModelContext<'_, Self>,
    ) -> Task<()> {
        let mut user_tasks_file_rx =
            watch_config_file(&cx.background_executor(), fs, paths::tasks_file().clone());
        let user_tasks_content = cx.background_executor().block(user_tasks_file_rx.next());
        cx.spawn(move |task_store, mut cx| async move {
            if let Some(user_tasks_content) = user_tasks_content {
                let Ok(_) = task_store.update(&mut cx, |task_store, cx| {
                    task_store
                        .update_user_tasks(None, Some(&user_tasks_content), cx)
                        .log_err();
                }) else {
                    return;
                };
            }
            while let Some(user_tasks_content) = user_tasks_file_rx.next().await {
                let Ok(()) = task_store.update(&mut cx, |task_store, cx| {
                    let result = task_store.update_user_tasks(None, Some(&user_tasks_content), cx);
                    if let Err(err) = &result {
                        log::error!("Failed to load user tasks: {err}");
                        cx.emit(crate::Event::Toast {
                            notification_id: "load-user-tasks".into(),
                            message: format!("Invalid global tasks file\n{err}"),
                        });
                    }
                    cx.refresh();
                }) else {
                    break; // App dropped
                };
            }
        })
    }
}

fn local_task_context_for_location(
    worktree_store: Model<WorktreeStore>,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
    environment: Model<ProjectEnvironment>,
    captured_variables: TaskVariables,
    location: Location,
    cx: &AppContext,
) -> Task<Option<TaskContext>> {
    let worktree_id = location.buffer.read(cx).file().map(|f| f.worktree_id(cx));
    let worktree_abs_path = worktree_id
        .and_then(|worktree_id| worktree_store.read(cx).worktree_for_id(worktree_id, cx))
        .and_then(|worktree| worktree.read(cx).root_dir());

    cx.spawn(|mut cx| async move {
        let worktree_abs_path = worktree_abs_path.clone();
        let project_env = environment
            .update(&mut cx, |environment, cx| {
                environment.get_environment(worktree_id, worktree_abs_path.clone(), cx)
            })
            .ok()?
            .await;

        let mut task_variables = cx
            .update(|cx| {
                combine_task_variables(
                    captured_variables,
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
    worktree_store: Model<WorktreeStore>,
    captured_variables: TaskVariables,
    location: Location,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
    cx: &mut AppContext,
) -> Task<Option<TaskContext>> {
    cx.spawn(|cx| async move {
        // We need to gather a client context, as the headless one may lack certain information (e.g. tree-sitter parsing is disabled there, so symbols are not available).
        let mut remote_context = cx
            .update(|cx| {
                BasicContextProvider::new(worktree_store).build_context(
                    &TaskVariables::default(),
                    &location,
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

fn combine_task_variables(
    mut captured_variables: TaskVariables,
    location: Location,
    project_env: Option<HashMap<String, String>>,
    baseline: BasicContextProvider,
    toolchain_store: Arc<dyn LanguageToolchainStore>,
    cx: &mut AppContext,
) -> Task<anyhow::Result<TaskVariables>> {
    let language_context_provider = location
        .buffer
        .read(cx)
        .language()
        .and_then(|language| language.context_provider());
    cx.spawn(move |cx| async move {
        let baseline = cx
            .update(|cx| {
                baseline.build_context(
                    &captured_variables,
                    &location,
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
                    provider.build_context(
                        &captured_variables,
                        &location,
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
