mod event;
#[cfg(target_os = "macos")]
pub mod mac;
pub mod current {
    #[cfg(target_os = "macos")]
    pub use super::mac::*;
}

use std::path::PathBuf;

use event::Event;

pub trait App {
    fn on_finish_launching<F: 'static + FnOnce()>(self, callback: F) -> Self where;
    fn on_become_active<F: 'static + FnMut()>(self, callback: F) -> Self;
    fn on_resign_active<F: 'static + FnMut()>(self, callback: F) -> Self;
    fn on_event<F: 'static + FnMut(Event) -> bool>(self, callback: F) -> Self;
    fn on_open_files<F: 'static + FnMut(Vec<PathBuf>)>(self, callback: F) -> Self;
    fn run(self);
}
