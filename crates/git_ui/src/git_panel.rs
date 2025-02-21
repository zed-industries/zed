use crate::git_panel_settings::StatusStyle;
use crate::project_diff::Diff;
use crate::repository_selector::RepositorySelectorPopoverMenu;
use crate::{
    git_panel_settings::GitPanelSettings, git_status_icon, repository_selector::RepositorySelector,
};
use crate::{project_diff, ProjectDiff};
use collections::HashMap;
use db::kvp::KEY_VALUE_STORE;
use editor::commit_tooltip::CommitTooltip;
use editor::{
    scroll::ScrollbarAutoHide, Editor, EditorElement, EditorMode, EditorSettings, MultiBuffer,
    ShowScrollbar,
};
use git::repository::{CommitDetails, ResetMode};
use git::{repository::RepoPath, status::FileStatus, Commit, ToggleStaged};
use git::{RestoreTrackedFiles, StageAll, TrashUntrackedFiles, UnstageAll};
use gpui::*;
use itertools::Itertools;
use language::{Buffer, File};
use menu::{Confirm, SecondaryConfirm, SelectFirst, SelectLast, SelectNext, SelectPrev};
use multi_buffer::ExcerptInfo;
use panel::{panel_editor_container, panel_editor_style, panel_filled_button, PanelHeader};
use project::{
    git::{GitEvent, Repository},
    Fs, Project, ProjectPath,
};
use serde::{Deserialize, Serialize};
use settings::Settings as _;
use std::{collections::HashSet, path::PathBuf, sync::Arc, time::Duration, usize};
use strum::{IntoEnumIterator, VariantNames};
use time::OffsetDateTime;
use ui::{
    prelude::*, ButtonLike, Checkbox, ContextMenu, Divider, DividerColor, ElevationIndex, ListItem,
    ListItemSpacing, Scrollbar, ScrollbarState, Tooltip,
};
use util::{maybe, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    notifications::{DetachAndPromptErr, NotificationId},
    Toast, Workspace,
};

actions!(
    git_panel,
    [
        Close,
        ToggleFocus,
        OpenMenu,
        FocusEditor,
        FocusChanges,
        ToggleFillCoAuthors,
    ]
);

fn prompt<T>(msg: &str, detail: Option<&str>, window: &mut Window, cx: &mut App) -> Task<Result<T>>
where
    T: IntoEnumIterator + VariantNames + 'static,
{
    let rx = window.prompt(PromptLevel::Info, msg, detail, &T::VARIANTS, cx);
    cx.spawn(|_| async move { Ok(T::iter().nth(rx.await?).unwrap()) })
}

#[derive(strum::EnumIter, strum::VariantNames)]
#[strum(serialize_all = "title_case")]
enum TrashCancel {
    Trash,
    Cancel,
}

const GIT_PANEL_KEY: &str = "GitPanel";

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
                workspace.toggle_panel_focus::<GitPanel>(window, cx);
            });

            // workspace.register_action(|workspace, _: &Commit, window, cx| {
            //     workspace.open_panel::<GitPanel>(window, cx);
            //     if let Some(git_panel) = workspace.panel::<GitPanel>(cx) {
            //         git_panel
            //             .read(cx)
            //             .commit_editor
            //             .focus_handle(cx)
            //             .focus(window);
            //     }
            // });
        },
    )
    .detach();
}

#[derive(Debug, Clone)]
pub enum Event {
    Focus,
}

