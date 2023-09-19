#[cfg(any(test, feature = "test"))]
mod test;
use crate::{
    AnyWindowHandle, Bounds, FontFeatures, FontId, FontMetrics, FontStyle, FontWeight, GlyphId,
    LineLayout, Pixels, Point, RunStyle, SharedString,
};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::sync::Arc;

#[cfg(any(test, feature = "test"))]
pub use test::*;

pub trait Platform {
    fn font_system(&self) -> Arc<dyn PlatformFontSystem>;

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow>;
}

pub trait PlatformWindow: HasRawWindowHandle + HasRawDisplayHandle {}

pub trait PlatformFontSystem: Send + Sync {
    fn add_fonts(&self, fonts: &[Arc<Vec<u8>>]) -> anyhow::Result<()>;
    fn all_families(&self) -> Vec<String>;
    fn load_family(&self, name: &str, features: &FontFeatures) -> anyhow::Result<Vec<FontId>>;
    fn select_font(
        &self,
        font_ids: &[FontId],
        weight: FontWeight,
        style: FontStyle,
    ) -> anyhow::Result<FontId>;
    fn font_metrics(&self, font_id: FontId) -> FontMetrics;
    fn typographic_bounds(
        &self,
        font_id: FontId,
        glyph_id: GlyphId,
    ) -> anyhow::Result<Bounds<Pixels>>;
    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<Point<Pixels>>;
    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId>;
    fn rasterize_glyph(
        &self,
        font_id: FontId,
        font_size: f32,
        glyph_id: GlyphId,
        subpixel_shift: Point<Pixels>,
        scale_factor: f32,
        options: RasterizationOptions,
    ) -> Option<(Bounds<u32>, Vec<u8>)>;
    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[(usize, RunStyle)]) -> LineLayout;
    fn wrap_line(
        &self,
        text: &str,
        font_id: FontId,
        font_size: Pixels,
        width: Pixels,
    ) -> Vec<usize>;
}

#[derive(Copy, Clone, Debug)]
pub enum RasterizationOptions {
    Alpha,
    Bgra,
}

#[derive(Debug)]
pub struct WindowOptions {
    pub bounds: WindowBounds,
    pub titlebar: Option<TitlebarOptions>,
    pub center: bool,
    pub focus: bool,
    pub show: bool,
    pub kind: WindowKind,
    pub is_movable: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            bounds: WindowBounds::default(),
            titlebar: Some(TitlebarOptions {
                title: Default::default(),
                appears_transparent: Default::default(),
                traffic_light_position: Default::default(),
            }),
            center: false,
            focus: true,
            show: true,
            kind: WindowKind::Normal,
            is_movable: true,
        }
    }
}


#[derive(Debug, Default)]
pub struct TitlebarOptions {
    pub title: Option<SharedString>,
    pub appears_transparent: bool,
    pub traffic_light_position: Option<Point<f32>>,
}

#[derive(Copy, Clone, Debug)]
pub enum Appearance {
    Light,
    VibrantLight,
    Dark,
    VibrantDark,
}

impl Default for Appearance {
    fn default() -> Self {
        Self::Light
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WindowKind {
    Normal,
    PopUp,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum WindowBounds {
    Fullscreen,
    #[default]
    Maximized,
    Fixed(Bounds<f32>),
}
