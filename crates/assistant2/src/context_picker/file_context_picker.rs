use std::fmt::Write as _;
use std::ops::RangeInclusive;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use fuzzy::PathMatch;
use gpui::{AppContext, DismissEvent, FocusHandle, FocusableView, Task, View, WeakModel, WeakView};
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, ProjectPath, WorktreeId};
use ui::{prelude::*, ListItem};
use util::ResultExt as _;
use workspace::Workspace;

use crate::context::ContextKind;
use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::ContextStore;

pub struct FileContextPicker {
    picker: View<Picker<FileContextPickerDelegate>>,
}

impl FileContextPicker {
    pub fn new(
        context_picker: WeakView<ContextPicker>,
        workspace: WeakView<Workspace>,
        context_store: WeakModel<ContextStore>,
        confirm_behavior: ConfirmBehavior,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let delegate = FileContextPickerDelegate::new(
            context_picker,
            workspace,
            context_store,
            confirm_behavior,
        );
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
    context_store: WeakModel<ContextStore>,
    confirm_behavior: ConfirmBehavior,
    matches: Vec<PathMatch>,
    selected_index: usize,
}

impl FileContextPickerDelegate {
    pub fn new(
        context_picker: WeakView<ContextPicker>,
        workspace: WeakView<Workspace>,
        context_store: WeakModel<ContextStore>,
        confirm_behavior: ConfirmBehavior,
    ) -> Self {
        Self {
            context_picker,
            workspace,
            context_store,
            confirm_behavior,
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
            let recent_matches = workspace
                .recent_navigation_history(Some(10), cx)
                .into_iter()
                .filter_map(|(project_path, _)| {
                    let worktree = project.worktree_for_id(project_path.worktree_id, cx)?;
                    Some(PathMatch {
                        score: 0.,
                        positions: Vec::new(),
                        worktree_id: project_path.worktree_id.to_usize(),
                        path: project_path.path,
                        path_prefix: worktree.read(cx).root_name().into(),
                        distance_to_relative_ancestor: 0,
                        is_dir: false,
                    })
                });

            let file_matches = project.worktrees(cx).flat_map(|worktree| {
                let worktree = worktree.read(cx);
                let path_prefix: Arc<str> = worktree.root_name().into();
                worktree.files(true, 0).map(move |entry| PathMatch {
                    score: 0.,
                    positions: Vec::new(),
                    worktree_id: worktree.id().to_usize(),
                    path: entry.path.clone(),
                    path_prefix: path_prefix.clone(),
                    distance_to_relative_ancestor: 0,
                    is_dir: false,
                })
            });

            Task::ready(recent_matches.chain(file_matches).collect())
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
        let Some(mat) = self.matches.get(self.selected_index) else {
            return;
        };

        let workspace = self.workspace.clone();
        let Some(project) = workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).project().clone())
        else {
            return;
        };
        let path = mat.path.clone();
        let worktree_id = WorktreeId::from_usize(mat.worktree_id);
        let confirm_behavior = self.confirm_behavior;
        cx.spawn(|this, mut cx| async move {
            let Some((entry_id, open_buffer_task)) = project
                .update(&mut cx, |project, cx| {
                    let project_path = ProjectPath {
                        worktree_id,
                        path: path.clone(),
                    };

                    let entry_id = project.entry_for_path(&project_path, cx)?.id;
                    let task = project.open_buffer(project_path, cx);

                    Some((entry_id, task))
                })
                .ok()
                .flatten()
            else {
                return anyhow::Ok(());
            };

            let buffer = open_buffer_task.await?;

            this.update(&mut cx, |this, cx| {
                this.delegate
                    .context_store
                    .update(cx, |context_store, cx| {
                        let mut text = String::new();
                        text.push_str(&codeblock_fence_for_path(Some(&path), None));
                        text.push_str(&buffer.read(cx).text());
                        if !text.ends_with('\n') {
                            text.push('\n');
                        }

                        text.push_str("```\n");

                        context_store.insert_context(
                            ContextKind::File(entry_id),
                            path.to_string_lossy().to_string(),
                            text,
                        );
                    })?;

                match confirm_behavior {
                    ConfirmBehavior::KeepOpen => {}
                    ConfirmBehavior::Close => this.delegate.dismissed(cx),
                }

                anyhow::Ok(())
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

        let (file_name, directory) = if path_match.path.as_ref() == Path::new("") {
            (SharedString::from(path_match.path_prefix.clone()), None)
        } else {
            let file_name = path_match
                .path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                .into();

            let mut directory = format!("{}/", path_match.path_prefix);
            if let Some(parent) = path_match
                .path
                .parent()
                .filter(|parent| parent != &Path::new(""))
            {
                directory.push_str(&parent.to_string_lossy());
                directory.push('/');
            }

            (file_name, Some(directory))
        };

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

pub(crate) fn codeblock_fence_for_path(
    path: Option<&Path>,
    row_range: Option<RangeInclusive<u32>>,
) -> String {
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
