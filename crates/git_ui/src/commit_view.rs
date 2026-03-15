use anyhow::{Context as _, Result};
use buffer_diff::BufferDiff;
use collections::HashMap;
use editor::{
    Addon, Editor, EditorEvent, EditorSettings, MultiBuffer, SplittableEditor,
    multibuffer_context_lines,
};
use feature_flags::{FeatureFlagAppExt as _, GitGraphFeatureFlag};
use git::repository::{CommitDetails, CommitDiff, RepoPath, is_binary_content};
use git::status::{FileStatus, StatusCode, TrackedStatus};
use git::{
    BuildCommitPermalinkParams, GitHostingProviderRegistry, GitRemote, ParsedGitRemote,
    parse_git_remote_url,
};
use gpui::{
    AnyElement, App, AppContext as _, AsyncApp, AsyncWindowContext, ClipboardItem, Context, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement,
    PromptLevel, Render, Styled, Task, WeakEntity, Window, actions,
};
use language::{
    Anchor, Buffer, Capability, DiskState, File, LanguageRegistry, LineEnding, OffsetRangeExt as _,
    ReplicaId, Rope, TextBuffer, language_settings,
};
use multi_buffer::PathKey;
use project::{Project, WorktreeId, git_store::Repository};
use settings::Settings;
use std::{
    any::{Any, TypeId},
    collections::HashSet,
    path::PathBuf,
    sync::Arc,
};
use theme::ActiveTheme;
use ui::{DiffStat, Divider, Tooltip, prelude::*};
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
    message_editor: Entity<Editor>,
    editor: Entity<SplittableEditor>,
    stash: Option<usize>,
    multibuffer: Entity<MultiBuffer>,
    repository: Entity<Repository>,
    remote: Option<GitRemote>,
    workspace: WeakEntity<Workspace>,
    binary_buffer_ids: HashSet<language::BufferId>,
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

