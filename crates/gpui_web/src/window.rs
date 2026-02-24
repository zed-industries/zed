use crate::display::WebDisplay;
use crate::events::{self, WebEventListeners};
use std::{cell::RefCell, rc::Rc, sync::Arc};

use gpui::{
    AnyWindowHandle, Bounds, Capslock, Decorations, DevicePixels, DispatchEventResult, GpuSpecs,
    Modifiers, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler,
    PlatformWindow, Point, PromptButton, PromptLevel, RequestFrameOptions, ResizeEdge, Scene, Size,
    WindowAppearance, WindowBackgroundAppearance, WindowBounds, WindowControlArea, WindowControls,
    WindowDecorations, WindowParams, px,
};
use gpui_wgpu::{WgpuContext, WgpuRenderer, WgpuSurfaceConfig};
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

pub struct WebWindow {
    browser_window: web_sys::Window,
    canvas: web_sys::HtmlCanvasElement,
    display: Rc<dyn PlatformDisplay>,
    #[allow(dead_code)]
    handle: AnyWindowHandle,
    state: Rc<RefCell<WebWindowMutableState>>,
    callbacks: Rc<RefCell<WebWindowCallbacks>>,
    // These closures are stored to prevent them from being garbage collected.
    _raf_closure: Closure<dyn FnMut()>,
    _resize_closure: Option<Closure<dyn FnMut(js_sys::Array)>>,
    _event_listeners: WebEventListeners,
    _focus_closure: Closure<dyn FnMut(JsValue)>,
    _blur_closure: Closure<dyn FnMut(JsValue)>,
    _pointer_enter_closure: Closure<dyn FnMut(JsValue)>,
    _pointer_leave_hover_closure: Closure<dyn FnMut(JsValue)>,
    _visibility_change_closure: Closure<dyn FnMut(JsValue)>,
    _appearance_change_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

fn effective_device_pixel_ratio(browser_window: &web_sys::Window) -> f32 {
    let dpr = browser_window.device_pixel_ratio().max(1.0) as f32;
    log::debug!("effective_device_pixel_ratio: dpr={dpr}");
    dpr
}

impl WebWindow {
    pub fn new(
        handle: AnyWindowHandle,
        params: WindowParams,
        context: &WgpuContext,
        browser_window: web_sys::Window,
    ) -> anyhow::Result<Self> {
        let document = browser_window
            .document()
            .ok_or_else(|| anyhow::anyhow!("No `document` found on window"))?;

        let canvas: web_sys::HtmlCanvasElement = document
            .create_element("canvas")
            .map_err(|e| anyhow::anyhow!("Failed to create canvas element: {e:?}"))?
            .dyn_into()
            .map_err(|e| anyhow::anyhow!("Created element is not a canvas: {e:?}"))?;

        let scale_factor = effective_device_pixel_ratio(&browser_window);
        let max_texture_dimension = context.device.limits().max_texture_dimension_2d;

        canvas.set_tab_index(0);

        let style = canvas.style();
        style
            .set_property("width", "100%")
            .map_err(|e| anyhow::anyhow!("Failed to set canvas width style: {e:?}"))?;
        style
            .set_property("height", "100%")
            .map_err(|e| anyhow::anyhow!("Failed to set canvas height style: {e:?}"))?;
        style
            .set_property("display", "block")
            .map_err(|e| anyhow::anyhow!("Failed to set canvas display style: {e:?}"))?;
        style
            .set_property("outline", "none")
            .map_err(|e| anyhow::anyhow!("Failed to set canvas outline style: {e:?}"))?;
        style
            .set_property("touch-action", "none")
            .map_err(|e| anyhow::anyhow!("Failed to set touch-action style: {e:?}"))?;

        let body = document
            .body()
            .ok_or_else(|| anyhow::anyhow!("No `body` found on document"))?;
        body.append_child(&canvas)
            .map_err(|e| anyhow::anyhow!("Failed to append canvas to body: {e:?}"))?;

        canvas.focus().ok();

        // TODO-Wasm: Seems bad
        let device_size = Size {
            width: DevicePixels(1),
            height: DevicePixels(1),
        };

        let renderer_config = WgpuSurfaceConfig {
            size: device_size,
            transparent: false,
        };

        let renderer = WgpuRenderer::new_from_canvas(context, &canvas, renderer_config)?;

        let display: Rc<dyn PlatformDisplay> = Rc::new(WebDisplay::new(browser_window.clone()));

        let initial_bounds = Bounds {
            origin: Point::default(),
            size: Size {
                width: px(1.0),
                height: px(1.0),
            },
        };

        let mutable_state = WebWindowMutableState {
            renderer,
            bounds: initial_bounds,
            scale_factor,
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

        let state = Rc::new(RefCell::new(mutable_state));
        let callbacks = Rc::new(RefCell::new(WebWindowCallbacks::default()));

        let raf_closure = Self::create_raf_closure(Rc::clone(&callbacks), browser_window.clone());
        Self::schedule_raf(&browser_window, &raf_closure);

        let resize_closure = Self::create_resize_observer(
            Rc::clone(&state),
            Rc::clone(&callbacks),
            &canvas,
            browser_window.clone(),
            max_texture_dimension,
        );

        let event_listeners = events::register_event_listeners(
            &canvas,
            Rc::clone(&callbacks),
            Rc::clone(&state),
            &browser_window,
        );

        let focus_closure = Self::create_focus_closure(Rc::clone(&state), Rc::clone(&callbacks));
        canvas
            .add_event_listener_with_callback("focus", focus_closure.as_ref().unchecked_ref())
            .ok();

        let blur_closure = Self::create_blur_closure(Rc::clone(&state), Rc::clone(&callbacks));
        canvas
            .add_event_listener_with_callback("blur", blur_closure.as_ref().unchecked_ref())
            .ok();

        let pointer_enter_closure =
            Self::create_pointer_enter_closure(Rc::clone(&state), Rc::clone(&callbacks));
        canvas
            .add_event_listener_with_callback(
                "pointerenter",
                pointer_enter_closure.as_ref().unchecked_ref(),
            )
            .ok();

        let pointer_leave_hover_closure =
            Self::create_pointer_leave_hover_closure(Rc::clone(&state), Rc::clone(&callbacks));
        canvas
            .add_event_listener_with_callback(
                "pointerleave",
                pointer_leave_hover_closure.as_ref().unchecked_ref(),
            )
            .ok();

        let visibility_change_closure = Self::create_visibility_change_closure(
            Rc::clone(&state),
            Rc::clone(&callbacks),
            browser_window.clone(),
        );
        document
            .add_event_listener_with_callback(
                "visibilitychange",
                visibility_change_closure.as_ref().unchecked_ref(),
            )
            .ok();

        let appearance_change_closure =
            Self::install_appearance_change_listener(Rc::clone(&callbacks), &browser_window);

        Ok(Self {
            browser_window,
            canvas,
            display,
            handle,
            state,
            callbacks,
            _raf_closure: raf_closure,
            _resize_closure: resize_closure,
            _event_listeners: event_listeners,
            _focus_closure: focus_closure,
            _blur_closure: blur_closure,
            _pointer_enter_closure: pointer_enter_closure,
            _pointer_leave_hover_closure: pointer_leave_hover_closure,
            _visibility_change_closure: visibility_change_closure,
            _appearance_change_closure: appearance_change_closure,
        })
    }

    fn create_raf_closure(
        callbacks: Rc<RefCell<WebWindowCallbacks>>,
        browser_window: web_sys::Window,
    ) -> Closure<dyn FnMut()> {
        let raf_handle: Rc<RefCell<Option<js_sys::Function>>> = Rc::new(RefCell::new(None));
        let raf_handle_inner = Rc::clone(&raf_handle);

        let closure = Closure::new(move || {
            {
                let mut cbs = callbacks.borrow_mut();
                if let Some(ref mut callback) = cbs.request_frame {
                    callback(RequestFrameOptions {
                        require_presentation: true,
                        force_render: false,
                    });
                }
            }

            // Re-schedule for the next frame
            if let Some(ref func) = *raf_handle_inner.borrow() {
                browser_window.request_animation_frame(func).ok();
            }
        });

        let js_func: js_sys::Function =
            closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
        *raf_handle.borrow_mut() = Some(js_func);

        closure
    }

    fn schedule_raf(browser_window: &web_sys::Window, closure: &Closure<dyn FnMut()>) {
        browser_window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .ok();
    }

    fn create_resize_observer(
        state: Rc<RefCell<WebWindowMutableState>>,
        callbacks: Rc<RefCell<WebWindowCallbacks>>,
        canvas: &web_sys::HtmlCanvasElement,
        browser_window: web_sys::Window,
        max_texture_dimension: u32,
    ) -> Option<Closure<dyn FnMut(js_sys::Array)>> {
        let canvas_clone = canvas.clone();

        let closure: Closure<dyn FnMut(js_sys::Array)> =
            Closure::new(move |entries: js_sys::Array| {
                let entry: web_sys::ResizeObserverEntry = match entries.get(0).dyn_into().ok() {
                    Some(entry) => entry,
                    None => return,
                };

                let device_pixel_ratio = browser_window.device_pixel_ratio().max(1.0) as f32;

                let content_box = entry.content_box_size();
                let (logical_width, logical_height) = match content_box_size(&content_box) {
                    Some(size) => size,
                    None => return,
                };

                let device_pixel_box = entry.device_pixel_content_box_size();
                let (physical_width, physical_height) =
                    match device_pixel_content_box_size(&device_pixel_box) {
                        Some((dw, dh)) => (dw, dh),
                        None => (
                            (logical_width * device_pixel_ratio).round() as u32,
                            (logical_height * device_pixel_ratio).round() as u32,
                        ),
                    };

                let clamped_width = physical_width.max(1).min(max_texture_dimension);
                let clamped_height = physical_height.max(1).min(max_texture_dimension);

                canvas_clone.set_width(clamped_width);
                canvas_clone.set_height(clamped_height);

                {
                    let mut s = state.borrow_mut();
                    s.bounds.size = Size {
                        width: px(logical_width),
                        height: px(logical_height),
                    };
                    s.scale_factor = device_pixel_ratio;
                    s.renderer.update_drawable_size(Size {
                        width: DevicePixels(clamped_width as i32),
                        height: DevicePixels(clamped_height as i32),
                    });
                }

                let new_size = Size {
                    width: px(logical_width),
                    height: px(logical_height),
                };

                let mut cbs = callbacks.borrow_mut();
                if let Some(ref mut callback) = cbs.resize {
                    callback(new_size, device_pixel_ratio);
                }
            });

        let observer = web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref()).ok()?;

        // Observe content-box so CSS size changes (including zoom) always drive updates.
        let mut observe_options = web_sys::ResizeObserverOptions::new();
        observe_options.set_box(web_sys::ResizeObserverBoxOptions::ContentBox);
        observer.observe_with_options(canvas, &observe_options);

        Some(closure)
    }

