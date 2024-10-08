use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;
use collections::HashMap;
use futures::{channel::mpsc, StreamExt as _};
use gpui::{AppContext, AsyncAppContext, Model, ModelContext, Task, WeakModel};
use language::{
    proto::{deserialize_anchor, serialize_anchor},
    ContextProvider as _, Location,
};
use rpc::{
    proto::{self, SSH_PROJECT_ID},
    AnyProtoClient, TypedEnvelope,
};
use settings::{watch_config_file, WorktreeId};
use task::{TaskContext, TaskVariables, VariableName};
use text::BufferId;
use util::ResultExt;

use crate::{
    buffer_store::BufferStore, worktree_store::WorktreeStore, BasicContextProvider, Inventory,
    ProjectEnvironment,
};

///
pub enum TaskStore {
    Functional(StoreState),
    Noop,
}

pub struct StoreState {
    mode: StoreMode,
    task_inventory: Model<Inventory>,
    buffer_store: WeakModel<BufferStore>,
    worktree_store: Model<WorktreeStore>,
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

/// A set of task templates, applicable in the current project.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct RawTaskTemplates<'a> {
    pub global: &'a [serde_json::Value],
    pub worktree: Vec<(&'a Arc<Path>, &'a serde_json::Value)>,
}

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
        buffer_store: WeakModel<BufferStore>,
        worktree_store: Model<WorktreeStore>,
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
            worktree_store,
        })
    }

    pub fn remote(
        buffer_store: WeakModel<BufferStore>,
        worktree_store: Model<WorktreeStore>,
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
            worktree_store,
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
                    environment.clone(),
                    captured_variables,
                    location,
                    cx,
                ),
                StoreMode::Remote {
                    upstream_client, ..
                } => remote_task_context_for_location(
                    upstream_client,
                    state.worktree_store.clone(),
                    captured_variables,
                    location,
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

    pub(super) fn update_task_templates<'a>(
        &'a self,
        worktree: Option<WorktreeId>,
        templates: RawTaskTemplates<'a>,
        cx: &mut AppContext,
    ) {
        let task_inventory = match self {
            TaskStore::Functional(state) => &state.task_inventory,
            TaskStore::Noop => return,
        };

        // task_inventory.update(cx, |inventory, cx| {
        //     let mut bad_templates = 0;
        //     match worktree {
        //         Some(worktree) => {
        //             if templates.worktree.is_empty() {
        //                 self.worktree_tasks.remove(&worktree);
        //             } else {
        //                 *self.worktree_tasks.entry(worktree).or_default() = templates
        //                     .worktree
        //                     .into_iter()
        //                     .filter_map(|(directory_path, raw_template)| {
        //                         match serde_json::from_value::<TaskTemplate>(raw_template.clone())
        //                             .log_err()
        //                         {
        //                             Some(template) => Some((Arc::clone(directory_path), template)),
        //                             None => {
        //                                 bad_templates += 1;
        //                                 None
        //                             }
        //                         }
        //                     })
        //                     .collect();
        //             }
        //         }
        //         None => {
        //             self.global_tasks = templates
        //                 .global
        //                 .into_iter()
        //                 .filter_map(|raw_template| {
        //                     match serde_json::from_value::<TaskTemplate>(raw_template.clone())
        //                         .log_err()
        //                     {
        //                         Some(template) => Some(template),
        //                         None => {
        //                             bad_templates += 1;
        //                             None
        //                         }
        //                     }
        //                 })
        //                 .collect();
        //         }
        //     }
        // });
    }

    pub(super) fn update_user_tasks(
        &self,
        content: &str,
        cx: &mut ModelContext<'_, TaskStore>,
    ) -> anyhow::Result<()> {
        todo!("TODO kb")
    }
}

fn subscribe_to_global_task_file_changes(fs: Arc<dyn fs::Fs>, cx: &mut AppContext) {
    let user_tasks_file_rx =
        watch_config_file(&cx.background_executor(), fs, paths::tasks_file().clone());

    handle_tasks_file_changes(user_tasks_file_rx, cx, handle_tasks_file_changed);
    todo!("TODO kb")
}