const FILE_NAMESPACE_SORT_PREFIX: u64 = 0;

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
                        let workspace_handle = cx.entity();
                        let commit_view = cx.new(|cx| {
                            CommitView::new(
                                commit_details,
                                commit_diff,
                                repo,
                                project.clone(),
                                workspace_handle.clone(),
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
        workspace: Entity<Workspace>,
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
        let message_editor = Self::new_message_editor(message_buffer, project.clone(), window, cx);
        let editor = Self::new_editor(
            multibuffer.clone(),
            project.clone(),
            workspace.clone(),
            window,
            cx,
        );

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

                let buffer = build_buffer(new_text.clone(), file, &language_registry, cx).await?;
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
                    Some(build_buffer_diff(Some(new_text), &buffer, &language_registry, cx).await?)
                } else {
                    Some(build_buffer_diff(old_text, &buffer, &language_registry, cx).await?)
                };

                this.update(cx, |this, cx| -> anyhow::Result<()> {
                    let snapshot = buffer.read(cx).snapshot();
                    let path = snapshot
                        .file()
                        .context("commit view buffer missing file metadata")?
                        .path()
                        .clone();
                    let excerpt_ranges = if let Some(buffer_diff) = &buffer_diff {
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

                    if let Some(buffer_diff) = buffer_diff {
                        this.editor.update(cx, |editor, cx| {
                            editor.set_excerpts_for_path(
                                PathKey::with_sort_prefix(FILE_NAMESPACE_SORT_PREFIX, path),
                                buffer,
                                excerpt_ranges,
                                multibuffer_context_lines(cx),
                                buffer_diff,
                                cx,
                            );
                        });
                    }
                    Ok(())
                })??;
            }

            this.update(cx, |this, cx| {
                this.editor.update(cx, |editor, _cx| {
                    editor.rhs_editor().update(_cx, |rhs_editor, _| {
                        rhs_editor.register_addon(CommitDiffAddon { file_statuses });
                    });
                });
                this.binary_buffer_ids = binary_buffer_ids.clone();
                if !binary_buffer_ids.is_empty() {
                    this.editor.update(cx, |editor, cx| {
                        editor.rhs_editor().update(cx, |rhs_editor, cx| {
                            rhs_editor.fold_buffers(binary_buffer_ids, cx);
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
            message_editor,
            editor,
            multibuffer,
            stash,
            repository,
            remote,
            workspace: workspace.downgrade(),
            binary_buffer_ids: HashSet::default(),
        }
    }

    fn new_editor(
        multibuffer: Entity<MultiBuffer>,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<SplittableEditor> {
        cx.new(|cx| {
            let editor = SplittableEditor::new(
                EditorSettings::get_global(cx).diff_view_style,
                multibuffer,
                project,
                workspace,
                window,
                cx,
            );
            editor.rhs_editor().update(cx, |rhs_editor, cx| {
                rhs_editor.disable_inline_diagnostics();
                rhs_editor.set_show_breakpoints(false, cx);
                rhs_editor.set_show_diff_review_button(true, cx);
            });
            editor
        })
    }

    fn new_message_editor(
        message_buffer: Entity<Buffer>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<Editor> {
        cx.new(|cx| {
            let mut editor = Editor::for_buffer(message_buffer, Some(project), window, cx);
            editor.set_read_only(true);
            editor.disable_inline_diagnostics();
            editor.set_show_breakpoints(false, cx);
            editor.set_show_gutter(false, cx);
            editor.set_show_horizontal_scrollbar(false, cx);
            editor.set_soft_wrap_mode(language_settings::SoftWrap::EditorWidth, cx);
            editor
        })
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

    fn message_editor_height(&self, window: &Window) -> gpui::Pixels {
        let visible_line_count = self.commit.message.lines().count().clamp(1, 12) as f32;
        window.line_height() * visible_line_count + px(12.)
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

        let gutter_width = self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |rhs_editor, cx| {
                let snapshot = rhs_editor.snapshot(window, cx);
                let style = rhs_editor.style(cx);
                let font_id = window.text_system().resolve_font(&style.text.font());
                let font_size = style.text.font_size.to_pixels(window.rem_size());
                snapshot
                    .gutter_dimensions(font_id, font_size, style, window, cx)
                    .full_width()
            })
        });

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

        h_flex()
            .py_2()
            .pr_2p5()
            .w_full()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                h_flex()
                    .child(h_flex().w(gutter_width).justify_center().child(
                        self.render_commit_avatar(&commit.sha, rems_from_px(40.), window, cx),
                    ))
                    .child(
                        v_flex().child(Label::new(author_name)).child(
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
                                Tooltip::with_meta("Copy Commit SHA", None, commit_sha.clone(), cx)
                            }
                        })
                        .on_click(move |_, _, cx| {
                            cx.stop_propagation();
                            cx.write_to_clipboard(ClipboardItem::new_string(
                                commit_sha.to_string(),
                            ));
                        }),
                )
            })
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
                Some(true),
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
        if self.multibuffer.read(cx).is_empty() {
            self.message_editor.focus_handle(cx)
        } else {
            self.editor.focus_handle(cx)
        }
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
        self.message_editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |rhs_editor, cx| {
                rhs_editor.deactivated(window, cx);
            });
        });
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        cx: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.read(cx).rhs_editor().clone().into())
        } else if type_id == TypeId::of::<SplittableEditor>() {
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
        self.editor
            .read(cx)
            .rhs_editor()
            .read(cx)
            .for_each_project_item(cx, f)
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |rhs_editor, _| {
                rhs_editor.set_nav_history(Some(nav_history));
            });
        });
    }

    fn navigate(
        &mut self,
        data: Arc<dyn Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor.update(cx, |editor, cx| {
            editor
                .rhs_editor()
                .update(cx, |rhs_editor, cx| rhs_editor.navigate(data, window, cx))
        })
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.message_editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx);
        });
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
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(None);
        };
        let project = workspace.read(cx).project().clone();
        let file_statuses = self
            .editor
            .read(cx)
            .rhs_editor()
            .read(cx)
            .addon::<CommitDiffAddon>()
            .map(|addon| addon.file_statuses.clone())
            .unwrap_or_default();
        let binary_buffer_ids = self.binary_buffer_ids.clone();
        let multibuffer = self.multibuffer.clone();
        Task::ready(Some(cx.new(|cx| {
            let message_buffer = cx.new(|cx| {
                let mut buffer = Buffer::local(self.commit.message.clone(), cx);
                buffer.set_capability(Capability::ReadOnly, cx);
                buffer
            });
            let message_editor =
                Self::new_message_editor(message_buffer, project.clone(), window, cx);
            let editor =
                Self::new_editor(multibuffer.clone(), project, workspace.clone(), window, cx);
            editor.update(cx, |editor, cx| {
                editor.rhs_editor().update(cx, |rhs_editor, cx| {
                    rhs_editor.register_addon(CommitDiffAddon {
                        file_statuses: file_statuses.clone(),
                    });
                    if !binary_buffer_ids.is_empty() {
                        rhs_editor.fold_buffers(binary_buffer_ids.clone(), cx);
                    }
                });
            });
            Self {
                commit: self.commit.clone(),
                message_editor,
                editor,
                stash: self.stash,
                multibuffer,
                repository: self.repository.clone(),
                remote: self.remote.clone(),
                workspace: workspace.downgrade(),
                binary_buffer_ids,
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
            .when(!self.commit.message.is_empty(), |this| {
                this.child(
                    div()
                        .border_b_1()
                        .border_color(cx.theme().colors().border_variant)
                        .h(self.message_editor_height(window))
                        .child(self.message_editor.clone()),
                )
            })
            .when(!self.multibuffer.read(cx).is_empty(), |this| {
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
                this.when(cx.has_flag::<GitGraphFeatureFlag>(), |this| {
                    this.child(
                        IconButton::new("show-in-git-graph", IconName::GitGraph)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Show in Git Graph"))
                            .on_click(move |_, window, cx| {
                                window.dispatch_action(
                                    Box::new(crate::git_panel::OpenAtCommit {
                                        sha: sha_for_graph.clone(),
                                    }),
                                    cx,
                                );
                            }),
                    )
                })
                .children(remote_info.map(|(provider_name, url)| {
                    let icon = match provider_name.as_str() {
                        "GitHub" => IconName::Github,
                        _ => IconName::Link,
                    };

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

    use git::repository::{CommitDetails, CommitDiff, CommitFile};
    use gpui::TestAppContext;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::{DiffViewStyle, SettingsStore};
    use util::path;
    use workspace::MultiWorkspace;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.diff_view_style = Some(DiffViewStyle::Split);
                });
            });
            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    #[gpui::test]
    async fn test_commit_view_uses_split_editor_and_syncs_excerpts(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "foo.txt": "after\n",
            }),
        )
        .await;
        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("foo.txt", "before\n".into())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("foo.txt", "before\n".into())],
        );

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        cx.run_until_parked();

        let repository = cx
            .read(|cx| project.read(cx).active_repository(cx))
            .expect("project should expose an active repository");

        let commit_view = cx.new_window_entity(|window, cx| {
            CommitView::new(
                CommitDetails {
                    sha: "deadbeefcafebabe".into(),
                    message: "subject\n\nbody".into(),
                    commit_timestamp: 0,
                    author_email: "test@example.com".into(),
                    author_name: "Test User".into(),
                },
                CommitDiff {
                    files: vec![
                        CommitFile {
                            path: RepoPath::new("foo.txt").unwrap(),
                            old_text: Some("before\n".into()),
                            new_text: Some("after\n".into()),
                            is_binary: false,
                        },
                        CommitFile {
                            path: RepoPath::new("bin.dat").unwrap(),
                            old_text: None,
                            new_text: Some("\0".into()),
                            is_binary: true,
                        },
                    ],
                },
                repository,
                project.clone(),
                workspace,
                None,
                window,
                cx,
            )
        });
        cx.run_until_parked();
        cx.run_until_parked();

        let has_splittable_editor = commit_view.read_with(cx, |view, cx| {
            view.act_as_type(TypeId::of::<SplittableEditor>(), &commit_view, cx)
                .is_some()
        });
        assert!(has_splittable_editor);

        commit_view.read_with(cx, |view, cx| {
            assert_eq!(view.message_editor.read(cx).text(cx), "subject\n\nbody");

            let editor = view.editor.read(cx);
            assert!(
                editor.is_split(),
                "commit view should honor split diff mode"
            );

            let rhs_excerpt_count = editor
                .rhs_editor()
                .read(cx)
                .buffer()
                .read(cx)
                .excerpt_ids()
                .len();
            let lhs_editor = editor
                .lhs_editor()
                .expect("split diff mode should create a left-hand editor");
            let lhs_excerpt_count = lhs_editor.read(cx).buffer().read(cx).excerpt_ids().len();

            assert_eq!(rhs_excerpt_count, lhs_excerpt_count);
            assert_eq!(rhs_excerpt_count, 2);
        });
    }
}
