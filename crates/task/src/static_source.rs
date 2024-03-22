//! A source of tasks, based on a static configuration, deserialized from the tasks config file, and related infrastructure for tracking changes to the file.

use std::{borrow::Cow, path::Path, sync::Arc};

use collections::HashMap;
use futures::StreamExt;
use gpui::{AppContext, Context, Model, ModelContext, Subscription};
use schemars::{gen::SchemaSettings, JsonSchema};
use serde::{Deserialize, Serialize};
use util::ResultExt;

use crate::{SpawnInTerminal, Task, TaskContext, TaskId, TaskSource};
use futures::channel::mpsc::UnboundedReceiver;

/// A single config file entry with the deserialized task definition.
#[derive(Clone, Debug, PartialEq)]
struct StaticTask {
    id: TaskId,
    definition: Definition,
}

impl StaticTask {
    fn new(definition: Definition, (id_base, index_in_file): (&str, usize)) -> Arc<Self> {
        Arc::new(Self {
            id: TaskId(format!(
                "static_{id_base}_{index_in_file}_{}",
                definition.label
            )),
            definition,
        })
    }
}

/// TODO: doc
pub fn tasks_for(tasks: TaskDefinitions, id_base: &str) -> Vec<Arc<dyn Task>> {
    tasks
        .0
        .into_iter()
        .enumerate()
        .map(|(index, task)| StaticTask::new(task, (id_base, index)) as Arc<_>)
        .collect()
}

impl Task for StaticTask {
    fn exec(&self, cx: TaskContext) -> Option<SpawnInTerminal> {
        let TaskContext {
            cwd,
            task_variables,
        } = cx;
        let cwd = self
            .definition
            .cwd
            .clone()
            .and_then(|path| {
                subst::substitute(&path, &task_variables.0)
                    .map(Into::into)
                    .ok()
            })
            .or(cwd);
        let mut definition_env = self.definition.env.clone();
        definition_env.extend(task_variables.0);
        Some(SpawnInTerminal {
            id: self.id.clone(),
            cwd,
            use_new_terminal: self.definition.use_new_terminal,
            allow_concurrent_runs: self.definition.allow_concurrent_runs,
            label: self.definition.label.clone(),
            command: self.definition.command.clone(),
            args: self.definition.args.clone(),
            reveal: self.definition.reveal,
            env: definition_env,
        })
    }

    fn name(&self) -> &str {
        &self.definition.label
    }

    fn id(&self) -> &TaskId {
        &self.id
    }

    fn cwd(&self) -> Option<&str> {
        self.definition.cwd.as_deref()
    }
}

/// The source of tasks defined in a tasks config file.
pub struct StaticSource {
    tasks: Vec<Arc<StaticTask>>,
    _definitions: Model<TrackedFile<TaskDefinitions>>,
    _subscription: Subscription,
}

/// Static task definition from the tasks config file.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Definition {
    /// Human readable name of the task to display in the UI.
    pub label: String,
    /// Executable command to spawn.
    pub command: String,
    /// Arguments to the command.
    #[serde(default)]
    pub args: Vec<String>,
    /// Env overrides for the command, will be appended to the terminal's environment from the settings.
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Current working directory to spawn the command into, defaults to current project root.
    #[serde(default)]
    pub cwd: Option<String>,
    /// Whether to use a new terminal tab or reuse the existing one to spawn the process.
    #[serde(default)]
    pub use_new_terminal: bool,
    /// Whether to allow multiple instances of the same task to be run, or rather wait for the existing ones to finish.
    #[serde(default)]
    pub allow_concurrent_runs: bool,
    /// What to do with the terminal pane and tab, after the command was started:
    /// * `always` — always show the terminal pane, add and focus the corresponding task's tab in it (default)
    /// * `never` — avoid changing current terminal pane focus, but still add/reuse the task's tab there
    #[serde(default)]
    pub reveal: RevealStrategy,
}

/// What to do with the terminal pane and tab, after the command was started.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RevealStrategy {
    /// Always show the terminal pane, add and focus the corresponding task's tab in it.
    #[default]
    Always,
    /// Do not change terminal pane focus, but still add/reuse the task's tab there.
    Never,
}

/// A group of Tasks defined in a JSON file.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TaskDefinitions(pub Vec<Definition>);

