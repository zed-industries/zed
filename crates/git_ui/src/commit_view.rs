use anyhow::{Context as _, Result};
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{
    Editor, EditorEvent, MultiBuffer, MultiBufferOffset, SelectionEffects,
    actions::{OpenExcerpts, OpenExcerptsSplit},
    multibuffer_context_lines,
};

use git::repository::{CommitDetails, CommitDiff, RepoPath};
use gpui::{
    Action, AnyElement, AnyView, App, AppContext as _, AsyncApp, AsyncWindowContext, Context,
    Entity, EventEmitter, FocusHandle, Focusable, IntoElement, PromptLevel, Render, Task,
    WeakEntity, Window, actions,
};
use language::{
    Anchor, Buffer, Capability, DiskState, File, LanguageRegistry, LineEnding, OffsetRangeExt as _,
    Point, ReplicaId, Rope, TextBuffer,
};
use multi_buffer::{BufferOffset, PathKey, ToPoint as _};
use project::{Project, ProjectPath, WorktreeId, git_store::Repository};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Write as _,
    ops::Range,
    path::PathBuf,
    sync::Arc,
};
use text;
use text::BufferId;
use ui::{
    Button, Color, Icon, IconName, Label, LabelCommon as _, SharedString, Tooltip, prelude::*,
};
use util::{ResultExt, paths::PathStyle, rel_path::RelPath, truncate_and_trailoff};
use workspace::{
    Item, ItemHandle, ItemNavHistory, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
    Workspace,
    item::{BreadcrumbText, ItemEvent, TabContentParams},
    notifications::NotifyTaskExt,
    pane::SaveIntent,
    searchable::SearchableItemHandle,
};

use crate::{git_panel::GitPanel, open_historical};

actions!(git, [ApplyCurrentStash, PopCurrentStash, DropCurrentStash,]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        register_workspace_action(workspace, |toolbar, _: &ApplyCurrentStash, window, cx| {
            toolbar.apply_stash(window, cx);
        });
        register_workspace_action(workspace, |toolbar, _: &DropCurrentStash, window, cx| {
            toolbar.remove_stash(window, cx);
        });
        register_workspace_action(workspace, |toolbar, _: &PopCurrentStash, window, cx| {
            toolbar.pop_stash(window, cx);
        });
    })
    .detach();
}

pub struct CommitView {
    commit: CommitDetails,
    editor: Entity<Editor>,
    stash: Option<usize>,
    multibuffer: Entity<MultiBuffer>,
    repository: Entity<Repository>,
}

struct GitBlob {
    path: RepoPath,
    worktree_id: WorktreeId,
    is_deleted: bool,
}

struct CommitMetadataFile {
    title: Arc<RelPath>,
    worktree_id: WorktreeId,
}

const COMMIT_METADATA_SORT_PREFIX: u64 = 0;
const FILE_NAMESPACE_SORT_PREFIX: u64 = 1;

impl CommitView {
    pub fn open(
        commit_sha: String,
        repo: WeakEntity<Repository>,
        workspace: WeakEntity<Workspace>,
        stash: Option<usize>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let commit_diff = repo
            .update(cx, |repo, _| repo.load_commit_diff(commit_sha.clone()))
            .ok();
        let commit_details = repo
            .update(cx, |repo, _| repo.show(commit_sha.clone()))
            .ok();

        window
            .spawn(cx, async move |cx| {
                let (commit_diff, commit_details) = futures::join!(commit_diff?, commit_details?);
                let commit_diff = commit_diff.log_err()?.log_err()?;
                let commit_details = commit_details.log_err()?.log_err()?;
                let repo = repo.upgrade()?;

                workspace
                    .update_in(cx, |workspace, window, cx| {
                        let project = workspace.project();
                        let commit_view = cx.new(|cx| {
                            CommitView::new(
                                commit_details,
                                commit_diff,
                                repo,
                                project.clone(),
                                stash,
                                window,
                                cx,
                            )
                        });

                        let pane = workspace.active_pane();
                        pane.update(cx, |pane, cx| {
                            let ix = pane.items().position(|item| {
                                let commit_view = item.downcast::<CommitView>();
                                commit_view
                                    .is_some_and(|view| view.read(cx).commit.sha == commit_sha)
                            });
                            if let Some(ix) = ix {
                                pane.activate_item(ix, true, true, window, cx);
                            } else {
                                pane.add_item(Box::new(commit_view), true, true, None, window, cx);
                            }
                        })
                    })
                    .log_err()
            })
            .detach();
    }

