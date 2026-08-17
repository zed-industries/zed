#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]

extern crate core_graphics2 as core_graphics;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[cfg_attr(feature = "link", link(name = "CoreVideo", kind = "framework"))]
extern "C" {}

pub type CGLContextObj = *mut libc::c_void;
pub type CGLPixelFormatObj = *mut libc::c_void;
pub type GLenum = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLint = libc::c_int;
pub type GLuint = libc::c_uint;
pub type OSType = u32;

pub mod base;
pub mod buffer;
#[cfg(all(target_os = "macos", feature = "display-link"))]
pub mod display_link;
pub mod host_time;
pub mod image_buffer;
#[cfg(feature = "metal")]
pub mod metal_texture;
#[cfg(feature = "metal")]
pub mod metal_texture_cache;
#[cfg(target_os = "macos")]
pub mod opengl_buffer;
#[cfg(target_os = "macos")]
pub mod opengl_buffer_pool;
#[cfg(target_os = "ios")]
pub mod opengl_es_texture;
#[cfg(target_os = "ios")]
pub mod opengl_es_texture_cache;
#[cfg(target_os = "macos")]
pub mod opengl_texture;
#[cfg(target_os = "macos")]
pub mod opengl_texture_cache;
pub mod pixel_buffer;
pub mod pixel_buffer_io_surface;
pub mod pixel_buffer_pool;
pub mod pixel_format_description;
pub mod r#return;
