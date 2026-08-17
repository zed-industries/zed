#[cfg(webgpu)]
pub mod webgpu;
#[cfg(webgpu)]
pub(crate) use webgpu::{get_browser_gpu_property, ContextWebGpu};

#[cfg(wgpu_core)]
pub mod wgpu_core;

#[cfg(wgpu_core)]
pub(crate) use wgpu_core::ContextWgpuCore;

#[cfg(custom)]
pub mod custom;
