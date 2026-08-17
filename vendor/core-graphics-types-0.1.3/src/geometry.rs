// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::base::CGFloat;
use core_foundation::base::TCFType;
use core_foundation::dictionary::CFDictionary;

pub const CG_ZERO_POINT: CGPoint = CGPoint { x: 0.0, y: 0.0 };

pub const CG_ZERO_SIZE: CGSize = CGSize {
    width: 0.0,
    height: 0.0,
};

pub const CG_ZERO_RECT: CGRect = CGRect {
    origin: CG_ZERO_POINT,
    size: CG_ZERO_SIZE,
};

pub const CG_AFFINE_TRANSFORM_IDENTITY: CGAffineTransform = CGAffineTransform {
    a: 1.0,
    b: 0.0,
    c: 0.0,
    d: 1.0,
    tx: 0.0,
    ty: 0.0,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    #[inline]
    pub fn new(width: CGFloat, height: CGFloat) -> CGSize {
        CGSize { width, height }
    }

    #[inline]
    pub fn apply_transform(&self, t: &CGAffineTransform) -> CGSize {
        unsafe { ffi::CGSizeApplyAffineTransform(*self, *t) }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

impl CGPoint {
    #[inline]
    pub fn new(x: CGFloat, y: CGFloat) -> CGPoint {
        CGPoint { x, y }
    }

    #[inline]
    pub fn apply_transform(&self, t: &CGAffineTransform) -> CGPoint {
        unsafe { ffi::CGPointApplyAffineTransform(*self, *t) }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

impl CGRect {
    #[inline]
    pub fn new(origin: &CGPoint, size: &CGSize) -> CGRect {
        CGRect {
            origin: *origin,
            size: *size,
        }
    }

    #[inline]
    pub fn inset(&self, size: &CGSize) -> CGRect {
        unsafe { ffi::CGRectInset(*self, size.width, size.height) }
    }

    #[inline]
    pub fn from_dict_representation(dict: &CFDictionary) -> Option<CGRect> {
        let mut rect = CGRect::new(&CGPoint::new(0., 0.), &CGSize::new(0., 0.));
        let result = unsafe {
            ffi::CGRectMakeWithDictionaryRepresentation(dict.as_concrete_TypeRef(), &mut rect)
        };
        if result == 0 {
            None
        } else {
            Some(rect)
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe {
            // I use one, as it seems that `YES` is not available from this crate.
            ffi::CGRectIsEmpty(*self) == 1
        }
    }

    #[inline]
    pub fn is_intersects(&self, other: &CGRect) -> bool {
        unsafe {
            // I use one, as it seems that `YES` is not available from this crate.
            ffi::CGRectIntersectsRect(*self, *other) == 1
        }
    }

    #[inline]
    pub fn apply_transform(&self, t: &CGAffineTransform) -> CGRect {
        unsafe { ffi::CGRectApplyAffineTransform(*self, *t) }
    }

    #[inline]
    pub fn contains(&self, point: &CGPoint) -> bool {
        unsafe { ffi::CGRectContainsPoint(*self, *point) == 1 }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGAffineTransform {
    pub a: CGFloat,
    pub b: CGFloat,
    pub c: CGFloat,
    pub d: CGFloat,
    pub tx: CGFloat,
    pub ty: CGFloat,
}

impl CGAffineTransform {
    #[inline]
    pub fn new(
        a: CGFloat,
        b: CGFloat,
        c: CGFloat,
        d: CGFloat,
        tx: CGFloat,
        ty: CGFloat,
    ) -> CGAffineTransform {
        CGAffineTransform { a, b, c, d, tx, ty }
    }

    #[inline]
    pub fn invert(&self) -> CGAffineTransform {
        unsafe { ffi::CGAffineTransformInvert(*self) }
    }
}

mod ffi {
    use crate::base::{boolean_t, CGFloat};
    use crate::geometry::{CGAffineTransform, CGPoint, CGRect, CGSize};
    use core_foundation::dictionary::CFDictionaryRef;

    #[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
    extern "C" {
        pub fn CGRectInset(rect: CGRect, dx: CGFloat, dy: CGFloat) -> CGRect;
        pub fn CGRectMakeWithDictionaryRepresentation(
            dict: CFDictionaryRef,
            rect: *mut CGRect,
        ) -> boolean_t;
        pub fn CGRectIsEmpty(rect: CGRect) -> boolean_t;
        pub fn CGRectIntersectsRect(rect1: CGRect, rect2: CGRect) -> boolean_t;

        pub fn CGAffineTransformInvert(t: CGAffineTransform) -> CGAffineTransform;

        pub fn CGPointApplyAffineTransform(point: CGPoint, t: CGAffineTransform) -> CGPoint;
        pub fn CGRectApplyAffineTransform(rect: CGRect, t: CGAffineTransform) -> CGRect;
        pub fn CGSizeApplyAffineTransform(size: CGSize, t: CGAffineTransform) -> CGSize;

        pub fn CGRectContainsPoint(rect: CGRect, point: CGPoint) -> boolean_t;
    }
}
