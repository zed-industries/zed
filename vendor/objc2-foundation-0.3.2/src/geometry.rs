use objc2::encode::{Encode, Encoding, RefEncode};
use objc2::ffi::NSUInteger;

#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::{CGPoint, CGRect, CGSize};

/// A point in a Cartesian coordinate system.
///
/// This is a convenience alias for [`CGPoint`]. For ease of use, it is
/// available on all platforms, though in practice it is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nspoint?language=objc).
#[cfg(feature = "objc2-core-foundation")]
pub type NSPoint = CGPoint;

/// A two-dimensional size.
///
/// This is a convenience alias for [`CGSize`]. For ease of use, it is
/// available on all platforms, though in practice it is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nssize?language=objc).
#[cfg(feature = "objc2-core-foundation")]
pub type NSSize = CGSize;

/// A rectangle.
///
/// This is a convenience alias for [`CGRect`]. For ease of use, it is
/// available on all platforms, though in practice it is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrect?language=objc).
#[cfg(feature = "objc2-core-foundation")]
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
    // We know the Rust implementation handles NaN, infinite, negative zero
    // and so on properly, so let's ensure that NSEqualXXX handles these as
    // well (so that we're confident that the implementations are equivalent).
    #[test]
    #[cfg(any(
        all(target_vendor = "apple", target_os = "macos"), // or macabi
        feature = "gnustep-1-7"
    ))]
    #[cfg(feature = "objc2-core-foundation")]
    fn test_partial_eq() {
        use super::*;
        use crate::{NSEqualPoints, NSEqualRects, NSEqualSizes};
        use objc2_core_foundation::CGFloat;

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
            let actual = NSEqualPoints(point_a, point_b);
            assert_eq!(point_a == point_b, actual);

            if case.0 >= 0.0 && case.1 >= 0.0 {
                let size_a = NSSize::new(case.0, case.1);
                let size_b = NSSize::new(case.0, case.1);
                let actual = NSEqualSizes(size_a, size_b);
                assert_eq!(size_a == size_b, actual);

                let rect_a = NSRect::new(point_a, size_a);
                let rect_b = NSRect::new(point_b, size_b);
                let actual = NSEqualRects(rect_a, rect_b);
                assert_eq!(rect_a == rect_b, actual);
            }
        }
    }
}