pub fn handle_tasks_file_changes(
    mut user_tasks_file_rx: mpsc::UnboundedReceiver<String>,
    cx: &mut AppContext,
    tasks_changed: impl Fn(Option<anyhow::Error>, &mut AppContext) + 'static,
) -> Task<()> {
    let user_tasks_content = cx
        .background_executor()
        .block(user_tasks_file_rx.next())
        .unwrap();
    // SettingsStore::update_global(cx, |store, cx| {
    //     store.set_user_tasks(&user_tasks_content, cx).log_err();
    // });
    // cx.spawn(move |cx| async move {
    //     while let Some(user_tasks_content) = user_tasks_file_rx.next().await {
    //         let result = cx.update_global(|store: &mut SettingsStore, cx| {
    //             let result = store.set_user_tasks(&user_tasks_content, cx);
    //             if let Err(err) = &result {
    //                 log::error!("Failed to load user tasks: {err}");
    //             }
    //             tasks_changed(result.err(), cx);
    //             cx.refresh();
    //         });
    //         if result.is_err() {
    //             break; // App dropped
    //         }
    //     }
    // })
    todo!("TODO kb")
}

fn handle_tasks_file_changed(error: Option<anyhow::Error>, cx: &mut AppContext) {
    struct TasksParseErrorNotification;
    // let id = NotificationId::unique::<TasksParseErrorNotification>();

    // TODO kb show proper error pop-ups here and for local worktree files
    // for workspace in workspace::local_workspace_windows(cx) {
    //     workspace
    //         .update(cx, |workspace, cx| match error.as_ref() {
    //             Some(error) => {
    //                 workspace.show_notification(id.clone(), cx, |cx| {
    //                     cx.new_view(|_| {
    //                         simple_message_notification::MessageNotification::new(format!(
    //                             "Invalid user tasks file\n{error}"
    //                         ))
    //                         .with_click_message("Open tasks file")
    //                         .on_click(|cx| {
    //                             cx.dispatch_action(zed::OpenTasks.boxed_clone());
    //                             cx.emit(DismissEvent);
    //                         })
    //                     })
    //                 });
    //             }
    //             None => workspace.dismiss_notification(&id, cx),
    //         })
    //         .log_err();
    // }
}

fn local_task_context_for_location(
    worktree_store: Model<WorktreeStore>,
    environment: Model<ProjectEnvironment>,
    captured_variables: TaskVariables,
    location: Location,
    cx: &AppContext,
) -> Task<Option<TaskContext>> {
    let worktree_id = location.buffer.read(cx).file().map(|f| f.worktree_id(cx));
    let worktree_abs_path = worktree_id
        .and_then(|worktree_id| worktree_store.read(cx).worktree_for_id(worktree_id, cx))
        .map(|worktree| worktree.read(cx).abs_path());

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
    worktree_store: Model<WorktreeStore>,
    captured_variables: TaskVariables,
    location: Location,
    cx: &mut AppContext,
) -> Task<Option<TaskContext>> {
    // We need to gather a client context, as the headless one may lack certain information (e.g. tree-sitter parsing is disabled there, so symbols are not available).
    let mut remote_context = BasicContextProvider::new(worktree_store)
        .build_context(&TaskVariables::default(), &location, None, cx)
        .log_err()
        .unwrap_or_default();
    remote_context.extend(captured_variables);

    let context_task = upstream_client.request(proto::TaskContextForLocation {
        project_id: SSH_PROJECT_ID,
        location: Some(proto::Location {
            buffer_id: location.buffer.read(cx).remote_id().into(),
            start: Some(serialize_anchor(&location.range.start)),
            end: Some(serialize_anchor(&location.range.end)),
        }),
        task_variables: remote_context
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
    });
    cx.spawn(|_| async move {
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
    project_env: Option<&HashMap<String, String>>,
    baseline: BasicContextProvider,
    cx: &mut AppContext,
) -> anyhow::Result<TaskVariables> {
    let language_context_provider = location
        .buffer
        .read(cx)
        .language()
        .and_then(|language| language.context_provider());
    let baseline = baseline
        .build_context(&captured_variables, &location, project_env, cx)
        .context("building basic default context")?;
    captured_variables.extend(baseline);
    if let Some(provider) = language_context_provider {
        captured_variables.extend(
            provider
                .build_context(&captured_variables, &location, project_env, cx)
                .context("building provider context")?,
        );
    }
    Ok(captured_variables)
}
