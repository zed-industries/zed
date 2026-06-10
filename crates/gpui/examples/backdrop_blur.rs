#![cfg_attr(target_family = "wasm", no_main)]

use std::time::Duration;

use gpui::{
    Animation, AnimationExt as _, App, BackdropBlurEffect, Bounds, Context, Corners, CursorStyle,
    Hsla, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PinchEvent, Pixels, Point,
    Render, ScrollWheelEvent, Size, Window, WindowBounds, WindowOptions, canvas, div, hsla, point,
    prelude::*, pulsating_between, px, rgb, rgba, size, white,
};
use gpui_platform::application;

const INITIAL_CARD_WIDTH: f32 = 258.;
const INITIAL_CARD_HEIGHT: f32 = 172.;
const MIN_CARD_WIDTH: f32 = 210.;
const MIN_CARD_HEIGHT: f32 = 136.;
const RESIZE_HANDLE_SIZE: f32 = 24.;
const MIN_ZOOM: f32 = 0.7;
const MAX_ZOOM: f32 = 1.6;
const ZOOM_STEP: f32 = 0.1;

struct BackdropBlurDemo {
    card_positions: [Point<Pixels>; 5],
    card_sizes: [Size<Pixels>; 5],
    zoom: f32,
    interaction: Option<CardInteraction>,
}

#[derive(Clone, Copy)]
enum CardInteraction {
    Drag(CardDrag),
    Resize(CardResize),
}

impl CardInteraction {
    fn sample_ix(&self) -> usize {
        match self {
            Self::Drag(drag) => drag.sample_ix,
            Self::Resize(resize) => resize.sample_ix,
        }
    }
}

#[derive(Clone, Copy)]
struct CardDrag {
    sample_ix: usize,
    pointer_offset: Point<Pixels>,
}

#[derive(Clone, Copy)]
struct CardResize {
    sample_ix: usize,
    pointer_start: Point<Pixels>,
    initial_size: Size<Pixels>,
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
            tint: Some(rgba(0xffffff42).into()),
            code: "BackdropBlurEffect::new(px(18.))\n    .tint(rgba(0xffffff42))",
        },
        BlurSample {
            radius: 32.,
            tint: Some(rgb(0xf59e0b).into()),
            code: "BackdropBlurEffect::new(px(32.))\n    .tint(rgb(0xf59e0b))",
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
            card_sizes: [size(px(INITIAL_CARD_WIDTH), px(INITIAL_CARD_HEIGHT)); 5],
            zoom: 1.,
            interaction: None,
        }
    }

    fn start_interaction(
        &mut self,
        sample_ix: usize,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let pointer = self.screen_to_scene(event.position);
        self.interaction = if self.in_resize_handle(sample_ix, event.position) {
            Some(CardInteraction::Resize(CardResize {
                sample_ix,
                pointer_start: pointer,
                initial_size: self.card_sizes[sample_ix],
            }))
        } else {
            Some(CardInteraction::Drag(CardDrag {
                sample_ix,
                pointer_offset: pointer - self.card_positions[sample_ix],
            }))
        };
        cx.notify();
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(interaction) = self.interaction else {
            return;
        };

        if !event.dragging() {
            self.interaction = None;
            cx.notify();
            return;
        }

        let pointer = self.screen_to_scene(event.position);
        match interaction {
            CardInteraction::Drag(drag) => {
                self.card_positions[drag.sample_ix] = pointer - drag.pointer_offset;
            }
            CardInteraction::Resize(resize) => {
                let delta = pointer - resize.pointer_start;
                self.card_sizes[resize.sample_ix] = size(
                    clamp_dimension(resize.initial_size.width + delta.x, px(MIN_CARD_WIDTH)),
                    clamp_dimension(resize.initial_size.height + delta.y, px(MIN_CARD_HEIGHT)),
                );
            }
        }
        cx.notify();
    }

    fn stop_interaction(&mut self, _: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if self.interaction.take().is_some() {
            cx.notify();
        }
    }

    fn zoom_from_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let delta = event.delta.pixel_delta(px(16.)).y.as_f32();
        if delta != 0. {
            self.adjust_zoom(delta / 480., cx);
        }
    }

    fn zoom_from_pinch(
        &mut self,
        event: &PinchEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.adjust_zoom(event.delta, cx);
    }

    fn adjust_zoom(&mut self, delta: f32, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom + delta, cx);
    }

    fn set_zoom(&mut self, zoom: f32, cx: &mut Context<Self>) {
        let zoom = zoom.clamp(MIN_ZOOM, MAX_ZOOM);
        if (self.zoom - zoom).abs() > f32::EPSILON {
            self.zoom = zoom;
            cx.notify();
        }
    }

    fn screen_to_scene(&self, position: Point<Pixels>) -> Point<Pixels> {
        point(
            px(position.x.as_f32() / self.zoom),
            px(position.y.as_f32() / self.zoom),
        )
    }

    fn scaled_point(&self, position: Point<Pixels>) -> Point<Pixels> {
        point(position.x * self.zoom, position.y * self.zoom)
    }

    fn scaled_size(&self, dimensions: Size<Pixels>) -> Size<Pixels> {
        size(dimensions.width * self.zoom, dimensions.height * self.zoom)
    }

    fn in_resize_handle(&self, sample_ix: usize, position: Point<Pixels>) -> bool {
        let card_position = self.scaled_point(self.card_positions[sample_ix]);
        let card_size = self.scaled_size(self.card_sizes[sample_ix]);
        let handle_size = px(RESIZE_HANDLE_SIZE);

        position.x >= card_position.x + card_size.width - handle_size
            && position.y >= card_position.y + card_size.height - handle_size
            && position.x <= card_position.x + card_size.width
            && position.y <= card_position.y + card_size.height
    }
}

