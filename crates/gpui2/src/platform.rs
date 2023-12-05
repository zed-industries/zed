mod app_menu;
mod keystroke;
#[cfg(target_os = "macos")]
mod mac;
#[cfg(any(test, feature = "test-support"))]
mod test;

use crate::{
    point, size, Action, AnyWindowHandle, BackgroundExecutor, Bounds, DevicePixels, Font, FontId,
    FontMetrics, FontRun, ForegroundExecutor, GlobalPixels, GlyphId, InputEvent, LineLayout,
    Pixels, Point, RenderGlyphParams, RenderImageParams, RenderSvgParams, Result, Scene,
    SharedString, Size, TaskLabel,
};
use anyhow::{anyhow, bail};
use async_task::Runnable;
use futures::channel::oneshot;
use parking::Unparker;
use seahash::SeaHasher;
use serde::{Deserialize, Serialize};
use sqlez::bindable::{Bind, Column, StaticColumnCount};
use sqlez::statement::Statement;
use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use std::{
    any::Any,
    fmt::{self, Debug, Display},
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
    sync::Arc,
};
use uuid::Uuid;

pub use app_menu::*;
pub use keystroke::*;
#[cfg(target_os = "macos")]
pub use mac::*;
#[cfg(any(test, feature = "test-support"))]
pub use test::*;
pub use time::UtcOffset;

#[cfg(target_os = "macos")]
pub(crate) fn current_platform() -> Rc<dyn Platform> {
    Rc::new(MacPlatform::new())
}

pub trait Platform: 'static {
    fn background_executor(&self) -> BackgroundExecutor;
    fn foreground_executor(&self) -> ForegroundExecutor;
    fn text_system(&self) -> Arc<dyn PlatformTextSystem>;

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>);
    fn quit(&self);
    fn restart(&self);
    fn activate(&self, ignoring_other_apps: bool);
    fn hide(&self);
    fn hide_other_apps(&self);
    fn unhide_other_apps(&self);

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>>;
    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>>;
    fn active_window(&self) -> Option<AnyWindowHandle>;
    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow>;

    fn set_display_link_output_callback(
        &self,
        display_id: DisplayId,
        callback: Box<dyn FnMut(&VideoTimestamp, &VideoTimestamp) + Send>,
    );
    fn start_display_link(&self, display_id: DisplayId);
    fn stop_display_link(&self, display_id: DisplayId);
    // fn add_status_item(&self, _handle: AnyWindowHandle) -> Box<dyn PlatformWindow>;

    fn open_url(&self, url: &str);
    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>);
    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>>;
    fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>>;
    fn reveal_path(&self, path: &Path);

    fn on_become_active(&self, callback: Box<dyn FnMut()>);
    fn on_resign_active(&self, callback: Box<dyn FnMut()>);
    fn on_quit(&self, callback: Box<dyn FnMut()>);
    fn on_reopen(&self, callback: Box<dyn FnMut()>);
    fn on_event(&self, callback: Box<dyn FnMut(InputEvent) -> bool>);

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>);
    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>);
    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>);

    fn os_name(&self) -> &'static str;
    fn os_version(&self) -> Result<SemanticVersion>;
    fn app_version(&self) -> Result<SemanticVersion>;
    fn app_path(&self) -> Result<PathBuf>;
    fn local_timezone(&self) -> UtcOffset;
    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf>;

    fn set_cursor_style(&self, style: CursorStyle);
    fn should_auto_hide_scrollbars(&self) -> bool;

    fn write_to_clipboard(&self, item: ClipboardItem);
    fn read_from_clipboard(&self) -> Option<ClipboardItem>;

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Result<()>;
    fn read_credentials(&self, url: &str) -> Result<Option<(String, Vec<u8>)>>;
    fn delete_credentials(&self, url: &str) -> Result<()>;
}

pub trait PlatformDisplay: Send + Sync + Debug {
    fn id(&self) -> DisplayId;
    /// Returns a stable identifier for this display that can be persisted and used
    /// across system restarts.
    fn uuid(&self) -> Result<Uuid>;
    fn as_any(&self) -> &dyn Any;
    fn bounds(&self) -> Bounds<GlobalPixels>;
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct DisplayId(pub(crate) u32);

impl Debug for DisplayId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DisplayId({})", self.0)
    }
}

unsafe impl Send for DisplayId {}

