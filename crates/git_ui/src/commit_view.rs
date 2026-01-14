use anyhow::{Context as _, Result};
use buffer_diff::BufferDiff;
use collections::HashMap;
use editor::display_map::{BlockPlacement, BlockProperties, BlockStyle};
use editor::{Addon, Editor, EditorEvent, ExcerptRange, MultiBuffer, multibuffer_context_lines};
use git::repository::{CommitDetails, CommitDiff, RepoPath, is_binary_content};
use git::status::{FileStatus, StatusCode, TrackedStatus};
use git::{
    BuildCommitPermalinkParams, GitHostingProviderRegistry, GitRemote, ParsedGitRemote,
    parse_git_remote_url,
};
use gpui::{
    AnyElement, App, AppContext as _, AsyncApp, AsyncWindowContext, ClipboardItem, Context,
    Element, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement,
    ParentElement, PromptLevel, Render, Styled, Task, WeakEntity, Window, actions,
};
use language::{
    Anchor, Buffer, Capability, DiskState, File, LanguageRegistry, LineEnding, OffsetRangeExt as _,
    Point, ReplicaId, Rope, TextBuffer,
};
use multi_buffer::PathKey;
use project::{Project, WorktreeId, git_store::Repository};
use std::{
    any::{Any, TypeId},
    collections::HashSet,
    path::PathBuf,
    sync::Arc,
};
use theme::ActiveTheme;
use ui::{ButtonLike, DiffStat, Tooltip, prelude::*};
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

actions!(git, [ApplyCurrentStash, PopCurrentStash, DropCurrentStash,]);

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
    editor: Entity<Editor>,
    stash: Option<usize>,
    multibuffer: Entity<MultiBuffer>,
    repository: Entity<Repository>,
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
}

