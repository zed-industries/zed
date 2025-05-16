mod app_menu;
mod keyboard;
mod keystroke;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod linux;

#[cfg(target_os = "macos")]
mod mac;

#[cfg(any(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        any(feature = "x11", feature = "wayland")
    ),
    target_os = "windows",
    feature = "macos-blade"
))]
mod blade;

#[cfg(any(test, feature = "test-support"))]
mod test;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(all(
    any(target_os = "linux", target_os = "freebsd"),
    any(feature = "wayland", feature = "x11"),
))]
pub(crate) mod scap_screen_capture;

use crate::{
    Action, AnyWindowHandle, App, AsyncWindowContext, BackgroundExecutor, Bounds,
    DEFAULT_WINDOW_SIZE, DevicePixels, DispatchEventResult, Font, FontId, FontMetrics, FontRun,
    ForegroundExecutor, GlyphId, GpuSpecs, ImageSource, Keymap, LineLayout, Pixels, PlatformInput,
    Point, RenderGlyphParams, RenderImage, RenderImageParams, RenderSvgParams, ScaledPixels, Scene,
    ShapedGlyph, ShapedRun, SharedString, Size, SvgRenderer, SvgSize, Task, TaskLabel, Window,
    hash, point, px, size,
};
use anyhow::Result;
use async_task::Runnable;
use futures::channel::oneshot;
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder as _, Frame};
use parking::Unparker;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use seahash::SeaHasher;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::io::Cursor;
use std::ops;
use std::time::{Duration, Instant};
use std::{
    fmt::{self, Debug},
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use strum::EnumIter;
use uuid::Uuid;

pub use app_menu::*;
pub use keyboard::*;
pub use keystroke::*;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub(crate) use linux::*;
#[cfg(target_os = "macos")]
pub(crate) use mac::*;
pub use semantic_version::SemanticVersion;
#[cfg(any(test, feature = "test-support"))]
pub(crate) use test::*;
#[cfg(target_os = "windows")]
pub(crate) use windows::*;

#[cfg(any(test, feature = "test-support"))]
pub use test::{TestDispatcher, TestScreenCaptureSource};

/// Returns a background executor for the current platform.
pub fn background_executor() -> BackgroundExecutor {
    current_platform(true).background_executor()
}

#[cfg(target_os = "macos")]
pub(crate) fn current_platform(headless: bool) -> Rc<dyn Platform> {
    Rc::new(MacPlatform::new(headless))
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub(crate) fn current_platform(headless: bool) -> Rc<dyn Platform> {
    if headless {
        return Rc::new(HeadlessClient::new());
    }

    match guess_compositor() {
        #[cfg(feature = "wayland")]
        "Wayland" => Rc::new(WaylandClient::new()),

        #[cfg(feature = "x11")]
        "X11" => Rc::new(X11Client::new()),

        "Headless" => Rc::new(HeadlessClient::new()),
        _ => unreachable!(),
    }
}

/// Return which compositor we're guessing we'll use.
/// Does not attempt to connect to the given compositor
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
#[inline]
pub fn guess_compositor() -> &'static str {
    if std::env::var_os("ZED_HEADLESS").is_some() {
        return "Headless";
    }

    #[cfg(feature = "wayland")]
    let wayland_display = std::env::var_os("WAYLAND_DISPLAY");
    #[cfg(not(feature = "wayland"))]
    let wayland_display: Option<std::ffi::OsString> = None;

    #[cfg(feature = "x11")]
    let x11_display = std::env::var_os("DISPLAY");
    #[cfg(not(feature = "x11"))]
    let x11_display: Option<std::ffi::OsString> = None;

    let use_wayland = wayland_display.is_some_and(|display| !display.is_empty());
    let use_x11 = x11_display.is_some_and(|display| !display.is_empty());

    if use_wayland {
        "Wayland"
    } else if use_x11 {
        "X11"
    } else {
        "Headless"
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn current_platform(_headless: bool) -> Rc<dyn Platform> {
    Rc::new(WindowsPlatform::new())
}

pub(crate) trait Platform: 'static {
    fn background_executor(&self) -> BackgroundExecutor;
    fn foreground_executor(&self) -> ForegroundExecutor;
    fn text_system(&self) -> Arc<dyn PlatformTextSystem>;

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>);
    fn quit(&self);
    fn restart(&self, binary_path: Option<PathBuf>);
    fn activate(&self, ignoring_other_apps: bool);
    fn hide(&self);
    fn hide_other_apps(&self);
    fn unhide_other_apps(&self);

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>>;
    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>>;
    fn active_window(&self) -> Option<AnyWindowHandle>;
    fn window_stack(&self) -> Option<Vec<AnyWindowHandle>> {
        None
    }

    fn is_screen_capture_supported(&self) -> bool;
    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<Result<Vec<Box<dyn ScreenCaptureSource>>>>;

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>>;

    /// Returns the appearance of the application's windows.
    fn window_appearance(&self) -> WindowAppearance;

    fn open_url(&self, url: &str);
    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>);
    fn register_url_scheme(&self, url: &str) -> Task<Result<()>>;

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>>;
    fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Result<Option<PathBuf>>>;
    fn can_select_mixed_files_and_dirs(&self) -> bool;
    fn reveal_path(&self, path: &Path);
    fn open_with_system(&self, path: &Path);

    fn on_quit(&self, callback: Box<dyn FnMut()>);
    fn on_reopen(&self, callback: Box<dyn FnMut()>);
    fn on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>);

    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap);
    fn get_menus(&self) -> Option<Vec<OwnedMenu>> {
        None
    }

    fn set_dock_menu(&self, menu: Vec<MenuItem>, keymap: &Keymap);
    fn perform_dock_menu_action(&self, _action: usize) {}
    fn add_recent_document(&self, _path: &Path) {}
    fn update_jump_list(
        &self,
        _menus: Vec<MenuItem>,
        _entries: Vec<SmallVec<[PathBuf; 2]>>,
    ) -> Vec<SmallVec<[PathBuf; 2]>> {
        Vec::new()
    }
    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>);
    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>);
    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>);
    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout>;

    fn compositor_name(&self) -> &'static str {
        ""
    }
    fn app_path(&self) -> Result<PathBuf>;
    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf>;

    fn set_cursor_style(&self, style: CursorStyle);
    fn should_auto_hide_scrollbars(&self) -> bool;

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn write_to_primary(&self, item: ClipboardItem);
    fn write_to_clipboard(&self, item: ClipboardItem);
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn read_from_primary(&self) -> Option<ClipboardItem>;
    fn read_from_clipboard(&self) -> Option<ClipboardItem>;

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>>;
    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>>;
    fn delete_credentials(&self, url: &str) -> Task<Result<()>>;
}

