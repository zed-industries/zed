use crate::platform::linux::wayland::WaylandClientState;
use wayland_backend::client::InvalidId;
use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_pointer::WlPointer;
use wayland_client::protocol::wl_shm::WlShm;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::{Connection, QueueHandle};
use wayland_cursor::{CursorImageBuffer, CursorTheme};

pub(crate) struct Cursor {
    theme: Result<CursorTheme, InvalidId>,
    current_icon_name: String,
    surface: WlSurface,
    serial_id: u32,
}

impl Cursor {
    pub fn new(
        connection: &Connection,
        compositor: &WlCompositor,
        qh: &QueueHandle<WaylandClientState>,
        shm: &WlShm,
        size: u32,
    ) -> Self {
        Self {
            theme: CursorTheme::load(&connection, shm.clone(), size),
            current_icon_name: "".to_string(),
            surface: compositor.create_surface(qh, ()),
            serial_id: 0,
        }
    }

    pub fn set_serial_id(&mut self, serial_id: u32) {
        self.serial_id = serial_id;
    }

    pub fn set_icon(&mut self, wl_pointer: &WlPointer, cursor_icon_name: String) {
        let mut cursor_icon_name = cursor_icon_name.clone();
        if self.current_icon_name != cursor_icon_name {
            if let Ok(theme) = &mut self.theme {
                let mut buffer: Option<&CursorImageBuffer>;

                if let Some(cursor) = theme.get_cursor(&cursor_icon_name) {
                    buffer = Some(&cursor[0]);
                } else if let Some(cursor) = theme.get_cursor("default") {
                    buffer = Some(&cursor[0]);
                    cursor_icon_name = "default".to_string();
                    log::warn!(
                        "Linux: Wayland: Unable to get cursor icon: {}. Using default cursor icon",
                        cursor_icon_name
                    );
                } else {
                    buffer = None;
                    log::warn!("Linux: Wayland: Unable to get default cursor too!");
                }

                if let Some(buffer) = &mut buffer {
                    let (width, height) = buffer.dimensions();
                    let (hot_x, hot_y) = buffer.hotspot();

                    wl_pointer.set_cursor(
                        self.serial_id,
                        Some(&self.surface),
                        hot_x as i32,
                        hot_y as i32,
                    );
                    self.surface.attach(Some(&buffer), 0, 0);
                    self.surface.damage(0, 0, width as i32, height as i32);
                    self.surface.commit();

                    self.current_icon_name = cursor_icon_name;
                }
            } else {
                log::warn!("Linux: Wayland: Unable to load cursor themes");
            }
        }
    }
}
