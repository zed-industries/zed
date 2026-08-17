// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFType, CFTypeRef, TCFType};
use core_foundation::date::CFTimeInterval;
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::{CFString, CFStringRef};
use core_graphics::base::CGFloat;
use core_graphics::color::{CGColor, SysCGColorRef};
use core_graphics::color_space::CGColorSpace;
use core_graphics::context::CGContext;
use core_graphics::geometry::{CGAffineTransform, CGPoint, CGRect, CGSize};
use core_graphics::path::{CGPath, SysCGPathRef};
use foreign_types::ForeignType;
use std::ops::Mul;
use std::ptr;

use appkit::CGLContextObj;
use base::{BOOL, id, nil, YES};
use foundation::NSUInteger;

// CABase.h

pub fn current_media_time() -> CFTimeInterval {
    unsafe {
        CACurrentMediaTime()
    }
}

// CALayer.h

pub struct CALayer(id);

unsafe impl Send for CALayer {}
unsafe impl Sync for CALayer {}

impl Clone for CALayer {
    #[inline]
    fn clone(&self) -> CALayer {
        unsafe {
            CALayer(msg_send![self.id(), retain])
        }
    }
}

impl Drop for CALayer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            msg_send![self.id(), release]
        }
    }
}

impl CALayer {
    #[inline]
    pub fn id(&self) -> id {
        self.0
    }

    #[inline]
    pub fn new() -> CALayer {
        unsafe {
            CALayer(msg_send![class!(CALayer), layer])
        }
    }

    #[inline]
    pub fn from_layer(other: &CALayer) -> CALayer {
        unsafe {
            let layer: id = msg_send![class!(CALayer), alloc];
            CALayer(msg_send![layer, initWithLayer:other.id()])
        }
    }

    #[inline]
    pub fn presentation_layer(&self) -> CALayer {
        unsafe {
            CALayer(msg_send![self.id(), presentationLayer])
        }
    }

    #[inline]
    pub fn model_layer(&self) -> CALayer {
        unsafe {
            CALayer(msg_send![self.id(), modelLayer])
        }
    }

    #[inline]
    pub fn default_value_for_key(key: &CFString) -> id {
        unsafe {
            msg_send![class!(CALayer), defaultValueForKey:(key.as_CFTypeRef())]
        }
    }

    #[inline]
    pub fn needs_display_for_key(key: &CFString) -> bool {
        unsafe {
            let flag: BOOL = msg_send![class!(CALayer), needsDisplayForKey:(key.as_CFTypeRef())];
            flag == YES
        }
    }

    #[inline]
    pub fn should_archive_value_for_key(key: &CFString) -> bool {
        unsafe {
            let flag: BOOL = msg_send![class!(CALayer), shouldArchiveValueForKey:(key.as_CFTypeRef())];
            flag == YES
        }
    }

    #[inline]
    pub fn bounds(&self) -> CGRect {
        unsafe {
            msg_send![self.id(), bounds]
        }
    }

    #[inline]
    pub fn set_bounds(&self, bounds: &CGRect) {
        unsafe {
            msg_send![self.id(), setBounds:*bounds]
        }
    }

    #[inline]
    pub fn position(&self) -> CGPoint {
        unsafe {
            msg_send![self.id(), position]
        }
    }

    #[inline]
    pub fn set_position(&self, position: &CGPoint) {
        unsafe {
            msg_send![self.id(), setPosition:*position]
        }
    }

    #[inline]
    pub fn z_position(&self) -> CGFloat {
        unsafe {
            msg_send![self.id(), zPosition]
        }
    }

    #[inline]
    pub fn set_z_position(&self, z_position: CGFloat) {
        unsafe {
            msg_send![self.id(), setZPosition:z_position]
        }
    }

    #[inline]
    pub fn anchor_point(&self) -> CGPoint {
        unsafe {
            msg_send![self.id(), anchorPoint]
        }
    }

    #[inline]
    pub fn set_anchor_point(&self, anchor_point: &CGPoint) {
        unsafe {
            msg_send![self.id(), setAnchorPoint:*anchor_point]
        }
    }

    #[inline]
    pub fn anchor_point_z(&self) -> CGFloat {
        unsafe {
            msg_send![self.id(), anchorPointZ]
        }
    }

    #[inline]
    pub fn set_anchor_point_z(&self, anchor_point_z: CGFloat) {
        unsafe {
            msg_send![self.id(), setAnchorPointZ:anchor_point_z]
        }
    }

    #[inline]
    pub fn transform(&self) -> CATransform3D {
        unsafe {
            msg_send![self.id(), transform]
        }
    }

    #[inline]
    pub fn set_transform(&self, transform: &CATransform3D) {
        unsafe {
            msg_send![self.id(), setTransform:*transform]
        }
    }

    #[inline]
    pub fn affine_transform(&self) -> CGAffineTransform {
        unsafe {
            msg_send![self.id(), affineTransform]
        }
    }

    #[inline]
    pub fn set_affine_transform(&self, affine_transform: &CGAffineTransform) {
        unsafe {
            msg_send![self.id(), setAffineTransform:*affine_transform]
        }
    }

    #[inline]
    pub fn frame(&self) -> CGRect {
        unsafe {
            msg_send![self.id(), frame]
        }
    }

    #[inline]
    pub fn set_frame(&self, frame: &CGRect) {
        unsafe {
            msg_send![self.id(), setFrame:*frame]
        }
    }

    #[inline]
    pub fn is_hidden(&self) -> bool {
        unsafe {
            let flag: BOOL = msg_send![self.id(), isHidden];
            flag == YES
        }
    }

    #[inline]
    pub fn set_hidden(&self, hidden: bool) {
        unsafe {
            msg_send![self.id(), setHidden:hidden as BOOL]
        }
    }

    #[inline]
    pub fn is_double_sided(&self) -> bool {
        unsafe {
            let flag: BOOL = msg_send![self.id(), isDoubleSided];
            flag == YES
        }
    }

    #[inline]
    pub fn set_double_sided(&self, double_sided: bool) {
        unsafe {
            msg_send![self.id(), setDoubleSided:double_sided as BOOL]
        }
    }

    #[inline]
    pub fn is_geometry_flipped(&self) -> bool {
        unsafe {
            let flag: BOOL = msg_send![self.id(), isGeometryFlipped];
            flag == YES
        }
    }

    #[inline]
    pub fn set_geometry_flipped(&self, geometry_flipped: bool) {
        unsafe {
            msg_send![self.id(), setGeometryFlipped:geometry_flipped as BOOL]
        }
    }

