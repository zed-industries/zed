use std::ptr::null;

use core_foundation::{
    base::{CFType, CFTypeID, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    string::CFString,
};
use libc::{c_int, c_void, size_t};
#[cfg(feature = "objc")]
use objc2::encode::{Encoding, RefEncode};

use crate::{
    affine_transform::CGAffineTransform,
    base::CGFloat,
    color::{CGColor, CGColorRef},
    color_space::{CGColorRenderingIntent, CGColorSpace, CGColorSpaceRef},
    font::{CGFont, CGFontRef, CGGlyph},
    geometry::{CGPoint, CGRect, CGSize},
    gradient::{CGGradient, CGGradientDrawingOptions, CGGradientRef},
    image::{CGImage, CGImageRef},
    path::{CGLineCap, CGLineJoin, CGPath, CGPathRef},
    pattern::{CGPattern, CGPatternRef},
    shading::{CGShading, CGShadingRef},
};

#[repr(C)]
pub struct __CGContext(c_void);

pub type CGContextRef = *mut __CGContext;

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGPathDrawingMode {
    #[doc(alias = "kCGPathFill")]
    Fill         = 0,
    #[doc(alias = "kCGPathEOFill")]
    EOFill       = 1,
    #[doc(alias = "kCGPathStroke")]
    Stroke       = 2,
    #[doc(alias = "kCGPathFillStroke")]
    FillStroke   = 3,
    #[doc(alias = "kCGPathEOFillStroke")]
    EOFillStroke = 4,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGTextDrawingMode {
    #[doc(alias = "kCGTextFill")]
    Fill           = 0,
    #[doc(alias = "kCGTextStroke")]
    Stroke         = 1,
    #[doc(alias = "kCGTextFillStroke")]
    FillStroke     = 2,
    #[doc(alias = "kCGTextInvisible")]
    Invisible      = 3,
    #[doc(alias = "kCGTextFillClip")]
    FillClip       = 4,
    #[doc(alias = "kCGTextStrokeClip")]
    StrokeClip     = 5,
    #[doc(alias = "kCGTextFillStrokeClip")]
    FillStrokeClip = 6,
    #[doc(alias = "kCGTextClip")]
    Clip           = 7,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGTextEncoding {
    #[doc(alias = "kCGEncodingFontSpecific")]
    FontSpecific = 0,
    #[doc(alias = "kCGEncodingMacRoman")]
    MacRoman     = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGInterpolationQuality {
    #[doc(alias = "kCGInterpolationDefault")]
    Default = 0,
    #[doc(alias = "kCGInterpolationNone")]
    None    = 1,
    #[doc(alias = "kCGInterpolationLow")]
    Low     = 2,
    #[doc(alias = "kCGInterpolationMedium")]
    Medium  = 4,
    #[doc(alias = "kCGInterpolationHigh")]
    High    = 3,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGBlendMode {
    #[doc(alias = "kCGBlendModeNormal")]
    Normal          = 0,
    #[doc(alias = "kCGBlendModeMultiply")]
    Multiply        = 1,
    #[doc(alias = "kCGBlendModeScreen")]
    Screen          = 2,
    #[doc(alias = "kCGBlendModeOverlay")]
    Overlay         = 3,
    #[doc(alias = "kCGBlendModeDarken")]
    Darken          = 4,
    #[doc(alias = "kCGBlendModeLighten")]
    Lighten         = 5,
    #[doc(alias = "kCGBlendModeColorDodge")]
    ColorDodge      = 6,
    #[doc(alias = "kCGBlendModeColorBurn")]
    ColorBurn       = 7,
    #[doc(alias = "kCGBlendModeSoftLight")]
    SoftLight       = 8,
    #[doc(alias = "kCGBlendModeHardLight")]
    HardLight       = 9,
    #[doc(alias = "kCGBlendModeDifference")]
    Difference      = 10,
    #[doc(alias = "kCGBlendModeExclusion")]
    Exclusion       = 11,
    #[doc(alias = "kCGBlendModeHue")]
    Hue             = 12,
    #[doc(alias = "kCGBlendModeSaturation")]
    Saturation      = 13,
    #[doc(alias = "kCGBlendModeColor")]
    Color           = 14,
    #[doc(alias = "kCGBlendModeLuminosity")]
    Luminosity      = 15,
    #[doc(alias = "kCGBlendModeClear")]
    Clear           = 16, // R = 0
    #[doc(alias = "kCGBlendModeCopy")]
    Copy            = 17, // R = S
    #[doc(alias = "kCGBlendModeSourceIn")]
    SourceIn        = 18, // R = S*Da
    #[doc(alias = "kCGBlendModeSourceOut")]
    SourceOut       = 19, // R = S*(1 - Da)
    #[doc(alias = "kCGBlendModeSourceAtop")]
    SourceAtop      = 20, // R = S*Da + D*(1 - Sa)
    #[doc(alias = "kCGBlendModeDestinationOver")]
    DestinationOver = 21, // R = S*(1 - Da) + D
    #[doc(alias = "kCGBlendModeDestinationIn")]
    DestinationIn   = 22, // R = D*Sa
    #[doc(alias = "kCGBlendModeDestinationOut")]
    DestinationOut  = 23, // R = D*(1 - Sa)
    #[doc(alias = "kCGBlendModeDestinationAtop")]
    DestinationAtop = 24, // R = S*(1 - Da) + D*Sa
    #[doc(alias = "kCGBlendModeXOR")]
    XOR             = 25, // R = S*(1 - Da) + D*(1 - Sa)
    #[doc(alias = "kCGBlendModePlusDarker")]
    PlusDarker      = 26, // R = MAX(0, (1 - D) + (1 - S))
    #[doc(alias = "kCGBlendModePlusLighter")]
    PlusLighter     = 27, // R = MIN(1, S + D)
}

extern "C" {
    pub fn CGContextGetTypeID() -> CFTypeID;
    pub fn CGContextSaveGState(c: CGContextRef);
    pub fn CGContextRestoreGState(c: CGContextRef);
    pub fn CGContextScaleCTM(c: CGContextRef, sx: CGFloat, sy: CGFloat);
    pub fn CGContextTranslateCTM(c: CGContextRef, tx: CGFloat, ty: CGFloat);
    pub fn CGContextRotateCTM(c: CGContextRef, angle: CGFloat);
    pub fn CGContextConcatCTM(c: CGContextRef, transform: CGAffineTransform);
    pub fn CGContextGetCTM(c: CGContextRef) -> CGAffineTransform;
    pub fn CGContextSetLineWidth(c: CGContextRef, width: CGFloat);
    pub fn CGContextSetLineCap(c: CGContextRef, cap: CGLineCap);
    pub fn CGContextSetLineJoin(c: CGContextRef, join: CGLineJoin);
    pub fn CGContextSetMiterLimit(c: CGContextRef, limit: CGFloat);
    pub fn CGContextSetLineDash(c: CGContextRef, phase: CGFloat, lengths: *const CGFloat, count: size_t);
    pub fn CGContextSetFlatness(c: CGContextRef, flatness: CGFloat);
    pub fn CGContextSetAlpha(c: CGContextRef, alpha: CGFloat);
    pub fn CGContextSetBlendMode(c: CGContextRef, mode: CGBlendMode);
    pub fn CGContextBeginPath(c: CGContextRef);
    pub fn CGContextMoveToPoint(c: CGContextRef, x: CGFloat, y: CGFloat);
    pub fn CGContextAddLineToPoint(c: CGContextRef, x: CGFloat, y: CGFloat);
    pub fn CGContextAddCurveToPoint(c: CGContextRef, cp1x: CGFloat, cp1y: CGFloat, cp2x: CGFloat, cp2y: CGFloat, x: CGFloat, y: CGFloat);
    pub fn CGContextAddQuadCurveToPoint(c: CGContextRef, cpx: CGFloat, cpy: CGFloat, x: CGFloat, y: CGFloat);
    pub fn CGContextClosePath(c: CGContextRef);
    pub fn CGContextAddRect(c: CGContextRef, rect: CGRect);
    pub fn CGContextAddRects(c: CGContextRef, rects: *const CGRect, count: size_t);
    pub fn CGContextAddLines(c: CGContextRef, points: *const CGPoint, count: size_t);
    pub fn CGContextAddEllipseInRect(c: CGContextRef, rect: CGRect);
    pub fn CGContextAddArc(c: CGContextRef, x: CGFloat, y: CGFloat, radius: CGFloat, startAngle: CGFloat, endAngle: CGFloat, clockwise: bool);
    pub fn CGContextAddArcToPoint(c: CGContextRef, x1: CGFloat, y1: CGFloat, x2: CGFloat, y2: CGFloat, radius: CGFloat);
    pub fn CGContextAddPath(c: CGContextRef, path: CGPathRef);
    pub fn CGContextReplacePathWithStrokedPath(c: CGContextRef);
    pub fn CGContextIsPathEmpty(c: CGContextRef) -> bool;
    pub fn CGContextGetPathCurrentPoint(c: CGContextRef) -> CGPoint;
    pub fn CGContextGetPathBoundingBox(c: CGContextRef) -> CGRect;
    pub fn CGContextCopyPath(c: CGContextRef) -> CGPathRef;
    pub fn CGContextPathContainsPoint(c: CGContextRef, point: CGPoint, mode: CGPathDrawingMode) -> bool;
    pub fn CGContextDrawPath(c: CGContextRef, mode: CGPathDrawingMode);
    pub fn CGContextFillPath(c: CGContextRef);
    pub fn CGContextEOFillPath(c: CGContextRef);
    pub fn CGContextStrokePath(c: CGContextRef);
    pub fn CGContextFillRect(c: CGContextRef, rect: CGRect);
    pub fn CGContextFillRects(c: CGContextRef, rects: *const CGRect, count: size_t);
    pub fn CGContextStrokeRect(c: CGContextRef, rect: CGRect);
    pub fn CGContextStrokeRectWithWidth(c: CGContextRef, rect: CGRect, width: CGFloat);
    pub fn CGContextClearRect(c: CGContextRef, rect: CGRect);
    pub fn CGContextFillEllipseInRect(c: CGContextRef, rect: CGRect);
    pub fn CGContextStrokeEllipseInRect(c: CGContextRef, rect: CGRect);
    pub fn CGContextStrokeLineSegments(c: CGContextRef, points: *const CGPoint, count: size_t);
    pub fn CGContextClip(c: CGContextRef);
    pub fn CGContextEOClip(c: CGContextRef);
    pub fn CGContextResetClip(c: CGContextRef);
    pub fn CGContextClipToMask(c: CGContextRef, rect: CGRect, mask: CGImageRef);
    pub fn CGContextGetClipBoundingBox(c: CGContextRef) -> CGRect;
    pub fn CGContextClipToRect(c: CGContextRef, rect: CGRect);
    pub fn CGContextClipToRects(c: CGContextRef, rects: *const CGRect, count: size_t);
    pub fn CGContextSetFillColorWithColor(c: CGContextRef, color: CGColorRef);
    pub fn CGContextSetStrokeColorWithColor(c: CGContextRef, color: CGColorRef);
    pub fn CGContextSetFillColorSpace(c: CGContextRef, space: CGColorSpaceRef);
    pub fn CGContextSetStrokeColorSpace(c: CGContextRef, space: CGColorSpaceRef);
    pub fn CGContextSetFillColor(c: CGContextRef, components: *const CGFloat);
    pub fn CGContextSetStrokeColor(c: CGContextRef, components: *const CGFloat);
    pub fn CGContextSetFillPattern(c: CGContextRef, pattern: CGPatternRef, components: *const CGFloat);
    pub fn CGContextSetStrokePattern(c: CGContextRef, pattern: CGPatternRef, components: *const CGFloat);
    pub fn CGContextSetPatternPhase(c: CGContextRef, phase: CGSize);
    pub fn CGContextSetGrayFillColor(c: CGContextRef, gray: CGFloat, alpha: CGFloat);
    pub fn CGContextSetGrayStrokeColor(c: CGContextRef, gray: CGFloat, alpha: CGFloat);
    pub fn CGContextSetRGBFillColor(c: CGContextRef, red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat);
    pub fn CGContextSetRGBStrokeColor(c: CGContextRef, red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat);
    pub fn CGContextSetCMYKFillColor(c: CGContextRef, cyan: CGFloat, magenta: CGFloat, yellow: CGFloat, black: CGFloat, alpha: CGFloat);
    pub fn CGContextSetCMYKStrokeColor(c: CGContextRef, cyan: CGFloat, magenta: CGFloat, yellow: CGFloat, black: CGFloat, alpha: CGFloat);
    pub fn CGContextSetRenderingIntent(c: CGContextRef, intent: CGColorRenderingIntent);
    pub fn CGContextDrawImage(c: CGContextRef, rect: CGRect, image: CGImageRef);
    pub fn CGContextDrawTiledImage(c: CGContextRef, rect: CGRect, image: CGImageRef);
    pub fn CGContextGetInterpolationQuality(c: CGContextRef) -> CGInterpolationQuality;
    pub fn CGContextSetInterpolationQuality(c: CGContextRef, quality: CGInterpolationQuality);
    pub fn CGContextSetShadowWithColor(c: CGContextRef, offset: CGSize, blur: CGFloat, color: CGColorRef);
    pub fn CGContextSetShadow(c: CGContextRef, offset: CGSize, blur: CGFloat);
    pub fn CGContextDrawLinearGradient(
        c: CGContextRef,
        gradient: CGGradientRef,
        startPoint: CGPoint,
        endPoint: CGPoint,
        options: CGGradientDrawingOptions,
    );
    pub fn CGContextDrawRadialGradient(
        c: CGContextRef,
        gradient: CGGradientRef,
        startCenter: CGPoint,
        startRadius: CGFloat,
        endCenter: CGPoint,
        endRadius: CGFloat,
        options: CGGradientDrawingOptions,
    );
    pub fn CGContextDrawShading(c: CGContextRef, shading: CGShadingRef);
    pub fn CGContextSetCharacterSpacing(c: CGContextRef, spacing: CGFloat);
    pub fn CGContextSetTextPosition(c: CGContextRef, x: CGFloat, y: CGFloat);
    pub fn CGContextGetTextPosition(c: CGContextRef) -> CGPoint;
    pub fn CGContextSetTextMatrix(c: CGContextRef, t: CGAffineTransform);
    pub fn CGContextGetTextMatrix(c: CGContextRef) -> CGAffineTransform;
    pub fn CGContextSetTextDrawingMode(c: CGContextRef, mode: CGTextDrawingMode);
    pub fn CGContextSetFont(c: CGContextRef, font: CGFontRef);
    pub fn CGContextSetFontSize(c: CGContextRef, size: CGFloat);
    pub fn CGContextShowGlyphsAtPositions(c: CGContextRef, glyphs: *const CGGlyph, positions: *const CGPoint, count: size_t);
    pub fn CGContextBeginPage(c: CGContextRef, mediaBox: *const CGRect);
    pub fn CGContextEndPage(c: CGContextRef);
    pub fn CGContextRetain(c: CGContextRef) -> CGContextRef;
    pub fn CGContextRelease(c: CGContextRef);
    pub fn CGContextFlush(c: CGContextRef);
    pub fn CGContextSynchronize(c: CGContextRef);
    pub fn CGContextSetShouldAntialias(c: CGContextRef, shouldAntialias: bool);
    pub fn CGContextSetAllowsAntialiasing(c: CGContextRef, allowsAntialiasing: bool);
    pub fn CGContextSetShouldSmoothFonts(c: CGContextRef, shouldSmoothFonts: bool);
    pub fn CGContextSetAllowsFontSmoothing(c: CGContextRef, allowsFontSmoothing: bool);
    pub fn CGContextSetShouldSubpixelPositionFonts(c: CGContextRef, shouldSubpixelPositionFonts: bool);
    pub fn CGContextSetAllowsFontSubpixelPositioning(c: CGContextRef, allowsFontSubpixelPositioning: bool);
    pub fn CGContextSetShouldSubpixelQuantizeFonts(c: CGContextRef, shouldSubpixelQuantizeFonts: bool);
    pub fn CGContextSetAllowsFontSubpixelQuantization(c: CGContextRef, allowsFontSubpixelQuantization: bool);
    pub fn CGContextSetFontSmoothingStyle(c: CGContextRef, style: c_int);
    pub fn CGContextBeginTransparencyLayer(c: CGContextRef, auxInfo: CFDictionaryRef);
    pub fn CGContextBeginTransparencyLayerWithRect(c: CGContextRef, rect: CGRect, auxInfo: CFDictionaryRef);
    pub fn CGContextEndTransparencyLayer(c: CGContextRef);
    pub fn CGContextGetUserSpaceToDeviceSpaceTransform(c: CGContextRef) -> CGAffineTransform;
    pub fn CGContextConvertPointToDeviceSpace(c: CGContextRef, point: CGPoint) -> CGPoint;
    pub fn CGContextConvertPointToUserSpace(c: CGContextRef, point: CGPoint) -> CGPoint;
    pub fn CGContextConvertSizeToDeviceSpace(c: CGContextRef, size: CGSize) -> CGSize;
    pub fn CGContextConvertSizeToUserSpace(c: CGContextRef, size: CGSize) -> CGSize;
    pub fn CGContextConvertRectToDeviceSpace(c: CGContextRef, rect: CGRect) -> CGRect;
    pub fn CGContextConvertRectToUserSpace(c: CGContextRef, rect: CGRect) -> CGRect;
}

pub struct CGContext(CGContextRef);

impl Drop for CGContext {
    fn drop(&mut self) {
        unsafe { CGContextRelease(self.as_concrete_TypeRef()) }
    }
}

impl_TCFType!(CGContext, CGContextRef, CGContextGetTypeID);
impl_CFTypeDescription!(CGContext);

impl CGContext {
    pub fn save_state(&self) {
        unsafe { CGContextSaveGState(self.as_concrete_TypeRef()) }
    }

    pub fn restore_state(&self) {
        unsafe { CGContextRestoreGState(self.as_concrete_TypeRef()) }
    }

    pub fn scale(&self, sx: CGFloat, sy: CGFloat) {
        unsafe {
            CGContextScaleCTM(self.as_concrete_TypeRef(), sx, sy);
        }
    }

    pub fn translate(&self, tx: CGFloat, ty: CGFloat) {
        unsafe {
            CGContextTranslateCTM(self.as_concrete_TypeRef(), tx, ty);
        }
    }

    pub fn rotate(&self, angle: CGFloat) {
        unsafe {
            CGContextRotateCTM(self.as_concrete_TypeRef(), angle);
        }
    }

    pub fn concat_ctm(&self, transform: CGAffineTransform) {
        unsafe { CGContextConcatCTM(self.as_concrete_TypeRef(), transform) }
    }

    pub fn get_ctm(&self) -> CGAffineTransform {
        unsafe { CGContextGetCTM(self.as_concrete_TypeRef()) }
    }

    pub fn set_line_width(&self, width: CGFloat) {
        unsafe { CGContextSetLineWidth(self.as_concrete_TypeRef(), width) }
    }

    pub fn set_line_cap(&self, cap: CGLineCap) {
        unsafe { CGContextSetLineCap(self.as_concrete_TypeRef(), cap) }
    }

    pub fn set_line_join(&self, join: CGLineJoin) {
        unsafe { CGContextSetLineJoin(self.as_concrete_TypeRef(), join) }
    }

    pub fn set_line_dash(&self, phase: CGFloat, lengths: &[CGFloat]) {
        unsafe { CGContextSetLineDash(self.as_concrete_TypeRef(), phase, lengths.as_ptr(), lengths.len()) }
    }

    pub fn set_miter_limit(&self, limit: CGFloat) {
        unsafe { CGContextSetMiterLimit(self.as_concrete_TypeRef(), limit) }
    }

    pub fn set_flatness(&self, flatness: CGFloat) {
        unsafe { CGContextSetFlatness(self.as_concrete_TypeRef(), flatness) }
    }

    pub fn set_alpha(&self, alpha: CGFloat) {
        unsafe {
            CGContextSetAlpha(self.as_concrete_TypeRef(), alpha);
        }
    }

    pub fn set_blend_mode(&self, mode: CGBlendMode) {
        unsafe {
            CGContextSetBlendMode(self.as_concrete_TypeRef(), mode);
        }
    }

    pub fn begin_path(&self) {
        unsafe {
            CGContextBeginPath(self.as_concrete_TypeRef());
        }
    }

    pub fn close_path(&self) {
        unsafe {
            CGContextClosePath(self.as_concrete_TypeRef());
        }
    }

    pub fn move_to_point(&self, x: CGFloat, y: CGFloat) {
        unsafe {
            CGContextMoveToPoint(self.as_concrete_TypeRef(), x, y);
        }
    }

    pub fn add_line_to_point(&self, x: CGFloat, y: CGFloat) {
        unsafe {
            CGContextAddLineToPoint(self.as_concrete_TypeRef(), x, y);
        }
    }

    pub fn add_curve_to_point(&self, cp1x: CGFloat, cp1y: CGFloat, cp2x: CGFloat, cp2y: CGFloat, x: CGFloat, y: CGFloat) {
        unsafe {
            CGContextAddCurveToPoint(self.as_concrete_TypeRef(), cp1x, cp1y, cp2x, cp2y, x, y);
        }
    }

    pub fn add_quad_curve_to_point(&self, cpx: CGFloat, cpy: CGFloat, x: CGFloat, y: CGFloat) {
        unsafe {
            CGContextAddQuadCurveToPoint(self.as_concrete_TypeRef(), cpx, cpy, x, y);
        }
    }

    pub fn add_rect(&self, rect: CGRect) {
        unsafe {
            CGContextAddRect(self.as_concrete_TypeRef(), rect);
        }
    }

    pub fn add_rects(&self, rects: &[CGRect]) {
        unsafe {
            CGContextAddRects(self.as_concrete_TypeRef(), rects.as_ptr(), rects.len());
        }
    }

    pub fn add_lines(&self, points: &[CGPoint]) {
        unsafe {
            CGContextAddLines(self.as_concrete_TypeRef(), points.as_ptr(), points.len());
        }
    }

    pub fn add_ellipse_in_rect(&self, rect: CGRect) {
        unsafe {
            CGContextAddEllipseInRect(self.as_concrete_TypeRef(), rect);
        }
    }

    pub fn add_arc(&self, x: CGFloat, y: CGFloat, radius: CGFloat, start_angle: CGFloat, end_angle: CGFloat, clockwise: bool) {
        unsafe {
            CGContextAddArc(self.as_concrete_TypeRef(), x, y, radius, start_angle, end_angle, clockwise);
        }
    }

    pub fn add_arc_to_point(&self, x1: CGFloat, y1: CGFloat, x2: CGFloat, y2: CGFloat, radius: CGFloat) {
        unsafe {
            CGContextAddArcToPoint(self.as_concrete_TypeRef(), x1, y1, x2, y2, radius);
        }
    }

    pub fn add_path(&self, path: &CGPath) {
        unsafe {
            CGContextAddPath(self.as_concrete_TypeRef(), path.as_concrete_TypeRef());
        }
    }

    pub fn replace_path_with_stroked_path(&self) {
        unsafe { CGContextReplacePathWithStrokedPath(self.as_concrete_TypeRef()) }
    }

    pub fn is_path_empty(&self) -> bool {
        unsafe { CGContextIsPathEmpty(self.as_concrete_TypeRef()) }
    }

    pub fn get_path_current_point(&self) -> CGPoint {
        unsafe { CGContextGetPathCurrentPoint(self.as_concrete_TypeRef()) }
    }

    pub fn get_path_bounding_box(&self) -> CGRect {
        unsafe { CGContextGetPathBoundingBox(self.as_concrete_TypeRef()) }
    }

    pub fn copy_path(&self) -> Option<CGPath> {
        unsafe {
            let path = CGContextCopyPath(self.as_concrete_TypeRef());
            if path.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(path))
            }
        }
    }

    pub fn path_contains_point(&self, point: CGPoint, mode: CGPathDrawingMode) -> bool {
        unsafe { CGContextPathContainsPoint(self.as_concrete_TypeRef(), point, mode) }
    }

    pub fn draw_path(&self, mode: CGPathDrawingMode) {
        unsafe {
            CGContextDrawPath(self.as_concrete_TypeRef(), mode);
        }
    }

    pub fn fill_path(&self) {
        unsafe {
            CGContextFillPath(self.as_concrete_TypeRef());
        }
    }

    pub fn eo_fill_path(&self) {
        unsafe {
            CGContextEOFillPath(self.as_concrete_TypeRef());
        }
    }

    pub fn stroke_path(&self) {
        unsafe {
            CGContextStrokePath(self.as_concrete_TypeRef());
        }
    }

    pub fn fill_rect(&self, rect: CGRect) {
        unsafe { CGContextFillRect(self.as_concrete_TypeRef(), rect) }
    }

    pub fn fill_rects(&self, rects: &[CGRect]) {
        unsafe { CGContextFillRects(self.as_concrete_TypeRef(), rects.as_ptr(), rects.len()) }
    }

    pub fn stroke_rect(&self, rect: CGRect) {
        unsafe { CGContextStrokeRect(self.as_concrete_TypeRef(), rect) }
    }

    pub fn stroke_rect_with_width(&self, rect: CGRect, width: CGFloat) {
        unsafe { CGContextStrokeRectWithWidth(self.as_concrete_TypeRef(), rect, width) }
    }

    pub fn clear_rect(&self, rect: CGRect) {
        unsafe { CGContextClearRect(self.as_concrete_TypeRef(), rect) }
    }

    pub fn fill_ellipse_in_rect(&self, rect: CGRect) {
        unsafe { CGContextFillEllipseInRect(self.as_concrete_TypeRef(), rect) }
    }

    pub fn stroke_ellipse_in_rect(&self, rect: CGRect) {
        unsafe { CGContextStrokeEllipseInRect(self.as_concrete_TypeRef(), rect) }
    }

    pub fn stroke_line_segments(&self, points: &[CGPoint]) {
        unsafe { CGContextStrokeLineSegments(self.as_concrete_TypeRef(), points.as_ptr(), points.len()) }
    }

    pub fn clip(&self) {
        unsafe {
            CGContextClip(self.as_concrete_TypeRef());
        }
    }

    pub fn eo_clip(&self) {
        unsafe {
            CGContextEOClip(self.as_concrete_TypeRef());
        }
    }

    pub fn reset_clip(&self) {
        unsafe {
            CGContextResetClip(self.as_concrete_TypeRef());
        }
    }

    pub fn clip_to_mask(&self, rect: CGRect, image: &CGImage) {
        unsafe { CGContextClipToMask(self.as_concrete_TypeRef(), rect, image.as_concrete_TypeRef()) }
    }

    pub fn clip_bounding_box(&self) -> CGRect {
        unsafe { CGContextGetClipBoundingBox(self.as_concrete_TypeRef()) }
    }

    pub fn clip_to_rect(&self, rect: CGRect) {
        unsafe { CGContextClipToRect(self.as_concrete_TypeRef(), rect) }
    }

    pub fn clip_to_rects(&self, rects: &[CGRect]) {
        unsafe { CGContextClipToRects(self.as_concrete_TypeRef(), rects.as_ptr(), rects.len()) }
    }

    pub fn set_fill_color(&self, color: &CGColor) {
        unsafe {
            CGContextSetFillColorWithColor(self.as_concrete_TypeRef(), color.as_concrete_TypeRef());
        }
    }

    pub fn set_stroke_color(&self, color: &CGColor) {
        unsafe {
            CGContextSetStrokeColorWithColor(self.as_concrete_TypeRef(), color.as_concrete_TypeRef());
        }
    }

    pub fn set_fill_color_space(&self, space: &CGColorSpace) {
        unsafe {
            CGContextSetFillColorSpace(self.as_concrete_TypeRef(), space.as_concrete_TypeRef());
        }
    }

    pub fn set_stroke_color_space(&self, space: &CGColorSpace) {
        unsafe {
            CGContextSetStrokeColorSpace(self.as_concrete_TypeRef(), space.as_concrete_TypeRef());
        }
    }

    pub unsafe fn set_fill_pattern(&self, pattern: &CGPattern, components: &[CGFloat]) {
        unsafe {
            CGContextSetFillPattern(self.as_concrete_TypeRef(), pattern.as_concrete_TypeRef(), components.as_ptr());
        }
    }

    pub unsafe fn set_stroke_pattern(&self, pattern: &CGPattern, components: &[CGFloat]) {
        unsafe {
            CGContextSetStrokePattern(self.as_concrete_TypeRef(), pattern.as_concrete_TypeRef(), components.as_ptr());
        }
    }

    pub fn set_pattern_phase(&self, phase: CGSize) {
        unsafe {
            CGContextSetPatternPhase(self.as_concrete_TypeRef(), phase);
        }
    }

    pub fn set_gray_fill_color(&self, gray: CGFloat, alpha: CGFloat) {
        unsafe {
            CGContextSetGrayFillColor(self.as_concrete_TypeRef(), gray, alpha);
        }
    }

    pub fn set_gray_stroke_color(&self, gray: CGFloat, alpha: CGFloat) {
        unsafe {
            CGContextSetGrayStrokeColor(self.as_concrete_TypeRef(), gray, alpha);
        }
    }

    pub fn set_rgb_fill_color(&self, red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) {
        unsafe { CGContextSetRGBFillColor(self.as_concrete_TypeRef(), red, green, blue, alpha) }
    }

    pub fn set_rgb_stroke_color(&self, red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) {
        unsafe { CGContextSetRGBStrokeColor(self.as_concrete_TypeRef(), red, green, blue, alpha) }
    }

    pub fn set_cmyk_fill_color(&self, cyan: CGFloat, magenta: CGFloat, yellow: CGFloat, black: CGFloat, alpha: CGFloat) {
        unsafe {
            CGContextSetCMYKFillColor(self.as_concrete_TypeRef(), cyan, magenta, yellow, black, alpha);
        }
    }

    pub fn set_cmyk_stroke_color(&self, cyan: CGFloat, magenta: CGFloat, yellow: CGFloat, black: CGFloat, alpha: CGFloat) {
        unsafe {
            CGContextSetCMYKStrokeColor(self.as_concrete_TypeRef(), cyan, magenta, yellow, black, alpha);
        }
    }

    pub fn set_rendering_intent(&self, intent: CGColorRenderingIntent) {
        unsafe {
            CGContextSetRenderingIntent(self.as_concrete_TypeRef(), intent);
        }
    }

    pub fn draw_image(&self, rect: CGRect, image: &CGImage) {
        unsafe {
            CGContextDrawImage(self.as_concrete_TypeRef(), rect, image.as_concrete_TypeRef());
        }
    }

    pub fn draw_tiled_image(&self, rect: CGRect, image: &CGImage) {
        unsafe {
            CGContextDrawTiledImage(self.as_concrete_TypeRef(), rect, image.as_concrete_TypeRef());
        }
    }

    pub fn get_interpolation_quality(&self) -> CGInterpolationQuality {
        unsafe { CGContextGetInterpolationQuality(self.as_concrete_TypeRef()) }
    }

    pub fn set_interpolation_quality(&self, quality: CGInterpolationQuality) {
        unsafe {
            CGContextSetInterpolationQuality(self.as_concrete_TypeRef(), quality);
        }
    }

    pub fn draw_linear_gradient(&self, gradient: &CGGradient, start_point: CGPoint, end_point: CGPoint, options: CGGradientDrawingOptions) {
        unsafe {
            CGContextDrawLinearGradient(self.as_concrete_TypeRef(), gradient.as_concrete_TypeRef(), start_point, end_point, options);
        }
    }

    pub fn draw_radial_gradient(
        &self,
        gradient: &CGGradient,
        start_center: CGPoint,
        start_radius: CGFloat,
        end_center: CGPoint,
        end_radius: CGFloat,
        options: CGGradientDrawingOptions,
    ) {
        unsafe {
            CGContextDrawRadialGradient(
                self.as_concrete_TypeRef(),
                gradient.as_concrete_TypeRef(),
                start_center,
                start_radius,
                end_center,
                end_radius,
                options,
            );
        }
    }

    pub fn set_shadow(&self, offset: CGSize, blur: CGFloat) {
        unsafe {
            CGContextSetShadow(self.as_concrete_TypeRef(), offset, blur);
        }
    }

    pub fn set_shadow_with_color(&self, offset: CGSize, blur: CGFloat, color: &CGColor) {
        unsafe {
            CGContextSetShadowWithColor(self.as_concrete_TypeRef(), offset, blur, color.as_concrete_TypeRef());
        }
    }

    pub fn draw_shading(&self, shading: &CGShading) {
        unsafe {
            CGContextDrawShading(self.as_concrete_TypeRef(), shading.as_concrete_TypeRef());
        }
    }

    pub fn set_font(&self, font: &CGFont) {
        unsafe { CGContextSetFont(self.as_concrete_TypeRef(), font.as_concrete_TypeRef()) }
    }

    pub fn set_font_size(&self, size: CGFloat) {
        unsafe { CGContextSetFontSize(self.as_concrete_TypeRef(), size) }
    }

    pub fn set_text_matrix(&self, t: &CGAffineTransform) {
        unsafe { CGContextSetTextMatrix(self.as_concrete_TypeRef(), *t) }
    }

    pub fn set_text_drawing_mode(&self, mode: CGTextDrawingMode) {
        unsafe { CGContextSetTextDrawingMode(self.as_concrete_TypeRef(), mode) }
    }

    pub fn set_text_position(&self, x: CGFloat, y: CGFloat) {
        unsafe { CGContextSetTextPosition(self.as_concrete_TypeRef(), x, y) }
    }

    pub fn set_allows_font_smoothing(&self, allows_font_smoothing: bool) {
        unsafe { CGContextSetAllowsFontSmoothing(self.as_concrete_TypeRef(), allows_font_smoothing) }
    }

    pub fn set_should_smooth_fonts(&self, should_smooth_fonts: bool) {
        unsafe { CGContextSetShouldSmoothFonts(self.as_concrete_TypeRef(), should_smooth_fonts) }
    }

    pub fn set_allows_antialiasing(&self, allows_antialiasing: bool) {
        unsafe { CGContextSetAllowsAntialiasing(self.as_concrete_TypeRef(), allows_antialiasing) }
    }

    pub fn set_should_antialias(&self, should_antialias: bool) {
        unsafe { CGContextSetShouldAntialias(self.as_concrete_TypeRef(), should_antialias) }
    }

    pub fn set_allows_font_subpixel_quantization(&self, allows_font_subpixel_quantization: bool) {
        unsafe { CGContextSetAllowsFontSubpixelQuantization(self.as_concrete_TypeRef(), allows_font_subpixel_quantization) }
    }

    pub fn set_should_subpixel_quantize_fonts(&self, should_subpixel_quantize_fonts: bool) {
        unsafe { CGContextSetShouldSubpixelQuantizeFonts(self.as_concrete_TypeRef(), should_subpixel_quantize_fonts) }
    }

    pub fn set_allows_font_subpixel_positioning(&self, allows_font_subpixel_positioning: bool) {
        unsafe { CGContextSetAllowsFontSubpixelPositioning(self.as_concrete_TypeRef(), allows_font_subpixel_positioning) }
    }

    pub fn set_should_subpixel_position_fonts(&self, should_subpixel_position_fonts: bool) {
        unsafe { CGContextSetShouldSubpixelPositionFonts(self.as_concrete_TypeRef(), should_subpixel_position_fonts) }
    }

    pub fn set_font_smoothing_style(&self, style: i32) {
        unsafe {
            CGContextSetFontSmoothingStyle(self.as_concrete_TypeRef(), style as _);
        }
    }

    pub fn show_glyphs_at_positions(&self, glyphs: &[CGGlyph], positions: &[CGPoint]) {
        unsafe {
            let count = std::cmp::min(glyphs.len(), positions.len());
            CGContextShowGlyphsAtPositions(self.as_concrete_TypeRef(), glyphs.as_ptr(), positions.as_ptr(), count)
        }
    }

    pub fn begin_transparency_layer(&self, aux_info: Option<&CFDictionary<CFString, CFType>>) {
        unsafe {
            CGContextBeginTransparencyLayer(self.as_concrete_TypeRef(), aux_info.map_or(null(), |info| info.as_concrete_TypeRef()));
        }
    }

    pub fn flush(&self) {
        unsafe { CGContextFlush(self.as_concrete_TypeRef()) }
    }

    pub fn synchronize(&self) {
        unsafe { CGContextSynchronize(self.as_concrete_TypeRef()) }
    }

    pub fn begin_page(&self, media_box: &CGRect) {
        unsafe { CGContextBeginPage(self.as_concrete_TypeRef(), media_box) }
    }

    pub fn end_page(&self) {
        unsafe { CGContextEndPage(self.as_concrete_TypeRef()) }
    }

    pub fn begin_transparency_layer_with_rect(&self, rect: CGRect, aux_info: Option<&CFDictionary<CFString, CFType>>) {
        unsafe {
            CGContextBeginTransparencyLayerWithRect(self.as_concrete_TypeRef(), rect, aux_info.map_or(null(), |info| info.as_concrete_TypeRef()));
        }
    }

    pub fn end_transparency_layer(&self) {
        unsafe {
            CGContextEndTransparencyLayer(self.as_concrete_TypeRef());
        }
    }

    pub fn get_user_space_to_device_space_transform(&self) -> CGAffineTransform {
        unsafe { CGContextGetUserSpaceToDeviceSpaceTransform(self.as_concrete_TypeRef()) }
    }

    pub fn convert_point_to_device_space(&self, point: CGPoint) -> CGPoint {
        unsafe { CGContextConvertPointToDeviceSpace(self.as_concrete_TypeRef(), point) }
    }

    pub fn convert_point_to_user_space(&self, point: CGPoint) -> CGPoint {
        unsafe { CGContextConvertPointToUserSpace(self.as_concrete_TypeRef(), point) }
    }

    pub fn convert_size_to_device_space(&self, size: CGSize) -> CGSize {
        unsafe { CGContextConvertSizeToDeviceSpace(self.as_concrete_TypeRef(), size) }
    }

    pub fn convert_size_to_user_space(&self, size: CGSize) -> CGSize {
        unsafe { CGContextConvertSizeToUserSpace(self.as_concrete_TypeRef(), size) }
    }

    pub fn convert_rect_to_device_space(&self, rect: CGRect) -> CGRect {
        unsafe { CGContextConvertRectToDeviceSpace(self.as_concrete_TypeRef(), rect) }
    }

    pub fn convert_rect_to_user_space(&self, rect: CGRect) -> CGRect {
        unsafe { CGContextConvertRectToUserSpace(self.as_concrete_TypeRef(), rect) }
    }
}

#[cfg(feature = "objc")]
unsafe impl RefEncode for __CGContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("CGContext", &[]));
}
