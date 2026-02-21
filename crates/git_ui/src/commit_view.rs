use anyhow::{Context as _, Result};
use buffer_diff::BufferDiff;
use collections::HashMap;
use editor::{
    Addon, Editor, EditorEvent, EditorSettings, MultiBuffer, SelectionEffects, SplittableEditor,
    multibuffer_context_lines, scroll::Autoscroll,
};
use git::repository::{CommitDetails, CommitDiff, RepoPath, is_binary_content};
use git::status::{FileStatus, StatusCode, TrackedStatus};
use git::{GitHostingProviderRegistry, GitRemote, parse_git_remote_url};
use gpui::{
    Action, AnyElement, App, AppContext as _, AsyncApp, AsyncWindowContext, Context, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement,
    PromptLevel, Render, SharedString, Styled, Task, WeakEntity, Window, actions,
};
use language::{
    Buffer, Capability, DiskState, File, LanguageRegistry, LineEnding, OffsetRangeExt as _,
    ReplicaId, Rope, TextBuffer,
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
use ui::{Avatar, ButtonLike, CopyButton, Tooltip, prelude::*};
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

use crate::commit_details_sidebar::{
    CommitDetailsSidebar, CommitDetailsSidebarData, get_remote_from_repository,
};
use crate::commit_tooltip::CommitAvatarAsset;
use crate::git_panel::GitPanel;

actions!(
    git,
    [
        ApplyCurrentStash,
        PopCurrentStash,
        DropCurrentStash,
        ToggleCommitDetailsSidebar,
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
        workspace.register_action(|workspace, _: &ToggleCommitDetailsSidebar, window, cx| {
            CommitView::toggle_details_sidebar(workspace, window, cx);
        });
    })
    .detach();
}