impl TaskDefinitions {
    /// Generates JSON schema of Tasks JSON definition format.
    pub fn generate_json_schema() -> serde_json_lenient::Value {
        let schema = SchemaSettings::draft07()
            .with(|settings| settings.option_add_null_type = false)
            .into_generator()
            .into_root_schema_for::<Self>();

        serde_json_lenient::to_value(schema).unwrap()
    }
}
/// A Wrapper around deserializable T that keeps track of its contents
/// via a provided channel. Once T value changes, the observers of [`TrackedFile`] are
/// notified.
pub struct TrackedFile<T> {
    parsed_contents: T,
}

impl<T: PartialEq + 'static> TrackedFile<T> {
    /// Initializes new [`TrackedFile`] with a type that's deserializable.
    pub fn new(mut tracker: UnboundedReceiver<String>, cx: &mut AppContext) -> Model<Self>
    where
        T: for<'a> Deserialize<'a> + Default,
    {
        cx.new_model(move |cx| {
            cx.spawn(|tracked_file, mut cx| async move {
                while let Some(new_contents) = tracker.next().await {
                    if !new_contents.trim().is_empty() {
                        // String -> T (ZedTaskFormat)
                        // String -> U (VsCodeFormat) -> Into::into T
                        let Some(new_contents) =
                            serde_json_lenient::from_str(&new_contents).log_err()
                        else {
                            continue;
                        };
                        tracked_file.update(&mut cx, |tracked_file: &mut TrackedFile<T>, cx| {
                            if tracked_file.parsed_contents != new_contents {
                                tracked_file.parsed_contents = new_contents;
                                cx.notify();
                            };
                        })?;
                    }
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
            Self {
                parsed_contents: Default::default(),
            }
        })
    }

    /// Initializes new [`TrackedFile`] with a type that's convertible from another deserializable type.
    pub fn new_convertible<U: for<'a> Deserialize<'a> + TryInto<T, Error = anyhow::Error>>(
        mut tracker: UnboundedReceiver<String>,
        cx: &mut AppContext,
    ) -> Model<Self>
    where
        T: Default,
    {
        cx.new_model(move |cx| {
            cx.spawn(|tracked_file, mut cx| async move {
                while let Some(new_contents) = tracker.next().await {
                    if !new_contents.trim().is_empty() {
                        let Some(new_contents) =
                            serde_json_lenient::from_str::<U>(&new_contents).log_err()
                        else {
                            continue;
                        };
                        let Some(new_contents) = new_contents.try_into().log_err() else {
                            continue;
                        };
                        tracked_file.update(&mut cx, |tracked_file: &mut TrackedFile<T>, cx| {
                            if tracked_file.parsed_contents != new_contents {
                                tracked_file.parsed_contents = new_contents;
                                cx.notify();
                            };
                        })?;
                    }
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
            Self {
                parsed_contents: Default::default(),
            }
        })
    }

    fn get(&self) -> &T {
        &self.parsed_contents
    }
}

impl StaticSource {
    /// Initializes the static source, reacting on tasks config changes.
    pub fn new(
        id_base: impl Into<Cow<'static, str>>,
        definitions: Model<TrackedFile<TaskDefinitions>>,
        cx: &mut AppContext,
    ) -> Model<Box<dyn TaskSource>> {
        cx.new_model(|cx| {
            let id_base = id_base.into();
            let _subscription = cx.observe(
                &definitions,
                move |source: &mut Box<(dyn TaskSource + 'static)>, new_definitions, cx| {
                    if let Some(static_source) = source.as_any().downcast_mut::<Self>() {
                        static_source.tasks = new_definitions
                            .read(cx)
                            .get()
                            .0
                            .clone()
                            .into_iter()
                            .enumerate()
                            .map(|(i, definition)| StaticTask::new(definition, (&id_base, i)))
                            .collect();
                        cx.notify();
                    }
                },
            );
            Box::new(Self {
                tasks: Vec::new(),
                _definitions: definitions,
                _subscription,
            })
        })
    }
}

impl TaskSource for StaticSource {
    fn tasks_for_path(
        &mut self,
        _: Option<&Path>,
        _: &mut ModelContext<Box<dyn TaskSource>>,
    ) -> Vec<Arc<dyn Task>> {
        self.tasks
            .iter()
            .map(|task| task.clone() as Arc<dyn Task>)
            .collect()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