/// A handle to a platform's display, e.g. a monitor or laptop screen.
pub trait PlatformDisplay: Send + Sync + Debug {
    /// Get the ID for this display
    fn id(&self) -> DisplayId;

    /// Returns a stable identifier for this display that can be persisted and used
    /// across system restarts.
    fn uuid(&self) -> Result<Uuid>;

    /// Get the bounds for this display
    fn bounds(&self) -> Bounds<Pixels>;

    /// Get the default bounds for this display to place a window
    fn default_bounds(&self) -> Bounds<Pixels> {
        let center = self.bounds().center();
        let offset = DEFAULT_WINDOW_SIZE / 2.0;
        let origin = point(center.x - offset.width, center.y - offset.height);
        Bounds::new(origin, DEFAULT_WINDOW_SIZE)
    }
}

/// A source of on-screen video content that can be captured.
pub trait ScreenCaptureSource {
    /// Returns the video resolution of this source.
    fn resolution(&self) -> Result<Size<DevicePixels>>;

    /// Start capture video from this source, invoking the given callback
    /// with each frame.
    fn stream(
        &self,
        foreground_executor: &ForegroundExecutor,
        frame_callback: Box<dyn Fn(ScreenCaptureFrame) + Send>,
    ) -> oneshot::Receiver<Result<Box<dyn ScreenCaptureStream>>>;
}

/// A video stream captured from a screen.
pub trait ScreenCaptureStream {}

/// A frame of video captured from a screen.
pub struct ScreenCaptureFrame(pub PlatformScreenCaptureFrame);

/// An opaque identifier for a hardware display
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct DisplayId(pub(crate) u32);

impl From<DisplayId> for u32 {
    fn from(id: DisplayId) -> Self {
        id.0
    }
}

impl Debug for DisplayId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DisplayId({})", self.0)
    }
}

unsafe impl Send for DisplayId {}

/// Which part of the window to resize
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeEdge {
    /// The top edge
    Top,
    /// The top right corner
    TopRight,
    /// The right edge
    Right,
    /// The bottom right corner
    BottomRight,
    /// The bottom edge
    Bottom,
    /// The bottom left corner
    BottomLeft,
    /// The left edge
    Left,
    /// The top left corner
    TopLeft,
}

/// A type to describe the appearance of a window
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum WindowDecorations {
    #[default]
    /// Server side decorations
    Server,
    /// Client side decorations
    Client,
}

/// A type to describe how this window is currently configured
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum Decorations {
    /// The window is configured to use server side decorations
    #[default]
    Server,
    /// The window is configured to use client side decorations
    Client {
        /// The edge tiling state
        tiling: Tiling,
    },
}

/// What window controls this platform supports
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowControls {
    /// Whether this platform supports fullscreen
    pub fullscreen: bool,
    /// Whether this platform supports maximize
    pub maximize: bool,
    /// Whether this platform supports minimize
    pub minimize: bool,
    /// Whether this platform supports a window menu
    pub window_menu: bool,
}

impl Default for WindowControls {
    fn default() -> Self {
        // Assume that we can do anything, unless told otherwise
        Self {
            fullscreen: true,
            maximize: true,
            minimize: true,
            window_menu: true,
        }
    }
}

/// A type to describe which sides of the window are currently tiled in some way
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Tiling {
    /// Whether the top edge is tiled
    pub top: bool,
    /// Whether the left edge is tiled
    pub left: bool,
    /// Whether the right edge is tiled
    pub right: bool,
    /// Whether the bottom edge is tiled
    pub bottom: bool,
}

impl Tiling {
    /// Initializes a [`Tiling`] type with all sides tiled
    pub fn tiled() -> Self {
        Self {
            top: true,
            left: true,
            right: true,
            bottom: true,
        }
    }

