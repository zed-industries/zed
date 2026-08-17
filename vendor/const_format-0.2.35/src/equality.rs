#![allow(missing_docs, unused_variables)]

use crate::wrapper_types::PWrapper;

use core::{
    cmp::Ordering,
    marker::{PhantomData, PhantomPinned},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
    sync::atomic::Ordering as AtomicOrdering,
};

////////////////////////////////////////////////////////////////////////////////

macro_rules! slice_of_const_eq {($($elem:ty),* $(,)?) => (
    $(
        impl PWrapper<&[$elem]> {
            /// This method is only available with the "assert" feature.
            pub const fn const_eq(&self, other: &[$elem]) -> bool {
                if self.0.len() != other.len() {
                    return false;
                }

                __for_range!{i in 0..self.0.len() =>
                    if !PWrapper(self.0[i]).const_eq(&other[i]) {
                        return false
                    }
                }
                true
            }
        }
    )*
)}

slice_of_const_eq! {
    &str,
}

macro_rules! slice_of_equal_op_impl {($($elem:ty),* $(,)?) => (
    $(
        impl PWrapper<&[$elem]> {
            /// This method is only available with the "assert" feature.
            pub const fn const_eq(&self, other: &[$elem]) -> bool {
                if self.0.len() != other.len() {
                    return false;
                }

                __for_range!{i in 0..self.0.len() =>
                    if self.0[i] != other[i] {
                        return false
                    }
                }
                true
            }
        }
    )*
)}

slice_of_equal_op_impl! {
    bool,
    char,
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128,
    usize, isize,
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_eq_for_option_prim {
    (
        (l=$l:ident, r=$r:ident)
        $( impl[$($impl_:tt)*] $type:ty = $comparison:expr; )*
    ) => (
        $(
            impl<$($impl_)*> PWrapper<Option<$type>> {
                /// This method is only available with the "assert" feature.
                pub const fn const_eq(&self, other:&Option<$type>) -> bool {
                    match (self.0, other) {
                        (Some($l), Some($r)) => $comparison,
                        (None, None) => true,
                        _ => false,
                    }
                }
            }
        )*
    )
}

impl_eq_for_option_prim! {
    (l=l, r=r)
    impl[] u8 = l == *r;
    impl[] i8 = l == *r;
    impl[] u16 = l == *r;
    impl[] i16 = l == *r;
    impl[] u32 = l == *r;
    impl[] i32 = l == *r;
    impl[] u64 = l == *r;
    impl[] i64 = l == *r;
    impl[] u128 = l == *r;
    impl[] i128 = l == *r;
    impl[] usize = l == *r;
    impl[] isize = l == *r;
    impl[] bool = l == *r;
    impl[] char = l == *r;
    impl[] &str = crate::slice_cmp::str_eq(l, r);
}

macro_rules! impl_eq_for_option {
    (
        (l=$l:ident, r=$r:ident)
        $( impl[$($impl_:tt)*] $type:ty = $comparison:expr; )*
    ) => (
        $(
            impl<$($impl_)*> PWrapper<$type> {
                /// This method is only available with the "assert" feature.
                pub const fn const_eq(&self, $r:&$type) -> bool {
                    let $l = self.0;
                    $comparison
                }
            }
        )*

        impl_eq_for_option_prim! {
            (l=$l, r=$r)
            $( impl[$($impl_)*] $type = $comparison; )*
        }
    )
}

impl_eq_for_option! {
    (l=l, r=r)

    impl[] NonZeroU8 = l.get() == r.get();
    impl[] NonZeroI8 = l.get() == r.get();
    impl[] NonZeroU16 = l.get() == r.get();
    impl[] NonZeroI16 = l.get() == r.get();
    impl[] NonZeroU32 = l.get() == r.get();
    impl[] NonZeroI32 = l.get() == r.get();
    impl[] NonZeroU64 = l.get() == r.get();
    impl[] NonZeroI64 = l.get() == r.get();
    impl[] NonZeroU128 = l.get() == r.get();
    impl[] NonZeroI128 = l.get() == r.get();
    impl[] NonZeroUsize = l.get() == r.get();
    impl[] NonZeroIsize = l.get() == r.get();
}

macro_rules! impl_equality {
    (
        (l=$l:ident, r=$r:ident)

        $( impl[$($impl_:tt)*] $type:ty = $comparison:expr ;)*
    ) => (
        $(
            impl<$($impl_)*> PWrapper<$type> {
                /// This method is only available with the "assert" feature.
                #[inline(always)]
                pub const fn const_eq(&self, $r: &$type) -> bool {
                    let $l = &self.0;
                    $comparison
                }
            }
        )*
    )
}

impl_equality! {
    (l=l, r=r)

    impl[T: ?Sized,] PhantomData<T> = true;
    impl[] PhantomPinned = true;
    impl[] () = true;

    impl[] Ordering = *l as u8 == *r as u8;
    impl[] AtomicOrdering = *l as u8 == *r as u8;

    impl[] Range<usize>            = l.start == r.start && l.end == r.end;
    impl[] RangeInclusive<usize>   = *l.start() == *r.start() && *l.end() == *r.end();
    impl[] RangeFrom<usize>        = l.start == r.start;
    impl[] RangeFull               = true;
    impl[] RangeTo<usize>          = l.end == r.end;
    impl[] RangeToInclusive<usize> = l.end == r.end;
}
