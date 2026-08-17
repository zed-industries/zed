// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use std::{ptr, slice};

#[inline(always)]
pub unsafe fn unsafe_slice<'a>(buf: &'a [u8], start: usize, new_len: usize) -> &'a [u8] {
    debug_assert!(start <= buf.len());
    debug_assert!(new_len <= (buf.len() - start));
    slice::from_raw_parts(buf.as_ptr().offset(start as isize), new_len)
}

#[inline(always)]
pub unsafe fn unsafe_slice_mut<'a>(
    buf: &'a mut [u8],
    start: usize,
    new_len: usize,
) -> &'a mut [u8] {
    debug_assert!(start <= buf.len());
    debug_assert!(new_len <= (buf.len() - start));
    slice::from_raw_parts_mut(buf.as_mut_ptr().offset(start as isize), new_len)
}

#[inline(always)]
pub unsafe fn copy_and_advance(dest: &mut *mut u8, src: &[u8]) {
    ptr::copy_nonoverlapping(src.as_ptr(), *dest, src.len());
    *dest = dest.offset(src.len() as isize)
}

#[inline(always)]
pub unsafe fn copy_lifetime_mut<'a, S: ?Sized, T: ?Sized + 'a>(
    _ptr: &'a mut S,
    ptr: &mut T,
) -> &'a mut T {
    mem::transmute(ptr)
}

#[inline(always)]
pub unsafe fn copy_lifetime<'a, S: ?Sized, T: ?Sized + 'a>(_ptr: &'a S, ptr: &T) -> &'a T {
    mem::transmute(ptr)
}
