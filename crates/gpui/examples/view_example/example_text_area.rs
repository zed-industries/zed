//! The `ExampleTextArea` view — a multi-line text area component.
//!
//! Same `ExampleEditor` entity, different presentation: taller box with configurable
//! row count. Demonstrates that the same entity type can back different `View`
//! components with different props and layouts.

use gpui::{
    App, BoxShadow, CursorStyle, Entity, Hsla, IntoViewElement, StyleRefinement, Window, div, hsla,
    point, prelude::*, px, white,
};

use crate::example_editor::ExampleEditor;
use crate::example_editor::ExampleEditorView;
use crate::{Backspace, Delete, End, Enter, Home, Left, Right};

#[derive(Hash, IntoViewElement)]
pub struct ExampleTextArea {
    editor: Entity<ExampleEditor>,
    rows: usize,
    color: Option<Hsla>,
}

impl ExampleTextArea {
    pub fn new(editor: Entity<ExampleEditor>, rows: usize) -> Self {
        Self {
            editor,
            rows,
            color: None,
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl gpui::View for ExampleTextArea {
    type Entity = ExampleEditor;

    fn entity(&self) -> Option<Entity<ExampleEditor>> {
        Some(self.editor.clone())
    }

    fn style(&self) -> Option<StyleRefinement> {
        let row_height = px(20.);
        let box_height = row_height * self.rows as f32 + px(16.);
        let mut style = StyleRefinement::default();
        style.size.width = Some(px(400.).into());
        style.size.height = Some(box_height.into());
        Some(style)
    }

    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = self.editor.read(cx).focus_handle.clone();
        let is_focused = focus_handle.is_focused(window);
        let text_color = self.color.unwrap_or(hsla(0., 0., 0.1, 1.));
        let row_height = px(20.);
        let box_height = row_height * self.rows as f32 + px(16.);
        let editor = self.editor;

        div()
            .id("text-area")
            .key_context("TextInput")
            .track_focus(&focus_handle)
            .cursor(CursorStyle::IBeam)
            .on_action({
                let editor = editor.clone();
                move |action: &Backspace, _window, cx| {
                    editor.update(cx, |state, cx| state.backspace(action, _window, cx));
                }
            })
            .on_action({
                let editor = editor.clone();
                move |action: &Delete, _window, cx| {
                    editor.update(cx, |state, cx| state.delete(action, _window, cx));
                }
            })
            .on_action({
                let editor = editor.clone();
                move |action: &Left, _window, cx| {
                    editor.update(cx, |state, cx| state.left(action, _window, cx));
                }
            })
            .on_action({
                let editor = editor.clone();
                move |action: &Right, _window, cx| {
                    editor.update(cx, |state, cx| state.right(action, _window, cx));
                }
            })
            .on_action({
                let editor = editor.clone();
                move |action: &Home, _window, cx| {
                    editor.update(cx, |state, cx| state.home(action, _window, cx));
                }
            })
            .on_action({
                let editor = editor.clone();
                move |action: &End, _window, cx| {
                    editor.update(cx, |state, cx| state.end(action, _window, cx));
                }
            })
            .on_action({
                let editor = editor.clone();
                move |_: &Enter, _window, cx| {
                    editor.update(cx, |state, cx| state.insert_newline(cx));
                }
            })
            .w(px(400.))
            .h(box_height)
            .p(px(8.))
            .bg(white())
            .border_1()
            .border_color(if is_focused {
                hsla(220. / 360., 0.8, 0.5, 1.)
            } else {
                hsla(0., 0., 0.75, 1.)
            })
            .when(is_focused, |this| {
                this.shadow(vec![BoxShadow {
                    color: hsla(220. / 360., 0.8, 0.5, 0.3),
                    offset: point(px(0.), px(0.)),
                    blur_radius: px(4.),
                    spread_radius: px(1.),
                }])
            })
            .rounded(px(4.))
            .overflow_hidden()
            .line_height(row_height)
            .text_size(px(14.))
            .text_color(text_color)
            .child(ExampleEditorView::new(editor).text_color(text_color))
    }
}
