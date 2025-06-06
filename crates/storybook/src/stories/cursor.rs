use gpui::{Div, Render, Stateful};
use story::Story;
use ui::prelude::*;

pub struct CursorStory;

impl Render for CursorStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let all_cursors: [(&str, Box<dyn Fn(Stateful<Div>) -> Stateful<Div>>); 19] = [
            (
                "cursor_default",
                Box::new(|el: Stateful<Div>| el.cursor_default()),
            ),
            (
                "cursor_pointer",
                Box::new(|el: Stateful<Div>| el.cursor_pointer()),
            ),
            (
                "cursor_text",
                Box::new(|el: Stateful<Div>| el.cursor_text()),
            ),
            (
                "cursor_move",
                Box::new(|el: Stateful<Div>| el.cursor_move()),
            ),
            (
                "cursor_not_allowed",
                Box::new(|el: Stateful<Div>| el.cursor_not_allowed()),
            ),
            (
                "cursor_context_menu",
                Box::new(|el: Stateful<Div>| el.cursor_context_menu()),
            ),
            (
                "cursor_crosshair",
                Box::new(|el: Stateful<Div>| el.cursor_crosshair()),
            ),
            (
                "cursor_vertical_text",
                Box::new(|el: Stateful<Div>| el.cursor_vertical_text()),
            ),
            (
                "cursor_alias",
                Box::new(|el: Stateful<Div>| el.cursor_alias()),
            ),
            (
                "cursor_copy",
                Box::new(|el: Stateful<Div>| el.cursor_copy()),
            ),
            (
                "cursor_no_drop",
                Box::new(|el: Stateful<Div>| el.cursor_no_drop()),
            ),
            (
                "cursor_grab",
                Box::new(|el: Stateful<Div>| el.cursor_grab()),
            ),
            (
                "cursor_grabbing",
                Box::new(|el: Stateful<Div>| el.cursor_grabbing()),
            ),
            (
                "cursor_col_resize",
                Box::new(|el: Stateful<Div>| el.cursor_col_resize()),
            ),
            (
                "cursor_row_resize",
                Box::new(|el: Stateful<Div>| el.cursor_row_resize()),
            ),
            (
                "cursor_n_resize",
                Box::new(|el: Stateful<Div>| el.cursor_n_resize()),
            ),
            (
                "cursor_e_resize",
                Box::new(|el: Stateful<Div>| el.cursor_e_resize()),
            ),
            (
                "cursor_s_resize",
                Box::new(|el: Stateful<Div>| el.cursor_s_resize()),
            ),
            (
                "cursor_w_resize",
                Box::new(|el: Stateful<Div>| el.cursor_w_resize()),
            ),
        ];

        Story::container(cx)
            .flex()
            .gap_1()
            .child(Story::title("cursor", cx))
            .children(all_cursors.map(|(name, apply_cursor)| {
                div().gap_1().flex().text_color(gpui::white()).child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .id(name)
                        .map(apply_cursor)
                        .w_64()
                        .h_8()
                        .bg(gpui::red())
                        .active(|style| style.bg(gpui::green()))
                        .text_sm()
                        .child(Story::label(name, cx)),
                )
            }))
    }
}
