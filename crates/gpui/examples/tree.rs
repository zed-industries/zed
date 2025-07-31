//! Renders a div with deep children hierarchy. This example is useful to exemplify that Zed can
//! handle deep hierarchies (even though it cannot just yet!).
use std::sync::LazyLock;

use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    size,
};

struct Tree {}

static DEPTH: LazyLock<u64> = LazyLock::new(|| {
    std::env::var("GPUI_TREE_DEPTH")
        .ok()
        .and_then(|depth| depth.parse().ok())
        .unwrap_or_else(|| 50)
});

impl Render for Tree {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let mut depth = *DEPTH;
        static COLORS: [gpui::Hsla; 4] = [gpui::red(), gpui::blue(), gpui::green(), gpui::yellow()];
        let mut colors = COLORS.iter().cycle().copied();
        let mut next_div = || div().p_0p5().bg(colors.next().unwrap());
        let mut innermost_node = next_div();
        while depth > 0 {
            innermost_node = next_div().child(innermost_node);
            depth -= 1;
        }
        innermost_node
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
            |_, cx| cx.new(|_| Tree {}),
        )
        .unwrap();
    });
}
