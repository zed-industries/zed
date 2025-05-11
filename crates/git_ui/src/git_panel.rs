use crate::askpass_modal::AskPassModal;
use crate::commit_modal::CommitModal;
use crate::commit_tooltip::CommitTooltip;
use crate::commit_view::CommitView;
use crate::git_panel_settings::StatusStyle;
use crate::project_diff::{self, Diff, ProjectDiff};
use crate::remote_output::{self, RemoteAction, SuccessMessage};
use crate::{branch_picker, picker_prompt, render_remote_button};
use crate::{
    git_panel_settings::GitPanelSettings, git_status_icon, repository_selector::RepositorySelector,
};
use anyhow::Result;
use askpass::AskPassDelegate;
use assistant_settings::AssistantSettings;
use db::kvp::KEY_VALUE_STORE;

use editor::{
    Editor, EditorElement, EditorMode, EditorSettings, MultiBuffer, ShowScrollbar,
    scroll::ScrollbarAutoHide,
};
use futures::StreamExt as _;
use git::blame::ParsedCommitMessage;
use git::repository::{
    Branch, CommitDetails, CommitOptions, CommitSummary, DiffType, PushOptions, Remote,
    RemoteCommandOutput, ResetMode, Upstream, UpstreamTracking, UpstreamTrackingStatus,
};
use git::status::StageStatus;
use git::{Amend, ToggleStaged, repository::RepoPath, status::FileStatus};
use git::{ExpandCommitEditor, RestoreTrackedFiles, StageAll, TrashUntrackedFiles, UnstageAll};
use gpui::{
    Action, Animation, AnimationExt as _, Axis, ClickEvent, Corner, DismissEvent, Entity,
    EventEmitter, FocusHandle, Focusable, KeyContext, ListHorizontalSizingBehavior,
    ListSizingBehavior, Modifiers, ModifiersChangedEvent, MouseButton, MouseDownEvent, Point,
    PromptLevel, ScrollStrategy, Subscription, Task, Transformation, UniformListScrollHandle,
    WeakEntity, actions, anchored, deferred, percentage, uniform_list,
};
use itertools::Itertools;
use language::language_settings::SoftWrap;
use language::{Buffer, File};
use language_model::{
    ConfiguredModel, LanguageModel, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, Role,
};
use menu::{Confirm, SecondaryConfirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use multi_buffer::ExcerptInfo;
use panel::{
    PanelHeader, panel_button, panel_editor_container, panel_editor_style, panel_filled_button,
    panel_icon_button,
};
use project::git_store::RepositoryEvent;
use project::{
    Fs, Project, ProjectPath,
    git_store::{GitStoreEvent, Repository},
};
use serde::{Deserialize, Serialize};
use settings::{Settings as _, SettingsStore};
use std::future::Future;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::{collections::HashSet, sync::Arc, time::Duration, usize};
use strum::{IntoEnumIterator, VariantNames};
use time::OffsetDateTime;
use ui::{
    Checkbox, ContextMenu, ElevationIndex, PopoverMenu, Scrollbar, ScrollbarState, SplitButton,
    Tooltip, prelude::*,
};
use util::{ResultExt, TryFutureExt, maybe, wrap_with_prefix};
use workspace::AppState;

use notifications::status_toast::{StatusToast, ToastIcon};
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
    notifications::DetachAndPromptErr,
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
        GenerateCommitMessage
    ]
);

fn prompt<T>(
    msg: &str,
    detail: Option<&str>,
    window: &mut Window,
    cx: &mut App,
) -> Task<anyhow::Result<T>>
where
    T: IntoEnumIterator + VariantNames + 'static,
{
    let rx = window.prompt(PromptLevel::Info, msg, detail, &T::VARIANTS, cx);
    cx.spawn(async move |_| Ok(T::iter().nth(rx.await?).unwrap()))
}

#[derive(strum::EnumIter, strum::VariantNames)]
#[strum(serialize_all = "title_case")]
enum TrashCancel {
    Trash,
    Cancel,
}

struct GitMenuState {
    has_tracked_changes: bool,
    has_staged_changes: bool,
    has_unstaged_changes: bool,
    has_new_changes: bool,
}

fn git_panel_context_menu(
    focus_handle: FocusHandle,
    state: GitMenuState,
    window: &mut Window,
    cx: &mut App,
) -> Entity<ContextMenu> {
    ContextMenu::build(window, cx, move |context_menu, _, _| {
        context_menu
            .context(focus_handle)
            .map(|menu| {
                if state.has_unstaged_changes {
                    menu.action("Stage All", StageAll.boxed_clone())
                } else {
                    menu.disabled_action("Stage All", StageAll.boxed_clone())
                }
            })
            .map(|menu| {
                if state.has_staged_changes {
                    menu.action("Unstage All", UnstageAll.boxed_clone())
                } else {
                    menu.disabled_action("Unstage All", UnstageAll.boxed_clone())
                }
            })
            .separator()
            .action("Open Diff", project_diff::Diff.boxed_clone())
            .separator()
            .map(|menu| {
                if state.has_tracked_changes {
                    menu.action("Discard Tracked Changes", RestoreTrackedFiles.boxed_clone())
                } else {
                    menu.disabled_action(
                        "Discard Tracked Changes",
                        RestoreTrackedFiles.boxed_clone(),
                    )
                }
            })
            .map(|menu| {
                if state.has_new_changes {
                    menu.action("Trash Untracked Files", TrashUntrackedFiles.boxed_clone())
                } else {
                    menu.disabled_action("Trash Untracked Files", TrashUntrackedFiles.boxed_clone())
                }
            })
    })
}

const GIT_PANEL_KEY: &str = "GitPanel";

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
        workspace.toggle_panel_focus::<GitPanel>(window, cx);
    });
    workspace.register_action(|workspace, _: &ExpandCommitEditor, window, cx| {
        CommitModal::toggle(workspace, None, window, cx)
    });
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
    pub(crate) abs_path: PathBuf,
    pub(crate) status: FileStatus,
    pub(crate) staging: StageStatus,
}

impl GitStatusEntry {
    fn display_name(&self) -> String {
        self.repo_path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| self.repo_path.to_string_lossy().into_owned())
    }

    fn parent_dir(&self) -> Option<String> {
        self.repo_path
            .parent()
            .map(|parent| parent.to_string_lossy().into_owned())
    }
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
    entries: Vec<GitStatusEntry>,
    op_id: usize,
}

// computed state related to how to render scrollbars
// one per axis
// on render we just read this off the panel
// we update it when
// - settings change
// - on focus in, on focus out, on hover, etc.
#[derive(Debug)]
struct ScrollbarProperties {
    axis: Axis,
    show_scrollbar: bool,
    show_track: bool,
    auto_hide: bool,
    hide_task: Option<Task<()>>,
    state: ScrollbarState,
}

impl ScrollbarProperties {
    // Shows the scrollbar and cancels any pending hide task
    fn show(&mut self, cx: &mut Context<GitPanel>) {
        if !self.auto_hide {
            return;
        }
        self.show_scrollbar = true;
        self.hide_task.take();
        cx.notify();
    }

    fn hide(&mut self, window: &mut Window, cx: &mut Context<GitPanel>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);

        if !self.auto_hide {
            return;
        }

        let axis = self.axis;
        self.hide_task = Some(cx.spawn_in(window, async move |panel, cx| {
            cx.background_executor()
                .timer(SCROLLBAR_SHOW_INTERVAL)
                .await;

            if let Some(panel) = panel.upgrade() {
                panel
                    .update(cx, |panel, cx| {
                        match axis {
                            Axis::Vertical => panel.vertical_scrollbar.show_scrollbar = false,
                            Axis::Horizontal => panel.horizontal_scrollbar.show_scrollbar = false,
                        }
                        cx.notify();
                    })
                    .log_err();
            }
        }));
    }
}

pub struct GitPanel {
    pub(crate) active_repository: Option<Entity<Repository>>,
    pub(crate) commit_editor: Entity<Editor>,
    conflicted_count: usize,
    conflicted_staged_count: usize,
    current_modifiers: Modifiers,
    add_coauthors: bool,
    generate_commit_message_task: Option<Task<Option<()>>>,
    entries: Vec<GitListEntry>,
    single_staged_entry: Option<GitStatusEntry>,
    single_tracked_entry: Option<GitStatusEntry>,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    horizontal_scrollbar: ScrollbarProperties,
    vertical_scrollbar: ScrollbarProperties,
    new_count: usize,
    entry_count: usize,
    new_staged_count: usize,
    pending: Vec<PendingOperation>,
    pending_commit: Option<Task<()>>,
    amend_pending: bool,
    pending_serialization: Task<Option<()>>,
    pub(crate) project: Entity<Project>,
    scroll_handle: UniformListScrollHandle,
    max_width_item_index: Option<usize>,
    selected_entry: Option<usize>,
    marked_entries: Vec<usize>,
    tracked_count: usize,
    tracked_staged_count: usize,
    update_visible_entries_task: Task<()>,
    width: Option<Pixels>,
    workspace: WeakEntity<Workspace>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    modal_open: bool,
    show_placeholders: bool,
    _settings_subscription: Subscription,
}

const MAX_PANEL_EDITOR_LINES: usize = 6;

