//todo!(linux): remove
#![allow(unused)]

use crate::{
    platform::blade::BladeRenderer, size, Bounds, GlobalPixels, Modifiers, Pixels, PlatformAtlas,
    PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow, Point, PromptLevel,
    Scene, Size, WindowAppearance, WindowBounds, WindowOptions,
};
use blade_graphics as gpu;
use parking_lot::Mutex;
use raw_window_handle as rwh;

use xcb::{
    x::{self, StackMode},
    Xid as _,
};

use std::{
    cell::RefCell,
    ffi::c_void,
    mem,
    num::NonZeroU32,
    ptr::NonNull,
    rc::Rc,
    sync::{self, Arc},
};

use super::X11Display;

#[derive(Default)]
struct Callbacks {
    request_frame: Option<Box<dyn FnMut()>>,
    input: Option<Box<dyn FnMut(PlatformInput) -> bool>>,
    active_status_change: Option<Box<dyn FnMut(bool)>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    fullscreen: Option<Box<dyn FnMut(bool)>>,
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
}

xcb::atoms_struct! {
    #[derive(Debug)]
    pub(crate) struct XcbAtoms {
        pub wm_protocols    => b"WM_PROTOCOLS",
        pub wm_del_window   => b"WM_DELETE_WINDOW",
        wm_state        => b"_NET_WM_STATE",
        wm_state_maxv   => b"_NET_WM_STATE_MAXIMIZED_VERT",
        wm_state_maxh   => b"_NET_WM_STATE_MAXIMIZED_HORZ",
    }
}

struct LinuxWindowInner {
    bounds: Bounds<i32>,
    scale_factor: f32,
    renderer: BladeRenderer,
    input_handler: Option<PlatformInputHandler>,
}

impl LinuxWindowInner {
    fn content_size(&self) -> Size<Pixels> {
        let size = self.renderer.viewport_size();
        Size {
            width: size.width.into(),
            height: size.height.into(),
        }
    }
}

fn query_render_extent(xcb_connection: &xcb::Connection, x_window: x::Window) -> gpu::Extent {
    let cookie = xcb_connection.send_request(&x::GetGeometry {
        drawable: x::Drawable::Window(x_window),
    });
    let reply = xcb_connection.wait_for_reply(cookie).unwrap();
    gpu::Extent {
        width: reply.width() as u32,
        height: reply.height() as u32,
        depth: 1,
    }
}

struct RawWindow {
    connection: *mut c_void,
    screen_id: i32,
    window_id: u32,
    visual_id: u32,
}

pub(crate) struct X11WindowState {
    xcb_connection: Rc<xcb::Connection>,
    display: Rc<dyn PlatformDisplay>,
    raw: RawWindow,
    x_window: x::Window,
    callbacks: RefCell<Callbacks>,
    inner: RefCell<LinuxWindowInner>,
}

#[derive(Clone)]
pub(crate) struct X11Window(pub(crate) Rc<X11WindowState>);

//todo!(linux): Remove other RawWindowHandle implementation
unsafe impl blade_rwh::HasRawWindowHandle for RawWindow {
    fn raw_window_handle(&self) -> blade_rwh::RawWindowHandle {
        let mut wh = blade_rwh::XcbWindowHandle::empty();
        wh.window = self.window_id;
        wh.visual_id = self.visual_id;
        wh.into()
    }
}
unsafe impl blade_rwh::HasRawDisplayHandle for RawWindow {
    fn raw_display_handle(&self) -> blade_rwh::RawDisplayHandle {
        let mut dh = blade_rwh::XcbDisplayHandle::empty();
        dh.connection = self.connection;
        dh.screen = self.screen_id;
        dh.into()
    }
}

impl rwh::HasWindowHandle for X11Window {
    fn window_handle(&self) -> Result<rwh::WindowHandle, rwh::HandleError> {
        Ok(unsafe {
            let non_zero = NonZeroU32::new(self.0.raw.window_id).unwrap();
            let handle = rwh::XcbWindowHandle::new(non_zero);
            rwh::WindowHandle::borrow_raw(handle.into())
        })
    }
}
impl rwh::HasDisplayHandle for X11Window {
    fn display_handle(&self) -> Result<rwh::DisplayHandle, rwh::HandleError> {
        Ok(unsafe {
            let non_zero = NonNull::new(self.0.raw.connection).unwrap();
            let handle = rwh::XcbDisplayHandle::new(Some(non_zero), self.0.raw.screen_id);
            rwh::DisplayHandle::borrow_raw(handle.into())
        })
    }
}

