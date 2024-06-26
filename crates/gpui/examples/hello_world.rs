use gpui::*;
use prelude::FluentBuilder;

struct HelloWorld {
    text: SharedString,
}

/*
Things to do:
1. We need a way of calculating which edge or corner the mouse is on,
    and then dispatch on that
2. We need to improve the shadow rendering significantly
3. We need to implement the techniques in here in Zed
*/

impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let decorations = cx.window_decorations();
        let tiling = cx.window_tiling();
        let rounding = px(10.0);
        let shadow_size = px(20.0);
        let border_size = px(1.0);
        let grey = rgb(0x808080);

        div()
            .id("window-backdrop")
            .when(decorations == WindowDecorations::Client, |div| {
                div.bg(gpui::transparent_black())
                    .shadow(smallvec::smallvec![gpui::BoxShadow {
                        color: Hsla {
                            h: 0.,
                            s: 0.,
                            l: 0.,
                            a: 1.0,
                        },
                        blur_radius: shadow_size,
                        spread_radius: px(0.0),
                        offset: point(px(0.0), px(0.0)),
                    }])
                    .when(!(tiling.top || tiling.right), |div| {
                        div.rounded_tr(rounding)
                    })
                    .when(!(tiling.top || tiling.left), |div| div.rounded_tl(rounding))
                    .when(!tiling.top, |div| div.pt(shadow_size))
                    .when(!tiling.bottom, |div| div.pb(shadow_size))
                    .when(!tiling.left, |div| div.pl(shadow_size))
                    .when(!tiling.right, |div| div.pr(shadow_size))
                    .on_mouse_move(|e, cx| {
                        if e.dragging() {
                            cx.start_window_resize(ResizeEdge::Left)
                        }
                    })
            })
            .size_full()
            .child(
                div()
                    .when(decorations == WindowDecorations::Client, |div| {
                        div.border_color(grey)
                            .when(!(tiling.top || tiling.right), |div| {
                                div.rounded_tr(rounding)
                            })
                            .when(!(tiling.top || tiling.left), |div| div.rounded_tl(rounding))
                            .when(!tiling.top, |div| div.border_t(border_size))
                            .when(!tiling.bottom, |div| div.border_b(border_size))
                            .when(!tiling.left, |div| div.border_l(border_size))
                            .when(!tiling.right, |div| div.border_r(border_size))
                    })
                    .on_mouse_move(|_e, cx| {
                        cx.stop_propagation();
                    })
                    .bg(gpui::rgb(0xCCCCFF))
                    .size_full()
                    .flex()
                    .flex_col()
                    .justify_around()
                    .child(
                        div().w_full().flex().flex_row().justify_around().child(
                            div()
                                .id("hello")
                                .flex()
                                .bg(white())
                                .size(Length::Definite(Pixels(300.0).into()))
                                .justify_center()
                                .items_center()
                                .shadow_lg()
                                .border_1()
                                .border_color(rgb(0x0000ff))
                                .text_xl()
                                .text_color(rgb(0xffffff))
                                .child(div().w(px(100.0)).h(px(50.0)).bg(green()).shadow(
                                    smallvec::smallvec![gpui::BoxShadow {
                                        color: Hsla {
                                            h: 0.,
                                            s: 0.,
                                            l: 0.,
                                            a: 1.0,
                                        },
                                        blur_radius: px(20.0),
                                        spread_radius: px(0.0),
                                        offset: point(px(0.0), px(0.0)),
                                    }],
                                ))
                                .on_mouse_move(|e, cx| {
                                    if e.dragging() {
                                        cx.start_window_move();
                                    }
                                }),
                        ),
                    ),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(600.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Transparent,
                ..Default::default()
            },
            |cx| {
                cx.new_view(|cx| {
                    cx.observe_window_appearance(|_, cx| {
                        cx.notify();
                    })
                    .detach();
                    HelloWorld {
                        text: "World".into(),
                    }
                })
            },
        )
        .unwrap();
    });
}
