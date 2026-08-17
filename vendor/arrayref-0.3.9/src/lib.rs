//! This package contains just four macros, which enable the creation
//! of array references to portions of arrays or slices (or things
//! that can be sliced).
//!
//! # Examples
//!
//! Here is a simple example of slicing and dicing a slice into array
//! references with these macros.  Here we implement a simple
//! little-endian conversion from bytes to `u16`, and demonstrate code
//! that uses `array_ref!` to extract an array reference from a larger
//! array.  Note that the documentation for each macro also has an
//! example of its use.
//!
//! ```
//! #[macro_use]
//! extern crate arrayref;
//!
//! fn read_u16(bytes: &[u8; 2]) -> u16 {
//!      bytes[0] as u16 + ((bytes[1] as u16) << 8)
//! }
//! // ...
//! # fn main() {
//! let data = [0,1,2,3,4,0,6,7,8,9];
//! assert_eq!(256, read_u16(array_ref![data,0,2]));
//! assert_eq!(4, read_u16(array_ref![data,4,2]));
//! # }
//! ```
#![deny(warnings)]
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

/// You can use `array_ref` to generate an array reference to a subset
/// of a sliceable bit of data (which could be an array, or a slice,
/// or a Vec).
///
/// **Panics** if the slice is out of bounds.
///
/// ```
/// #[macro_use]
/// extern crate arrayref;
///
/// fn read_u16(bytes: &[u8; 2]) -> u16 {
///      bytes[0] as u16 + ((bytes[1] as u16) << 8)
/// }
/// // ...
/// # fn main() {
/// let data = [0,1,2,3,4,0,6,7,8,9];
/// assert_eq!(256, read_u16(array_ref![data,0,2]));
/// assert_eq!(4, read_u16(array_ref![data,4,2]));
/// # }
/// ```

#[macro_export]
macro_rules! array_ref {
    ($arr:expr, $offset:expr, $len:expr) => {{
        {
            #[inline]
            const unsafe fn as_array<T>(slice: &[T]) -> &[T; $len] {
                &*(slice.as_ptr() as *const [_; $len])
            }
            let offset = $offset;
            let slice = &$arr[offset..offset + $len];
            #[allow(unused_unsafe)]
            unsafe {
                as_array(slice)
            }
        }
    }};
}

/// You can use `array_refs` to generate a series of array references
/// to an input array reference.  The idea is if you want to break an
/// array into a series of contiguous and non-overlapping arrays.
/// `array_refs` is a bit funny in that it insists on slicing up the
/// *entire* array.  This is intentional, as I find it handy to make
/// me ensure that my sub-arrays add up to the entire array.  This
/// macro will *never* panic, since the sizes are all checked at
/// compile time.
///
/// Note that unlike `array_ref!`, `array_refs` *requires* that the
/// first argument be an array reference.  The following arguments are
/// the lengths of each subarray you wish a reference to.  The total
/// of these arguments *must* equal the size of the array itself.
///
/// ```
/// #[macro_use]
/// extern crate arrayref;
///
/// fn read_u16(bytes: &[u8; 2]) -> u16 {
///      bytes[0] as u16 + ((bytes[1] as u16) << 8)
/// }
/// // ...
/// # fn main() {
/// let data = [0,1,2,3,4,0,6,7];
/// let (a,b,c) = array_refs![&data,2,2,4];
/// assert_eq!(read_u16(a), 256);
/// assert_eq!(read_u16(b), 3*256+2);
/// assert_eq!(*c, [4,0,6,7]);
/// # }
/// ```
#[macro_export]
macro_rules! array_refs {
    ( $arr:expr, $( $pre:expr ),* ; .. ;  $( $post:expr ),* ) => {{
        {
            use core::slice;
            #[inline]
            #[allow(unused_assignments)]
            #[allow(clippy::eval_order_dependence)]
            const unsafe fn as_arrays<T>(a: &[T]) -> ( $( &[T; $pre], )* &[T],  $( &[T; $post], )*) {
                const MIN_LEN: usize = 0usize $( .saturating_add($pre) )* $( .saturating_add($post) )*;
                assert!(MIN_LEN < usize::MAX, "Your arrays are too big, are you trying to hack yourself?!");
                let var_len = a.len() - MIN_LEN;
                assert!(a.len() >= MIN_LEN);
                let mut p = a.as_ptr();
                ( $( {
                    let aref = & *(p as *const [T; $pre]);
                    p = p.add($pre);
                    aref
                }, )* {
                    let sl = slice::from_raw_parts(p as *const T, var_len);
                    p = p.add(var_len);
                    sl
                }, $( {
                    let aref = & *(p as *const [T; $post]);
                    p = p.add($post);
                    aref
                }, )*)
            }
            let input = $arr;
            #[allow(unused_unsafe)]
            unsafe {
                as_arrays(input)
            }
        }
    }};
    ( $arr:expr, $( $len:expr ),* ) => {{
        {
            #[inline]
            #[allow(unused_assignments)]
            #[allow(clippy::eval_order_dependence)]
            const unsafe fn as_arrays<T>(a: &[T; $( $len + )* 0 ]) -> ( $( &[T; $len], )* ) {
                let mut p = a.as_ptr();
                ( $( {
                    let aref = &*(p as *const [T; $len]);
                    p = p.offset($len as isize);
                    aref
                }, )* )
            }
            let input = $arr;
            #[allow(unused_unsafe)]
            unsafe {
                as_arrays(input)
            }
        }
    }}
}

