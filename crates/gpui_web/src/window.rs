use crate::display::WebDisplay;
use crate::events::{ClickState, EventListenerHandle, WebEventListeners, is_mac_platform};
use crate::platform::WebWindowLifecycle;
use std::sync::Arc;
use std::{cell::Cell, cell::RefCell, rc::Rc};

use gpui::{
    AnyWindowHandle, Bounds, Capslock, Decorations, DevicePixels, DispatchEventResult, GpuSpecs,
    Modifiers, MouseButton, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput,
    PlatformInputHandler, PlatformWindow, Point, PromptButton, PromptLevel, RequestFrameOptions,
    ResizeEdge, Scene, Size, WindowAppearance, WindowBackgroundAppearance, WindowBounds,
    WindowControlArea, WindowControls, WindowDecorations, WindowParams, px,
};
use gpui_wgpu::{WgpuContext, WgpuRenderer, WgpuSurfaceConfig, wgpu};
use wasm_bindgen::prelude::*;

#[derive(Default)]
pub(crate) struct WebWindowCallbacks {
    pub(crate) request_frame: Option<Box<dyn FnMut(RequestFrameOptions)>>,
    pub(crate) input: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>,
    pub(crate) active_status_change: Option<Box<dyn FnMut(bool)>>,
    pub(crate) hover_status_change: Option<Box<dyn FnMut(bool)>>,
    pub(crate) resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    pub(crate) moved: Option<Box<dyn FnMut()>>,
    pub(crate) should_close: Option<Box<dyn FnMut() -> bool>>,
    pub(crate) close: Option<Box<dyn FnOnce()>>,
    pub(crate) appearance_changed: Option<Box<dyn FnMut()>>,
    pub(crate) hit_test_window_control: Option<Box<dyn FnMut() -> Option<WindowControlArea>>>,
}

pub(crate) struct WebWindowMutableState {
    pub(crate) renderer: WgpuRenderer,
    pub(crate) bounds: Bounds<Pixels>,
    pub(crate) scale_factor: f32,
    pub(crate) max_texture_dimension: u32,
    pub(crate) title: String,
    pub(crate) input_handler: Option<PlatformInputHandler>,
    pub(crate) is_fullscreen: bool,
    pub(crate) is_active: bool,
    pub(crate) is_hovered: bool,
    pub(crate) mouse_position: Point<Pixels>,
    pub(crate) modifiers: Modifiers,
    pub(crate) capslock: Capslock,
}

pub(crate) struct WebWindowInner {
    pub(crate) browser_window: web_sys::Window,
    pub(crate) canvas: web_sys::HtmlCanvasElement,
    pub(crate) input_element: web_sys::HtmlInputElement,
    pub(crate) has_device_pixel_support: bool,
    pub(crate) is_mac: bool,
    pub(crate) state: RefCell<WebWindowMutableState>,
    pub(crate) callbacks: RefCell<WebWindowCallbacks>,
    pub(crate) click_state: RefCell<ClickState>,
    pub(crate) pressed_button: Cell<Option<MouseButton>>,
    pub(crate) last_physical_size: Cell<(u32, u32)>,
    pub(crate) notify_scale: Cell<bool>,
    pub(crate) is_composing: Cell<bool>,
    mql_handle: RefCell<Option<MqlHandle>>,
    pending_physical_size: Cell<Option<(u32, u32)>>,
    raf_id: Cell<Option<i32>>,
    raf_function: RefCell<Option<js_sys::Function>>,
}

pub struct WebWindow {
    inner: Rc<WebWindowInner>,
    display: Rc<dyn PlatformDisplay>,
    lifecycle: Rc<Cell<WebWindowLifecycle>>,
    active_window: Rc<RefCell<Option<AnyWindowHandle>>>,
    _raf_closure: Closure<dyn FnMut()>,
    _resize_observer: Option<web_sys::ResizeObserver>,
    _resize_observer_closure: Closure<dyn FnMut(js_sys::Array)>,
    _event_listeners: WebEventListeners,
}

