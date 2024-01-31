use super::BladeRenderer;
use crate::{
    AnyWindowHandle, BladeAtlas, LinuxDisplay, Pixels, PlatformDisplay, PlatformInputHandler,
    PlatformWindow, Point, Size, WindowAppearance, WindowBounds, WindowOptions, XcbAtoms,
};
use blade_graphics as gpu;
use parking_lot::Mutex;
use std::{
    ffi::c_void,
    rc::Rc,
    sync::{self, Arc},
};
use xcb::{x, Xid as _};

#[derive(Default)]
struct Callbacks {
    request_frame: Option<Box<dyn FnMut()>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    moved: Option<Box<dyn FnMut()>>,
}

pub(crate) struct LinuxWindowState {
    display: Rc<dyn PlatformDisplay>,
    x_window: x::Window,
    window_bounds: WindowBounds,
    content_size: Size<Pixels>,
    sprite_atlas: Arc<BladeAtlas>,
    renderer: BladeRenderer,
    //TODO: move out into a separate struct
    callbacks: Callbacks,
}

pub(crate) type LinuxWindowStatePtr = Arc<Mutex<LinuxWindowState>>;
#[derive(Clone)]
pub(crate) struct LinuxWindow(pub(crate) LinuxWindowStatePtr);

struct RawWindow {
    connection: *mut c_void,
    screen_id: i32,
    window_id: u32,
    visual_id: u32,
}
unsafe impl raw_window_handle::HasRawWindowHandle for RawWindow {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        let mut wh = raw_window_handle::XcbWindowHandle::empty();
        wh.window = self.window_id;
        wh.visual_id = self.visual_id;
        wh.into()
    }
}
unsafe impl raw_window_handle::HasRawDisplayHandle for RawWindow {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        let mut dh = raw_window_handle::XcbDisplayHandle::empty();
        dh.connection = self.connection;
        dh.screen = self.screen_id;
        dh.into()
    }
}

impl LinuxWindowState {
    pub fn new_ptr(
        options: WindowOptions,
        xcb_connection: &xcb::Connection,
        x_main_screen_index: i32,
        x_window: x::Window,
        atoms: &XcbAtoms,
    ) -> LinuxWindowStatePtr {
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

        let (bound_x, bound_y, bound_width, bound_height) = match options.bounds {
            WindowBounds::Fullscreen | WindowBounds::Maximized => {
                (0, 0, screen.width_in_pixels(), screen.height_in_pixels())
            }
            WindowBounds::Fixed(bounds) => (
                bounds.origin.x.0 as i16,
                bounds.origin.y.0 as i16,
                bounds.size.width.0 as u16,
                bounds.size.height.0 as u16,
            ),
        };

        xcb_connection.send_request(&x::CreateWindow {
            depth: x::COPY_FROM_PARENT as u8,
            wid: x_window,
            parent: screen.root(),
            x: bound_x,
            y: bound_y,
            width: bound_width,
            height: bound_height,
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

        let raw_window = RawWindow {
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
                    &raw_window,
                    gpu::ContextDesc {
                        validation: cfg!(debug_assertions),
                        capture: false,
                    },
                )
            }
            .unwrap(),
        );
        let gpu_extent = gpu::Extent {
            width: bound_width as u32,
            height: bound_height as u32,
            depth: 1,
        };

        Arc::new(Mutex::new(Self {
            display: Rc::new(LinuxDisplay::new(xcb_connection, x_screen_index)),
            x_window,
            window_bounds: options.bounds,
            content_size: Size {
                width: Pixels(bound_width as f32),
                height: Pixels(bound_height as f32),
            },
            sprite_atlas: Arc::new(BladeAtlas::new(&gpu)),
            renderer: BladeRenderer::new(gpu, gpu_extent),
            callbacks: Callbacks::default(),
        }))
    }

    pub fn destroy(&mut self) {
        self.sprite_atlas.destroy();
        self.renderer.destroy();
    }

    pub fn resize(self_ptr: &LinuxWindowStatePtr, width: u16, height: u16) {
        let content_size = Size {
            width: Pixels(width as f32),
            height: Pixels(height as f32),
        };

        let mut fun = match self_ptr.lock().callbacks.resize.take() {
            Some(fun) => fun,
            None => return,
        };
        fun(content_size, 1.0);

        let mut this = self_ptr.lock();
        this.callbacks.resize = Some(fun);
        this.content_size = content_size;
        this.renderer.resize(gpu::Extent {
            width: width as u32,
            height: height as u32,
            depth: 1,
        });
    }

    pub fn request_frame(self_ptr: &LinuxWindowStatePtr) {
        let mut fun = match self_ptr.lock().callbacks.request_frame.take() {
            Some(fun) => fun,
            None => return,
        };
        fun();

        self_ptr.lock().callbacks.request_frame = Some(fun);
    }
}

impl PlatformWindow for LinuxWindow {
    fn bounds(&self) -> WindowBounds {
        //TODO: update when window moves
        self.0.lock().window_bounds
    }

    fn content_size(&self) -> Size<Pixels> {
        self.0.lock().content_size
    }

    fn scale_factor(&self) -> f32 {
        1.0
    }

    fn titlebar_height(&self) -> Pixels {
        unimplemented!()
    }

    fn appearance(&self) -> WindowAppearance {
        unimplemented!()
    }

    fn display(&self) -> Rc<dyn PlatformDisplay> {
        Rc::clone(&self.0.lock().display)
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
        self.0.lock().callbacks.request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(crate::PlatformInput) -> bool>) {}

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {}

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.lock().callbacks.resize = Some(callback);
    }

    fn on_fullscreen(&self, _callback: Box<dyn FnMut(bool)>) {}

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().callbacks.moved = Some(callback);
    }

    fn on_should_close(&self, _callback: Box<dyn FnMut() -> bool>) {}

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {}

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {}

    fn is_topmost_for_position(&self, _position: crate::Point<Pixels>) -> bool {
        unimplemented!()
    }

    fn invalidate(&self) {}

    fn draw(&self, scene: &crate::Scene) {
        self.0.lock().renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> sync::Arc<dyn crate::PlatformAtlas> {
        self.0.lock().sprite_atlas.clone()
    }
}
