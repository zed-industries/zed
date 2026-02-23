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
    pub(crate) title: String,
    pub(crate) input_handler: Option<PlatformInputHandler>,
    pub(crate) is_fullscreen: bool,
    pub(crate) is_active: bool,
    pub(crate) mouse_position: Point<Pixels>,
    pub(crate) modifiers: Modifiers,
    pub(crate) capslock: Capslock,
}

pub struct WebWindow {
    canvas: web_sys::HtmlCanvasElement,
    display: Rc<dyn PlatformDisplay>,
    #[allow(dead_code)]
    handle: AnyWindowHandle,
    state: Rc<RefCell<WebWindowMutableState>>,
    callbacks: Rc<RefCell<WebWindowCallbacks>>,
    // These closures are stored to prevent them from being garbage collected.
    // The browser (requestAnimationFrame, ResizeObserver) holds JS references
    // to the functions, but the Rust Closure must stay alive to back them.
    _raf_closure: Closure<dyn FnMut()>,
    _resize_closure: Option<Closure<dyn FnMut(js_sys::Array)>>,
    _event_listeners: WebEventListeners,
}

impl WebWindow {
    pub fn new(
        handle: AnyWindowHandle,
        params: WindowParams,
        context: &WgpuContext,
    ) -> anyhow::Result<Self> {
        let browser_window =
            web_sys::window().ok_or_else(|| anyhow::anyhow!("No global `window` object found"))?;
        let document = browser_window
            .document()
            .ok_or_else(|| anyhow::anyhow!("No `document` found on window"))?;

        let canvas: web_sys::HtmlCanvasElement = document
            .create_element("canvas")
            .map_err(|e| anyhow::anyhow!("Failed to create canvas element: {e:?}"))?
            .dyn_into()
            .map_err(|e| anyhow::anyhow!("Created element is not a canvas: {e:?}"))?;

        let scale_factor = browser_window.device_pixel_ratio() as f32;

        let logical_width = params.bounds.size.width;
        let logical_height = params.bounds.size.height;

        let physical_width = (f32::from(logical_width) * scale_factor) as u32;
        let physical_height = (f32::from(logical_height) * scale_factor) as u32;

        canvas.set_width(physical_width.max(1));
        canvas.set_height(physical_height.max(1));

        canvas.set_tab_index(0);

        let style = canvas.style();
        style
            .set_property("width", &format!("{}px", f32::from(logical_width)))
            .map_err(|e| anyhow::anyhow!("Failed to set canvas width style: {e:?}"))?;
        style
            .set_property("height", &format!("{}px", f32::from(logical_height)))
            .map_err(|e| anyhow::anyhow!("Failed to set canvas height style: {e:?}"))?;
        style
            .set_property("display", "block")
            .map_err(|e| anyhow::anyhow!("Failed to set canvas display style: {e:?}"))?;
        style
            .set_property("outline", "none")
            .map_err(|e| anyhow::anyhow!("Failed to set canvas outline style: {e:?}"))?;

        let body = document
            .body()
            .ok_or_else(|| anyhow::anyhow!("No `body` found on document"))?;
        body.append_child(&canvas)
            .map_err(|e| anyhow::anyhow!("Failed to append canvas to body: {e:?}"))?;

        canvas.focus().ok();

        let device_size = Size {
            width: DevicePixels(physical_width as i32),
            height: DevicePixels(physical_height as i32),
        };

        let renderer_config = WgpuSurfaceConfig {
            size: device_size,
            transparent: false,
        };

        let renderer = WgpuRenderer::new_from_canvas(context, &canvas, renderer_config)?;

        let display: Rc<dyn PlatformDisplay> = Rc::new(WebDisplay::new());

        let mutable_state = WebWindowMutableState {
            renderer,
            bounds: params.bounds,
            scale_factor,
            title: String::new(),
            input_handler: None,
            is_fullscreen: false,
            is_active: true,
            mouse_position: Point::default(),
            modifiers: Modifiers::default(),
            capslock: Capslock::default(),
        };

        let state = Rc::new(RefCell::new(mutable_state));
        let callbacks = Rc::new(RefCell::new(WebWindowCallbacks::default()));

        let raf_closure = Self::create_raf_closure(Rc::clone(&callbacks));
        Self::schedule_raf(&raf_closure);

        let resize_closure =
            Self::create_resize_observer(Rc::clone(&state), Rc::clone(&callbacks), &canvas);

        let event_listeners =
            events::register_event_listeners(&canvas, Rc::clone(&callbacks), Rc::clone(&state));

        Ok(Self {
            canvas,
            display,
            handle,
            state,
            callbacks,
            _raf_closure: raf_closure,
            _resize_closure: resize_closure,
            _event_listeners: event_listeners,
        })
    }

