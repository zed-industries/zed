#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, TitlebarOptions, Window, WindowBackgroundAppearance, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, rgba, size,
};
use gpui_platform::application;

/// A small AI chat client demonstrating [`Styled::glass`].
///
/// The window uses a system glass background, so the wallpaper shows through.
/// The root element is the glass surface (`.glass(true)`): it has a translucent
/// fill and glass mode is inherited by its children, so the sidebar's rounded
/// selected/hover rows only blend RGB and preserve the surface alpha — their
/// anti-aliased corners stay clean instead of punching through the glass. The
/// system titlebar is hidden (`appears_transparent`), so the glass surface
/// extends to the top of the window behind the traffic-light buttons.
///
/// Following Apple's Liquid Glass guidance the chat panel stays opaque so its
/// content reads crisply. It opts out of the inherited glass mode with
/// `.glass(false)`, so its white fill and border blend normally; without it the
/// border's alpha would be replaced by the glass backdrop's and wash out.
struct ChatApp {
    selected_item: usize,
}

const NAV: [&str; 5] = ["New chat", "Search", "Plugins", "Automations", "Sites"];

impl Render for ChatApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_item = self.selected_item;

        let fg = rgb(0x1A1C1F);
        let muted = rgb(0xa0a0a0);
        let border = rgb(0xe0e0e0);
        let subtle = rgba(0x0000000a); // hairline border + hover fill
        let highlight = rgba(0x0000000f); // selected row fill

        let nav_row = |ix: usize, label: &'static str, selected: bool| {
            div()
                .id(("nav", ix))
                .flex()
                .items_center()
                .h_7()
                .px_3()
                .rounded(px(7.))
                .text_sm()
                .text_color(fg)
                .when(selected, |row| row.bg(highlight))
                .when(!selected, |row| row.hover(|row| row.bg(subtle)))
                .child(label)
        };

        div()
            .flex()
            .flex_row()
            .size_full()
            .text_color(fg)
            .bg(rgba(0xFBFBFBaa))
            // The root is the glass surface: its translucent fill lets the
            // wallpaper through, and glass mode is inherited by the sidebar so
            // its rounded rows don't punch through the glass.
            .glass(true)
            .child(
                // Sidebar
                div()
                    .flex()
                    .flex_col()
                    .w(px(232.))
                    .flex_shrink_0()
                    .h_full()
                    .px_2()
                    .pt(px(40.))
                    .pb_2()
                    .gap(px(1.))
                    .children(NAV.iter().enumerate().map(|(ix, label)| {
                        nav_row(ix, label, ix == selected_item).on_click(cx.listener(
                            move |this, _, _, cx| {
                                this.selected_item = ix;
                                cx.notify();
                            },
                        ))
                    }))
                    .child(div().flex_1())
                    .child(nav_row(NAV.len(), "Settings", false)),
            )
            .child(
                // Chat
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .rounded_l(px(20.))
                    // Opaque content panel: turn glass back off so its white
                    // fill and left border blend normally, instead of inheriting
                    // the root's glass mode (which preserves the backdrop alpha
                    // and washes the border out).
                    .glass(false)
                    .min_w_0()
                    .h_full()
                    .items_center()
                    .px(px(32.))
                    .bg(gpui::white())
                    .border_l_1()
                    .border_color(border)
                    .shadow_xs()
                    .child(div().flex_1())
                    .child(
                        div()
                            .w_full()
                            .max_w(px(620.))
                            .pb(px(40.))
                            .text_center()
                            .text_2xl()
                            .text_color(fg)
                            .child("What should we build?"),
                    )
                    .child(
                        div().w_full().max_w(px(620.)).child(
                            // Input card
                            div()
                                .flex()
                                .flex_col()
                                .w_full()
                                .rounded(px(16.))
                                .border_1()
                                .border_color(border)
                                .bg(gpui::white())
                                .shadow_xs()
                                .child(div().p_4().h_20().text_color(muted).child("Do anything")),
                        ),
                    )
                    // Bottom whitespace.
                    .child(div().flex_1()),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(680.), px(500.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Blurred,
                // Hide the system titlebar so the glass sidebar reaches the top
                // of the window; the traffic-light buttons stay on top of it.
                titlebar: Some(TitlebarOptions {
                    #[cfg(target_os = "macos")]
                    appears_transparent: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ChatApp { selected_item: 0 }),
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
