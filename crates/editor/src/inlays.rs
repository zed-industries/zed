//! The logic, responsible for managing [`Inlay`]s in the editor.
//!
//! Inlays are "not real" text that gets mixed into the "real" buffer's text.
//! They are attached to a certain [`Anchor`], and display certain contents (usually, strings)
//! between real text around that anchor.
//!
//! Inlay examples in Zed:
//! * inlay hints, received from LSP
//! * inline values, shown in the debugger
//! * inline predictions, showing the Zeta/Copilot/etc. predictions
//! * document color values, if configured to be displayed as inlays
//! * ... anything else, potentially.
//!
//! Editor uses [`crate::DisplayMap`] and [`crate::display_map::InlayMap`] to manage what's rendered inside the editor, using
//! [`InlaySplice`] to update this state.

/// Logic, related to managing LSP inlay hint inlays.
pub mod inlay_hints;

use std::{any::TypeId, sync::OnceLock};

use gpui::{Context, HighlightStyle, Hsla, Rgba, Task};
use multi_buffer::Anchor;
use project::{InlayHint, InlayId};
use text::Rope;

use crate::{Editor, hover_links::InlayHighlight};

/// A splice to send into the `inlay_map` for updating the visible inlays on the screen.
/// "Visible" inlays may not be displayed in the buffer right away, but those are ready to be displayed on further buffer scroll, pane item activations, etc. right away without additional LSP queries or settings changes.
/// The data in the cache is never used directly for displaying inlays on the screen, to avoid races with updates from LSP queries and sync overhead.
/// Splice is picked to help avoid extra hint flickering and "jumps" on the screen.
#[derive(Debug, Default)]
pub struct InlaySplice {
    pub to_remove: Vec<InlayId>,
    pub to_insert: Vec<Inlay>,
}

impl InlaySplice {
    pub fn is_empty(&self) -> bool {
        self.to_remove.is_empty() && self.to_insert.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Inlay {
    pub id: InlayId,
    pub position: Anchor,
    pub content: InlayContent,
}

#[derive(Debug, Clone)]
pub enum InlayContent {
    Text(text::Rope),
    Color(Hsla),
}

impl Inlay {
    pub fn hint(id: InlayId, position: Anchor, hint: &InlayHint) -> Self {
        let mut text = hint.text();
        if hint.padding_right && text.reversed_chars_at(text.len()).next() != Some(' ') {
            text.push(" ");
        }
        if hint.padding_left && text.chars_at(0).next() != Some(' ') {
            text.push_front(" ");
        }
        Self {
            id,
            position,
            content: InlayContent::Text(text),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn mock_hint(id: usize, position: Anchor, text: impl Into<Rope>) -> Self {
        Self {
            id: InlayId::Hint(id),
            position,
            content: InlayContent::Text(text.into()),
        }
    }

    pub fn color(id: usize, position: Anchor, color: Rgba) -> Self {
        Self {
            id: InlayId::Color(id),
            position,
            content: InlayContent::Color(color.into()),
        }
    }

    pub fn edit_prediction<T: Into<Rope>>(id: usize, position: Anchor, text: T) -> Self {
        Self {
            id: InlayId::EditPrediction(id),
            position,
            content: InlayContent::Text(text.into()),
        }
    }

    pub fn debugger<T: Into<Rope>>(id: usize, position: Anchor, text: T) -> Self {
        Self {
            id: InlayId::DebuggerValue(id),
            position,
            content: InlayContent::Text(text.into()),
        }
    }

    pub fn text(&self) -> &Rope {
        static COLOR_TEXT: OnceLock<Rope> = OnceLock::new();
        match &self.content {
            InlayContent::Text(text) => text,
            InlayContent::Color(_) => COLOR_TEXT.get_or_init(|| Rope::from("◼")),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn get_color(&self) -> Option<Hsla> {
        match self.content {
            InlayContent::Color(color) => Some(color),
            _ => None,
        }
    }
}

pub struct InlineValueCache {
    pub enabled: bool,
    pub inlays: Vec<InlayId>,
    pub refresh_task: Task<Option<()>>,
}

impl InlineValueCache {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            inlays: Vec::new(),
            refresh_task: Task::ready(None),
        }
    }
}

impl Editor {
    /// Modify which hints are displayed in the editor.
    pub fn splice_inlays(
        &mut self,
        to_remove: &[InlayId],
        to_insert: Vec<Inlay>,
        cx: &mut Context<Self>,
    ) {
        if let Some(inlay_hints) = &mut self.inlay_hints {
            for id_to_remove in to_remove {
                inlay_hints.added_hints.remove(id_to_remove);
            }
        }
        self.display_map.update(cx, |display_map, cx| {
            display_map.splice_inlays(to_remove, to_insert, cx)
        });
        cx.notify();
    }

    pub(crate) fn highlight_inlays<T: 'static>(
        &mut self,
        highlights: Vec<InlayHighlight>,
        style: HighlightStyle,
        cx: &mut Context<Self>,
    ) {
        self.display_map.update(cx, |map, _| {
            map.highlight_inlays(TypeId::of::<T>(), highlights, style)
        });
        cx.notify();
    }

    pub fn inline_values_enabled(&self) -> bool {
        self.inline_value_cache.enabled
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn inline_value_inlays(&self, cx: &gpui::App) -> Vec<Inlay> {
        self.display_map
            .read(cx)
            .current_inlays()
            .filter(|inlay| matches!(inlay.id, InlayId::DebuggerValue(_)))
            .cloned()
            .collect()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn all_inlays(&self, cx: &gpui::App) -> Vec<Inlay> {
        self.display_map
            .read(cx)
            .current_inlays()
            .cloned()
            .collect()
    }
}