    #[inline]
    pub fn contents_are_flipped(&self) -> bool {
        unsafe {
            let flag: BOOL = msg_send![self.id(), contentsAreFlipped];
            flag == YES
        }
    }

    #[inline]
    pub fn superlayer(&self) -> Option<CALayer> {
        unsafe {
            let superlayer: id = msg_send![self.id(), superlayer];
            if superlayer.is_null() {
                None
            } else {
                Some(CALayer(superlayer))
            }
        }
    }

    #[inline]
    pub fn remove_from_superlayer(&self) {
        unsafe {
            msg_send![self.id(), removeFromSuperlayer]
        }
    }

    #[inline]
    pub fn sublayers(&self) -> CFArray<CALayer> {
        unsafe {
            let sublayers: CFArrayRef = msg_send![self.id(), sublayers];
            TCFType::wrap_under_create_rule(sublayers)
        }
    }

    #[inline]
    pub fn add_sublayer(&self, sublayer: &CALayer) {
        unsafe {
            msg_send![self.id(), addSublayer:sublayer.id()]
        }
    }

    #[inline]
    pub fn insert_sublayer_at_index(&self, sublayer: &CALayer, index: u32) {
        unsafe {
            msg_send![self.id(), insertSublayer:sublayer.id() atIndex:index]
        }
    }

    #[inline]
    pub fn insert_sublayer_below(&self, sublayer: &CALayer, sibling: &CALayer) {
        unsafe {
            msg_send![self.id(), insertSublayer:sublayer.id() below:sibling.id()]
        }
    }

    #[inline]
    pub fn insert_sublayer_above(&self, sublayer: &CALayer, sibling: &CALayer) {
        unsafe {
            msg_send![self.id(), insertSublayer:sublayer.id() above:sibling.id()]
        }
    }

    #[inline]
    pub fn replace_sublayer_with(&self, old_layer: &CALayer, new_layer: &CALayer) {
        unsafe {
            msg_send![self.id(), replaceSublayer:old_layer.id() with:new_layer.id()]
        }
    }

    #[inline]
    pub fn sublayer_transform(&self) -> CATransform3D {
        unsafe {
            msg_send![self.id(), sublayerTransform]
        }
    }

    #[inline]
    pub fn set_sublayer_transform(&self, sublayer_transform: CATransform3D) {
        unsafe {
            msg_send![self.id(), setSublayerTransform:sublayer_transform]
        }
    }

    #[inline]
    pub fn mask(&self) -> Option<CALayer> {
        unsafe {
            let mask: id = msg_send![self.id(), mask];
            if mask.is_null() {
                None
            } else {
                Some(CALayer(mask))
            }
        }
    }

    #[inline]
    pub fn set_mask(&self, mask: Option<CALayer>) {
        unsafe {
            match mask {
                None => msg_send![self.id(), setMask:nil],
                Some(mask) => msg_send![self.id(), setMask:(mask.id())],
            }
        }
    }

    #[inline]
    pub fn masks_to_bounds(&self) -> bool {
        unsafe {
            let flag: BOOL = msg_send![self.id(), masksToBounds];
            flag == YES
        }
    }

    #[inline]
    pub fn set_masks_to_bounds(&self, flag: bool) {
        unsafe {
            msg_send![self.id(), setMasksToBounds:flag as BOOL]
        }
    }

    #[inline]
    pub fn convert_point_from_layer(&self, point: &CGPoint, layer: Option<CALayer>) -> CGPoint {
        unsafe {
            let layer = match layer {
                None => nil,
                Some(ref layer) => layer.id(),
            };
            msg_send![self.id(), convertPoint:*point fromLayer:layer]
        }
    }

    #[inline]
    pub fn convert_point_to_layer(&self, point: &CGPoint, layer: Option<CALayer>) -> CGPoint {
        unsafe {
            let layer = match layer {
                None => nil,
                Some(ref layer) => layer.id(),
            };
            msg_send![self.id(), convertPoint:*point toLayer:layer]
        }
    }

    #[inline]
    pub fn convert_rect_from_layer(&self, rect: &CGRect, layer: Option<CALayer>) -> CGRect {
        unsafe {
            let layer = match layer {
                None => nil,
                Some(ref layer) => layer.id(),
            };
            msg_send![self.id(), convertRect:*rect fromLayer:layer]
        }
    }

    #[inline]
    pub fn convert_rect_to_layer(&self, rect: &CGRect, layer: Option<CALayer>) -> CGRect {
        unsafe {
            let layer = match layer {
                None => nil,
                Some(ref layer) => layer.id(),
            };
            msg_send![self.id(), convertRect:*rect toLayer:layer]
        }
    }

    #[inline]
    pub fn convert_time_from_layer(&self, time: CFTimeInterval, layer: Option<CALayer>)
                                   -> CFTimeInterval {
        unsafe {
            let layer = match layer {
                None => nil,
                Some(ref layer) => layer.id(),
            };
            msg_send![self.id(), convertTime:time fromLayer:layer]
        }
    }

    #[inline]
    pub fn convert_time_to_layer(&self, time: CFTimeInterval, layer: Option<CALayer>)
                                 -> CFTimeInterval {
        unsafe {
            let layer = match layer {
                None => nil,
                Some(ref layer) => layer.id(),
            };
            msg_send![self.id(), convertTime:time toLayer:layer]
        }
    }

    #[inline]
    pub fn hit_test(&self, point: &CGPoint) -> Option<CALayer> {
        unsafe {
            let layer: id = msg_send![self.id(), hitTest:*point];
            if layer == nil {
                None
            } else {
                Some(CALayer(layer))
            }
        }
    }

    #[inline]
    pub fn contains_point(&self, point: &CGPoint) -> bool {
        unsafe {
            let result: BOOL = msg_send![self.id(), containsPoint:*point];
            result == YES
        }
    }

    #[inline]
    pub fn contents(&self) -> id {
        unsafe {
            msg_send![self.id(), contents]
        }
    }

    #[inline]
    pub unsafe fn set_contents(&self, contents: id) {
        msg_send![self.id(), setContents:contents]
    }

    #[inline]
    pub fn contents_rect(&self) -> CGRect {
        unsafe {
            msg_send![self.id(), contentsRect]
        }
    }

    #[inline]
    pub fn set_contents_rect(&self, contents_rect: &CGRect) {
        unsafe {
            msg_send![self.id(), setContentsRect:*contents_rect]
        }
    }

    #[inline]
    pub fn contents_gravity(&self) -> ContentsGravity {
        unsafe {
            let string: CFStringRef = msg_send![self.id(), contentsGravity];
            ContentsGravity::from_CFString(TCFType::wrap_under_create_rule(string))
        }
    }

