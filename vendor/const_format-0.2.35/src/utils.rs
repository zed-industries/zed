//! Miscelaneous functions.

use core::ops::Range;

/// Newtype wrapper to get around limitations in `const fn`s
pub(crate) struct Constructor<T>(#[allow(dead_code)] fn() -> T);

pub use crate::slice_cmp::{str_eq, u8_slice_eq};

#[doc(hidden)]
#[inline]
pub const fn saturate_range(s: &[u8], range: &Range<usize>) -> Range<usize> {
    let len = s.len();
    let end = min_usize(range.end, len);
    min_usize(range.start, end)..end
}

#[doc(hidden)]
#[cfg(feature = "rust_1_64")]
#[repr(C)]
pub union Dereference<'a, T: ?Sized> {
    pub ptr: *const T,
    pub reff: &'a T,
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! slice_up_to_len_alt_docs {
    ($item:item) => {
        /// A const equivalent of `&slice[..len]`.
        ///
        /// If `slice.len() < len`, this simply returns `slice` back.
        ///
        /// # Runtime
        ///
        /// If the "rust_1_64" feature is disabled,
        /// thich takes linear time to remove the trailing elements,
        /// proportional to `slice.len() - len`.
        ///
        /// If the "rust_1_64" feature is enabled, it takes constant time to run.
        ///
        /// # Example
        ///
        /// ```rust
        /// use const_format::utils::slice_up_to_len_alt;
        ///
        /// const FIBB: &[u16] = &[3, 5, 8, 13, 21, 34, 55, 89];
        ///
        /// const TWO: &[u16] = slice_up_to_len_alt(FIBB, 2);
        /// const FOUR: &[u16] = slice_up_to_len_alt(FIBB, 4);
        /// const ALL: &[u16] = slice_up_to_len_alt(FIBB, usize::MAX);
        ///
        /// assert_eq!(TWO, &[3, 5]);
        /// assert_eq!(FOUR, &[3, 5, 8, 13]);
        /// assert_eq!(FIBB, ALL);
        ///
        /// ```
        $item
    };
}

slice_up_to_len_alt_docs! {
    #[cfg(feature = "rust_1_64")]
    #[inline(always)]
    pub const fn slice_up_to_len_alt<T>(slice: &[T], len: usize) -> &[T] {
        slice_up_to_len(slice, len)
    }
}
slice_up_to_len_alt_docs! {
    #[cfg(not(feature = "rust_1_64"))]
    pub const fn slice_up_to_len_alt<T>(slice: &[T], len: usize) -> &[T] {
        let mut rem = slice.len().saturating_add(1).saturating_sub(len);
        let mut ret = slice;

        if rem == 0 {
            return slice;
        }

        macro_rules! slice_up_to_len_alt_impl{
            (
                $( ($len:expr, [$($ignored:tt)*]) )*
            )=>{
                $(
                    while rem > $len {
                        if let [next @ .., $($ignored)* ] = ret {
                            ret = next;
                            rem -= $len;
                        }
                    }
                )*
            }
        }
        slice_up_to_len_alt_impl!{
            (36, [_, _, _, _, _, _,_, _, _, _, _, _,_, _, _, _, _, _,_, _, _, _, _, _,_, _, _, _, _, _,_, _, _, _, _, _,])
            (6, [_, _, _, _, _, _])
            (1, [_])
        }
        ret
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! slice_up_to_len_docs {
    ($item:item) => {
        /// A conditionally-const equivalent of `&slice[..len]`.
        ///
        /// If `slice.len() < len`, this simply returns `slice` back.
        ///
        /// # Constness
        ///
        /// This function takes constant time,
        /// and in order to be `const fn` it requires the "rust_1_64"
        /// feature to be enabled.
        ///
        /// # Example
        ///
        /// ```rust
        /// use const_format::utils::slice_up_to_len_alt;
        ///
        /// const FIBB: &[u16] = &[3, 5, 8, 13, 21, 34, 55, 89];
        ///
        /// const TWO: &[u16] = slice_up_to_len_alt(FIBB, 2);
        /// const FOUR: &[u16] = slice_up_to_len_alt(FIBB, 4);
        /// const ALL: &[u16] = slice_up_to_len_alt(FIBB, usize::MAX);
        ///
        /// assert_eq!(TWO, &[3, 5]);
        /// assert_eq!(FOUR, &[3, 5, 8, 13]);
        /// assert_eq!(FIBB, ALL);
        ///
        /// ```
        $item
    };
}

slice_up_to_len_docs! {
    #[cfg(feature = "rust_1_64")]
    #[inline]
    pub const fn slice_up_to_len<T>(slice: &[T], len: usize) -> &[T] {
        if len > slice.len() {
            return slice;
        }

        // Doing this to get a slice up to length at compile-time
        unsafe {
            let raw_slice = core::ptr::slice_from_raw_parts(slice.as_ptr(), len);
            Dereference { ptr: raw_slice }.reff
        }
    }
}

slice_up_to_len_docs! {
    #[cfg(not(feature = "rust_1_64"))]
    #[inline]
    pub fn slice_up_to_len<T>(slice: &[T], len: usize) -> &[T] {
        if len > slice.len() {
            return slice;
        }

        &slice[..len]
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) const fn min_usize(l: usize, r: usize) -> usize {
    if l < r {
        l
    } else {
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_up_to_len_alt() {
        let mut list = [0u16; 256];

        (100..).zip(list.iter_mut()).for_each(|(i, m)| *m = i);

        for i in 0..list.len() {
            assert_eq!(slice_up_to_len_alt(&list, i), &list[..i]);
        }
    }

    #[test]
    fn slice_in_bounds() {
        assert_eq!(slice_up_to_len(&[3, 5], 0), []);
        assert_eq!(slice_up_to_len(&[3, 5], 1), [3]);
        assert_eq!(slice_up_to_len(&[3, 5], 2), [3, 5]);
        assert_eq!(slice_up_to_len(&[3, 5], 3), [3, 5]);
        assert_eq!(slice_up_to_len(&[3, 5], 4), [3, 5]);
    }
}
