use crate::git_panel_settings::StatusStyle;
use crate::repository_selector::RepositorySelectorPopoverMenu;
use crate::{
    git_panel_settings::GitPanelSettings, git_status_icon, repository_selector::RepositorySelector,
};
use anyhow::Result;
use db::kvp::KEY_VALUE_STORE;
use editor::actions::MoveToEnd;
use editor::scroll::ScrollbarAutoHide;
use editor::{Editor, EditorMode, EditorSettings, MultiBuffer, ShowScrollbar};
use futures::channel::mpsc;
use futures::StreamExt as _;
use git::repository::RepoPath;
use git::status::FileStatus;
use git::{CommitAllChanges, CommitChanges, RevertAll, StageAll, ToggleStaged, UnstageAll};
use gpui::*;
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrev};
use project::git::RepositoryHandle;
use project::{Fs, Project, ProjectPath};
use serde::{Deserialize, Serialize};
use settings::Settings as _;
use std::{collections::HashSet, ops::Range, path::PathBuf, sync::Arc, time::Duration, usize};
use theme::ThemeSettings;
use ui::{
    prelude::*, ButtonLike, Checkbox, Divider, DividerColor, ElevationIndex, ListItem,
    ListItemSpacing, Scrollbar, ScrollbarState, Tooltip,
};
use util::{ResultExt, TryFutureExt};
use workspace::notifications::{DetachAndPromptErr, NotificationId};
use workspace::Toast;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

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
    current_modifiers: Modifiers,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    hide_scrollbar_task: Option<Task<()>>,
    pending_serialization: Task<Option<()>>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    active_repository: Option<RepositoryHandle>,
    scroll_handle: UniformListScrollHandle,
    scrollbar_state: ScrollbarState,
    selected_entry: Option<usize>,
    show_scrollbar: bool,
    update_visible_entries_task: Task<()>,
    repository_selector: Entity<RepositorySelector>,
    commit_editor: Entity<Editor>,
    visible_entries: Vec<GitListEntry>,
    all_staged: Option<bool>,
    width: Option<Pixels>,
    err_sender: mpsc::Sender<anyhow::Error>,
}

fn commit_message_editor(
    active_repository: Option<&RepositoryHandle>,
    window: &mut Window,
    cx: &mut Context<'_, Editor>,
) -> Editor {
    let theme = ThemeSettings::get_global(cx);

    let mut text_style = window.text_style();
    let refinement = TextStyleRefinement {
        font_family: Some(theme.buffer_font.family.clone()),
        font_features: Some(FontFeatures::disable_ligatures()),
        font_size: Some(px(12.).into()),
        color: Some(cx.theme().colors().editor_foreground),
        background_color: Some(gpui::transparent_black()),
        ..Default::default()
    };
    text_style.refine(&refinement);

    let mut commit_editor = if let Some(active_repository) = active_repository.as_ref() {
        let buffer = cx.new(|cx| MultiBuffer::singleton(active_repository.commit_message(), cx));
        Editor::new(
            EditorMode::AutoHeight { max_lines: 10 },
            buffer,
            None,
            false,
            window,
            cx,
        )
    } else {
        Editor::auto_height(10, window, cx)
    };
    commit_editor.set_use_autoclose(false);
    commit_editor.set_show_gutter(false, cx);
    commit_editor.set_show_wrap_guides(false, cx);
    commit_editor.set_show_indent_guides(false, cx);
    commit_editor.set_text_style_refinement(refinement);
    commit_editor.set_placeholder_text("Enter commit message", cx);
    commit_editor
}

