mod event;
#[cfg(target_os = "macos")]
pub mod mac;
pub mod test;
pub mod current {
    #[cfg(target_os = "macos")]
    pub use super::mac::*;
}

use crate::{
    executor,
    fonts::{FontId, GlyphId, Metrics as FontMetrics, Properties as FontProperties},
    geometry::{
        rect::{RectF, RectI},
        vector::Vector2F,
    },
    text_layout::Line,
    Menu, Scene,
};
use anyhow::Result;
use async_task::Runnable;
pub use event::Event;
use std::{ops::Range, path::PathBuf, rc::Rc, sync::Arc};

pub trait Runner {
    fn on_finish_launching<F: 'static + FnOnce()>(self, callback: F) -> Self;
    fn on_menu_command<F: 'static + FnMut(&str)>(self, callback: F) -> Self;
    fn on_become_active<F: 'static + FnMut()>(self, callback: F) -> Self;
    fn on_resign_active<F: 'static + FnMut()>(self, callback: F) -> Self;
    fn on_event<F: 'static + FnMut(Event) -> bool>(self, callback: F) -> Self;
    fn on_open_files<F: 'static + FnMut(Vec<PathBuf>)>(self, callback: F) -> Self;
    fn set_menus(self, menus: &[Menu]) -> Self;
    fn run(self);
}

pub trait App {
    fn dispatcher(&self) -> Arc<dyn Dispatcher>;
    fn activate(&self, ignoring_other_apps: bool);
    fn open_window(
        &self,
        options: WindowOptions,
        executor: Rc<executor::Foreground>,
    ) -> Result<Box<dyn Window>>;
    fn fonts(&self) -> Arc<dyn FontSystem>;
    fn quit(&self);
}

pub trait Dispatcher: Send + Sync {
    fn is_main_thread(&self) -> bool;
    fn run_on_main_thread(&self, task: Runnable);
}

pub trait Window: WindowContext {
    fn on_event(&mut self, callback: Box<dyn FnMut(Event)>);
    fn on_resize(&mut self, callback: Box<dyn FnMut(&mut dyn WindowContext)>);
}

pub trait WindowContext {
    fn size(&self) -> Vector2F;
    fn scale_factor(&self) -> f32;
    fn present_scene(&mut self, scene: Scene);
}

pub struct WindowOptions<'a> {
    pub bounds: RectF,
    pub title: Option<&'a str>,
}

pub trait FontSystem: Send + Sync {
    fn load_family(&self, name: &str) -> anyhow::Result<Vec<FontId>>;
    fn select_font(
        &self,
        font_ids: &[FontId],
        properties: &FontProperties,
    ) -> anyhow::Result<FontId>;
    fn font_metrics(&self, font_id: FontId) -> FontMetrics;
    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<RectF>;
    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId>;
    fn rasterize_glyph(
        &self,
        font_id: FontId,
        font_size: f32,
        glyph_id: GlyphId,
        subpixel_shift: Vector2F,
        scale_factor: f32,
    ) -> Option<(RectI, Vec<u8>)>;
    fn layout_str(&self, text: &str, font_size: f32, runs: &[(Range<usize>, FontId)]) -> Line;
}