    fn new(
        commit: CommitDetails,
        commit_diff: CommitDiff,
        repository: Entity<Repository>,
        project: Entity<Project>,
        stash: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let language_registry = project.read(cx).languages().clone();
        let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadOnly));
        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(multibuffer.clone(), Some(project.clone()), window, cx);
            editor.disable_inline_diagnostics();
            editor.set_expand_all_diff_hunks(cx);
            editor
        });

        let first_worktree_id = project
            .read(cx)
            .worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).id());

        let mut metadata_buffer_id = None;
        if let Some(worktree_id) = first_worktree_id {
            let title = if let Some(stash) = stash {
                format!("stash@{{{}}}", stash)
            } else {
                format!("commit {}", commit.sha)
            };
            let file = Arc::new(CommitMetadataFile {
                title: RelPath::unix(&title).unwrap().into(),
                worktree_id,
            });
            let buffer = cx.new(|cx| {
                let buffer = TextBuffer::new_normalized(
                    ReplicaId::LOCAL,
                    cx.entity_id().as_non_zero_u64().into(),
                    LineEnding::default(),
                    format_commit(&commit, stash.is_some()).into(),
                );
                metadata_buffer_id = Some(buffer.remote_id());
                Buffer::build(buffer, Some(file.clone()), Capability::ReadWrite)
            });
            multibuffer.update(cx, |multibuffer, cx| {
                multibuffer.set_excerpts_for_path(
                    PathKey::with_sort_prefix(COMMIT_METADATA_SORT_PREFIX, file.title.clone()),
                    buffer.clone(),
                    vec![Point::zero()..buffer.read(cx).max_point()],
                    0,
                    cx,
                );
            });
            editor.update(cx, |editor, cx| {
                editor.disable_header_for_buffer(metadata_buffer_id.unwrap(), cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges(vec![MultiBufferOffset(0)..MultiBufferOffset(0)]);
                });
            });
        }

        let repo_clone = repository.clone();
        cx.spawn(async move |this, cx| {
            for file in commit_diff.files {
                let is_deleted = file.new_text.is_none();
                let new_text = file.new_text.unwrap_or_default();
                let old_text = file.old_text;
                let worktree_id = repo_clone
                    .update(cx, |repository, cx| {
                        repository
                            .repo_path_to_project_path(&file.path, cx)
                            .map(|path| path.worktree_id)
                            .or(first_worktree_id)
                    })?
                    .context("project has no worktrees")?;
                let file = Arc::new(GitBlob {
                    path: file.path.clone(),
                    is_deleted,
                    worktree_id,
                }) as Arc<dyn language::File>;

                let buffer = build_buffer(new_text, file, &language_registry, cx).await?;
                let buffer_diff =
                    build_buffer_diff(old_text, &buffer, &language_registry, cx).await?;

                this.update(cx, |this, cx| {
                    this.multibuffer.update(cx, |multibuffer, cx| {
                        let snapshot = buffer.read(cx).snapshot();
                        let diff = buffer_diff.read(cx);
                        let diff_hunk_ranges = diff
                            .hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &snapshot, cx)
                            .map(|diff_hunk| diff_hunk.buffer_range.to_point(&snapshot))
                            .collect::<Vec<_>>();
                        let path = snapshot.file().unwrap().path().clone();
                        let _is_newly_added = multibuffer.set_excerpts_for_path(
                            PathKey::with_sort_prefix(FILE_NAMESPACE_SORT_PREFIX, path),
                            buffer,
                            diff_hunk_ranges,
                            multibuffer_context_lines(cx),
                            cx,
                        );
                        multibuffer.add_diff(buffer_diff, cx);
                    });
                })?;
            }
            anyhow::Ok(())
        })
        .detach();

        Self {
            commit,
            editor,
            multibuffer,
            stash,
            repository,
        }
    }

    pub fn open_excerpts(
        &mut self,
        action: &OpenExcerpts,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        log::info!(
            "OpenExcerpts action triggered with target: {:?}",
            action.target
        );
        match action.target {
            editor::actions::ExcerptTarget::OpenCurrent => {
                self.open_current_file_common(false, window, cx)
            }
            editor::actions::ExcerptTarget::OpenHistorical => {
                self.open_historical_file_common(false, false, window, cx)
            }
            editor::actions::ExcerptTarget::OpenModified => {
                self.open_historical_file_common(true, false, window, cx)
            }
            editor::actions::ExcerptTarget::OpenParent => {
                self.open_historical_file_common(false, false, window, cx)
            }
        }
    }

    pub fn open_excerpts_split(
        &mut self,
        action: &OpenExcerptsSplit,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        log::info!(
            "OpenExcerptsSplit action triggered with target: {:?}",
            action.target
        );
        match action.target {
            editor::actions::ExcerptTarget::OpenCurrent => {
                self.open_current_file_common(true, window, cx)
            }
            editor::actions::ExcerptTarget::OpenHistorical => {
                self.open_historical_file_common(false, true, window, cx)
            }
            editor::actions::ExcerptTarget::OpenModified => {
                self.open_historical_file_common(true, true, window, cx)
            }
            editor::actions::ExcerptTarget::OpenParent => {
                self.open_historical_file_common(false, true, window, cx)
            }
        }
    }

    fn open_historical_file_common(
        &mut self,
        is_changed: bool,
        split: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        log::info!(
            "open_historical_file_common: is_changed={}, split={}",
            is_changed,
            split
        );

        let Some(_workspace) = self.editor.read(cx).workspace() else {
            log::warn!("open_historical_file_common: No workspace found");
            cx.propagate();
            return;
        };

        // Get cursor position
        let cursor_position = self.editor.update(cx, |editor, cx| {
            let display_snapshot = editor.display_snapshot(cx);
            editor
                .selections
                .newest::<MultiBufferOffset>(&display_snapshot)
                .head()
        });
        log::debug!(
            "open_historical_file_common: Cursor position: {}",
            cursor_position
        );

        // Get the buffer snapshot at cursor using range_to_buffer_ranges
        let snapshot = self.multibuffer.read(cx).snapshot(cx);
        let mut buffer_info: Option<(Entity<Buffer>, BufferId, RepoPath, WorktreeId)> = None;

        // Try to find buffer at cursor position
        for (buffer_snapshot, _range, _excerpt_id, _anchor) in
            snapshot.range_to_buffer_ranges_with_deleted_hunks(cursor_position..cursor_position)
        {
            if let Some(file) = buffer_snapshot.file() {
                let buffer_id = buffer_snapshot.remote_id();
                let repo_path = RepoPath::from_rel_path(file.path());
                let worktree_id = file.worktree_id(cx);

                if let Some(buffer) = self.multibuffer.read(cx).buffer(buffer_id) {
                    log::info!(
                        "open_historical_file_common: Found buffer at cursor: path={:?}, buffer_id={:?}",
                        repo_path.as_std_path(),
                        buffer_id
                    );
                    buffer_info = Some((buffer, buffer_id, repo_path, worktree_id));
                    break;
                }
            }
        }

        // Fallback: try first excerpt if nothing at cursor
        if buffer_info.is_none() {
            log::warn!("open_historical_file_common: No buffer at cursor, trying first excerpt");
            for (_excerpt_id, buffer_snapshot, _range) in snapshot.excerpts() {
                if let Some(file) = buffer_snapshot.file() {
                    let buffer_id = buffer_snapshot.remote_id();
                    let repo_path = RepoPath::from_rel_path(file.path());
                    let worktree_id = file.worktree_id(cx);

                    if let Some(buffer) = self.multibuffer.read(cx).buffer(buffer_id) {
                        log::info!(
                            "open_historical_file_common: Using first excerpt: path={:?}",
                            repo_path.as_std_path()
                        );
                        buffer_info = Some((buffer, buffer_id, repo_path, worktree_id));
                        break;
                    }
                }
            }
        }

        let Some((buffer, buffer_id, repo_path, worktree_id)) = buffer_info else {
            log::error!("open_historical_file_common: No buffer found in multibuffer");
            return;
        };

        let commit_sha = self.commit.sha.to_string();
        log::info!(
            "open_historical_file_common: Opening historical file: path={:?}, commit={}, is_changed={}",
            repo_path.as_std_path(),
            &commit_sha[..7.min(commit_sha.len())],
            is_changed
        );

        // Get language registry from workspace project
        let language_registry = _workspace.read(cx).project().read(cx).languages().clone();
        log::debug!("open_historical_file_common: Got language registry");

        // Clone necessary data for async block
        let workspace_weak = _workspace.downgrade();
        let project = Some(_workspace.read(cx).project().clone());

        // Spawn the async work
        if is_changed {
            // Open the "changed" version (after commit)
            log::info!("open_historical_file_common: Preparing changed buffer");
            _window
                .spawn(cx, async move |mut cx| {
                    let buffer = open_historical::prepare_changed_buffer_from_commit(
                        buffer,
                        repo_path,
                        worktree_id,
                        commit_sha,
                        language_registry,
                        &mut cx,
                    )
                    .await?;

                    log::info!(
                        "open_historical_file_common: Changed buffer prepared, opening in editor"
                    );

                    // Open the buffer in an editor
                    workspace_weak.update_in(cx, |workspace, window, cx| {
                        let pane = if split {
                            log::debug!("open_historical_file_common: Splitting pane");
                            workspace.split_pane(
                                workspace.active_pane().clone(),
                                workspace::SplitDirection::Right,
                                window,
                                cx,
                            )
                        } else {
                            log::debug!("open_historical_file_common: Using active pane");
                            workspace.active_pane().clone()
                        };

                        let editor = cx.new(|cx| {
                            log::debug!("open_historical_file_common: Creating editor for buffer");
                            let mut editor = Editor::for_buffer(buffer, project, window, cx);
                            editor.set_read_only(true);
                            editor.set_should_serialize(false, cx);
                            editor
                        });

                        pane.update(cx, |pane, cx| {
                            log::info!("open_historical_file_common: Adding editor to pane");
                            pane.add_item(Box::new(editor), true, true, None, window, cx);
                        });
                    })?;

                    log::info!("open_historical_file_common: Successfully opened changed version");
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
        } else {
            // Open the "unchanged" version (before commit)
            log::info!("open_historical_file_common: Preparing unchanged buffer");

            let Some(buffer_diff) = self.multibuffer.read(cx).diff_for(buffer_id) else {
                log::error!(
                    "open_historical_file_common: Could not get buffer_diff for buffer_id: {:?}",
                    buffer_id
                );
                return;
            };
            log::debug!("open_historical_file_common: Got buffer_diff");

            _window
                .spawn(cx, async move |mut cx| {
                    let buffer = open_historical::prepare_unchanged_buffer_from_commit(
                        buffer_diff,
                        repo_path,
                        worktree_id,
                        commit_sha,
                        language_registry,
                        &mut cx,
                    )
                    .await?;

                    log::info!(
                        "open_historical_file_common: Unchanged buffer prepared, opening in editor"
                    );

                    // Open the buffer in an editor
                    workspace_weak.update_in(cx, |workspace, window, cx| {
                        let pane = if split {
                            log::debug!("open_historical_file_common: Splitting pane");
                            workspace.split_pane(
                                workspace.active_pane().clone(),
                                workspace::SplitDirection::Right,
                                window,
                                cx,
                            )
                        } else {
                            log::debug!("open_historical_file_common: Using active pane");
                            workspace.active_pane().clone()
                        };

                        let editor = cx.new(|cx| {
                            log::debug!("open_historical_file_common: Creating editor for buffer");
                            let mut editor = Editor::for_buffer(buffer, project, window, cx);
                            editor.set_read_only(true);
                            editor.set_should_serialize(false, cx);
                            editor
                        });

                        pane.update(cx, |pane, cx| {
                            log::info!("open_historical_file_common: Adding editor to pane");
                            pane.add_item(Box::new(editor), true, true, None, window, cx);
                        });
                    })?;

                    log::info!(
                        "open_historical_file_common: Successfully opened unchanged version"
                    );
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
        }
    }

    fn open_current_file_common(
        &mut self,
        split: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.editor.read(cx).workspace() else {
            cx.propagate();
            return;
        };

        // Build a map of git files to open with their selection ranges
        let mut files_by_repo_path: HashMap<
            RepoPath,
            (Vec<Range<BufferOffset>>, Option<Entity<Buffer>>),
        > = HashMap::default();

        let selections = self.editor.update(cx, |editor, cx| {
            let display_snapshot = editor.display_snapshot(cx);
            editor
                .selections
                .all::<MultiBufferOffset>(&display_snapshot)
        });

        let snapshot = self.multibuffer.read(cx).snapshot(cx);

        // Process all selections to build the map
        for selection in selections {
            for (buffer_snapshot, range, _excerpt_id, anchor) in
                snapshot.range_to_buffer_ranges_with_deleted_hunks(selection.range())
            {
                if let Some(file) = buffer_snapshot.file() {
                    // Check if this is a GitBlob (non-local file)
                    if file.as_local().is_none() {
                        let repo_path = RepoPath::from_rel_path(file.path());

                        // Get the proper cursor position in the buffer
                        let buffer_offset = anchor
                            .map(|a| {
                                BufferOffset(text::ToOffset::to_offset(
                                    &a.text_anchor,
                                    &buffer_snapshot,
                                ))
                            })
                            .unwrap_or(range.start);

                        // Get the buffer handle for this git blob
                        let buffer_handle = if let Some(anchor) = anchor {
                            self.multibuffer.read(cx).buffer_for_anchor(anchor, cx)
                        } else {
                            self.multibuffer
                                .read(cx)
                                .buffer(buffer_snapshot.remote_id())
                        };

                        files_by_repo_path
                            .entry(repo_path)
                            .or_insert((Vec::new(), buffer_handle))
                            .0
                            .push(buffer_offset..buffer_offset);
                    }
                }
            }
        }

        // Filter to only openable files (those that exist in the project)
        let mut openable_files: Vec<(ProjectPath, Vec<Range<BufferOffset>>)> = Vec::new();
        let mut first_unopenable: Option<(RepoPath, Vec<Range<BufferOffset>>)> = None;

        for (repo_path, (ranges, _)) in files_by_repo_path.iter() {
            if let Some(project_path) = self
                .repository
                .read(cx)
                .repo_path_to_project_path(repo_path, cx)
            {
                openable_files.push((project_path, ranges.clone()));
            } else if first_unopenable.is_none() {
                first_unopenable = Some((repo_path.clone(), ranges.clone()));
            }
        }

        // If no files can be opened, use fallback with first unopenable file
        if openable_files.is_empty() {
            if let Some((repo_path, _ranges)) = first_unopenable {
                self.open_file_finder_with_hint(&repo_path, workspace, window, cx);
            } else {
                // No git files found at all - try to extract context from cursor
                let cursor_offset = self.editor.update(cx, |editor, cx| {
                    let display_snapshot = editor.display_snapshot(cx);
                    editor
                        .selections
                        .newest::<MultiBufferOffset>(&display_snapshot)
                        .head()
                });

                if let Some((filename, line_number)) =
                    self.extract_context_from_cursor_position(cursor_offset.0, cx)
                {
                    let hint = format!("{}:{}", filename, line_number);
                    self.open_file_finder_with_filename_hint(&hint, workspace, window, cx);
                } else {
                    self.open_file_finder_fallback(workspace, window, cx);
                }
            }
            return;
        }

        // Open all openable files
        for (project_path, ranges) in openable_files {
            self.open_existing_file(project_path, ranges, split, workspace.clone(), window, cx);
        }
    }

    fn open_existing_file(
        &self,
        project_path: ProjectPath,
        ranges: Vec<Range<BufferOffset>>,
        split: bool,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.defer(cx, move |window, cx| {
            workspace.update(cx, |workspace, cx| {
                let pane = match split {
                    true => workspace.adjacent_pane(window, cx),
                    false => workspace.active_pane().clone(),
                };

                let open_task =
                    workspace.open_path(project_path, Some(pane.downgrade()), true, window, cx);

                window
                    .spawn(cx, async move |cx| {
                        if let Some(active_editor) = open_task
                            .await
                            .log_err()
                            .and_then(|item| item.downcast::<Editor>())
                        {
                            active_editor
                                .update_in(cx, |editor, window, cx| {
                                    // Convert offsets to points for each range
                                    let buffer = editor.buffer().read(cx);
                                    let Some(singleton_buffer) = buffer.as_singleton() else {
                                        return;
                                    };
                                    let buffer_snapshot = singleton_buffer.read(cx).snapshot();

                                    // Convert all offset ranges to point ranges
                                    let point_ranges: Vec<Range<Point>> = ranges
                                        .iter()
                                        .map(|range| {
                                            let start =
                                                buffer_snapshot.offset_to_point(range.start.0);
                                            let end = buffer_snapshot.offset_to_point(range.end.0);
                                            start..end
                                        })
                                        .collect();

                                    // Use the first range for positioning
                                    if let Some(first_range) = point_ranges.first() {
                                        editor.go_to_singleton_buffer_range(
                                            first_range.clone(),
                                            window,
                                            cx,
                                        );
                                    }
                                })
                                .ok();
                        }
                    })
                    .detach();
            })
        });
    }

    fn open_file_finder_with_hint(
        &self,
        repo_path: &RepoPath,
        _workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Extract just the filename for the search query
        let _filename = repo_path
            .file_name()
            .map(|name| name.to_string())
            .unwrap_or_else(String::new);

        window.defer(cx, move |window, cx| {
            // Open new file finder - user can type the filename
            let action = workspace::ToggleFileFinder {
                separate_history: false,
                query: None,
            };
            window.dispatch_action(action.boxed_clone(), cx);
        });
    }

    fn extract_context_from_cursor_position(
        &self,
        cursor_offset: usize,
        cx: &App,
    ) -> Option<(String, u32)> {
        // Try to extract filename and line context from the multibuffer structure
        let multibuffer_snapshot = self.multibuffer.read(cx).snapshot(cx);
        let cursor_point = multibuffer_snapshot.offset_to_point(MultiBufferOffset(cursor_offset));

        // Look through all excerpts to find which file section we're in
        for (excerpt_id, buffer_snapshot, excerpt_range) in multibuffer_snapshot.excerpts() {
            // Convert excerpt range to multibuffer points
            let start_anchor =
                multibuffer_snapshot.anchor_in_excerpt(excerpt_id, excerpt_range.context.start);
            let end_anchor =
                multibuffer_snapshot.anchor_in_excerpt(excerpt_id, excerpt_range.context.end);

            if let (Some(start_anchor), Some(end_anchor)) = (start_anchor, end_anchor) {
                let start_point = start_anchor.to_point(&multibuffer_snapshot);
                let end_point = end_anchor.to_point(&multibuffer_snapshot);

                if start_point <= cursor_point && cursor_point <= end_point {
                    // We're in this excerpt - try to get file info
                    if let Some(file) = buffer_snapshot.file() {
                        // Extract filename from the path
                        let filename = file
                            .path()
                            .file_name()
                            .map(|name| name.to_string())
                            .unwrap_or_else(|| "unknown".to_string());

                        // Calculate approximate line number within the file
                        let excerpt_offset = cursor_point.row.saturating_sub(start_point.row);
                        let line_number = excerpt_offset + 1; // 1-based line numbers

                        return Some((filename, line_number));
                    }
                }
            }
        }

        None
    }

    fn open_file_finder_with_filename_hint(
        &self,
        hint: &str,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let hint = hint.to_string();

        window.defer(cx, move |window, cx| {
            let _ = workspace.update(cx, |_ws, cx| {
                let action = workspace::ToggleFileFinder {
                    separate_history: false,
                    query: Some(hint.clone()),
                };
                window.dispatch_action(action.boxed_clone(), cx);
            });
        });
    }

    fn open_file_finder_fallback(
        &self,
        _workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.defer(cx, move |window, cx| {
            let action = workspace::ToggleFileFinder {
                separate_history: false,
                query: None,
            };
            window.dispatch_action(action.boxed_clone(), cx);
        });
    }
}

impl language::File for GitBlob {
    fn as_local(&self) -> Option<&dyn language::LocalFile> {
        None
    }

    fn disk_state(&self) -> DiskState {
        if self.is_deleted {
            DiskState::Deleted
        } else {
            DiskState::New
        }
    }

    fn path_style(&self, _: &App) -> PathStyle {
        PathStyle::Posix
    }

    fn path(&self) -> &Arc<RelPath> {
        self.path.as_ref()
    }

    fn full_path(&self, _: &App) -> PathBuf {
        self.path.as_std_path().to_path_buf()
    }

    fn file_name<'a>(&'a self, _: &'a App) -> &'a str {
        self.path.file_name().unwrap()
    }

    fn worktree_id(&self, _: &App) -> WorktreeId {
        self.worktree_id
    }

    fn to_proto(&self, _cx: &App) -> language::proto::File {
        unimplemented!()
    }

    fn is_private(&self) -> bool {
        false
    }
}

impl language::File for CommitMetadataFile {
    fn as_local(&self) -> Option<&dyn language::LocalFile> {
        None
    }

    fn disk_state(&self) -> DiskState {
        DiskState::New
    }

    fn path_style(&self, _: &App) -> PathStyle {
        PathStyle::Posix
    }

    fn path(&self) -> &Arc<RelPath> {
        &self.title
    }

    fn full_path(&self, _: &App) -> PathBuf {
        PathBuf::from(self.title.as_unix_str().to_owned())
    }

    fn file_name<'a>(&'a self, _: &'a App) -> &'a str {
        self.title.file_name().unwrap()
    }

    fn worktree_id(&self, _: &App) -> WorktreeId {
        self.worktree_id
    }

    fn to_proto(&self, _: &App) -> language::proto::File {
        unimplemented!()
    }

    fn is_private(&self) -> bool {
        false
    }
}

async fn build_buffer(
    mut text: String,
    blob: Arc<dyn File>,
    language_registry: &Arc<language::LanguageRegistry>,
    cx: &mut AsyncApp,
) -> Result<Entity<Buffer>> {
    let line_ending = LineEnding::detect(&text);
    LineEnding::normalize(&mut text);
    let text = Rope::from(text);
    let language = cx.update(|cx| language_registry.language_for_file(&blob, Some(&text), cx))?;
    let language = if let Some(language) = language {
        language_registry
            .load_language(&language)
            .await
            .ok()
            .and_then(|e| e.log_err())
    } else {
        None
    };
    let buffer = cx.new(|cx| {
        let buffer = TextBuffer::new_normalized(
            ReplicaId::LOCAL,
            cx.entity_id().as_non_zero_u64().into(),
            line_ending,
            text,
        );
        let mut buffer = Buffer::build(buffer, Some(blob), Capability::ReadWrite);
        buffer.set_language(language, cx);
        buffer
    })?;
    Ok(buffer)
}

async fn build_buffer_diff(
    mut old_text: Option<String>,
    buffer: &Entity<Buffer>,
    language_registry: &Arc<LanguageRegistry>,
    cx: &mut AsyncApp,
) -> Result<Entity<BufferDiff>> {
    if let Some(old_text) = &mut old_text {
        LineEnding::normalize(old_text);
    }

    let buffer = cx.update(|cx| buffer.read(cx).snapshot())?;

    let base_buffer = cx
        .update(|cx| {
            Buffer::build_snapshot(
                old_text.as_deref().unwrap_or("").into(),
                buffer.language().cloned(),
                Some(language_registry.clone()),
                cx,
            )
        })?
        .await;

    let diff_snapshot = cx
        .update(|cx| {
            BufferDiffSnapshot::new_with_base_buffer(
                buffer.text.clone(),
                old_text.map(Arc::new),
                base_buffer,
                cx,
            )
        })?
        .await;

    cx.new(|cx| {
        let mut diff = BufferDiff::new(&buffer.text, cx);
        diff.set_snapshot(diff_snapshot, &buffer.text, cx);
        diff
    })
}

fn format_commit(commit: &CommitDetails, is_stash: bool) -> String {
    let mut result = String::new();
    if is_stash {
        writeln!(&mut result, "stash commit {}", commit.sha).unwrap();
    } else {
        writeln!(&mut result, "commit {}", commit.sha).unwrap();
    }
    writeln!(
        &mut result,
        "Author: {} <{}>",
        commit.author_name, commit.author_email
    )
    .unwrap();
    let local_offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
    writeln!(
        &mut result,
        "Date:   {}",
        time_format::format_localized_timestamp(
            time::OffsetDateTime::from_unix_timestamp(commit.commit_timestamp).unwrap(),
            time::OffsetDateTime::now_utc(),
            local_offset,
            time_format::TimestampFormat::MediumAbsolute,
        ),
    )
    .unwrap();
    result.push('\n');
    for line in commit.message.split('\n') {
        if line.is_empty() {
            result.push('\n');
        } else {
            writeln!(&mut result, "    {}", line).unwrap();
        }
    }
    if result.ends_with("\n\n") {
        result.pop();
    }
    result
}

impl EventEmitter<EditorEvent> for CommitView {}

impl Focusable for CommitView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for CommitView {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::GitBranch).color(Color::Muted))
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(params.detail.unwrap_or_default(), cx))
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        let short_sha = self.commit.sha.get(0..7).unwrap_or(&*self.commit.sha);
        let subject = truncate_and_trailoff(self.commit.message.split('\n').next().unwrap(), 20);
        format!("{short_sha} - {subject}").into()
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<ui::SharedString> {
        let short_sha = self.commit.sha.get(0..16).unwrap_or(&*self.commit.sha);
        let subject = self.commit.message.split('\n').next().unwrap();
        Some(format!("{short_sha} - {subject}").into())
    }

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Commit View Opened")
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
        } else {
            None
        }
    }

    fn as_searchable(&self, _: &Entity<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.editor.for_each_project_item(cx, f)
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn navigate(
        &mut self,
        data: Box<dyn Any>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn breadcrumb_location(&self, _: &App) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &App) -> Option<Vec<BreadcrumbText>> {
        self.editor.breadcrumbs(theme, cx)
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx)
        });
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        Task::ready(Some(cx.new(|cx| {
            let editor = cx.new(|cx| {
                self.editor
                    .update(cx, |editor, cx| editor.clone(window, cx))
            });
            let multibuffer = editor.read(cx).buffer().clone();
            Self {
                editor,
                multibuffer,
                commit: self.commit.clone(),
                repository: self.repository.clone(),
                stash: self.stash,
            }
        })))
    }
}

