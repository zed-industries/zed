use std::ops::{Deref, DerefMut};

use collections::{HashMap, HashSet};
use gpui::ContextHandle;
use language::{OffsetRangeExt, Point};
use util::test::marked_text_offsets;

use super::{neovim_connection::NeovimConnection, NeovimBackedBindingTestContext, VimTestContext};
use crate::state::Mode;

pub const SUPPORTED_FEATURES: &[ExemptionFeatures] = &[
    ExemptionFeatures::DeletionOnEmptyLine,
    ExemptionFeatures::OperatorAbortsOnFailedMotion,
];

/// Enum representing features we have tests for but which don't work, yet. Used
/// to add exemptions and automatically
#[derive(PartialEq, Eq)]
pub enum ExemptionFeatures {
    // MOTIONS
    // Deletions on empty lines miss some newlines
    DeletionOnEmptyLine,
    // When a motion fails, it should should not apply linewise operations
    OperatorAbortsOnFailedMotion,
    // When an operator completes at the end of the file, an extra newline is left
    OperatorLastNewlineRemains,
    // Deleting a word on an empty line doesn't remove the newline
    DeleteWordOnEmptyLine,

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
    // Quote style surrounding text objects don't seek forward properly
    QuotesSeekForward,
    // Neovim freezes up for some reason with angle brackets
    AngleBracketsFreezeNeovim,
    // Sentence Doesn't backtrack when its at the end of the file
    SentenceAfterPunctuationAtEndOfFile,
}

impl ExemptionFeatures {
    pub fn supported(&self) -> bool {
        SUPPORTED_FEATURES.contains(self)
    }
}

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

                // None represents all keybindings being exempted for that initial state
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
