use std::any::TypeId;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use editor::{Editor, EditorEvent, MultiBuffer, multibuffer_context_lines};
use fs::Fs;
use git::repository::RepoPath;
use gpui::{
    AnyElement, App, AppContext as _, AsyncWindowContext, Context, Entity, EventEmitter,
    FocusHandle, Focusable, IntoElement, Render, SharedString, WeakEntity, Window,
};
use language::{Capability, DiskState, File};
use multi_buffer::PathKey;
use project::WorktreeId;
use ui::{Color, Icon, IconName, Label, LabelCommon as _};
use util::ResultExt as _;
use util::paths::PathStyle;
use util::rel_path::{RelPath, RelPathBuf};
use workspace::item::{Item, TabContentParams};
use workspace::{ItemNavHistory, Workspace};

use crate::commit_view::{build_buffer, build_buffer_diff};

/// A read-only diff of one uncommitted file in a git worktree that is not open
/// in the project.
///
/// Modelled on [`crate::commit_view::CommitView`]: the buffer is synthesised
/// from file contents rather than opened through the project, so viewing a
/// file never adds a worktree, language server, or search root.
pub struct WorktreeDiffView {
    editor: Entity<Editor>,
    buffer: Entity<language::Buffer>,
    buffer_diff: Entity<buffer_diff::BufferDiff>,
    worktree_abs_path: PathBuf,
    repo_path: RepoPath,
    title: SharedString,
}

/// A [`language::File`] for a file that exists on disk but outside the
/// project.
///
/// `DiskState::Historic` is deliberate: the file is real, but Zed has no
/// worktree entry to write it back through, so the buffer must never be
/// treated as savable.
struct WorktreeBlob {
    path: Arc<RelPath>,
    worktree_id: WorktreeId,
    display_name: String,
    is_deleted: bool,
}

impl File for WorktreeBlob {
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
        &self.path
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
        true
    }
}

impl WorktreeDiffView {
    /// Opens the diff in the workspace's active pane, reusing an existing tab
    /// for the same file if one is already open.
    pub fn open(
        fs: Arc<dyn Fs>,
        worktree_abs_path: PathBuf,
        repo_path: RepoPath,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        window
            .spawn(cx, async move |cx| {
                Self::open_inner(fs, worktree_abs_path, repo_path, workspace, cx)
                    .await
                    .log_err()
            })
            .detach();
    }

    async fn open_inner(
        fs: Arc<dyn Fs>,
        worktree_abs_path: PathBuf,
        repo_path: RepoPath,
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        let file_abs_path = worktree_abs_path.join(repo_path.as_std_path());
        let (working_copy_text, committed_text) = cx
            .background_spawn({
                let fs = fs.clone();
                let worktree_abs_path = worktree_abs_path.clone();
                let repo_path = repo_path.clone();
                async move {
                    let dot_git_abs_path = worktree_abs_path.join(".git");
                    let system_git_binary_path = which::which("git").ok();
                    let repository = fs
                        .open_repo(&dot_git_abs_path, system_git_binary_path.as_deref())
                        .with_context(|| {
                            format!("opening git worktree at {dot_git_abs_path:?}")
                        })?;
                    let committed_text = repository.load_committed_text(repo_path).await;
                    // A file staged for deletion or removed from the working
                    // tree still has committed text worth showing.
                    let working_copy_text = fs.load(&file_abs_path).await.ok();
                    anyhow::Ok((working_copy_text, committed_text))
                }
            })
            .await?;

        anyhow::ensure!(
            working_copy_text.is_some() || committed_text.is_some(),
            "{repo_path:?} has neither working-tree nor committed contents"
        );
        let is_deleted = working_copy_text.is_none();

        let (language_registry, worktree_id) = workspace.read_with(cx, |workspace, cx| {
            let project = workspace.project().read(cx);
            (
                project.languages().clone(),
                project.worktrees(cx).next().map(|w| w.read(cx).id()),
            )
        })?;
        let worktree_id = worktree_id.context("project has no worktrees")?;

        let worktree_name = worktree_abs_path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| worktree_abs_path.to_string_lossy().into_owned());
        let file_name = repo_path
            .file_name()
            .map(|name| name.to_owned())
            .unwrap_or_else(|| repo_path.display(PathStyle::local()).into_owned());
        let title = SharedString::from(format!("{worktree_name} — {file_name}"));

