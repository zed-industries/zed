use std::{
    fmt::{Debug, Error, Formatter},
    ops::Deref,
    ptr::null,
    slice,
};

use core_foundation::{
    base::{CFTypeID, TCFType},
    impl_CFTypeDescription, impl_TCFType,
};
use libc::{c_void, size_t};
#[cfg(feature = "objc")]
use objc2::encode::{Encoding, RefEncode};

use crate::{
    affine_transform::CGAffineTransform,
    base::CGFloat,
    geometry::{CGPoint, CGRect, CGRectZero},
};

#[repr(C)]
pub struct __CGPath(c_void);

pub type CGMutablePathRef = *mut __CGPath;
pub type CGPathRef = *const __CGPath;

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGLineJoin {
    #[doc(alias = "kCGLineJoinMiter")]
    Miter,
    #[doc(alias = "kCGLineJoinRound")]
    Round,
    #[doc(alias = "kCGLineJoinBevel")]
    Bevel,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGLineCap {
    #[doc(alias = "kCGLineCapButt")]
    Butt,
    #[doc(alias = "kCGLineCapRound")]
    Round,
    #[doc(alias = "kCGLineCapSquare")]
    Square,
}

extern "C" {
    pub fn CGPathGetTypeID() -> CFTypeID;
    pub fn CGPathCreateMutable() -> CGMutablePathRef;
    pub fn CGPathCreateCopy(path: CGPathRef) -> CGPathRef;
    pub fn CGPathCreateCopyByTransformingPath(path: CGPathRef, transform: *const CGAffineTransform) -> CGPathRef;
    pub fn CGPathCreateMutableCopy(path: CGPathRef) -> CGMutablePathRef;
    pub fn CGPathCreateMutableCopyByTransformingPath(path: CGPathRef, transform: *const CGAffineTransform) -> CGMutablePathRef;
    pub fn CGPathCreateWithRect(rect: CGRect, transform: *const CGAffineTransform) -> CGPathRef;
    pub fn CGPathCreateWithEllipseInRect(rect: CGRect, transform: *const CGAffineTransform) -> CGPathRef;
    pub fn CGPathCreateWithRoundedRect(rect: CGRect, cornerWidth: CGFloat, cornerHeight: CGFloat, transform: *const CGAffineTransform) -> CGPathRef;
    pub fn CGPathAddRoundedRect(
        path: CGMutablePathRef,
        transform: *const CGAffineTransform,
        rect: CGRect,
        cornerWidth: CGFloat,
        cornerHeight: CGFloat,
    );
    pub fn CGPathCreateCopyByDashingPath(
        path: CGPathRef,
        transform: *const CGAffineTransform,
        phase: CGFloat,
        lengths: *const CGFloat,
        count: size_t,
    ) -> CGPathRef;
    pub fn CGPathCreateCopyByStrokingPath(
        path: CGPathRef,
        transform: *const CGAffineTransform,
        lineWidth: CGFloat,
        lineCap: CGLineCap,
        lineJoin: CGLineJoin,
        miterLimit: CGFloat,
    ) -> CGPathRef;
    pub fn CGPathRetain(path: CGPathRef) -> CGPathRef;
    pub fn CGPathRelease(path: CGPathRef);
    pub fn CGPathEqualToPath(path1: CGPathRef, path2: CGPathRef) -> bool;
    pub fn CGPathMoveToPoint(path: CGMutablePathRef, transform: *const CGAffineTransform, x: CGFloat, y: CGFloat);
    pub fn CGPathAddLineToPoint(path: CGMutablePathRef, transform: *const CGAffineTransform, x: CGFloat, y: CGFloat);
    pub fn CGPathAddQuadCurveToPoint(path: CGMutablePathRef, transform: *const CGAffineTransform, cpx: CGFloat, cpy: CGFloat, x: CGFloat, y: CGFloat);
    pub fn CGPathAddCurveToPoint(
        path: CGMutablePathRef,
        transform: *const CGAffineTransform,
        cp1x: CGFloat,
        cp1y: CGFloat,
        cp2x: CGFloat,
        cp2y: CGFloat,
        x: CGFloat,
        y: CGFloat,
    );
    pub fn CGPathCloseSubpath(path: CGMutablePathRef);
    pub fn CGPathAddRect(path: CGMutablePathRef, transform: *const CGAffineTransform, rect: CGRect);
    pub fn CGPathAddRects(path: CGMutablePathRef, transform: *const CGAffineTransform, rects: *const CGRect, count: size_t);
    pub fn CGPathAddLines(path: CGMutablePathRef, transform: *const CGAffineTransform, points: *const CGPoint, count: size_t);
    pub fn CGPathAddEllipseInRect(path: CGMutablePathRef, transform: *const CGAffineTransform, rect: CGRect);
    pub fn CGPathAddRelativeArc(
        path: CGMutablePathRef,
        transform: *const CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
        radius: CGFloat,
        startAngle: CGFloat,
        delta: CGFloat,
    );
    pub fn CGPathAddArc(
        path: CGMutablePathRef,
        transform: *const CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
        radius: CGFloat,
        startAngle: CGFloat,
        endAngle: CGFloat,
        clockwise: bool,
    );
    pub fn CGPathAddArcToPoint(
        path: CGMutablePathRef,
        transform: *const CGAffineTransform,
        x1: CGFloat,
        y1: CGFloat,
        x2: CGFloat,
        y2: CGFloat,
        radius: CGFloat,
    );
    pub fn CGPathAddPath(path1: CGMutablePathRef, transform: *const CGAffineTransform, path2: CGPathRef);
    pub fn CGPathIsEmpty(path: CGPathRef) -> bool;
    pub fn CGPathIsRect(path: CGPathRef, rect: *mut CGRect) -> bool;
    pub fn CGPathGetCurrentPoint(path: CGPathRef) -> CGPoint;
    pub fn CGPathGetBoundingBox(path: CGPathRef) -> CGRect;
    pub fn CGPathGetPathBoundingBox(path: CGPathRef) -> CGRect;
    pub fn CGPathContainsPoint(path: CGPathRef, transform: *const CGAffineTransform, point: CGPoint, eoFill: bool) -> bool;
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGPathElementType {
    #[doc(alias = "kCGPathElementMoveToPoint")]
    MoveToPoint,
    #[doc(alias = "kCGPathElementAddLineToPoint")]
    AddLineToPoint,
    #[doc(alias = "kCGPathElementAddQuadCurveToPoint")]
    AddQuadCurveToPoint,
    #[doc(alias = "kCGPathElementAddCurveToPoint")]
    AddCurveToPoint,
    #[doc(alias = "kCGPathElementCloseSubpath")]
    CloseSubpath,
}

#[repr(C)]
pub struct CGPathElement {
    pub element_type: CGPathElementType,
    points: *mut CGPoint,
}

pub type CGPathApplierFunction = extern "C" fn(*mut c_void, *const CGPathElement);

extern "C" {
    pub fn CGPathApply(path: CGPathRef, info: *mut c_void, function: CGPathApplierFunction);
    pub fn CGPathCreateCopyByNormalizingPath(path: CGPathRef, evenOddFillRule: bool) -> CGPathRef;
    pub fn CGPathCreateCopyByUnioningPath(path: CGPathRef, maskPath: CGPathRef, evenOddFillRule: bool) -> CGPathRef;
    pub fn CGPathCreateCopyByIntersectingPath(path: CGPathRef, maskPath: CGPathRef, evenOddFillRule: bool) -> CGPathRef;
    pub fn CGPathCreateCopyBySubtractingPath(path: CGPathRef, maskPath: CGPathRef, evenOddFillRule: bool) -> CGPathRef;
    pub fn CGPathCreateCopyBySymmetricDifferenceOfPath(path: CGPathRef, maskPath: CGPathRef, evenOddFillRule: bool) -> CGPathRef;
    pub fn CGPathCreateCopyOfLineBySubtractingPath(path: CGPathRef, maskPath: CGPathRef, evenOddFillRule: bool) -> CGPathRef;
    pub fn CGPathCreateCopyOfLineByIntersectingPath(path: CGPathRef, maskPath: CGPathRef, evenOddFillRule: bool) -> CGPathRef;
    pub fn CGPathCreateSeparateComponents(path: CGPathRef, evenOddFillRule: bool) -> CGPathRef;
    pub fn CGPathCreateCopyByFlattening(path: CGPathRef, flatteningThreshold: CGFloat) -> CGPathRef;
    pub fn CGPathIntersectsPath(path1: CGPathRef, path2: CGPathRef, evenOddFillRule: bool) -> bool;
}

pub struct CGPath(CGPathRef);

impl Drop for CGPath {
    fn drop(&mut self) {
        unsafe { CGPathRelease(self.0) }
    }
}

impl_TCFType!(CGPath, CGPathRef, CGPathGetTypeID);
impl_CFTypeDescription!(CGPath);

pub struct CGMutablePath(CGMutablePathRef);

impl Drop for CGMutablePath {
    fn drop(&mut self) {
        unsafe { CGPathRelease(self.0) }
    }
}

impl_TCFType!(CGMutablePath, CGMutablePathRef, CGPathGetTypeID);
impl_CFTypeDescription!(CGMutablePath);

impl CGPathElement {
    pub fn points(&self) -> &[CGPoint] {
        unsafe {
            match self.element_type {
                CGPathElementType::MoveToPoint | CGPathElementType::AddLineToPoint => slice::from_raw_parts(self.points, 1),
                CGPathElementType::AddQuadCurveToPoint => slice::from_raw_parts(self.points, 2),
                CGPathElementType::AddCurveToPoint => slice::from_raw_parts(self.points, 3),
                CGPathElementType::CloseSubpath => &[],
            }
        }
    }
}

impl Debug for CGPathElement {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(formatter, "{:?}: {:?}", self.element_type, self.points())
    }
}

pub struct CGPathElementRef {
    element: *const CGPathElement,
}

impl CGPathElementRef {
    fn new(element: *const CGPathElement) -> CGPathElementRef {
        CGPathElementRef {
            element,
        }
    }
}

impl Deref for CGPathElementRef {
    type Target = CGPathElement;
    fn deref(&self) -> &CGPathElement {
        unsafe { &*self.element }
    }
}

impl CGPath {
    pub fn new_copy(&self) -> Option<CGPath> {
        unsafe {
            let path = CGPathCreateCopy(self.as_concrete_TypeRef());
            if path.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(path))
            }
        }
    }

    pub fn new_copy_by_transforming_path(&self, transform: Option<&CGAffineTransform>) -> Option<CGPath> {
        unsafe {
            let path = CGPathCreateCopyByTransformingPath(self.as_concrete_TypeRef(), transform.map_or(null(), |t| t as *const CGAffineTransform));
            if path.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(path))
            }
        }
    }

    pub fn new_mutable_copy(&self) -> Option<CGMutablePath> {
        unsafe {
            let path = CGPathCreateMutableCopy(self.as_concrete_TypeRef());
            if path.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(path))
            }
        }
    }

    pub fn new_mutable_copy_by_transforming_path(&self, transform: Option<&CGAffineTransform>) -> Option<CGMutablePath> {
        unsafe {
            let path =
                CGPathCreateMutableCopyByTransformingPath(self.as_concrete_TypeRef(), transform.map_or(null(), |t| t as *const CGAffineTransform));
            if path.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(path))
            }
        }
    }

    pub fn from_rect(rect: CGRect, transform: Option<&CGAffineTransform>) -> CGPath {
        unsafe {
            let path = CGPathCreateWithRect(rect, transform.map_or(null(), |t| t as *const CGAffineTransform));
            TCFType::wrap_under_create_rule(path)
        }
    }

    pub fn from_ellipse_in_rect(rect: CGRect, transform: Option<&CGAffineTransform>) -> CGPath {
        unsafe {
            let path = CGPathCreateWithEllipseInRect(rect, transform.map_or(null(), |t| t as *const CGAffineTransform));
            TCFType::wrap_under_create_rule(path)
        }
    }

    pub fn from_rounded_rect(rect: CGRect, corner_width: CGFloat, corner_height: CGFloat, transform: Option<&CGAffineTransform>) -> CGPath {
        unsafe {
            let path = CGPathCreateWithRoundedRect(rect, corner_width, corner_height, transform.map_or(null(), |t| t as *const CGAffineTransform));
            TCFType::wrap_under_create_rule(path)
        }
    }

    pub fn new_copy_by_dashing_path(&self, transform: Option<&CGAffineTransform>, phase: CGFloat, lengths: &[CGFloat]) -> Option<CGPath> {
        unsafe {
            let path = CGPathCreateCopyByDashingPath(
                self.as_concrete_TypeRef(),
                transform.map_or(null(), |t| t as *const CGAffineTransform),
                phase,
                lengths.as_ptr(),
                lengths.len(),
            );
            if path.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(path))
            }
        }
    }

    pub fn new_copy_by_stroking_path(
        &self,
        transform: Option<&CGAffineTransform>,
        line_width: CGFloat,
        line_cap: CGLineCap,
        line_join: CGLineJoin,
        miter_limit: CGFloat,
    ) -> Option<CGPath> {
        unsafe {
            let path = CGPathCreateCopyByStrokingPath(
                self.as_concrete_TypeRef(),
                transform.map_or(null(), |t| t as *const CGAffineTransform),
                line_width,
                line_cap,
                line_join,
                miter_limit,
            );
            if path.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(path))
            }
        }
    }

    pub fn equal(&self, path: &CGPath) -> bool {
        unsafe { CGPathEqualToPath(self.as_concrete_TypeRef(), path.as_concrete_TypeRef()) }
    }

    pub fn current_point(&self) -> CGPoint {
        unsafe { CGPathGetCurrentPoint(self.as_concrete_TypeRef()) }
    }

    pub fn bounding_box(&self) -> CGRect {
        unsafe { CGPathGetBoundingBox(self.as_concrete_TypeRef()) }
    }

    pub fn path_bounding_box(&self) -> CGRect {
        unsafe { CGPathGetPathBoundingBox(self.as_concrete_TypeRef()) }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { CGPathIsEmpty(self.as_concrete_TypeRef()) }
    }

    pub fn is_rect(&self) -> Option<CGRect> {
        let mut rect = CGRectZero;
        let result = unsafe { CGPathIsRect(self.as_concrete_TypeRef(), &mut rect) };
        if result {
            Some(rect)
        } else {
            None
        }
    }

    pub fn contains_point(&self, transform: Option<&CGAffineTransform>, point: CGPoint, eo_fill: bool) -> bool {
        unsafe { CGPathContainsPoint(self.as_concrete_TypeRef(), transform.map_or(null(), |t| t as *const CGAffineTransform), point, eo_fill) }
    }

    pub fn apply<F>(&self, closure: F)
    where
        F: FnMut(CGPathElementRef),
    {
        unsafe {
            CGPathApply(self.as_concrete_TypeRef(), &closure as *const _ as *mut c_void, callback::<F>);
        }

        extern "C" fn callback<F>(info: *mut c_void, element: *const CGPathElement)
        where
            F: FnMut(CGPathElementRef),
        {
            unsafe {
                let closure = info as *mut F;
                (*closure)(CGPathElementRef::new(element));
            }
        }
    }
}