    /// Whether any edge is tiled
    pub fn is_tiled(&self) -> bool {
        self.top || self.left || self.right || self.bottom
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub(crate) struct RequestFrameOptions {
    pub(crate) require_presentation: bool,
}

pub(crate) trait PlatformWindow: HasWindowHandle + HasDisplayHandle {
    fn bounds(&self) -> Bounds<Pixels>;
    fn is_maximized(&self) -> bool;
    fn window_bounds(&self) -> WindowBounds;
    fn content_size(&self) -> Size<Pixels>;
    fn resize(&mut self, size: Size<Pixels>);
    fn scale_factor(&self) -> f32;
    fn appearance(&self) -> WindowAppearance;
    fn display(&self) -> Option<Rc<dyn PlatformDisplay>>;
    fn mouse_position(&self) -> Point<Pixels>;
    fn modifiers(&self) -> Modifiers;
    fn set_input_handler(&mut self, input_handler: PlatformInputHandler);
    fn take_input_handler(&mut self) -> Option<PlatformInputHandler>;
    fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[&str],
    ) -> Option<oneshot::Receiver<usize>>;
    fn activate(&self);
    fn is_active(&self) -> bool;
    fn is_hovered(&self) -> bool;
    fn set_title(&mut self, title: &str);
    fn set_background_appearance(&self, background_appearance: WindowBackgroundAppearance);
    fn minimize(&self);
    fn zoom(&self);
    fn toggle_fullscreen(&self);
    fn is_fullscreen(&self) -> bool;
    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>);
    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>);
    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>);
    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>);
    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>);
    fn on_moved(&self, callback: Box<dyn FnMut()>);
    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>);
    fn on_close(&self, callback: Box<dyn FnOnce()>);
    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>);
    fn on_accesskit_action(&self, callback: Box<dyn FnMut(accesskit::ActionRequest)>);
    fn draw(&self, scene: &Scene);
    fn completed_frame(&self) {}
    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas>;

    // macOS specific methods
    fn set_edited(&mut self, _edited: bool) {}
    fn show_character_palette(&self) {}

    #[cfg(target_os = "windows")]
    fn get_raw_handle(&self) -> windows::HWND;

    // Linux specific methods
    fn inner_window_bounds(&self) -> WindowBounds {
        self.window_bounds()
    }
    fn request_decorations(&self, _decorations: WindowDecorations) {}
    fn show_window_menu(&self, _position: Point<Pixels>) {}
    fn start_window_move(&self) {}
    fn start_window_resize(&self, _edge: ResizeEdge) {}
    fn window_decorations(&self) -> Decorations {
        Decorations::Server
    }
    fn set_app_id(&mut self, _app_id: &str) {}
    fn map_window(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn window_controls(&self) -> WindowControls {
        WindowControls::default()
    }
    fn set_client_inset(&self, _inset: Pixels) {}
    fn gpu_specs(&self) -> Option<GpuSpecs>;

    fn update_ime_position(&self, _bounds: Bounds<ScaledPixels>);

    #[cfg(any(test, feature = "test-support"))]
    fn as_test(&mut self) -> Option<&mut TestWindow> {
        None
    }
}

/// This type is public so that our test macro can generate and use it, but it should not
/// be considered part of our public API.
#[doc(hidden)]
pub trait PlatformDispatcher: Send + Sync {
    fn is_main_thread(&self) -> bool;
    fn dispatch(&self, runnable: Runnable, label: Option<TaskLabel>);
    fn dispatch_on_main_thread(&self, runnable: Runnable);
    fn dispatch_after(&self, duration: Duration, runnable: Runnable);
    fn park(&self, timeout: Option<Duration>) -> bool;
    fn unparker(&self) -> Unparker;
    fn now(&self) -> Instant {
        Instant::now()
    }

    #[cfg(any(test, feature = "test-support"))]
    fn as_test(&self) -> Option<&TestDispatcher> {
        None
    }
}

pub(crate) trait PlatformTextSystem: Send + Sync {
    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()>;
    fn all_font_names(&self) -> Vec<String>;
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
}

pub(crate) struct NoopTextSystem;

impl NoopTextSystem {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }
}

impl PlatformTextSystem for NoopTextSystem {
    fn add_fonts(&self, _fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        Ok(())
    }

    fn all_font_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn font_id(&self, _descriptor: &Font) -> Result<FontId> {
        return Ok(FontId(1));
    }

    fn font_metrics(&self, _font_id: FontId) -> FontMetrics {
        FontMetrics {
            units_per_em: 1000,
            ascent: 1025.0,
            descent: -275.0,
            line_gap: 0.0,
            underline_position: -95.0,
            underline_thickness: 60.0,
            cap_height: 698.0,
            x_height: 516.0,
            bounding_box: Bounds {
                origin: Point {
                    x: -260.0,
                    y: -245.0,
                },
                size: Size {
                    width: 1501.0,
                    height: 1364.0,
                },
            },
        }
    }

    fn typographic_bounds(&self, _font_id: FontId, _glyph_id: GlyphId) -> Result<Bounds<f32>> {
        Ok(Bounds {
            origin: Point { x: 54.0, y: 0.0 },
            size: size(392.0, 528.0),
        })
    }

    fn advance(&self, _font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        Ok(size(600.0 * glyph_id.0 as f32, 0.0))
    }

    fn glyph_for_char(&self, _font_id: FontId, ch: char) -> Option<GlyphId> {
        Some(GlyphId(ch.len_utf16() as u32))
    }

    fn glyph_raster_bounds(&self, _params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        Ok(Default::default())
    }

    fn rasterize_glyph(
        &self,
        _params: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        Ok((raster_bounds.size, Vec::new()))
    }

