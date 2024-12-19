use anyhow::Result;
use collections::HashMap;
use db::kvp::KEY_VALUE_STORE;
use git::repository::GitFileStatus;
use gpui::{
    actions, prelude::*, uniform_list, Action, AppContext, AsyncWindowContext, ClickEvent,
    CursorStyle, EventEmitter, FocusHandle, FocusableView, KeyContext,
    ListHorizontalSizingBehavior, ListSizingBehavior, Model, Modifiers, ModifiersChangedEvent,
    MouseButton, Stateful, Task, UniformListScrollHandle, View, WeakView,
};
use project::{Entry, EntryKind, Fs, Project, ProjectEntryId, WorktreeId};
use serde::{Deserialize, Serialize};
use settings::Settings as _;
use std::{
    cell::OnceCell,
    collections::HashSet,
    ffi::OsStr,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use ui::{
    prelude::*, Checkbox, Divider, DividerColor, ElevationIndex, Scrollbar, ScrollbarState, Tooltip,
};
use util::{ResultExt, TryFutureExt};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::Workspace;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    ItemHandle,
};

use crate::{git_status_icon, settings::GitPanelSettings};
use crate::{CommitAllChanges, CommitStagedChanges, DiscardAll, StageAll, UnstageAll};

actions!(git_panel, [ToggleFocus]);

const GIT_PANEL_KEY: &str = "GitPanel";

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleFocus, cx| {
                workspace.toggle_panel_focus::<GitPanel>(cx);
            });
        },
    )
    .detach();
}

#[derive(Debug)]
pub enum Event {
    Focus,
}

pub struct GitStatusEntry {}

#[derive(Debug, PartialEq, Eq, Clone)]
struct EntryDetails {
    filename: String,
    display_name: String,
    path: Arc<Path>,
    kind: EntryKind,
    depth: usize,
    is_expanded: bool,
    status: Option<GitFileStatus>,
}

