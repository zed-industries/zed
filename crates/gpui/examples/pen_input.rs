#![cfg_attr(target_family = "wasm", no_main)]

//! Draws strokes whose width follows stylus pressure and shows the raw pen state
//! (tool, pressure, tilt, distance, buttons) so pen tablet support can be checked by hand.
//! A mouse works too: it draws at full width and reports no pen state.

#[path = "example_support/fonts.rs"]
mod example_support;

use std::collections::VecDeque;

use gpui::{
    Bounds, Context, Hsla, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PathBuilder,
    PathStyle, PenInput, PenTool, Pixels, Point, Render, SharedString, StrokeOptions, Window,
    WindowOptions, canvas, div, point, prelude::*, px, quad, rgb, size,
};
use gpui_platform::application;

const MAX_STROKE_WIDTH: f32 = 16.0;
const EVENT_LOG_LENGTH: usize = 12;

#[derive(Clone, Copy)]
struct StrokePoint {
    position: Point<Pixels>,
    width: Pixels,
}

#[derive(Clone)]
struct Stroke {
    color: Hsla,
    points: Vec<StrokePoint>,
}

#[derive(Default)]
struct PenInputExample {
    strokes: Vec<Stroke>,
    drawing: bool,
    hovered: bool,
    last_position: Option<Point<Pixels>>,
    last_pen: Option<PenInput>,
    pressed_button: Option<MouseButton>,
    move_count: usize,
    event_log: VecDeque<SharedString>,
}

fn pressure_of(pen: Option<PenInput>) -> f32 {
    pen.and_then(|pen| pen.pressure).unwrap_or(1.0)
}

fn stroke_width(pen: Option<PenInput>) -> Pixels {
    px(1.0 + pressure_of(pen) * (MAX_STROKE_WIDTH - 1.0))
}

fn stroke_color(pen: Option<PenInput>) -> Hsla {
    match pen.map(|pen| pen.tool) {
        // Erasing paints with the canvas color.
        Some(PenTool::Eraser) => gpui::white(),
        Some(PenTool::Pen) | None => gpui::black(),
        Some(PenTool::Pencil) => rgb(0x555555).into(),
        Some(PenTool::Brush) => rgb(0x1d4ed8).into(),
        Some(PenTool::Airbrush) => rgb(0xd946ef).into(),
    }
}

fn describe_pen(pen: Option<PenInput>) -> String {
    let Some(pen) = pen else {
        return "mouse".to_string();
    };
    let pressure = pen
        .pressure
        .map_or("-".to_string(), |pressure| format!("{pressure:.3}"));
    let tilt = pen.tilt.map_or("-".to_string(), |tilt| {
        format!("({:.1}, {:.1})", tilt.x, tilt.y)
    });
    let distance = pen
        .distance
        .map_or("-".to_string(), |distance| format!("{distance:.3}"));
    format!("{:?} p={pressure} tilt={tilt} d={distance}", pen.tool)
}

impl PenInputExample {
    fn log(&mut self, message: String) {
        if self.event_log.len() == EVENT_LOG_LENGTH {
            self.event_log.pop_front();
        }
        self.event_log.push_back(message.into());
    }

    fn on_mouse_down(&mut self, event: &MouseDownEvent, cx: &mut Context<Self>) {
        self.last_position = Some(event.position);
        self.last_pen = event.pen;
        self.pressed_button = Some(event.button);
        self.log(format!(
            "down {:?} x{} {}",
            event.button,
            event.click_count,
            describe_pen(event.pen)
        ));
        if event.button == MouseButton::Left {
            self.drawing = true;
            self.strokes.push(Stroke {
                color: stroke_color(event.pen),
                points: vec![StrokePoint {
                    position: event.position,
                    width: stroke_width(event.pen),
                }],
            });
        }
        cx.notify();
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        self.last_position = Some(event.position);
        self.last_pen = event.pen;
        self.pressed_button = event.pressed_button;
        self.move_count += 1;
        if self.drawing
            && let Some(stroke) = self.strokes.last_mut()
        {
            stroke.points.push(StrokePoint {
                position: event.position,
                width: stroke_width(event.pen),
            });
        }
        cx.notify();
    }

    fn on_mouse_up(&mut self, event: &MouseUpEvent, cx: &mut Context<Self>) {
        self.last_pen = event.pen;
        self.pressed_button = None;
        self.log(format!(
            "up   {:?} x{} {}",
            event.button,
            event.click_count,
            describe_pen(event.pen)
        ));
        if event.button == MouseButton::Left {
            self.drawing = false;
        }
        cx.notify();
    }