    fn layout_line(&self, text: &str, font_size: Pixels, _runs: &[FontRun]) -> LineLayout {
        let mut position = px(0.);
        let metrics = self.font_metrics(FontId(0));
        let em_width = font_size
            * self
                .advance(FontId(0), self.glyph_for_char(FontId(0), 'm').unwrap())
                .unwrap()
                .width
            / metrics.units_per_em as f32;
        let mut glyphs = SmallVec::default();
        for (ix, c) in text.char_indices() {
            if let Some(glyph) = self.glyph_for_char(FontId(0), c) {
                glyphs.push(ShapedGlyph {
                    id: glyph,
                    position: point(position, px(0.)),
                    index: ix,
                    is_emoji: glyph.0 == 2,
                });
                if glyph.0 == 2 {
                    position += em_width * 2.0;
                } else {
                    position += em_width;
                }
            } else {
                position += em_width
            }
        }
        let mut runs = Vec::default();
        if glyphs.len() > 0 {
            runs.push(ShapedRun {
                font_id: FontId(0),
                glyphs,
            });
        } else {
            position = px(0.);
        }

        LineLayout {
            font_size,
            width: position,
            ascent: font_size * (metrics.ascent / metrics.units_per_em as f32),
            descent: font_size * (metrics.descent / metrics.units_per_em as f32),
            runs,
            len: text.len(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub(crate) enum AtlasKey {
    Glyph(RenderGlyphParams),
    Svg(RenderSvgParams),
    Image(RenderImageParams),
}

impl AtlasKey {
    #[cfg_attr(
        all(
            any(target_os = "linux", target_os = "freebsd"),
            not(any(feature = "x11", feature = "wayland"))
        ),
        allow(dead_code)
    )]
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

pub(crate) trait PlatformAtlas: Send + Sync {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> Result<Option<(Size<DevicePixels>, Cow<'a, [u8]>)>>,
    ) -> Result<Option<AtlasTile>>;
    fn remove(&self, key: &AtlasKey);
}

struct AtlasTextureList<T> {
    textures: Vec<Option<T>>,
    free_list: Vec<usize>,
}

impl<T> Default for AtlasTextureList<T> {
    fn default() -> Self {
        Self {
            textures: Vec::default(),
            free_list: Vec::default(),
        }
    }
}

impl<T> ops::Index<usize> for AtlasTextureList<T> {
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.textures[index]
    }
}

impl<T> AtlasTextureList<T> {
    #[allow(unused)]
    fn drain(&mut self) -> std::vec::Drain<Option<T>> {
        self.free_list.clear();
        self.textures.drain(..)
    }

    #[allow(dead_code)]
    fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> {
        self.textures.iter_mut().flatten()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub(crate) struct AtlasTile {
    pub(crate) texture_id: AtlasTextureId,
    pub(crate) tile_id: TileId,
    pub(crate) padding: u32,
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
#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
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

pub(crate) struct PlatformInputHandler {
    cx: AsyncWindowContext,
    handler: Box<dyn InputHandler>,
}

#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
impl PlatformInputHandler {
    pub fn new(cx: AsyncWindowContext, handler: Box<dyn InputHandler>) -> Self {
        Self { cx, handler }
    }

    fn selected_text_range(&mut self, ignore_disabled_input: bool) -> Option<UTF16Selection> {
        self.cx
            .update(|window, cx| {
                self.handler
                    .selected_text_range(ignore_disabled_input, window, cx)
            })
            .ok()
            .flatten()
    }

    #[cfg_attr(target_os = "windows", allow(dead_code))]
    fn marked_text_range(&mut self) -> Option<Range<usize>> {
        self.cx
            .update(|window, cx| self.handler.marked_text_range(window, cx))
            .ok()
            .flatten()
    }

    #[cfg_attr(
        any(target_os = "linux", target_os = "freebsd", target_os = "windows"),
        allow(dead_code)
    )]
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted: &mut Option<Range<usize>>,
    ) -> Option<String> {
        self.cx
            .update(|window, cx| {
                self.handler
                    .text_for_range(range_utf16, adjusted, window, cx)
            })
            .ok()
            .flatten()
    }

    fn replace_text_in_range(&mut self, replacement_range: Option<Range<usize>>, text: &str) {
        self.cx
            .update(|window, cx| {
                self.handler
                    .replace_text_in_range(replacement_range, text, window, cx);
            })
            .ok();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
    ) {
        self.cx
            .update(|window, cx| {
                self.handler.replace_and_mark_text_in_range(
                    range_utf16,
                    new_text,
                    new_selected_range,
                    window,
                    cx,
                )
            })
            .ok();
    }

    #[cfg_attr(target_os = "windows", allow(dead_code))]
    fn unmark_text(&mut self) {
        self.cx
            .update(|window, cx| self.handler.unmark_text(window, cx))
            .ok();
    }

    fn bounds_for_range(&mut self, range_utf16: Range<usize>) -> Option<Bounds<Pixels>> {
        self.cx
            .update(|window, cx| self.handler.bounds_for_range(range_utf16, window, cx))
            .ok()
            .flatten()
    }

    #[allow(dead_code)]
    fn apple_press_and_hold_enabled(&mut self) -> bool {
        self.handler.apple_press_and_hold_enabled()
    }

    pub(crate) fn dispatch_input(&mut self, input: &str, window: &mut Window, cx: &mut App) {
        self.handler.replace_text_in_range(None, input, window, cx);
    }

    pub fn selected_bounds(&mut self, window: &mut Window, cx: &mut App) -> Option<Bounds<Pixels>> {
        let selection = self.handler.selected_text_range(true, window, cx)?;
        self.handler.bounds_for_range(
            if selection.reversed {
                selection.range.start..selection.range.start
            } else {
                selection.range.end..selection.range.end
            },
            window,
            cx,
        )
    }

    #[allow(unused)]
    pub fn character_index_for_point(&mut self, point: Point<Pixels>) -> Option<usize> {
        self.cx
            .update(|window, cx| self.handler.character_index_for_point(point, window, cx))
            .ok()
            .flatten()
    }
}

/// A struct representing a selection in a text buffer, in UTF16 characters.
/// This is different from a range because the head may be before the tail.
#[derive(Debug)]
pub struct UTF16Selection {
    /// The range of text in the document this selection corresponds to
    /// in UTF16 characters.
    pub range: Range<usize>,
    /// Whether the head of this selection is at the start (true), or end (false)
    /// of the range
    pub reversed: bool,
}

/// Zed's interface for handling text input from the platform's IME system
/// This is currently a 1:1 exposure of the NSTextInputClient API:
///
/// <https://developer.apple.com/documentation/appkit/nstextinputclient>
pub trait InputHandler: 'static {
    /// Get the range of the user's currently selected text, if any
    /// Corresponds to [selectedRange()](https://developer.apple.com/documentation/appkit/nstextinputclient/1438242-selectedrange)
    ///
    /// Return value is in terms of UTF-16 characters, from 0 to the length of the document
    fn selected_text_range(
        &mut self,
        ignore_disabled_input: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<UTF16Selection>;

    /// Get the range of the currently marked text, if any
    /// Corresponds to [markedRange()](https://developer.apple.com/documentation/appkit/nstextinputclient/1438250-markedrange)
    ///
    /// Return value is in terms of UTF-16 characters, from 0 to the length of the document
    fn marked_text_range(&mut self, window: &mut Window, cx: &mut App) -> Option<Range<usize>>;

    /// Get the text for the given document range in UTF-16 characters
    /// Corresponds to [attributedSubstring(forProposedRange: actualRange:)](https://developer.apple.com/documentation/appkit/nstextinputclient/1438238-attributedsubstring)
    ///
    /// range_utf16 is in terms of UTF-16 characters
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted_range: &mut Option<Range<usize>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<String>;

    /// Replace the text in the given document range with the given text
    /// Corresponds to [insertText(_:replacementRange:)](https://developer.apple.com/documentation/appkit/nstextinputclient/1438258-inserttext)
    ///
    /// replacement_range is in terms of UTF-16 characters
    fn replace_text_in_range(
        &mut self,
        replacement_range: Option<Range<usize>>,
        text: &str,
        window: &mut Window,
        cx: &mut App,
    );

    /// Replace the text in the given document range with the given text,
    /// and mark the given text as part of an IME 'composing' state
    /// Corresponds to [setMarkedText(_:selectedRange:replacementRange:)](https://developer.apple.com/documentation/appkit/nstextinputclient/1438246-setmarkedtext)
    ///
    /// range_utf16 is in terms of UTF-16 characters
    /// new_selected_range is in terms of UTF-16 characters
    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut App,
    );

    /// Remove the IME 'composing' state from the document
    /// Corresponds to [unmarkText()](https://developer.apple.com/documentation/appkit/nstextinputclient/1438239-unmarktext)
    fn unmark_text(&mut self, window: &mut Window, cx: &mut App);

    /// Get the bounds of the given document range in screen coordinates
    /// Corresponds to [firstRect(forCharacterRange:actualRange:)](https://developer.apple.com/documentation/appkit/nstextinputclient/1438240-firstrect)
    ///
    /// This is used for positioning the IME candidate window
    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Bounds<Pixels>>;

    /// Get the character offset for the given point in terms of UTF16 characters
    ///
    /// Corresponds to [characterIndexForPoint:](https://developer.apple.com/documentation/appkit/nstextinputclient/characterindex(for:))
    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<usize>;

    /// Allows a given input context to opt into getting raw key repeats instead of
    /// sending these to the platform.
    /// TODO: Ideally we should be able to set ApplePressAndHoldEnabled in NSUserDefaults
    /// (which is how iTerm does it) but it doesn't seem to work for me.
    #[allow(dead_code)]
    fn apple_press_and_hold_enabled(&mut self) -> bool {
        true
    }
}

/// The variables that can be configured when creating a new window
#[derive(Debug)]
pub struct WindowOptions {
    /// Specifies the state and bounds of the window in screen coordinates.
    /// - `None`: Inherit the bounds.
    /// - `Some(WindowBounds)`: Open a window with corresponding state and its restore size.
    pub window_bounds: Option<WindowBounds>,