impl X11WindowState {
    pub fn new(
        options: WindowOptions,
        xcb_connection: &Rc<xcb::Connection>,
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
                x::EventMask::EXPOSURE
                    | x::EventMask::STRUCTURE_NOTIFY
                    | x::EventMask::ENTER_WINDOW
                    | x::EventMask::LEAVE_WINDOW
                    | x::EventMask::FOCUS_CHANGE
                    | x::EventMask::KEY_PRESS
                    | x::EventMask::KEY_RELEASE
                    | x::EventMask::BUTTON_PRESS
                    | x::EventMask::BUTTON_RELEASE
                    | x::EventMask::POINTER_MOTION
                    | x::EventMask::BUTTON1_MOTION
                    | x::EventMask::BUTTON2_MOTION
                    | x::EventMask::BUTTON3_MOTION
                    | x::EventMask::BUTTON4_MOTION
                    | x::EventMask::BUTTON5_MOTION
                    | x::EventMask::BUTTON_MOTION,
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

        xcb_connection.send_request(&x::CreateWindow {
            depth: x::COPY_FROM_PARENT as u8,
            wid: x_window,
            parent: screen.root(),
            x: bounds.origin.x as i16,
            y: bounds.origin.y as i16,
            width: bounds.size.width as u16,
            height: bounds.size.height as u16,
            border_width: 0,
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
        xcb_connection.send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: x_window,
            property: atoms.wm_protocols,
            r#type: x::ATOM_ATOM,
            data: &[atoms.wm_del_window],
        });

        let fake_id = xcb_connection.generate_id();
        xcb_connection.send_request(&xcb::present::SelectInput {
            eid: fake_id,
            window: x_window,
            //Note: also consider `IDLE_NOTIFY`
            event_mask: xcb::present::EventMask::COMPLETE_NOTIFY,
        });

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
                        validation: false,
                        capture: false,
                    },
                )
            }
            .unwrap(),
        );

        // Note: this has to be done after the GPU init, or otherwise
        // the sizes are immediately invalidated.
        let gpu_extent = query_render_extent(xcb_connection, x_window);

        Self {
            xcb_connection: xcb_connection.clone(),
            display: Rc::new(X11Display::new(xcb_connection, x_screen_index)),
            raw,
            x_window,
            callbacks: RefCell::new(Callbacks::default()),
            inner: RefCell::new(LinuxWindowInner {
                bounds,
                scale_factor: 1.0,
                renderer: BladeRenderer::new(gpu, gpu_extent),
                input_handler: None,
            }),
        }
    }

    pub fn destroy(&self) {
        self.inner.borrow_mut().renderer.destroy();
        self.xcb_connection.send_request(&x::UnmapWindow {
            window: self.x_window,
        });
        self.xcb_connection.send_request(&x::DestroyWindow {
            window: self.x_window,
        });
        if let Some(fun) = self.callbacks.borrow_mut().close.take() {
            fun();
        }
        self.xcb_connection.flush().unwrap();
    }

    pub fn refresh(&self) {
        let mut cb = self.callbacks.borrow_mut();
        if let Some(mut fun) = cb.request_frame.take() {
            drop(cb);
            fun();
            let mut cb = self.callbacks.borrow_mut();
            cb.request_frame = Some(fun);
        }
    }

    pub fn configure(&self, bounds: Bounds<i32>) {
        let mut resize_args = None;
        let do_move;
        {
            let mut inner = self.inner.borrow_mut();
            let old_bounds = mem::replace(&mut inner.bounds, bounds);
            do_move = old_bounds.origin != bounds.origin;
            //todo!(linux): use normal GPUI types here, refactor out the double
            // viewport check and extra casts ( )
            let gpu_size = query_render_extent(&self.xcb_connection, self.x_window);
            if inner.renderer.viewport_size() != gpu_size {
                inner
                    .renderer
                    .update_drawable_size(size(gpu_size.width as f64, gpu_size.height as f64));
                resize_args = Some((inner.content_size(), inner.scale_factor));
            }
        }

        let mut callbacks = self.callbacks.borrow_mut();
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

    pub fn request_refresh(&self) {
        self.xcb_connection.send_request(&xcb::present::NotifyMsc {
            window: self.x_window,
            serial: 0,
            target_msc: 0,
            divisor: 1,
            remainder: 0,
        });
    }

    pub fn handle_input(&self, input: PlatformInput) {
        if let Some(ref mut fun) = self.callbacks.borrow_mut().input {
            if fun(input.clone()) {
                return;
            }
        }
        if let PlatformInput::KeyDown(event) = input {
            let mut inner = self.inner.borrow_mut();
            if let Some(ref mut input_handler) = inner.input_handler {
                if let Some(ime_key) = &event.keystroke.ime_key {
                    input_handler.replace_text_in_range(None, ime_key);
                }
            }
        }
    }

    pub fn set_focused(&self, focus: bool) {
        if let Some(ref mut fun) = self.callbacks.borrow_mut().active_status_change {
            fun(focus);
        }
    }
}

