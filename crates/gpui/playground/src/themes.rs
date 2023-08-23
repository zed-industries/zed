use crate::color::{Hsla, Lerp};
use std::ops::Range;

pub mod rose_pine;

pub struct ThemeColors {
    pub base: Range<Hsla>,
    pub surface: Range<Hsla>,
    pub overlay: Range<Hsla>,
    pub muted: Range<Hsla>,
    pub subtle: Range<Hsla>,
    pub text: Range<Hsla>,
    pub highlight_low: Range<Hsla>,
    pub highlight_med: Range<Hsla>,
    pub highlight_high: Range<Hsla>,
    pub success: Range<Hsla>,
    pub warning: Range<Hsla>,
    pub error: Range<Hsla>,
    pub inserted: Range<Hsla>,
    pub deleted: Range<Hsla>,
    pub modified: Range<Hsla>,
}

impl ThemeColors {
    pub fn base(&self, level: f32) -> Hsla {
        self.base.lerp(level)
    }

    pub fn surface(&self, level: f32) -> Hsla {
        self.surface.lerp(level)
    }

    pub fn overlay(&self, level: f32) -> Hsla {
        self.overlay.lerp(level)
    }

    pub fn muted(&self, level: f32) -> Hsla {
        self.muted.lerp(level)
    }

    pub fn subtle(&self, level: f32) -> Hsla {
        self.subtle.lerp(level)
    }

    pub fn text(&self, level: f32) -> Hsla {
        self.text.lerp(level)
    }

    pub fn highlight_low(&self, level: f32) -> Hsla {
        self.highlight_low.lerp(level)
    }

    pub fn highlight_med(&self, level: f32) -> Hsla {
        self.highlight_med.lerp(level)
    }

    pub fn highlight_high(&self, level: f32) -> Hsla {
        self.highlight_high.lerp(level)
    }

    pub fn success(&self, level: f32) -> Hsla {
        self.success.lerp(level)
    }

    pub fn warning(&self, level: f32) -> Hsla {
        self.warning.lerp(level)
    }

    pub fn error(&self, level: f32) -> Hsla {
        self.error.lerp(level)
    }

    pub fn inserted(&self, level: f32) -> Hsla {
        self.inserted.lerp(level)
    }

    pub fn deleted(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }

    pub fn modified(&self, level: f32) -> Hsla {
        self.modified.lerp(level)
    }
}