impl Render for CommitView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_stash = self.stash.is_some();
        let context = if is_stash { "StashDiff" } else { "CommitDiff" };

        // Create action handlers
        let open_excerpts_handler = cx.listener(|this: &mut Self, action, window, cx| {
            this.open_excerpts(action, window, cx);
        });

        let open_excerpts_split_handler = cx.listener(|this: &mut Self, action, window, cx| {
            this.open_excerpts_split(action, window, cx);
        });

        div()
            .key_context(context)
            .bg(cx.theme().colors().editor_background)
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .on_action(open_excerpts_handler)
            .on_action(open_excerpts_split_handler)
            .child(self.editor.clone())
    }
}

pub struct CommitViewToolbar {
    commit_view: Option<WeakEntity<CommitView>>,
    workspace: WeakEntity<Workspace>,
}

impl CommitViewToolbar {
    pub fn new(workspace: &Workspace, _cx: &mut Context<Self>) -> Self {
        Self {
            commit_view: None,
            workspace: workspace.weak_handle(),
        }
    }

    fn commit_view(&self, _: &App) -> Option<Entity<CommitView>> {
        self.commit_view.as_ref()?.upgrade()
    }

    async fn close_commit_view(
        commit_view: Entity<CommitView>,
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncWindowContext,
    ) -> anyhow::Result<()> {
        workspace
            .update_in(cx, |workspace, window, cx| {
                let active_pane = workspace.active_pane();
                let commit_view_id = commit_view.entity_id();
                active_pane.update(cx, |pane, cx| {
                    pane.close_item_by_id(commit_view_id, SaveIntent::Skip, window, cx)
                })
            })?
            .await?;
        anyhow::Ok(())
    }