impl WebWindow {
    pub(crate) fn prepare_canvas(
        browser_window: &web_sys::Window,
    ) -> anyhow::Result<web_sys::HtmlCanvasElement> {
        let document = browser_window
            .document()
            .ok_or_else(|| anyhow::anyhow!("No `document` found on window"))?;
        let canvas: web_sys::HtmlCanvasElement = document
            .create_element("canvas")
            .map_err(|error| anyhow::anyhow!("Failed to create canvas element: {error:?}"))?
            .dyn_into()
            .map_err(|error| anyhow::anyhow!("Created element is not a canvas: {error:?}"))?;
        canvas.set_tab_index(-1);

        let style = canvas.style();
        for (property, value) in [
            ("width", "100%"),
            ("height", "100%"),
            ("display", "block"),
            ("outline", "none"),
            ("touch-action", "none"),
        ] {
            style.set_property(property, value).map_err(|error| {
                anyhow::anyhow!("Failed to set canvas {property} style: {error:?}")
            })?;
        }

        let body = document
            .body()
            .ok_or_else(|| anyhow::anyhow!("No `body` found on document"))?;
        body.append_child(&canvas)
            .map_err(|error| anyhow::anyhow!("Failed to append canvas to body: {error:?}"))?;
        Ok(canvas)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        _handle: AnyWindowHandle,
        _params: WindowParams,
        context: &WgpuContext,
        canvas: web_sys::HtmlCanvasElement,
        surface: wgpu::Surface<'static>,
        browser_window: web_sys::Window,
        lifecycle: Rc<Cell<WebWindowLifecycle>>,
        active_window: Rc<RefCell<Option<AnyWindowHandle>>>,
    ) -> anyhow::Result<Self> {
        let document = browser_window
            .document()
            .ok_or_else(|| anyhow::anyhow!("No `document` found on window"))?;
        let body = document
            .body()
            .ok_or_else(|| anyhow::anyhow!("No `body` found on document"))?;
        let dpr = browser_window.device_pixel_ratio() as f32;
        let max_texture_dimension = context.device.limits().max_texture_dimension_2d;
        let has_device_pixel_support = check_device_pixel_support();
        let renderer_config = WgpuSurfaceConfig {
            size: Size {
                width: DevicePixels(0),
                height: DevicePixels(0),
            },
            transparent: false,
            preferred_present_mode: None,
        };
        let renderer = WgpuRenderer::new_from_surface(context, surface, renderer_config)?;

        let input_element: web_sys::HtmlInputElement = document
            .create_element("input")
            .map_err(|e| anyhow::anyhow!("Failed to create input element: {e:?}"))?
            .dyn_into()
            .map_err(|e| anyhow::anyhow!("Created element is not an input: {e:?}"))?;
        let input_style = input_element.style();
        input_style.set_property("position", "fixed").ok();
        input_style.set_property("top", "0").ok();
        input_style.set_property("left", "0").ok();
        input_style.set_property("width", "1px").ok();
        input_style.set_property("height", "1px").ok();
        input_style.set_property("opacity", "0").ok();
        body.append_child(&input_element)
            .map_err(|e| anyhow::anyhow!("Failed to append input to body: {e:?}"))?;
        input_element.focus().ok();

        let display: Rc<dyn PlatformDisplay> = Rc::new(WebDisplay::new(browser_window.clone()));

        let initial_bounds = Bounds {
            origin: Point::default(),
            size: Size::default(),
        };

        let mutable_state = WebWindowMutableState {
            renderer,
            bounds: initial_bounds,
            scale_factor: dpr,
            max_texture_dimension,
            title: String::new(),
            input_handler: None,
            is_fullscreen: false,
            is_active: true,
            is_hovered: false,
            mouse_position: Point::default(),
            modifiers: Modifiers::default(),
            capslock: Capslock::default(),
        };

        let is_mac = is_mac_platform(&browser_window);

        let inner = Rc::new(WebWindowInner {
            browser_window,
            canvas,
            input_element,
            has_device_pixel_support,
            is_mac,
            state: RefCell::new(mutable_state),
            callbacks: RefCell::new(WebWindowCallbacks::default()),
            click_state: RefCell::new(ClickState::default()),
            pressed_button: Cell::new(None),
            last_physical_size: Cell::new((0, 0)),
            notify_scale: Cell::new(false),
            is_composing: Cell::new(false),
            mql_handle: RefCell::new(None),
            pending_physical_size: Cell::new(None),
            raf_id: Cell::new(None),
            raf_function: RefCell::new(None),
        });

        let raf_closure = inner.create_raf_closure();
        inner.wake_frame_loop();

        let resize_observer_closure = Self::create_resize_observer_closure(Rc::clone(&inner));
        let resize_observer =
            web_sys::ResizeObserver::new(resize_observer_closure.as_ref().unchecked_ref()).ok();

        if let Some(ref observer) = resize_observer {
            inner.observe_canvas(observer);
            inner.watch_dpr_changes(observer);
        }

        let event_listeners = inner.register_event_listeners();

        Ok(Self {
            inner,
            display,
            lifecycle,
            active_window,
            _raf_closure: raf_closure,
            _resize_observer: resize_observer,
            _resize_observer_closure: resize_observer_closure,
            _event_listeners: event_listeners,
        })
    }

