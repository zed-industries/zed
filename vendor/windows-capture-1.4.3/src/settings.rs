use windows::Graphics::Capture::GraphicsCaptureItem;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ColorFormat {
    Rgba16F = 10,
    Rgba8 = 28,
    Bgra8 = 87,
}

impl Default for ColorFormat {
    #[inline]
    fn default() -> Self {
        Self::Rgba8
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum CursorCaptureSettings {
    Default,
    WithCursor,
    WithoutCursor,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum DrawBorderSettings {
    Default,
    WithBorder,
    WithoutBorder,
}

#[derive(Eq, PartialEq, Clone, Debug)]
/// Represents the settings for screen capturing.
pub struct Settings<Flags, T: TryInto<GraphicsCaptureItem>> {
    /// The graphics capture item to capture.
    pub(crate) item: T,
    /// Specifies whether to capture the cursor.
    pub(crate) cursor_capture: CursorCaptureSettings,
    /// Specifies whether to draw a border around the captured region.
    pub(crate) draw_border: DrawBorderSettings,
    /// The color format for the captured graphics.
    pub(crate) color_format: ColorFormat,
    /// Additional flags for capturing graphics.
    pub(crate) flags: Flags,
}

impl<Flags, T: TryInto<GraphicsCaptureItem>> Settings<Flags, T> {
    /// Create Capture Settings
    ///
    /// # Arguments
    ///
    /// * `item` - The graphics capture item.
    /// * `capture_cursor` - Whether to capture the cursor or not.
    /// * `draw_border` - Whether to draw a border around the captured region or not.
    /// * `color_format` - The desired color format for the captured frame.
    /// * `flags` - Additional flags for the capture settings that will be passed to user defined `new` function.
    #[must_use]
    #[inline]
    pub const fn new(
        item: T,
        cursor_capture: CursorCaptureSettings,
        draw_border: DrawBorderSettings,
        color_format: ColorFormat,
        flags: Flags,
    ) -> Self {
        Self {
            item,
            cursor_capture,
            draw_border,
            color_format,
            flags,
        }
    }

    /// Get the item
    ///
    /// # Returns
    ///
    /// The item to be captured
    #[must_use]
    #[inline]
    pub const fn item(&self) -> &T {
        &self.item
    }

    /// Get the cursor capture settings
    ///
    /// # Returns
    ///
    /// The cursor capture settings
    #[must_use]
    #[inline]
    pub const fn cursor_capture(&self) -> CursorCaptureSettings {
        self.cursor_capture
    }

    /// Get the draw border settings
    ///
    /// # Returns
    ///
    /// The draw border settings
    #[must_use]
    #[inline]
    pub const fn draw_border(&self) -> DrawBorderSettings {
        self.draw_border
    }

    /// Get the color format
    ///
    /// # Returns
    ///
    /// The color format
    #[must_use]
    #[inline]
    pub const fn color_format(&self) -> ColorFormat {
        self.color_format
    }

    /// Get the flags
    ///
    /// # Returns
    ///
    /// The flags
    #[must_use]
    #[inline]
    pub const fn flags(&self) -> &Flags {
        &self.flags
    }
}