    fn apply_stash(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.stash_action(
            "Apply",
            window,
            cx,
            async move |repository, sha, stash, commit_view, workspace, cx| {
                let result = repository.update(cx, |repo, cx| {
                    if !stash_matches_index(&sha, stash, repo) {
                        return Err(anyhow::anyhow!("Stash has changed, not applying"));
                    }
                    Ok(repo.stash_apply(Some(stash), cx))
                })?;

                match result {
                    Ok(task) => task.await?,
                    Err(err) => {
                        Self::close_commit_view(commit_view, workspace, cx).await?;
                        return Err(err);
                    }
                };
                Self::close_commit_view(commit_view, workspace, cx).await?;
                anyhow::Ok(())
            },
        );
    }

    fn pop_stash(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.stash_action(
            "Pop",
            window,
            cx,
            async move |repository, sha, stash, commit_view, workspace, cx| {
                let result = repository.update(cx, |repo, cx| {
                    if !stash_matches_index(&sha, stash, repo) {
                        return Err(anyhow::anyhow!("Stash has changed, pop aborted"));
                    }
                    Ok(repo.stash_pop(Some(stash), cx))
                })?;

                match result {
                    Ok(task) => task.await?,
                    Err(err) => {
                        Self::close_commit_view(commit_view, workspace, cx).await?;
                        return Err(err);
                    }
                };
                Self::close_commit_view(commit_view, workspace, cx).await?;
                anyhow::Ok(())
            },
        );
    }

