use gpui::{
    App, Application, Context, Global, Menu, MenuItem, SystemMenuType, Window, WindowOptions,
    actions, div, prelude::*, rgb,
};

struct SetMenus;

impl Render for SetMenus {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
    Application::new().run(|cx: &mut App| {
        cx.set_global(AppState::new());

        // Bring the menu bar to the foreground (so you can see the menu bar)
        cx.activate(true);
        // Register the `quit` function so it can be referenced by the `MenuItem::action` in the menu bar
        cx.on_action(quit);
        cx.on_action(toggle_check);
        // Add menu items
        set_app_menus(cx);
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| SetMenus {}))
            .unwrap();
    });
}

struct AppState {
    checked: bool,
}

impl AppState {
    fn new() -> Self {
        Self { checked: false }
    }
}

impl Global for AppState {}

fn set_app_menus(cx: &mut App) {
    let app_state = cx.global::<AppState>();
    cx.set_menus(vec![Menu {
        name: "set_menus".into(),
        items: vec![
            MenuItem::os_submenu("Services", SystemMenuType::Services),
            MenuItem::separator(),
            MenuItem::action("Checkable Item", ToggleCheck).checked(app_state.checked),
            MenuItem::separator(),
            MenuItem::action("Quit", Quit),
        ],
    }]);
}

// Associate actions using the `actions!` macro (or `Action` derive macro)
actions!(set_menus, [Quit, ToggleCheck]);

// Define the quit function that is registered with the App
fn quit(_: &Quit, cx: &mut App) {
    println!("Gracefully quitting the application . . .");
    cx.quit();
}

fn toggle_check(_: &ToggleCheck, cx: &mut App) {
    let app_state = cx.global_mut::<AppState>();
    app_state.checked = !app_state.checked;
    set_app_menus(cx);
}
