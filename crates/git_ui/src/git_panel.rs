use crate::git_panel_settings::StatusStyle;
use crate::{git_panel_settings::GitPanelSettings, git_status_icon};
use anyhow::{Context as _, Result};
use db::kvp::KEY_VALUE_STORE;
use editor::actions::MoveToEnd;
use editor::scroll::ScrollbarAutoHide;
use editor::{Editor, EditorSettings, ShowScrollbar};
use futures::channel::mpsc;
use futures::StreamExt as _;
use git::repository::{GitRepository, RepoPath};
use git::status::FileStatus;
use git::{CommitAllChanges, CommitChanges, RevertAll, StageAll, ToggleStaged, UnstageAll};
use gpui::*;
use language::Buffer;
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrev};
use project::git::GitState;
use project::{Fs, Project, ProjectPath, WorktreeId};
use serde::{Deserialize, Serialize};
use settings::Settings as _;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{collections::HashSet, ops::Range, path::PathBuf, sync::Arc, time::Duration, usize};
use theme::ThemeSettings;
use ui::{
    prelude::*, Checkbox, Divider, DividerColor, ElevationIndex, Scrollbar, ScrollbarState, Tooltip,
};
use util::{maybe, ResultExt, TryFutureExt};
use workspace::notifications::{DetachAndPromptErr, NotificationId};
use workspace::Toast;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};
use worktree::RepositoryEntry;

actions!(
    git_panel,
    [
        Close,
        ToggleFocus,
        OpenMenu,
        OpenSelected,
        FocusEditor,
        FocusChanges,
        FillCoAuthors,
    ]
);

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

#[derive(Debug, Clone)]
pub enum Event {
    Focus,
    OpenedEntry { path: ProjectPath },
}

