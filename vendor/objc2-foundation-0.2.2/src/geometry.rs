use objc2::encode::{Encode, Encoding, RefEncode};
use objc2::ffi::NSUInteger;

#[cfg(target_pointer_width = "64")]
type InnerFloat = f64;
#[cfg(not(target_pointer_width = "64"))]
type InnerFloat = f32;

/// The basic type for all floating-point values.
///
/// This is [`f32`] on 32-bit platforms and [`f64`] on 64-bit platforms.
///
/// This technically belongs to the `CoreGraphics` framework, but we define it
/// here for convenience.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgfloat?language=objc).
// Defined in CoreGraphics/CGBase.h and CoreFoundation/CFCGTypes.h
// TODO: Use a newtype here?
pub type CGFloat = InnerFloat;

// NSGeometry types are aliases to CGGeometry types on iOS, tvOS, watchOS and
// macOS 64bit (and hence their Objective-C encodings are different).
//
// TODO: Adjust `objc2-encode` so that this is handled there, and so that we
// can effectively forget about it and use `NS` and `CG` types equally.
#[cfg(not(any(
    feature = "gnustep-1-7",
    all(target_os = "macos", target_pointer_width = "32")
)))]
mod names {
    pub(super) const POINT: &str = "CGPoint";
    pub(super) const SIZE: &str = "CGSize";
    pub(super) const RECT: &str = "CGRect";
}

#[cfg(any(
    feature = "gnustep-1-7",
    all(target_os = "macos", target_pointer_width = "32")
))]
mod names {
    pub(super) const POINT: &str = "_NSPoint";
    pub(super) const SIZE: &str = "_NSSize";
    pub(super) const RECT: &str = "_NSRect";
}

/// A point in a two-dimensional coordinate system.
///
/// This technically belongs to the `CoreGraphics` framework, but we define it
/// here for convenience.
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

unsafe impl Encode for CGPoint {
    const ENCODING: Encoding =
        Encoding::Struct(names::POINT, &[CGFloat::ENCODING, CGFloat::ENCODING]);
}

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
    /// use objc2_foundation::CGPoint;
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
    /// use objc2_foundation::CGPoint;
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
/// This technically belongs to the `CoreGraphics` framework, but we define it
/// here for convenience.
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

unsafe impl Encode for CGSize {
    const ENCODING: Encoding =
        Encoding::Struct(names::SIZE, &[CGFloat::ENCODING, CGFloat::ENCODING]);
}

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
    /// use objc2_foundation::CGSize;
    /// let size = CGSize::new(10.0, 2.3);
    /// assert_eq!(size.width, 10.0);
    /// assert_eq!(size.height, 2.3);
    /// ```
    ///
    /// Negative values are allowed (though often undesired).
    ///
    /// ```
    /// use objc2_foundation::CGSize;
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
    /// use objc2_foundation::CGSize;
    /// assert_eq!(CGSize::new(-1.0, 1.0).abs(), CGSize::new(1.0, 1.0));
    /// ```
    #[inline]
    pub fn abs(self) -> Self {
        Self::new(self.width.abs(), self.height.abs())
    }

    /// A size that is 0.0 in both dimensions.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::CGSize;
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
/// This technically belongs to the `CoreGraphics` framework, but we define it
/// here for convenience.
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

unsafe impl Encode for CGRect {
    const ENCODING: Encoding =
        Encoding::Struct(names::RECT, &[CGPoint::ENCODING, CGSize::ENCODING]);
}

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
    /// use objc2_foundation::{CGPoint, CGRect, CGSize};
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
    /// This is often useful
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{CGPoint, CGRect, CGSize};
    /// let origin = CGPoint::new(1.0, 1.0);
    /// let size = CGSize::new(-5.0, -2.0);
    /// let rect = CGRect::new(origin, size);
    /// assert_eq!(rect.standardize().size, CGSize::new(5.0, 2.0));
    /// ```
    #[inline]
    #[doc(alias = "CGRectStandardize")]
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
    /// use objc2_foundation::{CGPoint, CGRect, CGSize};
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