    fn create_resize_observer_closure(
        inner: Rc<WebWindowInner>,
    ) -> Closure<dyn FnMut(js_sys::Array)> {
        Closure::new(move |entries: js_sys::Array| {
            let entry: web_sys::ResizeObserverEntry = match entries.get(0).dyn_into().ok() {
                Some(entry) => entry,
                None => return,
            };

            let dpr = inner.browser_window.device_pixel_ratio();
            let dpr_f32 = dpr as f32;

            let (physical_width, physical_height, logical_width, logical_height) =
                if inner.has_device_pixel_support {
                    let size: web_sys::ResizeObserverSize = entry
                        .device_pixel_content_box_size()
                        .get(0)
                        .unchecked_into();
                    let pw = size.inline_size() as u32;
                    let ph = size.block_size() as u32;
                    let lw = pw as f64 / dpr;
                    let lh = ph as f64 / dpr;
                    (pw, ph, lw as f32, lh as f32)
                } else {
                    // Safari fallback: use contentRect (always CSS px).
                    let rect = entry.content_rect();
                    let lw = rect.width() as f32;
                    let lh = rect.height() as f32;
                    let pw = (lw as f64 * dpr).round() as u32;
                    let ph = (lh as f64 * dpr).round() as u32;
                    (pw, ph, lw, lh)
                };

            let scale_changed = inner.notify_scale.replace(false);
            let prev = inner.last_physical_size.get();
            let size_changed = prev != (physical_width, physical_height);

            if !scale_changed && !size_changed {
                return;
            }
            inner
                .last_physical_size
                .set((physical_width, physical_height));

            // Skip rendering to a zero-size canvas (e.g. display:none).
            if physical_width == 0 || physical_height == 0 {
                {
                    let mut s = inner.state.borrow_mut();
                    s.bounds.size = Size::default();
                    s.scale_factor = dpr_f32;
                }
                // Still fire the callback so GPUI knows the window is gone.
                inner.with_callback(
                    |callbacks| &mut callbacks.resize,
                    |callback| callback(Size::default(), dpr_f32),
                );
                return;
            }

            let max_texture_dimension = inner.state.borrow().max_texture_dimension;
            let clamped_width = physical_width.min(max_texture_dimension);
            let clamped_height = physical_height.min(max_texture_dimension);

            // Recompute the logical size from the clamped physical size so
            // that scale_factor still maps GPUI's logical bounds exactly onto
            // the surface; otherwise clamping would silently distort the
            // effective scale.
            let (logical_width, logical_height) =
                if (clamped_width, clamped_height) != (physical_width, physical_height) {
                    (
                        (clamped_width as f64 / dpr) as f32,
                        (clamped_height as f64 / dpr) as f32,
                    )
                } else {
                    (logical_width, logical_height)
                };

            inner
                .pending_physical_size
                .set(Some((clamped_width, clamped_height)));

            {
                let mut s = inner.state.borrow_mut();
                s.bounds.size = Size {
                    width: px(logical_width),
                    height: px(logical_height),
                };
                s.scale_factor = dpr_f32;
            }

            let new_size = Size {
                width: px(logical_width),
                height: px(logical_height),
            };

            inner.with_callback(
                |callbacks| &mut callbacks.resize,
                |callback| callback(new_size, dpr_f32),
            );
        })
    }
}

impl WebWindowInner {
    /// Invokes a registered callback with take/call/restore semantics.
    ///
    /// The callback is removed from the slot for the duration of the call, so
    /// the `RefCell` is not borrowed while user code runs: a callback that
    /// re-enters the platform window (dispatching input, registering
    /// handlers) would otherwise panic with a `BorrowMutError`. A re-entrant
    /// invocation of the same callback finds the slot empty and is a no-op.
    pub(crate) fn with_callback<C, R>(
        &self,
        select: impl Fn(&mut WebWindowCallbacks) -> &mut Option<C>,
        invoke: impl FnOnce(&mut C) -> R,
    ) -> Option<R> {
        let mut callback = select(&mut self.callbacks.borrow_mut()).take()?;
        let result = invoke(&mut callback);
        *select(&mut self.callbacks.borrow_mut()) = Some(callback);
        Some(result)
    }