#[derive(Serialize, Deserialize)]
struct SerializedGitPanel {
    width: Option<Pixels>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GitListEntry {
    depth: usize,
    display_name: String,
    repo_path: RepoPath,
    status: FileStatus,
    is_staged: Option<bool>,
}

pub struct GitPanel {
    weak_workspace: WeakView<Workspace>,
    current_modifiers: Modifiers,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    hide_scrollbar_task: Option<Task<()>>,
    pending_serialization: Task<Option<()>>,
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    scroll_handle: UniformListScrollHandle,
    scrollbar_state: ScrollbarState,
    selected_entry: Option<usize>,
    show_scrollbar: bool,
    rebuild_requested: Arc<AtomicBool>,
    commit_editor: View<Editor>,
    visible_entries: Vec<GitListEntry>,
    all_staged: Option<bool>,
    width: Option<Pixels>,
    reveal_in_editor: Task<()>,
    err_sender: mpsc::Sender<anyhow::Error>,
}

fn first_worktree_repository(
    project: &Model<Project>,
    worktree_id: WorktreeId,
    cx: &mut AppContext,
) -> Option<(RepositoryEntry, Arc<dyn GitRepository>)> {
    project
        .read(cx)
        .worktree_for_id(worktree_id, cx)
        .and_then(|worktree| {
            let snapshot = worktree.read(cx).snapshot();
            let repo = snapshot.repositories().iter().next()?.clone();
            let git_repo = worktree
                .read(cx)
                .as_local()?
                .get_local_repo(&repo)?
                .repo()
                .clone();
            Some((repo, git_repo))
        })
}

fn first_repository_in_project(
    project: &Model<Project>,
    cx: &mut AppContext,
) -> Option<(WorktreeId, RepositoryEntry, Arc<dyn GitRepository>)> {
    project.read(cx).worktrees(cx).next().and_then(|worktree| {
        let snapshot = worktree.read(cx).snapshot();
        let repo = snapshot.repositories().iter().next()?.clone();
        let git_repo = worktree
            .read(cx)
            .as_local()?
            .get_local_repo(&repo)?
            .repo()
            .clone();
        Some((snapshot.id(), repo, git_repo))
    })
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
        let weak_workspace = cx.view().downgrade();
        let git_state = project.read(cx).git_state().cloned();
        let language_registry = workspace.app_state().languages.clone();
        let current_commit_message = git_state
            .as_ref()
            .map(|git_state| git_state.read(cx).commit_message.clone());
        let (err_sender, mut err_receiver) = mpsc::channel(1);
        let workspace = cx.view().downgrade();

        let git_panel = cx.new_view(|cx: &mut ViewContext<Self>| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, Self::focus_in).detach();
            cx.on_focus_out(&focus_handle, |this, _, cx| {
                this.hide_scrollbar(cx);
            })
            .detach();
            cx.subscribe(&project, move |this, project, event, cx| {
                use project::Event;

                let first_worktree_id = project.read(cx).worktrees(cx).next().map(|worktree| {
                    let snapshot = worktree.read(cx).snapshot();
                    snapshot.id()
                });
                let first_repo_in_project = first_repository_in_project(&project, cx);

                let Some(git_state) = project.read(cx).git_state().cloned() else {
                    return;
                };
                git_state.update(cx, |git_state, _| {
                    match event {
                        project::Event::WorktreeRemoved(id) => {
                            let Some((worktree_id, _, _)) = git_state.active_repository.as_ref()
                            else {
                                return;
                            };
                            if worktree_id == id {
                                git_state.active_repository = first_repo_in_project;
                                this.schedule_update();
                            }
                        }
                        project::Event::WorktreeOrderChanged => {
                            // activate the new first worktree if the first was moved
                            let Some(first_id) = first_worktree_id else {
                                return;
                            };
                            if !git_state
                                .active_repository
                                .as_ref()
                                .is_some_and(|(id, _, _)| id == &first_id)
                            {
                                git_state.active_repository = first_repo_in_project;
                                this.schedule_update();
                            }
                        }
                        Event::WorktreeAdded(_) => {
                            let Some(first_id) = first_worktree_id else {
                                return;
                            };
                            if !git_state
                                .active_repository
                                .as_ref()
                                .is_some_and(|(id, _, _)| id == &first_id)
                            {
                                git_state.active_repository = first_repo_in_project;
                                this.schedule_update();
                            }
                        }
                        project::Event::WorktreeUpdatedEntries(id, _) => {
                            if git_state
                                .active_repository
                                .as_ref()
                                .is_some_and(|(active_id, _, _)| active_id == id)
                            {
                                git_state.active_repository = first_repo_in_project;
                                this.schedule_update();
                            }
                        }
                        project::Event::WorktreeUpdatedGitRepositories(_) => {
                            let Some(first) = first_repo_in_project else {
                                return;
                            };
                            git_state.active_repository = Some(first);
                            this.schedule_update();
                        }
                        project::Event::Closed => {
                            this.reveal_in_editor = Task::ready(());
                            this.visible_entries.clear();
                        }
                        _ => {}
                    };
                });
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

            let mut visible_worktrees = project.read(cx).visible_worktrees(cx);
            let first_worktree = visible_worktrees.next();
            drop(visible_worktrees);
            if let Some(first_worktree) = first_worktree {
                let snapshot = first_worktree.read(cx).snapshot();

                if let Some(((repo, git_repo), git_state)) =
                    first_worktree_repository(&project, snapshot.id(), cx).zip(git_state)
                {
                    git_state.update(cx, |git_state, _| {
                        git_state.activate_repository(snapshot.id(), repo, git_repo);
                    });
                }
            };

            let rebuild_requested = Arc::new(AtomicBool::new(false));
            let flag = rebuild_requested.clone();
            let handle = cx.view().downgrade();
            cx.spawn(|_, mut cx| async move {
                loop {
                    cx.background_executor().timer(UPDATE_DEBOUNCE).await;
                    if flag.load(Ordering::Relaxed) {
                        if let Some(this) = handle.upgrade() {
                            this.update(&mut cx, |this, cx| {
                                this.update_visible_entries(cx);
                            })
                            .ok();
                        }
                        flag.store(false, Ordering::Relaxed);
                    }
                }
            })
            .detach();

            let mut git_panel = Self {
                weak_workspace,
                focus_handle: cx.focus_handle(),
                fs,
                pending_serialization: Task::ready(None),
                visible_entries: Vec::new(),
                all_staged: None,
                current_modifiers: cx.modifiers(),
                width: Some(px(360.)),
                scrollbar_state: ScrollbarState::new(scroll_handle.clone()).parent_view(cx.view()),
                scroll_handle,
                selected_entry: None,
                show_scrollbar: false,
                hide_scrollbar_task: None,
                rebuild_requested,
                commit_editor,
                project,
                reveal_in_editor: Task::ready(()),
                err_sender,
                workspace,
            };
            git_panel.schedule_update();
            git_panel.show_scrollbar = git_panel.should_show_scrollbar(cx);
            git_panel
        });

        let handle = git_panel.downgrade();
        cx.spawn(|_, mut cx| async move {
            while let Some(e) = err_receiver.next().await {
                let Some(this) = handle.upgrade() else {
                    break;
                };
                if this
                    .update(&mut cx, |this, cx| {
                        this.show_err_toast("git operation error", e, cx);
                    })
                    .is_err()
                {
                    break;
                }
            }
        })
        .detach();

        cx.subscribe(
            &git_panel,
            move |workspace, _, event: &Event, cx| match event.clone() {
                Event::OpenedEntry { path } => {
                    workspace
                        .open_path_preview(path, None, false, false, cx)
                        .detach_and_prompt_err("Failed to open file", cx, |e, _| {
                            Some(format!("{e}"))
                        });
                }
                Event::Focus => { /* TODO */ }
            },
        )
        .detach();

        git_panel
    }

