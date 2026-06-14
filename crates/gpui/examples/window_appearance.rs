//! Window appearance demo.
//!
//! Run with: `cargo run -p gpui --example window_appearance`
//!
//! This app demonstrates [`App::set_window_appearance`], which overrides the native
//! window chrome (the window border and the titlebar) of every window to be light
//! or dark independent of the OS-wide setting.
//!
//! To see the effect on macOS: set the system to Light mode, then click "Dark".
//! The window's border and titlebar should switch to dark to match a dark theme,
//! instead of staying light. Click "Auto" to follow the system again.

#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, Rgba, Window, WindowAppearance, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use gpui_platform::application;

/// A palette whose colors switch together so the whole UI re-themes when the
/// appearance changes.
struct Palette {
    bg: Rgba,
    fg: Rgba,
    muted: Rgba,
    accent: Rgba,
    accent_fg: Rgba,
    control: Rgba,
}

impl Palette {
    fn new(is_dark: bool) -> Self {
        if is_dark {
            Self {
                bg: rgb(0x1e1e1e),
                fg: rgb(0xf4f4f5),
                muted: rgb(0x9a9a9a),
                accent: rgb(0x0059d1),
                accent_fg: rgb(0xffffff),
                control: rgb(0x2f2f2f),
            }
        } else {
            Self {
                bg: rgb(0xffffff),
                fg: rgb(0x18181b),
                muted: rgb(0x6a6a72),
                accent: rgb(0x0076f7),
                accent_fg: rgb(0xffffff),
                control: rgb(0xe4e4e4),
            }
        }
    }
}

struct AppearanceExample {
    /// The forced appearance, or `None` to follow the system.
    selected: Option<WindowAppearance>,
}

impl AppearanceExample {
    fn button(
        &self,
        label: &'static str,
        value: Option<WindowAppearance>,
        palette: &Palette,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let selected = self.selected == value;
        let (bg, fg) = if selected {
            (palette.accent, palette.accent_fg)
        } else {
            (palette.control, palette.fg)
        };

        div()
            .id(label)
            .flex()
            .items_center()
            .justify_center()
            .px_4()
            .py_1()
            .text_sm()
            .rounded_md()
            .cursor_pointer()
            .bg(bg)
            .text_color(fg)
            .child(label)
            .on_click(cx.listener(move |this, _event, _window, cx| {
                this.selected = value;
                cx.set_window_appearance(value);
                cx.notify();
            }))
    }
}

impl Render for AppearanceExample {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let appearance = window.appearance();
        // Theme the UI from the selection so the content re-themes immediately on click;
        // when following the system (`None`), use the effective appearance.
        let is_dark = match self.selected {
            Some(WindowAppearance::Dark | WindowAppearance::VibrantDark) => true,
            Some(_) => false,
            None => matches!(
                appearance,
                WindowAppearance::Dark | WindowAppearance::VibrantDark
            ),
        };
        let palette = Palette::new(is_dark);
        let selected_label = match self.selected {
            None => "Auto",
            Some(WindowAppearance::Light | WindowAppearance::VibrantLight) => "Light",
            Some(WindowAppearance::Dark | WindowAppearance::VibrantDark) => "Dark",
        };

        div()
            .flex()
            .size_full()
            .justify_center()
            .items_center()
            .bg(palette.bg)
            .text_color(palette.fg)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .w(px(340.))
                    .p_6()
                    .child(div().text_xl().child("Window Appearance"))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .text_sm()
                            .text_color(palette.muted)
                            .child(format!("Selected: {selected_label}"))
                            .child(format!("Effective appearance: {appearance:?}")),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(self.button("Auto", None, &palette, cx))
                            .child(self.button(
                                "Light",
                                Some(WindowAppearance::Light),
                                &palette,
                                cx,
                            ))
                            .child(self.button("Dark", Some(WindowAppearance::Dark), &palette, cx)),
                    )
                    .child(div().text_xs().text_color(palette.muted).child(
                        "Set the system to Light mode, then choose Dark: the native \
                                 window border and titlebar switch to dark to match.",
                    )),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(440.), px(380.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| {
                    // Re-render when the effective appearance changes, so the labels
                    // stay accurate while in `Auto` mode and the system theme toggles.
                    cx.observe_window_appearance(window, |_, _, cx| {
                        cx.notify();
                    })
                    .detach();
                    AppearanceExample { selected: None }
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