impl GitPanel {
    pub fn load(
        workspace: WeakEntity<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<Entity<Self>>> {
        cx.spawn(|mut cx| async move { workspace.update_in(&mut cx, Self::new) })
    }

    pub fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let fs = workspace.app_state().fs.clone();
        let project = workspace.project().clone();
        let git_state = project.read(cx).git_state().cloned();
        let active_repository = project.read(cx).active_repository(cx);
        let (err_sender, mut err_receiver) = mpsc::channel(1);
        let workspace = cx.entity().downgrade();

        let git_panel = cx.new(|cx| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, window, Self::focus_in).detach();
            cx.on_focus_out(&focus_handle, window, |this, _, window, cx| {
                this.hide_scrollbar(window, cx);
            })
            .detach();

            let commit_editor =
                cx.new(|cx| commit_message_editor(active_repository.as_ref(), window, cx));

            let scroll_handle = UniformListScrollHandle::new();

            if let Some(git_state) = git_state {
                cx.subscribe_in(
                    &git_state,
                    window,
                    move |this, git_state, event, window, cx| match event {
                        project::git::Event::RepositoriesUpdated => {
                            this.active_repository = git_state.read(cx).active_repository();
                            this.schedule_update(window, cx);
                        }
                    },
                )
                .detach();
            }

            let repository_selector =
                cx.new(|cx| RepositorySelector::new(project.clone(), window, cx));

            let mut git_panel = Self {
                focus_handle: cx.focus_handle(),
                pending_serialization: Task::ready(None),
                visible_entries: Vec::new(),
                all_staged: None,
                current_modifiers: window.modifiers(),
                width: Some(px(360.)),
                scrollbar_state: ScrollbarState::new(scroll_handle.clone())
                    .parent_model(&cx.entity()),
                repository_selector,
                selected_entry: None,
                show_scrollbar: false,
                hide_scrollbar_task: None,
                update_visible_entries_task: Task::ready(()),
                active_repository,
                scroll_handle,
                fs,
                commit_editor,
                project,
                err_sender,
                workspace,
            };
            git_panel.schedule_update(window, cx);
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

        cx.subscribe_in(
            &git_panel,
            window,
            move |workspace, _, event: &Event, window, cx| match event.clone() {
                Event::OpenedEntry { path } => {
                    workspace
                        .open_path_preview(path, None, false, false, window, cx)
                        .detach_and_prompt_err("Failed to open file", window, cx, |e, _, _| {
                            Some(format!("{e}"))
                        });
                }
                Event::Focus => { /* TODO */ }
            },
        )
        .detach();

        git_panel
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
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
        if self.visible_entries.first().is_some() {
            self.selected_entry = Some(0);
            self.scroll_to_selected_entry(cx);
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, _window: &mut Window, cx: &mut Context<Self>) {
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

    fn select_next(&mut self, _: &SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
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

    fn select_last(&mut self, _: &SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        if self.visible_entries.last().is_some() {
            self.selected_entry = Some(self.visible_entries.len() - 1);
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
                active_repository.entry_count() > 0
            });
        if have_entries && self.selected_entry.is_none() {
            self.selected_entry = Some(0);
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
        self.selected_entry
            .and_then(|i| self.visible_entries.get(i))
    }

    fn open_selected(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(entry) = self
            .selected_entry
            .and_then(|i| self.visible_entries.get(i))
        {
            self.open_entry(entry, cx);
        }
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
        let result = if entry.status.is_staged().unwrap_or(false) {
            active_repository
                .unstage_entries(vec![entry.repo_path.clone()], self.err_sender.clone())
        } else {
            active_repository.stage_entries(vec![entry.repo_path.clone()], self.err_sender.clone())
        };
        if let Err(e) = result {
            self.show_err_toast("toggle staged error", e, cx);
        }
        cx.notify();
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

    fn open_entry(&self, entry: &GitListEntry, cx: &mut Context<Self>) {
        let Some(active_repository) = self.active_repository.as_ref() else {
            return;
        };
        let Some(path) = active_repository.unrelativize(&entry.repo_path) else {
            return;
        };
        let path_exists = self.project.update(cx, |project, cx| {
            project.entry_for_path(&path, cx).is_some()
        });
        if !path_exists {
            return;
        }
        // TODO maybe move all of this into project?
        cx.emit(Event::OpenedEntry { path });
    }

    fn stage_all(&mut self, _: &git::StageAll, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_repository) = self.active_repository.as_ref() else {
            return;
        };
        for entry in &mut self.visible_entries {
            entry.is_staged = Some(true);
        }
        self.all_staged = Some(true);

        if let Err(e) = active_repository.stage_all(self.err_sender.clone()) {
            self.show_err_toast("stage all error", e, cx);
        };
    }

    fn unstage_all(&mut self, _: &git::UnstageAll, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_repository) = self.active_repository.as_ref() else {
            return;
        };
        for entry in &mut self.visible_entries {
            entry.is_staged = Some(false);
        }
        self.all_staged = Some(false);
        if let Err(e) = active_repository.unstage_all(self.err_sender.clone()) {
            self.show_err_toast("unstage all error", e, cx);
        };
    }

    fn discard_all(&mut self, _: &git::RevertAll, _window: &mut Window, _cx: &mut Context<Self>) {
        // TODO: Implement discard all
        println!("Discard all triggered");
    }

    /// Commit all staged changes
    fn commit_changes(
        &mut self,
        _: &git::CommitChanges,
        name_and_email: Option<(SharedString, SharedString)>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_repository) = self.active_repository.as_ref() else {
            return;
        };
        if !active_repository.can_commit(false, cx) {
            return;
        }
        active_repository.commit(name_and_email, self.err_sender.clone(), cx);
    }

    /// Commit all changes, regardless of whether they are staged or not
    fn commit_all_changes(
        &mut self,
        _: &git::CommitAllChanges,
        name_and_email: Option<(SharedString, SharedString)>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_repository) = self.active_repository.as_ref() else {
            return;
        };
        if !active_repository.can_commit(true, cx) {
            return;
        }
        active_repository.commit_all(name_and_email, self.err_sender.clone(), cx);
    }

