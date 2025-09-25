//! Project-wide storage of the tasks available, capable of updating itself from the sources set.

use std::{
    borrow::Cow,
    cmp::{self, Reverse},
    collections::hash_map,
    path::PathBuf,
    sync::Arc,
};

use anyhow::Result;
use collections::{HashMap, HashSet, VecDeque};
use dap::DapRegistry;
use gpui::{App, AppContext as _, Context, Entity, SharedString, Task, WeakEntity};
use itertools::Itertools;
use language::{
    Buffer, ContextLocation, ContextProvider, File, Language, LanguageToolchainStore, Location,
    language_settings::language_settings,
};
use lsp::{LanguageServerId, LanguageServerName};
use paths::{debug_task_file_name, task_file_name};
use settings::{InvalidSettingsError, parse_json_with_comments};
use task::{
    DebugScenario, ResolvedTask, TaskContext, TaskId, TaskTemplate, TaskTemplates, TaskVariables,
    VariableName,
};
use text::{BufferId, Point, ToPoint};
use util::{NumericPrefixWithSuffix, ResultExt as _, post_inc, rel_path::RelPath};
use worktree::WorktreeId;

use crate::{task_store::TaskSettingsLocation, worktree_store::WorktreeStore};

#[derive(Clone, Debug, Default)]
pub struct DebugScenarioContext {
    pub task_context: TaskContext,
    pub worktree_id: Option<WorktreeId>,
    pub active_buffer: Option<WeakEntity<Buffer>>,
}

/// Inventory tracks available tasks for a given project.
pub struct Inventory {
    last_scheduled_tasks: VecDeque<(TaskSourceKind, ResolvedTask)>,
    last_scheduled_scenarios: VecDeque<(DebugScenario, DebugScenarioContext)>,
    templates_from_settings: InventoryFor<TaskTemplate>,
    scenarios_from_settings: InventoryFor<DebugScenario>,
}

impl std::fmt::Debug for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inventory")
            .field("last_scheduled_tasks", &self.last_scheduled_tasks)
            .field("last_scheduled_scenarios", &self.last_scheduled_scenarios)
            .field("templates_from_settings", &self.templates_from_settings)
            .field("scenarios_from_settings", &self.scenarios_from_settings)
            .finish()
    }
}

// Helper trait for better error messages in [InventoryFor]
trait InventoryContents: Clone {
    const GLOBAL_SOURCE_FILE: &'static str;
    const LABEL: &'static str;
}

impl InventoryContents for TaskTemplate {
    const GLOBAL_SOURCE_FILE: &'static str = "tasks.json";
    const LABEL: &'static str = "tasks";
}

impl InventoryContents for DebugScenario {
    const GLOBAL_SOURCE_FILE: &'static str = "debug.json";

    const LABEL: &'static str = "debug scenarios";
}

#[derive(Debug)]
struct InventoryFor<T> {
    global: HashMap<PathBuf, Vec<T>>,
    worktree: HashMap<WorktreeId, HashMap<Arc<RelPath>, Vec<T>>>,
}

impl<T: InventoryContents> InventoryFor<T> {
    fn worktree_scenarios(
        &self,
        worktree: WorktreeId,
    ) -> impl '_ + Iterator<Item = (TaskSourceKind, T)> {
        self.worktree
            .get(&worktree)
            .into_iter()
            .flatten()
            .flat_map(|(directory, templates)| {
                templates.iter().map(move |template| (directory, template))
            })
            .map(move |(directory, template)| {
                (
                    TaskSourceKind::Worktree {
                        id: worktree,
                        directory_in_worktree: directory.clone(),
                        id_base: Cow::Owned(format!(
                            "local worktree {} from directory {directory:?}",
                            T::LABEL
                        )),
                    },
                    template.clone(),
                )
            })
    }

    fn global_scenarios(&self) -> impl '_ + Iterator<Item = (TaskSourceKind, T)> {
        self.global.iter().flat_map(|(file_path, templates)| {
            templates.iter().map(|template| {
                (
                    TaskSourceKind::AbsPath {
                        id_base: Cow::Owned(format!("global {}", T::GLOBAL_SOURCE_FILE)),
                        abs_path: file_path.clone(),
                    },
                    template.clone(),
                )
            })
        })
    }
}

impl<T> Default for InventoryFor<T> {
    fn default() -> Self {
        Self {
            global: HashMap::default(),
            worktree: HashMap::default(),
        }
    }
}

/// Kind of a source the tasks are fetched from, used to display more source information in the UI.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TaskSourceKind {
    /// bash-like commands spawned by users, not associated with any path
    UserInput,
    /// Tasks from the worktree's .zed/task.json
    Worktree {
        id: WorktreeId,
        directory_in_worktree: Arc<RelPath>,
        id_base: Cow<'static, str>,
    },
    /// ~/.config/zed/task.json - like global files with task definitions, applicable to any path
    AbsPath {
        id_base: Cow<'static, str>,
        abs_path: PathBuf,
    },
    /// Languages-specific tasks coming from extensions.
    Language { name: SharedString },
    /// Language-specific tasks coming from LSP servers.
    Lsp {
        language_name: SharedString,
        server: LanguageServerId,
    },
}

/// A collection of task contexts, derived from the current state of the workspace.
/// Only contains worktrees that are visible and with their root being a directory.
#[derive(Debug, Default)]
pub struct TaskContexts {
    /// A context, related to the currently opened item.
    /// Item can be opened from an invisible worktree, or any other, not necessarily active worktree.
    pub active_item_context: Option<(Option<WorktreeId>, Option<Location>, TaskContext)>,
    /// A worktree that corresponds to the active item, or the only worktree in the workspace.
    pub active_worktree_context: Option<(WorktreeId, TaskContext)>,
    /// If there are multiple worktrees in the workspace, all non-active ones are included here.
    pub other_worktree_contexts: Vec<(WorktreeId, TaskContext)>,
    pub lsp_task_sources: HashMap<LanguageServerName, Vec<BufferId>>,
    pub latest_selection: Option<text::Anchor>,
}

impl TaskContexts {
    pub fn active_context(&self) -> Option<&TaskContext> {
        self.active_item_context
            .as_ref()
            .map(|(_, _, context)| context)
            .or_else(|| {
                self.active_worktree_context
                    .as_ref()
                    .map(|(_, context)| context)
            })
    }

    pub fn location(&self) -> Option<&Location> {
        self.active_item_context
            .as_ref()
            .and_then(|(_, location, _)| location.as_ref())
    }

