// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::cmp::min;
use std::mem::{size_of, zeroed};

//
// public functions
//

pub unsafe fn mem_eq<T: Sized>(a: &T, b: &T) -> bool {
    let a_addr = a as *const T as usize;
    let b_addr = b as *const T as usize;

    for i in 0..size_of::<T>() {
        if *((a_addr + i) as *const u8) != *((b_addr + i) as *const u8) {
            return false;
        }
    }

    true
}

pub unsafe fn transmute_union<I, O>(input: &I) -> O
where
    I: Sized,
    O: Sized,
{
    let mut output: O = zeroed();
    let copy_len = min(size_of::<I>(), size_of::<O>());

    for i in 0..copy_len {
        *((&mut output as *mut O as usize + i) as *mut u8) =
            *((input as *const I as usize + i) as *const u8);
    }

    output
}
