#![cfg(target_os = "windows")]

mod clipboard;
mod destination_list;
mod direct_write;
mod directx_atlas;
mod directx_devices;
mod directx_renderer;
mod dispatcher;
mod display;
mod events;
mod keyboard;
mod platform;
mod system_settings;
mod util;
mod vsync;
mod window;
mod wrapper;

pub(crate) use clipboard::*;
pub(crate) use destination_list::*;
pub(crate) use direct_write::*;
pub(crate) use directx_atlas::*;
pub(crate) use directx_devices::*;
pub(crate) use directx_renderer::*;
pub(crate) use dispatcher::*;
pub(crate) use display::*;
pub(crate) use events::*;
pub(crate) use keyboard::*;
pub(crate) use platform::*;
pub(crate) use system_settings::*;
pub(crate) use util::*;
pub(crate) use vsync::*;
pub(crate) use window::*;
pub(crate) use wrapper::*;

pub use platform::WindowsPlatform;

pub(crate) use windows::Win32::Foundation::HWND;

/// Returns a `DirectWriteTextSystem` as a platform-neutral text system.
///
/// This is useful for tests that need real text shaping on Windows (e.g. with
/// `HeadlessAppContext`). Requires Direct3D device initialization.
pub fn platform_text_system() -> anyhow::Result<std::sync::Arc<dyn gpui::PlatformTextSystem>> {
    let devices = DirectXDevices::new()?;
    let text_system = DirectWriteTextSystem::new(&devices)?;
    Ok(std::sync::Arc::new(text_system))
}
