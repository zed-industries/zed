use anyhow::{Context as _, Result};
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, EditorEvent, MultiBuffer, SelectionEffects, multibuffer_context_lines};
use git::repository::{CommitDetails, CommitDiff, RepoPath};
use gpui::{
    Action, AnyElement, AnyView, App, AppContext as _, AsyncApp, AsyncWindowContext, Context,
    Entity, EventEmitter, FocusHandle, Focusable, IntoElement, PromptLevel, Render, WeakEntity,
    Window, actions,
};
use language::{
    Anchor, Buffer, Capability, DiskState, File, LanguageRegistry, LineEnding, OffsetRangeExt as _,
    Point, ReplicaId, Rope, TextBuffer,
};
use multi_buffer::PathKey;
use project::{Project, WorktreeId, git_store::Repository};
use std::{
    any::{Any, TypeId},
    fmt::Write as _,
    path::PathBuf,
    sync::Arc,
};
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

use crate::git_panel::GitPanel;

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
                    selections.select_ranges(vec![0..0]);
                });
            });
        }

        cx.spawn(async move |this, cx| {
            for file in commit_diff.files {
                let is_deleted = file.new_text.is_none();
                let new_text = file.new_text.unwrap_or_default();
                let old_text = file.old_text;
                let worktree_id = repository
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
        }
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
        &self.path.0
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
    writeln!(
        &mut result,
        "Date:   {}",
        time_format::format_local_timestamp(
            time::OffsetDateTime::from_unix_timestamp(commit.commit_timestamp).unwrap(),
            time::OffsetDateTime::now_utc(),
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

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>>
    where
        Self: Sized,
    {
        Some(cx.new(|cx| {
            let editor = cx.new(|cx| {
                self.editor
                    .update(cx, |editor, cx| editor.clone(window, cx))
            });
            let multibuffer = editor.read(cx).buffer().clone();
            Self {
                editor,
                multibuffer,
                commit: self.commit.clone(),
                stash: self.stash,
            }
        }))
    }
}

impl Render for CommitView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_stash = self.stash.is_some();
        div()
            .key_context(if is_stash { "StashDiff" } else { "CommitDiff" })
            .bg(cx.theme().colors().editor_background)
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .child(self.editor.clone())
    }
}

pub struct CommitViewToolbar {
    commit_view: Option<WeakEntity<CommitView>>,
    workspace: WeakEntity<Workspace>,
}

impl CommitViewToolbar {
    pub fn new(workspace: &Workspace, _: &mut Context<Self>) -> Self {
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

impl Render for CommitViewToolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(commit_view) = self.commit_view(cx) else {
            return div();
        };

        let is_stash = commit_view.read(cx).stash.is_some();
        if !is_stash {
            return div();
        }

        let focus_handle = commit_view.focus_handle(cx);

        h_group_xl().my_neg_1().py_1().items_center().child(
            h_group_sm()
                .child(
                    Button::new("apply-stash", "Apply")
                        .tooltip(Tooltip::for_action_title_in(
                            "Apply current stash",
                            &ApplyCurrentStash,
                            &focus_handle,
                        ))
                        .on_click(cx.listener(|this, _, window, cx| this.apply_stash(window, cx))),
                )
                .child(
                    Button::new("pop-stash", "Pop")
                        .tooltip(Tooltip::for_action_title_in(
                            "Pop current stash",
                            &PopCurrentStash,
                            &focus_handle,
                        ))
                        .on_click(cx.listener(|this, _, window, cx| this.pop_stash(window, cx))),
                )
                .child(
                    Button::new("remove-stash", "Remove")
                        .icon(IconName::Trash)
                        .tooltip(Tooltip::for_action_title_in(
                            "Remove current stash",
                            &DropCurrentStash,
                            &focus_handle,
                        ))
                        .on_click(cx.listener(|this, _, window, cx| this.remove_stash(window, cx))),
                ),
        )
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