#[derive(Serialize, Deserialize)]
struct SerializedGitPanel {
    width: Option<Pixels>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Section {
    Conflict,
    Tracked,
    New,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GitHeaderEntry {
    header: Section,
}

impl GitHeaderEntry {
    pub fn contains(&self, status_entry: &GitStatusEntry, repo: &Repository) -> bool {
        let this = &self.header;
        let status = status_entry.status;
        match this {
            Section::Conflict => repo.has_conflict(&status_entry.repo_path),
            Section::Tracked => !status.is_created(),
            Section::New => status.is_created(),
        }
    }
    pub fn title(&self) -> &'static str {
        match self.header {
            Section::Conflict => "Conflicts",
            Section::Tracked => "Tracked",
            Section::New => "Untracked",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum GitListEntry {
    GitStatusEntry(GitStatusEntry),
    Header(GitHeaderEntry),
}

impl GitListEntry {
    fn status_entry(&self) -> Option<&GitStatusEntry> {
        match self {
            GitListEntry::GitStatusEntry(entry) => Some(entry),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GitStatusEntry {
    pub(crate) depth: usize,
    pub(crate) display_name: String,
    pub(crate) repo_path: RepoPath,
    pub(crate) status: FileStatus,
    pub(crate) is_staged: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TargetStatus {
    Staged,
    Unstaged,
    Reverted,
    Unchanged,
}

struct PendingOperation {
    finished: bool,
    target_status: TargetStatus,
    repo_paths: HashSet<RepoPath>,
    op_id: usize,
}

pub struct GitPanel {
    pub(crate) active_repository: Option<Entity<Repository>>,
    commit_editor: Entity<Editor>,
    conflicted_count: usize,
    conflicted_staged_count: usize,
    current_modifiers: Modifiers,
    add_coauthors: bool,
    entries: Vec<GitListEntry>,
    entries_by_path: collections::HashMap<RepoPath, usize>,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    hide_scrollbar_task: Option<Task<()>>,
    new_count: usize,
    new_staged_count: usize,
    pending: Vec<PendingOperation>,
    pending_commit: Option<Task<()>>,
    pending_serialization: Task<Option<()>>,
    pub(crate) project: Entity<Project>,
    repository_selector: Entity<RepositorySelector>,
    scroll_handle: UniformListScrollHandle,
    scrollbar_state: ScrollbarState,
    selected_entry: Option<usize>,
    show_scrollbar: bool,
    tracked_count: usize,
    tracked_staged_count: usize,
    update_visible_entries_task: Task<()>,
    width: Option<Pixels>,
    workspace: WeakEntity<Workspace>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    modal_open: bool,
}

pub(crate) fn commit_message_editor(
    commit_message_buffer: Entity<Buffer>,
    project: Entity<Project>,
    in_panel: bool,
    window: &mut Window,
    cx: &mut Context<'_, Editor>,
) -> Editor {
    let buffer = cx.new(|cx| MultiBuffer::singleton(commit_message_buffer, cx));
    let max_lines = if in_panel { 6 } else { 18 };
    let mut commit_editor = Editor::new(
        EditorMode::AutoHeight { max_lines },
        buffer,
        None,
        false,
        window,
        cx,
    );
    commit_editor.set_collaboration_hub(Box::new(project));
    commit_editor.set_use_autoclose(false);
    commit_editor.set_show_gutter(false, cx);
    commit_editor.set_show_wrap_guides(false, cx);
    commit_editor.set_show_indent_guides(false, cx);
    commit_editor.set_placeholder_text("Enter commit message", cx);
    commit_editor
}

impl GitPanel {
    pub fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let fs = workspace.app_state().fs.clone();
        let project = workspace.project().clone();
        let git_store = project.read(cx).git_store().clone();
        let active_repository = project.read(cx).active_repository(cx);
        let workspace = cx.entity().downgrade();

        cx.new(|cx| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, window, Self::focus_in).detach();
            cx.on_focus_out(&focus_handle, window, |this, _, window, cx| {
                this.hide_scrollbar(window, cx);
            })
            .detach();

            // just to let us render a placeholder editor.
            // Once the active git repo is set, this buffer will be replaced.
            let temporary_buffer = cx.new(|cx| Buffer::local("", cx));
            let commit_editor = cx.new(|cx| {
                commit_message_editor(temporary_buffer, project.clone(), true, window, cx)
            });
            commit_editor.update(cx, |editor, cx| {
                editor.clear(window, cx);
            });

            let scroll_handle = UniformListScrollHandle::new();

            cx.subscribe_in(
                &git_store,
                window,
                move |this, git_store, event, window, cx| match event {
                    GitEvent::FileSystemUpdated => {
                        this.schedule_update(false, window, cx);
                    }
                    GitEvent::ActiveRepositoryChanged | GitEvent::GitStateUpdated => {
                        this.active_repository = git_store.read(cx).active_repository();
                        this.schedule_update(true, window, cx);
                    }
                },
            )
            .detach();

            let scrollbar_state =
                ScrollbarState::new(scroll_handle.clone()).parent_entity(&cx.entity());

            let repository_selector =
                cx.new(|cx| RepositorySelector::new(project.clone(), window, cx));

            let mut git_panel = Self {
                active_repository,
                commit_editor,
                conflicted_count: 0,
                conflicted_staged_count: 0,
                current_modifiers: window.modifiers(),
                add_coauthors: true,
                entries: Vec::new(),
                entries_by_path: HashMap::default(),
                focus_handle: cx.focus_handle(),
                fs,
                hide_scrollbar_task: None,
                new_count: 0,
                new_staged_count: 0,
                pending: Vec::new(),
                pending_commit: None,
                pending_serialization: Task::ready(None),
                project,
                repository_selector,
                scroll_handle,
                scrollbar_state,
                selected_entry: None,
                show_scrollbar: false,
                tracked_count: 0,
                tracked_staged_count: 0,
                update_visible_entries_task: Task::ready(()),
                width: Some(px(360.)),
                context_menu: None,
                workspace,
                modal_open: false,
            };
            git_panel.schedule_update(false, window, cx);
            git_panel.show_scrollbar = git_panel.should_show_scrollbar(cx);
            git_panel
        })
    }

    pub fn select_entry_by_path(
        &mut self,
        path: ProjectPath,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(git_repo) = self.active_repository.as_ref() else {
            return;
        };
        let Some(repo_path) = git_repo.read(cx).project_path_to_repo_path(&path) else {
            return;
        };
        let Some(ix) = self.entries_by_path.get(&repo_path) else {
            return;
        };
        self.selected_entry = Some(*ix);
        cx.notify();
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background_spawn(
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

    pub(crate) fn set_modal_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.modal_open = open;
        cx.notify();
    }

    fn dispatch_context(&self, window: &mut Window, cx: &Context<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("GitPanel");

        if self.is_focused(window, cx) {
            dispatch_context.add("menu");
            dispatch_context.add("ChangesList");
        }

        if self.commit_editor.read(cx).is_focused(window) {
            dispatch_context.add("CommitEditor");
        }

        dispatch_context
    }

    fn is_focused(&self, window: &Window, cx: &Context<Self>) -> bool {
        window
            .focused(cx)
            .map_or(false, |focused| self.focus_handle == focused)
    }

    fn close_panel(&mut self, _: &Close, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(PanelEvent::Close);
    }

    fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_handle.contains_focused(window, cx) {
            cx.emit(Event::Focus);
        }
    }

    fn show_scrollbar(&self, cx: &mut Context<Self>) -> ShowScrollbar {
        GitPanelSettings::get_global(cx)
            .scrollbar
            .show
            .unwrap_or_else(|| EditorSettings::get_global(cx).scrollbar.show)
    }

    fn should_show_scrollbar(&self, cx: &mut Context<Self>) -> bool {
        let show = self.show_scrollbar(cx);
        match show {
            ShowScrollbar::Auto => true,
            ShowScrollbar::System => true,
            ShowScrollbar::Always => true,
            ShowScrollbar::Never => false,
        }
    }

    fn should_autohide_scrollbar(&self, cx: &mut Context<Self>) -> bool {
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

    fn hide_scrollbar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);
        if !self.should_autohide_scrollbar(cx) {
            return;
        }
        self.hide_scrollbar_task = Some(cx.spawn_in(window, |panel, mut cx| async move {
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
        _: &mut Window,
        cx: &mut Context<Self>,
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

    fn scroll_to_selected_entry(&mut self, cx: &mut Context<Self>) {
        if let Some(selected_entry) = self.selected_entry {
            self.scroll_handle
                .scroll_to_item(selected_entry, ScrollStrategy::Center);
        }

        cx.notify();
    }

    fn select_first(&mut self, _: &SelectFirst, _window: &mut Window, cx: &mut Context<Self>) {
        if self.entries.first().is_some() {
            self.selected_entry = Some(1);
            self.scroll_to_selected_entry(cx);
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, _window: &mut Window, cx: &mut Context<Self>) {
        let item_count = self.entries.len();
        if item_count == 0 {
            return;
        }

        if let Some(selected_entry) = self.selected_entry {
            let new_selected_entry = if selected_entry > 0 {
                selected_entry - 1
            } else {
                selected_entry
            };

            if matches!(
                self.entries.get(new_selected_entry),
                Some(GitListEntry::Header(..))
            ) {
                if new_selected_entry > 0 {
                    self.selected_entry = Some(new_selected_entry - 1)
                }
            } else {
                self.selected_entry = Some(new_selected_entry);
            }

            self.scroll_to_selected_entry(cx);
        }

        cx.notify();
    }

    fn select_next(&mut self, _: &SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        let item_count = self.entries.len();
        if item_count == 0 {
            return;
        }

        if let Some(selected_entry) = self.selected_entry {
            let new_selected_entry = if selected_entry < item_count - 1 {
                selected_entry + 1
            } else {
                selected_entry
            };
            if matches!(
                self.entries.get(new_selected_entry),
                Some(GitListEntry::Header(..))
            ) {
                self.selected_entry = Some(new_selected_entry + 1);
            } else {
                self.selected_entry = Some(new_selected_entry);
            }

            self.scroll_to_selected_entry(cx);
        }

        cx.notify();
    }

    fn select_last(&mut self, _: &SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        if self.entries.last().is_some() {
            self.selected_entry = Some(self.entries.len() - 1);
            self.scroll_to_selected_entry(cx);
        }
    }

    fn focus_editor(&mut self, _: &FocusEditor, window: &mut Window, cx: &mut Context<Self>) {
        self.commit_editor.update(cx, |editor, cx| {
            window.focus(&editor.focus_handle(cx));
        });
        cx.notify();
    }

    fn select_first_entry_if_none(&mut self, cx: &mut Context<Self>) {
        let have_entries = self
            .active_repository
            .as_ref()
            .map_or(false, |active_repository| {
                active_repository.read(cx).entry_count() > 0
            });
        if have_entries && self.selected_entry.is_none() {
            self.selected_entry = Some(1);
            self.scroll_to_selected_entry(cx);
            cx.notify();
        }
    }

    fn focus_changes_list(
        &mut self,
        _: &FocusChanges,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_first_entry_if_none(cx);

        cx.focus_self(window);
        cx.notify();
    }

    fn get_selected_entry(&self) -> Option<&GitListEntry> {
        self.selected_entry.and_then(|i| self.entries.get(i))
    }

    fn open_diff(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        maybe!({
            let entry = self.entries.get(self.selected_entry?)?.status_entry()?;

            self.workspace
                .update(cx, |workspace, cx| {
                    ProjectDiff::deploy_at(workspace, Some(entry.clone()), window, cx);
                })
                .ok()
        });
    }

    fn open_file(
        &mut self,
        _: &menu::SecondaryConfirm,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let entry = self.entries.get(self.selected_entry?)?.status_entry()?;
            let active_repo = self.active_repository.as_ref()?;
            let path = active_repo
                .read(cx)
                .repo_path_to_project_path(&entry.repo_path)?;
            if entry.status.is_deleted() {
                return None;
            }

            self.workspace
                .update(cx, |workspace, cx| {
                    workspace
                        .open_path_preview(path, None, false, false, window, cx)
                        .detach_and_prompt_err("Failed to open file", window, cx, |e, _, _| {
                            Some(format!("{e}"))
                        });
                })
                .ok()
        });
    }

    fn revert_selected(
        &mut self,
        _: &git::RestoreFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let list_entry = self.entries.get(self.selected_entry?)?.clone();
            let entry = list_entry.status_entry()?;
            self.revert_entry(&entry, window, cx);
            Some(())
        });
    }

    fn revert_entry(
        &mut self,
        entry: &GitStatusEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let active_repo = self.active_repository.clone()?;
            let path = active_repo
                .read(cx)
                .repo_path_to_project_path(&entry.repo_path)?;
            let workspace = self.workspace.clone();

            if entry.status.is_staged() != Some(false) {
                self.perform_stage(false, vec![entry.repo_path.clone()], cx);
            }
            let filename = path.path.file_name()?.to_string_lossy();

            if !entry.status.is_created() {
                self.perform_checkout(vec![entry.repo_path.clone()], cx);
            } else {
                let prompt = prompt(&format!("Trash {}?", filename), None, window, cx);
                cx.spawn_in(window, |_, mut cx| async move {
                    match prompt.await? {
                        TrashCancel::Trash => {}
                        TrashCancel::Cancel => return Ok(()),
                    }
                    let task = workspace.update(&mut cx, |workspace, cx| {
                        workspace
                            .project()
                            .update(cx, |project, cx| project.delete_file(path, true, cx))
                    })?;
                    if let Some(task) = task {
                        task.await?;
                    }
                    Ok(())
                })
                .detach_and_prompt_err(
                    "Failed to trash file",
                    window,
                    cx,
                    |e, _, _| Some(format!("{e}")),
                );
            }
            Some(())
        });
    }

    fn perform_checkout(&mut self, repo_paths: Vec<RepoPath>, cx: &mut Context<Self>) {
        let workspace = self.workspace.clone();
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };

        let op_id = self.pending.iter().map(|p| p.op_id).max().unwrap_or(0) + 1;
        self.pending.push(PendingOperation {
            op_id,
            target_status: TargetStatus::Reverted,
            repo_paths: repo_paths.iter().cloned().collect(),
            finished: false,
        });
        self.update_visible_entries(cx);
        let task = cx.spawn(|_, mut cx| async move {
            let tasks: Vec<_> = workspace.update(&mut cx, |workspace, cx| {
                workspace.project().update(cx, |project, cx| {
                    repo_paths
                        .iter()
                        .filter_map(|repo_path| {
                            let path = active_repository
                                .read(cx)
                                .repo_path_to_project_path(&repo_path)?;
                            Some(project.open_buffer(path, cx))
                        })
                        .collect()
                })
            })?;

            let buffers = futures::future::join_all(tasks).await;

            active_repository
                .update(&mut cx, |repo, _| repo.checkout_files("HEAD", repo_paths))?
                .await??;

            let tasks: Vec<_> = cx.update(|cx| {
                buffers
                    .iter()
                    .filter_map(|buffer| {
                        buffer.as_ref().ok()?.update(cx, |buffer, cx| {
                            buffer.is_dirty().then(|| buffer.reload(cx))
                        })
                    })
                    .collect()
            })?;

            futures::future::join_all(tasks).await;

            Ok(())
        });

        cx.spawn(|this, mut cx| async move {
            let result = task.await;

            this.update(&mut cx, |this, cx| {
                for pending in this.pending.iter_mut() {
                    if pending.op_id == op_id {
                        pending.finished = true;
                        if result.is_err() {
                            pending.target_status = TargetStatus::Unchanged;
                            this.update_visible_entries(cx);
                        }
                        break;
                    }
                }
                result
                    .map_err(|e| {
                        this.show_err_toast(e, cx);
                    })
                    .ok();
            })
            .ok();
        })
        .detach();
    }

    fn discard_tracked_changes(
        &mut self,
        _: &RestoreTrackedFiles,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let entries = self
            .entries
            .iter()
            .filter_map(|entry| entry.status_entry().cloned())
            .filter(|status_entry| !status_entry.status.is_created())
            .collect::<Vec<_>>();

        match entries.len() {
            0 => return,
            1 => return self.revert_entry(&entries[0], window, cx),
            _ => {}
        }
        let details = entries
            .iter()
            .filter_map(|entry| entry.repo_path.0.file_name())
            .map(|filename| filename.to_string_lossy())
            .join("\n");

        #[derive(strum::EnumIter, strum::VariantNames)]
        #[strum(serialize_all = "title_case")]
        enum DiscardCancel {
            DiscardTrackedChanges,
            Cancel,
        }
        let prompt = prompt(
            "Discard changes to these files?",
            Some(&details),
            window,
            cx,
        );
        cx.spawn(|this, mut cx| async move {
            match prompt.await {
                Ok(DiscardCancel::DiscardTrackedChanges) => {
                    this.update(&mut cx, |this, cx| {
                        let repo_paths = entries.into_iter().map(|entry| entry.repo_path).collect();
                        this.perform_checkout(repo_paths, cx);
                    })
                    .ok();
                }
                _ => {
                    return;
                }
            }
        })
        .detach();
    }

    fn clean_all(&mut self, _: &TrashUntrackedFiles, window: &mut Window, cx: &mut Context<Self>) {
        let workspace = self.workspace.clone();
        let Some(active_repo) = self.active_repository.clone() else {
            return;
        };
        let to_delete = self
            .entries
            .iter()
            .filter_map(|entry| entry.status_entry())
            .filter(|status_entry| status_entry.status.is_created())
            .cloned()
            .collect::<Vec<_>>();

        match to_delete.len() {
            0 => return,
            1 => return self.revert_entry(&to_delete[0], window, cx),
            _ => {}
        };

        let details = to_delete
            .iter()
            .map(|entry| {
                entry
                    .repo_path
                    .0
                    .file_name()
                    .map(|f| f.to_string_lossy())
                    .unwrap_or_default()
            })
            .join("\n");

        let prompt = prompt("Trash these files?", Some(&details), window, cx);
        cx.spawn_in(window, |this, mut cx| async move {
            match prompt.await? {
                TrashCancel::Trash => {}
                TrashCancel::Cancel => return Ok(()),
            }
            let tasks = workspace.update(&mut cx, |workspace, cx| {
                to_delete
                    .iter()
                    .filter_map(|entry| {
                        workspace.project().update(cx, |project, cx| {
                            let project_path = active_repo
                                .read(cx)
                                .repo_path_to_project_path(&entry.repo_path)?;
                            project.delete_file(project_path, true, cx)
                        })
                    })
                    .collect::<Vec<_>>()
            })?;
            let to_unstage = to_delete
                .into_iter()
                .filter_map(|entry| {
                    if entry.status.is_staged() != Some(false) {
                        Some(entry.repo_path.clone())
                    } else {
                        None
                    }
                })
                .collect();
            this.update(&mut cx, |this, cx| {
                this.perform_stage(false, to_unstage, cx)
            })?;
            for task in tasks {
                task.await?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to trash files", window, cx, |e, _, _| {
            Some(format!("{e}"))
        });
    }

    fn stage_all(&mut self, _: &StageAll, _window: &mut Window, cx: &mut Context<Self>) {
        let repo_paths = self
            .entries
            .iter()
            .filter_map(|entry| entry.status_entry())
            .filter(|status_entry| status_entry.is_staged != Some(true))
            .map(|status_entry| status_entry.repo_path.clone())
            .collect::<Vec<_>>();
        self.perform_stage(true, repo_paths, cx);
    }

    fn unstage_all(&mut self, _: &UnstageAll, _window: &mut Window, cx: &mut Context<Self>) {
        let repo_paths = self
            .entries
            .iter()
            .filter_map(|entry| entry.status_entry())
            .filter(|status_entry| status_entry.is_staged != Some(false))
            .map(|status_entry| status_entry.repo_path.clone())
            .collect::<Vec<_>>();
        self.perform_stage(false, repo_paths, cx);
    }

    fn toggle_staged_for_entry(
        &mut self,
        entry: &GitListEntry,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_repository) = self.active_repository.as_ref() else {
            return;
        };
        let (stage, repo_paths) = match entry {
            GitListEntry::GitStatusEntry(status_entry) => {
                if status_entry.status.is_staged().unwrap_or(false) {
                    (false, vec![status_entry.repo_path.clone()])
                } else {
                    (true, vec![status_entry.repo_path.clone()])
                }
            }
            GitListEntry::Header(section) => {
                let goal_staged_state = !self.header_state(section.header).selected();
                let repository = active_repository.read(cx);
                let entries = self
                    .entries
                    .iter()
                    .filter_map(|entry| entry.status_entry())
                    .filter(|status_entry| {
                        section.contains(&status_entry, repository)
                            && status_entry.is_staged != Some(goal_staged_state)
                    })
                    .map(|status_entry| status_entry.repo_path.clone())
                    .collect::<Vec<_>>();

                (goal_staged_state, entries)
            }
        };
        self.perform_stage(stage, repo_paths, cx);
    }

    fn perform_stage(&mut self, stage: bool, repo_paths: Vec<RepoPath>, cx: &mut Context<Self>) {
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };
        let op_id = self.pending.iter().map(|p| p.op_id).max().unwrap_or(0) + 1;
        self.pending.push(PendingOperation {
            op_id,
            target_status: if stage {
                TargetStatus::Staged
            } else {
                TargetStatus::Unstaged
            },
            repo_paths: repo_paths.iter().cloned().collect(),
            finished: false,
        });
        let repo_paths = repo_paths.clone();
        let repository = active_repository.read(cx);
        self.update_counts(repository);
        cx.notify();

        cx.spawn({
            |this, mut cx| async move {
                let result = cx
                    .update(|cx| {
                        if stage {
                            active_repository
                                .update(cx, |repo, cx| repo.stage_entries(repo_paths.clone(), cx))
                        } else {
                            active_repository
                                .update(cx, |repo, cx| repo.unstage_entries(repo_paths.clone(), cx))
                        }
                    })?
                    .await;

                this.update(&mut cx, |this, cx| {
                    for pending in this.pending.iter_mut() {
                        if pending.op_id == op_id {
                            pending.finished = true
                        }
                    }
                    result
                        .map_err(|e| {
                            this.show_err_toast(e, cx);
                        })
                        .ok();
                    cx.notify();
                })
            }
        })
        .detach();
    }

    pub fn commit_message_buffer(&self, cx: &App) -> Entity<Buffer> {
        self.commit_editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .unwrap()
            .clone()
    }

    fn toggle_staged_for_selected(
        &mut self,
        _: &git::ToggleStaged,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selected_entry) = self.get_selected_entry().cloned() {
            self.toggle_staged_for_entry(&selected_entry, window, cx);
        }
    }