    pub fn file(&self, cx: &App) -> Option<Arc<dyn File>> {
        self.active_item_context
            .as_ref()
            .and_then(|(_, location, _)| location.as_ref())
            .and_then(|location| location.buffer.read(cx).file().cloned())
    }

    pub fn worktree(&self) -> Option<WorktreeId> {
        self.active_item_context
            .as_ref()
            .and_then(|(worktree_id, _, _)| worktree_id.as_ref())
            .or_else(|| {
                self.active_worktree_context
                    .as_ref()
                    .map(|(worktree_id, _)| worktree_id)
            })
            .copied()
    }

    pub fn task_context_for_worktree_id(&self, worktree_id: WorktreeId) -> Option<&TaskContext> {
        self.active_worktree_context
            .iter()
            .chain(self.other_worktree_contexts.iter())
            .find(|(id, _)| *id == worktree_id)
            .map(|(_, context)| context)
    }
}

impl TaskSourceKind {
    pub fn to_id_base(&self) -> String {
        match self {
            Self::UserInput => "oneshot".to_string(),
            Self::AbsPath { id_base, abs_path } => {
                format!("{id_base}_{}", abs_path.display())
            }
            Self::Worktree {
                id,
                id_base,
                directory_in_worktree,
            } => {
                format!("{id_base}_{id}_{}", directory_in_worktree.as_str())
            }
            Self::Language { name } => format!("language_{name}"),
            Self::Lsp {
                server,
                language_name,
            } => format!("lsp_{language_name}_{server}"),
        }
    }
}

