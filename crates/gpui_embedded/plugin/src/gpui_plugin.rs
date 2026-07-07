//! Guest-side GPUI platform for the "GPUI embedded in GPUI" spike.
//!
//! A plugin runs a real GPUI [`App`] inside a `wasm32-wasip2` component. Each host view slot
//! becomes a GPUI window backed by [`window::PluginWindow`], whose painted scenes are
//! serialized over the `gpui:embedded` WIT protocol instead of being sent to a GPU. See
//! `crates/gpui_embedded/DESIGN.md`.

mod dispatcher;
mod platform;
pub mod shared;
mod text_system;
mod window;

pub(crate) mod wit {
    #![allow(clippy::too_many_arguments)]

    wit_bindgen::generate!({
        path: "../wit",
        world: "plugin",
        skip: ["init-plugin"],
    });
}

use gpui::{
    AnyView, App, AppCell, Application, AssetSource, AsyncApp, Bounds, KeyDownEvent, KeyUpEvent,
    Keystroke, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PlatformInput, Point,
    Render, ScrollDelta, ScrollWheelEvent, SharedString, Window, WindowBounds, WindowOptions, div,
    point, prelude::*, px, size,
};
use platform::PluginPlatform;
use std::cell::RefCell;
use std::rc::Rc;

/// A GPUI plugin. Implement this and call [`register_plugin!`] to make your crate a loadable
/// plugin component.
pub trait Plugin: 'static {
    /// Build the plugin's shared state. Runs once, when the host initializes the component.
    fn new(cx: &mut App) -> Self
    where
        Self: Sized;

    /// Called when the host creates a view slot. Return the root view to render in it.
    fn create_view(&mut self, view_id: u32, window: &mut Window, cx: &mut App) -> AnyView;

    /// Assets (e.g. SVGs) bundled with the plugin, loadable by path from GPUI elements.
    fn assets() -> Option<Box<dyn AssetSource>>
    where
        Self: Sized,
    {
        None
    }
}

/// Registers a [`Plugin`] implementation as this component's entry point.
#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ty) => {
        #[unsafe(export_name = "init-plugin")]
        pub extern "C" fn __init_plugin() {
            $crate::initialize(<$plugin_type as $crate::Plugin>::assets(), |cx| {
                Box::new(<$plugin_type as $crate::Plugin>::new(cx))
            });
        }
    };
}

struct Runtime {
    // Keeps the guest App alive: PluginPlatform::run returns immediately, so unlike native
    // platforms nothing on the stack owns the app after launch.
    _app_cell: Rc<AppCell>,
    async_app: AsyncApp,
    platform: Rc<PluginPlatform>,
    plugin: SharedPlugin,
}

thread_local! {
    static RUNTIME: RefCell<Option<Runtime>> = const { RefCell::new(None) };
}

/// Delegating wrapper because `Box<dyn AssetSource>` itself does not implement the trait.
struct PluginAssets(Box<dyn AssetSource>);

impl AssetSource for PluginAssets {
    fn load(&self, path: &str) -> anyhow::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        self.0.load(path)
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        self.0.list(path)
    }
}

#[doc(hidden)]
pub fn initialize(
    assets: Option<Box<dyn AssetSource>>,
    build_plugin: impl FnOnce(&mut App) -> Box<dyn Plugin> + 'static,
) {
    init_logger();

    let platform = Rc::new(PluginPlatform::new());
    let mut application = Application::with_platform(platform.clone());
    if let Some(assets) = assets {
        application = application.with_assets(PluginAssets(assets));
    }
    let app_cell = application.app_cell();
    let platform_for_runtime = platform.clone();
    application.run(move |cx| {
        let plugin = build_plugin(cx);
        RUNTIME.with(|slot| {
            *slot.borrow_mut() = Some(Runtime {
                _app_cell: app_cell,
                async_app: cx.to_async(),
                platform: platform_for_runtime,
                plugin: Rc::new(RefCell::new(plugin)),
            });
        });
    });
}

