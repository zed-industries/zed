//! # Bindings to the `AppKit` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/appkit/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
//!
//! Note that a lot of functionality in AppKit requires that the application
//! has initialized properly, which is only done after the application
//! delegate has received `applicationDidFinishLaunching`.
//!
//! You should aspire to do all your UI initialization work in there!
//!
//!
//! ## NSWindow
//!
//! Be careful when creating `NSWindow` if it's not inside a window
//! controller; in those cases you're _required_ to call
//! `window.releasedWhenClosed(false)` to get correct memory management, which
//! is also why the creation methods for `NSWindow` are `unsafe`.
//!
//!
//! ## Examples
//!
//! Implementing `NSApplicationDelegate` for a custom class.
//!
//! ```ignore
#![doc = include_str!("../examples/delegate.rs")]
//! ```
//!
//! An example showing basic and a bit more advanced usage of `NSPasteboard`.
//!
//! ```ignore
#![doc = include_str!("../examples/nspasteboard.rs")]
//! ```
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-app-kit/0.2.2")]
#![recursion_limit = "512"]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unreachable_pub)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg_attr(feature = "gnustep-1-7", link(name = "gnustep-gui", kind = "dylib"))]
extern "C" {}

/// (!TARGET_CPU_X86_64 || (TARGET_OS_IPHONE && !TARGET_OS_MACCATALYST))
///
/// <https://github.com/xamarin/xamarin-macios/issues/12111>
// TODO: Make this work with mac catalyst
#[allow(dead_code)]
pub(crate) const TARGET_ABI_USES_IOS_VALUES: bool =
    !cfg!(any(target_arch = "x86", target_arch = "x86_64")) || cfg!(not(target_os = "macos"));

#[cfg(feature = "NSApplication")]
mod application;
mod generated;
#[cfg(feature = "NSImage")]
mod image;
#[cfg(feature = "NSText")]
mod text;

#[cfg(feature = "NSApplication")]
pub use self::application::*;
pub use self::generated::*;
#[cfg(feature = "NSImage")]
pub use self::image::*;
#[cfg(feature = "NSText")]
pub use self::text::*;

// MacTypes.h
#[allow(unused)]
pub(crate) type UTF32Char = u32; // Or maybe Rust's char?

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "NSAccessibilityProtocols")]
    fn accessibility_element_protocol() {
        use crate::NSAccessibilityElementProtocol;
        use objc2::ProtocolType;
        let actual = <dyn NSAccessibilityElementProtocol>::NAME;
        assert_eq!(actual, "NSAccessibilityElement");
    }
}
