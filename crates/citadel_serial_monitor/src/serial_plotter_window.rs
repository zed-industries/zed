use crate::plot_parser::parse_plot_line;
use crate::serial_connection::{
    GlobalSerialConnection, SerialConnection, SerialConnectionError, SerialLineReceived,
    default_port_name,
};
use editor::Editor;
use gpui::{
    App, Bounds, Context, Entity, FocusHandle, Focusable, Hsla, MouseButton, Pixels, Render,
    Size, Subscription, TitlebarOptions, Window, WindowBounds, WindowControlArea, WindowDecorations,
    WindowKind, WindowOptions, canvas, hsla, point, px,
};
use std::collections::VecDeque;
use ui::prelude::*;
use util::ResultExt;

const MAX_PLOT_POINTS: usize = 500;

#[derive(Clone)]
struct PlotSeries {
    label: String,
    color: Hsla,
    points: VecDeque<f32>,
}

fn series_color(index: usize) -> Hsla {
    const COLORS: [(f32, f32, f32); 5] = [
        (0.0, 0.7, 0.55),
        (0.33, 0.6, 0.45),
        (0.58, 0.7, 0.55),
        (0.13, 0.8, 0.55),
        (0.75, 0.6, 0.55),
    ];
    let (h, s, l) = COLORS[index % COLORS.len()];
    hsla(h, s, l, 1.0)
}

fn min_max(values: &[f32]) -> (f32, f32) {
    if values.is_empty() {
        return (0.0, 1.0);
    }
    let min = values.iter().copied().fold(f32::INFINITY, f32::min);
    let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    if (max - min).abs() < f32::EPSILON {
        (min - 1.0, max + 1.0)
    } else {
        (min, max)
    }
}

fn draw_series(
    series: &PlotSeries,
    bounds: Bounds<Pixels>,
    min_value: f32,
    max_value: f32,
    window: &mut Window,
) {
    if series.points.len() < 2 {
        return;
    }
    let range = (max_value - min_value).max(f32::EPSILON);
    let step_x = bounds.size.width.as_f32() / (series.points.len() - 1).max(1) as f32;
    let mut builder = gpui::PathBuilder::stroke(px(2.));
    for (index, value) in series.points.iter().enumerate() {
        let x = bounds.origin.x.as_f32() + index as f32 * step_x;
        let normalized = (value - min_value) / range;
        let y = bounds.origin.y.as_f32() + bounds.size.height.as_f32() * (1.0 - normalized);
        let point = point(px(x), px(y));
        if index == 0 {
            builder.move_to(point);
        } else {
            builder.line_to(point);
        }
    }
    if let Ok(path) = builder.build() {
        window.paint_path(path, series.color);
    }
}

pub struct SerialPlotterWindow {
    focus_handle: FocusHandle,
    connection: Entity<SerialConnection>,
    port_editor: Entity<Editor>,
    series: Vec<PlotSeries>,
    last_error: Option<String>,
    /// Set on mouse-down in the title row, consumed (and cleared) on the
    /// next mouse-move to trigger the actual OS-level window move -- not
    /// just moving on down, so a plain click doesn't start dragging. Mirrors
    /// `platform_title_bar`'s own drag handling, this codebase's only other
    /// place a window needs to be moved without relying on server-side
    /// decorations (which many Wayland compositors don't provide).
    should_move: bool,
    _subscriptions: Vec<Subscription>,
}

impl SerialPlotterWindow {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Invariant: citadel_serial_monitor::init(cx) runs at app startup,
        // before any window (and therefore this one) can be opened.
        let connection = cx.global::<GlobalSerialConnection>().0.clone();

