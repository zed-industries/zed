use std::ops::{Deref, DerefMut};

use util::test::marked_text_offsets;

use super::NeovimBackedTestContext;

pub struct NeovimBackedBindingTestContext<'a, const COUNT: usize> {
    cx: NeovimBackedTestContext<'a>,
    keystrokes_under_test: [&'static str; COUNT],
}

impl<'a, const COUNT: usize> NeovimBackedBindingTestContext<'a, COUNT> {
    pub fn new(
        keystrokes_under_test: [&'static str; COUNT],
        cx: NeovimBackedTestContext<'a>,
    ) -> Self {
        Self {
            cx,
            keystrokes_under_test,
        }
    }

    pub fn consume(self) -> NeovimBackedTestContext<'a> {
        self.cx
    }

    pub async fn assert(&mut self, initial_state: &str) {
        self.cx
            .assert_binding_matches(self.keystrokes_under_test, initial_state)
            .await
    }

    pub async fn assert_all(&mut self, marked_positions: &str) {
        let (unmarked_text, cursor_offsets) = marked_text_offsets(marked_positions);

        for cursor_offset in cursor_offsets.iter() {
            let mut marked_text = unmarked_text.clone();
            marked_text.insert(*cursor_offset, 'ˇ');
            self.assert(&marked_text).await;
        }
    }
}

impl<'a, const COUNT: usize> Deref for NeovimBackedBindingTestContext<'a, COUNT> {
    type Target = NeovimBackedTestContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl<'a, const COUNT: usize> DerefMut for NeovimBackedBindingTestContext<'a, COUNT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}
