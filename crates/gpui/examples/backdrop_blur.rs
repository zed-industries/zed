#![cfg_attr(target_family = "wasm", no_main)]

use std::time::Duration;

use gpui::{
    Animation, AnimationExt as _, App, BackdropBlurEffect, Bounds, Context, Corners, Hsla,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, Render, Window,
    WindowBounds, WindowOptions, canvas, div, hsla, point, prelude::*, pulsating_between, px, rgb,
    size, white,
};
use gpui_platform::application;

const CARD_WIDTH: f32 = 258.;
const CARD_HEIGHT: f32 = 172.;

struct BackdropBlurDemo {
    card_positions: [Point<Pixels>; 5],
    dragging: Option<CardDrag>,
}

struct CardDrag {
    sample_ix: usize,
    pointer_offset: Point<Pixels>,
}

#[derive(Clone, Copy)]
struct BlurSample {
    radius: f32,
    tint: Option<Hsla>,
    code: &'static str,
}

fn samples() -> [BlurSample; 5] {
    [
        BlurSample {
            radius: 6.,
            tint: Some(hsla(188. / 360., 0.88, 0.53, 0.08)),
            code: "BackdropBlurEffect::new(px(6.))\n    .tint(hsla(\n        188. / 360.,\n        0.88, 0.53, 0.08,\n    ))",
        },
        BlurSample {
            radius: 10.,
            tint: None,
            code: "BackdropBlurEffect::new(px(10.))",
        },
        BlurSample {
            radius: 18.,
            tint: Some(hsla(0., 0., 1., 0.26)),
            code: "BackdropBlurEffect::new(px(18.))\n    .tint(hsla(\n        0., 0., 1., 0.26,\n    ))",
        },
        BlurSample {
            radius: 32.,
            tint: Some(hsla(36. / 360., 0.92, 0.55, 0.56)),
            code: "BackdropBlurEffect::new(px(32.))\n    .tint(hsla(\n        36. / 360.,\n        0.92, 0.55, 0.56,\n    ))",
        },
        BlurSample {
            radius: 48.,
            tint: Some(hsla(268. / 360., 0.78, 0.46, 0.82)),
            code: "BackdropBlurEffect::new(px(48.))\n    .tint(hsla(\n        268. / 360.,\n        0.78, 0.46, 0.82,\n    ))",
        },
    ]
}

impl BackdropBlurDemo {
    fn new() -> Self {
        Self {
            card_positions: [
                point(px(28.), px(32.)),
                point(px(318.), px(64.)),
                point(px(604.), px(32.)),
                point(px(96.), px(382.)),
                point(px(504.), px(370.)),
            ],
            dragging: None,
        }
    }

    fn start_drag(
        &mut self,
        sample_ix: usize,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.dragging = Some(CardDrag {
            sample_ix,
            pointer_offset: event.position - self.card_positions[sample_ix],
        });
        cx.notify();
    }

    fn drag_card(&mut self, event: &MouseMoveEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(drag) = &self.dragging else {
            return;
        };

        if !event.dragging() {
            self.dragging = None;
            cx.notify();
            return;
        }

        self.card_positions[drag.sample_ix] = event.position - drag.pointer_offset;
        cx.notify();
    }

    fn stop_drag(&mut self, _: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if self.dragging.take().is_some() {
            cx.notify();
        }
    }
}

impl Render for BackdropBlurDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let samples = samples();
        let dragging_ix = self.dragging.as_ref().map(|drag| drag.sample_ix);
        let mut overlay = div().absolute().inset_0();

        for sample_ix in 0..samples.len() {
            if dragging_ix == Some(sample_ix) {
                continue;
            }

            overlay = overlay.child(self.render_sample_card(sample_ix, samples[sample_ix], cx));
        }

        if let Some(sample_ix) = dragging_ix {
            overlay = overlay.child(self.render_sample_card(sample_ix, samples[sample_ix], cx));
        }

        div()
            .size_full()
            .bg(rgb(0x111827))
            .on_mouse_move(cx.listener(Self::drag_card))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::stop_drag))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::stop_drag))
            .child(background_pattern())
            .child(overlay)
    }
}