    fn create_raf_closure(self: &Rc<Self>) -> Closure<dyn FnMut()> {
        let this = Rc::clone(self);
        let closure = Closure::new(move || {
            // The request that fired is no longer pending; clear it before
            // running the frame so wakeups issued while the frame executes
            // (e.g. views invalidated during draw) schedule the next request
            // instead of being swallowed.
            this.raf_id.set(None);
            this.with_callback(
                |callbacks| &mut callbacks.request_frame,
                |callback| {
                    callback(RequestFrameOptions {
                        require_presentation: false,
                        force_render: false,
                    })
                },
            );
        });

        let js_func: js_sys::Function =
            closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
        *self.raf_function.borrow_mut() = Some(js_func);

        closure
    }

    pub(crate) fn wake_frame_loop(&self) {
        if self.raf_id.get().is_some() {
            return;
        }
        let raf_function = self.raf_function.borrow();
        if let Some(func) = raf_function.as_ref() {
            self.raf_id
                .set(self.browser_window.request_animation_frame(func).ok());
        }
    }

    fn observe_canvas(&self, observer: &web_sys::ResizeObserver) {
        observer.unobserve(&self.canvas);
        if self.has_device_pixel_support {
            let options = web_sys::ResizeObserverOptions::new();
            options.set_box(web_sys::ResizeObserverBoxOptions::DevicePixelContentBox);
            observer.observe_with_options(&self.canvas, &options);
        } else {
            observer.observe(&self.canvas);
        }
    }

    fn watch_dpr_changes(self: &Rc<Self>, observer: &web_sys::ResizeObserver) {
        let current_dpr = self.browser_window.device_pixel_ratio();
        let media_query =
            format!("(resolution: {current_dpr}dppx), (-webkit-device-pixel-ratio: {current_dpr})");
        let Some(mql) = self.browser_window.match_media(&media_query).ok().flatten() else {
            return;
        };

        let this = Rc::clone(self);
        let observer = observer.clone();

        let closure = Closure::<dyn FnMut(JsValue)>::new(move |_event: JsValue| {
            this.notify_scale.set(true);
            this.observe_canvas(&observer);
            this.watch_dpr_changes(&observer);
        });

        mql.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .ok();

        *self.mql_handle.borrow_mut() = Some(MqlHandle {
            mql,
            _closure: closure,
        });
    }

    pub(crate) fn register_visibility_change(self: &Rc<Self>) -> Option<EventListenerHandle> {
        let document = self.browser_window.document()?;
        let this = Rc::clone(self);

        Some(EventListenerHandle::add(
            document.as_ref(),
            "visibilitychange",
            move |_event: JsValue| {
                let is_visible = this
                    .browser_window
                    .document()
                    .map(|doc| {
                        let state_str: String =
                            js_sys::Reflect::get(&doc, &"visibilityState".into())
                                .ok()
                                .and_then(|v| v.as_string())
                                .unwrap_or_default();
                        state_str == "visible"
                    })
                    .unwrap_or(true);

                {
                    let mut state = this.state.borrow_mut();
                    state.is_active = is_visible;
                }
                this.with_callback(
                    |callbacks| &mut callbacks.active_status_change,
                    |callback| callback(is_visible),
                );
            },
        ))
    }

    /// Tracks `fullscreenchange` instead of toggling a local flag: the user
    /// can exit fullscreen with Esc, and `requestFullscreen` can be rejected,
    /// so the document is the only reliable source of truth.
    pub(crate) fn register_fullscreen_change(self: &Rc<Self>) -> Option<EventListenerHandle> {
        let document = self.browser_window.document()?;
        let this = Rc::clone(self);

        Some(EventListenerHandle::add(
            document.as_ref(),
            "fullscreenchange",
            move |_event: JsValue| {
                let is_fullscreen = this
                    .browser_window
                    .document()
                    .is_some_and(|document| document.fullscreen_element().is_some());
                this.state.borrow_mut().is_fullscreen = is_fullscreen;
            },
        ))
    }

