use editor::test::editor_test_context::ContextHandle;
use gpui::{px, size, BorrowAppContext, Context};
use indoc::indoc;
use settings::SettingsStore;
use std::{
    ops::{Deref, DerefMut},
    panic, thread,
};

use collections::{HashMap, HashSet};
use language::language_settings::{AllLanguageSettings, SoftWrap};
use util::test::marked_text_offsets;

use super::{neovim_connection::NeovimConnection, NeovimBackedBindingTestContext, VimTestContext};
use crate::state::Mode;

pub const SUPPORTED_FEATURES: &[ExemptionFeatures] = &[];

/// Enum representing features we have tests for but which don't work, yet. Used
/// to add exemptions and automatically
#[derive(PartialEq, Eq)]
pub enum ExemptionFeatures {
    // MOTIONS
    // When an operator completes at the end of the file, an extra newline is left
    OperatorLastNewlineRemains,

    // OBJECTS
    // Resulting position after the operation is slightly incorrect for unintuitive reasons.
    IncorrectLandingPosition,
    // Operator around the text object at the end of the line doesn't remove whitespace.
    AroundObjectLeavesWhitespaceAtEndOfLine,
    // Sentence object on empty lines
    SentenceOnEmptyLines,
    // Whitespace isn't included with text objects at the start of the line
    SentenceAtStartOfLineWithWhitespace,
    // Whitespace around sentences is slightly incorrect when starting between sentences
    AroundSentenceStartingBetweenIncludesWrongWhitespace,
    // Non empty selection with text objects in visual mode
    NonEmptyVisualTextObjects,
    // Sentence Doesn't backtrack when its at the end of the file
    SentenceAfterPunctuationAtEndOfFile,
}

impl ExemptionFeatures {
    pub fn supported(&self) -> bool {
        SUPPORTED_FEATURES.contains(self)
    }
}

pub struct NeovimBackedTestContext {
    cx: VimTestContext,
    // Lookup for exempted assertions. Keyed by the insertion text, and with a value indicating which
    // bindings are exempted. If None, all bindings are ignored for that insertion text.
    exemptions: HashMap<String, Option<HashSet<String>>>,
    pub(crate) neovim: NeovimConnection,

    last_set_state: Option<String>,
    recent_keystrokes: Vec<String>,