    /// Fires `active_status_change(true)` when the canvas gains focus.
    fn create_focus_closure(
        state: Rc<RefCell<WebWindowMutableState>>,
        callbacks: Rc<RefCell<WebWindowCallbacks>>,
    ) -> Closure<dyn FnMut(JsValue)> {
        Closure::new(move |_event: JsValue| {
            {
                let mut s = state.borrow_mut();
                s.is_active = true;
            }
            let mut cbs = callbacks.borrow_mut();
            if let Some(ref mut callback) = cbs.active_status_change {
                callback(true);
            }
        })
    }

    /// Fires `active_status_change(false)` when the canvas loses focus.
    fn create_blur_closure(
        state: Rc<RefCell<WebWindowMutableState>>,
        callbacks: Rc<RefCell<WebWindowCallbacks>>,
    ) -> Closure<dyn FnMut(JsValue)> {
        Closure::new(move |_event: JsValue| {
            {
                let mut s = state.borrow_mut();
                s.is_active = false;
            }
            let mut cbs = callbacks.borrow_mut();
            if let Some(ref mut callback) = cbs.active_status_change {
                callback(false);
            }
        })
    }

    /// Fires `hover_status_change(true)` when the pointer enters the canvas.
    fn create_pointer_enter_closure(
        state: Rc<RefCell<WebWindowMutableState>>,
        callbacks: Rc<RefCell<WebWindowCallbacks>>,
    ) -> Closure<dyn FnMut(JsValue)> {
        Closure::new(move |_event: JsValue| {
            {
                let mut s = state.borrow_mut();
                s.is_hovered = true;
            }
            let mut cbs = callbacks.borrow_mut();
            if let Some(ref mut callback) = cbs.hover_status_change {
                callback(true);
            }
        })
    }