impl CGMutablePath {
    pub fn new() -> CGMutablePath {
        unsafe { TCFType::wrap_under_create_rule(CGPathCreateMutable()) }
    }

    pub fn add_rounded_rect(&self, transform: Option<&CGAffineTransform>, rect: CGRect, corner_width: CGFloat, corner_height: CGFloat) {
        unsafe {
            CGPathAddRoundedRect(
                self.as_concrete_TypeRef(),
                transform.map_or(null(), |t| t as *const CGAffineTransform),
                rect,
                corner_width,
                corner_height,
            )
        }
    }

    pub fn add_line_to_point(&self, transform: Option<&CGAffineTransform>, x: CGFloat, y: CGFloat) {
        unsafe { CGPathAddLineToPoint(self.as_concrete_TypeRef(), transform.map_or(null(), |t| t as *const CGAffineTransform), x, y) }
    }

    pub fn add_quad_curve_to_point(&self, transform: Option<&CGAffineTransform>, cpx: CGFloat, cpy: CGFloat, x: CGFloat, y: CGFloat) {
        unsafe { CGPathAddQuadCurveToPoint(self.as_concrete_TypeRef(), transform.map_or(null(), |t| t as *const CGAffineTransform), cpx, cpy, x, y) }
    }

    pub fn add_curve_to_point(
        &self,
        transform: Option<&CGAffineTransform>,
        cp1x: CGFloat,
        cp1y: CGFloat,
        cp2x: CGFloat,
        cp2y: CGFloat,
        x: CGFloat,
        y: CGFloat,
    ) {
        unsafe {
            CGPathAddCurveToPoint(
                self.as_concrete_TypeRef(),
                transform.map_or(null(), |t| t as *const CGAffineTransform),
                cp1x,
                cp1y,
                cp2x,
                cp2y,
                x,
                y,
            )
        }
    }

