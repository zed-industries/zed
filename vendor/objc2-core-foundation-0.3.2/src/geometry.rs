#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, RefEncode};

use crate::{CGAffineTransform, CGVector};

#[cfg(target_pointer_width = "64")]
type InnerFloat = f64;
#[cfg(not(target_pointer_width = "64"))]
type InnerFloat = f32;

/// The basic type for all floating-point values.
///
/// This is [`f32`] on 32-bit platforms and [`f64`] on 64-bit platforms.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cgfloat?language=objc).
// Defined in CoreGraphics/CGBase.h and CoreFoundation/CFCGTypes.h
// TODO: Use a newtype here?
pub type CGFloat = InnerFloat;

// NSGeometry types are aliases to CGGeometry types on iOS, tvOS, watchOS and
// macOS 64bit (and hence their Objective-C encodings are different).
//
// TODO: Adjust `objc2-encode` so that this is handled there, and so that we
// can effectively forget about it and use `NS` and `CG` types equally.
#[cfg(not(any(
    not(target_vendor = "apple"),
    all(target_os = "macos", target_pointer_width = "32")
)))]
#[cfg(feature = "objc2")]
mod names {
    pub(super) const POINT: &str = "CGPoint";
    pub(super) const SIZE: &str = "CGSize";
    pub(super) const RECT: &str = "CGRect";
}

#[cfg(any(
    not(target_vendor = "apple"),
    all(target_os = "macos", target_pointer_width = "32")
))]
#[cfg(feature = "objc2")]
mod names {
    pub(super) const POINT: &str = "_NSPoint";
    pub(super) const SIZE: &str = "_NSSize";
    pub(super) const RECT: &str = "_NSRect";
}

