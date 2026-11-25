use std::{collections::HashSet, sync::Arc};

use futures::future::{FutureExt, Shared};
use fuzzy_nucleo::{StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, ParentElement, SharedString, Task, WeakEntity, Window,
    actions,
};
use language::Buffer;
use picker::{MatchLocation, Picker, PickerDelegate, PreviewUpdate};
use project::Project;
use project::bookmark_store::{BookmarkStore, ProjectBookmark};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
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
    pub display_path: SharedString,
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
    bookmarks: Shared<Task<Arc<Vec<ProjectBookmark>>>>,
}

impl ProjectBookmarksDelegate {
    fn new(workspace: WeakEntity<Workspace>, project: Entity<Project>, cx: &mut App) -> Self {
        let bookmark_store = project.read(cx).bookmark_store();
        bookmark_store.update(cx, |bookmark_store, cx| {
            bookmark_store.forget_failed_paths(cx)
        });

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
            bookmarks,
        }
    }

    fn labels_for_match(&self, bookmark_match: &Match) -> (HighlightedLabel, Div) {
        let Match {
            label,
            positions,
            display_path,
            ..
        } = bookmark_match;

        (
            HighlightedLabel::new(label.clone(), positions.clone()),
            h_flex()
                .min_w_0()
                .child(
                    Label::new(display_path.clone())
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

            let matches = match_strings(candidates.as_slice(), &query, Default::default());

            picker
                .update(cx, |picker, cx| {
                    let visible_worktree_ids = picker
                        .delegate
                        .project
                        .read(cx)
                        .visible_worktrees(cx)
                        .map(|worktree| worktree.read(cx).id())
                        .collect::<HashSet<_>>();
                    let has_multiple_visible_worktrees = visible_worktree_ids.len() > 1;
                    picker.delegate.matches = matches
                        .into_iter()
                        .filter_map(|mat| {
                            let ProjectBookmark {
                                buffer,
                                anchor,
                                path,
                                ..
                            } = bookmarks.get(mat.candidate_id)?;

                            let snapshot = buffer.read(cx).snapshot();
                            let display_path = snapshot
                                .file()
                                .and_then(|file| {
                                    let include_root = has_multiple_visible_worktrees
                                        || !visible_worktree_ids.contains(&file.worktree_id(cx));
                                    snapshot.resolve_file_path(include_root, cx)
                                })
                                .unwrap_or_else(|| path.to_string_lossy().into_owned());
                            let line_number =
                                snapshot.summary_for_anchor::<text::Point>(anchor).row + 1;

                            Some(Match {
                                display_path: display_path.into(),
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
        _cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let bookmark_match = self.matches.get(match_index)?;
        let (bookmark_label, full_path_label) = self.labels_for_match(bookmark_match);
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

#[cfg(test)]
mod tests {
    use super::*;

    use editor::Editor;
    use gpui::{Entity, TestAppContext, VisualTestContext};
    use menu::{Confirm, SecondaryConfirm};
    use project::{ProjectPath, bookmark_store::SerializedBookmark};
    use serde_json::json;
    use std::path::PathBuf;
    use text::Point;
    use util::{path, rel_path::rel_path};
    use workspace::{AppState, MultiWorkspace};

    #[gpui::test]
    async fn test_project_bookmarks_picker_matches_and_confirms(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "one.txt": "alpha\nbeta\ngamma\n",
                    "two.txt": "delta\nepsilon\nzeta\n",
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        add_bookmark(&project, worktree_id, "one.txt", 0, "alpha mark", cx).await;
        add_bookmark(&project, worktree_id, "one.txt", 2, "gamma mark", cx).await;
        add_bookmark(&project, worktree_id, "two.txt", 1, "epsilon mark", cx).await;

        let (picker, workspace, cx) = build_bookmarks_picker(project, cx);

        picker.update(cx, |picker, _| {
            assert_eq!(picker.delegate.matches.len(), 3);
            assert_eq!(picker.delegate.match_count(), 3);
            assert_eq!(picker.delegate.selected_index(), 0);
        });

        simulate_input(cx, "gamma");
        picker.update(cx, |picker, _| {
            assert_eq!(picker.delegate.matches.len(), 1);
            assert_eq!(picker.delegate.match_count(), 1);
            assert_eq!(picker.delegate.selected_index(), 0);
            assert_eq!(
                picker
                    .delegate
                    .matches
                    .first()
                    .expect("matching bookmark should exist")
                    .display_path
                    .as_ref(),
                "one.txt"
            );
        });

        cx.dispatch_action(Confirm);
        cx.run_until_parked();

        let active_editor = workspace.read_with(cx, |workspace, cx| {
            workspace.active_item_as::<Editor>(cx).unwrap()
        });
        active_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.target_file_abs_path(cx).unwrap(),
                std::path::PathBuf::from(path!("/root/one.txt"))
            );

            let selection = editor
                .selections
                .newest_adjusted(&editor.display_snapshot(cx));
            assert_eq!(selection.start, selection.end);
            assert_eq!(selection.start, Point::new(2, 0));
        });
    }

    #[gpui::test]
    async fn test_project_bookmarks_displays_single_file_and_external_paths(
        cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "single.txt": "bookmark\n",
                    "external-a": { "note.txt": "bookmark\n" },
                    "external-b": { "note.txt": "bookmark\n" },
                }),
            )
            .await;

        let project = Project::test(
            app_state.fs.clone(),
            [path!("/root/single.txt").as_ref()],
            cx,
        )
        .await;
        let bookmark_store = project.read_with(cx, |project, _| project.bookmark_store());
        let bookmarks = [
            path!("/root/single.txt"),
            path!("/root/external-a/note.txt"),
            path!("/root/external-b/note.txt"),
        ]
        .into_iter()
        .map(|path| {
            (
                Arc::from(std::path::Path::new(path)),
                vec![SerializedBookmark {
                    row: 0,
                    label: "same label".to_string(),
                }],
            )
        })
        .collect();
        bookmark_store
            .update(cx, |bookmark_store, cx| {
                bookmark_store.load_serialized_bookmarks(bookmarks, cx)
            })
            .await
            .expect("serialized bookmarks should load");

        let (picker, _workspace, cx) = build_bookmarks_picker(project, cx);
        picker.read_with(cx, |picker, cx| {
            let project = picker.delegate.project.read(cx);
            assert_eq!(project.visible_worktrees(cx).count(), 1);
            assert_eq!(project.worktrees(cx).count(), 3);

            let mut paths = picker
                .delegate
                .matches
                .iter()
                .map(|bookmark_match| bookmark_match.display_path.as_ref())
                .collect::<Vec<_>>();
            paths.sort_unstable();
            let mut expected_paths = vec![
                "single.txt",
                path!("/root/external-a/note.txt"),
                path!("/root/external-b/note.txt"),
            ];
            expected_paths.sort_unstable();
            assert_eq!(paths, expected_paths);
        });
    }

    #[gpui::test]
    async fn test_project_bookmarks_updates_paths_after_worktree_removal(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "first": { "shared.txt": "bookmark\n" },
                    "second": { "shared.txt": "bookmark\n" },
                }),
            )
            .await;

        let project =
            Project::test(app_state.fs.clone(), [path!("/root/first").as_ref()], cx).await;
        let first_worktree_id = project.read_with(cx, |project, cx| {
            project
                .visible_worktrees(cx)
                .next()
                .expect("first worktree should exist")
                .read(cx)
                .id()
        });
        add_bookmark(
            &project,
            first_worktree_id,
            "shared.txt",
            0,
            "same label",
            cx,
        )
        .await;

        let (picker, workspace, cx) = build_bookmarks_picker(project.clone(), cx);
        let display_paths = |picker: &Entity<Picker<ProjectBookmarksDelegate>>,
                             cx: &VisualTestContext| {
            picker.read_with(cx, |picker, _| {
                picker
                    .delegate
                    .matches
                    .iter()
                    .map(|bookmark_match| PathBuf::from(bookmark_match.display_path.as_ref()))
                    .collect::<HashSet<_>>()
            })
        };
        assert_eq!(
            display_paths(&picker, cx),
            HashSet::from([PathBuf::from("shared.txt")])
        );
        cx.dispatch_action(menu::Cancel);
        cx.run_until_parked();
        drop(picker);

        let (second_worktree, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path!("/root/second"), true, cx)
            })
            .await
            .expect("second worktree should open");
        let second_worktree_id = second_worktree.read_with(cx, |worktree, _| worktree.id());
        add_bookmark(
            &project,
            second_worktree_id,
            "shared.txt",
            0,
            "same label",
            cx,
        )
        .await;
        drop(second_worktree);

        let picker = open_bookmarks_picker(&workspace, cx);
        assert_eq!(
            display_paths(&picker, cx),
            HashSet::from([
                PathBuf::from("first").join("shared.txt"),
                PathBuf::from("second").join("shared.txt"),
            ])
        );

        project.update(cx, |project, cx| {
            project.remove_worktree(second_worktree_id, cx);
        });
        picker.update_in(cx, |picker, window, cx| {
            picker.update_matches(String::new(), window, cx);
        });
        cx.run_until_parked();

        project.read_with(cx, |project, cx| {
            assert_eq!(project.visible_worktrees(cx).count(), 1);
            assert!(project.worktree_for_id(second_worktree_id, cx).is_none());
        });
        assert_eq!(
            display_paths(&picker, cx),
            HashSet::from([
                PathBuf::from("shared.txt"),
                PathBuf::from("second").join("shared.txt"),
            ])
        );
    }

    #[gpui::test]
    async fn test_project_bookmarks_confirm_reuses_preview_and_secondary_confirm_opens_new_tabs(
        cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "a.txt": "alpha\nbeta\ngamma\n",
                    "b.txt": "delta\nepsilon\nzeta\n",
                    "c.txt": "eta\ntheta\niota\n",
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        add_bookmark(&project, worktree_id, "a.txt", 0, "alpha mark", cx).await;
        add_bookmark(&project, worktree_id, "b.txt", 1, "epsilon mark", cx).await;
        add_bookmark(&project, worktree_id, "c.txt", 2, "iota mark", cx).await;

        let (picker, workspace, cx) = build_bookmarks_picker(project, cx);
        select_bookmark_with_label(&picker, "alpha mark", cx);
        cx.dispatch_action(Confirm);
        cx.run_until_parked();
        assert_active_pane_items_len(&workspace, 1, cx);

        let picker = open_bookmarks_picker(&workspace, cx);
        select_bookmark_with_label(&picker, "epsilon mark", cx);
        cx.dispatch_action(Confirm);
        cx.run_until_parked();
        assert_active_pane_items_len(&workspace, 1, cx);

        let picker = open_bookmarks_picker(&workspace, cx);
        select_bookmark_with_label(&picker, "alpha mark", cx);
        cx.dispatch_action(SecondaryConfirm);
        cx.run_until_parked();
        assert_active_pane_items_len(&workspace, 2, cx);

        let picker = open_bookmarks_picker(&workspace, cx);
        select_bookmark_with_label(&picker, "iota mark", cx);
        cx.dispatch_action(SecondaryConfirm);
        cx.run_until_parked();
        assert_active_pane_items_len(&workspace, 3, cx);
    }

    fn init_test(cx: &mut TestAppContext) -> std::sync::Arc<AppState> {
        zlog::init_test();
        cx.update(|cx| {
            let state = AppState::test(cx);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            super::init(cx);
            editor::init(cx);
            state
        })
    }

    async fn add_bookmark(
        project: &Entity<Project>,
        worktree_id: project::WorktreeId,
        path: &str,
        row: u32,
        label: &str,
        cx: &mut TestAppContext,
    ) {
        let project_path = ProjectPath {
            worktree_id,
            path: rel_path(path).into(),
        };
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))
            .await
            .unwrap();
        let anchor = buffer.read_with(cx, |buffer, _cx| {
            buffer.text_snapshot().anchor_before(Point::new(row, 0))
        });
        let bookmark_store = project.read_with(cx, |project, _| project.bookmark_store());
        bookmark_store.update(cx, |bookmark_store, cx| {
            bookmark_store.toggle_bookmark(buffer, anchor, label.to_string(), cx);
        });
    }

    fn build_bookmarks_picker(
        project: Entity<Project>,
        cx: &mut TestAppContext,
    ) -> (
        Entity<Picker<ProjectBookmarksDelegate>>,
        Entity<workspace::Workspace>,
        &mut VisualTestContext,
    ) {
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let picker = open_bookmarks_picker(&workspace, cx);
        cx.run_until_parked();
        (picker, workspace, cx)
    }

    fn open_bookmarks_picker(
        workspace: &Entity<workspace::Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<Picker<ProjectBookmarksDelegate>> {
        cx.dispatch_action(ToggleProjectBookmarks);
        cx.run_until_parked();
        workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<Picker<ProjectBookmarksDelegate>>(cx)
                .expect("project bookmarks picker is not open")
        })
    }

    fn simulate_input(cx: &mut VisualTestContext, input: &str) {
        cx.simulate_input(input);
        cx.run_until_parked();
    }

    fn select_bookmark_with_label(
        picker: &Entity<Picker<ProjectBookmarksDelegate>>,
        label: &str,
        cx: &mut VisualTestContext,
    ) {
        picker.update_in(cx, |picker, window, cx| {
            let match_index = picker
                .delegate
                .matches
                .iter()
                .position(|search_match| search_match.label.as_ref() == label)
                .expect("bookmark label should be present");
            picker.set_selected_index(match_index, None, true, window, cx);
        });
    }

    fn assert_active_pane_items_len(
        workspace: &Entity<workspace::Workspace>,
        expected_len: usize,
        cx: &mut VisualTestContext,
    ) {
        workspace.read_with(cx, |workspace, cx| {
            let actual_len = workspace.active_pane().read(cx).items_len();
            assert_eq!(actual_len, expected_len);
        });
    }
}