    fn git_state(&self, cx: &AppContext) -> Option<Model<GitState>> {
        self.project.read(cx).git_state().cloned()
    }

    fn active_repository<'a>(
        &self,
        cx: &'a AppContext,
    ) -> Option<&'a (WorktreeId, RepositoryEntry, Arc<dyn GitRepository>)> {
        let git_state = self.git_state(cx)?;
        let active_repository = git_state.read(cx).active_repository.as_ref()?;
        Some(active_repository)
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

    fn dispatch_context(&self, cx: &ViewContext<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("GitPanel");

        if self.is_focused(cx) {
            dispatch_context.add("menu");
            dispatch_context.add("ChangesList");
        }

        if self.commit_editor.read(cx).is_focused(cx) {
            dispatch_context.add("CommitEditor");
        }

        dispatch_context
    }

    fn is_focused(&self, cx: &ViewContext<Self>) -> bool {
        cx.focused()
            .map_or(false, |focused| self.focus_handle == focused)
    }

    fn close_panel(&mut self, _: &Close, cx: &mut ViewContext<Self>) {
        cx.emit(PanelEvent::Close);
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        if !self.focus_handle.contains_focused(cx) {
            cx.emit(Event::Focus);
        }
    }

    fn show_scrollbar(&self, cx: &mut ViewContext<Self>) -> ShowScrollbar {
        GitPanelSettings::get_global(cx)
            .scrollbar
            .show
            .unwrap_or_else(|| EditorSettings::get_global(cx).scrollbar.show)
    }

    fn should_show_scrollbar(&self, cx: &mut ViewContext<Self>) -> bool {
        let show = self.show_scrollbar(cx);
        match show {
            ShowScrollbar::Auto => true,
            ShowScrollbar::System => true,
            ShowScrollbar::Always => true,
            ShowScrollbar::Never => false,
        }
    }

    fn should_autohide_scrollbar(&self, cx: &mut ViewContext<Self>) -> bool {
        let show = self.show_scrollbar(cx);
        match show {
            ShowScrollbar::Auto => true,
            ShowScrollbar::System => cx
                .try_global::<ScrollbarAutoHide>()
                .map_or_else(|| cx.should_auto_hide_scrollbars(), |autohide| autohide.0),
            ShowScrollbar::Always => false,
            ShowScrollbar::Never => true,
        }
    }

    fn hide_scrollbar(&mut self, cx: &mut ViewContext<Self>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);
        if !self.should_autohide_scrollbar(cx) {
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
        let ancestors = repo_path.ancestors().skip(1);
        for ancestor in ancestors {
            if let Some(parent_entry) = visible_entries.get(ancestor) {
                let entry_component_count = repo_path.components().count();
                let parent_component_count = parent_entry.components().count();

                let difference = entry_component_count - parent_component_count;

                let parent_depth = parent_entry
                    .ancestors()
                    .skip(1) // Skip the parent itself
                    .filter(|ancestor| visible_entries.contains(*ancestor))
                    .count();

                return (parent_depth + 1, difference);
            }
        }

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
                selected_entry
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

    fn focus_editor(&mut self, _: &FocusEditor, cx: &mut ViewContext<Self>) {
        self.commit_editor.update(cx, |editor, cx| {
            editor.focus(cx);
        });
        cx.notify();
    }

    fn select_first_entry_if_none(&mut self, cx: &mut ViewContext<Self>) {
        if !self.no_entries(cx) && self.selected_entry.is_none() {
            self.selected_entry = Some(0);
            self.scroll_to_selected_entry(cx);
            cx.notify();
        }
    }

    fn focus_changes_list(&mut self, _: &FocusChanges, cx: &mut ViewContext<Self>) {
        self.select_first_entry_if_none(cx);

        cx.focus_self();
        cx.notify();
    }

    fn get_selected_entry(&self) -> Option<&GitListEntry> {
        self.selected_entry
            .and_then(|i| self.visible_entries.get(i))
    }

    fn open_selected(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        if let Some(entry) = self
            .selected_entry
            .and_then(|i| self.visible_entries.get(i))
        {
            self.open_entry(entry, cx);
        }
    }

    fn toggle_staged_for_entry(&mut self, entry: &GitListEntry, cx: &mut ViewContext<Self>) {
        let Some(git_state) = self.git_state(cx) else {
            return;
        };
        let result = git_state.update(cx, |git_state, _| {
            if entry.status.is_staged().unwrap_or(false) {
                git_state.unstage_entries(vec![entry.repo_path.clone()], self.err_sender.clone())
            } else {
                git_state.stage_entries(vec![entry.repo_path.clone()], self.err_sender.clone())
            }
        });
        if let Err(e) = result {
            self.show_err_toast("toggle staged error", e, cx);
        }
        cx.notify();
    }

    fn toggle_staged_for_selected(&mut self, _: &git::ToggleStaged, cx: &mut ViewContext<Self>) {
        if let Some(selected_entry) = self.get_selected_entry().cloned() {
            self.toggle_staged_for_entry(&selected_entry, cx);
        }
    }

    fn open_entry(&self, entry: &GitListEntry, cx: &mut ViewContext<Self>) {
        let Some((worktree_id, path)) = maybe!({
            let git_state = self.git_state(cx)?;
            let (id, repo, _) = git_state.read(cx).active_repository.as_ref()?;
            let path = repo.work_directory.unrelativize(&entry.repo_path)?;
            Some((*id, path))
        }) else {
            return;
        };
        let path = (worktree_id, path).into();
        let path_exists = self.project.update(cx, |project, cx| {
            project.entry_for_path(&path, cx).is_some()
        });
        if !path_exists {
            return;
        }
        cx.emit(Event::OpenedEntry { path });
    }

    fn stage_all(&mut self, _: &git::StageAll, cx: &mut ViewContext<Self>) {
        let Some(git_state) = self.git_state(cx) else {
            return;
        };
        for entry in &mut self.visible_entries {
            entry.is_staged = Some(true);
        }
        self.all_staged = Some(true);

        if let Err(e) = git_state.read(cx).stage_all(self.err_sender.clone()) {
            self.show_err_toast("stage all error", e, cx);
        };
    }

    fn unstage_all(&mut self, _: &git::UnstageAll, cx: &mut ViewContext<Self>) {
        let Some(git_state) = self.git_state(cx) else {
            return;
        };
        for entry in &mut self.visible_entries {
            entry.is_staged = Some(false);
        }
        self.all_staged = Some(false);
        if let Err(e) = git_state.read(cx).unstage_all(self.err_sender.clone()) {
            self.show_err_toast("unstage all error", e, cx);
        };
    }

    fn discard_all(&mut self, _: &git::RevertAll, _cx: &mut ViewContext<Self>) {
        // TODO: Implement discard all
        println!("Discard all triggered");
    }

    /// Commit all staged changes
    fn commit_changes(&mut self, _: &git::CommitChanges, cx: &mut ViewContext<Self>) {
        let Some(git_state) = self.git_state(cx) else {
            return;
        };
        if let Err(e) =
            git_state.update(cx, |git_state, _| git_state.commit(self.err_sender.clone()))
        {
            self.show_err_toast("commit error", e, cx);
        };
        self.commit_editor
            .update(cx, |editor, cx| editor.set_text("", cx));
    }

    /// Commit all changes, regardless of whether they are staged or not
    fn commit_all_changes(&mut self, _: &git::CommitAllChanges, cx: &mut ViewContext<Self>) {
        let Some(git_state) = self.git_state(cx) else {
            return;
        };
        if let Err(e) = git_state.update(cx, |git_state, _| {
            git_state.commit_all(self.err_sender.clone())
        }) {
            self.show_err_toast("commit all error", e, cx);
        };
        self.commit_editor
            .update(cx, |editor, cx| editor.set_text("", cx));
    }

    fn fill_co_authors(&mut self, _: &FillCoAuthors, cx: &mut ViewContext<Self>) {
        const CO_AUTHOR_PREFIX: &str = "Co-authored-by: ";

        let Some(room) = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).active_call()?.read(cx).room().cloned())
        else {
            return;
        };

        let mut existing_text = self.commit_editor.read(cx).text(cx);
        existing_text.make_ascii_lowercase();
        let lowercase_co_author_prefix = CO_AUTHOR_PREFIX.to_lowercase();
        let mut ends_with_co_authors = false;
        let existing_co_authors = existing_text
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.starts_with(&lowercase_co_author_prefix) {
                    ends_with_co_authors = true;
                    Some(line)
                } else {
                    ends_with_co_authors = false;
                    None
                }
            })
            .collect::<HashSet<_>>();

        let new_co_authors = room
            .read(cx)
            .remote_participants()
            .values()
            .filter(|participant| participant.can_write())
            .map(|participant| participant.user.clone())
            .filter_map(|user| {
                let email = user.email.as_deref()?;
                let name = user.name.as_deref().unwrap_or(&user.github_login);
                Some(format!("{CO_AUTHOR_PREFIX}{name} <{email}>"))
            })
            .filter(|co_author| {
                !existing_co_authors.contains(co_author.to_ascii_lowercase().as_str())
            })
            .collect::<Vec<_>>();
        if new_co_authors.is_empty() {
            return;
        }

        self.commit_editor.update(cx, |editor, cx| {
            let editor_end = editor.buffer().read(cx).read(cx).len();
            let mut edit = String::new();
            if !ends_with_co_authors {
                edit.push('\n');
            }
            for co_author in new_co_authors {
                edit.push('\n');
                edit.push_str(&co_author);
            }

            editor.edit(Some((editor_end..editor_end, edit)), cx);
            editor.move_to_end(&MoveToEnd, cx);
            editor.focus(cx);
        });
    }

    fn no_entries(&self, cx: &mut ViewContext<Self>) -> bool {
        self.git_state(cx)
            .map_or(true, |git_state| git_state.read(cx).entry_count() == 0)
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
                is_staged: entry.is_staged,
            };

            callback(ix, details, cx);
        }
    }

    fn schedule_update(&mut self) {
        self.rebuild_requested.store(true, Ordering::Relaxed);
    }

    #[track_caller]
    fn update_visible_entries(&mut self, cx: &mut ViewContext<Self>) {
        self.visible_entries.clear();

        let Some((_, repo, _)) = self.active_repository(cx) else {
            // Just clear entries if no repository is active.
            cx.notify();
            return;
        };

        // First pass - collect all paths
        let path_set = HashSet::from_iter(repo.status().map(|entry| entry.repo_path));

        // Second pass - create entries with proper depth calculation
        let mut all_staged = None;
        for (ix, entry) in repo.status().enumerate() {
            let (depth, difference) =
                Self::calculate_depth_and_difference(&entry.repo_path, &path_set);
            let is_staged = entry.status.is_staged();
            all_staged = if ix == 0 {
                is_staged
            } else {
                match (all_staged, is_staged) {
                    (None, _) | (_, None) => None,
                    (Some(a), Some(b)) => (a == b).then_some(a),
                }
            };

            let display_name = if difference > 1 {
                // Show partial path for deeply nested files
                entry
                    .repo_path
                    .as_ref()
                    .iter()
                    .skip(entry.repo_path.components().count() - difference)
                    .collect::<PathBuf>()
                    .to_string_lossy()
                    .into_owned()
            } else {
                // Just show filename
                entry
                    .repo_path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_default()
            };

            let entry = GitListEntry {
                depth,
                display_name,
                repo_path: entry.repo_path,
                status: entry.status,
                is_staged,
            };

            self.visible_entries.push(entry);
        }
        self.all_staged = all_staged;

        // Sort entries by path to maintain consistent order
        self.visible_entries
            .sort_by(|a, b| a.repo_path.cmp(&b.repo_path));

        self.select_first_entry_if_none(cx);

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

            let Some(git_state) = self.git_state(cx) else {
                return;
            };
            git_state.update(cx, |git_state, _| {
                git_state.commit_message = commit_message.into();
            });

            cx.notify();
        }
    }

    fn show_err_toast(&self, id: &'static str, e: anyhow::Error, cx: &mut ViewContext<Self>) {
        let Some(workspace) = self.weak_workspace.upgrade() else {
            return;
        };
        let notif_id = NotificationId::Named(id.into());
        let message = e.to_string();
        workspace.update(cx, |workspace, cx| {
            let toast = Toast::new(notif_id, message).on_click("Open Zed Log", |cx| {
                cx.dispatch_action(workspace::OpenLog.boxed_clone());
            });
            workspace.show_toast(toast, cx);
        });
    }
}