impl Inventory {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self {
            last_scheduled_tasks: VecDeque::default(),
            last_scheduled_scenarios: VecDeque::default(),
            templates_from_settings: InventoryFor::default(),
            scenarios_from_settings: InventoryFor::default(),
        })
    }

    pub fn scenario_scheduled(
        &mut self,
        scenario: DebugScenario,
        task_context: TaskContext,
        worktree_id: Option<WorktreeId>,
        active_buffer: Option<WeakEntity<Buffer>>,
    ) {
        self.last_scheduled_scenarios
            .retain(|(s, _)| s.label != scenario.label);
        self.last_scheduled_scenarios.push_front((
            scenario,
            DebugScenarioContext {
                task_context,
                worktree_id,
                active_buffer,
            },
        ));
        if self.last_scheduled_scenarios.len() > 5_000 {
            self.last_scheduled_scenarios.pop_front();
        }
    }

    pub fn last_scheduled_scenario(&self) -> Option<&(DebugScenario, DebugScenarioContext)> {
        self.last_scheduled_scenarios.back()
    }

    pub fn list_debug_scenarios(
        &self,
        task_contexts: &TaskContexts,
        lsp_tasks: Vec<(TaskSourceKind, task::ResolvedTask)>,
        current_resolved_tasks: Vec<(TaskSourceKind, task::ResolvedTask)>,
        add_current_language_tasks: bool,
        cx: &mut App,
    ) -> Task<(
        Vec<(DebugScenario, DebugScenarioContext)>,
        Vec<(TaskSourceKind, DebugScenario)>,
    )> {
        let mut scenarios = Vec::new();

        if let Some(worktree_id) = task_contexts
            .active_worktree_context
            .iter()
            .chain(task_contexts.other_worktree_contexts.iter())
            .map(|context| context.0)
            .next()
        {
            scenarios.extend(self.worktree_scenarios_from_settings(worktree_id));
        }
        scenarios.extend(self.global_debug_scenarios_from_settings());

        let last_scheduled_scenarios = self.last_scheduled_scenarios.iter().cloned().collect();

        let adapter = task_contexts.location().and_then(|location| {
            let (file, language) = {
                let buffer = location.buffer.read(cx);
                (buffer.file(), buffer.language())
            };
            let language_name = language.as_ref().map(|l| l.name());
            let adapter = language_settings(language_name, file, cx)
                .debuggers
                .first()
                .map(SharedString::from)
                .or_else(|| {
                    language.and_then(|l| l.config().debuggers.first().map(SharedString::from))
                });
            adapter.map(|adapter| (adapter, DapRegistry::global(cx).locators()))
        });
        cx.background_spawn(async move {
            if let Some((adapter, locators)) = adapter {
                for (kind, task) in
                    lsp_tasks
                        .into_iter()
                        .chain(current_resolved_tasks.into_iter().filter(|(kind, _)| {
                            add_current_language_tasks
                                || !matches!(kind, TaskSourceKind::Language { .. })
                        }))
                {
                    let adapter = adapter.clone().into();

                    for locator in locators.values() {
                        if let Some(scenario) = locator
                            .create_scenario(task.original_task(), task.display_label(), &adapter)
                            .await
                        {
                            scenarios.push((kind, scenario));
                            break;
                        }
                    }
                }
            }
            (last_scheduled_scenarios, scenarios)
        })
    }

    pub fn task_template_by_label(
        &self,
        buffer: Option<Entity<Buffer>>,
        worktree_id: Option<WorktreeId>,
        label: &str,
        cx: &App,
    ) -> Task<Option<TaskTemplate>> {
        let (buffer_worktree_id, file, language) = buffer
            .map(|buffer| {
                let buffer = buffer.read(cx);
                let file = buffer.file().cloned();
                (
                    file.as_ref().map(|file| file.worktree_id(cx)),
                    file,
                    buffer.language().cloned(),
                )
            })
            .unwrap_or((None, None, None));

        let tasks = self.list_tasks(file, language, worktree_id.or(buffer_worktree_id), cx);
        let label = label.to_owned();
        cx.background_spawn(async move {
            tasks
                .await
                .into_iter()
                .find(|(_, template)| template.label == label)
                .map(|val| val.1)
        })
    }

    /// Pulls its task sources relevant to the worktree and the language given,
    /// returns all task templates with their source kinds, worktree tasks first, language tasks second
    /// and global tasks last. No specific order inside source kinds groups.
    pub fn list_tasks(
        &self,
        file: Option<Arc<dyn File>>,
        language: Option<Arc<Language>>,
        worktree: Option<WorktreeId>,
        cx: &App,
    ) -> Task<Vec<(TaskSourceKind, TaskTemplate)>> {
        let global_tasks = self.global_templates_from_settings().collect::<Vec<_>>();
        let mut worktree_tasks = worktree
            .into_iter()
            .flat_map(|worktree| self.worktree_templates_from_settings(worktree))
            .collect::<Vec<_>>();

        let task_source_kind = language.as_ref().map(|language| TaskSourceKind::Language {
            name: language.name().into(),
        });
        let language_tasks = language
            .filter(|language| {
                language_settings(Some(language.name()), file.as_ref(), cx)
                    .tasks
                    .enabled
            })
            .and_then(|language| {
                language
                    .context_provider()
                    .map(|provider| provider.associated_tasks(file, cx))
            });
        cx.background_spawn(async move {
            if let Some(t) = language_tasks {
                worktree_tasks.extend(t.await.into_iter().flat_map(|tasks| {
                    tasks
                        .0
                        .into_iter()
                        .filter_map(|task| Some((task_source_kind.clone()?, task)))
                }));
            }
            worktree_tasks.extend(global_tasks);
            worktree_tasks
        })
    }

    /// Pulls its task sources relevant to the worktree and the language given and resolves them with the [`TaskContexts`] given.
    /// Joins the new resolutions with the resolved tasks that were used (spawned) before,
    /// orders them so that the most recently used come first, all equally used ones are ordered so that the most specific tasks come first.
    /// Deduplicates the tasks by their labels and context and splits the ordered list into two: used tasks and the rest, newly resolved tasks.
    pub fn used_and_current_resolved_tasks(
        &self,
        task_contexts: Arc<TaskContexts>,
        cx: &mut Context<Self>,
    ) -> Task<(
        Vec<(TaskSourceKind, ResolvedTask)>,
        Vec<(TaskSourceKind, ResolvedTask)>,
    )> {
        let worktree = task_contexts.worktree();
        let location = task_contexts.location();
        let language = location.and_then(|location| location.buffer.read(cx).language());
        let task_source_kind = language.as_ref().map(|language| TaskSourceKind::Language {
            name: language.name().into(),
        });
        let file = location.and_then(|location| location.buffer.read(cx).file().cloned());

        let mut task_labels_to_ids = HashMap::<String, HashSet<TaskId>>::default();
        let mut lru_score = 0_u32;
        let previously_spawned_tasks = self
            .last_scheduled_tasks
            .iter()
            .rev()
            .filter(|(task_kind, _)| {
                if matches!(task_kind, TaskSourceKind::Language { .. }) {
                    Some(task_kind) == task_source_kind.as_ref()
                } else {
                    true
                }
            })
            .filter(|(_, resolved_task)| {
                match task_labels_to_ids.entry(resolved_task.resolved_label.clone()) {
                    hash_map::Entry::Occupied(mut o) => {
                        o.get_mut().insert(resolved_task.id.clone());
                        // Neber allow duplicate reused tasks with the same labels
                        false
                    }
                    hash_map::Entry::Vacant(v) => {
                        v.insert(HashSet::from_iter(Some(resolved_task.id.clone())));
                        true
                    }
                }
            })
            .map(|(task_source_kind, resolved_task)| {
                (
                    task_source_kind.clone(),
                    resolved_task.clone(),
                    post_inc(&mut lru_score),
                )
            })
            .sorted_unstable_by(task_lru_comparator)
            .map(|(kind, task, _)| (kind, task))
            .collect::<Vec<_>>();

        let not_used_score = post_inc(&mut lru_score);
        let global_tasks = self.global_templates_from_settings().collect::<Vec<_>>();
        let associated_tasks = language
            .filter(|language| {
                language_settings(Some(language.name()), file.as_ref(), cx)
                    .tasks
                    .enabled
            })
            .and_then(|language| {
                language
                    .context_provider()
                    .map(|provider| provider.associated_tasks(file, cx))
            });
        let worktree_tasks = worktree
            .into_iter()
            .flat_map(|worktree| self.worktree_templates_from_settings(worktree))
            .collect::<Vec<_>>();
        let task_contexts = task_contexts.clone();
        cx.background_spawn(async move {
            let language_tasks = if let Some(task) = associated_tasks {
                task.await.map(|templates| {
                    templates
                        .0
                        .into_iter()
                        .flat_map(|task| Some((task_source_kind.clone()?, task)))
                })
            } else {
                None
            };

            let worktree_tasks = worktree_tasks
                .into_iter()
                .chain(language_tasks.into_iter().flatten())
                .chain(global_tasks);

            let new_resolved_tasks = worktree_tasks
                .flat_map(|(kind, task)| {
                    let id_base = kind.to_id_base();
                    if let TaskSourceKind::Worktree { id, .. } = &kind {
                        None.or_else(|| {
                            let (_, _, item_context) =
                                task_contexts.active_item_context.as_ref().filter(
                                    |(worktree_id, _, _)| Some(id) == worktree_id.as_ref(),
                                )?;
                            task.resolve_task(&id_base, item_context)
                        })
                        .or_else(|| {
                            let (_, worktree_context) = task_contexts
                                .active_worktree_context
                                .as_ref()
                                .filter(|(worktree_id, _)| id == worktree_id)?;
                            task.resolve_task(&id_base, worktree_context)
                        })
                        .or_else(|| {
                            if let TaskSourceKind::Worktree { id, .. } = &kind {
                                let worktree_context = task_contexts
                                    .other_worktree_contexts
                                    .iter()
                                    .find(|(worktree_id, _)| worktree_id == id)
                                    .map(|(_, context)| context)?;
                                task.resolve_task(&id_base, worktree_context)
                            } else {
                                None
                            }
                        })
                    } else {
                        None.or_else(|| {
                            let (_, _, item_context) =
                                task_contexts.active_item_context.as_ref()?;
                            task.resolve_task(&id_base, item_context)
                        })
                        .or_else(|| {
                            let (_, worktree_context) =
                                task_contexts.active_worktree_context.as_ref()?;
                            task.resolve_task(&id_base, worktree_context)
                        })
                    }
                    .or_else(|| task.resolve_task(&id_base, &TaskContext::default()))
                    .map(move |resolved_task| (kind.clone(), resolved_task, not_used_score))
                })
                .filter(|(_, resolved_task, _)| {
                    match task_labels_to_ids.entry(resolved_task.resolved_label.clone()) {
                        hash_map::Entry::Occupied(mut o) => {
                            // Allow new tasks with the same label, if their context is different
                            o.get_mut().insert(resolved_task.id.clone())
                        }
                        hash_map::Entry::Vacant(v) => {
                            v.insert(HashSet::from_iter(Some(resolved_task.id.clone())));
                            true
                        }
                    }
                })
                .sorted_unstable_by(task_lru_comparator)
                .map(|(kind, task, _)| (kind, task))
                .collect::<Vec<_>>();

            (previously_spawned_tasks, new_resolved_tasks)
        })
    }

    /// Returns the last scheduled task by task_id if provided.
    /// Otherwise, returns the last scheduled task.
    pub fn last_scheduled_task(
        &self,
        task_id: Option<&TaskId>,
    ) -> Option<(TaskSourceKind, ResolvedTask)> {
        if let Some(task_id) = task_id {
            self.last_scheduled_tasks
                .iter()
                .find(|(_, task)| &task.id == task_id)
                .cloned()
        } else {
            self.last_scheduled_tasks.back().cloned()
        }
    }

    /// Registers task "usage" as being scheduled – to be used for LRU sorting when listing all tasks.
    pub fn task_scheduled(
        &mut self,
        task_source_kind: TaskSourceKind,
        resolved_task: ResolvedTask,
    ) {
        self.last_scheduled_tasks
            .push_back((task_source_kind, resolved_task));
        if self.last_scheduled_tasks.len() > 5_000 {
            self.last_scheduled_tasks.pop_front();
        }
    }

    /// Deletes a resolved task from history, using its id.
    /// A similar may still resurface in `used_and_current_resolved_tasks` when its [`TaskTemplate`] is resolved again.
    pub fn delete_previously_used(&mut self, id: &TaskId) {
        self.last_scheduled_tasks.retain(|(_, task)| &task.id != id);
    }

    fn global_templates_from_settings(
        &self,
    ) -> impl '_ + Iterator<Item = (TaskSourceKind, TaskTemplate)> {
        self.templates_from_settings.global_scenarios()
    }

    fn global_debug_scenarios_from_settings(
        &self,
    ) -> impl '_ + Iterator<Item = (TaskSourceKind, DebugScenario)> {
        self.scenarios_from_settings.global_scenarios()
    }

    fn worktree_scenarios_from_settings(
        &self,
        worktree: WorktreeId,
    ) -> impl '_ + Iterator<Item = (TaskSourceKind, DebugScenario)> {
        self.scenarios_from_settings.worktree_scenarios(worktree)
    }

    fn worktree_templates_from_settings(
        &self,
        worktree: WorktreeId,
    ) -> impl '_ + Iterator<Item = (TaskSourceKind, TaskTemplate)> {
        self.templates_from_settings.worktree_scenarios(worktree)
    }

    /// Updates in-memory task metadata from the JSON string given.
    /// Will fail if the JSON is not a valid array of objects, but will continue if any object will not parse into a [`TaskTemplate`].
    ///
    /// Global tasks are updated for no worktree provided, otherwise the worktree metadata for a given path will be updated.
    pub(crate) fn update_file_based_tasks(
        &mut self,
        location: TaskSettingsLocation<'_>,
        raw_tasks_json: Option<&str>,
    ) -> Result<(), InvalidSettingsError> {
        let raw_tasks = match parse_json_with_comments::<Vec<serde_json::Value>>(
            raw_tasks_json.unwrap_or("[]"),
        ) {
            Ok(tasks) => tasks,
            Err(e) => {
                return Err(InvalidSettingsError::Tasks {
                    path: match location {
                        TaskSettingsLocation::Global(path) => path.to_owned(),
                        TaskSettingsLocation::Worktree(settings_location) => {
                            settings_location.path.as_std_path().join(task_file_name())
                        }
                    },
                    message: format!("Failed to parse tasks file content as a JSON array: {e}"),
                });
            }
        };
        let new_templates = raw_tasks.into_iter().filter_map(|raw_template| {
            serde_json::from_value::<TaskTemplate>(raw_template).log_err()
        });

        let parsed_templates = &mut self.templates_from_settings;
        match location {
            TaskSettingsLocation::Global(path) => {
                parsed_templates
                    .global
                    .entry(path.to_owned())
                    .insert_entry(new_templates.collect());
                self.last_scheduled_tasks.retain(|(kind, _)| {
                    if let TaskSourceKind::AbsPath { abs_path, .. } = kind {
                        abs_path != path
                    } else {
                        true
                    }
                });
            }
            TaskSettingsLocation::Worktree(location) => {
                let new_templates = new_templates.collect::<Vec<_>>();
                if new_templates.is_empty() {
                    if let Some(worktree_tasks) =
                        parsed_templates.worktree.get_mut(&location.worktree_id)
                    {
                        worktree_tasks.remove(location.path);
                    }
                } else {
                    parsed_templates
                        .worktree
                        .entry(location.worktree_id)
                        .or_default()
                        .insert(Arc::from(location.path), new_templates);
                }
                self.last_scheduled_tasks.retain(|(kind, _)| {
                    if let TaskSourceKind::Worktree {
                        directory_in_worktree,
                        id,
                        ..
                    } = kind
                    {
                        *id != location.worktree_id
                            || directory_in_worktree.as_ref() != location.path
                    } else {
                        true
                    }
                });
            }
        }

        Ok(())
    }

    /// Updates in-memory task metadata from the JSON string given.
    /// Will fail if the JSON is not a valid array of objects, but will continue if any object will not parse into a [`TaskTemplate`].
    ///
    /// Global tasks are updated for no worktree provided, otherwise the worktree metadata for a given path will be updated.
    pub(crate) fn update_file_based_scenarios(
        &mut self,
        location: TaskSettingsLocation<'_>,
        raw_tasks_json: Option<&str>,
    ) -> Result<(), InvalidSettingsError> {
        let raw_tasks = match parse_json_with_comments::<Vec<serde_json::Value>>(
            raw_tasks_json.unwrap_or("[]"),
        ) {
            Ok(tasks) => tasks,
            Err(e) => {
                return Err(InvalidSettingsError::Debug {
                    path: match location {
                        TaskSettingsLocation::Global(path) => path.to_owned(),
                        TaskSettingsLocation::Worktree(settings_location) => settings_location
                            .path
                            .as_std_path()
                            .join(debug_task_file_name()),
                    },
                    message: format!("Failed to parse tasks file content as a JSON array: {e}"),
                });
            }
        };

        let new_templates = raw_tasks
            .into_iter()
            .filter_map(|raw_template| {
                serde_json::from_value::<DebugScenario>(raw_template).log_err()
            })
            .collect::<Vec<_>>();

        let parsed_scenarios = &mut self.scenarios_from_settings;
        let mut new_definitions: HashMap<_, _> = new_templates
            .iter()
            .map(|template| (template.label.clone(), template.clone()))
            .collect();
        let previously_existing_scenarios;

        match location {
            TaskSettingsLocation::Global(path) => {
                previously_existing_scenarios = parsed_scenarios
                    .global_scenarios()
                    .map(|(_, scenario)| scenario.label)
                    .collect::<HashSet<_>>();
                parsed_scenarios
                    .global
                    .entry(path.to_owned())
                    .insert_entry(new_templates);
            }
            TaskSettingsLocation::Worktree(location) => {
                previously_existing_scenarios = parsed_scenarios
                    .worktree_scenarios(location.worktree_id)
                    .map(|(_, scenario)| scenario.label)
                    .collect::<HashSet<_>>();

                if new_templates.is_empty() {
                    if let Some(worktree_tasks) =
                        parsed_scenarios.worktree.get_mut(&location.worktree_id)
                    {
                        worktree_tasks.remove(location.path);
                    }
                } else {
                    parsed_scenarios
                        .worktree
                        .entry(location.worktree_id)
                        .or_default()
                        .insert(Arc::from(location.path), new_templates);
                }
            }
        }
        self.last_scheduled_scenarios.retain_mut(|(scenario, _)| {
            if !previously_existing_scenarios.contains(&scenario.label) {
                return true;
            }
            if let Some(new_definition) = new_definitions.remove(&scenario.label) {
                *scenario = new_definition;
                true
            } else {
                false
            }
        });

        Ok(())
    }
}

