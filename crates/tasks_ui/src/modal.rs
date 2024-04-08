use std::{path::PathBuf, sync::Arc};

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    impl_actions, rems, AppContext, DismissEvent, EventEmitter, FocusableView, InteractiveElement,
    Model, ParentElement, Render, SharedString, Styled, Subscription, View, ViewContext,
    VisualContext, WeakView,
};
use picker::{
    highlighted_match_with_paths::{HighlightedMatchWithPaths, HighlightedText},
    Picker, PickerDelegate,
};
use project::{Inventory, ProjectPath, TaskSourceKind};
use task::{oneshot_source::OneshotSource, Task, TaskContext};
use ui::{
    div, v_flex, ButtonCommon, ButtonSize, Clickable, Color, FluentBuilder as _, IconButton,
    IconButtonShape, IconName, IconSize, ListItem, ListItemSpacing, RenderOnce, Selectable,
    Tooltip, WindowContext,
};
use util::{paths::PathExt, ResultExt};
use workspace::{ModalView, Workspace};

use crate::schedule_task;
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

/// Rerun last task
#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct Rerun {
    #[serde(default)]
    /// Controls whether the task context is reevaluated prior to execution of a task.
    /// If it is not, environment variables such as ZED_COLUMN, ZED_FILE are gonna be the same as in the last execution of a task
    /// If it is, these variables will be updated to reflect current state of editor at the time task::Rerun is executed.
    /// default: false
    pub reevaluate_context: bool,
}

impl_actions!(task, [Rerun, Spawn]);

/// A modal used to spawn new tasks.
pub(crate) struct TasksModalDelegate {
    inventory: Model<Inventory>,
    candidates: Option<Vec<(TaskSourceKind, Arc<dyn Task>)>>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    workspace: WeakView<Workspace>,
    prompt: String,
    task_context: TaskContext,
}

impl TasksModalDelegate {
    fn new(
        inventory: Model<Inventory>,
        task_context: TaskContext,
        workspace: WeakView<Workspace>,
    ) -> Self {
        Self {
            inventory,
            workspace,
            candidates: None,
            matches: Vec::new(),
            selected_index: 0,
            prompt: String::default(),
            task_context,
        }
    }

    fn spawn_oneshot(&mut self, cx: &mut AppContext) -> Option<Arc<dyn Task>> {
        if self.prompt.trim().is_empty() {
            return None;
        }

        self.inventory
            .update(cx, |inventory, _| inventory.source::<OneshotSource>())?
            .update(cx, |oneshot_source, _| {
                Some(
                    oneshot_source
                        .as_any()
                        .downcast_mut::<OneshotSource>()?
                        .spawn(self.prompt.clone()),
                )
            })
    }

    fn delete_oneshot(&mut self, ix: usize, cx: &mut AppContext) {
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
        self.inventory.update(cx, |inventory, cx| {
            let oneshot_source = inventory.source::<OneshotSource>()?;
            let task_id = task.id();

            oneshot_source.update(cx, |this, _| {
                let oneshot_source = this.as_any().downcast_mut::<OneshotSource>()?;
                oneshot_source.remove(task_id);
                Some(())
            });
            Some(())
        });
    }
    fn active_item_path(
        workspace: &WeakView<Workspace>,
        cx: &mut ViewContext<'_, Picker<Self>>,
    ) -> Option<(PathBuf, ProjectPath)> {
        let workspace = workspace.upgrade()?.read(cx);
        let project = workspace.project().read(cx);
        let active_item = workspace.active_item(cx)?;
        active_item.project_path(cx).and_then(|project_path| {
            project
                .worktree_for_id(project_path.worktree_id, cx)
                .map(|worktree| worktree.read(cx).abs_path().join(&project_path.path))
                .zip(Some(project_path))
        })
    }
}

pub(crate) struct TasksModal {
    picker: View<Picker<TasksModalDelegate>>,
    _subscription: Subscription,
}

