// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::{Color, Point, Rect};
use accesskit_consumer::{Node, TextPosition, TextRange};
use objc2::encode::{Encoding, RefEncode};
use objc2::{msg_send, rc::Id, runtime::AnyObject};
use objc2_app_kit::*;
use objc2_foundation::{NSPoint, NSRange, NSRect, NSSize};

pub(crate) fn from_ns_range<'a>(node: &'a Node<'a>, ns_range: NSRange) -> Option<TextRange<'a>> {
    let pos = node.text_position_from_global_utf16_index(ns_range.location)?;
    let mut range = pos.to_degenerate_range();
    if ns_range.length > 0 {
        let end =
            node.text_position_from_global_utf16_index(ns_range.location + ns_range.length)?;
        range.set_end(end);
    }
    Some(range)
}

pub(crate) fn to_ns_range(range: &TextRange) -> NSRange {
    let start = range.start().to_global_utf16_index();
    let end = range.end().to_global_utf16_index();
    NSRange::from(start..end)
}

pub(crate) fn to_ns_range_for_character(pos: &TextPosition) -> NSRange {
    let mut range = pos.to_degenerate_range();
    if !pos.is_document_end() {
        range.set_end(pos.forward_to_character_end());
    }
    to_ns_range(&range)
}

pub(crate) fn from_ns_point(view: &NSView, node: &Node, point: NSPoint) -> Point {
    let window = view.window().unwrap();
    let point = window.convertPointFromScreen(point);
    let point = view.convertPoint_fromView(point, None);
    // AccessKit coordinates are in physical (DPI-dependent) pixels, but
    // macOS provides logical (DPI-independent) coordinates here.
    let factor = window.backingScaleFactor();
    let point = Point::new(
        point.x * factor,
        if view.isFlipped() {
            point.y * factor
        } else {
            let view_bounds = view.bounds();
            (view_bounds.size.height - point.y) * factor
        },
    );
    node.transform().inverse() * point
}

pub(crate) fn to_ns_rect(view: &NSView, rect: Rect) -> NSRect {
    let window = view.window().unwrap();
    // AccessKit coordinates are in physical (DPI-dependent)
    // pixels, but macOS expects logical (DPI-independent)
    // coordinates here.
    let factor = window.backingScaleFactor();
    let rect = NSRect {
        origin: NSPoint {
            x: rect.x0 / factor,
            y: if view.isFlipped() {
                rect.y0 / factor
            } else {
                let view_bounds = view.bounds();
                view_bounds.size.height - rect.y1 / factor
            },
        },
        size: NSSize {
            width: rect.width() / factor,
            height: rect.height() / factor,
        },
    };
    let rect = view.convertRect_toView(rect, None);
    let window = view.window().unwrap();
    window.convertRectToScreen(rect)
}

fn color_channel_to_f64(channel: u8) -> f64 {
    (channel as f64) / 255.0
}

// TODO: can be removed after updating objc2 to 0.6 which has proper `CGColor` support
#[repr(C)]
struct CGColor {
    _private: [u8; 0],
}

unsafe impl RefEncode for CGColor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("CGColor", &[]));
}

pub(crate) fn to_color_attribute(color: Color) -> Id<AnyObject> {
    let ns_color = unsafe {
        NSColor::colorWithSRGBRed_green_blue_alpha(
            color_channel_to_f64(color.red),
            color_channel_to_f64(color.green),
            color_channel_to_f64(color.blue),
            color_channel_to_f64(color.alpha),
        )
    };
    let cg_color: *const CGColor = unsafe { msg_send![&ns_color, CGColor] };
    unsafe { Id::retain(cg_color as *mut AnyObject).unwrap() }
}