pub trait PlatformWindow {
    fn bounds(&self) -> WindowBounds;
    fn content_size(&self) -> Size<Pixels>;
    fn scale_factor(&self) -> f32;
    fn titlebar_height(&self) -> Pixels;
    fn appearance(&self) -> WindowAppearance;
    fn display(&self) -> Rc<dyn PlatformDisplay>;
    fn mouse_position(&self) -> Point<Pixels>;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn set_input_handler(&mut self, input_handler: Box<dyn PlatformInputHandler>);
    fn prompt(&self, level: PromptLevel, msg: &str, answers: &[&str]) -> oneshot::Receiver<usize>;
    fn activate(&self);
    fn set_title(&mut self, title: &str);
    fn set_edited(&mut self, edited: bool);
    fn show_character_palette(&self);
    fn minimize(&self);
    fn zoom(&self);
    fn toggle_full_screen(&self);
    fn on_input(&self, callback: Box<dyn FnMut(InputEvent) -> bool>);
    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>);
    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>);
    fn on_fullscreen(&self, callback: Box<dyn FnMut(bool)>);
    fn on_moved(&self, callback: Box<dyn FnMut()>);
    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>);
    fn on_close(&self, callback: Box<dyn FnOnce()>);
    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>);
    fn is_topmost_for_position(&self, position: Point<Pixels>) -> bool;
    fn draw(&self, scene: Scene);

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas>;

    #[cfg(any(test, feature = "test-support"))]
    fn as_test(&mut self) -> Option<&mut TestWindow> {
        None
    }
}

pub trait PlatformDispatcher: Send + Sync {
    fn is_main_thread(&self) -> bool;
    fn dispatch(&self, runnable: Runnable, label: Option<TaskLabel>);
    fn dispatch_on_main_thread(&self, runnable: Runnable);
    fn dispatch_after(&self, duration: Duration, runnable: Runnable);
    fn tick(&self, background_only: bool) -> bool;
    fn park(&self);
    fn unparker(&self) -> Unparker;

    #[cfg(any(test, feature = "test-support"))]
    fn as_test(&self) -> Option<&TestDispatcher> {
        None
    }
}

pub trait PlatformTextSystem: Send + Sync {
    fn add_fonts(&self, fonts: &[Arc<Vec<u8>>]) -> Result<()>;
    fn all_font_families(&self) -> Vec<String>;
    fn font_id(&self, descriptor: &Font) -> Result<FontId>;
    fn font_metrics(&self, font_id: FontId) -> FontMetrics;
    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>>;
    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>>;
    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId>;
    fn glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>>;
    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)>;
    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout;
    fn wrap_line(
        &self,
        text: &str,
        font_id: FontId,
        font_size: Pixels,
        width: Pixels,
    ) -> Vec<usize>;
}

#[derive(Clone, Debug)]
pub struct AppMetadata {
    pub os_name: &'static str,
    pub os_version: Option<SemanticVersion>,
    pub app_version: Option<SemanticVersion>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum AtlasKey {
    Glyph(RenderGlyphParams),
    Svg(RenderSvgParams),
    Image(RenderImageParams),
}

impl AtlasKey {
    pub(crate) fn texture_kind(&self) -> AtlasTextureKind {
        match self {
            AtlasKey::Glyph(params) => {
                if params.is_emoji {
                    AtlasTextureKind::Polychrome
                } else {
                    AtlasTextureKind::Monochrome
                }
            }
            AtlasKey::Svg(_) => AtlasTextureKind::Monochrome,
            AtlasKey::Image(_) => AtlasTextureKind::Polychrome,
        }
    }
}

impl From<RenderGlyphParams> for AtlasKey {
    fn from(params: RenderGlyphParams) -> Self {
        Self::Glyph(params)
    }
}

impl From<RenderSvgParams> for AtlasKey {
    fn from(params: RenderSvgParams) -> Self {
        Self::Svg(params)
    }
}

impl From<RenderImageParams> for AtlasKey {
    fn from(params: RenderImageParams) -> Self {
        Self::Image(params)
    }
}

pub trait PlatformAtlas: Send + Sync {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> Result<(Size<DevicePixels>, Cow<'a, [u8]>)>,
    ) -> Result<AtlasTile>;