    pub(crate) fn with_input_handler<R>(
        &self,
        f: impl FnOnce(&mut PlatformInputHandler) -> R,
    ) -> Option<R> {
        let mut handler = self.state.borrow_mut().input_handler.take()?;
        let result = f(&mut handler);
        self.state.borrow_mut().input_handler = Some(handler);
        Some(result)
    }

    pub(crate) fn register_appearance_change(self: &Rc<Self>) -> Option<EventListenerHandle> {
        let mql = self
            .browser_window
            .match_media("(prefers-color-scheme: dark)")
            .ok()??;

        let this = Rc::clone(self);
        Some(EventListenerHandle::add(
            mql.as_ref(),
            "change",
            move |_event: JsValue| {
                this.with_callback(
                    |callbacks| &mut callbacks.appearance_changed,
                    |callback| callback(),
                );
            },
        ))
    }
}

impl Drop for WebWindow {
    fn drop(&mut self) {
        // Cancel the pending requestAnimationFrame callback before
        // `_raf_closure` is freed, and disconnect the resize observer before
        // `_resize_observer_closure` is freed; a late invocation of either
        // would throw "closure invoked after being dropped".
        if let Some(raf_id) = self.inner.raf_id.take() {
            self.inner
                .browser_window
                .cancel_animation_frame(raf_id)
                .ok();
        }
        // A frame waker that outlives this window must not re-schedule the
        // freed closure; without a stored function, `wake_frame_loop` no-ops.
        self.inner.raf_function.borrow_mut().take();
        if let Some(ref observer) = self._resize_observer {
            observer.disconnect();
        }

        // The DPR media-query closure captures an `Rc<WebWindowInner>` and is
        // stored inside the inner itself, forming a reference cycle; take it
        // out so the inner can actually be freed.
        self.inner.mql_handle.borrow_mut().take();

        let canvas: &web_sys::Element = self.inner.canvas.as_ref();
        canvas.remove();
        let input_element: &web_sys::Element = self.inner.input_element.as_ref();
        input_element.remove();
        self.active_window.borrow_mut().take();
        self.lifecycle.set(WebWindowLifecycle::Closed);
    }
}

fn current_appearance(browser_window: &web_sys::Window) -> WindowAppearance {
    let is_dark = browser_window
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
        .map(|mql| mql.matches())
        .unwrap_or(false);

    if is_dark {
        WindowAppearance::Dark
    } else {
        WindowAppearance::Light
    }
}

struct MqlHandle {
    mql: web_sys::MediaQueryList,
    _closure: Closure<dyn FnMut(JsValue)>,
}

impl Drop for MqlHandle {
    fn drop(&mut self) {
        self.mql
            .remove_event_listener_with_callback("change", self._closure.as_ref().unchecked_ref())
            .ok();
    }
}

// Safari does not support `devicePixelContentBoxSize`, so detect whether it's available.
fn check_device_pixel_support() -> bool {
    let global: JsValue = js_sys::global().into();
    let Ok(constructor) = js_sys::Reflect::get(&global, &"ResizeObserverEntry".into()) else {
        return false;
    };
    let Ok(prototype) = js_sys::Reflect::get(&constructor, &"prototype".into()) else {
        return false;
    };
    let descriptor = js_sys::Object::get_own_property_descriptor(
        &prototype.unchecked_into::<js_sys::Object>(),
        &"devicePixelContentBoxSize".into(),
    );
    !descriptor.is_undefined()
}

impl raw_window_handle::HasWindowHandle for WebWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let canvas_ref: &JsValue = self.inner.canvas.as_ref();
        let obj = std::ptr::NonNull::from(canvas_ref).cast::<std::ffi::c_void>();
        let handle = raw_window_handle::WebCanvasWindowHandle::new(obj);
        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(handle.into()) })
    }
}

impl raw_window_handle::HasDisplayHandle for WebWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Ok(raw_window_handle::DisplayHandle::web())
    }
}