impl Render for BackdropBlurDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let samples = samples();
        let interacting_ix = self.interaction.as_ref().map(CardInteraction::sample_ix);
        let mut overlay = div().absolute().inset_0();

        for sample_ix in 0..samples.len() {
            if interacting_ix == Some(sample_ix) {
                continue;
            }

            overlay = overlay.child(self.render_sample_card(sample_ix, samples[sample_ix], cx));
        }

        if let Some(sample_ix) = interacting_ix {
            overlay = overlay.child(self.render_sample_card(sample_ix, samples[sample_ix], cx));
        }

        div()
            .size_full()
            .bg(rgb(0x111827))
            .on_scroll_wheel(cx.listener(Self::zoom_from_wheel))
            .on_pinch(cx.listener(Self::zoom_from_pinch))
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::stop_interaction))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::stop_interaction))
            .child(background_pattern())
            .child(overlay)
            .child(self.render_zoom_controls(cx))
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
            self.scaled_point(self.card_positions[sample_ix]),
            self.scaled_size(self.card_sizes[sample_ix]),
            self.interaction
                .as_ref()
                .map(|interaction| interaction.sample_ix() == sample_ix)
                .unwrap_or(false),
            self.zoom,
            cx,
        )
    }

    fn render_zoom_controls(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let zoom_label = format!("{:.0}%", self.zoom * 100.);

        div()
            .absolute()
            .top(px(20.))
            .right(px(20.))
            .p_1()
            .flex()
            .items_center()
            .gap_1()
            .rounded(px(10.))
            .border_1()
            .border_color(white().opacity(0.24))
            .bg(hsla(0., 0., 0., 0.28))
            .text_color(white())
            .font_family(".ZedMono")
            .child(
                div()
                    .id("zoom-out")
                    .w(px(30.))
                    .h(px(28.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(7.))
                    .cursor_pointer()
                    .hover(|this| this.bg(white().opacity(0.14)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.adjust_zoom(-ZOOM_STEP, cx);
                    }))
                    .child("-"),
            )
            .child(
                div()
                    .w(px(58.))
                    .h(px(28.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xs()
                    .child(zoom_label),
            )
            .child(
                div()
                    .id("zoom-in")
                    .w(px(30.))
                    .h(px(28.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(7.))
                    .cursor_pointer()
                    .hover(|this| this.bg(white().opacity(0.14)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.adjust_zoom(ZOOM_STEP, cx);
                    }))
                    .child("+"),
            )
            .child(
                div()
                    .id("zoom-reset")
                    .w(px(48.))
                    .h(px(28.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(7.))
                    .text_xs()
                    .cursor_pointer()
                    .hover(|this| this.bg(white().opacity(0.14)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.set_zoom(1., cx);
                    }))
                    .child("1x"),
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
    dimensions: Size<Pixels>,
    is_interacting: bool,
    zoom: f32,
    cx: &mut Context<BackdropBlurDemo>,
) -> impl IntoElement {
    let radius = px(sample.radius);
    let corner_radii = Corners::all(px(18. * zoom));
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
        .w(dimensions.width)
        .h(dimensions.height)
        .rounded(px(18. * zoom))
        .border_1()
        .border_color(if is_interacting {
            hsla(0., 0., 1., 0.72)
        } else {
            hsla(0., 0., 1., 0.38)
        })
        .cursor_move()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, event, window, cx| {
                this.start_interaction(sample_ix, event, window, cx);
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
                .p(px(20. * zoom))
                .flex()
                .flex_col()
                .text_color(white())
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .font_family(".ZedMono")
                        .text_size(px(14. * zoom))
                        .line_height(px(18. * zoom))
                        .text_color(hsla(0., 0., 1., 0.86))
                        .children(sample.code.lines().map(|line| div().child(line))),
                ),
        )
        .child(resize_handle(zoom))
}

fn resize_handle(zoom: f32) -> impl IntoElement {
    let handle_size = px(RESIZE_HANDLE_SIZE);
    let line_color = white().opacity(0.72);

    div()
        .absolute()
        .right_0()
        .bottom_0()
        .size(handle_size)
        .cursor(CursorStyle::ResizeUpLeftDownRight)
        .child(
            div()
                .absolute()
                .right(px(5. * zoom))
                .bottom(px(7. * zoom))
                .w(px(12. * zoom))
                .h(px(1.))
                .bg(line_color),
        )
        .child(
            div()
                .absolute()
                .right(px(5. * zoom))
                .bottom(px(12. * zoom))
                .w(px(7. * zoom))
                .h(px(1.))
                .bg(line_color),
        )
}

fn clamp_dimension(value: Pixels, min: Pixels) -> Pixels {
    if value < min { min } else { value }
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
