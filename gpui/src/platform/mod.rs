mod event;
#[cfg(target_os = "macos")]
pub mod mac;
pub mod current {
    #[cfg(target_os = "macos")]
    pub use super::mac::*;
}

use crate::{
    executor,
    geometry::{rect::RectF, vector::Vector2F},
    Scene,
};
use anyhow::Result;
use async_task::Runnable;
pub use event::Event;
use std::{path::PathBuf, rc::Rc, sync::Arc};

pub trait Runner {
    fn on_finish_launching<F: 'static + FnOnce()>(self, callback: F) -> Self where;
    fn on_become_active<F: 'static + FnMut()>(self, callback: F) -> Self;
    fn on_resign_active<F: 'static + FnMut()>(self, callback: F) -> Self;
    fn on_event<F: 'static + FnMut(Event) -> bool>(self, callback: F) -> Self;
    fn on_open_files<F: 'static + FnMut(Vec<PathBuf>)>(self, callback: F) -> Self;
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