    #[inline]
    pub fn set_contents_gravity(&self, new_contents_gravity: ContentsGravity) {
        unsafe {
            let contents_gravity: CFString = new_contents_gravity.into_CFString();
            msg_send![self.id(), setContentsGravity:contents_gravity.as_CFTypeRef()]
        }
    }

    #[inline]
    pub fn contents_scale(&self) -> CGFloat {
        unsafe {
            msg_send![self.id(), contentsScale]
        }
    }

    #[inline]
    pub fn set_contents_scale(&self, new_contents_scale: CGFloat) {
        unsafe {
            msg_send![self.id(), setContentsScale:new_contents_scale]
        }
    }

    #[inline]
    pub fn contents_center(&self) -> CGRect {
        unsafe {
            msg_send![self.id(), contentsCenter]
        }
    }

    #[inline]
    pub fn set_contents_center(&self, new_rect: &CGRect) {
        unsafe {
            msg_send![self.id(), setContentsCenter:*new_rect]
        }
    }

    #[inline]
    pub fn contents_format(&self) -> ContentsFormat {
        unsafe {
            let string: CFStringRef = msg_send![self.id(), contentsFormat];
            ContentsFormat::from_CFString(TCFType::wrap_under_create_rule(string))
        }
    }

    #[inline]
    pub fn set_contents_format(&self, new_contents_format: ContentsFormat) {
        unsafe {
            let contents_format: CFString = new_contents_format.into_CFString();
            msg_send![self.id(), setContentsFormat:contents_format.as_CFTypeRef()]
        }
    }

    #[inline]
    pub fn minification_filter(&self) -> Filter {
        unsafe {
            let string: CFStringRef = msg_send![self.id(), minificationFilter];
            Filter::from_CFString(TCFType::wrap_under_create_rule(string))
        }
    }

    #[inline]
    pub fn set_minification_filter(&self, new_filter: Filter) {
        unsafe {
            let filter: CFString = new_filter.into_CFString();
            msg_send![self.id(), setMinificationFilter:filter.as_CFTypeRef()]
        }
    }

    #[inline]
    pub fn magnification_filter(&self) -> Filter {
        unsafe {
            let string: CFStringRef = msg_send![self.id(), magnificationFilter];
            Filter::from_CFString(TCFType::wrap_under_create_rule(string))
        }
    }

    #[inline]
    pub fn set_magnification_filter(&self, new_filter: Filter) {
        unsafe {
            let filter: CFString = new_filter.into_CFString();
            msg_send![self.id(), setMagnificationFilter:filter.as_CFTypeRef()]
        }
    }

    #[inline]
    pub fn minification_filter_bias(&self) -> f32 {
        unsafe {
            msg_send![self.id(), minificationFilterBias]
        }
    }

    #[inline]
    pub fn set_minification_filter_bias(&self, new_filter_bias: f32) {
        unsafe {
            msg_send![self.id(), setMinificationFilterBias:new_filter_bias]
        }
    }

    #[inline]
    pub fn is_opaque(&self) -> bool {
        unsafe {
            let flag: BOOL = msg_send![self.id(), isOpaque];
            flag == YES
        }
    }

    #[inline]
    pub fn set_opaque(&self, opaque: bool) {
        unsafe {
            msg_send![self.id(), setOpaque:opaque as BOOL]
        }
    }

    #[inline]
    pub fn display(&self) {
        unsafe {
            msg_send![self.id(), display]
        }
    }

    #[inline]
    pub fn set_needs_display(&self) {
        unsafe {
            msg_send![self.id(), setNeedsDisplay]
        }
    }

    #[inline]
    pub fn set_needs_display_in_rect(&self, rect: &CGRect) {
        unsafe {
            msg_send![self.id(), setNeedsDisplayInRect:*rect]
        }
    }

    #[inline]
    pub fn needs_display(&self) -> bool {
        unsafe {
            let flag: BOOL = msg_send![self.id(), needsDisplay];
            flag == YES
        }
    }

    #[inline]
    pub fn display_if_needed(&self) {
        unsafe {
            msg_send![self.id(), displayIfNeeded]
        }
    }

    #[inline]
    pub fn needs_display_on_bounds_change(&self) -> bool {
        unsafe {
            let flag: BOOL = msg_send![self.id(), needsDisplayOnBoundsChange];
            flag == YES
        }
    }

    #[inline]
    pub fn set_needs_display_on_bounds_change(&self, flag: bool) {
        unsafe {
            msg_send![self.id(), setNeedsDisplayOnBoundsChange:flag as BOOL]
        }
    }

    #[inline]
    pub fn draws_asynchronously(&self) -> bool {
        unsafe {
            let flag: BOOL = msg_send![self.id(), drawsAsynchronously];
            flag == YES
        }
    }

    #[inline]
    pub fn set_draws_asynchronously(&self, flag: bool) {
        unsafe {
            msg_send![self.id(), setDrawsAsynchronously:flag as BOOL]
        }
    }

    #[inline]
    pub fn draw_in_context(&self, context: &CGContext) {
        unsafe {
            msg_send![self.id(), drawInContext:(*context).as_ptr()]
        }
    }

    #[inline]
    pub fn render_in_context(&self, context: &CGContext) {
        unsafe {
            msg_send![self.id(), renderInContext:(*context).as_ptr()]
        }
    }

    #[inline]
    pub fn edge_antialiasing_mask(&self) -> EdgeAntialiasingMask {
        unsafe {
            EdgeAntialiasingMask::from_bits_truncate(msg_send![self.id(), edgeAntialiasingMask])
        }
    }

    #[inline]
    pub fn set_edge_antialiasing_mask(&self, mask: EdgeAntialiasingMask) {
        unsafe {
            msg_send![self.id(), setEdgeAntialiasingMask:mask.bits()]
        }
    }

    #[inline]
    pub fn background_color(&self) -> Option<CGColor> {
        unsafe {
            let color: SysCGColorRef = msg_send![self.id(), backgroundColor];
            if color.is_null() {
                None
            } else {
                Some(CGColor::wrap_under_get_rule(color))
            }
        }
    }

    #[inline]
    pub fn set_background_color(&self, color: Option<CGColor>) {
        unsafe {
            let color = match color {
                None => ptr::null(),
                Some(color) => color.as_CFTypeRef(),
            };
            msg_send![self.id(), setBackgroundColor:color]
        }
    }

    #[inline]
    pub fn corner_radius(&self) -> CGFloat {
        unsafe {
            msg_send![self.id(), cornerRadius]
        }
    }

