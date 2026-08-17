//! # Bindings to the `AudioToolbox` frameworks
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! This also contains bindings to [the deprecated `AudioUnit` framework][audiounit].
//!
//! [apple-doc]: https://developer.apple.com/documentation/audiotoolbox/
//! [audiounit]: https://developer.apple.com/documentation/audiounit/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-audio-toolbox/0.3.2")]

// NOTE: If we still supported macOS 10.10 or below, we'd have to link
// AudioUnit as well, because certain symbols were originally available there
// (reflected in `AudioToolbox.tbd` with `$ld$hide$os10.10$` etc.).
//
// In newer macOS versions, those symbols were moved to AudioToolbox.
//
// See also https://github.com/RustAudio/coreaudio-sys/pull/51

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// MacTypes.h
#[allow(dead_code)]
pub(crate) type Boolean = u8;
#[allow(dead_code)]
pub(crate) type OSStatus = i32;
#[allow(dead_code)]
pub(crate) type Byte = u8;
#[allow(dead_code)]
pub(crate) type OSType = u32;
