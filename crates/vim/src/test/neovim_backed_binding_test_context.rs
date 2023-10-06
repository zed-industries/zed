use std::ops::{Deref, DerefMut};

use crate::state::Mode;

use super::{ExemptionFeatures, NeovimBackedTestContext, SUPPORTED_FEATURES};

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

    pub fn binding<const NEW_COUNT: usize>(
        self,
        keystrokes: [&'static str; NEW_COUNT],
    ) -> NeovimBackedBindingTestContext<'a, NEW_COUNT> {
        self.consume().binding(keystrokes)
    }

    pub async fn assert(&mut self, marked_positions: &str) {
        self.cx
            .assert_binding_matches(self.keystrokes_under_test, marked_positions)
            .await;
    }

    pub async fn assert_exempted(&mut self, marked_positions: &str, feature: ExemptionFeatures) {
        if SUPPORTED_FEATURES.contains(&feature) {
            self.cx
                .assert_binding_matches(self.keystrokes_under_test, marked_positions)
                .await
        }
    }

    pub fn assert_manual(
        &mut self,
        initial_state: &str,
        mode_before: Mode,
        state_after: &str,
        mode_after: Mode,
    ) {
        self.cx.assert_binding(
            self.keystrokes_under_test,
            initial_state,
            mode_before,
            state_after,
            mode_after,
        );
    }

    pub async fn assert_all(&mut self, marked_positions: &str) {
        self.cx
            .assert_binding_matches_all(self.keystrokes_under_test, marked_positions)
            .await
    }

    pub async fn assert_all_exempted(
        &mut self,
        marked_positions: &str,
        feature: ExemptionFeatures,
    ) {
        if SUPPORTED_FEATURES.contains(&feature) {
            self.cx
                .assert_binding_matches_all(self.keystrokes_under_test, marked_positions)
                .await
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
