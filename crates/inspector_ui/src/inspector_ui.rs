#[cfg(debug_assertions)]
mod div_inspector;
#[cfg(debug_assertions)]
mod init;
#[cfg(debug_assertions)]
mod options;

#[cfg(debug_assertions)]
pub use init::init;

#[cfg(not(debug_assertions))]
pub fn init(_cx: &mut gpui::App) {}