    /// Commit all staged changes
    fn commit(&mut self, _: &git::Commit, window: &mut Window, cx: &mut Context<Self>) {
        let editor = self.commit_editor.read(cx);
        if editor.is_empty(cx) {
            if !editor.focus_handle(cx).contains_focused(window, cx) {
                editor.focus_handle(cx).focus(window);
                return;
            }
        }

        self.commit_changes(window, cx)
    }

    pub(crate) fn commit_changes(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };
        let error_spawn = |message, window: &mut Window, cx: &mut App| {
            let prompt = window.prompt(PromptLevel::Warning, message, None, &["Ok"], cx);
            cx.spawn(|_| async move {
                prompt.await.ok();
            })
            .detach();
        };

        if self.has_unstaged_conflicts() {
            error_spawn(
                "There are still conflicts. You must stage these before committing",
                window,
                cx,
            );
            return;
        }

        let mut message = self.commit_editor.read(cx).text(cx);
        if message.trim().is_empty() {
            self.commit_editor.read(cx).focus_handle(cx).focus(window);
            return;
        }
        if self.add_coauthors {
            self.fill_co_authors(&mut message, cx);
        }

        let task = if self.has_staged_changes() {
            // Repository serializes all git operations, so we can just send a commit immediately
            let commit_task = active_repository.read(cx).commit(message.into(), None);
            cx.background_spawn(async move { commit_task.await? })
        } else {
            let changed_files = self
                .entries
                .iter()
                .filter_map(|entry| entry.status_entry())
                .filter(|status_entry| !status_entry.status.is_created())
                .map(|status_entry| status_entry.repo_path.clone())
                .collect::<Vec<_>>();

            if changed_files.is_empty() {
                error_spawn("No changes to commit", window, cx);
                return;
            }

            let stage_task =
                active_repository.update(cx, |repo, cx| repo.stage_entries(changed_files, cx));
            cx.spawn(|_, mut cx| async move {
                stage_task.await?;
                let commit_task = active_repository
                    .update(&mut cx, |repo, _| repo.commit(message.into(), None))?;
                commit_task.await?
            })
        };
        let task = cx.spawn_in(window, |this, mut cx| async move {
            let result = task.await;
            this.update_in(&mut cx, |this, window, cx| {
                this.pending_commit.take();
                match result {
                    Ok(()) => {
                        this.commit_editor
                            .update(cx, |editor, cx| editor.clear(window, cx));
                    }
                    Err(e) => this.show_err_toast(e, cx),
                }
            })
            .ok();
        });