    /// The titlebar configuration of the window
    pub titlebar: Option<TitlebarOptions>,

    /// Whether the window should be focused when created
    pub focus: bool,

    /// Whether the window should be shown when created
    pub show: bool,

    /// The kind of window to create
    pub kind: WindowKind,

    /// Whether the window should be movable by the user
    pub is_movable: bool,

    /// The display to create the window on, if this is None,
    /// the window will be created on the main display
    pub display_id: Option<DisplayId>,

    /// The appearance of the window background.
    pub window_background: WindowBackgroundAppearance,

    /// Application identifier of the window. Can by used by desktop environments to group applications together.
    pub app_id: Option<String>,

    /// Window minimum size
    pub window_min_size: Option<Size<Pixels>>,

    /// Whether to use client or server side decorations. Wayland only
    /// Note that this may be ignored.
    pub window_decorations: Option<WindowDecorations>,
}

/// The variables that can be configured when creating a new window
#[derive(Debug)]
#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
pub(crate) struct WindowParams {
    pub bounds: Bounds<Pixels>,

    /// The titlebar configuration of the window
    #[cfg_attr(feature = "wayland", allow(dead_code))]
    pub titlebar: Option<TitlebarOptions>,

    /// The kind of window to create
    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
    pub kind: WindowKind,

    /// Whether the window should be movable by the user
    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
    pub is_movable: bool,

    #[cfg_attr(
        any(target_os = "linux", target_os = "freebsd", target_os = "windows"),
        allow(dead_code)
    )]
    pub focus: bool,

    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
    pub show: bool,

    #[cfg_attr(feature = "wayland", allow(dead_code))]
    pub display_id: Option<DisplayId>,

    pub window_min_size: Option<Size<Pixels>>,
}