fn task_lru_comparator(
    (kind_a, task_a, lru_score_a): &(TaskSourceKind, ResolvedTask, u32),
    (kind_b, task_b, lru_score_b): &(TaskSourceKind, ResolvedTask, u32),
) -> cmp::Ordering {
    lru_score_a
        // First, display recently used templates above all.
        .cmp(lru_score_b)
        // Then, ensure more specific sources are displayed first.
        .then(task_source_kind_preference(kind_a).cmp(&task_source_kind_preference(kind_b)))
        // After that, display first more specific tasks, using more template variables.
        // Bonus points for tasks with symbol variables.
        .then(task_variables_preference(task_a).cmp(&task_variables_preference(task_b)))
        // Finally, sort by the resolved label, but a bit more specifically, to avoid mixing letters and digits.
        .then({
            NumericPrefixWithSuffix::from_numeric_prefixed_str(&task_a.resolved_label)
                .cmp(&NumericPrefixWithSuffix::from_numeric_prefixed_str(
                    &task_b.resolved_label,
                ))
                .then(task_a.resolved_label.cmp(&task_b.resolved_label))
                .then(kind_a.cmp(kind_b))
        })
}

fn task_source_kind_preference(kind: &TaskSourceKind) -> u32 {
    match kind {
        TaskSourceKind::Lsp { .. } => 0,
        TaskSourceKind::Language { .. } => 1,
        TaskSourceKind::UserInput => 2,
        TaskSourceKind::Worktree { .. } => 3,
        TaskSourceKind::AbsPath { .. } => 4,
    }
}

