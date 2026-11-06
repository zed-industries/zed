use std::{collections::HashMap, sync::Arc};

#[cfg(test)]
mod project_bookmarks_test;


use futures::future::{FutureExt, Shared};
use fuzzy_nucleo::{Case, LengthPenalty, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, ParentElement, SharedString, Task, WeakEntity, Window,
    actions,
};
use language::Buffer;
use picker::{MatchLocation, Picker, PickerDelegate, PreviewUpdate};
use project::WorktreeId;
use project::bookmark_store::ProjectBookmark;
use project::{Project, ProjectPath, bookmark_store::BookmarkStore};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::rel_path::RelPath;
use workspace::Workspace;

actions!(
    project_bookmarks,
    [
        /// Toggles the project bookmarks search.
        #[action(name = "Toggle")]
        ToggleProjectBookmarks,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleProjectBookmarks, window, cx| {
                let project = workspace.project().clone();
                let workspace_handle = cx.entity().downgrade();

                workspace.toggle_modal(window, cx, move |window, cx| {
                    let delegate =
                        ProjectBookmarksDelegate::new(workspace_handle, project.clone(), cx);
                    let preview = picker_preview::editor_preview(project, window, cx);
                    Picker::uniform_list_with_preview(delegate, preview, window, cx)
                        .reopenable(false, cx)
                })
            });
        },
    )
    .detach();
}

struct Match {
    pub path: ProjectPath,
    pub label: SharedString,
    pub buffer: Entity<Buffer>,
    pub positions: Vec<usize>,
    pub anchor: text::Anchor,
    pub line_number: u32,
}


struct ProjectBookmarksDelegate {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    selected_match_index: usize,
    matches: Vec<Match>,
    worktree_root_names: HashMap<WorktreeId, Arc<RelPath>>,
    bookmarks: Shared<Task<Arc<Vec<ProjectBookmark>>>>,
}

impl ProjectBookmarksDelegate {
    fn new(workspace: WeakEntity<Workspace>, project: Entity<Project>, cx: &mut App) -> Self {
        let bookmark_store = project.read(cx).bookmark_store();
        bookmark_store.update(cx, |bookmark_store, cx| {
            bookmark_store.forget_failed_paths(cx)
        });

        let worktree_root_names = project
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| {
                let worktree = worktree.read(cx);
                (worktree.id(), worktree.root_name().into_arc())
            })
            .collect();

        let bookmarks = cx
            .spawn(async move |cx| {
                let bookmarks = BookmarkStore::all_bookmarks(&bookmark_store, cx).await;
                bookmarks
                    .map(|bookmarks| Arc::new(bookmarks))
                    .unwrap_or_default()
            })
            .shared();

        Self {
            workspace,
            project,
            selected_match_index: 0,
            matches: Vec::new(),
            worktree_root_names,
            bookmarks,
        }
    }

    fn with_optional_worktree_root_name(
        &self,
        worktree_id: WorktreeId,
        rel_path: &RelPath,
    ) -> Option<Arc<RelPath>> {
        if self.worktree_root_names.len() > 1 {
            self.worktree_root_names
                .get(&worktree_id)
                .map(|root_name| root_name.join(rel_path).into())
        } else {
            Some(rel_path.into_arc())
        }
    }

    fn labels_for_match(&self, bookmark_match: &Match, cx: &App) -> (HighlightedLabel, Div) {
        let path_style = self.project.read(cx).path_style(cx);
        let Match {
            label,
            positions,
            path: project_path,
            ..
        } = bookmark_match;

        let full_path_name =
            self.with_optional_worktree_root_name(project_path.worktree_id, &project_path.path);

        (
            HighlightedLabel::new(label.clone(), positions.clone()),
            h_flex()
                .min_w_0()
                .child(
                    Label::new(
                        full_path_name
                            .unwrap_or(project_path.path.clone())
                            .display(path_style),
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .truncate_start(),
                )
                .child(
                    Label::new(format!(":{}", bookmark_match.line_number))
                        .size(LabelSize::Small)
                        .color(Color::Placeholder),
                ),
        )
    }

}

