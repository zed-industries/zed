//! This library safely implements WebGPU on native platforms.
//! It is designed for integration into browsers, as well as wrapping
//! into other language-specific user-friendly libraries.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
//!

#![no_std]
// When we have no backends, we end up with a lot of dead or otherwise unreachable code.
#![cfg_attr(
    all(
        not(all(feature = "vulkan", not(target_arch = "wasm32"))),
        not(all(feature = "metal", any(target_vendor = "apple"))),
        not(all(feature = "dx12", windows)),
        not(feature = "gles"),
    ),
    allow(unused, clippy::let_and_return)
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    // It is much clearer to assert negative conditions with eq! false
    clippy::bool_assert_comparison,
    // We don't use syntax sugar where it's not necessary.
    clippy::match_like_matches_macro,
    // Redundant matching is more explicit.
    clippy::redundant_pattern_matching,
    // Explicit lifetimes are often easier to reason about.
    clippy::needless_lifetimes,
    // No need for defaults in the internal types.
    clippy::new_without_default,
    // Needless updates are more scalable, easier to play with features.
    clippy::needless_update,
    // Need many arguments for some core functions to be able to re-use code in many situations.
    clippy::too_many_arguments,
    // It gets in the way a lot and does not prevent bugs in practice.
    clippy::pattern_type_mismatch,
    // `wgpu-core` isn't entirely user-facing, so it's useful to document internal items.
    rustdoc::private_intra_doc_links,
)]
#![warn(
    clippy::alloc_instead_of_core,
    clippy::ptr_as_ptr,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_op_in_unsafe_fn,
    unused_extern_crates,
    unused_qualifications
)]
// We use `Arc` in wgpu-core, but on wasm (unless opted out via `fragile-send-sync-non-atomic-wasm`)
// wgpu-hal resources are not Send/Sync, causing a clippy warning for unnecessary `Arc`s.
// We could use `Rc`s in this case as recommended, but unless atomics are enabled
// this doesn't make a difference.
// Therefore, this is only really a concern for users targeting WebGL
// (the only reason to use wgpu-core on the web in the first place) that have atomics enabled.
//
// NOTE: Keep this in sync with `wgpu`.
#![cfg_attr(not(send_sync), allow(clippy::arc_with_non_send_sync))]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;
extern crate wgpu_hal as hal;
extern crate wgpu_types as wgt;

mod as_hal;
pub mod binding_model;
pub mod command;
mod conv;
pub mod device;
pub mod error;
pub mod global;
mod hash_utils;
pub mod hub;
pub mod id;
pub mod identity;
mod indirect_validation;
mod init_tracker;
pub mod instance;
mod lock;
pub mod pipeline;
mod pipeline_cache;
mod pool;
pub mod present;
pub mod ray_tracing;
pub mod registry;
pub mod resource;
mod snatch;
pub mod storage;
mod timestamp_normalization;
mod track;
mod weak_vec;
// This is public for users who pre-compile shaders while still wanting to
// preserve all run-time checks that `wgpu-core` does.
// See <https://github.com/gfx-rs/wgpu/issues/3103>, after which this can be
// made private again.
mod scratch;
pub mod validation;

pub use validation::{map_storage_format_from_naga, map_storage_format_to_naga};

pub use hal::{api, MAX_BIND_GROUPS, MAX_COLOR_ATTACHMENTS, MAX_VERTEX_BUFFERS};
pub use naga;

use alloc::{
    borrow::{Cow, ToOwned as _},
    string::String,
};

pub(crate) use hash_utils::*;

/// The index of a queue submission.
///
/// These are the values stored in `Device::fence`.
pub type SubmissionIndex = hal::FenceValue;

type Index = u32;
type Epoch = u32;

pub type RawString = *const core::ffi::c_char;
pub type Label<'a> = Option<Cow<'a, str>>;