/// You can use `mut_array_refs` to generate a series of mutable array
/// references to an input mutable array reference.  The idea is if
/// you want to break an array into a series of contiguous and
/// non-overlapping mutable array references.  Like `array_refs!`,
/// `mut_array_refs!` is a bit funny in that it insists on slicing up
/// the *entire* array.  This is intentional, as I find it handy to
/// make me ensure that my sub-arrays add up to the entire array.
/// This macro will *never* panic, since the sizes are all checked at
/// compile time.
///
/// Note that unlike `array_mut_ref!`, `mut_array_refs` *requires*
/// that the first argument be a mutable array reference.  The
/// following arguments are the lengths of each subarray you wish a
/// reference to.  The total of these arguments *must* equal the size
/// of the array itself.  Also note that this macro allows you to take
/// out multiple mutable references to a single object, which is both
/// weird and powerful.
///
/// ```
/// #[macro_use]
/// extern crate arrayref;
///
/// fn write_u16(bytes: &mut [u8; 2], num: u16) {
///      bytes[0] = num as u8;
///      bytes[1] = (num >> 8) as u8;
/// }
/// fn write_u32(bytes: &mut [u8; 4], num: u32) {
///      bytes[0] = num as u8;
///      bytes[1] = (num >> 8) as u8; // this is buggy to save space...
/// }
/// // ...
/// # fn main() {
/// let mut data = [0,1,2,3,4,0,6,7];
/// let (a,b,c) = mut_array_refs![&mut data,2,2,4];
/// // let's write out some nice prime numbers!
/// write_u16(a, 37);
/// write_u16(b, 73);
/// write_u32(c, 137); // approximate inverse of the fine structure constant!
/// # }
/// ```
#[macro_export]
macro_rules! mut_array_refs {
    ( $arr:expr, $( $pre:expr ),* ; .. ;  $( $post:expr ),* ) => {{
        {
            use core::slice;
            #[inline]
            #[allow(unused_assignments)]
            #[allow(clippy::eval_order_dependence)]
            unsafe fn as_arrays<T>(a: &mut [T]) -> ( $( &mut [T; $pre], )* &mut [T],  $( &mut [T; $post], )*) {
                const MIN_LEN: usize = 0usize $( .saturating_add($pre) )* $( .saturating_add($post) )*;
                assert!(MIN_LEN < usize::MAX, "Your arrays are too big, are you trying to hack yourself?!");
                let var_len = a.len() - MIN_LEN;
                assert!(a.len() >= MIN_LEN);
                let mut p = a.as_mut_ptr();
                ( $( {
                    let aref = &mut *(p as *mut [T; $pre]);
                    p = p.add($pre);
                    aref
                }, )* {
                    let sl = slice::from_raw_parts_mut(p as *mut T, var_len);
                    p = p.add(var_len);
                    sl
                }, $( {
                    let aref = &mut *(p as *mut [T; $post]);
                    p = p.add($post);
                    aref
                }, )*)
            }
            let input = $arr;
            #[allow(unused_unsafe)]
            unsafe {
                as_arrays(input)
            }
        }
    }};
    ( $arr:expr, $( $len:expr ),* ) => {{
        {
            #[inline]
            #[allow(unused_assignments)]
            #[allow(clippy::eval_order_dependence)]
            unsafe fn as_arrays<T>(a: &mut [T; $( $len + )* 0 ]) -> ( $( &mut [T; $len], )* ) {
                let mut p = a.as_mut_ptr();
                ( $( {
                    let aref = &mut *(p as *mut [T; $len]);
                    p = p.add($len);
                    aref
                }, )* )
            }
            let input = $arr;
            #[allow(unused_unsafe)]
            unsafe {
                as_arrays(input)
            }
        }
    }};
}

