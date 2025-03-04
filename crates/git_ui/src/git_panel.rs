use crate::branch_picker::{self, BranchList};
use crate::git_panel_settings::StatusStyle;
use crate::remote_output_toast::{RemoteAction, RemoteOutputToast};
use crate::repository_selector::RepositorySelectorPopoverMenu;
use crate::{
    git_panel_settings::GitPanelSettings, git_status_icon, repository_selector::RepositorySelector,
};
use crate::{picker_prompt, project_diff, ProjectDiff};
use db::kvp::KEY_VALUE_STORE;
use editor::commit_tooltip::CommitTooltip;
use editor::{
    scroll::ScrollbarAutoHide, Editor, EditorElement, EditorMode, EditorSettings, MultiBuffer,
    ShowScrollbar,
};
use git::repository::{
    Branch, CommitDetails, CommitSummary, PushOptions, Remote, RemoteCommandOutput, ResetMode,
    Upstream, UpstreamTracking, UpstreamTrackingStatus,
};
use git::{repository::RepoPath, status::FileStatus, Commit, ToggleStaged};
use git::{RestoreTrackedFiles, StageAll, TrashUntrackedFiles, UnstageAll};
use gpui::*;
use itertools::Itertools;
use language::{Buffer, File};
use menu::{Confirm, SecondaryConfirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use multi_buffer::ExcerptInfo;
use panel::{
    panel_editor_container, panel_editor_style, panel_filled_button, panel_icon_button, PanelHeader,
};
use project::{
    git::{GitEvent, Repository},
    Fs, Project, ProjectPath,
};
use serde::{Deserialize, Serialize};
use settings::Settings as _;
use smallvec::smallvec;
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
use std::{collections::HashSet, sync::Arc, time::Duration, usize};
use strum::{IntoEnumIterator, VariantNames};
use time::OffsetDateTime;
use ui::{
    prelude::*, ButtonLike, Checkbox, ContextMenu, ElevationIndex, PopoverButton, PopoverMenu,
    Scrollbar, ScrollbarState, Tooltip,
};
use util::{maybe, post_inc, ResultExt, TryFutureExt};

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

fn git_panel_context_menu(window: &mut Window, cx: &mut App) -> Entity<ContextMenu> {
    ContextMenu::build(window, cx, |context_menu, _, _| {
        context_menu
            .action("Stage All", StageAll.boxed_clone())
            .action("Unstage All", UnstageAll.boxed_clone())
            .separator()
            .action("Open Diff", project_diff::Diff.boxed_clone())
            .separator()
            .action("Discard Tracked Changes", RestoreTrackedFiles.boxed_clone())
            .action("Trash Untracked Files", TrashUntrackedFiles.boxed_clone())
    })
}

const GIT_PANEL_KEY: &str = "GitPanel";

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
                workspace.toggle_panel_focus::<GitPanel>(window, cx);
            });
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

type RemoteOperations = Rc<RefCell<HashSet<u32>>>;

pub struct GitPanel {
    remote_operation_id: u32,
    pending_remote_operations: RemoteOperations,
    pub(crate) active_repository: Option<Entity<Repository>>,
    commit_editor: Entity<Editor>,
    pub(crate) suggested_commit_message: Option<String>,
    conflicted_count: usize,
    conflicted_staged_count: usize,
    current_modifiers: Modifiers,
    add_coauthors: bool,
    entries: Vec<GitListEntry>,
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
    marked_entries: Vec<usize>,
    show_scrollbar: bool,
    tracked_count: usize,
    tracked_staged_count: usize,
    update_visible_entries_task: Task<()>,
    width: Option<Pixels>,
    workspace: WeakEntity<Workspace>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    modal_open: bool,
}

struct RemoteOperationGuard {
    id: u32,
    pending_remote_operations: RemoteOperations,
}

impl Drop for RemoteOperationGuard {
    fn drop(&mut self) {
        self.pending_remote_operations.borrow_mut().remove(&self.id);
    }
}