        self.pending_commit = Some(task);
    }

    fn uncommit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repo) = self.active_repository.clone() else {
            return;
        };
        let prior_head = self.load_commit_details("HEAD", cx);

        let task = cx.spawn(|_, mut cx| async move {
            let prior_head = prior_head.await?;

            repo.update(&mut cx, |repo, _| repo.reset("HEAD^", ResetMode::Soft))?
                .await??;

            Ok(prior_head)
        });

        let task = cx.spawn_in(window, |this, mut cx| async move {
            let result = task.await;
            this.update_in(&mut cx, |this, window, cx| {
                this.pending_commit.take();
                match result {
                    Ok(prior_commit) => {
                        this.commit_editor.update(cx, |editor, cx| {
                            editor.set_text(prior_commit.message, window, cx)
                        });
                    }
                    Err(e) => this.show_err_toast(e, cx),
                }
            })
            .ok();
        });

        self.pending_commit = Some(task);
    }

    fn potential_co_authors(&self, cx: &App) -> Vec<(String, String)> {
        let mut new_co_authors = Vec::new();
        let project = self.project.read(cx);

        let Some(room) = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).active_call()?.read(cx).room().cloned())
        else {
            return Vec::default();
        };

        let room = room.read(cx);

        for (peer_id, collaborator) in project.collaborators() {
            if collaborator.is_host {
                continue;
            }

            let Some(participant) = room.remote_participant_for_peer_id(*peer_id) else {
                continue;
            };
            if participant.can_write() && participant.user.email.is_some() {
                let email = participant.user.email.clone().unwrap();

                new_co_authors.push((
                    participant
                        .user
                        .name
                        .clone()
                        .unwrap_or_else(|| participant.user.github_login.clone()),
                    email,
                ))
            }
        }
        if !project.is_local() && !project.is_read_only(cx) {
            if let Some(user) = room.local_participant_user(cx) {
                if let Some(email) = user.email.clone() {
                    new_co_authors.push((
                        user.name
                            .clone()
                            .unwrap_or_else(|| user.github_login.clone()),
                        email.clone(),
                    ))
                }
            }
        }
        new_co_authors
    }

    fn toggle_fill_co_authors(
        &mut self,
        _: &ToggleFillCoAuthors,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.add_coauthors = !self.add_coauthors;
        cx.notify();
    }

    fn fill_co_authors(&mut self, message: &mut String, cx: &mut Context<Self>) {
        const CO_AUTHOR_PREFIX: &str = "Co-authored-by: ";

        let existing_text = message.to_ascii_lowercase();
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

        let new_co_authors = self
            .potential_co_authors(cx)
            .into_iter()
            .filter(|(_, email)| {
                !existing_co_authors
                    .iter()
                    .any(|existing| existing.contains(email.as_str()))
            })
            .collect::<Vec<_>>();

        if new_co_authors.is_empty() {
            return;
        }

        if !ends_with_co_authors {
            message.push('\n');
        }
        for (name, email) in new_co_authors {
            message.push('\n');
            message.push_str(CO_AUTHOR_PREFIX);
            message.push_str(&name);
            message.push_str(" <");
            message.push_str(&email);
            message.push('>');
        }
        message.push('\n');
    }

    fn schedule_update(
        &mut self,
        clear_pending: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let handle = cx.entity().downgrade();
        self.reopen_commit_buffer(window, cx);
        self.update_visible_entries_task = cx.spawn_in(window, |_, mut cx| async move {
            cx.background_executor().timer(UPDATE_DEBOUNCE).await;
            if let Some(git_panel) = handle.upgrade() {
                git_panel
                    .update_in(&mut cx, |git_panel, _, cx| {
                        if clear_pending {
                            git_panel.clear_pending();
                        }
                        git_panel.update_visible_entries(cx);
                    })
                    .ok();
            }
        });
    }

    fn reopen_commit_buffer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_repo) = self.active_repository.as_ref() else {
            return;
        };
        let load_buffer = active_repo.update(cx, |active_repo, cx| {
            let project = self.project.read(cx);
            active_repo.open_commit_buffer(
                Some(project.languages().clone()),
                project.buffer_store().clone(),
                cx,
            )
        });

        cx.spawn_in(window, |git_panel, mut cx| async move {
            let buffer = load_buffer.await?;
            git_panel.update_in(&mut cx, |git_panel, window, cx| {
                if git_panel
                    .commit_editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .as_ref()
                    != Some(&buffer)
                {
                    git_panel.commit_editor = cx.new(|cx| {
                        commit_message_editor(buffer, git_panel.project.clone(), true, window, cx)
                    });
                }
            })
        })
        .detach_and_log_err(cx);
    }

    fn clear_pending(&mut self) {
        self.pending.retain(|v| !v.finished)
    }

    fn update_visible_entries(&mut self, cx: &mut Context<Self>) {
        self.entries.clear();
        self.entries_by_path.clear();
        let mut changed_entries = Vec::new();
        let mut new_entries = Vec::new();
        let mut conflict_entries = Vec::new();

        let Some(repo) = self.active_repository.as_ref() else {
            // Just clear entries if no repository is active.
            cx.notify();
            return;
        };

        // First pass - collect all paths
        let repo = repo.read(cx);
        let path_set = HashSet::from_iter(repo.status().map(|entry| entry.repo_path));

        // Second pass - create entries with proper depth calculation
        for entry in repo.status() {
            let (depth, difference) =
                Self::calculate_depth_and_difference(&entry.repo_path, &path_set);

            let is_conflict = repo.has_conflict(&entry.repo_path);
            let is_new = entry.status.is_created();
            let is_staged = entry.status.is_staged();

            if self.pending.iter().any(|pending| {
                pending.target_status == TargetStatus::Reverted
                    && !pending.finished
                    && pending.repo_paths.contains(&entry.repo_path)
            }) {
                continue;
            }

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

            let entry = GitStatusEntry {
                depth,
                display_name,
                repo_path: entry.repo_path.clone(),
                status: entry.status,
                is_staged,
            };

            if is_conflict {
                conflict_entries.push(entry);
            } else if is_new {
                new_entries.push(entry);
            } else {
                changed_entries.push(entry);
            }
        }

        // Sort entries by path to maintain consistent order
        conflict_entries.sort_by(|a, b| a.repo_path.cmp(&b.repo_path));
        changed_entries.sort_by(|a, b| a.repo_path.cmp(&b.repo_path));
        new_entries.sort_by(|a, b| a.repo_path.cmp(&b.repo_path));

        if conflict_entries.len() > 0 {
            self.entries.push(GitListEntry::Header(GitHeaderEntry {
                header: Section::Conflict,
            }));
            self.entries.extend(
                conflict_entries
                    .into_iter()
                    .map(GitListEntry::GitStatusEntry),
            );
        }

        if changed_entries.len() > 0 {
            self.entries.push(GitListEntry::Header(GitHeaderEntry {
                header: Section::Tracked,
            }));
            self.entries.extend(
                changed_entries
                    .into_iter()
                    .map(GitListEntry::GitStatusEntry),
            );
        }
        if new_entries.len() > 0 {
            self.entries.push(GitListEntry::Header(GitHeaderEntry {
                header: Section::New,
            }));
            self.entries
                .extend(new_entries.into_iter().map(GitListEntry::GitStatusEntry));
        }

        for (ix, entry) in self.entries.iter().enumerate() {
            if let Some(status_entry) = entry.status_entry() {
                self.entries_by_path
                    .insert(status_entry.repo_path.clone(), ix);
            }
        }
        self.update_counts(repo);

        self.select_first_entry_if_none(cx);

        cx.notify();
    }

    fn header_state(&self, header_type: Section) -> ToggleState {
        let (staged_count, count) = match header_type {
            Section::New => (self.new_staged_count, self.new_count),
            Section::Tracked => (self.tracked_staged_count, self.tracked_count),
            Section::Conflict => (self.conflicted_staged_count, self.conflicted_count),
        };
        if staged_count == 0 {
            ToggleState::Unselected
        } else if count == staged_count {
            ToggleState::Selected
        } else {
            ToggleState::Indeterminate
        }
    }

    fn update_counts(&mut self, repo: &Repository) {
        self.conflicted_count = 0;
        self.conflicted_staged_count = 0;
        self.new_count = 0;
        self.tracked_count = 0;
        self.new_staged_count = 0;
        self.tracked_staged_count = 0;
        for entry in &self.entries {
            let Some(status_entry) = entry.status_entry() else {
                continue;
            };
            if repo.has_conflict(&status_entry.repo_path) {
                self.conflicted_count += 1;
                if self.entry_is_staged(status_entry) != Some(false) {
                    self.conflicted_staged_count += 1;
                }
            } else if status_entry.status.is_created() {
                self.new_count += 1;
                if self.entry_is_staged(status_entry) != Some(false) {
                    self.new_staged_count += 1;
                }
            } else {
                self.tracked_count += 1;
                if self.entry_is_staged(status_entry) != Some(false) {
                    self.tracked_staged_count += 1;
                }
            }
        }
    }

    fn entry_is_staged(&self, entry: &GitStatusEntry) -> Option<bool> {
        for pending in self.pending.iter().rev() {
            if pending.repo_paths.contains(&entry.repo_path) {
                match pending.target_status {
                    TargetStatus::Staged => return Some(true),
                    TargetStatus::Unstaged => return Some(false),
                    TargetStatus::Reverted => continue,
                    TargetStatus::Unchanged => continue,
                }
            }
        }
        entry.is_staged
    }

    pub(crate) fn has_staged_changes(&self) -> bool {
        self.tracked_staged_count > 0
            || self.new_staged_count > 0
            || self.conflicted_staged_count > 0
    }

    pub(crate) fn has_unstaged_changes(&self) -> bool {
        self.tracked_count > self.tracked_staged_count
            || self.new_count > self.new_staged_count
            || self.conflicted_count > self.conflicted_staged_count
    }

    fn has_conflicts(&self) -> bool {
        self.conflicted_count > 0
    }

    fn has_tracked_changes(&self) -> bool {
        self.tracked_count > 0
    }

    pub fn has_unstaged_conflicts(&self) -> bool {
        self.conflicted_count > 0 && self.conflicted_count != self.conflicted_staged_count
    }

    fn show_err_toast(&self, e: anyhow::Error, cx: &mut App) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let notif_id = NotificationId::Named("git-operation-error".into());

        let message = e.to_string();
        workspace.update(cx, |workspace, cx| {
            let toast = Toast::new(notif_id, message).on_click("Open Zed Log", |window, cx| {
                window.dispatch_action(workspace::OpenLog.boxed_clone(), cx);
            });
            workspace.show_toast(toast, cx);
        });
    }

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

    pub fn indent_size(&self, window: &Window, cx: &mut Context<Self>) -> Pixels {
        Checkbox::container_size(cx).to_pixels(window.rem_size())
    }

    pub fn render_divider(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .items_center()
            .h(px(8.))
            .child(Divider::horizontal_dashed().color(DividerColor::Border))
    }

    pub fn render_panel_header(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let all_repositories = self
            .project
            .read(cx)
            .git_store()
            .read(cx)
            .all_repositories();

        let has_repo_above = all_repositories.iter().any(|repo| {
            repo.read(cx)
                .repository_entry
                .work_directory
                .is_above_project()
        });

        let has_visible_repo = all_repositories.len() > 0 || has_repo_above;

        if has_visible_repo {
            Some(
                self.panel_header_container(window, cx)
                    .child(
                        Label::new("Repository")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(self.render_repository_selector(cx))
                    .child(div().flex_grow())
                    .child(
                        Button::new("diff", "+/-")
                            .tooltip(Tooltip::for_action_title("Open diff", &Diff))
                            .on_click(|_, _, cx| {
                                cx.defer(|cx| {
                                    cx.dispatch_action(&Diff);
                                })
                            }),
                    ),
            )
        } else {
            None
        }
    }

    pub fn render_repository_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let active_repository = self.project.read(cx).active_repository(cx);
        let repository_display_name = active_repository
            .as_ref()
            .map(|repo| repo.read(cx).display_name(self.project.read(cx), cx))
            .unwrap_or_default();

        RepositorySelectorPopoverMenu::new(
            self.repository_selector.clone(),
            ButtonLike::new("active-repository")
                .style(ButtonStyle::Subtle)
                .child(Label::new(repository_display_name).size(LabelSize::Small)),
            Tooltip::text("Select a repository"),
        )
    }

    pub fn can_commit(&self) -> bool {
        (self.has_staged_changes() || self.has_tracked_changes()) && !self.has_unstaged_conflicts()
    }

    pub fn can_stage_all(&self) -> bool {
        self.has_unstaged_changes()
    }

    pub fn can_unstage_all(&self) -> bool {
        self.has_staged_changes()
    }

    pub(crate) fn render_co_authors(&self, cx: &Context<Self>) -> Option<AnyElement> {
        let potential_co_authors = self.potential_co_authors(cx);
        if potential_co_authors.is_empty() {
            None
        } else {
            Some(
                IconButton::new("co-authors", IconName::Person)
                    .icon_color(Color::Disabled)
                    .selected_icon_color(Color::Selected)
                    .toggle_state(self.add_coauthors)
                    .tooltip(move |_, cx| {
                        let title = format!(
                            "Add co-authored-by:{}{}",
                            if potential_co_authors.len() == 1 {
                                ""
                            } else {
                                "\n"
                            },
                            potential_co_authors
                                .iter()
                                .map(|(name, email)| format!(" {} <{}>", name, email))
                                .join("\n")
                        );
                        Tooltip::simple(title, cx)
                    })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.add_coauthors = !this.add_coauthors;
                        cx.notify();
                    }))
                    .into_any_element(),
            )
        }
    }

    pub fn render_commit_editor(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let editor = self.commit_editor.clone();
        let can_commit = self.can_commit()
            && self.pending_commit.is_none()
            && !editor.read(cx).is_empty(cx)
            && self.has_write_access(cx);
        let panel_editor_style = panel_editor_style(true, window, cx);
        let enable_coauthors = self.render_co_authors(cx);

        let editor_focus_handle = editor.read(cx).focus_handle(cx).clone();

        let focus_handle_1 = self.focus_handle(cx).clone();
        let tooltip = if self.has_staged_changes() {
            "Commit staged changes"
        } else {
            "Commit changes to tracked files"
        };
        let title = if self.has_staged_changes() {
            "Commit"
        } else {
            "Commit All"
        };

        let commit_button = panel_filled_button(title)
            .tooltip(move |window, cx| {
                let focus_handle = focus_handle_1.clone();
                Tooltip::for_action_in(tooltip, &Commit, &focus_handle, window, cx)
            })
            .disabled(!can_commit)
            .on_click({
                cx.listener(move |this, _: &ClickEvent, window, cx| this.commit_changes(window, cx))
            });

        let branch = self
            .active_repository
            .as_ref()
            .and_then(|repo| repo.read(cx).branch().map(|b| b.name.clone()))
            .unwrap_or_else(|| "<no branch>".into());

        let branch_selector = Button::new("branch-selector", branch)
            .color(Color::Muted)
            .style(ButtonStyle::Subtle)
            .icon(IconName::GitBranch)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .size(ButtonSize::Compact)
            .icon_position(IconPosition::Start)
            .tooltip(Tooltip::for_action_title(
                "Switch Branch",
                &zed_actions::git::Branch,
            ))
            .on_click(cx.listener(|_, _, window, cx| {
                window.dispatch_action(zed_actions::git::Branch.boxed_clone(), cx);
            }))
            .style(ButtonStyle::Transparent);

        let footer_size = px(32.);
        let gap = px(16.0);

        let max_height = window.line_height() * 6. + gap + footer_size;

        panel_editor_container(window, cx)
            .id("commit-editor-container")
            .relative()
            .h(max_height)
            .w_full()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().editor_background)
            .cursor_text()
            .on_click(cx.listener(move |_, _: &ClickEvent, window, _cx| {
                window.focus(&editor_focus_handle);
            }))
            .when(!self.modal_open, |el| {
                el.child(EditorElement::new(&self.commit_editor, panel_editor_style))
                    .child(
                        h_flex()
                            .absolute()
                            .bottom_0()
                            .left_2()
                            .h(footer_size)
                            .flex_none()
                            .child(branch_selector),
                    )
                    .child(
                        h_flex()
                            .absolute()
                            .bottom_0()
                            .right_2()
                            .h(footer_size)
                            .flex_none()
                            .children(enable_coauthors)
                            .child(commit_button),
                    )
            })
    }

    fn render_previous_commit(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let active_repository = self.active_repository.as_ref()?;
        let branch = active_repository.read(cx).branch()?;
        let commit = branch.most_recent_commit.as_ref()?.clone();

        if branch.upstream.as_ref().is_some_and(|upstream| {
            if let Some(tracking) = &upstream.tracking {
                tracking.ahead == 0
            } else {
                true
            }
        }) {
            return None;
        }
        let tooltip = if self.has_staged_changes() {
            "git reset HEAD^ --soft"
        } else {
            "git reset HEAD^"
        };

        let this = cx.entity();
        Some(
            h_flex()
                .items_center()
                .py_1p5()
                .px(px(8.))
                .bg(cx.theme().colors().background)
                .border_t_1()
                .border_color(cx.theme().colors().border)
                .gap_1p5()
                .child(
                    div()
                        .flex_grow()
                        .overflow_hidden()
                        .max_w(relative(0.6))
                        .h_full()
                        .child(
                            Label::new(commit.subject.clone())
                                .size(LabelSize::Small)
                                .text_ellipsis(),
                        )
                        .id("commit-msg-hover")
                        .hoverable_tooltip(move |window, cx| {
                            GitPanelMessageTooltip::new(
                                this.clone(),
                                commit.sha.clone(),
                                window,
                                cx,
                            )
                            .into()
                        }),
                )
                .child(div().flex_1())
                .child(
                    panel_filled_button("Uncommit")
                        .icon(IconName::Undo)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .icon_position(IconPosition::Start)
                        .tooltip(Tooltip::for_action_title(tooltip, &git::Uncommit))
                        .on_click(cx.listener(|this, _, window, cx| this.uncommit(window, cx))),
                ),
        )
    }

    fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .h_full()
            .flex_grow()
            .justify_center()
            .items_center()
            .child(
                v_flex()
                    .gap_3()
                    .child(if self.active_repository.is_some() {
                        "No changes to commit"
                    } else {
                        "No Git repositories"
                    })
                    .text_ui_sm(cx)
                    .mx_auto()
                    .text_color(Color::Placeholder.color(cx)),
            )
    }

    fn render_scrollbar(&self, cx: &mut Context<Self>) -> Option<Stateful<Div>> {
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
                .on_mouse_move(cx.listener(|_, _, _, cx| {
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        if !this.scrollbar_state.is_dragging()
                            && !this.focus_handle.contains_focused(window, cx)
                        {
                            this.hide_scrollbar(window, cx);
                            cx.notify();
                        }

                        cx.stop_propagation();
                    }),
                )
                .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                    cx.notify();
                }))
                .children(Scrollbar::vertical(
                    // percentage as f32..end_offset as f32,
                    self.scrollbar_state.clone(),
                )),
        )
    }

    pub fn render_buffer_header_controls(
        &self,
        entity: &Entity<Self>,
        file: &Arc<dyn File>,
        _: &Window,
        cx: &App,
    ) -> Option<AnyElement> {
        let repo = self.active_repository.as_ref()?.read(cx);
        let repo_path = repo.worktree_id_path_to_repo_path(file.worktree_id(cx), file.path())?;
        let ix = self.entries_by_path.get(&repo_path)?;
        let entry = self.entries.get(*ix)?;

        let is_staged = self.entry_is_staged(entry.status_entry()?);

        let checkbox = Checkbox::new("stage-file", is_staged.into())
            .disabled(!self.has_write_access(cx))
            .fill()
            .elevation(ElevationIndex::Surface)
            .on_click({
                let entry = entry.clone();
                let git_panel = entity.downgrade();
                move |_, window, cx| {
                    git_panel
                        .update(cx, |this, cx| {
                            this.toggle_staged_for_entry(&entry, window, cx);
                            cx.stop_propagation();
                        })
                        .ok();
                }
            });
        Some(
            h_flex()
                .id("start-slot")
                .text_lg()
                .child(checkbox)
                .on_mouse_down(MouseButton::Left, |_, _, cx| {
                    // prevent the list item active state triggering when toggling checkbox
                    cx.stop_propagation();
                })
                .into_any_element(),
        )
    }

    fn render_entries(
        &self,
        has_write_access: bool,
        _: &Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let entry_count = self.entries.len();

        v_flex()
            .size_full()
            .flex_grow()
            .overflow_hidden()
            .child(
                uniform_list(cx.entity().clone(), "entries", entry_count, {
                    move |this, range, window, cx| {
                        let mut items = Vec::with_capacity(range.end - range.start);

                        for ix in range {
                            match &this.entries.get(ix) {
                                Some(GitListEntry::GitStatusEntry(entry)) => {
                                    items.push(this.render_entry(
                                        ix,
                                        entry,
                                        has_write_access,
                                        window,
                                        cx,
                                    ));
                                }
                                Some(GitListEntry::Header(header)) => {
                                    items.push(this.render_list_header(
                                        ix,
                                        header,
                                        has_write_access,
                                        window,
                                        cx,
                                    ));
                                }
                                None => {}
                            }
                        }

                        items
                    }
                })
                .size_full()
                .with_sizing_behavior(ListSizingBehavior::Infer)
                .with_horizontal_sizing_behavior(ListHorizontalSizingBehavior::Unconstrained)
                .track_scroll(self.scroll_handle.clone()),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                    this.deploy_panel_context_menu(event.position, window, cx)
                }),
            )
            .children(self.render_scrollbar(cx))
    }

    fn entry_label(&self, label: impl Into<SharedString>, color: Color) -> Label {
        Label::new(label.into()).color(color).single_line()
    }

    fn render_list_header(
        &self,
        ix: usize,
        header: &GitHeaderEntry,
        _: bool,
        _: &Window,
        _: &Context<Self>,
    ) -> AnyElement {
        div()
            .w_full()
            .child(
                ListItem::new(ix)
                    .spacing(ListItemSpacing::Sparse)
                    .disabled(true)
                    .child(
                        Label::new(header.title())
                            .color(Color::Muted)
                            .size(LabelSize::Small)
                            .single_line(),
                    ),
            )
            .into_any_element()
    }

    fn load_commit_details(
        &self,
        sha: &str,
        cx: &mut Context<Self>,
    ) -> Task<Result<CommitDetails>> {
        let Some(repo) = self.active_repository.clone() else {
            return Task::ready(Err(anyhow::anyhow!("no active repo")));
        };
        repo.update(cx, |repo, cx| repo.show(sha, cx))
    }

    fn deploy_entry_context_menu(
        &mut self,
        position: Point<Pixels>,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(entry) = self.entries.get(ix).and_then(|e| e.status_entry()) else {
            return;
        };
        let stage_title = if entry.status.is_staged() == Some(true) {
            "Unstage File"
        } else {
            "Stage File"
        };
        let revert_title = if entry.status.is_deleted() {
            "Restore file"
        } else if entry.status.is_created() {
            "Trash file"
        } else {
            "Discard changes"
        };
        let context_menu = ContextMenu::build(window, cx, |context_menu, _, _| {
            context_menu
                .action(stage_title, ToggleStaged.boxed_clone())
                .action(revert_title, git::RestoreFile.boxed_clone())
                .separator()
                .action("Open Diff", Confirm.boxed_clone())
                .action("Open File", SecondaryConfirm.boxed_clone())
        });
        self.selected_entry = Some(ix);
        self.set_context_menu(context_menu, position, window, cx);
    }

    fn deploy_panel_context_menu(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let context_menu = ContextMenu::build(window, cx, |context_menu, _, _| {
            context_menu
                .action("Stage All", StageAll.boxed_clone())
                .action("Unstage All", UnstageAll.boxed_clone())
                .action("Open Diff", project_diff::Diff.boxed_clone())
                .separator()
                .action("Discard Tracked Changes", RestoreTrackedFiles.boxed_clone())
                .action("Trash Untracked Files", TrashUntrackedFiles.boxed_clone())
        });
        self.set_context_menu(context_menu, position, window, cx);
    }

    fn set_context_menu(
        &mut self,
        context_menu: Entity<ContextMenu>,
        position: Point<Pixels>,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        let subscription = cx.subscribe_in(
            &context_menu,
            window,
            |this, _, _: &DismissEvent, window, cx| {
                if this.context_menu.as_ref().is_some_and(|context_menu| {
                    context_menu.0.focus_handle(cx).contains_focused(window, cx)
                }) {
                    cx.focus_self(window);
                }
                this.context_menu.take();
                cx.notify();
            },
        );
        self.context_menu = Some((context_menu, position, subscription));
        cx.notify();
    }

    fn render_entry(
        &self,
        ix: usize,
        entry: &GitStatusEntry,
        has_write_access: bool,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let display_name = entry
            .repo_path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| entry.repo_path.to_string_lossy().into_owned());

        let repo_path = entry.repo_path.clone();
        let selected = self.selected_entry == Some(ix);
        let status_style = GitPanelSettings::get_global(cx).status_style;
        let status = entry.status;
        let has_conflict = status.is_conflicted();
        let is_modified = status.is_modified();
        let is_deleted = status.is_deleted();

        let label_color = if status_style == StatusStyle::LabelColor {
            if has_conflict {
                Color::Conflict
            } else if is_modified {
                Color::Modified
            } else if is_deleted {
                // We don't want a bunch of red labels in the list
                Color::Disabled
            } else {
                Color::Created
            }
        } else {
            Color::Default
        };

        let path_color = if status.is_deleted() {
            Color::Disabled
        } else {
            Color::Muted
        };

        let id: ElementId = ElementId::Name(format!("entry_{}", display_name).into());

        let mut is_staged: ToggleState = self.entry_is_staged(entry).into();

        if !self.has_staged_changes() && !self.has_conflicts() && !entry.status.is_created() {
            is_staged = ToggleState::Selected;
        }

        let checkbox = Checkbox::new(id, is_staged)
            .disabled(!has_write_access)
            .fill()
            .placeholder(!self.has_staged_changes() && !self.has_conflicts())
            .elevation(ElevationIndex::Surface)
            .on_click({
                let entry = entry.clone();
                cx.listener(move |this, _, window, cx| {
                    this.toggle_staged_for_entry(
                        &GitListEntry::GitStatusEntry(entry.clone()),
                        window,
                        cx,
                    );
                    cx.stop_propagation();
                })
            });

        let start_slot = h_flex()
            .id(("start-slot", ix))
            .gap(DynamicSpacing::Base04.rems(cx))
            .child(checkbox)
            .tooltip(|window, cx| Tooltip::for_action("Stage File", &ToggleStaged, window, cx))
            .child(git_status_icon(status, cx))
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                // prevent the list item active state triggering when toggling checkbox
                cx.stop_propagation();
            });

        div()
            .w_full()
            .child(
                ListItem::new(ix)
                    .spacing(ListItemSpacing::Sparse)
                    .start_slot(start_slot)
                    .toggle_state(selected)
                    .focused(selected && self.focus_handle(cx).is_focused(window))
                    .disabled(!has_write_access)
                    .on_click({
                        cx.listener(move |this, event: &ClickEvent, window, cx| {
                            this.selected_entry = Some(ix);
                            cx.notify();
                            if event.modifiers().secondary() {
                                this.open_file(&Default::default(), window, cx)
                            } else {
                                this.open_diff(&Default::default(), window, cx);
                            }
                        })
                    })
                    .on_secondary_mouse_down(cx.listener(
                        move |this, event: &MouseDownEvent, window, cx| {
                            this.deploy_entry_context_menu(event.position, ix, window, cx);
                            cx.stop_propagation();
                        },
                    ))
                    .child(
                        h_flex()
                            .when_some(repo_path.parent(), |this, parent| {
                                let parent_str = parent.to_string_lossy();
                                if !parent_str.is_empty() {
                                    this.child(
                                        self.entry_label(format!("{}/", parent_str), path_color)
                                            .when(status.is_deleted(), |this| this.strikethrough()),
                                    )
                                } else {
                                    this
                                }
                            })
                            .child(
                                self.entry_label(display_name.clone(), label_color)
                                    .when(status.is_deleted(), |this| this.strikethrough()),
                            ),
                    ),
            )
            .into_any_element()
    }

    fn has_write_access(&self, cx: &App) -> bool {
        !self.project.read(cx).is_read_only(cx)
    }
}

