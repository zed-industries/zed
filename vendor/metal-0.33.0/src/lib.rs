//! # ⚠️ Deprecated ⚠️
//!
//! Use of this crate is deprecated as the [`objc`] ecosystem of mac system bindings are unmaintained.
//! For new development, please use [`objc2`] and [`objc2-metal`] instead. We will continue to merge basic
//! PRs and keep things maintained, at least as long as it takes to migrate [`wgpu`] to the `objc2` ecosystem [PR 5641].
//!
//! [`objc`]: https://crates.io/crates/objc
//! [`objc2`]: https://crates.io/crates/objc2
//! [`objc2-metal`]: https://crates.io/crates/objc2-metal
//! [`wgpu`]: https://crates.io/crates/wgpu
//! [PR 5641]: https://github.com/gfx-rs/wgpu/pull/5641

// Copyright 2023 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::new_without_default, clippy::new_ret_no_self)]

#[macro_use]
pub extern crate objc;
#[macro_use]
pub extern crate foreign_types;
#[macro_use]
pub extern crate paste;

use std::{
    borrow::Borrow,
    ffi::{c_char, c_void},
    marker::PhantomData,
    mem,
    ops::Deref,
};

use core_graphics_types::{base::CGFloat, geometry::CGSize};
use foreign_types::ForeignType;
use objc::runtime::{Object, NO, YES};

/// See <https://developer.apple.com/documentation/objectivec/nsinteger>
#[cfg(target_pointer_width = "64")]
pub type NSInteger = i64;

/// See <https://developer.apple.com/documentation/objectivec/nsinteger>
#[cfg(not(target_pointer_width = "64"))]
pub type NSInteger = i32;

/// See <https://developer.apple.com/documentation/objectivec/nsuinteger>
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = u64;

/// See <https://developer.apple.com/documentation/objectivec/nsuinteger>
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = u32;

/// See <https://developer.apple.com/documentation/foundation/nsrange>
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

impl NSRange {
    #[inline]
    pub fn new(location: NSUInteger, length: NSUInteger) -> NSRange {
        NSRange { location, length }
    }
}

fn nsstring_as_str(nsstr: &objc::runtime::Object) -> &str {
    let bytes = unsafe {
        let bytes: *const c_char = msg_send![nsstr, UTF8String];
        bytes.cast()
    };
    let len: NSUInteger = unsafe { msg_send![nsstr, length] };
    unsafe {
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
}

fn nsstring_from_str(string: &str) -> *mut objc::runtime::Object {
    const UTF8_ENCODING: usize = 4;

    let cls = class!(NSString);
    let bytes = string.as_ptr().cast::<c_void>();
    unsafe {
        let obj: *mut objc::runtime::Object = msg_send![cls, alloc];
        let obj: *mut objc::runtime::Object = msg_send![
            obj,
            initWithBytes:bytes
            length:string.len()
            encoding:UTF8_ENCODING
        ];
        let _: *mut c_void = msg_send![obj, autorelease];
        obj
    }
}

/// Define a Rust wrapper for an Objective-C opaque type.
///
/// This macro adapts the `foreign-types` crate's [`foreign_type!`]
/// macro to Objective-C, defining Rust types that represent owned and
/// borrowed forms of some underlying Objective-C type, using
/// Objective-C's reference counting to manage its lifetime.
///
/// Given a use of the form:
///
/// ```ignore
/// foreign_obj_type! {
///     type CType = MTLBuffer;   // underlying Objective-C type
///     pub struct Buffer;        // owned Rust type
///     pub struct BufferRef;     // borrowed Rust type
///     type ParentType = ResourceRef;  // borrowed parent class
/// }
/// ```
///
/// This defines the types `Buffer` and `BufferRef` as owning and
/// non-owning types, analogous to `String` and `str`, that manage
/// some underlying `*mut MTLBuffer`:
///
/// - Both `Buffer` and `BufferRef` implement [`obj::Message`], indicating
///   that they can be sent Objective-C messages.
///
/// - Dropping a `Buffer` sends the underlying `MTLBuffer` a `release`
///   message, and cloning a `BufferRef` sends a `retain` message and
///   returns a new `Buffer`.
///
/// - `Buffer` dereferences to `BufferRef`.
///
/// - `BufferRef` dereferences to its parent type `ResourceRef`. The
///   `ParentType` component is optional; if omitted, the `Ref` type
///   doesn't implement `Deref` or `DerefMut`.
///
/// - Both `Buffer` and `BufferRef` implement `std::fmt::Debug`,
///   sending an Objective-C `debugDescription` message to the
///   underlying `MTLBuffer`.
///
/// Following the `foreign_types` crate's nomenclature, the `Ref`
/// suffix indicates that `BufferRef` and `ResourceRef` are non-owning
/// types, used *by reference*, like `&BufferRef` or `&ResourceRef`.
/// These types are not, themselves, references.
macro_rules! foreign_obj_type {
    {
        type CType = $raw_ident:ident;
        pub struct $owned_ident:ident;
        type ParentType = $parent_ident:ident;
    } => {
        foreign_obj_type! {
            type CType = $raw_ident;
            pub struct $owned_ident;
        }

        impl ::std::ops::Deref for paste!{[<$owned_ident Ref>]} {
            type Target = paste!{[<$parent_ident Ref>]};

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl ::std::convert::From<$owned_ident> for $parent_ident {
            fn from(item: $owned_ident) -> Self {
                unsafe { Self::from_ptr(item.into_ptr().cast()) }
            }
        }
    };
    {
        type CType = $raw_ident:ident;
        pub struct $owned_ident:ident;
    } => {
        foreign_type! {
            pub unsafe type $owned_ident: Sync + Send {
                type CType = $raw_ident;
                fn drop = crate::obj_drop;
                fn clone = crate::obj_clone;
            }
        }

        unsafe impl ::objc::Message for $raw_ident {
        }
        unsafe impl ::objc::Message for paste!{[<$owned_ident Ref>]} {
        }

        impl ::std::fmt::Debug for paste!{[<$owned_ident Ref>]} {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                unsafe {
                    let string: *mut ::objc::runtime::Object = msg_send![self, debugDescription];
                    write!(f, "{}", crate::nsstring_as_str(&*string))
                }
            }
        }

        impl ::std::fmt::Debug for $owned_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::ops::Deref::deref(self).fmt(f)
            }
        }
    };
}

