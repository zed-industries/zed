mod event;
#[cfg(target_os = "macos")]
pub mod mac;
pub mod test;
pub mod current {
    #[cfg(target_os = "macos")]
    pub use super::mac::*;
}

use crate::{
    color::ColorU,
    executor,
    fonts::{FontId, GlyphId, Metrics as FontMetrics, Properties as FontProperties},
    geometry::{
        rect::{RectF, RectI},
        vector::Vector2F,
    },
    text_layout::LineLayout,
    ClipboardItem, Menu, Scene,
};
use async_task::Runnable;
pub use event::Event;
use std::{
    any::Any,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

pub trait Platform: Send + Sync {
    fn dispatcher(&self) -> Arc<dyn Dispatcher>;
    fn fonts(&self) -> Arc<dyn FontSystem>;

    fn activate(&self, ignoring_other_apps: bool);
    fn open_window(
        &self,
        id: usize,
        options: WindowOptions,
        executor: Rc<executor::Foreground>,
    ) -> Box<dyn Window>;
    fn key_window_id(&self) -> Option<usize>;
    fn quit(&self);

    fn write_to_clipboard(&self, item: ClipboardItem);
    fn read_from_clipboard(&self) -> Option<ClipboardItem>;
    fn open_url(&self, url: &str);

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]);
    fn read_credentials(&self, url: &str) -> Option<(String, Vec<u8>)>;
}

pub(crate) trait ForegroundPlatform {
    fn on_become_active(&self, callback: Box<dyn FnMut()>);
    fn on_resign_active(&self, callback: Box<dyn FnMut()>);
    fn on_event(&self, callback: Box<dyn FnMut(Event) -> bool>);
    fn on_open_files(&self, callback: Box<dyn FnMut(Vec<PathBuf>)>);
    fn run(&self, on_finish_launching: Box<dyn FnOnce() -> ()>);

    fn on_menu_command(&self, callback: Box<dyn FnMut(&str, Option<&dyn Any>)>);
    fn set_menus(&self, menus: Vec<Menu>);
    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
        done_fn: Box<dyn FnOnce(Option<Vec<std::path::PathBuf>>)>,
    );
    fn prompt_for_new_path(
        &self,
        directory: &Path,
        done_fn: Box<dyn FnOnce(Option<std::path::PathBuf>)>,
    );
}

pub trait Dispatcher: Send + Sync {
    fn is_main_thread(&self) -> bool;
    fn run_on_main_thread(&self, task: Runnable);
}

pub trait Window: WindowContext {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn on_event(&mut self, callback: Box<dyn FnMut(Event)>);
    fn on_resize(&mut self, callback: Box<dyn FnMut(&mut dyn WindowContext)>);
    fn on_close(&mut self, callback: Box<dyn FnOnce()>);
    fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        answers: &[&str],
        done_fn: Box<dyn FnOnce(usize)>,
    );
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

pub struct PathPromptOptions {
    pub files: bool,
    pub directories: bool,
    pub multiple: bool,
}

pub enum PromptLevel {
    Info,
    Warning,
    Critical,
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
    fn layout_line(
        &self,
        text: &str,
        font_size: f32,
        runs: &[(usize, FontId, ColorU)],
    ) -> LineLayout;
    fn wrap_line(&self, text: &str, font_id: FontId, font_size: f32, width: f32) -> Vec<usize>;
}
