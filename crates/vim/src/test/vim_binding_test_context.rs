use std::ops::{Deref, DerefMut};

use crate::*;

use super::VimTestContext;

pub struct VimBindingTestContext<'a, const COUNT: usize> {
    cx: VimTestContext<'a>,
    keystrokes_under_test: [&'static str; COUNT],
    mode_before: Mode,
    mode_after: Mode,
}

impl<'a, const COUNT: usize> VimBindingTestContext<'a, COUNT> {
    pub fn new(
        keystrokes_under_test: [&'static str; COUNT],
        mode_before: Mode,
        mode_after: Mode,
        cx: VimTestContext<'a>,
    ) -> Self {
        Self {
            cx,
            keystrokes_under_test,
            mode_before,
            mode_after,
        }
    }

    pub fn binding<const NEW_COUNT: usize>(
        self,
        keystrokes_under_test: [&'static str; NEW_COUNT],
    ) -> VimBindingTestContext<'a, NEW_COUNT> {
        VimBindingTestContext {
            keystrokes_under_test,
            cx: self.cx,
            mode_before: self.mode_before,
            mode_after: self.mode_after,
        }
    }

    pub fn mode_after(mut self, mode_after: Mode) -> Self {
        self.mode_after = mode_after;
        self
    }

    pub fn assert(&mut self, initial_state: &str, state_after: &str) {
        self.cx.assert_binding(
            self.keystrokes_under_test,
            initial_state,
            self.mode_before,
            state_after,
            self.mode_after,
        )
    }
}

impl<'a, const COUNT: usize> Deref for VimBindingTestContext<'a, COUNT> {
    type Target = VimTestContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl<'a, const COUNT: usize> DerefMut for VimBindingTestContext<'a, COUNT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}