macro_rules! try_objc {
    {
        $err_name: ident => $body:expr
    } => {
        {
            let mut $err_name: *mut Object = ::std::ptr::null_mut();
            let value = $body;
            if !$err_name.is_null() {
                let desc: *mut Object = msg_send![$err_name, localizedDescription];
                let compile_error: *const c_char = msg_send![desc, UTF8String];
                let message = CStr::from_ptr(compile_error).to_string_lossy().into_owned();
                return Err(message);
            }
            value
        }
    };
}

macro_rules! msg_send_bool {
    ($obj:expr, $name:ident) => {{
        match msg_send![$obj, $name] {
            YES => true,
            NO => false,
            #[cfg(not(target_arch = "aarch64"))]
            _ => unreachable!(),
        }
    }};
    ($obj:expr, $name:ident : $arg:expr) => {{
        match msg_send![$obj, $name: $arg] {
            YES => true,
            NO => false,
            #[cfg(not(target_arch = "aarch64"))]
            _ => unreachable!(),
        }
    }};
}

macro_rules! msg_send_bool_error_check {
    ($obj:expr, $name:ident: $arg:expr) => {{
        let mut err: *mut Object = ptr::null_mut();
        let result: BOOL = msg_send![$obj, $name:$arg
                                                    error:&mut err];
        if !err.is_null() {
            let desc: *mut Object = msg_send![err, localizedDescription];
            let c_msg: *const c_char = msg_send![desc, UTF8String];
            let message = CStr::from_ptr(c_msg).to_string_lossy().into_owned();
            Err(message)
        } else {
            match result {
                YES => Ok(true),
                NO => Ok(false),
                #[cfg(not(target_arch = "aarch64"))]
                _ => unreachable!(),
            }
        }
    }};
}

/// See <https://developer.apple.com/documentation/foundation/nsarray>
pub struct NSArray<T> {
    _phantom: PhantomData<T>,
}

pub struct Array<T>(*mut NSArray<T>)
where
    T: ForeignType + 'static,
    T::Ref: objc::Message + 'static;

pub struct ArrayRef<T>(foreign_types::Opaque, PhantomData<T>)
where
    T: ForeignType + 'static,
    T::Ref: objc::Message + 'static;

impl<T> Drop for Array<T>
where
    T: ForeignType + 'static,
    T::Ref: objc::Message + 'static,
{
    fn drop(&mut self) {
        unsafe {
            let () = msg_send![self.0, release];
        }
    }
}

impl<T> Clone for Array<T>
where
    T: ForeignType + 'static,
    T::Ref: objc::Message + 'static,
{
    fn clone(&self) -> Self {
        unsafe { Array(msg_send![self.0, retain]) }
    }
}

unsafe impl<T> objc::Message for NSArray<T>
where
    T: ForeignType + 'static,
    T::Ref: objc::Message + 'static,
{
}

unsafe impl<T> objc::Message for ArrayRef<T>
where
    T: ForeignType + 'static,
    T::Ref: objc::Message + 'static,
{
}

