//! The `ExampleInput` view — a single-line text input component.
//!
//! Composes `ExampleEditorText` inside a styled container with focus ring, border,
//! and action handlers. Implements the `View` trait with `#[derive(Hash)]`
//! so that prop changes (color, width) automatically invalidate the render
//! cache via `ViewElement::cached()`.

use std::time::Duration;

use gpui::{
    Animation, AnimationExt as _, App, BoxShadow, CursorStyle, Entity, Hsla, IntoViewElement,
    Pixels, SharedString, StyleRefinement, Window, bounce, div, ease_in_out, hsla, point,
    prelude::*, px, white,
};

use crate::example_editor::ExampleEditor;
use crate::example_editor::ExampleEditorView;
use crate::{Backspace, Delete, End, Enter, Home, Left, Right};

struct FlashState {
    count: usize,
}

#[derive(Hash, IntoViewElement)]
pub struct ExampleInput {
    editor: Entity<ExampleEditor>,
    width: Option<Pixels>,
    color: Option<Hsla>,
}

impl ExampleInput {
    pub fn new(editor: Entity<ExampleEditor>) -> Self {
        Self {
            editor,
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

impl gpui::View for ExampleInput {
    type Entity = ExampleEditor;

    fn entity(&self) -> Option<Entity<ExampleEditor>> {
        Some(self.editor.clone())
    }

    fn style(&self) -> Option<StyleRefinement> {
        let mut style = StyleRefinement::default();
        if let Some(w) = self.width {
            style.size.width = Some(w.into());
        }
        style.size.height = Some(px(36.).into());
        Some(style)
    }

    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let flash_state = window.use_state(cx, |_window, _cx| FlashState { count: 0 });
        let count = flash_state.read(cx).count;

        let focus_handle = self.editor.read(cx).focus_handle.clone();
        let is_focused = focus_handle.is_focused(window);
        let text_color = self.color.unwrap_or(hsla(0., 0., 0.1, 1.));
        let box_width = self.width.unwrap_or(px(300.));
        let editor = self.editor;

        let focused_border = hsla(220. / 360., 0.8, 0.5, 1.);
        let unfocused_border = hsla(0., 0., 0.75, 1.);
        let normal_border = if is_focused {
            focused_border
        } else {
            unfocused_border
        };
        let highlight_border = hsla(140. / 360., 0.8, 0.5, 1.);

        let base = div()
            .id("input")
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
                let flash_state = flash_state;
                move |_: &Enter, _window, cx| {
                    flash_state.update(cx, |state, cx| {
                        state.count += 1;
                        cx.notify();
                    });
                }
            })
            .w(box_width)
            .h(px(36.))
            .px(px(8.))
            .bg(white())
            .border_1()
            .border_color(normal_border)
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
            .flex()
            .items_center()
            .line_height(px(20.))
            .text_size(px(14.))
            .text_color(text_color)
            .child(ExampleEditorView::new(editor).text_color(text_color));

        if count > 0 {
            base.with_animation(
                SharedString::from(format!("enter-bounce-{count}")),
                Animation::new(Duration::from_millis(300)).with_easing(bounce(ease_in_out)),
                move |this, delta| {
                    let h = normal_border.h + (highlight_border.h - normal_border.h) * delta;
                    let s = normal_border.s + (highlight_border.s - normal_border.s) * delta;
                    let l = normal_border.l + (highlight_border.l - normal_border.l) * delta;
                    this.border_color(hsla(h, s, l, 1.0))
                },
            )
            .into_any_element()
        } else {
            base.into_any_element()
        }
    }
}