impl Render for GitPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let project = self.project.read(cx);
        let has_entries = self.entries.len() > 0;
        let room = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).active_call()?.read(cx).room().cloned());

        let has_write_access = self.has_write_access(cx);

        let has_co_authors = room.map_or(false, |room| {
            room.read(cx)
                .remote_participants()
                .values()
                .any(|remote_participant| remote_participant.can_write())
        });

        v_flex()
            .id("git_panel")
            .key_context(self.dispatch_context(window, cx))
            .track_focus(&self.focus_handle)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .when(has_write_access && !project.is_read_only(cx), |this| {
                this.on_action(cx.listener(|this, &ToggleStaged, window, cx| {
                    this.toggle_staged_for_selected(&ToggleStaged, window, cx)
                }))
                .on_action(cx.listener(GitPanel::commit))
            })
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::close_panel))
            .on_action(cx.listener(Self::open_diff))
            .on_action(cx.listener(Self::open_file))
            .on_action(cx.listener(Self::revert_selected))
            .on_action(cx.listener(Self::focus_changes_list))
            .on_action(cx.listener(Self::focus_editor))
            .on_action(cx.listener(Self::toggle_staged_for_selected))
            .on_action(cx.listener(Self::stage_all))
            .on_action(cx.listener(Self::unstage_all))
            .on_action(cx.listener(Self::discard_tracked_changes))
            .on_action(cx.listener(Self::clean_all))
            .when(has_write_access && has_co_authors, |git_panel| {
                git_panel.on_action(cx.listener(Self::toggle_fill_co_authors))
            })
            // .on_action(cx.listener(|this, &OpenSelected, cx| this.open_selected(&OpenSelected, cx)))
            .on_hover(cx.listener(|this, hovered, window, cx| {
                if *hovered {
                    this.show_scrollbar = true;
                    this.hide_scrollbar_task.take();
                    cx.notify();
                } else if !this.focus_handle.contains_focused(window, cx) {
                    this.hide_scrollbar(window, cx);
                }
            }))
            .size_full()
            .overflow_hidden()
            .bg(ElevationIndex::Surface.bg(cx))
            .child(if has_entries {
                v_flex()
                    .size_full()
                    .children(self.render_panel_header(window, cx))
                    .child(self.render_entries(has_write_access, window, cx))
                    .children(self.render_previous_commit(cx))
                    .child(self.render_commit_editor(window, cx))
                    .into_any_element()
            } else {
                self.render_empty_state(cx).into_any_element()
            })
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(gpui::Corner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

impl Focusable for GitPanel {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<Event> for GitPanel {}

impl EventEmitter<PanelEvent> for GitPanel {}

pub(crate) struct GitPanelAddon {
    pub(crate) workspace: WeakEntity<Workspace>,
}

impl editor::Addon for GitPanelAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn render_buffer_header_controls(
        &self,
        excerpt_info: &ExcerptInfo,
        window: &Window,
        cx: &App,
    ) -> Option<AnyElement> {
        let file = excerpt_info.buffer.file()?;
        let git_panel = self.workspace.upgrade()?.read(cx).panel::<GitPanel>(cx)?;

        git_panel
            .read(cx)
            .render_buffer_header_controls(&git_panel, &file, window, cx)
    }
}

impl Panel for GitPanel {
    fn persistent_name() -> &'static str {
        "GitPanel"
    }

    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        GitPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file::<GitPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| settings.dock = Some(position),
        );
    }

    fn size(&self, _: &Window, cx: &App) -> Pixels {
        self.width
            .unwrap_or_else(|| GitPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, _: &mut Window, cx: &mut Context<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<ui::IconName> {
        Some(ui::IconName::GitBranch).filter(|_| GitPanelSettings::get_global(cx).button)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Git Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        2
    }
}

impl PanelHeader for GitPanel {}

struct GitPanelMessageTooltip {
    commit_tooltip: Option<Entity<CommitTooltip>>,
}

impl GitPanelMessageTooltip {
    fn new(
        git_panel: Entity<GitPanel>,
        sha: SharedString,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            cx.spawn_in(window, |this, mut cx| async move {
                let details = git_panel
                    .update(&mut cx, |git_panel, cx| {
                        git_panel.load_commit_details(&sha, cx)
                    })?
                    .await?;

                let commit_details = editor::commit_tooltip::CommitDetails {
                    sha: details.sha.clone(),
                    committer_name: details.committer_name.clone(),
                    committer_email: details.committer_email.clone(),
                    commit_time: OffsetDateTime::from_unix_timestamp(details.commit_timestamp)?,
                    message: Some(editor::commit_tooltip::ParsedCommitMessage {
                        message: details.message.clone(),
                        ..Default::default()
                    }),
                };

                this.update_in(&mut cx, |this: &mut GitPanelMessageTooltip, window, cx| {
                    this.commit_tooltip =
                        Some(cx.new(move |cx| CommitTooltip::new(commit_details, window, cx)));
                    cx.notify();
                })
            })
            .detach();

            Self {
                commit_tooltip: None,
            }
        })
    }
}

impl Render for GitPanelMessageTooltip {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        if let Some(commit_tooltip) = &self.commit_tooltip {
            commit_tooltip.clone().into_any_element()
        } else {
            gpui::Empty.into_any_element()
        }
    }
}
