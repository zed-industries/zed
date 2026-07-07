//! A demo GPUI plugin: two views (a button and a panel) sharing one entity inside the guest
//! App, rendered by the `gpui_embedded` host. The panel exercises text, SVGs, images,
//! paths, and keyboard input. See `crates/gpui_embedded/DESIGN.md`.

use gpui::{
    AnyView, App, AssetSource, Bounds, Context, ElementInputHandler, Entity, EntityInputHandler,
    FocusHandle, KeyDownEvent, MouseButton, PathBuilder, Pixels, RenderImage, SharedString,
    UTF16Selection, Window, canvas, div, hsla, img, point, prelude::*, px, rgb, svg,
};
use gpui_embedded_shared::demo::{CounterSnapshot, CounterSpec, Increment, TextSnapshot, TextSpec};
use gpui_plugin::shared::{Remote, SharedEntitySource, SharedProjection};
use gpui_plugin::{Plugin, register_plugin};
use std::borrow::Cow;
use std::ops::Range;
use std::sync::Arc;

const STAR_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2l2.9 6.3 6.9.8-5.1 4.7 1.4 6.8L12 17.2 5.9 20.6l1.4-6.8L2.2 9.1l6.9-.8L12 2z"/></svg>"#;

struct PluginAssets;

impl AssetSource for PluginAssets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        if path == "icons/star.svg" {
            Ok(Some(Cow::Borrowed(STAR_SVG.as_bytes())))
        } else {
            Ok(None)
        }
    }

    fn list(&self, _path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(vec!["icons/star.svg".into()])
    }
}

struct ExamplePlugin {
    /// A projection of the click counter homed on the HOST: reads come from snapshots,
    /// writes are `Increment` messages dispatched to the host's handler.
    counter: Remote<CounterSpec>,
}

impl Plugin for ExamplePlugin {
    fn new(cx: &mut App) -> Self {
        Self {
            counter: gpui_plugin::shared::remote::<CounterSpec>("clicks", cx),
        }
    }

    fn create_view(&mut self, view_id: u32, _window: &mut Window, cx: &mut App) -> AnyView {
        match view_id {
            0 => cx
                .new(|cx| ButtonView::new(self.counter.clone(), cx))
                .into(),
            _ => cx
                .new(|cx| PanelView::new(self.counter.clone(), cx))
                .into(),
        }
    }

    fn assets() -> Option<Box<dyn AssetSource>> {
        Some(Box::new(PluginAssets))
    }
}

fn clicks(replica: &Entity<SharedProjection<CounterSnapshot>>, cx: &App) -> u32 {
    replica
        .read(cx)
        .state
        .as_ref()
        .map_or(0, |snapshot| snapshot.clicks)
}

register_plugin!(ExamplePlugin);

struct ButtonView {
    counter: Remote<CounterSpec>,
}

impl ButtonView {
    fn new(counter: Remote<CounterSpec>, cx: &mut Context<Self>) -> Self {
        cx.observe(counter.replica(), |_, _, cx| cx.notify()).detach();
        Self { counter }
    }
}

impl Render for ButtonView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let click_count = clicks(self.counter.replica(), cx);
        let counter = self.counter.clone();
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .id("plugin-button")
            .rounded(px(10.))
            .bg(rgb(0x2d5a88))
            .hover(|style| style.bg(rgb(0x3f76ad)))
            .border_2()
            .border_color(rgb(0x69a2d6))
            .font_family("Helvetica")
            .text_color(gpui::white())
            .text_size(px(15.))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |_, _, _, cx| {
                    let receipt = counter.send(Increment { by: 1 });
                    // Read-your-writes proof: once the receipt resolves, the local replica
                    // is guaranteed to already contain this increment.
                    let counter = counter.clone();
                    cx.spawn(async move |_, cx| {
                        if receipt.await.is_ok() {
                            let observed = cx.update(|cx| clicks(counter.replica(), cx));
                            eprintln!(
                                "[example_plugin] increment acked; replica already shows {observed} clicks"
                            );
                        }
                    })
                    .detach();
                }),
            )
            .child(if click_count == 0 {
                "Click me!".to_string()
            } else {
                format!("Clicked {click_count}x")
            })
    }
}

struct PanelView {
    counter: Remote<CounterSpec>,
    input_line: Entity<InputLine>,
    gradient: Arc<RenderImage>,
    wave_phase: f32,
    _animation: gpui::Task<()>,
}

impl SharedEntitySource<TextSpec> for InputLine {
    fn snapshot(&self, _cx: &App) -> TextSnapshot {
        TextSnapshot {
            text: self.text.clone(),
        }
    }
}

impl PanelView {
    fn new(counter: Remote<CounterSpec>, cx: &mut Context<Self>) -> Self {
        cx.observe(counter.replica(), |_, _, cx| cx.notify()).detach();
        // Drives the wave at ~30fps through the guest's timer path: each await arms a
        // dispatcher timer, which asks the host for a wakeup via `request-tick`.
        let animation = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(33))
                    .await;
                let still_alive = this.update(cx, |this, cx| {
                    this.wave_phase += 0.15;
                    cx.notify();
                });
                if still_alive.is_err() {
                    break;
                }
            }
        });
        let input_line = cx.new(InputLine::new);
        // Homed HERE in the guest: the host projects this entity to mirror the typed text.
        gpui_plugin::shared::share::<TextSpec, _>(&input_line, "typed-text", |_methods| {}, cx);
        Self {
            counter,
            input_line,
            gradient: Arc::new(RenderImage::new(vec![image::Frame::new(
                gradient_bitmap(48, 48),
            )])),
            wave_phase: 0.0,
            _animation: animation,
        }
    }
}

