use std::sync::LazyLock;

use gpui::{
    App, Application, Context, Global, Image, MenuItem, SharedString, SystemTray, Window,
    WindowOptions, actions, div, prelude::*,
};

struct Example;

impl Render for Example {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(gpui::white())
            .flex()
            .size_full()
            .justify_center()
            .items_center()
            .child("Example for set Tray Icon")
    }
}

static TRAY_ICON: LazyLock<Image> = LazyLock::new(|| {
    Image::from_bytes(
        gpui::ImageFormat::Png,
        include_bytes!("image/app-icon.png").to_vec(),
    )
});

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_global(AppState::new());

        // Bring the menu bar to the foreground (so you can see the menu bar)
        cx.activate(true);
        // Register the `quit` function so it can be referenced by the `MenuItem::action` in the menu bar
        cx.on_action(quit);
        cx.on_action(toggle_check);

        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Example))
            .unwrap();

        let tray = SystemTray::new()
            .icon(TRAY_ICON.clone())
            .title("Tray")
            .tooltip("This is a tray icon")
            .menu(build_menus);
        cx.set_tray(tray);
    });
}

fn build_tray() -> SystemTray {
    SystemTray::new()
        .icon(TRAY_ICON.clone())
        .title("Tray")
        .tooltip("This is a tray icon")
        .menu(build_menus)
}

#[derive(PartialEq)]
enum ViewMode {
    List,
    Grid,
}

impl ViewMode {
    fn as_str(&self) -> &'static str {
        match self {
            ViewMode::List => "List",
            ViewMode::Grid => "Grid",
        }
    }

    fn toggle(&mut self) {
        *self = match self {
            ViewMode::List => ViewMode::Grid,
            ViewMode::Grid => ViewMode::List,
        }
    }
}

impl Into<SharedString> for ViewMode {
    fn into(self) -> SharedString {
        match self {
            ViewMode::List => "List",
            ViewMode::Grid => "Grid",
        }
        .into()
    }
}

struct AppState {
    view_mode: ViewMode,
}

impl AppState {
    fn new() -> Self {
        Self {
            view_mode: ViewMode::List,
        }
    }
}

impl Global for AppState {}

fn build_menus(cx: &mut App) -> Vec<MenuItem> {
    let app_state = cx.global::<AppState>();

    vec![
        MenuItem::action(ViewMode::List, ToggleCheck)
            .checked(app_state.view_mode == ViewMode::List),
        MenuItem::action(ViewMode::Grid, ToggleCheck)
            .checked(app_state.view_mode == ViewMode::Grid),
        MenuItem::separator(),
        MenuItem::action("Quit", Quit),
    ]
}

// Associate actions using the `actions!` macro (or `Action` derive macro)
actions!(example, [Quit, ToggleCheck]);

// Define the quit function that is registered with the App
fn quit(_: &Quit, cx: &mut App) {
    println!("Gracefully quitting the application . . .");
    cx.quit();
}

fn toggle_check(_: &ToggleCheck, cx: &mut App) {
    println!("Toggling view mode . . .");
    {
        let app_state = cx.global_mut::<AppState>();
        app_state.view_mode.toggle();
    }

    let app_state = cx.global::<AppState>();
    cx.set_tray(build_tray().title(format!("Mode: {}", app_state.view_mode.as_str())));
}
