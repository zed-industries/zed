use std::fmt::Write as _;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use fuzzy::PathMatch;
use gpui::{AppContext, DismissEvent, FocusHandle, FocusableView, Task, View, WeakView};
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, WorktreeId};
use ui::{prelude::*, ListItem};
use util::ResultExt as _;
use workspace::Workspace;

use crate::context::ContextKind;
use crate::context_picker::ContextPicker;
use crate::message_editor::MessageEditor;

pub struct FileContextPicker {
    picker: View<Picker<FileContextPickerDelegate>>,
}

impl FileContextPicker {
    pub fn new(
        context_picker: WeakView<ContextPicker>,
        workspace: WeakView<Workspace>,
        message_editor: WeakView<MessageEditor>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let delegate = FileContextPickerDelegate::new(context_picker, workspace, message_editor);
        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));

        Self { picker }
    }
}

impl FocusableView for FileContextPicker {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for FileContextPicker {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

pub struct FileContextPickerDelegate {
    context_picker: WeakView<ContextPicker>,
    workspace: WeakView<Workspace>,
    message_editor: WeakView<MessageEditor>,
    matches: Vec<PathMatch>,
    selected_index: usize,
}

impl FileContextPickerDelegate {
    pub fn new(
        context_picker: WeakView<ContextPicker>,
        workspace: WeakView<Workspace>,
        message_editor: WeakView<MessageEditor>,
    ) -> Self {
        Self {
            context_picker,
            workspace,
            message_editor,
            matches: Vec::new(),
            selected_index: 0,
        }
    }

    fn search(
        &mut self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        workspace: &View<Workspace>,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Task<Vec<PathMatch>> {
        if query.is_empty() {
            let workspace = workspace.read(cx);
            let project = workspace.project().read(cx);
            let entries = workspace.recent_navigation_history(Some(10), cx);

            let entries = entries
                .into_iter()
                .map(|entries| entries.0)
                .chain(project.worktrees(cx).flat_map(|worktree| {
                    let worktree = worktree.read(cx);
                    let id = worktree.id();
                    worktree
                        .child_entries(Path::new(""))
                        .filter(|entry| entry.kind.is_file())
                        .map(move |entry| project::ProjectPath {
                            worktree_id: id,
                            path: entry.path.clone(),
                        })
                }))
                .collect::<Vec<_>>();

            let path_prefix: Arc<str> = Arc::default();
            Task::ready(
                entries
                    .into_iter()
                    .filter_map(|entry| {
                        let worktree = project.worktree_for_id(entry.worktree_id, cx)?;
                        let mut full_path = PathBuf::from(worktree.read(cx).root_name());
                        full_path.push(&entry.path);
                        Some(PathMatch {
                            score: 0.,
                            positions: Vec::new(),
                            worktree_id: entry.worktree_id.to_usize(),
                            path: full_path.into(),
                            path_prefix: path_prefix.clone(),
                            distance_to_relative_ancestor: 0,
                            is_dir: false,
                        })
                    })
                    .collect(),
            )
        } else {
            let worktrees = workspace.read(cx).visible_worktrees(cx).collect::<Vec<_>>();
            let candidate_sets = worktrees
                .into_iter()
                .map(|worktree| {
                    let worktree = worktree.read(cx);

                    PathMatchCandidateSet {
                        snapshot: worktree.snapshot(),
                        include_ignored: worktree
                            .root_entry()
                            .map_or(false, |entry| entry.is_ignored),
                        include_root_name: true,
                        candidates: project::Candidates::Files,
                    }
                })
                .collect::<Vec<_>>();

            let executor = cx.background_executor().clone();
            cx.foreground_executor().spawn(async move {
                fuzzy::match_path_sets(
                    candidate_sets.as_slice(),
                    query.as_str(),
                    None,
                    false,
                    100,
                    &cancellation_flag,
                    executor,
                )
                .await
            })
        }
    }
}

impl PickerDelegate for FileContextPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search files…".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(());
        };

        let search_task = self.search(query, Arc::<AtomicBool>::default(), &workspace, cx);

        cx.spawn(|this, mut cx| async move {
            // TODO: This should be probably be run in the background.
            let paths = search_task.await;

            this.update(&mut cx, |this, _cx| {
                this.delegate.matches = paths;
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        let mat = &self.matches[self.selected_index];

        let workspace = self.workspace.clone();
        let Some(project) = workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).project().clone())
        else {
            return;
        };
        let path = mat.path.clone();
        let worktree_id = WorktreeId::from_usize(mat.worktree_id);
        cx.spawn(|this, mut cx| async move {
            let Some(open_buffer_task) = project
                .update(&mut cx, |project, cx| {
                    project.open_buffer((worktree_id, path.clone()), cx)
                })
                .ok()
            else {
                return anyhow::Ok(());
            };

            let buffer = open_buffer_task.await?;

            this.update(&mut cx, |this, cx| {
                this.delegate
                    .message_editor
                    .update(cx, |message_editor, cx| {
                        let mut text = String::new();
                        text.push_str(&codeblock_fence_for_path(Some(&path), None));
                        text.push_str(&buffer.read(cx).text());
                        if !text.ends_with('\n') {
                            text.push('\n');
                        }

                        text.push_str("```\n");

                        message_editor.insert_context(
                            ContextKind::File,
                            path.to_string_lossy().to_string(),
                            text,
                        );
                    })
            })??;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.context_picker
            .update(cx, |this, cx| {
                this.reset_mode();
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let path_match = &self.matches[ix];
        let file_name = path_match
            .path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let directory = path_match
            .path
            .parent()
            .map(|directory| format!("{}/", directory.to_string_lossy()));

        Some(
            ListItem::new(ix).inset(true).toggle_state(selected).child(
                h_flex()
                    .gap_2()
                    .child(Label::new(file_name))
                    .children(directory.map(|directory| {
                        Label::new(directory)
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    })),
            ),
        )
    }
}

fn codeblock_fence_for_path(path: Option<&Path>, row_range: Option<RangeInclusive<u32>>) -> String {
    let mut text = String::new();
    write!(text, "```").unwrap();

    if let Some(path) = path {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            write!(text, "{} ", extension).unwrap();
        }

        write!(text, "{}", path.display()).unwrap();
    } else {
        write!(text, "untitled").unwrap();
    }

    if let Some(row_range) = row_range {
        write!(text, ":{}-{}", row_range.start() + 1, row_range.end() + 1).unwrap();
    }

    text.push('\n');
    text
}