impl<T> Array<T>
where
    T: ForeignType + 'static,
    T::Ref: objc::Message + 'static,
{
    pub fn from_slice<'a>(s: &[&T::Ref]) -> &'a ArrayRef<T> {
        unsafe {
            let class = class!(NSArray);
            msg_send![class, arrayWithObjects: s.as_ptr() count: s.len()]
        }
    }

    pub fn from_owned_slice<'a>(s: &[T]) -> &'a ArrayRef<T> {
        unsafe {
            let class = class!(NSArray);
            msg_send![class, arrayWithObjects: s.as_ptr() count: s.len()]
        }
    }
}

unsafe impl<T> foreign_types::ForeignType for Array<T>
where
    T: ForeignType + 'static,
    T::Ref: objc::Message + 'static,
{
    type CType = NSArray<T>;
    type Ref = ArrayRef<T>;

    unsafe fn from_ptr(p: *mut NSArray<T>) -> Self {
        Array(p)
    }

    fn as_ptr(&self) -> *mut NSArray<T> {
        self.0
    }
}

unsafe impl<T> foreign_types::ForeignTypeRef for ArrayRef<T>
where
    T: ForeignType + 'static,
    T::Ref: objc::Message + 'static,
{
    type CType = NSArray<T>;
}

impl<T> Deref for Array<T>
where
    T: ForeignType + 'static,
    T::Ref: objc::Message + 'static,
{
    type Target = ArrayRef<T>;

    #[inline]
    fn deref(&self) -> &ArrayRef<T> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> Borrow<ArrayRef<T>> for Array<T>
where
    T: ForeignType + 'static,
    T::Ref: objc::Message + 'static,
{
    fn borrow(&self) -> &ArrayRef<T> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> ToOwned for ArrayRef<T>
where
    T: ForeignType + 'static,
    T::Ref: objc::Message + 'static,
{
    type Owned = Array<T>;

    fn to_owned(&self) -> Array<T> {
        unsafe { Array::from_ptr(msg_send![self, retain]) }
    }
}

/// See <https://developer.apple.com/documentation/quartzcore/cametaldrawable>
pub enum CAMetalDrawable {}

foreign_obj_type! {
    type CType = CAMetalDrawable;
    pub struct MetalDrawable;
    type ParentType = Drawable;
}

impl MetalDrawableRef {
    pub fn texture(&self) -> &TextureRef {
        unsafe { msg_send![self, texture] }
    }
}

pub enum NSObject {}

foreign_obj_type! {
    type CType = NSObject;
    pub struct NsObject;
}

impl NsObjectRef {
    pub fn conforms_to_protocol<T>(&self) -> Result<bool, String> {
        let name = ::std::any::type_name::<T>();
        if let Some(name) = name.split("::").last() {
            if let Some(protocol) = objc::runtime::Protocol::get(name) {
                Ok(unsafe { msg_send![self, conformsToProtocol: protocol] })
            } else {
                Err(format!("Can not find the protocol for type: {}.", name))
            }
        } else {
            Err(format!("Unexpected type name: {}.", name))
        }
    }
}

// See <https://developer.apple.com/documentation/quartzcore/cametallayer>
pub enum CAMetalLayer {}

foreign_obj_type! {
    type CType = CAMetalLayer;
    pub struct MetalLayer;
}

impl MetalLayer {
    pub fn new() -> Self {
        unsafe {
            let class = class!(CAMetalLayer);
            msg_send![class, new]
        }
    }
}

impl MetalLayerRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }

    pub fn set_device(&self, device: &DeviceRef) {
        unsafe { msg_send![self, setDevice: device] }
    }

    pub fn pixel_format(&self) -> MTLPixelFormat {
        unsafe { msg_send![self, pixelFormat] }
    }

    pub fn set_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe { msg_send![self, setPixelFormat: pixel_format] }
    }

    pub fn drawable_size(&self) -> CGSize {
        unsafe { msg_send![self, drawableSize] }
    }

    pub fn set_drawable_size(&self, size: CGSize) {
        unsafe { msg_send![self, setDrawableSize: size] }
    }

    pub fn presents_with_transaction(&self) -> bool {
        unsafe { msg_send_bool![self, presentsWithTransaction] }
    }

    pub fn set_presents_with_transaction(&self, transaction: bool) {
        unsafe { msg_send![self, setPresentsWithTransaction: transaction] }
    }

    pub fn display_sync_enabled(&self) -> bool {
        unsafe { msg_send_bool![self, displaySyncEnabled] }
    }

    pub fn set_display_sync_enabled(&self, enabled: bool) {
        unsafe { msg_send![self, setDisplaySyncEnabled: enabled] }
    }

    pub fn maximum_drawable_count(&self) -> NSUInteger {
        unsafe { msg_send![self, maximumDrawableCount] }
    }

    pub fn set_maximum_drawable_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setMaximumDrawableCount: count] }
    }

    pub fn set_edge_antialiasing_mask(&self, mask: u64) {
        unsafe { msg_send![self, setEdgeAntialiasingMask: mask] }
    }

    pub fn set_masks_to_bounds(&self, masks: bool) {
        unsafe { msg_send![self, setMasksToBounds: masks] }
    }

    pub fn remove_all_animations(&self) {
        unsafe { msg_send![self, removeAllAnimations] }
    }

    pub fn next_drawable(&self) -> Option<&MetalDrawableRef> {
        unsafe { msg_send![self, nextDrawable] }
    }

    pub fn contents_scale(&self) -> CGFloat {
        unsafe { msg_send![self, contentsScale] }
    }

    pub fn set_contents_scale(&self, scale: CGFloat) {
        unsafe { msg_send![self, setContentsScale: scale] }
    }

    /// [framebufferOnly Apple Docs](https://developer.apple.com/documentation/metal/mtltexture/1515749-framebufferonly?language=objc)
    pub fn framebuffer_only(&self) -> bool {
        unsafe { msg_send_bool!(self, framebufferOnly) }
    }

    pub fn set_framebuffer_only(&self, framebuffer_only: bool) {
        unsafe { msg_send![self, setFramebufferOnly: framebuffer_only] }
    }

    pub fn is_opaque(&self) -> bool {
        unsafe { msg_send_bool!(self, isOpaque) }
    }

    pub fn set_opaque(&self, opaque: bool) {
        unsafe { msg_send![self, setOpaque: opaque] }
    }

    pub fn wants_extended_dynamic_range_content(&self) -> bool {
        unsafe { msg_send_bool![self, wantsExtendedDynamicRangeContent] }
    }

    pub fn set_wants_extended_dynamic_range_content(
        &self,
        wants_extended_dynamic_range_content: bool,
    ) {
        unsafe {
            msg_send![
                self,
                setWantsExtendedDynamicRangeContent: wants_extended_dynamic_range_content
            ]
        }
    }
}

