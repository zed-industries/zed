use anyhow::{Context as _, Result};
use buffer_diff::BufferDiff;
use collections::HashMap;
use editor::{
    Addon, Editor, EditorEvent, EditorSettings, MultiBuffer, RestoreOnlyDiffHunkDelegate,
    SplittableEditor, hover_markdown_style, multibuffer_context_lines,
};
use futures_lite::future::yield_now;
use git::repository::{CommitDetails, CommitDiff, RepoPath, is_binary_content};
use git::status::{FileStatus, StatusCode, TrackedStatus};
use git::{
    BuildCommitPermalinkParams, GitHostingProviderRegistry, GitRemote, ParsedGitRemote,
    parse_git_remote_url,
};
use gpui::{
    AnyElement, App, AppContext as _, AsyncWindowContext, ClipboardItem, Context, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement,
    PromptLevel, Render, ScrollHandle, StatefulInteractiveElement as _, Styled, Task, WeakEntity,
    Window, actions,
};
use language::{
    Buffer, Capability, DiskState, File, LanguageRegistry, LineEnding, OffsetRangeExt as _,
    ReplicaId, Rope, TextBuffer,
};
use markdown::{Markdown, MarkdownElement};
use multi_buffer::PathKey;
use project::{Project, ProjectPath, WorktreeId, git_store::Repository};
use settings::{DiffViewStyle, Settings};
use std::{
    any::{Any, TypeId},
    collections::HashSet,
    path::PathBuf,
    sync::Arc,
};
use theme::ActiveTheme;
use ui::{ContextMenu, DiffStat, Disclosure, Divider, Tooltip, WithScrollbar, prelude::*};
use util::{ResultExt, paths::PathStyle, rel_path::RelPath, truncate_and_trailoff};
use workspace::item::TabTooltipContent;
use workspace::{
    Item, ItemHandle, ItemNavHistory, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
    Workspace,
    item::{ItemEvent, TabContentParams},
    notifications::NotifyTaskExt,
    pane::SaveIntent,
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
}

struct GitBlob {
    path: RepoPath,
    worktree_id: WorktreeId,
    is_deleted: bool,
    is_binary: bool,
    display_name: String,
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
                        .update(cx, |view, cx| view.open_file_at_head(&file, window, cx))
                        .log_err();
                },
            )
        })
    }
}

const FILE_NAMESPACE_SORT_PREFIX: u64 = 1;

