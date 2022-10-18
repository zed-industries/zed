use std::ops::{Deref, DerefMut};

use collections::{HashMap, HashSet};
use gpui::ContextHandle;
use language::{OffsetRangeExt, Point};
use util::test::marked_text_offsets;

use super::{neovim_connection::NeovimConnection, NeovimBackedBindingTestContext, VimTestContext};
use crate::state::Mode;

pub struct NeovimBackedTestContext<'a> {
    cx: VimTestContext<'a>,
    // Lookup for exempted assertions. Keyed by the insertion text, and with a value indicating which
    // bindings are exempted. If None, all bindings are ignored for that insertion text.
    exemptions: HashMap<String, Option<HashSet<String>>>,
    neovim: NeovimConnection,
}

impl<'a> NeovimBackedTestContext<'a> {
    pub async fn new(cx: &'a mut gpui::TestAppContext) -> NeovimBackedTestContext<'a> {
        let function_name = cx.function_name.clone();
        let cx = VimTestContext::new(cx, true).await;
        Self {
            cx,
            exemptions: Default::default(),
            neovim: NeovimConnection::new(function_name).await,
        }
    }

    pub fn add_initial_state_exemption(&mut self, initial_state: &str) {
        let initial_state = initial_state.to_string();
        // None represents all keybindings being exempted for that initial state
        self.exemptions.insert(initial_state, None);
    }

    pub async fn simulate_shared_keystroke(&mut self, keystroke_text: &str) -> ContextHandle {
        self.neovim.send_keystroke(keystroke_text).await;
        self.simulate_keystroke(keystroke_text)
    }

    pub async fn simulate_shared_keystrokes<const COUNT: usize>(
        &mut self,
        keystroke_texts: [&str; COUNT],
    ) -> ContextHandle {
        for keystroke_text in keystroke_texts.into_iter() {
            self.neovim.send_keystroke(keystroke_text).await;
        }
        self.simulate_keystrokes(keystroke_texts)
    }

    pub async fn set_shared_state(&mut self, marked_text: &str) -> ContextHandle {
        let context_handle = self.set_state(marked_text, Mode::Normal);

        let selection = self.editor(|editor, cx| editor.selections.newest::<Point>(cx));
        let text = self.buffer_text();
        self.neovim.set_state(selection, &text).await;

        context_handle
    }

    pub async fn assert_state_matches(&mut self) {
        assert_eq!(
            self.neovim.text().await,
            self.buffer_text(),
            "{}",
            self.assertion_context()
        );

        let mut neovim_selection = self.neovim.selection().await;
        // Zed selections adjust themselves to make the end point visually make sense
        if neovim_selection.start > neovim_selection.end {
            neovim_selection.start.column += 1;
        }
        let neovim_selection = neovim_selection.to_offset(&self.buffer_snapshot());
        self.assert_editor_selections(vec![neovim_selection]);

        if let Some(neovim_mode) = self.neovim.mode().await {
            assert_eq!(neovim_mode, self.mode(), "{}", self.assertion_context(),);
        }
    }

    pub async fn assert_binding_matches<const COUNT: usize>(
        &mut self,
        keystrokes: [&str; COUNT],
        initial_state: &str,
    ) -> Option<(ContextHandle, ContextHandle)> {
        if let Some(possible_exempted_keystrokes) = self.exemptions.get(initial_state) {
            match possible_exempted_keystrokes {
                Some(exempted_keystrokes) => {
                    if exempted_keystrokes.contains(&format!("{keystrokes:?}")) {
                        // This keystroke was exempted for this insertion text
                        return None;
                    }
                }
                None => {
                    // All keystrokes for this insertion text are exempted
                    return None;
                }
            }
        }

        let _state_context = self.set_shared_state(initial_state).await;
        let _keystroke_context = self.simulate_shared_keystrokes(keystrokes).await;
        self.assert_state_matches().await;
        Some((_state_context, _keystroke_context))
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

    pub fn binding<const COUNT: usize>(
        self,
        keystrokes: [&'static str; COUNT],
    ) -> NeovimBackedBindingTestContext<'a, COUNT> {
        NeovimBackedBindingTestContext::new(keystrokes, self)
    }
}

impl<'a> Deref for NeovimBackedTestContext<'a> {
    type Target = VimTestContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl<'a> DerefMut for NeovimBackedTestContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}

#[cfg(test)]
mod test {
    use gpui::TestAppContext;

    use crate::test::NeovimBackedTestContext;

    #[gpui::test]
    async fn neovim_backed_test_context_works(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_state_matches().await;
        cx.set_shared_state("This is a tesˇt").await;
        cx.assert_state_matches().await;
    }
}
