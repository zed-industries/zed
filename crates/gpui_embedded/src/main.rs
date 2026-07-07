//! Demo host binary: opens a native GPUI window with two embedded plugin views driven by the
//! `example_plugin` guest component.

use std::path::{Path, PathBuf};

use gpui::{
    App, Application, Bounds, Context, Entity, MouseButton, Pixels, WindowBounds, WindowOptions,
    div, prelude::*, px, rgb, size,
};
use gpui_embedded::{
    HandleShared, HostRemote, PluginHost, PluginInstance, PluginViewState, SharedEntitySource,
};
use gpui_embedded_shared::demo::{CounterSnapshot, CounterSpec, Increment, TextSpec};

/// The home of the shared click counter: a plain host entity. The guest's views project it
/// and send `Increment` messages; native UI reads and mutates it directly.
struct Counter {
    clicks: u32,
}

impl SharedEntitySource<CounterSpec> for Counter {
    fn snapshot(&self, _cx: &App) -> CounterSnapshot {
        CounterSnapshot {
            clicks: self.clicks,
        }
    }
}

impl HandleShared<Increment> for Counter {
    fn handle(&mut self, message: Increment, cx: &mut Context<Self>) {
        self.clicks += message.by;
        cx.notify();
    }
}

fn main() {
    env_logger::init();

    let Some(wasm_path) = resolve_wasm_path() else {
        eprintln!("run crates/gpui_embedded/build_plugin.sh first");
        std::process::exit(1);
    };

    let platform = gpui_platform::current_platform(false);
    let text_system = platform.text_system();

    Application::with_platform(platform).run(move |cx: &mut App| {
        let instance = match PluginInstance::new(&wasm_path, text_system) {
            Ok(instance) => instance,
            Err(error) => {
                log::error!("gpui_embedded: failed to load plugin: {error:#}");
                cx.quit();
                return;
            }
        };

        let bounds = Bounds::centered(None, size(px(900.), px(700.)), cx);
        let opened = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            move |window, cx| {
                let scale = window.scale_factor();
                let counter = cx.new(|_| Counter { clicks: 0 });
                let host = cx.new(|_| PluginHost::new(instance));
                let (view0, view1, typed_text) = host.update(cx, |host, cx| {
                    host.init(cx);
                    host.share(&counter, "clicks", |methods| {
                        methods.on::<Increment>();
                    }, cx);
                    // Homed in the GUEST: the wasm input line's text, projected natively.
                    let typed_text = host.remote::<TextSpec>("typed-text", cx);
                    let view0 = host.create_view(0, size(px(240.), px(100.)), scale, cx);
                    let view1 = host.create_view(1, size(px(480.), px(320.)), scale, cx);
                    (view0, view1, typed_text)
                });
                cx.new(|cx| {
                    cx.observe(&counter, |_, _, cx| cx.notify()).detach();
                    cx.observe(typed_text.replica(), |_, _, cx| cx.notify())
                        .detach();
                    DemoView {
                        _host: host,
                        counter,
                        typed_text,
                        view0,
                        view1,
                    }
                })
            },
        );

        if let Err(error) = opened {
            log::error!("gpui_embedded: failed to open window: {error:#}");
            cx.quit();
            return;
        }

        cx.activate(true);
    });
}

fn resolve_wasm_path() -> Option<PathBuf> {
    if let Some(argument) = std::env::args().nth(1) {
        return Some(PathBuf::from(argument));
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for profile in ["release", "debug"] {
        let candidate = manifest_dir
            .join("example_plugin/target/wasm32-wasip2")
            .join(profile)
            .join("example_plugin.wasm");
        if candidate.exists() {
            return Some(candidate);
        }
    }

    None
}

struct DemoView {
    _host: Entity<PluginHost>,
    counter: Entity<Counter>,
    typed_text: HostRemote<TextSpec>,
    view0: Entity<PluginViewState>,
    view1: Entity<PluginViewState>,
}

impl Render for DemoView {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let clicks = self.counter.read(cx).clicks;
        let typed = self
            .typed_text
            .replica()
            .read(cx)
            .state
            .as_ref()
            .map(|snapshot| snapshot.text.clone())
            .unwrap_or_default();
        let counter = self.counter.clone();
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .child(div().text_xl().child("GPUI embedded in GPUI"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x9aa3af))
                            .child(format!("shared counter (native view): {clicks}")),
                    )
                    .child(
                        div()
                            .id("native-increment")
                            .px_2()
                            .py_1()
                            .rounded(px(6.))
                            .bg(rgb(0x3a3f45))
                            .hover(|style| style.bg(rgb(0x4a5058)))
                            .text_sm()
                            .child("+5 from native")
                            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                counter.update(cx, |counter, cx| {
                                    counter.clicks += 5;
                                    cx.notify();
                                });
                            }),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x9aa3af))
                            .child(format!("wasm says: {typed:?}")),
                    ),
            )
            .child(framed_slot(px(240.), px(100.), self.view0.clone()))
            .child(framed_slot(px(480.), px(320.), self.view1.clone()))
    }
}

fn framed_slot(
    width: Pixels,
    height: Pixels,
    view: Entity<PluginViewState>,
) -> impl IntoElement {
    div()
        .w(width)
        .h(height)
        .border_1()
        .border_color(rgb(0x3c3c3c))
        .child(view)
}