/// You can use `array_mut_ref` to generate a mutable array reference
/// to a subset of a sliceable bit of data (which could be an array,
/// or a slice, or a Vec).
///
/// **Panics** if the slice is out of bounds.
///
/// ```
/// #[macro_use]
/// extern crate arrayref;
///
/// fn write_u16(bytes: &mut [u8; 2], num: u16) {
///      bytes[0] = num as u8;
///      bytes[1] = (num >> 8) as u8;
/// }
/// // ...
/// # fn main() {
/// let mut data = [0,1,2,3,4,0,6,7,8,9];
/// write_u16(array_mut_ref![data,0,2], 1);
/// write_u16(array_mut_ref![data,2,2], 5);
/// assert_eq!(*array_ref![data,0,4], [1,0,5,0]);
/// *array_mut_ref![data,4,5] = [4,3,2,1,0];
/// assert_eq!(data, [1,0,5,0,4,3,2,1,0,9]);
/// # }
/// ```
#[macro_export]
macro_rules! array_mut_ref {
    ($arr:expr, $offset:expr, $len:expr) => {{
        {
            #[inline]
            unsafe fn as_array<T>(slice: &mut [T]) -> &mut [T; $len] {
                &mut *(slice.as_mut_ptr() as *mut [_; $len])
            }
            let offset = $offset;
            let slice = &mut $arr[offset..offset + $len];
            #[allow(unused_unsafe)]
            unsafe {
                as_array(slice)
            }
        }
    }};
}

#[allow(clippy::all)]
#[cfg(test)]
mod test {

    extern crate quickcheck;

    use std::vec::Vec;

    // use super::*;

    #[test]
    #[should_panic]
    fn checks_bounds() {
        let foo: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let bar = array_ref!(foo, 1, 11);
        println!("I am checking that I can dereference bar[0] = {}", bar[0]);
    }

    #[test]
    fn simple_case_works() {
        fn check(expected: [u8; 3], actual: &[u8; 3]) {
            for (e, a) in (&expected).iter().zip(actual.iter()) {
                assert_eq!(e, a)
            }
        }
        let mut foo: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        {
            let bar = array_ref!(foo, 2, 3);
            check([2, 3, 4], bar);
        }
        check([0, 1, 2], array_ref!(foo, 0, 3));
        fn zero2(x: &mut [u8; 2]) {
            x[0] = 0;
            x[1] = 0;
        }
        zero2(array_mut_ref!(foo, 8, 2));
        check([0, 0, 10], array_ref!(foo, 8, 3));
    }

    #[test]
    fn check_array_ref_5() {
        fn f(data: Vec<u8>, offset: usize) -> quickcheck::TestResult {
            // Compute the following, with correct results even if the sum would overflow:
            //   if data.len() < offset + 5
            if data.len() < 5 || data.len() - 5 < offset {
                return quickcheck::TestResult::discard();
            }
            let out = array_ref!(data, offset, 5);
            quickcheck::TestResult::from_bool(out.len() == 5)
        }
        quickcheck::quickcheck(f as fn(Vec<u8>, usize) -> quickcheck::TestResult);
    }

    #[test]
    fn check_array_ref_out_of_bounds_5() {
        fn f(data: Vec<u8>, offset: usize) -> quickcheck::TestResult {
            // Compute the following, with correct results even if the sum would overflow:
            //   if data.len() >= offset + 5
            if data.len() >= 5 && data.len() - 5 >= offset {
                return quickcheck::TestResult::discard();
            }
            quickcheck::TestResult::must_fail(move || {
                array_ref!(data, offset, 5);
            })
        }
        quickcheck::quickcheck(f as fn(Vec<u8>, usize) -> quickcheck::TestResult);
    }

    #[test]
    fn check_array_mut_ref_7() {
        fn f(mut data: Vec<u8>, offset: usize) -> quickcheck::TestResult {
            // Compute the following, with correct results even if the sum would overflow:
            //   if data.len() < offset + 7
            if data.len() < 7 || data.len() - 7 < offset {
                return quickcheck::TestResult::discard();
            }
            let out = array_mut_ref!(data, offset, 7);
            out[6] = 3;
            quickcheck::TestResult::from_bool(out.len() == 7)
        }
        quickcheck::quickcheck(f as fn(Vec<u8>, usize) -> quickcheck::TestResult);
    }

    #[test]
    fn check_array_mut_ref_out_of_bounds_32() {
        fn f(mut data: Vec<u8>, offset: usize) -> quickcheck::TestResult {
            // Compute the following, with correct results even if the sum would overflow:
            //   if data.len() >= offset + 32
            if data.len() >= 32 && data.len() - 32 >= offset {
                return quickcheck::TestResult::discard();
            }
            quickcheck::TestResult::must_fail(move || {
                array_mut_ref!(data, offset, 32);
            })
        }
        quickcheck::quickcheck(f as fn(Vec<u8>, usize) -> quickcheck::TestResult);
    }

