//! Tests for multi-selecting files in the file finder: toggling items in and
//! out of the selection, pinning the selection to the top across query
//! changes, and the various ways of opening the whole selection.

use gpui::{BorrowAppContext, Entity, TestAppContext, VisualTestContext};
use picker::{MultiSelectNext, Picker, PickerDelegate as _};
use pretty_assertions::assert_eq;
use project::Project;
use serde_json::json;
use settings::SettingsStore;
use util::path;
use workspace::{MultiWorkspace, Workspace, pane};

use crate::file_finder_tests::{self, open_file_picker};
use crate::{FileFinder, FileFinderDelegate, SEARCH_DEBOUNCE};

struct TestContext {
    picker: Entity<Picker<FileFinderDelegate>>,
    workspace: Entity<Workspace>,
    cx: VisualTestContext,
}

impl TestContext {
    /// The test tree is `a.rs`, `b.rs` and `c.rs` in the worktree root.
    /// Preview tabs from the file finder are enabled to verify that batch
    /// opens produce real tabs regardless.
    async fn new(cx: &mut TestAppContext) -> TestContext {
        let app_state = file_finder_tests::init_test(cx);
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .preview_tabs
                        .get_or_insert_default()
                        .enable_preview_from_file_finder = Some(true);
                });
            })
        });
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "a.rs": "// a",
                    "b.rs": "// b",
                    "c.rs": "// c",
                }),
            )
            .await;
        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window.into(), cx);
        let picker = open_file_picker(&workspace, &mut cx);
        TestContext {
            picker,
            workspace,
            cx,
        }
    }

    fn reopen_finder(&mut self) {
        self.picker = open_file_picker(&self.workspace, &mut self.cx);
    }

    fn search(&mut self, query: &str) {
        self.picker.update_in(&mut self.cx, |picker, window, cx| {
            picker.set_query(query, window, cx)
        });
        self.cx.executor().advance_clock(SEARCH_DEBOUNCE);
        self.cx.run_until_parked();
    }

    fn select(&mut self, file_name: &str) {
        let index = self.index_of(file_name);
        self.picker.update_in(&mut self.cx, |picker, window, cx| {
            picker.delegate.set_selected_index(index, window, cx);
        });
        self.cx.dispatch_action(MultiSelectNext);
        self.cx.run_until_parked();
    }

    fn deselect(&mut self, file_name: &str) {
        let index = self.index_of(file_name);
        self.picker.update_in(&mut self.cx, |picker, window, cx| {
            picker.delegate.toggle_item_selected(index, window, cx);
        });
        self.cx.run_until_parked();
    }

    fn confirm(&mut self) {
        self.cx.dispatch_action(menu::Confirm);
        self.cx.run_until_parked();
    }

    fn secondary_confirm(&mut self) {
        self.cx.dispatch_action(menu::SecondaryConfirm);
        self.cx.run_until_parked();
    }

    fn split_right(&mut self) {
        self.cx.dispatch_action(pane::SplitRight::default());
        self.cx.run_until_parked();
    }

    fn index_of(&mut self, file_name: &str) -> usize {
        self.picker.update(&mut self.cx, |picker, _| {
            picker
                .delegate
                .matches
                .matches
                .iter()
                .position(|m| {
                    m.relative_path()
                        .is_some_and(|path| path.file_name() == Some(file_name))
                })
                .unwrap_or_else(|| panic!("{file_name} is not in the match list"))
        })
    }

    fn match_names(&mut self) -> Vec<String> {
        self.picker.update(&mut self.cx, |picker, _| {
            picker
                .delegate
                .matches
                .matches
                .iter()
                .filter_map(|m| Some(m.relative_path()?.file_name()?.to_owned()))
                .collect()
        })
    }

    #[track_caller]
    fn assert_selected(&mut self, expected: &[&str]) {
        let selected: Vec<String> = self.picker.update(&mut self.cx, |picker, _| {
            picker
                .delegate
                .selected_matches
                .iter()
                .filter_map(|selected| Some(selected.0.relative_path()?.file_name()?.to_owned()))
                .collect()
        });
        assert_eq!(selected, expected, "wrong files selected");
    }

    #[track_caller]
    fn assert_finder_closed(&mut self) {
        let finder_open = self.workspace.update(&mut self.cx, |workspace, cx| {
            workspace.active_modal::<FileFinder>(cx).is_some()
        });
        assert!(!finder_open, "the finder should have been dismissed");
    }

    #[track_caller]
    fn assert_active_pane_items(&mut self, expected: &[&str]) {
        let mut items = self.pane_item_names(
            &self
                .workspace
                .read_with(&self.cx, |workspace, _| workspace.active_pane().clone()),
        );
        let mut expected: Vec<String> = expected.iter().map(|name| name.to_string()).collect();
        items.sort();
        expected.sort();
        assert_eq!(items, expected, "wrong items in the active pane");
    }

    fn pane_item_names(&mut self, pane: &Entity<pane::Pane>) -> Vec<String> {
        pane.read_with(&self.cx, |pane, cx| {
            pane.items()
                .filter_map(|item| Some(item.project_path(cx)?.path.file_name()?.to_owned()))
                .collect()
        })
    }

    fn pane_count(&mut self) -> usize {
        self.workspace
            .read_with(&self.cx, |workspace, _| workspace.panes().len())
    }
}