    fn clear(&self);
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct AtlasTile {
    pub(crate) texture_id: AtlasTextureId,
    pub(crate) tile_id: TileId,
    pub(crate) bounds: Bounds<DevicePixels>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub(crate) struct AtlasTextureId {
    // We use u32 instead of usize for Metal Shader Language compatibility
    pub(crate) index: u32,
    pub(crate) kind: AtlasTextureKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub(crate) enum AtlasTextureKind {
    Monochrome = 0,
    Polychrome = 1,
    Path = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub(crate) struct TileId(pub(crate) u32);

impl From<etagere::AllocId> for TileId {
    fn from(id: etagere::AllocId) -> Self {
        Self(id.serialize())
    }
}

impl From<TileId> for etagere::AllocId {
    fn from(id: TileId) -> Self {
        Self::deserialize(id.0)
    }
}

pub trait PlatformInputHandler: 'static {
    fn selected_text_range(&mut self) -> Option<Range<usize>>;
    fn marked_text_range(&mut self) -> Option<Range<usize>>;
    fn text_for_range(&mut self, range_utf16: Range<usize>) -> Option<String>;
    fn replace_text_in_range(&mut self, replacement_range: Option<Range<usize>>, text: &str);
    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
    );
    fn unmark_text(&mut self);
    fn bounds_for_range(&mut self, range_utf16: Range<usize>) -> Option<Bounds<Pixels>>;
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
    pub display_id: Option<DisplayId>,
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
            display_id: None,
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
    Fixed(Bounds<GlobalPixels>),
}

impl StaticColumnCount for WindowBounds {
    fn column_count() -> usize {
        5
    }
}

impl Bind for WindowBounds {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let (region, next_index) = match self {
            WindowBounds::Fullscreen => {
                let next_index = statement.bind(&"Fullscreen", start_index)?;
                (None, next_index)
            }
            WindowBounds::Maximized => {
                let next_index = statement.bind(&"Maximized", start_index)?;
                (None, next_index)
            }
            WindowBounds::Fixed(region) => {
                let next_index = statement.bind(&"Fixed", start_index)?;
                (Some(*region), next_index)
            }
        };

        statement.bind(
            &region.map(|region| {
                (
                    region.origin.x,
                    region.origin.y,
                    region.size.width,
                    region.size.height,
                )
            }),
            next_index,
        )
    }
}

impl Column for WindowBounds {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (window_state, next_index) = String::column(statement, start_index)?;
        let bounds = match window_state.as_str() {
            "Fullscreen" => WindowBounds::Fullscreen,
            "Maximized" => WindowBounds::Maximized,
            "Fixed" => {
                let ((x, y, width, height), _) = Column::column(statement, next_index)?;
                let x: f64 = x;
                let y: f64 = y;
                let width: f64 = width;
                let height: f64 = height;
                WindowBounds::Fixed(Bounds {
                    origin: point(x.into(), y.into()),
                    size: size(width.into(), height.into()),
                })
            }
            _ => bail!("Window State did not have a valid string"),
        };

        Ok((bounds, next_index + 4))
    }
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

#[derive(Copy, Clone, Debug)]
pub struct PathPromptOptions {
    pub files: bool,
    pub directories: bool,
    pub multiple: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum PromptLevel {
    Info,
    Warning,
    Critical,
}

/// The style of the cursor (pointer)
#[derive(Copy, Clone, Debug)]
pub enum CursorStyle {
    Arrow,
    IBeam,
    Crosshair,
    ClosedHand,
    OpenHand,
    PointingHand,
    ResizeLeft,
    ResizeRight,
    ResizeLeftRight,
    ResizeUp,
    ResizeDown,
    ResizeUpDown,
    DisappearingItem,
    IBeamCursorForVerticalLayout,
    OperationNotAllowed,
    DragLink,
    DragCopy,
    ContextualMenu,
}

impl Default for CursorStyle {
    fn default() -> Self {
        Self::Arrow
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticVersion {
    major: usize,
    minor: usize,
    patch: usize,
}

impl FromStr for SemanticVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut components = s.trim().split('.');
        let major = components
            .next()
            .ok_or_else(|| anyhow!("missing major version number"))?
            .parse()?;
        let minor = components
            .next()
            .ok_or_else(|| anyhow!("missing minor version number"))?
            .parse()?;
        let patch = components
            .next()
            .ok_or_else(|| anyhow!("missing patch version number"))?
            .parse()?;
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl Display for SemanticVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClipboardItem {
    pub(crate) text: String,
    pub(crate) metadata: Option<String>,
}

impl ClipboardItem {
    pub fn new(text: String) -> Self {
        Self {
            text,
            metadata: None,
        }
    }

    pub fn with_metadata<T: Serialize>(mut self, metadata: T) -> Self {
        self.metadata = Some(serde_json::to_string(&metadata).unwrap());
        self
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn metadata<T>(&self) -> Option<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        self.metadata
            .as_ref()
            .and_then(|m| serde_json::from_str(m).ok())
    }

    pub(crate) fn text_hash(text: &str) -> u64 {
        let mut hasher = SeaHasher::new();
        text.hash(&mut hasher);
        hasher.finish()
    }
}