fn task_variables_preference(task: &ResolvedTask) -> Reverse<usize> {
    let task_variables = task.substituted_variables();
    Reverse(if task_variables.contains(&VariableName::Symbol) {
        task_variables.len() + 1
    } else {
        task_variables.len()
    })
}

#[cfg(test)]
mod test_inventory {
    use gpui::{AppContext as _, Entity, Task, TestAppContext};
    use itertools::Itertools;
    use task::TaskContext;
    use worktree::WorktreeId;

    use crate::Inventory;

    use super::TaskSourceKind;

    pub(super) fn task_template_names(
        inventory: &Entity<Inventory>,
        worktree: Option<WorktreeId>,
        cx: &mut TestAppContext,
    ) -> Task<Vec<String>> {
        let new_tasks = inventory.update(cx, |inventory, cx| {
            inventory.list_tasks(None, None, worktree, cx)
        });
        cx.background_spawn(async move {
            new_tasks
                .await
                .into_iter()
                .map(|(_, task)| task.label)
                .sorted()
                .collect()
        })
    }

    pub(super) fn register_task_used(
        inventory: &Entity<Inventory>,
        task_name: &str,
        cx: &mut TestAppContext,
    ) -> Task<()> {
        let tasks = inventory.update(cx, |inventory, cx| {
            inventory.list_tasks(None, None, None, cx)
        });

        let task_name = task_name.to_owned();
        let inventory = inventory.clone();
        cx.spawn(|mut cx| async move {
            let (task_source_kind, task) = tasks
                .await
                .into_iter()
                .find(|(_, task)| task.label == task_name)
                .unwrap_or_else(|| panic!("Failed to find task with name {task_name}"));

            let id_base = task_source_kind.to_id_base();
            inventory
                .update(&mut cx, |inventory, _| {
                    inventory.task_scheduled(
                        task_source_kind.clone(),
                        task.resolve_task(&id_base, &TaskContext::default())
                            .unwrap_or_else(|| {
                                panic!("Failed to resolve task with name {task_name}")
                            }),
                    )
                })
                .unwrap();
        })
    }

    pub(super) fn register_worktree_task_used(
        inventory: &Entity<Inventory>,
        worktree_id: WorktreeId,
        task_name: &str,
        cx: &mut TestAppContext,
    ) -> Task<()> {
        let tasks = inventory.update(cx, |inventory, cx| {
            inventory.list_tasks(None, None, Some(worktree_id), cx)
        });

        let inventory = inventory.clone();
        let task_name = task_name.to_owned();
        cx.spawn(|mut cx| async move {
            let (task_source_kind, task) = tasks
                .await
                .into_iter()
                .find(|(_, task)| task.label == task_name)
                .unwrap_or_else(|| panic!("Failed to find task with name {task_name}"));
            let id_base = task_source_kind.to_id_base();
            inventory
                .update(&mut cx, |inventory, _| {
                    inventory.task_scheduled(
                        task_source_kind.clone(),
                        task.resolve_task(&id_base, &TaskContext::default())
                            .unwrap_or_else(|| {
                                panic!("Failed to resolve task with name {task_name}")
                            }),
                    );
                })
                .unwrap();
        })
    }

    pub(super) async fn list_tasks(
        inventory: &Entity<Inventory>,
        worktree: Option<WorktreeId>,
        cx: &mut TestAppContext,
    ) -> Vec<(TaskSourceKind, String)> {
        let task_context = &TaskContext::default();
        inventory
            .update(cx, |inventory, cx| {
                inventory.list_tasks(None, None, worktree, cx)
            })
            .await
            .into_iter()
            .filter_map(|(source_kind, task)| {
                let id_base = source_kind.to_id_base();
                Some((source_kind, task.resolve_task(&id_base, task_context)?))
            })
            .map(|(source_kind, resolved_task)| (source_kind, resolved_task.resolved_label))
            .collect()
    }
}