impl PlatformWindow for X11Window {
    fn bounds(&self) -> WindowBounds {
        WindowBounds::Fixed(
            self.0
                .inner
                .borrow_mut()
                .bounds
                .map(|v| GlobalPixels(v as f32)),
        )
    }

    fn content_size(&self) -> Size<Pixels> {
        self.0.inner.borrow_mut().content_size()
    }

    fn scale_factor(&self) -> f32 {
        self.0.inner.borrow_mut().scale_factor
    }

    //todo!(linux)
    fn titlebar_height(&self) -> Pixels {
        unimplemented!()
    }

    //todo!(linux)
    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn display(&self) -> Rc<dyn PlatformDisplay> {
        Rc::clone(&self.0.display)
    }

    fn mouse_position(&self) -> Point<Pixels> {
        let cookie = self.0.xcb_connection.send_request(&x::QueryPointer {
            window: self.0.x_window,
        });
        let reply: x::QueryPointerReply = self.0.xcb_connection.wait_for_reply(cookie).unwrap();
        Point::new(
            (reply.root_x() as u32).into(),
            (reply.root_y() as u32).into(),
        )
    }

    //todo!(linux)
    fn modifiers(&self) -> Modifiers {
        Modifiers::default()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.0.inner.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.0.inner.borrow_mut().input_handler.take()
    }

    //todo!(linux)
    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[&str],
    ) -> futures::channel::oneshot::Receiver<usize> {
        unimplemented!()
    }

    fn activate(&self) {
        self.0.xcb_connection.send_request(&x::ConfigureWindow {
            window: self.0.x_window,
            value_list: &[x::ConfigWindow::StackMode(x::StackMode::Above)],
        });
    }

    fn set_title(&mut self, title: &str) {
        self.0.xcb_connection.send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: self.0.x_window,
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: title.as_bytes(),
        });
    }

    //todo!(linux)
    fn set_edited(&mut self, edited: bool) {}

    //todo!(linux), this corresponds to `orderFrontCharacterPalette` on macOS,
    // but it looks like the equivalent for Linux is GTK specific:
    //
    // https://docs.gtk.org/gtk3/signal.Entry.insert-emoji.html
    //
    // This API might need to change, or we might need to build an emoji picker into GPUI
    fn show_character_palette(&self) {
        unimplemented!()
    }

    //todo!(linux)
    fn minimize(&self) {
        unimplemented!()
    }

    //todo!(linux)
    fn zoom(&self) {
        unimplemented!()
    }

    //todo!(linux)
    fn toggle_full_screen(&self) {
        unimplemented!()
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut()>) {
        self.0.callbacks.borrow_mut().request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> bool>) {
        self.0.callbacks.borrow_mut().input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.callbacks.borrow_mut().active_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.callbacks.borrow_mut().resize = Some(callback);
    }

    fn on_fullscreen(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.callbacks.borrow_mut().fullscreen = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.callbacks.borrow_mut().moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.0.callbacks.borrow_mut().should_close = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.0.callbacks.borrow_mut().close = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.0.callbacks.borrow_mut().appearance_changed = Some(callback);
    }

    //todo!(linux)
    fn is_topmost_for_position(&self, _position: Point<Pixels>) -> bool {
        unimplemented!()
    }

    fn draw(&self, scene: &Scene) {
        let mut inner = self.0.inner.borrow_mut();
        inner.renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> sync::Arc<dyn PlatformAtlas> {
        let inner = self.0.inner.borrow_mut();
        inner.renderer.sprite_atlas().clone()
    }
}