type SharedPlugin = Rc<RefCell<Box<dyn Plugin>>>;

fn runtime_handles() -> Option<(AsyncApp, Rc<PluginPlatform>, SharedPlugin)> {
    RUNTIME.with(|slot| {
        slot.borrow().as_ref().map(|runtime| {
            (
                runtime.async_app.clone(),
                runtime.platform.clone(),
                runtime.plugin.clone(),
            )
        })
    })
}

/// Drain the guest scheduler and let dirty windows redraw, then arrange the next wakeup.
///
/// Wakeup requests are suppressed for the duration: everything queued during the pump is
/// drained before it returns, so only the earliest remaining timer needs a host tick.
fn pump(platform: &PluginPlatform) {
    let dispatcher = platform.dispatcher();
    dispatcher.set_wakeups_suppressed(true);
    dispatcher.run_until_idle();
    for window in platform.window_states() {
        window.pump_frame();
    }
    dispatcher.run_until_idle();
    dispatcher.set_wakeups_suppressed(false);
    if let Some(delay) = dispatcher.next_timer_delay() {
        wit::request_tick(delay.as_millis().min(u32::MAX as u128) as u32);
    }
}

/// Wraps a plugin-provided root view so `open_window` has a concrete `Render` type.
struct PluginRoot {
    view: AnyView,
}

impl Render for PluginRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div().size_full().child(self.view.clone())
    }
}

struct Component;

impl wit::Guest for Component {
    fn create_view(view_id: u32, extent: wit::Extent, scale_factor: f32) {
        let Some((async_app, platform, plugin)) = runtime_handles() else {
            log::error!("gpui_plugin: create-view before init-plugin");
            return;
        };
        let view_size = size(px(extent.width), px(extent.height));
        platform.set_pending_view(view_id, view_size, scale_factor);
        let opened = async_app.update(|cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: view_size,
                    })),
                    ..Default::default()
                },
                |window, cx| {
                    let view = plugin.borrow_mut().create_view(view_id, window, cx);
                    cx.new(|_| PluginRoot { view })
                },
            )
        });
        if let Err(error) = opened {
            log::error!("gpui_plugin: opening view {view_id} failed: {error:#}");
        }
        pump(&platform);
    }

    fn resize_view(view_id: u32, extent: wit::Extent, scale_factor: f32) {
        let Some((_, platform, _)) = runtime_handles() else {
            return;
        };
        if let Some(window) = platform.window(view_id) {
            window.resized(size(px(extent.width), px(extent.height)), scale_factor);
        } else {
            log::warn!("gpui_plugin: resize-view for unknown view {view_id}");
        }
        pump(&platform);
    }

    fn handle_mouse(view_id: u32, event: wit::MouseEvent) {
        let Some((_, platform, _)) = runtime_handles() else {
            return;
        };
        if let Some(window) = platform.window(view_id) {
            window.dispatch_input(platform_input_from_wire(event));
        } else {
            log::warn!("gpui_plugin: handle-mouse for unknown view {view_id}");
        }
        pump(&platform);
    }

    fn handle_key(view_id: u32, event: wit::KeyEvent) {
        let Some((_, platform, _)) = runtime_handles() else {
            return;
        };
        if let Some(window) = platform.window(view_id) {
            let input = match event {
                wit::KeyEvent::Down(event) => PlatformInput::KeyDown(KeyDownEvent {
                    keystroke: keystroke_from_wire(event.keystroke),
                    is_held: event.is_held,
                    prefer_character_input: false,
                }),
                wit::KeyEvent::Up(event) => PlatformInput::KeyUp(KeyUpEvent {
                    keystroke: keystroke_from_wire(event.keystroke),
                }),
            };
            window.dispatch_input(input);
        } else {
            log::warn!("gpui_plugin: handle-key for unknown view {view_id}");
        }
        pump(&platform);
    }

    fn tick() {
        let Some((_, platform, _)) = runtime_handles() else {
            return;
        };
        pump(&platform);
    }

    fn shared_entity_announced(announcement: wit::SharedEntityAnnouncement) {
        let Some((_, platform, _)) = runtime_handles() else {
            return;
        };
        shared::entity_announced(announcement);
        pump(&platform);
    }

    fn deliver_shared_message(message: wit::SharedMessage) {
        let Some((mut async_app, platform, _)) = runtime_handles() else {
            return;
        };
        shared::message_delivered(message, &mut async_app);
        pump(&platform);
    }

    fn deliver_shared_snapshot(snapshot: wit::SharedSnapshot) {
        let Some((mut async_app, platform, _)) = runtime_handles() else {
            return;
        };
        shared::snapshot_delivered(snapshot, &mut async_app);
        pump(&platform);
    }
}

