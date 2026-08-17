use cfg_if::cfg_if;
use core_foundation::{
    base::TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
};
#[cfg(feature = "objc")]
use objc2::encode::{Encode, Encoding, RefEncode};

use crate::{affine_transform::*, base::CGFloat};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CGVector {
    pub dx: CGFloat,
    pub dy: CGFloat,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGRectEdge {
    #[doc(alias = "CGRectMinXEdge")]
    MinX,
    #[doc(alias = "CGRectMinYEdge")]
    MinY,
    #[doc(alias = "CGRectMaxXEdge")]
    MaxX,
    #[doc(alias = "CGRectMaxYEdge")]
    MaxY,
}

pub const CGPointZero: CGPoint = CGPoint {
    x: 0.0,
    y: 0.0,
};
pub const CGSizeZero: CGSize = CGSize {
    width: 0.0,
    height: 0.0,
};
pub const CGRectZero: CGRect = CGRect {
    origin: CGPointZero,
    size: CGSizeZero,
};
pub const CGRectNull: CGRect = CGRect {
    origin: CGPoint {
        x: CGFloat::MAX,
        y: CGFloat::MAX,
    },
    size: CGSizeZero,
};
pub const CGRectInfinite: CGRect = CGRect {
    origin: CGPoint {
        x: CGFloat::MIN,
        y: CGFloat::MIN,
    },
    size: CGSize {
        width: CGFloat::MAX,
        height: CGFloat::MAX,
    },
};

extern "C" {
    pub fn CGRectGetMinX(rect: CGRect) -> CGFloat;
    pub fn CGRectGetMidX(rect: CGRect) -> CGFloat;
    pub fn CGRectGetMaxX(rect: CGRect) -> CGFloat;
    pub fn CGRectGetMinY(rect: CGRect) -> CGFloat;
    pub fn CGRectGetMidY(rect: CGRect) -> CGFloat;
    pub fn CGRectGetMaxY(rect: CGRect) -> CGFloat;
    pub fn CGRectGetWidth(rect: CGRect) -> CGFloat;
    pub fn CGRectGetHeight(rect: CGRect) -> CGFloat;
    pub fn CGRectStandardize(rect: CGRect) -> CGRect;
    pub fn CGRectIsEmpty(rect: CGRect) -> bool;
    pub fn CGRectIsNull(rect: CGRect) -> bool;
    pub fn CGRectIsInfinite(rect: CGRect) -> bool;
    pub fn CGRectInset(rect: CGRect, dx: CGFloat, dy: CGFloat) -> CGRect;
    pub fn CGRectIntegral(rect: CGRect) -> CGRect;
    pub fn CGRectUnion(rect1: CGRect, rect2: CGRect) -> CGRect;
    pub fn CGRectIntersection(rect1: CGRect, rect2: CGRect) -> CGRect;
    pub fn CGRectOffset(rect: CGRect, dx: CGFloat, dy: CGFloat) -> CGRect;
    pub fn CGRectDivide(rect: CGRect, slice: *mut CGRect, remainder: *mut CGRect, amount: CGFloat, edge: CGRectEdge);
    pub fn CGRectContainsPoint(rect: CGRect, point: CGPoint) -> bool;
    pub fn CGRectContainsRect(rect1: CGRect, rect2: CGRect) -> bool;
    pub fn CGRectIntersectsRect(rect1: CGRect, rect2: CGRect) -> bool;
    pub fn CGPointCreateDictionaryRepresentation(point: CGPoint) -> CFDictionaryRef;
    pub fn CGPointMakeWithDictionaryRepresentation(dict: CFDictionaryRef, point: *mut CGPoint) -> bool;
    pub fn CGSizeCreateDictionaryRepresentation(size: CGSize) -> CFDictionaryRef;
    pub fn CGSizeMakeWithDictionaryRepresentation(dict: CFDictionaryRef, size: *mut CGSize) -> bool;
    pub fn CGRectCreateDictionaryRepresentation(rect: CGRect) -> CFDictionaryRef;
    pub fn CGRectMakeWithDictionaryRepresentation(dict: CFDictionaryRef, rect: *mut CGRect) -> bool;
}

impl CGPoint {
    pub fn new(x: CGFloat, y: CGFloat) -> CGPoint {
        CGPoint {
            x,
            y,
        }
    }

    pub fn from_dict_representation(dict: &CFDictionary) -> Option<Self> {
        let mut point = CGPoint::default();
        let result = unsafe { CGPointMakeWithDictionaryRepresentation(dict.as_concrete_TypeRef(), &mut point) };
        if result {
            Some(point)
        } else {
            None
        }
    }

    pub fn apply_transform(&self, transform: &CGAffineTransform) -> CGPoint {
        unsafe { CGPointApplyAffineTransform(*self, *transform) }
    }
}

impl CGSize {
    pub fn new(width: CGFloat, height: CGFloat) -> CGSize {
        CGSize {
            width,
            height,
        }
    }

    pub fn from_dict_representation(dict: &CFDictionary) -> Option<Self> {
        let mut size = CGSize::default();
        let result = unsafe { CGSizeMakeWithDictionaryRepresentation(dict.as_concrete_TypeRef(), &mut size) };
        if result {
            Some(size)
        } else {
            None
        }
    }

    pub fn apply_transform(&self, transform: &CGAffineTransform) -> CGSize {
        unsafe { CGSizeApplyAffineTransform(*self, *transform) }
    }
}

impl CGRect {
    pub fn new(x: CGFloat, y: CGFloat, width: CGFloat, height: CGFloat) -> CGRect {
        CGRect {
            origin: CGPoint::new(x, y),
            size: CGSize::new(width, height),
        }
    }

    pub fn from_dict_representation(dict: &CFDictionary) -> Option<Self> {
        let mut rect = CGRect::default();
        let result = unsafe { CGRectMakeWithDictionaryRepresentation(dict.as_concrete_TypeRef(), &mut rect) };
        if result {
            Some(rect)
        } else {
            None
        }
    }

    pub fn min(&self) -> (CGFloat, CGFloat) {
        unsafe { (CGRectGetMinX(*self), CGRectGetMinY(*self)) }
    }
    pub fn mid(&self) -> (CGFloat, CGFloat) {
        unsafe { (CGRectGetMidX(*self), CGRectGetMidY(*self)) }
    }

    pub fn max(&self) -> (CGFloat, CGFloat) {
        unsafe { (CGRectGetMaxX(*self), CGRectGetMaxY(*self)) }
    }

    pub fn width(&self) -> CGFloat {
        unsafe { CGRectGetWidth(*self) }
    }

    pub fn height(&self) -> CGFloat {
        unsafe { CGRectGetHeight(*self) }
    }

    pub fn standardize(&self) -> CGRect {
        unsafe { CGRectStandardize(*self) }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { CGRectIsEmpty(*self) }
    }

    pub fn is_null(&self) -> bool {
        unsafe { CGRectIsNull(*self) }
    }

    pub fn is_infinite(&self) -> bool {
        unsafe { CGRectIsInfinite(*self) }
    }

    pub fn inset(&self, dx: CGFloat, dy: CGFloat) -> CGRect {
        unsafe { CGRectInset(*self, dx, dy) }
    }

    pub fn integral(&self) -> CGRect {
        unsafe { CGRectIntegral(*self) }
    }

    pub fn union(&self, rect: &CGRect) -> CGRect {
        unsafe { CGRectUnion(*self, *rect) }
    }

    pub fn intersection(&self, rect: &CGRect) -> CGRect {
        unsafe { CGRectIntersection(*self, *rect) }
    }

    pub fn offset(&self, dx: CGFloat, dy: CGFloat) -> CGRect {
        unsafe { CGRectOffset(*self, dx, dy) }
    }

    pub fn divide(&self, slice: &mut CGRect, remainder: &mut CGRect, amount: CGFloat, edge: CGRectEdge) {
        unsafe { CGRectDivide(*self, slice, remainder, amount, edge) }
    }

    pub fn contains_point(&self, point: &CGPoint) -> bool {
        unsafe { CGRectContainsPoint(*self, *point) }
    }

    pub fn contains_rect(&self, rect: &CGRect) -> bool {
        unsafe { CGRectContainsRect(*self, *rect) }
    }

    pub fn intersects_rect(&self, rect: &CGRect) -> bool {
        unsafe { CGRectIntersectsRect(*self, *rect) }
    }

    pub fn apply_transform(&self, transform: &CGAffineTransform) -> CGRect {
        unsafe { CGRectApplyAffineTransform(*self, *transform) }
    }
}

cfg_if!(
    if #[cfg(feature = "objc")] {
        unsafe impl Encode for CGPoint {
            const ENCODING: Encoding =
                Encoding::Struct("CGPoint", &[CGFloat::ENCODING, CGFloat::ENCODING]);
        }

        unsafe impl RefEncode for CGPoint {
            const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
        }

        unsafe impl Encode for CGSize {
            const ENCODING: Encoding =
                Encoding::Struct("CGSize", &[CGFloat::ENCODING, CGFloat::ENCODING]);
        }

        unsafe impl RefEncode for CGSize {
            const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
        }

        unsafe impl Encode for CGVector {
            const ENCODING: Encoding =
                Encoding::Struct("CGVector", &[CGFloat::ENCODING, CGFloat::ENCODING]);
        }

        unsafe impl RefEncode for CGVector {
            const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
        }

        unsafe impl Encode for CGRect {
            const ENCODING: Encoding =
                Encoding::Struct("CGRect", &[CGPoint::ENCODING, CGSize::ENCODING]);
        }

        unsafe impl RefEncode for CGRect {
            const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
        }
    }
);
