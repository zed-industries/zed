mod test;
use super::{AnyWindowHandle, Bounds, Point};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub trait Platform {
    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow>;
}

pub trait PlatformWindow: HasRawWindowHandle + HasRawDisplayHandle {}

#[derive(Debug)]
pub struct WindowOptions<'a> {
    pub bounds: WindowBounds,
    pub titlebar: Option<TitlebarOptions<'a>>,
    pub center: bool,
    pub focus: bool,
    pub show: bool,
    pub kind: WindowKind,
    pub is_movable: bool,
}

#[derive(Debug, Default)]
pub struct TitlebarOptions<'a> {
    pub title: Option<&'a str>,
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WindowBounds {
    Fullscreen,
    Maximized,
    Fixed(Bounds<f32>),
}