/// A context provided that tries to provide values for all non-custom [`VariableName`] variants for a currently opened file.
/// Applied as a base for every custom [`ContextProvider`] unless explicitly oped out.
pub struct BasicContextProvider {
    worktree_store: Entity<WorktreeStore>,
}

impl BasicContextProvider {
    pub fn new(worktree_store: Entity<WorktreeStore>) -> Self {
        Self { worktree_store }
    }
}

impl ContextProvider for BasicContextProvider {
    fn build_context(
        &self,
        _: &TaskVariables,
        location: ContextLocation<'_>,
        _: Option<HashMap<String, String>>,
        _: Arc<dyn LanguageToolchainStore>,
        cx: &mut App,
    ) -> Task<Result<TaskVariables>> {
        let location = location.file_location;
        let buffer = location.buffer.read(cx);
        let buffer_snapshot = buffer.snapshot();
        let symbols = buffer_snapshot.symbols_containing(location.range.start, None);
        let symbol = symbols.last().map(|symbol| {
            let range = symbol
                .name_ranges
                .last()
                .cloned()
                .unwrap_or(0..symbol.text.len());
            symbol.text[range].to_string()
        });

        let current_file = buffer.file().and_then(|file| file.as_local());
        let Point { row, column } = location.range.start.to_point(&buffer_snapshot);
        let row = row + 1;
        let column = column + 1;
        let selected_text = buffer
            .chars_for_range(location.range.clone())
            .collect::<String>();

        let mut task_variables = TaskVariables::from_iter([
            (VariableName::Row, row.to_string()),
            (VariableName::Column, column.to_string()),
        ]);

        if let Some(symbol) = symbol {
            task_variables.insert(VariableName::Symbol, symbol);
        }
        if !selected_text.trim().is_empty() {
            task_variables.insert(VariableName::SelectedText, selected_text);
        }
        let worktree = buffer
            .file()
            .map(|file| file.worktree_id(cx))
            .and_then(|worktree_id| {
                self.worktree_store
                    .read(cx)
                    .worktree_for_id(worktree_id, cx)
            });

        if let Some(worktree) = worktree {
            let worktree = worktree.read(cx);
            let path_style = worktree.path_style();
            task_variables.insert(
                VariableName::WorktreeRoot,
                worktree.abs_path().to_string_lossy().to_string(),
            );
            if let Some(current_file) = current_file.as_ref() {
                let relative_path = current_file.path();
                task_variables.insert(
                    VariableName::RelativeFile,
                    relative_path.display(path_style).to_string(),
                );
                if let Some(relative_dir) = relative_path.parent() {
                    task_variables.insert(
                        VariableName::RelativeDir,
                        if relative_dir.is_empty() {
                            String::from(".")
                        } else {
                            relative_dir.display(path_style).to_string()
                        },
                    );
                }
            }
        }

        if let Some(current_file) = current_file {
            let path = current_file.abs_path(cx);
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                task_variables.insert(VariableName::Filename, String::from(filename));
            }

            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                task_variables.insert(VariableName::Stem, stem.into());
            }

            if let Some(dirname) = path.parent().and_then(|s| s.to_str()) {
                task_variables.insert(VariableName::Dirname, dirname.into());
            }

            task_variables.insert(VariableName::File, path.to_string_lossy().to_string());
        }

        Task::ready(Ok(task_variables))
    }
}

/// A ContextProvider that doesn't provide any task variables on it's own, though it has some associated tasks.
pub struct ContextProviderWithTasks {
    templates: TaskTemplates,
}

impl ContextProviderWithTasks {
    pub fn new(definitions: TaskTemplates) -> Self {
        Self {
            templates: definitions,
        }
    }
}

impl ContextProvider for ContextProviderWithTasks {
    fn associated_tasks(&self, _: Option<Arc<dyn File>>, _: &App) -> Task<Option<TaskTemplates>> {
        Task::ready(Some(self.templates.clone()))
    }
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use paths::tasks_file;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use settings::SettingsLocation;
    use std::path::Path;
    use util::rel_path::rel_path;

    use crate::task_store::TaskStore;

    use super::test_inventory::*;
    use super::*;