mod acceleration_structure;
mod acceleration_structure_pass;
mod argument;
mod blitpass;
mod buffer;
mod capturedescriptor;
mod capturemanager;
mod commandbuffer;
mod commandqueue;
mod computepass;
mod constants;
mod counters;
mod depthstencil;
mod device;
mod drawable;
mod encoder;
mod heap;
mod indirect_encoder;
mod library;
#[cfg(feature = "mps")]
pub mod mps;
mod pipeline;
mod renderpass;
mod resource;
mod sampler;
mod sync;
mod texture;
mod types;
mod vertexdescriptor;

#[rustfmt::skip]
pub use {
    acceleration_structure::*,
    acceleration_structure_pass::*,
    argument::*,
    blitpass::*,
    buffer::*,
    counters::*,
    computepass::*,
    capturedescriptor::*,
    capturemanager::*,
    commandbuffer::*,
    commandqueue::*,
    constants::*,
    depthstencil::*,
    device::*,
    drawable::*,
    encoder::*,
    heap::*,
    indirect_encoder::*,
    library::*,
    pipeline::*,
    renderpass::*,
    resource::*,
    sampler::*,
    texture::*,
    types::*,
    vertexdescriptor::*,
    sync::*,
};

#[inline]
unsafe fn obj_drop<T>(p: *mut T) {
    msg_send![p.cast::<Object>(), release]
}

#[inline]
unsafe fn obj_clone<T: 'static>(p: *mut T) -> *mut T {
    msg_send![p.cast::<Object>(), retain]
}

#[allow(non_camel_case_types)]
type c_size_t = usize;

// TODO: expand supported interface
/// See <https://developer.apple.com/documentation/foundation/nsurl>
pub enum NSURL {}

foreign_obj_type! {
    type CType = NSURL;
    pub struct URL;
}

impl URL {
    pub fn new_with_string(string: &str) -> Self {
        unsafe {
            let ns_str = crate::nsstring_from_str(string);
            let class = class!(NSURL);
            msg_send![class, URLWithString: ns_str]
        }
    }
}

impl URLRef {
    pub fn absolute_string(&self) -> &str {
        unsafe {
            let absolute_string = msg_send![self, absoluteString];
            crate::nsstring_as_str(absolute_string)
        }
    }

    pub fn path(&self) -> &str {
        unsafe {
            let path = msg_send![self, path];
            crate::nsstring_as_str(path)
        }
    }
}