impl EntryDetails {
    pub fn is_dir(&self) -> bool {
        self.kind.is_dir()
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedGitPanel {
    width: Option<Pixels>,
}

pub struct GitPanel {
    workspace: WeakView<Workspace>,
    current_modifiers: Modifiers,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    hide_scrollbar_task: Option<Task<()>>,
    pending_serialization: Task<Option<()>>,
    project: Model<Project>,
    scroll_handle: UniformListScrollHandle,
    scrollbar_state: ScrollbarState,
    selected_item: Option<usize>,
    show_scrollbar: bool,
    expanded_dir_ids: HashMap<WorktreeId, Vec<ProjectEntryId>>,

    // The entries that are currently shown in the panel, aka
    // not hidden by folding or such
    visible_entries: Vec<(WorktreeId, Vec<Entry>, OnceCell<HashSet<Arc<Path>>>)>,
    width: Option<Pixels>,
    git_diff_editor: View<Editor>,
    git_diff_editor_updates: Task<()>,
}

impl GitPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            // Clippy incorrectly classifies this as a redundant closure
            #[allow(clippy::redundant_closure)]
            workspace.update(&mut cx, |workspace, cx| Self::new(workspace, cx))
        })
    }

    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let fs = workspace.app_state().fs.clone();
        let weak_workspace = workspace.weak_handle();
        let project = workspace.project().clone();

        let git_panel = cx.new_view(|cx: &mut ViewContext<Self>| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, Self::focus_in).detach();
            cx.on_focus_out(&focus_handle, |this, _, cx| {
                this.hide_scrollbar(cx);
            })
            .detach();
            cx.subscribe(&project, |this, project, event, cx| match event {
                project::Event::WorktreeRemoved(id) => {
                    this.expanded_dir_ids.remove(id);
                    this.update_visible_entries(None, None, cx);
                    cx.notify();
                }
                project::Event::WorktreeOrderChanged => {
                    this.update_visible_entries(None, None, cx);
                    cx.notify();
                }
                project::Event::WorktreeUpdatedEntries(id, _)
                | project::Event::WorktreeAdded(id)
                | project::Event::WorktreeUpdatedGitRepositories(id) => {
                    this.update_visible_entries(Some(*id), None, cx);
                    cx.notify();
                }
                project::Event::Closed => {
                    this.git_diff_editor_updates = Task::ready(());
                    this.expanded_dir_ids.clear();
                    this.visible_entries.clear();
                    this.git_diff_editor = diff_display_editor(project.clone(), cx);
                }
                _ => {}
            })
            .detach();

            let scroll_handle = UniformListScrollHandle::new();

            let mut this = Self {
                workspace: weak_workspace,
                focus_handle: cx.focus_handle(),
                fs,
                pending_serialization: Task::ready(None),
                visible_entries: Vec::new(),
                current_modifiers: cx.modifiers(),
                expanded_dir_ids: Default::default(),

                width: Some(px(360.)),
                scrollbar_state: ScrollbarState::new(scroll_handle.clone()).parent_view(cx.view()),
                scroll_handle,
                selected_item: None,
                show_scrollbar: !Self::should_autohide_scrollbar(cx),
                hide_scrollbar_task: None,
                git_diff_editor: diff_display_editor(project.clone(), cx),
                git_diff_editor_updates: Task::ready(()),
                project,
            };
            this.update_visible_entries(None, None, cx);
            this
        });

        git_panel
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background_executor().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        GIT_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedGitPanel { width })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn dispatch_context(&self) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("GitPanel");
        dispatch_context.add("menu");

        dispatch_context
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        if !self.focus_handle.contains_focused(cx) {
            cx.emit(Event::Focus);
        }
    }

    fn should_show_scrollbar(_cx: &AppContext) -> bool {
        // TODO: plug into settings
        true
    }

    fn should_autohide_scrollbar(_cx: &AppContext) -> bool {
        // TODO: plug into settings
        true
    }

    fn hide_scrollbar(&mut self, cx: &mut ViewContext<Self>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);
        if !Self::should_autohide_scrollbar(cx) {
            return;
        }
        self.hide_scrollbar_task = Some(cx.spawn(|panel, mut cx| async move {
            cx.background_executor()
                .timer(SCROLLBAR_SHOW_INTERVAL)
                .await;
            panel
                .update(&mut cx, |panel, cx| {
                    panel.show_scrollbar = false;
                    cx.notify();
                })
                .log_err();
        }))
    }

    fn handle_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        self.current_modifiers = event.modifiers;
        cx.notify();
    }

    fn calculate_depth_and_difference(
        entry: &Entry,
        visible_worktree_entries: &HashSet<Arc<Path>>,
    ) -> (usize, usize) {
        let (depth, difference) = entry
            .path
            .ancestors()
            .skip(1) // Skip the entry itself
            .find_map(|ancestor| {
                if let Some(parent_entry) = visible_worktree_entries.get(ancestor) {
                    let entry_path_components_count = entry.path.components().count();
                    let parent_path_components_count = parent_entry.components().count();
                    let difference = entry_path_components_count - parent_path_components_count;
                    let depth = parent_entry
                        .ancestors()
                        .skip(1)
                        .filter(|ancestor| visible_worktree_entries.contains(*ancestor))
                        .count();
                    Some((depth + 1, difference))
                } else {
                    None
                }
            })
            .unwrap_or((0, 0));

        (depth, difference)
    }
}

impl GitPanel {
    fn stage_all(&mut self, _: &StageAll, _cx: &mut ViewContext<Self>) {
        // TODO: Implement stage all
        println!("Stage all triggered");
    }

    fn unstage_all(&mut self, _: &UnstageAll, _cx: &mut ViewContext<Self>) {
        // TODO: Implement unstage all
        println!("Unstage all triggered");
    }

    fn discard_all(&mut self, _: &DiscardAll, _cx: &mut ViewContext<Self>) {
        // TODO: Implement discard all
        println!("Discard all triggered");
    }

