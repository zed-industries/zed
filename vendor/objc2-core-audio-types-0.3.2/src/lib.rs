//! # Bindings to the `CoreAudioTypes` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/coreaudiotypes/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-core-audio-types/0.3.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "CoreAudioBaseTypes")]
mod base_types;
#[allow(clippy::eq_op)]
mod generated;
#[cfg(feature = "AudioSessionTypes")]
mod session;

#[cfg(feature = "CoreAudioBaseTypes")]
pub use self::base_types::*;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "AudioSessionTypes")]
pub use self::session::{AVAudioInteger, AVAudioUInteger};

// MacTypes.h
#[allow(dead_code)]
pub(crate) type OSStatus = i32;
#[allow(dead_code)]
pub(crate) type OSType = u32;
