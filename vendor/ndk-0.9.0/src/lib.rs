//! # Android NDK
//!
//! Bindings to the [Android NDK].
//!
//! [Android NDK]: https://developer.android.com/ndk/reference
#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    trivial_casts,
    unused_qualifications
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub mod asset;
pub mod audio;
pub mod bitmap;
pub mod configuration;
pub mod data_space;
pub mod event;
pub mod font;
pub mod hardware_buffer;
pub mod hardware_buffer_format;
pub mod input_queue;
pub mod looper;
pub mod media;
pub mod media_error;
pub mod native_activity;
pub mod native_window;
pub mod shared_memory;
pub mod surface_texture;
pub mod sync;
pub mod trace;
mod utils;