    /// Commit all staged changes
    fn commit_staged_changes(&mut self, _: &CommitStagedChanges, _cx: &mut ViewContext<Self>) {
        // TODO: Implement commit all staged
        println!("Commit staged changes triggered");
    }

    /// Commit all changes, regardless of whether they are staged or not
    fn commit_all_changes(&mut self, _: &CommitAllChanges, _cx: &mut ViewContext<Self>) {
        // TODO: Implement commit all changes
        println!("Commit all changes triggered");
    }

    fn all_staged(&self) -> bool {
        // TODO: Implement all_staged
        true
    }

    fn no_entries(&self) -> bool {
        self.visible_entries.is_empty()
    }

    fn entry_count(&self) -> usize {
        self.visible_entries
            .iter()
            .map(|(_, entries, _)| {
                entries
                    .iter()
                    .filter(|entry| entry.git_status.is_some())
                    .count()
            })
            .sum()
    }

    fn for_each_visible_entry(
        &self,
        range: Range<usize>,
        cx: &mut ViewContext<Self>,
        mut callback: impl FnMut(ProjectEntryId, EntryDetails, &mut ViewContext<Self>),
    ) {
        let mut ix = 0;
        for (worktree_id, visible_worktree_entries, entries_paths) in &self.visible_entries {
            if ix >= range.end {
                return;
            }

            if ix + visible_worktree_entries.len() <= range.start {
                ix += visible_worktree_entries.len();
                continue;
            }

            let end_ix = range.end.min(ix + visible_worktree_entries.len());
            // let entry_range = range.start.saturating_sub(ix)..end_ix - ix;
            if let Some(worktree) = self.project.read(cx).worktree_for_id(*worktree_id, cx) {
                let snapshot = worktree.read(cx).snapshot();
                let root_name = OsStr::new(snapshot.root_name());
                let expanded_entry_ids = self
                    .expanded_dir_ids
                    .get(&snapshot.id())
                    .map(Vec::as_slice)
                    .unwrap_or(&[]);

                let entry_range = range.start.saturating_sub(ix)..end_ix - ix;
                let entries = entries_paths.get_or_init(|| {
                    visible_worktree_entries
                        .iter()
                        .map(|e| (e.path.clone()))
                        .collect()
                });

                for entry in visible_worktree_entries[entry_range].iter() {
                    let status = entry.git_status;
                    let is_expanded = expanded_entry_ids.binary_search(&entry.id).is_ok();

                    let (depth, difference) = Self::calculate_depth_and_difference(entry, entries);

                    let filename = match difference {
                        diff if diff > 1 => entry
                            .path
                            .iter()
                            .skip(entry.path.components().count() - diff)
                            .collect::<PathBuf>()
                            .to_str()
                            .unwrap_or_default()
                            .to_string(),
                        _ => entry
                            .path
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_else(|| root_name.to_string_lossy().to_string()),
                    };

                    let display_name = entry.path.to_string_lossy().into_owned();

                    let details = EntryDetails {
                        filename,
                        display_name,
                        kind: entry.kind,
                        is_expanded,
                        path: entry.path.clone(),
                        status,
                        depth,
                    };
                    callback(entry.id, details, cx);
                }
            }
            ix = end_ix;
        }
    }

