//! `Input` — a single-line text input. The shaping layer over `Editor`.
//!
//! Construct it two ways, depending on how much state you want to own:
//!   * `Input::new(value: ProjectionMut<String>)` — you hold just the text; the
//!     input allocates the `Editor` internally via `use_state`. Value readable,
//!     cursor hidden. The text can be a whole `Entity<String>` (via `.into()`) or
//!     one field of a bigger struct (via `project!`) — the input can't tell.
//!   * `Input::editor(editor: Entity<Editor>)` — you hold the editor; cursor/selection
//!     are now yours to read and drive too.
//!
//! Either way the chrome is identical.

use gpui::{
    App, BoxShadow, CursorStyle, Entity, EntityId, Hsla, IntoElement, Pixels, ProjectionMut,
    StyleRefinement, Window, div, hsla, point, prelude::*, px, white,
};

use crate::example_editor::{Editor, standard_actions};

enum Source {
    Value(ProjectionMut<String>),
    Editor(Entity<Editor>),
}

#[derive(IntoElement)]
pub struct Input {
    source: Source,
    width: Option<Pixels>,
    color: Option<Hsla>,
}

impl Input {
    /// Backed by a projected string; the editor is allocated internally.
    pub fn new(value: ProjectionMut<String>) -> Self {
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
        match &self.source {
            // A view's identity is the notify target for state allocated inside
            // it. The editor below is allocated here and observes the value, so
            // identifying this view by the value would route the editor's
            // notifications back into the thing it observes and spin forever.
            // Positional identity is correct here.
            Source::Value(_) => None,
            // Nothing is allocated in this branch, so the editor we were handed
            // is a safe identity and survives moving around the tree.
            Source::Editor(editor) => Some(editor.entity_id()),
        }
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
