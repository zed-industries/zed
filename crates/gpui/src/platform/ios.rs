//! iOS platform implementation for GPUI.
//!
//! iOS uses UIKit instead of AppKit, so the platform implementation differs
//! significantly from macOS despite sharing many underlying technologies:
//! - Grand Central Dispatch (GCD) for threading
//! - CoreText for text rendering
//! - Metal for GPU rendering
//! - CoreFoundation for many utilities

pub mod demos;
mod dispatcher;
mod display;
mod events;
pub mod ffi;
mod platform;
mod text_input;
mod window;

// Re-use the macOS text system since CoreText is available on iOS
#[cfg(feature = "font-kit")]
mod text_system;

use crate::{DevicePixels, Pixels, Size, px, size};
use objc::runtime::{BOOL, NO, YES};
use std::ops::Range;

pub(crate) use dispatcher::*;
pub(crate) use display::*;
pub(crate) use platform::*;
pub(crate) use window::*;

#[cfg(feature = "font-kit")]
pub(crate) use text_system::*;

/// Placeholder for iOS screen capture frame type.
/// iOS uses ReplayKit for screen capture, which would require additional implementation.
pub(crate) type PlatformScreenCaptureFrame = ();

trait BoolExt {
    fn to_objc(self) -> BOOL;
}

impl BoolExt for bool {
    fn to_objc(self) -> BOOL {
        if self { YES } else { NO }
    }
}

/// NSRange equivalent for iOS (same structure as macOS)
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct NSRange {
    pub location: usize,
    pub length: usize,
}

impl NSRange {
    fn invalid() -> Self {
        Self {
            location: usize::MAX,
            length: 0,
        }
    }

    fn is_valid(&self) -> bool {
        self.location != usize::MAX
    }

    fn to_range(self) -> Option<Range<usize>> {
        if self.is_valid() {
            let start = self.location;
            let end = start + self.length;
            Some(start..end)
        } else {
            None
        }
    }
}

impl From<Range<usize>> for NSRange {
    fn from(range: Range<usize>) -> Self {
        NSRange {
            location: range.start,
            length: range.len(),
        }
    }
}

unsafe impl objc::Encode for NSRange {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{NSRange={}{}}}",
            usize::encode().as_str(),
            usize::encode().as_str()
        );
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

/// Convert a CGSize to Size<Pixels>
impl From<core_graphics::geometry::CGSize> for Size<Pixels> {
    fn from(value: core_graphics::geometry::CGSize) -> Self {
        Size {
            width: px(value.width as f32),
            height: px(value.height as f32),
        }
    }
}

/// Convert a CGRect to Size<Pixels>
impl From<core_graphics::geometry::CGRect> for Size<Pixels> {
    fn from(rect: core_graphics::geometry::CGRect) -> Self {
        let core_graphics::geometry::CGSize { width, height } = rect.size;
        size(px(width as f32), px(height as f32))
    }
}

/// Convert a CGRect to Size<DevicePixels>
impl From<core_graphics::geometry::CGRect> for Size<DevicePixels> {
    fn from(rect: core_graphics::geometry::CGRect) -> Self {
        let core_graphics::geometry::CGSize { width, height } = rect.size;
        size(DevicePixels(width as i32), DevicePixels(height as i32))
    }
}