/// Represents the status of how a window should be opened.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WindowBounds {
    /// Indicates that the window should open in a windowed state with the given bounds.
    Windowed(Bounds<Pixels>),
    /// Indicates that the window should open in a maximized state.
    /// The bounds provided here represent the restore size of the window.
    Maximized(Bounds<Pixels>),
    /// Indicates that the window should open in fullscreen mode.
    /// The bounds provided here represent the restore size of the window.
    Fullscreen(Bounds<Pixels>),
}

impl Default for WindowBounds {
    fn default() -> Self {
        WindowBounds::Windowed(Bounds::default())
    }
}

impl WindowBounds {
    /// Retrieve the inner bounds
    pub fn get_bounds(&self) -> Bounds<Pixels> {
        match self {
            WindowBounds::Windowed(bounds) => *bounds,
            WindowBounds::Maximized(bounds) => *bounds,
            WindowBounds::Fullscreen(bounds) => *bounds,
        }
    }
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            window_bounds: None,
            titlebar: Some(TitlebarOptions {
                title: Default::default(),
                appears_transparent: Default::default(),
                traffic_light_position: Default::default(),
            }),
            focus: true,
            show: true,
            kind: WindowKind::Normal,
            is_movable: true,
            display_id: None,
            window_background: WindowBackgroundAppearance::default(),
            app_id: None,
            window_min_size: None,
            window_decorations: None,
        }
    }
}

/// The options that can be configured for a window's titlebar
#[derive(Debug, Default)]
pub struct TitlebarOptions {
    /// The initial title of the window
    pub title: Option<SharedString>,

    /// Should the default system titlebar be hidden to allow for a custom-drawn titlebar? (macOS and Windows only)
    /// Refer to [`WindowOptions::window_decorations`] on Linux
    pub appears_transparent: bool,

    /// The position of the macOS traffic light buttons
    pub traffic_light_position: Option<Point<Pixels>>,
}

/// The kind of window to create
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WindowKind {
    /// A normal application window
    Normal,

    /// A window that appears above all other windows, usually used for alerts or popups
    /// use sparingly!
    PopUp,
}

/// The appearance of the window, as defined by the operating system.
///
/// On macOS, this corresponds to named [`NSAppearance`](https://developer.apple.com/documentation/appkit/nsappearance)
/// values.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WindowAppearance {
    /// A light appearance.
    ///
    /// On macOS, this corresponds to the `aqua` appearance.
    Light,

    /// A light appearance with vibrant colors.
    ///
    /// On macOS, this corresponds to the `NSAppearanceNameVibrantLight` appearance.
    VibrantLight,

    /// A dark appearance.
    ///
    /// On macOS, this corresponds to the `darkAqua` appearance.
    Dark,

    /// A dark appearance with vibrant colors.
    ///
    /// On macOS, this corresponds to the `NSAppearanceNameVibrantDark` appearance.
    VibrantDark,
}

impl Default for WindowAppearance {
    fn default() -> Self {
        Self::Light
    }
}

/// The appearance of the background of the window itself, when there is
/// no content or the content is transparent.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum WindowBackgroundAppearance {
    /// Opaque.
    ///
    /// This lets the window manager know that content behind this
    /// window does not need to be drawn.
    ///
    /// Actual color depends on the system and themes should define a fully
    /// opaque background color instead.
    #[default]
    Opaque,
    /// Plain alpha transparency.
    Transparent,
    /// Transparency, but the contents behind the window are blurred.
    ///
    /// Not always supported.
    Blurred,
}

/// The options that can be configured for a file dialog prompt
#[derive(Copy, Clone, Debug)]
pub struct PathPromptOptions {
    /// Should the prompt allow files to be selected?
    pub files: bool,
    /// Should the prompt allow directories to be selected?
    pub directories: bool,
    /// Should the prompt allow multiple files to be selected?
    pub multiple: bool,
}

/// What kind of prompt styling to show
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PromptLevel {
    /// A prompt that is shown when the user should be notified of something
    Info,

    /// A prompt that is shown when the user needs to be warned of a potential problem
    Warning,

    /// A prompt that is shown when a critical problem has occurred
    Critical,
}

/// The style of the cursor (pointer)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CursorStyle {
    /// The default cursor
    Arrow,

    /// A text input cursor
    /// corresponds to the CSS cursor value `text`
    IBeam,

    /// A crosshair cursor
    /// corresponds to the CSS cursor value `crosshair`
    Crosshair,

    /// A closed hand cursor
    /// corresponds to the CSS cursor value `grabbing`
    ClosedHand,

    /// An open hand cursor
    /// corresponds to the CSS cursor value `grab`
    OpenHand,

    /// A pointing hand cursor
    /// corresponds to the CSS cursor value `pointer`
    PointingHand,

    /// A resize left cursor
    /// corresponds to the CSS cursor value `w-resize`
    ResizeLeft,

    /// A resize right cursor
    /// corresponds to the CSS cursor value `e-resize`
    ResizeRight,

    /// A resize cursor to the left and right
    /// corresponds to the CSS cursor value `ew-resize`
    ResizeLeftRight,

    /// A resize up cursor
    /// corresponds to the CSS cursor value `n-resize`
    ResizeUp,

    /// A resize down cursor
    /// corresponds to the CSS cursor value `s-resize`
    ResizeDown,

    /// A resize cursor directing up and down
    /// corresponds to the CSS cursor value `ns-resize`
    ResizeUpDown,

    /// A resize cursor directing up-left and down-right
    /// corresponds to the CSS cursor value `nesw-resize`
    ResizeUpLeftDownRight,

    /// A resize cursor directing up-right and down-left
    /// corresponds to the CSS cursor value `nwse-resize`
    ResizeUpRightDownLeft,

    /// A cursor indicating that the item/column can be resized horizontally.
    /// corresponds to the CSS cursor value `col-resize`
    ResizeColumn,

    /// A cursor indicating that the item/row can be resized vertically.
    /// corresponds to the CSS cursor value `row-resize`
    ResizeRow,

    /// A text input cursor for vertical layout
    /// corresponds to the CSS cursor value `vertical-text`
    IBeamCursorForVerticalLayout,

    /// A cursor indicating that the operation is not allowed
    /// corresponds to the CSS cursor value `not-allowed`
    OperationNotAllowed,

    /// A cursor indicating that the operation will result in a link
    /// corresponds to the CSS cursor value `alias`
    DragLink,

    /// A cursor indicating that the operation will result in a copy
    /// corresponds to the CSS cursor value `copy`
    DragCopy,

    /// A cursor indicating that the operation will result in a context menu
    /// corresponds to the CSS cursor value `context-menu`
    ContextualMenu,

    /// Hide the cursor
    None,
}

