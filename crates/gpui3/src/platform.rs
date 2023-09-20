mod events;
mod keystroke;
#[cfg(target_os = "macos")]
mod mac;
#[cfg(any(test, feature = "test"))]
mod test;

use crate::{
    AnyWindowHandle, Bounds, FontFeatures, FontId, FontMetrics, FontStyle, FontWeight, GlyphId,
    LineLayout, Pixels, Point, RunStyle, SharedString, Size,
};
use async_task::Runnable;
use futures::channel::oneshot;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::{any::Any, fmt::Debug, ops::Range, rc::Rc, sync::Arc};
use uuid::Uuid;

pub use events::*;
pub use keystroke::*;
#[cfg(target_os = "macos")]
pub use mac::*;
#[cfg(any(test, feature = "test"))]
pub use test::*;

pub trait Platform {
    fn dispatcher(&self) -> Arc<dyn PlatformDispatcher>;
    fn font_system(&self) -> Arc<dyn PlatformTextSystem>;

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow>;
}

pub trait PlatformScreen: Debug {
    fn as_any(&self) -> &dyn Any;
    fn bounds(&self) -> Bounds<Pixels>;
    fn content_bounds(&self) -> Bounds<Pixels>;
    fn display_uuid(&self) -> Option<Uuid>;
}

pub trait PlatformWindow: HasRawWindowHandle + HasRawDisplayHandle {
    fn bounds(&self) -> WindowBounds;
    fn content_size(&self) -> Size<Pixels>;
    fn scale_factor(&self) -> f32;
    fn titlebar_height(&self) -> Pixels;
    fn appearance(&self) -> WindowAppearance;
    fn screen(&self) -> Rc<dyn PlatformScreen>;
    fn mouse_position(&self) -> Point<Pixels>;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn set_input_handler(&mut self, input_handler: Box<dyn InputHandler>);
    fn prompt(
        &self,
        level: WindowPromptLevel,
        msg: &str,
        answers: &[&str],
    ) -> oneshot::Receiver<usize>;
    fn activate(&self);
    fn set_title(&mut self, title: &str);
    fn set_edited(&mut self, edited: bool);
    fn show_character_palette(&self);
    fn minimize(&self);
    fn zoom(&self);
    fn toggle_full_screen(&self);
    fn on_event(&mut self, callback: Box<dyn FnMut(Event) -> bool>);
    fn on_active_status_change(&mut self, callback: Box<dyn FnMut(bool)>);
    fn on_resize(&mut self, callback: Box<dyn FnMut()>);
    fn on_fullscreen(&mut self, callback: Box<dyn FnMut(bool)>);
    fn on_moved(&mut self, callback: Box<dyn FnMut()>);
    fn on_should_close(&mut self, callback: Box<dyn FnMut() -> bool>);
    fn on_close(&mut self, callback: Box<dyn FnOnce()>);
    fn on_appearance_changed(&mut self, callback: Box<dyn FnMut()>);
    fn is_topmost_for_position(&self, position: Point<Pixels>) -> bool;
}

pub trait PlatformDispatcher: Send + Sync {
    fn is_main_thread(&self) -> bool;
    fn run_on_main_thread(&self, task: Runnable);
}

pub trait PlatformTextSystem: Send + Sync {
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

pub trait InputHandler {
    fn selected_text_range(&self) -> Option<Range<usize>>;
    fn marked_text_range(&self) -> Option<Range<usize>>;
    fn text_for_range(&self, range_utf16: Range<usize>) -> Option<String>;
    fn replace_text_in_range(&mut self, replacement_range: Option<Range<usize>>, text: &str);
    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
    );
    fn unmark_text(&mut self);
    fn bounds_for_range(&self, range_utf16: Range<usize>) -> Option<Bounds<f32>>;
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
    pub screen: Option<Rc<dyn PlatformScreen>>,
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
            screen: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct TitlebarOptions {
    pub title: Option<SharedString>,
    pub appears_transparent: bool,
    pub traffic_light_position: Option<Point<Pixels>>,
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
    Fixed(Bounds<Pixels>),
}

#[derive(Copy, Clone, Debug)]
pub enum WindowAppearance {
    Light,
    VibrantLight,
    Dark,
    VibrantDark,
}

impl Default for WindowAppearance {
    fn default() -> Self {
        Self::Light
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum WindowPromptLevel {
    #[default]
    Info,
    Warning,
    Critical,
}
