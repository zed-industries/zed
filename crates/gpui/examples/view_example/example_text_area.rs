//! `TextArea` — a multi-line text box. Same `Editor` workhorse, taller chrome,
//! and `Enter` inserts a newline instead of being ignored. Constructible from a
//! string or an editor, exactly like [`Input`](crate::example_input::Input).

use gpui::{
    App, BoxShadow, CursorStyle, Entity, EntityId, Hsla, IntoElement, StyleRefinement, Window, div,
    hsla, point, prelude::*, px, white,
};

use crate::Enter;
use crate::example_editor::{Editor, standard_actions};

enum Source {
    Value(Entity<String>),
    Editor(Entity<Editor>),
}

#[derive(IntoElement)]
pub struct TextArea {
    source: Source,
    rows: usize,
    color: Option<Hsla>,
}

impl TextArea {
    pub fn new(value: Entity<String>, rows: usize) -> Self {
        Self {
            source: Source::Value(value),
            rows,
            color: None,
        }
    }

    pub fn editor(editor: Entity<Editor>, rows: usize) -> Self {
        Self {
            source: Source::Editor(editor),
            rows,
            color: None,
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl gpui::View for TextArea {
    fn entity_id(&self) -> Option<EntityId> {
        Some(match &self.source {
            Source::Value(value) => value.entity_id(),
            Source::Editor(editor) => editor.entity_id(),
        })
    }

    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let editor = match self.source {
            Source::Value(value) => {
                window.use_state(cx, move |window, cx| Editor::over(value, window, cx))
            }
            Source::Editor(editor) => editor,
        };

        let focus_handle = editor.read(cx).focus_handle.clone();
        let is_focused = focus_handle.is_focused(window);
        let text_color = self.color.unwrap_or(hsla(0., 0., 0.1, 1.));
        let row_height = px(20.);
        let box_height = row_height * self.rows as f32 + px(16.);

        let border = if is_focused {
            hsla(220. / 360., 0.8, 0.5, 1.)
        } else {
            hsla(0., 0., 0.75, 1.)
        };

        div()
            .id("text-area")
            .key_context("TextInput")
            .track_focus(&focus_handle)
            .cursor(CursorStyle::IBeam)
            .map(standard_actions(editor.clone()))
            // Enter is the one binding that differs from a single-line input.
            .on_action({
                let editor = editor.clone();
                move |_: &Enter, _window, cx| editor.update(cx, |e, cx| e.insert_newline(cx))
            })
            .w(px(400.))
            .h(box_height)
            .p(px(8.))
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
            .line_height(row_height)
            .text_size(px(14.))
            .text_color(text_color)
            // The cache style is computed from the `rows` prop: change `rows` and
            // the editor's cached bounds change, busting its cache and re-laying
            // out the text. (`Input` just uses `size_full()` — nothing to vary.)
            .child(
                editor.cached(
                    StyleRefinement::default()
                        .w_full()
                        .h(row_height * self.rows as f32),
                ),
            )
    }
}
