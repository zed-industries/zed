use crate::{
    git_status_icon, settings::GitPanelSettings, CommitAllChanges, CommitStagedChanges, GitState,
    RevertAll, StageAll, UnstageAll,
};
use anyhow::{Context as _, Result};
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use git::{
    diff::DiffHunk,
    repository::{GitFileStatus, RepoPath},
};
use gpui::*;
use language::Buffer;
use menu::{SelectNext, SelectPrev};
use project::{EntryKind, Fs, Project, ProjectEntryId, WorktreeId};
use serde::{Deserialize, Serialize};
use settings::Settings as _;
use std::{
    cell::OnceCell,
    collections::HashSet,
    ffi::OsStr,
    ops::{Deref, Range},
    path::PathBuf,
    rc::Rc,
    sync::Arc,
    time::Duration,
    usize,
};
use theme::ThemeSettings;
use ui::{
    prelude::*, Checkbox, Divider, DividerColor, ElevationIndex, Scrollbar, ScrollbarState, Tooltip,
};
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};
use worktree::StatusEntry;

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

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum ViewMode {
    #[default]
    List,
    Tree,
}

pub struct GitStatusEntry {}

#[derive(Debug, PartialEq, Eq, Clone)]
struct EntryDetails {
    filename: String,
    display_name: String,
    path: RepoPath,
    kind: EntryKind,
    depth: usize,
    is_expanded: bool,
    status: Option<GitFileStatus>,
    hunks: Rc<OnceCell<Vec<DiffHunk>>>,
    index: usize,
}

#[derive(Serialize, Deserialize)]
struct SerializedGitPanel {
    width: Option<Pixels>,
}

pub struct GitPanel {
    // workspace: WeakView<Workspace>,
    current_modifiers: Modifiers,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    hide_scrollbar_task: Option<Task<()>>,
    pending_serialization: Task<Option<()>>,
    project: Model<Project>,
    scroll_handle: UniformListScrollHandle,
    scrollbar_state: ScrollbarState,
    selected_item: Option<usize>,
    view_mode: ViewMode,
    show_scrollbar: bool,
    // TODO Reintroduce expanded directories, once we're deriving directories from paths
    // expanded_dir_ids: HashMap<WorktreeId, Vec<ProjectEntryId>>,
    git_state: Model<GitState>,
    commit_editor: View<Editor>,
    // The entries that are currently shown in the panel, aka
    // not hidden by folding or such
    visible_entries: Vec<WorktreeEntries>,
    width: Option<Pixels>,
    // git_diff_editor: Option<View<Editor>>,
    // git_diff_editor_updates: Task<()>,
    reveal_in_editor: Task<()>,
}

#[derive(Debug, Clone)]
struct WorktreeEntries {
    worktree_id: WorktreeId,
    // TODO support multiple repositories per worktree
    // work_directory: worktree::WorkDirectory,
    visible_entries: Vec<GitPanelEntry>,
    paths: Rc<OnceCell<HashSet<RepoPath>>>,
}

#[derive(Debug, Clone)]
struct GitPanelEntry {
    entry: worktree::StatusEntry,
    hunks: Rc<OnceCell<Vec<DiffHunk>>>,
}

