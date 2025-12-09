use std::collections::btree_map::Entry;

use gpui::{
    App, Application, Bounds, Context, Entity, InputBindings, InputState, InputStateEvent, Rgba,
    SharedString, Window, WindowBounds, WindowOptions, bind_input_keys, div, input, prelude::*, px,
    rgb, size,
};

struct TodoMvc {
    todo_items: Vec<TodoItem>,
}

struct TodoItem {
    text: SharedString,
    checked: bool,
}

impl Render for TodoMvc {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.weak_entity();

        let input_state = window.use_state(cx, move |_window, cx| {
            let mut state = InputState::new(cx);
            state.set_placeholder("What needs to be done?", cx);
            cx.subscribe_self(move |input_state, e: &InputStateEvent, cx| match e {
                InputStateEvent::Enter => {
                    this.update(cx, |todo_mvc, cx| {
                        let todo_text = input_state.content();
                        todo_mvc.todo_items.push(TodoItem {
                            text: todo_text.to_string().into(),
                            checked: false,
                        });
                        cx.notify();
                    })
                    .ok();
                }
                _ => {}
            })
            .detach();
            state
        });

        div()
            .size_full()
            .bg(rgb(0xffb3b3))
            .child(
                div().flex().items_center().h(px(40.)).child(
                    input(&input_state)
                        .size_full()
                        .bg(rgb(0xbf00ff))
                        .text_color(rgb(0xd4d4d4))
                        .text_base()
                        .selection_color(gpui::rgba(0x3388ff44))
                        .cursor_color(rgb(0xffffff)),
                ),
            )
            .children(self.todo_items.iter().enumerate().map(|(ix, todo_item)| {
                div()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .w_5()
                            .h_5()
                            .id(("todo-item-checkbox", ix))
                            .on_click(cx.listener(move |todo_mvc, _, _, cx| {
                                let todo_item = &mut todo_mvc.todo_items[ix];
                                todo_item.checked = !todo_item.checked;
                                cx.notify();
                            }))
                            .bg(if todo_item.checked {
                                gpui::green()
                            } else {
                                gpui::red()
                            }),
                    )
                    .child(todo_item.text.clone())
            }))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .child(format!(
                        "Checked items: {}",
                        self.todo_items
                            .iter()
                            .filter(|todo_item| todo_item.checked)
                            .count()
                    ))
                    .child(format!(
                        "Unchecked items: {}",
                        self.todo_items
                            .iter()
                            .filter(|todo_item| !todo_item.checked)
                            .count()
                    )),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        bind_input_keys(cx, Some(InputBindings::default()));
        let bounds = Bounds::centered(None, size(px(700.0), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| TodoMvc { todo_items: vec![] }),
        )
        .unwrap();
        cx.activate(true);
    });
}

// Text box at the top
// Type in it, hit enter, adds a new todo item
// TODO items can be marked as complete
// Metadata: # of items left, filtering item type
// ALSO: everything is live updating