wit::export!(Component with_types_in wit);

fn platform_input_from_wire(event: wit::MouseEvent) -> PlatformInput {
    match event {
        wit::MouseEvent::Down(event) => PlatformInput::MouseDown(MouseDownEvent {
            button: button_from_wire(event.button),
            position: point_from_wire(event.position),
            modifiers: modifiers_from_wire(event.modifiers),
            click_count: event.click_count as usize,
            first_mouse: false,
        }),
        wit::MouseEvent::Up(event) => PlatformInput::MouseUp(MouseUpEvent {
            button: button_from_wire(event.button),
            position: point_from_wire(event.position),
            modifiers: modifiers_from_wire(event.modifiers),
            click_count: event.click_count as usize,
        }),
        wit::MouseEvent::Move(event) => PlatformInput::MouseMove(MouseMoveEvent {
            position: point_from_wire(event.position),
            pressed_button: event.pressed_button.map(button_from_wire),
            modifiers: modifiers_from_wire(event.modifiers),
        }),
        wit::MouseEvent::Scroll(event) => PlatformInput::ScrollWheel(ScrollWheelEvent {
            position: point_from_wire(event.position),
            delta: if event.precise {
                ScrollDelta::Pixels(point(px(event.delta_x), px(event.delta_y)))
            } else {
                ScrollDelta::Lines(point(event.delta_x, event.delta_y))
            },
            modifiers: modifiers_from_wire(event.modifiers),
            touch_phase: Default::default(),
        }),
    }
}

fn button_from_wire(button: wit::MouseButton) -> MouseButton {
    match button {
        wit::MouseButton::Left => MouseButton::Left,
        wit::MouseButton::Right => MouseButton::Right,
        wit::MouseButton::Middle => MouseButton::Middle,
    }
}

fn point_from_wire(value: wit::Point) -> Point<gpui::Pixels> {
    point(px(value.x), px(value.y))
}

fn modifiers_from_wire(modifiers: wit::Modifiers) -> gpui::Modifiers {
    gpui::Modifiers {
        control: modifiers.control,
        alt: modifiers.alt,
        shift: modifiers.shift,
        platform: modifiers.platform,
        function: false,
    }
}

fn keystroke_from_wire(keystroke: wit::Keystroke) -> Keystroke {
    Keystroke {
        modifiers: modifiers_from_wire(keystroke.modifiers),
        key: keystroke.key,
        key_char: keystroke.key_char,
    }
}

fn init_logger() {
    struct StderrLogger;

    impl log::Log for StderrLogger {
        fn enabled(&self, _metadata: &log::Metadata) -> bool {
            true
        }

        fn log(&self, record: &log::Record) {
            eprintln!("[plugin {}] {}", record.level(), record.args());
        }

        fn flush(&self) {}
    }

    static LOGGER: StderrLogger = StderrLogger;
    if log::set_logger(&LOGGER).is_ok() {
        log::set_max_level(log::LevelFilter::Info);
    }
}