        let default_port = default_port_name(cx).unwrap_or_default();
        let port_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(default_port.clone(), window, cx);
            editor.set_placeholder_text("Port (e.g. /dev/ttyACM0)", window, cx);
            editor
        });

        if !connection.read(cx).is_open && !default_port.is_empty() {
            let baud_rate = connection.read(cx).baud_rate;
            connection.update(cx, |connection, cx| {
                connection.connect(default_port, baud_rate, cx)
            });
        }

        let mut subscriptions = Vec::new();
        subscriptions.push(cx.subscribe(
            &connection,
            |this, _connection, event: &SerialLineReceived, cx| {
                this.ingest_line(&event.0, cx);
            },
        ));
        subscriptions.push(cx.subscribe(
            &connection,
            |this, _connection, event: &SerialConnectionError, cx| {
                this.last_error = Some(event.0.clone());
                cx.notify();
            },
        ));

        Self {
            focus_handle: cx.focus_handle(),
            connection,
            port_editor,
            series: Vec::new(),
            last_error: None,
            should_move: false,
            _subscriptions: subscriptions,
        }
    }

    fn ingest_line(&mut self, line: &str, cx: &mut Context<Self>) {
        for point in parse_plot_line(line) {
            let series = match self.series.iter_mut().find(|series| series.label == point.label) {
                Some(series) => series,
                None => {
                    let color = series_color(self.series.len());
                    self.series.push(PlotSeries {
                        label: point.label.clone(),
                        color,
                        points: VecDeque::new(),
                    });
                    self.series
                        .last_mut()
                        .expect("just pushed onto self.series above")
                }
            };
            series.points.push_back(point.value);
            if series.points.len() > MAX_PLOT_POINTS {
                series.points.pop_front();
            }
        }
        cx.notify();
    }

    fn reconnect(&mut self, cx: &mut Context<Self>) {
        let port_name = self.port_editor.read(cx).text(cx).trim().to_string();
        if port_name.is_empty() {
            return;
        }
        self.last_error = None;
        let baud_rate = self.connection.read(cx).baud_rate;
        self.connection
            .update(cx, |connection, cx| connection.connect(port_name, baud_rate, cx));
    }

    fn render_legend(&self) -> impl IntoElement {
        v_flex().gap_1().p_2().children(self.series.iter().map(|series| {
            h_flex()
                .gap_2()
                .child(div().w(px(10.)).h(px(10.)).bg(series.color))
                .child(Label::new(series.label.clone()))
                .child(Label::new(
                    series
                        .points
                        .back()
                        .map(|value| value.to_string())
                        .unwrap_or_default(),
                ))
        }))
    }
}

impl Focusable for SerialPlotterWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SerialPlotterWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let series = self.series.clone();

        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .text_color(cx.theme().colors().text)
            .child(
                // Most Wayland compositors don't render server-side
                // decorations for a plain xdg-toplevel, so a window with no
                // client-drawn titlebar of its own ends up with no way to
                // move it. This row is that titlebar: window_control_area
                // marks it as the drag region, and the down/move pair below
                // starts the actual OS-level move once the mouse has
                // genuinely moved (not on a plain click).
                h_flex()
                    .window_control_area(WindowControlArea::Drag)
                    .w_full()
                    .h(px(32.))
                    .px_2()
                    .items_center()
                    .bg(cx.theme().colors().title_bar_background)
                    .on_mouse_down_out(cx.listener(|this, _, _window, _cx| {
                        this.should_move = false;
                    }))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _window, _cx| {
                            this.should_move = false;
                        }),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _window, _cx| {
                            this.should_move = true;
                        }),
                    )
                    .on_mouse_move(cx.listener(|this, _, window, _cx| {
                        if this.should_move {
                            this.should_move = false;
                            window.start_window_move();
                        }
                    }))
                    .child(Label::new("Serial Plotter")),
            )
            .child(
                h_flex()
                    .gap_2()
                    .p_2()
                    .child(self.port_editor.clone())
                    .child(
                        Button::new("reconnect", "Connect").on_click(cx.listener(
                            |this, _, _window, cx| {
                                this.reconnect(cx);
                            },
                        )),
                    ),
            )
            .when_some(self.last_error.clone(), |this, error| {
                this.child(div().p_2().bg(gpui::red()).child(Label::new(error)))
            })
            .child(
                h_flex()
                    .flex_1()
                    .child(
                        canvas(
                            move |bounds, _window, _cx| bounds,
                            move |bounds, _prepaint_bounds, window, _cx| {
                                let all_values: Vec<f32> =
                                    series.iter().flat_map(|s| s.points.iter().copied()).collect();
                                let (min_value, max_value) = min_max(&all_values);
                                for plot_series in &series {
                                    draw_series(plot_series, bounds, min_value, max_value, window);
                                }
                            },
                        )
                        .size_full(),
                    )
                    .child(self.render_legend()),
            )
    }
}

pub fn open_serial_plotter_window(_window: &mut Window, cx: &mut App) {
    if let Some(existing) = cx
        .windows()
        .into_iter()
        .find_map(|handle| handle.downcast::<SerialPlotterWindow>())
    {
        existing
            .update(cx, |_, window, _cx| window.activate_window())
            .log_err();
        return;
    }

    let window_size = Size {
        width: px(640.),
        height: px(420.),
    };
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("Serial Plotter".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            window_bounds: Some(WindowBounds::centered(window_size, cx)),
            is_resizable: true,
            is_minimizable: true,
            kind: WindowKind::Floating,
            // Requesting server decorations (the default) leaves the window
            // completely undecorated -- and thus unmovable, since we never
            // implemented our own drag handling either -- on Wayland
            // compositors that don't support server-side xdg-decorations
            // (e.g. stock GNOME/Mutter). Client decorations plus the
            // title row's own drag handling in `Render` gives a movable
            // window regardless of what the compositor supports, matching
            // this app's own default (`"window_decorations": "client"` in
            // assets/settings/default.json).
            window_decorations: Some(WindowDecorations::Client),
            ..Default::default()
        },
        |window, cx| {
            window.activate_window();
            cx.new(|cx| SerialPlotterWindow::new(window, cx))
        },
    )
    .log_err();
}
