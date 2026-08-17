#![allow(missing_docs)]

use crate::{
    fmt::{Error, Formatter},
    marker_traits::IsStdKind,
    wrapper_types::PWrapper,
};

mod ranges;

////////////////////////////////////////////////////////////////////////////////

impl PWrapper<&str> {
    pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.0)
    }

    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str_debug(self.0)
    }
}

impl PWrapper<bool> {
    pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(if self.0 { "true" } else { "false" })
    }

    #[inline(always)]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.const_display_fmt(f)
    }
}

impl PWrapper<char> {
    pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_char(self.0)
    }

    #[inline(always)]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_char_debug(self.0)
    }
}

macro_rules! slice_of_std_impl {($($elem:ty),* $(,)?) => (
    $(

        impl PWrapper<&[$elem]> {
            pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                let mut f = f.debug_list();
                __for_range!{i in 0..self.0.len() =>
                    try_!(PWrapper(self.0[i]).const_debug_fmt(f.entry()));
                }
                f.finish()
            }
        }
    )*
)}

slice_of_std_impl! {
    &str,
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

use core::{
    marker::{PhantomData, PhantomPinned},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ptr::NonNull,
};

impl_fmt! {
    is_std_type;

    impl[T,] Option<NonNull<T>>;
    impl[] Option<NonZeroU8>;
    impl[] Option<NonZeroI8>;
    impl[] Option<NonZeroU16>;
    impl[] Option<NonZeroI16>;
    impl[] Option<NonZeroU32>;
    impl[] Option<NonZeroI32>;
    impl[] Option<NonZeroU64>;
    impl[] Option<NonZeroI64>;
    impl[] Option<NonZeroU128>;
    impl[] Option<NonZeroI128>;
    impl[] Option<NonZeroUsize>;
    impl[] Option<NonZeroIsize>;
    impl[] Option<u8>;
    impl[] Option<i8>;
    impl[] Option<u16>;
    impl[] Option<i16>;
    impl[] Option<u32>;
    impl[] Option<i32>;
    impl[] Option<u64>;
    impl[] Option<i64>;
    impl[] Option<u128>;
    impl[] Option<i128>;
    impl[] Option<usize>;
    impl[] Option<isize>;
    impl[] Option<bool>;
    impl[] Option<char>;
    impl['a,] Option<&'a str>;

    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self.0 {
            Some(x) => {
                let mut f = f.debug_tuple("Some");
                try_!(PWrapper(x).const_debug_fmt(f.field()));
                f.finish()
            },
            None => f.write_str("None"),
        }
    }
}

macro_rules! non_zero_impls {
    ($($ty:ident,)*) => (
        $(
            std_kind_impl!{ impl[] $ty }

            impl PWrapper<$ty> {
                #[inline(always)]
                pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                    PWrapper(self.0.get()).const_debug_fmt(f)
                }

                #[inline(always)]
                pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                    PWrapper(self.0.get()).const_display_fmt(f)
                }
            }
        )*
    )
}

non_zero_impls! {
    NonZeroU8, NonZeroI8, NonZeroU16, NonZeroI16,
    NonZeroU32, NonZeroI32, NonZeroU64, NonZeroI64,
    NonZeroU128, NonZeroI128, NonZeroUsize, NonZeroIsize,
}

std_kind_impl! { impl[T,] *mut T }
// Unfortunately, can't print pointer addresses at compile-time.
impl<T> PWrapper<*mut T> {
    const PTR: &'static str = "<pointer>";

    #[inline(always)]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(Self::PTR)
    }
}

std_kind_impl! { impl[T,] *const T }
impl<T> PWrapper<*const T> {
    #[inline(always)]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        PWrapper(self.0 as *mut T).const_debug_fmt(f)
    }
}

std_kind_impl! { impl[T,] NonNull<T> }
impl<T> PWrapper<NonNull<T>> {
    #[inline(always)]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        PWrapper(self.0.as_ptr()).const_debug_fmt(f)
    }
}

macro_rules! impl_std_marker_type {
    (
        $( impl[$($impl_:tt)*] $type:ty = $tyname:expr ;)*
    ) => (
        $(
            std_kind_impl!{ impl[$($impl_)*] $type }

            impl<$($impl_)*> PWrapper<$type> {
                const NAME: &'static str = $tyname;

                #[inline(always)]
                pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                    PWrapper(Self::NAME).const_display_fmt(f)
                }
            }
        )*
    )
}

impl_std_marker_type! {
    impl[T: ?Sized,] PhantomData<T> = "PhantomData";
    impl[] PhantomPinned = "PhantomPinned";
    impl[] () = "()";
}

////////////////////////////////////////////////////////////////////////////////

use core::{cmp::Ordering, sync::atomic::Ordering as AtomicOrdering};

impl_fmt! {
    is_std_type;

    impl AtomicOrdering;

    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self.0 {
            AtomicOrdering::Relaxed => f.write_str("Relaxed"),
            AtomicOrdering::Release => f.write_str("Release"),
            AtomicOrdering::Acquire => f.write_str("Acquire"),
            AtomicOrdering::AcqRel => f.write_str("AcqRel"),
            AtomicOrdering::SeqCst => f.write_str("SeqCst"),
            _ => f.write_str("<core::atomic::Ordering>"),
        }
    }
}

impl_fmt! {
    is_std_type;

    impl Ordering;

    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self.0 {
            Ordering::Less => f.write_str("Less"),
            Ordering::Equal => f.write_str("Equal"),
            Ordering::Greater => f.write_str("Greater"),
        }
    }
}
