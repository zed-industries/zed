use crate::{
    first_worktree_repository, git_status_icon, settings::GitPanelSettings, CommitAllChanges,
    CommitStagedChanges, GitListEntry, GitState, GitViewMode, RevertAll, StageAll, UnstageAll,
};
use anyhow::{Context as _, Result};
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use futures::future::OptionFuture;
use git::repository::{GitFileStatus, RepoPath};
use gpui::*;
use language::Buffer;
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrev};
use project::{Fs, Project, ProjectEntryId, WorktreeId};
use serde::{Deserialize, Serialize};
use settings::Settings as _;
use std::{collections::HashSet, ops::Range, path::PathBuf, sync::Arc, time::Duration, usize};
use theme::ThemeSettings;
use ui::{
    prelude::*, Checkbox, Divider, DividerColor, ElevationIndex, Scrollbar, ScrollbarState, Tooltip,
};
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

actions!(git_panel, [ToggleFocus, OpenEntryMenu]);

const GIT_PANEL_KEY: &str = "GitPanel";

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

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

#[derive(Serialize, Deserialize)]
struct SerializedGitPanel {
    width: Option<Pixels>,
}

pub struct GitPanel {
    current_modifiers: Modifiers,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    hide_scrollbar_task: Option<Task<()>>,
    pending_serialization: Task<Option<()>>,
    project: Model<Project>,
    scroll_handle: UniformListScrollHandle,
    scrollbar_state: ScrollbarState,
    selected_entry: Option<usize>,
    show_scrollbar: bool,
    pending_update: Task<()>,
    git_state: Model<GitState>,
    commit_editor: View<Editor>,
    /// The visible entries in the list, accounting for folding & expanded state.
    ///
    /// At this point it doesn't matter what repository the entry belongs to,
    /// as only one repositories' entries are visible in the list at a time.
    visible_entries: Vec<GitListEntry>,
    width: Option<Pixels>,
    reveal_in_editor: Task<()>,
}