    /// Fires `hover_status_change(false)` when the pointer leaves the canvas.
    fn create_pointer_leave_hover_closure(
        state: Rc<RefCell<WebWindowMutableState>>,
        callbacks: Rc<RefCell<WebWindowCallbacks>>,
    ) -> Closure<dyn FnMut(JsValue)> {
        Closure::new(move |_event: JsValue| {
            {
                let mut s = state.borrow_mut();
                s.is_hovered = false;
            }
            let mut cbs = callbacks.borrow_mut();
            if let Some(ref mut callback) = cbs.hover_status_change {
                callback(false);
            }
        })
    }

    /// Fires `active_status_change` when the tab becomes hidden or visible.
    ///
    /// This covers the case where the user switches to another tab — the canvas
    /// doesn't fire `blur` in that situation, but the document's visibility changes.
    fn create_visibility_change_closure(
        state: Rc<RefCell<WebWindowMutableState>>,
        callbacks: Rc<RefCell<WebWindowCallbacks>>,
        browser_window: web_sys::Window,
    ) -> Closure<dyn FnMut(JsValue)> {
        Closure::new(move |_event: JsValue| {
            let is_visible = browser_window
                .document()
                .map(|doc| {
                    let state_str: String = js_sys::Reflect::get(&doc, &"visibilityState".into())
                        .ok()
                        .and_then(|v| v.as_string())
                        .unwrap_or_default();
                    state_str == "visible"
                })
                .unwrap_or(true);

            {
                let mut s = state.borrow_mut();
                s.is_active = is_visible;
            }
            let mut cbs = callbacks.borrow_mut();
            if let Some(ref mut callback) = cbs.active_status_change {
                callback(is_visible);
            }
        })
    }

