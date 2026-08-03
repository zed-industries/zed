use gpui::{
    App, AppContext as _, Bounds, BoxShadow, Context, Div, IntoElement, ParentElement as _, Render,
    Styled as _, Window, WindowBounds, WindowOptions, div, hsla, point, px, size,
};
use gpui_platform::application;

const BG: u32 = 0xf1f5f9;
const TEXT: u32 = 0x334155;
const TEXT_MUTED: u32 = 0x64748b;
const BORDER: u32 = 0xe2e8f0;

fn card(label: &str) -> Div {
    div()
        .flex()
        .items_center()
        .justify_center()
        .w(px(100.))
        .h(px(60.))
        .rounded(px(8.))
        .bg(gpui::white())
        .border_1()
        .border_color(gpui::rgb(BORDER))
        .text_color(gpui::rgb(TEXT))
        .child(label.to_string())
}

fn row_label(text: &str) -> Div {
    div()
        .text_color(gpui::rgb(TEXT_MUTED))
        .w(px(120.))
        .flex()
        .items_center()
        .text_xs()
        .child(text.to_string())
}

struct InsetShadowExample;

impl Render for InsetShadowExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(gpui::rgb(BG))
            .items_center()
            .justify_center()
            .gap_6()
            .child(
                div()
                    .text_xl()
                    .text_color(gpui::rgb(TEXT))
                    .child("Inset Shadow Helpers"),
            )
            // Tailwind preset helpers
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_6()
                    .child(row_label("Presets"))
                    .child(card("inset_2xs").shadow_inset_2xs())
                    .child(card("inset_xs").shadow_inset_xs())
                    .child(card("inset_sm").shadow_inset_sm()),
            )
            // Custom inset shadows with larger values
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_6()
                    .child(row_label("Custom"))
                    .child(card("inset md").shadow(vec![BoxShadow {
                        color: hsla(0., 0., 0., 0.15),
                        offset: point(px(0.), px(6.)),
                        blur_radius: px(5.),
                        spread_radius: px(0.),
                        inset: true,
                    }]))
                    .child(card("inset lg").shadow(vec![BoxShadow {
                        color: hsla(0., 0., 0., 0.15),
                        offset: point(px(0.), px(8.)),
                        blur_radius: px(6.),
                        spread_radius: px(0.),
                        inset: true,
                    }]))
                    .child(card("inset xl").shadow(vec![BoxShadow {
                        color: hsla(0., 0., 0., 0.15),
                        offset: point(px(0.), px(10.)),
                        blur_radius: px(7.),
                        spread_radius: px(0.),
                        inset: true,
                    }])),
            )
            // Inset shadows with spread radius
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_6()
                    .child(row_label("Spread"))
                    .child(card("spread 0").shadow(vec![BoxShadow {
                        color: hsla(0., 0., 0., 0.15),
                        offset: point(px(0.), px(4.)),
                        blur_radius: px(3.),
                        spread_radius: px(0.),
                        inset: true,
                    }]))
                    .child(card("spread 2").shadow(vec![BoxShadow {
                        color: hsla(0., 0., 0., 0.15),
                        offset: point(px(0.), px(4.)),
                        blur_radius: px(3.),
                        spread_radius: px(2.),
                        inset: true,
                    }]))
                    .child(card("spread 4").shadow(vec![BoxShadow {
                        color: hsla(0., 0., 0., 0.15),
                        offset: point(px(0.), px(4.)),
                        blur_radius: px(3.),
                        spread_radius: px(4.),
                        inset: true,
                    }])),
            )
            // Outer shadows for comparison
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_6()
                    .child(row_label("Outer"))
                    .child(card("shadow_sm").shadow_sm())
                    .child(card("shadow_md").shadow_md())
                    .child(card("shadow_lg").shadow_lg()),
            )
    }
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(900.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                window.set_rem_size(px(12.));
                cx.new(|_cx| InsetShadowExample)
            },
        )
        .unwrap();
    });
}
