use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};

struct UniformTableExample {}

impl Render for UniformTableExample {
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        const COLS: usize = 24;
        const ROWS: usize = 100;
        let mut headers = [0; COLS];

        for column in 0..COLS {
            headers[column] = column;
        }

        div().bg(rgb(0xffffff)).child(
            gpui::uniform_table("simple table", ROWS, move |range, _, _| {
                dbg!(&range);
                range
                    .map(|row_index| {
                        let mut row = [0; COLS];
                        for col in 0..COLS {
                            row[col] = (row_index + 1) * (col + 1);
                        }
                        row.map(|cell| ToString::to_string(&cell))
                            .map(|cell| div().flex().flex_row().child(cell))
                            .map(IntoElement::into_any_element)
                    })
                    .collect()
            })
            .with_width_from_item(Some(ROWS - 1)),
        )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| UniformTableExample {}),
        )
        .unwrap();
    });
}