impl CommitView {
    pub fn open(
        commit_sha: String,
        repo: WeakEntity<Repository>,
        workspace: WeakEntity<Workspace>,
        stash: Option<usize>,
        file_filter: Option<RepoPath>,
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
                let commit_diff = commit_diff?;
                let commit_details = commit_details?;
                let (commit_diff, commit_details) = futures::join!(commit_diff, commit_details);
                let mut commit_diff = commit_diff.log_err()?.log_err()?;
                let commit_details = commit_details.log_err()?.log_err()?;

                // Filter to specific file if requested
                if let Some(ref filter_path) = file_filter {
                    commit_diff.files.retain(|f| &f.path == filter_path);
                }

                let repo = repo.upgrade()?;

                workspace
                    .update_in(cx, |workspace, window, cx| {
                        let project = workspace.project();
                        let workspace_entity = cx.entity();
                        let workspace_handle = cx.weak_entity();
                        let commit_view = cx.new(|cx| {
                            CommitView::new(
                                commit_details,
                                commit_diff,
                                repo,
                                project.clone(),
                                workspace_entity,
                                workspace_handle,
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
                                let existing = pane
                                    .items()
                                    .filter_map(|item| item.downcast::<CommitView>())
                                    .find(|view| view.read(cx).commit.sha == commit_sha)
                                    .unwrap();

                                pane.remove_item(existing.item_id(), false, false, window, cx);
                                pane.add_item(
                                    Box::new(commit_view),
                                    true,
                                    true,
                                    Some(ix),
                                    window,
                                    cx,
                                );
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
        workspace_entity: Entity<Workspace>,
        workspace: WeakEntity<Workspace>,
        stash: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let language_registry = project.read(cx).languages().clone();
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
                workspace_entity.clone(),
                window,
                cx,
            );
            editor.set_diff_hunk_delegate(Some(Arc::new(RestoreOnlyDiffHunkDelegate)), cx);

            editor.rhs_editor().update(cx, |editor, cx| {
                editor.set_show_bookmarks(false, cx);
                editor.set_show_breakpoints(false, cx);
                editor.set_show_diff_review_button(true, cx);
            });

            editor
        });
        let commit_sha = Arc::<str>::from(commit.sha.as_ref());

        let first_worktree_id = project
            .read(cx)
            .worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).id());

        let repository_clone = repository.clone();

        cx.spawn_in(window, async move |this, cx| {
            let mut binary_buffer_ids: HashSet<language::BufferId> = HashSet::default();
            let mut file_statuses: HashMap<language::BufferId, FileStatus> = HashMap::default();

            for file in commit_diff.files {
                let is_created = file.old_text.is_none();
                let is_deleted = file.new_text.is_none();
                let raw_new_text = file.new_text.unwrap_or_default();
                let raw_old_text = file.old_text;

                let is_binary = file.is_binary
                    || is_binary_content(raw_new_text.as_bytes())
                    || raw_old_text
                        .as_ref()
                        .is_some_and(|text| is_binary_content(text.as_bytes()));

                let new_text = if is_binary {
                    "(binary file not shown)".to_string()
                } else {
                    raw_new_text
                };
                let old_text = if is_binary { None } else { raw_old_text };
                let worktree_id = repository_clone
                    .update(cx, |repository, cx| {
                        repository
                            .repo_path_to_project_path(&file.path, cx)
                            .map(|path| path.worktree_id)
                            .or(first_worktree_id)
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

                let buffer = build_buffer(new_text, file, &language_registry, cx).await?;
                let buffer_id = cx.update(|_, cx| buffer.read(cx).remote_id())?;

                let status_code = if is_created {
                    StatusCode::Added
                } else if is_deleted {
                    StatusCode::Deleted
                } else {
                    StatusCode::Modified
                };
                file_statuses.insert(
                    buffer_id,
                    FileStatus::Tracked(TrackedStatus {
                        index_status: status_code,
                        worktree_status: StatusCode::Unmodified,
                    }),
                );

                if is_binary {
                    binary_buffer_ids.insert(buffer_id);
                }

                let buffer_diff = if is_binary {
                    cx.update(|_, cx| {
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
                    build_buffer_diff(old_text, &buffer, &language_registry, cx).await?
                };

                let (excerpt_ranges, path) = cx.update(|_, cx| {
                    let snapshot = buffer.read(cx).snapshot();
                    let path = PathKey::with_sort_prefix(
                        FILE_NAMESPACE_SORT_PREFIX,
                        snapshot.file().unwrap().path().clone(),
                    );
                    let ranges = if is_binary {
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

                // Batch the insertion of excerpts and yield between batches, to avoid blocking the main thread when a single file has many hunks.
                const EXCERPT_BATCH_SIZE: usize = 10;
                let total = excerpt_ranges.len();
                let mut batch_end = 0;
                while batch_end < total {
                    let is_first_batch = batch_end == 0;
                    batch_end = (batch_end + EXCERPT_BATCH_SIZE).min(total);
                    let ranges = excerpt_ranges[..batch_end].to_vec();
                    this.update_in(cx, |this, window, cx| {
                        this.editor.update(cx, |editor, cx| {
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
            }

            this.update(cx, |this, cx| {
                let commit_view = cx.weak_entity();
                this.editor.update(cx, |editor, cx| {
                    editor.rhs_editor().update(cx, |editor, _cx| {
                        editor.register_addon(CommitDiffAddon {
                            file_statuses,
                            commit_view,
                        });
                    });
                });
                if !binary_buffer_ids.is_empty() {
                    this.editor.update(cx, |editor, cx| {
                        editor.rhs_editor().update(cx, |editor, cx| {
                            editor.fold_buffers(binary_buffer_ids, cx);
                        });
                    });
                }
            })?;

            anyhow::Ok(())
        })
        .detach();

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
            message_expanded: false,
            message_scroll_handle: ScrollHandle::new(),
            multibuffer,
            stash,
            repository,
            project,
            workspace,
            remote,
        }
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

    fn open_file_at_head(
        &mut self,
        file: &Arc<dyn language::File>,
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

        self.workspace
            .update(cx, |workspace, cx| {
                workspace
                    .open_path_preview(project_path, None, false, false, true, window, cx)
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
        self.open_file_at_head(&file, window, cx);
    }

    fn render_header(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let commit = &self.commit;
        let author_name = commit.author_name.clone();
        let author_email = commit.author_email.clone();
        let commit_sha = commit.sha.clone();
        let commit_date = time::OffsetDateTime::from_unix_timestamp(commit.commit_timestamp)
            .unwrap_or_else(|_| time::OffsetDateTime::now_utc());
        let local_offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
        let date_string = time_format::format_localized_timestamp(
            commit_date,
            time::OffsetDateTime::now_utc(),
            local_offset,
            time_format::TimestampFormat::MediumAbsolute,
        );

        let avatar_size = rems_from_px(40.);
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
        let avatar_min_side_padding = rems_from_px(6.).to_pixels(window.rem_size());
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

        let markdown_style = hover_markdown_style(window, cx);

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

async fn build_buffer(
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
        let mut buffer = Buffer::build(buffer, Some(blob), Capability::ReadWrite);
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
                move |cx| {
                    let editor = SplittableEditor::new(
                        diff_view_style,
                        multibuffer.clone(),
                        project.clone(),
                        workspace_entity.clone(),
                        window,
                        cx,
                    );
                    editor.set_diff_hunk_delegate(Some(Arc::new(RestoreOnlyDiffHunkDelegate)), cx);
                    editor.rhs_editor().update(cx, |editor, cx| {
                        editor.set_show_bookmarks(false, cx);
                        editor.set_show_breakpoints(false, cx);
                        editor.set_show_diff_review_button(true, cx);
                        editor.register_addon(CommitDiffAddon {
                            file_statuses,
                            commit_view,
                        });
                    });
                    editor
                }
            });
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
                    let icon = crate::get_provider_icon(provider_name.as_str());

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
