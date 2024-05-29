use std::sync::Arc;

use crate::active_item_selection_properties;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    impl_actions, rems, Action, AnyElement, AppContext, DismissEvent, EventEmitter, FocusableView,
    InteractiveElement, Model, ParentElement, Render, SharedString, Styled, Subscription, Task,
    View, ViewContext, VisualContext, WeakView,
};
use picker::{highlighted_match_with_paths::HighlightedText, Picker, PickerDelegate};
use project::{Project, TaskSourceKind};
use task::{ResolvedTask, TaskContext, TaskId, TaskTemplate};
use ui::{
    div, h_flex, v_flex, ActiveTheme, Button, ButtonCommon, ButtonSize, Clickable, Color,
    FluentBuilder as _, Icon, IconButton, IconButtonShape, IconName, IconSize, IntoElement,
    KeyBinding, LabelSize, ListItem, ListItemSpacing, RenderOnce, Selectable, Tooltip,
    WindowContext,
};
use util::ResultExt;
use workspace::{tasks::schedule_resolved_task, ModalView, Workspace};

use serde::Deserialize;

/// Spawn a task with name or open tasks modal
#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct Spawn {
    #[serde(default)]
    /// Name of the task to spawn.
    /// If it is not set, a modal with a list of available tasks is opened instead.
    /// Defaults to None.
    pub task_name: Option<String>,
}

impl Spawn {
    pub fn modal() -> Self {
        Self { task_name: None }
    }
}

/// Rerun last task
#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct Rerun {
    /// Controls whether the task context is reevaluated prior to execution of a task.
    /// If it is not, environment variables such as ZED_COLUMN, ZED_FILE are gonna be the same as in the last execution of a task
    /// If it is, these variables will be updated to reflect current state of editor at the time task::Rerun is executed.
    /// default: false
    #[serde(default)]
    pub reevaluate_context: bool,
    /// Overrides `allow_concurrent_runs` property of the task being reran.
    /// Default: null
    #[serde(default)]
    pub allow_concurrent_runs: Option<bool>,
    /// Overrides `use_new_terminal` property of the task being reran.
    /// Default: null
    #[serde(default)]
    pub use_new_terminal: Option<bool>,

    /// If present, rerun the task with this ID, otherwise rerun the last task.
    pub task_id: Option<TaskId>,
}

impl_actions!(task, [Rerun, Spawn]);

/// A modal used to spawn new tasks.
pub(crate) struct TasksModalDelegate {
    project: Model<Project>,
    candidates: Option<Vec<(TaskSourceKind, ResolvedTask)>>,
    last_used_candidate_index: Option<usize>,
    divider_index: Option<usize>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    workspace: WeakView<Workspace>,
    prompt: String,
    task_context: TaskContext,
    placeholder_text: Arc<str>,
}

impl TasksModalDelegate {
    fn new(
        project: Model<Project>,
        task_context: TaskContext,
        workspace: WeakView<Workspace>,
    ) -> Self {
        Self {
            project,
            workspace,
            candidates: None,
            matches: Vec::new(),
            last_used_candidate_index: None,
            divider_index: None,
            selected_index: 0,
            prompt: String::default(),
            task_context,
            placeholder_text: Arc::from("Find a task, or run a command"),
        }
    }

    fn spawn_oneshot(&mut self) -> Option<(TaskSourceKind, ResolvedTask)> {
        if self.prompt.trim().is_empty() {
            return None;
        }

        let source_kind = TaskSourceKind::UserInput;
        let id_base = source_kind.to_id_base();
        let new_oneshot = TaskTemplate {
            label: self.prompt.clone(),
            command: self.prompt.clone(),
            ..TaskTemplate::default()
        };
        Some((
            source_kind,
            new_oneshot.resolve_task(&id_base, &self.task_context)?,
        ))
    }

