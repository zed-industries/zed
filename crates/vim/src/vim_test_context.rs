use std::ops::{Deref, DerefMut};

use editor::test::EditorTestContext;
use gpui::json::json;
use project::Project;
use workspace::{pane, AppState, WorkspaceHandle};

use crate::{state::Operator, *};

pub struct VimTestContext<'a> {
    cx: EditorTestContext<'a>,
}

impl<'a> VimTestContext<'a> {
    pub async fn new(cx: &'a mut gpui::TestAppContext, enabled: bool) -> VimTestContext<'a> {
        cx.update(|cx| {
            editor::init(cx);
            pane::init(cx);
            crate::init(cx);

            settings::KeymapFileContent::load("keymaps/vim.json", cx).unwrap();
        });

        let params = cx.update(AppState::test);
        let project = Project::test(params.fs.clone(), [], cx).await;

        cx.update(|cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.vim_mode = enabled;
            });
        });

        params
            .fs
            .as_fake()
            .insert_tree("/root", json!({ "dir": { "test.txt": "" } }))
            .await;

        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(project.clone(), cx));
        project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root", true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;

        let file = cx.read(|cx| workspace.file_project_paths(cx)[0].clone());
        let item = workspace
            .update(cx, |workspace, cx| workspace.open_path(file, true, cx))
            .await
            .expect("Could not open test file");

        let editor = cx.update(|cx| {
            item.act_as::<Editor>(cx)
                .expect("Opened test file wasn't an editor")
        });
        editor.update(cx, |_, cx| cx.focus_self());

        Self {
            cx: EditorTestContext {
                cx,
                window_id,
                editor,
            },
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

    pub fn mode(&mut self) -> Mode {
        self.cx.read(|cx| cx.global::<Vim>().state.mode)
    }

    pub fn active_operator(&mut self) -> Option<Operator> {
        self.cx
            .read(|cx| cx.global::<Vim>().state.operator_stack.last().copied())
    }

    pub fn set_state(&mut self, text: &str, mode: Mode) {
        self.cx.update(|cx| {
            Vim::update(cx, |vim, cx| {
                vim.switch_mode(mode, cx);
            })
        });
        self.cx.set_state(text);
    }

    pub fn assert_binding<const COUNT: usize>(
        &mut self,
        keystrokes: [&str; COUNT],
        initial_state: &str,
        initial_mode: Mode,
        state_after: &str,
        mode_after: Mode,
    ) {
        self.set_state(initial_state, initial_mode);
        self.cx.simulate_keystrokes(keystrokes);
        self.cx.assert_editor_state(state_after);
        assert_eq!(self.mode(), mode_after);
        assert_eq!(self.active_operator(), None);
    }

    pub fn binding<const COUNT: usize>(
        mut self,
        keystrokes: [&'static str; COUNT],
    ) -> VimBindingTestContext<'a, COUNT> {
        let mode = self.mode();
        VimBindingTestContext::new(keystrokes, mode, mode, self)
    }

    pub fn assert_clipboard_content(&mut self, expected_content: Option<&str>) {
        self.cx.update(|cx| {
            let actual_content = cx.read_from_clipboard().map(|item| item.text().to_owned());
            let expected_content = expected_content.map(|content| content.to_owned());
            assert_eq!(actual_content, expected_content);
        })
    }
}

impl<'a> Deref for VimTestContext<'a> {
    type Target = EditorTestContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl<'a> DerefMut for VimTestContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}

pub struct VimBindingTestContext<'a, const COUNT: usize> {
    cx: VimTestContext<'a>,
    keystrokes_under_test: [&'static str; COUNT],
    mode_before: Mode,
    mode_after: Mode,
}

impl<'a, const COUNT: usize> VimBindingTestContext<'a, COUNT> {
    pub fn new(
        keystrokes_under_test: [&'static str; COUNT],
        mode_before: Mode,
        mode_after: Mode,
        cx: VimTestContext<'a>,
    ) -> Self {
        Self {
            cx,
            keystrokes_under_test,
            mode_before,
            mode_after,
        }
    }

    pub fn binding<const NEW_COUNT: usize>(
        self,
        keystrokes_under_test: [&'static str; NEW_COUNT],
    ) -> VimBindingTestContext<'a, NEW_COUNT> {
        VimBindingTestContext {
            keystrokes_under_test,
            cx: self.cx,
            mode_before: self.mode_before,
            mode_after: self.mode_after,
        }
    }

    pub fn mode_after(mut self, mode_after: Mode) -> Self {
        self.mode_after = mode_after;
        self
    }

    pub fn assert(&mut self, initial_state: &str, state_after: &str) {
        self.cx.assert_binding(
            self.keystrokes_under_test,
            initial_state,
            self.mode_before,
            state_after,
            self.mode_after,
        )
    }
}

impl<'a, const COUNT: usize> Deref for VimBindingTestContext<'a, COUNT> {
    type Target = VimTestContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl<'a, const COUNT: usize> DerefMut for VimBindingTestContext<'a, COUNT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}
