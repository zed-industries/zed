use std::ops::Deref;

use editor::{display_map::ToDisplayPoint, DisplayPoint};
use gpui::{json::json, keymap::Keystroke, ViewHandle};
use language::{Point, Selection};
use workspace::{WorkspaceHandle, WorkspaceParams};

use crate::*;

#[gpui::test]
async fn test_insert_mode(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestAppContext::new(cx, "").await;
    assert_eq!(cx.mode(), Mode::Normal);
    cx.simulate_keystroke("i");
    assert_eq!(cx.mode(), Mode::Insert);
    cx.simulate_keystrokes(&["T", "e", "s", "t"]);
    assert_eq!(cx.editor_text(), "Test".to_owned());
    cx.simulate_keystroke("escape");
    assert_eq!(cx.mode(), Mode::Normal);
}

#[gpui::test]
async fn test_normal_hjkl(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestAppContext::new(cx, "Test\nTestTest\nTest").await;
    assert_eq!(cx.mode(), Mode::Normal);
    cx.simulate_keystroke("l");
    assert_eq!(cx.newest_selection().head(), DisplayPoint::new(0, 1));
    cx.simulate_keystroke("h");
    assert_eq!(cx.newest_selection().head(), DisplayPoint::new(0, 0));
    cx.simulate_keystroke("j");
    assert_eq!(cx.newest_selection().head(), DisplayPoint::new(1, 0));
    cx.simulate_keystroke("k");
    assert_eq!(cx.newest_selection().head(), DisplayPoint::new(0, 0));

    cx.simulate_keystroke("j");
    assert_eq!(cx.newest_selection().head(), DisplayPoint::new(1, 0));

    // When moving left, cursor does not wrap to the previous line
    cx.simulate_keystroke("h");
    assert_eq!(cx.newest_selection().head(), DisplayPoint::new(1, 0));

    // When moving right, cursor does not reach the line end or wrap to the next line
    for _ in 0..9 {
        cx.simulate_keystroke("l");
    }
    assert_eq!(cx.newest_selection().head(), DisplayPoint::new(1, 7));

    // Goal column respects the inability to reach the end of the line
    cx.simulate_keystroke("k");
    assert_eq!(cx.newest_selection().head(), DisplayPoint::new(0, 3));
    cx.simulate_keystroke("j");
    assert_eq!(cx.newest_selection().head(), DisplayPoint::new(1, 7));
}

struct VimTestAppContext<'a> {
    cx: &'a mut gpui::TestAppContext,
    window_id: usize,
    editor: ViewHandle<Editor>,
}

impl<'a> VimTestAppContext<'a> {
    async fn new(
        cx: &'a mut gpui::TestAppContext,
        initial_editor_text: &str,
    ) -> VimTestAppContext<'a> {
        cx.update(|cx| {
            editor::init(cx);
            crate::init(cx);
        });
        let params = cx.update(WorkspaceParams::test);
        params
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({ "dir": { "test.txt": initial_editor_text } }),
            )
            .await;

        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        params
            .project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root", true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;

        let file = cx.read(|cx| workspace.file_project_paths(cx)[0].clone());
        let item = workspace
            .update(cx, |workspace, cx| workspace.open_path(file, cx))
            .await
            .expect("Could not open test file");

        let editor = cx.update(|cx| {
            item.act_as::<Editor>(cx)
                .expect("Opened test file wasn't an editor")
        });
        editor.update(cx, |_, cx| cx.focus_self());

        Self {
            cx,
            window_id,
            editor,
        }
    }

    fn newest_selection(&mut self) -> Selection<DisplayPoint> {
        self.editor.update(self.cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            editor
                .newest_selection::<Point>(cx)
                .map(|point| point.to_display_point(&snapshot.display_snapshot))
        })
    }

    fn mode(&mut self) -> Mode {
        self.cx.update(|cx| cx.global::<VimState>().mode)
    }

    fn editor_text(&mut self) -> String {
        self.editor
            .update(self.cx, |editor, cx| editor.snapshot(cx).text())
    }

    fn simulate_keystroke(&mut self, keystroke_text: &str) {
        let keystroke = Keystroke::parse(keystroke_text).unwrap();
        let input = if keystroke.modified() {
            None
        } else {
            Some(keystroke.key.clone())
        };
        self.cx
            .dispatch_keystroke(self.window_id, keystroke, input, false);
    }

    fn simulate_keystrokes(&mut self, keystroke_texts: &[&str]) {
        for keystroke_text in keystroke_texts.into_iter() {
            self.simulate_keystroke(keystroke_text);
        }
    }
}

impl<'a> Deref for VimTestAppContext<'a> {
    type Target = gpui::TestAppContext;

    fn deref(&self) -> &Self::Target {
        self.cx
    }
}