/// A point in a Cartesian coordinate system.
///
/// This is a convenience alias for [`CGPoint`]. For ease of use, it is
/// available on all platforms, though in practice it is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nspoint?language=objc).
pub type NSPoint = CGPoint;

/// A two-dimensional size.
///
/// This is a convenience alias for [`CGSize`]. For ease of use, it is
/// available on all platforms, though in practice it is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nssize?language=objc).
pub type NSSize = CGSize;

/// A rectangle.
///
/// This is a convenience alias for [`CGRect`]. For ease of use, it is
/// available on all platforms, though in practice it is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrect?language=objc).
pub type NSRect = CGRect;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRectEdge(pub NSUInteger);

unsafe impl Encode for NSRectEdge {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSRectEdge {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[allow(non_upper_case_globals)]
impl NSRectEdge {
    #[doc(alias = "NSRectEdgeMinX")]
    pub const MinX: Self = Self(0);
    #[doc(alias = "NSRectEdgeMinY")]
    pub const MinY: Self = Self(1);
    #[doc(alias = "NSRectEdgeMaxX")]
    pub const MaxX: Self = Self(2);
    #[doc(alias = "NSRectEdgeMaxY")]
    pub const MaxY: Self = Self(3);
    pub const NSMinXEdge: Self = Self(NSRectEdge::MinX.0);
    pub const NSMinYEdge: Self = Self(NSRectEdge::MinY.0);
    pub const NSMaxXEdge: Self = Self(NSRectEdge::MaxX.0);
    pub const NSMaxYEdge: Self = Self(NSRectEdge::MaxY.0);
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

    // We know the Rust implementation handles NaN, infinite, negative zero
    // and so on properly, so let's ensure that NSEqualXXX handles these as
    // well (so that we're confident that the implementations are equivalent).
    #[test]
    #[cfg(any(
        all(target_vendor = "apple", target_os = "macos"), // or macabi
        feature = "gnustep-1-7"
    ))]
    #[cfg(feature = "NSGeometry")]
    fn test_partial_eq() {
        use crate::Foundation::{NSEqualPoints, NSEqualRects, NSEqualSizes};

        // We assume that comparisons handle e.g. `x` and `y` in the same way,
        // therefore we set the coordinates / dimensions to the same.
        let cases: &[(CGFloat, CGFloat)] = &[
            (0.0, 0.0),
            (-0.0, -0.0),
            (0.0, -0.0),
            (1.0, 1.0 + CGFloat::EPSILON),
            (0.0, CGFloat::MIN_POSITIVE),
            (0.0, CGFloat::EPSILON),
            (1.0, 1.0),
            (1.0, -1.0),
            // Infinity
            (CGFloat::INFINITY, CGFloat::INFINITY),
            (CGFloat::INFINITY, CGFloat::NEG_INFINITY),
            (CGFloat::NEG_INFINITY, CGFloat::NEG_INFINITY),
            // NaN
            (CGFloat::NAN, 0.0),
            (CGFloat::NAN, 1.0),
            (CGFloat::NAN, CGFloat::NAN),
            (CGFloat::NAN, -CGFloat::NAN),
            (-CGFloat::NAN, -CGFloat::NAN),
            (CGFloat::NAN, CGFloat::INFINITY),
        ];

        for case in cases {
            let point_a = NSPoint::new(case.0, case.1);
            let point_b = NSPoint::new(case.0, case.1);
            let actual = unsafe { NSEqualPoints(point_a, point_b).as_bool() };
            assert_eq!(point_a == point_b, actual);

            if case.0 >= 0.0 && case.1 >= 0.0 {
                let size_a = NSSize::new(case.0, case.1);
                let size_b = NSSize::new(case.0, case.1);
                let actual = unsafe { NSEqualSizes(size_a, size_b).as_bool() };
                assert_eq!(size_a == size_b, actual);

                let rect_a = NSRect::new(point_a, size_a);
                let rect_b = NSRect::new(point_b, size_b);
                let actual = unsafe { NSEqualRects(rect_a, rect_b).as_bool() };
                assert_eq!(rect_a == rect_b, actual);
            }
        }
    }
}
