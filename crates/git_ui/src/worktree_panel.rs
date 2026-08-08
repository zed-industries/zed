use std::path::{Path, PathBuf};
use std::sync::Arc;

use collections::{HashMap, HashSet};
use git::status::StageStatus;
use git_ui_core::worktree_status::{WorktreeRepository, WorktreeStatusEntry};
use gpui::{
    Action, AnyElement, App, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, Pixels, SharedString, Subscription, Task, WeakEntity, Window,
};
use editor::{Editor, EditorElement};
use project::{Project, git_store::Repository};
use settings::Settings as _;
use theme_settings::ThemeSettings;
use ui::{Disclosure, Divider, ElevationIndex, IconButton, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

pub use zed_actions::worktree_panel::{CollapseAllWorktrees, ExpandAllWorktrees, ToggleFocus};

use crate::{
    git_panel::git_commit_editor_style,
    git_status_list_item::{GitStatusListItem, StageAction, git_list_item_height},
    worktree_diff_view::WorktreeDiffView,
    worktree_panel_settings::{WorktreePanelSettings, WorktreeRowStyle, worktree_panel_dock},
};

const WORKTREE_PANEL_KEY: &str = "WorktreePanel";

/// The commit box sits inline in the list rather than in a panel footer, so it
/// stays smaller than the git panel's editor to leave room for the worktrees
/// below it.
const MAX_COMMIT_EDITOR_LINES: usize = 4;

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
        workspace.toggle_panel_focus::<WorktreePanel>(window, cx);
    });
}

/// One git worktree of the repository, as shown in the panel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorktreeEntry {
    /// The worktree directory's name. This identifies the worktree and, unlike
    /// its branch, does not change under the user.
    pub name: SharedString,
    /// The branch checked out in the worktree, or `None` when its HEAD is
    /// detached.
    pub branch: Option<SharedString>,
    pub abs_path: PathBuf,
    /// Whether this is the worktree the current project is open in.
    pub is_current: bool,
}

/// The status of a worktree that has been expanded at least once.
///
/// The panel never polls. A worktree's changes are read when it is first
/// expanded, again after any staging or commit made here, and when the user
/// asks for a refresh — so the panel is always right about changes it caused,
/// and cheap at rest with twenty worktrees listed.
enum WorktreeStatus {
    /// The task is held so that dropping the panel cancels the load.
    Loading { _task: Task<()> },
    Loaded(Vec<WorktreeStatusEntry>),
    Failed(SharedString),
}

/// One of the two lists an expanded worktree's changes are split into.
///
/// This is the git panel's "staged & unstaged" grouping, which the panel
/// applies unconditionally: without a staging checkbox on every row, the
/// section a file sits in is what tells the user whether it is staged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FileSection {
    Staged,
    Changes,
}

/// Groups a section heading so its bulk staging control appears only while the
/// heading is hovered, as a file row's own control does.
const SECTION_GROUP: &str = "worktree-panel-section";

impl FileSection {
    pub(crate) fn title(self) -> &'static str {
        match self {
            FileSection::Staged => "Staged",
            FileSection::Changes => "Changes",
        }
    }

    fn stage_action(self) -> StageAction {
        match self {
            FileSection::Staged => StageAction::Unstage,
            FileSection::Changes => StageAction::Stage,
        }
    }
}

/// Splits a worktree's changes into the lists the panel shows.
///
/// Empty sections are dropped, so a worktree with nothing staged shows a
/// single "Changes" list rather than an empty heading. A partially staged file
/// has both staged and unstaged content and so is listed in both: the section
/// a row sits in, not the file's own state, decides which way its stage button
/// moves it.
fn sections_for(
    statuses: &[WorktreeStatusEntry],
) -> Vec<(FileSection, Vec<WorktreeStatusEntry>)> {
    let collect = |keep: fn(&StageStatus) -> bool| {
        statuses
            .iter()
            .filter(|status| keep(&status.status.staging()))
            .cloned()
            .collect::<Vec<_>>()
    };
    [
        (FileSection::Staged, collect(StageStatus::has_staged)),
        (FileSection::Changes, collect(StageStatus::has_unstaged)),
    ]
    .into_iter()
    .filter(|(_, statuses)| !statuses.is_empty())
    .collect()
}

/// A row in the panel: either a worktree header or one of its changed files.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PanelEntry {
    Worktree {
        worktree: WorktreeEntry,
        is_expanded: bool,
    },
    Section {
        worktree_abs_path: PathBuf,
        section: FileSection,
    },
    File {
        worktree_abs_path: PathBuf,
        section: FileSection,
        status: WorktreeStatusEntry,
    },
    Message {
        worktree_abs_path: PathBuf,
        text: SharedString,
    },
    CommitBox {
        worktree_abs_path: PathBuf,
        staged_count: usize,
    },
}

pub struct WorktreePanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    fs: Arc<dyn fs::Fs>,
    focus_handle: FocusHandle,
    scroll_handle: gpui::ScrollHandle,
    active_repository: Option<Entity<Repository>>,
    worktrees: Vec<WorktreeEntry>,
    expanded: HashSet<PathBuf>,
    repositories: HashMap<PathBuf, WorktreeRepository>,
    statuses: HashMap<PathBuf, WorktreeStatus>,
    commit_editors: HashMap<PathBuf, Entity<Editor>>,
    pending_commits: HashSet<PathBuf>,
    selected_index: Option<usize>,
    context_menu: Option<(Entity<ui::ContextMenu>, gpui::Point<Pixels>, Subscription)>,
    _subscriptions: Vec<Subscription>,
}

