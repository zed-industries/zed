use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use file_icons::FileIcons;
use fuzzy::PathMatch;
use gpui::{
    AppContext, DismissEvent, FocusHandle, FocusableView, Stateful, Task, View, WeakModel, WeakView,
};
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, ProjectPath, WorktreeId};
use ui::{prelude::*, ListItem, Tooltip};
use util::ResultExt as _;
use workspace::Workspace;

use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::{ContextStore, FileInclusion};

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

        let project_path = ProjectPath {
            worktree_id: WorktreeId::from_usize(mat.worktree_id),
            path: mat.path.clone(),
        };

        let Some(task) = self
            .context_store
            .update(cx, |context_store, cx| {
                context_store.add_file_from_path(project_path, cx)
            })
            .ok()
        else {
            return;
        };

        let workspace = self.workspace.clone();
        let confirm_behavior = self.confirm_behavior;
        cx.spawn(|this, mut cx| async move {
            match task.await {
                Ok(()) => {
                    this.update(&mut cx, |this, cx| match confirm_behavior {
                        ConfirmBehavior::KeepOpen => {}
                        ConfirmBehavior::Close => this.delegate.dismissed(cx),
                    })?;
                }
                Err(err) => {
                    let Some(workspace) = workspace.upgrade() else {
                        return anyhow::Ok(());
                    };

                    workspace.update(&mut cx, |workspace, cx| {
                        workspace.show_error(&err, cx);
                    })?;
                }
            }

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.context_picker
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let path_match = &self.matches[ix];

        Some(
            ListItem::new(ix)
                .inset(true)
                .toggle_state(selected)
                .child(render_file_context_entry(
                    ElementId::NamedInteger("file-ctx-picker".into(), ix),
                    &path_match.path,
                    &path_match.path_prefix,
                    self.context_store.clone(),
                    cx,
                )),
        )
    }
}

pub fn render_file_context_entry(
    id: ElementId,
    path: &Path,
    path_prefix: &Arc<str>,
    context_store: WeakModel<ContextStore>,
    cx: &WindowContext,
) -> Stateful<Div> {
    let (file_name, directory) = if path == Path::new("") {
        (SharedString::from(path_prefix.clone()), None)
    } else {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
            .into();

        let mut directory = format!("{}/", path_prefix);

        if let Some(parent) = path.parent().filter(|parent| parent != &Path::new("")) {
            directory.push_str(&parent.to_string_lossy());
            directory.push('/');
        }

        (file_name, Some(directory))
    };

    let added = context_store
        .upgrade()
        .and_then(|context_store| context_store.read(cx).will_include_file_path(path, cx));

    let file_icon = FileIcons::get_icon(&path, cx)
        .map(Icon::from_path)
        .unwrap_or_else(|| Icon::new(IconName::File));

    h_flex()
        .id(id)
        .gap_1()
        .w_full()
        .child(file_icon.size(IconSize::Small))
        .child(
            h_flex()
                .gap_2()
                .child(Label::new(file_name))
                .children(directory.map(|directory| {
                    Label::new(directory)
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                })),
        )
        .child(div().w_full())
        .when_some(added, |el, added| match added {
            FileInclusion::Direct(_) => el.child(
                h_flex()
                    .gap_1()
                    .child(
                        Icon::new(IconName::Check)
                            .size(IconSize::Small)
                            .color(Color::Success),
                    )
                    .child(Label::new("Added").size(LabelSize::Small)),
            ),
            FileInclusion::InDirectory(dir_name) => {
                let dir_name = dir_name.to_string_lossy().into_owned();

                el.child(
                    h_flex()
                        .gap_1()
                        .child(
                            Icon::new(IconName::Check)
                                .size(IconSize::Small)
                                .color(Color::Success),
                        )
                        .child(Label::new("Included").size(LabelSize::Small)),
                )
                .tooltip(move |cx| Tooltip::text(format!("in {dir_name}"), cx))
            }
        })
}
