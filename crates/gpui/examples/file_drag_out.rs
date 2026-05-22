#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, ExternalPaths, FileDragSession, KeyBinding, Menu, MenuItem, MouseButton,
    SharedString, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size,
};
use gpui_platform::application;
use std::path::PathBuf;
use std::sync::OnceLock;

static TEST_DIR: OnceLock<PathBuf> = OnceLock::new();

fn test_dir() -> &'static PathBuf {
    TEST_DIR.get().unwrap()
}

struct FileDragOut {
    _drag_session: Option<FileDragSession>,
}

impl Render for FileDragOut {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let names = ["document.txt", "image.png", "archive.zip"];

        div()
            .flex()
            .flex_col()
            .gap_4()
            .bg(rgb(0xf5f5f5))
            .size_full()
            .justify_center()
            .items_center()
            .child(
                div()
                    .text_lg()
                    .text_color(rgb(0x333333))
                    .child("Click a file and drag it out."),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .children(names.iter().copied().map(|name| {
                        div()
                            .id(SharedString::from(name))
                            .px_6()
                            .py_4()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(rgb(0xcccccc))
                            .rounded_md()
                            .shadow_sm()
                            .cursor_pointer()
                            .hover(|this| this.bg(rgb(0xe8f0fe)))
                            .text_color(rgb(0x333333))
                            .child(SharedString::from(name))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, window, _| {
                                    let paths = ExternalPaths(vec![test_dir().join(name)].into());
                                    this._drag_session = window.start_file_drag(paths).ok();
                                }),
                            )
                    })),
            )
    }
}

fn run_example() {
    let tmp = std::env::temp_dir().join("gpui-file-drag-test");
    let _ = std::fs::create_dir_all(&tmp);
    for name in ["document.txt", "image.png", "archive.zip"] {
        let path = tmp.join(name);
        let _ = std::fs::write(&path, b"test content");
    }

    TEST_DIR.set(tmp).ok();
    let _cleanup = gpui_util::defer(|| {
        if let Some(dir) = TEST_DIR.get()
            && let Err(error) = std::fs::remove_dir_all(dir)
        {
            eprintln!("failed to remove temporary file drag directory: {error}");
        }
    });

    application().run(|cx: &mut App| {
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus([Menu::new("File Drag Out").items([MenuItem::action("Quit", Quit)])]);

        let bounds = Bounds::centered(None, size(px(600.), px(400.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| FileDragOut {
                    _drag_session: None,
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

actions!(example, [Quit]);

fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
