#![cfg(test)]

use collections::HashSet;
use fs::FakeFs;
use gpui::{Entity, VisualTestContext};
use project::Project;
use serde_json::{Value, json};
use std::path::Path;
use std::sync::Arc;
use workspace::MultiWorkspace;

use crate::project_panel_tests::{self, find_project_entry, select_path};
use crate::{NewDirectory, NewFile, ProjectPanel, Redo, Rename, Undo};

struct TestContext {
    panel: Entity<ProjectPanel>,
    fs: Arc<FakeFs>,
    cx: VisualTestContext,
}

// Using the `util::path` macro requires a string literal, which would mean that
// callers of, for example, `rename`, would now need to know about `/` and
// use `path!` in tests.
//
// As such, we define it as a function here to make the helper methods more
// ergonomic for our use case.
fn path(path: impl AsRef<str>) -> String {
    let path = path.as_ref();
    #[cfg(target_os = "windows")]
    {
        path = path.replace("/", "\\");
        if path.starts_with("\\") {
            path = format!("C:{}", path);
        }
        path
    }
    path.to_string()
}

impl TestContext {
    async fn undo(&mut self) {
        self.panel.update_in(&mut self.cx, |panel, window, cx| {
            panel.undo(&Undo, window, cx);
        });
        self.cx.run_until_parked();
    }
    async fn redo(&mut self) {
        self.panel.update_in(&mut self.cx, |panel, window, cx| {
            panel.redo(&Redo, window, cx);
        });
        self.cx.run_until_parked();
    }

    /// Note this only works when every file has an extension
    fn assert_fs_state_is(&mut self, state: &[&str]) {
        let state: HashSet<_> = state
            .into_iter()
            .map(|s| path(format!("/workspace/{s}")))
            .chain([path("/workspace"), path("/")])
            .map(|s| Path::new(&s).to_path_buf())
            .collect();

        let dirs: HashSet<_> = state
            .iter()
            .map(|p| {
                if p.extension().is_some() {
                    p.parent().unwrap_or(Path::new(&path("/"))).to_owned()
                } else {
                    // TODO!(dino): Make this prettier please!
                    p.clone()
                }
            })
            .collect();

        assert_eq!(
            self.fs
                .directories(true)
                .into_iter()
                .collect::<HashSet<_>>(),
            dirs
        );
        assert_eq!(
            self.fs.paths(true).into_iter().collect::<HashSet<_>>(),
            state
        );
    }

    fn assert_exists(&mut self, file: &str) {
        assert!(
            find_project_entry(
                &self.panel,
                &path(format!("workspace/{file}")),
                &mut self.cx
            )
            .is_some(),
            "{file} should exist"
        );
    }

    fn assert_not_exists(&mut self, file: &str) {
        assert_eq!(
            find_project_entry(
                &self.panel,
                &path(format!("workspace/{file}")),
                &mut self.cx
            ),
            None,
            "{file} should not exist"
        );
    }

    async fn rename(&mut self, from: &str, to: &str) {
        let from = path(format!("workspace/{from}"));
        let Self { panel, cx, .. } = self;
        select_path(&panel, &from, cx);
        panel.update_in(cx, |panel, window, cx| panel.rename(&Rename, window, cx));
        cx.run_until_parked();

        let confirm = panel.update_in(cx, |panel, window, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text(to, window, cx));
            panel.confirm_edit(true, window, cx).unwrap()
        });
        confirm.await.unwrap();
        cx.run_until_parked();
    }

    async fn create_file(&mut self, path: &str) {
        let Self { panel, cx, .. } = self;
        select_path(&panel, "workspace", cx);
        panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
        cx.run_until_parked();

        let confirm = panel.update_in(cx, |panel, window, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text(path, window, cx));
            panel.confirm_edit(true, window, cx).unwrap()
        });
        confirm.await.unwrap();
        cx.run_until_parked();
    }

    async fn create_directory(&mut self, path: &str) {
        let Self { panel, cx, .. } = self;

        select_path(&panel, "workspace", cx);
        panel.update_in(cx, |panel, window, cx| {
            panel.new_directory(&NewDirectory, window, cx)
        });
        cx.run_until_parked();

        let confirm = panel.update_in(cx, |panel, window, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text(path, window, cx));
            panel.confirm_edit(true, window, cx).unwrap()
        });
        confirm.await.unwrap();
        cx.run_until_parked();
    }

    /// Drags the `files` to the provided `directory`.
    fn drag(&mut self, files: &[&str], directory: &str) {
        // TODO do we need this?
        // for dir in files
        //     .iter()
        //     .map(|p| path(format!("workspace/{p}")))
        //     .filter_map(|p| Path::new(&p).parent().map(Path::to_owned))
        //     .map(|p| p.display().to_string())
        // {
        //     project_panel_tests::toggle_expand_dir(&self.panel, &dir, &mut self.cx);
        // }

        self.panel
            .update(&mut self.cx, |panel, _| panel.marked_entries.clear());
        files.into_iter().for_each(|file| {
            project_panel_tests::select_path_with_mark(
                &self.panel,
                &path(format!("workspace/{file}")),
                &mut self.cx,
            )
        });
        project_panel_tests::drag_selection_to(
            &self.panel,
            &path(format!("workspace/{directory}")),
            false,
            &mut self.cx,
        );
    }

    /// Only supports files in root (otherwise would need toggle_expand_dir).
    /// For undo redo the paths themselves do not matter so this is fine
    async fn cut(&mut self, file: &str) {
        project_panel_tests::select_path_with_mark(
            &self.panel,
            &path(format!("workspace/{file}")),
            &mut self.cx,
        );
        self.panel.update_in(&mut self.cx, |panel, window, cx| {
            panel.cut(&Default::default(), window, cx);
        });
    }

    /// Only supports files in root (otherwise would need toggle_expand_dir).
    /// For undo redo the paths themselves do not matter so this is fine
    async fn paste(&mut self, file: &str) {
        select_path(
            &self.panel,
            &path(format!("workspace/{file}")),
            &mut self.cx,
        );
        self.panel.update_in(&mut self.cx, |panel, window, cx| {
            panel.paste(&Default::default(), window, cx);
        });
        self.cx.run_until_parked();
    }

    /// The test tree is:
    /// ```txt
    /// a.txt
    /// b.txt
    /// ```
    /// a and b are empty, x has the text "content" inside
    async fn new(cx: &mut gpui::TestAppContext) -> TestContext {
        Self::new_with_tree(
            cx,
            json!({
                    "a.txt": "",
                    "b.txt": "",
            }),
        )
        .await
    }

    async fn new_with_tree(cx: &mut gpui::TestAppContext, tree: Value) -> TestContext {
        project_panel_tests::init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/workspace", tree).await;
        let project = Project::test(fs.clone(), ["/workspace".as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window.into(), cx);
        let panel = workspace.update_in(&mut cx, ProjectPanel::new);
        cx.run_until_parked();

        TestContext { panel, fs, cx }
    }
}