/// A point in a two-dimensional coordinate system.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cgpoint?language=objc).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CGPoint {
    /// The x-coordinate of the point.
    pub x: CGFloat,
    /// The y-coordinate of the point.
    pub y: CGFloat,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGPoint {
    const ENCODING: Encoding =
        Encoding::Struct(names::POINT, &[CGFloat::ENCODING, CGFloat::ENCODING]);
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGPoint {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl CGPoint {
    /// Create a new point with the given coordinates.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_core_foundation::CGPoint;
    /// assert_eq!(CGPoint::new(10.0, -2.3), CGPoint { x: 10.0, y: -2.3 });
    /// ```
    #[inline]
    #[doc(alias = "NSMakePoint")]
    #[doc(alias = "CGPointMake")]
    pub const fn new(x: CGFloat, y: CGFloat) -> Self {
        Self { x, y }
    }

    /// A point with both coordinates set to `0.0`.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_core_foundation::CGPoint;
    /// assert_eq!(CGPoint::ZERO, CGPoint { x: 0.0, y: 0.0 });
    /// ```
    #[doc(alias = "NSZeroPoint")]
    #[doc(alias = "CGPointZero")]
    #[doc(alias = "ORIGIN")]
    pub const ZERO: Self = Self::new(0.0, 0.0);
}

/// A two-dimensional size.
///
/// As this is sometimes used to represent a distance vector, rather than a
/// physical size, the width and height are _not_ guaranteed to be
/// non-negative! Methods that expect that must use one of [`CGSize::abs`] or
/// [`CGRect::standardize`].
///
/// See [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cgsize?language=objc).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CGSize {
    /// The dimensions along the x-axis.
    pub width: CGFloat,
    /// The dimensions along the y-axis.
    pub height: CGFloat,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGSize {
    const ENCODING: Encoding =
        Encoding::Struct(names::SIZE, &[CGFloat::ENCODING, CGFloat::ENCODING]);
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl CGSize {
    /// Create a new size with the given dimensions.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_core_foundation::CGSize;
    /// let size = CGSize::new(10.0, 2.3);
    /// assert_eq!(size.width, 10.0);
    /// assert_eq!(size.height, 2.3);
    /// ```
    ///
    /// Negative values are allowed (though often undesired).
    ///
    /// ```
    /// use objc2_core_foundation::CGSize;
    /// let size = CGSize::new(-1.0, 0.0);
    /// assert_eq!(size.width, -1.0);
    /// ```
    #[inline]
    #[doc(alias = "NSMakeSize")]
    #[doc(alias = "CGSizeMake")]
    pub const fn new(width: CGFloat, height: CGFloat) -> Self {
        // The documentation for NSSize explicitly says:
        // > If the value of width or height is negative, however, the
        // > behavior of some methods may be undefined.
        //
        // But since this type can come from FFI, we'll leave it up to the
        // user to ensure that it is used safely.
        Self { width, height }
    }

    /// Convert the size to a non-negative size.
    ///
    /// This can be used to convert the size to a safe value.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_core_foundation::CGSize;
    /// assert_eq!(CGSize::new(-1.0, 1.0).abs(), CGSize::new(1.0, 1.0));
    /// ```
    #[inline]
    #[cfg(feature = "std")] // Only available in core since Rust 1.85
    pub fn abs(self) -> Self {
        Self::new(self.width.abs(), self.height.abs())
    }

    /// A size that is 0.0 in both dimensions.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_core_foundation::CGSize;
    /// assert_eq!(CGSize::ZERO, CGSize { width: 0.0, height: 0.0 });
    /// ```
    #[doc(alias = "NSZeroSize")]
    #[doc(alias = "CGSizeZero")]
    pub const ZERO: Self = Self::new(0.0, 0.0);
}

/// The location and dimensions of a rectangle.
///
/// In the default Core Graphics coordinate space (macOS), the origin is
/// located in the lower-left corner of the rectangle and the rectangle
/// extends towards the upper-right corner.
///
/// If the context has a flipped coordinate space (iOS, tvOS, watchOS) the
/// origin is in the upper-left corner and the rectangle extends towards the
/// lower-right corner.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cgrect?language=objc).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CGRect {
    /// The coordinates of the rectangle’s origin.
    pub origin: CGPoint,
    /// The dimensions of the rectangle.
    pub size: CGSize,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGRect {
    const ENCODING: Encoding =
        Encoding::Struct(names::RECT, &[CGPoint::ENCODING, CGSize::ENCODING]);
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGRect {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl CGRect {
    /// Create a new rectangle with the given origin and dimensions.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_core_foundation::{CGPoint, CGRect, CGSize};
    /// let origin = CGPoint::new(10.0, -2.3);
    /// let size = CGSize::new(5.0, 0.0);
    /// let rect = CGRect::new(origin, size);
    /// ```
    #[inline]
    #[doc(alias = "NSMakeRect")]
    #[doc(alias = "CGRectMake")]
    pub const fn new(origin: CGPoint, size: CGSize) -> Self {
        Self { origin, size }
    }

    /// A rectangle with origin (0.0, 0.0) and zero width and height.
    #[doc(alias = "NSZeroRect")]
    #[doc(alias = "CGRectZero")]
    pub const ZERO: Self = Self::new(CGPoint::ZERO, CGSize::ZERO);

    /// Returns a rectangle with a positive width and height.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_core_foundation::{CGPoint, CGRect, CGSize};
    /// let origin = CGPoint::new(1.0, 1.0);
    /// let size = CGSize::new(-5.0, -2.0);
    /// let rect = CGRect::new(origin, size);
    /// assert_eq!(rect.standardize().size, CGSize::new(5.0, 2.0));
    /// ```
    #[inline]
    #[doc(alias = "CGRectStandardize")]
    #[cfg(feature = "std")] // `abs` only available in core since Rust 1.85
    pub fn standardize(self) -> Self {
        Self::new(self.origin, self.size.abs())
    }

    /// The smallest coordinate of the rectangle.
    #[inline]
    #[doc(alias = "CGRectGetMinX")]
    #[doc(alias = "CGRectGetMinY")]
    #[doc(alias = "NSMinX")]
    #[doc(alias = "NSMinY")]
    pub fn min(self) -> CGPoint {
        self.origin
    }

    /// The center point of the rectangle.
    #[inline]
    #[doc(alias = "CGRectGetMidX")]
    #[doc(alias = "CGRectGetMidY")]
    #[doc(alias = "NSMidX")]
    #[doc(alias = "NSMidY")]
    pub fn mid(self) -> CGPoint {
        CGPoint::new(
            self.origin.x + (self.size.width * 0.5),
            self.origin.y + (self.size.height * 0.5),
        )
    }

    /// The largest coordinate of the rectangle.
    #[inline]
    #[doc(alias = "CGRectGetMaxX")]
    #[doc(alias = "CGRectGetMaxY")]
    #[doc(alias = "NSMaxX")]
    #[doc(alias = "NSMaxY")]
    pub fn max(self) -> CGPoint {
        CGPoint::new(
            self.origin.x + self.size.width,
            self.origin.y + self.size.height,
        )
    }

    /// Returns whether a rectangle has zero width or height.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_core_foundation::{CGPoint, CGRect, CGSize};
    /// assert!(CGRect::ZERO.is_empty());
    /// let point = CGPoint::new(1.0, 2.0);
    /// assert!(CGRect::new(point, CGSize::ZERO).is_empty());
    /// assert!(!CGRect::new(point, CGSize::new(1.0, 1.0)).is_empty());
    /// ```
    #[inline]
    #[doc(alias = "CGRectIsEmpty")]
    pub fn is_empty(self) -> bool {
        !(self.size.width > 0.0 && self.size.height > 0.0)
        // TODO: NaN handling?
        // self.size.width <= 0.0 || self.size.height <= 0.0
    }

    // TODO: NSContainsRect / CGRectContainsRect
    // TODO: NSDivideRect / CGRectDivide
    // TODO: NSInsetRect / CGRectInset
    // TODO: NSIntegralRect / CGRectIntegral
    // TODO: NSIntersectionRect / CGRectIntersection
    // TODO: NSUnionRect / CGRectUnion
    // TODO: NSIntersectsRect / CGRectIntersectsRect
    // TODO: NSMouseInRect
    // TODO: NSMouseInRect
    // TODO: NSPointInRect / CGRectContainsPoint
    // TODO: NSOffsetRect / CGRectOffset

    // TODO: CGRectIsNull
    // TODO: CGRectIsInfinite
    // TODO: CGRectInfinite
    // TODO: CGRectNull

    // TODO: NSHeight / CGRectGetHeight (standardized)
    // TODO: NSWidth / CGRectGetWidth (standardized)
}

// TODO: Derive this
impl Default for CGVector {
    fn default() -> Self {
        Self { dx: 0.0, dy: 0.0 }
    }
}

impl CGVector {
    #[inline]
    #[doc(alias = "CGVectorMake")]
    pub const fn new(dx: CGFloat, dy: CGFloat) -> Self {
        Self { dx, dy }
    }
}

// TODO: Derive this
impl Default for CGAffineTransform {
    fn default() -> Self {
        Self {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            tx: 0.0,
            ty: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgsize_new() {
        CGSize::new(1.0, 1.0);
        CGSize::new(0.0, -0.0);
        CGSize::new(-0.0, 0.0);
        CGSize::new(-0.0, -0.0);
        CGSize::new(-1.0, -1.0);
        CGSize::new(-1.0, 1.0);
        CGSize::new(1.0, -1.0);
    }
}
