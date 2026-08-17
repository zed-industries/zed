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
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-app-kit/0.3.2")]
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

#[cfg(feature = "NSApplication")]
mod application;
#[cfg(feature = "NSEvent")]
mod event;
mod generated;
#[cfg(feature = "NSGestureRecognizer")]
mod gesture_recognizer;
#[cfg(feature = "NSImage")]
mod image;
#[cfg(feature = "NSSlider")]
mod slider;
#[cfg(feature = "NSSliderCell")]
mod slider_cell;
#[cfg(feature = "NSText")]
mod text;

#[cfg(feature = "NSApplication")]
#[cfg(feature = "NSResponder")]
pub use self::application::*;
pub use self::generated::*;

/// (!TARGET_CPU_X86_64 || (TARGET_OS_IPHONE && !TARGET_OS_MACCATALYST))
///
/// <https://github.com/xamarin/xamarin-macios/issues/12111>
#[allow(unused)]
#[allow(unexpected_cfgs)]
pub(crate) const TARGET_ABI_USES_IOS_VALUES: bool = !cfg!(target_arch = "x86_64")
    || (cfg!(all(target_vendor = "apple", not(target_os = "macos")))
        && !cfg!(target_env = "macabi"));

// MacTypes.h
#[allow(unused)]
pub(crate) type UTF32Char = u32; // Or maybe Rust's char?

// OpenGL/gltypes.h
// Not re-exported by objc2-open-gl.
#[allow(unused)]
pub(crate) type GLbitfield = u32;
#[allow(unused)]
pub(crate) type GLenum = u32;
#[allow(unused)]
pub(crate) type GLint = i32;
#[allow(unused)]
pub(crate) type GLsizei = i32;

// TODO: Send + Sync for NSColor. Documentation says:
// > Color objects are immutable and thread-safe
//
// But unsure if this applies for things like `-setFill`?

// TODO: Send + Sync for NSCursor. It is immutable, stated here:
// https://developer.apple.com/documentation/appkit/nscursor/1527062-image?language=objc
//
// But unsure if `push`/`pop` methods are safe from non-main threads?

// NOTE: NSEvent is immutable, so it _may_ be possible to make Send + Sync,
// but let's refrain from doing so, because of:
// > Safely handled only on the same thread, whether that be the main
// > thread or a secondary thread; otherwise you run the risk of having
// > events get out of sequence.
//
// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaFundamentals/AddingBehaviortoaCocoaProgram/AddingBehaviorCocoa.html#//apple_ref/doc/uid/TP40002974-CH5-SW47>
// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html#//apple_ref/doc/uid/10000057i-CH12-123383>

// NOTE: `NSShadow` is marked as `@unchecked Swift.Sendable` in
// `AppKit.Framework/Modules/AppKit.swiftmodule/*.swiftinterface`, but
// only when the deployment target is macOS 14.0 (so we can only do that too
// when https://github.com/rust-lang/rfcs/pull/3750 lands).

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
