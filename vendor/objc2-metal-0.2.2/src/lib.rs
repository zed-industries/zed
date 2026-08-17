//! # Bindings to the `Metal` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/metal/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
//!
//! Metal has tools for validating that you're using it correctly, using these
//! is highly recommended! See [Apple's documentation on it][apple-doc], or
//! run `man MetalValidation` to get information on environment variables.
//!
//! [apple-doc]: https://developer.apple.com/documentation/xcode/validating-your-apps-metal-api-usage/.
//!
//! Note: To use `MTLCreateSystemDefaultDevice` you need to link to
//! `CoreGraphics`, this can be done by using `objc2-app-kit`, or by doing:
//! ```rust
//! #[link(name = "CoreGraphics", kind = "framework")]
//! extern "C" {}
//! ```
//!
//!
//! ## Example
//!
//! Drawing a rotating triangle.
//!
//! ```ignore
#![doc = include_str!("../examples/triangle.rs")]
//! ```
//!
//! With the following shader.
//!
//! ```ignore
#![doc = include_str!("../examples/triangle.metal")]
//! ```
#![recursion_limit = "256"]
#![allow(non_snake_case)]
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-metal/0.2.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "MTLCaptureManager")]
mod capture;
#[cfg(feature = "MTLCounters")]
mod counters;
#[cfg(feature = "MTLDevice")]
mod device;
mod generated;
#[cfg(feature = "MTLAccelerationStructureTypes")]
mod packed;
#[cfg(feature = "unstable-private")]
mod private;
#[cfg(feature = "MTLResource")]
mod resource;
mod slice;
#[cfg(feature = "MTLTexture")]
mod texture;

#[cfg(feature = "MTLCounters")]
pub use self::counters::*;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "MTLAccelerationStructureTypes")]
pub use self::packed::MTLPackedFloat3;
#[cfg(feature = "unstable-private")]
pub use self::private::MTLDevicePrivate;
#[cfg(feature = "MTLResource")]
pub use self::resource::*;
#[cfg(all(feature = "MTLRenderCommandEncoder", feature = "MTLCommandEncoder"))]
pub use self::slice::MTLRenderCommandEncoderSliceExt;
#[cfg(feature = "MTLTexture")]
pub use self::texture::*;

// CoreFoundation
#[allow(dead_code)]
pub(crate) type CFTimeInterval = std::os::raw::c_double;
