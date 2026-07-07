use std::{collections::HashMap, sync::Arc};

#[cfg(test)]
mod project_bookmarks_test;

use file_icons::FileIcons;
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
use settings::Settings;
use ui::{Divider, HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use util::rel_path::RelPath;
use workspace::Workspace;
use workspace::item::ItemSettings;

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
                    Picker::list_with_preview(delegate, preview, window, cx).reopenable(false, cx)
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

enum Entry {
    Header(ProjectPath),
    Match(usize),
    Separator,
}

struct ProjectBookmarksDelegate {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    group_result_by_path: bool,
    selected_entry_index: usize,
    matches: Vec<Match>,
    max_match_line_number: u32,
    entries: Vec<Entry>,
    worktree_root_names: HashMap<WorktreeId, Arc<RelPath>>,
    bookmarks: Shared<Task<Arc<Vec<ProjectBookmark>>>>,
}

impl ProjectBookmarksDelegate {
    fn new(workspace: WeakEntity<Workspace>, project: Entity<Project>, cx: &App) -> Self {
        let bookmark_store = project.read(cx).bookmark_store();

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
            group_result_by_path: false,
            selected_entry_index: 0,
            matches: Vec::new(),
            entries: Vec::new(),
            worktree_root_names,
            bookmarks,
            max_match_line_number: 0,
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
                .map(|root_name| root_name.join(rel_path))
        } else {
            Some(rel_path.into_arc())
        }
    }

    fn labels_for_match(
        &self,
        bookmark_match: &Match,
        _window: &mut Window,
        cx: &App,
    ) -> (HighlightedLabel, Div) {
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
                    Label::new(format!(":{}", bookmark_match.line_number,))
                        .size(LabelSize::Small)
                        .color(Color::Placeholder),
                ),
        )
    }

    fn rebuild_entries(&mut self) {
        let mut entries = Vec::with_capacity(self.matches.len());
        let mut last_path: Option<&ProjectPath> = None;

        for (match_index, search_match) in self.matches.iter().enumerate() {
            if last_path != Some(&search_match.path) {
                if last_path.is_some() {
                    entries.push(Entry::Separator);
                }
                entries.push(Entry::Header(search_match.path.clone()));
                last_path = Some(&search_match.path);
            }
            entries.push(Entry::Match(match_index));
        }

        self.entries = entries;
        self.select_first_available()
    }

    fn select_first_available(&mut self) {
        for (i, entry) in self.entries.iter().enumerate() {
            if let Entry::Match(_) = entry {
                self.selected_entry_index = i;
                return;
            }
        }
    }

    fn render_header(&self, project_path: &ProjectPath, cx: &mut App) -> AnyElement {
        let path_style = self.project.read(cx).path_style(cx);
        let file_name = project_path
            .path
            .file_name()
            .map(|name| name.to_string())
            .unwrap_or_default();
        let directory = project_path
            .path
            .parent()
            .and_then(|parent| {
                self.with_optional_worktree_root_name(project_path.worktree_id, parent)
            })
            .map(|parent| SharedString::new(parent.display(path_style)))
            .unwrap_or_default();
        let file_icon = ItemSettings::get_global(cx)
            .file_icons
            .then(|| FileIcons::get_icon(project_path.path.as_std_path(), cx))
            .flatten()
            .map(|icon| {
                Icon::from_path(icon)
                    .color(Color::Muted)
                    .size(IconSize::Small)
            });

        h_flex()
            .w_full()
            .min_w_0()
            .px(DynamicSpacing::Base06.rems(cx))
            .py_1()
            .gap_1p5()
            .children(file_icon)
            .child(
                h_flex()
                    .min_w_0()
                    .gap_1()
                    .child(Label::new(file_name).size(LabelSize::Small))
                    .when(!directory.is_empty(), |this| {
                        this.child(
                            Label::new(directory)
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .truncate_start(),
                        )
                    }),
            )
            .into_any_element()
    }
}

impl PickerDelegate for ProjectBookmarksDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "bookmarks"
    }

    fn match_count(&self) -> usize {
        self.entries.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_entry_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) {
        self.selected_entry_index = ix;
    }

    fn can_select(&self, ix: usize, _window: &mut Window, _cx: &mut Context<Picker<Self>>) -> bool {
        self.entries
            .get(ix)
            .map(|entry| matches!(entry, Entry::Match(_)))
            .unwrap_or(false)
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
                100,
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

                    picker.delegate.max_match_line_number = picker
                        .delegate
                        .matches
                        .iter()
                        .map(|m| m.line_number)
                        .max()
                        .unwrap_or(0);
                    picker.delegate.rebuild_entries();
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
        let Some(&Entry::Match(ix)) = self.entries.get(self.selected_entry_index) else {
            return;
        };
        let Some(selected_bookmark) = self.matches.get(ix) else {
            return;
        };

        let open_task = workspace.update(cx, |workspace, cx| {
            workspace.open_path_preview(
                selected_bookmark.path.clone(),
                None,
                true,
                !secondary,
                true,
                window,
                cx,
            )
        });

        let point = selected_bookmark
            .buffer
            .read(cx)
            .snapshot()
            .summary_for_anchor::<text::Point>(&selected_bookmark.anchor);

        cx.spawn_in(window, async move |_, cx| {
            let item = open_task.await.log_err()?;
            if let Some(active_editor) = item.downcast::<editor::Editor>() {
                active_editor
                    .downgrade()
                    .update_in(cx, |editor, window, cx| {
                        editor.go_to_singleton_buffer_point(point, window, cx);
                    })
                    .log_err();
            }
            Some(())
        })
        .detach();

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<picker::Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = self.entries.get(ix)?;
        let icon = Icon::new(IconName::Bookmark)
            .size(IconSize::Small)
            .color(Color::Info);

        match entry {
            Entry::Header(project_path) => self
                .group_result_by_path
                .then(|| self.render_header(project_path, cx)),
            &Entry::Match(ix) => self.matches.get(ix).map(|mat| {
                let item_base = ListItem::new(ix)
                    .spacing(ListItemSpacing::Sparse)
                    .inset(true)
                    .toggle_state(selected);

                let (bookmark_label, full_path_label) = self.labels_for_match(mat, window, cx);

                if self.group_result_by_path {
                    item_base.child(
                        h_flex()
                            .w_full()
                            .gap_1p5()
                            .justify_between()
                            .overflow_x_hidden()
                            .child(bookmark_label.truncate().into_any_element())
                            .child(
                                Label::new(mat.line_number.to_string())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                } else {
                    item_base.start_slot::<Icon>(Some(icon)).child(
                        h_flex()
                            .w_full()
                            .min_w_0()
                            .gap_1p5()
                            .child(bookmark_label.truncate())
                            .child(full_path_label)
                            .into_any_element(),
                    )
                }
                .into_any_element()
            }),
            Entry::Separator => self.group_result_by_path.then(|| {
                div()
                    .py(DynamicSpacing::Base04.rems(cx))
                    .child(Divider::horizontal())
                    .into_any_element()
            }),
        }
    }

    fn preview_layout_changed(&mut self, layout_is_horizontal: bool) {
        self.group_result_by_path = layout_is_horizontal;
    }

    fn try_get_preview_data_for_match(&self, cx: &App) -> Option<PreviewUpdate> {
        let selected_bookmark = match self.entries.get(self.selected_entry_index)? {
            &Entry::Match(ix) => self.matches.get(ix),
            _ => None,
        }?;

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
