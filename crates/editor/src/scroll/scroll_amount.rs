use gpui::ViewContext;
use serde::Deserialize;
use util::iife;

use crate::Editor;

#[derive(Clone, PartialEq, Deserialize)]
pub enum ScrollAmount {
    // Scroll N lines (positive is towards the end of the document)
    Line(f32),
    // Scroll N pages (positive is towards the end of the document)
    Page(f32),
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
                Self::Line(c) if *c > 0. => context_menu.select_next(cx),
                Self::Line(_) => context_menu.select_prev(cx),
                Self::Page(c) if *c > 0. => context_menu.select_last(cx),
                Self::Page(_) => context_menu.select_first(cx),
            }
            .then_some(())
        })
        .is_some()
    }

    pub fn lines(&self, editor: &mut Editor) -> f32 {
        match self {
            Self::Line(count) => *count,
            Self::Page(count) => editor
                .visible_line_count()
                // subtract one to leave an anchor line
                // round towards zero (so page-up and page-down are symmetric)
                .map(|l| ((l - 1.) * count).trunc())
                .unwrap_or(0.),
        }
    }
}
