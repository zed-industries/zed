use crate::color::{Hsla, Lerp};
use serde::{Deserialize, Serialize};
use std::{ops::Range, sync::Arc};

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
        self.deleted.lerp(level)
    }
    pub fn surface(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn overlay(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn muted(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn subtle(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn text(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn highlight_low(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn highlight_med(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn highlight_high(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn success(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn warning(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn error(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn inserted(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn deleted(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
    pub fn modified(&self, level: f32) -> Hsla {
        self.deleted.lerp(level)
    }
}

#[derive(Serialize, Deserialize)]
struct Entity {
    class: String,
    #[serde(rename = "type")]
    kind: String,
    id: Arc<str>,
    name: String,
    value: String,
    description: String,
    category_id: String,
    last_updated_by: String,
    last_updated: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Category {
    id: String,
    label: String,
}