impl TasksModal {
    pub(crate) fn new(
        inventory: Model<Inventory>,
        task_context: TaskContext,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let picker = cx.new_view(|cx| {
            Picker::uniform_list(
                TasksModalDelegate::new(inventory, task_context, workspace),
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        v_flex()
            .key_context("TasksModal")
            .w(rems(34.))
            .child(self.picker.clone())
            .on_mouse_down_out(cx.listener(|modal, _, cx| {
                modal.picker.update(cx, |picker, cx| {
                    picker.cancel(&Default::default(), cx);
                })
            }))
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

    fn placeholder_text(&self, cx: &mut WindowContext) -> Arc<str> {
        Arc::from(format!(
            "{} use task name as prompt, {} spawns a bash-like task from the prompt, {} runs the selected task",
            cx.keystroke_text_for(&picker::UseSelectedQuery),
            cx.keystroke_text_for(&picker::ConfirmInput {secondary: false}),
            cx.keystroke_text_for(&menu::Confirm),
        ))
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<picker::Picker<Self>>,
    ) -> gpui::Task<()> {
        cx.spawn(move |picker, mut cx| async move {
            let Some(candidates) = picker
                .update(&mut cx, |picker, cx| {
                    let candidates = picker.delegate.candidates.get_or_insert_with(|| {
                        let (path, worktree) =
                            match Self::active_item_path(&picker.delegate.workspace, cx) {
                                Some((abs_path, project_path)) => {
                                    (Some(abs_path), Some(project_path.worktree_id))
                                }
                                None => (None, None),
                            };
                        picker.delegate.inventory.update(cx, |inventory, cx| {
                            inventory.list_tasks(path.as_deref(), worktree, true, cx)
                        })
                    });

                    candidates
                        .iter()
                        .enumerate()
                        .map(|(index, (_, candidate))| StringMatchCandidate {
                            id: index,
                            char_bag: candidate.name().chars().collect(),
                            string: candidate.name().into(),
                        })
                        .collect::<Vec<_>>()
                })
                .ok()
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
                    delegate.prompt = query;

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
                    .map(|candidates| candidates[ix].1.clone())
            });
        let Some(task) = task else {
            return;
        };

        self.workspace
            .update(cx, |workspace, cx| {
                schedule_task(
                    workspace,
                    task.as_ref(),
                    self.task_context.clone(),
                    omit_history_entry,
                    cx,
                );
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
        let hit = &self.matches.get(ix)?;
        let (source_kind, _) = &candidates.get(hit.candidate_id)?;
        let details = match source_kind {
            TaskSourceKind::UserInput => "user input".to_string(),
            TaskSourceKind::Buffer => "language extension".to_string(),
            TaskSourceKind::Worktree { abs_path, .. } | TaskSourceKind::AbsPath(abs_path) => {
                abs_path.compact().to_string_lossy().to_string()
            }
        };

        let highlighted_location = HighlightedMatchWithPaths {
            match_label: HighlightedText {
                text: hit.string.clone(),
                highlight_positions: hit.positions.clone(),
                char_count: hit.string.chars().count(),
            },
            paths: vec![HighlightedText {
                char_count: details.chars().count(),
                highlight_positions: Vec::new(),
                text: details,
            }],
        };
        Some(
            ListItem::new(SharedString::from(format!("tasks-modal-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .map(|this| {
                    if matches!(source_kind, TaskSourceKind::UserInput) {
                        let task_index = hit.candidate_id;
                        let delete_button = div().child(
                            IconButton::new("delete", IconName::Close)
                                .shape(IconButtonShape::Square)
                                .icon_color(Color::Muted)
                                .size(ButtonSize::None)
                                .icon_size(IconSize::XSmall)
                                .on_click(cx.listener(move |this, _event, cx| {
                                    cx.stop_propagation();
                                    cx.prevent_default();

                                    this.delegate.delete_oneshot(task_index, cx);
                                    this.refresh(cx);
                                }))
                                .tooltip(|cx| Tooltip::text("Delete an one-shot task", cx)),
                        );
                        this.end_hover_slot(delete_button)
                    } else {
                        this
                    }
                })
                .selected(selected)
                .child(highlighted_location.render(cx)),
        )
    }

    fn selected_as_query(&self) -> Option<String> {
        use itertools::intersperse;
        let task_index = self.matches.get(self.selected_index())?.candidate_id;
        let tasks = self.candidates.as_ref()?;
        let (_, task) = tasks.get(task_index)?;
        // .exec doesn't actually spawn anything; it merely prepares a spawning command,
        // which we can use for substitution.
        let mut spawn_prompt = task.exec(self.task_context.clone())?;
        if !spawn_prompt.args.is_empty() {
            spawn_prompt.command.push(' ');
            spawn_prompt
                .command
                .extend(intersperse(spawn_prompt.args, " ".to_string()));
        }
        Some(spawn_prompt.command)
    }

    fn confirm_input(&mut self, omit_history_entry: bool, cx: &mut ViewContext<Picker<Self>>) {
        let Some(task) = self.spawn_oneshot(cx) else {
            return;
        };
        self.workspace
            .update(cx, |workspace, cx| {
                schedule_task(
                    workspace,
                    task.as_ref(),
                    self.task_context.clone(),
                    omit_history_entry,
                    cx,
                );
            })
            .ok();
        cx.emit(DismissEvent);
    }
}

#[cfg(test)]
mod tests {
    use gpui::{TestAppContext, VisualTestContext};
    use project::{FakeFs, Project};
    use serde_json::json;

    use super::*;

    #[gpui::test]
    async fn test_spawn_tasks_modal_query_reuse(cx: &mut TestAppContext) {
        crate::tests::init_test(cx);
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
        project.update(cx, |project, cx| {
            project.task_inventory().update(cx, |inventory, cx| {
                inventory.add_source(TaskSourceKind::UserInput, |cx| OneshotSource::new(cx), cx)
            })
        });

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
            vec!["echo 4", "another one", "example task", "echo 40"],
            "Last recently used one show task should be listed last, as it is a fire-and-forget task"
        );
    }

    fn open_spawn_tasks(
        workspace: &View<Workspace>,
        cx: &mut VisualTestContext,
    ) -> View<Picker<TasksModalDelegate>> {
        cx.dispatch_action(crate::modal::Spawn::default());
        workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<TasksModal>(cx)
                .unwrap()
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