    #[test]
    fn test_5_array_refs() {
        let mut data: [usize; 128] = [0; 128];
        for i in 0..128 {
            data[i] = i;
        }
        let data = data;
        let (a, b, c, d, e) = array_refs!(&data, 1, 14, 3, 100, 10);
        assert_eq!(a.len(), 1 as usize);
        assert_eq!(b.len(), 14 as usize);
        assert_eq!(c.len(), 3 as usize);
        assert_eq!(d.len(), 100 as usize);
        assert_eq!(e.len(), 10 as usize);
        assert_eq!(a, array_ref![data, 0, 1]);
        assert_eq!(b, array_ref![data, 1, 14]);
        assert_eq!(c, array_ref![data, 15, 3]);
        assert_eq!(e, array_ref![data, 118, 10]);
    }

    #[test]
    fn test_5_array_refs_dotdot() {
        let mut data: [usize; 128] = [0; 128];
        for i in 0..128 {
            data[i] = i;
        }
        let data = data;
        let (a, b, c, d, e) = array_refs!(&data, 1, 14, 3; ..; 10);
        assert_eq!(a.len(), 1 as usize);
        assert_eq!(b.len(), 14 as usize);
        assert_eq!(c.len(), 3 as usize);
        assert_eq!(d.len(), 100 as usize);
        assert_eq!(e.len(), 10 as usize);
        assert_eq!(a, array_ref![data, 0, 1]);
        assert_eq!(b, array_ref![data, 1, 14]);
        assert_eq!(c, array_ref![data, 15, 3]);
        assert_eq!(e, array_ref![data, 118, 10]);
    }

    #[test]
    fn test_5_mut_xarray_refs() {
        let mut data: [usize; 128] = [0; 128];
        {
            // temporarily borrow the data to modify it.
            let (a, b, c, d, e) = mut_array_refs!(&mut data, 1, 14, 3, 100, 10);
            assert_eq!(a.len(), 1 as usize);
            assert_eq!(b.len(), 14 as usize);
            assert_eq!(c.len(), 3 as usize);
            assert_eq!(d.len(), 100 as usize);
            assert_eq!(e.len(), 10 as usize);
            *a = [1; 1];
            *b = [14; 14];
            *c = [3; 3];
            *d = [100; 100];
            *e = [10; 10];
        }
        assert_eq!(&[1; 1], array_ref![data, 0, 1]);
        assert_eq!(&[14; 14], array_ref![data, 1, 14]);
        assert_eq!(&[3; 3], array_ref![data, 15, 3]);
        assert_eq!(&[10; 10], array_ref![data, 118, 10]);
    }

    #[test]
    fn test_5_mut_xarray_refs_with_dotdot() {
        let mut data: [usize; 128] = [0; 128];
        {
            // temporarily borrow the data to modify it.
            let (a, b, c, d, e) = mut_array_refs!(&mut data, 1, 14, 3; ..; 10);
            assert_eq!(a.len(), 1 as usize);
            assert_eq!(b.len(), 14 as usize);
            assert_eq!(c.len(), 3 as usize);
            assert_eq!(d.len(), 100 as usize);
            assert_eq!(e.len(), 10 as usize);
            *a = [1; 1];
            *b = [14; 14];
            *c = [3; 3];
            *e = [10; 10];
        }
        assert_eq!(&[1; 1], array_ref![data, 0, 1]);
        assert_eq!(&[14; 14], array_ref![data, 1, 14]);
        assert_eq!(&[3; 3], array_ref![data, 15, 3]);
        assert_eq!(&[10; 10], array_ref![data, 118, 10]);
    }

    #[forbid(clippy::ptr_offset_with_cast)]
    #[test]
    fn forbidden_clippy_lints_do_not_fire() {
        let mut data = [0u8; 32];
        let _ = array_refs![&data, 8; .. ;];
        let _ = mut_array_refs![&mut data, 8; .. ; 10];
    }

    #[test]
    fn single_arg_refs() {
        let mut data = [0u8; 8];
        let (_,) = array_refs![&data, 8];
        let (_,) = mut_array_refs![&mut data, 8];

        let (_, _) = array_refs![&data, 4; ..;];
        let (_, _) = mut_array_refs![&mut data, 4; ..;];

        let (_, _) = array_refs![&data,; ..; 4];
        let (_, _) = mut_array_refs![&mut data,; ..; 4];

        let (_,) = array_refs![&data,; ..;];
        let (_,) = mut_array_refs![&mut data,; ..;];
    }
} // mod test
