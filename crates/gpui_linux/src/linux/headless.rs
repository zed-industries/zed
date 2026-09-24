mod client;
mod window;

pub(crate) use client::*;
#[cfg(feature = "wayland")]
pub(crate) use window::{HeadlessDisplay, HeadlessWindow};
