use indoc::indoc;
use std::ops::Deref;

use editor::{display_map::ToDisplayPoint, DisplayPoint};
use gpui::{json::json, keymap::Keystroke, ViewHandle};
use language::{Point, Selection};
use util::test::marked_text;
use workspace::{WorkspaceHandle, WorkspaceParams};

use crate::*;

#[gpui::test]
async fn test_insert_mode(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestAppContext::new(cx, true, "").await;
    cx.simulate_keystroke("i");
    assert_eq!(cx.mode(), Mode::Insert);
    cx.simulate_keystrokes(&["T", "e", "s", "t"]);
    cx.assert_newest_selection_head("Test|");
    cx.simulate_keystroke("escape");
    assert_eq!(cx.mode(), Mode::Normal);
    cx.assert_newest_selection_head("Tes|t");
}

#[gpui::test]
async fn test_normal_hjkl(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestAppContext::new(cx, true, "Test\nTestTest\nTest").await;
    cx.simulate_keystroke("l");
    cx.assert_newest_selection_head(indoc! {"
        T|est
        TestTest
        Test"});
    cx.simulate_keystroke("h");
    cx.assert_newest_selection_head(indoc! {"
        |Test
        TestTest
        Test"});
    cx.simulate_keystroke("j");
    cx.assert_newest_selection_head(indoc! {"
        Test
        |TestTest
        Test"});
    cx.simulate_keystroke("k");
    cx.assert_newest_selection_head(indoc! {"
        |Test
        TestTest
        Test"});
    cx.simulate_keystroke("j");
    cx.assert_newest_selection_head(indoc! {"
        Test
        |TestTest
        Test"});

    // When moving left, cursor does not wrap to the previous line
    cx.simulate_keystroke("h");
    cx.assert_newest_selection_head(indoc! {"
        Test
        |TestTest
        Test"});

    // When moving right, cursor does not reach the line end or wrap to the next line
    for _ in 0..9 {
        cx.simulate_keystroke("l");
    }
    cx.assert_newest_selection_head(indoc! {"
        Test
        TestTes|t
        Test"});

    // Goal column respects the inability to reach the end of the line
    cx.simulate_keystroke("k");
    cx.assert_newest_selection_head(indoc! {"
        Tes|t
        TestTest
        Test"});
    cx.simulate_keystroke("j");
    cx.assert_newest_selection_head(indoc! {"
        Test
        TestTes|t
        Test"});
}

#[gpui::test]
async fn test_toggle_through_settings(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestAppContext::new(cx, true, "").await;

    cx.simulate_keystroke("i");
    assert_eq!(cx.mode(), Mode::Insert);

    // Editor acts as though vim is disabled
    cx.disable_vim();
    cx.simulate_keystrokes(&["h", "j", "k", "l"]);
    cx.assert_newest_selection_head("hjkl|");

    // Enabling dynamically sets vim mode again and restores normal mode
    cx.enable_vim();
    assert_eq!(cx.mode(), Mode::Normal);
    cx.simulate_keystrokes(&["h", "h", "h", "l"]);
    assert_eq!(cx.editor_text(), "hjkl".to_owned());
    cx.assert_newest_selection_head("hj|kl");
    cx.simulate_keystrokes(&["i", "T", "e", "s", "t"]);
    cx.assert_newest_selection_head("hjTest|kl");

    // Disabling and enabling resets to normal mode
    assert_eq!(cx.mode(), Mode::Insert);
    cx.disable_vim();
    assert_eq!(cx.mode(), Mode::Insert);
    cx.enable_vim();
    assert_eq!(cx.mode(), Mode::Normal);
}

#[gpui::test]
async fn test_initially_disabled(cx: &mut gpui::TestAppContext) {
    let mut cx = VimTestAppContext::new(cx, false, "").await;
    cx.simulate_keystrokes(&["h", "j", "k", "l"]);
    cx.assert_newest_selection_head("hjkl|");
}

struct VimTestAppContext<'a> {
    cx: &'a mut gpui::TestAppContext,
    window_id: usize,
    editor: ViewHandle<Editor>,
}

impl<'a> VimTestAppContext<'a> {
    async fn new(
        cx: &'a mut gpui::TestAppContext,
        enabled: bool,
        initial_editor_text: &str,
    ) -> VimTestAppContext<'a> {
        cx.update(|cx| {
            editor::init(cx);
            crate::init(cx);
        });
        let params = cx.update(WorkspaceParams::test);

        cx.update(|cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.vim_mode = enabled;
            });
        });

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

    fn enable_vim(&mut self) {
        self.cx.update(|cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.vim_mode = true;
            });
        })
    }

    fn disable_vim(&mut self) {
        self.cx.update(|cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.vim_mode = false;
            });
        })
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

    fn assert_newest_selection_head(&mut self, text: &str) {
        let (unmarked_text, markers) = marked_text(&text);
        assert_eq!(
            self.editor_text(),
            unmarked_text,
            "Unmarked text doesn't match editor text"
        );
        let newest_selection = self.newest_selection();
        let expected_head = self.editor.update(self.cx, |editor, cx| {
            markers[0].to_display_point(&editor.snapshot(cx))
        });
        assert_eq!(newest_selection.head(), expected_head)
    }
}

impl<'a> Deref for VimTestAppContext<'a> {
    type Target = gpui::TestAppContext;

    fn deref(&self) -> &Self::Target {
        self.cx
    }
}