pub struct CommitView {
    commit: CommitDetails,
    editor: Entity<SplittableEditor>,
    stash: Option<usize>,
    multibuffer: Entity<MultiBuffer>,
    repository: Entity<Repository>,
    workspace: WeakEntity<Workspace>,
    remote: Option<GitRemote>,
    changed_files: Vec<(RepoPath, FileStatus)>,
    show_details_sidebar: bool,
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
                        let workspace_handle = cx.entity();
                        let commit_view = cx.new(|cx| {
                            CommitView::new(
                                commit_details,
                                commit_diff,
                                repo,
                                project.clone(),
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

        let changed_files: Vec<(RepoPath, FileStatus)> = commit_diff
            .files
            .iter()
            .map(|file| {
                let is_created = file.old_text.is_none();
                let is_deleted = file.new_text.is_none();
                let status_code = if is_created {
                    StatusCode::Added
                } else if is_deleted {
                    StatusCode::Deleted
                } else {
                    StatusCode::Modified
                };
                (
                    file.path.clone(),
                    FileStatus::Tracked(TrackedStatus {
                        index_status: status_code,
                        worktree_status: StatusCode::Unmodified,
                    }),
                )
            })
            .collect();

        let editor = cx.new(|cx| {
            let editor = SplittableEditor::new(
                EditorSettings::get_global(cx).diff_view_style,
                multibuffer.clone(),
                project.clone(),
                workspace.clone(),
                window,
                cx,
            );
            editor.rhs_editor().update(cx, |editor, cx| {
                editor.disable_inline_diagnostics();
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
                let old_text = if is_binary {
                    Some(new_text.clone())
                } else {
                    raw_old_text
                };
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

                let buffer_diff =
                    build_buffer_diff(old_text, &buffer, &language_registry, cx).await?;

                this.update(cx, |this, cx| {
                    let snapshot = buffer.read(cx).snapshot();
                    let path = snapshot.file().unwrap().path().clone();
                    let path_key = PathKey::with_sort_prefix(FILE_NAMESPACE_SORT_PREFIX, path);

                    let diff_snapshot = buffer_diff.read(cx).snapshot(cx);
                    let mut hunks = diff_snapshot.hunks(&snapshot).peekable();
                    let excerpt_ranges = if hunks.peek().is_none() {
                        vec![language::Point::zero()..snapshot.max_point()]
                    } else {
                        hunks
                            .map(|hunk| hunk.buffer_range.to_point(&snapshot))
                            .collect::<Vec<_>>()
                    };

                    this.editor.update(cx, |editor, cx| {
                        editor.set_excerpts_for_path(
                            path_key,
                            buffer,
                            excerpt_ranges,
                            multibuffer_context_lines(cx),
                            buffer_diff,
                            cx,
                        );
                    });
                })?;
            }

            this.update(cx, |this, cx| {
                this.editor.update(cx, |editor, cx| {
                    editor.rhs_editor().update(cx, |rhs_editor, _cx| {
                        rhs_editor.register_addon(CommitDiffAddon { file_statuses });
                    });
                });
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
            editor,
            multibuffer,
            stash,
            repository,
            workspace: workspace.downgrade(),
            remote,
            changed_files,
            show_details_sidebar: false,
        }
    }

    pub fn toggle_details_sidebar(workspace: &mut Workspace, _window: &mut Window, cx: &mut App) {
        if let Some(commit_view) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<CommitView>(cx))
        {
            commit_view.update(cx, |this, cx| {
                this.show_details_sidebar = !this.show_details_sidebar;
                cx.notify();
            });
        }
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

    fn to_item_events(event: &EditorEvent, f: &mut dyn FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Commit View Opened")
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor
                .rhs_editor()
                .update(cx, |rhs_editor, cx| rhs_editor.deactivated(window, cx));
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
        let file_statuses = self
            .editor
            .read(cx)
            .rhs_editor()
            .read(cx)
            .addon::<CommitDiffAddon>()
            .map(|addon| addon.file_statuses.clone())
            .unwrap_or_default();
        Task::ready(Some(cx.new(|cx| {
            let editor = cx.new({
                let file_statuses = file_statuses.clone();
                let workspace = workspace.clone();
                |cx| {
                    let editor = SplittableEditor::new(
                        EditorSettings::get_global(cx).diff_view_style,
                        self.multibuffer.clone(),
                        workspace.read(cx).project().clone(),
                        workspace,
                        window,
                        cx,
                    );
                    editor.rhs_editor().update(cx, |rhs_editor, _cx| {
                        rhs_editor.register_addon(CommitDiffAddon { file_statuses });
                    });
                    editor
                }
            });
            let multibuffer = self.multibuffer.clone();
            Self {
                editor,
                multibuffer,
                commit: self.commit.clone(),
                stash: self.stash,
                repository: self.repository.clone(),
                workspace: self.workspace.clone(),
                remote: self.remote.clone(),
                changed_files: self.changed_files.clone(),
                show_details_sidebar: self.show_details_sidebar,
            }
        })))
    }
}

impl Render for CommitView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_stash = self.stash.is_some();

        let sidebar = self.show_details_sidebar.then(|| {
            let mut lines = self.commit.message.lines();
            let subject: SharedString = lines.next().unwrap_or("").to_string().into();
            let body: SharedString = lines
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string()
                .into();

            let remote = self
                .repository
                .update(cx, |repo, cx| get_remote_from_repository(repo, cx));

            let data = CommitDetailsSidebarData::new(
                self.commit.sha.clone(),
                self.commit.author_name.clone(),
                self.commit.author_email.clone(),
                self.commit.commit_timestamp,
                subject,
                body,
            );

            let multibuffer = self.multibuffer.clone();
            let editor = self.editor.clone();

            CommitDetailsSidebar::new(data)
                .remote(remote)
                .changed_files(self.changed_files.clone())
                .on_file_click(move |repo_path, _, window, cx| {
                    let path_key = PathKey::with_sort_prefix(
                        FILE_NAMESPACE_SORT_PREFIX,
                        repo_path.as_ref().clone(),
                    );
                    if let Some(position) = multibuffer.read(cx).location_for_path(&path_key, cx) {
                        editor.update(cx, |editor, cx| {
                            editor.rhs_editor().update(cx, |rhs_editor, cx| {
                                rhs_editor.change_selections(
                                    SelectionEffects::scroll(Autoscroll::focused()),
                                    window,
                                    cx,
                                    |s| {
                                        s.select_ranges([position..position]);
                                    },
                                );
                            });
                        });
                    }
                })
                .render(window, cx)
        });

        h_flex()
            .key_context(if is_stash { "StashDiff" } else { "CommitDiff" })
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .when(!self.multibuffer.read(cx).is_empty(), |this| {
                this.child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .h_full()
                        .overflow_hidden()
                        .child(self.editor.clone()),
                )
            })
            .children(sidebar)
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(commit_view) = self.commit_view.as_ref().and_then(|cv| cv.upgrade()) else {
            return div().into_any_element();
        };

        let (sha, author_name, author_email, remote, summary, show_details_sidebar) = {
            let commit_view_ref = commit_view.read(cx);
            (
                commit_view_ref.commit.sha.clone(),
                commit_view_ref.commit.author_name.clone(),
                commit_view_ref.commit.author_email.clone(),
                commit_view_ref.remote.clone(),
                commit_view_ref
                    .commit
                    .message
                    .split('\n')
                    .next()
                    .unwrap_or("")
                    .to_string(),
                commit_view_ref.show_details_sidebar,
            )
        };

        let short_sha: SharedString = sha.chars().take(7).collect::<String>().into();
        let summary: SharedString = summary.into();

        let avatar_element =
            if let Some(remote) = remote.as_ref().filter(|r| r.host_supports_avatars()) {
                let asset = CommitAvatarAsset::new(remote.clone(), sha.clone(), Some(author_email));
                if let Some(Some(url)) = window.use_asset::<CommitAvatarAsset>(&asset, cx) {
                    Avatar::new(url.to_string()).into_any_element()
                } else {
                    Icon::new(IconName::Person)
                        .color(Color::Muted)
                        .size(IconSize::Small)
                        .into_any_element()
                }
            } else {
                Icon::new(IconName::Person)
                    .color(Color::Muted)
                    .size(IconSize::Small)
                    .into_any_element()
            };

        h_flex()
            .id("commit-view-toolbar")
            .pl_3()
            .gap_2()
            .items_center()
            .flex_grow()
            .justify_between()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .min_w_0()
                    .child(avatar_element)
                    .child(Label::new(author_name).color(Color::Default))
                    .child(
                        ButtonLike::new("commit-summary")
                            .child(
                                Label::new(summary)
                                    .color(Color::Muted)
                                    .single_line()
                                    .truncate(),
                            )
                            .on_click(|_, window, cx| {
                                window
                                    .dispatch_action(ToggleCommitDetailsSidebar.boxed_clone(), cx);
                            }),
                    ),
            )
            .child(
                h_flex()
                    .gap_1()
                    .flex_shrink_0()
                    .child(Label::new(short_sha).color(Color::Muted).buffer_font(cx))
                    .child(
                        CopyButton::new("copy-commit-sha", sha.to_string())
                            .tooltip_label("Copy SHA"),
                    )
                    .child(
                        IconButton::new("toggle-details-sidebar", IconName::Info)
                            .icon_size(IconSize::Small)
                            .toggle_state(show_details_sidebar)
                            .tooltip(Tooltip::text("Toggle Commit Details"))
                            .on_click(|_, window, cx| {
                                window
                                    .dispatch_action(ToggleCommitDetailsSidebar.boxed_clone(), cx);
                            }),
                    ),
            )
            .into_any_element()
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
            return ToolbarItemLocation::PrimaryLeft;
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