    #[gpui::test]
    async fn test_task_list_sorting(cx: &mut TestAppContext) {
        init_test(cx);
        let inventory = cx.update(|cx| Inventory::new(cx));
        let initial_tasks = resolved_task_names(&inventory, None, cx).await;
        assert!(
            initial_tasks.is_empty(),
            "No tasks expected for empty inventory, but got {initial_tasks:?}"
        );
        let initial_tasks = task_template_names(&inventory, None, cx).await;
        assert!(
            initial_tasks.is_empty(),
            "No tasks expected for empty inventory, but got {initial_tasks:?}"
        );
        cx.run_until_parked();
        let expected_initial_state = [
            "1_a_task".to_string(),
            "1_task".to_string(),
            "2_task".to_string(),
            "3_task".to_string(),
        ];

        inventory.update(cx, |inventory, _| {
            inventory
                .update_file_based_tasks(
                    TaskSettingsLocation::Global(tasks_file()),
                    Some(&mock_tasks_from_names(
                        expected_initial_state.iter().map(|name| name.as_str()),
                    )),
                )
                .unwrap();
        });
        assert_eq!(
            task_template_names(&inventory, None, cx).await,
            &expected_initial_state,
        );
        assert_eq!(
            resolved_task_names(&inventory, None, cx).await,
            &expected_initial_state,
            "Tasks with equal amount of usages should be sorted alphanumerically"
        );

        register_task_used(&inventory, "2_task", cx).await;
        assert_eq!(
            task_template_names(&inventory, None, cx).await,
            &expected_initial_state,
        );
        assert_eq!(
            resolved_task_names(&inventory, None, cx).await,
            vec![
                "2_task".to_string(),
                "1_a_task".to_string(),
                "1_task".to_string(),
                "3_task".to_string()
            ],
        );

        register_task_used(&inventory, "1_task", cx).await;
        register_task_used(&inventory, "1_task", cx).await;
        register_task_used(&inventory, "1_task", cx).await;
        register_task_used(&inventory, "3_task", cx).await;
        assert_eq!(
            task_template_names(&inventory, None, cx).await,
            &expected_initial_state,
        );
        assert_eq!(
            resolved_task_names(&inventory, None, cx).await,
            vec![
                "3_task".to_string(),
                "1_task".to_string(),
                "2_task".to_string(),
                "1_a_task".to_string(),
            ],
            "Most recently used task should be at the top"
        );

        let worktree_id = WorktreeId::from_usize(0);
        let local_worktree_location = SettingsLocation {
            worktree_id,
            path: RelPath::new("foo").unwrap(),
        };
        inventory.update(cx, |inventory, _| {
            inventory
                .update_file_based_tasks(
                    TaskSettingsLocation::Worktree(local_worktree_location),
                    Some(&mock_tasks_from_names(["worktree_task_1"])),
                )
                .unwrap();
        });
        assert_eq!(
            resolved_task_names(&inventory, None, cx).await,
            vec![
                "3_task".to_string(),
                "1_task".to_string(),
                "2_task".to_string(),
                "1_a_task".to_string(),
            ],
            "Most recently used task should be at the top"
        );
        assert_eq!(
            resolved_task_names(&inventory, Some(worktree_id), cx).await,
            vec![
                "3_task".to_string(),
                "1_task".to_string(),
                "2_task".to_string(),
                "worktree_task_1".to_string(),
                "1_a_task".to_string(),
            ],
        );
        register_worktree_task_used(&inventory, worktree_id, "worktree_task_1", cx).await;
        assert_eq!(
            resolved_task_names(&inventory, Some(worktree_id), cx).await,
            vec![
                "worktree_task_1".to_string(),
                "3_task".to_string(),
                "1_task".to_string(),
                "2_task".to_string(),
                "1_a_task".to_string(),
            ],
            "Most recently used worktree task should be at the top"
        );

        inventory.update(cx, |inventory, _| {
            inventory
                .update_file_based_tasks(
                    TaskSettingsLocation::Global(tasks_file()),
                    Some(&mock_tasks_from_names(
                        ["10_hello", "11_hello"]
                            .into_iter()
                            .chain(expected_initial_state.iter().map(|name| name.as_str())),
                    )),
                )
                .unwrap();
        });
        cx.run_until_parked();
        let expected_updated_state = [
            "10_hello".to_string(),
            "11_hello".to_string(),
            "1_a_task".to_string(),
            "1_task".to_string(),
            "2_task".to_string(),
            "3_task".to_string(),
        ];
        assert_eq!(
            task_template_names(&inventory, None, cx).await,
            &expected_updated_state,
        );
        assert_eq!(
            resolved_task_names(&inventory, None, cx).await,
            vec![
                "worktree_task_1".to_string(),
                "1_a_task".to_string(),
                "1_task".to_string(),
                "2_task".to_string(),
                "3_task".to_string(),
                "10_hello".to_string(),
                "11_hello".to_string(),
            ],
            "After global tasks update, worktree task usage is not erased and it's the first still; global task is back to regular order as its file was updated"
        );

        register_task_used(&inventory, "11_hello", cx).await;
        assert_eq!(
            task_template_names(&inventory, None, cx).await,
            &expected_updated_state,
        );
        assert_eq!(
            resolved_task_names(&inventory, None, cx).await,
            vec![
                "11_hello".to_string(),
                "worktree_task_1".to_string(),
                "1_a_task".to_string(),
                "1_task".to_string(),
                "2_task".to_string(),
                "3_task".to_string(),
                "10_hello".to_string(),
            ],
        );
    }

    #[gpui::test]
    async fn test_reloading_debug_scenarios(cx: &mut TestAppContext) {
        init_test(cx);
        let inventory = cx.update(|cx| Inventory::new(cx));
        inventory.update(cx, |inventory, _| {
            inventory
                .update_file_based_scenarios(
                    TaskSettingsLocation::Global(Path::new("")),
                    Some(
                        r#"
                        [{
                            "label": "test scenario",
                            "adapter": "CodeLLDB",
                            "request": "launch",
                            "program": "wowzer",
                        }]
                        "#,
                    ),
                )
                .unwrap();
        });

        let (_, scenario) = inventory
            .update(cx, |this, cx| {
                this.list_debug_scenarios(&TaskContexts::default(), vec![], vec![], false, cx)
            })
            .await
            .1
            .first()
            .unwrap()
            .clone();

        inventory.update(cx, |this, _| {
            this.scenario_scheduled(scenario.clone(), TaskContext::default(), None, None);
        });

        assert_eq!(
            inventory
                .update(cx, |this, cx| {
                    this.list_debug_scenarios(&TaskContexts::default(), vec![], vec![], false, cx)
                })
                .await
                .0
                .first()
                .unwrap()
                .clone()
                .0,
            scenario
        );

        inventory.update(cx, |this, _| {
            this.update_file_based_scenarios(
                TaskSettingsLocation::Global(Path::new("")),
                Some(
                    r#"
                        [{
                            "label": "test scenario",
                            "adapter": "Delve",
                            "request": "launch",
                            "program": "wowzer",
                        }]
                        "#,
                ),
            )
            .unwrap();
        });

        assert_eq!(
            inventory
                .update(cx, |this, cx| {
                    this.list_debug_scenarios(&TaskContexts::default(), vec![], vec![], false, cx)
                })
                .await
                .0
                .first()
                .unwrap()
                .0
                .adapter,
            "Delve",
        );

        inventory.update(cx, |this, _| {
            this.update_file_based_scenarios(
                TaskSettingsLocation::Global(Path::new("")),
                Some(
                    r#"
                        [{
                            "label": "testing scenario",
                            "adapter": "Delve",
                            "request": "launch",
                            "program": "wowzer",
                        }]
                        "#,
                ),
            )
            .unwrap();
        });

        assert!(
            inventory
                .update(cx, |this, cx| {
                    this.list_debug_scenarios(&TaskContexts::default(), vec![], vec![], false, cx)
                })
                .await
                .0
                .is_empty(),
        );
    }

