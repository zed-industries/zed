//! # Bindings to the `CoreAnimation` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/quartzcore/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
//!
//! This actually lives in the `QuartzCore` framework, but `CoreAnimation` is
//! the name that people use to refer to it.
#![recursion_limit = "256"]
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-quartz-core/0.2.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// CoreFoundation
#[allow(dead_code)]
pub(crate) type CFTimeInterval = std::os::raw::c_double;
