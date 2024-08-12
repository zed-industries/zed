use crate::{CursorStyle, Globals};
use util::ResultExt;

use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::protocol::{wl_pointer::WlPointer, wl_shm::WlShm};
use wayland_client::Connection;
use wayland_cursor::{CursorImageBuffer, CursorTheme};
use wayland_protocols::wp::cursor_shape::v1::client::wp_cursor_shape_device_v1::WpCursorShapeDeviceV1;

pub(crate) struct Cursor {
    theme: Option<CursorTheme>,
    theme_name: Option<String>,
    surface: WlSurface,
    size: u32,
    hidden: bool,
    shm: WlShm,
    shape_device: Option<WpCursorShapeDeviceV1>,
    wl_pointer: Option<WlPointer>,
    connection: Connection,
}

impl Drop for Cursor {
    fn drop(&mut self) {
        if let Some(shape_device) = &self.shape_device {
            shape_device.destroy();
        }
        self.theme.take();
        self.surface.destroy();
    }
}

impl Cursor {
    pub fn new(connection: &Connection, globals: &Globals, size: u32) -> Self {
        Self {
            theme: CursorTheme::load(&connection, globals.shm.clone(), size).log_err(),
            theme_name: None,
            surface: globals.compositor.create_surface(&globals.qh, ()),
            shm: globals.shm.clone(),
            shape_device: None,
            wl_pointer: None,
            connection: connection.clone(),
            size,
            hidden: false,
        }
    }

    pub fn set_pointer(
        &mut self,
        wl_pointer: WlPointer,
        shape_device: Option<WpCursorShapeDeviceV1>,
    ) {
        if let Some(shape_device) = self.shape_device.take() {
            shape_device.destroy();
        }
        self.wl_pointer = Some(wl_pointer);
        self.shape_device = shape_device;
    }

    pub fn set_theme(&mut self, theme_name: &str, size: Option<u32>) {
        if let Some(size) = size {
            self.size = size;
        }
        if let Some(theme) =
            CursorTheme::load_from_name(&self.connection, self.shm.clone(), theme_name, self.size)
                .log_err()
        {
            self.theme = Some(theme);
            self.theme_name = Some(theme_name.to_string());
        } else if let Some(theme) =
            CursorTheme::load(&self.connection, self.shm.clone(), self.size).log_err()
        {
            self.theme = Some(theme);
            self.theme_name = None;
        }
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size;
        self.theme = self
            .theme_name
            .as_ref()
            .and_then(|name| {
                CursorTheme::load_from_name(
                    &self.connection,
                    self.shm.clone(),
                    name.as_str(),
                    self.size,
                )
                .log_err()
            })
            .or_else(|| CursorTheme::load(&self.connection, self.shm.clone(), self.size).log_err());
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn hide(&mut self, serial: u32) {
        if let Some(wl_pointer) = &self.wl_pointer {
            self.hidden = true;
            wl_pointer.set_cursor(serial, None, 0, 0);
        }
    }

    pub fn unhide(&mut self, serial: u32, style: CursorStyle) {
        self.hidden = false;
        self.set_style(serial, style);
    }

    pub fn set_style(&mut self, serial: u32, style: CursorStyle) {
        if let Some(shape_device) = &self.shape_device {
            shape_device.set_shape(serial, style.to_shape());
            return;
        }
        // cursor-shape-v1 isn't supported; fallback to surface method
        self.set_style_surface(serial, &style.to_icon_name());
    }

    fn set_style_surface(&mut self, serial: u32, mut cursor_icon_name: &str) {
        if let (Some(wl_pointer), Some(theme)) = (&self.wl_pointer, &mut self.theme) {
            let mut buffer: Option<&CursorImageBuffer>;

            if let Some(cursor) = theme.get_cursor(&cursor_icon_name) {
                buffer = Some(&cursor[0]);
            } else if let Some(cursor) = theme.get_cursor("default") {
                buffer = Some(&cursor[0]);
                cursor_icon_name = "default";
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

                wl_pointer.set_cursor(serial, Some(&self.surface), hot_x as i32, hot_y as i32);
                self.surface.attach(Some(&buffer), 0, 0);
                self.surface.damage(0, 0, width as i32, height as i32);
                self.surface.commit();
            }
        } else {
            log::warn!("Linux: Wayland: Unable to load cursor themes");
        }
    }
}
