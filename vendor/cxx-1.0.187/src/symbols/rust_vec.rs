#![cfg(feature = "alloc")]

use crate::rust_string::RustString;
use crate::rust_vec::RustVec;
use alloc::vec::Vec;
use core::ffi::c_char;
use core::mem;
use core::ptr;

macro_rules! rust_vec_shims {
    ($segment:expr, $ty:ty) => {
        const_assert_eq!(mem::size_of::<[usize; 3]>(), mem::size_of::<RustVec<$ty>>());
        const_assert_eq!(mem::size_of::<Vec<$ty>>(), mem::size_of::<RustVec<$ty>>());
        const_assert_eq!(mem::align_of::<Vec<$ty>>(), mem::align_of::<RustVec<$ty>>());

        const _: () = {
            #[export_name = concat!("cxxbridge1$rust_vec$", $segment, "$new")]
            unsafe extern "C" fn __new(this: *mut RustVec<$ty>) {
                unsafe { ptr::write(this, RustVec::new()) }
            }
            #[export_name = concat!("cxxbridge1$rust_vec$", $segment, "$drop")]
            unsafe extern "C" fn __drop(this: *mut RustVec<$ty>) {
                unsafe { ptr::drop_in_place(this) }
            }
            #[export_name = concat!("cxxbridge1$rust_vec$", $segment, "$len")]
            unsafe extern "C" fn __len(this: *const RustVec<$ty>) -> usize {
                unsafe { &*this }.len()
            }
            #[export_name = concat!("cxxbridge1$rust_vec$", $segment, "$capacity")]
            unsafe extern "C" fn __capacity(this: *const RustVec<$ty>) -> usize {
                unsafe { &*this }.capacity()
            }
            #[export_name = concat!("cxxbridge1$rust_vec$", $segment, "$data")]
            unsafe extern "C" fn __data(this: *const RustVec<$ty>) -> *const $ty {
                unsafe { &*this }.as_ptr()
            }
            #[export_name = concat!("cxxbridge1$rust_vec$", $segment, "$reserve_total")]
            unsafe extern "C" fn __reserve_total(this: *mut RustVec<$ty>, new_cap: usize) {
                unsafe { &mut *this }.reserve_total(new_cap);
            }
            #[export_name = concat!("cxxbridge1$rust_vec$", $segment, "$set_len")]
            unsafe extern "C" fn __set_len(this: *mut RustVec<$ty>, len: usize) {
                unsafe { (*this).set_len(len) }
            }
            #[export_name = concat!("cxxbridge1$rust_vec$", $segment, "$truncate")]
            unsafe extern "C" fn __truncate(this: *mut RustVec<$ty>, len: usize) {
                unsafe { (*this).truncate(len) }
            }
        };
    };
}

macro_rules! rust_vec_shims_for_primitive {
    ($ty:ident) => {
        rust_vec_shims!(stringify!($ty), $ty);
    };
}

rust_vec_shims_for_primitive!(bool);
rust_vec_shims_for_primitive!(u8);
rust_vec_shims_for_primitive!(u16);
rust_vec_shims_for_primitive!(u32);
rust_vec_shims_for_primitive!(u64);
rust_vec_shims_for_primitive!(usize);
rust_vec_shims_for_primitive!(i8);
rust_vec_shims_for_primitive!(i16);
rust_vec_shims_for_primitive!(i32);
rust_vec_shims_for_primitive!(i64);
rust_vec_shims_for_primitive!(isize);
rust_vec_shims_for_primitive!(f32);
rust_vec_shims_for_primitive!(f64);

rust_vec_shims!("char", c_char);
rust_vec_shims!("string", RustString);
rust_vec_shims!("str", &str);