    fn remove_stash(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.stash_action(
            "Drop",
            window,
            cx,
            async move |repository, sha, stash, commit_view, workspace, cx| {
                let result = repository.update(cx, |repo, cx| {
                    if !stash_matches_index(&sha, stash, repo) {
                        return Err(anyhow::anyhow!("Stash has changed, drop aborted"));
                    }
                    Ok(repo.stash_drop(Some(stash), cx))
                })?;

                match result {
                    Ok(task) => task.await??,
                    Err(err) => {
                        Self::close_commit_view(commit_view, workspace, cx).await?;
                        return Err(err);
                    }
                };
                Self::close_commit_view(commit_view, workspace, cx).await?;
                anyhow::Ok(())
            },
        );
    }

    fn stash_action<AsyncFn>(
        &mut self,
        str_action: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
        callback: AsyncFn,
    ) where
        AsyncFn: AsyncFnOnce(
                Entity<Repository>,
                &SharedString,
                usize,
                Entity<CommitView>,
                WeakEntity<Workspace>,
                &mut AsyncWindowContext,
            ) -> anyhow::Result<()>
            + 'static,
    {
        let Some(commit_view) = self.commit_view(cx) else {
            return;
        };
        let Some(stash) = commit_view.read(cx).stash else {
            return;
        };
        let sha = commit_view.read(cx).commit.sha.clone();
        let answer = window.prompt(
            PromptLevel::Info,
            &format!("{} stash@{{{}}}?", str_action, stash),
            None,
            &[str_action, "Cancel"],
            cx,
        );

        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |_, cx| {
            if answer.await != Ok(0) {
                return anyhow::Ok(());
            }
            let repo = workspace.update(cx, |workspace, cx| {
                workspace
                    .panel::<GitPanel>(cx)
                    .and_then(|p| p.read(cx).active_repository.clone())
            })?;

            let Some(repo) = repo else {
                return Ok(());
            };
            callback(repo, &sha, stash, commit_view, workspace, cx).await?;
            anyhow::Ok(())
        })
        .detach_and_notify_err(window, cx);
    }
}

