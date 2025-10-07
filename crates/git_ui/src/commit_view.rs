use anyhow::{Context as _, Result};
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, EditorEvent, MultiBuffer, SelectionEffects, multibuffer_context_lines};
use git::repository::{CommitDetails, CommitDiff, CommitSummary, RepoPath};
use gpui::{
    AnyElement, AnyView, App, AppContext as _, AsyncApp, Context, Entity, EventEmitter,
    FocusHandle, Focusable, IntoElement, Render, WeakEntity, Window,
};
use language::{
    Anchor, Buffer, Capability, DiskState, File, LanguageRegistry, LineEnding, OffsetRangeExt as _,
    Point, Rope, TextBuffer,
};
use multi_buffer::PathKey;
use project::{Project, WorktreeId, git_store::Repository};
use std::{
    any::{Any, TypeId},
    fmt::Write as _,
    path::PathBuf,
    sync::Arc,
};
use ui::{Color, Icon, IconName, Label, LabelCommon as _, SharedString};
use util::{ResultExt, paths::PathStyle, rel_path::RelPath, truncate_and_trailoff};
use workspace::{
    Item, ItemHandle as _, ItemNavHistory, ToolbarItemLocation, Workspace,
    item::{BreadcrumbText, ItemEvent, TabContentParams},
    searchable::SearchableItemHandle,
};

pub struct CommitView {
    commit: CommitDetails,
    editor: Entity<Editor>,
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

const COMMIT_METADATA_NAMESPACE: u64 = 0;
const FILE_NAMESPACE: u64 = 1;

impl CommitView {
    pub fn open(
        commit: CommitSummary,
        repo: WeakEntity<Repository>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let commit_diff = repo
            .update(cx, |repo, _| repo.load_commit_diff(commit.sha.to_string()))
            .ok();
        let commit_details = repo
            .update(cx, |repo, _| repo.show(commit.sha.to_string()))
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
                                window,
                                cx,
                            )
                        });

                        let pane = workspace.active_pane();
                        pane.update(cx, |pane, cx| {
                            let ix = pane.items().position(|item| {
                                let commit_view = item.downcast::<CommitView>();
                                commit_view
                                    .is_some_and(|view| view.read(cx).commit.sha == commit.sha)
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
            let file = Arc::new(CommitMetadataFile {
                title: RelPath::unix(&format!("commit {}", commit.sha))
                    .unwrap()
                    .into(),
                worktree_id,
            });
            let buffer = cx.new(|cx| {
                let buffer = TextBuffer::new_normalized(
                    0,
                    cx.entity_id().as_non_zero_u64().into(),
                    LineEnding::default(),
                    format_commit(&commit).into(),
                );
                metadata_buffer_id = Some(buffer.remote_id());
                Buffer::build(buffer, Some(file.clone()), Capability::ReadWrite)
            });
            multibuffer.update(cx, |multibuffer, cx| {
                multibuffer.set_excerpts_for_path(
                    PathKey::namespaced(COMMIT_METADATA_NAMESPACE, file.title.clone()),
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
                            PathKey::namespaced(FILE_NAMESPACE, path),
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
            0,
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

fn format_commit(commit: &CommitDetails) -> String {
    let mut result = String::new();
    writeln!(&mut result, "commit {}", commit.sha).unwrap();
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
            }
        }))
    }
}

impl Render for CommitView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.editor.clone()
    }
}