    #[inline]
    pub fn set_corner_radius(&self, radius: CGFloat) {
        unsafe {
            msg_send![self.id(), setCornerRadius:radius]
        }
    }

    #[inline]
    pub fn masked_corners(&self) -> CornerMask {
        unsafe {
            CornerMask::from_bits_truncate(msg_send![self.id(), maskedCorners])
        }
    }

    #[inline]
    pub fn set_masked_corners(&self, mask: CornerMask) {
        unsafe {
            msg_send![self.id(), setCornerMask:mask.bits()]
        }
    }

    #[inline]
    pub fn border_width(&self) -> CGFloat {
        unsafe {
            msg_send![self.id(), borderWidth]
        }
    }

    #[inline]
    pub fn set_border_width(&self, border_width: CGFloat) {
        unsafe {
            msg_send![self.id(), setBorderWidth:border_width]
        }
    }

    #[inline]
    pub fn border_color(&self) -> Option<CGColor> {
        unsafe {
            let color: SysCGColorRef = msg_send![self.id(), borderColor];
            if color.is_null() {
                None
            } else {
                Some(CGColor::wrap_under_get_rule(color))
            }
        }
    }

    #[inline]
    pub fn set_border_color(&self, color: Option<CGColor>) {
        unsafe {
            let color = match color {
                None => ptr::null(),
                Some(color) => color.as_CFTypeRef(),
            };
            msg_send![self.id(), setBorderColor:color]
        }
    }

    #[inline]
    pub fn opacity(&self) -> f32 {
        unsafe {
            msg_send![self.id(), opacity]
        }
    }

    #[inline]
    pub fn set_opacity(&self, opacity: f32) {
        unsafe {
            msg_send![self.id(), setOpacity:opacity]
        }
    }

    #[inline]
    pub fn compositing_filter(&self) -> id {
        unsafe {
            msg_send![self.id(), compositingFilter]
        }
    }

    #[inline]
    pub unsafe fn set_compositing_filter(&self, filter: id) {
        msg_send![self.id(), setCompositingFilter:filter]
    }

    #[inline]
    pub unsafe fn filters(&self) -> Option<CFArray> {
        let filters: CFArrayRef = msg_send![self.id(), filters];
        if filters.is_null() {
            None
        } else {
            Some(CFArray::wrap_under_get_rule(filters))
        }
    }

    #[inline]
    pub unsafe fn set_filters(&self, filters: Option<CFArray>) {
        let filters: CFTypeRef = match filters {
            Some(ref filters) => filters.as_CFTypeRef(),
            None => ptr::null(),
        };
        msg_send![self.id(), setFilters:filters]
    }

    #[inline]
    pub unsafe fn background_filters(&self) -> Option<CFArray> {
        let filters: CFArrayRef = msg_send![self.id(), backgroundFilters];
        if filters.is_null() {
            None
        } else {
            Some(CFArray::wrap_under_get_rule(filters))
        }
    }

    #[inline]
    pub unsafe fn set_background_filters(&self, filters: Option<CFArray>) {
        let filters: CFTypeRef = match filters {
            Some(ref filters) => filters.as_CFTypeRef(),
            None => ptr::null(),
        };
        msg_send![self.id(), setBackgroundFilters:filters]
    }

    #[inline]
    pub fn should_rasterize(&self) -> bool {
        unsafe {
            let flag: BOOL = msg_send![self.id(), shouldRasterize];
            flag == YES
        }
    }

    #[inline]
    pub fn set_should_rasterize(&self, flag: bool) {
        unsafe {
            msg_send![self.id(), setShouldRasterize:(flag as BOOL)]
        }
    }

    #[inline]
    pub fn rasterization_scale(&self) -> CGFloat {
        unsafe {
            msg_send![self.id(), rasterizationScale]
        }
    }

    #[inline]
    pub fn set_rasterization_scale(&self, scale: CGFloat) {
        unsafe {
            msg_send![self.id(), setRasterizationScale:scale]
        }
    }

    // Shadow properties

    #[inline]
    pub fn shadow_color(&self) -> Option<CGColor> {
        unsafe {
            let color: SysCGColorRef = msg_send![self.id(), shadowColor];
            if color.is_null() {
                None
            } else {
                Some(CGColor::wrap_under_get_rule(color))
            }
        }
    }

    #[inline]
    pub fn set_shadow_color(&self, color: Option<CGColor>) {
        unsafe {
            let color = match color {
                None => ptr::null(),
                Some(color) => color.as_CFTypeRef(),
            };
            msg_send![self.id(), setShadowColor:color]
        }
    }

    #[inline]
    pub fn shadow_opacity(&self) -> f32 {
        unsafe {
            msg_send![self.id(), shadowOpacity]
        }
    }

    #[inline]
    pub fn set_shadow_opacity(&self, opacity: f32) {
        unsafe {
            msg_send![self.id(), setShadowOpacity:opacity]
        }
    }

    #[inline]
    pub fn shadow_offset(&self) -> CGSize {
        unsafe {
            msg_send![self.id(), shadowOffset]
        }
    }

    #[inline]
    pub fn set_shadow_offset(&self, offset: &CGSize) {
        unsafe {
            msg_send![self.id(), setShadowOffset:*offset]
        }
    }

    #[inline]
    pub fn shadow_radius(&self) -> CGFloat {
        unsafe {
            msg_send![self.id(), shadowRadius]
        }
    }

    #[inline]
    pub fn set_shadow_radius(&self, radius: CGFloat) {
        unsafe {
            msg_send![self.id(), setShadowRadius:radius]
        }
    }

    #[inline]
    pub fn shadow_path(&self) -> Option<CGPath> {
        unsafe {
            let path: SysCGPathRef = msg_send![self.id(), shadowPath];
            if path.is_null() {
                None
            } else {
                Some(CGPath::from_ptr(path))
            }
        }
    }

    #[inline]
    pub fn set_shadow_path(&self, path: Option<CGPath>) {
        unsafe {
            let sys_path_ref = match path {
                None => ptr::null(),
                Some(path) => path.as_ptr(),
            };
            msg_send![self.id(), setShadowPath:sys_path_ref]
        }
    }

    // Layout methods

    #[inline]
    pub fn autoresizing_mask(&self) -> AutoresizingMask {
        unsafe {
            AutoresizingMask::from_bits_truncate(msg_send![self.id(), autoresizingMask])
        }
    }

    #[inline]
    pub fn set_autoresizing_mask(&self, mask: AutoresizingMask) {
        unsafe {
            msg_send![self.id(), setAutoresizingMask:mask.bits()]
        }
    }