    fn delete_previously_used(&mut self, ix: usize, cx: &mut AppContext) {
        let Some(candidates) = self.candidates.as_mut() else {
            return;
        };
        let Some(task) = candidates.get(ix).map(|(_, task)| task.clone()) else {
            return;
        };
        // We remove this candidate manually instead of .taking() the candidates, as we already know the index;
        // it doesn't make sense to requery the inventory for new candidates, as that's potentially costly and more often than not it should just return back
        // the original list without a removed entry.
        candidates.remove(ix);
        self.project.update(cx, |project, cx| {
            project.task_inventory().update(cx, |inventory, _| {
                inventory.delete_previously_used(&task.id);
            })
        });
    }
}

pub(crate) struct TasksModal {
    picker: View<Picker<TasksModalDelegate>>,
    _subscription: Subscription,
}

impl TasksModal {
    pub(crate) fn new(
        project: Model<Project>,
        task_context: TaskContext,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let picker = cx.new_view(|cx| {
            Picker::uniform_list(
                TasksModalDelegate::new(project, task_context, workspace),
                cx,
            )
        });
        let _subscription = cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        });
        Self {
            picker,
            _subscription,
        }
    }
}

impl Render for TasksModal {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        v_flex()
            .key_context("TasksModal")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

impl EventEmitter<DismissEvent> for TasksModal {}

impl FocusableView for TasksModal {
    fn focus_handle(&self, cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.picker.read(cx).focus_handle(cx)
    }
}

impl ModalView for TasksModal {}

impl PickerDelegate for TasksModalDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<picker::Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _: &mut WindowContext) -> Arc<str> {
        self.placeholder_text.clone()
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<picker::Picker<Self>>,
    ) -> Task<()> {
        cx.spawn(move |picker, mut cx| async move {
            let Some(candidates_task) = picker
                .update(&mut cx, |picker, cx| {
                    match &mut picker.delegate.candidates {
                        Some(candidates) => {
                            Task::ready(Ok(string_match_candidates(candidates.iter())))
                        }
                        None => {
                            let Ok((worktree, location)) =
                                picker.delegate.workspace.update(cx, |workspace, cx| {
                                    active_item_selection_properties(workspace, cx)
                                })
                            else {
                                return Task::ready(Ok(Vec::new()));
                            };

                            let resolved_task =
                                picker.delegate.project.update(cx, |project, cx| {
                                    let ssh_connection_string = project.ssh_connection_string(cx);
                                    if project.is_remote() && ssh_connection_string.is_none() {
                                        Task::ready((Vec::new(), Vec::new()))
                                    } else {
                                        let remote_templates = if project.is_local() {
                                            None
                                        } else {
                                            project
                                                .remote_id()
                                                .filter(|_| ssh_connection_string.is_some())
                                                .map(|project_id| {
                                                    project.query_remote_task_templates(
                                                        project_id,
                                                        worktree,
                                                        location.as_ref(),
                                                        cx,
                                                    )
                                                })
                                        };
                                        project
                                            .task_inventory()
                                            .read(cx)
                                            .used_and_current_resolved_tasks(
                                                remote_templates,
                                                worktree,
                                                location,
                                                &picker.delegate.task_context,
                                                cx,
                                            )
                                    }
                                });
                            cx.spawn(|picker, mut cx| async move {
                                let (used, current) = resolved_task.await;
                                picker.update(&mut cx, |picker, _| {
                                    picker.delegate.last_used_candidate_index = if used.is_empty() {
                                        None
                                    } else {
                                        Some(used.len() - 1)
                                    };

                                    let mut new_candidates = used;
                                    new_candidates.extend(current);
                                    let match_candidates =
                                        string_match_candidates(new_candidates.iter());
                                    let _ = picker.delegate.candidates.insert(new_candidates);
                                    match_candidates
                                })
                            })
                        }
                    }
                })
                .ok()
            else {
                return;
            };
            let Some(candidates): Option<Vec<StringMatchCandidate>> =
                candidates_task.await.log_err()
            else {
                return;
            };
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                true,
                1000,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;
            picker
                .update(&mut cx, |picker, _| {
                    let delegate = &mut picker.delegate;
                    delegate.matches = matches;
                    if let Some(index) = delegate.last_used_candidate_index {
                        delegate.matches.sort_by_key(|m| m.candidate_id > index);
                    }

                    delegate.prompt = query;
                    delegate.divider_index = delegate.last_used_candidate_index.and_then(|index| {
                        let index = delegate
                            .matches
                            .partition_point(|matching_task| matching_task.candidate_id <= index);
                        Some(index).and_then(|index| (index != 0).then(|| index - 1))
                    });

                    if delegate.matches.is_empty() {
                        delegate.selected_index = 0;
                    } else {
                        delegate.selected_index =
                            delegate.selected_index.min(delegate.matches.len() - 1);
                    }
                })
                .log_err();
        })
    }

    fn confirm(&mut self, omit_history_entry: bool, cx: &mut ViewContext<picker::Picker<Self>>) {
        let current_match_index = self.selected_index();
        let task = self
            .matches
            .get(current_match_index)
            .and_then(|current_match| {
                let ix = current_match.candidate_id;
                self.candidates
                    .as_ref()
                    .map(|candidates| candidates[ix].clone())
            });
        let Some((task_source_kind, task)) = task else {
            return;
        };

        self.workspace
            .update(cx, |workspace, cx| {
                schedule_resolved_task(workspace, task_source_kind, task, omit_history_entry, cx);
            })
            .ok();
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<picker::Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let candidates = self.candidates.as_ref()?;
        let hit = &self.matches[ix];
        let (source_kind, resolved_task) = &candidates.get(hit.candidate_id)?;
        let template = resolved_task.original_task();
        let display_label = resolved_task.display_label();

        let mut tooltip_label_text = if display_label != &template.label {
            resolved_task.resolved_label.clone()
        } else {
            String::new()
        };
        if let Some(resolved) = resolved_task.resolved.as_ref() {
            if resolved.command_label != display_label
                && resolved.command_label != resolved_task.resolved_label
            {
                if !tooltip_label_text.trim().is_empty() {
                    tooltip_label_text.push('\n');
                }
                tooltip_label_text.push_str(&resolved.command_label);
            }
        }
        let tooltip_label = if tooltip_label_text.trim().is_empty() {
            None
        } else {
            Some(Tooltip::text(tooltip_label_text, cx))
        };

        let highlighted_location = HighlightedText {
            text: hit.string.clone(),
            highlight_positions: hit.positions.clone(),
            char_count: hit.string.chars().count(),
            color: Color::Default,
        };
        let icon = match source_kind {
            TaskSourceKind::UserInput => Some(Icon::new(IconName::Terminal)),
            TaskSourceKind::AbsPath { .. } => Some(Icon::new(IconName::Settings)),
            TaskSourceKind::Worktree { .. } => Some(Icon::new(IconName::FileTree)),
            TaskSourceKind::Language { name } => file_icons::FileIcons::get(cx)
                .get_type_icon(&name.to_lowercase())
                .map(|icon_path| Icon::from_path(icon_path)),
        }
        .map(|icon| icon.color(Color::Muted).size(IconSize::Small));
        let history_run_icon = if Some(ix) <= self.divider_index {
            Some(
                Icon::new(IconName::HistoryRerun)
                    .color(Color::Muted)
                    .size(IconSize::Small)
                    .into_any_element(),
            )
        } else {
            Some(
                v_flex()
                    .flex_none()
                    .size(IconSize::Small.rems())
                    .into_any_element(),
            )
        };

        Some(
            ListItem::new(SharedString::from(format!("tasks-modal-{ix}")))
                .inset(false)
                .start_slot::<Icon>(icon)
                .end_slot::<AnyElement>(history_run_icon)
                .spacing(ListItemSpacing::Sparse)
                // .map(|this| {
                //     if Some(ix) <= self.divider_index {
                //         this.start_slot(Icon::new(IconName::HistoryRerun).size(IconSize::Small))
                //     } else {
                //         this.start_slot(v_flex().flex_none().size(IconSize::Small.rems()))
                //     }
                // })
                .when_some(tooltip_label, |list_item, item_label| {
                    list_item.tooltip(move |_| item_label.clone())
                })
                .map(|item| {
                    let item = if matches!(source_kind, TaskSourceKind::UserInput)
                        || Some(ix) <= self.divider_index
                    {
                        let task_index = hit.candidate_id;
                        let delete_button = div().child(
                            IconButton::new("delete", IconName::Close)
                                .shape(IconButtonShape::Square)
                                .icon_color(Color::Muted)
                                .size(ButtonSize::None)
                                .icon_size(IconSize::XSmall)
                                .on_click(cx.listener(move |picker, _event, cx| {
                                    cx.stop_propagation();
                                    cx.prevent_default();

                                    picker.delegate.delete_previously_used(task_index, cx);
                                    picker.delegate.last_used_candidate_index = picker
                                        .delegate
                                        .last_used_candidate_index
                                        .unwrap_or(0)
                                        .checked_sub(1);
                                    picker.refresh(cx);
                                }))
                                .tooltip(|cx| {
                                    Tooltip::text("Delete previously scheduled task", cx)
                                }),
                        );
                        item.end_hover_slot(delete_button)
                    } else {
                        item
                    };
                    item
                })
                .selected(selected)
                .child(highlighted_location.render(cx)),
        )
    }

    fn selected_as_query(&self) -> Option<String> {
        let task_index = self.matches.get(self.selected_index())?.candidate_id;
        let tasks = self.candidates.as_ref()?;
        let (_, task) = tasks.get(task_index)?;
        Some(task.resolved.as_ref()?.command_label.clone())
    }

    fn confirm_input(&mut self, omit_history_entry: bool, cx: &mut ViewContext<Picker<Self>>) {
        let Some((task_source_kind, task)) = self.spawn_oneshot() else {
            return;
        };
        self.workspace
            .update(cx, |workspace, cx| {
                schedule_resolved_task(workspace, task_source_kind, task, omit_history_entry, cx);
            })
            .ok();
        cx.emit(DismissEvent);
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        if let Some(i) = self.divider_index {
            vec![i]
        } else {
            Vec::new()
        }
    }
    fn render_footer(&self, cx: &mut ViewContext<Picker<Self>>) -> Option<gpui::AnyElement> {
        let is_recent_selected = self.divider_index >= Some(self.selected_index);
        let current_modifiers = cx.modifiers();
        Some(
            h_flex()
                .w_full()
                .h_8()
                .p_2()
                .justify_between()
                .rounded_b_md()
                .bg(cx.theme().colors().ghost_element_selected)
                .children(
                    KeyBinding::for_action(&picker::UseSelectedQuery, cx).map(|keybind| {
                        let edit_entry_label = if is_recent_selected {
                            "Edit task"
                        } else if !self.matches.is_empty() {
                            "Edit template"
                        } else {
                            "Rerun last task"
                        };

                        Button::new("edit-current-task", edit_entry_label)
                            .label_size(LabelSize::Small)
                            .key_binding(keybind)
                            .on_click(|_, cx| {
                                cx.dispatch_action(picker::UseSelectedQuery.boxed_clone())
                            })
                    }),
                )
                .map(|this| {
                    if (current_modifiers.alt || self.matches.is_empty()) && !self.prompt.is_empty()
                    {
                        let action = picker::ConfirmInput {
                            secondary: current_modifiers.secondary(),
                        }
                        .boxed_clone();
                        this.children(KeyBinding::for_action(&*action, cx).map(|keybind| {
                            let spawn_oneshot_label = if current_modifiers.secondary() {
                                "Spawn oneshot without history"
                            } else {
                                "Spawn oneshot"
                            };

                            Button::new("spawn-onehshot", spawn_oneshot_label)
                                .label_size(LabelSize::Small)
                                .key_binding(keybind)
                                .on_click(move |_, cx| cx.dispatch_action(action.boxed_clone()))
                        }))
                    } else if current_modifiers.secondary() {
                        this.children(KeyBinding::for_action(&menu::SecondaryConfirm, cx).map(
                            |keybind| {
                                let label = if is_recent_selected {
                                    "Rerun without history"
                                } else {
                                    "Spawn without history"
                                };
                                Button::new("spawn", label)
                                    .label_size(LabelSize::Small)
                                    .key_binding(keybind)
                                    .on_click(move |_, cx| {
                                        cx.dispatch_action(menu::SecondaryConfirm.boxed_clone())
                                    })
                            },
                        ))
                    } else {
                        this.children(KeyBinding::for_action(&menu::Confirm, cx).map(|keybind| {
                            let run_entry_label =
                                if is_recent_selected { "Rerun" } else { "Spawn" };

                            Button::new("spawn", run_entry_label)
                                .label_size(LabelSize::Small)
                                .key_binding(keybind)
                                .on_click(|_, cx| {
                                    cx.dispatch_action(menu::Confirm.boxed_clone());
                                })
                        }))
                    }
                })
                .into_any_element(),
        )
    }
}

