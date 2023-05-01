use std::ops::{Deref, DerefMut};

use editor::test::{
    editor_lsp_test_context::EditorLspTestContext, editor_test_context::EditorTestContext,
};
use gpui::ContextHandle;
use search::{BufferSearchBar, ProjectSearchBar};

use crate::{state::Operator, *};

use super::VimBindingTestContext;

pub struct VimTestContext<'a> {
    cx: EditorLspTestContext<'a>,
}

impl<'a> VimTestContext<'a> {
    pub async fn new(cx: &'a mut gpui::TestAppContext, enabled: bool) -> VimTestContext<'a> {
        let mut cx = EditorLspTestContext::new_rust(Default::default(), cx).await;
        cx.update(|cx| {
            cx.update_global(|settings: &mut Settings, _| {
                settings.vim_mode = enabled;
            });
            search::init(cx);
            crate::init(cx);

            settings::KeymapFileContent::load("keymaps/vim.json", cx).unwrap();
        });

        // Setup search toolbars and keypress hook
        cx.update_workspace(|workspace, cx| {
            observe_keystrokes(cx);
            workspace.active_pane().update(cx, |pane, cx| {
                pane.toolbar().update(cx, |toolbar, cx| {
                    let buffer_search_bar = cx.add_view(BufferSearchBar::new);
                    toolbar.add_item(buffer_search_bar, cx);
                    let project_search_bar = cx.add_view(|_| ProjectSearchBar::new());
                    toolbar.add_item(project_search_bar, cx);
                })
            });
        });

        Self { cx }
    }

    pub fn workspace<F, T>(&mut self, read: F) -> T
    where
        F: FnOnce(&Workspace, &ViewContext<Workspace>) -> T,
    {
        self.cx.workspace.read_with(self.cx.cx.cx, read)
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

    pub fn set_state(&mut self, text: &str, mode: Mode) -> ContextHandle {
        let window_id = self.window_id;
        self.update_window(window_id, |cx| {
            Vim::update(cx, |vim, cx| {
                vim.switch_mode(mode, false, cx);
            })
        });
        self.cx.set_state(text)
    }

    pub fn assert_state(&mut self, text: &str, mode: Mode) {
        self.assert_editor_state(text);
        assert_eq!(self.mode(), mode, "{}", self.assertion_context());
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
        assert_eq!(self.mode(), mode_after, "{}", self.assertion_context());
        assert_eq!(self.active_operator(), None, "{}", self.assertion_context());
    }

    pub fn binding<const COUNT: usize>(
        mut self,
        keystrokes: [&'static str; COUNT],
    ) -> VimBindingTestContext<'a, COUNT> {
        let mode = self.mode();
        VimBindingTestContext::new(keystrokes, mode, mode, self)
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
