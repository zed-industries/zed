use std::ptr::null;

use core_foundation::{
    base::{CFType, CFTypeID, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    string::CFString,
};
use libc::c_void;

use crate::{
    context::{CGContext, CGContextRef},
    geometry::{CGPoint, CGRect, CGSize},
};

#[repr(C)]
pub struct __CGLayer(c_void);

pub type CGLayerRef = *mut __CGLayer;

extern "C" {
    pub fn CGLayerCreateWithContext(context: CGContextRef, size: CGSize, auxiliaryInfo: CFDictionaryRef) -> CGLayerRef;
    pub fn CGLayerRetain(layer: CGLayerRef) -> CGLayerRef;
    pub fn CGLayerRelease(layer: CGLayerRef);
    pub fn CGLayerGetSize(layer: CGLayerRef) -> CGSize;
    pub fn CGLayerGetContext(layer: CGLayerRef) -> CGContextRef;
    pub fn CGContextDrawLayerInRect(context: CGContextRef, rect: CGRect, layer: CGLayerRef);
    pub fn CGContextDrawLayerAtPoint(context: CGContextRef, point: CGPoint, layer: CGLayerRef);
    pub fn CGLayerGetTypeID() -> CFTypeID;
}

pub struct CGLayer(CGLayerRef);

impl Drop for CGLayer {
    fn drop(&mut self) {
        unsafe { CGLayerRelease(self.0) }
    }
}

impl_TCFType!(CGLayer, CGLayerRef, CGLayerGetTypeID);
impl_CFTypeDescription!(CGLayer);

impl CGLayer {
    pub fn new_with_context(context: &CGContext, size: CGSize, auxiliary_info: Option<&CFDictionary<CFString, CFType>>) -> Option<Self> {
        unsafe {
            let layer = CGLayerCreateWithContext(context.as_concrete_TypeRef(), size, auxiliary_info.map_or(null(), |ai| ai.as_concrete_TypeRef()));
            if layer.is_null() {
                None
            } else {
                Some(CGLayer(layer))
            }
        }
    }

    pub fn size(&self) -> CGSize {
        unsafe { CGLayerGetSize(self.as_concrete_TypeRef()) }
    }

    pub fn context(&self) -> Option<CGContext> {
        unsafe {
            let context = CGLayerGetContext(self.as_concrete_TypeRef());
            if context.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(context))
            }
        }
    }

    pub fn draw_in_rect(&self, context: &CGContext, rect: CGRect) {
        unsafe { CGContextDrawLayerInRect(context.as_concrete_TypeRef(), rect, self.as_concrete_TypeRef()) }
    }

    pub fn draw_at_point(&self, context: &CGContext, point: CGPoint) {
        unsafe { CGContextDrawLayerAtPoint(context.as_concrete_TypeRef(), point, self.as_concrete_TypeRef()) }
    }
}

impl CGContext {
    pub fn draw_layer_in_rect(&self, layer: &CGLayer, rect: CGRect) {
        layer.draw_in_rect(self, rect)
    }

    pub fn draw_layer_at_point(&self, layer: &CGLayer, point: CGPoint) {
        layer.draw_at_point(self, point)
    }
}