impl GitPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move { workspace.update(&mut cx, Self::new) })
    }

    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let fs = workspace.app_state().fs.clone();
        let project = workspace.project().clone();
        let language_registry = workspace.app_state().languages.clone();
        let git_state = GitState::get_global(cx);
        let current_commit_message = {
            let state = git_state.read(cx);
            state.commit_message.clone()
        };

        let git_panel = cx.new_view(|cx: &mut ViewContext<Self>| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, Self::focus_in).detach();
            cx.on_focus_out(&focus_handle, |this, _, cx| {
                this.hide_scrollbar(cx);
            })
            .detach();
            cx.subscribe(&project, move |this, project, event, cx| {
                // TODO: Don't get another git_state here
                // was running into a borrow issue
                let git_state = GitState::get_global(cx);
                let active_repo = git_state.read(cx).active_repository();

                match event {
                    project::Event::WorktreeRemoved(_id) => {
                        this.schedule_update(None, None, cx);
                    }
                    project::Event::WorktreeOrderChanged => {
                        this.schedule_update(None, None, cx);
                    }
                    project::Event::WorktreeUpdatedEntries(id, _)
                    | project::Event::WorktreeAdded(id)
                    | project::Event::WorktreeUpdatedGitRepositories(id) => {
                        let git_state = git_state.clone();
                        if let Some(first_repo) = first_worktree_repository(&project, *id, cx) {
                            git_state.update(cx, |state, _| state.activate_repository(first_repo));

                            this.schedule_update(active_repo, None, cx);
                        } else {
                            this.schedule_update(None, None, cx);
                        }
                    }
                    project::Event::Closed => {
                        this.reveal_in_editor = Task::ready(());
                        this.visible_entries.clear();
                    }
                    _ => {}
                };
            })
            .detach();

            let commit_editor = cx.new_view(|cx| {
                let theme = ThemeSettings::get_global(cx);

                let mut text_style = cx.text_style();
                let refinement = TextStyleRefinement {
                    font_family: Some(theme.buffer_font.family.clone()),
                    font_features: Some(FontFeatures::disable_ligatures()),
                    font_size: Some(px(12.).into()),
                    color: Some(cx.theme().colors().editor_foreground),
                    background_color: Some(gpui::transparent_black()),
                    ..Default::default()
                };

                text_style.refine(&refinement);

                let mut commit_editor = Editor::auto_height(10, cx);
                if let Some(message) = current_commit_message {
                    commit_editor.set_text(message, cx);
                } else {
                    commit_editor.set_text("", cx);
                }
                commit_editor.set_use_autoclose(false);
                commit_editor.set_show_gutter(false, cx);
                commit_editor.set_show_wrap_guides(false, cx);
                commit_editor.set_show_indent_guides(false, cx);
                commit_editor.set_text_style_refinement(refinement);
                commit_editor.set_placeholder_text("Enter commit message", cx);
                commit_editor
            });

            let buffer = commit_editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .expect("commit editor must be singleton");

            cx.subscribe(&buffer, Self::on_buffer_event).detach();

            let markdown = language_registry.language_for_name("Markdown");
            cx.spawn(|_, mut cx| async move {
                let markdown = markdown.await.context("failed to load Markdown language")?;
                buffer.update(&mut cx, |buffer, cx| {
                    buffer.set_language(Some(markdown), cx)
                })
            })
            .detach_and_log_err(cx);

            let scroll_handle = UniformListScrollHandle::new();

            git_state.update(cx, |state, cx| {
                let mut visible_worktrees = project.read(cx).visible_worktrees(cx);
                let Some(first_worktree) = visible_worktrees.next() else {
                    return;
                };
                drop(visible_worktrees);
                let snapshot = first_worktree.read(cx).snapshot();

                if let Some(repository) = first_worktree_repository(&project, snapshot.id(), cx) {
                    state.activate_repository(repository);
                }
            });

            let mut git_panel = Self {
                focus_handle: cx.focus_handle(),
                fs,
                pending_serialization: Task::ready(None),
                visible_entries: Vec::new(),
                current_modifiers: cx.modifiers(),
                width: Some(px(360.)),
                scrollbar_state: ScrollbarState::new(scroll_handle.clone()).parent_view(cx.view()),
                scroll_handle,
                selected_entry: None,
                show_scrollbar: !Self::should_autohide_scrollbar(cx),
                hide_scrollbar_task: None,
                pending_update: Task::ready(()),
                commit_editor,
                git_state,
                reveal_in_editor: Task::ready(()),
                project,
            };
            git_panel.update_visible_entries(None, None, cx);
            git_panel
        });

        git_panel
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        // TODO: we can store stage status here
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
        repo_path: &RepoPath,
        visible_entries: &HashSet<RepoPath>,
    ) -> (usize, usize) {
        // Skip the entry itself when looking at ancestors
        let ancestors = repo_path.ancestors().skip(1);

        // Find the first ancestor that exists in our visible entries
        for ancestor in ancestors {
            if let Some(parent_entry) = visible_entries.get(ancestor) {
                let entry_component_count = repo_path.components().count();
                let parent_component_count = parent_entry.components().count();

                // Calculate how many levels deep this entry is from its parent
                let difference = entry_component_count - parent_component_count;

                // Calculate the parent's depth by counting its ancestors that are in visible_entries
                let parent_depth = parent_entry
                    .ancestors()
                    .skip(1) // Skip the parent itself
                    .filter(|ancestor| visible_entries.contains(*ancestor))
                    .count();

                // This entry's depth is parent's depth + 1
                return (parent_depth + 1, difference);
            }
        }

        // If no ancestor was found, this is a root-level entry
        (0, 0)
    }

    fn scroll_to_selected_entry(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(selected_entry) = self.selected_entry {
            self.scroll_handle
                .scroll_to_item(selected_entry, ScrollStrategy::Center);
        }

        cx.notify();
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        if self.visible_entries.first().is_some() {
            self.selected_entry = Some(0);
            self.scroll_to_selected_entry(cx);
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        let item_count = self.visible_entries.len();
        if item_count == 0 {
            return;
        }

        if let Some(selected_entry) = self.selected_entry {
            let new_selected_entry = if selected_entry > 0 {
                selected_entry - 1
            } else {
                self.selected_entry = Some(item_count - 1);
                item_count - 1
            };

            self.selected_entry = Some(new_selected_entry);

            self.scroll_to_selected_entry(cx);
        }

        cx.notify();
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        let item_count = self.visible_entries.len();
        if item_count == 0 {
            return;
        }

        if let Some(selected_entry) = self.selected_entry {
            let new_selected_entry = if selected_entry < item_count - 1 {
                selected_entry + 1
            } else {
                selected_entry
            };

            self.selected_entry = Some(new_selected_entry);

            self.scroll_to_selected_entry(cx);
        }

        cx.notify();
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        if self.visible_entries.last().is_some() {
            self.selected_entry = Some(self.visible_entries.len() - 1);
            self.scroll_to_selected_entry(cx);
        }
    }
}

