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
//! NOTE: To use [`MTLCreateSystemDefaultDevice`] you need to link to
//! `CoreGraphics`, this can be done by using `objc2-core-graphics`, or by
//! doing:
//! ```rust
//! #[link(name = "CoreGraphics", kind = "framework")]
//! extern "C" {}
//! ```
//!
#![cfg_attr(
    not(feature = "MTLDevice"),
    doc = "[`MTLCreateSystemDefaultDevice`]: #needs-MTLDevice-feature"
)]
//!
//! # Safety considerations
//!
//! Metal allows running arbitrary code on the GPU. We treat memory safety
//! issues on the GPU as just as unsafe as that which applies to the CPU. A
//! few notes on this below.
//!
//! ## Shaders
//!
//! Shaders are (often) written in an unsafe C-like language.
//!
//! Loading them (via `MTLLibrary`, function stitching etc.) is perfectly
//! safe, it is similar to dynamic linking. The restrictions that e.g.
//! `libloading::Library::new` labours under do not apply, since there are no
//! ctors in [the Metal Shading Language][msl-spec] (see section 4.2).
//!
//! Similarly, getting individual shaders (`MTLFunction`) is safe, we can
//! model this as the same as calling `dlsym` (which just returns a pointer).
//!
//! _Calling_ functions though, is not safe. Even though they can have their
//! parameter and return types checked at runtime, they may have additional
//! restrictions not present in the signature (e.g. `__builtin_unreachable()`
//! is possible in MSL, so is out-of-bounds accesses). If you view
//! `MTLFunction` as essentially just an `unsafe fn()` pointer, this should be
//! apparent.
//!
//! [msl-spec]: https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf
//!
//! ## Bounds checks
//!
//! It is yet unclear whether Metal APIs are bounds-checked on the CPU side or
//! not, so APIs that take offsets / lengths are often unsafe.
//!
//! ## Synchronization
//!
//! `MTLResource` subclasses such as `MTLBuffer` and `MTLTexture` require
//! synchronization between the CPU and the GPU, or between different threads
//! on the GPU itself, so APIs taking these are often unsafe.
//!
//! ## Memory management and lifetimes
//!
//! Resources used in `MTL4CommandBuffer`s or command buffers with created
//! with one of:
//! - `MTLCommandBufferDescriptor::setRetainedReferences(false)`.
//! - `MTLCommandQueue::commandBufferWithUnretainedReferences()`.
//!
//! Must be kept alive for as long as they're used.
//!
//! ## Type safety
//!
//! `MTLBuffer` is untyped (in a similar manner as a `[u8]` slice), you must
//! ensure that any usage of it is done with valid types.
#![recursion_limit = "256"]
#![allow(non_snake_case)]
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-metal/0.3.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "MTLAccelerationStructureTypes")]
mod acceleration_structure_types;
#[cfg(feature = "MTLCaptureManager")]
mod capture;
#[cfg(feature = "MTLCounters")]
mod counters;
#[cfg(feature = "MTLDevice")]
mod device;
mod generated;
#[cfg(feature = "unstable-private")]
mod private;
#[cfg(feature = "MTLRasterizationRate")]
mod rasterization_rate;
#[cfg(feature = "MTLResource")]
mod resource;
mod slice;
#[cfg(feature = "MTLTexture")]
mod texture;
#[cfg(feature = "MTLTypes")]
mod types;

#[cfg(feature = "MTLAccelerationStructureTypes")]
pub use self::acceleration_structure_types::MTLPackedFloat3;
#[cfg(feature = "MTLCounters")]
pub use self::counters::*;
#[cfg(feature = "MTLDevice")]
pub use self::device::*;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "unstable-private")]
pub use self::private::MTLDevicePrivate;
#[cfg(feature = "MTLResource")]
pub use self::resource::*;
#[cfg(all(feature = "MTLRenderCommandEncoder", feature = "MTLCommandEncoder"))]
pub use self::slice::MTLRenderCommandEncoderSliceExt;
#[cfg(feature = "MTLTexture")]
pub use self::texture::*;