pub(crate) fn commit_message_editor(
    commit_message_buffer: Entity<Buffer>,
    placeholder: Option<&str>,
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
    let placeholder = placeholder.unwrap_or("Enter commit message");
    commit_editor.set_placeholder_text(placeholder, cx);
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
                commit_message_editor(temporary_buffer, None, project.clone(), true, window, cx)
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
                pending_remote_operations: Default::default(),
                remote_operation_id: 0,
                active_repository,
                commit_editor,
                suggested_commit_message: None,
                conflicted_count: 0,
                conflicted_staged_count: 0,
                current_modifiers: window.modifiers(),
                add_coauthors: true,
                entries: Vec::new(),
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
                marked_entries: Vec::new(),
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

    pub fn entry_by_path(&self, path: &RepoPath) -> Option<usize> {
        fn binary_search<F>(mut low: usize, mut high: usize, is_target: F) -> Option<usize>
        where
            F: Fn(usize) -> std::cmp::Ordering,
        {
            while low < high {
                let mid = low + (high - low) / 2;
                match is_target(mid) {
                    std::cmp::Ordering::Equal => return Some(mid),
                    std::cmp::Ordering::Less => low = mid + 1,
                    std::cmp::Ordering::Greater => high = mid,
                }
            }
            None
        }
        if self.conflicted_count > 0 {
            let conflicted_start = 1;
            if let Some(ix) = binary_search(
                conflicted_start,
                conflicted_start + self.conflicted_count,
                |ix| {
                    self.entries[ix]
                        .status_entry()
                        .unwrap()
                        .repo_path
                        .cmp(&path)
                },
            ) {
                return Some(ix);
            }
        }
        if self.tracked_count > 0 {
            let tracked_start = if self.conflicted_count > 0 {
                1 + self.conflicted_count
            } else {
                0
            } + 1;
            if let Some(ix) =
                binary_search(tracked_start, tracked_start + self.tracked_count, |ix| {
                    self.entries[ix]
                        .status_entry()
                        .unwrap()
                        .repo_path
                        .cmp(&path)
                })
            {
                return Some(ix);
            }
        }
        if self.new_count > 0 {
            let untracked_start = if self.conflicted_count > 0 {
                1 + self.conflicted_count
            } else {
                0
            } + if self.tracked_count > 0 {
                1 + self.tracked_count
            } else {
                0
            } + 1;
            if let Some(ix) =
                binary_search(untracked_start, untracked_start + self.new_count, |ix| {
                    self.entries[ix]
                        .status_entry()
                        .unwrap()
                        .repo_path
                        .cmp(&path)
                })
            {
                return Some(ix);
            }
        }
        None
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
        let Some(ix) = self.entry_by_path(&repo_path) else {
            return;
        };
        self.selected_entry = Some(ix);
        cx.notify();
    }

    fn start_remote_operation(&mut self) -> RemoteOperationGuard {
        let id = post_inc(&mut self.remote_operation_id);
        self.pending_remote_operations.borrow_mut().insert(id);

        RemoteOperationGuard {
            id,
            pending_remote_operations: self.pending_remote_operations.clone(),
        }
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

    fn scroll_to_selected_entry(&mut self, cx: &mut Context<Self>) {
        if let Some(selected_entry) = self.selected_entry {
            self.scroll_handle
                .scroll_to_item(selected_entry, ScrollStrategy::Center);
        }

        cx.notify();
    }

    fn select_first(&mut self, _: &SelectFirst, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.entries.is_empty() {
            self.selected_entry = Some(1);
            self.scroll_to_selected_entry(cx);
        }
    }

    fn select_previous(
        &mut self,
        _: &SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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

    pub(crate) fn editor_focus_handle(&self, cx: &mut Context<Self>) -> FocusHandle {
        self.commit_editor.focus_handle(cx).clone()
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
            let workspace = self.workspace.upgrade()?;
            let git_repo = self.active_repository.as_ref()?;

            if let Some(project_diff) = workspace.read(cx).active_item_as::<ProjectDiff>(cx) {
                if let Some(project_path) = project_diff.read(cx).active_path(cx) {
                    if Some(&entry.repo_path)
                        == git_repo
                            .read(cx)
                            .project_path_to_repo_path(&project_path)
                            .as_ref()
                    {
                        project_diff.focus_handle(cx).focus(window);
                        return None;
                    }
                }
            };

            self.workspace
                .update(cx, |workspace, cx| {
                    ProjectDiff::deploy_at(workspace, Some(entry.clone()), window, cx);
                })
                .ok();
            self.focus_handle.focus(window);

            Some(())
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
                        .open_path_preview(path, None, false, false, true, window, cx)
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

    fn restore_tracked_files(
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
        let mut details = entries
            .iter()
            .filter_map(|entry| entry.repo_path.0.file_name())
            .map(|filename| filename.to_string_lossy())
            .take(5)
            .join("\n");
        if entries.len() > 5 {
            details.push_str(&format!("\nand {} more…", entries.len() - 5))
        }

        #[derive(strum::EnumIter, strum::VariantNames)]
        #[strum(serialize_all = "title_case")]
        enum RestoreCancel {
            RestoreTrackedFiles,
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
                Ok(RestoreCancel::RestoreTrackedFiles) => {
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

        let mut details = to_delete
            .iter()
            .map(|entry| {
                entry
                    .repo_path
                    .0
                    .file_name()
                    .map(|f| f.to_string_lossy())
                    .unwrap_or_default()
            })
            .take(5)
            .join("\n");

        if to_delete.len() > 5 {
            details.push_str(&format!("\nand {} more…", to_delete.len() - 5))
        }

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

    pub fn total_staged_count(&self) -> usize {
        self.tracked_staged_count + self.new_staged_count + self.conflicted_staged_count
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

    fn commit(&mut self, _: &git::Commit, window: &mut Window, cx: &mut Context<Self>) {
        if self
            .commit_editor
            .focus_handle(cx)
            .contains_focused(window, cx)
        {
            self.commit_changes(window, cx)
        } else {
            cx.propagate();
        }
    }

    fn custom_or_suggested_commit_message(&self, cx: &mut Context<Self>) -> Option<String> {
        let message = self.commit_editor.read(cx).text(cx);

        if !message.is_empty() {
            return Some(message);
        }

        self.suggested_commit_message
            .clone()
            .filter(|message| !message.is_empty())
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

        let commit_message = self.custom_or_suggested_commit_message(cx);

        let Some(mut message) = commit_message else {
            self.commit_editor.read(cx).focus_handle(cx).focus(window);
            return;
        };

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

        // TODO: Use git merge-base to find the upstream and main branch split
        let confirmation = Task::ready(true);
        // let confirmation = if self.commit_editor.read(cx).is_empty(cx) {
        //     Task::ready(true)
        // } else {
        //     let prompt = window.prompt(
        //         PromptLevel::Warning,
        //         "Uncomitting will replace the current commit message with the previous commit's message",
        //         None,
        //         &["Ok", "Cancel"],
        //         cx,
        //     );
        //     cx.spawn(|_, _| async move { prompt.await.is_ok_and(|i| i == 0) })
        // };

        let prior_head = self.load_commit_details("HEAD", cx);

        let task = cx.spawn_in(window, |this, mut cx| async move {
            let result = maybe!(async {
                if !confirmation.await {
                    Ok(None)
                } else {
                    let prior_head = prior_head.await?;

                    repo.update(&mut cx, |repo, _| repo.reset("HEAD^", ResetMode::Soft))?
                        .await??;

                    Ok(Some(prior_head))
                }
            })
            .await;

            this.update_in(&mut cx, |this, window, cx| {
                this.pending_commit.take();
                match result {
                    Ok(None) => {}
                    Ok(Some(prior_commit)) => {
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

    /// Suggests a commit message based on the changed files and their statuses
    pub fn suggest_commit_message(&self) -> Option<String> {
        if self.total_staged_count() != 1 {
            return None;
        }

        let entry = self
            .entries
            .iter()
            .find(|entry| match entry.status_entry() {
                Some(entry) => entry.is_staged.unwrap_or(false),
                _ => false,
            })?;

        let GitListEntry::GitStatusEntry(git_status_entry) = entry.clone() else {
            return None;
        };

        let action_text = if git_status_entry.status.is_deleted() {
            Some("Delete")
        } else if git_status_entry.status.is_created() {
            Some("Create")
        } else if git_status_entry.status.is_modified() {
            Some("Update")
        } else {
            None
        }?;

        let file_name = git_status_entry
            .repo_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        Some(format!("{} {}", action_text, file_name))
    }

    fn update_editor_placeholder(&mut self, cx: &mut Context<Self>) {
        self.suggested_commit_message = self.suggest_commit_message();
        let placeholder_text = self
            .suggested_commit_message
            .as_deref()
            .unwrap_or("Enter commit message");

        self.commit_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text(Arc::from(placeholder_text), cx)
        });

        cx.notify();
    }

    fn fetch(&mut self, _: &git::Fetch, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(repo) = self.active_repository.clone() else {
            return;
        };
        let guard = self.start_remote_operation();
        let fetch = repo.read(cx).fetch();
        cx.spawn(|this, mut cx| async move {
            let remote_message = fetch.await?;
            drop(guard);
            this.update(&mut cx, |this, cx| {
                match remote_message {
                    Ok(remote_message) => {
                        this.show_remote_output(RemoteAction::Fetch, remote_message, cx);
                    }
                    Err(e) => {
                        this.show_err_toast(e, cx);
                    }
                }

                anyhow::Ok(())
            })
            .ok();
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn pull(&mut self, _: &git::Pull, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repo) = self.active_repository.clone() else {
            return;
        };
        let Some(branch) = repo.read(cx).current_branch() else {
            return;
        };
        let branch = branch.clone();
        let guard = self.start_remote_operation();
        let remote = self.get_current_remote(window, cx);
        cx.spawn(move |this, mut cx| async move {
            let remote = match remote.await {
                Ok(Some(remote)) => remote,
                Ok(None) => {
                    return Ok(());
                }
                Err(e) => {
                    log::error!("Failed to get current remote: {}", e);
                    this.update(&mut cx, |this, cx| this.show_err_toast(e, cx))
                        .ok();
                    return Ok(());
                }
            };

            let pull = repo.update(&mut cx, |repo, _cx| {
                repo.pull(branch.name.clone(), remote.name.clone())
            })?;

            let remote_message = pull.await?;
            drop(guard);

            this.update(&mut cx, |this, cx| match remote_message {
                Ok(remote_message) => {
                    this.show_remote_output(RemoteAction::Pull, remote_message, cx)
                }
                Err(err) => this.show_err_toast(err, cx),
            })
            .ok();

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn push(&mut self, action: &git::Push, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repo) = self.active_repository.clone() else {
            return;
        };
        let Some(branch) = repo.read(cx).current_branch() else {
            return;
        };
        let branch = branch.clone();
        let guard = self.start_remote_operation();
        let options = action.options;
        let remote = self.get_current_remote(window, cx);

        cx.spawn(move |this, mut cx| async move {
            let remote = match remote.await {
                Ok(Some(remote)) => remote,
                Ok(None) => {
                    return Ok(());
                }
                Err(e) => {
                    log::error!("Failed to get current remote: {}", e);
                    this.update(&mut cx, |this, cx| this.show_err_toast(e, cx))
                        .ok();
                    return Ok(());
                }
            };

            let push = repo.update(&mut cx, |repo, _cx| {
                repo.push(branch.name.clone(), remote.name.clone(), options)
            })?;

            let remote_output = push.await?;

            drop(guard);

            this.update(&mut cx, |this, cx| match remote_output {
                Ok(remote_message) => {
                    this.show_remote_output(RemoteAction::Push(remote), remote_message, cx);
                }
                Err(e) => {
                    this.show_err_toast(e, cx);
                }
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn get_current_remote(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl Future<Output = Result<Option<Remote>>> {
        let repo = self.active_repository.clone();
        let workspace = self.workspace.clone();
        let mut cx = window.to_async(cx);

        async move {
            let Some(repo) = repo else {
                return Err(anyhow::anyhow!("No active repository"));
            };

            let mut current_remotes: Vec<Remote> = repo
                .update(&mut cx, |repo, _| {
                    let Some(current_branch) = repo.current_branch() else {
                        return Err(anyhow::anyhow!("No active branch"));
                    };

                    Ok(repo.get_remotes(Some(current_branch.name.to_string())))
                })??
                .await??;

            if current_remotes.len() == 0 {
                return Err(anyhow::anyhow!("No active remote"));
            } else if current_remotes.len() == 1 {
                return Ok(Some(current_remotes.pop().unwrap()));
            } else {
                let current_remotes: Vec<_> = current_remotes
                    .into_iter()
                    .map(|remotes| remotes.name)
                    .collect();
                let selection = cx
                    .update(|window, cx| {
                        picker_prompt::prompt(
                            "Pick which remote to push to",
                            current_remotes.clone(),
                            workspace,
                            window,
                            cx,
                        )
                    })?
                    .await?;

                Ok(selection.map(|selection| Remote {
                    name: current_remotes[selection].clone(),
                }))
            }
        }
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
                        git_panel.update_editor_placeholder(cx);
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
                        commit_message_editor(
                            buffer,
                            git_panel.suggested_commit_message.as_deref(),
                            git_panel.project.clone(),
                            true,
                            window,
                            cx,
                        )
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

        // Second pass - create entries with proper depth calculation
        for entry in repo.status() {
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

            let entry = GitStatusEntry {
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

        let mut message = e.to_string().trim().to_string();
        let toast;
        if message.matches("Authentication failed").count() >= 1 {
            message = format!(
                "{}\n\n{}",
                message, "Please set your credentials via the CLI"
            );
            toast = Toast::new(notif_id, message);
        } else {
            toast = Toast::new(notif_id, message).on_click("Open Zed Log", |window, cx| {
                window.dispatch_action(workspace::OpenLog.boxed_clone(), cx);
            });
        }
        workspace.update(cx, |workspace, cx| {
            workspace.show_toast(toast, cx);
        });
    }

    fn show_remote_output(&self, action: RemoteAction, info: RemoteCommandOutput, cx: &mut App) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let notification_id = NotificationId::Named("git-remote-info".into());

        workspace.update(cx, |workspace, cx| {
            workspace.show_notification(notification_id.clone(), cx, |cx| {
                let workspace = cx.weak_entity();
                cx.new(|cx| RemoteOutputToast::new(action, info, notification_id, workspace, cx))
            });
        });
    }

    pub fn render_spinner(&self) -> Option<impl IntoElement> {
        (!self.pending_remote_operations.borrow().is_empty()).then(|| {
            Icon::new(IconName::ArrowCircle)
                .size(IconSize::XSmall)
                .color(Color::Info)
                .with_animation(
                    "arrow-circle",
                    Animation::new(Duration::from_secs(2)).repeat(),
                    |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                )
                .into_any_element()
        })
    }

    pub fn can_open_commit_editor(&self) -> bool {
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

    pub fn configure_commit_button(&self, cx: &Context<Self>) -> (bool, &'static str) {
        if self.has_unstaged_conflicts() {
            (false, "You must resolve conflicts before committing")
        } else if !self.has_staged_changes() && !self.has_tracked_changes() {
            (
                false,
                "You must have either staged changes or tracked files to commit",
            )
        } else if self.pending_commit.is_some() {
            (false, "Commit in progress")
        } else if self.suggested_commit_message.is_none()
            && self.commit_editor.read(cx).is_empty(cx)
        {
            (false, "No commit message")
        } else if !self.has_write_access(cx) {
            (false, "You do not have write access to this project")
        } else {
            (true, self.commit_button_title())
        }
    }

    pub fn commit_button_title(&self) -> &'static str {
        if self.has_staged_changes() {
            "Commit"
        } else {
            "Commit Tracked"
        }
    }

    pub fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let project = self.project.clone().read(cx);
        let active_repository = self.active_repository.clone();
        let panel_editor_style = panel_editor_style(true, window, cx);

        if let Some(active_repo) = active_repository {
            let can_open_commit_editor = self.can_open_commit_editor();
            let (can_commit, tooltip) = self.configure_commit_button(cx);

            let enable_coauthors = self.render_co_authors(cx);

            let title = self.commit_button_title();
            let editor_focus_handle = self.commit_editor.focus_handle(cx);

            let branch = active_repo.read(cx).current_branch().cloned();

            let footer_size = px(32.);
            let gap = px(8.0);

            let max_height = window.line_height() * 5. + gap + footer_size;

            let expand_button_size = px(16.);

            let git_panel = cx.entity().clone();
            let display_name = SharedString::from(Arc::from(
                active_repo
                    .read(cx)
                    .display_name(project, cx)
                    .trim_end_matches("/"),
            ));
            let branches = branch_picker::popover(self.project.clone(), window, cx);
            let footer = v_flex()
                .child(PanelRepoFooter::new(
                    "footer-button",
                    display_name,
                    branch,
                    Some(git_panel),
                    Some(branches),
                ))
                .child(
                    panel_editor_container(window, cx)
                        .id("commit-editor-container")
                        .relative()
                        .h(max_height)
                        // .w_full()
                        // .border_t_1()
                        // .border_color(cx.theme().colors().border)
                        .bg(cx.theme().colors().editor_background)
                        .cursor_text()
                        .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                            window.focus(&this.commit_editor.focus_handle(cx));
                        }))
                        .child(
                            h_flex()
                                .id("commit-footer")
                                .absolute()
                                .bottom_0()
                                .right_2()
                                .h(footer_size)
                                .flex_none()
                                .children(enable_coauthors)
                                .child(
                                    panel_filled_button(title)
                                        .tooltip(move |window, cx| {
                                            if can_commit {
                                                Tooltip::for_action_in(
                                                    tooltip,
                                                    &Commit,
                                                    &editor_focus_handle,
                                                    window,
                                                    cx,
                                                )
                                            } else {
                                                Tooltip::simple(tooltip, cx)
                                            }
                                        })
                                        .disabled(!can_commit || self.modal_open)
                                        .on_click({
                                            cx.listener(move |this, _: &ClickEvent, window, cx| {
                                                this.commit_changes(window, cx)
                                            })
                                        }),
                                ),
                        )
                        // .when(!self.modal_open, |el| {
                        .child(EditorElement::new(&self.commit_editor, panel_editor_style))
                        .child(
                            div()
                                .absolute()
                                .top_1()
                                .right_2()
                                .opacity(0.5)
                                .hover(|this| this.opacity(1.0))
                                .w(expand_button_size)
                                .child(
                                    panel_icon_button("expand-commit-editor", IconName::Maximize)
                                        .icon_size(IconSize::Small)
                                        .style(ButtonStyle::Transparent)
                                        .width(expand_button_size.into())
                                        .disabled(!can_open_commit_editor)
                                        .on_click(cx.listener({
                                            move |_, _, window, cx| {
                                                window.dispatch_action(
                                                    git::ShowCommitEditor.boxed_clone(),
                                                    cx,
                                                )
                                            }
                                        })),
                                ),
                        ),
                );

            Some(footer)
        } else {
            None
        }
    }

    fn render_previous_commit(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let active_repository = self.active_repository.as_ref()?;
        let branch = active_repository.read(cx).current_branch()?;
        let commit = branch.most_recent_commit.as_ref()?.clone();

        let this = cx.entity();
        Some(
            h_flex()
                .items_center()
                .py_2()
                .px(px(8.))
                // .bg(cx.theme().colors().background)
                // .border_t_1()
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
                                .truncate(),
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
                    panel_icon_button("undo", IconName::Undo)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .tooltip(Tooltip::for_action_title(
                            if self.has_staged_changes() {
                                "git reset HEAD^ --soft"
                            } else {
                                "git reset HEAD^"
                            },
                            &git::Uncommit,
                        ))
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

    fn render_buffer_header_controls(
        &self,
        entity: &Entity<Self>,
        file: &Arc<dyn File>,
        _: &Window,
        cx: &App,
    ) -> Option<AnyElement> {
        let repo = self.active_repository.as_ref()?.read(cx);
        let repo_path = repo.worktree_id_path_to_repo_path(file.worktree_id(cx), file.path())?;
        let ix = self.entry_by_path(&repo_path)?;
        let entry = self.entries.get(ix)?;

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

        h_flex()
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
                .with_sizing_behavior(ListSizingBehavior::Auto)
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

    fn list_item_height(&self) -> Rems {
        rems(1.75)
    }

    fn render_list_header(
        &self,
        ix: usize,
        header: &GitHeaderEntry,
        _: bool,
        _: &Window,
        _: &Context<Self>,
    ) -> AnyElement {
        let id: ElementId = ElementId::Name(format!("header_{}", ix).into());

        h_flex()
            .id(id)
            .h(self.list_item_height())
            .w_full()
            .items_end()
            .px(rems(0.75)) // ~12px
            .pb(rems(0.3125)) // ~ 5px
            .child(
                Label::new(header.title())
                    .color(Color::Muted)
                    .size(LabelSize::Small)
                    .line_height_style(LineHeightStyle::UiLabel)
                    .single_line(),
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
        repo.update(cx, |repo, cx| {
            let show = repo.show(sha);
            cx.spawn(|_, _| async move { show.await? })
        })
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
        let restore_title = if entry.status.is_created() {
            "Trash File"
        } else {
            "Restore File"
        };
        let context_menu = ContextMenu::build(window, cx, |context_menu, _, _| {
            context_menu
                .action(stage_title, ToggleStaged.boxed_clone())
                .action(restore_title, git::RestoreFile.boxed_clone())
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
        let context_menu = git_panel_context_menu(window, cx);
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
        let marked = self.marked_entries.contains(&ix);
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

        let id: ElementId = ElementId::Name(format!("entry_{}_{}", display_name, ix).into());
        let checkbox_wrapper_id: ElementId =
            ElementId::Name(format!("entry_{}_{}_checkbox_wrapper", display_name, ix).into());
        let checkbox_id: ElementId =
            ElementId::Name(format!("entry_{}_{}_checkbox", display_name, ix).into());

        let is_entry_staged = self.entry_is_staged(entry);
        let mut is_staged: ToggleState = self.entry_is_staged(entry).into();

        if !self.has_staged_changes() && !self.has_conflicts() && !entry.status.is_created() {
            is_staged = ToggleState::Selected;
        }

        let handle = cx.weak_entity();

        let selected_bg_alpha = 0.08;
        let marked_bg_alpha = 0.12;
        let state_opacity_step = 0.04;

        let base_bg = match (selected, marked) {
            (true, true) => cx
                .theme()
                .status()
                .info
                .alpha(selected_bg_alpha + marked_bg_alpha),
            (true, false) => cx.theme().status().info.alpha(selected_bg_alpha),
            (false, true) => cx.theme().status().info.alpha(marked_bg_alpha),
            _ => cx.theme().colors().ghost_element_background,
        };

        let hover_bg = if selected {
            cx.theme()
                .status()
                .info
                .alpha(selected_bg_alpha + state_opacity_step)
        } else {
            cx.theme().colors().ghost_element_hover
        };

        let active_bg = if selected {
            cx.theme()
                .status()
                .info
                .alpha(selected_bg_alpha + state_opacity_step * 2.0)
        } else {
            cx.theme().colors().ghost_element_active
        };

        h_flex()
            .id(id)
            .h(self.list_item_height())
            .w_full()
            .items_center()
            .border_1()
            .when(selected && self.focus_handle.is_focused(window), |el| {
                el.border_color(cx.theme().colors().border_focused)
            })
            .px(rems(0.75)) // ~12px
            .overflow_hidden()
            .flex_none()
            .gap(DynamicSpacing::Base04.rems(cx))
            .bg(base_bg)
            .hover(|this| this.bg(hover_bg))
            .active(|this| this.bg(active_bg))
            .on_click({
                cx.listener(move |this, event: &ClickEvent, window, cx| {
                    this.selected_entry = Some(ix);
                    cx.notify();
                    if event.modifiers().secondary() {
                        this.open_file(&Default::default(), window, cx)
                    } else {
                        this.open_diff(&Default::default(), window, cx);
                        this.focus_handle.focus(window);
                    }
                })
            })
            .on_mouse_down(
                MouseButton::Right,
                move |event: &MouseDownEvent, window, cx| {
                    // why isn't this happening automatically? we are passing MouseButton::Right to `on_mouse_down`?
                    if event.button != MouseButton::Right {
                        return;
                    }

                    let Some(this) = handle.upgrade() else {
                        return;
                    };
                    this.update(cx, |this, cx| {
                        this.deploy_entry_context_menu(event.position, ix, window, cx);
                    });
                    cx.stop_propagation();
                },
            )
            // .on_secondary_mouse_down(cx.listener(
            //     move |this, event: &MouseDownEvent, window, cx| {
            //         this.deploy_entry_context_menu(event.position, ix, window, cx);
            //         cx.stop_propagation();
            //     },
            // ))
            .child(
                div()
                    .id(checkbox_wrapper_id)
                    .flex_none()
                    .occlude()
                    .cursor_pointer()
                    .child(
                        Checkbox::new(checkbox_id, is_staged)
                            .disabled(!has_write_access)
                            .fill()
                            .placeholder(!self.has_staged_changes() && !self.has_conflicts())
                            .elevation(ElevationIndex::Surface)
                            .on_click({
                                let entry = entry.clone();
                                cx.listener(move |this, _, window, cx| {
                                    if !has_write_access {
                                        return;
                                    }
                                    this.toggle_staged_for_entry(
                                        &GitListEntry::GitStatusEntry(entry.clone()),
                                        window,
                                        cx,
                                    );
                                    cx.stop_propagation();
                                })
                            })
                            .tooltip(move |window, cx| {
                                let tooltip_name = if is_entry_staged.unwrap_or(false) {
                                    "Unstage"
                                } else {
                                    "Stage"
                                };

                                Tooltip::for_action(tooltip_name, &ToggleStaged, window, cx)
                            }),
                    ),
            )
            .child(git_status_icon(status, cx))
            .child(
                h_flex()
                    .items_center()
                    .overflow_hidden()
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
            .on_action(cx.listener(Self::select_previous))
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
            .on_action(cx.listener(Self::restore_tracked_files))
            .on_action(cx.listener(Self::clean_all))
            .on_action(cx.listener(Self::fetch))
            .on_action(cx.listener(Self::pull))
            .on_action(cx.listener(Self::push))
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
            .child(
                v_flex()
                    .size_full()
                    .map(|this| {
                        if has_entries {
                            this.child(self.render_entries(has_write_access, window, cx))
                        } else {
                            this.child(self.render_empty_state(cx).into_any_element())
                        }
                    })
                    .children(self.render_footer(window, cx))
                    .children(self.render_previous_commit(cx))
                    .into_any_element(),
            )
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

fn git_action_tooltip(
    label: impl Into<SharedString>,
    action: &dyn Action,
    command: impl Into<SharedString>,
    focus_handle: Option<FocusHandle>,
    window: &mut Window,
    cx: &mut App,
) -> AnyView {
    let label = label.into();
    let command = command.into();

    if let Some(handle) = focus_handle {
        Tooltip::with_meta_in(
            label.clone(),
            Some(action),
            command.clone(),
            &handle,
            window,
            cx,
        )
    } else {
        Tooltip::with_meta(label.clone(), Some(action), command.clone(), window, cx)
    }
}

#[derive(IntoElement)]
struct SplitButton {
    pub left: ButtonLike,
    pub right: AnyElement,
}

impl SplitButton {
    fn new(
        id: impl Into<SharedString>,
        left_label: impl Into<SharedString>,
        ahead_count: usize,
        behind_count: usize,
        left_icon: Option<IconName>,
        left_on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        let id = id.into();

        fn count(count: usize) -> impl IntoElement {
            h_flex()
                .ml_neg_px()
                .h(rems(0.875))
                .items_center()
                .overflow_hidden()
                .px_0p5()
                .child(
                    Label::new(count.to_string())
                        .size(LabelSize::XSmall)
                        .line_height_style(LineHeightStyle::UiLabel),
                )
        }

        let should_render_counts = left_icon.is_none() && (ahead_count > 0 || behind_count > 0);

        let left = ui::ButtonLike::new_rounded_left(ElementId::Name(
            format!("split-button-left-{}", id).into(),
        ))
        .layer(ui::ElevationIndex::ModalSurface)
        .size(ui::ButtonSize::Compact)
        .when(should_render_counts, |this| {
            this.child(
                h_flex()
                    .ml_neg_0p5()
                    .mr_1()
                    .when(behind_count > 0, |this| {
                        this.child(Icon::new(IconName::ArrowDown).size(IconSize::XSmall))
                            .child(count(behind_count))
                    })
                    .when(ahead_count > 0, |this| {
                        this.child(Icon::new(IconName::ArrowUp).size(IconSize::XSmall))
                            .child(count(ahead_count))
                    }),
            )
        })
        .when_some(left_icon, |this, left_icon| {
            this.child(
                h_flex()
                    .ml_neg_0p5()
                    .mr_1()
                    .child(Icon::new(left_icon).size(IconSize::XSmall)),
            )
        })
        .child(
            div()
                .child(Label::new(left_label).size(LabelSize::Small))
                .mr_0p5(),
        )
        .on_click(left_on_click)
        .tooltip(tooltip);

        let right =
            render_git_action_menu(ElementId::Name(format!("split-button-right-{}", id).into()))
                .into_any_element();
        // .on_click(right_on_click);

        Self { left, right }
    }
}

impl RenderOnce for SplitButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().colors().text_muted.alpha(0.12))
            .child(self.left)
            .child(
                div()
                    .h_full()
                    .w_px()
                    .bg(cx.theme().colors().text_muted.alpha(0.16)),
            )
            .child(self.right)
            .bg(ElevationIndex::Surface.on_elevation_bg(cx))
            .shadow(smallvec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.16),
                offset: point(px(0.), px(1.)),
                blur_radius: px(0.),
                spread_radius: px(0.),
            }])
    }
}

fn render_git_action_menu(id: impl Into<ElementId>) -> impl IntoElement {
    PopoverMenu::new(id.into())
        .trigger(
            ui::ButtonLike::new_rounded_right("split-button-right")
                .layer(ui::ElevationIndex::ModalSurface)
                .size(ui::ButtonSize::None)
                .child(
                    div()
                        .px_1()
                        .child(Icon::new(IconName::ChevronDownSmall).size(IconSize::XSmall)),
                ),
        )
        .menu(move |window, cx| {
            Some(ContextMenu::build(window, cx, |context_menu, _, _| {
                context_menu
                    .action("Fetch", git::Fetch.boxed_clone())
                    .action("Pull", git::Pull.boxed_clone())
                    .separator()
                    .action("Push", git::Push { options: None }.boxed_clone())
                    .action(
                        "Force Push",
                        git::Push {
                            options: Some(PushOptions::Force),
                        }
                        .boxed_clone(),
                    )
            }))
        })
        .anchor(Corner::TopRight)
}

#[derive(IntoElement, IntoComponent)]
#[component(scope = "git_panel")]
pub struct PanelRepoFooter {
    id: SharedString,
    active_repository: SharedString,
    branch: Option<Branch>,
    // Getting a GitPanel in previews will be difficult.
    //
    // For now just take an option here, and we won't bind handlers to buttons in previews.
    git_panel: Option<Entity<GitPanel>>,
    branches: Option<Entity<BranchList>>,
}

impl PanelRepoFooter {
    pub fn new(
        id: impl Into<SharedString>,
        active_repository: SharedString,
        branch: Option<Branch>,
        git_panel: Option<Entity<GitPanel>>,
        branches: Option<Entity<BranchList>>,
    ) -> Self {
        Self {
            id: id.into(),
            active_repository,
            branch,
            git_panel,
            branches,
        }
    }

    pub fn new_preview(
        id: impl Into<SharedString>,
        active_repository: SharedString,
        branch: Option<Branch>,
    ) -> Self {
        Self {
            id: id.into(),
            active_repository,
            branch,
            git_panel: None,
            branches: None,
        }
    }

    fn render_overflow_menu(&self, id: impl Into<ElementId>) -> impl IntoElement {
        PopoverMenu::new(id.into())
            .trigger(
                IconButton::new("overflow-menu-trigger", IconName::EllipsisVertical)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted),
            )
            .menu(move |window, cx| Some(git_panel_context_menu(window, cx)))
            .anchor(Corner::TopRight)
    }

    fn panel_focus_handle(&self, cx: &App) -> Option<FocusHandle> {
        if let Some(git_panel) = self.git_panel.clone() {
            Some(git_panel.focus_handle(cx))
        } else {
            None
        }
    }

    fn render_push_button(&self, id: SharedString, ahead: u32, cx: &mut App) -> SplitButton {
        let panel = self.git_panel.clone();
        let panel_focus_handle = self.panel_focus_handle(cx);

        SplitButton::new(
            id,
            "Push",
            ahead as usize,
            0,
            None,
            move |_, window, cx| {
                if let Some(panel) = panel.as_ref() {
                    panel.update(cx, |panel, cx| {
                        panel.push(&git::Push { options: None }, window, cx);
                    });
                }
            },
            move |window, cx| {
                git_action_tooltip(
                    "Push committed changes to remote",
                    &git::Push { options: None },
                    "git push",
                    panel_focus_handle.clone(),
                    window,
                    cx,
                )
            },
        )
    }

    fn render_pull_button(
        &self,
        id: SharedString,
        ahead: u32,
        behind: u32,
        cx: &mut App,
    ) -> SplitButton {
        let panel = self.git_panel.clone();
        let panel_focus_handle = self.panel_focus_handle(cx);

        SplitButton::new(
            id,
            "Pull",
            ahead as usize,
            behind as usize,
            None,
            move |_, window, cx| {
                if let Some(panel) = panel.as_ref() {
                    panel.update(cx, |panel, cx| {
                        panel.pull(&git::Pull, window, cx);
                    });
                }
            },
            move |window, cx| {
                git_action_tooltip(
                    "Pull",
                    &git::Pull,
                    "git pull",
                    panel_focus_handle.clone(),
                    window,
                    cx,
                )
            },
        )
    }

    fn render_fetch_button(&self, id: SharedString, cx: &mut App) -> SplitButton {
        let panel = self.git_panel.clone();
        let panel_focus_handle = self.panel_focus_handle(cx);

        SplitButton::new(
            id,
            "Fetch",
            0,
            0,
            Some(IconName::ArrowCircle),
            move |_, window, cx| {
                if let Some(panel) = panel.as_ref() {
                    panel.update(cx, |panel, cx| {
                        panel.fetch(&git::Fetch, window, cx);
                    });
                }
            },
            move |window, cx| {
                git_action_tooltip(
                    "Fetch updates from remote",
                    &git::Fetch,
                    "git fetch",
                    panel_focus_handle.clone(),
                    window,
                    cx,
                )
            },
        )
    }

    fn render_publish_button(&self, id: SharedString, cx: &mut App) -> SplitButton {
        let panel = self.git_panel.clone();
        let panel_focus_handle = self.panel_focus_handle(cx);

        SplitButton::new(
            id,
            "Publish",
            0,
            0,
            Some(IconName::ArrowUpFromLine),
            move |_, window, cx| {
                if let Some(panel) = panel.as_ref() {
                    panel.update(cx, |panel, cx| {
                        panel.push(
                            &git::Push {
                                options: Some(PushOptions::SetUpstream),
                            },
                            window,
                            cx,
                        );
                    });
                }
            },
            move |window, cx| {
                git_action_tooltip(
                    "Publish branch to remote",
                    &git::Push {
                        options: Some(PushOptions::SetUpstream),
                    },
                    "git push --set-upstream",
                    panel_focus_handle.clone(),
                    window,
                    cx,
                )
            },
        )
    }

    fn render_republish_button(&self, id: SharedString, cx: &mut App) -> SplitButton {
        let panel = self.git_panel.clone();
        let panel_focus_handle = self.panel_focus_handle(cx);

        SplitButton::new(
            id,
            "Republish",
            0,
            0,
            Some(IconName::ArrowUpFromLine),
            move |_, window, cx| {
                if let Some(panel) = panel.as_ref() {
                    panel.update(cx, |panel, cx| {
                        panel.push(
                            &git::Push {
                                options: Some(PushOptions::SetUpstream),
                            },
                            window,
                            cx,
                        );
                    });
                }
            },
            move |window, cx| {
                git_action_tooltip(
                    "Re-publish branch to remote",
                    &git::Push {
                        options: Some(PushOptions::SetUpstream),
                    },
                    "git push --set-upstream",
                    panel_focus_handle.clone(),
                    window,
                    cx,
                )
            },
        )
    }

    fn render_relevant_button(
        &self,
        id: impl Into<SharedString>,
        branch: &Branch,
        cx: &mut App,
    ) -> impl IntoElement {
        let id = id.into();
        let upstream = branch.upstream.as_ref();
        match upstream {
            Some(Upstream {
                tracking: UpstreamTracking::Tracked(UpstreamTrackingStatus { ahead, behind }),
                ..
            }) => match (*ahead, *behind) {
                (0, 0) => self.render_fetch_button(id, cx),
                (ahead, 0) => self.render_push_button(id, ahead, cx),
                (ahead, behind) => self.render_pull_button(id, ahead, behind, cx),
            },
            Some(Upstream {
                tracking: UpstreamTracking::Gone,
                ..
            }) => self.render_republish_button(id, cx),
            None => self.render_publish_button(id, cx),
        }
    }
}

impl RenderOnce for PanelRepoFooter {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let active_repo = self.active_repository.clone();
        let overflow_menu_id: SharedString = format!("overflow-menu-{}", active_repo).into();
        let repo_selector_trigger = Button::new("repo-selector", active_repo)
            .style(ButtonStyle::Transparent)
            .size(ButtonSize::None)
            .label_size(LabelSize::Small)
            .color(Color::Muted);

        let repo_selector = if let Some(panel) = self.git_panel.clone() {
            let repo_selector = panel.read(cx).repository_selector.clone();
            let repo_count = repo_selector.read(cx).repositories_len(cx);
            let single_repo = repo_count == 1;

            RepositorySelectorPopoverMenu::new(
                panel.read(cx).repository_selector.clone(),
                repo_selector_trigger.disabled(single_repo).truncate(true),
                Tooltip::text("Switch active repository"),
            )
            .into_any_element()
        } else {
            // for rendering preview, we don't have git_panel there
            repo_selector_trigger.into_any_element()
        };

        let branch = self.branch.clone();
        let branch_name = branch
            .as_ref()
            .map_or(" (no branch)".into(), |branch| branch.name.clone());

        let branches = self.branches.clone();

        let branch_selector_button = Button::new("branch-selector", branch_name)
            .style(ButtonStyle::Transparent)
            .size(ButtonSize::None)
            .label_size(LabelSize::Small)
            .truncate(true)
            .tooltip(Tooltip::for_action_title(
                "Switch Branch",
                &zed_actions::git::Branch,
            ))
            .on_click(|_, window, cx| {
                window.dispatch_action(zed_actions::git::Branch.boxed_clone(), cx);
            });

        let branch_selector = if let Some(branches) = branches {
            PopoverButton::new(
                branches,
                Corner::BottomLeft,
                branch_selector_button,
                Tooltip::for_action_title("Switch Branch", &zed_actions::git::Branch),
            )
            .render(window, cx)
            .into_any_element()
        } else {
            branch_selector_button.into_any_element()
        };

        let spinner = self
            .git_panel
            .as_ref()
            .and_then(|git_panel| git_panel.read(cx).render_spinner());

        h_flex()
            .w_full()
            .px_2()
            .h(px(36.))
            .items_center()
            .justify_between()
            .child(
                h_flex()
                    .flex_1()
                    .overflow_hidden()
                    .items_center()
                    .child(
                        div().child(
                            Icon::new(IconName::GitBranchSmall)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .child(repo_selector)
                    .when_some(branch.clone(), |this, _| {
                        this.child(
                            div()
                                .text_color(cx.theme().colors().text_muted)
                                .text_sm()
                                .child("/"),
                        )
                    })
                    .child(branch_selector),
            )
            .child(
                h_flex()
                    .gap_1()
                    .flex_shrink_0()
                    .children(spinner)
                    .child(self.render_overflow_menu(overflow_menu_id))
                    .when_some(branch, |this, branch| {
                        let button = self.render_relevant_button(self.id.clone(), &branch, cx);
                        this.child(button)
                    }),
            )
    }
}

impl ComponentPreview for PanelRepoFooter {
    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
        let unknown_upstream = None;
        let no_remote_upstream = Some(UpstreamTracking::Gone);
        let ahead_of_upstream = Some(
            UpstreamTrackingStatus {
                ahead: 2,
                behind: 0,
            }
            .into(),
        );
        let behind_upstream = Some(
            UpstreamTrackingStatus {
                ahead: 0,
                behind: 2,
            }
            .into(),
        );
        let ahead_and_behind_upstream = Some(
            UpstreamTrackingStatus {
                ahead: 3,
                behind: 1,
            }
            .into(),
        );

        let not_ahead_or_behind_upstream = Some(
            UpstreamTrackingStatus {
                ahead: 0,
                behind: 0,
            }
            .into(),
        );

        fn branch(upstream: Option<UpstreamTracking>) -> Branch {
            Branch {
                is_head: true,
                name: "some-branch".into(),
                upstream: upstream.map(|tracking| Upstream {
                    ref_name: "origin/some-branch".into(),
                    tracking,
                }),
                most_recent_commit: Some(CommitSummary {
                    sha: "abc123".into(),
                    subject: "Modify stuff".into(),
                    commit_timestamp: 1710932954,
                }),
            }
        }

        fn custom(branch_name: &str, upstream: Option<UpstreamTracking>) -> Branch {
            Branch {
                is_head: true,
                name: branch_name.to_string().into(),
                upstream: upstream.map(|tracking| Upstream {
                    ref_name: format!("zed/{}", branch_name).into(),
                    tracking,
                }),
                most_recent_commit: Some(CommitSummary {
                    sha: "abc123".into(),
                    subject: "Modify stuff".into(),
                    commit_timestamp: 1710932954,
                }),
            }
        }

        fn active_repository(id: usize) -> SharedString {
            format!("repo-{}", id).into()
        }

        let example_width = px(340.);

        v_flex()
            .gap_6()
            .w_full()
            .flex_none()
            .children(vec![example_group_with_title(
                "Action Button States",
                vec![
                    single_example(
                        "No Branch",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "no-branch",
                                active_repository(1).clone(),
                                None,
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                    single_example(
                        "Remote status unknown",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "unknown-upstream",
                                active_repository(2).clone(),
                                Some(branch(unknown_upstream)),
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                    single_example(
                        "No Remote Upstream",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "no-remote-upstream",
                                active_repository(3).clone(),
                                Some(branch(no_remote_upstream)),
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                    single_example(
                        "Not Ahead or Behind",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "not-ahead-or-behind",
                                active_repository(4).clone(),
                                Some(branch(not_ahead_or_behind_upstream)),
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                    single_example(
                        "Behind remote",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "behind-remote",
                                active_repository(5).clone(),
                                Some(branch(behind_upstream)),
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                    single_example(
                        "Ahead of remote",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "ahead-of-remote",
                                active_repository(6).clone(),
                                Some(branch(ahead_of_upstream)),
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                    single_example(
                        "Ahead and behind remote",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "ahead-and-behind",
                                active_repository(7).clone(),
                                Some(branch(ahead_and_behind_upstream)),
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                ],
            )
            .grow()
            .vertical()])
            .children(vec![example_group_with_title(
                "Labels",
                vec![
                    single_example(
                        "Short Branch & Repo",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "short-branch",
                                SharedString::from("zed"),
                                Some(custom("main", behind_upstream)),
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                    single_example(
                        "Long Branch",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "long-branch",
                                SharedString::from("zed"),
                                Some(custom(
                                    "redesign-and-update-git-ui-list-entry-style",
                                    behind_upstream,
                                )),
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                    single_example(
                        "Long Repo",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "long-repo",
                                SharedString::from("zed-industries-community-examples"),
                                Some(custom("gpui", ahead_of_upstream)),
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                    single_example(
                        "Long Repo & Branch",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "long-repo-and-branch",
                                SharedString::from("zed-industries-community-examples"),
                                Some(custom(
                                    "redesign-and-update-git-ui-list-entry-style",
                                    behind_upstream,
                                )),
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                    single_example(
                        "Uppercase Repo",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "uppercase-repo",
                                SharedString::from("LICENSES"),
                                Some(custom("main", ahead_of_upstream)),
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                    single_example(
                        "Uppercase Branch",
                        div()
                            .w(example_width)
                            .overflow_hidden()
                            .child(PanelRepoFooter::new_preview(
                                "uppercase-branch",
                                SharedString::from("zed"),
                                Some(custom("update-README", behind_upstream)),
                            ))
                            .into_any_element(),
                    )
                    .grow(),
                ],
            )
            .grow()
            .vertical()])
            .into_any_element()
    }
}