impl BackdropBlurDemo {
    fn render_sample_card(
        &self,
        sample_ix: usize,
        sample: BlurSample,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        sample_card(
            sample_ix,
            sample,
            self.card_positions[sample_ix],
            self.dragging
                .as_ref()
                .map(|drag| drag.sample_ix == sample_ix)
                .unwrap_or(false),
            cx,
        )
    }
}

fn background_pattern() -> impl IntoElement {
    div()
        .absolute()
        .size_full()
        .overflow_hidden()
        .child(opaque_color_fields())
        .child(translucent_background_elements())
        .child(animated_background_elements())
}

fn opaque_color_fields() -> impl IntoElement {
    div()
        .absolute()
        .size_full()
        .child(
            div()
                .absolute()
                .top(px(72.))
                .left(px(70.))
                .size(px(250.))
                .rounded(px(125.))
                .bg(rgb(0x22c55e)),
        )
        .child(
            div()
                .absolute()
                .bottom(px(48.))
                .right(px(96.))
                .size(px(280.))
                .rounded(px(140.))
                .bg(rgb(0x38bdf8)),
        )
        .child(
            div()
                .absolute()
                .top(px(286.))
                .left(px(118.))
                .right(px(138.))
                .h(px(82.))
                .bg(rgb(0xfacc15)),
        )
}

fn translucent_background_elements() -> impl IntoElement {
    div().absolute().size_full().child(
        div()
            .absolute()
            .top(px(172.))
            .left(px(430.))
            .w(px(290.))
            .h(px(180.))
            .rounded(px(28.))
            .border_1()
            .border_color(white().opacity(0.42))
            .bg(white().opacity(0.20)),
    )
}

fn animated_background_elements() -> impl IntoElement {
    div().absolute().size_full().child(
        div()
            .absolute()
            .top(px(74.))
            .right(px(312.))
            .size(px(168.))
            .rounded(px(84.))
            .with_animation(
                "animated-orb-alpha",
                Animation::new(Duration::from_secs(4))
                    .repeat()
                    .with_easing(pulsating_between(0.12, 0.62)),
                |this, alpha| this.bg(hsla(210. / 360., 0.40, 0.98, alpha)),
            ),
    )
}

fn sample_card(
    sample_ix: usize,
    sample: BlurSample,
    position: Point<Pixels>,
    is_dragging: bool,
    cx: &mut Context<BackdropBlurDemo>,
) -> impl IntoElement {
    let radius = px(sample.radius);
    let corner_radii = Corners::all(px(18.));
    let effect = sample.tint.map_or_else(
        || BackdropBlurEffect::new(radius),
        |tint| BackdropBlurEffect::new(radius).tint(tint),
    );
    div()
        .id(("backdrop-blur-card", sample_ix))
        .absolute()
        .left(position.x)
        .top(position.y)
        .overflow_hidden()
        .w(px(CARD_WIDTH))
        .h(px(CARD_HEIGHT))
        .rounded(px(18.))
        .border_1()
        .border_color(if is_dragging {
            hsla(0., 0., 1., 0.72)
        } else {
            hsla(0., 0., 1., 0.38)
        })
        .cursor_move()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, event, window, cx| {
                this.start_drag(sample_ix, event, window, cx);
            }),
        )
        .child(
            canvas(
                |_, _, _| {},
                move |bounds, _, window, _| {
                    window.paint_backdrop_blur_rect(bounds, corner_radii, effect);
                },
            )
            .absolute()
            .size_full(),
        )
        .child(
            div()
                .absolute()
                .size_full()
                .p_5()
                .flex()
                .flex_col()
                .text_color(white())
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .font_family(".ZedMono")
                        .text_sm()
                        .line_height(px(18.))
                        .text_color(hsla(0., 0., 1., 0.86))
                        .children(sample.code.lines().map(|line| div().child(line))),
                ),
        )
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(900.), px(620.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| BackdropBlurDemo::new()),
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
