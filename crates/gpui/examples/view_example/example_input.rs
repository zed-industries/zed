//! `Input` — a single-line text input. The shaping layer over `Editor`.
//!
//! Construct it two ways, depending on how much state you want to own:
//!   * `Input::new(value: Entity<String>)`  — you hold just the string; the input
//!     allocates the `Editor` internally via `use_state`. Value readable, cursor hidden.
//!   * `Input::editor(editor: Entity<Editor>)` — you hold the editor; cursor/selection
//!     are now yours to read and drive too.
//!
//! Either way the chrome is identical. Because the string (or editor) is the
//! input's *identity*, the internal `use_state(Editor)` is collision-safe across
//! any number of inputs.

use gpui::{
    App, BoxShadow, CursorStyle, Entity, EntityId, Hsla, IntoElement, Pixels, StyleRefinement,
    Window, div, hsla, point, prelude::*, px, white,
};

use crate::example_editor::{Editor, standard_actions};

enum Source {
    Value(Entity<String>),
    Editor(Entity<Editor>),
}

#[derive(IntoElement)]
pub struct Input {
    source: Source,
    width: Option<Pixels>,
    color: Option<Hsla>,
}

impl Input {
    /// Backed by a bare string; the editor is allocated internally.
    pub fn new(value: Entity<String>) -> Self {
        Self {
            source: Source::Value(value),
            width: None,
            color: None,
        }
    }

    /// Backed by an editor you own (so you can read/drive its cursor).
    pub fn editor(editor: Entity<Editor>) -> Self {
        Self {
            source: Source::Editor(editor),
            width: None,
            color: None,
        }
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl gpui::View for Input {
    fn entity_id(&self) -> Option<EntityId> {
        Some(match &self.source {
            Source::Value(value) => value.entity_id(),
            Source::Editor(editor) => editor.entity_id(),
        })
    }

    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // Get the editor: use the one we were handed, or allocate it under our
        // own (string-derived) identity so it persists and never collides.
        let editor = match self.source {
            Source::Value(value) => {
                window.use_state(cx, move |window, cx| Editor::over(value, window, cx))
            }
            Source::Editor(editor) => editor,
        };

        let focus_handle = editor.read(cx).focus_handle.clone();
        let is_focused = focus_handle.is_focused(window);
        let text_color = self.color.unwrap_or(hsla(0., 0., 0.1, 1.));
        let box_width = self.width.unwrap_or(px(300.));

        let border = if is_focused {
            hsla(220. / 360., 0.8, 0.5, 1.)
        } else {
            hsla(0., 0., 0.75, 1.)
        };

        div()
            .id("input")
            .key_context("TextInput")
            .track_focus(&focus_handle)
            .cursor(CursorStyle::IBeam)
            .map(standard_actions(editor.clone()))
            .w(box_width)
            .h(px(36.))
            .px(px(8.))
            .bg(white())
            .border_1()
            .border_color(border)
            .when(is_focused, |this| {
                this.shadow(vec![BoxShadow {
                    color: hsla(220. / 360., 0.8, 0.5, 0.3),
                    offset: point(px(0.), px(0.)),
                    blur_radius: px(4.),
                    spread_radius: px(1.),
                    inset: false,
                }])
            })
            .rounded(px(4.))
            .overflow_hidden()
            .flex()
            .items_center()
            .line_height(px(20.))
            .text_size(px(14.))
            .text_color(text_color)
            .child(editor.cached(StyleRefinement::default().size_full()))
    }
}
