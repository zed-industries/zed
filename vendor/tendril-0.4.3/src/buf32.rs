// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides an unsafe owned buffer type, used in implementing `Tendril`.

use std::{mem, ptr, slice, u32};

use OFLOW;

pub const MIN_CAP: u32 = 16;

pub const MAX_LEN: usize = u32::MAX as usize;

/// A buffer points to a header of type `H`, which is followed by `MIN_CAP` or more
/// bytes of storage.
pub struct Buf32<H> {
    pub ptr: *mut H,
    pub len: u32,
    pub cap: u32,
}

#[inline(always)]
fn bytes_to_vec_capacity<H>(x: u32) -> usize {
    let header = mem::size_of::<H>();
    debug_assert!(header > 0);
    let x = (x as usize).checked_add(header).expect(OFLOW);
    // Integer ceil https://stackoverflow.com/a/2745086/1162888
    1 + ((x - 1) / header)
}

impl<H> Buf32<H> {
    #[inline]
    pub unsafe fn with_capacity(mut cap: u32, h: H) -> Buf32<H> {
        if cap < MIN_CAP {
            cap = MIN_CAP;
        }

        let mut vec = Vec::<H>::with_capacity(bytes_to_vec_capacity::<H>(cap));
        let ptr = vec.as_mut_ptr();
        mem::forget(vec);
        ptr::write(ptr, h);

        Buf32 {
            ptr: ptr,
            len: 0,
            cap: cap,
        }
    }

    #[inline]
    pub unsafe fn destroy(self) {
        mem::drop(Vec::from_raw_parts(
            self.ptr,
            1,
            bytes_to_vec_capacity::<H>(self.cap),
        ));
    }

    #[inline(always)]
    pub unsafe fn data_ptr(&self) -> *mut u8 {
        (self.ptr as *mut u8).offset(mem::size_of::<H>() as isize)
    }

    #[inline(always)]
    pub unsafe fn data(&self) -> &[u8] {
        slice::from_raw_parts(self.data_ptr(), self.len as usize)
    }

    #[inline(always)]
    pub unsafe fn data_mut(&mut self) -> &mut [u8] {
        slice::from_raw_parts_mut(self.data_ptr(), self.len as usize)
    }

    /// Grow the capacity to at least `new_cap`.
    ///
    /// This will panic if the capacity calculation overflows `u32`.
    #[inline]
    pub unsafe fn grow(&mut self, new_cap: u32) {
        if new_cap <= self.cap {
            return;
        }

        let new_cap = new_cap.checked_next_power_of_two().expect(OFLOW);
        let mut vec = Vec::from_raw_parts(self.ptr, 0, bytes_to_vec_capacity::<H>(self.cap));
        vec.reserve_exact(bytes_to_vec_capacity::<H>(new_cap));
        self.ptr = vec.as_mut_ptr();
        self.cap = new_cap;
        mem::forget(vec);
    }
}

#[cfg(test)]
mod test {
    use super::Buf32;
    use std::ptr;

    #[test]
    fn smoke_test() {
        unsafe {
            let mut b = Buf32::with_capacity(0, 0u8);
            assert_eq!(b"", b.data());

            b.grow(5);
            ptr::copy_nonoverlapping(b"Hello".as_ptr(), b.data_ptr(), 5);

            assert_eq!(b"", b.data());
            b.len = 5;
            assert_eq!(b"Hello", b.data());

            b.grow(1337);
            assert!(b.cap >= 1337);
            assert_eq!(b"Hello", b.data());

            b.destroy();
        }
    }
}