    #[gpui::test]
    async fn test_inventory_static_task_filters(cx: &mut TestAppContext) {
        init_test(cx);
        let inventory = cx.update(|cx| Inventory::new(cx));
        let common_name = "common_task_name";
        let worktree_1 = WorktreeId::from_usize(1);
        let worktree_2 = WorktreeId::from_usize(2);

        cx.run_until_parked();
        let worktree_independent_tasks = vec![
            (
                TaskSourceKind::AbsPath {
                    id_base: "global tasks.json".into(),
                    abs_path: paths::tasks_file().clone(),
                },
                common_name.to_string(),
            ),
            (
                TaskSourceKind::AbsPath {
                    id_base: "global tasks.json".into(),
                    abs_path: paths::tasks_file().clone(),
                },
                "static_source_1".to_string(),
            ),
            (
                TaskSourceKind::AbsPath {
                    id_base: "global tasks.json".into(),
                    abs_path: paths::tasks_file().clone(),
                },
                "static_source_2".to_string(),
            ),
        ];
        let worktree_1_tasks = [
            (
                TaskSourceKind::Worktree {
                    id: worktree_1,
                    directory_in_worktree: rel_path(".zed").into(),
                    id_base: "local worktree tasks from directory \".zed\"".into(),
                },
                common_name.to_string(),
            ),
            (
                TaskSourceKind::Worktree {
                    id: worktree_1,
                    directory_in_worktree: rel_path(".zed").into(),
                    id_base: "local worktree tasks from directory \".zed\"".into(),
                },
                "worktree_1".to_string(),
            ),
        ];
        let worktree_2_tasks = [
            (
                TaskSourceKind::Worktree {
                    id: worktree_2,
                    directory_in_worktree: rel_path(".zed").into(),
                    id_base: "local worktree tasks from directory \".zed\"".into(),
                },
                common_name.to_string(),
            ),
            (
                TaskSourceKind::Worktree {
                    id: worktree_2,
                    directory_in_worktree: rel_path(".zed").into(),
                    id_base: "local worktree tasks from directory \".zed\"".into(),
                },
                "worktree_2".to_string(),
            ),
        ];

        inventory.update(cx, |inventory, _| {
            inventory
                .update_file_based_tasks(
                    TaskSettingsLocation::Global(tasks_file()),
                    Some(&mock_tasks_from_names(
                        worktree_independent_tasks
                            .iter()
                            .map(|(_, name)| name.as_str()),
                    )),
                )
                .unwrap();
            inventory
                .update_file_based_tasks(
                    TaskSettingsLocation::Worktree(SettingsLocation {
                        worktree_id: worktree_1,
                        path: RelPath::new(".zed").unwrap(),
                    }),
                    Some(&mock_tasks_from_names(
                        worktree_1_tasks.iter().map(|(_, name)| name.as_str()),
                    )),
                )
                .unwrap();
            inventory
                .update_file_based_tasks(
                    TaskSettingsLocation::Worktree(SettingsLocation {
                        worktree_id: worktree_2,
                        path: RelPath::new(".zed").unwrap(),
                    }),
                    Some(&mock_tasks_from_names(
                        worktree_2_tasks.iter().map(|(_, name)| name.as_str()),
                    )),
                )
                .unwrap();
        });

        assert_eq!(
            list_tasks_sorted_by_last_used(&inventory, None, cx).await,
            worktree_independent_tasks,
            "Without a worktree, only worktree-independent tasks should be listed"
        );
        assert_eq!(
            list_tasks_sorted_by_last_used(&inventory, Some(worktree_1), cx).await,
            worktree_1_tasks
                .iter()
                .chain(worktree_independent_tasks.iter())
                .cloned()
                .sorted_by_key(|(kind, label)| (task_source_kind_preference(kind), label.clone()))
                .collect::<Vec<_>>(),
        );
        assert_eq!(
            list_tasks_sorted_by_last_used(&inventory, Some(worktree_2), cx).await,
            worktree_2_tasks
                .iter()
                .chain(worktree_independent_tasks.iter())
                .cloned()
                .sorted_by_key(|(kind, label)| (task_source_kind_preference(kind), label.clone()))
                .collect::<Vec<_>>(),
        );

        assert_eq!(
            list_tasks(&inventory, None, cx).await,
            worktree_independent_tasks,
            "Without a worktree, only worktree-independent tasks should be listed"
        );
        assert_eq!(
            list_tasks(&inventory, Some(worktree_1), cx).await,
            worktree_1_tasks
                .iter()
                .chain(worktree_independent_tasks.iter())
                .cloned()
                .collect::<Vec<_>>(),
        );
        assert_eq!(
            list_tasks(&inventory, Some(worktree_2), cx).await,
            worktree_2_tasks
                .iter()
                .chain(worktree_independent_tasks.iter())
                .cloned()
                .collect::<Vec<_>>(),
        );
    }

    fn init_test(_cx: &mut TestAppContext) {
        zlog::init_test();
        TaskStore::init(None);
    }

    fn resolved_task_names(
        inventory: &Entity<Inventory>,
        worktree: Option<WorktreeId>,
        cx: &mut TestAppContext,
    ) -> Task<Vec<String>> {
        let tasks = inventory.update(cx, |inventory, cx| {
            let mut task_contexts = TaskContexts::default();
            task_contexts.active_worktree_context =
                worktree.map(|worktree| (worktree, TaskContext::default()));

            inventory.used_and_current_resolved_tasks(Arc::new(task_contexts), cx)
        });

        cx.background_spawn(async move {
            let (used, current) = tasks.await;
            used.into_iter()
                .chain(current)
                .map(|(_, task)| task.original_task().label.clone())
                .collect()
        })
    }

    fn mock_tasks_from_names<'a>(task_names: impl IntoIterator<Item = &'a str> + 'a) -> String {
        serde_json::to_string(&serde_json::Value::Array(
            task_names
                .into_iter()
                .map(|task_name| {
                    json!({
                        "label": task_name,
                        "command": "echo",
                        "args": vec![task_name],
                    })
                })
                .collect::<Vec<_>>(),
        ))
        .unwrap()
    }

    async fn list_tasks_sorted_by_last_used(
        inventory: &Entity<Inventory>,
        worktree: Option<WorktreeId>,
        cx: &mut TestAppContext,
    ) -> Vec<(TaskSourceKind, String)> {
        let (used, current) = inventory
            .update(cx, |inventory, cx| {
                let mut task_contexts = TaskContexts::default();
                task_contexts.active_worktree_context =
                    worktree.map(|worktree| (worktree, TaskContext::default()));

                inventory.used_and_current_resolved_tasks(Arc::new(task_contexts), cx)
            })
            .await;
        let mut all = used;
        all.extend(current);
        all.into_iter()
            .map(|(source_kind, task)| (source_kind, task.resolved_label))
            .sorted_by_key(|(kind, label)| (task_source_kind_preference(kind), label.clone()))
            .collect()
    }
}
