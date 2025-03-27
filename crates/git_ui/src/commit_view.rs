use anyhow::{anyhow, Result};
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, EditorEvent, MultiBuffer};
use git::repository::{CommitDiff, CommitSummary, RepoPath};
use gpui::{
    AnyElement, App, AppContext as _, AsyncApp, AsyncWindowContext, Context, Entity, EventEmitter,
    FocusHandle, Focusable, IntoElement, Render, WeakEntity, Window,
};
use language::{
    Anchor, Buffer, Capability, DiskState, File, LanguageRegistry, LineEnding, OffsetRangeExt as _,
    Rope, TextBuffer,
};
use multi_buffer::PathKey;
use project::{git_store::Repository, Project, WorktreeId};
use std::{
    any::Any,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};
use ui::{Color, Icon, IconName, Label, LabelCommon as _};
use util::ResultExt;
use workspace::{item::TabContentParams, Item, Workspace};

pub struct CommitView {
    commit: CommitSummary,
    editor: Entity<Editor>,
    multibuffer: Entity<MultiBuffer>,
}

struct GitBlob {
    path: RepoPath,
    worktree_id: WorktreeId,
    is_deleted: bool,
}

impl CommitView {
    pub fn open(
        commit: CommitSummary,
        commit_diff: CommitDiff,
        repo: WeakEntity<Repository>,
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        workspace.update_in(cx, |workspace, window, cx| {
            let repo = repo.upgrade().ok_or_else(|| anyhow!("repo removed"))?;
            let project = workspace.project();
            let commit_view = cx
                .new(|cx| CommitView::new(commit, commit_diff, repo, project.clone(), window, cx));
            workspace.add_item_to_center(Box::new(commit_view), window, cx);
            anyhow::Ok(())
        })?
    }

    pub fn new(
        commit: CommitSummary,
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

        cx.spawn(async move |this, mut cx| {
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
                    .ok_or_else(|| anyhow!("project has no worktrees"))?;
                let file = Arc::new(GitBlob {
                    path: file.path.clone(),
                    is_deleted,
                    worktree_id,
                }) as Arc<dyn language::File>;

                let buffer = build_buffer(new_text, file, &language_registry, &mut cx).await?;
                let buffer_diff =
                    build_buffer_diff(old_text, &buffer, &language_registry, &mut cx).await?;

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
                            PathKey::namespaced("", path),
                            buffer,
                            diff_hunk_ranges,
                            editor::DEFAULT_MULTIBUFFER_CONTEXT,
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

    fn path(&self) -> &Arc<Path> {
        &self.path.0
    }

    fn full_path(&self, _: &App) -> PathBuf {
        self.path.to_path_buf()
    }

    fn file_name<'a>(&'a self, _: &'a App) -> &'a OsStr {
        self.path.file_name().unwrap()
    }

    fn worktree_id(&self, _: &App) -> WorktreeId {
        self.worktree_id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_proto(&self, _cx: &App) -> language::proto::File {
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
        diff.set_snapshot(diff_snapshot, &buffer.text, None, cx);
        diff
    })
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

    fn tab_content(&self, params: TabContentParams, _window: &Window, _: &App) -> AnyElement {
        Label::new(format!("Commit '{}'", self.commit.subject))
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
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
}

impl Render for CommitView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.editor.clone()
    }
}