    #[inline]
    pub fn layout_manager(&self) -> id {
        unsafe {
            msg_send![self.id(), layoutManager]
        }
    }

    #[inline]
    pub unsafe fn set_layout_manager(&self, manager: id) {
        msg_send![self.id(), setLayoutManager:manager]
    }

    #[inline]
    pub fn preferred_frame_size(&self) -> CGSize {
        unsafe {
            msg_send![self.id(), preferredFrameSize]
        }
    }

    #[inline]
    pub fn set_needs_layout(&self) {
        unsafe {
            msg_send![self.id(), setNeedsLayout]
        }
    }

    #[inline]
    pub fn needs_layout(&self) -> bool {
        unsafe {
            let flag: BOOL = msg_send![self.id(), needsLayout];
            flag == YES
        }
    }

    #[inline]
    pub fn layout_if_needed(&self) {
        unsafe {
            msg_send![self.id(), layoutIfNeeded]
        }
    }

    #[inline]
    pub fn layout_sublayers(&self) {
        unsafe {
            msg_send![self.id(), layoutSublayers]
        }
    }

    #[inline]
    pub fn resize_sublayers_with_old_size(&self, size: &CGSize) {
        unsafe {
            msg_send![self.id(), resizeSublayersWithOldSize:*size]
        }
    }

    #[inline]
    pub fn resize_with_old_superlayer_size(&self, size: &CGSize) {
        unsafe {
            msg_send![self.id(), resizeWithOldSuperlayerSize:*size]
        }
    }

    // Action methods

    #[inline]
    pub fn default_action_for_key(event: &str) -> id {
        unsafe {
            let event: CFString = CFString::from(event);
            msg_send![class!(CALayer), defaultActionForKey:event.as_CFTypeRef()]
        }
    }

    #[inline]
    pub fn action_for_key(&self, event: &str) -> id {
        unsafe {
            let event: CFString = CFString::from(event);
            msg_send![self.id(), actionForKey:event.as_CFTypeRef()]
        }
    }

    #[inline]
    pub fn actions(&self) -> CFDictionary<CFStringRef, CFTypeRef> {
        unsafe {
            msg_send![self.id(), actions]
        }
    }

    #[inline]
    pub unsafe fn set_actions(&self, actions: CFDictionary<CFStringRef, CFTypeRef>) {
        msg_send![self.id(), setActions:actions]
    }

    // TODO(pcwalton): Wrap `CAAnimation`.
    #[inline]
    pub unsafe fn add_animation_for_key(&self, animation: id, for_key: Option<&str>) {
        let for_key: Option<CFString> = for_key.map(CFString::from);
        let for_key: CFTypeRef = match for_key {
            Some(ref for_key) => for_key.as_CFTypeRef(),
            None => ptr::null(),
        };
        msg_send![self.id(), addAnimation:animation forKey:for_key]
    }

    #[inline]
    pub fn remove_all_animation(&self) {
        unsafe {
            msg_send![self.id(), removeAllAnimations]
        }
    }

    #[inline]
    pub fn remove_animation_for_key(&self, key: &str) {
        unsafe {
            let key = CFString::from(key);
            msg_send![self.id(), removeAnimationForKey:key.as_CFTypeRef()]
        }
    }

    #[inline]
    pub fn animation_keys(&self) -> Vec<String> {
        unsafe {
            let keys: CFArrayRef = msg_send![self.id(), animationKeys];
            let keys: CFArray = TCFType::wrap_under_create_rule(keys);
            keys.into_iter().map(|string| {
                CFString::wrap_under_get_rule(*string as CFStringRef).to_string()
            }).collect()
        }
    }

    #[inline]
    pub fn animation_for_key(&self, key: &str) -> id {
        unsafe {
            let key = CFString::from(key);
            msg_send![self.id(), animationForKey:key.as_CFTypeRef()]
        }
    }

    // Miscellaneous properties

    #[inline]
    pub fn name(&self) -> String {
        unsafe {
            let name: CFStringRef = msg_send![self.id(), name];
            CFString::wrap_under_get_rule(name).to_string()
        }
    }

    #[inline]
    pub fn set_name(&self, name: &str) {
        unsafe {
            let name = CFString::from(name);
            msg_send![self.id(), setName:name.as_CFTypeRef()]
        }
    }

    #[inline]
    pub fn delegate(&self) -> id {
        unsafe {
            msg_send![self.id(), delegate]
        }
    }

    #[inline]
    pub unsafe fn set_delegate(&self, delegate: id) {
        msg_send![self.id(), setDelegate:delegate]
    }

    #[inline]
    pub fn style(&self) -> Option<CFDictionary> {
        unsafe {
            let dictionary: CFDictionaryRef = msg_send![self.id(), style];
            if dictionary.is_null() {
                None
            } else {
                Some(CFDictionary::wrap_under_get_rule(dictionary))
            }
        }
    }

    #[inline]
    pub fn set_style(&self, dictionary: Option<CFDictionary>) {
        unsafe {
            let dictionary = match dictionary {
                None => ptr::null(),
                Some(ref dictionary) => dictionary.as_CFTypeRef(),
            };
            msg_send![self.id(), setStyle:dictionary]
        }
    }

    // Private methods

    #[inline]
    pub fn reload_value_for_key_path(&self, key: &str) {
        unsafe {
            let key = CFString::from(key);
            msg_send![self.id(), reloadValueForKeyPath:key.as_CFTypeRef()]
        }
    }