fn string_match_candidates<'a>(
    candidates: impl Iterator<Item = &'a (TaskSourceKind, ResolvedTask)> + 'a,
) -> Vec<StringMatchCandidate> {
    candidates
        .enumerate()
        .map(|(index, (_, candidate))| StringMatchCandidate {
            id: index,
            char_bag: candidate.resolved_label.chars().collect(),
            string: candidate.display_label().to_owned(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use editor::Editor;
    use gpui::{TestAppContext, VisualTestContext};
    use language::{Language, LanguageConfig, LanguageMatcher, Point};
    use project::{ContextProviderWithTasks, FakeFs, Project};
    use serde_json::json;
    use task::TaskTemplates;
    use workspace::CloseInactiveTabsAndPanes;

    use crate::{modal::Spawn, tests::init_test};

    use super::*;

    #[gpui::test]
    async fn test_spawn_tasks_modal_query_reuse(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/dir",
            json!({
                ".zed": {
                    "tasks.json": r#"[
                        {
                            "label": "example task",
                            "command": "echo",
                            "args": ["4"]
                        },
                        {
                            "label": "another one",
                            "command": "echo",
                            "args": ["55"]
                        },
                    ]"#,
                },
                "a.ts": "a"
            }),
        )
        .await;

        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));

        let tasks_picker = open_spawn_tasks(&workspace, cx);
        assert_eq!(
            query(&tasks_picker, cx),
            "",
            "Initial query should be empty"
        );
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec!["another one", "example task"],
            "Initial tasks should be listed in alphabetical order"
        );

        let query_str = "tas";
        cx.simulate_input(query_str);
        assert_eq!(query(&tasks_picker, cx), query_str);
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec!["example task"],
            "Only one task should match the query {query_str}"
        );

        cx.dispatch_action(picker::UseSelectedQuery);
        assert_eq!(
            query(&tasks_picker, cx),
            "echo 4",
            "Query should be set to the selected task's command"
        );
        assert_eq!(
            task_names(&tasks_picker, cx),
            Vec::<String>::new(),
            "No task should be listed"
        );
        cx.dispatch_action(picker::ConfirmInput { secondary: false });

        let tasks_picker = open_spawn_tasks(&workspace, cx);
        assert_eq!(
            query(&tasks_picker, cx),
            "",
            "Query should be reset after confirming"
        );
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec!["echo 4", "another one", "example task"],
            "New oneshot task should be listed first"
        );

        let query_str = "echo 4";
        cx.simulate_input(query_str);
        assert_eq!(query(&tasks_picker, cx), query_str);
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec!["echo 4"],
            "New oneshot should match custom command query"
        );

        cx.dispatch_action(picker::ConfirmInput { secondary: false });
        let tasks_picker = open_spawn_tasks(&workspace, cx);
        assert_eq!(
            query(&tasks_picker, cx),
            "",
            "Query should be reset after confirming"
        );
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec![query_str, "another one", "example task"],
            "Last recently used one show task should be listed first"
        );

        cx.dispatch_action(picker::UseSelectedQuery);
        assert_eq!(
            query(&tasks_picker, cx),
            query_str,
            "Query should be set to the custom task's name"
        );
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec![query_str],
            "Only custom task should be listed"
        );

        let query_str = "0";
        cx.simulate_input(query_str);
        assert_eq!(query(&tasks_picker, cx), "echo 40");
        assert_eq!(
            task_names(&tasks_picker, cx),
            Vec::<String>::new(),
            "New oneshot should not match any command query"
        );

        cx.dispatch_action(picker::ConfirmInput { secondary: true });
        let tasks_picker = open_spawn_tasks(&workspace, cx);
        assert_eq!(
            query(&tasks_picker, cx),
            "",
            "Query should be reset after confirming"
        );
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec!["echo 4", "another one", "example task"],
            "No query should be added to the list, as it was submitted with secondary action (that maps to omit_history = true)"
        );

        cx.dispatch_action(Spawn {
            task_name: Some("example task".to_string()),
        });
        let tasks_picker = workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<TasksModal>(cx)
                .unwrap()
                .read(cx)
                .picker
                .clone()
        });
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec!["echo 4", "another one", "example task"],
        );
    }

    #[gpui::test]
    async fn test_basic_context_for_simple_files(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/dir",
            json!({
                ".zed": {
                    "tasks.json": r#"[
                        {
                            "label": "hello from $ZED_FILE:$ZED_ROW:$ZED_COLUMN",
                            "command": "echo",
                            "args": ["hello", "from", "$ZED_FILE", ":", "$ZED_ROW", ":", "$ZED_COLUMN"]
                        },
                        {
                            "label": "opened now: $ZED_WORKTREE_ROOT",
                            "command": "echo",
                            "args": ["opened", "now:", "$ZED_WORKTREE_ROOT"]
                        }
                    ]"#,
                },
                "file_without_extension": "aaaaaaaaaaaaaaaaaaaa\naaaaaaaaaaaaaaaaaa",
                "file_with.odd_extension": "b",
            }),
        )
        .await;

        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

        let tasks_picker = open_spawn_tasks(&workspace, cx);
        assert_eq!(
            task_names(&tasks_picker, cx),
            Vec::<String>::new(),
            "Should list no file or worktree context-dependent when no file is open"
        );
        tasks_picker.update(cx, |_, cx| {
            cx.emit(DismissEvent);
        });
        drop(tasks_picker);
        cx.executor().run_until_parked();

        let _ = workspace
            .update(cx, |workspace, cx| {
                workspace.open_abs_path(PathBuf::from("/dir/file_with.odd_extension"), true, cx)
            })
            .await
            .unwrap();
        cx.executor().run_until_parked();
        let tasks_picker = open_spawn_tasks(&workspace, cx);
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec![
                "hello from …th.odd_extension:1:1".to_string(),
                "opened now: /dir".to_string()
            ],
            "Second opened buffer should fill the context, labels should be trimmed if long enough"
        );
        tasks_picker.update(cx, |_, cx| {
            cx.emit(DismissEvent);
        });
        drop(tasks_picker);
        cx.executor().run_until_parked();

        let second_item = workspace
            .update(cx, |workspace, cx| {
                workspace.open_abs_path(PathBuf::from("/dir/file_without_extension"), true, cx)
            })
            .await
            .unwrap();

        let editor = cx.update(|cx| second_item.act_as::<Editor>(cx)).unwrap();
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_ranges(Some(Point::new(1, 2)..Point::new(1, 5)))
            })
        });
        cx.executor().run_until_parked();
        let tasks_picker = open_spawn_tasks(&workspace, cx);
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec![
                "hello from …ithout_extension:2:3".to_string(),
                "opened now: /dir".to_string()
            ],
            "Opened buffer should fill the context, labels should be trimmed if long enough"
        );
        tasks_picker.update(cx, |_, cx| {
            cx.emit(DismissEvent);
        });
        drop(tasks_picker);
        cx.executor().run_until_parked();
    }

    #[gpui::test]
    async fn test_language_task_filtering(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/dir",
            json!({
                "a1.ts": "// a1",
                "a2.ts": "// a2",
                "b.rs": "// b",
            }),
        )
        .await;

        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        project.read_with(cx, |project, _| {
            let language_registry = project.languages();
            language_registry.add(Arc::new(
                Language::new(
                    LanguageConfig {
                        name: "TypeScript".into(),
                        matcher: LanguageMatcher {
                            path_suffixes: vec!["ts".to_string()],
                            ..LanguageMatcher::default()
                        },
                        ..LanguageConfig::default()
                    },
                    None,
                )
                .with_context_provider(Some(Arc::new(
                    ContextProviderWithTasks::new(TaskTemplates(vec![
                        TaskTemplate {
                            label: "Task without variables".to_string(),
                            command: "npm run clean".to_string(),
                            ..TaskTemplate::default()
                        },
                        TaskTemplate {
                            label: "TypeScript task from file $ZED_FILE".to_string(),
                            command: "npm run build".to_string(),
                            ..TaskTemplate::default()
                        },
                        TaskTemplate {
                            label: "Another task from file $ZED_FILE".to_string(),
                            command: "npm run lint".to_string(),
                            ..TaskTemplate::default()
                        },
                    ])),
                ))),
            ));
            language_registry.add(Arc::new(
                Language::new(
                    LanguageConfig {
                        name: "Rust".into(),
                        matcher: LanguageMatcher {
                            path_suffixes: vec!["rs".to_string()],
                            ..LanguageMatcher::default()
                        },
                        ..LanguageConfig::default()
                    },
                    None,
                )
                .with_context_provider(Some(Arc::new(
                    ContextProviderWithTasks::new(TaskTemplates(vec![TaskTemplate {
                        label: "Rust task".to_string(),
                        command: "cargo check".into(),
                        ..TaskTemplate::default()
                    }])),
                ))),
            ));
        });
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

        let _ts_file_1 = workspace
            .update(cx, |workspace, cx| {
                workspace.open_abs_path(PathBuf::from("/dir/a1.ts"), true, cx)
            })
            .await
            .unwrap();
        let tasks_picker = open_spawn_tasks(&workspace, cx);
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec![
                "Another task from file /dir/a1.ts",
                "TypeScript task from file /dir/a1.ts",
                "Task without variables",
            ],
            "Should open spawn TypeScript tasks for the opened file, tasks with most template variables above, all groups sorted alphanumerically"
        );
        emulate_task_schedule(
            tasks_picker,
            &project,
            "TypeScript task from file /dir/a1.ts",
            cx,
        );

        let tasks_picker = open_spawn_tasks(&workspace, cx);
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec!["TypeScript task from file /dir/a1.ts", "TypeScript task from file /dir/a1.ts", "Another task from file /dir/a1.ts", "Task without variables"],
            "After spawning the task and getting it into the history, it should be up in the sort as recently used"
        );
        tasks_picker.update(cx, |_, cx| {
            cx.emit(DismissEvent);
        });
        drop(tasks_picker);
        cx.executor().run_until_parked();

        let _ts_file_2 = workspace
            .update(cx, |workspace, cx| {
                workspace.open_abs_path(PathBuf::from("/dir/a2.ts"), true, cx)
            })
            .await
            .unwrap();
        let tasks_picker = open_spawn_tasks(&workspace, cx);
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec![
                "TypeScript task from file /dir/a1.ts",
                "Another task from file /dir/a2.ts",
                "TypeScript task from file /dir/a2.ts",
                "Task without variables"
            ],
            "Even when both TS files are open, should only show the history (on the top), and tasks, resolved for the current file"
        );
        tasks_picker.update(cx, |_, cx| {
            cx.emit(DismissEvent);
        });
        drop(tasks_picker);
        cx.executor().run_until_parked();

        let _rs_file = workspace
            .update(cx, |workspace, cx| {
                workspace.open_abs_path(PathBuf::from("/dir/b.rs"), true, cx)
            })
            .await
            .unwrap();
        let tasks_picker = open_spawn_tasks(&workspace, cx);
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec!["Rust task"],
            "Even when both TS files are open and one TS task spawned, opened file's language tasks should be displayed only"
        );

        cx.dispatch_action(CloseInactiveTabsAndPanes::default());
        emulate_task_schedule(tasks_picker, &project, "Rust task", cx);
        let _ts_file_2 = workspace
            .update(cx, |workspace, cx| {
                workspace.open_abs_path(PathBuf::from("/dir/a2.ts"), true, cx)
            })
            .await
            .unwrap();
        let tasks_picker = open_spawn_tasks(&workspace, cx);
        assert_eq!(
            task_names(&tasks_picker, cx),
            vec![
                "TypeScript task from file /dir/a1.ts",
                "Another task from file /dir/a2.ts",
                "TypeScript task from file /dir/a2.ts",
                "Task without variables"
            ],
            "After closing all but *.rs tabs, running a Rust task and switching back to TS tasks, \
            same TS spawn history should be restored"
        );
    }

    fn emulate_task_schedule(
        tasks_picker: View<Picker<TasksModalDelegate>>,
        project: &Model<Project>,
        scheduled_task_label: &str,
        cx: &mut VisualTestContext,
    ) {
        let scheduled_task = tasks_picker.update(cx, |tasks_picker, _| {
            tasks_picker
                .delegate
                .candidates
                .iter()
                .flatten()
                .find(|(_, task)| task.resolved_label == scheduled_task_label)
                .cloned()
                .unwrap()
        });
        project.update(cx, |project, cx| {
            project.task_inventory().update(cx, |inventory, _| {
                let (kind, task) = scheduled_task;
                inventory.task_scheduled(kind, task);
            })
        });
        tasks_picker.update(cx, |_, cx| {
            cx.emit(DismissEvent);
        });
        drop(tasks_picker);
        cx.executor().run_until_parked()
    }

    fn open_spawn_tasks(
        workspace: &View<Workspace>,
        cx: &mut VisualTestContext,
    ) -> View<Picker<TasksModalDelegate>> {
        cx.dispatch_action(Spawn::default());
        workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<TasksModal>(cx)
                .expect("no task modal after `Spawn` action was dispatched")
                .read(cx)
                .picker
                .clone()
        })
    }

    fn query(spawn_tasks: &View<Picker<TasksModalDelegate>>, cx: &mut VisualTestContext) -> String {
        spawn_tasks.update(cx, |spawn_tasks, cx| spawn_tasks.query(cx))
    }

    fn task_names(
        spawn_tasks: &View<Picker<TasksModalDelegate>>,
        cx: &mut VisualTestContext,
    ) -> Vec<String> {
        spawn_tasks.update(cx, |spawn_tasks, _| {
            spawn_tasks
                .delegate
                .matches
                .iter()
                .map(|hit| hit.string.clone())
                .collect::<Vec<_>>()
        })
    }
}
