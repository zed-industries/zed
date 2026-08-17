#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]

use cfg_if::cfg_if;

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {}

pub mod affine_transform;
pub mod base;
pub mod bitmap_context;
pub mod color;
pub mod color_conversion_info;
pub mod color_space;
pub mod context;
pub mod data_provider;

cfg_if!(
    if #[cfg(all(target_os = "macos", feature = "display"))] {
        pub mod direct_display;
        pub mod display;
        pub mod display_configuration;
        pub mod display_fade;
    }
);
#[cfg(all(target_os = "macos", feature = "display", feature = "metal"))]
pub mod direct_display_metal;
#[cfg(all(target_os = "macos", feature = "display-stream", feature = "objc"))]
pub mod display_stream;
pub mod error;
cfg_if!(
    if #[cfg(all(target_os = "macos", feature = "event"))] {
        pub mod event;
        pub mod event_source;
        pub mod event_types;
    }
);
pub mod font;
pub mod function;
pub mod geometry;
pub mod gradient;
pub mod image;
pub mod layer;
pub mod path;
pub mod pattern;
#[cfg(all(target_os = "macos", any(feature = "display", feature = "event")))]
pub mod remote_operation;
pub mod shading;
cfg_if!(
    if #[cfg(all(target_os = "macos", feature = "window"))] {
        pub mod window;
        pub mod window_level;
    }
);
