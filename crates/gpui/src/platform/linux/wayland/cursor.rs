use wayland_client::{Connection, QueueHandle};
use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_pointer::WlPointer;
use wayland_client::protocol::wl_shm::WlShm;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_cursor::{CursorImageBuffer, CursorTheme};
use crate::platform::linux::wayland::WaylandClientState;

pub(crate) struct Cursor {
    theme: CursorTheme,
    current_icon_name: String,
    surface: WlSurface,
    serial_id: u32,
}

impl Cursor {
    pub fn new(connection: &Connection, compositor: &WlCompositor, qh: &QueueHandle<WaylandClientState>, shm: &WlShm, size: u32) -> Self {
        let mut cursor_theme = CursorTheme::load(&connection, shm.clone(), size)
            .expect("Could not load cursor theme");
        Self {
            theme: cursor_theme,
            current_icon_name: "".to_string(),
            surface: compositor.create_surface(qh, ()),
            serial_id: 0,
        }
    }

    pub fn set_serial_id(&mut self, serial_id: u32) {
        self.serial_id = serial_id;
    }

    pub fn set_icon(&mut self, wl_pointer: &WlPointer, cursor_icon_name: String) {
        if self.current_icon_name != cursor_icon_name {
            let cursor = self.theme.get_cursor(cursor_icon_name.as_str()).expect("Cursor not provided by theme");
            let buffer: &CursorImageBuffer = &cursor[0];
            let (width, height) = buffer.dimensions();
            let (hot_x, hot_y) = buffer.hotspot();

            wl_pointer.set_cursor(self.serial_id, Some(&self.surface), hot_x as i32, hot_y as i32);

            self.surface.attach(Some(&buffer), 0, 0);
            self.surface.damage(0, 0, width as i32, height as i32);
            self.surface.commit();

            self.current_icon_name = cursor_icon_name;
        }
    }
}