impl EventEmitter<ToolbarItemEvent> for CommitViewToolbar {}

impl ToolbarItemView for CommitViewToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        if let Some(entity) = active_pane_item.and_then(|i| i.act_as::<CommitView>(cx)) {
            self.commit_view = Some(entity.downgrade());
            return ToolbarItemLocation::PrimaryRight;
        }
        ToolbarItemLocation::Hidden
    }

    fn pane_focus_update(
        &mut self,
        _pane_focused: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

impl Render for CommitViewToolbar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(commit_view) = self.commit_view(cx) else {
            return div();
        };

        let is_stash = commit_view.read(cx).stash.is_some();

        // Only show toolbar for stashes
        if !is_stash {
            return div();
        }

        let focus_handle = commit_view.focus_handle(cx);

        h_group_xl()
            .my_neg_1()
            .py_1()
            .w_full()
            .items_center()
            .justify_between()
            .child(
                h_group_sm()
                    .child(
                        Button::new("apply-stash", "Apply")
                            .tooltip(Tooltip::for_action_title_in(
                                "Apply current stash",
                                &ApplyCurrentStash,
                                &focus_handle,
                            ))
                            .on_click(
                                cx.listener(|this, _, window, cx| this.apply_stash(window, cx)),
                            ),
                    )
                    .child(
                        Button::new("pop-stash", "Pop")
                            .tooltip(Tooltip::for_action_title_in(
                                "Pop current stash",
                                &PopCurrentStash,
                                &focus_handle,
                            ))
                            .on_click(
                                cx.listener(|this, _, window, cx| this.pop_stash(window, cx)),
                            ),
                    )
                    .child(
                        Button::new("remove-stash", "Remove")
                            .icon(IconName::Trash)
                            .tooltip(Tooltip::for_action_title_in(
                                "Remove current stash",
                                &DropCurrentStash,
                                &focus_handle,
                            ))
                            .on_click(
                                cx.listener(|this, _, window, cx| this.remove_stash(window, cx)),
                            ),
                    ),
            )
            .child(div())
    }
}

fn register_workspace_action<A: Action>(
    workspace: &mut Workspace,
    callback: fn(&mut CommitViewToolbar, &A, &mut Window, &mut Context<CommitViewToolbar>),
) {
    workspace.register_action(move |workspace, action: &A, window, cx| {
        if workspace.has_active_modal(window, cx) {
            cx.propagate();
            return;
        }

        workspace.active_pane().update(cx, |pane, cx| {
            pane.toolbar().update(cx, move |workspace, cx| {
                if let Some(toolbar) = workspace.item_of_type::<CommitViewToolbar>() {
                    toolbar.update(cx, move |toolbar, cx| {
                        callback(toolbar, action, window, cx);
                        cx.notify();
                    });
                }
            });
        })
    });
}

fn stash_matches_index(sha: &str, index: usize, repo: &mut Repository) -> bool {
    match repo
        .cached_stash()
        .entries
        .iter()
        .find(|entry| entry.index == index)
    {
        Some(entry) => entry.oid.to_string() == sha,
        None => false,
    }
}
