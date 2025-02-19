use std::collections::BTreeSet;
use std::ops::Range;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use editor::actions::FoldAt;
use editor::display_map::{Crease, FoldId};
use editor::scroll::Autoscroll;
use editor::{Anchor, AnchorRangeExt, Editor, FoldPlaceholder, ToPoint};
use file_icons::FileIcons;
use fuzzy::PathMatch;
use gpui::{
    AnyElement, App, AppContext, DismissEvent, Empty, Entity, FocusHandle, Focusable, Stateful,
    Task, WeakEntity,
};
use multi_buffer::{MultiBufferPoint, MultiBufferRow};
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, ProjectPath, WorktreeId};
use rope::Point;
use text::SelectionGoal;
use ui::{prelude::*, ButtonLike, Disclosure, ListItem, TintColor, Tooltip};
use util::ResultExt as _;
use workspace::{notifications::NotifyResultExt, Workspace};

use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::{ContextStore, FileInclusion};

pub struct FileContextPicker {
    picker: Entity<Picker<FileContextPickerDelegate>>,
}

impl FileContextPicker {
    pub fn new(
        context_picker: WeakEntity<ContextPicker>,
        workspace: WeakEntity<Workspace>,
        editor: WeakEntity<Editor>,
        context_store: WeakEntity<ContextStore>,
        confirm_behavior: ConfirmBehavior,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = FileContextPickerDelegate::new(
            context_picker,
            workspace,
            editor,
            context_store,
            confirm_behavior,
        );
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
    editor: WeakEntity<Editor>,
    context_store: WeakEntity<ContextStore>,
    confirm_behavior: ConfirmBehavior,
    matches: Vec<PathMatch>,
    selected_index: usize,
}

impl FileContextPickerDelegate {
    pub fn new(
        context_picker: WeakEntity<ContextPicker>,
        workspace: WeakEntity<Workspace>,
        editor: WeakEntity<Editor>,
        context_store: WeakEntity<ContextStore>,
        confirm_behavior: ConfirmBehavior,
    ) -> Self {
        Self {
            context_picker,
            workspace,
            editor,
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
        workspace: &Entity<Workspace>,

        cx: &mut Context<Picker<Self>>,
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
                worktree.files(false, 0).map(move |entry| PathMatch {
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

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search files…".into()
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

        let search_task = self.search(query, Arc::<AtomicBool>::default(), &workspace, cx);

        cx.spawn_in(window, |this, mut cx| async move {
            // TODO: This should be probably be run in the background.
            let paths = search_task.await;

            this.update(&mut cx, |this, _cx| {
                this.delegate.matches = paths;
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(mat) = self.matches.get(self.selected_index) else {
            return;
        };

        let Some(file_name) = mat
            .path
            .file_name()
            .map(|os_str| os_str.to_string_lossy().into_owned())
        else {
            return;
        };

        let full_path = mat.path.display().to_string();

        let project_path = ProjectPath {
            worktree_id: WorktreeId::from_usize(mat.worktree_id),
            path: mat.path.clone(),
        };

        let Some(editor_entity) = self.editor.upgrade() else {
            return;
        };

        editor_entity.update(cx, |editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                // Move empty selections left by 1 column to select the `@`s, so they get overwritten when we insert.
                {
                    let mut selections = editor.selections.all::<MultiBufferPoint>(cx);

                    for selection in selections.iter_mut() {
                        if selection.is_empty() {
                            let old_head = selection.head();
                            let new_head = MultiBufferPoint::new(
                                old_head.row,
                                old_head.column.saturating_sub(1),
                            );
                            selection.set_head(new_head, SelectionGoal::None);
                        }
                    }

                    editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                        s.select(selections)
                    });
                }

                let start_anchors = {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    editor
                        .selections
                        .all::<Point>(cx)
                        .into_iter()
                        .map(|selection| snapshot.anchor_before(selection.start))
                        .collect::<Vec<_>>()
                };

                editor.insert(&full_path, window, cx);

                let end_anchors = {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    editor
                        .selections
                        .all::<Point>(cx)
                        .into_iter()
                        .map(|selection| snapshot.anchor_after(selection.end))
                        .collect::<Vec<_>>()
                };

                editor.insert("\n", window, cx); // Needed to end the fold

                let file_icon = FileIcons::get_icon(&Path::new(&full_path), cx)
                    .unwrap_or_else(|| SharedString::new(""));

                let placeholder = FoldPlaceholder {
                    render: render_fold_icon_button(
                        file_icon,
                        file_name.into(),
                        editor_entity.downgrade(),
                    ),
                    ..Default::default()
                };

                let render_trailer =
                    move |_row, _unfold, _window: &mut Window, _cx: &mut App| Empty.into_any();

                let buffer = editor.buffer().read(cx).snapshot(cx);
                let mut rows_to_fold = BTreeSet::new();
                let crease_iter = start_anchors
                    .into_iter()
                    .zip(end_anchors)
                    .map(|(start, end)| {
                        rows_to_fold.insert(MultiBufferRow(start.to_point(&buffer).row));

                        Crease::inline(
                            start..end,
                            placeholder.clone(),
                            fold_toggle("tool-use"),
                            render_trailer,
                        )
                    });

                editor.insert_creases(crease_iter, cx);

                for buffer_row in rows_to_fold {
                    editor.fold_at(&FoldAt { buffer_row }, window, cx);
                }
            });
        });

        let Some(task) = self
            .context_store
            .update(cx, |context_store, cx| {
                context_store.add_file_from_path(project_path, cx)
            })
            .ok()
        else {
            return;
        };

        let confirm_behavior = self.confirm_behavior;
        cx.spawn_in(window, |this, mut cx| async move {
            match task.await.notify_async_err(&mut cx) {
                None => anyhow::Ok(()),
                Some(()) => this.update_in(&mut cx, |this, window, cx| match confirm_behavior {
                    ConfirmBehavior::KeepOpen => {}
                    ConfirmBehavior::Close => this.delegate.dismissed(window, cx),
                }),
            }
        })
        .detach_and_log_err(cx);
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
    context_store: WeakEntity<ContextStore>,
    cx: &App,
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
            FileInclusion::Direct(_) => el.child(
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
            FileInclusion::InDirectory(dir_name) => {
                let dir_name = dir_name.to_string_lossy().into_owned();

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
                .tooltip(Tooltip::text(format!("in {dir_name}")))
            }
        })
}

fn render_fold_icon_button(
    icon: SharedString,
    label: SharedString,
    editor: WeakEntity<Editor>,
) -> Arc<dyn Send + Sync + Fn(FoldId, Range<Anchor>, &mut App) -> AnyElement> {
    Arc::new(move |fold_id, fold_range, cx| {
        let is_in_text_selection = editor.upgrade().is_some_and(|editor| {
            editor.update(cx, |editor, cx| {
                let snapshot = editor
                    .buffer()
                    .update(cx, |multi_buffer, cx| multi_buffer.snapshot(cx));

                let is_in_pending_selection = || {
                    editor
                        .selections
                        .pending
                        .as_ref()
                        .is_some_and(|pending_selection| {
                            pending_selection
                                .selection
                                .range()
                                .includes(&fold_range, &snapshot)
                        })
                };

                let mut is_in_complete_selection = || {
                    editor
                        .selections
                        .disjoint_in_range::<usize>(fold_range.clone(), cx)
                        .into_iter()
                        .any(|selection| {
                            // This is needed to cover a corner case, if we just check for an existing
                            // selection in the fold range, having a cursor at the start of the fold
                            // marks it as selected. Non-empty selections don't cause this.
                            let length = selection.end - selection.start;
                            length > 0
                        })
                };

                is_in_pending_selection() || is_in_complete_selection()
            })
        });

        ButtonLike::new(fold_id)
            .style(ButtonStyle::Filled)
            .selected_style(ButtonStyle::Tinted(TintColor::Accent))
            .toggle_state(is_in_text_selection)
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Icon::from_path(icon.clone())
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        Label::new(label.clone())
                            .size(LabelSize::Small)
                            .single_line(),
                    ),
            )
            .into_any_element()
    })
}

fn fold_toggle(
    name: &'static str,
) -> impl Fn(
    MultiBufferRow,
    bool,
    Arc<dyn Fn(bool, &mut Window, &mut App) + Send + Sync>,
    &mut Window,
    &mut App,
) -> AnyElement {
    move |row, is_folded, fold, _window, _cx| {
        Disclosure::new((name, row.0 as u64), !is_folded)
            .toggle_state(is_folded)
            .on_click(move |_e, window, cx| fold(!is_folded, window, cx))
            .into_any_element()
    }
}