    /// Listen for dark/light mode changes via `matchMedia('(prefers-color-scheme: dark)')`.
    ///
    /// Qt uses `matchMedia` change event listeners to detect color scheme transitions
    /// and fires the theme-changed callback.
    fn install_appearance_change_listener(
        callbacks: Rc<RefCell<WebWindowCallbacks>>,
        browser_window: &web_sys::Window,
    ) -> Option<Closure<dyn FnMut(JsValue)>> {
        let mql = browser_window
            .match_media("(prefers-color-scheme: dark)")
            .ok()??;

        let closure = Closure::<dyn FnMut(JsValue)>::new(move |_event: JsValue| {
            let mut cbs = callbacks.borrow_mut();
            if let Some(ref mut callback) = cbs.appearance_changed {
                callback();
            }
        });

        mql.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .ok();

        Some(closure)
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
}

/// Extract the exact device pixel dimensions from a `ResizeObserverEntry`'s
/// `devicePixelContentBoxSize` array returned by `device_pixel_content_box_size()`.
fn device_pixel_content_box_size(box_array: &js_sys::Array) -> Option<(u32, u32)> {
    let box_value: &JsValue = box_array.as_ref();
    if box_value.is_undefined() || box_value.is_null() {
        return None;
    }
    let first = js_sys::Reflect::get_u32(box_value, 0).ok()?;
    if first.is_undefined() {
        return None;
    }
    let inline_size = js_sys::Reflect::get(&first, &"inlineSize".into())
        .ok()?
        .as_f64()? as u32;
    let block_size = js_sys::Reflect::get(&first, &"blockSize".into())
        .ok()?
        .as_f64()? as u32;
    Some((inline_size, block_size))
}

/// Extract CSS pixel dimensions from a `ResizeObserverEntry`'s
/// `contentBoxSize` array returned by `content_box_size()`. This is the modern
/// replacement for `contentRect` and correctly handles writing modes via
/// `inlineSize`/`blockSize`.
fn content_box_size(box_array: &js_sys::Array) -> Option<(f32, f32)> {
    let box_value: &JsValue = box_array.as_ref();
    if box_value.is_undefined() || box_value.is_null() {
        return None;
    }
    let first = js_sys::Reflect::get_u32(box_value, 0).ok()?;
    if first.is_undefined() {
        return None;
    }
    let inline_size = js_sys::Reflect::get(&first, &"inlineSize".into())
        .ok()?
        .as_f64()? as f32;
    let block_size = js_sys::Reflect::get(&first, &"blockSize".into())
        .ok()?
        .as_f64()? as f32;
    Some((inline_size, block_size))
}

impl raw_window_handle::HasWindowHandle for WebWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let canvas_ref: &JsValue = self.canvas.as_ref();
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
        self.state.borrow().bounds
    }

