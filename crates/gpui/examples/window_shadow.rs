use gpui::*;
use prelude::FluentBuilder;

/*
Things to do:
1. We need a way of calculating which edge or corner the mouse is on,
    and then dispatch on that
2. We need to improve the shadow rendering significantly
3. We need to implement the techniques in here in Zed
*/

struct WindowShadowExample;

impl Render for WindowShadowExample {
    fn render(
        &mut self,
        _model: &Model<Self>,
        window: &mut Window,
        _cx: &mut AppContext,
    ) -> impl IntoElement {
        let decorations = window.window_decorations();
        let rounding = px(10.0);
        let shadow_size = px(10.0);
        let border_size = px(1.0);
        let grey = rgb(0x808080);
        window.set_client_inset(shadow_size);

        div()
            .id("window-backdrop")
            .bg(transparent_black())
            .map(|div| match decorations {
                Decorations::Server => div,
                Decorations::Client { tiling, .. } => div
                    .bg(gpui::transparent_black())
                    .child(
                        canvas(
                            |_bounds, window, _cx| {
                                window.insert_hitbox(
                                    Bounds::new(
                                        point(px(0.0), px(0.0)),
                                        window.window_bounds().get_bounds().size,
                                    ),
                                    false,
                                )
                            },
                            move |_bounds, hitbox, window, _cx| {
                                let mouse = window.mouse_position();
                                let size = window.window_bounds().get_bounds().size;
                                let Some(edge) = resize_edge(mouse, shadow_size, size) else {
                                    return;
                                };
                                window.set_cursor_style(
                                    match edge {
                                        ResizeEdge::Top | ResizeEdge::Bottom => {
                                            CursorStyle::ResizeUpDown
                                        }
                                        ResizeEdge::Left | ResizeEdge::Right => {
                                            CursorStyle::ResizeLeftRight
                                        }
                                        ResizeEdge::TopLeft | ResizeEdge::BottomRight => {
                                            CursorStyle::ResizeUpLeftDownRight
                                        }
                                        ResizeEdge::TopRight | ResizeEdge::BottomLeft => {
                                            CursorStyle::ResizeUpRightDownLeft
                                        }
                                    },
                                    &hitbox,
                                );
                            },
                        )
                        .size_full()
                        .absolute(),
                    )
                    .when(!(tiling.top || tiling.right), |div| {
                        div.rounded_tr(rounding)
                    })
                    .when(!(tiling.top || tiling.left), |div| div.rounded_tl(rounding))
                    .when(!tiling.top, |div| div.pt(shadow_size))
                    .when(!tiling.bottom, |div| div.pb(shadow_size))
                    .when(!tiling.left, |div| div.pl(shadow_size))
                    .when(!tiling.right, |div| div.pr(shadow_size))
                    .on_mouse_move(|_e, window, _cx| window.refresh())
                    .on_mouse_down(MouseButton::Left, move |e, window, _cx| {
                        let size = window.window_bounds().get_bounds().size;
                        let pos = e.position;

                        match resize_edge(pos, shadow_size, size) {
                            Some(edge) => window.start_resize(edge),
                            None => window.start_move(),
                        };
                    }),
            })
            .size_full()
            .child(
                div()
                    .cursor(CursorStyle::Arrow)
                    .map(|div| match decorations {
                        Decorations::Server => div,
                        Decorations::Client { tiling } => div
                            .border_color(grey)
                            .when(!(tiling.top || tiling.right), |div| {
                                div.rounded_tr(rounding)
                            })
                            .when(!(tiling.top || tiling.left), |div| div.rounded_tl(rounding))
                            .when(!tiling.top, |div| div.border_t(border_size))
                            .when(!tiling.bottom, |div| div.border_b(border_size))
                            .when(!tiling.left, |div| div.border_l(border_size))
                            .when(!tiling.right, |div| div.border_r(border_size))
                            .when(!tiling.is_tiled(), |div| {
                                div.shadow(smallvec::smallvec![gpui::BoxShadow {
                                    color: Hsla {
                                        h: 0.,
                                        s: 0.,
                                        l: 0.,
                                        a: 0.4,
                                    },
                                    blur_radius: shadow_size / 2.,
                                    spread_radius: px(0.),
                                    offset: point(px(0.0), px(0.0)),
                                }])
                            }),
                    })
                    .on_mouse_move(|_e, _window, cx| {
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
                                .child(
                                    div()
                                        .id("hello")
                                        .w(px(200.0))
                                        .h(px(100.0))
                                        .bg(green())
                                        .shadow(smallvec::smallvec![gpui::BoxShadow {
                                            color: Hsla {
                                                h: 0.,
                                                s: 0.,
                                                l: 0.,
                                                a: 1.0,
                                            },
                                            blur_radius: px(20.0),
                                            spread_radius: px(0.0),
                                            offset: point(px(0.0), px(0.0)),
                                        }])
                                        .map(|div| match decorations {
                                            Decorations::Server => div,
                                            Decorations::Client { .. } => div
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    |_e, window, _cx| {
                                                        window.start_move();
                                                    },
                                                )
                                                .on_click(|e, window, _cx| {
                                                    if e.down.button == MouseButton::Right {
                                                        window.show_menu(e.up.position);
                                                    }
                                                })
                                                .text_color(black())
                                                .child("this is the custom titlebar"),
                                        }),
                                ),
                        ),
                    ),
            )
    }
}

fn resize_edge(pos: Point<Pixels>, shadow_size: Pixels, size: Size<Pixels>) -> Option<ResizeEdge> {
    let edge = if pos.y < shadow_size && pos.x < shadow_size {
        ResizeEdge::TopLeft
    } else if pos.y < shadow_size && pos.x > size.width - shadow_size {
        ResizeEdge::TopRight
    } else if pos.y < shadow_size {
        ResizeEdge::Top
    } else if pos.y > size.height - shadow_size && pos.x < shadow_size {
        ResizeEdge::BottomLeft
    } else if pos.y > size.height - shadow_size && pos.x > size.width - shadow_size {
        ResizeEdge::BottomRight
    } else if pos.y > size.height - shadow_size {
        ResizeEdge::Bottom
    } else if pos.x < shadow_size {
        ResizeEdge::Left
    } else if pos.x > size.width - shadow_size {
        ResizeEdge::Right
    } else {
        return None;
    };
    Some(edge)
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(600.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Opaque,
                window_decorations: Some(WindowDecorations::Client),
                ..Default::default()
            },
            |_model, window, _cx| {
                window
                    .observe_appearance(|_, cx| {
                        cx.refresh();
                    })
                    .detach();
                WindowShadowExample
            },
        )
        .unwrap();
    });
}