// GitPanel –– Render
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
        let entry_count = self
            .git_state(cx)
            .map_or(0, |git_state| git_state.read(cx).entry_count());

        let changes_string = match entry_count {
            0 => "No changes".to_string(),
            1 => "1 change".to_string(),
            n => format!("{} changes", n),
        };

        // for our use case treat None as false
        let all_staged = self.all_staged.unwrap_or(false);

        h_flex()
            .h(px(32.))
            .items_center()
            .px_2()
            .bg(ElevationIndex::Surface.bg(cx))
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Checkbox::new(
                            "all-changes",
                            if self.no_entries(cx) {
                                ToggleState::Selected
                            } else {
                                self.all_staged
                                    .map_or(ToggleState::Indeterminate, ToggleState::from)
                            },
                        )
                        .fill()
                        .elevation(ElevationIndex::Surface)
                        .tooltip(move |cx| {
                            if all_staged {
                                Tooltip::text("Unstage all changes", cx)
                            } else {
                                Tooltip::text("Stage all changes", cx)
                            }
                        })
                        .on_click(cx.listener(move |git_panel, _, cx| match all_staged {
                            true => git_panel.unstage_all(&UnstageAll, cx),
                            false => git_panel.stage_all(&StageAll, cx),
                        })),
                    )
                    .child(div().text_buffer(cx).text_ui_sm(cx).child(changes_string)),
            )
            .child(div().flex_grow())
            .child(
                h_flex()
                    .gap_2()
                    // TODO: Re-add once revert all is added
                    // .child(
                    //     IconButton::new("discard-changes", IconName::Undo)
                    //         .tooltip({
                    //             let focus_handle = focus_handle.clone();
                    //             move |cx| {
                    //                 Tooltip::for_action_in(
                    //                     "Discard all changes",
                    //                     &RevertAll,
                    //                     &focus_handle,
                    //                     cx,
                    //                 )
                    //             }
                    //         })
                    //         .icon_size(IconSize::Small)
                    //         .disabled(true),
                    // )
                    .child(if self.all_staged.unwrap_or(false) {
                        self.panel_button("unstage-all", "Unstage All")
                            .tooltip({
                                let focus_handle = focus_handle.clone();
                                move |cx| {
                                    Tooltip::for_action_in(
                                        "Unstage all changes",
                                        &UnstageAll,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            })
                            .key_binding(ui::KeyBinding::for_action_in(
                                &UnstageAll,
                                &focus_handle,
                                cx,
                            ))
                            .on_click(
                                cx.listener(move |this, _, cx| this.unstage_all(&UnstageAll, cx)),
                            )
                    } else {
                        self.panel_button("stage-all", "Stage All")
                            .tooltip({
                                let focus_handle = focus_handle.clone();
                                move |cx| {
                                    Tooltip::for_action_in(
                                        "Stage all changes",
                                        &StageAll,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            })
                            .key_binding(ui::KeyBinding::for_action_in(
                                &StageAll,
                                &focus_handle,
                                cx,
                            ))
                            .on_click(cx.listener(move |this, _, cx| this.stage_all(&StageAll, cx)))
                    }),
            )
    }

    pub fn render_commit_editor(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let editor = self.commit_editor.clone();
        let editor_focus_handle = editor.read(cx).focus_handle(cx).clone();
        let (can_commit, can_commit_all) = self.git_state(cx).map_or((false, false), |git_state| {
            let git_state = git_state.read(cx);
            (git_state.can_commit(false), git_state.can_commit(true))
        });

        let focus_handle_1 = self.focus_handle(cx).clone();
        let focus_handle_2 = self.focus_handle(cx).clone();

        let commit_staged_button = self
            .panel_button("commit-staged-changes", "Commit")
            .tooltip(move |cx| {
                let focus_handle = focus_handle_1.clone();
                Tooltip::for_action_in(
                    "Commit all staged changes",
                    &CommitChanges,
                    &focus_handle,
                    cx,
                )
            })
            .disabled(!can_commit)
            .on_click(
                cx.listener(|this, _: &ClickEvent, cx| this.commit_changes(&CommitChanges, cx)),
            );

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
            .disabled(!can_commit_all)
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
        let scroll_bar_style = self.show_scrollbar(cx);
        let show_container = matches!(scroll_bar_style, ShowScrollbar::Always);

        if !self.should_show_scrollbar(cx)
            || !(self.show_scrollbar || self.scrollbar_state.is_dragging())
        {
            return None;
        }

        Some(
            div()
                .id("git-panel-vertical-scroll")
                .occlude()
                .flex_none()
                .h_full()
                .cursor_default()
                .when(show_container, |this| this.pl_1().px_1p5())
                .when(!show_container, |this| {
                    this.absolute().right_1().top_1().bottom_1().w(px(12.))
                })
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
                .children(Scrollbar::vertical(
                    // percentage as f32..end_offset as f32,
                    self.scrollbar_state.clone(),
                )),
        )
    }

    fn render_entries(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let entry_count = self.visible_entries.len();

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
        let repo_path = entry_details.repo_path.clone();
        let selected = self.selected_entry == Some(ix);
        let status_style = GitPanelSettings::get_global(cx).status_style;
        let status = entry_details.status;

        let mut label_color = cx.theme().colors().text;
        if status_style == StatusStyle::LabelColor {
            label_color = if status.is_conflicted() {
                cx.theme().status().conflict
            } else if status.is_modified() {
                cx.theme().status().modified
            } else if status.is_deleted() {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().status().created
            }
        }

        let path_color = status
            .is_deleted()
            .then_some(cx.theme().colors().text_disabled)
            .unwrap_or(cx.theme().colors().text_muted);

        let entry_id = ElementId::Name(format!("entry_{}", entry_details.display_name).into());
        let checkbox_id =
            ElementId::Name(format!("checkbox_{}", entry_details.display_name).into());
        let is_tree_view = false;
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

        if is_tree_view {
            entry = entry.pl(px(8. + 12. * entry_details.depth as f32))
        } else {
            entry = entry.pl(px(8.))
        }

        if selected {
            entry = entry.bg(cx.theme().status().info_background);
        }

        entry = entry
            .child(
                Checkbox::new(
                    checkbox_id,
                    entry_details
                        .is_staged
                        .map_or(ToggleState::Indeterminate, ToggleState::from),
                )
                .fill()
                .elevation(ElevationIndex::Surface)
                .on_click({
                    let handle = handle.clone();
                    let repo_path = repo_path.clone();
                    move |toggle, cx| {
                        let Some(this) = handle.upgrade() else {
                            return;
                        };
                        this.update(cx, |this, cx| {
                            this.visible_entries[ix].is_staged = match *toggle {
                                ToggleState::Selected => Some(true),
                                ToggleState::Unselected => Some(false),
                                ToggleState::Indeterminate => None,
                            };
                            let repo_path = repo_path.clone();
                            let Some(git_state) = this.git_state(cx) else {
                                return;
                            };
                            let result = git_state.update(cx, |git_state, _| match toggle {
                                ToggleState::Selected | ToggleState::Indeterminate => git_state
                                    .stage_entries(vec![repo_path], this.err_sender.clone()),
                                ToggleState::Unselected => git_state
                                    .unstage_entries(vec![repo_path], this.err_sender.clone()),
                            });
                            if let Err(e) = result {
                                this.show_err_toast("toggle staged error", e, cx);
                            }
                        });
                    }
                }),
            )
            .when(status_style == StatusStyle::Icon, |this| {
                this.child(git_status_icon(status))
            })
            .child(
                h_flex()
                    .text_color(label_color)
                    .when(status.is_deleted(), |this| this.line_through())
                    .when_some(repo_path.parent(), |this, parent| {
                        let parent_str = parent.to_string_lossy();
                        if !parent_str.is_empty() {
                            this.child(
                                div()
                                    .text_color(path_color)
                                    .child(format!("{}/", parent_str)),
                            )
                        } else {
                            this
                        }
                    })
                    .child(div().child(entry_details.display_name.clone())),
            )
            .child(div().flex_1())
            .child(end_slot)
            .on_click(move |_, cx| {
                // TODO: add `select_entry` method then do after that
                cx.dispatch_action(Box::new(OpenSelected));

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
        let has_co_authors = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).active_call()?.read(cx).room().cloned())
            .map(|room| {
                let room = room.read(cx);
                room.local_participant().can_write()
                    && room
                        .remote_participants()
                        .values()
                        .any(|remote_participant| remote_participant.can_write())
            })
            .unwrap_or(false);

        v_flex()
            .id("git_panel")
            .key_context(self.dispatch_context(cx))
            .track_focus(&self.focus_handle)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .when(!project.is_read_only(cx), |this| {
                this.on_action(cx.listener(|this, &ToggleStaged, cx| {
                    this.toggle_staged_for_selected(&ToggleStaged, cx)
                }))
                .on_action(cx.listener(|this, &StageAll, cx| this.stage_all(&StageAll, cx)))
                .on_action(cx.listener(|this, &UnstageAll, cx| this.unstage_all(&UnstageAll, cx)))
                .on_action(cx.listener(|this, &RevertAll, cx| this.discard_all(&RevertAll, cx)))
                .on_action(
                    cx.listener(|this, &CommitChanges, cx| this.commit_changes(&CommitChanges, cx)),
                )
                .on_action(cx.listener(|this, &CommitAllChanges, cx| {
                    this.commit_all_changes(&CommitAllChanges, cx)
                }))
            })
            .when(self.is_focused(cx), |this| {
                this.on_action(cx.listener(Self::select_first))
                    .on_action(cx.listener(Self::select_next))
                    .on_action(cx.listener(Self::select_prev))
                    .on_action(cx.listener(Self::select_last))
                    .on_action(cx.listener(Self::close_panel))
            })
            .on_action(cx.listener(Self::open_selected))
            .on_action(cx.listener(Self::focus_changes_list))
            .on_action(cx.listener(Self::focus_editor))
            .on_action(cx.listener(Self::toggle_staged_for_selected))
            .when(has_co_authors, |git_panel| {
                git_panel.on_action(cx.listener(Self::fill_co_authors))
            })
            // .on_action(cx.listener(|this, &OpenSelected, cx| this.open_selected(&OpenSelected, cx)))
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
            .child(if !self.no_entries(cx) {
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