impl WorktreePanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        workspace.update_in(&mut cx, Self::new)
    }

    fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let workspace_handle = cx.weak_entity();
        let project = workspace.project().clone();
        let fs = workspace.app_state().fs.clone();
        let git_store = project.read(cx).git_store().clone();
        let active_repository = project.read(cx).active_repository(cx);

        cx.new(|cx| {
            let workspace = workspace_handle;
            let mut subscriptions = vec![cx.subscribe_in(
                &git_store,
                window,
                |this: &mut Self, _, event, window, cx| {
                    if matches!(
                        event,
                        project::git_store::GitStoreEvent::ActiveRepositoryChanged(_)
                    ) {
                        this.active_repository = this.project.read(cx).active_repository(cx);
                        this.subscribe_to_active_repository(window, cx);
                    }
                    this.update_worktrees(cx);
                },
            )];
            if let Some(repository) = active_repository.as_ref() {
                subscriptions.push(Self::repository_subscription(repository, cx));
            }

            let mut this = Self {
                workspace,
                project,
                fs,
                focus_handle: cx.focus_handle(),
                scroll_handle: gpui::ScrollHandle::new(),
                active_repository,
                worktrees: Vec::new(),
                expanded: HashSet::default(),
                repositories: HashMap::default(),
                statuses: HashMap::default(),
                commit_editors: HashMap::default(),
                pending_commits: HashSet::default(),
                selected_index: None,
                context_menu: None,
                _subscriptions: subscriptions,
            };
            this.update_worktrees(cx);
            this
        })
    }

    fn repository_subscription(
        repository: &Entity<Repository>,
        cx: &mut Context<Self>,
    ) -> Subscription {
        cx.subscribe(repository, |this, _, event, cx| {
            if matches!(
                event,
                project::git_store::RepositoryEvent::GitWorktreeListChanged
            ) {
                this.update_worktrees(cx);
            }
        })
    }

    fn subscribe_to_active_repository(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(repository) = self.active_repository.clone() {
            self._subscriptions
                .push(Self::repository_subscription(&repository, cx));
        }
    }

    /// Rebuilds the worktree list from the active repository's snapshot.
    ///
    /// `linked_worktrees` deliberately excludes the repository's own work
    /// directory, so the currently-open worktree is added here and sorted to
    /// the top.
    fn update_worktrees(&mut self, cx: &mut Context<Self>) {
        let worktrees = self
            .active_repository
            .as_ref()
            .map(|repository| {
                let snapshot = repository.read(cx).snapshot();
                let current_path = snapshot.work_directory_abs_path.to_path_buf();
                let mut worktrees = vec![WorktreeEntry {
                    name: worktree_display_name(&current_path),
                    branch: snapshot
                        .branch
                        .as_ref()
                        .map(|branch| SharedString::from(branch.name().to_owned())),
                    abs_path: current_path,
                    is_current: true,
                }];
                worktrees.extend(snapshot.linked_worktrees.iter().map(|worktree| {
                    WorktreeEntry {
                        name: worktree_display_name(&worktree.path),
                        branch: branch_display_name(worktree.ref_name.as_ref()),
                        abs_path: worktree.path.clone(),
                        is_current: false,
                    }
                }));
                worktrees[1..].sort_by(|a, b| a.name.cmp(&b.name));
                worktrees
            })
            .unwrap_or_default();

        if worktrees != self.worktrees {
            self.worktrees = worktrees;
            cx.notify();
        }
    }

    #[cfg(test)]
    pub(crate) fn worktrees(&self) -> &[WorktreeEntry] {
        &self.worktrees
    }

    #[cfg(test)]
    pub(crate) fn is_expanded(&self, worktree_abs_path: &Path) -> bool {
        self.expanded.contains(worktree_abs_path)
    }

    pub(crate) fn expand_all(
        &mut self,
        _: &ExpandAllWorktrees,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        for worktree in self.worktrees.clone() {
            if self.expanded.insert(worktree.abs_path.clone())
                && !self.statuses.contains_key(&worktree.abs_path)
            {
                self.load_status(worktree.abs_path, cx);
            }
        }
        cx.notify();
    }

    pub(crate) fn collapse_all(
        &mut self,
        _: &CollapseAllWorktrees,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.expanded.clear();
        self.selected_index = None;
        cx.notify();
    }

    /// Expands or collapses a worktree, loading its changes the first time it
    /// is expanded. A collapsed worktree keeps its already-loaded changes.
    pub(crate) fn toggle_expanded(&mut self, worktree_abs_path: &Path, cx: &mut Context<Self>) {
        if self.expanded.remove(worktree_abs_path) {
            cx.notify();
            return;
        }

        self.expanded.insert(worktree_abs_path.to_path_buf());
        if !self.statuses.contains_key(worktree_abs_path) {
            self.load_status(worktree_abs_path.to_path_buf(), cx);
        }
        cx.notify();
    }

    /// Re-reads a worktree's changes.
    ///
    /// The panel does no background work, so this is the only thing that moves
    /// a worktree's state forward: it runs on first expand, after any staging
    /// or commit, and when the user asks for it.
    pub(crate) fn load_status(&mut self, worktree_abs_path: PathBuf, cx: &mut Context<Self>) {
        let cached = self.repositories.get(&worktree_abs_path).cloned();
        let fs = self.fs.clone();
        let environment = self.project.read(cx).environment().clone();
        let task = cx.spawn({
            let worktree_abs_path = worktree_abs_path.clone();
            async move |this, cx| {
                let load = async {
                    let repository = match cached {
                        Some(repository) => repository,
                        None => {
                            let repository = cx
                                .update(|cx| {
                                    WorktreeRepository::open(
                                        fs,
                                        environment,
                                        worktree_abs_path.clone(),
                                        cx,
                                    )
                                })
                                .await?;
                            this.update(cx, |this, _| {
                                this.repositories
                                    .insert(worktree_abs_path.clone(), repository.clone());
                            })?;
                            repository
                        }
                    };
                    cx.update(|cx| repository.status(cx)).await
                };
                let status = match load.await {
                    Ok(entries) => WorktreeStatus::Loaded(entries),
                    Err(error) => WorktreeStatus::Failed(SharedString::from(format!("{error:#}"))),
                };
                this.update(cx, |this, cx| {
                    this.statuses.insert(worktree_abs_path, status);
                    cx.notify();
                })
                .log_err();
            }
        });
        self.statuses
            .insert(worktree_abs_path, WorktreeStatus::Loading { _task: task });
    }

    /// Stages or unstages one file, then re-reads that worktree.
    pub(crate) fn toggle_staged(
        &mut self,
        worktree_abs_path: PathBuf,
        repo_path: git::repository::RepoPath,
        currently_staged: bool,
        cx: &mut Context<Self>,
    ) {
        self.set_staged(
            worktree_abs_path,
            vec![repo_path],
            !currently_staged,
            cx,
        );
    }

    /// Stages every file listed under `Changes`, or unstages every file listed
    /// under `Staged`.
    pub(crate) fn stage_section(
        &mut self,
        worktree_abs_path: PathBuf,
        section: FileSection,
        cx: &mut Context<Self>,
    ) {
        let Some(WorktreeStatus::Loaded(statuses)) = self.statuses.get(&worktree_abs_path) else {
            return;
        };
        let paths = sections_for(statuses)
            .into_iter()
            .find(|(candidate, _)| *candidate == section)
            .map(|(_, statuses)| {
                statuses
                    .into_iter()
                    .map(|status| status.repo_path)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if paths.is_empty() {
            return;
        }
        self.set_staged(
            worktree_abs_path,
            paths,
            section == FileSection::Changes,
            cx,
        );
    }

    fn set_staged(
        &mut self,
        worktree_abs_path: PathBuf,
        paths: Vec<git::repository::RepoPath>,
        stage: bool,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.repositories.get(&worktree_abs_path).cloned() else {
            return;
        };
        cx.spawn(async move |this, cx| {
            let result = if !stage {
                cx.update(|cx| repository.unstage(paths, cx)).await
            } else {
                cx.update(|cx| repository.stage(paths, cx)).await
            };
            this.update(cx, |this, cx| {
                if let Err(error) = result {
                    this.report_error("stage", error, cx);
                }
                // Re-read either way: a failure part-way through still leaves
                // the index somewhere, and guessing would be worse.
                this.load_status(worktree_abs_path, cx);
            })
        })
        .detach_and_log_err(cx);
    }

    fn report_error(&self, operation: &'static str, error: anyhow::Error, cx: &mut Context<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            git_ui_core::notifications::show_error_toast(workspace, operation, error, cx);
        }
    }

    /// Flattens the worktrees and the changes of the expanded ones into the
    /// rows the panel renders.
    pub(crate) fn visible_entries(&self) -> Vec<PanelEntry> {
        let mut entries = Vec::new();
        for worktree in &self.worktrees {
            let is_expanded = self.expanded.contains(&worktree.abs_path);
            entries.push(PanelEntry::Worktree {
                worktree: worktree.clone(),
                is_expanded,
            });
            if !is_expanded {
                continue;
            }
            match self.statuses.get(&worktree.abs_path) {
                Some(WorktreeStatus::Loaded(statuses)) if statuses.is_empty() => {
                    entries.push(PanelEntry::Message {
                        worktree_abs_path: worktree.abs_path.clone(),
                        text: "No uncommitted changes".into(),
                    })
                }
                Some(WorktreeStatus::Loaded(statuses)) => {
                    for (section, statuses) in sections_for(statuses) {
                        entries.push(PanelEntry::Section {
                            worktree_abs_path: worktree.abs_path.clone(),
                            section,
                        });
                        entries.extend(statuses.into_iter().map(|status| PanelEntry::File {
                            worktree_abs_path: worktree.abs_path.clone(),
                            section,
                            status,
                        }));
                    }
                }
                Some(WorktreeStatus::Failed(error)) => entries.push(PanelEntry::Message {
                    worktree_abs_path: worktree.abs_path.clone(),
                    text: error.clone(),
                }),
                Some(WorktreeStatus::Loading { .. }) | None => entries.push(PanelEntry::Message {
                    worktree_abs_path: worktree.abs_path.clone(),
                    text: "Loading…".into(),
                }),
            }
            // The commit box lives inside the worktree it commits to, so there
            // is never a question of which branch a commit lands on.
            if let Some(WorktreeStatus::Loaded(statuses)) = self.statuses.get(&worktree.abs_path) {
                entries.push(PanelEntry::CommitBox {
                    worktree_abs_path: worktree.abs_path.clone(),
                    staged_count: statuses
                        .iter()
                        .filter(|status| status.status.staging().has_staged())
                        .count(),
                });
            }
        }
        entries
    }

    fn commit_editor(
        &mut self,
        worktree_abs_path: &Path,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<Editor> {
        if let Some(editor) = self.commit_editors.get(worktree_abs_path) {
            return editor.clone();
        }
        // Configured like the git panel's commit editor, minus the chrome that
        // only makes sense for a full-height panel footer.
        let editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(1, MAX_COMMIT_EDITOR_LINES, window, cx);
            editor.set_placeholder_text("Enter commit message", window, cx);
            editor.set_use_autoclose(false);
            editor.set_show_gutter(false, cx);
            editor.set_use_modal_editing(true);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor
        });
        self.commit_editors
            .insert(worktree_abs_path.to_path_buf(), editor.clone());
        editor
    }

    /// Commits whatever is staged in the given worktree.
    pub(crate) fn commit(
        &mut self,
        worktree_abs_path: PathBuf,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.repositories.get(&worktree_abs_path).cloned() else {
            return;
        };
        let Some(editor) = self.commit_editors.get(&worktree_abs_path).cloned() else {
            return;
        };
        let message = editor.read(cx).text(cx);
        if message.trim().is_empty() || !self.pending_commits.insert(worktree_abs_path.clone()) {
            return;
        }

        let askpass = self.askpass_delegate("commit", window, cx);
        cx.spawn_in(window, async move |this, cx| {
            let result = cx
                .update(|_, cx| repository.commit(message.into(), askpass, cx))?
                .await;
            this.update_in(cx, |this, window, cx| {
                this.pending_commits.remove(&worktree_abs_path);
                match result {
                    Ok(()) => editor.update(cx, |editor, cx| editor.clear(window, cx)),
                    Err(error) => this.report_error("commit", error, cx),
                }
                this.load_status(worktree_abs_path, cx);
            })
        })
        .detach_and_log_err(cx);
    }

    /// Mirrors the git panel: a signing or credential prompt during commit
    /// surfaces as the workspace's askpass modal.
    fn askpass_delegate(
        &self,
        operation: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> askpass::AskPassDelegate {
        let workspace = self.workspace.clone();
        let operation = operation.into();
        let window = window.window_handle();
        askpass::AskPassDelegate::new(&mut cx.to_async(), move |prompt, tx, cx| {
            window
                .update(cx, |_, window, cx| {
                    workspace.update(cx, |workspace, cx| {
                        workspace.toggle_modal(window, cx, |window, cx| {
                            git_ui_core::askpass_modal::AskPassModal::new(
                                operation.clone(),
                                prompt.into(),
                                tx,
                                window,
                                cx,
                            )
                        });
                    })
                })
                .ok();
        })
    }

    /// Why the panel cannot show worktrees, if it cannot.
    ///
    /// [`fs::Fs::open_repo`] is local-only, so a remote or collaborative
    /// project is explained rather than silently listed as empty.
    fn unavailable_reason(&self, cx: &App) -> Option<SharedString> {
        let project = self.project.read(cx);
        if project.is_via_collab() {
            Some("Worktrees aren't available in shared projects".into())
        } else if project.is_via_remote_server() {
            Some("Worktrees aren't available in remote projects".into())
        } else if self.active_repository.is_none() {
            Some("No Git repository in this project".into())
        } else {
            None
        }
    }

    fn selected_entry(&self) -> Option<PanelEntry> {
        let entries = self.visible_entries();
        entries.get(self.selected_index?).cloned()
    }

    fn select_index(&mut self, index: Option<usize>, cx: &mut Context<Self>) {
        if self.selected_index != index {
            self.selected_index = index;
            cx.notify();
        }
    }

    /// Rows the user can move a selection onto.
    ///
    /// Status messages and the commit box are skipped: neither does anything
    /// on `Confirm`, and the commit box owns a focusable editor of its own.
    fn selectable_indices(&self) -> Vec<usize> {
        self.visible_entries()
            .iter()
            .enumerate()
            .filter(|(_, entry)| {
                matches!(entry, PanelEntry::Worktree { .. } | PanelEntry::File { .. })
            })
            .map(|(index, _)| index)
            .collect()
    }

    fn select_next(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        let selectable = self.selectable_indices();
        let next = match self.selected_index {
            Some(current) => selectable
                .iter()
                .find(|index| **index > current)
                .or_else(|| selectable.iter().find(|index| **index == current))
                .copied(),
            None => selectable.first().copied(),
        };
        if next.is_some() {
            self.select_index(next, cx);
        }
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selectable = self.selectable_indices();
        let previous = match self.selected_index {
            Some(current) => selectable
                .iter()
                .rev()
                .find(|index| **index < current)
                .or_else(|| selectable.iter().find(|index| **index == current))
                .copied(),
            None => selectable.last().copied(),
        };
        if previous.is_some() {
            self.select_index(previous, cx);
        }
    }

    fn select_first(&mut self, _: &menu::SelectFirst, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(first) = self.selectable_indices().first().copied() {
            self.select_index(Some(first), cx);
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(last) = self.selectable_indices().last().copied() {
            self.select_index(Some(last), cx);
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        match self.selected_entry() {
            Some(PanelEntry::Worktree { worktree, .. }) => {
                self.toggle_expanded(&worktree.abs_path, cx)
            }
            Some(PanelEntry::File {
                worktree_abs_path,
                status,
                ..
            }) => self.open_diff(worktree_abs_path, status.repo_path, window, cx),
            Some(PanelEntry::Section { .. })
            | Some(PanelEntry::CommitBox { .. })
            | Some(PanelEntry::Message { .. })
            | None => {}
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        self.select_index(None, cx);
    }

    /// Switches the workspace to the given worktree, the same way the worktree
    /// picker does.
    fn open_worktree(&mut self, worktree: &WorktreeEntry, window: &mut Window, cx: &mut App) {
        if worktree.is_current {
            return;
        }
        self.workspace
            .update(cx, |workspace, cx| {
                git_ui_core::worktree_service::handle_switch_worktree(
                    workspace,
                    &zed_actions::SwitchWorktree {
                        path: worktree.abs_path.clone(),
                        display_name: worktree.name.to_string(),
                    },
                    window,
                    None,
                    cx,
                );
            })
            .log_err();
    }

    fn deploy_worktree_context_menu(
        &mut self,
        worktree: WorktreeEntry,
        position: gpui::Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let this = cx.entity();
        let menu = ui::ContextMenu::build(window, cx, move |menu, _, _| {
            let abs_path = worktree.abs_path.clone();
            menu.when(!worktree.is_current, |menu| {
                menu.entry("Open Worktree", None, {
                    let this = this.clone();
                    let worktree = worktree.clone();
                    move |window, cx| {
                        this.update(cx, |this, cx| this.open_worktree(&worktree, window, cx));
                    }
                })
                .entry("Open in New Window", None, {
                    let abs_path = abs_path.clone();
                    move |window, cx| {
                        window.dispatch_action(
                            Box::new(zed_actions::OpenWorktreeInNewWindow {
                                path: abs_path.clone(),
                            }),
                            cx,
                        );
                    }
                })
            })
            .entry(
                if cfg!(target_os = "macos") {
                    "Reveal in Finder"
                } else {
                    "Reveal in File Manager"
                },
                None,
                move |_, cx| {
                    this.update(cx, |this, cx| {
                        this.project
                            .update(cx, |project, cx| project.reveal_path(&abs_path, cx));
                    });
                },
            )
        });

        let subscription = cx.subscribe(&menu, |this, _, _: &gpui::DismissEvent, cx| {
            this.context_menu.take();
            cx.notify();
        });
        self.context_menu = Some((menu, position, subscription));
        cx.notify();
    }

    /// Opens a read-only diff of a changed file in the workspace's active pane.
    pub(crate) fn open_diff(
        &mut self,
        worktree_abs_path: PathBuf,
        repo_path: git::repository::RepoPath,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        WorktreeDiffView::open(
            self.fs.clone(),
            worktree_abs_path,
            repo_path,
            self.workspace.clone(),
            window,
            cx,
        );
    }

    fn render_entry(
        &mut self,
        index: usize,
        entry: &PanelEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let is_selected = self.selected_index == Some(index);
        match entry {
            PanelEntry::Worktree {
                worktree,
                is_expanded,
            } => {
                let (base_bg, hover_bg, active_bg) =
                    crate::git_status_list_item::row_backgrounds(is_selected, cx);
                h_flex()
                    .id(("worktree", index))
                    .h(git_list_item_height())
                    .flex_none()
                    .w_full()
                    .pl_1()
                    .pr_1()
                    .gap_1p5()
                    .border_1()
                    .border_r_2()
                    .bg(base_bg)
                    .hover(|this| this.bg(hover_bg))
                    .active(|this| this.bg(active_bg))
                    .child(
                        Disclosure::new(("worktree-disclosure", index), *is_expanded).on_click(
                            cx.listener({
                                let abs_path = worktree.abs_path.clone();
                                move |this, _, _, cx| this.toggle_expanded(&abs_path, cx)
                            }),
                        ),
                    )
                    .child(
                        h_flex()
                            .min_w_0()
                            .flex_1()
                            .gap_1p5()
                            .child(
                                Icon::new(IconName::GitWorktree)
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(Label::new(worktree.name.clone()).single_line().truncate()),
                    )
                    .child(
                        h_flex()
                            .flex_none()
                            .gap_1()
                            .when(worktree.is_current, |this| {
                                this.child(
                                    Label::new("current")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                            })
                            .when_some(worktree.branch.clone(), |this, branch| {
                                this.child(
                                    h_flex()
                                        .gap_0p5()
                                        .max_w_32()
                                        .child(
                                            Icon::new(IconName::GitBranch)
                                                .size(IconSize::XSmall)
                                                .color(Color::Muted),
                                        )
                                        .child(
                                            Label::new(branch)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted)
                                                .truncate(),
                                        ),
                                )
                            })
                            .when(*is_expanded, |this| {
                                this.child(
                                    IconButton::new(("refresh", index), IconName::RotateCw)
                                        .icon_size(IconSize::XSmall)
                                        .tooltip(Tooltip::text("Re-read this worktree's changes"))
                                        .on_click(cx.listener({
                                            let abs_path = worktree.abs_path.clone();
                                            move |this, _, _, cx| {
                                                this.load_status(abs_path.clone(), cx);
                                                cx.stop_propagation();
                                            }
                                        })),
                                )
                            }),
                    )
                    .on_click(cx.listener({
                        let abs_path = worktree.abs_path.clone();
                        move |this, _, _, cx| {
                            this.select_index(Some(index), cx);
                            this.toggle_expanded(&abs_path, cx)
                        }
                    }))
                    .on_mouse_down(
                        gpui::MouseButton::Right,
                        cx.listener({
                            let worktree = worktree.clone();
                            move |this, event: &gpui::MouseDownEvent, window, cx| {
                                this.select_index(Some(index), cx);
                                this.deploy_worktree_context_menu(
                                    worktree.clone(),
                                    event.position,
                                    window,
                                    cx,
                                );
                            }
                        }),
                    )
                    .tooltip({
                        let path = worktree.abs_path.to_string_lossy().into_owned();
                        move |_window, cx| Tooltip::simple(path.clone(), cx)
                    })
                    .into_any_element()
            }
            PanelEntry::Section {
                worktree_abs_path,
                section,
            } => {
                let (icon, tooltip) = match section.stage_action() {
                    StageAction::Stage => (IconName::Plus, "Stage All"),
                    StageAction::Unstage => (IconName::Dash, "Unstage All"),
                };
                h_flex()
                    .group(SECTION_GROUP)
                    .h(git_list_item_height())
                    .flex_none()
                    .w_full()
                    .pl_2p5()
                    .pr_1()
                    .gap_1p5()
                    .justify_between()
                    // The transparent border is what the git panel's rows
                    // carry, so a heading's label lines up with the file names
                    // beneath it.
                    .border_1()
                    .border_r_2()
                    .child(
                        Label::new(section.title())
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
                    .child(
                        IconButton::new(("stage-section", index), icon)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .visible_on_hover(SECTION_GROUP)
                            .tooltip(Tooltip::text(tooltip))
                            .on_click(cx.listener({
                                let worktree_abs_path = worktree_abs_path.clone();
                                let section = *section;
                                move |this, _, _, cx| {
                                    this.stage_section(worktree_abs_path.clone(), section, cx)
                                }
                            })),
                    )
                    .into_any_element()
            }
            PanelEntry::File {
                worktree_abs_path,
                section,
                status,
            } => GitStatusListItem::new(
                ("file", index),
                status.repo_path.clone(),
                status.status,
                self.project.read(cx).path_style(cx),
            )
            .style(WorktreeRowStyle::get(cx))
            .diff_stat(status.diff_stat)
            .stage_action(section.stage_action())
            .selected(is_selected)
            .on_stage(cx.listener({
                let worktree_abs_path = worktree_abs_path.clone();
                let repo_path = status.repo_path.clone();
                // The section, not the file's own state: a partially staged
                // file is listed under both, and each row moves it its own way.
                let currently_staged = *section == FileSection::Staged;
                move |this, _, _, cx| {
                    this.toggle_staged(
                        worktree_abs_path.clone(),
                        repo_path.clone(),
                        currently_staged,
                        cx,
                    )
                }
            }))
            .on_click(cx.listener({
                let worktree_abs_path = worktree_abs_path.clone();
                let repo_path = status.repo_path.clone();
                move |this, _, window, cx| {
                    this.select_index(Some(index), cx);
                    this.open_diff(worktree_abs_path.clone(), repo_path.clone(), window, cx)
                }
            }))
            .into_any_element(),
            PanelEntry::Message { text, .. } => h_flex()
                .h(git_list_item_height())
                .flex_none()
                .w_full()
                .pl_2p5()
                .pr_1()
                .child(
                    Label::new(text.clone())
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .single_line(),
                )
                .into_any_element(),
            PanelEntry::CommitBox {
                worktree_abs_path,
                staged_count,
            } => {
                let editor = self.commit_editor(worktree_abs_path, window, cx);
                let is_committing = self.pending_commits.contains(worktree_abs_path);
                let has_message = !editor.read(cx).text(cx).trim().is_empty();
                let can_commit = *staged_count > 0 && has_message && !is_committing;
                let editor_style = git_commit_editor_style(
                    ThemeSettings::get_global(cx).git_commit_buffer_font_size(cx),
                    cx,
                );
                let focus_handle = editor.focus_handle(cx);

                // Full-bleed and separated by rules, as the git panel's commit
                // editor sits against its file list. Unlike the git panel's,
                // this one is sized to its content rather than filling a
                // footer, so it does not leave a slab of editor background
                // between one worktree and the next.
                v_flex()
                    .id(("commit-editor-container", index))
                    .w_full()
                    .flex_none()
                    .bg(cx.theme().colors().editor_background)
                    .border_t_1()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .on_click(move |_, window, cx| window.focus(&focus_handle, cx))
                    .child(
                        div()
                            .w_full()
                            .px_2()
                            .pt_2()
                            .pb_1()
                            .cursor_text()
                            // Otherwise arrowing within the message would also
                            // move the panel's selection.
                            .on_action(|&zed_actions::editor::MoveUp, _, cx| {
                                cx.stop_propagation();
                            })
                            .on_action(|&zed_actions::editor::MoveDown, _, cx| {
                                cx.stop_propagation();
                            })
                            .child(EditorElement::new(&editor, editor_style)),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .p_1p5()
                            .gap_1()
                            .justify_between()
                            .child(
                                Label::new(match staged_count {
                                    0 => "Nothing staged".to_owned(),
                                    1 => "1 file staged".to_owned(),
                                    count => format!("{count} files staged"),
                                })
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                            )
                            .child(
                                Button::new(("commit", index), "Commit")
                                    .label_size(LabelSize::Small)
                                    .size(ButtonSize::Compact)
                                    .layer(ElevationIndex::ModalSurface)
                                    .disabled(!can_commit)
                                    .on_click(cx.listener({
                                        let abs_path = worktree_abs_path.clone();
                                        move |this, _, window, cx| {
                                            this.commit(abs_path.clone(), window, cx)
                                        }
                                    })),
                            ),
                    )
                    .into_any_element()
            }
        }
    }
}

/// The branch a worktree has checked out, as shown beside its name.
///
/// Refs that are not local branches keep their full name, so a worktree
/// checked out at a tag is not mistaken for a branch of the same name.
fn branch_display_name(ref_name: Option<&SharedString>) -> Option<SharedString> {
    ref_name.map(|ref_name| match ref_name.strip_prefix("refs/heads/") {
        Some(branch) => SharedString::from(branch.to_owned()),
        None => ref_name.clone(),
    })
}

fn worktree_display_name(path: &Path) -> SharedString {
    path.file_name()
        .map(|name| SharedString::from(name.to_string_lossy().into_owned()))
        .unwrap_or_else(|| SharedString::from(path.to_string_lossy().into_owned()))
}

impl Render for WorktreePanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let unavailable_reason = self.unavailable_reason(cx);
        let all_expanded = !self.worktrees.is_empty()
            && self
                .worktrees
                .iter()
                .all(|worktree| self.expanded.contains(&worktree.abs_path));
        let entries = self.visible_entries();
        let rows = entries
            .iter()
            .enumerate()
            .map(|(index, entry)| self.render_entry(index, entry, window, cx))
            .collect::<Vec<_>>();

        v_flex()
            .key_context("WorktreePanel")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::expand_all))
            .on_action(cx.listener(Self::collapse_all))
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .child(
                h_flex()
                    .px_2()
                    .py_1()
                    .justify_between()
                    .child(
                        Label::new("Worktrees")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(if all_expanded {
                        IconButton::new("collapse-all", IconName::ChevronDownUp)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Collapse All Worktrees"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.collapse_all(&CollapseAllWorktrees, window, cx)
                            }))
                    } else {
                        IconButton::new("expand-all", IconName::ChevronUpDown)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Expand All Worktrees"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.expand_all(&ExpandAllWorktrees, window, cx)
                            }))
                    }),
            )
            .child(Divider::horizontal())
            .map(|this| match unavailable_reason {
                Some(reason) => this.child(
                    v_flex().p_4().items_center().child(
                        Label::new(reason)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
                ),
                None => this.child(
                    v_flex()
                        .id("worktree-list")
                        .size_full()
                        .overflow_y_scroll()
                        .track_scroll(&self.scroll_handle)
                        .children(rows),
                ),
            })
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                gpui::deferred(
                    gpui::anchored()
                        .position(*position)
                        .anchor(gpui::Anchor::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

impl Focusable for WorktreePanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for WorktreePanel {}

impl Panel for WorktreePanel {
    fn persistent_name() -> &'static str {
        "WorktreePanel"
    }

    fn panel_key() -> &'static str {
        WORKTREE_PANEL_KEY
    }

    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        worktree_panel_dock(cx)
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings.worktree_panel.get_or_insert_default().dock = Some(position.into())
        });
    }

    fn default_size(&self, _: &Window, cx: &App) -> Pixels {
        WorktreePanelSettings::get_global(cx).default_width
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<IconName> {
        Some(IconName::GitWorktree).filter(|_| WorktreePanelSettings::get_global(cx).button)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("Worktree Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        4
    }

    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.worktree_panel.get_or_insert_default().button = Some(false);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use git::status::FileStatus;
    use gpui::{TestAppContext, UpdateGlobal as _, VisualTestContext};
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use theme::LoadThemes;
    use util::path;
    use util::rel_path::rel_path;
    use workspace::MultiWorkspace;

    fn init_test(cx: &mut TestAppContext) {
        zlog::init_test();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(LoadThemes::JustBase, cx);
            language_model::init(cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    async fn setup(
        cx: &mut TestAppContext,
    ) -> (Arc<FakeFs>, Entity<WorktreePanel>, &mut VisualTestContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "main.rs": "fn main() {}",
            }),
        )
        .await;
        fs.insert_branches(Path::new(path!("/project/.git")), &["main"]);
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let panel =
            workspace.update_in(cx, |workspace, window, cx| WorktreePanel::new(workspace, window, cx));
        cx.run_until_parked();
        (fs, panel, cx)
    }

    async fn add_linked_worktree(fs: &Arc<FakeFs>, name: &str) {
        fs.add_linked_worktree_for_repo(
            Path::new(path!("/project/.git")),
            true,
            git::repository::Worktree {
                path: PathBuf::from(path!("/worktrees")).join(name),
                ref_name: Some(format!("refs/heads/{name}").into()),
                sha: "aaa".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;
    }

    fn worktree_names(panel: &Entity<WorktreePanel>, cx: &mut VisualTestContext) -> Vec<String> {
        panel.read_with(cx, |panel, _| {
            panel
                .worktrees()
                .iter()
                .map(|entry| entry.name.to_string())
                .collect()
        })
    }

    #[gpui::test]
    async fn test_a_worktree_reports_its_directory_name_and_its_branch(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;

        // A worktree's directory and its branch are independent names, and the
        // panel has to show both.
        fs.add_linked_worktree_for_repo(
            Path::new(path!("/project/.git")),
            true,
            git::repository::Worktree {
                path: PathBuf::from(path!("/worktrees/review-copy")),
                ref_name: Some("refs/heads/crash-on-open".into()),
                sha: "aaa".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;
        cx.run_until_parked();

        let entries = panel.read_with(cx, |panel, _| panel.worktrees().to_vec());
        let linked = &entries[1];
        assert_eq!(linked.name, "review-copy");
        assert_eq!(linked.branch.as_deref(), Some("crash-on-open"));
    }

    #[test]
    fn test_branch_display_name() {
        assert_eq!(
            branch_display_name(Some(&"refs/heads/main".into())).as_deref(),
            Some("main")
        );
        // A worktree can hold any ref, not only a local branch.
        assert_eq!(
            branch_display_name(Some(&"refs/tags/v1.0".into())).as_deref(),
            Some("refs/tags/v1.0")
        );
        // A detached HEAD has no branch to show.
        assert_eq!(branch_display_name(None), None);
    }

    #[gpui::test]
    async fn test_expand_all_and_collapse_all(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        add_linked_worktree(&fs, "feature-a").await;
        add_linked_worktree(&fs, "feature-b").await;
        cx.run_until_parked();

        let expanded_count = |cx: &mut VisualTestContext| {
            panel.read_with(cx, |panel, _| {
                panel
                    .worktrees()
                    .iter()
                    .filter(|worktree| panel.is_expanded(&worktree.abs_path))
                    .count()
            })
        };
        assert_eq!(expanded_count(cx), 0);

        panel.update_in(cx, |panel, window, cx| {
            panel.expand_all(&ExpandAllWorktrees, window, cx)
        });
        cx.run_until_parked();
        assert_eq!(expanded_count(cx), 3);

        panel.update_in(cx, |panel, window, cx| {
            panel.collapse_all(&CollapseAllWorktrees, window, cx)
        });
        assert_eq!(expanded_count(cx), 0);
    }

    #[gpui::test]
    async fn test_lists_linked_worktrees(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;

        add_linked_worktree(&fs, "feature-b").await;
        add_linked_worktree(&fs, "feature-a").await;
        cx.run_until_parked();

        assert_eq!(
            worktree_names(&panel, cx),
            vec![
                "project".to_owned(),
                "feature-a".to_owned(),
                "feature-b".to_owned()
            ]
        );
    }

    #[gpui::test]
    async fn test_current_worktree_sorts_first(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;

        // Sorts before "main" alphabetically, so only the current-worktree rule
        // can keep "main" at the top.
        add_linked_worktree(&fs, "aardvark").await;
        cx.run_until_parked();

        let entries = panel.read_with(cx, |panel, _| panel.worktrees().to_vec());
        assert_eq!(entries[0].name, "project");
        assert_eq!(entries[0].branch.as_deref(), Some("main"));
        assert!(entries[0].is_current);
        assert_eq!(entries[0].abs_path, PathBuf::from(path!("/project")));
        assert_eq!(entries[1].name, "aardvark");
        assert!(!entries[1].is_current);
    }

    #[gpui::test]
    async fn test_list_updates_when_worktrees_change(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        assert_eq!(worktree_names(&panel, cx), vec!["project".to_owned()]);

        add_linked_worktree(&fs, "feature-a").await;
        cx.run_until_parked();
        assert_eq!(
            worktree_names(&panel, cx),
            vec!["project".to_owned(), "feature-a".to_owned()]
        );

        fs.remove_worktree_for_repo(
            Path::new(path!("/project/.git")),
            true,
            "refs/heads/feature-a",
        )
        .await;
        cx.run_until_parked();
        assert_eq!(worktree_names(&panel, cx), vec!["project".to_owned()]);
    }

    async fn add_worktree_with_changes(fs: &Arc<FakeFs>, name: &str) -> PathBuf {
        add_linked_worktree(fs, name).await;
        let worktree_abs_path = PathBuf::from(path!("/worktrees")).join(name);
        fs.insert_file(
            worktree_abs_path.join("main.rs"),
            b"fn main() { changed(); }".to_vec(),
        )
        .await;
        fs.insert_file(worktree_abs_path.join("extra.rs"), b"// new".to_vec())
            .await;
        fs.set_head_and_index_for_repo(
            &worktree_abs_path.join(".git"),
            &[("main.rs", "fn main() {}".into())],
        );
        worktree_abs_path
    }

    fn file_rows(panel: &Entity<WorktreePanel>, cx: &mut VisualTestContext) -> Vec<String> {
        panel.read_with(cx, |panel, _| {
            panel
                .visible_entries()
                .into_iter()
                .filter_map(|entry| match entry {
                    PanelEntry::File { status, .. } => Some(format!(
                        "{} {:?}",
                        status.repo_path.as_unix_str(),
                        status.status
                    )),
                    _ => None,
                })
                .collect()
        })
    }

    #[gpui::test]
    async fn test_expanding_a_worktree_shows_its_changed_files(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();

        assert!(file_rows(&panel, cx).is_empty());

        panel.update(cx, |panel, cx| {
            panel.toggle_expanded(&worktree_abs_path, cx)
        });
        cx.run_until_parked();

        assert_eq!(
            file_rows(&panel, cx),
            vec![
                "extra.rs Untracked".to_owned(),
                "main.rs Tracked(TrackedStatus { index_status: Unmodified, worktree_status: Modified })"
                    .to_owned(),
            ]
        );
    }

    #[gpui::test]
    async fn test_expanding_does_not_add_the_worktree_to_the_project(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();

        let project = panel.read_with(cx, |panel, _| panel.project.clone());
        let worktree_count_before = project.read_with(cx, |project, cx| project.worktrees(cx).count());

        panel.update(cx, |panel, cx| {
            panel.toggle_expanded(&worktree_abs_path, cx)
        });
        cx.run_until_parked();

        assert!(!file_rows(&panel, cx).is_empty());
        assert_eq!(
            project.read_with(cx, |project, cx| project.worktrees(cx).count()),
            worktree_count_before,
            "expanding a worktree must not pull it into the project"
        );
    }

    #[gpui::test]
    async fn test_collapsing_keeps_the_loaded_status(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();

        panel.update(cx, |panel, cx| {
            panel.toggle_expanded(&worktree_abs_path, cx)
        });
        cx.run_until_parked();
        let loaded = file_rows(&panel, cx);
        assert_eq!(loaded.len(), 2);

        panel.update(cx, |panel, cx| {
            panel.toggle_expanded(&worktree_abs_path, cx)
        });
        cx.run_until_parked();
        assert!(file_rows(&panel, cx).is_empty());

        // The panel does no background work, so a change made while collapsed
        // is deliberately not picked up when expanding again.
        fs.insert_file(worktree_abs_path.join("later.rs"), b"// later".to_vec())
            .await;
        panel.update(cx, |panel, cx| {
            panel.toggle_expanded(&worktree_abs_path, cx)
        });
        cx.run_until_parked();
        assert_eq!(file_rows(&panel, cx), loaded);
    }

    #[gpui::test]
    async fn test_expanding_a_deleted_worktree_shows_an_error(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        add_linked_worktree(&fs, "feature-a").await;
        cx.run_until_parked();

        let missing = PathBuf::from(path!("/worktrees/never-existed"));
        panel.update(cx, |panel, cx| panel.toggle_expanded(&missing, cx));
        cx.run_until_parked();

        let has_error = panel.read_with(cx, |panel, _| {
            matches!(
                panel.statuses.get(&missing),
                Some(WorktreeStatus::Failed(_))
            )
        });
        assert!(has_error);
    }

    fn open_first_changed_file(
        panel: &Entity<WorktreePanel>,
        cx: &mut VisualTestContext,
    ) -> (PathBuf, git::repository::RepoPath) {
        let target = panel.read_with(cx, |panel, _| {
            panel
                .visible_entries()
                .into_iter()
                .find_map(|entry| match entry {
                    PanelEntry::File {
                        worktree_abs_path,
                        status,
                        ..
                    } => Some((worktree_abs_path, status.repo_path)),
                    _ => None,
                })
                .expect("expected at least one changed file")
        });
        panel.update_in(cx, |panel, window, cx| {
            panel.open_diff(target.0.clone(), target.1.clone(), window, cx)
        });
        cx.run_until_parked();
        target
    }

    #[gpui::test]
    async fn test_clicking_a_file_opens_a_read_only_diff(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();

        let project = panel.read_with(cx, |panel, _| panel.project.clone());
        let worktree_count_before =
            project.read_with(cx, |project, cx| project.worktrees(cx).count());

        panel.update(cx, |panel, cx| {
            panel.toggle_expanded(&worktree_abs_path, cx)
        });
        cx.run_until_parked();
        open_first_changed_file(&panel, cx);

        let workspace = panel.read_with(cx, |panel, _| panel.workspace.clone());
        let diff_view = workspace
            .update(cx, |workspace, cx| {
                workspace.active_item_as::<crate::worktree_diff_view::WorktreeDiffView>(cx)
            })
            .expect("workspace was dropped")
            .expect("expected a worktree diff view in the active pane");

        diff_view.read_with(cx, |diff_view, cx| {
            assert!(
                diff_view.editor().read(cx).read_only(cx),
                "the diff must not be editable"
            );
            assert_eq!(diff_view.buffer().read(cx).text(), "// new");
            // Untracked file: no committed text, so the whole file is an addition.
            assert_eq!(diff_view.buffer_diff().read(cx).base_text_string(cx), None);
        });

        assert_eq!(
            project.read_with(cx, |project, cx| project.worktrees(cx).count()),
            worktree_count_before,
            "opening a diff must not pull the worktree into the project"
        );
    }

    #[gpui::test]
    async fn test_diff_base_is_the_worktrees_committed_text(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();

        panel.update_in(cx, |panel, window, cx| {
            panel.open_diff(
                worktree_abs_path.clone(),
                git::repository::RepoPath::from_rel_path(rel_path("main.rs")),
                window,
                cx,
            )
        });
        cx.run_until_parked();

        let workspace = panel.read_with(cx, |panel, _| panel.workspace.clone());
        let diff_view = workspace
            .update(cx, |workspace, cx| {
                workspace.active_item_as::<crate::worktree_diff_view::WorktreeDiffView>(cx)
            })
            .expect("workspace was dropped")
            .expect("expected a worktree diff view in the active pane");

        diff_view.read_with(cx, |diff_view, cx| {
            assert_eq!(
                diff_view.buffer().read(cx).text(),
                "fn main() { changed(); }"
            );
            assert_eq!(
                diff_view.buffer_diff().read(cx).base_text_string(cx).as_deref(),
                Some("fn main() {}")
            );
        });
    }

    #[gpui::test]
    async fn test_opening_the_same_file_twice_reuses_the_tab(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();

        panel.update(cx, |panel, cx| {
            panel.toggle_expanded(&worktree_abs_path, cx)
        });
        cx.run_until_parked();

        open_first_changed_file(&panel, cx);
        open_first_changed_file(&panel, cx);

        let workspace = panel.read_with(cx, |panel, _| panel.workspace.clone());
        let item_count = workspace
            .read_with(cx, |workspace, cx| workspace.active_pane().read(cx).items().count())
            .expect("workspace was dropped");
        assert_eq!(item_count, 1);
    }

    fn selected_row(panel: &Entity<WorktreePanel>, cx: &mut VisualTestContext) -> Option<String> {
        panel.read_with(cx, |panel, _| {
            panel.selected_entry().map(|entry| match entry {
                PanelEntry::Worktree { worktree, .. } => format!("worktree:{}", worktree.name),
                PanelEntry::File { status, .. } => {
                    format!("file:{}", status.repo_path.as_unix_str())
                }
                PanelEntry::Section { section, .. } => format!("section:{}", section.title()),
                PanelEntry::Message { text, .. } => format!("message:{text}"),
                PanelEntry::CommitBox { .. } => "commit-box".to_owned(),
            })
        })
    }

    #[gpui::test]
    async fn test_keyboard_navigation_walks_worktrees_and_files(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();

        panel.update(cx, |panel, cx| {
            panel.toggle_expanded(&worktree_abs_path, cx)
        });
        cx.run_until_parked();

        assert_eq!(selected_row(&panel, cx), None);

        let select_next = |cx: &mut VisualTestContext| {
            panel.update_in(cx, |panel, window, cx| {
                panel.select_next(&menu::SelectNext, window, cx)
            });
        };

        select_next(cx);
        assert_eq!(selected_row(&panel, cx).as_deref(), Some("worktree:project"));
        select_next(cx);
        assert_eq!(
            selected_row(&panel, cx).as_deref(),
            Some("worktree:feature-a")
        );
        select_next(cx);
        assert_eq!(selected_row(&panel, cx).as_deref(), Some("file:extra.rs"));
        select_next(cx);
        assert_eq!(selected_row(&panel, cx).as_deref(), Some("file:main.rs"));
        // The last row holds; selection never wraps past the end.
        select_next(cx);
        assert_eq!(selected_row(&panel, cx).as_deref(), Some("file:main.rs"));

        panel.update_in(cx, |panel, window, cx| {
            panel.select_previous(&menu::SelectPrevious, window, cx)
        });
        assert_eq!(selected_row(&panel, cx).as_deref(), Some("file:extra.rs"));
    }

    #[gpui::test]
    async fn test_confirm_toggles_a_worktree_and_opens_a_file(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();

        // Select the linked worktree row and confirm it to expand.
        panel.update_in(cx, |panel, window, cx| {
            panel.select_next(&menu::SelectNext, window, cx);
            panel.select_next(&menu::SelectNext, window, cx);
            panel.confirm(&menu::Confirm, window, cx);
        });
        cx.run_until_parked();
        assert_eq!(file_rows(&panel, cx).len(), 2);

        // Move onto the first file row and confirm it to open the diff.
        panel.update_in(cx, |panel, window, cx| {
            panel.select_next(&menu::SelectNext, window, cx);
            panel.confirm(&menu::Confirm, window, cx);
        });
        cx.run_until_parked();

        let workspace = panel.read_with(cx, |panel, _| panel.workspace.clone());
        let opened = workspace
            .update(cx, |workspace, cx| {
                workspace
                    .active_item_as::<crate::worktree_diff_view::WorktreeDiffView>(cx)
                    .is_some()
            })
            .expect("workspace was dropped");
        assert!(opened, "confirming a file row should open its diff");
    }

    #[gpui::test]
    async fn test_selection_is_cleared_on_cancel(cx: &mut TestAppContext) {
        let (_fs, panel, cx) = setup(cx).await;

        panel.update_in(cx, |panel, window, cx| {
            panel.select_next(&menu::SelectNext, window, cx)
        });
        assert!(selected_row(&panel, cx).is_some());

        panel.update_in(cx, |panel, window, cx| {
            panel.cancel(&menu::Cancel, window, cx)
        });
        assert_eq!(selected_row(&panel, cx), None);
    }

    #[gpui::test]
    async fn test_a_project_without_a_repository_explains_itself(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/plain"), json!({ "main.rs": "fn main() {}" }))
            .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project = Project::test(fs.clone(), [path!("/plain").as_ref()], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let panel = workspace
            .update_in(cx, |workspace, window, cx| {
                WorktreePanel::new(workspace, window, cx)
            });
        cx.run_until_parked();

        assert!(panel.read_with(cx, |panel, _| panel.worktrees().is_empty()));
        assert_eq!(
            panel
                .read_with(cx, |panel, cx| panel.unavailable_reason(cx))
                .as_deref(),
            Some("No Git repository in this project")
        );
    }

    /// The rows of an expanded worktree, with files indented under the section
    /// heading they sit in.
    fn section_rows(panel: &Entity<WorktreePanel>, cx: &mut VisualTestContext) -> Vec<String> {
        panel.read_with(cx, |panel, _| {
            panel
                .visible_entries()
                .into_iter()
                .filter_map(|entry| match entry {
                    PanelEntry::Section { section, .. } => Some(section.title().to_owned()),
                    PanelEntry::File { status, .. } => {
                        Some(format!("  {}", status.repo_path.as_unix_str()))
                    }
                    _ => None,
                })
                .collect()
        })
    }

    #[gpui::test]
    async fn test_unstaged_changes_are_listed_under_changes(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();
        expand(&panel, &worktree_abs_path, cx).await;

        assert_eq!(
            section_rows(&panel, cx),
            vec!["Changes", "  extra.rs", "  main.rs"],
            "with nothing staged there is no staged section to show"
        );
    }

    #[gpui::test]
    async fn test_staging_a_file_moves_it_to_the_staged_section(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();
        expand(&panel, &worktree_abs_path, cx).await;

        panel.update(cx, |panel, cx| {
            panel.toggle_staged(
                worktree_abs_path.clone(),
                git::repository::RepoPath::from_rel_path(rel_path("main.rs")),
                false,
                cx,
            )
        });
        cx.run_until_parked();

        assert_eq!(
            section_rows(&panel, cx),
            vec!["Staged", "  main.rs", "Changes", "  extra.rs"]
        );
    }

    #[gpui::test]
    async fn test_staging_a_whole_section(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();
        expand(&panel, &worktree_abs_path, cx).await;

        panel.update(cx, |panel, cx| {
            panel.stage_section(worktree_abs_path.clone(), FileSection::Changes, cx)
        });
        cx.run_until_parked();

        assert_eq!(
            section_rows(&panel, cx),
            vec!["Staged", "  extra.rs", "  main.rs"],
            "staging the changes section should leave nothing unstaged"
        );

        panel.update(cx, |panel, cx| {
            panel.stage_section(worktree_abs_path.clone(), FileSection::Staged, cx)
        });
        cx.run_until_parked();

        assert_eq!(
            section_rows(&panel, cx),
            vec!["Changes", "  extra.rs", "  main.rs"],
            "unstaging the staged section should send everything back"
        );
    }

    #[test]
    fn test_a_partially_staged_file_is_listed_in_both_sections() {
        let entry = |path: &str, status: FileStatus| WorktreeStatusEntry {
            repo_path: git::repository::RepoPath::from_rel_path(rel_path(path)),
            status,
            diff_stat: None,
        };
        let partially_staged = FileStatus::Tracked(git::status::TrackedStatus {
            index_status: git::status::StatusCode::Modified,
            worktree_status: git::status::StatusCode::Modified,
        });

        let sections = sections_for(&[
            entry("both.rs", partially_staged),
            entry("new.rs", FileStatus::Untracked),
        ]);

        assert_eq!(
            sections
                .iter()
                .map(|(section, statuses)| (
                    section.title(),
                    statuses
                        .iter()
                        .map(|status| status.repo_path.as_unix_str().to_owned())
                        .collect::<Vec<_>>()
                ))
                .collect::<Vec<_>>(),
            vec![
                ("Staged", vec!["both.rs".to_owned()]),
                (
                    "Changes",
                    vec!["both.rs".to_owned(), "new.rs".to_owned()]
                ),
            ]
        );
    }

    async fn expand(
        panel: &Entity<WorktreePanel>,
        worktree_abs_path: &Path,
        cx: &mut VisualTestContext,
    ) {
        panel.update(cx, |panel, cx| panel.toggle_expanded(worktree_abs_path, cx));
        cx.run_until_parked();
    }

    fn staging_of(
        panel: &Entity<WorktreePanel>,
        path: &str,
        cx: &mut VisualTestContext,
    ) -> Option<git::status::StageStatus> {
        panel.read_with(cx, |panel, _| {
            panel
                .visible_entries()
                .into_iter()
                .find_map(|entry| match entry {
                    PanelEntry::File { status, .. }
                        if status.repo_path.as_unix_str() == path =>
                    {
                        Some(status.status.staging())
                    }
                    _ => None,
                })
        })
    }

    #[gpui::test]
    async fn test_staging_a_file_from_the_panel(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();
        expand(&panel, &worktree_abs_path, cx).await;

        assert_eq!(
            staging_of(&panel, "main.rs", cx),
            Some(git::status::StageStatus::Unstaged)
        );

        panel.update(cx, |panel, cx| {
            panel.toggle_staged(
                worktree_abs_path.clone(),
                git::repository::RepoPath::from_rel_path(rel_path("main.rs")),
                false,
                cx,
            )
        });
        cx.run_until_parked();

        // The row re-read itself, so the checkbox reflects the real index.
        assert_eq!(
            staging_of(&panel, "main.rs", cx),
            Some(git::status::StageStatus::Staged)
        );

        panel.update(cx, |panel, cx| {
            panel.toggle_staged(
                worktree_abs_path.clone(),
                git::repository::RepoPath::from_rel_path(rel_path("main.rs")),
                true,
                cx,
            )
        });
        cx.run_until_parked();
        assert_eq!(
            staging_of(&panel, "main.rs", cx),
            Some(git::status::StageStatus::Unstaged)
        );
    }

    #[gpui::test]
    async fn test_refresh_picks_up_changes_made_while_expanded(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();
        expand(&panel, &worktree_abs_path, cx).await;
        assert_eq!(file_rows(&panel, cx).len(), 2);

        // Something else writes to the worktree; the panel does no background
        // work, so it does not notice on its own.
        fs.insert_file(worktree_abs_path.join("later.rs"), b"// later".to_vec())
            .await;
        cx.run_until_parked();
        assert_eq!(file_rows(&panel, cx).len(), 2);

        panel.update(cx, |panel, cx| {
            panel.load_status(worktree_abs_path.clone(), cx)
        });
        cx.run_until_parked();
        assert_eq!(file_rows(&panel, cx).len(), 3);
    }

    #[gpui::test]
    async fn test_line_counts_reach_the_rows(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();
        expand(&panel, &worktree_abs_path, cx).await;

        let diff_stats = panel.read_with(cx, |panel, _| {
            panel
                .visible_entries()
                .into_iter()
                .filter_map(|entry| match entry {
                    PanelEntry::File { status, .. } => {
                        Some((status.repo_path.as_unix_str().to_owned(), status.diff_stat))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        });

        let main = diff_stats
            .iter()
            .find(|(path, _)| path == "main.rs")
            .expect("expected main.rs");
        assert!(main.1.is_some(), "a tracked change should carry counts");
        let extra = diff_stats
            .iter()
            .find(|(path, _)| path == "extra.rs")
            .expect("expected extra.rs");
        assert_eq!(extra.1, None, "untracked files have no counts");
    }

    #[gpui::test]
    async fn test_committing_from_a_worktree(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();
        expand(&panel, &worktree_abs_path, cx).await;

        // A commit box appears under the worktree it commits to.
        let commit_boxes = panel.read_with(cx, |panel, _| {
            panel
                .visible_entries()
                .into_iter()
                .filter_map(|entry| match entry {
                    PanelEntry::CommitBox {
                        worktree_abs_path,
                        staged_count,
                    } => Some((worktree_abs_path, staged_count)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        });
        assert_eq!(commit_boxes, vec![(worktree_abs_path.clone(), 0)]);

        panel.update(cx, |panel, cx| {
            panel.toggle_staged(
                worktree_abs_path.clone(),
                git::repository::RepoPath::from_rel_path(rel_path("main.rs")),
                false,
                cx,
            )
        });
        cx.run_until_parked();

        panel.update_in(cx, |panel, window, cx| {
            let editor = panel.commit_editor(&worktree_abs_path, window, cx);
            editor.update(cx, |editor, cx| {
                editor.set_text("Update main", window, cx);
            });
        });
        cx.run_until_parked();

        panel.update_in(cx, |panel, window, cx| {
            panel.commit(worktree_abs_path.clone(), window, cx)
        });
        cx.run_until_parked();

        // The committed file is gone from the list and the message is cleared.
        let remaining = file_rows(&panel, cx);
        assert!(
            !remaining.iter().any(|row| row.starts_with("main.rs")),
            "main.rs should have been committed, still saw {remaining:?}"
        );
        let message = panel.read_with(cx, |panel, cx| {
            panel
                .commit_editors
                .get(&worktree_abs_path)
                .map(|editor| editor.read(cx).text(cx))
        });
        assert_eq!(message.as_deref(), Some(""));
    }

    #[gpui::test]
    async fn test_commit_does_nothing_without_a_message(cx: &mut TestAppContext) {
        let (fs, panel, cx) = setup(cx).await;
        let worktree_abs_path = add_worktree_with_changes(&fs, "feature-a").await;
        cx.run_until_parked();
        expand(&panel, &worktree_abs_path, cx).await;

        panel.update(cx, |panel, cx| {
            panel.toggle_staged(
                worktree_abs_path.clone(),
                git::repository::RepoPath::from_rel_path(rel_path("main.rs")),
                false,
                cx,
            )
        });
        cx.run_until_parked();

        panel.update_in(cx, |panel, window, cx| {
            panel.commit_editor(&worktree_abs_path, window, cx);
            panel.commit(worktree_abs_path.clone(), window, cx)
        });
        cx.run_until_parked();

        assert_eq!(
            staging_of(&panel, "main.rs", cx),
            Some(git::status::StageStatus::Staged),
            "an empty message must not commit"
        );
    }

    #[gpui::test]
    async fn test_dock_follows_the_git_panel_unless_overridden(cx: &mut TestAppContext) {
        let (_fs, panel, cx) = setup(cx).await;

        let dock = |cx: &mut VisualTestContext| {
            panel.update_in(cx, |panel, window, cx| panel.position(window, cx))
        };

        // Both panels default to the right.
        assert_eq!(dock(cx), DockPosition::Right);

        // Moving the git panel takes the worktree panel with it.
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().dock =
                        Some(settings::DockPosition::Left);
                });
            });
        });
        assert_eq!(dock(cx), DockPosition::Left);

        // An explicit worktree panel setting wins over the git panel's.
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.worktree_panel.get_or_insert_default().dock =
                        Some(settings::DockPosition::Right);
                });
            });
        });
        assert_eq!(dock(cx), DockPosition::Right);
    }
}
