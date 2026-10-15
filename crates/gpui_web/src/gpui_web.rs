//! GPUI's browser platform uses one document-owned canvas and supports one top-level window.
//! Browser WebGPU is preferred by default, with an automatic WebGL2 fallback. Applications can
//! force either backend with `WebBackendPreference`. Opening a second top-level window, or
//! reopening one after it closes, returns `WebWindowError`.

pub mod canvas_fallback;

pub use canvas_fallback::CanvasFontFallback;

#[cfg(target_family = "wasm")]
mod canvas_text;
#[cfg(target_family = "wasm")]
mod dispatcher;
#[cfg(target_family = "wasm")]
mod display;
#[cfg(target_family = "wasm")]
mod events;
#[cfg(any(target_family = "wasm", test))]
mod glyph_cache;
#[cfg(target_family = "wasm")]
mod http_client;
#[cfg(target_family = "wasm")]
mod ime_mirror;
#[cfg(target_family = "wasm")]
mod keyboard;
#[cfg(target_family = "wasm")]
mod logging;
#[cfg(target_family = "wasm")]
mod platform;
#[cfg(any(target_family = "wasm", test))]
mod run_replacements;
#[cfg(target_family = "wasm")]
mod text_system;
#[cfg(target_family = "wasm")]
mod viewport;
#[cfg(target_family = "wasm")]
mod window;

#[cfg(target_family = "wasm")]
pub use dispatcher::WebDispatcher;
#[cfg(target_family = "wasm")]
pub use display::WebDisplay;
#[cfg(target_family = "wasm")]
pub use gpui_wgpu::WebBackendPreference;
#[cfg(target_family = "wasm")]
pub use http_client::{FetchCredentials, FetchHttpClient};
#[cfg(target_family = "wasm")]
pub use keyboard::WebKeyboardLayout;
#[cfg(target_family = "wasm")]
pub use logging::init_logging;
#[cfg(target_family = "wasm")]
pub use platform::{WebPlatform, WebWindowError};
#[cfg(target_family = "wasm")]
pub use window::WebWindow;
