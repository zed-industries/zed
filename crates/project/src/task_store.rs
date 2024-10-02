use std::path::PathBuf;

use anyhow::Context as _;
use gpui::{AppContext, AsyncAppContext, Model, ModelContext, Task, WeakModel};
use language::{proto::serialize_anchor, Location};
use rpc::{
    proto::{self, SSH_PROJECT_ID},
    AnyProtoClient, TypedEnvelope,
};
use task::{TaskContext, TaskVariables, VariableName};
use util::{debug_panic, ResultExt};

use crate::{
    buffer_store::BufferStore, combine_task_variables, worktree_store::WorktreeStore,
    BasicContextProvider, Inventory, ProjectEnvironment,
};

use super::deserialize_location;

/// TODO kb docs
///
pub enum TaskStore {
    Local {
        task_inventory: Model<Inventory>,
        downstream_client: Option<(AnyProtoClient, u64)>,
        buffer_store: WeakModel<BufferStore>,
        worktree_store: Model<WorktreeStore>,
        environment: Model<ProjectEnvironment>,
    },
    Remote {
        task_inventory: Model<Inventory>,
        upstream_client: AnyProtoClient,
        project_id: u64,
        buffer_store: WeakModel<BufferStore>,
        worktree_store: Model<WorktreeStore>,
    },
    Empty,
}

impl TaskStore {
    pub async fn handle_task_context_for_location(
        store: Model<Self>,
        envelope: TypedEnvelope<proto::TaskContextForLocation>,
        mut cx: AsyncAppContext,
    ) -> anyhow::Result<proto::TaskContext> {
        let location = envelope
            .payload
            .location
            .context("no location given for task context handling")?;
        let buffer_store = store.update(&mut cx, |store, _| match store {
            TaskStore::Local { buffer_store, .. } => Ok(buffer_store.clone()),
            TaskStore::Remote { buffer_store, .. } => Ok(buffer_store.clone()),
            TaskStore::Empty => {
                anyhow::bail!("empty task store cannot handle task context requests")
            }
        })??;
        let buffer_store = buffer_store
            .upgrade()
            .context("no buffer store when handling task context request")?;
        let location = cx
            .update(|cx| deserialize_location(&buffer_store, location, cx))?
            .await?;
        let context_task = store.update(&mut cx, |store, cx| {
            // TODO kb why not send the original task variables from the client?
            let captured_variables = {
                let mut variables = TaskVariables::default();
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
        buffer_store: WeakModel<BufferStore>,
        worktree_store: Model<WorktreeStore>,
        environment: Model<ProjectEnvironment>,
        cx: &mut ModelContext<'_, Self>,
    ) -> Self {
        Self::Local {
            task_inventory: Inventory::new(cx),
            downstream_client: None,
            buffer_store,
            worktree_store,
            environment,
        }
    }

    pub fn remote(
        buffer_store: WeakModel<BufferStore>,
        worktree_store: Model<WorktreeStore>,
        upstream_client: AnyProtoClient,
        project_id: u64,
        cx: &mut ModelContext<'_, Self>,
    ) -> Self {
        Self::Remote {
            task_inventory: Inventory::new(cx),
            buffer_store,
            worktree_store,
            upstream_client,
            project_id,
        }
    }

    pub fn task_context_for_location(
        &self,
        captured_variables: TaskVariables,
        location: Location,
        cx: &mut ModelContext<TaskStore>,
    ) -> Task<Option<TaskContext>> {
        match self {
            TaskStore::Local {
                worktree_store,
                environment,
                ..
            } => local_task_context_for_location(
                worktree_store.clone(),
                environment.clone(),
                captured_variables,
                location,
                cx,
            ),
            TaskStore::Remote {
                upstream_client, ..
            } => remote_task_context_for_location(upstream_client, location, cx),
            TaskStore::Empty => Task::ready(None),
        }
    }

    pub fn task_inventory(&self) -> Option<&Model<Inventory>> {
        match self {
            TaskStore::Local { task_inventory, .. } => Some(task_inventory),
            TaskStore::Remote { task_inventory, .. } => Some(task_inventory),
            TaskStore::Empty => None,
        }
    }

    pub fn shared(
        &mut self,
        remote_id: u64,
        new_downstream_client: AnyProtoClient,
        _cx: &mut AppContext,
    ) {
        if let Self::Local {
            downstream_client, ..
        } = self
        {
            *downstream_client = Some((new_downstream_client, remote_id));
        } else {
            debug_panic!("called shared on a non-local task store");
        }
    }

    pub fn unshared(&mut self, _: &mut ModelContext<Self>) {
        if let Self::Local {
            downstream_client, ..
        } = self
        {
            *downstream_client = None;
        } else {
            debug_panic!("called unshared on a non-local task store");
        }
    }
}

fn local_task_context_for_location(
    worktree_store: Model<WorktreeStore>,
    environment: Model<ProjectEnvironment>,
    captured_variables: TaskVariables,
    location: Location,
    cx: &mut ModelContext<TaskStore>,
) -> Task<Option<TaskContext>> {
    let worktree_id = location.buffer.read(cx).file().map(|f| f.worktree_id(cx));
    let worktree_abs_path = worktree_id
        .and_then(|worktree_id| worktree_store.read(cx).worktree_for_id(worktree_id, cx))
        .map(|worktree| worktree.read(cx).abs_path());

    cx.spawn(|_, mut cx| async move {
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
                    project_env.as_ref(),
                    BasicContextProvider::new(worktree_store),
                    cx,
                )
                .log_err()
            })
            .ok()
            .flatten()?;
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
    upstream_client: &AnyProtoClient,
    location: Location,
    cx: &mut ModelContext<TaskStore>,
) -> Task<Option<TaskContext>> {
    let context_task = upstream_client.request(proto::TaskContextForLocation {
        project_id: SSH_PROJECT_ID,
        location: Some(proto::Location {
            buffer_id: location.buffer.read(cx).remote_id().into(),
            start: Some(serialize_anchor(&location.range.start)),
            end: Some(serialize_anchor(&location.range.end)),
        }),
    });
    cx.spawn(|_, _| async move {
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