        // Namespace the buffer's path by worktree so two worktrees' versions of
        // the same file are distinct excerpts and distinct tabs.
        let mut namespaced_path = RelPathBuf::new();
        namespaced_path.push_component(&worktree_name)?;
        namespaced_path.push(&repo_path);

        let blob = Arc::new(WorktreeBlob {
            path: namespaced_path.as_rel_path().into_arc(),
            worktree_id,
            display_name: title.to_string(),
            is_deleted,
        }) as Arc<dyn File>;

        let buffer = build_buffer(
            working_copy_text.unwrap_or_default(),
            blob,
            &language_registry,
            cx,
        )
        .await?;
        let buffer_diff = build_buffer_diff(committed_text, &buffer, &language_registry, cx).await?;

        workspace.update_in(cx, |workspace, window, cx| {
            let pane = workspace.active_pane().clone();
            let existing = pane.read(cx).items().find_map(|item| {
                let view = item.downcast::<Self>()?;
                let matches = {
                    let view = view.read(cx);
                    view.worktree_abs_path == worktree_abs_path && view.repo_path == repo_path
                };
                matches.then_some(view)
            });
            if let Some(existing) = existing {
                pane.update(cx, |pane, cx| {
                    pane.activate_item(
                        pane.index_for_item(&existing).unwrap_or_default(),
                        true,
                        true,
                        window,
                        cx,
                    );
                });
                return existing;
            }

            let project = workspace.project().clone();
            let view = cx.new(|cx| {
                let multibuffer = cx.new(|cx| {
                    let mut multibuffer = MultiBuffer::new(Capability::ReadOnly);
                    multibuffer.set_all_diff_hunks_expanded(cx);
                    multibuffer
                });
                let snapshot = buffer.read(cx).snapshot();
                let path = PathKey::for_buffer(&buffer, cx);
                let ranges = vec![language::Point::zero()..snapshot.max_point()];
                multibuffer.update(cx, |multibuffer, cx| {
                    multibuffer.set_excerpts_for_path(
                        path,
                        buffer.clone(),
                        ranges,
                        multibuffer_context_lines(cx),
                        cx,
                    );
                    multibuffer.add_diff(buffer_diff.clone(), cx);
                });

                let editor = cx.new(|cx| {
                    let mut editor =
                        Editor::for_multibuffer(multibuffer, Some(project), window, cx);
                    editor.set_read_only(true);
                    editor.set_expand_all_diff_hunks(cx);
                    editor
                });

                Self {
                    editor,
                    buffer: buffer.clone(),
                    buffer_diff: buffer_diff.clone(),
                    worktree_abs_path,
                    repo_path,
                    title,
                }
            });
            pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(view.clone()), true, true, None, window, cx);
            });
            view
        })
    }

    pub fn editor(&self) -> &Entity<Editor> {
        &self.editor
    }

    /// The synthesised buffer holding the file's working-tree contents.
    pub fn buffer(&self) -> &Entity<language::Buffer> {
        &self.buffer
    }

    /// The diff between the working-tree contents and the worktree's HEAD.
    pub fn buffer_diff(&self) -> &Entity<buffer_diff::BufferDiff> {
        &self.buffer_diff
    }
}

impl EventEmitter<EditorEvent> for WorktreeDiffView {}

impl Focusable for WorktreeDiffView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for WorktreeDiffView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.editor.clone()
    }
}

impl Item for WorktreeDiffView {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::GitWorktree).color(Color::Muted))
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, _cx: &App) -> AnyElement {
        Label::new(self.title.clone())
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        self.title.clone()
    }

    fn to_item_events(event: &EditorEvent, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Worktree Diff View Opened")
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _cx: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.clone().into())
        } else {
            None
        }
    }

    fn as_searchable(
        &self,
        _: &Entity<Self>,
        _: &App,
    ) -> Option<Box<dyn workspace::searchable::SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
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
        data: Arc<dyn std::any::Any + Send>,
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
}
