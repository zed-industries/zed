#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    improper_ctypes
)]

#[macro_use]
extern crate cfg_if;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[link(name = "CoreVideo", kind = "framework")]
extern "C" {}

pub(crate) type OSType = u32;
pub(crate) type GLenum = libc::c_uint;
pub(crate) type GLsizei = libc::c_int;
pub(crate) type GLint = libc::c_int;
pub(crate) type GLuint = libc::c_uint;

pub mod base;
pub mod buffer;
pub mod image_buffer;
pub mod pixel_buffer;
pub mod pixel_buffer_pool;
pub mod pixel_format_description;
pub mod return_;

pub use self::base::*;
pub use self::buffer::*;
pub use self::image_buffer::*;
pub use self::pixel_buffer::*;
pub use self::pixel_buffer_pool::*;
pub use self::pixel_format_description::*;
pub use self::return_::*;

cfg_if! {
    if #[cfg(feature = "metal")] {
        pub mod metal_texture;
        pub mod metal_texture_cache;

        pub use self::metal_texture::*;
        pub use self::metal_texture_cache::*;
    }
}

cfg_if! {
    if #[cfg(feature = "display_link")] {
        pub mod host_time;
        pub mod display_link;

        pub use self::host_time::*;
        pub use self::display_link::*;
    }
}

cfg_if! {
    if #[cfg(feature = "opengl")] {
        pub mod opengl_buffer;
        pub mod opengl_buffer_pool;
        pub mod opengl_texture;
        pub mod opengl_texture_cache;

        pub use self::opengl_buffer::*;
        pub use self::opengl_buffer_pool::*;
        pub use self::opengl_texture::*;
        pub use self::opengl_texture_cache::*;
    }
}

cfg_if! {
    if #[cfg(feature = "io_suface")] {
        pub mod pixel_buffer_io_surface;

        pub use self::pixel_buffer_io_surface::*;
    }
}

pub mod open_gl_es_texture;
pub mod open_gl_es_texture_cache;

pub use self::open_gl_es_texture::*;
pub use self::open_gl_es_texture_cache::*;