    pub fn add_rect(&self, transform: Option<&CGAffineTransform>, rect: CGRect) {
        unsafe { CGPathAddRect(self.as_concrete_TypeRef(), transform.map_or(null(), |t| t as *const CGAffineTransform), rect) }
    }

    pub fn add_rects(&self, transform: Option<&CGAffineTransform>, rects: &[CGRect]) {
        unsafe {
            CGPathAddRects(self.as_concrete_TypeRef(), transform.map_or(null(), |t| t as *const CGAffineTransform), rects.as_ptr(), rects.len())
        }
    }

    pub fn add_lines(&self, transform: Option<&CGAffineTransform>, points: &[CGPoint]) {
        unsafe {
            CGPathAddLines(self.as_concrete_TypeRef(), transform.map_or(null(), |t| t as *const CGAffineTransform), points.as_ptr(), points.len())
        }
    }

    pub fn add_ellipse_in_rect(&self, transform: Option<&CGAffineTransform>, rect: CGRect) {
        unsafe { CGPathAddEllipseInRect(self.as_concrete_TypeRef(), transform.map_or(null(), |t| t as *const CGAffineTransform), rect) }
    }

    pub fn add_relative_arc(
        &self,
        transform: Option<&CGAffineTransform>,
        x: CGFloat,
        y: CGFloat,
        radius: CGFloat,
        start_angle: CGFloat,
        delta: CGFloat,
    ) {
        unsafe {
            CGPathAddRelativeArc(
                self.as_concrete_TypeRef(),
                transform.map_or(null(), |t| t as *const CGAffineTransform),
                x,
                y,
                radius,
                start_angle,
                delta,
            )
        }
    }