impl PickerDelegate for ProjectBookmarksDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "bookmarks"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(
        &mut self,
        match_index: usize,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) {
        self.selected_match_index = match_index;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> std::sync::Arc<str> {
        "Search for a bookmark...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Task<()> {
        let bookmarks = self.bookmarks.clone();

        cx.spawn_in(window, async move |picker, cx| {
            let bookmarks = bookmarks.await;

            let candidates: Vec<StringMatchCandidate> = bookmarks
                .iter()
                .enumerate()
                .map(|(i, bookmark)| StringMatchCandidate::new(i, &bookmark.label))
                .collect();

            let matches = match_strings(
                candidates.as_slice(),
                &query,
                Case::Smart,
                LengthPenalty::On,
                candidates.len(),
            );

            picker
                .update(cx, |picker, cx| {
                    picker.delegate.matches = matches
                        .into_iter()
                        .filter_map(|mat| {
                            let ProjectBookmark {
                                buffer,
                                anchor,
                                path,
                                ..
                            } = bookmarks.get(mat.candidate_id)?;

                            let project_path = picker
                                .delegate
                                .project
                                .read(cx)
                                .project_path_for_absolute_path(path, cx)?;

                            let line_number = {
                                let snapshot = buffer.read(cx).text_snapshot();
                                snapshot.summary_for_anchor::<text::Point>(anchor).row + 1
                            };

                            Some(Match {
                                path: project_path,
                                label: mat.string,
                                positions: mat.positions,
                                buffer: buffer.clone(),
                                anchor: *anchor,
                                line_number,
                            })
                        })
                        .collect();

                    picker.delegate.selected_match_index = 0;
                })
                .ok();
        })
    }

    fn confirm(
        &mut self,
        secondary: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let Some(selected_bookmark) = self.matches.get(self.selected_match_index) else {
            return;
        };

        let editor = workspace.update(cx, |workspace, cx| {
            workspace.open_project_item::<editor::Editor>(
                None,
                selected_bookmark.buffer.clone(),
                true,
                true,
                true,
                !secondary,
                window,
                cx,
            )
        });

        editor.update(cx, |editor, cx| {
            let point = selected_bookmark
                .buffer
                .read(cx)
                .snapshot()
                .summary_for_anchor::<text::Point>(&selected_bookmark.anchor);

            editor.go_to_singleton_buffer_point(point, window, cx);
        });

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<picker::Picker<Self>>) {}

    fn render_match(
        &self,
        match_index: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let bookmark_match = self.matches.get(match_index)?;
        let (bookmark_label, full_path_label) = self.labels_for_match(bookmark_match, cx);
        let icon = Icon::new(IconName::Bookmark)
            .size(IconSize::Small)
            .color(Color::Info);

        Some(
            ListItem::new(match_index)
                .spacing(ListItemSpacing::Sparse)
                .inset(true)
                .toggle_state(selected)
                .start_slot::<Icon>(Some(icon))
                .child(
                    v_flex()
                        .w_full()
                        .min_w_0()
                        .child(bookmark_label.truncate())
                        .child(full_path_label),
                ),
        )
    }

    fn try_get_preview_data_for_match(&self, cx: &App) -> Option<PreviewUpdate> {
        let selected_bookmark = self.matches.get(self.selected_match_index)?;

        let snapshot = selected_bookmark.buffer.read(cx).snapshot();
        let offset = snapshot.offset_for_anchor(&selected_bookmark.anchor);
        let row = snapshot
            .summary_for_anchor::<text::Point>(&selected_bookmark.anchor)
            .row;
        let start = text::Point::new(row, 0);
        let end = start + text::Point::new(1, 0);
        let start_anchor = snapshot.anchor_before(start);
        let end_anchor = snapshot.anchor_before(end);

        Some(PreviewUpdate::from_buffer(
            selected_bookmark.buffer.clone(),
            MatchLocation {
                anchor_range: start_anchor..end_anchor,
                range: offset..offset,
            },
        ))
    }
}
