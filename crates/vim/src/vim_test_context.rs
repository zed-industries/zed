use std::ops::Deref;

use editor::{display_map::ToDisplayPoint, Bias, DisplayPoint};
use gpui::{json::json, keymap::Keystroke, ViewHandle};
use language::{Point, Selection};
use util::test::marked_text;
use workspace::{WorkspaceHandle, WorkspaceParams};

use crate::*;

pub struct VimTestContext<'a> {
    cx: &'a mut gpui::TestAppContext,
    window_id: usize,
    editor: ViewHandle<Editor>,
}

impl<'a> VimTestContext<'a> {
    pub async fn new(
        cx: &'a mut gpui::TestAppContext,
        enabled: bool,
        initial_editor_text: &str,
    ) -> VimTestContext<'a> {
        cx.update(|cx| {
            editor::init(cx);
            crate::init(cx);

            settings::KeyMapFile::load("keymaps/vim.json", cx).unwrap();
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

    pub fn enable_vim(&mut self) {
        self.cx.update(|cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.vim_mode = true;
            });
        })
    }

    pub fn disable_vim(&mut self) {
        self.cx.update(|cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.vim_mode = false;
            });
        })
    }

    pub fn newest_selection(&mut self) -> Selection<DisplayPoint> {
        self.editor.update(self.cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            editor
                .newest_selection::<Point>(cx)
                .map(|point| point.to_display_point(&snapshot.display_snapshot))
        })
    }

    pub fn mode(&mut self) -> Mode {
        self.cx.update(|cx| cx.global::<VimState>().mode)
    }

    pub fn editor_text(&mut self) -> String {
        self.editor
            .update(self.cx, |editor, cx| editor.snapshot(cx).text())
    }

    pub fn simulate_keystroke(&mut self, keystroke_text: &str) {
        let keystroke = Keystroke::parse(keystroke_text).unwrap();
        let input = if keystroke.modified() {
            None
        } else {
            Some(keystroke.key.clone())
        };
        self.cx
            .dispatch_keystroke(self.window_id, keystroke, input, false);
    }

    pub fn simulate_keystrokes(&mut self, keystroke_texts: &[&str]) {
        for keystroke_text in keystroke_texts.into_iter() {
            self.simulate_keystroke(keystroke_text);
        }
    }

    pub fn assert_newest_selection_head_offset(&mut self, expected_offset: usize) {
        let actual_head = self.newest_selection().head();
        let (actual_offset, expected_head) = self.editor.update(self.cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            (
                actual_head.to_offset(&snapshot, Bias::Left),
                expected_offset.to_display_point(&snapshot),
            )
        });
        let mut actual_position_text = self.editor_text();
        let mut expected_position_text = actual_position_text.clone();
        actual_position_text.insert(actual_offset, '|');
        expected_position_text.insert(expected_offset, '|');
        assert_eq!(
            actual_head, expected_head,
            "\nActual Position: {}\nExpected Position: {}",
            actual_position_text, expected_position_text
        )
    }

    pub fn assert_editor_state(&mut self, text: &str) {
        let (unmarked_text, markers) = marked_text(&text);
        let editor_text = self.editor_text();
        assert_eq!(
            editor_text, unmarked_text,
            "Unmarked text doesn't match editor text"
        );
        let expected_offset = markers[0];
        let actual_head = self.newest_selection().head();
        let (actual_offset, expected_head) = self.editor.update(self.cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            (
                actual_head.to_offset(&snapshot, Bias::Left),
                expected_offset.to_display_point(&snapshot),
            )
        });
        let mut actual_position_text = self.editor_text();
        let mut expected_position_text = actual_position_text.clone();
        actual_position_text.insert(actual_offset, '|');
        expected_position_text.insert(expected_offset, '|');
        assert_eq!(
            actual_head, expected_head,
            "\nActual Position: {}\nExpected Position: {}",
            actual_position_text, expected_position_text
        )
    }
}

impl<'a> Deref for VimTestContext<'a> {
    type Target = gpui::TestAppContext;

    fn deref(&self) -> &Self::Target {
        self.cx
    }
}
