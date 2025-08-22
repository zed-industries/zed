use std::ops::{Deref, DerefMut};

use editor::test::editor_lsp_test_context::EditorLspTestContext;
use gpui::{Context, Entity, SemanticVersion, UpdateGlobal};
use search::{BufferSearchBar, project_search::ProjectSearchBar};

use crate::{state::Operator, *};

pub struct VimTestContext {
    cx: EditorLspTestContext,
}

impl VimTestContext {
    pub fn init(cx: &mut gpui::TestAppContext) {
        if cx.has_global::<VimGlobals>() {
            return;
        }
        env_logger::try_init().ok();
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            release_channel::init(SemanticVersion::default(), cx);
            command_palette::init(cx);
            project_panel::init(cx);
            git_ui::init(cx);
            crate::init(cx);
            search::init(cx);
            workspace::init_settings(cx);
            language::init(cx);
            editor::init_settings(cx);
            project::Project::init_settings(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
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

    pub fn init_keybindings(enabled: bool, cx: &mut App) {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings::<VimModeSetting>(cx, |s| *s = Some(enabled));
        });
        let default_key_bindings = settings::KeymapFile::load_asset_allow_partial_failure(
            "keymaps/default-macos.json",
            cx,
        )
        .unwrap();
        cx.bind_keys(default_key_bindings);
        if enabled {
            let vim_key_bindings = settings::KeymapFile::load_asset(
                "keymaps/vim.json",
                Some(settings::KeybindSource::Vim),
                cx,
            )
            .unwrap();
            cx.bind_keys(vim_key_bindings);
        }
    }

    pub fn new_with_lsp(mut cx: EditorLspTestContext, enabled: bool) -> VimTestContext {
        cx.update(|_, cx| {
            Self::init_keybindings(enabled, cx);
        });

        // Setup search toolbars and keypress hook
        cx.update_workspace(|workspace, window, cx| {
            workspace.active_pane().update(cx, |pane, cx| {
                pane.toolbar().update(cx, |toolbar, cx| {
                    let buffer_search_bar = cx.new(|cx| BufferSearchBar::new(None, window, cx));
                    toolbar.add_item(buffer_search_bar, window, cx);

                    let project_search_bar = cx.new(|_| ProjectSearchBar::new());
                    toolbar.add_item(project_search_bar, window, cx);
                })
            });
            workspace.status_bar().update(cx, |status_bar, cx| {
                let vim_mode_indicator = cx.new(|cx| ModeIndicator::new(window, cx));
                status_bar.add_right_item(vim_mode_indicator, window, cx);
            });
        });

        Self { cx }
    }

    pub fn update_entity<F, T, R>(&mut self, entity: Entity<T>, update: F) -> R
    where
        T: 'static,
        F: FnOnce(&mut T, &mut Window, &mut Context<T>) -> R + 'static,
    {
        let window = self.window;
        self.update_window(window, move |_, window, cx| {
            entity.update(cx, |t, cx| update(t, window, cx))
        })
        .unwrap()
    }

    pub fn workspace<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut Workspace, &mut Window, &mut Context<Workspace>) -> T,
    {
        self.cx.update_workspace(update)
    }

    pub fn enable_vim(&mut self) {
        self.cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<VimModeSetting>(cx, |s| *s = Some(true));
            });
        })
    }

    pub fn disable_vim(&mut self) {
        self.cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<VimModeSetting>(cx, |s| *s = Some(false));
            });
        })
    }

    pub fn enable_helix(&mut self) {
        self.cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<vim_mode_setting::HelixModeSetting>(cx, |s| {
                    *s = Some(true)
                });
            });
        })
    }

    pub fn mode(&mut self) -> Mode {
        self.update_editor(|editor, _, cx| editor.addon::<VimAddon>().unwrap().entity.read(cx).mode)
    }

    pub fn forced_motion(&mut self) -> bool {
        self.update_editor(|_, _, cx| cx.global::<VimGlobals>().forced_motion)
    }

    pub fn active_operator(&mut self) -> Option<Operator> {
        self.update_editor(|editor, _, cx| {
            editor
                .addon::<VimAddon>()
                .unwrap()
                .entity
                .read(cx)
                .operator_stack
                .last()
                .cloned()
        })
    }

    pub fn set_state(&mut self, text: &str, mode: Mode) {
        self.cx.set_state(text);
        let vim =
            self.update_editor(|editor, _window, _cx| editor.addon::<VimAddon>().cloned().unwrap());

        self.update(|window, cx| {
            vim.entity.update(cx, |vim, cx| {
                vim.switch_mode(mode, true, window, cx);
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

    pub fn shared_clipboard(&mut self) -> VimClipboard {
        VimClipboard {
            editor: self
                .read_from_clipboard()
                .map(|item| item.text().unwrap())
                .unwrap_or_default(),
        }
    }
}

pub struct VimClipboard {
    editor: String,
}

impl VimClipboard {
    #[track_caller]
    pub fn assert_eq(&self, expected: &str) {
        assert_eq!(self.editor, expected);
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
