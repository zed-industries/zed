//! Local wrappers for CoreGraphics geometry types with objc2::Encode impls.
//!
//! `core-graphics` types (CGRect, CGPoint, CGSize) do not implement
//! `objc2::encode::Encode` because the `core-graphics` crate predates
//! objc2's encoding system. We define `#[repr(C)]` newtype wrappers with the
//! same memory layout and provide the needed impls, then offer cheap
//! conversion helpers.

use core_graphics::geometry::{CGPoint, CGRect, CGSize};
use objc2::encode::{Encode, Encoding, RefEncode};

// ── CGRect ────────────────────────────────────────────────────────────────────

/// objc2-encodable mirror of `core_graphics::geometry::CGRect`.
/// Layout: `{ origin: { x: f64, y: f64 }, size: { width: f64, height: f64 } }`
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ObjcCGRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

unsafe impl Encode for ObjcCGRect {
    const ENCODING: Encoding = Encoding::Struct(
        "CGRect",
        &[
            Encoding::Struct("CGPoint", &[Encoding::Double, Encoding::Double]),
            Encoding::Struct("CGSize", &[Encoding::Double, Encoding::Double]),
        ],
    );
}

unsafe impl RefEncode for ObjcCGRect {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl ObjcCGRect {
    #[allow(dead_code)]
    pub fn from_cg(r: CGRect) -> Self {
        Self {
            x: r.origin.x,
            y: r.origin.y,
            width: r.size.width,
            height: r.size.height,
        }
    }

    #[allow(dead_code)]
    pub fn to_cg(self) -> CGRect {
        CGRect {
            origin: CGPoint {
                x: self.x,
                y: self.y,
            },
            size: CGSize {
                width: self.width,
                height: self.height,
            },
        }
    }

    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

// ── CGPoint ───────────────────────────────────────────────────────────────────

/// objc2-encodable mirror of `core_graphics::geometry::CGPoint`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ObjcCGPoint {
    pub x: f64,
    pub y: f64,
}

unsafe impl Encode for ObjcCGPoint {
    const ENCODING: Encoding = Encoding::Struct("CGPoint", &[Encoding::Double, Encoding::Double]);
}

unsafe impl RefEncode for ObjcCGPoint {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl ObjcCGPoint {
    #[allow(dead_code)]
    pub fn to_cg(self) -> CGPoint {
        CGPoint {
            x: self.x,
            y: self.y,
        }
    }
}

// ── CGSize ────────────────────────────────────────────────────────────────────

/// objc2-encodable mirror of `core_graphics::geometry::CGSize`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ObjcCGSize {
    pub width: f64,
    pub height: f64,
}

unsafe impl Encode for ObjcCGSize {
    const ENCODING: Encoding = Encoding::Struct("CGSize", &[Encoding::Double, Encoding::Double]);
}

unsafe impl RefEncode for ObjcCGSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl ObjcCGSize {
    #[allow(dead_code)]
    pub fn to_cg(self) -> CGSize {
        CGSize {
            width: self.width,
            height: self.height,
        }
    }
}