const COMMIT_MESSAGE_SORT_PREFIX: u64 = 0;
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
                let (commit_diff, commit_details) = futures::join!(commit_diff?, commit_details?);
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

        let message_buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(commit.message.clone(), cx);
            buffer.set_capability(Capability::ReadOnly, cx);
            buffer
        });

        multibuffer.update(cx, |multibuffer, cx| {
            let snapshot = message_buffer.read(cx).snapshot();
            let full_range = Point::zero()..snapshot.max_point();
            let range = ExcerptRange {
                context: full_range.clone(),
                primary: full_range,
            };
            multibuffer.set_excerpt_ranges_for_path(
                PathKey::with_sort_prefix(
                    COMMIT_MESSAGE_SORT_PREFIX,
                    RelPath::unix("commit message").unwrap().into(),
                ),
                message_buffer.clone(),
                &snapshot,
                vec![range],
                cx,
            )
        });

        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(multibuffer.clone(), Some(project.clone()), window, cx);

            editor.disable_inline_diagnostics();
            editor.set_show_breakpoints(false, cx);
            editor.set_show_diff_review_button(true, cx);
            editor.set_expand_all_diff_hunks(cx);
            editor.disable_header_for_buffer(message_buffer.read(cx).remote_id(), cx);
            editor.disable_indent_guides_for_buffer(message_buffer.read(cx).remote_id(), cx);

            editor.insert_blocks(
                [BlockProperties {
                    placement: BlockPlacement::Above(editor::Anchor::min()),
                    height: Some(1),
                    style: BlockStyle::Sticky,
                    render: Arc::new(|_| gpui::Empty.into_any_element()),
                    priority: 0,
                }]
                .into_iter()
                .chain(
                    editor
                        .buffer()
                        .read(cx)
                        .buffer_anchor_to_anchor(&message_buffer, Anchor::MAX, cx)
                        .map(|anchor| BlockProperties {
                            placement: BlockPlacement::Below(anchor),
                            height: Some(1),
                            style: BlockStyle::Sticky,
                            render: Arc::new(|_| gpui::Empty.into_any_element()),
                            priority: 0,
                        }),
                ),
                None,
                cx,
            );

            editor
        });

        let commit_sha = Arc::<str>::from(commit.sha.as_ref());

        let first_worktree_id = project
            .read(cx)
            .worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).id());

        let repository_clone = repository.clone();

        cx.spawn(async move |this, cx| {
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
                let short_sha = commit_sha.get(0..7).unwrap_or(&commit_sha);
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
                let buffer_id = cx.update(|cx| buffer.read(cx).remote_id());

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
                    None
                } else {
                    Some(build_buffer_diff(old_text, &buffer, &language_registry, cx).await?)
                };

                this.update(cx, |this, cx| {
                    this.multibuffer.update(cx, |multibuffer, cx| {
                        let snapshot = buffer.read(cx).snapshot();
                        let path = snapshot.file().unwrap().path().clone();
                        let excerpt_ranges = if is_binary {
                            vec![language::Point::zero()..snapshot.max_point()]
                        } else if let Some(buffer_diff) = &buffer_diff {
                            let diff_snapshot = buffer_diff.read(cx).snapshot(cx);
                            let mut hunks = diff_snapshot.hunks(&snapshot).peekable();
                            if hunks.peek().is_none() {
                                vec![language::Point::zero()..snapshot.max_point()]
                            } else {
                                hunks
                                    .map(|hunk| hunk.buffer_range.to_point(&snapshot))
                                    .collect::<Vec<_>>()
                            }
                        } else {
                            vec![language::Point::zero()..snapshot.max_point()]
                        };

                        let _is_newly_added = multibuffer.set_excerpts_for_path(
                            PathKey::with_sort_prefix(FILE_NAMESPACE_SORT_PREFIX, path),
                            buffer,
                            excerpt_ranges,
                            multibuffer_context_lines(cx),
                            cx,
                        );
                        if let Some(buffer_diff) = buffer_diff {
                            multibuffer.add_diff(buffer_diff, cx);
                        }
                    });
                })?;
            }

            this.update(cx, |this, cx| {
                this.editor.update(cx, |editor, _cx| {
                    editor.register_addon(CommitDiffAddon { file_statuses });
                });
                if !binary_buffer_ids.is_empty() {
                    this.editor.update(cx, |editor, cx| {
                        for buffer_id in binary_buffer_ids {
                            editor.fold_buffer(buffer_id, cx);
                        }
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
            multibuffer,
            stash,
            repository,
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
        let size = size.into();
        let avatar = CommitAvatar::new(sha, self.remote.as_ref());

        v_flex()
            .w(size)
            .h(size)
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_full()
            .justify_center()
            .items_center()
            .child(
                avatar
                    .avatar(window, cx)
                    .map(|a| a.size(size).into_any_element())
                    .unwrap_or_else(|| {
                        Icon::new(IconName::Person)
                            .color(Color::Muted)
                            .size(IconSize::Medium)
                            .into_any_element()
                    }),
            )
            .into_any()
    }

    fn calculate_changed_lines(&self, cx: &App) -> (u32, u32) {
        let snapshot = self.multibuffer.read(cx).snapshot(cx);
        let mut total_additions = 0u32;
        let mut total_deletions = 0u32;

        let mut seen_buffers = std::collections::HashSet::new();
        for (_, buffer, _) in snapshot.excerpts() {
            let buffer_id = buffer.remote_id();
            if !seen_buffers.insert(buffer_id) {
                continue;
            }

            let Some(diff) = snapshot.diff_for_buffer_id(buffer_id) else {
                continue;
            };

            let base_text = diff.base_text();

            for hunk in diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, buffer) {
                let added_rows = hunk.range.end.row.saturating_sub(hunk.range.start.row);
                total_additions += added_rows;

                let base_start = base_text
                    .offset_to_point(hunk.diff_base_byte_range.start)
                    .row;
                let base_end = base_text.offset_to_point(hunk.diff_base_byte_range.end).row;
                let deleted_rows = base_end.saturating_sub(base_start);

                total_deletions += deleted_rows;
            }
        }

        (total_additions, total_deletions)
    }

    fn render_header(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let commit = &self.commit;
        let author_name = commit.author_name.clone();
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

        let remote_info = self.remote.as_ref().map(|remote| {
            let provider = remote.host.name();
            let parsed_remote = ParsedGitRemote {
                owner: remote.owner.as_ref().into(),
                repo: remote.repo.as_ref().into(),
            };
            let params = BuildCommitPermalinkParams { sha: &commit.sha };
            let url = remote
                .host
                .build_commit_permalink(&parsed_remote, params)
                .to_string();
            (provider, url)
        });

        let (additions, deletions) = self.calculate_changed_lines(cx);

        let commit_diff_stat = if additions > 0 || deletions > 0 {
            Some(DiffStat::new(
                "commit-diff-stat",
                additions as usize,
                deletions as usize,
            ))
        } else {
            None
        };

        let gutter_width = self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(window, cx);
            let style = editor.style(cx);
            let font_id = window.text_system().resolve_font(&style.text.font());
            let font_size = style.text.font_size.to_pixels(window.rem_size());
            snapshot
                .gutter_dimensions(font_id, font_size, style, window, cx)
                .full_width()
        });

        let clipboard_has_link = cx
            .read_from_clipboard()
            .and_then(|entry| entry.text())
            .map_or(false, |clipboard_text| {
                clipboard_text.trim() == commit_sha.as_ref()
            });

        let (copy_icon, copy_icon_color) = if clipboard_has_link {
            (IconName::Check, Color::Success)
        } else {
            (IconName::Copy, Color::Muted)
        };

        h_flex()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .w_full()
            .child(
                h_flex()
                    .w(gutter_width)
                    .justify_center()
                    .child(self.render_commit_avatar(&commit.sha, rems_from_px(48.), window, cx)),
            )
            .child(
                h_flex()
                    .py_4()
                    .pl_1()
                    .pr_4()
                    .w_full()
                    .items_start()
                    .justify_between()
                    .flex_wrap()
                    .child(
                        v_flex()
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(Label::new(author_name).color(Color::Default))
                                    .child({
                                        ButtonLike::new("sha")
                                            .child(
                                                h_flex()
                                                    .group("sha_btn")
                                                    .size_full()
                                                    .max_w_32()
                                                    .gap_0p5()
                                                    .child(
                                                        Label::new(commit_sha.clone())
                                                            .color(Color::Muted)
                                                            .size(LabelSize::Small)
                                                            .truncate()
                                                            .buffer_font(cx),
                                                    )
                                                    .child(
                                                        div().visible_on_hover("sha_btn").child(
                                                            Icon::new(copy_icon)
                                                                .color(copy_icon_color)
                                                                .size(IconSize::Small),
                                                        ),
                                                    ),
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
                                            })
                                    }),
                            )
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
                                            .color(Color::Ignored)
                                            .size(LabelSize::Small),
                                    )
                                    .children(commit_diff_stat),
                            ),
                    )
                    .children(remote_info.map(|(provider_name, url)| {
                        let icon = match provider_name.as_str() {
                            "GitHub" => IconName::Github,
                            _ => IconName::Link,
                        };

                        Button::new("view_on_provider", format!("View on {}", provider_name))
                            .icon(icon)
                            .icon_color(Color::Muted)
                            .icon_size(IconSize::Small)
                            .icon_position(IconPosition::Start)
                            .on_click(move |_, _, cx| cx.open_url(&url))
                    })),
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
            .detach_and_notify_err(window, cx);
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
    cx: &mut AsyncApp,
) -> Result<Entity<Buffer>> {
    let line_ending = LineEnding::detect(&text);
    LineEnding::normalize(&mut text);
    let text = Rope::from(text);
    let language = cx.update(|cx| language_registry.language_for_file(&blob, Some(&text), cx));
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
        buffer.set_language_async(language, cx);
        buffer
    });
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

    let language = cx.update(|cx| buffer.read(cx).language().cloned());
    let buffer = cx.update(|cx| buffer.read(cx).snapshot());

    let diff = cx.new(|cx| BufferDiff::new(&buffer.text, cx));

    let update = diff
        .update(cx, |diff, cx| {
            diff.update_diff(
                buffer.text.clone(),
                old_text.map(|old_text| Arc::from(old_text.as_str())),
                true,
                language.clone(),
                cx,
            )
        })
        .await;

    diff.update(cx, |diff, cx| {
        diff.language_changed(language, Some(language_registry.clone()), cx);
        diff.set_snapshot(update, &buffer.text, cx)
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
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.clone().into())
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
            .addon::<CommitDiffAddon>()
            .map(|addon| addon.file_statuses.clone())
            .unwrap_or_default();
        Task::ready(Some(cx.new(|cx| {
            let editor = cx.new({
                let file_statuses = file_statuses.clone();
                |cx| {
                    let mut editor = self
                        .editor
                        .update(cx, |editor, cx| editor.clone(window, cx));
                    editor.register_addon(CommitDiffAddon { file_statuses });
                    editor
                }
            });
            let multibuffer = editor.read(cx).buffer().clone();
            Self {
                editor,
                multibuffer,
                commit: self.commit.clone(),
                stash: self.stash,
                repository: self.repository.clone(),
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
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(self.render_header(window, cx))
            .when(!self.editor.read(cx).is_empty(cx), |this| {
                this.child(div().flex_grow().child(self.editor.clone()))
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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().hidden()
    }
}

impl ToolbarItemView for CommitViewToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        if let Some(entity) = active_pane_item.and_then(|i| i.act_as::<CommitView>(cx))
            && entity.read(cx).stash.is_some()
        {
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

fn stash_matches_index(sha: &str, stash_index: usize, repo: &Repository) -> bool {
    repo.stash_entries
        .entries
        .get(stash_index)
        .map(|entry| entry.oid.to_string() == sha)
        .unwrap_or(false)
}