trait LabelHelpers<'a> {
    fn to_hal(&'a self, flags: wgt::InstanceFlags) -> Option<&'a str>;
    fn to_string(&self) -> String;
}
impl<'a> LabelHelpers<'a> for Label<'a> {
    fn to_hal(&'a self, flags: wgt::InstanceFlags) -> Option<&'a str> {
        if flags.contains(wgt::InstanceFlags::DISCARD_HAL_LABELS) {
            return None;
        }

        self.as_deref()
    }
    fn to_string(&self) -> String {
        self.as_deref().map(str::to_owned).unwrap_or_default()
    }
}

pub fn hal_label<T: AsRef<str>>(opt: Option<T>, flags: wgt::InstanceFlags) -> Option<T> {
    if flags.contains(wgt::InstanceFlags::DISCARD_HAL_LABELS) {
        return None;
    }

    opt
}

const DOWNLEVEL_WARNING_MESSAGE: &str = concat!(
    "The underlying API or device in use does not ",
    "support enough features to be a fully compliant implementation of WebGPU. ",
    "A subset of the features can still be used. ",
    "If you are running this program on native and not in a browser and wish to limit ",
    "the features you use to the supported subset, ",
    "call Adapter::downlevel_properties or Device::downlevel_properties to get ",
    "a listing of the features the current ",
    "platform supports."
);

const DOWNLEVEL_ERROR_MESSAGE: &str = concat!(
    "This is not an invalid use of WebGPU: the underlying API or device does not ",
    "support enough features to be a fully compliant implementation. ",
    "A subset of the features can still be used. ",
    "If you are running this program on native and not in a browser ",
    "and wish to work around this issue, call ",
    "Adapter::downlevel_properties or Device::downlevel_properties ",
    "to get a listing of the features the current platform supports."
);

#[cfg(feature = "api_log_info")]
macro_rules! api_log {
    ($($arg:tt)+) => (log::info!($($arg)+))
}
#[cfg(not(feature = "api_log_info"))]
macro_rules! api_log {
    ($($arg:tt)+) => (log::trace!($($arg)+))
}

#[cfg(feature = "api_log_info")]
macro_rules! api_log_debug {
    ($($arg:tt)+) => (log::info!($($arg)+))
}
#[cfg(not(feature = "api_log_info"))]
macro_rules! api_log_debug {
    ($($arg:tt)+) => (log::debug!($($arg)+))
}

pub(crate) use api_log;
pub(crate) use api_log_debug;

#[cfg(feature = "resource_log_info")]
macro_rules! resource_log {
    ($($arg:tt)+) => (log::info!($($arg)+))
}
#[cfg(not(feature = "resource_log_info"))]
macro_rules! resource_log {
    ($($arg:tt)+) => (log::trace!($($arg)+))
}
pub(crate) use resource_log;

#[inline]
pub(crate) fn get_lowest_common_denom(a: u32, b: u32) -> u32 {
    let gcd = if a >= b {
        get_greatest_common_divisor(a, b)
    } else {
        get_greatest_common_divisor(b, a)
    };
    a * b / gcd
}

#[inline]
pub(crate) fn get_greatest_common_divisor(mut a: u32, mut b: u32) -> u32 {
    assert!(a >= b);
    loop {
        let c = a % b;
        if c == 0 {
            return b;
        } else {
            a = b;
            b = c;
        }
    }
}

#[cfg(not(feature = "std"))]
use core::cell::OnceCell as OnceCellOrLock;
#[cfg(feature = "std")]
use std::sync::OnceLock as OnceCellOrLock;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcd() {
        assert_eq!(get_lowest_common_denom(2, 2), 2);
        assert_eq!(get_lowest_common_denom(2, 3), 6);
        assert_eq!(get_lowest_common_denom(6, 4), 12);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(get_greatest_common_divisor(5, 1), 1);
        assert_eq!(get_greatest_common_divisor(4, 2), 2);
        assert_eq!(get_greatest_common_divisor(6, 4), 2);
        assert_eq!(get_greatest_common_divisor(7, 7), 7);
    }
}
