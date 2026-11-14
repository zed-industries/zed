use anyhow::{Context as _, Result};
use buffer_diff::BufferDiff;
use collections::HashMap;
use editor::{
    Addon, Editor, EditorEvent, EditorSettings, HiddenDiffHunkRenderer, MultiBuffer, RowExt,
    SelectionEffects, SplittableEditor,
    display_map::{Block, DisplayRow, ToDisplayPoint},
    hover_markdown_style, multibuffer_context_lines,
    scroll::{Autoscroll, ScrollAnchor},
};
use futures_lite::future::yield_now;
use git::repository::{CommitDetails, RepoPath};
use git::status::{FileStatus, StatusCode, TrackedStatus};
use git::{
    BuildCommitPermalinkParams, GitHostingProviderRegistry, GitRemote, ParsedGitRemote,
    parse_git_remote_url,
};
use gpui::{
    AnyElement, App, AppContext as _, AsyncWindowContext, ClipboardItem, Context, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement,
    PromptLevel, Render, ScrollHandle, StatefulInteractiveElement as _, Styled, Subscription, Task,
    WeakEntity, Window, actions,
};
use language::{
    Buffer, BufferId, Capability, DiskState, File, LanguageRegistry, LineEnding,
    OffsetRangeExt as _, ReplicaId, Rope, TextBuffer,
};
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use multi_buffer::{BufferOffset, PathKey};
use project::{
    Project, ProjectPath, WorktreeId,
    git_store::{CommitDiff, CommitFile, Repository, UnshallowState},
};
use settings::{DiffViewStyle, Settings};
use std::{
    any::{Any, TypeId},
    collections::HashSet,
    ops::Range,
    path::PathBuf,
    sync::Arc,
};
use theme::ActiveTheme;
use ui::{ContextMenu, DiffStat, Disclosure, Divider, Tooltip, WithScrollbar, prelude::*};
use util::{ResultExt, paths::PathStyle, rel_path::RelPath, truncate_and_trailoff};
use workspace::item::TabTooltipContent;
use workspace::{
    Item, ItemHandle, ItemNavHistory, Pane, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
    Workspace, item::ItemEvent, notifications::NotifyTaskExt, pane::SaveIntent,
    searchable::SearchableItemHandle,
};

use crate::commit_tooltip::CommitAvatar;
use crate::git_panel::GitPanel;

actions!(
    git,
    [
        ApplyCurrentStash,
        PopCurrentStash,
        DropCurrentStash,
        OpenFileAtHead,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|workspace, _: &ApplyCurrentStash, window, cx| {
            CommitView::apply_stash(workspace, window, cx);
        });
        workspace.register_action(|workspace, _: &DropCurrentStash, window, cx| {
            CommitView::remove_stash(workspace, window, cx);
        });
        workspace.register_action(|workspace, _: &PopCurrentStash, window, cx| {
            CommitView::pop_stash(workspace, window, cx);
        });
    })
    .detach();
}

pub struct CommitView {
    commit: CommitDetails,
    editor: Entity<SplittableEditor>,
    message: Entity<Markdown>,
    message_expanded: bool,
    message_scroll_handle: ScrollHandle,
    stash: Option<usize>,
    multibuffer: Entity<MultiBuffer>,
    repository: Entity<Repository>,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    remote: Option<GitRemote>,
    is_shallow_boundary: bool,
    file_filter: Option<RepoPath>,
    reveal_file: Option<RepoPath>,
    /// Whether to show the whole content of each file instead of only its changed hunks.
    show_full_file: bool,
    _editor_subscription: Subscription,
    _load_diff_task: Task<Result<()>>,
}

pub(crate) struct GitBlob {
    pub(crate) path: RepoPath,
    pub(crate) worktree_id: WorktreeId,
    pub(crate) is_deleted: bool,
    pub(crate) is_binary: bool,
    pub(crate) display_name: String,
}

pub(crate) fn worktree_id_for_repo_path(
    repository: &Repository,
    project: &Project,
    path: &RepoPath,
    cx: &App,
) -> Option<WorktreeId> {
    repository
        .repo_path_to_project_path(path, cx)
        .map(|project_path| project_path.worktree_id)
        .or_else(|| {
            let (worktree, _) = project.find_worktree(&repository.work_directory_abs_path, cx)?;
            Some(worktree.read(cx).id())
        })
        .or_else(|| {
            project
                .worktrees(cx)
                .next()
                .map(|worktree| worktree.read(cx).id())
        })
}

struct CommitDiffAddon {
    file_statuses: HashMap<language::BufferId, FileStatus>,
    commit_view: WeakEntity<CommitView>,
}

impl Addon for CommitDiffAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn override_status_for_buffer_id(
        &self,
        buffer_id: language::BufferId,
        _cx: &App,
    ) -> Option<FileStatus> {
        self.file_statuses.get(&buffer_id).copied()
    }

    fn extend_buffer_header_context_menu(
        &self,
        menu: ContextMenu,
        buffer: &language::BufferSnapshot,
        _window: &mut Window,
        cx: &mut App,
    ) -> ContextMenu {
        let file_to_open = buffer.file().and_then(|file| {
            let commit_view = self.commit_view.upgrade()?;
            let commit_view = commit_view.read(cx);
            let project_path = commit_view
                .repository
                .read(cx)
                .repo_path_to_project_path(&RepoPath::from_rel_path(file.path()), cx)?;
            let exists_at_head = commit_view
                .workspace
                .upgrade()?
                .read(cx)
                .project()
                .read(cx)
                .entry_for_path(&project_path, cx)
                .is_some();
            exists_at_head.then(|| file.clone())
        });

        menu.when_some(file_to_open, |menu, file| {
            let commit_view = self.commit_view.clone();
            menu.entry(
                "Open File in Project",
                Some(Box::new(OpenFileAtHead)),
                move |window, cx| {
                    commit_view
                        .update(cx, |view, cx| {
                            view.open_file_at_head(&file, None, window, cx)
                        })
                        .log_err();
                },
            )
        })
    }
}

const FILE_NAMESPACE_SORT_PREFIX: u64 = 1;

#[derive(Clone, Default)]
pub struct CommitViewOptions {
    /// The zero-based stash index when displaying a stash instead of a regular
    /// commit.
    pub stash: Option<usize>,
    /// Restricts the displayed diff to this repository-relative file path.
    /// When absent, all files in the commit are displayed.
    pub file_filter: Option<RepoPath>,
    /// Loads this repository-relative file first, places the cursor at its
    /// first excerpt, and reveals its header at the top of the viewport.
    /// Other files remain visible in their normal order, subject to
    /// `file_filter`.
    /// If the file is absent from the displayed diff, normal loading is
    /// unchanged.
    pub reveal_file: Option<RepoPath>,
    /// Shows the whole content of each file instead of only its changed hunks.
    pub show_full_file: bool,
    /// Selects these ranges in the file's content at this commit and scrolls to them. Used with
    /// `file_filter` and `show_full_file`; ignored for a file revealed by `reveal_file`.
    pub selections: Vec<Range<language::Point>>,
}

/// Where [`CommitView::open_file`] shows a file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OpenFileTarget {
    /// The file's full diff at this commit, next to the commit view.
    FullFileDiff,
    /// The file's full diff at this commit, in the pane right of the commit view's.
    FullFileDiffInSplit,
    /// The file in the project.
    Project,
}

impl CommitView {
    pub fn open(
        commit_sha: String,
        repo: WeakEntity<Repository>,
        workspace: WeakEntity<Workspace>,
        options: CommitViewOptions,
        window: &mut Window,
        cx: &mut App,
    ) {
        Self::open_with_options(commit_sha, repo, workspace, options, false, window, cx)
    }

    pub(crate) fn commit_sha(&self) -> &SharedString {
        &self.commit.sha
    }

    pub(crate) fn repository(&self) -> &Entity<Repository> {
        &self.repository
    }

