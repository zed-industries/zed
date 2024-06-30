use gpui::*;

struct SetTrayIcons;

impl Render for SetTrayIcons {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child("Set Tray Menu Example")
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        // Register the `quit` function so it can be referenced by the `MenuItem::action` in the menu bar
        cx.on_action(quit);
        cx.set_tray_item(TrayItem {
            icon: TrayIcon::Name("kmail"),
            title: "Testing",
            description: "Description",
            submenus: Vec::from_iter([
                TrayMenuItem::Submenu {
                    id: "Quit",
                    label: "Quit",
                    icon: Some(TrayIcon::Name("konqueror")),
                    toggle_type: None,
                    on_click: Some(Box::new(Quit)),
                    children: Vec::default(),
                },
                TrayMenuItem::Separator {
                    id: "Sep1",
                    label: Some("Testing"),
                },
                TrayMenuItem::Submenu {
                    id: "test",
                    label: "test",
                    icon: Some(TrayIcon::Name("konqueror")),
                    children: Vec::default(),
                    on_click: None,
                    toggle_type: None,
                },
            ]),
            ..Default::default()
        });
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| SetTrayIcons {})
        })
        .unwrap();
    });
}

// Associate actions using the `actions!` macro (or `impl_actions!` macro)
actions!(set_tray_menus, [Quit]);

// Define the quit function that is registered with the AppContext
fn quit(q: &Quit, cx: &mut AppContext) {
    println!("Gracefully quitting the application . . .");
    cx.quit();
}
