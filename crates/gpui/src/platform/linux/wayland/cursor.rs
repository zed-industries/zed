use crate::Globals;
use crate::platform::linux::DEFAULT_CURSOR_ICON_NAME;
use util::ResultExt;

use wayland_client::Connection;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::protocol::{wl_pointer::WlPointer, wl_shm::WlShm};
use wayland_cursor::{CursorImageBuffer, CursorTheme};

pub(crate) struct Cursor {
    theme: Option<CursorTheme>,
    theme_name: Option<String>,
    theme_size: u32,
    surface: WlSurface,
    size: u32,
    shm: WlShm,
    connection: Connection,
}

impl Drop for Cursor {
    fn drop(&mut self) {
        self.theme.take();
        self.surface.destroy();
    }
}

impl Cursor {
    pub fn new(connection: &Connection, globals: &Globals, size: u32) -> Self {
        Self {
            theme: CursorTheme::load(&connection, globals.shm.clone(), size).log_err(),
            theme_name: None,
            theme_size: size,
            surface: globals.compositor.create_surface(&globals.qh, ()),
            shm: globals.shm.clone(),
            connection: connection.clone(),
            size,
        }
    }

    pub fn set_theme(&mut self, theme_name: &str) {
        if let Some(theme) = CursorTheme::load_from_name(
            &self.connection,
            self.shm.clone(),
            theme_name,
            self.theme_size,
        )
        .log_err()
        {
            self.theme = Some(theme);
            self.theme_name = Some(theme_name.to_string());
        } else if let Some(theme) =
            CursorTheme::load(&self.connection, self.shm.clone(), self.theme_size).log_err()
        {
            self.theme = Some(theme);
            self.theme_name = None;
        }
    }

    fn set_theme_size(&mut self, theme_size: u32) {
        self.theme = self
            .theme_name
            .as_ref()
            .and_then(|name| {
                CursorTheme::load_from_name(
                    &self.connection,
                    self.shm.clone(),
                    name.as_str(),
                    theme_size,
                )
                .log_err()
            })
            .or_else(|| {
                CursorTheme::load(&self.connection, self.shm.clone(), theme_size).log_err()
            });
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size;
        self.set_theme_size(size);
    }

    pub fn set_icon(
        &mut self,
        wl_pointer: &WlPointer,
        serial_id: u32,
        mut cursor_icon_names: &[&str],
        scale: i32,
    ) {
        self.set_theme_size(self.size * scale as u32);

        let Some(theme) = &mut self.theme else {
            log::warn!("Wayland: Unable to load cursor themes");
            return;
        };

        let mut buffer: &CursorImageBuffer;
        'outer: {
            for cursor_icon_name in cursor_icon_names {
                if let Some(cursor) = theme.get_cursor(cursor_icon_name) {
                    buffer = &cursor[0];
                    break 'outer;
                }
            }

            if let Some(cursor) = theme.get_cursor(DEFAULT_CURSOR_ICON_NAME) {
                buffer = &cursor[0];
                log::warn!(
                    "Wayland: Unable to get cursor icon {:?}. \
                    Using default cursor icon: '{}'",
                    cursor_icon_names,
                    DEFAULT_CURSOR_ICON_NAME
                );
            } else {
                log::warn!(
                    "Wayland: Unable to fallback on default cursor icon '{}' for theme '{}'",
                    DEFAULT_CURSOR_ICON_NAME,
                    self.theme_name.as_deref().unwrap_or("default")
                );
                return;
            }
        }

        let (width, height) = buffer.dimensions();
        let (hot_x, hot_y) = buffer.hotspot();

        self.surface.set_buffer_scale(scale);

        wl_pointer.set_cursor(
            serial_id,
            Some(&self.surface),
            hot_x as i32 / scale,
            hot_y as i32 / scale,
        );

        self.surface.attach(Some(&buffer), 0, 0);
        self.surface.damage(0, 0, width as i32, height as i32);
        self.surface.commit();
    }
}
