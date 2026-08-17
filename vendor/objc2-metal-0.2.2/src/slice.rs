#![allow(unused_imports)]
#![allow(clippy::missing_safety_doc)]
use std::ptr::NonNull;

use crate::*;

#[allow(dead_code)]
fn slice_to_ptr_count<T>(slice: &[T]) -> (NonNull<T>, usize) {
    let ptr: *const T = slice.as_ptr();
    let ptr: *mut T = ptr as *mut T;
    // SAFETY: Slice pointers are always non-null
    let ptr = unsafe { NonNull::new_unchecked(ptr) };
    (ptr, slice.len())
}

#[cfg(all(feature = "MTLRenderCommandEncoder", feature = "MTLCommandEncoder"))]
pub trait MTLRenderCommandEncoderSliceExt: MTLRenderCommandEncoder + objc2::Message {
    // TODO: Safety
    unsafe fn setViewports(&self, viewports: &[MTLViewport]);
}

#[cfg(all(feature = "MTLRenderCommandEncoder", feature = "MTLCommandEncoder"))]
impl<P: MTLRenderCommandEncoder + objc2::Message> MTLRenderCommandEncoderSliceExt for P {
    unsafe fn setViewports(&self, viewports: &[MTLViewport]) {
        let (ptr, count) = slice_to_ptr_count(viewports);
        unsafe { self.setViewports_count(ptr, count) }
    }
}

// TODO: Many more methods take `(NonNull<Foo>, NSUInteger)` that can be
// turned into `&[Foo]`.