impl Deref for GitPanelEntry {
    type Target = worktree::StatusEntry;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

impl WorktreeEntries {
    fn paths(&self) -> &HashSet<RepoPath> {
        self.paths.get_or_init(|| {
            self.visible_entries
                .iter()
                .map(|e| (e.entry.repo_path.clone()))
                .collect()
        })
    }
}

impl GitPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move { workspace.update(&mut cx, Self::new) })
    }

    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let git_state = GitState::get_global(cx);

        let fs = workspace.app_state().fs.clone();
        // let weak_workspace = workspace.weak_handle();
        let project = workspace.project().clone();
        let language_registry = workspace.app_state().languages.clone();

        let git_panel = cx.new_view(|cx: &mut ViewContext<Self>| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, Self::focus_in).detach();
            cx.on_focus_out(&focus_handle, |this, _, cx| {
                this.hide_scrollbar(cx);
            })
            .detach();
            cx.subscribe(&project, |this, _, event, cx| match event {
                project::Event::WorktreeRemoved(_id) => {
                    // this.expanded_dir_ids.remove(id);
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
                    // this.git_diff_editor_updates = Task::ready(());
                    this.reveal_in_editor = Task::ready(());
                    // this.expanded_dir_ids.clear();
                    this.visible_entries.clear();
                    // this.git_diff_editor = None;
                }
                _ => {}
            })
            .detach();

            let state = git_state.read(cx);
            let current_commit_message = state.commit_message.clone();

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
                // commit_editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
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

            let mut git_panel = Self {
                // workspace: weak_workspace,
                focus_handle: cx.focus_handle(),
                fs,
                pending_serialization: Task::ready(None),
                visible_entries: Vec::new(),
                current_modifiers: cx.modifiers(),
                // expanded_dir_ids: Default::default(),
                width: Some(px(360.)),
                scrollbar_state: ScrollbarState::new(scroll_handle.clone()).parent_view(cx.view()),
                scroll_handle,
                selected_item: None,
                view_mode: ViewMode::default(),
                show_scrollbar: !Self::should_autohide_scrollbar(cx),
                hide_scrollbar_task: None,
                // git_diff_editor: Some(diff_display_editor(cx)),
                // git_diff_editor_updates: Task::ready(()),
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
        entry: &StatusEntry,
        visible_worktree_entries: &HashSet<RepoPath>,
    ) -> (usize, usize) {
        let (depth, difference) = entry
            .repo_path
            .ancestors()
            .skip(1) // Skip the entry itself
            .find_map(|ancestor| {
                if let Some(parent_entry) = visible_worktree_entries.get(ancestor) {
                    let entry_path_components_count = entry.repo_path.components().count();
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

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        let item_count = self
            .visible_entries
            .iter()
            .map(|worktree_entries| worktree_entries.visible_entries.len())
            .sum::<usize>();
        if item_count == 0 {
            return;
        }
        let selection = match self.selected_item {
            Some(i) => {
                if i < item_count - 1 {
                    self.selected_item = Some(i + 1);
                    i + 1
                } else {
                    self.selected_item = Some(0);
                    0
                }
            }
            None => {
                self.selected_item = Some(0);
                0
            }
        };
        self.scroll_handle
            .scroll_to_item(selection, ScrollStrategy::Center);

        let mut hunks = None;
        self.for_each_visible_entry(selection..selection + 1, cx, |_, entry, _| {
            hunks = Some(entry.hunks.clone());
        });
        if let Some(hunks) = hunks {
            self.reveal_entry_in_git_editor(hunks, false, Some(UPDATE_DEBOUNCE), cx);
        }

        cx.notify();
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        let item_count = self
            .visible_entries
            .iter()
            .map(|worktree_entries| worktree_entries.visible_entries.len())
            .sum::<usize>();
        if item_count == 0 {
            return;
        }
        let selection = match self.selected_item {
            Some(i) => {
                if i > 0 {
                    self.selected_item = Some(i - 1);
                    i - 1
                } else {
                    self.selected_item = Some(item_count - 1);
                    item_count - 1
                }
            }
            None => {
                self.selected_item = Some(0);
                0
            }
        };
        self.scroll_handle
            .scroll_to_item(selection, ScrollStrategy::Center);

        let mut hunks = None;
        self.for_each_visible_entry(selection..selection + 1, cx, |_, entry, _| {
            hunks = Some(entry.hunks.clone());
        });
        if let Some(hunks) = hunks {
            self.reveal_entry_in_git_editor(hunks, false, Some(UPDATE_DEBOUNCE), cx);
        }

        cx.notify();
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

    fn discard_all(&mut self, _: &RevertAll, _cx: &mut ViewContext<Self>) {
        // TODO: Implement discard all
        println!("Discard all triggered");
    }

    fn clear_message(&mut self, cx: &mut ViewContext<Self>) {
        let git_state = self.git_state.clone();
        git_state.update(cx, |state, _cx| state.clear_message());
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
        self.visible_entries
            .iter()
            .map(|worktree_entries| worktree_entries.visible_entries.len())
            .sum()
    }

    fn for_each_visible_entry(
        &self,
        range: Range<usize>,
        cx: &mut ViewContext<Self>,
        mut callback: impl FnMut(usize, EntryDetails, &mut ViewContext<Self>),
    ) {
        let mut ix = 0;
        for worktree_entries in &self.visible_entries {
            if ix >= range.end {
                return;
            }

            if ix + worktree_entries.visible_entries.len() <= range.start {
                ix += worktree_entries.visible_entries.len();
                continue;
            }

            let end_ix = range.end.min(ix + worktree_entries.visible_entries.len());
            // let entry_range = range.start.saturating_sub(ix)..end_ix - ix;
            if let Some(worktree) = self
                .project
                .read(cx)
                .worktree_for_id(worktree_entries.worktree_id, cx)
            {
                let snapshot = worktree.read(cx).snapshot();
                let root_name = OsStr::new(snapshot.root_name());
                // let expanded_entry_ids = self
                //     .expanded_dir_ids
                //     .get(&snapshot.id())
                //     .map(Vec::as_slice)
                //     .unwrap_or(&[]);

                let entry_range = range.start.saturating_sub(ix)..end_ix - ix;
                let entries = worktree_entries.paths();

                let index_start = entry_range.start;
                for (i, entry) in worktree_entries.visible_entries[entry_range]
                    .iter()
                    .enumerate()
                {
                    let index = index_start + i;
                    let status = entry.status;
                    let is_expanded = true; //expanded_entry_ids.binary_search(&entry.id).is_ok();

                    let (depth, difference) = Self::calculate_depth_and_difference(entry, entries);

                    let filename = match difference {
                        diff if diff > 1 => entry
                            .repo_path
                            .iter()
                            .skip(entry.repo_path.components().count() - diff)
                            .collect::<PathBuf>()
                            .to_str()
                            .unwrap_or_default()
                            .to_string(),
                        _ => entry
                            .repo_path
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_else(|| root_name.to_string_lossy().to_string()),
                    };

                    let details = EntryDetails {
                        filename,
                        display_name: entry.repo_path.to_string_lossy().into_owned(),
                        // TODO get it from StatusEntry?
                        kind: EntryKind::File,
                        is_expanded,
                        path: entry.repo_path.clone(),
                        status: Some(status),
                        hunks: entry.hunks.clone(),
                        depth,
                        index,
                    };
                    callback(ix, details, cx);
                }
            }
            ix = end_ix;
        }
    }

    // TODO: Update expanded directory state
    // TODO: Updates happen in the main loop, could be long for large workspaces
    #[track_caller]
    fn update_visible_entries(
        &mut self,
        for_worktree: Option<WorktreeId>,
        _new_selected_entry: Option<(WorktreeId, ProjectEntryId)>,
        cx: &mut ViewContext<Self>,
    ) {
        let project = self.project.read(cx);
        let mut old_entries_removed = false;
        let mut after_update = Vec::new();
        self.visible_entries
            .retain(|worktree_entries| match for_worktree {
                Some(for_worktree) => {
                    if worktree_entries.worktree_id == for_worktree {
                        old_entries_removed = true;
                        false
                    } else if old_entries_removed {
                        after_update.push(worktree_entries.clone());
                        false
                    } else {
                        true
                    }
                }
                None => false,
            });
        for worktree in project.visible_worktrees(cx) {
            let snapshot = worktree.read(cx).snapshot();
            let worktree_id = snapshot.id();

            if for_worktree.is_some() && for_worktree != Some(worktree_id) {
                continue;
            }

            let mut visible_worktree_entries = Vec::new();
            // Only use the first repository for now
            let repositories = snapshot.repositories().take(1);
            // let mut work_directory = None;
            for repository in repositories {
                visible_worktree_entries.extend(repository.status());
                // work_directory = Some(worktree::WorkDirectory::clone(repository));
            }

            // TODO use the GitTraversal
            // let mut visible_worktree_entries = snapshot
            //     .entries(false, 0)
            //     .filter(|entry| !entry.is_external)
            //     .filter(|entry| entry.git_status.is_some())
            //     .cloned()
            //     .collect::<Vec<_>>();
            // snapshot.propagate_git_statuses(&mut visible_worktree_entries);
            // project::sort_worktree_entries(&mut visible_worktree_entries);

            if !visible_worktree_entries.is_empty() {
                self.visible_entries.push(WorktreeEntries {
                    worktree_id,
                    // work_directory: work_directory.unwrap(),
                    visible_entries: visible_worktree_entries
                        .into_iter()
                        .map(|entry| GitPanelEntry {
                            entry,
                            hunks: Rc::default(),
                        })
                        .collect(),
                    paths: Rc::default(),
                });
            }
        }
        self.visible_entries.extend(after_update);

        // TODO re-implement this
        // if let Some((worktree_id, entry_id)) = new_selected_entry {
        //     self.selected_item = self.visible_entries.iter().enumerate().find_map(
        //         |(worktree_index, worktree_entries)| {
        //             if worktree_entries.worktree_id == worktree_id {
        //                 worktree_entries
        //                     .visible_entries
        //                     .iter()
        //                     .position(|entry| entry.id == entry_id)
        //                     .map(|entry_index| {
        //                         worktree_index * worktree_entries.visible_entries.len()
        //                             + entry_index
        //                     })
        //             } else {
        //                 None
        //             }
        //         },
        //     );
        // }

        // let project = self.project.downgrade();
        // self.git_diff_editor_updates = cx.spawn(|git_panel, mut cx| async move {
        //     cx.background_executor()
        //         .timer(UPDATE_DEBOUNCE)
        //         .await;
        //     let Some(project_buffers) = git_panel
        //         .update(&mut cx, |git_panel, cx| {
        //             futures::future::join_all(git_panel.visible_entries.iter_mut().flat_map(
        //                 |worktree_entries| {
        //                     worktree_entries
        //                         .visible_entries
        //                         .iter()
        //                         .filter_map(|entry| {
        //                             let git_status = entry.status;
        //                             let entry_hunks = entry.hunks.clone();
        //                             let (entry_path, unstaged_changes_task) =
        //                                 project.update(cx, |project, cx| {
        //                                     let entry_path = ProjectPath {
        //                                         worktree_id: worktree_entries.worktree_id,
        //                                         path: worktree_entries.work_directory.unrelativize(&entry.repo_path)?,
        //                                     };
        //                                     let open_task =
        //                                         project.open_path(entry_path.clone(), cx);
        //                                     let unstaged_changes_task =
        //                                         cx.spawn(|project, mut cx| async move {
        //                                             let (_, opened_model) = open_task
        //                                                 .await
        //                                                 .context("opening buffer")?;
        //                                             let buffer = opened_model
        //                                                 .downcast::<Buffer>()
        //                                                 .map_err(|_| {
        //                                                     anyhow::anyhow!(
        //                                                         "accessing buffer for entry"
        //                                                     )
        //                                                 })?;
        //                                             // TODO added files have noop changes and those are not expanded properly in the multi buffer
        //                                             let unstaged_changes = project
        //                                                 .update(&mut cx, |project, cx| {
        //                                                     project.open_unstaged_changes(
        //                                                         buffer.clone(),
        //                                                         cx,
        //                                                     )
        //                                                 })?
        //                                                 .await
        //                                                 .context("opening unstaged changes")?;

        //                                             let hunks = cx.update(|cx| {
        //                                                 entry_hunks
        //                                                     .get_or_init(|| {
        //                                                         match git_status {
        //                                                             GitFileStatus::Added => {
        //                                                                 let buffer_snapshot = buffer.read(cx).snapshot();
        //                                                                 let entire_buffer_range =
        //                                                                     buffer_snapshot.anchor_after(0)
        //                                                                         ..buffer_snapshot
        //                                                                             .anchor_before(
        //                                                                                 buffer_snapshot.len(),
        //                                                                             );
        //                                                                 let entire_buffer_point_range =
        //                                                                     entire_buffer_range
        //                                                                         .clone()
        //                                                                         .to_point(&buffer_snapshot);

        //                                                                 vec![DiffHunk {
        //                                                                     row_range: entire_buffer_point_range
        //                                                                         .start
        //                                                                         .row
        //                                                                         ..entire_buffer_point_range
        //                                                                             .end
        //                                                                             .row,
        //                                                                     buffer_range: entire_buffer_range,
        //                                                                     diff_base_byte_range: 0..0,
        //                                                                 }]
        //                                                             }
        //                                                             GitFileStatus::Modified => {
        //                                                                     let buffer_snapshot =
        //                                                                         buffer.read(cx).snapshot();
        //                                                                     unstaged_changes.read(cx)
        //                                                                         .diff_to_buffer
        //                                                                         .hunks_in_row_range(
        //                                                                             0..BufferRow::MAX,
        //                                                                             &buffer_snapshot,
        //                                                                         )
        //                                                                         .collect()
        //                                                             }
        //                                                             // TODO support these
        //                                                             GitFileStatus::Conflict | GitFileStatus::Deleted | GitFileStatus::Untracked => Vec::new(),
        //                                                         }
        //                                                     }).clone()
        //                                             })?;

        //                                             anyhow::Ok((buffer, unstaged_changes, hunks))
        //                                         });
        //                                     Some((entry_path, unstaged_changes_task))
        //                                 }).ok()??;
        //                             Some((entry_path, unstaged_changes_task))
        //                         })
        //                         .map(|(entry_path, open_task)| async move {
        //                             (entry_path, open_task.await)
        //                         })
        //                         .collect::<Vec<_>>()
        //                 },
        //             ))
        //         })
        //         .ok()
        //     else {
        //         return;
        //     };

        //     let project_buffers = project_buffers.await;
        //     if project_buffers.is_empty() {
        //         return;
        //     }
        //     let mut change_sets = Vec::with_capacity(project_buffers.len());
        //     if let Some(buffer_update_task) = git_panel
        //         .update(&mut cx, |git_panel, cx| {
        //             let editor = git_panel.git_diff_editor.clone()?;
        //             let multi_buffer = editor.read(cx).buffer().clone();
        //             let mut buffers_with_ranges = Vec::with_capacity(project_buffers.len());
        //             for (buffer_path, open_result) in project_buffers {
        //                 if let Some((buffer, unstaged_changes, diff_hunks)) = open_result
        //                     .with_context(|| format!("opening buffer {buffer_path:?}"))
        //                     .log_err()
        //                 {
        //                     change_sets.push(unstaged_changes);
        //                     buffers_with_ranges.push((
        //                         buffer,
        //                         diff_hunks
        //                             .into_iter()
        //                             .map(|hunk| hunk.buffer_range)
        //                             .collect(),
        //                     ));
        //                 }
        //             }

        //             Some(multi_buffer.update(cx, |multi_buffer, cx| {
        //                 multi_buffer.clear(cx);
        //                 multi_buffer.push_multiple_excerpts_with_context_lines(
        //                     buffers_with_ranges,
        //                     DEFAULT_MULTIBUFFER_CONTEXT,
        //                     cx,
        //                 )
        //             }))
        //         })
        //         .ok().flatten()
        //     {
        //         buffer_update_task.await;
        //         git_panel
        //             .update(&mut cx, |git_panel, cx| {
        //                 if let Some(diff_editor) = git_panel.git_diff_editor.as_ref() {
        //                     diff_editor.update(cx, |editor, cx| {
        //                         for change_set in change_sets {
        //                             editor.add_change_set(change_set, cx);
        //                         }
        //                     });
        //                 }
        //             })
        //             .ok();
        //     }
        // });

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
        let item_count = self
            .visible_entries
            .iter()
            .map(|worktree_entries| worktree_entries.visible_entries.len())
            .sum();
        let selected_entry = self.selected_item;
        h_flex()
            .size_full()
            .overflow_hidden()
            .child(
                uniform_list(cx.view().clone(), "entries", item_count, {
                    move |git_panel, range, cx| {
                        let mut items = Vec::with_capacity(range.end - range.start);
                        git_panel.for_each_visible_entry(range, cx, |id, details, cx| {
                            items.push(git_panel.render_entry(
                                id,
                                Some(details.index) == selected_entry,
                                details,
                                cx,
                            ));
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
        selected: bool,
        details: EntryDetails,
        cx: &ViewContext<Self>,
    ) -> impl IntoElement {
        let view_mode = self.view_mode.clone();
        let checkbox_id = ElementId::Name(format!("checkbox_{}", ix).into());
        let is_staged = ToggleState::Selected;
        let handle = cx.view().downgrade();

        // TODO: At this point, an entry should really have a status.
        // Is this fixed with the new git status stuff?
        let status = details.status.unwrap_or(GitFileStatus::Untracked);

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
            .id(("git-panel-entry", ix))
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

        if view_mode == ViewMode::Tree {
            entry = entry.pl(px(12. + 12. * details.depth as f32))
        } else {
            entry = entry.pl(px(12.))
        }

        if selected {
            entry = entry.bg(cx.theme().status().info_background);
        }

        entry = entry
            .child(Checkbox::new(checkbox_id, is_staged))
            .child(git_status_icon(status))
            .child(
                h_flex()
                    .gap_1p5()
                    .when(status == GitFileStatus::Deleted, |this| {
                        this.text_color(cx.theme().colors().text_disabled)
                            .line_through()
                    })
                    .child(details.display_name.clone()),
            )
            .child(div().flex_1())
            .child(end_slot)
            // TODO: Only fire this if the entry is not currently revealed, otherwise the ui flashes
            .on_click(move |e, cx| {
                handle
                    .update(cx, |git_panel, cx| {
                        git_panel.selected_item = Some(details.index);
                        let change_focus = e.down.click_count > 1;
                        git_panel.reveal_entry_in_git_editor(
                            details.hunks.clone(),
                            change_focus,
                            None,
                            cx,
                        );
                    })
                    .ok();
            });

        entry
    }

    fn reveal_entry_in_git_editor(
        &mut self,
        _hunks: Rc<OnceCell<Vec<DiffHunk>>>,
        _change_focus: bool,
        _debounce: Option<Duration>,
        _cx: &mut ViewContext<Self>,
    ) {
        // let workspace = self.workspace.clone();
        // let Some(diff_editor) = self.git_diff_editor.clone() else {
        //     return;
        // };
        // self.reveal_in_editor = cx.spawn(|_, mut cx| async move {
        //     if let Some(debounce) = debounce {
        //         cx.background_executor().timer(debounce).await;
        //     }

        //     let Some(editor) = workspace
        //         .update(&mut cx, |workspace, cx| {
        //             let git_diff_editor = workspace
        //                 .items_of_type::<Editor>(cx)
        //                 .find(|editor| &diff_editor == editor);
        //             match git_diff_editor {
        //                 Some(existing_editor) => {
        //                     workspace.activate_item(&existing_editor, true, change_focus, cx);
        //                     existing_editor
        //                 }
        //                 None => {
        //                     workspace.active_pane().update(cx, |pane, cx| {
        //                         pane.add_item(
        //                          `   diff_editor.boxed_clone(),
        //                             true,
        //                             change_focus,
        //                             None,
        //                             cx,
        //                         )
        //                     });
        //                     diff_editor.clone()
        //                 }
        //             }
        //         })
        //         .ok()
        //     else {
        //         return;
        //     };

        //     if let Some(first_hunk) = hunks.get().and_then(|hunks| hunks.first()) {
        //         let hunk_buffer_range = &first_hunk.buffer_range;
        //         if let Some(buffer_id) = hunk_buffer_range
        //             .start
        //             .buffer_id
        //             .or_else(|| first_hunk.buffer_range.end.buffer_id)
        //         {
        //             editor
        //                 .update(&mut cx, |editor, cx| {
        //                     let multi_buffer = editor.buffer().read(cx);
        //                     let buffer = multi_buffer.buffer(buffer_id)?;
        //                     let buffer_snapshot = buffer.read(cx).snapshot();
        //                     let (excerpt_id, _) = multi_buffer
        //                         .excerpts_for_buffer(&buffer, cx)
        //                         .into_iter()
        //                         .find(|(_, excerpt)| {
        //                             hunk_buffer_range
        //                                 .start
        //                                 .cmp(&excerpt.context.start, &buffer_snapshot)
        //                                 .is_ge()
        //                                 && hunk_buffer_range
        //                                     .end
        //                                     .cmp(&excerpt.context.end, &buffer_snapshot)
        //                                     .is_le()
        //                         })?;
        //                     let multi_buffer_hunk_start = multi_buffer
        //                         .snapshot(cx)
        //                         .anchor_in_excerpt(excerpt_id, hunk_buffer_range.start)?;
        //                     editor.change_selections(
        //                         Some(Autoscroll::Strategy(AutoscrollStrategy::Center)),
        //                         cx,
        //                         |s| {
        //                             s.select_ranges(Some(
        //                                 multi_buffer_hunk_start..multi_buffer_hunk_start,
        //                             ))
        //                         },
        //                     );
        //                     cx.notify();
        //                     Some(())
        //                 })
        //                 .ok()
        //                 .flatten();
        //         }
        //     }
        // });
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
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_prev))
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
