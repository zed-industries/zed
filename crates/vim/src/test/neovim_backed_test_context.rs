use indoc::indoc;
use std::ops::{Deref, DerefMut, Range};

use collections::{HashMap, HashSet};
use gpui::ContextHandle;
use language::OffsetRangeExt;
use util::test::{generate_marked_text, marked_text_offsets};

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

    last_set_state: Option<String>,
    recent_keystrokes: Vec<String>,
}

impl<'a> NeovimBackedTestContext<'a> {
    pub async fn new(cx: &'a mut gpui::TestAppContext) -> NeovimBackedTestContext<'a> {
        let function_name = cx.function_name.clone();
        let cx = VimTestContext::new(cx, true).await;
        Self {
            cx,
            exemptions: Default::default(),
            neovim: NeovimConnection::new(function_name).await,

            last_set_state: None,
            recent_keystrokes: Default::default(),
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
    ) -> ContextHandle {
        for keystroke_text in keystroke_texts.into_iter() {
            self.recent_keystrokes.push(keystroke_text.to_string());
            self.neovim.send_keystroke(keystroke_text).await;
        }
        self.simulate_keystrokes(keystroke_texts)
    }

    pub async fn set_shared_state(&mut self, marked_text: &str) -> ContextHandle {
        let mode = if marked_text.contains("»") {
            Mode::Visual
        } else {
            Mode::Normal
        };
        let context_handle = self.set_state(marked_text, mode);
        self.last_set_state = Some(marked_text.to_string());
        self.recent_keystrokes = Vec::new();
        self.neovim.set_state(marked_text).await;
        context_handle
    }

    pub async fn assert_shared_state(&mut self, marked_text: &str) {
        let neovim = self.neovim_state().await;
        if neovim != marked_text {
            let initial_state = self
                .last_set_state
                .as_ref()
                .unwrap_or(&"N/A".to_string())
                .clone();
            panic!(
                indoc! {"Test is incorrect (currently expected != neovim state)
                # initial state:
                {}
                # keystrokes:
                {}
                # currently expected:
                {}
                # neovim state:
                {}
                # zed state:
                {}"},
                initial_state,
                self.recent_keystrokes.join(" "),
                marked_text,
                neovim,
                self.editor_state(),
            )
        }
        self.assert_editor_state(marked_text)
    }

    pub async fn neovim_state(&mut self) -> String {
        generate_marked_text(
            self.neovim.text().await.as_str(),
            &self.neovim_selections().await[..],
            true,
        )
    }

    pub async fn neovim_mode(&mut self) -> Mode {
        self.neovim.mode().await.unwrap()
    }

    async fn neovim_selections(&mut self) -> Vec<Range<usize>> {
        let neovim_selections = self.neovim.selections().await;
        neovim_selections
            .into_iter()
            .map(|selection| selection.to_offset(&self.buffer_snapshot()))
            .collect()
    }

    pub async fn assert_state_matches(&mut self) {
        let neovim = self.neovim_state().await;
        let editor = self.editor_state();
        let initial_state = self
            .last_set_state
            .as_ref()
            .unwrap_or(&"N/A".to_string())
            .clone();

        if neovim != editor {
            panic!(
                indoc! {"Test failed (zed does not match nvim behaviour)
                    # initial state:
                    {}
                    # keystrokes:
                    {}
                    # neovim state:
                    {}
                    # zed state:
                    {}"},
                initial_state,
                self.recent_keystrokes.join(" "),
                neovim,
                editor,
            )
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
