use gpui::{AppContext as _, UpdateGlobal, px, size};
use indoc::indoc;
use settings::SettingsStore;
use std::{
    ops::{Deref, DerefMut},
    panic, thread,
};

use language::language_settings::SoftWrap;
use util::test::marked_text_offsets;

use super::{VimTestContext, neovim_connection::NeovimConnection};
use crate::state::{Mode, VimGlobals};

pub struct NeovimBackedTestContext {
    pub(crate) cx: VimTestContext,
    pub(crate) neovim: NeovimConnection,

    last_set_state: Option<String>,
    recent_keystrokes: Vec<String>,
}

#[derive(Default)]
pub struct SharedState {
    neovim: String,
    editor: String,
    initial: String,
    neovim_mode: Mode,
    editor_mode: Mode,
    recent_keystrokes: String,
}

impl SharedState {
    /// Assert that both Zed and NeoVim have the same content and mode.
    #[track_caller]
    pub fn assert_matches(&self) {
        if self.neovim != self.editor || self.neovim_mode != self.editor_mode {
            panic!(
                indoc! {"Test failed (zed does not match nvim behavior)
                    # initial state:
                    {}
                    # keystrokes:
                    {}
                    # neovim ({}):
                    {}
                    # zed ({}):
                    {}"},
                self.initial,
                self.recent_keystrokes,
                self.neovim_mode,
                self.neovim,
                self.editor_mode,
                self.editor,
            )
        }
    }

    #[track_caller]
    pub fn assert_eq(&mut self, marked_text: &str) {
        let marked_text = marked_text.replace('•', " ");
        if self.neovim == marked_text
            && self.neovim == self.editor
            && self.neovim_mode == self.editor_mode
        {
            return;
        }

        let message = if self.neovim != marked_text {
            "Test is incorrect (currently expected != neovim_state)"
        } else {
            "Editor does not match nvim behavior"
        };
        panic!(
            indoc! {"{}
                # initial state:
                {}
                # keystrokes:
                {}
                # currently expected:
                {}
                # neovim ({}):
                {}
                # zed ({}):
                {}"},
            message,
            self.initial,
            self.recent_keystrokes,
            marked_text.replace(" \n", "•\n"),
            self.neovim_mode,
            self.neovim.replace(" \n", "•\n"),
            self.editor_mode,
            self.editor.replace(" \n", "•\n"),
        )
    }
}

pub struct SharedClipboard {
    register: char,
    neovim: String,
    editor: String,
    state: SharedState,
}

impl SharedClipboard {
    #[track_caller]
    pub fn assert_eq(&self, expected: &str) {
        if expected == self.neovim && self.neovim == self.editor {
            return;
        }

        let message = if expected != self.neovim {
            "Test is incorrect (currently expected != neovim_state)"
        } else {
            "Editor does not match nvim behavior"
        };

        panic!(
            indoc! {"{}
                # initial state:
                {}
                # keystrokes:
                {}
                # currently expected: {:?}
                # neovim register \"{}: {:?}
                # zed register \"{}: {:?}"},
            message,
            self.state.initial,
            self.state.recent_keystrokes,
            expected,
            self.register,
            self.neovim,
            self.register,
            self.editor
        )
    }
}

impl NeovimBackedTestContext {
    pub async fn new(cx: &mut gpui::TestAppContext) -> NeovimBackedTestContext {
        #[cfg(feature = "neovim")]
        cx.executor().allow_parking();
        // rust stores the name of the test on the current thread.
        // We use this to automatically name a file that will store
        // the neovim connection's requests/responses so that we can
        // run without neovim on CI.
        let thread = thread::current();
        let test_name = thread
            .name()
            .expect("thread is not named")
            .split(':')
            .next_back()
            .unwrap()
            .to_string();
        Self {
            cx: VimTestContext::new(cx, true).await,
            neovim: NeovimConnection::new(test_name).await,

            last_set_state: None,
            recent_keystrokes: Default::default(),
        }
    }

    pub async fn new_html(cx: &mut gpui::TestAppContext) -> NeovimBackedTestContext {
        #[cfg(feature = "neovim")]
        cx.executor().allow_parking();
        // rust stores the name of the test on the current thread.
        // We use this to automatically name a file that will store
        // the neovim connection's requests/responses so that we can
        // run without neovim on CI.
        let thread = thread::current();
        let test_name = thread
            .name()
            .expect("thread is not named")
            .split(':')
            .next_back()
            .unwrap()
            .to_string();
        Self {
            cx: VimTestContext::new_html(cx).await,
            neovim: NeovimConnection::new(test_name).await,

            last_set_state: None,
            recent_keystrokes: Default::default(),
        }
    }

    pub async fn new_markdown_with_rust(cx: &mut gpui::TestAppContext) -> NeovimBackedTestContext {
        #[cfg(feature = "neovim")]
        cx.executor().allow_parking();
        let thread = thread::current();
        let test_name = thread
            .name()
            .expect("thread is not named")
            .split(':')
            .next_back()
            .unwrap()
            .to_string();
        Self {
            cx: VimTestContext::new_markdown_with_rust(cx).await,
            neovim: NeovimConnection::new(test_name).await,

            last_set_state: None,
            recent_keystrokes: Default::default(),
        }
    }

    pub async fn new_typescript(cx: &mut gpui::TestAppContext) -> NeovimBackedTestContext {
        #[cfg(feature = "neovim")]
        cx.executor().allow_parking();
        // rust stores the name of the test on the current thread.
        // We use this to automatically name a file that will store
        // the neovim connection's requests/responses so that we can
        // run without neovim on CI.
        let thread = thread::current();
        let test_name = thread
            .name()
            .expect("thread is not named")
            .split(':')
            .next_back()
            .unwrap()
            .to_string();
        Self {
            cx: VimTestContext::new_typescript(cx).await,
            neovim: NeovimConnection::new(test_name).await,

            last_set_state: None,
            recent_keystrokes: Default::default(),
        }
    }

    pub async fn new_tsx(cx: &mut gpui::TestAppContext) -> NeovimBackedTestContext {
        #[cfg(feature = "neovim")]
        cx.executor().allow_parking();
        let thread = thread::current();
        let test_name = thread
            .name()
            .expect("thread is not named")
            .split(':')
            .next_back()
            .unwrap()
            .to_string();
        Self {
            cx: VimTestContext::new_tsx(cx).await,
            neovim: NeovimConnection::new(test_name).await,

            last_set_state: None,
            recent_keystrokes: Default::default(),
        }
    }

    pub async fn set_shared_state(&mut self, marked_text: &str) {
        let mode = if marked_text.contains('»') {
            Mode::Visual
        } else {
            Mode::Normal
        };
        self.set_state(marked_text, mode);
        self.last_set_state = Some(marked_text.to_string());
        self.recent_keystrokes = Vec::new();
        self.neovim.set_state(marked_text).await;
    }

    pub async fn simulate_shared_keystrokes(&mut self, keystroke_texts: &str) {
        for keystroke_text in keystroke_texts.split(' ') {
            self.recent_keystrokes.push(keystroke_text.to_string());
            self.neovim.send_keystroke(keystroke_text).await;
        }
        self.simulate_keystrokes(keystroke_texts);
    }

    #[must_use]
    pub async fn simulate(&mut self, keystrokes: &str, initial_state: &str) -> SharedState {
        self.set_shared_state(initial_state).await;
        self.simulate_shared_keystrokes(keystrokes).await;
        self.shared_state().await
    }

    pub async fn set_shared_wrap(&mut self, columns: u32) {
        if columns < 12 {
            panic!("nvim doesn't support columns < 12")
        }
        self.neovim.set_option("wrap").await;
        self.neovim
            .set_option(&format!("columns={}", columns))
            .await;

        self.update(|_, cx| {
            SettingsStore::update_global(cx, |settings, cx| {
                settings.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.soft_wrap =
                        Some(SoftWrap::PreferredLineLength);
                    settings
                        .project
                        .all_languages
                        .defaults
                        .preferred_line_length = Some(columns);
                });
            })
        })
    }

    pub async fn set_scroll_height(&mut self, rows: u32) {
        // match Zed's scrolling behavior
        self.neovim.set_option(&format!("scrolloff={}", 3)).await;
        // +2 to account for the vim command UI at the bottom.
        self.neovim.set_option(&format!("lines={}", rows + 2)).await;
        let (line_height, visible_line_count) = self.update_editor(|editor, window, cx| {
            (
                editor
                    .style(cx)
                    .text
                    .line_height_in_pixels(window.rem_size()),
                editor.visible_line_count().unwrap(),
            )
        });

        let window = self.window;
        let margin = self
            .update_window(window, |_, window, _cx| {
                window.viewport_size().height - line_height * (visible_line_count as f32)
            })
            .unwrap();

        self.simulate_window_resize(
            self.window,
            size(px(1000.), margin + (rows as f32) * line_height),
        );
    }

    pub async fn set_neovim_option(&mut self, option: &str) {
        self.neovim.set_option(option).await;
    }

    #[must_use]
    pub async fn shared_clipboard(&mut self) -> SharedClipboard {
        SharedClipboard {
            register: '"',
            state: self.shared_state().await,
            neovim: self.neovim.read_register('"').await,
            editor: self.read_from_clipboard().unwrap().text().unwrap(),
        }
    }

    #[must_use]
    pub async fn shared_register(&mut self, register: char) -> SharedClipboard {
        SharedClipboard {
            register,
            state: self.shared_state().await,
            neovim: self.neovim.read_register(register).await,
            editor: self.update(|_, cx| {
                cx.global::<VimGlobals>()
                    .registers
                    .get(&register)
                    .cloned()
                    .unwrap_or_default()
                    .text
                    .into()
            }),
        }
    }

    #[must_use]
    pub async fn shared_state(&mut self) -> SharedState {
        let (mode, marked_text) = self.neovim.state().await;
        SharedState {
            neovim: marked_text,
            neovim_mode: mode,
            editor: self.editor_state(),
            editor_mode: self.mode(),
            initial: self
                .last_set_state
                .as_ref()
                .cloned()
                .unwrap_or("N/A".to_string()),
            recent_keystrokes: self.recent_keystrokes.join(" "),
        }
    }

    #[must_use]
    pub async fn simulate_at_each_offset(
        &mut self,
        keystrokes: &str,
        marked_positions: &str,
    ) -> SharedState {
        let (unmarked_text, cursor_offsets) = marked_text_offsets(marked_positions);

        for cursor_offset in cursor_offsets.iter() {
            let mut marked_text = unmarked_text.clone();
            marked_text.insert(*cursor_offset, 'ˇ');

            let state = self.simulate(keystrokes, &marked_text).await;
            if state.neovim != state.editor || state.neovim_mode != state.editor_mode {
                return state;
            }
        }

        SharedState::default()
    }
}

impl Deref for NeovimBackedTestContext {
    type Target = VimTestContext;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl DerefMut for NeovimBackedTestContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}

#[cfg(test)]
mod test {
    use crate::test::NeovimBackedTestContext;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn neovim_backed_test_context_works(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.shared_state().await.assert_matches();
        cx.set_shared_state("This is a tesˇt").await;
        cx.shared_state().await.assert_matches();
    }
}
