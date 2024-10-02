use gpui::*;

struct GridExample;

impl Render for GridExample {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .grid()
            .grid_cols(3)
            .grid_rows(3)
            .gap_4()
            .bg(rgb(0x2e7d32))
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .children((1..=9).map(|i| {
                div()
                    .bg(rgb(0x1b5e20))
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(format!("Cell {}", i * i * i))
            }))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| GridExample)
        })
        .unwrap();
    });
}