impl GitPanel {
    fn stage_all(&mut self, _: &StageAll, cx: &mut ViewContext<Self>) {
        self.git_state
            .update(cx, |state, cx| state.stage_all(&self.project, cx));
    }

    fn unstage_all(&mut self, _: &UnstageAll, cx: &mut ViewContext<Self>) {
        let state = self.git_state.clone();
        state.update(cx, |state, cx| {
            state.unstage_all(&self.project, cx);
        });
    }

    fn discard_all(&mut self, _: &RevertAll, _cx: &mut ViewContext<Self>) {
        // TODO: Implement discard all
        println!("Discard all triggered");
    }

    fn clear_message(&mut self, cx: &mut ViewContext<Self>) {
        let git_state = self.git_state.clone();
        git_state.update(cx, |state, _cx| state.clear_commit_message());
        self.commit_editor
            .update(cx, |editor, cx| editor.set_text("", cx));
    }

    /// Commit all staged changes
    fn commit_staged_changes(&mut self, _: &CommitStagedChanges, cx: &mut ViewContext<Self>) {
        self.clear_message(cx);

        // TODO: Implement commit all staged
        println!("Commit staged changes triggered");
    }

    /// Commit all changes, regardless of whether they are staged or not
    fn commit_all_changes(&mut self, _: &CommitAllChanges, cx: &mut ViewContext<Self>) {
        self.clear_message(cx);

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
        self.visible_entries.len()
    }