    // TODO: Update expanded directory state
    // TODO: update happens in the main loop, could be long for large workspaces
    fn update_visible_entries(
        &mut self,
        for_worktree: Option<WorktreeId>,
        new_selected_entry: Option<(WorktreeId, ProjectEntryId)>,
        cx: &mut ViewContext<Self>,
    ) {
        let project = self.project.read(cx);
        let mut old_entries_removed = false;
        let mut after_update = Vec::new();
        self.visible_entries.retain(
            |visible_entry @ (visible_worktree_id, _, _)| match for_worktree {
                Some(for_worktree) => {
                    if *visible_worktree_id == for_worktree {
                        old_entries_removed = true;
                        false
                    } else if old_entries_removed {
                        after_update.push(visible_entry.clone());
                        false
                    } else {
                        true
                    }
                }
                None => true,
            },
        );
        for worktree in project.visible_worktrees(cx) {
            let worktree_id = worktree.read(cx).id();
            if for_worktree.is_some() && for_worktree != Some(worktree_id) {
                continue;
            }
            let snapshot = worktree.read(cx).snapshot();

            let mut visible_worktree_entries = snapshot
                .entries(false, 0)
                .filter(|entry| !entry.is_external)
                .filter(|entry| entry.git_status.is_some())
                .cloned()
                .collect::<Vec<_>>();
            snapshot.propagate_git_statuses(&mut visible_worktree_entries);
            project::sort_worktree_entries(&mut visible_worktree_entries);

            if !visible_worktree_entries.is_empty() {
                self.visible_entries
                    .push((worktree_id, visible_worktree_entries, OnceCell::new()));
            }
        }
        self.visible_entries.extend(after_update);

        if let Some((worktree_id, entry_id)) = new_selected_entry {
            self.selected_item = self.visible_entries.iter().enumerate().find_map(
                |(worktree_index, (id, entries, _))| {
                    if *id == worktree_id {
                        entries
                            .iter()
                            .position(|entry| entry.id == entry_id)
                            .map(|entry_index| worktree_index * entries.len() + entry_index)
                    } else {
                        None
                    }
                },
            );
        }

        let project = self.project.clone();
        self.git_diff_editor_updates = cx.spawn(|git_panel, mut cx| async move {
            cx.background_executor()
                .timer(Duration::from_millis(50))
                .await;
            let Some(project_buffers) = git_panel
                .update(&mut cx, |git_panel, cx| {
                    futures::future::join_all(git_panel.visible_entries.iter().flat_map(
                        move |(_, entries, _)| {
                            entries
                                .iter()
                                .filter_map(|entry| {
                                    let (entry_path, unstaged_changes_task) =
                                        project.update(cx, |project, cx| {
                                            // TODO kb are these None for Added?
                                            let entry_path =
                                                project.path_for_entry(entry.id, cx)?;
                                            let open_task =
                                                project.open_path(entry_path.clone(), cx);
                                            let unstaged_changes_task =
                                                cx.spawn(|project, mut cx| async move {
                                                    let (_, opened_model) = open_task
                                                        .await
                                                        .context("opening buffer")?;
                                                    let buffer = opened_model
                                                        .downcast::<Buffer>()
                                                        .map_err(|_| {
                                                            anyhow::anyhow!(
                                                                "accessing buffer for entry"
                                                            )
                                                        })?;
                                                    let unstaged_changes = project
                                                        .update(&mut cx, |project, cx| {
                                                            project.open_unstaged_changes(
                                                                buffer.clone(),
                                                                cx,
                                                            )
                                                        })?
                                                        .await
                                                        .context("opening unstaged changes")?;

                                                    let hunks = unstaged_changes.update(
                                                        &mut cx,
                                                        |unstaged_changes, cx| {
                                                            let buffer_snapshot =
                                                                buffer.read(cx).snapshot();
                                                            unstaged_changes
                                                                .diff_to_buffer
                                                                .hunks_in_row_range(
                                                                    0..BufferRow::MAX,
                                                                    &buffer_snapshot,
                                                                )
                                                                .collect::<Vec<_>>()
                                                        },
                                                    )?;

                                                    anyhow::Ok((buffer, unstaged_changes, hunks))
                                                });
                                            Some((entry_path, unstaged_changes_task))
                                        })?;
                                    Some((entry_path, unstaged_changes_task))
                                })
                                .map(|(entry_path, open_task)| async move {
                                    (entry_path, open_task.await)
                                })
                                .collect::<Vec<_>>()
                        },
                    ))
                })
                .ok()
            else {
                return;
            };

            let project_buffers = project_buffers.await;
            if project_buffers.is_empty() {
                return;
            }
            if let Some(buffer_update_task) = git_panel
                .update(&mut cx, |git_panel, cx| {
                    let editor = git_panel.git_diff_editor.clone();
                    let multi_buffer = editor.read(cx).buffer().clone();
                    let mut buffers_with_ranges = Vec::with_capacity(project_buffers.len());
                    for (buffer_path, open_result) in project_buffers {
                        if let Some((buffer, change_set, diff_hunks)) = open_result
                            .with_context(|| format!("opening buffer {buffer_path:?}"))
                            .log_err()
                        {
                            editor.update(cx, |editor, cx| editor.add_change_set(change_set, cx));
                            buffers_with_ranges.push((
                                buffer,
                                diff_hunks
                                    .into_iter()
                                    .map(|hunk| hunk.buffer_range)
                                    .collect(),
                            ));
                        }
                    }

                    multi_buffer.update(cx, |multi_buffer, cx| {
                        multi_buffer.clear(cx);
                        multi_buffer.push_multiple_excerpts_with_context_lines(
                            buffers_with_ranges,
                            DEFAULT_MULTIBUFFER_CONTEXT,
                            cx,
                        )
                    })
                })
                .ok()
            {
                buffer_update_task.await;
                git_panel
                    .update(&mut cx, |git_panel, cx| {
                        git_panel.git_diff_editor.update(cx, |editor, cx| {
                            editor.expand_all_hunk_diffs(&Default::default(), cx)
                        })
                    })
                    .ok();
            }
        });

        cx.notify();
    }
}

