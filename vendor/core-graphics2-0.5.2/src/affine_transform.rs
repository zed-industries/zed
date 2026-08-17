use crate::{
    base::CGFloat,
    geometry::{CGPoint, CGRect, CGSize, CGVector},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CGAffineTransform {
    pub a: CGFloat,
    pub b: CGFloat,
    pub c: CGFloat,
    pub d: CGFloat,
    pub tx: CGFloat,
    pub ty: CGFloat,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CGAffineTransformComponents {
    pub scale: CGSize,
    pub horizontal_shear: CGFloat,
    pub rotation: CGFloat,
    pub translation: CGVector,
}

pub const CGAffineTransformIdentity: CGAffineTransform = CGAffineTransform {
    a: 1.0,
    b: 0.0,
    c: 0.0,
    d: 1.0,
    tx: 0.0,
    ty: 0.0,
};

extern "C" {
    pub fn CGAffineTransformMake(a: CGFloat, b: CGFloat, c: CGFloat, d: CGFloat, tx: CGFloat, ty: CGFloat) -> CGAffineTransform;
    pub fn CGAffineTransformMakeTranslation(tx: CGFloat, ty: CGFloat) -> CGAffineTransform;
    pub fn CGAffineTransformMakeScale(sx: CGFloat, sy: CGFloat) -> CGAffineTransform;
    pub fn CGAffineTransformMakeRotation(angle: CGFloat) -> CGAffineTransform;
    pub fn CGAffineTransformIsIdentity(t: CGAffineTransform) -> bool;
    pub fn CGAffineTransformTranslate(t: CGAffineTransform, tx: CGFloat, ty: CGFloat) -> CGAffineTransform;
    pub fn CGAffineTransformScale(t: CGAffineTransform, sx: CGFloat, sy: CGFloat) -> CGAffineTransform;
    pub fn CGAffineTransformRotate(t: CGAffineTransform, angle: CGFloat) -> CGAffineTransform;
    pub fn CGAffineTransformInvert(t: CGAffineTransform) -> CGAffineTransform;
    pub fn CGAffineTransformConcat(t1: CGAffineTransform, t2: CGAffineTransform) -> CGAffineTransform;
    pub fn CGPointApplyAffineTransform(point: CGPoint, t: CGAffineTransform) -> CGPoint;
    pub fn CGSizeApplyAffineTransform(size: CGSize, t: CGAffineTransform) -> CGSize;
    pub fn CGRectApplyAffineTransform(rect: CGRect, t: CGAffineTransform) -> CGRect;
    pub fn CGAffineTransformDecompose(transform: CGAffineTransform) -> CGAffineTransformComponents;
    pub fn CGAffineTransformMakeWithComponents(components: CGAffineTransformComponents) -> CGAffineTransform;
}

impl CGAffineTransform {
    pub fn new(a: CGFloat, b: CGFloat, c: CGFloat, d: CGFloat, tx: CGFloat, ty: CGFloat) -> CGAffineTransform {
        unsafe { CGAffineTransformMake(a, b, c, d, tx, ty) }
    }

    pub fn new_translate(tx: CGFloat, ty: CGFloat) -> CGAffineTransform {
        unsafe { CGAffineTransformMakeTranslation(tx, ty) }
    }

    pub fn new_scale(sx: CGFloat, sy: CGFloat) -> CGAffineTransform {
        unsafe { CGAffineTransformMakeScale(sx, sy) }
    }

    pub fn new_rotate(angle: CGFloat) -> CGAffineTransform {
        unsafe { CGAffineTransformMakeRotation(angle) }
    }

    pub fn from_components(components: CGAffineTransformComponents) -> CGAffineTransform {
        unsafe { CGAffineTransformMakeWithComponents(components) }
    }

    pub fn translate(&self, tx: CGFloat, ty: CGFloat) -> CGAffineTransform {
        unsafe { CGAffineTransformTranslate(*self, tx, ty) }
    }

    pub fn scale(&self, sx: CGFloat, sy: CGFloat) -> CGAffineTransform {
        unsafe { CGAffineTransformScale(*self, sx, sy) }
    }

    pub fn rotate(&self, angle: CGFloat) -> CGAffineTransform {
        unsafe { CGAffineTransformRotate(*self, angle) }
    }

    pub fn is_identity(&self) -> bool {
        unsafe { CGAffineTransformIsIdentity(*self) }
    }

    pub fn invert(&self) -> CGAffineTransform {
        unsafe { CGAffineTransformInvert(*self) }
    }

    pub fn concat(&self, other: &CGAffineTransform) -> CGAffineTransform {
        unsafe { CGAffineTransformConcat(*self, *other) }
    }

    pub fn decompose(&self) -> CGAffineTransformComponents {
        unsafe { CGAffineTransformDecompose(*self) }
    }
}
