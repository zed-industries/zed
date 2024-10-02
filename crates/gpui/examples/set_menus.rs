use gpui::*;

struct SetMenus;

impl Render for SetMenus {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child("Set Menus Example")
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        // Bring the menu bar to the foreground (so you can see the menu bar)
        cx.activate(true);
        // Register the `quit` function so it can be referenced by the `MenuItem::action` in the menu bar
        cx.on_action(quit);
        // Add menu items
        cx.set_menus(vec![Menu {
            name: "set_menus".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| SetMenus {})
        })
        .unwrap();
    });
}

// Associate actions using the `actions!` macro (or `impl_actions!` macro)
actions!(set_menus, [Quit]);

// Define the quit function that is registered with the AppContext
fn quit(_: &Quit, cx: &mut AppContext) {
    println!("Gracefully quitting the application . . .");
    cx.quit();
}