    fn fill_co_authors(&mut self, _: &FillCoAuthors, window: &mut Window, cx: &mut Context<Self>) {
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
            .map(|participant| participant.user.as_ref())
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
            editor.move_to_end(&MoveToEnd, window, cx);
            editor.focus_handle(cx).focus(window);
        });
    }

    fn for_each_visible_entry(
        &self,
        range: Range<usize>,
        cx: &mut Context<Self>,
        mut callback: impl FnMut(usize, GitListEntry, &mut Context<Self>),
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

    fn schedule_update(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let handle = cx.entity().downgrade();
        self.update_visible_entries_task = cx.spawn_in(window, |_, mut cx| async move {
            cx.background_executor().timer(UPDATE_DEBOUNCE).await;
            if let Some(this) = handle.upgrade() {
                this.update_in(&mut cx, |this, window, cx| {
                    this.update_visible_entries(cx);
                    let active_repository = this.active_repository.as_ref();
                    this.commit_editor =
                        cx.new(|cx| commit_message_editor(active_repository, window, cx));
                })
                .ok();
            }
        });
    }

    fn update_visible_entries(&mut self, cx: &mut Context<Self>) {
        self.visible_entries.clear();

        let Some(repo) = self.active_repository.as_ref() else {
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
                repo_path: entry.repo_path.clone(),
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

    fn show_err_toast(&self, id: &'static str, e: anyhow::Error, cx: &mut Context<Self>) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let notif_id = NotificationId::Named(id.into());
        let message = e.to_string();
        workspace.update(cx, |workspace, cx| {
            let toast = Toast::new(notif_id, message).on_click("Open Zed Log", |window, cx| {
                window.dispatch_action(workspace::OpenLog.boxed_clone(), cx);
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

    pub fn render_divider(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .items_center()
            .h(px(8.))
            .child(Divider::horizontal_dashed().color(DividerColor::Border))
    }

    pub fn render_panel_header(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx).clone();
        let all_repositories = self
            .project
            .read(cx)
            .git_state()
            .map(|state| state.read(cx).all_repositories())
            .unwrap_or_default();
        let entry_count = self
            .active_repository
            .as_ref()
            .map_or(0, RepositoryHandle::entry_count);

        let changes_string = match entry_count {
            0 => "No changes".to_string(),
            1 => "1 change".to_string(),
            n => format!("{} changes", n),
        };

        h_flex()
            .h(px(32.))
            .items_center()
            .px_2()
            .bg(ElevationIndex::Surface.bg(cx))
            .child(h_flex().gap_2().child(if all_repositories.len() <= 1 {
                div()
                    .id("changes-label")
                    .text_buffer(cx)
                    .text_ui_sm(cx)
                    .child(
                        Label::new(changes_string)
                            .single_line()
                            .size(LabelSize::Small),
                    )
                    .into_any_element()
            } else {
                self.render_repository_selector(cx).into_any_element()
            }))
            .child(div().flex_grow())
            .child(h_flex().gap_2().child(if self.all_staged.unwrap_or(false) {
                self.panel_button("unstage-all", "Unstage All")
                    .tooltip({
                        let focus_handle = focus_handle.clone();
                        move |window, cx| {
                            Tooltip::for_action_in(
                                "Unstage all changes",
                                &UnstageAll,
                                &focus_handle,
                                window,
                                cx,
                            )
                        }
                    })
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.unstage_all(&UnstageAll, window, cx)
                    }))
            } else {
                self.panel_button("stage-all", "Stage All")
                    .tooltip({
                        let focus_handle = focus_handle.clone();
                        move |window, cx| {
                            Tooltip::for_action_in(
                                "Stage all changes",
                                &StageAll,
                                &focus_handle,
                                window,
                                cx,
                            )
                        }
                    })
                    .on_click(
                        cx.listener(move |this, _, window, cx| {
                            this.stage_all(&StageAll, window, cx)
                        }),
                    )
            }))
    }

    pub fn render_repository_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let active_repository = self.project.read(cx).active_repository(cx);
        let repository_display_name = active_repository
            .as_ref()
            .map(|repo| repo.display_name(self.project.read(cx), cx))
            .unwrap_or_default();

        let entry_count = self.visible_entries.len();

        RepositorySelectorPopoverMenu::new(
            self.repository_selector.clone(),
            ButtonLike::new("active-repository")
                .style(ButtonStyle::Subtle)
                .child(
                    h_flex().w_full().gap_0p5().child(
                        div()
                            .overflow_x_hidden()
                            .flex_grow()
                            .whitespace_nowrap()
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        Label::new(repository_display_name).size(LabelSize::Small),
                                    )
                                    .when(entry_count > 0, |flex| {
                                        flex.child(
                                            Label::new(format!("({})", entry_count))
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                    })
                                    .into_any_element(),
                            ),
                    ),
                ),
        )
    }

    pub fn render_commit_editor(
        &self,
        name_and_email: Option<(SharedString, SharedString)>,
        can_commit: bool,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let editor = self.commit_editor.clone();
        let editor_focus_handle = editor.read(cx).focus_handle(cx).clone();
        let (can_commit, can_commit_all) =
            self.active_repository
                .as_ref()
                .map_or((false, false), |active_repository| {
                    (
                        can_commit && active_repository.can_commit(false, cx),
                        can_commit && active_repository.can_commit(true, cx),
                    )
                });

        let focus_handle_1 = self.focus_handle(cx).clone();
        let focus_handle_2 = self.focus_handle(cx).clone();

        let commit_staged_button = self
            .panel_button("commit-staged-changes", "Commit")
            .tooltip(move |window, cx| {
                let focus_handle = focus_handle_1.clone();
                Tooltip::for_action_in(
                    "Commit all staged changes",
                    &CommitChanges,
                    &focus_handle,
                    window,
                    cx,
                )
            })
            .disabled(!can_commit)
            .on_click({
                let name_and_email = name_and_email.clone();
                cx.listener(move |this, _: &ClickEvent, window, cx| {
                    this.commit_changes(&CommitChanges, name_and_email.clone(), window, cx)
                })
            });

        let commit_all_button = self
            .panel_button("commit-all-changes", "Commit All")
            .tooltip(move |window, cx| {
                let focus_handle = focus_handle_2.clone();
                Tooltip::for_action_in(
                    "Commit all changes, including unstaged changes",
                    &CommitAllChanges,
                    &focus_handle,
                    window,
                    cx,
                )
            })
            .disabled(!can_commit_all)
            .on_click({
                let name_and_email = name_and_email.clone();
                cx.listener(move |this, _: &ClickEvent, window, cx| {
                    this.commit_all_changes(&CommitAllChanges, name_and_email.clone(), window, cx)
                })
            });

        div().w_full().h(px(140.)).px_2().pt_1().pb_2().child(
            v_flex()
                .id("commit-editor-container")
                .relative()
                .h_full()
                .py_2p5()
                .px_3()
                .bg(cx.theme().colors().editor_background)
                .on_click(cx.listener(move |_, _: &ClickEvent, window, _cx| {
                    window.focus(&editor_focus_handle);
                }))
                .child(self.commit_editor.clone())
                .child(
                    h_flex()
                        .absolute()
                        .bottom_2p5()
                        .right_3()
                        .gap_1p5()
                        .child(div().gap_1().flex_grow())
                        .child(commit_all_button)
                        .child(commit_staged_button),
                ),
        )
    }

    fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
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

    fn render_entries(&self, has_write_access: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let entry_count = self.visible_entries.len();

        v_flex()
            .size_full()
            .overflow_hidden()
            .child(
                uniform_list(cx.entity().clone(), "entries", entry_count, {
                    move |git_panel, range, _window, cx| {
                        let mut items = Vec::with_capacity(range.end - range.start);
                        git_panel.for_each_visible_entry(range, cx, |ix, details, cx| {
                            items.push(git_panel.render_entry(ix, details, has_write_access, cx));
                        });
                        items
                    }
                })
                .size_full()
                .with_sizing_behavior(ListSizingBehavior::Infer)
                .with_horizontal_sizing_behavior(ListHorizontalSizingBehavior::Unconstrained)
                .track_scroll(self.scroll_handle.clone()),
            )
            .children(self.render_scrollbar(cx))
    }

    fn entry_label(&self, label: impl Into<SharedString>, color: Color) -> Label {
        Label::new(label.into()).color(color).single_line()
    }

    fn render_entry(
        &self,
        ix: usize,
        entry_details: GitListEntry,
        has_write_access: bool,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let repo_path = entry_details.repo_path.clone();
        let selected = self.selected_entry == Some(ix);
        let status_style = GitPanelSettings::get_global(cx).status_style;
        let status = entry_details.status;
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

        let id: ElementId = ElementId::Name(format!("entry_{}", entry_details.display_name).into());

        let checkbox = Checkbox::new(
            id,
            entry_details
                .is_staged
                .map_or(ToggleState::Indeterminate, ToggleState::from),
        )
        .disabled(!has_write_access)
        .fill()
        .elevation(ElevationIndex::Surface)
        .on_click({
            let handle = cx.entity().downgrade();
            let repo_path = repo_path.clone();
            move |toggle, _window, cx| {
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
                    let Some(active_repository) = this.active_repository.as_ref() else {
                        return;
                    };
                    let result = match toggle {
                        ToggleState::Selected | ToggleState::Indeterminate => active_repository
                            .stage_entries(vec![repo_path], this.err_sender.clone()),
                        ToggleState::Unselected => active_repository
                            .unstage_entries(vec![repo_path], this.err_sender.clone()),
                    };
                    if let Err(e) = result {
                        this.show_err_toast("toggle staged error", e, cx);
                    }
                });
            }
        });

        let start_slot = h_flex()
            .gap(DynamicSpacing::Base04.rems(cx))
            .child(checkbox)
            .child(git_status_icon(status, cx));

        let id = ElementId::Name(format!("entry_{}", entry_details.display_name).into());

        div().w_full().px_0p5().child(
            ListItem::new(id)
                .spacing(ListItemSpacing::Sparse)
                .start_slot(start_slot)
                .toggle_state(selected)
                .disabled(!has_write_access)
                .on_click({
                    let handle = cx.entity().downgrade();
                    move |_, window, cx| {
                        let Some(this) = handle.upgrade() else {
                            return;
                        };
                        this.update(cx, |this, cx| {
                            this.selected_entry = Some(ix);
                            window.dispatch_action(Box::new(OpenSelected), cx);
                            cx.notify();
                        });
                    }
                })
                .child(
                    h_flex()
                        .when_some(repo_path.parent(), |this, parent| {
                            let parent_str = parent.to_string_lossy();
                            if !parent_str.is_empty() {
                                this.child(
                                    self.entry_label(format!("{}/", parent_str), path_color)
                                        .when(status.is_deleted(), |this| this.strikethrough(true)),
                                )
                            } else {
                                this
                            }
                        })
                        .child(
                            self.entry_label(entry_details.display_name.clone(), label_color)
                                .when(status.is_deleted(), |this| this.strikethrough(true)),
                        ),
                ),
        )
    }
}