#[gpui::test]
async fn open_selection_as_tabs(cx: &mut TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.search("rs");
    cx.select("b.rs");
    cx.select("c.rs");
    cx.assert_selected(&["b.rs", "c.rs"]);

    cx.confirm();
    cx.assert_finder_closed();
    // Both files must open as real tabs even though preview tabs from the
    // file finder are enabled; a batch open must not reuse one preview tab.
    cx.assert_active_pane_items(&["b.rs", "c.rs"]);
}

#[gpui::test]
async fn tabbing_a_selected_row_deselects_it(cx: &mut TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.search("rs");
    cx.select("b.rs");
    cx.select("c.rs");
    cx.select("b.rs");
    cx.assert_selected(&["c.rs"]);
}

#[gpui::test]
async fn selection_pins_to_top_across_queries(cx: &mut TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.search("c");
    cx.select("c.rs");

    // `c.rs` doesn't match the new query, but stays selected and pinned to
    // the top of the results.
    cx.search("b");
    cx.assert_selected(&["c.rs"]);
    assert_eq!(cx.match_names(), ["c.rs", "b.rs"]);
}

#[gpui::test]
async fn deselecting_survives_queries(cx: &mut TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.search("c");
    cx.select("c.rs");

    cx.search("b");
    cx.deselect("c.rs");
    cx.assert_selected(&[]);

    // A deselected file must not come back selected or pinned on requery.
    cx.search("a");
    cx.assert_selected(&[]);
    assert_eq!(cx.match_names(), ["a.rs"]);
}

#[gpui::test]
async fn create_new_file_row_is_not_selectable(cx: &mut TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    // A query matching nothing produces only the "create new" row.
    cx.search("zzz");
    cx.picker.update_in(&mut cx.cx, |picker, window, cx| {
        picker.delegate.set_selected_index(0, window, cx);
    });
    cx.cx.dispatch_action(MultiSelectNext);
    cx.cx.run_until_parked();
    cx.assert_selected(&[]);
}

#[gpui::test]
async fn open_selection_in_one_split(cx: &mut TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.search("rs");
    cx.select("b.rs");
    cx.select("c.rs");

    cx.split_right();
    cx.assert_finder_closed();
    // One new pane holding both files as tabs, not one pane per file.
    assert_eq!(cx.pane_count(), 2);
    cx.assert_active_pane_items(&["b.rs", "c.rs"]);
}

#[gpui::test]
async fn secondary_confirm_opens_one_split_per_file(cx: &mut TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    // Open a file normally first so the workspace has a non-empty pane to
    // split off from.
    cx.search("a");
    cx.confirm();
    cx.assert_active_pane_items(&["a.rs"]);

    cx.reopen_finder();
    cx.search("rs");
    cx.select("b.rs");
    cx.select("c.rs");

    cx.secondary_confirm();
    cx.assert_finder_closed();
    assert_eq!(
        cx.pane_count(),
        3,
        "each selected file opens in its own split"
    );
}