pub(crate) fn commit_message_editor(
    commit_message_buffer: Entity<Buffer>,
    placeholder: Option<SharedString>,
    project: Entity<Project>,
    in_panel: bool,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> Editor {
    let buffer = cx.new(|cx| MultiBuffer::singleton(commit_message_buffer, cx));
    let max_lines = if in_panel { MAX_PANEL_EDITOR_LINES } else { 18 };
    let mut commit_editor = Editor::new(
        EditorMode::AutoHeight { max_lines },
        buffer,
        None,
        window,
        cx,
    );
    commit_editor.set_collaboration_hub(Box::new(project));
    commit_editor.set_use_autoclose(false);
    commit_editor.set_show_gutter(false, cx);
    commit_editor.set_show_wrap_guides(false, cx);
    commit_editor.set_show_indent_guides(false, cx);
    let placeholder = placeholder.unwrap_or("Enter commit message".into());
    commit_editor.set_placeholder_text(placeholder, cx);
    commit_editor
}

impl GitPanel {
    pub fn new(
        workspace: Entity<Workspace>,
        project: Entity<Project>,
        app_state: Arc<AppState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let fs = app_state.fs.clone();
        let git_store = project.read(cx).git_store().clone();
        let active_repository = project.read(cx).active_repository(cx);
        let workspace = workspace.downgrade();

        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, window, Self::focus_in).detach();
        cx.on_focus_out(&focus_handle, window, |this, _, window, cx| {
            this.hide_scrollbars(window, cx);
        })
        .detach();

        let mut was_sort_by_path = GitPanelSettings::get_global(cx).sort_by_path;
        cx.observe_global::<SettingsStore>(move |this, cx| {
            let is_sort_by_path = GitPanelSettings::get_global(cx).sort_by_path;
            if is_sort_by_path != was_sort_by_path {
                this.update_visible_entries(cx);
            }
            was_sort_by_path = is_sort_by_path
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
                GitStoreEvent::ActiveRepositoryChanged(_) => {
                    this.active_repository = git_store.read(cx).active_repository();
                    this.schedule_update(true, window, cx);
                }
                GitStoreEvent::RepositoryUpdated(
                    _,
                    RepositoryEvent::Updated { full_scan },
                    true,
                ) => {
                    this.schedule_update(*full_scan, window, cx);
                }

                GitStoreEvent::RepositoryAdded(_) | GitStoreEvent::RepositoryRemoved(_) => {
                    this.schedule_update(false, window, cx);
                }
                GitStoreEvent::IndexWriteError(error) => {
                    this.workspace
                        .update(cx, |workspace, cx| {
                            workspace.show_error(error, cx);
                        })
                        .ok();
                }
                GitStoreEvent::RepositoryUpdated(_, _, _) => {}
                GitStoreEvent::JobsUpdated | GitStoreEvent::ConflictsUpdated => {}
            },
        )
        .detach();

        let vertical_scrollbar = ScrollbarProperties {
            axis: Axis::Vertical,
            state: ScrollbarState::new(scroll_handle.clone()).parent_entity(&cx.entity()),
            show_scrollbar: false,
            show_track: false,
            auto_hide: false,
            hide_task: None,
        };

        let horizontal_scrollbar = ScrollbarProperties {
            axis: Axis::Horizontal,
            state: ScrollbarState::new(scroll_handle.clone()).parent_entity(&cx.entity()),
            show_scrollbar: false,
            show_track: false,
            auto_hide: false,
            hide_task: None,
        };

        let mut assistant_enabled = AssistantSettings::get_global(cx).enabled;
        let _settings_subscription = cx.observe_global::<SettingsStore>(move |_, cx| {
            if assistant_enabled != AssistantSettings::get_global(cx).enabled {
                assistant_enabled = AssistantSettings::get_global(cx).enabled;
                cx.notify();
            }
        });

        let mut git_panel = Self {
            active_repository,
            commit_editor,
            conflicted_count: 0,
            conflicted_staged_count: 0,
            current_modifiers: window.modifiers(),
            add_coauthors: true,
            generate_commit_message_task: None,
            entries: Vec::new(),
            focus_handle: cx.focus_handle(),
            fs,
            new_count: 0,
            new_staged_count: 0,
            pending: Vec::new(),
            pending_commit: None,
            amend_pending: false,
            pending_serialization: Task::ready(None),
            single_staged_entry: None,
            single_tracked_entry: None,
            project,
            scroll_handle,
            max_width_item_index: None,
            selected_entry: None,
            marked_entries: Vec::new(),
            tracked_count: 0,
            tracked_staged_count: 0,
            update_visible_entries_task: Task::ready(()),
            width: None,
            show_placeholders: false,
            context_menu: None,
            workspace,
            modal_open: false,
            entry_count: 0,
            horizontal_scrollbar,
            vertical_scrollbar,
            _settings_subscription,
        };
        git_panel.schedule_update(false, window, cx);
        git_panel
    }

    fn hide_scrollbars(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.horizontal_scrollbar.hide(window, cx);
        self.vertical_scrollbar.hide(window, cx);
    }

    fn update_scrollbar_properties(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        // TODO: This PR should have defined Editor's `scrollbar.axis`
        // as an Option<ScrollbarAxis>, not a ScrollbarAxes as it would allow you to
        // `.unwrap_or(EditorSettings::get_global(cx).scrollbar.show)`.
        //
        // Once this is fixed we can extend the GitPanelSettings with a `scrollbar.axis`
        // so we can show each axis based on the settings.
        //
        // We should fix this. PR: https://github.com/zed-industries/zed/pull/19495

        let show_setting = GitPanelSettings::get_global(cx)
            .scrollbar
            .show
            .unwrap_or(EditorSettings::get_global(cx).scrollbar.show);

        let scroll_handle = self.scroll_handle.0.borrow();

        let autohide = |show: ShowScrollbar, cx: &mut Context<Self>| match show {
            ShowScrollbar::Auto => true,
            ShowScrollbar::System => cx
                .try_global::<ScrollbarAutoHide>()
                .map_or_else(|| cx.should_auto_hide_scrollbars(), |autohide| autohide.0),
            ShowScrollbar::Always => false,
            ShowScrollbar::Never => false,
        };

        let longest_item_width = scroll_handle.last_item_size.and_then(|size| {
            (size.contents.width > size.item.width).then_some(size.contents.width)
        });

        // is there an item long enough that we should show a horizontal scrollbar?
        let item_wider_than_container = if let Some(longest_item_width) = longest_item_width {
            longest_item_width > px(scroll_handle.base_handle.bounds().size.width.0)
        } else {
            true
        };

        let show_horizontal = match (show_setting, item_wider_than_container) {
            (_, false) => false,
            (ShowScrollbar::Auto | ShowScrollbar::System | ShowScrollbar::Always, true) => true,
            (ShowScrollbar::Never, true) => false,
        };

        let show_vertical = match show_setting {
            ShowScrollbar::Auto | ShowScrollbar::System | ShowScrollbar::Always => true,
            ShowScrollbar::Never => false,
        };

        let show_horizontal_track =
            show_horizontal && matches!(show_setting, ShowScrollbar::Always);

        // TODO: we probably should hide the scroll track when the list doesn't need to scroll
        let show_vertical_track = show_vertical && matches!(show_setting, ShowScrollbar::Always);

        self.vertical_scrollbar = ScrollbarProperties {
            axis: self.vertical_scrollbar.axis,
            state: self.vertical_scrollbar.state.clone(),
            show_scrollbar: show_vertical,
            show_track: show_vertical_track,
            auto_hide: autohide(show_setting, cx),
            hide_task: None,
        };

        self.horizontal_scrollbar = ScrollbarProperties {
            axis: self.horizontal_scrollbar.axis,
            state: self.horizontal_scrollbar.state.clone(),
            show_scrollbar: show_horizontal,
            show_track: show_horizontal_track,
            auto_hide: autohide(show_setting, cx),
            hide_task: None,
        };

        cx.notify();
    }

    pub fn entry_by_path(&self, path: &RepoPath, cx: &App) -> Option<usize> {
        if GitPanelSettings::get_global(cx).sort_by_path {
            return self
                .entries
                .binary_search_by(|entry| entry.status_entry().unwrap().repo_path.cmp(&path))
                .ok();
        }

        if self.conflicted_count > 0 {
            let conflicted_start = 1;
            if let Ok(ix) = self.entries[conflicted_start..conflicted_start + self.conflicted_count]
                .binary_search_by(|entry| entry.status_entry().unwrap().repo_path.cmp(&path))
            {
                return Some(conflicted_start + ix);
            }
        }
        if self.tracked_count > 0 {
            let tracked_start = if self.conflicted_count > 0 {
                1 + self.conflicted_count
            } else {
                0
            } + 1;
            if let Ok(ix) = self.entries[tracked_start..tracked_start + self.tracked_count]
                .binary_search_by(|entry| entry.status_entry().unwrap().repo_path.cmp(&path))
            {
                return Some(tracked_start + ix);
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
            if let Ok(ix) = self.entries[untracked_start..untracked_start + self.new_count]
                .binary_search_by(|entry| entry.status_entry().unwrap().repo_path.cmp(&path))
            {
                return Some(untracked_start + ix);
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
        let Some(repo_path) = git_repo.read(cx).project_path_to_repo_path(&path, cx) else {
            return;
        };
        let Some(ix) = self.entry_by_path(&repo_path, cx) else {
            return;
        };
        self.selected_entry = Some(ix);
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

        if window
            .focused(cx)
            .map_or(false, |focused| self.focus_handle == focused)
        {
            dispatch_context.add("menu");
            dispatch_context.add("ChangesList");
        }

        if self.commit_editor.read(cx).is_focused(window) {
            dispatch_context.add("CommitEditor");
        }

        dispatch_context
    }

    fn close_panel(&mut self, _: &Close, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(PanelEvent::Close);
    }

    fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_handle.contains_focused(window, cx) {
            cx.emit(Event::Focus);
        }
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
                active_repository.read(cx).status_summary().count > 0
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
                            .project_path_to_repo_path(&project_path, cx)
                            .as_ref()
                    {
                        project_diff.focus_handle(cx).focus(window);
                        project_diff.update(cx, |project_diff, cx| project_diff.autoscroll(cx));
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
                .repo_path_to_project_path(&entry.repo_path, cx)?;
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
        action: &git::RestoreFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let list_entry = self.entries.get(self.selected_entry?)?.clone();
            let entry = list_entry.status_entry()?.to_owned();
            let skip_prompt = action.skip_prompt || entry.status.is_created();

            let prompt = if skip_prompt {
                Task::ready(Ok(0))
            } else {
                let prompt = window.prompt(
                    PromptLevel::Warning,
                    &format!(
                        "Are you sure you want to restore {}?",
                        entry
                            .repo_path
                            .file_name()
                            .unwrap_or(entry.repo_path.as_os_str())
                            .to_string_lossy()
                    ),
                    None,
                    &["Restore", "Cancel"],
                    cx,
                );
                cx.background_spawn(prompt)
            };

            let this = cx.weak_entity();
            window
                .spawn(cx, async move |cx| {
                    if prompt.await? != 0 {
                        return anyhow::Ok(());
                    }

                    this.update_in(cx, |this, window, cx| {
                        this.revert_entry(&entry, window, cx);
                    })?;

                    Ok(())
                })
                .detach();
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
                .repo_path_to_project_path(&entry.repo_path, cx)?;
            let workspace = self.workspace.clone();

            if entry.status.staging().has_staged() {
                self.change_file_stage(false, vec![entry.clone()], cx);
            }
            let filename = path.path.file_name()?.to_string_lossy();

            if !entry.status.is_created() {
                self.perform_checkout(vec![entry.clone()], cx);
            } else {
                let prompt = prompt(&format!("Trash {}?", filename), None, window, cx);
                cx.spawn_in(window, async move |_, cx| {
                    match prompt.await? {
                        TrashCancel::Trash => {}
                        TrashCancel::Cancel => return Ok(()),
                    }
                    let task = workspace.update(cx, |workspace, cx| {
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

    fn perform_checkout(&mut self, entries: Vec<GitStatusEntry>, cx: &mut Context<Self>) {
        let workspace = self.workspace.clone();
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };

        let op_id = self.pending.iter().map(|p| p.op_id).max().unwrap_or(0) + 1;
        self.pending.push(PendingOperation {
            op_id,
            target_status: TargetStatus::Reverted,
            entries: entries.clone(),
            finished: false,
        });
        self.update_visible_entries(cx);
        let task = cx.spawn(async move |_, cx| {
            let tasks: Vec<_> = workspace.update(cx, |workspace, cx| {
                workspace.project().update(cx, |project, cx| {
                    entries
                        .iter()
                        .filter_map(|entry| {
                            let path = active_repository
                                .read(cx)
                                .repo_path_to_project_path(&entry.repo_path, cx)?;
                            Some(project.open_buffer(path, cx))
                        })
                        .collect()
                })
            })?;

            let buffers = futures::future::join_all(tasks).await;

            active_repository
                .update(cx, |repo, cx| {
                    repo.checkout_files(
                        "HEAD",
                        entries
                            .iter()
                            .map(|entries| entries.repo_path.clone())
                            .collect(),
                        cx,
                    )
                })?
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

        cx.spawn(async move |this, cx| {
            let result = task.await;

            this.update(cx, |this, cx| {
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
                        this.show_error_toast("checkout", e, cx);
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
        cx.spawn(async move |this, cx| match prompt.await {
            Ok(RestoreCancel::RestoreTrackedFiles) => {
                this.update(cx, |this, cx| {
                    this.perform_checkout(entries, cx);
                })
                .ok();
            }
            _ => {
                return;
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
        cx.spawn_in(window, async move |this, cx| {
            match prompt.await? {
                TrashCancel::Trash => {}
                TrashCancel::Cancel => return Ok(()),
            }
            let tasks = workspace.update(cx, |workspace, cx| {
                to_delete
                    .iter()
                    .filter_map(|entry| {
                        workspace.project().update(cx, |project, cx| {
                            let project_path = active_repo
                                .read(cx)
                                .repo_path_to_project_path(&entry.repo_path, cx)?;
                            project.delete_file(project_path, true, cx)
                        })
                    })
                    .collect::<Vec<_>>()
            })?;
            let to_unstage = to_delete
                .into_iter()
                .filter(|entry| !entry.status.staging().is_fully_unstaged())
                .collect();
            this.update(cx, |this, cx| this.change_file_stage(false, to_unstage, cx))?;
            for task in tasks {
                task.await?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to trash files", window, cx, |e, _, _| {
            Some(format!("{e}"))
        });
    }

    pub fn stage_all(&mut self, _: &StageAll, _window: &mut Window, cx: &mut Context<Self>) {
        let entries = self
            .entries
            .iter()
            .filter_map(|entry| entry.status_entry())
            .filter(|status_entry| status_entry.staging.has_unstaged())
            .cloned()
            .collect::<Vec<_>>();
        self.change_file_stage(true, entries, cx);
    }

    pub fn unstage_all(&mut self, _: &UnstageAll, _window: &mut Window, cx: &mut Context<Self>) {
        let entries = self
            .entries
            .iter()
            .filter_map(|entry| entry.status_entry())
            .filter(|status_entry| status_entry.staging.has_staged())
            .cloned()
            .collect::<Vec<_>>();
        self.change_file_stage(false, entries, cx);
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
                if status_entry.status.staging().is_fully_staged() {
                    (false, vec![status_entry.clone()])
                } else {
                    (true, vec![status_entry.clone()])
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
                            && status_entry.staging.as_bool() != Some(goal_staged_state)
                    })
                    .map(|status_entry| status_entry.clone())
                    .collect::<Vec<_>>();

                (goal_staged_state, entries)
            }
        };
        self.change_file_stage(stage, repo_paths, cx);
    }

    fn change_file_stage(
        &mut self,
        stage: bool,
        entries: Vec<GitStatusEntry>,
        cx: &mut Context<Self>,
    ) {
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
            entries: entries.clone(),
            finished: false,
        });
        let repository = active_repository.read(cx);
        self.update_counts(repository);
        cx.notify();

        cx.spawn({
            async move |this, cx| {
                let result = cx
                    .update(|cx| {
                        if stage {
                            active_repository.update(cx, |repo, cx| {
                                let repo_paths = entries
                                    .iter()
                                    .map(|entry| entry.repo_path.clone())
                                    .collect();
                                repo.stage_entries(repo_paths, cx)
                            })
                        } else {
                            active_repository.update(cx, |repo, cx| {
                                let repo_paths = entries
                                    .iter()
                                    .map(|entry| entry.repo_path.clone())
                                    .collect();
                                repo.unstage_entries(repo_paths, cx)
                            })
                        }
                    })?
                    .await;

                this.update(cx, |this, cx| {
                    for pending in this.pending.iter_mut() {
                        if pending.op_id == op_id {
                            pending.finished = true
                        }
                    }
                    result
                        .map_err(|e| {
                            this.show_error_toast(if stage { "add" } else { "reset" }, e, cx);
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

    fn stage_selected(&mut self, _: &git::StageFile, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(selected_entry) = self.get_selected_entry() else {
            return;
        };
        let Some(status_entry) = selected_entry.status_entry() else {
            return;
        };
        if status_entry.staging != StageStatus::Staged {
            self.change_file_stage(true, vec![status_entry.clone()], cx);
        }
    }

    fn unstage_selected(
        &mut self,
        _: &git::UnstageFile,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(selected_entry) = self.get_selected_entry() else {
            return;
        };
        let Some(status_entry) = selected_entry.status_entry() else {
            return;
        };
        if status_entry.staging != StageStatus::Unstaged {
            self.change_file_stage(false, vec![status_entry.clone()], cx);
        }
    }

    fn commit(&mut self, _: &git::Commit, window: &mut Window, cx: &mut Context<Self>) {
        if self.amend_pending {
            return;
        }
        if self
            .commit_editor
            .focus_handle(cx)
            .contains_focused(window, cx)
        {
            telemetry::event!("Git Committed", source = "Git Panel");
            self.commit_changes(CommitOptions { amend: false }, window, cx)
        } else {
            cx.propagate();
        }
    }

    fn amend(&mut self, _: &git::Amend, window: &mut Window, cx: &mut Context<Self>) {
        if self
            .commit_editor
            .focus_handle(cx)
            .contains_focused(window, cx)
        {
            if self
                .active_repository
                .as_ref()
                .and_then(|repo| repo.read(cx).head_commit.as_ref())
                .is_some()
            {
                if !self.amend_pending {
                    self.set_amend_pending(true, cx);
                    self.load_last_commit_message_if_empty(cx);
                } else {
                    telemetry::event!("Git Amended", source = "Git Panel");
                    self.set_amend_pending(false, cx);
                    self.commit_changes(CommitOptions { amend: true }, window, cx);
                }
            }
        } else {
            cx.propagate();
        }
    }

    pub fn load_last_commit_message_if_empty(&mut self, cx: &mut Context<Self>) {
        if !self.commit_editor.read(cx).is_empty(cx) {
            return;
        }
        let Some(active_repository) = self.active_repository.as_ref() else {
            return;
        };
        let Some(recent_sha) = active_repository
            .read(cx)
            .head_commit
            .as_ref()
            .map(|commit| commit.sha.to_string())
        else {
            return;
        };
        let detail_task = self.load_commit_details(recent_sha, cx);
        cx.spawn(async move |this, cx| {
            if let Ok(message) = detail_task.await.map(|detail| detail.message) {
                this.update(cx, |this, cx| {
                    this.commit_message_buffer(cx).update(cx, |buffer, cx| {
                        let start = buffer.anchor_before(0);
                        let end = buffer.anchor_after(buffer.len());
                        buffer.edit([(start..end, message)], None, cx);
                    });
                })
                .log_err();
            }
        })
        .detach();
    }

    fn cancel(&mut self, _: &git::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        if self.amend_pending {
            self.set_amend_pending(false, cx);
        }
    }

    fn custom_or_suggested_commit_message(&self, cx: &mut Context<Self>) -> Option<String> {
        let message = self.commit_editor.read(cx).text(cx);
        let width = self
            .commit_editor
            .read(cx)
            .buffer()
            .read(cx)
            .language_settings(cx)
            .preferred_line_length as usize;

        if !message.trim().is_empty() {
            let message = wrap_with_prefix(
                String::new(),
                message,
                width,
                NonZeroU32::new(8).unwrap(), // tab size doesn't matter when prefix is empty
                false,
            );
            return Some(message);
        }

        self.suggest_commit_message(cx)
            .filter(|message| !message.trim().is_empty())
    }

    pub(crate) fn commit_changes(
        &mut self,
        options: CommitOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };
        let error_spawn = |message, window: &mut Window, cx: &mut App| {
            let prompt = window.prompt(PromptLevel::Warning, message, None, &["Ok"], cx);
            cx.spawn(async move |_| {
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
            let commit_task = active_repository.update(cx, |repo, cx| {
                repo.commit(message.into(), None, options, cx)
            });
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
            cx.spawn(async move |_, cx| {
                stage_task.await?;
                let commit_task = active_repository.update(cx, |repo, cx| {
                    repo.commit(message.into(), None, options, cx)
                })?;
                commit_task.await?
            })
        };
        let task = cx.spawn_in(window, async move |this, cx| {
            let result = task.await;
            this.update_in(cx, |this, window, cx| {
                this.pending_commit.take();
                match result {
                    Ok(()) => {
                        this.commit_editor
                            .update(cx, |editor, cx| editor.clear(window, cx));
                    }
                    Err(e) => this.show_error_toast("commit", e, cx),
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
        telemetry::event!("Git Uncommitted");

        let confirmation = self.check_for_pushed_commits(window, cx);
        let prior_head = self.load_commit_details("HEAD".to_string(), cx);

        let task = cx.spawn_in(window, async move |this, cx| {
            let result = maybe!(async {
                if let Ok(true) = confirmation.await {
                    let prior_head = prior_head.await?;

                    repo.update(cx, |repo, cx| {
                        repo.reset("HEAD^".to_string(), ResetMode::Soft, cx)
                    })?
                    .await??;

                    Ok(Some(prior_head))
                } else {
                    Ok(None)
                }
            })
            .await;

            this.update_in(cx, |this, window, cx| {
                this.pending_commit.take();
                match result {
                    Ok(None) => {}
                    Ok(Some(prior_commit)) => {
                        this.commit_editor.update(cx, |editor, cx| {
                            editor.set_text(prior_commit.message, window, cx)
                        });
                    }
                    Err(e) => this.show_error_toast("reset", e, cx),
                }
            })
            .ok();
        });

        self.pending_commit = Some(task);
    }

    fn check_for_pushed_commits(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl Future<Output = Result<bool, anyhow::Error>> + use<> {
        let repo = self.active_repository.clone();
        let mut cx = window.to_async(cx);

        async move {
            let Some(repo) = repo else {
                return Err(anyhow::anyhow!("No active repository"));
            };

            let pushed_to: Vec<SharedString> = repo
                .update(&mut cx, |repo, _| repo.check_for_pushed_commits())?
                .await??;

            if pushed_to.is_empty() {
                Ok(true)
            } else {
                #[derive(strum::EnumIter, strum::VariantNames)]
                #[strum(serialize_all = "title_case")]
                enum CancelUncommit {
                    Uncommit,
                    Cancel,
                }
                let detail = format!(
                    "This commit was already pushed to {}.",
                    pushed_to.into_iter().join(", ")
                );
                let result = cx
                    .update(|window, cx| prompt("Are you sure?", Some(&detail), window, cx))?
                    .await?;

                match result {
                    CancelUncommit::Cancel => Ok(false),
                    CancelUncommit::Uncommit => Ok(true),
                }
            }
        }
    }

    /// Suggests a commit message based on the changed files and their statuses
    pub fn suggest_commit_message(&self, cx: &App) -> Option<String> {
        if let Some(merge_message) = self
            .active_repository
            .as_ref()
            .and_then(|repo| repo.read(cx).merge.message.as_ref())
        {
            return Some(merge_message.to_string());
        }

        let git_status_entry = if let Some(staged_entry) = &self.single_staged_entry {
            Some(staged_entry)
        } else if let Some(single_tracked_entry) = &self.single_tracked_entry {
            Some(single_tracked_entry)
        } else {
            None
        }?;

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

    fn generate_commit_message_action(
        &mut self,
        _: &git::GenerateCommitMessage,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.generate_commit_message(cx);
    }

    /// Generates a commit message using an LLM.
    pub fn generate_commit_message(&mut self, cx: &mut Context<Self>) {
        if !self.can_commit() {
            return;
        }

        let model = match current_language_model(cx) {
            Some(value) => value,
            None => return,
        };

        let Some(repo) = self.active_repository.as_ref() else {
            return;
        };

        telemetry::event!("Git Commit Message Generated");

        let diff = repo.update(cx, |repo, cx| {
            if self.has_staged_changes() {
                repo.diff(DiffType::HeadToIndex, cx)
            } else {
                repo.diff(DiffType::HeadToWorktree, cx)
            }
        });

        let temperature = AssistantSettings::temperature_for_model(&model, cx);

        self.generate_commit_message_task = Some(cx.spawn(async move |this, cx| {
             async move {
                let _defer = cx.on_drop(&this, |this, _cx| {
                    this.generate_commit_message_task.take();
                });

                let mut diff_text = diff.await??;

                const ONE_MB: usize = 1_000_000;
                if diff_text.len() > ONE_MB {
                    diff_text = diff_text.chars().take(ONE_MB).collect()
                }

                let subject = this.update(cx, |this, cx| {
                    this.commit_editor.read(cx).text(cx).lines().next().map(ToOwned::to_owned).unwrap_or_default()
                })?;

                let text_empty = subject.trim().is_empty();

                let content = if text_empty {
                    format!("{PROMPT}\nHere are the changes in this commit:\n{diff_text}")
                } else {
                    format!("{PROMPT}\nHere is the user's subject line:\n{subject}\nHere are the changes in this commit:\n{diff_text}\n")
                };

                const PROMPT: &str = include_str!("commit_message_prompt.txt");

                let request = LanguageModelRequest {
                    thread_id: None,
                    prompt_id: None,
                    mode: None,
                    messages: vec![LanguageModelRequestMessage {
                        role: Role::User,
                        content: vec![content.into()],
                        cache: false,
                    }],
                    tools: Vec::new(),
                    tool_choice: None,
                    stop: Vec::new(),
                    temperature,
                };

                let stream = model.stream_completion_text(request, &cx);
                let mut messages = stream.await?;

                if !text_empty {
                    this.update(cx, |this, cx| {
                        this.commit_message_buffer(cx).update(cx, |buffer, cx| {
                            let insert_position = buffer.anchor_before(buffer.len());
                            buffer.edit([(insert_position..insert_position, "\n")], None, cx)
                        });
                    })?;
                }

                while let Some(message) = messages.stream.next().await {
                    let text = message?;

                    this.update(cx, |this, cx| {
                        this.commit_message_buffer(cx).update(cx, |buffer, cx| {
                            let insert_position = buffer.anchor_before(buffer.len());
                            buffer.edit([(insert_position..insert_position, text)], None, cx);
                        });
                    })?;
                }

                anyhow::Ok(())
            }
            .log_err().await
        }));
    }

    pub(crate) fn fetch(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_push_and_pull(cx) {
            return;
        }

        let Some(repo) = self.active_repository.clone() else {
            return;
        };
        telemetry::event!("Git Fetched");
        let askpass = self.askpass_delegate("git fetch", window, cx);
        let this = cx.weak_entity();
        window
            .spawn(cx, async move |cx| {
                let fetch = repo.update(cx, |repo, cx| repo.fetch(askpass, cx))?;

                let remote_message = fetch.await?;
                this.update(cx, |this, cx| {
                    let action = RemoteAction::Fetch;
                    match remote_message {
                        Ok(remote_message) => this.show_remote_output(action, remote_message, cx),
                        Err(e) => {
                            log::error!("Error while fetching {:?}", e);
                            this.show_error_toast(action.name(), e, cx)
                        }
                    }

                    anyhow::Ok(())
                })
                .ok();
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
    }

    pub(crate) fn git_init(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let worktrees = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .collect::<Vec<_>>();

        let worktree = if worktrees.len() == 1 {
            Task::ready(Some(worktrees.first().unwrap().clone()))
        } else if worktrees.len() == 0 {
            let result = window.prompt(
                PromptLevel::Warning,
                "Unable to initialize a git repository",
                Some("Open a directory first"),
                &["Ok"],
                cx,
            );
            cx.background_executor()
                .spawn(async move {
                    result.await.ok();
                })
                .detach();
            return;
        } else {
            let worktree_directories = worktrees
                .iter()
                .map(|worktree| worktree.read(cx).abs_path())
                .map(|worktree_abs_path| {
                    if let Ok(path) = worktree_abs_path.strip_prefix(util::paths::home_dir()) {
                        Path::new("~")
                            .join(path)
                            .to_string_lossy()
                            .to_string()
                            .into()
                    } else {
                        worktree_abs_path.to_string_lossy().to_string().into()
                    }
                })
                .collect_vec();
            let prompt = picker_prompt::prompt(
                "Where would you like to initialize this git repository?",
                worktree_directories,
                self.workspace.clone(),
                window,
                cx,
            );

            cx.spawn(async move |_, _| prompt.await.map(|ix| worktrees[ix].clone()))
        };

        cx.spawn_in(window, async move |this, cx| {
            let worktree = match worktree.await {
                Some(worktree) => worktree,
                None => {
                    return;
                }
            };

            let Ok(result) = this.update(cx, |this, cx| {
                let fallback_branch_name = GitPanelSettings::get_global(cx)
                    .fallback_branch_name
                    .clone();
                this.project.read(cx).git_init(
                    worktree.read(cx).abs_path(),
                    fallback_branch_name,
                    cx,
                )
            }) else {
                return;
            };

            let result = result.await;

            this.update_in(cx, |this, _, cx| match result {
                Ok(()) => {}
                Err(e) => this.show_error_toast("init", e, cx),
            })
            .ok();
        })
        .detach();
    }

    pub(crate) fn pull(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_push_and_pull(cx) {
            return;
        }
        let Some(repo) = self.active_repository.clone() else {
            return;
        };
        let Some(branch) = repo.read(cx).branch.as_ref() else {
            return;
        };
        telemetry::event!("Git Pulled");
        let branch = branch.clone();
        let remote = self.get_current_remote(window, cx);
        cx.spawn_in(window, async move |this, cx| {
            let remote = match remote.await {
                Ok(Some(remote)) => remote,
                Ok(None) => {
                    return Ok(());
                }
                Err(e) => {
                    log::error!("Failed to get current remote: {}", e);
                    this.update(cx, |this, cx| this.show_error_toast("pull", e, cx))
                        .ok();
                    return Ok(());
                }
            };

            let askpass = this.update_in(cx, |this, window, cx| {
                this.askpass_delegate(format!("git pull {}", remote.name), window, cx)
            })?;

            let pull = repo.update(cx, |repo, cx| {
                repo.pull(
                    branch.name().to_owned().into(),
                    remote.name.clone(),
                    askpass,
                    cx,
                )
            })?;

            let remote_message = pull.await?;

            let action = RemoteAction::Pull(remote);
            this.update(cx, |this, cx| match remote_message {
                Ok(remote_message) => this.show_remote_output(action, remote_message, cx),
                Err(e) => {
                    log::error!("Error while pulling {:?}", e);
                    this.show_error_toast(action.name(), e, cx)
                }
            })
            .ok();

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub(crate) fn push(&mut self, force_push: bool, window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_push_and_pull(cx) {
            return;
        }
        let Some(repo) = self.active_repository.clone() else {
            return;
        };
        let Some(branch) = repo.read(cx).branch.as_ref() else {
            return;
        };
        telemetry::event!("Git Pushed");
        let branch = branch.clone();

        let options = if force_push {
            Some(PushOptions::Force)
        } else {
            match branch.upstream {
                Some(Upstream {
                    tracking: UpstreamTracking::Gone,
                    ..
                })
                | None => Some(PushOptions::SetUpstream),
                _ => None,
            }
        };
        let remote = self.get_current_remote(window, cx);

        cx.spawn_in(window, async move |this, cx| {
            let remote = match remote.await {
                Ok(Some(remote)) => remote,
                Ok(None) => {
                    return Ok(());
                }
                Err(e) => {
                    log::error!("Failed to get current remote: {}", e);
                    this.update(cx, |this, cx| this.show_error_toast("push", e, cx))
                        .ok();
                    return Ok(());
                }
            };

            let askpass_delegate = this.update_in(cx, |this, window, cx| {
                this.askpass_delegate(format!("git push {}", remote.name), window, cx)
            })?;

            let push = repo.update(cx, |repo, cx| {
                repo.push(
                    branch.name().to_owned().into(),
                    remote.name.clone(),
                    options,
                    askpass_delegate,
                    cx,
                )
            })?;

            let remote_output = push.await?;

            let action = RemoteAction::Push(branch.name().to_owned().into(), remote);
            this.update(cx, |this, cx| match remote_output {
                Ok(remote_message) => this.show_remote_output(action, remote_message, cx),
                Err(e) => {
                    log::error!("Error while pushing {:?}", e);
                    this.show_error_toast(action.name(), e, cx)
                }
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn askpass_delegate(
        &self,
        operation: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AskPassDelegate {
        let this = cx.weak_entity();
        let operation = operation.into();
        let window = window.window_handle();
        AskPassDelegate::new(&mut cx.to_async(), move |prompt, tx, cx| {
            window
                .update(cx, |_, window, cx| {
                    this.update(cx, |this, cx| {
                        this.workspace.update(cx, |workspace, cx| {
                            workspace.toggle_modal(window, cx, |window, cx| {
                                AskPassModal::new(operation.clone(), prompt.into(), tx, window, cx)
                            });
                        })
                    })
                })
                .ok();
        })
    }

    fn can_push_and_pull(&self, cx: &App) -> bool {
        !self.project.read(cx).is_via_collab()
    }

    fn get_current_remote(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl Future<Output = anyhow::Result<Option<Remote>>> + use<> {
        let repo = self.active_repository.clone();
        let workspace = self.workspace.clone();
        let mut cx = window.to_async(cx);

        async move {
            let Some(repo) = repo else {
                return Err(anyhow::anyhow!("No active repository"));
            };

            let mut current_remotes: Vec<Remote> = repo
                .update(&mut cx, |repo, _| {
                    let Some(current_branch) = repo.branch.as_ref() else {
                        return Err(anyhow::anyhow!("No active branch"));
                    };

                    Ok(repo.get_remotes(Some(current_branch.name().to_string())))
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
                    .await;

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
        self.update_visible_entries_task = cx.spawn_in(window, async move |_, cx| {
            cx.background_executor().timer(UPDATE_DEBOUNCE).await;
            if let Some(git_panel) = handle.upgrade() {
                git_panel
                    .update_in(cx, |git_panel, window, cx| {
                        if clear_pending {
                            git_panel.clear_pending();
                        }
                        git_panel.update_visible_entries(cx);
                        git_panel.update_scrollbar_properties(window, cx);
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

        cx.spawn_in(window, async move |git_panel, cx| {
            let buffer = load_buffer.await?;
            git_panel.update_in(cx, |git_panel, window, cx| {
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
                            git_panel.suggest_commit_message(cx).map(SharedString::from),
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
        self.single_staged_entry.take();
        self.single_tracked_entry.take();
        self.conflicted_count = 0;
        self.conflicted_staged_count = 0;
        self.new_count = 0;
        self.tracked_count = 0;
        self.new_staged_count = 0;
        self.tracked_staged_count = 0;
        self.entry_count = 0;

        let sort_by_path = GitPanelSettings::get_global(cx).sort_by_path;

        let mut changed_entries = Vec::new();
        let mut new_entries = Vec::new();
        let mut conflict_entries = Vec::new();
        let mut last_staged = None;
        let mut staged_count = 0;
        let mut max_width_item: Option<(RepoPath, usize)> = None;

        let Some(repo) = self.active_repository.as_ref() else {
            // Just clear entries if no repository is active.
            cx.notify();
            return;
        };

        let repo = repo.read(cx);

        for entry in repo.cached_status() {
            let is_conflict = repo.has_conflict(&entry.repo_path);
            let is_new = entry.status.is_created();
            let staging = entry.status.staging();

            if self.pending.iter().any(|pending| {
                pending.target_status == TargetStatus::Reverted
                    && !pending.finished
                    && pending
                        .entries
                        .iter()
                        .any(|pending| pending.repo_path == entry.repo_path)
            }) {
                continue;
            }

            let abs_path = repo.work_directory_abs_path.join(&entry.repo_path.0);
            let entry = GitStatusEntry {
                repo_path: entry.repo_path.clone(),
                abs_path,
                status: entry.status,
                staging,
            };

            if staging.has_staged() {
                staged_count += 1;
                last_staged = Some(entry.clone());
            }

            let width_estimate = Self::item_width_estimate(
                entry.parent_dir().map(|s| s.len()).unwrap_or(0),
                entry.display_name().len(),
            );

            match max_width_item.as_mut() {
                Some((repo_path, estimate)) => {
                    if width_estimate > *estimate {
                        *repo_path = entry.repo_path.clone();
                        *estimate = width_estimate;
                    }
                }
                None => max_width_item = Some((entry.repo_path.clone(), width_estimate)),
            }

            if sort_by_path {
                changed_entries.push(entry);
            } else if is_conflict {
                conflict_entries.push(entry);
            } else if is_new {
                new_entries.push(entry);
            } else {
                changed_entries.push(entry);
            }
        }

        let mut pending_staged_count = 0;
        let mut last_pending_staged = None;
        let mut pending_status_for_last_staged = None;
        for pending in self.pending.iter() {
            if pending.target_status == TargetStatus::Staged {
                pending_staged_count += pending.entries.len();
                last_pending_staged = pending.entries.iter().next().cloned();
            }
            if let Some(last_staged) = &last_staged {
                if pending
                    .entries
                    .iter()
                    .any(|entry| entry.repo_path == last_staged.repo_path)
                {
                    pending_status_for_last_staged = Some(pending.target_status);
                }
            }
        }

        if conflict_entries.len() == 0 && staged_count == 1 && pending_staged_count == 0 {
            match pending_status_for_last_staged {
                Some(TargetStatus::Staged) | None => {
                    self.single_staged_entry = last_staged;
                }
                _ => {}
            }
        } else if conflict_entries.len() == 0 && pending_staged_count == 1 {
            self.single_staged_entry = last_pending_staged;
        }

        if conflict_entries.len() == 0 && changed_entries.len() == 1 {
            self.single_tracked_entry = changed_entries.first().cloned();
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
            if !sort_by_path {
                self.entries.push(GitListEntry::Header(GitHeaderEntry {
                    header: Section::Tracked,
                }));
            }
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

        if let Some((repo_path, _)) = max_width_item {
            self.max_width_item_index = self.entries.iter().position(|entry| match entry {
                GitListEntry::GitStatusEntry(git_status_entry) => {
                    git_status_entry.repo_path == repo_path
                }
                GitListEntry::Header(_) => false,
            });
        }

        self.update_counts(repo);

        self.select_first_entry_if_none(cx);

        let suggested_commit_message = self.suggest_commit_message(cx);
        let placeholder_text = suggested_commit_message.unwrap_or("Enter commit message".into());

        self.commit_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text(Arc::from(placeholder_text), cx)
        });

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
        self.show_placeholders = false;
        self.conflicted_count = 0;
        self.conflicted_staged_count = 0;
        self.new_count = 0;
        self.tracked_count = 0;
        self.new_staged_count = 0;
        self.tracked_staged_count = 0;
        self.entry_count = 0;
        for entry in &self.entries {
            let Some(status_entry) = entry.status_entry() else {
                continue;
            };
            self.entry_count += 1;
            if repo.has_conflict(&status_entry.repo_path) {
                self.conflicted_count += 1;
                if self.entry_staging(status_entry).has_staged() {
                    self.conflicted_staged_count += 1;
                }
            } else if status_entry.status.is_created() {
                self.new_count += 1;
                if self.entry_staging(status_entry).has_staged() {
                    self.new_staged_count += 1;
                }
            } else {
                self.tracked_count += 1;
                if self.entry_staging(status_entry).has_staged() {
                    self.tracked_staged_count += 1;
                }
            }
        }
    }

    fn entry_staging(&self, entry: &GitStatusEntry) -> StageStatus {
        for pending in self.pending.iter().rev() {
            if pending
                .entries
                .iter()
                .any(|pending_entry| pending_entry.repo_path == entry.repo_path)
            {
                match pending.target_status {
                    TargetStatus::Staged => return StageStatus::Staged,
                    TargetStatus::Unstaged => return StageStatus::Unstaged,
                    TargetStatus::Reverted => continue,
                    TargetStatus::Unchanged => continue,
                }
            }
        }
        entry.staging
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

    fn has_tracked_changes(&self) -> bool {
        self.tracked_count > 0
    }

    pub fn has_unstaged_conflicts(&self) -> bool {
        self.conflicted_count > 0 && self.conflicted_count != self.conflicted_staged_count
    }

    fn show_error_toast(&self, action: impl Into<SharedString>, e: anyhow::Error, cx: &mut App) {
        let action = action.into();
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let message = e.to_string().trim().to_string();
        if message
            .matches(git::repository::REMOTE_CANCELLED_BY_USER)
            .next()
            .is_some()
        {
            return; // Hide the cancelled by user message
        } else {
            workspace.update(cx, |workspace, cx| {
                let workspace_weak = cx.weak_entity();
                let toast =
                    StatusToast::new(format!("git {} failed", action.clone()), cx, |this, _cx| {
                        this.icon(ToastIcon::new(IconName::XCircle).color(Color::Error))
                            .action("View Log", move |window, cx| {
                                let message = message.clone();
                                let action = action.clone();
                                workspace_weak
                                    .update(cx, move |workspace, cx| {
                                        Self::open_output(action, workspace, &message, window, cx)
                                    })
                                    .ok();
                            })
                    });
                workspace.toggle_status_toast(toast, cx)
            });
        }
    }

    fn show_remote_output(&self, action: RemoteAction, info: RemoteCommandOutput, cx: &mut App) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        workspace.update(cx, |workspace, cx| {
            let SuccessMessage { message, style } = remote_output::format_output(&action, info);
            let workspace_weak = cx.weak_entity();
            let operation = action.name();

            let status_toast = StatusToast::new(message, cx, move |this, _cx| {
                use remote_output::SuccessStyle::*;
                match style {
                    Toast { .. } => this,
                    ToastWithLog { output } => this
                        .icon(ToastIcon::new(IconName::GitBranchSmall).color(Color::Muted))
                        .action("View Log", move |window, cx| {
                            let output = output.clone();
                            let output =
                                format!("stdout:\n{}\nstderr:\n{}", output.stdout, output.stderr);
                            workspace_weak
                                .update(cx, move |workspace, cx| {
                                    Self::open_output(operation, workspace, &output, window, cx)
                                })
                                .ok();
                        }),
                    PushPrLink { link } => this
                        .icon(ToastIcon::new(IconName::GitBranchSmall).color(Color::Muted))
                        .action("Open Pull Request", move |_, cx| cx.open_url(&link)),
                }
            });
            workspace.toggle_status_toast(status_toast, cx)
        });
    }

    fn open_output(
        operation: impl Into<SharedString>,
        workspace: &mut Workspace,
        output: &str,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let operation = operation.into();
        let buffer = cx.new(|cx| Buffer::local(output, cx));
        buffer.update(cx, |buffer, cx| {
            buffer.set_capability(language::Capability::ReadOnly, cx);
        });
        let editor = cx.new(|cx| {
            let mut editor = Editor::for_buffer(buffer, None, window, cx);
            editor.buffer().update(cx, |buffer, cx| {
                buffer.set_title(format!("Output from git {operation}"), cx);
            });
            editor.set_read_only(true);
            editor
        });

        workspace.add_item_to_center(Box::new(editor), window, cx);
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

    // eventually we'll need to take depth into account here
    // if we add a tree view
    fn item_width_estimate(path: usize, file_name: usize) -> usize {
        path + file_name
    }

    fn render_overflow_menu(&self, id: impl Into<ElementId>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let has_tracked_changes = self.has_tracked_changes();
        let has_staged_changes = self.has_staged_changes();
        let has_unstaged_changes = self.has_unstaged_changes();
        let has_new_changes = self.new_count > 0;

        PopoverMenu::new(id.into())
            .trigger(
                IconButton::new("overflow-menu-trigger", IconName::EllipsisVertical)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted),
            )
            .menu(move |window, cx| {
                Some(git_panel_context_menu(
                    focus_handle.clone(),
                    GitMenuState {
                        has_tracked_changes,
                        has_staged_changes,
                        has_unstaged_changes,
                        has_new_changes,
                    },
                    window,
                    cx,
                ))
            })
            .anchor(Corner::TopRight)
    }

    pub(crate) fn render_generate_commit_message_button(
        &self,
        cx: &Context<Self>,
    ) -> Option<AnyElement> {
        current_language_model(cx).is_some().then(|| {
            if self.generate_commit_message_task.is_some() {
                return h_flex()
                    .gap_1()
                    .child(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::XSmall)
                            .color(Color::Info)
                            .with_animation(
                                "arrow-circle",
                                Animation::new(Duration::from_secs(2)).repeat(),
                                |icon, delta| {
                                    icon.transform(Transformation::rotate(percentage(delta)))
                                },
                            ),
                    )
                    .child(
                        Label::new("Generating Commit...")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .into_any_element();
            }

            let can_commit = self.can_commit();
            let editor_focus_handle = self.commit_editor.focus_handle(cx);
            IconButton::new("generate-commit-message", IconName::AiEdit)
                .shape(ui::IconButtonShape::Square)
                .icon_color(Color::Muted)
                .tooltip(move |window, cx| {
                    if can_commit {
                        Tooltip::for_action_in(
                            "Generate Commit Message",
                            &git::GenerateCommitMessage,
                            &editor_focus_handle,
                            window,
                            cx,
                        )
                    } else {
                        Tooltip::simple("No changes to commit", cx)
                    }
                })
                .disabled(!can_commit)
                .on_click(cx.listener(move |this, _event, _window, cx| {
                    this.generate_commit_message(cx);
                }))
                .into_any_element()
        })
    }

    pub(crate) fn render_co_authors(&self, cx: &Context<Self>) -> Option<AnyElement> {
        let potential_co_authors = self.potential_co_authors(cx);

        let (tooltip_label, icon) = if self.add_coauthors {
            ("Remove co-authored-by", IconName::Person)
        } else {
            ("Add co-authored-by", IconName::UserCheck)
        };

        if potential_co_authors.is_empty() {
            None
        } else {
            Some(
                IconButton::new("co-authors", icon)
                    .shape(ui::IconButtonShape::Square)
                    .icon_color(Color::Disabled)
                    .selected_icon_color(Color::Selected)
                    .toggle_state(self.add_coauthors)
                    .tooltip(move |_, cx| {
                        let title = format!(
                            "{}:{}{}",
                            tooltip_label,
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

    fn render_git_commit_menu(
        &self,
        id: impl Into<ElementId>,
        keybinding_target: Option<FocusHandle>,
    ) -> impl IntoElement {
        PopoverMenu::new(id.into())
            .trigger(
                ui::ButtonLike::new_rounded_right("commit-split-button-right")
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
                        .when_some(keybinding_target.clone(), |el, keybinding_target| {
                            el.context(keybinding_target.clone())
                        })
                        .action("Amend", Amend.boxed_clone())
                }))
            })
            .anchor(Corner::TopRight)
    }

    pub fn configure_commit_button(&self, cx: &mut Context<Self>) -> (bool, &'static str) {
        if self.has_unstaged_conflicts() {
            (false, "You must resolve conflicts before committing")
        } else if !self.has_staged_changes() && !self.has_tracked_changes() {
            (false, "No changes to commit")
        } else if self.pending_commit.is_some() {
            (false, "Commit in progress")
        } else if self.custom_or_suggested_commit_message(cx).is_none() {
            (false, "No commit message")
        } else if !self.has_write_access(cx) {
            (false, "You do not have write access to this project")
        } else {
            (true, self.commit_button_title())
        }
    }

    pub fn commit_button_title(&self) -> &'static str {
        if self.amend_pending {
            if self.has_staged_changes() {
                "Amend"
            } else {
                "Amend Tracked"
            }
        } else {
            if self.has_staged_changes() {
                "Commit"
            } else {
                "Commit Tracked"
            }
        }
    }

    fn expand_commit_editor(
        &mut self,
        _: &git::ExpandCommitEditor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let workspace = self.workspace.clone();
        window.defer(cx, move |window, cx| {
            workspace
                .update(cx, |workspace, cx| {
                    CommitModal::toggle(workspace, None, window, cx)
                })
                .ok();
        })
    }

    fn render_panel_header(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        self.active_repository.as_ref()?;

        let text;
        let action;
        let tooltip;
        if self.total_staged_count() == self.entry_count && self.entry_count > 0 {
            text = "Unstage All";
            action = git::UnstageAll.boxed_clone();
            tooltip = "git reset";
        } else {
            text = "Stage All";
            action = git::StageAll.boxed_clone();
            tooltip = "git add --all ."
        }

        let change_string = match self.entry_count {
            0 => "No Changes".to_string(),
            1 => "1 Change".to_string(),
            _ => format!("{} Changes", self.entry_count),
        };

        Some(
            self.panel_header_container(window, cx)
                .px_2()
                .child(
                    panel_button(change_string)
                        .color(Color::Muted)
                        .tooltip(Tooltip::for_action_title_in(
                            "Open Diff",
                            &Diff,
                            &self.focus_handle,
                        ))
                        .on_click(|_, _, cx| {
                            cx.defer(|cx| {
                                cx.dispatch_action(&Diff);
                            })
                        }),
                )
                .child(div().flex_grow()) // spacer
                .child(self.render_overflow_menu("overflow_menu"))
                .child(div().w_2()) // another spacer
                .child(
                    panel_filled_button(text)
                        .tooltip(Tooltip::for_action_title_in(
                            tooltip,
                            action.as_ref(),
                            &self.focus_handle,
                        ))
                        .disabled(self.entry_count == 0)
                        .on_click(move |_, _, cx| {
                            let action = action.boxed_clone();
                            cx.defer(move |cx| {
                                cx.dispatch_action(action.as_ref());
                            })
                        }),
                ),
        )
    }

    pub(crate) fn render_remote_button(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let branch = self.active_repository.as_ref()?.read(cx).branch.clone();
        if !self.can_push_and_pull(cx) {
            return None;
        }
        Some(
            h_flex()
                .gap_1()
                .flex_shrink_0()
                .when_some(branch, |this, branch| {
                    let focus_handle = Some(self.focus_handle(cx));

                    this.children(render_remote_button(
                        "remote-button",
                        &branch,
                        focus_handle,
                        true,
                    ))
                })
                .into_any_element(),
        )
    }

    pub fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let active_repository = self.active_repository.clone()?;
        let panel_editor_style = panel_editor_style(true, window, cx);

        let enable_coauthors = self.render_co_authors(cx);

        let editor_focus_handle = self.commit_editor.focus_handle(cx);
        let expand_tooltip_focus_handle = editor_focus_handle.clone();

        let branch = active_repository.read(cx).branch.clone();
        let head_commit = active_repository.read(cx).head_commit.clone();

        let footer_size = px(32.);
        let gap = px(9.0);
        let max_height = panel_editor_style
            .text
            .line_height_in_pixels(window.rem_size())
            * MAX_PANEL_EDITOR_LINES
            + gap;

        let git_panel = cx.entity().clone();
        let display_name = SharedString::from(Arc::from(
            active_repository
                .read(cx)
                .display_name()
                .trim_end_matches("/"),
        ));
        let editor_is_long = self.commit_editor.update(cx, |editor, cx| {
            editor.max_point(cx).row().0 >= MAX_PANEL_EDITOR_LINES as u32
        });
        let has_previous_commit = head_commit.is_some();

        let footer = v_flex()
            .child(PanelRepoFooter::new(
                display_name,
                branch,
                head_commit,
                Some(git_panel.clone()),
            ))
            .child(
                panel_editor_container(window, cx)
                    .id("commit-editor-container")
                    .relative()
                    .w_full()
                    .h(max_height + footer_size)
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .cursor_text()
                    .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                        window.focus(&this.commit_editor.focus_handle(cx));
                    }))
                    .child(
                        h_flex()
                            .id("commit-footer")
                            .border_t_1()
                            .when(editor_is_long, |el| {
                                el.border_color(cx.theme().colors().border_variant)
                            })
                            .absolute()
                            .bottom_0()
                            .left_0()
                            .w_full()
                            .px_2()
                            .h(footer_size)
                            .flex_none()
                            .justify_between()
                            .child(
                                self.render_generate_commit_message_button(cx)
                                    .unwrap_or_else(|| div().into_any_element()),
                            )
                            .child(
                                h_flex()
                                    .gap_0p5()
                                    .children(enable_coauthors)
                                    .child(self.render_commit_button(has_previous_commit, cx)),
                            ),
                    )
                    .child(
                        div()
                            .pr_2p5()
                            .on_action(|&editor::actions::MoveUp, _, cx| {
                                cx.stop_propagation();
                            })
                            .on_action(|&editor::actions::MoveDown, _, cx| {
                                cx.stop_propagation();
                            })
                            .child(EditorElement::new(&self.commit_editor, panel_editor_style)),
                    )
                    .child(
                        h_flex()
                            .absolute()
                            .top_2()
                            .right_2()
                            .opacity(0.5)
                            .hover(|this| this.opacity(1.0))
                            .child(
                                panel_icon_button("expand-commit-editor", IconName::Maximize)
                                    .icon_size(IconSize::Small)
                                    .size(ui::ButtonSize::Default)
                                    .tooltip(move |window, cx| {
                                        Tooltip::for_action_in(
                                            "Open Commit Modal",
                                            &git::ExpandCommitEditor,
                                            &expand_tooltip_focus_handle,
                                            window,
                                            cx,
                                        )
                                    })
                                    .on_click(cx.listener({
                                        move |_, _, window, cx| {
                                            window.dispatch_action(
                                                git::ExpandCommitEditor.boxed_clone(),
                                                cx,
                                            )
                                        }
                                    })),
                            ),
                    ),
            );

        Some(footer)
    }

    fn render_commit_button(
        &self,
        has_previous_commit: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let (can_commit, tooltip) = self.configure_commit_button(cx);
        let title = self.commit_button_title();
        let commit_tooltip_focus_handle = self.commit_editor.focus_handle(cx);
        div()
            .id("commit-wrapper")
            .on_hover(cx.listener(move |this, hovered, _, cx| {
                this.show_placeholders =
                    *hovered && !this.has_staged_changes() && !this.has_unstaged_conflicts();
                cx.notify()
            }))
            .when(self.amend_pending, {
                |this| {
                    this.h_flex()
                        .gap_1()
                        .child(
                            panel_filled_button("Cancel")
                                .tooltip({
                                    let handle = commit_tooltip_focus_handle.clone();
                                    move |window, cx| {
                                        Tooltip::for_action_in(
                                            "Cancel amend",
                                            &git::Cancel,
                                            &handle,
                                            window,
                                            cx,
                                        )
                                    }
                                })
                                .on_click(move |_, window, cx| {
                                    window.dispatch_action(Box::new(git::Cancel), cx);
                                }),
                        )
                        .child(
                            panel_filled_button(title)
                                .tooltip({
                                    let handle = commit_tooltip_focus_handle.clone();
                                    move |window, cx| {
                                        if can_commit {
                                            Tooltip::for_action_in(
                                                tooltip, &Amend, &handle, window, cx,
                                            )
                                        } else {
                                            Tooltip::simple(tooltip, cx)
                                        }
                                    }
                                })
                                .disabled(!can_commit || self.modal_open)
                                .on_click({
                                    let git_panel = cx.weak_entity();
                                    move |_, window, cx| {
                                        telemetry::event!("Git Amended", source = "Git Panel");
                                        git_panel
                                            .update(cx, |git_panel, cx| {
                                                git_panel.set_amend_pending(false, cx);
                                                git_panel.commit_changes(
                                                    CommitOptions { amend: true },
                                                    window,
                                                    cx,
                                                );
                                            })
                                            .ok();
                                    }
                                }),
                        )
                }
            })
            .when(!self.amend_pending, |this| {
                this.when(has_previous_commit, |this| {
                    this.child(SplitButton::new(
                        ui::ButtonLike::new_rounded_left(ElementId::Name(
                            format!("split-button-left-{}", title).into(),
                        ))
                        .layer(ui::ElevationIndex::ModalSurface)
                        .size(ui::ButtonSize::Compact)
                        .child(
                            div()
                                .child(Label::new(title).size(LabelSize::Small))
                                .mr_0p5(),
                        )
                        .on_click({
                            let git_panel = cx.weak_entity();
                            move |_, window, cx| {
                                telemetry::event!("Git Committed", source = "Git Panel");
                                git_panel
                                    .update(cx, |git_panel, cx| {
                                        git_panel.commit_changes(
                                            CommitOptions { amend: false },
                                            window,
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        })
                        .disabled(!can_commit || self.modal_open)
                        .tooltip({
                            let handle = commit_tooltip_focus_handle.clone();
                            move |window, cx| {
                                if can_commit {
                                    Tooltip::with_meta_in(
                                        tooltip,
                                        Some(&git::Commit),
                                        "git commit",
                                        &handle.clone(),
                                        window,
                                        cx,
                                    )
                                } else {
                                    Tooltip::simple(tooltip, cx)
                                }
                            }
                        }),
                        self.render_git_commit_menu(
                            ElementId::Name(format!("split-button-right-{}", title).into()),
                            Some(commit_tooltip_focus_handle.clone()),
                        )
                        .into_any_element(),
                    ))
                })
                .when(!has_previous_commit, |this| {
                    this.child(
                        panel_filled_button(title)
                            .tooltip(move |window, cx| {
                                if can_commit {
                                    Tooltip::with_meta_in(
                                        tooltip,
                                        Some(&git::Commit),
                                        "git commit",
                                        &commit_tooltip_focus_handle,
                                        window,
                                        cx,
                                    )
                                } else {
                                    Tooltip::simple(tooltip, cx)
                                }
                            })
                            .disabled(!can_commit || self.modal_open)
                            .on_click({
                                let git_panel = cx.weak_entity();
                                move |_, window, cx| {
                                    telemetry::event!("Git Committed", source = "Git Panel");
                                    git_panel
                                        .update(cx, |git_panel, cx| {
                                            git_panel.commit_changes(
                                                CommitOptions { amend: false },
                                                window,
                                                cx,
                                            );
                                        })
                                        .ok();
                                }
                            }),
                    )
                })
            })
    }

    fn render_pending_amend(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .py_2()
            .px(px(8.))
            .border_color(cx.theme().colors().border)
            .child(
                Label::new(
                    "This will update your most recent commit. Cancel to make a new one instead.",
                )
                .size(LabelSize::Small),
            )
    }

    fn render_previous_commit(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let active_repository = self.active_repository.as_ref()?;
        let branch = active_repository.read(cx).branch.as_ref()?;
        let commit = branch.most_recent_commit.as_ref()?.clone();
        let workspace = self.workspace.clone();

        let this = cx.entity();
        Some(
            h_flex()
                .items_center()
                .py_2()
                .px(px(8.))
                .border_color(cx.theme().colors().border)
                .gap_1p5()
                .child(
                    div()
                        .flex_grow()
                        .overflow_hidden()
                        .items_center()
                        .max_w(relative(0.85))
                        .h_full()
                        .child(
                            Label::new(commit.subject.clone())
                                .size(LabelSize::Small)
                                .truncate(),
                        )
                        .id("commit-msg-hover")
                        .on_click({
                            let commit = commit.clone();
                            let repo = active_repository.downgrade();
                            move |_, window, cx| {
                                CommitView::open(
                                    commit.clone(),
                                    repo.clone(),
                                    workspace.clone().clone(),
                                    window,
                                    cx,
                                );
                            }
                        })
                        .hoverable_tooltip({
                            let repo = active_repository.clone();
                            move |window, cx| {
                                GitPanelMessageTooltip::new(
                                    this.clone(),
                                    commit.sha.clone(),
                                    repo.clone(),
                                    window,
                                    cx,
                                )
                                .into()
                            }
                        }),
                )
                .child(div().flex_1())
                .when(commit.has_parent, |this| {
                    let has_unstaged = self.has_unstaged_changes();
                    this.child(
                        panel_icon_button("undo", IconName::Undo)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tooltip(move |window, cx| {
                                Tooltip::with_meta(
                                    "Uncommit",
                                    Some(&git::Uncommit),
                                    if has_unstaged {
                                        "git reset HEAD^ --soft"
                                    } else {
                                        "git reset HEAD^"
                                    },
                                    window,
                                    cx,
                                )
                            })
                            .on_click(cx.listener(|this, _, window, cx| this.uncommit(window, cx))),
                    )
                }),
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
                    .gap_2()
                    .child(h_flex().w_full().justify_around().child(
                        if self.active_repository.is_some() {
                            "No changes to commit"
                        } else {
                            "No Git repositories"
                        },
                    ))
                    .children({
                        let worktree_count = self.project.read(cx).visible_worktrees(cx).count();
                        (worktree_count > 0 && self.active_repository.is_none()).then(|| {
                            h_flex().w_full().justify_around().child(
                                panel_filled_button("Initialize Repository")
                                    .tooltip(Tooltip::for_action_title_in(
                                        "git init",
                                        &git::Init,
                                        &self.focus_handle,
                                    ))
                                    .on_click(move |_, _, cx| {
                                        cx.defer(move |cx| {
                                            cx.dispatch_action(&git::Init);
                                        })
                                    }),
                            )
                        })
                    })
                    .text_ui_sm(cx)
                    .mx_auto()
                    .text_color(Color::Placeholder.color(cx)),
            )
    }

    fn render_vertical_scrollbar(
        &self,
        show_horizontal_scrollbar_container: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id("git-panel-vertical-scroll")
            .occlude()
            .flex_none()
            .h_full()
            .cursor_default()
            .absolute()
            .right_0()
            .top_0()
            .bottom_0()
            .w(px(12.))
            .when(show_horizontal_scrollbar_container, |this| {
                this.pb_neg_3p5()
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
                    if !this.vertical_scrollbar.state.is_dragging()
                        && !this.focus_handle.contains_focused(window, cx)
                    {
                        this.vertical_scrollbar.hide(window, cx);
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
                self.vertical_scrollbar.state.clone(),
            ))
    }

    /// Renders the horizontal scrollbar.
    ///
    /// The right offset is used to determine how far to the right the
    /// scrollbar should extend to, useful for ensuring it doesn't collide
    /// with the vertical scrollbar when visible.
    fn render_horizontal_scrollbar(
        &self,
        right_offset: Pixels,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id("git-panel-horizontal-scroll")
            .occlude()
            .flex_none()
            .w_full()
            .cursor_default()
            .absolute()
            .bottom_neg_px()
            .left_0()
            .right_0()
            .pr(right_offset)
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
                    if !this.horizontal_scrollbar.state.is_dragging()
                        && !this.focus_handle.contains_focused(window, cx)
                    {
                        this.horizontal_scrollbar.hide(window, cx);
                        cx.notify();
                    }

                    cx.stop_propagation();
                }),
            )
            .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                cx.notify();
            }))
            .children(Scrollbar::horizontal(
                // percentage as f32..end_offset as f32,
                self.horizontal_scrollbar.state.clone(),
            ))
    }

    fn render_buffer_header_controls(
        &self,
        entity: &Entity<Self>,
        file: &Arc<dyn File>,
        _: &Window,
        cx: &App,
    ) -> Option<AnyElement> {
        let repo = self.active_repository.as_ref()?.read(cx);
        let project_path = (file.worktree_id(cx), file.path()).into();
        let repo_path = repo.project_path_to_repo_path(&project_path, cx)?;
        let ix = self.entry_by_path(&repo_path, cx)?;
        let entry = self.entries.get(ix)?;

        let entry_staging = self.entry_staging(entry.status_entry()?);

        let checkbox = Checkbox::new("stage-file", entry_staging.as_bool().into())
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

        let scroll_track_size = px(16.);

        let h_scroll_offset = if self.vertical_scrollbar.show_scrollbar {
            // magic number
            px(3.)
        } else {
            px(0.)
        };

        v_flex()
            .flex_1()
            .size_full()
            .overflow_hidden()
            .relative()
            // Show a border on the top and bottom of the container when
            // the vertical scrollbar container is visible so we don't have a
            // floating left border in the panel.
            .when(self.vertical_scrollbar.show_track, |this| {
                this.border_t_1()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
            })
            .child(
                h_flex()
                    .flex_1()
                    .size_full()
                    .relative()
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
                        .when(
                            !self.horizontal_scrollbar.show_track
                                && self.horizontal_scrollbar.show_scrollbar,
                            |this| {
                                // when not showing the horizontal scrollbar track, make sure we don't
                                // obscure the last entry
                                this.pb(scroll_track_size)
                            },
                        )
                        .size_full()
                        .flex_grow()
                        .with_sizing_behavior(ListSizingBehavior::Auto)
                        .with_horizontal_sizing_behavior(
                            ListHorizontalSizingBehavior::Unconstrained,
                        )
                        .with_width_from_item(self.max_width_item_index)
                        .track_scroll(self.scroll_handle.clone()),
                    )
                    .on_mouse_down(
                        MouseButton::Right,
                        cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                            this.deploy_panel_context_menu(event.position, window, cx)
                        }),
                    )
                    .when(self.vertical_scrollbar.show_track, |this| {
                        this.child(
                            v_flex()
                                .h_full()
                                .flex_none()
                                .w(scroll_track_size)
                                .bg(cx.theme().colors().panel_background)
                                .child(
                                    div()
                                        .size_full()
                                        .flex_1()
                                        .border_l_1()
                                        .border_color(cx.theme().colors().border),
                                ),
                        )
                    })
                    .when(self.vertical_scrollbar.show_scrollbar, |this| {
                        this.child(
                            self.render_vertical_scrollbar(
                                self.horizontal_scrollbar.show_track,
                                cx,
                            ),
                        )
                    }),
            )
            .when(self.horizontal_scrollbar.show_track, |this| {
                this.child(
                    h_flex()
                        .w_full()
                        .h(scroll_track_size)
                        .flex_none()
                        .relative()
                        .child(
                            div()
                                .w_full()
                                .flex_1()
                                // for some reason the horizontal scrollbar is 1px
                                // taller than the vertical scrollbar??
                                .h(scroll_track_size - px(1.))
                                .bg(cx.theme().colors().panel_background)
                                .border_t_1()
                                .border_color(cx.theme().colors().border),
                        )
                        .when(self.vertical_scrollbar.show_track, |this| {
                            this.child(
                                div()
                                    .flex_none()
                                    // -1px prevents a missing pixel between the two container borders
                                    .w(scroll_track_size - px(1.))
                                    .h_full(),
                            )
                            .child(
                                // HACK: Fill the missing 1px 🥲
                                div()
                                    .absolute()
                                    .right(scroll_track_size - px(1.))
                                    .bottom(scroll_track_size - px(1.))
                                    .size_px()
                                    .bg(cx.theme().colors().border),
                            )
                        }),
                )
            })
            .when(self.horizontal_scrollbar.show_scrollbar, |this| {
                this.child(self.render_horizontal_scrollbar(h_scroll_offset, cx))
            })
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

    pub fn load_commit_details(
        &self,
        sha: String,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<CommitDetails>> {
        let Some(repo) = self.active_repository.clone() else {
            return Task::ready(Err(anyhow::anyhow!("no active repo")));
        };
        repo.update(cx, |repo, cx| {
            let show = repo.show(sha);
            cx.spawn(async move |_, _| show.await?)
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
        let stage_title = if entry.status.staging().is_fully_staged() {
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
                .context(self.focus_handle.clone())
                .action(stage_title, ToggleStaged.boxed_clone())
                .action(restore_title, git::RestoreFile::default().boxed_clone())
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
        let context_menu = git_panel_context_menu(
            self.focus_handle.clone(),
            GitMenuState {
                has_tracked_changes: self.has_tracked_changes(),
                has_staged_changes: self.has_staged_changes(),
                has_unstaged_changes: self.has_unstaged_changes(),
                has_new_changes: self.new_count > 0,
            },
            window,
            cx,
        );
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
        let display_name = entry.display_name();

        let selected = self.selected_entry == Some(ix);
        let marked = self.marked_entries.contains(&ix);
        let status_style = GitPanelSettings::get_global(cx).status_style;
        let status = entry.status;
        let modifiers = self.current_modifiers;
        let shift_held = modifiers.shift;

        let has_conflict = status.is_conflicted();
        let is_modified = status.is_modified();
        let is_deleted = status.is_deleted();

        let label_color = if status_style == StatusStyle::LabelColor {
            if has_conflict {
                Color::VersionControlConflict
            } else if is_modified {
                Color::VersionControlModified
            } else if is_deleted {
                // We don't want a bunch of red labels in the list
                Color::Disabled
            } else {
                Color::VersionControlAdded
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

        let entry_staging = self.entry_staging(entry);
        let mut is_staged: ToggleState = self.entry_staging(entry).as_bool().into();
        if self.show_placeholders && !self.has_staged_changes() && !entry.status.is_created() {
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
            .gap_1p5()
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
                                let is_staged = entry_staging.is_fully_staged();

                                let action = if is_staged { "Unstage" } else { "Stage" };
                                let tooltip_name = if shift_held {
                                    format!("{} section", action)
                                } else {
                                    action.to_string()
                                };

                                let meta = if shift_held {
                                    format!(
                                        "Release shift to {} single entry",
                                        action.to_lowercase()
                                    )
                                } else {
                                    format!("Shift click to {} section", action.to_lowercase())
                                };

                                Tooltip::with_meta(
                                    tooltip_name,
                                    Some(&ToggleStaged),
                                    meta,
                                    window,
                                    cx,
                                )
                            }),
                    ),
            )
            .child(git_status_icon(status))
            .child(
                h_flex()
                    .items_center()
                    .flex_1()
                    // .overflow_hidden()
                    .when_some(entry.parent_dir(), |this, parent| {
                        if !parent.is_empty() {
                            this.child(
                                self.entry_label(format!("{}/", parent), path_color)
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

    pub fn amend_pending(&self) -> bool {
        self.amend_pending
    }

    pub fn set_amend_pending(&mut self, value: bool, cx: &mut Context<Self>) {
        self.amend_pending = value;
        cx.notify();
    }
}

fn current_language_model(cx: &Context<'_, GitPanel>) -> Option<Arc<dyn LanguageModel>> {
    assistant_settings::AssistantSettings::get_global(cx)
        .enabled
        .then(|| {
            let ConfiguredModel { provider, model } =
                LanguageModelRegistry::read_global(cx).commit_message_model()?;

            provider.is_authenticated(cx).then(|| model)
        })
        .flatten()
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
                this.on_action(cx.listener(Self::toggle_staged_for_selected))
                    .on_action(cx.listener(GitPanel::commit))
                    .on_action(cx.listener(GitPanel::amend))
                    .on_action(cx.listener(GitPanel::cancel))
                    .on_action(cx.listener(Self::stage_all))
                    .on_action(cx.listener(Self::unstage_all))
                    .on_action(cx.listener(Self::stage_selected))
                    .on_action(cx.listener(Self::unstage_selected))
                    .on_action(cx.listener(Self::restore_tracked_files))
                    .on_action(cx.listener(Self::revert_selected))
                    .on_action(cx.listener(Self::clean_all))
                    .on_action(cx.listener(Self::generate_commit_message_action))
            })
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::close_panel))
            .on_action(cx.listener(Self::open_diff))
            .on_action(cx.listener(Self::open_file))
            .on_action(cx.listener(Self::focus_changes_list))
            .on_action(cx.listener(Self::focus_editor))
            .on_action(cx.listener(Self::expand_commit_editor))
            .when(has_write_access && has_co_authors, |git_panel| {
                git_panel.on_action(cx.listener(Self::toggle_fill_co_authors))
            })
            .on_hover(cx.listener(move |this, hovered, window, cx| {
                if *hovered {
                    this.horizontal_scrollbar.show(cx);
                    this.vertical_scrollbar.show(cx);
                    cx.notify();
                } else if !this.focus_handle.contains_focused(window, cx) {
                    this.hide_scrollbars(window, cx);
                }
            }))
            .size_full()
            .overflow_hidden()
            .bg(cx.theme().colors().panel_background)
            .child(
                v_flex()
                    .size_full()
                    .children(self.render_panel_header(window, cx))
                    .map(|this| {
                        if has_entries {
                            this.child(self.render_entries(has_write_access, window, cx))
                        } else {
                            this.child(self.render_empty_state(cx).into_any_element())
                        }
                    })
                    .children(self.render_footer(window, cx))
                    .when(self.amend_pending, |this| {
                        this.child(self.render_pending_amend(cx))
                    })
                    .when(!self.amend_pending, |this| {
                        this.children(self.render_previous_commit(cx))
                    })
                    .into_any_element(),
            )
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(Corner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

impl Focusable for GitPanel {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        if self.entries.is_empty() {
            self.commit_editor.focus_handle(cx)
        } else {
            self.focus_handle.clone()
        }
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
        Some(ui::IconName::GitBranchSmall).filter(|_| GitPanelSettings::get_global(cx).button)
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
        repository: Entity<Repository>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            cx.spawn_in(window, async move |this, cx| {
                let (details, workspace) = git_panel.update(cx, |git_panel, cx| {
                    (
                        git_panel.load_commit_details(sha.to_string(), cx),
                        git_panel.workspace.clone(),
                    )
                })?;
                let details = details.await?;

                let commit_details = crate::commit_tooltip::CommitDetails {
                    sha: details.sha.clone(),
                    author_name: details.author_name.clone(),
                    author_email: details.author_email.clone(),
                    commit_time: OffsetDateTime::from_unix_timestamp(details.commit_timestamp)?,
                    message: Some(ParsedCommitMessage {
                        message: details.message.clone(),
                        ..Default::default()
                    }),
                };

                this.update(cx, |this: &mut GitPanelMessageTooltip, cx| {
                    this.commit_tooltip = Some(cx.new(move |cx| {
                        CommitTooltip::new(commit_details, repository, workspace, cx)
                    }));
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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(commit_tooltip) = &self.commit_tooltip {
            commit_tooltip.clone().into_any_element()
        } else {
            gpui::Empty.into_any_element()
        }
    }
}

#[derive(IntoElement, RegisterComponent)]
pub struct PanelRepoFooter {
    active_repository: SharedString,
    branch: Option<Branch>,
    head_commit: Option<CommitDetails>,

    // Getting a GitPanel in previews will be difficult.
    //
    // For now just take an option here, and we won't bind handlers to buttons in previews.
    git_panel: Option<Entity<GitPanel>>,
}

impl PanelRepoFooter {
    pub fn new(
        active_repository: SharedString,
        branch: Option<Branch>,
        head_commit: Option<CommitDetails>,
        git_panel: Option<Entity<GitPanel>>,
    ) -> Self {
        Self {
            active_repository,
            branch,
            head_commit,
            git_panel,
        }
    }

    pub fn new_preview(active_repository: SharedString, branch: Option<Branch>) -> Self {
        Self {
            active_repository,
            branch,
            head_commit: None,
            git_panel: None,
        }
    }
}

impl RenderOnce for PanelRepoFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let project = self
            .git_panel
            .as_ref()
            .map(|panel| panel.read(cx).project.clone());

        let repo = self
            .git_panel
            .as_ref()
            .and_then(|panel| panel.read(cx).active_repository.clone());

        let single_repo = project
            .as_ref()
            .map(|project| project.read(cx).git_store().read(cx).repositories().len() == 1)
            .unwrap_or(true);

        const MAX_BRANCH_LEN: usize = 16;
        const MAX_REPO_LEN: usize = 16;
        const LABEL_CHARACTER_BUDGET: usize = MAX_BRANCH_LEN + MAX_REPO_LEN;
        const MAX_SHORT_SHA_LEN: usize = 8;

        let branch_name = self
            .branch
            .as_ref()
            .map(|branch| branch.name().to_owned())
            .or_else(|| {
                self.head_commit.as_ref().map(|commit| {
                    commit
                        .sha
                        .chars()
                        .take(MAX_SHORT_SHA_LEN)
                        .collect::<String>()
                })
            })
            .unwrap_or_else(|| " (no branch)".to_owned());
        let show_separator = self.branch.is_some() || self.head_commit.is_some();

        let active_repo_name = self.active_repository.clone();

        let branch_actual_len = branch_name.len();
        let repo_actual_len = active_repo_name.len();

        // ideally, show the whole branch and repo names but
        // when we can't, use a budget to allocate space between the two
        let (repo_display_len, branch_display_len) = if branch_actual_len + repo_actual_len
            <= LABEL_CHARACTER_BUDGET
        {
            (repo_actual_len, branch_actual_len)
        } else {
            if branch_actual_len <= MAX_BRANCH_LEN {
                let repo_space = (LABEL_CHARACTER_BUDGET - branch_actual_len).min(MAX_REPO_LEN);
                (repo_space, branch_actual_len)
            } else if repo_actual_len <= MAX_REPO_LEN {
                let branch_space = (LABEL_CHARACTER_BUDGET - repo_actual_len).min(MAX_BRANCH_LEN);
                (repo_actual_len, branch_space)
            } else {
                (MAX_REPO_LEN, MAX_BRANCH_LEN)
            }
        };

        let truncated_repo_name = if repo_actual_len <= repo_display_len {
            active_repo_name.to_string()
        } else {
            util::truncate_and_trailoff(active_repo_name.trim_ascii(), repo_display_len)
        };

        let truncated_branch_name = if branch_actual_len <= branch_display_len {
            branch_name.to_string()
        } else {
            util::truncate_and_trailoff(branch_name.trim_ascii(), branch_display_len)
        };

        let repo_selector_trigger = Button::new("repo-selector", truncated_repo_name)
            .style(ButtonStyle::Transparent)
            .size(ButtonSize::None)
            .label_size(LabelSize::Small)
            .color(Color::Muted);

        let repo_selector = PopoverMenu::new("repository-switcher")
            .menu({
                let project = project.clone();
                move |window, cx| {
                    let project = project.clone()?;
                    Some(cx.new(|cx| RepositorySelector::new(project, rems(16.), window, cx)))
                }
            })
            .trigger_with_tooltip(
                repo_selector_trigger.disabled(single_repo).truncate(true),
                Tooltip::text("Switch active repository"),
            )
            .anchor(Corner::BottomLeft)
            .into_any_element();

        let branch_selector_button = Button::new("branch-selector", truncated_branch_name)
            .style(ButtonStyle::Transparent)
            .size(ButtonSize::None)
            .label_size(LabelSize::Small)
            .truncate(true)
            .tooltip(Tooltip::for_action_title(
                "Switch Branch",
                &zed_actions::git::Switch,
            ))
            .on_click(|_, window, cx| {
                window.dispatch_action(zed_actions::git::Switch.boxed_clone(), cx);
            });

        let branch_selector = PopoverMenu::new("popover-button")
            .menu(move |window, cx| Some(branch_picker::popover(repo.clone(), window, cx)))
            .trigger_with_tooltip(
                branch_selector_button,
                Tooltip::for_action_title("Switch Branch", &zed_actions::git::Switch),
            )
            .anchor(Corner::BottomLeft)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            });

        h_flex()
            .w_full()
            .px_2()
            .h(px(36.))
            .items_center()
            .justify_between()
            .gap_1()
            .child(
                h_flex()
                    .flex_1()
                    .overflow_hidden()
                    .items_center()
                    .child(
                        div().child(
                            Icon::new(IconName::GitBranchSmall)
                                .size(IconSize::Small)
                                .color(if single_repo {
                                    Color::Disabled
                                } else {
                                    Color::Muted
                                }),
                        ),
                    )
                    .child(repo_selector)
                    .when(show_separator, |this| {
                        this.child(
                            div()
                                .text_color(cx.theme().colors().text_muted)
                                .text_sm()
                                .child("/"),
                        )
                    })
                    .child(branch_selector),
            )
            .children(if let Some(git_panel) = self.git_panel {
                git_panel.update(cx, |git_panel, cx| git_panel.render_remote_button(cx))
            } else {
                None
            })
    }
}

impl Component for PanelRepoFooter {
    fn scope() -> ComponentScope {
        ComponentScope::VersionControl
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
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
                ref_name: "some-branch".into(),
                upstream: upstream.map(|tracking| Upstream {
                    ref_name: "origin/some-branch".into(),
                    tracking,
                }),
                most_recent_commit: Some(CommitSummary {
                    sha: "abc123".into(),
                    subject: "Modify stuff".into(),
                    commit_timestamp: 1710932954,
                    has_parent: true,
                }),
            }
        }

        fn custom(branch_name: &str, upstream: Option<UpstreamTracking>) -> Branch {
            Branch {
                is_head: true,
                ref_name: branch_name.to_string().into(),
                upstream: upstream.map(|tracking| Upstream {
                    ref_name: format!("zed/{}", branch_name).into(),
                    tracking,
                }),
                most_recent_commit: Some(CommitSummary {
                    sha: "abc123".into(),
                    subject: "Modify stuff".into(),
                    commit_timestamp: 1710932954,
                    has_parent: true,
                }),
            }
        }

        fn active_repository(id: usize) -> SharedString {
            format!("repo-{}", id).into()
        }

        let example_width = px(340.);
        Some(
            v_flex()
                .gap_6()
                .w_full()
                .flex_none()
                .children(vec![
                    example_group_with_title(
                        "Action Button States",
                        vec![
                            single_example(
                                "No Branch",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(1).clone(),
                                        None,
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Remote status unknown",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(2).clone(),
                                        Some(branch(unknown_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "No Remote Upstream",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(3).clone(),
                                        Some(branch(no_remote_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Not Ahead or Behind",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(4).clone(),
                                        Some(branch(not_ahead_or_behind_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Behind remote",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(5).clone(),
                                        Some(branch(behind_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Ahead of remote",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(6).clone(),
                                        Some(branch(ahead_of_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Ahead and behind remote",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(7).clone(),
                                        Some(branch(ahead_and_behind_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                        ],
                    )
                    .grow()
                    .vertical(),
                ])
                .children(vec![
                    example_group_with_title(
                        "Labels",
                        vec![
                            single_example(
                                "Short Branch & Repo",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        SharedString::from("zed"),
                                        Some(custom("main", behind_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Long Branch",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        SharedString::from("zed"),
                                        Some(custom(
                                            "redesign-and-update-git-ui-list-entry-style",
                                            behind_upstream,
                                        )),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Long Repo",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        SharedString::from("zed-industries-community-examples"),
                                        Some(custom("gpui", ahead_of_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Long Repo & Branch",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        SharedString::from("zed-industries-community-examples"),
                                        Some(custom(
                                            "redesign-and-update-git-ui-list-entry-style",
                                            behind_upstream,
                                        )),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Uppercase Repo",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        SharedString::from("LICENSES"),
                                        Some(custom("main", ahead_of_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Uppercase Branch",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        SharedString::from("zed"),
                                        Some(custom("update-README", behind_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                        ],
                    )
                    .grow()
                    .vertical(),
                ])
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use git::status::StatusCode;
    use gpui::TestAppContext;
    use project::{FakeFs, WorktreeSettings};
    use serde_json::json;
    use settings::SettingsStore;
    use theme::LoadThemes;
    use util::path;

    use super::*;

    fn init_test(cx: &mut gpui::TestAppContext) {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::try_init().ok();
        }

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            AssistantSettings::register(cx);
            WorktreeSettings::register(cx);
            workspace::init_settings(cx);
            theme::init(LoadThemes::JustBase, cx);
            language::init(cx);
            editor::init(cx);
            Project::init_settings(cx);
            crate::init(cx);
        });
    }

    #[gpui::test]
    async fn test_entry_worktree_paths(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                "zed": {
                    ".git": {},
                    "crates": {
                        "gpui": {
                            "gpui.rs": "fn main() {}"
                        },
                        "util": {
                            "util.rs": "fn do_it() {}"
                        }
                    }
                },
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/zed/.git")),
            &[
                (
                    Path::new("crates/gpui/gpui.rs"),
                    StatusCode::Modified.worktree(),
                ),
                (
                    Path::new("crates/util/util.rs"),
                    StatusCode::Modified.worktree(),
                ),
            ],
        );

        let project =
            Project::test(fs.clone(), [path!("/root/zed/crates/gpui").as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        cx.read(|cx| {
            project
                .read(cx)
                .worktrees(cx)
                .nth(0)
                .unwrap()
                .read(cx)
                .as_local()
                .unwrap()
                .scan_complete()
        })
        .await;

        cx.executor().run_until_parked();

        let app_state = workspace.update(cx, |workspace, _| workspace.app_state().clone());
        let panel = cx.new_window_entity(|window, cx| {
            GitPanel::new(workspace.clone(), project.clone(), app_state, window, cx)
        });

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        let entries = panel.update(cx, |panel, _| panel.entries.clone());
        pretty_assertions::assert_eq!(
            entries,
            [
                GitListEntry::Header(GitHeaderEntry {
                    header: Section::Tracked
                }),
                GitListEntry::GitStatusEntry(GitStatusEntry {
                    abs_path: path!("/root/zed/crates/gpui/gpui.rs").into(),
                    repo_path: "crates/gpui/gpui.rs".into(),
                    status: StatusCode::Modified.worktree(),
                    staging: StageStatus::Unstaged,
                }),
                GitListEntry::GitStatusEntry(GitStatusEntry {
                    abs_path: path!("/root/zed/crates/util/util.rs").into(),
                    repo_path: "crates/util/util.rs".into(),
                    status: StatusCode::Modified.worktree(),
                    staging: StageStatus::Unstaged,
                },),
            ],
        );

        // TODO(cole) restore this once repository deduplication is implemented properly.
        //cx.update_window_entity(&panel, |panel, window, cx| {
        //    panel.select_last(&Default::default(), window, cx);
        //    assert_eq!(panel.selected_entry, Some(2));
        //    panel.open_diff(&Default::default(), window, cx);
        //});
        //cx.run_until_parked();

        //let worktree_roots = workspace.update(cx, |workspace, cx| {
        //    workspace
        //        .worktrees(cx)
        //        .map(|worktree| worktree.read(cx).abs_path())
        //        .collect::<Vec<_>>()
        //});
        //pretty_assertions::assert_eq!(
        //    worktree_roots,
        //    vec![
        //        Path::new(path!("/root/zed/crates/gpui")).into(),
        //        Path::new(path!("/root/zed/crates/util/util.rs")).into(),
        //    ]
        //);

        //project.update(cx, |project, cx| {
        //    let git_store = project.git_store().read(cx);
        //    // The repo that comes from the single-file worktree can't be selected through the UI.
        //    let filtered_entries = filtered_repository_entries(git_store, cx)
        //        .iter()
        //        .map(|repo| repo.read(cx).worktree_abs_path.clone())
        //        .collect::<Vec<_>>();
        //    assert_eq!(
        //        filtered_entries,
        //        [Path::new(path!("/root/zed/crates/gpui")).into()]
        //    );
        //    // But we can select it artificially here.
        //    let repo_from_single_file_worktree = git_store
        //        .repositories()
        //        .values()
        //        .find(|repo| {
        //            repo.read(cx).worktree_abs_path.as_ref()
        //                == Path::new(path!("/root/zed/crates/util/util.rs"))
        //        })
        //        .unwrap()
        //        .clone();

        //    // Paths still make sense when we somehow activate a repo that comes from a single-file worktree.
        //    repo_from_single_file_worktree.update(cx, |repo, cx| repo.set_as_active_repository(cx));
        //});

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;
        let entries = panel.update(cx, |panel, _| panel.entries.clone());
        pretty_assertions::assert_eq!(
            entries,
            [
                GitListEntry::Header(GitHeaderEntry {
                    header: Section::Tracked
                }),
                GitListEntry::GitStatusEntry(GitStatusEntry {
                    abs_path: path!("/root/zed/crates/gpui/gpui.rs").into(),
                    repo_path: "crates/gpui/gpui.rs".into(),
                    status: StatusCode::Modified.worktree(),
                    staging: StageStatus::Unstaged,
                }),
                GitListEntry::GitStatusEntry(GitStatusEntry {
                    abs_path: path!("/root/zed/crates/util/util.rs").into(),
                    repo_path: "crates/util/util.rs".into(),
                    status: StatusCode::Modified.worktree(),
                    staging: StageStatus::Unstaged,
                },),
            ],
        );
    }
}