    fn render_drawing_area(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let strokes = self.strokes.clone();
        let cursor = self
            .hovered
            .then_some(self.last_position)
            .flatten()
            .map(|position| (position, stroke_width(self.last_pen)));

        div()
            .id("drawing-area")
            .flex_1()
            .h_full()
            .bg(gpui::white())
            .child(
                canvas(
                    |_, _, _| {},
                    move |_, _, window, _| {
                        for stroke in &strokes {
                            paint_stroke(stroke, window);
                        }
                        // Shows the brush size while hovering, so pressure can be
                        // seen even before the tip touches.
                        if let Some((position, width)) = cursor {
                            let radius = width / 2.0;
                            window.paint_quad(quad(
                                Bounds {
                                    origin: position - point(radius, radius),
                                    size: size(width, width),
                                },
                                radius,
                                gpui::transparent_black(),
                                px(1.),
                                rgb(0xef4444),
                                Default::default(),
                            ));
                        }
                    },
                )
                .size_full(),
            )
            .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                this.hovered = *hovered;
                this.log(if *hovered { "enter" } else { "exit" }.to_string());
                cx.notify();
            }))
            .on_any_mouse_down(cx.listener(|this, event, _, cx| this.on_mouse_down(event, cx)))
            .on_mouse_move(cx.listener(|this, event, _, cx| this.on_mouse_move(event, cx)))
            .capture_any_mouse_up(cx.listener(|this, event, _, cx| this.on_mouse_up(event, cx)))
    }

    fn render_status(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let pen = self.last_pen;
        let tilt = pen.and_then(|pen| pen.tilt);
        let position = self.last_position.map_or("-".to_string(), |position| {
            format!("({:.1}, {:.1})", position.x.as_f32(), position.y.as_f32())
        });
        let pressure = pen.and_then(|pen| pen.pressure);
        let distance = pen.and_then(|pen| pen.distance);

        div()
            .w(px(320.))
            .h_full()
            .p_3()
            .flex()
            .flex_col()
            .gap_2()
            .bg(rgb(0xf3f4f6))
            .border_l_1()
            .border_color(rgb(0xd1d5db))
            .text_sm()
            .child(
                div()
                    .id("clear")
                    .px_3()
                    .py_1()
                    .bg(gpui::black())
                    .text_color(gpui::white())
                    .active(|this| this.opacity(0.8))
                    .child("Clear")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.strokes.clear();
                        this.event_log.clear();
                        cx.notify();
                    })),
            )
            .child(format!(
                "device: {}",
                pen.map_or("mouse".to_string(), |pen| format!("{:?}", pen.tool))
            ))
            .child(format!("position: {position}"))
            .child(format!("pressed button: {:?}", self.pressed_button))
            .child(format!("move events: {}", self.move_count))
            .child(format!(
                "pressure: {}",
                pressure.map_or("not reported".to_string(), |pressure| format!(
                    "{pressure:.3}"
                ))
            ))
            .child(level_bar(pressure, rgb(0x2563eb).into()))
            .child(format!(
                "distance: {}",
                distance.map_or("not reported".to_string(), |distance| format!(
                    "{distance:.3}"
                ))
            ))
            .child(level_bar(distance, rgb(0x16a34a).into()))
            .child(format!(
                "tilt: {}",
                tilt.map_or("not reported".to_string(), |tilt| format!(
                    "x {:.1}°, y {:.1}°",
                    tilt.x, tilt.y
                ))
            ))
            .child(tilt_indicator(tilt))
            .child("events:")
            .children(self.event_log.iter().rev().cloned())
    }
}

fn level_bar(level: Option<f32>, color: Hsla) -> impl IntoElement {
    div()
        .h(px(10.))
        .w_full()
        .bg(rgb(0xd1d5db))
        .child(div().h_full().w(px(296. * level.unwrap_or(0.0))).bg(color))
}

fn paint_stroke(stroke: &Stroke, window: &mut Window) {
    // Each segment gets its own width so the stroke follows the pressure along
    // its length; round caps hide the joins between segments.
    let segments = stroke
        .points
        .windows(2)
        .filter_map(|pair| Some((pair.first()?, pair.get(1)?)));
    let mut painted_segment = false;
    for (start, end) in segments {
        painted_segment = true;
        paint_line(
            start.position,
            end.position,
            end.width,
            stroke.color,
            window,
        );
    }
    if !painted_segment && let Some(point) = stroke.points.first() {
        paint_line(
            point.position,
            point.position,
            point.width,
            stroke.color,
            window,
        );
    }
}

fn paint_line(
    start: Point<Pixels>,
    end: Point<Pixels>,
    width: Pixels,
    color: Hsla,
    window: &mut Window,
) {
    let options = StrokeOptions::default()
        .with_line_width(width.as_f32())
        .with_line_cap(lyon::path::LineCap::Round);
    let mut builder = PathBuilder::stroke(width).with_style(PathStyle::Stroke(options));
    builder.move_to(start);
    builder.line_to(end);
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

/// A top-down view of the stylus: the line points the way the pen leans, and
/// reaches the edge of the circle at 90°.
fn tilt_indicator(tilt: Option<Point<f32>>) -> impl IntoElement {
    canvas(
        |_, _, _| {},
        move |bounds, _, window, _| {
            let diameter = bounds.size.width.min(bounds.size.height);
            let radius = diameter / 2.0;
            let center = bounds.center();
            window.paint_quad(quad(
                Bounds {
                    origin: center - point(radius, radius),
                    size: size(diameter, diameter),
                },
                radius,
                gpui::white(),
                px(1.),
                rgb(0x9ca3af),
                Default::default(),
            ));
            let Some(tilt) = tilt else {
                return;
            };
            let end = center + point(radius * (tilt.x / 90.0), radius * (tilt.y / 90.0));
            paint_line(center, end, px(3.), rgb(0xef4444).into(), window);
        },
    )
    .size(px(120.))
}

impl Render for PenInputExample {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_row()
            .text_color(gpui::black())
            .child(self.render_drawing_area(cx))
            .child(self.render_status(cx))
    }
}

fn run_example() {
    application().run(|cx| {
        if !example_support::load_fonts(cx) {
            return;
        }
        cx.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            |_, cx| cx.new(|_| PenInputExample::default()),
        )
        .expect("failed to open window");
        cx.on_window_closed(|cx, _window_id| {
            cx.quit();
        })
        .detach();
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