impl Default for CursorStyle {
    fn default() -> Self {
        Self::Arrow
    }
}

/// A clipboard item that should be copied to the clipboard
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClipboardItem {
    entries: Vec<ClipboardEntry>,
}

/// Either a ClipboardString or a ClipboardImage
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClipboardEntry {
    /// A string entry
    String(ClipboardString),
    /// An image entry
    Image(Image),
}

impl ClipboardItem {
    /// Create a new ClipboardItem::String with no associated metadata
    pub fn new_string(text: String) -> Self {
        Self {
            entries: vec![ClipboardEntry::String(ClipboardString::new(text))],
        }
    }

    /// Create a new ClipboardItem::String with the given text and associated metadata
    pub fn new_string_with_metadata(text: String, metadata: String) -> Self {
        Self {
            entries: vec![ClipboardEntry::String(ClipboardString {
                text,
                metadata: Some(metadata),
            })],
        }
    }

    /// Create a new ClipboardItem::String with the given text and associated metadata
    pub fn new_string_with_json_metadata<T: Serialize>(text: String, metadata: T) -> Self {
        Self {
            entries: vec![ClipboardEntry::String(
                ClipboardString::new(text).with_json_metadata(metadata),
            )],
        }
    }

    /// Create a new ClipboardItem::Image with the given image with no associated metadata
    pub fn new_image(image: &Image) -> Self {
        Self {
            entries: vec![ClipboardEntry::Image(image.clone())],
        }
    }

    /// Concatenates together all the ClipboardString entries in the item.
    /// Returns None if there were no ClipboardString entries.
    pub fn text(&self) -> Option<String> {
        let mut answer = String::new();
        let mut any_entries = false;

        for entry in self.entries.iter() {
            if let ClipboardEntry::String(ClipboardString { text, metadata: _ }) = entry {
                answer.push_str(&text);
                any_entries = true;
            }
        }

        if any_entries { Some(answer) } else { None }
    }

    /// If this item is one ClipboardEntry::String, returns its metadata.
    #[cfg_attr(not(target_os = "windows"), allow(dead_code))]
    pub fn metadata(&self) -> Option<&String> {
        match self.entries().first() {
            Some(ClipboardEntry::String(clipboard_string)) if self.entries.len() == 1 => {
                clipboard_string.metadata.as_ref()
            }
            _ => None,
        }
    }

    /// Get the item's entries
    pub fn entries(&self) -> &[ClipboardEntry] {
        &self.entries
    }

    /// Get owned versions of the item's entries
    pub fn into_entries(self) -> impl Iterator<Item = ClipboardEntry> {
        self.entries.into_iter()
    }
}

impl From<ClipboardString> for ClipboardEntry {
    fn from(value: ClipboardString) -> Self {
        Self::String(value)
    }
}

impl From<String> for ClipboardEntry {
    fn from(value: String) -> Self {
        Self::from(ClipboardString::from(value))
    }
}

impl From<Image> for ClipboardEntry {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}

impl From<ClipboardEntry> for ClipboardItem {
    fn from(value: ClipboardEntry) -> Self {
        Self {
            entries: vec![value],
        }
    }
}

impl From<String> for ClipboardItem {
    fn from(value: String) -> Self {
        Self::from(ClipboardEntry::from(value))
    }
}

impl From<Image> for ClipboardItem {
    fn from(value: Image) -> Self {
        Self::from(ClipboardEntry::from(value))
    }
}

/// One of the editor's supported image formats (e.g. PNG, JPEG) - used when dealing with images in the clipboard
#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter, Hash)]
pub enum ImageFormat {
    // Sorted from most to least likely to be pasted into an editor,
    // which matters when we iterate through them trying to see if
    // clipboard content matches them.
    /// .png
    Png,
    /// .jpeg or .jpg
    Jpeg,
    /// .webp
    Webp,
    /// .gif
    Gif,
    /// .svg
    Svg,
    /// .bmp
    Bmp,
    /// .tif or .tiff
    Tiff,
}

impl ImageFormat {
    /// Returns the mime type for the ImageFormat
    pub const fn mime_type(self) -> &'static str {
        match self {
            ImageFormat::Png => "image/png",
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Webp => "image/webp",
            ImageFormat::Gif => "image/gif",
            ImageFormat::Svg => "image/svg+xml",
            ImageFormat::Bmp => "image/bmp",
            ImageFormat::Tiff => "image/tiff",
        }
    }

    /// Returns the ImageFormat for the given mime type
    pub fn from_mime_type(mime_type: &str) -> Option<Self> {
        match mime_type {
            "image/png" => Some(Self::Png),
            "image/jpeg" | "image/jpg" => Some(Self::Jpeg),
            "image/webp" => Some(Self::Webp),
            "image/gif" => Some(Self::Gif),
            "image/svg+xml" => Some(Self::Svg),
            "image/bmp" => Some(Self::Bmp),
            "image/tiff" | "image/tif" => Some(Self::Tiff),
            _ => None,
        }
    }
}

