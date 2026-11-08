use crate::linux::Globals;
use crate::linux::{DEFAULT_CURSOR_ICON_NAME, log_cursor_icon_warning};
use anyhow::{Context as _, anyhow};
use gpui_util::ResultExt;

use wayland_client::Connection;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::protocol::{wl_pointer::WlPointer, wl_shm::WlShm};
use wayland_cursor::{CursorImageBuffer, CursorTheme};
use wayland_protocols::wp::tablet::zv2::client::zwp_tablet_tool_v2::ZwpTabletToolV2;

/// The input device whose cursor image is being set. Each tablet tool needs
/// its own surface because its cursor role cannot be reused by another tool.
pub(crate) enum CursorTarget<'a> {
    Pointer(&'a WlPointer),
    TabletTool(&'a ZwpTabletToolV2),
}

impl CursorTarget<'_> {
    fn set_cursor(&self, serial: u32, surface: &WlSurface, hotspot_x: i32, hotspot_y: i32) {
        match self {
            CursorTarget::Pointer(pointer) => {
                pointer.set_cursor(serial, Some(surface), hotspot_x, hotspot_y)
            }
            CursorTarget::TabletTool(tool) => {
                tool.set_cursor(serial, Some(surface), hotspot_x, hotspot_y)
            }
        }
    }
}

pub(crate) struct Cursor {
    loaded_theme: Option<LoadedTheme>,
    size: u32,
    scaled_size: u32,
    surface: WlSurface,
    shm: WlShm,
    connection: Connection,
}

pub(crate) struct LoadedTheme {
    theme: CursorTheme,
    name: Option<String>,
    scaled_size: u32,
}

impl Drop for Cursor {
    fn drop(&mut self) {
        self.loaded_theme.take();
        self.surface.destroy();
    }
}

impl Cursor {
    pub fn new(connection: &Connection, globals: &Globals, size: u32) -> Self {
        let mut this = Self {
            loaded_theme: None,
            size,
            scaled_size: size,
            surface: globals.compositor.create_surface(&globals.qh, ()),
            shm: globals.shm.clone(),
            connection: connection.clone(),
        };
        this.set_theme_internal(None);
        this
    }

    fn set_theme_internal(&mut self, theme_name: Option<String>) {
        if let Some(loaded_theme) = self.loaded_theme.as_ref()
            && loaded_theme.name == theme_name
            && loaded_theme.scaled_size == self.scaled_size
        {
            return;
        }
        let result = if let Some(theme_name) = theme_name.as_ref() {
            CursorTheme::load_from_name(
                &self.connection,
                self.shm.clone(),
                theme_name,
                self.scaled_size,
            )
        } else {
            CursorTheme::load(&self.connection, self.shm.clone(), self.scaled_size)
        };
        if let Some(theme) = result
            .context("Wayland: Failed to load cursor theme")
            .log_err()
        {
            self.loaded_theme = Some(LoadedTheme {
                theme,
                name: theme_name,
                scaled_size: self.scaled_size,
            });
        }
    }

    pub fn set_theme(&mut self, theme_name: String) {
        self.set_theme_internal(Some(theme_name));
    }

    fn set_scaled_size(&mut self, scaled_size: u32) {
        self.scaled_size = scaled_size;
        let theme_name = self
            .loaded_theme
            .as_ref()
            .and_then(|loaded_theme| loaded_theme.name.clone());
        self.set_theme_internal(theme_name);
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size;
        self.set_scaled_size(size);
    }

    pub fn set_icon(
        &mut self,
        wl_pointer: &WlPointer,
        serial_id: u32,
        cursor_icon_names: &[&str],
        scale: i32,
    ) {
        let surface = self.surface.clone();
        self.set_icon_on_surface(
            CursorTarget::Pointer(wl_pointer),
            &surface,
            serial_id,
            cursor_icon_names,
            scale,
        );
    }

    /// Shows the themed icon on `surface` and makes it `target`'s cursor.
    /// Uses the shared cursor theme and size with the supplied surface scale.
    pub fn set_icon_on_surface(
        &mut self,
        target: CursorTarget,
        surface: &WlSurface,
        serial_id: u32,
        cursor_icon_names: &[&str],
        scale: i32,
    ) {
        self.set_scaled_size(self.size * scale as u32);

        let Some(loaded_theme) = &mut self.loaded_theme else {
            log::warn!("Wayland: Unable to load cursor themes");
            return;
        };
        let theme = &mut loaded_theme.theme;

        let buffer: &CursorImageBuffer;
        'outer: {
            for cursor_icon_name in cursor_icon_names {
                if let Some(cursor) = theme.get_cursor(cursor_icon_name) {
                    buffer = &cursor[0];
                    break 'outer;
                }
            }

            if let Some(cursor) = theme.get_cursor(DEFAULT_CURSOR_ICON_NAME) {
                buffer = &cursor[0];
                log_cursor_icon_warning(anyhow!(
                    "wayland: Unable to get cursor icon {:?}. \
                    Using default cursor icon: '{}'",
                    cursor_icon_names,
                    DEFAULT_CURSOR_ICON_NAME
                ));
            } else {
                log_cursor_icon_warning(anyhow!(
                    "wayland: Unable to fallback on default cursor icon '{}' for theme '{}'",
                    DEFAULT_CURSOR_ICON_NAME,
                    loaded_theme.name.as_deref().unwrap_or("default")
                ));
                return;
            }
        }

        let (width, height) = buffer.dimensions();
        let (hot_x, hot_y) = buffer.hotspot();

        surface.set_buffer_scale(scale);

        target.set_cursor(
            serial_id,
            surface,
            hot_x as i32 / scale,
            hot_y as i32 / scale,
        );

        surface.attach(Some(buffer), 0, 0);
        surface.damage(0, 0, width as i32, height as i32);
        surface.commit();
    }
}