/// A small generated bitmap, stored as premultiplied BGRA like every `RenderImage` frame.
fn gradient_bitmap(width: u32, height: u32) -> image::RgbaImage {
    image::RgbaImage::from_fn(width, height, |x, y| {
        let horizontal = x as f32 / width as f32;
        let vertical = y as f32 / height as f32;
        // Channel order is BGRA.
        image::Rgba([
            (200.0 * (1.0 - horizontal)) as u8,
            (160.0 * vertical) as u8,
            (240.0 * horizontal) as u8,
            255,
        ])
    })
}

impl Render for PanelView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let clicks = clicks(self.counter.replica(), cx);
        let bar_width = px(16. + (clicks as f32 * 14.) % 380.);
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap(px(12.))
            .p(px(16.))
            .rounded(px(12.))
            .bg(rgb(0x1e2227))
            .border_2()
            .border_color(rgb(0x454b54))
            .font_family("Helvetica")
            .text_color(rgb(0xd8dee9))
            .child(
                div()
                    .text_size(px(20.))
                    .text_color(gpui::white())
                    .child("Wasm plugin panel"),
            )
            .child(div().text_size(px(14.)).child(format!(
                "The button view has been clicked {clicks} time{}.",
                if clicks == 1 { "" } else { "s" }
            )))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.))
                    .child(
                        svg()
                            .path("icons/star.svg")
                            .w(px(22.))
                            .h(px(22.))
                            .text_color(hsla(0.13, 0.9, 0.6, 1.0)),
                    )
                    .child(
                        img(self.gradient.clone())
                            .w(px(48.))
                            .h(px(48.))
                            .rounded(px(8.)),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(0x9aa3af))
                            .child("an SVG asset and a generated image"),
                    ),
            )
            .child(self.input_line.clone())
            .child(wave_canvas(self.wave_phase))
            .child(
                div()
                    .h(px(10.))
                    .w(bar_width)
                    .rounded(px(5.))
                    .bg(hsla(0.55, 0.65, 0.55, 1.0)),
            )
    }
}

/// A tessellated path, drawn with GPUI's `PathBuilder` inside the guest and animated by a
/// guest-side timer.
fn wave_canvas(phase: f32) -> impl IntoElement {
    canvas(
        |_bounds, _window, _cx| (),
        move |bounds: Bounds<Pixels>, _prepaint, window: &mut Window, _cx: &mut App| {
            let mut builder = PathBuilder::stroke(px(2.));
            let steps = 60;
            for step in 0..=steps {
                let progress = step as f32 / steps as f32;
                let x = bounds.origin.x + bounds.size.width * progress;
                let y = bounds.origin.y
                    + bounds.size.height * 0.5
                    + px((progress * std::f32::consts::TAU * 2.0 + phase).sin() * 10.0);
                if step == 0 {
                    builder.move_to(point(x, y));
                } else {
                    builder.line_to(point(x, y));
                }
            }
            match builder.build() {
                Ok(path) => window.paint_path(path, hsla(0.85, 0.6, 0.6, 1.0)),
                Err(error) => eprintln!("failed to build wave path: {error:#}"),
            }
        },
    )
    .w_full()
    .h(px(28.))
}

/// A deliberately minimal editable line: enough of `EntityInputHandler` to receive text
/// through the input-handler pipeline (the same path a real editor uses), plus a backspace
/// key binding. Selections, marked text, and cursor movement are out of scope.
struct InputLine {
    focus_handle: FocusHandle,
    text: String,
}

impl InputLine {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            text: String::new(),
        }
    }
}

impl EntityInputHandler for InputLine {
    fn text_for_range(
        &mut self,
        _range: Range<usize>,
        _adjusted_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        None
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let end = self.text.encode_utf16().count();
        Some(UTF16Selection {
            range: end..end,
            reversed: false,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        None
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn replace_text_in_range(
        &mut self,
        _range: Option<Range<usize>>,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.text.push_str(text);
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _range: Option<Range<usize>>,
        new_text: &str,
        _new_selected_range: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.text.push_str(new_text);
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: Range<usize>,
        _element_bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        None
    }

    fn character_index_for_point(
        &mut self,
        _point: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        None
    }
}

impl Render for InputLine {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.focus_handle.is_focused(window);
        let entity = cx.entity();
        let focus_handle = self.focus_handle.clone();
        let shown = if self.text.is_empty() && !focused {
            "click and type\u{2026}".to_string()
        } else if focused {
            format!("{}\u{258f}", self.text)
        } else {
            self.text.clone()
        };
        div()
            .id("input-line")
            .track_focus(&self.focus_handle)
            .relative()
            .w_full()
            .h(px(30.))
            .px(px(8.))
            .flex()
            .items_center()
            .rounded(px(6.))
            .bg(rgb(0x14171b))
            .border_1()
            .border_color(if focused {
                rgb(0x69a2d6)
            } else {
                rgb(0x454b54)
            })
            .text_size(px(13.))
            .text_color(if self.text.is_empty() && !focused {
                rgb(0x6f7883)
            } else {
                rgb(0xe6ebf2)
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    window.focus(&this.focus_handle, cx);
                    cx.notify();
                }),
            )
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                if event.keystroke.key == "backspace" {
                    this.text.pop();
                    cx.notify();
                }
            }))
            .child(shown)
            .child(
                // Registers the input handler each paint while focused, which is what
                // routes host-forwarded printable keys into `replace_text_in_range`.
                canvas(
                    |_bounds, _window, _cx| (),
                    move |bounds: Bounds<Pixels>, _prepaint, window: &mut Window, cx: &mut App| {
                        window.handle_input(
                            &focus_handle,
                            ElementInputHandler::new(bounds, entity.clone()),
                            cx,
                        );
                    },
                )
                .absolute()
                .inset_0(),
            )
    }
}