    #[inline]
    pub fn set_contents_opaque(&self, opaque: bool) {
        unsafe {
            msg_send![self.id(), setContentsOpaque:opaque as BOOL]
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContentsGravity {
    Center,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Resize,
    ResizeAspect,
    ResizeAspectFill,
    Other(CFString),
}

impl ContentsGravity {
    fn into_CFString(self) -> CFString {
        let string = match self {
            ContentsGravity::Center             => "center",
            ContentsGravity::Top                => "top",
            ContentsGravity::Bottom             => "bottom",
            ContentsGravity::Left               => "left",
            ContentsGravity::Right              => "right",
            ContentsGravity::TopLeft            => "topLeft",
            ContentsGravity::TopRight           => "topRight",
            ContentsGravity::BottomLeft         => "bottomLeft",
            ContentsGravity::BottomRight        => "bottomRight",
            ContentsGravity::Resize             => "resize",
            ContentsGravity::ResizeAspect       => "resizeAspect",
            ContentsGravity::ResizeAspectFill   => "resizeAspectFill",
            ContentsGravity::Other(other)       => return other,
        };
        CFString::from(string)
    }

    // FIXME(pcwalton): Inefficient.
    fn from_CFString(string: CFString) -> ContentsGravity {
        match string.to_string() {
            ref s if s == "center"              => ContentsGravity::Center,
            ref s if s == "top"                 => ContentsGravity::Top,
            ref s if s == "bottom"              => ContentsGravity::Bottom,
            ref s if s == "left"                => ContentsGravity::Left,
            ref s if s == "right"               => ContentsGravity::Right,
            ref s if s == "topLeft"             => ContentsGravity::TopLeft,
            ref s if s == "topRight"            => ContentsGravity::TopRight,
            ref s if s == "bottomLeft"          => ContentsGravity::BottomLeft,
            ref s if s == "bottomRight"         => ContentsGravity::BottomRight,
            ref s if s == "resize"              => ContentsGravity::Resize,
            ref s if s == "resizeAspect"        => ContentsGravity::ResizeAspect,
            ref s if s == "resizeAspectFill"    => ContentsGravity::ResizeAspectFill,
            _                                   => ContentsGravity::Other(string),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContentsFormat {
    RGBA8Uint,      // kCAContentsFormatRGBA8Uint, "RGBA"
    RGBA16Float,    // kCAContentsFormatRGBA16Float, "RGBAh"
    Gray8Uint,      // kCAContentsFormatGray8Uint, "Gray8"
    Other(CFString),
}

impl ContentsFormat {
    fn into_CFString(self) -> CFString {
        let string = match self {
            ContentsFormat::RGBA8Uint       => "RGBA8",
            ContentsFormat::RGBA16Float     => "RGBAh",
            ContentsFormat::Gray8Uint       => "Gray8",
            ContentsFormat::Other(other)    => return other,
        };
        CFString::from(string)
    }

    // FIXME(pcwalton): Inefficient.
    fn from_CFString(string: CFString) -> ContentsFormat {
        match string.to_string() {
            ref s if s == "RGBA8"   => ContentsFormat::RGBA8Uint,
            ref s if s == "RGBAh"   => ContentsFormat::RGBA16Float,
            ref s if s == "Gray8"   => ContentsFormat::Gray8Uint,
            _                       => ContentsFormat::Other(string),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Filter {
    Nearest,
    Linear,
    Trilinear,
    Other(CFString),
}

impl Filter {
    fn into_CFString(self) -> CFString {
        let string = match self {
            Filter::Nearest         => "nearest",
            Filter::Linear          => "linear",
            Filter::Trilinear       => "trilinear",
            Filter::Other(other)    => return other,
        };
        CFString::from(string)
    }

    // FIXME(pcwalton): Inefficient.
    fn from_CFString(string: CFString) -> Filter {
        match string.to_string() {
            ref s if s == "nearest"     => Filter::Nearest,
            ref s if s == "linear"      => Filter::Linear,
            ref s if s == "trilinear"   => Filter::Trilinear,
            _                           => Filter::Other(string),
        }
    }
}

bitflags! {
    pub struct EdgeAntialiasingMask: u32 {
        const LEFT_EDGE     = 1 << 0;   // kCALayerLeftEdge
        const RIGHT_EDGE    = 1 << 1;   // kCALayerRightEdge
        const BOTTOM_EDGE   = 1 << 2;   // kCALayerBottomEdge
        const TOP_EDGE      = 1 << 3;   // kCALayerTopEdge
    }
}

bitflags! {
    pub struct CornerMask: NSUInteger {
        const MIN_X_MIN_Y_CORNER =  1 << 0; // kCALayerMinXMinYCorner
        const MAX_X_MIN_Y_CORNER =  1 << 1; // kCALayerMaxXMinYCorner
        const MIN_X_MAX_Y_CORNER =  1 << 2; // kCALayerMinXMaxYCorner
        const MAX_X_MAX_Y_CORNER =  1 << 3; // kCALayerMaxXMaxYCorner
    }
}

bitflags! {
    pub struct AutoresizingMask: u32 {
        const NOT_SIZABLE       = 0;        // kCALayerNotSizable
        const MIN_X_MARGIN      = 1 << 0;   // kCALayerMinXMargin
        const WIDTH_SIZABLE     = 1 << 1;   // kCALayerWidthSizable
        const MAX_X_MARGIN      = 1 << 2;   // kCALayerMaxXMargin
        const MIN_Y_MARGIN      = 1 << 3;   // kCALayerMinYMargin
        const HEIGHT_SIZABLE    = 1 << 4;   // kCALayerHeightSizable
        const MAX_Y_MARGIN      = 1 << 5;   // kCALayerMaxYMargin
    }
}

// CARenderer.h

pub struct CARenderer(id);

unsafe impl Send for CARenderer {}
unsafe impl Sync for CARenderer {}

impl Clone for CARenderer {
    #[inline]
    fn clone(&self) -> CARenderer {
        unsafe {
            CARenderer(msg_send![self.id(), retain])
        }
    }
}

impl Drop for CARenderer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            msg_send![self.id(), release]
        }
    }
}

impl CARenderer {
    #[inline]
    pub fn id(&self) -> id {
        self.0
    }

    #[inline]
    pub unsafe fn from_cgl_context(context: CGLContextObj, color_space: Option<CGColorSpace>)
                                   -> CARenderer {
        let mut pairs: Vec<(CFString, CFType)> = vec![];
        if let Some(color_space) = color_space {
            pairs.push((CFString::wrap_under_get_rule(kCARendererColorSpace),
                        CFType::wrap_under_get_rule(color_space.as_ptr() as *const _ as *const _)))
        }

        let options: CFDictionary<CFString, CFType> = CFDictionary::from_CFType_pairs(&pairs);

        let renderer: id =
            msg_send![class!(CARenderer), rendererWithCGLContext:context
                                                         options:options.as_CFTypeRef()];
        debug_assert!(renderer != nil);
        CARenderer(renderer)
    }

    #[inline]
    pub unsafe fn from_metal_texture(metal_texture: id,
                                     metal_command_queue: id,
                                     color_space: Option<CGColorSpace>)
                                     -> CARenderer {
        let mut pairs: Vec<(CFString, CFType)> = vec![
            (CFString::wrap_under_get_rule(kCARendererMetalCommandQueue),
             CFType::wrap_under_get_rule(metal_command_queue as *const _ as *const _)),
        ];
        if let Some(color_space) = color_space {
            pairs.push((CFString::wrap_under_get_rule(kCARendererColorSpace),
                        CFType::wrap_under_get_rule(color_space.as_ptr() as *const _ as *const _)))
        }

        let options: CFDictionary<CFString, CFType> = CFDictionary::from_CFType_pairs(&pairs);

        let renderer: id =
            msg_send![class!(CARenderer), rendererWithMTLTexture:metal_texture
                                                         options:options.as_CFTypeRef()];
        debug_assert!(renderer != nil);
        CARenderer(renderer)
    }

    #[inline]
    pub fn layer(&self) -> Option<CALayer> {
        unsafe {
            let layer: id = msg_send![self.id(), layer];
            if layer.is_null() {
                None
            } else {
                Some(CALayer(layer))
            }
        }
    }

    #[inline]
    pub fn set_layer(&self, layer: Option<CALayer>) {
        unsafe {
            let layer = match layer {
                Some(ref layer) => layer.id(),
                None => nil,
            };
            msg_send![self.id(), setLayer:layer]
        }
    }

    #[inline]
    pub fn bounds(&self) -> CGRect {
        unsafe {
            msg_send![self.id(), bounds]
        }
    }

    #[inline]
    pub fn set_bounds(&self, bounds: CGRect) {
        unsafe {
            msg_send![self.id(), setBounds:bounds]
        }
    }

    #[inline]
    pub fn begin_frame_at(&self, time: CFTimeInterval, timestamp: Option<&CVTimeStamp>) {
        unsafe {
            msg_send![self.id(), beginFrameAtTime:time timeStamp:timestamp]
        }
    }

    #[inline]
    pub fn update_bounds(&self) -> CGRect {
        unsafe {
            msg_send![self.id(), updateBounds]
        }
    }

    #[inline]
    pub fn add_update_rect(&self, rect: CGRect) {
        unsafe {
            msg_send![self.id(), addUpdateRect:rect]
        }
    }

    #[inline]
    pub fn render(&self) {
        unsafe {
            msg_send![self.id(), render]
        }
    }

    #[inline]
    pub fn next_frame_time(&self) -> CFTimeInterval {
        unsafe {
            msg_send![self.id(), nextFrameTime]
        }
    }

    #[inline]
    pub fn end_frame(&self) {
        unsafe {
            msg_send![self.id(), endFrame]
        }
    }

    #[inline]
    pub unsafe fn set_destination(&self, metal_texture: id) {
        msg_send![self.id(), setDestination:metal_texture]
    }
}

// CATransaction.h

// You can't actually construct any `CATransaction` objects, so that class is
// really just a module.
pub mod transaction {
    use block::{Block, ConcreteBlock, IntoConcreteBlock, RcBlock};
    use core_foundation::date::CFTimeInterval;
    use core_foundation::string::CFString;

    use base::{BOOL, YES, id};

    #[inline]
    pub fn begin() {
        unsafe {
            msg_send![class!(CATransaction), begin]
        }
    }

    #[inline]
    pub fn commit() {
        unsafe {
            msg_send![class!(CATransaction), commit]
        }
    }

    #[inline]
    pub fn flush() {
        unsafe {
            msg_send![class!(CATransaction), flush]
        }
    }

    #[inline]
    pub fn lock() {
        unsafe {
            msg_send![class!(CATransaction), lock]
        }
    }

    #[inline]
    pub fn unlock() {
        unsafe {
            msg_send![class!(CATransaction), unlock]
        }
    }

    #[inline]
    pub fn animation_duration() -> CFTimeInterval {
        unsafe {
            msg_send![class!(CATransaction), animationDuration]
        }
    }

    #[inline]
    pub fn set_animation_duration(duration: CFTimeInterval) {
        unsafe {
            msg_send![class!(CATransaction), setAnimationDuration:duration]
        }
    }

    #[inline]
    pub fn animation_timing_function() -> id {
        unsafe {
            msg_send![class!(CATransaction), animationTimingFunction]
        }
    }

    #[inline]
    pub unsafe fn set_animation_timing_function(function: id) {
        msg_send![class!(CATransaction), setAnimationTimingFunction:function]
    }

    #[inline]
    pub fn disable_actions() -> bool {
        unsafe {
            let flag: BOOL = msg_send![class!(CATransaction), disableActions];
            flag == YES
        }
    }

    #[inline]
    pub fn set_disable_actions(flag: bool) {
        unsafe {
            msg_send![class!(CATransaction), setDisableActions:flag as BOOL]
        }
    }

    #[inline]
    pub fn completion_block() -> Option<RcBlock<(), ()>> {
        unsafe {
            let completion_block: *mut Block<(), ()> =
                msg_send![class!(CATransaction), completionBlock];
            if completion_block.is_null() {
                None
            } else {
                Some(RcBlock::new(completion_block))
            }
        }
    }

    #[inline]
    pub fn set_completion_block<F>(block: ConcreteBlock<(), (), F>)
                                   where F: 'static + IntoConcreteBlock<(), Ret = ()> {
        unsafe {
            let block = block.copy();
            msg_send![class!(CATransaction), setCompletionBlock:&*block]
        }
    }

    #[inline]
    pub fn value_for_key(key: &str) -> id {
        unsafe {
            let key: CFString = CFString::from(key);
            msg_send![class!(CATransaction), valueForKey:key]
        }
    }

    #[inline]
    pub fn set_value_for_key(value: id, key: &str) {
        unsafe {
            let key: CFString = CFString::from(key);
            msg_send![class!(CATransaction), setValue:value forKey:key]
        }
    }
}

// CATransform3D.h

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CATransform3D {
    pub m11: CGFloat, pub m12: CGFloat, pub m13: CGFloat, pub m14: CGFloat,
    pub m21: CGFloat, pub m22: CGFloat, pub m23: CGFloat, pub m24: CGFloat,
    pub m31: CGFloat, pub m32: CGFloat, pub m33: CGFloat, pub m34: CGFloat,
    pub m41: CGFloat, pub m42: CGFloat, pub m43: CGFloat, pub m44: CGFloat,
}

impl PartialEq for CATransform3D {
    #[inline]
    fn eq(&self, other: &CATransform3D) -> bool {
        unsafe {
            CATransform3DEqualToTransform(*self, *other)
        }
    }
}

impl Mul<CATransform3D> for CATransform3D {
    type Output = CATransform3D;

    #[inline]
    fn mul(self, other: CATransform3D) -> CATransform3D {
        unsafe {
            CATransform3DConcat(self, other)
        }
    }
}

impl CATransform3D {
    pub const IDENTITY: CATransform3D = CATransform3D {
        m11: 1.0, m12: 0.0, m13: 0.0, m14: 0.0,
        m21: 0.0, m22: 1.0, m23: 0.0, m24: 0.0,
        m31: 0.0, m32: 0.0, m33: 1.0, m34: 0.0,
        m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
    };

    #[inline]
    pub fn from_translation(tx: CGFloat, ty: CGFloat, tz: CGFloat) -> CATransform3D {
        unsafe {
            CATransform3DMakeTranslation(tx, ty, tz)
        }
    }

    #[inline]
    pub fn from_scale(sx: CGFloat, sy: CGFloat, sz: CGFloat) -> CATransform3D {
        unsafe {
            CATransform3DMakeScale(sx, sy, sz)
        }
    }

    #[inline]
    pub fn from_rotation(angle: CGFloat, x: CGFloat, y: CGFloat, z: CGFloat) -> CATransform3D {
        unsafe {
            CATransform3DMakeRotation(angle, x, y, z)
        }
    }

    #[inline]
    pub fn affine(affine_transform: CGAffineTransform) -> CATransform3D {
        unsafe {
            CATransform3DMakeAffineTransform(affine_transform)
        }
    }

    #[inline]
    pub fn is_identity(&self) -> bool {
        unsafe {
            CATransform3DIsIdentity(*self)
        }
    }

    #[inline]
    pub fn translate(&self, tx: CGFloat, ty: CGFloat, tz: CGFloat) -> CATransform3D {
        unsafe {
            CATransform3DTranslate(*self, tx, ty, tz)
        }
    }

    #[inline]
    pub fn scale(&self, sx: CGFloat, sy: CGFloat, sz: CGFloat) -> CATransform3D {
        unsafe {
            CATransform3DScale(*self, sx, sy, sz)
        }
    }

    #[inline]
    pub fn rotate(&self, angle: CGFloat, x: CGFloat, y: CGFloat, z: CGFloat) -> CATransform3D {
        unsafe {
            CATransform3DRotate(*self, angle, x, y, z)
        }
    }

    #[inline]
    pub fn invert(&self) -> CATransform3D {
        unsafe {
            CATransform3DInvert(*self)
        }
    }

    #[inline]
    pub fn is_affine(&self) -> bool {
        unsafe {
            CATransform3DIsAffine(*self)
        }
    }

    #[inline]
    pub fn to_affine(&self) -> CGAffineTransform {
        unsafe {
            CATransform3DGetAffineTransform(*self)
        }
    }
}

#[link(name = "QuartzCore", kind = "framework")]
extern {
    static kCARendererColorSpace: CFStringRef;
    static kCARendererMetalCommandQueue: CFStringRef;

    fn CACurrentMediaTime() -> CFTimeInterval;

    fn CATransform3DIsIdentity(t: CATransform3D) -> bool;
    fn CATransform3DEqualToTransform(a: CATransform3D, b: CATransform3D) -> bool;
    fn CATransform3DMakeTranslation(tx: CGFloat, ty: CGFloat, tz: CGFloat) -> CATransform3D;
    fn CATransform3DMakeScale(sx: CGFloat, sy: CGFloat, sz: CGFloat) -> CATransform3D;
    fn CATransform3DMakeRotation(angle: CGFloat, x: CGFloat, y: CGFloat, z: CGFloat)
                                 -> CATransform3D;
    fn CATransform3DTranslate(t: CATransform3D, tx: CGFloat, ty: CGFloat, tz: CGFloat)
                              -> CATransform3D;
    fn CATransform3DScale(t: CATransform3D, sx: CGFloat, sy: CGFloat, sz: CGFloat)
                          -> CATransform3D;
    fn CATransform3DRotate(t: CATransform3D, angle: CGFloat, x: CGFloat, y: CGFloat, z: CGFloat)
                           -> CATransform3D;
    fn CATransform3DConcat(a: CATransform3D, b: CATransform3D) -> CATransform3D;
    fn CATransform3DInvert(t: CATransform3D) -> CATransform3D;
    fn CATransform3DMakeAffineTransform(m: CGAffineTransform) -> CATransform3D;
    fn CATransform3DIsAffine(t: CATransform3D) -> bool;
    fn CATransform3DGetAffineTransform(t: CATransform3D) -> CGAffineTransform;
}

// Miscellaneous structures in other frameworks.
//
// These should be moved elsewhere eventually.

// CoreVideo/CVBase.h

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CVTimeStamp {
    pub version: u32,
    pub videoTimeScale: i32,
    pub videoTime: i64,
    pub hostTime: u64,
    pub rateScalar: f64,
    pub videoRefreshPeriod: i64,
    pub smpteTime: CVSMPTETime,
    pub flags: u64,
    pub reserved: u64,
}

pub type CVTimeStampFlags = u64;

pub const kCVTimeStampVideoTimeValid: CVTimeStampFlags = 1 << 0;
pub const kCVTimeStampHostTimeValid: CVTimeStampFlags = 1 << 1;
pub const kCVTimeStampSMPTETimeValid: CVTimeStampFlags = 1 << 2;
pub const kCVTimeStampVideoRefreshPeriodValid: CVTimeStampFlags = 1 << 3;
pub const kCVTimeStampRateScalarValid: CVTimeStampFlags = 1 << 4;
pub const kCVTimeStampTopField: CVTimeStampFlags = 1 << 16;
pub const kCVTimeStampBottomField: CVTimeStampFlags = 1 << 17;
pub const kCVTimeStampVideoHostTimeValid: CVTimeStampFlags =
    kCVTimeStampVideoTimeValid | kCVTimeStampHostTimeValid;
pub const kCVTimeStampIsInterlaced: CVTimeStampFlags =
    kCVTimeStampTopField | kCVTimeStampBottomField;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CVSMPTETime {
    pub subframes: i16,
    pub subframeDivisor: i16,
    pub counter: u32,
    pub time_type: u32,
    pub flags: u32,
    pub hours: i16,
    pub minutes: i16,
    pub seconds: i16,
    pub frames: i16,
}

pub type CVSMPTETimeType = u32;

pub const kCVSMPTETimeType24:       CVSMPTETimeType = 0;
pub const kCVSMPTETimeType25:       CVSMPTETimeType = 1;
pub const kCVSMPTETimeType30Drop:   CVSMPTETimeType = 2;
pub const kCVSMPTETimeType30:       CVSMPTETimeType = 3;
pub const kCVSMPTETimeType2997:     CVSMPTETimeType = 4;
pub const kCVSMPTETimeType2997Drop: CVSMPTETimeType = 5;
pub const kCVSMPTETimeType60:       CVSMPTETimeType = 6;
pub const kCVSMPTETimeType5994:     CVSMPTETimeType = 7;

pub type CVSMPTETimeFlags = u32;

pub const kCVSMPTETimeValid:    CVSMPTETimeFlags = 1 << 0;
pub const kCVSMPTETimeRunning:  CVSMPTETimeFlags = 1 << 1;