impl GitPanel {
    pub fn panel_button(
        &self,
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
    ) -> Button {
        let id = id.into().clone();
        let label = label.into().clone();

        Button::new(id, label)
            .label_size(LabelSize::Small)
            .layer(ElevationIndex::ElevatedSurface)
            .size(ButtonSize::Compact)
            .style(ButtonStyle::Filled)
    }

    pub fn render_divider(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .items_center()
            .h(px(8.))
            .child(Divider::horizontal_dashed().color(DividerColor::Border))
    }

    pub fn render_panel_header(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx).clone();

        let changes_string = format!("{} changes", self.entry_count());

        h_flex()
            .h(px(32.))
            .items_center()
            .px_3()
            .bg(ElevationIndex::Surface.bg(cx))
            .child(
                h_flex()
                    .gap_2()
                    .child(Checkbox::new("all-changes", true.into()).disabled(true))
                    .child(div().text_buffer(cx).text_ui_sm(cx).child(changes_string)),
            )
            .child(div().flex_grow())
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        IconButton::new("discard-changes", IconName::Undo)
                            .tooltip(move |cx| {
                                let focus_handle = focus_handle.clone();

                                Tooltip::for_action_in(
                                    "Discard all changes",
                                    &DiscardAll,
                                    &focus_handle,
                                    cx,
                                )
                            })
                            .icon_size(IconSize::Small)
                            .disabled(true),
                    )
                    .child(if self.all_staged() {
                        self.panel_button("unstage-all", "Unstage All").on_click(
                            cx.listener(move |_, _, cx| cx.dispatch_action(Box::new(DiscardAll))),
                        )
                    } else {
                        self.panel_button("stage-all", "Stage All").on_click(
                            cx.listener(move |_, _, cx| cx.dispatch_action(Box::new(StageAll))),
                        )
                    }),
            )
    }

    pub fn render_commit_editor(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let focus_handle_1 = self.focus_handle(cx).clone();
        let focus_handle_2 = self.focus_handle(cx).clone();

        let commit_staged_button = self
            .panel_button("commit-staged-changes", "Commit")
            .tooltip(move |cx| {
                let focus_handle = focus_handle_1.clone();
                Tooltip::for_action_in(
                    "Commit all staged changes",
                    &CommitStagedChanges,
                    &focus_handle,
                    cx,
                )
            })
            .on_click(cx.listener(|this, _: &ClickEvent, cx| {
                this.commit_staged_changes(&CommitStagedChanges, cx)
            }));

        let commit_all_button = self
            .panel_button("commit-all-changes", "Commit All")
            .tooltip(move |cx| {
                let focus_handle = focus_handle_2.clone();
                Tooltip::for_action_in(
                    "Commit all changes, including unstaged changes",
                    &CommitAllChanges,
                    &focus_handle,
                    cx,
                )
            })
            .on_click(cx.listener(|this, _: &ClickEvent, cx| {
                this.commit_all_changes(&CommitAllChanges, cx)
            }));

        div().w_full().h(px(140.)).px_2().pt_1().pb_2().child(
            v_flex()
                .h_full()
                .py_2p5()
                .px_3()
                .bg(cx.theme().colors().editor_background)
                .font_buffer(cx)
                .text_ui_sm(cx)
                .text_color(cx.theme().colors().text_muted)
                .child("Add a message")
                .gap_1()
                .child(div().flex_grow())
                .child(h_flex().child(div().gap_1().flex_grow()).child(
                    if self.current_modifiers.alt {
                        commit_all_button
                    } else {
                        commit_staged_button
                    },
                ))
                .cursor(CursorStyle::OperationNotAllowed)
                .opacity(0.5),
        )
    }

    fn render_empty_state(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .h_full()
            .flex_1()
            .justify_center()
            .items_center()
            .child(
                v_flex()
                    .gap_3()
                    .child("No changes to commit")
                    .text_ui_sm(cx)
                    .mx_auto()
                    .text_color(Color::Placeholder.color(cx)),
            )
    }

    fn render_scrollbar(&self, cx: &mut ViewContext<Self>) -> Option<Stateful<Div>> {
        if !Self::should_show_scrollbar(cx)
            || !(self.show_scrollbar || self.scrollbar_state.is_dragging())
        {
            return None;
        }
        Some(
            div()
                .occlude()
                .id("project-panel-vertical-scroll")
                .on_mouse_move(cx.listener(|_, _, cx| {
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _, cx| {
                        if !this.scrollbar_state.is_dragging()
                            && !this.focus_handle.contains_focused(cx)
                        {
                            this.hide_scrollbar(cx);
                            cx.notify();
                        }

                        cx.stop_propagation();
                    }),
                )
                .on_scroll_wheel(cx.listener(|_, _, cx| {
                    cx.notify();
                }))
                .h_full()
                .absolute()
                .right_1()
                .top_1()
                .bottom_1()
                .w(px(12.))
                .cursor_default()
                .children(Scrollbar::vertical(
                    // percentage as f32..end_offset as f32,
                    self.scrollbar_state.clone(),
                )),
        )
    }

    fn render_entries(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let item_count = self
            .visible_entries
            .iter()
            .map(|(_, worktree_entries, _)| worktree_entries.len())
            .sum();
        h_flex()
            .size_full()
            .overflow_hidden()
            .child(
                uniform_list(cx.view().clone(), "entries", item_count, {
                    |this, range, cx| {
                        let mut items = Vec::with_capacity(range.end - range.start);
                        this.for_each_visible_entry(range, cx, |id, details, cx| {
                            items.push(this.render_entry(id, details, cx));
                        });
                        items
                    }
                })
                .size_full()
                .with_sizing_behavior(ListSizingBehavior::Infer)
                .with_horizontal_sizing_behavior(ListHorizontalSizingBehavior::Unconstrained)
                // .with_width_from_item(self.max_width_item_index)
                .track_scroll(self.scroll_handle.clone()),
            )
            .children(self.render_scrollbar(cx))
    }

    fn render_entry(
        &self,
        id: ProjectEntryId,
        details: EntryDetails,
        cx: &ViewContext<Self>,
    ) -> impl IntoElement {
        let id = id.to_proto() as usize;
        let checkbox_id = ElementId::Name(format!("checkbox_{}", id).into());
        let is_staged = ToggleState::Selected;
        let handle = cx.view().clone();

        h_flex()
            .id(id)
            .h(px(28.))
            .w_full()
            .pl(px(12. + 12. * details.depth as f32))
            .pr(px(4.))
            .items_center()
            .gap_2()
            .font_buffer(cx)
            .text_ui_sm(cx)
            .when(!details.is_dir(), |this| {
                this.child(Checkbox::new(checkbox_id, is_staged))
            })
            .when_some(details.status, |this, status| {
                this.child(git_status_icon(status))
            })
            .child(h_flex().gap_1p5().child(details.display_name.clone()))
            .on_click(move |_, cx| {
                handle.update(cx, |git_panel, cx| {
                    git_panel
                        .workspace
                        .update(cx, |workspace, cx| {
                            let diff_editor = workspace
                                .items_of_type::<Editor>(cx)
                                .find(|editor| &git_panel.git_diff_editor == editor);
                            let editor = match diff_editor {
                                Some(existing_editor) => {
                                    workspace.activate_item(&existing_editor, true, true, cx);
                                    existing_editor
                                }
                                None => {
                                    workspace.active_pane().update(cx, |pane, cx| {
                                        pane.add_item(
                                            git_panel.git_diff_editor.boxed_clone(),
                                            true,
                                            true,
                                            None,
                                            cx,
                                        )
                                    });
                                    git_panel.git_diff_editor.clone()
                                }
                            };
                            // TODO kb scroll to the entry clicked
                            // TODO kb all new added files are not displayed
                        })
                        .ok();
                });
            })
    }
}