impl PlatformWindow for WebWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.inner.state.borrow().bounds
    }

    fn is_maximized(&self) -> bool {
        false
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Windowed(self.bounds())
    }

    fn content_size(&self) -> Size<Pixels> {
        self.inner.state.borrow().bounds.size
    }

    fn resize(&mut self, size: Size<Pixels>) {
        let style = self.inner.canvas.style();
        style
            .set_property("width", &format!("{}px", f32::from(size.width)))
            .ok();
        style
            .set_property("height", &format!("{}px", f32::from(size.height)))
            .ok();
    }

    fn scale_factor(&self) -> f32 {
        self.inner.state.borrow().scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        current_appearance(&self.inner.browser_window)
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.inner.state.borrow().mouse_position
    }

    fn modifiers(&self) -> Modifiers {
        self.inner.state.borrow().modifiers
    }

    fn capslock(&self) -> Capslock {
        self.inner.state.borrow().capslock
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.inner.state.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.inner.state.borrow_mut().input_handler.take()
    }

    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[PromptButton],
    ) -> Option<futures::channel::oneshot::Receiver<usize>> {
        None
    }

    fn activate(&self) {
        self.inner.state.borrow_mut().is_active = true;
    }

    fn is_active(&self) -> bool {
        self.inner.state.borrow().is_active
    }

    fn is_hovered(&self) -> bool {
        self.inner.state.borrow().is_hovered
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn set_title(&mut self, title: &str) {
        self.inner.state.borrow_mut().title = title.to_owned();
        if let Some(document) = self.inner.browser_window.document() {
            document.set_title(title);
        }
    }

    fn set_background_appearance(&self, _background: WindowBackgroundAppearance) {}

    fn minimize(&self) {
        log::warn!("WebWindow::minimize is not supported in the browser");
    }

    fn zoom(&self) {
        log::warn!("WebWindow::zoom is not supported in the browser");
    }

    fn toggle_fullscreen(&self) {
        let Some(document) = self.inner.browser_window.document() else {
            return;
        };

        // `is_fullscreen` is updated by the `fullscreenchange` listener once
        // the transition actually happens (or not, if the request fails).
        if document.fullscreen_element().is_some() {
            document.exit_fullscreen();
        } else {
            let canvas: &web_sys::Element = self.inner.canvas.as_ref();
            canvas.request_fullscreen().ok();
        }
    }

    fn is_fullscreen(&self) -> bool {
        self.inner.state.borrow().is_fullscreen
    }

    fn frame_waker(&self) -> Option<Rc<dyn Fn()>> {
        // Hold the inner window weakly: the waker is stored in the window's
        // invalidator, and `callbacks.request_frame` captures a clone of that
        // invalidator, so a strong reference here would form a cycle and leak
        // the window on close.
        let inner = Rc::downgrade(&self.inner);
        Some(Rc::new(move || {
            if let Some(inner) = inner.upgrade() {
                inner.wake_frame_loop();
            }
        }))
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.inner.callbacks.borrow_mut().request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.inner.callbacks.borrow_mut().input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.inner.callbacks.borrow_mut().active_status_change = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.inner.callbacks.borrow_mut().hover_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.inner.callbacks.borrow_mut().resize = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.inner.callbacks.borrow_mut().should_close = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.inner.callbacks.borrow_mut().close = Some(callback);
    }

    fn on_hit_test_window_control(&self, callback: Box<dyn FnMut() -> Option<WindowControlArea>>) {
        self.inner.callbacks.borrow_mut().hit_test_window_control = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().appearance_changed = Some(callback);
    }

    fn draw(&self, scene: &Scene) {
        if let Some((width, height)) = self.inner.pending_physical_size.take() {
            if self.inner.canvas.width() != width || self.inner.canvas.height() != height {
                self.inner.canvas.set_width(width);
                self.inner.canvas.set_height(height);
            }

            let mut state = self.inner.state.borrow_mut();
            state.renderer.update_drawable_size(Size {
                width: DevicePixels(width as i32),
                height: DevicePixels(height as i32),
            });
            drop(state);
        }

        self.inner.state.borrow_mut().renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.inner.state.borrow().renderer.sprite_atlas().clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        self.inner
            .state
            .borrow()
            .renderer
            .supports_dual_source_blending()
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        Some(self.inner.state.borrow().renderer.gpu_specs())
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}

    fn request_decorations(&self, _decorations: WindowDecorations) {}

    fn show_window_menu(&self, _position: Point<Pixels>) {}

    fn start_window_move(&self) {}

    fn start_window_resize(&self, _edge: ResizeEdge) {}

    fn window_decorations(&self) -> Decorations {
        Decorations::Server
    }

    fn set_app_id(&mut self, _app_id: &str) {}

    fn window_controls(&self) -> WindowControls {
        WindowControls {
            fullscreen: true,
            maximize: false,
            minimize: false,
            window_menu: false,
        }
    }

    fn set_client_inset(&self, _inset: Pixels) {}
}
