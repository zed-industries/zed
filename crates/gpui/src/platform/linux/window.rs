use super::BladeRenderer;
use crate::{
    px, AnyWindowHandle, AtlasKey, AtlasTextureId, AtlasTile, BladeAtlas, Bounds, KeyDownEvent,
    Keystroke, LinuxDisplay, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput,
    PlatformInputHandler, PlatformWindow, Point, Size, TileId, WindowAppearance, WindowBounds,
    WindowOptions, WmAtoms,
};
use collections::HashMap;
use parking_lot::Mutex;
use std::{
    rc::{Rc, Weak},
    sync::{self, Arc},
};
use x11rb::{
    connection::Connection as _,
    protocol::xproto::{
        AtomEnum, ConnectionExt as _, CreateWindowAux, EventMask, PropMode, WindowClass,
    },
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
};

pub(crate) struct LinuxWindowState {
    display: Rc<dyn PlatformDisplay>,
    x11_window: u32,
    window_bounds: WindowBounds,
    content_size: Size<Pixels>,
    sprite_atlas: Arc<BladeAtlas>,
    renderer: BladeRenderer,
}

pub(crate) type LinuxWindowStatePtr = Arc<Mutex<LinuxWindowState>>;
#[derive(Clone)]
pub(crate) struct LinuxWindow(pub(crate) LinuxWindowStatePtr);

impl LinuxWindowState {
    pub fn new_ptr(
        options: WindowOptions,
        handle: AnyWindowHandle,
        x11_connection: &RustConnection,
        x11_main_screen_index: usize,
        x11_window: u32,
        atoms: &WmAtoms,
    ) -> LinuxWindowStatePtr {
        let x11_screen_index = options
            .display_id
            .map_or(x11_main_screen_index, |did| did.0 as usize);
        let screen = &x11_connection.setup().roots[x11_screen_index];

        let win_aux = CreateWindowAux::new()
            .event_mask(
                EventMask::EXPOSURE | EventMask::STRUCTURE_NOTIFY | EventMask::POINTER_MOTION,
            )
            .background_pixel(screen.white_pixel);

        let (bound_x, bound_y, bound_width, bound_height) = match options.bounds {
            WindowBounds::Fullscreen | WindowBounds::Maximized => {
                (0, 0, screen.width_in_pixels, screen.height_in_pixels)
            }
            WindowBounds::Fixed(bounds) => (
                bounds.origin.x.0 as i16,
                bounds.origin.y.0 as i16,
                bounds.size.width.0 as u16,
                bounds.size.height.0 as u16,
            ),
        };

        x11_connection
            .create_window(
                x11rb::COPY_DEPTH_FROM_PARENT,
                x11_window,
                screen.root,
                bound_x,
                bound_y,
                bound_width,
                bound_height,
                0,
                WindowClass::INPUT_OUTPUT,
                0,
                &win_aux,
            )
            .unwrap();

        if let Some(titlebar) = options.titlebar {
            if let Some(title) = titlebar.title {
                x11_connection
                    .change_property8(
                        PropMode::REPLACE,
                        x11_window,
                        AtomEnum::WM_NAME,
                        AtomEnum::STRING,
                        title.as_bytes(),
                    )
                    .unwrap();
            }
        }
        x11_connection
            .change_property32(
                PropMode::REPLACE,
                x11_window,
                atoms.protocols,
                AtomEnum::ATOM,
                &[atoms.delete_window],
            )
            .unwrap();

        x11_connection.map_window(x11_window).unwrap();
        x11_connection.flush().unwrap();

        let gpu = Arc::new(
            unsafe {
                blade::Context::init(blade::ContextDesc {
                    validation: cfg!(debug_assertions),
                    capture: false,
                })
            }
            .unwrap(),
        );

        Arc::new(Mutex::new(Self {
            display: Rc::new(LinuxDisplay::new(x11_connection, x11_screen_index)),
            x11_window,
            window_bounds: options.bounds,
            content_size: Size {
                width: Pixels(bound_width as f32),
                height: Pixels(bound_height as f32),
            },
            sprite_atlas: Arc::new(BladeAtlas::new(&gpu)),
            renderer: BladeRenderer::new(gpu),
        }))
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.content_size = Size {
            width: Pixels(width as f32),
            height: Pixels(height as f32),
        };
    }

    pub fn destroy(&mut self) {
        self.sprite_atlas.destroy();
    }

    pub fn paint(&mut self) {
        //TODO
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

    fn on_request_frame(&self, _callback: Box<dyn FnMut()>) {}

    fn on_input(&self, callback: Box<dyn FnMut(crate::PlatformInput) -> bool>) {}

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {}

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {}

    fn on_fullscreen(&self, _callback: Box<dyn FnMut(bool)>) {}

    fn on_moved(&self, callback: Box<dyn FnMut()>) {}

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {}

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {
        unimplemented!()
    }

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn is_topmost_for_position(&self, _position: crate::Point<Pixels>) -> bool {
        unimplemented!()
    }

    fn invalidate(&self) {}

    fn draw(&self, _scene: &crate::Scene) {}

    fn sprite_atlas(&self) -> sync::Arc<dyn crate::PlatformAtlas> {
        self.0.lock().sprite_atlas.clone()
    }
}
