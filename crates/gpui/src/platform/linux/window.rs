use super::BladeRenderer;
use crate::{
    Bounds, GlobalPixels, LinuxDisplay, Pixels, PlatformDisplay, PlatformInputHandler,
    PlatformWindow, Point, Size, WindowAppearance, WindowBounds, WindowOptions, XcbAtoms,
};
use blade_graphics as gpu;
use parking_lot::Mutex;
use raw_window_handle as rwh;
use std::{
    ffi::c_void,
    mem,
    rc::Rc,
    sync::{self, Arc},
};
use xcb::{x, Xid as _};

#[derive(Default)]
struct Callbacks {
    request_frame: Option<Box<dyn FnMut()>>,
    input: Option<Box<dyn FnMut(crate::PlatformInput) -> bool>>,
    active_status_change: Option<Box<dyn FnMut(bool)>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    fullscreen: Option<Box<dyn FnMut(bool)>>,
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
}

struct LinuxWindowInner {
    bounds: Bounds<i32>,
    title_height: i32,
    border_width: i32,
    scale_factor: f32,
    renderer: BladeRenderer,
}

impl LinuxWindowInner {
    fn render_extent(&self) -> gpu::Extent {
        gpu::Extent {
            width: (self.bounds.size.width - 2 * self.border_width) as u32,
            height: (self.bounds.size.height - 2 * self.border_width - self.title_height) as u32,
            depth: 1,
        }
    }
    fn content_size(&self) -> Size<Pixels> {
        let extent = self.render_extent();
        Size {
            width: extent.width.into(),
            height: extent.height.into(),
        }
    }
}

struct RawWindow {
    connection: *mut c_void,
    screen_id: i32,
    window_id: u32,
    visual_id: u32,
}

pub(crate) struct LinuxWindowState {
    xcb_connection: Arc<xcb::Connection>,
    display: Rc<dyn PlatformDisplay>,
    raw: RawWindow,
    x_window: x::Window,
    callbacks: Mutex<Callbacks>,
    inner: Mutex<LinuxWindowInner>,
}

#[derive(Clone)]
pub(crate) struct LinuxWindow(pub(crate) Arc<LinuxWindowState>);

unsafe impl rwh::HasRawWindowHandle for RawWindow {
    fn raw_window_handle(&self) -> rwh::RawWindowHandle {
        let mut wh = rwh::XcbWindowHandle::empty();
        wh.window = self.window_id;
        wh.visual_id = self.visual_id;
        wh.into()
    }
}
unsafe impl rwh::HasRawDisplayHandle for RawWindow {
    fn raw_display_handle(&self) -> rwh::RawDisplayHandle {
        let mut dh = rwh::XcbDisplayHandle::empty();
        dh.connection = self.connection;
        dh.screen = self.screen_id;
        dh.into()
    }
}

impl rwh::HasWindowHandle for LinuxWindow {
    fn window_handle(&self) -> Result<rwh::WindowHandle, rwh::HandleError> {
        Ok(unsafe {
            let raw_handle = rwh::HasRawWindowHandle::raw_window_handle(&self.0.raw);
            rwh::WindowHandle::borrow_raw(raw_handle, rwh::ActiveHandle::new())
        })
    }
}
impl rwh::HasDisplayHandle for LinuxWindow {
    fn display_handle(&self) -> Result<rwh::DisplayHandle, rwh::HandleError> {
        Ok(unsafe {
            let raw_handle = rwh::HasRawDisplayHandle::raw_display_handle(&self.0.raw);
            rwh::DisplayHandle::borrow_raw(raw_handle)
        })
    }
}

impl LinuxWindowState {
    pub fn new(
        options: WindowOptions,
        xcb_connection: &Arc<xcb::Connection>,
        x_main_screen_index: i32,
        x_window: x::Window,
        atoms: &XcbAtoms,
    ) -> Self {
        let x_screen_index = options
            .display_id
            .map_or(x_main_screen_index, |did| did.0 as i32);
        let screen = xcb_connection
            .get_setup()
            .roots()
            .nth(x_screen_index as usize)
            .unwrap();

        let xcb_values = [
            x::Cw::BackPixel(screen.white_pixel()),
            x::Cw::EventMask(
                x::EventMask::EXPOSURE | x::EventMask::STRUCTURE_NOTIFY | x::EventMask::KEY_PRESS,
            ),
        ];

        let bounds = match options.bounds {
            WindowBounds::Fullscreen | WindowBounds::Maximized => Bounds {
                origin: Point::default(),
                size: Size {
                    width: screen.width_in_pixels() as i32,
                    height: screen.height_in_pixels() as i32,
                },
            },
            WindowBounds::Fixed(bounds) => bounds.map(|p| p.0 as i32),
        };
        let border_width = 0i32;

        xcb_connection.send_request(&x::CreateWindow {
            depth: x::COPY_FROM_PARENT as u8,
            wid: x_window,
            parent: screen.root(),
            x: bounds.origin.x as i16,
            y: bounds.origin.y as i16,
            width: bounds.size.width as u16,
            height: bounds.size.height as u16,
            border_width: border_width as u16,
            class: x::WindowClass::InputOutput,
            visual: screen.root_visual(),
            value_list: &xcb_values,
        });

        if let Some(titlebar) = options.titlebar {
            if let Some(title) = titlebar.title {
                xcb_connection.send_request(&x::ChangeProperty {
                    mode: x::PropMode::Replace,
                    window: x_window,
                    property: x::ATOM_WM_NAME,
                    r#type: x::ATOM_STRING,
                    data: title.as_bytes(),
                });
            }
        }
        xcb_connection
            .send_and_check_request(&x::ChangeProperty {
                mode: x::PropMode::Replace,
                window: x_window,
                property: atoms.wm_protocols,
                r#type: x::ATOM_ATOM,
                data: &[atoms.wm_del_window],
            })
            .unwrap();

        xcb_connection.send_request(&x::MapWindow { window: x_window });
        xcb_connection.flush().unwrap();

        let raw = RawWindow {
            connection: as_raw_xcb_connection::AsRawXcbConnection::as_raw_xcb_connection(
                xcb_connection,
            ) as *mut _,
            screen_id: x_screen_index,
            window_id: x_window.resource_id(),
            visual_id: screen.root_visual(),
        };
        let gpu = Arc::new(
            unsafe {
                gpu::Context::init_windowed(
                    &raw,
                    gpu::ContextDesc {
                        validation: cfg!(debug_assertions),
                        capture: false,
                    },
                )
            }
            .unwrap(),
        );

        let gpu_extent = gpu::Extent {
            width: bounds.size.width as u32,
            height: bounds.size.height as u32,
            depth: 1,
        };

        Self {
            xcb_connection: Arc::clone(xcb_connection),
            display: Rc::new(LinuxDisplay::new(xcb_connection, x_screen_index)),
            raw,
            x_window,
            callbacks: Mutex::new(Callbacks::default()),
            inner: Mutex::new(LinuxWindowInner {
                bounds,
                title_height: 0, //TODO
                border_width,
                scale_factor: 1.0,
                renderer: BladeRenderer::new(gpu, gpu_extent),
            }),
        }
    }