/// An image, with a format and certain bytes
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image {
    /// The image format the bytes represent (e.g. PNG)
    pub format: ImageFormat,
    /// The raw image bytes
    pub bytes: Vec<u8>,
    /// The unique ID for the image
    id: u64,
}

impl Hash for Image {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.id);
    }
}

impl Image {
    /// An empty image containing no data
    pub fn empty() -> Self {
        Self::from_bytes(ImageFormat::Png, Vec::new())
    }

    /// Create an image from a format and bytes
    pub fn from_bytes(format: ImageFormat, bytes: Vec<u8>) -> Self {
        Self {
            id: hash(&bytes),
            format,
            bytes,
        }
    }

    /// Get this image's ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Use the GPUI `use_asset` API to make this image renderable
    pub fn use_render_image(
        self: Arc<Self>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Arc<RenderImage>> {
        ImageSource::Image(self)
            .use_data(None, window, cx)
            .and_then(|result| result.ok())
    }

    /// Use the GPUI `get_asset` API to make this image renderable
    pub fn get_render_image(
        self: Arc<Self>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Arc<RenderImage>> {
        ImageSource::Image(self)
            .get_data(None, window, cx)
            .and_then(|result| result.ok())
    }

    /// Use the GPUI `remove_asset` API to drop this image, if possible.
    pub fn remove_asset(self: Arc<Self>, cx: &mut App) {
        ImageSource::Image(self).remove_asset(cx);
    }

    /// Convert the clipboard image to an `ImageData` object.
    pub fn to_image_data(&self, svg_renderer: SvgRenderer) -> Result<Arc<RenderImage>> {
        fn frames_for_image(
            bytes: &[u8],
            format: image::ImageFormat,
        ) -> Result<SmallVec<[Frame; 1]>> {
            let mut data = image::load_from_memory_with_format(bytes, format)?.into_rgba8();

            // Convert from RGBA to BGRA.
            for pixel in data.chunks_exact_mut(4) {
                pixel.swap(0, 2);
            }

            Ok(SmallVec::from_elem(Frame::new(data), 1))
        }

        let frames = match self.format {
            ImageFormat::Gif => {
                let decoder = GifDecoder::new(Cursor::new(&self.bytes))?;
                let mut frames = SmallVec::new();

                for frame in decoder.into_frames() {
                    let mut frame = frame?;
                    // Convert from RGBA to BGRA.
                    for pixel in frame.buffer_mut().chunks_exact_mut(4) {
                        pixel.swap(0, 2);
                    }
                    frames.push(frame);
                }

                frames
            }
            ImageFormat::Png => frames_for_image(&self.bytes, image::ImageFormat::Png)?,
            ImageFormat::Jpeg => frames_for_image(&self.bytes, image::ImageFormat::Jpeg)?,
            ImageFormat::Webp => frames_for_image(&self.bytes, image::ImageFormat::WebP)?,
            ImageFormat::Bmp => frames_for_image(&self.bytes, image::ImageFormat::Bmp)?,
            ImageFormat::Tiff => frames_for_image(&self.bytes, image::ImageFormat::Tiff)?,
            ImageFormat::Svg => {
                let pixmap = svg_renderer.render_pixmap(&self.bytes, SvgSize::ScaleFactor(1.0))?;

                let buffer =
                    image::ImageBuffer::from_raw(pixmap.width(), pixmap.height(), pixmap.take())
                        .unwrap();

                SmallVec::from_elem(Frame::new(buffer), 1)
            }
        };

        Ok(Arc::new(RenderImage::new(frames)))
    }

    /// Get the format of the clipboard image
    pub fn format(&self) -> ImageFormat {
        self.format
    }

    /// Get the raw bytes of the clipboard image
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

/// A clipboard item that should be copied to the clipboard
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClipboardString {
    pub(crate) text: String,
    pub(crate) metadata: Option<String>,
}

impl ClipboardString {
    /// Create a new clipboard string with the given text
    pub fn new(text: String) -> Self {
        Self {
            text,
            metadata: None,
        }
    }

    /// Return a new clipboard item with the metadata replaced by the given metadata,
    /// after serializing it as JSON.
    pub fn with_json_metadata<T: Serialize>(mut self, metadata: T) -> Self {
        self.metadata = Some(serde_json::to_string(&metadata).unwrap());
        self
    }

    /// Get the text of the clipboard string
    pub fn text(&self) -> &String {
        &self.text
    }

    /// Get the owned text of the clipboard string
    pub fn into_text(self) -> String {
        self.text
    }

    /// Get the metadata of the clipboard string, formatted as JSON
    pub fn metadata_json<T>(&self) -> Option<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        self.metadata
            .as_ref()
            .and_then(|m| serde_json::from_str(m).ok())
    }

    #[cfg_attr(any(target_os = "linux", target_os = "freebsd"), allow(dead_code))]
    pub(crate) fn text_hash(text: &str) -> u64 {
        let mut hasher = SeaHasher::new();
        text.hash(&mut hasher);
        hasher.finish()
    }
}

impl From<String> for ClipboardString {
    fn from(value: String) -> Self {
        Self {
            text: value,
            metadata: None,
        }
    }
}