    fn create_raf_closure(callbacks: Rc<RefCell<WebWindowCallbacks>>) -> Closure<dyn FnMut()> {
        // The RAF closure holds a strong Rc to callbacks. This is not a reference
        // cycle because: WebWindow owns the Closure, and if WebWindow is dropped
        // the Closure is dropped, releasing the Rc. The Rc<callbacks> is also held
        // by WebWindow, which means callbacks lives exactly as long as WebWindow.
        //
        // We also need a handle to re-schedule ourselves. We store the Closure in
        // an Rc<RefCell> so the closure can reference its own JS function wrapper.
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
                if let Some(window) = web_sys::window() {
                    window.request_animation_frame(func).ok();
                }
            }
        });

        // Store the JS function so the closure can re-schedule itself
        let js_func: js_sys::Function =
            closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
        *raf_handle.borrow_mut() = Some(js_func);

        closure
    }

    fn schedule_raf(closure: &Closure<dyn FnMut()>) {
        if let Some(window) = web_sys::window() {
            window
                .request_animation_frame(closure.as_ref().unchecked_ref())
                .ok();
        }
    }

    fn create_resize_observer(
        state: Rc<RefCell<WebWindowMutableState>>,
        callbacks: Rc<RefCell<WebWindowCallbacks>>,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> Option<Closure<dyn FnMut(js_sys::Array)>> {
        let canvas_clone = canvas.clone();

        let closure: Closure<dyn FnMut(js_sys::Array)> =
            Closure::new(move |entries: js_sys::Array| {
                let entry = entries.get(0);
                if entry.is_undefined() {
                    return;
                }

                let browser_window = match web_sys::window() {
                    Some(w) => w,
                    None => return,
                };

                let scale_factor = browser_window.device_pixel_ratio() as f32;

                let content_rect: web_sys::DomRect =
                    match js_sys::Reflect::get(&entry, &"contentRect".into())
                        .ok()
                        .and_then(|v| v.dyn_into().ok())
                    {
                        Some(rect) => rect,
                        None => return,
                    };

                let logical_width = content_rect.width() as f32;
                let logical_height = content_rect.height() as f32;
                let physical_width = (logical_width * scale_factor) as u32;
                let physical_height = (logical_height * scale_factor) as u32;

                canvas_clone.set_width(physical_width.max(1));
                canvas_clone.set_height(physical_height.max(1));

                {
                    let mut s = state.borrow_mut();
                    s.bounds.size = Size {
                        width: px(logical_width),
                        height: px(logical_height),
                    };
                    s.scale_factor = scale_factor;
                    s.renderer.update_drawable_size(Size {
                        width: DevicePixels(physical_width as i32),
                        height: DevicePixels(physical_height as i32),
                    });
                }

                let new_size = Size {
                    width: px(logical_width),
                    height: px(logical_height),
                };

                let mut cbs = callbacks.borrow_mut();
                if let Some(ref mut callback) = cbs.resize {
                    callback(new_size, scale_factor);
                }
            });

        // Create ResizeObserver via raw JS interop since web-sys bindings
        // for ResizeObserver require additional feature flags that pull in
        // a large amount of generated code.
        let observer = create_resize_observer_js(closure.as_ref().unchecked_ref())?;
        js_sys::Reflect::apply(
            &js_sys::Reflect::get(&observer, &"observe".into())
                .ok()?
                .dyn_into::<js_sys::Function>()
                .ok()?,
            &observer,
            &js_sys::Array::of1(canvas),
        )
        .ok()?;

        Some(closure)
    }
}

fn create_resize_observer_js(callback: &js_sys::Function) -> Option<JsValue> {
    let global = js_sys::global();
    let constructor = js_sys::Reflect::get(&global, &"ResizeObserver".into()).ok()?;
    if constructor.is_undefined() || constructor.is_null() {
        log::warn!("ResizeObserver not available in this browser");
        return None;
    }
    let constructor: js_sys::Function = constructor.dyn_into().ok()?;
    js_sys::Reflect::construct(&constructor, &js_sys::Array::of1(callback)).ok()
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
        let physical_width = (f32::from(size.width) * scale_factor) as u32;
        let physical_height = (f32::from(size.height) * scale_factor) as u32;

        self.canvas.set_width(physical_width.max(1));
        self.canvas.set_height(physical_height.max(1));

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
        WindowAppearance::Dark
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
        false
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn set_title(&mut self, title: &str) {
        self.state.borrow_mut().title = title.to_owned();
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                document.set_title(title);
            }
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
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    document.exit_fullscreen();
                }
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
        false
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