    fn for_each_visible_entry(
        &self,
        range: Range<usize>,
        cx: &mut ViewContext<Self>,
        mut callback: impl FnMut(usize, GitListEntry, &mut ViewContext<Self>),
    ) {
        let visible_entries = &self.visible_entries;

        for (ix, entry) in visible_entries
            .iter()
            .enumerate()
            .skip(range.start)
            .take(range.end - range.start)
        {
            let status = entry.status;
            let filename = entry
                .repo_path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| entry.repo_path.to_string_lossy().into_owned());

            let details = GitListEntry {
                repo_path: entry.repo_path.clone(),
                status,
                depth: 0,
                display_name: filename,
            };

            callback(ix, details, cx);
        }
    }

    fn schedule_update(
        &mut self,
        repository: Option<ProjectEntryId>,
        new_selected_entry: Option<(WorktreeId, ProjectEntryId)>,
        cx: &mut ViewContext<Self>,
    ) {
        let handle = cx.view().downgrade();
        let mut rx = self.git_state.read(cx).git_task_rx.clone();

        self.pending_update = cx.spawn(|_, mut cx| async move {
            futures::join!(
                OptionFuture::from(rx.as_mut().map(|rx| rx.recv())),
                cx.background_executor().timer(UPDATE_DEBOUNCE)
            );

            if let Some(handle) = handle.upgrade() {
                handle
                    .update(&mut cx, |panel, cx| {
                        panel.update_visible_entries(repository, new_selected_entry, cx);
                    })
                    .ok();
            }
        });
    }

    /// Update the visible entries for the given repository
    #[track_caller]
    fn update_visible_entries(
        &mut self,
        repository: Option<ProjectEntryId>,
        _new_selected_entry: Option<(WorktreeId, ProjectEntryId)>,
        cx: &mut ViewContext<Self>,
    ) {
        let project = self.project.read(cx);
        let git_state = self.git_state.read(cx);

        // Clear entries if no repository is active
        if git_state.active_repository().is_none() {
            self.visible_entries.clear();
            cx.notify();
            return;
        }

        // If repository is specified, only update for that repository
        let active_repo_id = if let Some(repo_id) = repository {
            // Only update if this is the active repository
            if Some(repo_id) != git_state.active_repository() {
                return;
            }
            repo_id
        } else {
            // Otherwise use the active repository
            match git_state.active_repository() {
                Some(id) => id,
                None => return,
            }
        };

        // Clear existing entries since we're rebuilding
        self.visible_entries.clear();

        // Find the active repository in any worktree
        for worktree in project.visible_worktrees(cx) {
            let snapshot = worktree.read(cx).snapshot();
            let mut repositories = snapshot.repositories();

            if let Some(repository) =
                repositories.find(|repo| repo.work_directory_id() == active_repo_id)
            {
                let entries = || {
                    repository
                        .status()
                        .filter_map(|status| Some((status.repo_path, status.worktree_status?)))
                };
                // First pass - collect all paths
                let path_set = HashSet::from_iter(entries().map(|(repo_path, _)| repo_path));

                // Second pass - create entries with proper depth calculation
                for (repo_path, status) in entries() {
                    let (depth, difference) =
                        Self::calculate_depth_and_difference(&repo_path, &path_set);

                    let display_name = if difference > 1 {
                        // Show partial path for deeply nested files
                        repo_path
                            .as_ref()
                            .iter()
                            .skip(repo_path.components().count() - difference)
                            .collect::<PathBuf>()
                            .to_string_lossy()
                            .into_owned()
                    } else {
                        // Just show filename
                        repo_path
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_default()
                    };

                    let entry = GitListEntry {
                        depth,
                        display_name,
                        repo_path,
                        status,
                    };

                    self.visible_entries.push(entry);
                }

                // Sort entries by path to maintain consistent order
                self.visible_entries
                    .sort_by(|a, b| a.repo_path.cmp(&b.repo_path));

                // We found the active repository, no need to continue
                break;
            }
        }

        cx.notify();
    }

    fn on_buffer_event(
        &mut self,
        _buffer: Model<Buffer>,
        event: &language::BufferEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if let language::BufferEvent::Reparsed | language::BufferEvent::Edited = event {
            let commit_message = self.commit_editor.update(cx, |editor, cx| editor.text(cx));

            self.git_state.update(cx, |state, _cx| {
                state.commit_message = Some(commit_message.into());
            });

            cx.notify();
        }
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

        let changes_string = match self.entry_count() {
            0 => "No changes".to_string(),
            1 => "1 change".to_string(),
            n => format!("{} changes", n),
        };

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
                                    &RevertAll,
                                    &focus_handle,
                                    cx,
                                )
                            })
                            .icon_size(IconSize::Small)
                            .disabled(true),
                    )
                    .child(if self.all_staged() {
                        self.panel_button("unstage-all", "Unstage All").on_click(
                            cx.listener(move |_, _, cx| cx.dispatch_action(Box::new(RevertAll))),
                        )
                    } else {
                        self.panel_button("stage-all", "Stage All").on_click(
                            cx.listener(move |_, _, cx| cx.dispatch_action(Box::new(StageAll))),
                        )
                    }),
            )
    }

    pub fn render_commit_editor(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let editor = self.commit_editor.clone();
        let editor_focus_handle = editor.read(cx).focus_handle(cx).clone();

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
                .id("commit-editor-container")
                .relative()
                .h_full()
                .py_2p5()
                .px_3()
                .bg(cx.theme().colors().editor_background)
                .on_click(cx.listener(move |_, _: &ClickEvent, cx| cx.focus(&editor_focus_handle)))
                .child(self.commit_editor.clone())
                .child(
                    h_flex()
                        .absolute()
                        .bottom_2p5()
                        .right_3()
                        .child(div().gap_1().flex_grow())
                        .child(if self.current_modifiers.alt {
                            commit_all_button
                        } else {
                            commit_staged_button
                        }),
                ),
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
        let entry_count = self.entry_count();
        h_flex()
            .size_full()
            .overflow_hidden()
            .child(
                uniform_list(cx.view().clone(), "entries", entry_count, {
                    move |git_panel, range, cx| {
                        let mut items = Vec::with_capacity(range.end - range.start);
                        git_panel.for_each_visible_entry(range, cx, |ix, details, cx| {
                            items.push(git_panel.render_entry(ix, details, cx));
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
        ix: usize,
        entry_details: GitListEntry,
        cx: &ViewContext<Self>,
    ) -> impl IntoElement {
        let state = self.git_state.clone();
        let repo_path = entry_details.repo_path.clone();
        let is_staged = state
            .read(cx)
            .is_staged(repo_path.clone(), &self.project, cx);
        let selected = self.selected_entry == Some(ix);

        let status = entry_details.status;
        let entry_id = ElementId::Name(format!("entry_{}", entry_details.display_name).into());
        let checkbox_id =
            ElementId::Name(format!("checkbox_{}", entry_details.display_name).into());
        let view_mode = state.read(cx).list_view_mode.clone();
        let handle = cx.view().downgrade();

        let end_slot = h_flex()
            .invisible()
            .when(selected, |this| this.visible())
            .when(!selected, |this| {
                this.group_hover("git-panel-entry", |this| this.visible())
            })
            .gap_1()
            .items_center()
            .child(
                IconButton::new("more", IconName::EllipsisVertical)
                    .icon_color(Color::Placeholder)
                    .icon_size(IconSize::Small),
            );

        let mut entry = h_flex()
            .id(entry_id)
            .group("git-panel-entry")
            .h(px(28.))
            .w_full()
            .pr(px(4.))
            .items_center()
            .gap_2()
            .font_buffer(cx)
            .text_ui_sm(cx)
            .when(!selected, |this| {
                this.hover(|this| this.bg(cx.theme().colors().ghost_element_hover))
            });

        if view_mode == GitViewMode::Tree {
            entry = entry.pl(px(12. + 12. * entry_details.depth as f32))
        } else {
            entry = entry.pl(px(12.))
        }

        if selected {
            entry = entry.bg(cx.theme().status().info_background);
        }

        entry = entry
            .child(
                Checkbox::new(checkbox_id, is_staged.into())
                    .fill()
                    .elevation(ElevationIndex::Surface)
                    .on_click({
                        let project = self.project.clone();
                        move |_, cx| {
                            let repo_path = repo_path.clone();
                            let project = project.clone();
                            state.update(cx, {
                                move |git_state, cx| {
                                    git_state.toggle_staged_entry(repo_path, &project, cx);
                                }
                            });
                        }
                    }),
            )
            .child(git_status_icon(status))
            .child(
                h_flex()
                    .gap_1p5()
                    .when(status == GitFileStatus::Deleted, |this| {
                        this.text_color(cx.theme().colors().text_disabled)
                            .line_through()
                    })
                    .child(entry_details.display_name.clone()),
            )
            .child(div().flex_1())
            .child(end_slot)
            // TODO: Only fire this if the entry is not currently revealed, otherwise the ui flashes
            .on_click(move |_, cx| {
                handle
                    .update(cx, |git_panel, _| {
                        git_panel.selected_entry = Some(ix);
                    })
                    .ok();
            });

        entry
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
                    .on_action(cx.listener(|this, &RevertAll, cx| this.discard_all(&RevertAll, cx)))
                    .on_action(cx.listener(|this, &CommitStagedChanges, cx| {
                        this.commit_staged_changes(&CommitStagedChanges, cx)
                    }))
                    .on_action(cx.listener(|this, &CommitAllChanges, cx| {
                        this.commit_all_changes(&CommitAllChanges, cx)
                    }))
            })
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_last))
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

    fn position(&self, cx: &WindowContext) -> DockPosition {
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

    fn size(&self, cx: &WindowContext) -> Pixels {
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

    fn activation_priority(&self) -> u32 {
        2
    }
}

// fn diff_display_editor(cx: &mut WindowContext) -> View<Editor> {
//     cx.new_view(|cx| {
//         let multi_buffer = cx.new_model(|_| {
//             MultiBuffer::new(language::Capability::ReadWrite).with_title("Project diff".to_string())
//         });
//         let mut editor = Editor::for_multibuffer(multi_buffer, None, true, cx);
//         editor.set_expand_all_diff_hunks();
//         editor
//     })
// }