    is_dirty: bool,
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
            .last()
            .unwrap()
            .to_string();
        Self {
            cx: VimTestContext::new(cx, true).await,
            exemptions: Default::default(),
            neovim: NeovimConnection::new(test_name).await,

            last_set_state: None,
            recent_keystrokes: Default::default(),
            is_dirty: false,
        }
    }

    pub fn add_initial_state_exemptions(
        &mut self,
        marked_positions: &str,
        missing_feature: ExemptionFeatures, // Feature required to support this exempted test case
    ) {
        if !missing_feature.supported() {
            let (unmarked_text, cursor_offsets) = marked_text_offsets(marked_positions);

            for cursor_offset in cursor_offsets.iter() {
                let mut marked_text = unmarked_text.clone();
                marked_text.insert(*cursor_offset, 'ˇ');

                // None represents all key bindings being exempted for that initial state
                self.exemptions.insert(marked_text, None);
            }
        }
    }

    pub async fn simulate_shared_keystroke(&mut self, keystroke_text: &str) -> ContextHandle {
        self.neovim.send_keystroke(keystroke_text).await;
        self.simulate_keystroke(keystroke_text)
    }

    pub async fn simulate_shared_keystrokes<const COUNT: usize>(
        &mut self,
        keystroke_texts: [&str; COUNT],
    ) {
        for keystroke_text in keystroke_texts.into_iter() {
            self.recent_keystrokes.push(keystroke_text.to_string());
            self.neovim.send_keystroke(keystroke_text).await;
        }
        self.simulate_keystrokes(keystroke_texts);
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
        self.is_dirty = true;
    }

    pub async fn set_shared_wrap(&mut self, columns: u32) {
        if columns < 12 {
            panic!("nvim doesn't support columns < 12")
        }
        self.neovim.set_option("wrap").await;
        self.neovim
            .set_option(&format!("columns={}", columns))
            .await;

        self.update(|cx| {
            cx.update_global(|settings: &mut SettingsStore, cx| {
                settings.update_user_settings::<AllLanguageSettings>(cx, |settings| {
                    settings.defaults.soft_wrap = Some(SoftWrap::PreferredLineLength);
                    settings.defaults.preferred_line_length = Some(columns);
                });
            })
        })
    }

    pub async fn set_scroll_height(&mut self, rows: u32) {
        // match Zed's scrolling behavior
        self.neovim.set_option(&format!("scrolloff={}", 3)).await;
        // +2 to account for the vim command UI at the bottom.
        self.neovim.set_option(&format!("lines={}", rows + 2)).await;
        let (line_height, visible_line_count) = self.editor(|editor, cx| {
            (
                editor
                    .style()
                    .unwrap()
                    .text
                    .line_height_in_pixels(cx.rem_size()),
                editor.visible_line_count().unwrap(),
            )
        });

        let window = self.window;
        let margin = self
            .update_window(window, |_, cx| {
                cx.viewport_size().height - line_height * visible_line_count
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

    pub async fn assert_shared_state(&mut self, marked_text: &str) {
        self.is_dirty = false;
        let marked_text = marked_text.replace('•', " ");
        let neovim = self.neovim_state().await;
        let neovim_mode = self.neovim_mode().await;
        let editor = self.editor_state();
        let editor_mode = self.mode();
        if neovim == marked_text && neovim == editor && neovim_mode == editor_mode {
            return;
        }
        let initial_state = self
            .last_set_state
            .as_ref()
            .unwrap_or(&"N/A".to_string())
            .clone();

        let message = if neovim != marked_text {
            "Test is incorrect (currently expected != neovim_state)"
        } else {
            "Editor does not match nvim behaviour"
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
            initial_state,
            self.recent_keystrokes.join(" "),
            marked_text.replace(" \n", "•\n"),
            neovim_mode,
            neovim.replace(" \n", "•\n"),
            editor_mode,
            editor.replace(" \n", "•\n"),
        )
    }

    pub async fn assert_shared_clipboard(&mut self, text: &str) {
        let neovim = self.neovim.read_register('"').await;
        let editor = self.read_from_clipboard().unwrap().text().clone();

        if text == neovim && text == editor {
            return;
        }

        let message = if neovim != text {
            "Test is incorrect (currently expected != neovim)"
        } else {
            "Editor does not match nvim behaviour"
        };

        let initial_state = self
            .last_set_state
            .as_ref()
            .unwrap_or(&"N/A".to_string())
            .clone();

        panic!(
            indoc! {"{}
                # initial state:
                {}
                # keystrokes:
                {}
                # currently expected:
                {}
                # neovim clipboard:
                {}
                # zed clipboard:
                {}"},
            message,
            initial_state,
            self.recent_keystrokes.join(" "),
            text,
            neovim,
            editor
        )
    }

    pub async fn neovim_state(&mut self) -> String {
        self.neovim.marked_text().await
    }

    pub async fn neovim_mode(&mut self) -> Mode {
        self.neovim.mode().await.unwrap()
    }

    pub async fn assert_shared_mode(&mut self, mode: Mode) {
        let neovim = self.neovim_mode().await;
        let editor = self.cx.mode();

        if neovim != mode || editor != mode {
            panic!(
                indoc! {"Test failed (zed does not match nvim behaviour)
                    # desired mode:
                    {:?}
                    # neovim mode:
                    {:?}
                    # zed mode:
                    {:?}"},
                mode, neovim, editor,
            )
        }
    }

    pub async fn assert_state_matches(&mut self) {
        self.is_dirty = false;
        let neovim = self.neovim_state().await;
        let neovim_mode = self.neovim_mode().await;
        let editor = self.editor_state();
        let editor_mode = self.mode();
        let initial_state = self
            .last_set_state
            .as_ref()
            .unwrap_or(&"N/A".to_string())
            .clone();

        if neovim != editor || neovim_mode != editor_mode {
            panic!(
                indoc! {"Test failed (zed does not match nvim behaviour)
                    # initial state:
                    {}
                    # keystrokes:
                    {}
                    # neovim ({}):
                    {}
                    # zed ({}):
                    {}"},
                initial_state,
                self.recent_keystrokes.join(" "),
                neovim_mode,
                neovim,
                editor_mode,
                editor,
            )
        }
    }

    pub async fn assert_binding_matches<const COUNT: usize>(
        &mut self,
        keystrokes: [&str; COUNT],
        initial_state: &str,
    ) {
        if let Some(possible_exempted_keystrokes) = self.exemptions.get(initial_state) {
            match possible_exempted_keystrokes {
                Some(exempted_keystrokes) => {
                    if exempted_keystrokes.contains(&format!("{keystrokes:?}")) {
                        // This keystroke was exempted for this insertion text
                        return;
                    }
                }
                None => {
                    // All keystrokes for this insertion text are exempted
                    return;
                }
            }
        }

        let _state_context = self.set_shared_state(initial_state).await;
        let _keystroke_context = self.simulate_shared_keystrokes(keystrokes).await;
        self.assert_state_matches().await;
    }

    pub async fn assert_binding_matches_all<const COUNT: usize>(
        &mut self,
        keystrokes: [&str; COUNT],
        marked_positions: &str,
    ) {
        let (unmarked_text, cursor_offsets) = marked_text_offsets(marked_positions);

        for cursor_offset in cursor_offsets.iter() {
            let mut marked_text = unmarked_text.clone();
            marked_text.insert(*cursor_offset, 'ˇ');

            self.assert_binding_matches(keystrokes, &marked_text).await;
        }
    }

    pub fn each_marked_position(&self, marked_positions: &str) -> Vec<String> {
        let (unmarked_text, cursor_offsets) = marked_text_offsets(marked_positions);
        let mut ret = Vec::with_capacity(cursor_offsets.len());

        for cursor_offset in cursor_offsets.iter() {
            let mut marked_text = unmarked_text.clone();
            marked_text.insert(*cursor_offset, 'ˇ');
            ret.push(marked_text)
        }

        ret
    }

    pub async fn assert_neovim_compatible<const COUNT: usize>(
        &mut self,
        marked_positions: &str,
        keystrokes: [&str; COUNT],
    ) {
        self.set_shared_state(&marked_positions).await;
        self.simulate_shared_keystrokes(keystrokes).await;
        self.assert_state_matches().await;
    }

    pub async fn assert_matches_neovim<const COUNT: usize>(
        &mut self,
        marked_positions: &str,
        keystrokes: [&str; COUNT],
        result: &str,
    ) {
        self.set_shared_state(marked_positions).await;
        self.simulate_shared_keystrokes(keystrokes).await;
        self.assert_shared_state(result).await;
    }

    pub async fn assert_binding_matches_all_exempted<const COUNT: usize>(
        &mut self,
        keystrokes: [&str; COUNT],
        marked_positions: &str,
        feature: ExemptionFeatures,
    ) {
        if SUPPORTED_FEATURES.contains(&feature) {
            self.assert_binding_matches_all(keystrokes, marked_positions)
                .await
        }
    }

    pub fn binding<const COUNT: usize>(
        self,
        keystrokes: [&'static str; COUNT],
    ) -> NeovimBackedBindingTestContext<COUNT> {
        NeovimBackedBindingTestContext::new(keystrokes, self)
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

// a common mistake in tests is to call set_shared_state when
// you mean asswert_shared_state. This notices that and lets
// you know.
impl Drop for NeovimBackedTestContext {
    fn drop(&mut self) {
        if self.is_dirty {
            panic!("Test context was dropped after set_shared_state before assert_shared_state")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test::NeovimBackedTestContext;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn neovim_backed_test_context_works(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_state_matches().await;
        cx.set_shared_state("This is a tesˇt").await;
        cx.assert_state_matches().await;
    }
}