impl Render for GitPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let project = self.project.read(cx);

        v_flex()
            .id("git_panel")
            .key_context(self.dispatch_context())
            .track_focus(&self.focus_handle)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .when(!project.is_read_only(cx), |this| {
                this.on_action(cx.listener(|this, &StageAll, cx| this.stage_all(&StageAll, cx)))
                    .on_action(
                        cx.listener(|this, &UnstageAll, cx| this.unstage_all(&UnstageAll, cx)),
                    )
                    .on_action(
                        cx.listener(|this, &DiscardAll, cx| this.discard_all(&DiscardAll, cx)),
                    )
                    .on_action(cx.listener(|this, &CommitStagedChanges, cx| {
                        this.commit_staged_changes(&CommitStagedChanges, cx)
                    }))
                    .on_action(cx.listener(|this, &CommitAllChanges, cx| {
                        this.commit_all_changes(&CommitAllChanges, cx)
                    }))
            })
            .on_hover(cx.listener(|this, hovered, cx| {
                if *hovered {
                    this.show_scrollbar = true;
                    this.hide_scrollbar_task.take();
                    cx.notify();
                } else if !this.focus_handle.contains_focused(cx) {
                    this.hide_scrollbar(cx);
                }
            }))
            .size_full()
            .overflow_hidden()
            .font_buffer(cx)
            .py_1()
            .bg(ElevationIndex::Surface.bg(cx))
            .child(self.render_panel_header(cx))
            .child(self.render_divider(cx))
            .child(if !self.no_entries() {
                self.render_entries(cx).into_any_element()
            } else {
                self.render_empty_state(cx).into_any_element()
            })
            .child(self.render_divider(cx))
            .child(self.render_commit_editor(cx))
    }
}

impl FocusableView for GitPanel {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<Event> for GitPanel {}

impl EventEmitter<PanelEvent> for GitPanel {}

impl Panel for GitPanel {
    fn persistent_name() -> &'static str {
        "GitPanel"
    }

    fn position(&self, cx: &gpui::WindowContext) -> DockPosition {
        GitPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<GitPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| settings.dock = Some(position),
        );
    }

    fn size(&self, cx: &gpui::WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| GitPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, cx: &WindowContext) -> Option<ui::IconName> {
        Some(ui::IconName::GitBranch).filter(|_| GitPanelSettings::get_global(cx).button)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Git Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
}

fn diff_display_editor(project: Model<Project>, cx: &mut WindowContext) -> View<Editor> {
    cx.new_view(|cx| {
        let multi_buffer = cx.new_model(|cx| {
            MultiBuffer::new(project.read(cx).capability()).with_title("Project diff".to_string())
        });
        let mut editor = Editor::for_multibuffer(multi_buffer, Some(project), true, cx);
        editor.set_expand_all_diff_hunks();
        editor
    })
}