#[gpui::test]
async fn rename_undo_redo(cx: &mut gpui::TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.rename("a.txt", "renamed.txt").await;
    cx.assert_exists("renamed.txt");
    cx.assert_not_exists("a.txt");

    cx.undo().await;
    cx.assert_exists("a.txt");
    cx.assert_not_exists("renamed.txt");

    cx.redo().await;
    cx.assert_exists("renamed.txt");
    cx.assert_not_exists("a.txt");
}

// TODO(dino): Would be nice if this test also actually confirmed that, if
// `new.txt` has some content before removal, that same content is preserved
// when restoring the file.
#[gpui::test]
async fn create_undo_redo(cx: &mut gpui::TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.create_file("new.txt").await;
    cx.assert_exists("new.txt");

    cx.undo().await;
    cx.assert_not_exists("new.txt");

    cx.redo().await;
    // THIS IS FAILING ▼▼▼▼▼▼▼▼
    cx.assert_exists("new.txt");
}

#[gpui::test]
async fn create_dir_undo(cx: &mut gpui::TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.create_directory("new_dir").await;
    cx.assert_exists("new_dir");
    cx.undo().await;
    cx.assert_not_exists("new_dir");
}

#[gpui::test]
async fn cut_paste_undo(cx: &mut gpui::TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.cut("a.txt").await;
    cx.paste("a.txt").await;
    cx.assert_exists("a.txt");

    cx.undo().await;
    cx.assert_not_exists("a.txt");
}

#[gpui::test]
async fn drag_undo_redo(cx: &mut gpui::TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.create_directory("src").await;
    cx.create_file("src/a.rs").await;

    cx.drag(&["src/a.rs"], "");
    cx.assert_exists("a.rs");
    cx.assert_not_exists("src/a.rs");

    cx.undo().await;
    cx.assert_exists("src/a.rs");
    cx.assert_not_exists("a.rs");

    cx.redo().await;
    cx.assert_exists("a.rs");
    cx.assert_not_exists("src/a.rs");
}

#[gpui::test]
async fn drag_multiple_undo_redo(cx: &mut gpui::TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.create_directory("src").await;
    cx.create_file("src/x.rs").await;
    cx.create_file("src/y.rs").await;

    cx.drag(&["src/x.rs", "src/y.rs"], "");
    cx.assert_fs_state_is(&["a.txt", "b.txt", "x.rs", "y.rs", "src/"]);
    // cx.assert_exists("x.rs");
    // cx.assert_not_exists("src/x.rs");
    // cx.assert_exists("b.rs");
    // cx.assert_not_exists("src/b.rs");

    cx.undo().await;
    cx.assert_fs_state_is(&["a.txt", "b.txt", "src/", "src/x.rs", "src/y.rs"]);
    // cx.assert_exists("src/x.rs");
    // cx.assert_not_exists("x.rs");
    // cx.assert_exists("src/b.rs");
    // cx.assert_not_exists("b.rs");

    cx.redo().await;
    cx.assert_fs_state_is(&["a.txt", "b.txt", "x.rs", "y.rs", "src/"]);
    // cx.assert_exists("a.rs");
    // cx.assert_not_exists("src/a.rs");
    // cx.assert_exists("b.rs");
    // cx.assert_not_exists("src/b.rs");
}

#[gpui::test]
async fn two_sequential_undos(cx: &mut gpui::TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.rename("a.txt", "x.txt").await;
    cx.create_file("y.txt").await;

    cx.undo().await; // TODO(yara) should we have an assert fs state instead?
    cx.assert_not_exists("y.txt");
    cx.assert_exists("x.txt");

    cx.undo().await;
    cx.assert_not_exists("x.txt");
    cx.assert_exists("a.txt");
}

#[gpui::test]
async fn undo_without_history(cx: &mut gpui::TestAppContext) {
    let mut cx = TestContext::new(cx).await;

    cx.undo().await;
    cx.assert_fs_state_is(&["a.txt", "b.txt"]) // default tree
}