    fn open_with_options(
        commit_sha: String,
        repo: WeakEntity<Repository>,
        workspace: WeakEntity<Workspace>,
        options: CommitViewOptions,
        ignore_shallow_boundary: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        let commit_diff = repo
            .update(cx, |repo, cx| {
                repo.load_commit_diff(commit_sha.clone(), ignore_shallow_boundary, cx)
            })
            .ok();
        let commit_details = repo
            .update(cx, |repo, cx| repo.show_commit(commit_sha.clone(), cx))
            .ok();

        window
            .spawn(cx, async move |cx| {
                let commit_diff = commit_diff?;
                let commit_details = commit_details?;
                let (commit_diff, commit_details) = futures::join!(commit_diff, commit_details);
                let mut commit_diff = commit_diff.log_err()?;
                let commit_details = commit_details.log_err()?;

                // Filter to specific file if requested
                if let Some(ref filter_path) = options.file_filter {
                    commit_diff.files.retain(|f| &f.path == filter_path);
                }

                let repo = repo.upgrade()?;

                workspace
                    .update_in(cx, |workspace, window, cx| {
                        let project = workspace.project();
                        let workspace_entity = cx.entity();
                        let commit_view = cx.new(|cx| {
                            CommitView::new(
                                commit_details,
                                commit_diff,
                                repo,
                                project.clone(),
                                workspace_entity,
                                options,
                                window,
                                cx,
                            )
                        });
                        Self::add_to_pane(workspace.active_pane(), commit_view, window, cx);
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
        workspace: Entity<Workspace>,
        options: CommitViewOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let CommitViewOptions {
            stash,
            file_filter,
            reveal_file,
            show_full_file,
            selections,
        } = options;
        let language_registry = project.read(cx).languages().clone();
        let is_shallow_boundary = commit_diff.is_shallow_boundary;
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadOnly);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let message = cx.new(|cx| {
            Markdown::new(
                commit.message.clone(),
                Some(language_registry.clone()),
                None,
                cx,
            )
        });

        let editor = cx.new(|cx| {
            let editor = SplittableEditor::new(
                EditorSettings::get_global(cx).diff_view_style,
                multibuffer.clone(),
                project.clone(),
                workspace.clone(),
                window,
                cx,
            );
            editor.set_diff_hunk_renderer(Some(Arc::new(HiddenDiffHunkRenderer)), cx);

            editor.rhs_editor().update(cx, |editor, cx| {
                editor.set_show_bookmarks(false, cx);
                editor.set_show_breakpoints(false, cx);
                editor.set_show_diff_review_button(true, cx);
                editor.set_delegate_open_excerpts(true);
            });

            editor
        });
        let editor_subscription = Self::subscribe_to_editor(&editor, window, cx);
        let commit_sha = Arc::<str>::from(commit.sha.as_ref());

        let repository_clone = repository.clone();
        let project_clone = project.clone();
        let reveal_file_clone = reveal_file.clone();
        let mut files = commit_diff.files;
        let commit_view_settings = EditorSettings::get_global(cx).commit_view;
        // A view filtered to a single file is opened to look at that file, so keep it expanded.
        let collapse_files = !commit_view_settings.expand_files && file_filter.is_none();

        let load_diff_task = cx.spawn_in(window, async move |this, cx| {
            let mut folded_buffer_ids: HashSet<language::BufferId> = HashSet::default();
            let mut file_statuses: HashMap<language::BufferId, FileStatus> = HashMap::default();

            // In case there's a file we should scroll to, i.e.,
            // `CommitViewOptions::reveal_file` is set, we'll append that file
            // first and immediately set the scroll anchor to its buffer
            // header.
            let target_file = reveal_file_clone
                .as_ref()
                .and_then(|target_path| files.iter().position(|file| &file.path == target_path))
                .map(|index| files.remove(index));

            if let Some(file) = target_file {
                let is_binary = file.is_binary;
                let (buffer_id, file_status) = Self::append_commit_file(
                    &this,
                    file,
                    &repository_clone,
                    &project_clone,
                    &commit_sha,
                    &language_registry,
                    show_full_file,
                    cx,
                )
                .await?;

                file_statuses.insert(buffer_id, file_status);

                // The revealed file is the one asked for, so keep it expanded.
                if is_binary {
                    folded_buffer_ids.insert(buffer_id);
                }

                // Establish the scroll anchor before inserting files that may
                // end up being sorted before it.
                this.update_in(cx, |this, window, cx| {
                    this.reveal_first_file(buffer_id, window, cx);
                })?;
            }

            let mut selections = selections;
            for file in files {
                let is_binary = file.is_binary;
                let (buffer_id, file_status) = Self::append_commit_file(
                    &this,
                    file,
                    &repository_clone,
                    &project_clone,
                    &commit_sha,
                    &language_registry,
                    show_full_file,
                    cx,
                )
                .await?;

                file_statuses.insert(buffer_id, file_status);

                if is_binary || collapse_files {
                    folded_buffer_ids.insert(buffer_id);
                }

                if !selections.is_empty() {
                    let selections = std::mem::take(&mut selections);
                    this.update_in(cx, |this, window, cx| {
                        this.select_ranges(buffer_id, selections, window, cx);
                    })?;
                }
            }

            this.update_in(cx, |this, _window, cx| {
                let commit_view = cx.weak_entity();

                this.editor.update(cx, |editor, cx| {
                    editor.rhs_editor().update(cx, |editor, cx| {
                        editor.register_addon(CommitDiffAddon {
                            file_statuses,
                            commit_view,
                        });

                        if !folded_buffer_ids.is_empty() {
                            editor.fold_buffers(folded_buffer_ids, cx);
                        }
                    });
                });
            })?;

            anyhow::Ok(())
        });

        let snapshot = repository.read(cx).snapshot();
        let remote_url = snapshot
            .remote_upstream_url
            .as_ref()
            .or(snapshot.remote_origin_url.as_ref());

        let remote = remote_url.and_then(|url| {
            let provider_registry = GitHostingProviderRegistry::default_global(cx);
            parse_git_remote_url(provider_registry, url).map(|(host, parsed)| GitRemote {
                host,
                owner: parsed.owner.into(),
                repo: parsed.repo.into(),
            })
        });

        Self {
            commit,
            editor,
            message,
            message_expanded: commit_view_settings.expand_description,
            message_scroll_handle: ScrollHandle::new(),
            multibuffer,
            stash,
            repository,
            project,
            workspace: workspace.downgrade(),
            remote,
            is_shallow_boundary,
            file_filter,
            reveal_file,
            show_full_file,
            _editor_subscription: editor_subscription,
            _load_diff_task: load_diff_task,
        }
    }

    /// Selects `ranges` in the buffer's content at this commit, centered in the view.
    fn select_ranges(
        &mut self,
        buffer_id: BufferId,
        ranges: Vec<Range<language::Point>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(buffer) = self.multibuffer.read(cx).buffer(buffer_id) else {
            return;
        };
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |editor, cx| {
                let multibuffer = editor.buffer().read(cx);
                let anchor_ranges = ranges
                    .into_iter()
                    .filter_map(|range| {
                        let start = multibuffer.buffer_point_to_anchor(&buffer, range.start, cx)?;
                        let end = multibuffer.buffer_point_to_anchor(&buffer, range.end, cx)?;
                        Some(start..end)
                    })
                    .collect::<Vec<_>>();
                if anchor_ranges.is_empty() {
                    return;
                }
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::center()),
                    window,
                    cx,
                    |selections| selections.select_anchor_ranges(anchor_ranges),
                );
            });
        });
    }

    async fn append_commit_file(
        commit_view: &WeakEntity<Self>,
        file: CommitFile,
        repository: &Entity<Repository>,
        project: &Entity<Project>,
        commit_sha: &str,
        language_registry: &Arc<LanguageRegistry>,
        show_full_file: bool,
        cx: &mut AsyncWindowContext,
    ) -> Result<(BufferId, FileStatus)> {
        let is_created = file.old_text.is_none();
        let is_deleted = file.new_text.is_none();
        let is_binary = file.is_binary;
        let raw_old_text = file.old_text;
        let raw_new_text = file.new_text.unwrap_or_default();
        let old_text = if is_binary { None } else { raw_old_text };
        let new_text = if is_binary {
            "(binary file not shown)".to_string()
        } else {
            raw_new_text
        };

        let worktree_id = repository
            .read_with(cx, |repository, cx| {
                worktree_id_for_repo_path(repository, project.read(cx), &file.path, cx)
            })
            .context("project has no worktrees")?;

        let short_sha = commit_sha
            .get(0..git::SHORT_SHA_LENGTH)
            .unwrap_or(&commit_sha);
        let file_name = file
            .path
            .file_name()
            .map(|name| name.to_string())
            .unwrap_or_else(|| file.path.display(PathStyle::local()).to_string());
        let display_name = format!("{short_sha} - {file_name}");
        let file = Arc::new(GitBlob {
            path: file.path.clone(),
            is_deleted,
            is_binary,
            worktree_id,
            display_name,
        }) as Arc<dyn language::File>;

        let buffer = build_buffer(new_text, file, language_registry, cx).await?;
        let buffer_id = cx.update(|_window, cx| buffer.read(cx).remote_id())?;

        let status_code = match (is_created, is_deleted) {
            (true, _) => StatusCode::Added,
            (_, true) => StatusCode::Deleted,
            (_, _) => StatusCode::Modified,
        };
        let file_status = FileStatus::Tracked(TrackedStatus {
            index_status: status_code,
            worktree_status: StatusCode::Unmodified,
        });

        let buffer_diff = if is_binary {
            cx.update(|_window, cx| {
                let snapshot = buffer.read(cx).snapshot();

                cx.new(|cx| {
                    BufferDiff::new_unchanged(
                        &snapshot,
                        snapshot.language().cloned(),
                        Some(language_registry.clone()),
                        cx,
                    )
                })
            })?
        } else {
            build_buffer_diff(old_text, &buffer, language_registry, cx).await?
        };

        let (excerpt_ranges, path) = cx.update(|_window, cx| {
            let snapshot = buffer.read(cx).snapshot();
            let path = PathKey::with_sort_prefix(
                FILE_NAMESPACE_SORT_PREFIX,
                snapshot.file().unwrap().path().clone(),
            );

            let ranges = if is_binary || show_full_file {
                vec![language::Point::zero()..snapshot.max_point()]
            } else {
                let diff_snapshot = buffer_diff.read(cx).snapshot(cx);
                let mut hunks = diff_snapshot.hunks(&snapshot).peekable();

                if hunks.peek().is_none() {
                    vec![language::Point::zero()..snapshot.max_point()]
                } else {
                    hunks
                        .map(|hunk| hunk.buffer_range.to_point(&snapshot))
                        .collect::<Vec<_>>()
                }
            };

            (ranges, path)
        })?;

        // Batch the insertion of excerpts and yield between batches, to avoid
        // blocking the main thread when a single file has many hunks.
        const EXCERPT_BATCH_SIZE: usize = 10;
        let total = excerpt_ranges.len();
        let mut batch_end = 0;
        while batch_end < total {
            let is_first_batch = batch_end == 0;
            batch_end = (batch_end + EXCERPT_BATCH_SIZE).min(total);
            let ranges = excerpt_ranges[..batch_end].to_vec();
            commit_view.update_in(cx, |commit_view, window, cx| {
                commit_view.editor.update(cx, |editor, cx| {
                    editor.update_excerpts_for_path(
                        path.clone(),
                        buffer.clone(),
                        ranges,
                        multibuffer_context_lines(cx),
                        buffer_diff.clone(),
                        cx,
                    );
                    if is_first_batch && editor.diff_view_style() == DiffViewStyle::Split {
                        editor.split(window, cx);
                    }
                });
            })?;
            if batch_end < total {
                yield_now().await;
            }
        }

        Ok((buffer_id, file_status))
    }

    /// Positions the file's header at the top of the viewport and places the cursor
    /// at the start of its first excerpt, without adding a navigation history entry.
    ///
    /// The buffer must be the first file inserted into the view, with its header
    /// within the initial header-height display rows. Call this before appending
    /// other files so the scroll anchor preserves its position as they are inserted.
    fn reveal_first_file(
        &mut self,
        buffer_id: BufferId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |editor, cx| {
                let display_snapshot = editor.display_snapshot(cx);
                let header_height = display_snapshot.buffer_header_height();

                // Since we're guaranteed that the file we're
                // dealing with is the first one for which excerpts
                // are added, we can constrain our search range to
                // simply
                // `DisplayRow(0)..DisplayRow(header_height)`, as
                // the block should always be found within that.
                let header = display_snapshot
                    .blocks_in_range(DisplayRow(0)..DisplayRow(header_height))
                    .find_map(|(row, block)| match block {
                        Block::BufferHeader { excerpt, .. } if excerpt.buffer_id() == buffer_id => {
                            Some((row, excerpt.start_anchor))
                        }
                        _ => None,
                    });

                if let Some((header_row, anchor)) = header {
                    editor.change_selections(
                        SelectionEffects::no_scroll().nav_history(false),
                        window,
                        cx,
                        |selections| selections.select_ranges([anchor..anchor]),
                    );

                    // The excerpt's anchor points to its first text line, below the file header.
                    // Scrolling to that anchor alone would leave the header above the viewport.
                    //
                    // As such, we use the difference between the header and text display rows as an
                    // offset so the viewport starts at the header while following the text anchor
                    // as other files might be inserted before it.
                    let anchor_row = anchor.to_display_point(&display_snapshot).row().as_f64();
                    let offset = gpui::point(0.0, header_row.as_f64() - anchor_row);

                    editor.set_scroll_anchor(ScrollAnchor { offset, anchor }, window, cx);
                }
            });
        });
    }

    fn subscribe_to_editor(
        editor: &Entity<SplittableEditor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Subscription {
        cx.subscribe_in(
            editor,
            window,
            |this, _, event: &EditorEvent, window, cx| {
                if let EditorEvent::OpenExcerptsRequested {
                    selections_by_buffer,
                    split,
                } = event
                {
                    let secondary = window.modifiers().secondary(cx);
                    for (buffer_id, (ranges, line_offset_from_top)) in selections_by_buffer {
                        // Only clicks carry the clicked line's offset from the top, and only
                        // secondary-clicks open the project file. The split action holds the
                        // secondary modifier too, but has no offset.
                        let target = match (split, line_offset_from_top.is_some() && secondary) {
                            (false, _) => OpenFileTarget::FullFileDiff,
                            (true, true) => OpenFileTarget::Project,
                            (true, false) => OpenFileTarget::FullFileDiffInSplit,
                        };
                        this.open_file(*buffer_id, ranges, target, window, cx);
                    }
                }
            },
        )
    }

    /// Opens a changed file at `target`, with `ranges` of the file's content at this commit
    /// selected.
    fn open_file(
        &mut self,
        buffer_id: language::BufferId,
        ranges: &[Range<BufferOffset>],
        target: OpenFileTarget,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let multibuffer = self.multibuffer.read(cx);
        let (Some(buffer), Some(diff)) = (
            multibuffer.buffer(buffer_id),
            multibuffer.diff_for(buffer_id),
        ) else {
            return;
        };
        let buffer = buffer.read(cx);
        let Some(file) = buffer.file().cloned() else {
            return;
        };
        let selections = ranges
            .iter()
            .map(|range| buffer.offset_to_point(range.start.0)..buffer.offset_to_point(range.end.0))
            .collect::<Vec<_>>();
        if target == OpenFileTarget::Project {
            let point = selections.first().map(|range| range.start);
            self.open_file_at_head(&file, point, window, cx);
            return;
        }

        // Rebuild the file from the loaded buffer and diff instead of loading the whole
        // commit again.
        let is_deleted = matches!(file.disk_state(), DiskState::Historic { was_deleted: true });
        let commit_file = CommitFile {
            path: RepoPath::from_rel_path(file.path()),
            old_text: diff.read(cx).base_text_string(cx),
            new_text: (!is_deleted).then(|| buffer.text()),
            is_binary: !file.can_open(),
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let commit = self.commit.clone();
        let repository = self.repository.clone();
        let project = self.project.clone();
        let options = CommitViewOptions {
            stash: self.stash,
            file_filter: Some(commit_file.path.clone()),
            show_full_file: true,
            selections,
            ..Default::default()
        };
        let origin = cx.entity();
        // Adding an item to the pane deactivates this view, which must not be updating then.
        window.defer(cx, move |window, cx| {
            workspace.update(cx, |workspace, cx| {
                let workspace_entity = cx.entity();
                let commit_view = cx.new(|cx| {
                    CommitView::new(
                        commit,
                        CommitDiff {
                            files: vec![commit_file],
                            is_shallow_boundary: false,
                        },
                        repository,
                        project,
                        workspace_entity,
                        options,
                        window,
                        cx,
                    )
                });
                let pane = workspace
                    .pane_for(&origin)
                    .unwrap_or_else(|| workspace.active_pane().clone());
                let pane = if target == OpenFileTarget::FullFileDiffInSplit {
                    workspace.adjacent_pane_of(&pane, window, cx)
                } else {
                    pane
                };
                Self::add_to_pane(&pane, commit_view, window, cx);
            });
        });
    }

    /// Adds the view to the pane, in place of an existing view of the same commit and files.
    fn add_to_pane(
        pane: &Entity<Pane>,
        commit_view: Entity<CommitView>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let new_view = commit_view.read(cx);
        let sha = new_view.commit.sha.clone();
        let show_full_file = new_view.show_full_file;
        let file_filter = new_view.file_filter.clone();
        // A full-file view lives next to the commit view it was opened from.
        let is_same_view = |view: &CommitView| {
            view.commit.sha == sha
                && view.show_full_file == show_full_file
                && (!show_full_file || view.file_filter == file_filter)
        };
        pane.update(cx, |pane, cx| {
            let existing = pane.items().enumerate().find_map(|(ix, item)| {
                item.downcast::<CommitView>()
                    .filter(|view| is_same_view(view.read(cx)))
                    .map(|view| (ix, view))
            });
            let ix = existing.map(|(ix, existing)| {
                pane.remove_item(existing.item_id(), false, false, window, cx);
                ix
            });
            pane.add_item(Box::new(commit_view), true, true, ix, window, cx);
        });
    }

    fn render_shallow_boundary_notice(&self, cx: &App) -> impl IntoElement {
        let commit_sha = self.commit.sha.to_string();
        let repository = self.repository.clone();
        let workspace = self.workspace.clone();
        let stash = self.stash;
        let file_filter = self.file_filter.clone();
        let reveal_file = self.reveal_file.clone();
        let show_full_file = self.show_full_file;
        let unshallow_state = self.repository.read(cx).unshallow_state();
        let can_fetch = !self.project.read(cx).is_via_collab()
            && unshallow_state != UnshallowState::Unshallowed;
        let fetch_in_flight = unshallow_state == UnshallowState::InProgress;
        v_flex()
            .flex_grow(1.)
            .items_center()
            .justify_center()
            .gap_2()
            .child(
                Label::new("This commit is at the boundary of a shallow clone.")
                    .color(Color::Muted),
            )
            .child(
                Label::new(
                    "Its parent history was not fetched, so the changes it introduced cannot be shown.",
                )
                .color(Color::Muted),
            )
            .child(
                h_flex()
                    .gap_2()
                    .when(can_fetch, |this| {
                        let commit_sha = commit_sha.clone();
                        let repository = repository.clone();
                        let workspace = workspace.clone();
                        let file_filter = file_filter.clone();
                        let reveal_file = reveal_file.clone();
                        this.child(
                            Button::new(
                                "fetch-unshallow",
                                if fetch_in_flight {
                                    "Fetching…"
                                } else {
                                    "Fetch Missing History"
                                },
                            )
                                .style(ButtonStyle::Filled)
                                .disabled(fetch_in_flight)
                                .tooltip(Tooltip::text(
                                    "Run `git fetch --unshallow` to download the full history, then show this commit's changes.",
                                ))
                                .on_click(move |_, window, cx| {
                                    let fetch = crate::commit_tooltip::fetch_unshallow(
                                        repository.clone(),
                                        workspace.clone(),
                                        window,
                                        cx,
                                    );
                                    let commit_sha = commit_sha.clone();
                                    let repository = repository.downgrade();
                                    let workspace = workspace.clone();
                                    let file_filter = file_filter.clone();
                                    let reveal_file = reveal_file.clone();
                                    window
                                        .spawn(cx, async move |cx| {
                                            fetch.await?;
                                            cx.update(|window, cx| {
                                                Self::open_with_options(
                                                    commit_sha,
                                                    repository,
                                                    workspace,
                                                    CommitViewOptions {
                                                        stash,
                                                        file_filter,
                                                        reveal_file,
                                                        show_full_file,
                                                        selections: Vec::new(),
                                                    },
                                                    false,
                                                    window,
                                                    cx,
                                                )
                                            })
                                        })
                                        .detach_and_log_err(cx);
                                }),
                        )
                    })
                    .child(
                        Button::new(
                            "load-shallow-snapshot",
                            if file_filter.is_some() {
                                "Load File Snapshot"
                            } else {
                                "Load Full Snapshot"
                            },
                        )
                            .style(ButtonStyle::Outlined)
                            .tooltip(Tooltip::text(if file_filter.is_some() {
                                "Show this file's full contents at this commit as added."
                            } else {
                                "Show every file at this commit as added. This can be slow in large repositories."
                            }))
                            .on_click(move |_, window, cx| {
                                Self::open_with_options(
                                    commit_sha.clone(),
                                    repository.downgrade(),
                                    workspace.clone(),
                                    CommitViewOptions {
                                        stash,
                                        file_filter: file_filter.clone(),
                                        reveal_file: reveal_file.clone(),
                                        show_full_file,
                                        selections: Vec::new(),
                                    },
                                    true,
                                    window,
                                    cx,
                                );
                            }),
                    ),
            )
    }

    fn render_commit_avatar(
        &self,
        sha: &SharedString,
        size: impl Into<gpui::AbsoluteLength>,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        CommitAvatar::new(
            sha,
            Some(self.commit.author_email.clone()),
            self.remote.as_ref(),
        )
        .size(size)
        .render(window, cx)
    }

    fn calculate_changed_lines(&self, cx: &App) -> (u32, u32) {
        self.multibuffer.read(cx).snapshot(cx).total_changed_lines()
    }

    /// Opens the file in the project, with the cursor at `point` clipped to the file's current content.
    fn open_file_at_head(
        &mut self,
        file: &Arc<dyn language::File>,
        point: Option<language::Point>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let rel_path = file.path().clone();
        let worktree_id = file.worktree_id(cx);
        let repo_path = RepoPath::from_rel_path(&rel_path);
        let project_path = self
            .repository
            .read(cx)
            .repo_path_to_project_path(&repo_path, cx)
            .unwrap_or(project::ProjectPath {
                worktree_id,
                path: rel_path,
            });

        let commit_view = cx.entity();
        self.workspace
            .update(cx, |workspace, cx| {
                let pane = workspace
                    .pane_for(&commit_view)
                    .map(|pane| pane.downgrade());
                let open_item =
                    workspace.open_path_preview(project_path, pane, true, false, true, window, cx);
                window
                    .spawn(cx, async move |cx| {
                        let item = open_item.await?;
                        if let Some(point) = point
                            && let Some(editor) = item.downcast::<Editor>()
                        {
                            editor.update_in(cx, |editor, window, cx| {
                                let point = editor
                                    .buffer()
                                    .read(cx)
                                    .snapshot(cx)
                                    .clip_point(point, language::Bias::Left);
                                editor.go_to_singleton_buffer_point(point, window, cx);
                            })?;
                        }
                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
            })
            .log_err();
    }

    fn open_file_at_head_action(
        &mut self,
        _: &OpenFileAtHead,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(file) = self
            .editor
            .read(cx)
            .focused_editor()
            .read(cx)
            .active_buffer(cx)
            .and_then(|buffer| buffer.read(cx).file().cloned())
        else {
            return;
        };
        self.open_file_at_head(&file, None, window, cx);
    }

    fn render_header(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let commit = &self.commit;
        let author_name = commit.author_name.clone();
        let author_email = commit.author_email.clone();
        let commit_sha = commit.sha.clone();
        let commit_date = time::OffsetDateTime::from_unix_timestamp(commit.commit_timestamp)
            .unwrap_or_else(|_| time::OffsetDateTime::now_utc());
        let date_string = crate::git_timestamp::format_git_timestamp(
            commit_date,
            time_format::TimestampFormat::MediumAbsolute,
            cx,
        );

        let avatar_size = rems_from_px(40_f32);
        let avatar_size_px = avatar_size.to_pixels(window.rem_size());
        let gutter_width = self.editor.update(cx, |editor, cx| {
            let editor = editor.rhs_editor().clone();
            editor.update(cx, |editor, cx| {
                let snapshot = editor.snapshot(window, cx);
                let style = editor.style(cx);
                let font_id = window.text_system().resolve_font(&style.text.font());
                let font_size = style.text.font_size.to_pixels(window.rem_size());
                snapshot
                    .gutter_dimensions(font_id, font_size, style, window, cx)
                    .full_width()
            })
        });
        let avatar_min_side_padding = rems_from_px(6_f32).to_pixels(window.rem_size());
        let avatar_container_min = avatar_size_px + avatar_min_side_padding;
        let avatar_container_width = gutter_width.max(avatar_container_min);

        let clipboard_has_sha = cx
            .read_from_clipboard()
            .and_then(|entry| entry.text())
            .map_or(false, |clipboard_text| {
                clipboard_text.trim() == commit_sha.as_ref()
            });

        let (copy_icon, copy_icon_color) = if clipboard_has_sha {
            (IconName::Check, Color::Success)
        } else {
            (IconName::Copy, Color::Muted)
        };

        let has_more = self.commit.message.trim().contains('\n');
        let is_expanded = self.message_expanded;
        let expand_tooltip = if is_expanded {
            "Fold Commit Description"
        } else {
            "Expand Commit Description"
        };

        v_flex()
            .w_full()
            .py_2p5()
            .gap_2()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                h_flex()
                    .pr_2p5()
                    .w_full()
                    .flex_wrap()
                    .justify_between()
                    .child(
                        h_flex()
                            .child(
                                h_flex()
                                    .flex_none()
                                    .w(avatar_container_width)
                                    .justify_center()
                                    .child(self.render_commit_avatar(
                                        &commit.sha,
                                        avatar_size,
                                        window,
                                        cx,
                                    )),
                            )
                            .child(
                                v_flex()
                                    .child(h_flex().gap_1().child(Label::new(author_name)).when(
                                        has_more,
                                        |this| {
                                            this.child(
                                                Disclosure::new(
                                                    "commit-message-disclosure",
                                                    is_expanded,
                                                )
                                                .closed_icon(IconName::ExpandVertical)
                                                .opened_icon(IconName::FoldVertical)
                                                .tooltip(Tooltip::text(expand_tooltip))
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.message_expanded = !this.message_expanded;
                                                    cx.notify();
                                                })),
                                            )
                                        },
                                    ))
                                    .child(
                                        h_flex()
                                            .gap_1p5()
                                            .child(
                                                Label::new(date_string)
                                                    .color(Color::Muted)
                                                    .size(LabelSize::Small),
                                            )
                                            .child(
                                                Label::new("•")
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted)
                                                    .alpha(0.5),
                                            )
                                            .child(
                                                Label::new(author_email)
                                                    .color(Color::Muted)
                                                    .size(LabelSize::Small),
                                            ),
                                    ),
                            ),
                    )
                    .when(self.stash.is_none(), |this| {
                        this.child(
                            Button::new("sha", "Commit SHA")
                                .start_icon(
                                    Icon::new(copy_icon)
                                        .size(IconSize::Small)
                                        .color(copy_icon_color),
                                )
                                .tooltip({
                                    let commit_sha = commit_sha.clone();
                                    move |_, cx| {
                                        Tooltip::with_meta(
                                            "Copy Commit SHA",
                                            None,
                                            commit_sha.clone(),
                                            cx,
                                        )
                                    }
                                })
                                .on_click(move |_, _, cx| {
                                    cx.stop_propagation();
                                    cx.write_to_clipboard(ClipboardItem::new_string(
                                        commit_sha.to_string(),
                                    ));
                                }),
                        )
                    }),
            )
            .children(self.render_commit_message(avatar_container_width, window, cx))
    }

    fn render_commit_message(
        &self,
        avatar_spacer: Pixels,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let message = self.commit.message.trim();
        if message.is_empty() {
            return None;
        }

        let markdown_style = MarkdownStyle {
            soft_break_as_hard_break: true,
            ..hover_markdown_style(window, cx)
        };

        let is_expanded = self.message_expanded;

        let has_more = message.contains('\n');
        let collapsed = has_more && !is_expanded;
        let collapsed_height = window.line_height();
        let max_expanded_height = window.line_height() * 12.;

        Some(
            h_flex()
                .w_full()
                .pr_2p5()
                .child(h_flex().flex_none().w(avatar_spacer))
                .child(
                    div()
                        .relative()
                        .flex_1()
                        .min_w_0()
                        .child(
                            div()
                                .id("commit-message")
                                .size_full()
                                .text_sm()
                                .when(collapsed, |this| this.h(collapsed_height).overflow_hidden())
                                .when(!collapsed, |this| {
                                    this.max_h(max_expanded_height)
                                        .overflow_y_scroll()
                                        .track_scroll(&self.message_scroll_handle)
                                })
                                .child(MarkdownElement::new(self.message.clone(), markdown_style)),
                        )
                        .vertical_scrollbar_for(&self.message_scroll_handle, window, cx),
                ),
        )
    }

    fn apply_stash(workspace: &mut Workspace, window: &mut Window, cx: &mut App) {
        Self::stash_action(
            workspace,
            "Apply",
            window,
            cx,
            async move |repository, sha, stash, commit_view, workspace, cx| {
                let result = repository.update(cx, |repo, cx| {
                    if !stash_matches_index(&sha, stash, repo) {
                        return Err(anyhow::anyhow!("Stash has changed, not applying"));
                    }
                    Ok(repo.stash_apply(Some(stash), cx))
                });

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

    fn pop_stash(workspace: &mut Workspace, window: &mut Window, cx: &mut App) {
        Self::stash_action(
            workspace,
            "Pop",
            window,
            cx,
            async move |repository, sha, stash, commit_view, workspace, cx| {
                let result = repository.update(cx, |repo, cx| {
                    if !stash_matches_index(&sha, stash, repo) {
                        return Err(anyhow::anyhow!("Stash has changed, pop aborted"));
                    }
                    Ok(repo.stash_pop(Some(stash), cx))
                });

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

    fn remove_stash(workspace: &mut Workspace, window: &mut Window, cx: &mut App) {
        Self::stash_action(
            workspace,
            "Drop",
            window,
            cx,
            async move |repository, sha, stash, commit_view, workspace, cx| {
                let result = repository.update(cx, |repo, cx| {
                    if !stash_matches_index(&sha, stash, repo) {
                        return Err(anyhow::anyhow!("Stash has changed, drop aborted"));
                    }
                    Ok(repo.stash_drop(Some(stash), cx))
                });

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
        workspace: &mut Workspace,
        str_action: &str,
        window: &mut Window,
        cx: &mut App,
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
        let Some(commit_view) = workspace.active_item_as::<CommitView>(cx) else {
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

        let workspace_weak = workspace.weak_handle();
        let commit_view_entity = commit_view;

        window
            .spawn(cx, async move |cx| {
                if answer.await != Ok(0) {
                    return anyhow::Ok(());
                }

                let Some(workspace) = workspace_weak.upgrade() else {
                    return Ok(());
                };

                let repo = workspace.update(cx, |workspace, cx| {
                    workspace
                        .panel::<GitPanel>(cx)
                        .and_then(|p| p.read(cx).active_repository.clone())
                });

                let Some(repo) = repo else {
                    return Ok(());
                };

                callback(repo, &sha, stash, commit_view_entity, workspace_weak, cx).await?;
                anyhow::Ok(())
            })
            .detach_and_notify_err(workspace.weak_handle(), window, cx);
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
}

impl language::File for GitBlob {
    fn as_local(&self) -> Option<&dyn language::LocalFile> {
        None
    }

    fn disk_state(&self) -> DiskState {
        DiskState::Historic {
            was_deleted: self.is_deleted,
        }
    }

    fn path_style(&self, _: &App) -> PathStyle {
        PathStyle::local()
    }

    fn path(&self) -> &Arc<RelPath> {
        self.path.as_ref()
    }

    fn full_path(&self, _: &App) -> PathBuf {
        self.path.as_std_path().to_path_buf()
    }

    fn file_name<'a>(&'a self, _: &'a App) -> &'a str {
        self.display_name.as_ref()
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

    fn can_open(&self) -> bool {
        !self.is_binary
    }
}

pub(crate) async fn build_buffer(
    mut text: String,
    blob: Arc<dyn File>,
    language_registry: &Arc<language::LanguageRegistry>,
    cx: &mut AsyncWindowContext,
) -> Result<Entity<Buffer>> {
    let line_ending = LineEnding::detect(&text);
    LineEnding::normalize(&mut text);
    let text = Rope::from(text);
    let language =
        cx.update(|_, cx| language_registry.language_for_file(&blob, Some(&text), cx))?;
    let language = if let Some(language_id) = language {
        language_registry
            .load_language(language_id)
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
        let mut buffer = Buffer::build(buffer, Some(blob), Capability::ReadWrite, cx);
        buffer.set_language_registry(language_registry.clone());
        buffer.set_language_async(language, cx);
        buffer
    });
    Ok(buffer)
}

async fn build_buffer_diff(
    mut old_text: Option<String>,
    buffer: &Entity<Buffer>,
    language_registry: &Arc<LanguageRegistry>,
    cx: &mut AsyncWindowContext,
) -> Result<Entity<BufferDiff>> {
    if let Some(old_text) = &mut old_text {
        LineEnding::normalize(old_text);
    }

    let language = cx.update(|_, cx| buffer.read(cx).language().cloned())?;
    let buffer = cx.update(|_, cx| buffer.read(cx).snapshot())?;

    let diff =
        cx.new(|cx| BufferDiff::new(&buffer.text, language, Some(language_registry.clone()), cx));

    diff.update(cx, |diff, cx| {
        diff.set_base_text(
            old_text.map(|old_text| Arc::from(old_text.as_str())),
            buffer.text.clone(),
            cx,
        )
    })
    .await;

    Ok(diff)
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
        Some(Icon::new(IconName::GitCommit).color(Color::Muted))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        let short_sha = self.commit.sha.get(0..7).unwrap_or(&*self.commit.sha);
        if self.show_full_file
            && let Some(file_name) = self.file_filter.as_ref().and_then(|path| path.file_name())
        {
            return format!("{short_sha} — {file_name}").into();
        }
        let subject = truncate_and_trailoff(self.commit.message.split('\n').next().unwrap(), 20);
        format!("{short_sha} — {subject}").into()
    }

    fn tab_tooltip_content(&self, _: &App) -> Option<TabTooltipContent> {
        let short_sha = self.commit.sha.get(0..16).unwrap_or(&*self.commit.sha);
        let subject = self.commit.message.split('\n').next().unwrap();

        Some(TabTooltipContent::Custom(Box::new(Tooltip::element({
            let subject = subject.to_string();
            let short_sha = short_sha.to_string();

            move |_, _| {
                v_flex()
                    .child(Label::new(subject.clone()))
                    .child(
                        Label::new(short_sha.clone())
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
                    .into_any_element()
            }
        }))))
    }

    fn to_item_events(event: &EditorEvent, f: &mut dyn FnMut(ItemEvent)) {
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
        cx: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<SplittableEditor>() {
            Some(self.editor.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.read(cx).rhs_editor().clone().into())
        } else {
            None
        }
    }

    fn as_searchable(&self, _: &Entity<Self>, _: &App) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.editor.read(cx).for_each_project_item(cx, f)
    }

    fn active_project_path(&self, cx: &App) -> Option<ProjectPath> {
        self.editor.read(cx).active_project_path(cx)
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |editor, _| {
                editor.set_nav_history(Some(nav_history));
            });
        });
    }

    fn navigate(
        &mut self,
        data: Arc<dyn Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
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
        let file_statuses = self
            .editor
            .read(cx)
            .rhs_editor()
            .read(cx)
            .addon::<CommitDiffAddon>()
            .map(|addon| addon.file_statuses.clone())
            .unwrap_or_default();
        let Some(workspace_entity) = self.workspace.upgrade() else {
            return Task::ready(None);
        };
        let project = self.project.clone();
        let diff_view_style = self.editor.read(cx).diff_view_style();
        let multibuffer = self.multibuffer.clone();
        Task::ready(Some(cx.new(|cx| {
            let commit_view = cx.weak_entity();
            let editor = cx.new({
                let file_statuses = file_statuses.clone();
                let project = project.clone();
                let workspace_entity = workspace_entity.clone();
                let multibuffer = multibuffer.clone();
                let window = &mut *window;
                move |cx| {
                    let editor = SplittableEditor::new(
                        diff_view_style,
                        multibuffer.clone(),
                        project.clone(),
                        workspace_entity.clone(),
                        window,
                        cx,
                    );
                    editor.set_diff_hunk_renderer(Some(Arc::new(HiddenDiffHunkRenderer)), cx);
                    editor.rhs_editor().update(cx, |editor, cx| {
                        editor.set_show_bookmarks(false, cx);
                        editor.set_show_breakpoints(false, cx);
                        editor.set_show_diff_review_button(true, cx);
                        editor.set_delegate_open_excerpts(true);
                        editor.register_addon(CommitDiffAddon {
                            file_statuses,
                            commit_view,
                        });
                    });
                    editor
                }
            });
            let editor_subscription = Self::subscribe_to_editor(&editor, window, cx);
            let language_registry = project.read(cx).languages().clone();
            let message = cx.new(|cx| {
                Markdown::new(
                    self.commit.message.clone(),
                    Some(language_registry),
                    None,
                    cx,
                )
            });
            Self {
                editor,
                message,
                message_expanded: self.message_expanded,
                message_scroll_handle: ScrollHandle::new(),
                multibuffer: self.multibuffer.clone(),
                commit: self.commit.clone(),
                stash: self.stash,
                repository: self.repository.clone(),
                project: self.project.clone(),
                workspace: self.workspace.clone(),
                remote: self.remote.clone(),
                is_shallow_boundary: self.is_shallow_boundary,
                file_filter: self.file_filter.clone(),
                reveal_file: self.reveal_file.clone(),
                show_full_file: self.show_full_file,
                _editor_subscription: editor_subscription,
                _load_diff_task: Task::ready(Ok(())),
            }
        })))
    }
}

impl Render for CommitView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_stash = self.stash.is_some();

        v_flex()
            .key_context(if is_stash { "StashDiff" } else { "CommitDiff" })
            .on_action(cx.listener(Self::open_file_at_head_action))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(self.render_header(window, cx))
            .when(
                !self.editor.read(cx).rhs_editor().read(cx).is_empty(cx),
                |this| this.child(div().flex_grow(1.).child(self.editor.clone())),
            )
            .when(self.is_shallow_boundary, |this| {
                this.child(self.render_shallow_boundary_notice(cx))
            })
    }
}

pub struct CommitViewToolbar {
    commit_view: Option<WeakEntity<CommitView>>,
}

impl CommitViewToolbar {
    pub fn new() -> Self {
        Self { commit_view: None }
    }
}

impl EventEmitter<ToolbarItemEvent> for CommitViewToolbar {}

impl Render for CommitViewToolbar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(commit_view) = self.commit_view.as_ref().and_then(|w| w.upgrade()) else {
            return div();
        };

        let commit_view_ref = commit_view.read(cx);
        let is_stash = commit_view_ref.stash.is_some();

        let (additions, deletions) = commit_view_ref.calculate_changed_lines(cx);

        let commit_sha = commit_view_ref.commit.sha.clone();

        let remote_info = commit_view_ref.remote.as_ref().map(|remote| {
            let provider = remote.host.name();
            let parsed_remote = ParsedGitRemote {
                owner: remote.owner.as_ref().into(),
                repo: remote.repo.as_ref().into(),
            };
            let params = BuildCommitPermalinkParams { sha: &commit_sha };
            let url = remote
                .host
                .build_commit_permalink(&parsed_remote, params)
                .to_string();
            (provider, url)
        });

        let sha_for_graph = commit_sha.to_string();

        h_flex()
            .gap_1()
            .when(additions > 0 || deletions > 0, |this| {
                this.child(
                    h_flex()
                        .gap_2()
                        .child(DiffStat::new(
                            "toolbar-diff-stat",
                            additions as usize,
                            deletions as usize,
                        ))
                        .child(Divider::vertical()),
                )
            })
            .child(
                IconButton::new("buffer-search", IconName::MagnifyingGlass)
                    .icon_size(IconSize::Small)
                    .tooltip(move |_, cx| {
                        Tooltip::for_action(
                            "Buffer Search",
                            &zed_actions::buffer_search::Deploy::find(),
                            cx,
                        )
                    })
                    .on_click(|_, window, cx| {
                        window.dispatch_action(
                            Box::new(zed_actions::buffer_search::Deploy::find()),
                            cx,
                        );
                    }),
            )
            .when(!is_stash, |this| {
                this.child(
                    IconButton::new("show-in-git-graph", IconName::GitGraph)
                        .icon_size(IconSize::Small)
                        .tooltip(Tooltip::text("Show in Git Graph"))
                        .on_click(move |_, window, cx| {
                            window.dispatch_action(
                                Box::new(crate::git_graph::OpenAtCommit {
                                    sha: sha_for_graph.clone(),
                                }),
                                cx,
                            );
                        }),
                )
                .children(remote_info.map(|(provider_name, url)| {
                    let icon = ui::git_hosting_provider_icon(provider_name.as_str());

                    IconButton::new("view_on_provider", icon)
                        .icon_size(IconSize::Small)
                        .tooltip(Tooltip::text(format!("View on {}", provider_name)))
                        .on_click(move |_, _, cx| cx.open_url(&url))
                }))
            })
    }
}

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
        self.commit_view = None;
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

fn stash_matches_index(sha: &str, stash_index: usize, repo: &Repository) -> bool {
    repo.stash_entries
        .entries
        .get(stash_index)
        .map(|entry| entry.oid.to_string() == sha)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{EmptyView, TestAppContext, UpdateGlobal, VisualTestContext};
    use indoc::indoc;
    use language::{Language, LanguageConfig, LanguageMatcher, Point, markdown_lang};
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::Path;
    use util::path;
    use workspace::MultiWorkspace;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            language_model::init(cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    /// Opens a commit view for a commit that modifies `a.txt` and `b.txt`.
    async fn open_test_commit_view(
        file_filter: Option<RepoPath>,
        cx: &mut TestAppContext,
    ) -> (Entity<CommitView>, &mut VisualTestContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({ ".git": {}, "a.txt": "a new\n", "b.txt": "b new\n" }),
        )
        .await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;
        cx.run_until_parked();
        let repository = project.read_with(cx, |project, cx| {
            project
                .active_repository(cx)
                .expect("project should have a repository")
        });

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(&*cx, |multi, _| multi.workspace().clone());

        let commit = CommitDetails {
            sha: "0123456789abcdef0123456789abcdef01234567".into(),
            message: "Subject\n\nA longer description.".into(),
            ..Default::default()
        };
        let commit_diff = CommitDiff {
            files: ["a.txt", "b.txt"]
                .map(|path| CommitFile {
                    path: RepoPath::new(path).expect("valid repository path"),
                    old_text: Some(format!("{path} old\n")),
                    new_text: Some(format!("{path} new\n")),
                    is_binary: false,
                })
                .into(),
            is_shallow_boundary: false,
        };
        let commit_view = workspace.update_in(cx, |workspace, window, cx| {
            let workspace_entity = cx.entity();
            let commit_view = cx.new(|cx| {
                CommitView::new(
                    commit,
                    commit_diff,
                    repository,
                    project.clone(),
                    workspace_entity,
                    CommitViewOptions {
                        file_filter,
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            });
            workspace.add_item_to_active_pane(
                Box::new(commit_view.clone()),
                None,
                true,
                window,
                cx,
            );
            commit_view
        });
        cx.run_until_parked();
        (commit_view, cx)
    }

    fn folded_files(commit_view: &CommitView, cx: &App) -> usize {
        let editor = commit_view.editor.read(cx).rhs_editor().read(cx);
        commit_view
            .multibuffer
            .read(cx)
            .all_buffers()
            .iter()
            .filter(|buffer| editor.is_buffer_folded(buffer.read(cx).remote_id(), cx))
            .count()
    }

    #[gpui::test]
    async fn test_commit_view_opens_with_defaults(cx: &mut TestAppContext) {
        init_test(cx);
        let (commit_view, cx) = open_test_commit_view(None, cx).await;

        commit_view.read_with(cx, |commit_view, cx| {
            assert!(!commit_view.message_expanded);
            assert_eq!(folded_files(commit_view, cx), 0);
        });
    }

    #[gpui::test]
    async fn test_commit_view_settings_expand_description_and_collapse_files(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    let commit_view = settings
                        .git
                        .get_or_insert_default()
                        .commit_view
                        .get_or_insert_default();
                    commit_view.expand_description = Some(true);
                    commit_view.expand_files = Some(false);
                })
            });
        });
        let (commit_view, cx) = open_test_commit_view(None, cx).await;

        commit_view.read_with(cx, |commit_view, cx| {
            assert!(commit_view.message_expanded);
            assert_eq!(folded_files(commit_view, cx), 2);
        });
    }

    #[gpui::test]
    async fn test_commit_view_filtered_to_file_ignores_collapse_files(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .git
                        .get_or_insert_default()
                        .commit_view
                        .get_or_insert_default()
                        .expand_files = Some(false);
                })
            });
        });
        let file_filter = RepoPath::new("a.txt").expect("valid repository path");
        let (commit_view, cx) = open_test_commit_view(Some(file_filter), cx).await;

        commit_view.read_with(cx, |commit_view, cx| {
            assert_eq!(folded_files(commit_view, cx), 0);
        });
    }

    const TEST_SHA: &str = "0123456789abcdef0123456789abcdef01234567";

    /// The content of `a.txt` before and after the test commit: 30 lines with line 15 changed.
    fn test_file_text(changed_line: &str) -> String {
        (1..=30)
            .map(|row| match row {
                1 => "first\n".to_string(),
                15 => format!("{changed_line}\n"),
                30 => "last\n".to_string(),
                _ => format!("line {row}\n"),
            })
            .collect()
    }

    /// Opens a commit view for a commit that changes line 15 of `a.txt`.
    async fn open_line_change_commit_view<'a>(
        show_full_file: bool,
        project_text: &str,
        cx: &'a mut TestAppContext,
    ) -> (
        Entity<CommitView>,
        Entity<Workspace>,
        &'a mut VisualTestContext,
    ) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({ ".git": {}, "a.txt": project_text }),
        )
        .await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;
        cx.run_until_parked();
        let repository = project.read_with(cx, |project, cx| {
            project
                .active_repository(cx)
                .expect("project should have a repository")
        });

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(&*cx, |multi, _| multi.workspace().clone());

        let commit = CommitDetails {
            sha: TEST_SHA.into(),
            message: "Change a line".into(),
            ..Default::default()
        };
        let commit_diff = CommitDiff {
            files: vec![CommitFile {
                path: RepoPath::new("a.txt").expect("valid repository path"),
                old_text: Some(test_file_text("original")),
                new_text: Some(test_file_text("changed")),
                is_binary: false,
            }],
            is_shallow_boundary: false,
        };
        let commit_view = workspace.update_in(cx, |workspace, window, cx| {
            let workspace_entity = cx.entity();
            let options = CommitViewOptions {
                file_filter: show_full_file.then(|| RepoPath::new("a.txt").unwrap()),
                show_full_file,
                ..Default::default()
            };
            let commit_view = cx.new(|cx| {
                CommitView::new(
                    commit,
                    commit_diff,
                    repository,
                    project.clone(),
                    workspace_entity,
                    options,
                    window,
                    cx,
                )
            });
            workspace.add_item_to_active_pane(
                Box::new(commit_view.clone()),
                None,
                true,
                window,
                cx,
            );
            commit_view
        });
        cx.run_until_parked();
        (commit_view, workspace, cx)
    }

    fn commit_buffer_id(
        commit_view: &Entity<CommitView>,
        cx: &VisualTestContext,
    ) -> language::BufferId {
        commit_view.read_with(cx, |commit_view, cx| {
            let buffers = commit_view.multibuffer.read(cx).all_buffers();
            assert_eq!(buffers.len(), 1);
            buffers.iter().next().unwrap().read(cx).remote_id()
        })
    }

    #[gpui::test]
    async fn test_commit_view_shows_hunks_only(cx: &mut TestAppContext) {
        init_test(cx);
        let (commit_view, _, cx) =
            open_line_change_commit_view(false, &test_file_text("changed"), cx).await;

        let text =
            commit_view.read_with(cx, |view, cx| view.multibuffer.read(cx).snapshot(cx).text());
        assert!(text.contains("changed"));
        assert!(!text.contains("first"));
        assert!(!text.contains("last"));
    }

    #[gpui::test]
    async fn test_commit_view_show_full_file(cx: &mut TestAppContext) {
        init_test(cx);
        let (commit_view, _, cx) =
            open_line_change_commit_view(true, &test_file_text("changed"), cx).await;

        let text =
            commit_view.read_with(cx, |view, cx| view.multibuffer.read(cx).snapshot(cx).text());
        assert!(text.contains("first"));
        assert!(text.contains("changed"));
        assert!(text.contains("last"));
    }

    #[gpui::test]
    async fn test_open_file_opens_full_file_view_next_to_commit_view(cx: &mut TestAppContext) {
        init_test(cx);
        let (commit_view, workspace, cx) =
            open_line_change_commit_view(false, &test_file_text("changed"), cx).await;
        let buffer_id = commit_buffer_id(&commit_view, cx);

        commit_view.update_in(cx, |commit_view, window, cx| {
            commit_view.open_file(buffer_id, &[], OpenFileTarget::FullFileDiff, window, cx);
        });
        cx.run_until_parked();

        workspace.read_with(cx, |workspace, cx| {
            let pane = workspace.active_pane().read(cx);
            let commit_views = pane
                .items()
                .filter_map(|item| item.downcast::<CommitView>())
                .collect::<Vec<_>>();
            assert_eq!(
                commit_views.len(),
                2,
                "the full-file view should open next to the commit view"
            );
            assert!(commit_views.contains(&commit_view));
            let full_file_view = pane
                .active_item()
                .and_then(|item| item.downcast::<CommitView>())
                .expect("the full-file view should be active");
            let full_file_view = full_file_view.read(cx);
            assert!(full_file_view.show_full_file);
            assert_eq!(
                full_file_view.file_filter,
                Some(RepoPath::new("a.txt").unwrap())
            );
            assert_eq!(full_file_view.tab_content_text(0, cx), "0123456 — a.txt");
            let text = full_file_view.multibuffer.read(cx).snapshot(cx).text();
            assert!(text.contains("first"));
            assert!(text.contains("changed"));
            assert!(text.contains("last"));
        });
    }

    /// Emits a split open request for `ranges` of `buffer_id` from the commit view's editor, as
    /// the split action does without `line_offset_from_top` and a click does with it.
    fn request_open_split(
        commit_view: &Entity<CommitView>,
        buffer_id: language::BufferId,
        ranges: Vec<Range<BufferOffset>>,
        line_offset_from_top: Option<u32>,
        cx: &mut VisualTestContext,
    ) {
        let rhs_editor = commit_view.read_with(cx, |commit_view, cx| {
            commit_view.editor.read(cx).rhs_editor().clone()
        });
        rhs_editor.update(cx, |_, cx| {
            cx.emit(EditorEvent::OpenExcerptsRequested {
                selections_by_buffer: HashMap::from_iter([(
                    buffer_id,
                    (ranges, line_offset_from_top),
                )]),
                split: true,
            });
        });
        cx.run_until_parked();
    }

    fn full_file_view_in_split_pane(workspace: &Workspace, cx: &App) -> Entity<CommitView> {
        assert_eq!(workspace.panes().len(), 2);
        let full_file_view = workspace.panes()[1]
            .read(cx)
            .active_item()
            .and_then(|item| item.downcast::<CommitView>())
            .expect("the full-file view should be active in the split pane");
        assert!(full_file_view.read(cx).show_full_file);
        full_file_view
    }

    #[gpui::test]
    async fn test_split_action_opens_full_file_view_in_split_pane(cx: &mut TestAppContext) {
        init_test(cx);
        let (commit_view, workspace, cx) =
            open_line_change_commit_view(false, &test_file_text("changed"), cx).await;
        let buffer_id = commit_buffer_id(&commit_view, cx);
        let start = BufferOffset(0)..BufferOffset(0);

        request_open_split(&commit_view, buffer_id, vec![start.clone()], None, cx);
        request_open_split(&commit_view, buffer_id, vec![start], None, cx);

        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(
                workspace.pane_for(&commit_view),
                Some(workspace.panes()[0].clone()),
                "the commit view should stay in its pane"
            );
            full_file_view_in_split_pane(workspace, cx);
            assert_eq!(
                workspace.panes()[1].read(cx).items_len(),
                1,
                "opening the file again should replace its full-file view"
            );
        });
    }

    #[gpui::test]
    async fn test_alt_click_opens_full_file_view_in_split_pane(cx: &mut TestAppContext) {
        init_test(cx);
        let (commit_view, workspace, cx) =
            open_line_change_commit_view(false, &test_file_text("changed"), cx).await;
        let buffer_id = commit_buffer_id(&commit_view, cx);

        request_open_split(
            &commit_view,
            buffer_id,
            vec![BufferOffset(0)..BufferOffset(0)],
            Some(0),
            cx,
        );

        workspace.read_with(cx, |workspace, cx| {
            full_file_view_in_split_pane(workspace, cx);
        });
    }

    #[gpui::test]
    async fn test_full_file_view_keeps_selections(cx: &mut TestAppContext) {
        init_test(cx);
        let (commit_view, workspace, cx) =
            open_line_change_commit_view(false, &test_file_text("changed"), cx).await;
        let buffer_id = commit_buffer_id(&commit_view, cx);
        let text = test_file_text("changed");
        let changed = text.find("changed").unwrap();
        let line = text.find("line 14").unwrap();

        request_open_split(
            &commit_view,
            buffer_id,
            vec![
                BufferOffset(line)..BufferOffset(line + "line 14".len()),
                BufferOffset(changed)..BufferOffset(changed),
            ],
            None,
            cx,
        );

        let full_file_view = workspace.read_with(cx, |workspace, cx| {
            full_file_view_in_split_pane(workspace, cx)
        });
        let rhs_editor = full_file_view.read_with(cx, |full_file_view, cx| {
            full_file_view.editor.read(cx).rhs_editor().clone()
        });
        rhs_editor.update(cx, |editor, cx| {
            let selections = editor
                .selections
                .all::<Point>(&editor.display_snapshot(cx))
                .into_iter()
                .map(|selection| selection.range())
                .collect::<Vec<_>>();
            assert_eq!(
                selections,
                vec![
                    Point::new(13, 0)..Point::new(13, 7),
                    Point::new(14, 0)..Point::new(14, 0),
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_secondary_click_opens_project_file(cx: &mut TestAppContext) {
        init_test(cx);
        let (commit_view, workspace, cx) =
            open_line_change_commit_view(false, &test_file_text("changed"), cx).await;
        let buffer_id = commit_buffer_id(&commit_view, cx);

        let secondary = cx.update(|_, cx| gpui::Modifiers::secondary_key(cx));
        cx.simulate_modifiers_change(secondary);
        request_open_split(
            &commit_view,
            buffer_id,
            vec![BufferOffset(0)..BufferOffset(0)],
            Some(0),
            cx,
        );

        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.panes().len(), 1);
            let editor = workspace
                .active_item(cx)
                .and_then(|item| item.downcast::<Editor>())
                .expect("the project file should be active");
            let file = editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .and_then(|buffer| buffer.read(cx).file().cloned())
                .expect("the editor should show a project file");
            assert!(file.disk_state().exists());
        });
    }

    #[gpui::test]
    async fn test_open_file_in_project_jumps_to_line(cx: &mut TestAppContext) {
        init_test(cx);
        let (commit_view, workspace, cx) =
            open_line_change_commit_view(false, &test_file_text("changed"), cx).await;
        let buffer_id = commit_buffer_id(&commit_view, cx);
        let offset = test_file_text("changed").find("changed").unwrap();

        commit_view.update_in(cx, |commit_view, window, cx| {
            commit_view.open_file(
                buffer_id,
                &[BufferOffset(offset)..BufferOffset(offset)],
                OpenFileTarget::Project,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        workspace.update_in(cx, |workspace, window, cx| {
            let pane = workspace.active_pane().read(cx);
            assert!(
                pane.items()
                    .any(|item| item.downcast::<CommitView>() == Some(commit_view.clone())),
                "the project file should open in the commit view's pane"
            );
            let editor = pane
                .active_item()
                .and_then(|item| item.downcast::<Editor>())
                .expect("the project file should be active");
            editor.update(cx, |editor, cx| {
                assert!(editor.is_focused(window));
                let file = editor
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .and_then(|buffer| buffer.read(cx).file().cloned())
                    .expect("the editor should show a project file");
                assert!(file.disk_state().exists());
                assert_eq!(file.file_name(cx), "a.txt");
                let cursor = editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx))
                    .head();
                assert_eq!(cursor, Point::new(14, 0));
            });
        });
    }

    #[gpui::test]
    async fn test_open_file_in_project_clips_to_current_content(cx: &mut TestAppContext) {
        init_test(cx);
        let (commit_view, workspace, cx) = open_line_change_commit_view(false, "short\n", cx).await;
        let buffer_id = commit_buffer_id(&commit_view, cx);
        let offset = test_file_text("changed").find("changed").unwrap();

        commit_view.update_in(cx, |commit_view, window, cx| {
            commit_view.open_file(
                buffer_id,
                &[BufferOffset(offset)..BufferOffset(offset)],
                OpenFileTarget::Project,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        workspace.update_in(cx, |workspace, _window, cx| {
            let editor = workspace
                .active_item(cx)
                .and_then(|item| item.downcast::<Editor>())
                .expect("the project file should be active");
            editor.update(cx, |editor, cx| {
                let cursor = editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx))
                    .head();
                assert_eq!(cursor, Point::new(1, 0));
            });
        });
    }

    #[gpui::test]
    async fn test_build_buffer_resolves_injected_languages(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
        });

        let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
        language_registry.add(markdown_lang());
        language_registry.add(Arc::new(markdown_inline_lang()));

        let window = cx.add_window(|_, _| EmptyView);
        let mut async_cx = window
            .update(cx, |_, window, cx| window.to_async(cx))
            .expect("window should be open");

        let text = indoc! {"
            # Title

            Some *emphasized* text.
        "}
        .to_string();
        let blob = Arc::new(GitBlob {
            path: RepoPath::new("notes.md").unwrap(),
            worktree_id: WorktreeId::from_usize(0),
            is_deleted: false,
            is_binary: false,
            display_name: "abc1234 - notes.md".into(),
        }) as Arc<dyn File>;

        let buffer = build_buffer(text, blob, &language_registry, &mut async_cx)
            .await
            .expect("buffer should build");

        cx.run_until_parked();

        buffer.read_with(cx, |buffer, _| {
            let language = buffer.language().expect("buffer should have a language");
            assert_eq!(language.name().as_ref(), "Markdown");

            let layers = buffer
                .snapshot()
                .syntax_layers()
                .map(|layer| layer.language.name().to_string())
                .collect::<Vec<_>>();
            assert!(
                layers.iter().any(|name| name == "Markdown-Inline"),
                "emphasis, links, and fenced code blocks are highlighted by the injected \
                 Markdown-Inline grammar, but the buffer parsed with layers {layers:?}",
            );
        });
    }

    #[gpui::test]
    fn test_git_blob_language_detection_ignores_display_name(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let mut store = SettingsStore::test(cx);
            store
                .set_user_settings(r#"{"file_types":{"JSONC":["tsconfig*.json"]}}"#, cx)
                .expect("valid file type settings");
            cx.set_global(store);
        });

        let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
        language_registry.register_test_language(LanguageConfig {
            name: "JSON".into(),
            matcher: Arc::new(LanguageMatcher {
                path_suffixes: vec!["json".into(), "flake.lock".into()],
                ..LanguageMatcher::default()
            }),
            ..LanguageConfig::default()
        });
        language_registry.register_test_language(LanguageConfig {
            name: "JSONC".into(),
            ..LanguageConfig::default()
        });

        let languages = cx.update(|cx| {
            [
                ("flake.lock", "abc1234 - flake.lock"),
                ("flake.lock", "flake.lock @ abc1234"),
                ("nix/flake.lock", "abc1234 - flake.lock"),
                ("nix/flake.lock", "flake.lock @ abc1234"),
                ("web/tsconfig.json", "abc1234 - tsconfig.json"),
                ("web/tsconfig.json", "tsconfig.json @ abc1234"),
            ]
            .map(|(path, display_name)| {
                let blob = Arc::new(GitBlob {
                    path: RepoPath::new(path).expect("valid repository path"),
                    worktree_id: WorktreeId::from_usize(0),
                    is_deleted: false,
                    is_binary: false,
                    display_name: display_name.to_string(),
                }) as Arc<dyn File>;
                assert_eq!(blob.file_system_abs_path(cx), None);
                language_registry
                    .language_for_file(&blob, None, cx)
                    .and_then(|id| language_registry.language_name_for_id(id))
                    .map(|name| name.to_string())
            })
        });

        assert_eq!(
            languages.each_ref().map(|name| name.as_deref()),
            [
                Some("JSON"),
                Some("JSON"),
                Some("JSON"),
                Some("JSON"),
                Some("JSONC"),
                Some("JSONC"),
            ]
        );
    }

    #[gpui::test]
    async fn test_reveal_file_preserves_header_position(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);

            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "a.txt": "",
                "b.txt": "",
                "c.txt": "",
            }),
        )
        .await;

        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;
        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            workspace::MultiWorkspace::test_new(project.clone(), window, cx)
        });

        let workspace = multi_workspace.read_with(cx, |multi_workspace, _cx| {
            multi_workspace.workspace().clone()
        });
        cx.run_until_parked();

        let repository = project
            .read_with(cx, |project, cx| project.active_repository(cx))
            .expect("project should have a repository");

        let commit_details = CommitDetails {
            sha: "a".repeat(40).into(),
            message: "commit message".into(),
            commit_timestamp: 0,
            author_email: "dino@zed.dev".into(),
            author_name: "dino".into(),
        };

        let make_file = |name: &str| {
            let old_text = (0..80)
                .map(|row| format!("{name}: original line {row}\n"))
                .collect::<String>();
            let new_text = (0..80)
                .map(|row| format!("{name}: changed line {row}\n"))
                .collect::<String>();

            CommitFile {
                path: RepoPath::new(name).expect("should be valid repository path"),
                old_text: Some(old_text),
                new_text: Some(new_text),
                is_binary: false,
            }
        };

        let commit_diff = CommitDiff {
            files: vec![make_file("a.txt"), make_file("b.txt"), make_file("c.txt")],
            is_shallow_boundary: false,
        };

        let commit_view = workspace.update_in(cx, |workspace, window, cx| {
            let workspace_entity = cx.entity();
            let project = workspace.project().clone();
            let options = CommitViewOptions {
                reveal_file: Some(RepoPath::new("b.txt").unwrap()),
                ..Default::default()
            };

            let commit_view = cx.new(|cx| {
                CommitView::new(
                    commit_details,
                    commit_diff,
                    repository,
                    project,
                    workspace_entity,
                    options,
                    window,
                    cx,
                )
            });

            workspace.active_pane().update(cx, |pane, cx| {
                pane.add_item(Box::new(commit_view.clone()), true, true, None, window, cx);
            });

            commit_view
        });
        cx.run_until_parked();

        let target_path = RepoPath::new("b.txt").expect("should be valid repository path");
        commit_view.update(cx, |commit_view, cx| {
            // Assert that the `Block::BufferHeader` for b.txt is at the viewport top.
            commit_view.editor.update(cx, |editor, cx| {
                editor.rhs_editor().update(cx, |editor, cx| {
                    let multi_buffer = editor.buffer().read(cx);
                    let multi_buffer_snapshot = multi_buffer.snapshot(cx);
                    let display_snapshot = editor.display_snapshot(cx);
                    let max_display_row = display_snapshot.max_point().row();

                    // Confirm that, even though the revealed file's hunks were
                    // added first to the multibuffer, the final order is
                    // unaffected by this.
                    assert_eq!(
                        display_snapshot
                            .blocks_in_range(DisplayRow(0)..max_display_row)
                            .filter_map(|(_display_row, block)| match block {
                                Block::BufferHeader { excerpt, .. } => {
                                    let buffer = excerpt.buffer(&multi_buffer_snapshot);
                                    let file = buffer.file()?;
                                    Some(file.path().as_unix_str().to_owned())
                                }
                                _ => None,
                            })
                            .collect::<Vec<_>>(),
                        ["a.txt", "b.txt", "c.txt"]
                    );

                    let (display_row, anchor) = display_snapshot
                        .blocks_in_range(DisplayRow(0)..max_display_row)
                        .find_map(|(display_row, block)| match block {
                            Block::BufferHeader { excerpt, .. } => {
                                let buffer = excerpt.buffer(&multi_buffer_snapshot);
                                let file = buffer.file()?;

                                (file.path().as_ref() == &*target_path)
                                    .then_some((display_row, excerpt.start_anchor))
                            }
                            _ => None,
                        })
                        .expect("b.txt should have a file header");

                    // Assert that the editor's scroll is anchored to the buffer
                    // header block for the revealed file.
                    let scroll_position = editor.scroll_position(cx);
                    assert!((scroll_position.y - display_row.as_f64()).abs() < 0.001);

                    // Assert that the editor's cursor is at the first excerpt
                    // of the revealed file.
                    let selections = editor.selections.all::<language::Point>(&display_snapshot);
                    let selection = selections.first().expect("should have one selection");
                    let expected_point = multi_buffer::ToPoint::to_point(
                        &anchor,
                        display_snapshot.buffer_snapshot(),
                    );
                    assert_eq!(selections.len(), 1);
                    assert_eq!(selection.start, expected_point);
                    assert_eq!(selection.end, expected_point);
                });
            });
        });
    }

    fn markdown_inline_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Markdown-Inline".into(),
                hidden: true,
                ..LanguageConfig::default()
            },
            Some(tree_sitter_md::INLINE_LANGUAGE.into()),
        )
        .with_highlights_query("(emphasis) @emphasis")
        .unwrap()
    }
}
