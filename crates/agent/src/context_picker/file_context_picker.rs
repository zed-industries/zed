use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use file_icons::FileIcons;
use fuzzy::PathMatch;
use gpui::{
    App, AppContext, DismissEvent, Entity, FocusHandle, Focusable, Stateful, Task, WeakEntity,
};
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, ProjectPath, WorktreeId};
use ui::{ListItem, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::Workspace;

use crate::context_picker::ContextPicker;
use crate::context_store::{ContextStore, FileInclusion};

pub struct FileContextPicker {
    picker: Entity<Picker<FileContextPickerDelegate>>,
}

impl FileContextPicker {
    pub fn new(
        context_picker: WeakEntity<ContextPicker>,
        workspace: WeakEntity<Workspace>,
        context_store: WeakEntity<ContextStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = FileContextPickerDelegate::new(context_picker, workspace, context_store);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        Self { picker }
    }
}

impl Focusable for FileContextPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for FileContextPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

pub struct FileContextPickerDelegate {
    context_picker: WeakEntity<ContextPicker>,
    workspace: WeakEntity<Workspace>,
    context_store: WeakEntity<ContextStore>,
    matches: Vec<FileMatch>,
    selected_index: usize,
}

impl FileContextPickerDelegate {
    pub fn new(
        context_picker: WeakEntity<ContextPicker>,
        workspace: WeakEntity<Workspace>,
        context_store: WeakEntity<ContextStore>,
    ) -> Self {
        Self {
            context_picker,
            workspace,
            context_store,
            matches: Vec::new(),
            selected_index: 0,
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

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search files & directories…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(());
        };

        let search_task = search_files(query, Arc::<AtomicBool>::default(), &workspace, cx);

        cx.spawn_in(window, async move |this, cx| {
            // TODO: This should be probably be run in the background.
            let paths = search_task.await;

            this.update(cx, |this, _cx| {
                this.delegate.matches = paths;
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(FileMatch { mat, .. }) = self.matches.get(self.selected_index) else {
            return;
        };

        let project_path = ProjectPath {
            worktree_id: WorktreeId::from_usize(mat.worktree_id),
            path: mat.path.clone(),
        };

        let is_directory = mat.is_dir;

        let Some(task) = self
            .context_store
            .update(cx, |context_store, cx| {
                if is_directory {
                    Task::ready(context_store.add_directory(&project_path, true, cx))
                } else {
                    context_store.add_file_from_path(project_path.clone(), true, cx)
                }
            })
            .ok()
        else {
            return;
        };

        task.detach_and_log_err(cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
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
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let FileMatch { mat, .. } = &self.matches[ix];

        Some(
            ListItem::new(ix)
                .inset(true)
                .toggle_state(selected)
                .child(render_file_context_entry(
                    ElementId::NamedInteger("file-ctx-picker".into(), ix),
                    WorktreeId::from_usize(mat.worktree_id),
                    &mat.path,
                    &mat.path_prefix,
                    mat.is_dir,
                    self.context_store.clone(),
                    cx,
                )),
        )
    }
}

pub struct FileMatch {
    pub mat: PathMatch,
    pub is_recent: bool,
}

pub(crate) fn search_files(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    workspace: &Entity<Workspace>,
    cx: &App,
) -> Task<Vec<FileMatch>> {
    if query.is_empty() {
        let workspace = workspace.read(cx);
        let project = workspace.project().read(cx);
        let recent_matches = workspace
            .recent_navigation_history(Some(10), cx)
            .into_iter()
            .filter_map(|(project_path, _)| {
                let worktree = project.worktree_for_id(project_path.worktree_id, cx)?;
                Some(FileMatch {
                    mat: PathMatch {
                        score: 0.,
                        positions: Vec::new(),
                        worktree_id: project_path.worktree_id.to_usize(),
                        path: project_path.path,
                        path_prefix: worktree.read(cx).root_name().into(),
                        distance_to_relative_ancestor: 0,
                        is_dir: false,
                    },
                    is_recent: true,
                })
            });

        let file_matches = project.worktrees(cx).flat_map(|worktree| {
            let worktree = worktree.read(cx);
            let path_prefix: Arc<str> = worktree.root_name().into();
            worktree.entries(false, 0).map(move |entry| FileMatch {
                mat: PathMatch {
                    score: 0.,
                    positions: Vec::new(),
                    worktree_id: worktree.id().to_usize(),
                    path: entry.path.clone(),
                    path_prefix: path_prefix.clone(),
                    distance_to_relative_ancestor: 0,
                    is_dir: entry.is_dir(),
                },
                is_recent: false,
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
                    candidates: project::Candidates::Entries,
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
            .into_iter()
            .map(|mat| FileMatch {
                mat,
                is_recent: false,
            })
            .collect::<Vec<_>>()
        })
    }
}

pub fn extract_file_name_and_directory(
    path: &Path,
    path_prefix: &str,
) -> (SharedString, Option<SharedString>) {
    if path == Path::new("") {
        (
            SharedString::from(
                path_prefix
                    .trim_end_matches(std::path::MAIN_SEPARATOR)
                    .to_string(),
            ),
            None,
        )
    } else {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
            .into();

        let mut directory = path_prefix
            .trim_end_matches(std::path::MAIN_SEPARATOR)
            .to_string();
        if !directory.ends_with('/') {
            directory.push('/');
        }
        if let Some(parent) = path.parent().filter(|parent| parent != &Path::new("")) {
            directory.push_str(&parent.to_string_lossy());
            directory.push('/');
        }

        (file_name, Some(directory.into()))
    }
}

pub fn render_file_context_entry(
    id: ElementId,
    worktree_id: WorktreeId,
    path: &Arc<Path>,
    path_prefix: &Arc<str>,
    is_directory: bool,
    context_store: WeakEntity<ContextStore>,
    cx: &App,
) -> Stateful<Div> {
    let (file_name, directory) = extract_file_name_and_directory(&path, path_prefix);

    let added = context_store.upgrade().and_then(|context_store| {
        let project_path = ProjectPath {
            worktree_id,
            path: path.clone(),
        };
        if is_directory {
            context_store
                .read(cx)
                .path_included_in_directory(&project_path, cx)
        } else {
            context_store.read(cx).file_path_included(&project_path, cx)
        }
    });

    let file_icon = if is_directory {
        FileIcons::get_folder_icon(false, cx)
    } else {
        FileIcons::get_icon(&path, cx)
    }
    .map(Icon::from_path)
    .unwrap_or_else(|| Icon::new(IconName::File));

    h_flex()
        .id(id)
        .gap_1p5()
        .w_full()
        .child(file_icon.size(IconSize::Small).color(Color::Muted))
        .child(
            h_flex()
                .gap_1()
                .child(Label::new(file_name))
                .children(directory.map(|directory| {
                    Label::new(directory)
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                })),
        )
        .when_some(added, |el, added| match added {
            FileInclusion::Direct => el.child(
                h_flex()
                    .w_full()
                    .justify_end()
                    .gap_0p5()
                    .child(
                        Icon::new(IconName::Check)
                            .size(IconSize::Small)
                            .color(Color::Success),
                    )
                    .child(Label::new("Added").size(LabelSize::Small)),
            ),
            FileInclusion::InDirectory { full_path } => {
                let directory_full_path = full_path.to_string_lossy().into_owned();

                el.child(
                    h_flex()
                        .w_full()
                        .justify_end()
                        .gap_0p5()
                        .child(
                            Icon::new(IconName::Check)
                                .size(IconSize::Small)
                                .color(Color::Success),
                        )
                        .child(Label::new("Included").size(LabelSize::Small)),
                )
                .tooltip(Tooltip::text(format!("in {directory_full_path}")))
            }
        })
}