impl Render for GitPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let project = self.project.read(cx);
        let has_entries = self
            .active_repository
            .as_ref()
            .map_or(false, |active_repository| {
                active_repository.entry_count() > 0
            });
        let room = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).active_call()?.read(cx).room().cloned());

        let has_write_access = room
            .as_ref()
            .map_or(true, |room| room.read(cx).local_participant().can_write());
        let (can_commit, name_and_email) = match &room {
            Some(room) => {
                if project.is_via_collab() {
                    if has_write_access {
                        let name_and_email =
                            room.read(cx).local_participant_user(cx).and_then(|user| {
                                let email = SharedString::from(user.email.clone()?);
                                let name = user
                                    .name
                                    .clone()
                                    .map(SharedString::from)
                                    .unwrap_or(SharedString::from(user.github_login.clone()));
                                Some((name, email))
                            });
                        (name_and_email.is_some(), name_and_email)
                    } else {
                        (false, None)
                    }
                } else {
                    (has_write_access, None)
                }
            }
            None => (has_write_access, None),
        };

        let has_co_authors = can_commit
            && has_write_access
            && room.map_or(false, |room| {
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
                .on_action(
                    cx.listener(|this, &StageAll, window, cx| {
                        this.stage_all(&StageAll, window, cx)
                    }),
                )
                .on_action(cx.listener(|this, &UnstageAll, window, cx| {
                    this.unstage_all(&UnstageAll, window, cx)
                }))
                .on_action(cx.listener(|this, &RevertAll, window, cx| {
                    this.discard_all(&RevertAll, window, cx)
                }))
                .when(can_commit, |git_panel| {
                    git_panel
                        .on_action({
                            let name_and_email = name_and_email.clone();
                            cx.listener(move |git_panel, &CommitChanges, window, cx| {
                                git_panel.commit_changes(
                                    &CommitChanges,
                                    name_and_email.clone(),
                                    window,
                                    cx,
                                )
                            })
                        })
                        .on_action({
                            let name_and_email = name_and_email.clone();
                            cx.listener(move |git_panel, &CommitAllChanges, window, cx| {
                                git_panel.commit_all_changes(
                                    &CommitAllChanges,
                                    name_and_email.clone(),
                                    window,
                                    cx,
                                )
                            })
                        })
                })
            })
            .when(self.is_focused(window, cx), |this| {
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
            .py_1()
            .bg(ElevationIndex::Surface.bg(cx))
            .child(self.render_panel_header(window, cx))
            .child(self.render_divider(cx))
            .child(if has_entries {
                self.render_entries(has_write_access, cx).into_any_element()
            } else {
                self.render_empty_state(cx).into_any_element()
            })
            .child(self.render_divider(cx))
            .child(self.render_commit_editor(name_and_email, can_commit, cx))
    }
}

impl Focusable for GitPanel {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<Event> for GitPanel {}

impl EventEmitter<PanelEvent> for GitPanel {}

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