    fn is_maximized(&self) -> bool {
        false
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Windowed(self.bounds())
    }

    fn content_size(&self) -> Size<Pixels> {
        self.state.borrow().bounds.size
    }

    fn resize(&mut self, size: Size<Pixels>) {
        let mut state = self.state.borrow_mut();
        let scale_factor = state.scale_factor;
        let max_texture_dimension = state.max_texture_dimension;
        let physical_width = ((f32::from(size.width) * scale_factor).round() as u32)
            .max(1)
            .min(max_texture_dimension);
        let physical_height = ((f32::from(size.height) * scale_factor).round() as u32)
            .max(1)
            .min(max_texture_dimension);

        self.canvas.set_width(physical_width);
        self.canvas.set_height(physical_height);

        let style = self.canvas.style();
        style
            .set_property("width", &format!("{}px", f32::from(size.width)))
            .ok();
        style
            .set_property("height", &format!("{}px", f32::from(size.height)))
            .ok();

        state.bounds.size = size;
        state.renderer.update_drawable_size(Size {
            width: DevicePixels(physical_width as i32),
            height: DevicePixels(physical_height as i32),
        });
    }

    fn scale_factor(&self) -> f32 {
        self.state.borrow().scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        Self::current_appearance(&self.browser_window)
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.state.borrow().mouse_position
    }

    fn modifiers(&self) -> Modifiers {
        self.state.borrow().modifiers
    }

    fn capslock(&self) -> Capslock {
        self.state.borrow().capslock
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.state.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.state.borrow_mut().input_handler.take()
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
        self.state.borrow_mut().is_active = true;
    }

    fn is_active(&self) -> bool {
        self.state.borrow().is_active
    }

    fn is_hovered(&self) -> bool {
        self.state.borrow().is_hovered
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn set_title(&mut self, title: &str) {
        self.state.borrow_mut().title = title.to_owned();
        if let Some(document) = self.browser_window.document() {
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
        let mut state = self.state.borrow_mut();
        state.is_fullscreen = !state.is_fullscreen;

        if state.is_fullscreen {
            let canvas: &web_sys::Element = self.canvas.as_ref();
            canvas.request_fullscreen().ok();
        } else {
            if let Some(document) = self.browser_window.document() {
                document.exit_fullscreen();
            }
        }
    }

    fn is_fullscreen(&self) -> bool {
        self.state.borrow().is_fullscreen
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.callbacks.borrow_mut().request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.callbacks.borrow_mut().input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.callbacks.borrow_mut().active_status_change = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.callbacks.borrow_mut().hover_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.callbacks.borrow_mut().resize = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.callbacks.borrow_mut().should_close = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.callbacks.borrow_mut().close = Some(callback);
    }

    fn on_hit_test_window_control(&self, callback: Box<dyn FnMut() -> Option<WindowControlArea>>) {
        self.callbacks.borrow_mut().hit_test_window_control = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().appearance_changed = Some(callback);
    }

    fn draw(&self, scene: &Scene) {
        self.state.borrow_mut().renderer.draw(scene);
    }

    fn completed_frame(&self) {
        // On web, presentation happens automatically via wgpu surface present
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.state.borrow().renderer.sprite_atlas().clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        self.state.borrow().renderer.supports_dual_source_blending()
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        Some(self.state.borrow().renderer.gpu_specs())
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