    pub fn add_arc(
        &self,
        transform: Option<&CGAffineTransform>,
        x: CGFloat,
        y: CGFloat,
        radius: CGFloat,
        start_angle: CGFloat,
        end_angle: CGFloat,
        clockwise: bool,
    ) {
        unsafe {
            CGPathAddArc(
                self.as_concrete_TypeRef(),
                transform.map_or(null(), |t| t as *const CGAffineTransform),
                x,
                y,
                radius,
                start_angle,
                end_angle,
                clockwise,
            )
        }
    }

    pub fn add_arc_to_point(&self, transform: Option<&CGAffineTransform>, x1: CGFloat, y1: CGFloat, x2: CGFloat, y2: CGFloat, radius: CGFloat) {
        unsafe {
            CGPathAddArcToPoint(self.as_concrete_TypeRef(), transform.map_or(null(), |t| t as *const CGAffineTransform), x1, y1, x2, y2, radius)
        }
    }

    pub fn add_path(&self, transform: Option<&CGAffineTransform>, path: &CGPath) {
        unsafe { CGPathAddPath(self.as_concrete_TypeRef(), transform.map_or(null(), |t| t as *const CGAffineTransform), path.as_concrete_TypeRef()) }
    }

    pub fn close_subpath(&self) {
        unsafe { CGPathCloseSubpath(self.as_concrete_TypeRef()) }
    }

    pub fn move_to_point(&self, transform: Option<&CGAffineTransform>, x: CGFloat, y: CGFloat) {
        unsafe { CGPathMoveToPoint(self.as_concrete_TypeRef(), transform.map_or(null(), |t| t as *const CGAffineTransform), x, y) }
    }

    pub fn to_immutable(&self) -> CGPath {
        unsafe { CGPath::wrap_under_get_rule(self.as_concrete_TypeRef()) }
    }
}

#[cfg(feature = "objc")]
unsafe impl RefEncode for __CGPath {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("CGPath", &[]));
}
