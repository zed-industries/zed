use gpui::{
    App, Application, Context, FocusHandle, KeyBinding, Menu, MenuItem, PromptLevel,
    SystemMenuType, Window, WindowOptions, actions, div, prelude::*, rgb,
};

struct SetMenus {
    focus_handle: FocusHandle,
}

impl Render for SetMenus {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("root")
            .track_focus(&self.focus_handle)
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            // Actions can also be registered within elements so they are only active when relevant
            .on_action(|_: &Open, window, cx| {
                let _ = window.prompt(PromptLevel::Info, "Open action fired", None, &["OK"], cx);
            })
            .on_action(|_: &Copy, window, cx| {
                let _ = window.prompt(PromptLevel::Info, "Copy action fired", None, &["OK"], cx);
            })
            .on_action(|_: &Paste, window, cx| {
                let _ = window.prompt(PromptLevel::Info, "Paste action fired", None, &["OK"], cx);
            })
            .child("Set Menus Example")
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Bring the menu bar to the foreground (so you can see the menu bar)
        cx.activate(true);
        // Bind keys to some menu actions
        cx.bind_keys([
            KeyBinding::new("secondary-o", Open, None),
            KeyBinding::new("secondary-c", Copy, None),
            KeyBinding::new("secondary-v", Paste, None),
        ]);
        // Register the `quit` function so it can be referenced by the `MenuItem::action` in the menu bar
        cx.on_action(quit);
        // Add menu items
        cx.set_menus(vec![
            Menu {
                name: "set_menus".into(),
                items: vec![
                    MenuItem::os_submenu("Services", SystemMenuType::Services),
                    MenuItem::separator(),
                    MenuItem::action("Quit", Quit),
                ],
            },
            Menu {
                name: "File".into(),
                items: vec![MenuItem::action("Open", Open)],
            },
            Menu {
                name: "Edit".into(),
                items: vec![
                    MenuItem::action("Copy", Copy),
                    MenuItem::action("Paste", Paste),
                ],
            },
        ]);
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| SetMenus {
                focus_handle: cx.focus_handle(),
            })
        })
        .unwrap();
    });
}

// Associate actions using the `actions!` macro (or `Action` derive macro)
actions!(set_menus, [Quit, Open, Copy, Paste]);

// Define the quit function that is registered with the App
fn quit(_: &Quit, cx: &mut App) {
    println!("Gracefully quitting the application . . .");
    cx.quit();
}