    pub fn destroy(&self) {
        self.inner.lock().renderer.destroy();
        self.xcb_connection.send_request(&x::UnmapWindow {
            window: self.x_window,
        });
        self.xcb_connection.send_request(&x::DestroyWindow {
            window: self.x_window,
        });
        if let Some(fun) = self.callbacks.lock().close.take() {
            fun();
        }
    }

    pub fn expose(&self) {
        let mut cb = self.callbacks.lock();
        if let Some(ref mut fun) = cb.request_frame {
            fun();
        }
    }

    pub fn configure(&self, bounds: Bounds<i32>) {
        let mut resize_args = None;
        let do_move;
        {
            let mut inner = self.inner.lock();
            let old_bounds = mem::replace(&mut inner.bounds, bounds);
            do_move = old_bounds.origin != bounds.origin;
            if old_bounds.size != bounds.size {
                let extent = inner.render_extent();
                inner.renderer.resize(extent);
                resize_args = Some((inner.content_size(), inner.scale_factor));
            }
        }

        let mut callbacks = self.callbacks.lock();
        if let Some((content_size, scale_factor)) = resize_args {
            if let Some(ref mut fun) = callbacks.resize {
                fun(content_size, scale_factor)
            }
        }
        if do_move {
            if let Some(ref mut fun) = callbacks.moved {
                fun()
            }
        }
    }
}

impl PlatformWindow for LinuxWindow {
    fn bounds(&self) -> WindowBounds {
        WindowBounds::Fixed(self.0.inner.lock().bounds.map(|v| GlobalPixels(v as f32)))
    }

    fn content_size(&self) -> Size<Pixels> {
        self.0.inner.lock().content_size()
    }

    fn scale_factor(&self) -> f32 {
        self.0.inner.lock().scale_factor
    }

    fn titlebar_height(&self) -> Pixels {
        unimplemented!()
    }

    fn appearance(&self) -> WindowAppearance {
        unimplemented!()
    }

    fn display(&self) -> Rc<dyn PlatformDisplay> {
        Rc::clone(&self.0.display)
    }

    fn mouse_position(&self) -> Point<Pixels> {
        Point::default()
    }

    fn modifiers(&self) -> crate::Modifiers {
        crate::Modifiers::default()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {}

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        None
    }

    fn prompt(
        &self,
        _level: crate::PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[&str],
    ) -> futures::channel::oneshot::Receiver<usize> {
        unimplemented!()
    }

    fn activate(&self) {}

    fn set_title(&mut self, title: &str) {}

    fn set_edited(&mut self, edited: bool) {}

    fn show_character_palette(&self) {
        unimplemented!()
    }

    fn minimize(&self) {
        unimplemented!()
    }

    fn zoom(&self) {
        unimplemented!()
    }

    fn toggle_full_screen(&self) {
        unimplemented!()
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut()>) {
        self.0.callbacks.lock().request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(crate::PlatformInput) -> bool>) {
        self.0.callbacks.lock().input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.callbacks.lock().active_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.callbacks.lock().resize = Some(callback);
    }

    fn on_fullscreen(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.callbacks.lock().fullscreen = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.callbacks.lock().moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.0.callbacks.lock().should_close = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.0.callbacks.lock().close = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.0.callbacks.lock().appearance_changed = Some(callback);
    }

    fn is_topmost_for_position(&self, _position: crate::Point<Pixels>) -> bool {
        unimplemented!()
    }

    fn invalidate(&self) {}

    fn draw(&self, scene: &crate::Scene) {
        let mut inner = self.0.inner.lock();
        inner.renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> sync::Arc<dyn crate::PlatformAtlas> {
        let inner = self.0.inner.lock();
        inner.renderer.atlas().clone()
    }
}
