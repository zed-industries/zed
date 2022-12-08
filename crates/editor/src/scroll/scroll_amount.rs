use gpui::ViewContext;
use serde::Deserialize;
use util::iife;

use crate::Editor;

#[derive(Clone, PartialEq, Deserialize)]
pub enum ScrollAmount {
    LineUp,
    LineDown,
    HalfPageUp,
    HalfPageDown,
    PageUp,
    PageDown,
}

impl ScrollAmount {
    pub fn move_context_menu_selection(
        &self,
        editor: &mut Editor,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        iife!({
            let context_menu = editor.context_menu.as_mut()?;

            match self {
                Self::LineDown | Self::HalfPageDown => context_menu.select_next(cx),
                Self::LineUp | Self::HalfPageUp => context_menu.select_prev(cx),
                Self::PageDown => context_menu.select_last(cx),
                Self::PageUp => context_menu.select_first(cx),
            }
            .then_some(())
        })
        .is_some()
    }

    pub fn lines(&self, editor: &mut Editor) -> f32 {
        match self {
            Self::LineDown => 1.,
            Self::LineUp => -1.,
            Self::HalfPageDown => editor.visible_line_count().map(|l| l / 2.).unwrap_or(1.),
            Self::HalfPageUp => -editor.visible_line_count().map(|l| l / 2.).unwrap_or(1.),
            // Minus 1. here so that there is a pivot line that stays on the screen
            Self::PageDown => editor.visible_line_count().unwrap_or(1.) - 1.,
            Self::PageUp => -editor.visible_line_count().unwrap_or(1.) - 1.,
        }
    }
}
