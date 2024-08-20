use std::ops::{Deref, DerefMut};

use editor::test::editor_lsp_test_context::EditorLspTestContext;
use gpui::{Context, SemanticVersion, UpdateGlobal, View, VisualContext};
use search::{project_search::ProjectSearchBar, BufferSearchBar};

use crate::{state::Operator, *};

pub struct VimTestContext {
    cx: EditorLspTestContext,
}

impl VimTestContext {
    pub fn init(cx: &mut gpui::TestAppContext) {
        if cx.has_global::<VimGlobals>() {
            return;
        }
        cx.update(|cx| {
            search::init(cx);
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            release_channel::init(SemanticVersion::default(), cx);
            command_palette::init(cx);
            crate::init(cx);
        });
    }

    pub async fn new(cx: &mut gpui::TestAppContext, enabled: bool) -> VimTestContext {
        Self::init(cx);
        let lsp = EditorLspTestContext::new_rust(Default::default(), cx).await;
        Self::new_with_lsp(lsp, enabled)
    }

    pub async fn new_html(cx: &mut gpui::TestAppContext) -> VimTestContext {
        Self::init(cx);
        Self::new_with_lsp(EditorLspTestContext::new_html(cx).await, true)
    }

    pub async fn new_typescript(cx: &mut gpui::TestAppContext) -> VimTestContext {
        Self::init(cx);
        Self::new_with_lsp(
            EditorLspTestContext::new_typescript(
                lsp::ServerCapabilities {
                    rename_provider: Some(lsp::OneOf::Right(lsp::RenameOptions {
                        prepare_provider: Some(true),
                        work_done_progress_options: Default::default(),
                    })),
                    ..Default::default()
                },
                cx,
            )
            .await,
            true,
        )
    }

    pub fn new_with_lsp(mut cx: EditorLspTestContext, enabled: bool) -> VimTestContext {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<VimModeSetting>(cx, |s| *s = Some(enabled));
            });
            settings::KeymapFile::load_asset("keymaps/default-macos.json", cx).unwrap();
            if enabled {
                settings::KeymapFile::load_asset("keymaps/vim.json", cx).unwrap();
            }
        });

        // Setup search toolbars and keypress hook
        cx.update_workspace(|workspace, cx| {
            workspace.active_pane().update(cx, |pane, cx| {
                pane.toolbar().update(cx, |toolbar, cx| {
                    let buffer_search_bar = cx.new_view(BufferSearchBar::new);
                    toolbar.add_item(buffer_search_bar, cx);

                    let project_search_bar = cx.new_view(|_| ProjectSearchBar::new());
                    toolbar.add_item(project_search_bar, cx);
                })
            });
            workspace.status_bar().update(cx, |status_bar, cx| {
                let vim_mode_indicator = cx.new_view(ModeIndicator::new);
                status_bar.add_right_item(vim_mode_indicator, cx);
            });
        });

        Self { cx }
    }

    pub fn update_view<F, T, R>(&mut self, view: View<T>, update: F) -> R
    where
        T: 'static,
        F: FnOnce(&mut T, &mut ViewContext<T>) -> R + 'static,
    {
        let window = self.window;
        self.update_window(window, move |_, cx| view.update(cx, update))
            .unwrap()
    }

    pub fn workspace<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut Workspace, &mut ViewContext<Workspace>) -> T,
    {
        self.cx.update_workspace(update)
    }

    pub fn enable_vim(&mut self) {
        self.cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<VimModeSetting>(cx, |s| *s = Some(true));
            });
        })
    }

    pub fn disable_vim(&mut self) {
        self.cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<VimModeSetting>(cx, |s| *s = Some(false));
            });
        })
    }

    pub fn mode(&mut self) -> Mode {
        self.update_editor(|editor, cx| editor.addon::<VimAddon>().unwrap().view.read(cx).mode)
    }

    pub fn active_operator(&mut self) -> Option<Operator> {
        self.update_editor(|editor, cx| {
            editor
                .addon::<VimAddon>()
                .unwrap()
                .view
                .read(cx)
                .operator_stack
                .last()
                .cloned()
        })
    }

    pub fn set_state(&mut self, text: &str, mode: Mode) {
        self.cx.set_state(text);
        let vim = self.update_editor(|editor, _cx| editor.addon::<VimAddon>().cloned().unwrap());

        self.update(|cx| {
            vim.view.update(cx, |vim, cx| {
                vim.switch_mode(mode, true, cx);
            });
        });
        self.cx.cx.cx.run_until_parked();
    }

    #[track_caller]
    pub fn assert_state(&mut self, text: &str, mode: Mode) {
        self.assert_editor_state(text);
        assert_eq!(self.mode(), mode, "{}", self.assertion_context());
    }

    pub fn assert_binding(
        &mut self,
        keystrokes: &str,
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

    pub fn assert_binding_normal(
        &mut self,
        keystrokes: &str,
        initial_state: &str,
        state_after: &str,
    ) {
        self.set_state(initial_state, Mode::Normal);
        self.cx.simulate_keystrokes(keystrokes);
        self.cx.assert_editor_state(state_after);
        assert_eq!(self.mode(), Mode::Normal, "{}", self.assertion_context());
        assert_eq!(self.active_operator(), None, "{}", self.assertion_context());
    }
}

impl Deref for VimTestContext {
    type Target = EditorLspTestContext;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl DerefMut for VimTestContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}
