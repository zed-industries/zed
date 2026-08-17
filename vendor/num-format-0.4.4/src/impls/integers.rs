#![allow(trivial_numeric_casts)]

use core::marker::PhantomData;
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
use core::ptr;

use crate::buffer::Buffer;
use crate::constants::*;
use crate::format::Format;
use crate::grouping::Grouping;
use crate::sealed::Sealed;
use crate::to_formatted_str::ToFormattedStr;

// unsigned integers

impl ToFormattedStr for u8 {
    #[doc(hidden)]
    #[inline(always)]
    fn read_to_buffer<'a, F>(&self, buf: &'a mut Buffer, _: &F) -> usize
    where
        F: Format,
    {
        buf.write_with_itoa(*self)
    }
}

macro_rules! impl_unsigned {
    ($type:ty, $max_len:expr) => {
        impl ToFormattedStr for $type {
            #[doc(hidden)]
            #[inline(always)]
            fn read_to_buffer<'a, F>(&self, buf: &'a mut Buffer, format: &F) -> usize
            where
                F: Format,
            {
                let n = *self as u128;
                run_core_algorithm(n, buf, format)
            }
        }
    };
}

impl_unsigned!(u16, U16_MAX_LEN);
impl_unsigned!(u32, U32_MAX_LEN);
impl_unsigned!(usize, USIZE_MAX_LEN);
impl_unsigned!(u64, U64_MAX_LEN);
impl_unsigned!(u128, U128_MAX_LEN);

impl Sealed for u8 {}
impl Sealed for u16 {}
impl Sealed for u32 {}
impl Sealed for usize {}
impl Sealed for u64 {}
impl Sealed for u128 {}

// signed integers

macro_rules! impl_signed {
    ($type:ty, $max_len:expr) => {
        impl ToFormattedStr for $type {
            #[doc(hidden)]
            #[inline(always)]
            fn read_to_buffer<'a, F>(&self, buf: &'a mut Buffer, format: &F) -> usize
            where
                F: Format,
            {
                if self.is_negative() {
                    let n = (!(*self as u128)).wrapping_add(1); // make positive by adding 1 to the 2s complement
                    let c = run_core_algorithm(n, buf, format);
                    let minus_sign = format.minus_sign().into_str();
                    let min_len = minus_sign.len();
                    buf.pos -= min_len;
                    for (i, byte) in minus_sign.as_bytes().iter().enumerate() {
                        buf.inner[buf.pos + i] = *byte;
                    }
                    c + min_len
                } else {
                    let n = *self as u128;
                    let c = run_core_algorithm(n, buf, format);
                    c
                }
            }
        }
    };
}

impl_signed!(i8, I8_MAX_LEN);
impl_signed!(i16, I16_MAX_LEN);
impl_signed!(i32, I32_MAX_LEN);
impl_signed!(isize, ISIZE_MAX_LEN);
impl_signed!(i64, I64_MAX_LEN);
impl_signed!(i128, I128_MAX_LEN);

impl Sealed for i8 {}
impl Sealed for i16 {}
impl Sealed for i32 {}
impl Sealed for isize {}
impl Sealed for i64 {}
impl Sealed for i128 {}

// non-zero unsigned integers

impl ToFormattedStr for NonZeroU8 {
    #[doc(hidden)]
    #[inline(always)]
    fn read_to_buffer<'a, F>(&self, buf: &'a mut Buffer, _: &F) -> usize
    where
        F: Format,
    {
        buf.write_with_itoa(self.get())
    }
}

macro_rules! impl_non_zero {
    ($type:ty, $related_type:ty, $max_len:expr) => {
        impl ToFormattedStr for $type {
            #[doc(hidden)]
            #[inline(always)]
            fn read_to_buffer<'a, F>(&self, buf: &'a mut Buffer, format: &F) -> usize
            where
                F: Format,
            {
                let n = self.get() as u128;
                run_core_algorithm(n, buf, format)
            }
        }
    };
}

impl_non_zero!(NonZeroU16, u16, U16_MAX_LEN);
impl_non_zero!(NonZeroU32, u32, U32_MAX_LEN);
impl_non_zero!(NonZeroUsize, usize, USIZE_MAX_LEN);
impl_non_zero!(NonZeroU64, u64, U64_MAX_LEN);
impl_non_zero!(NonZeroU128, u128, U128_MAX_LEN);

impl Sealed for NonZeroU8 {}
impl Sealed for NonZeroU16 {}
impl Sealed for NonZeroU32 {}
impl Sealed for NonZeroUsize {}
impl Sealed for NonZeroU64 {}
impl Sealed for NonZeroU128 {}

// helper functions

#[inline(always)]
fn run_core_algorithm<F>(mut n: u128, buf: &mut Buffer, format: &F) -> usize
where
    F: Format,
{
    // Bail out early if we can just use itoa
    // (i.e. if we don't have a separator)
    let separator = format.separator().into_str();
    let grouping = format.grouping();
    if separator.is_empty() || grouping == Grouping::Posix {
        return buf.write_with_itoa(n);
    }

    // Reset our position to the end of the buffer
    buf.pos = MAX_BUF_LEN;
    buf.end = MAX_BUF_LEN;

    // Collect separator information
    let mut sep = Sep {
        ptr: separator.as_bytes().as_ptr(),
        len: separator.len(),
        pos: MAX_BUF_LEN as isize - 4,
        step: match grouping {
            Grouping::Standard => 4isize,
            Grouping::Indian => 3isize,
            Grouping::Posix => unreachable!(),
        },
        phantom: PhantomData,
    };

    // Start the main algorithm
    while n >= 10_000 {
        let remainder = n % 10_000;
        let table_index = ((remainder % 100) << 1) as isize;
        write_two_bytes(buf, &mut sep, table_index);
        let table_index = ((remainder / 100) << 1) as isize;
        write_two_bytes(buf, &mut sep, table_index);
        n /= 10_000;
    }
    let mut n = n as isize;
    while n >= 100 {
        let table_index = (n % 100) << 1;
        write_two_bytes(buf, &mut sep, table_index);
        n /= 100;
    }
    if n >= 10 {
        let table_index = n << 1;
        write_two_bytes(buf, &mut sep, table_index);
    } else {
        let table_index = n << 1;
        write_one_byte(buf, &mut sep, table_index + 1);
    }

    buf.end - buf.pos
}

struct Sep<'a> {
    ptr: *const u8,
    len: usize,
    pos: isize,
    step: isize,
    phantom: PhantomData<&'a ()>,
}

#[inline(always)]
fn write_one_byte(buf: &mut Buffer, sep: &mut Sep<'_>, table_index: isize) {
    buf.pos -= 1;
    if sep.pos == (buf.pos as isize) {
        buf.pos -= sep.len - 1;
        unsafe { ptr::copy_nonoverlapping(sep.ptr, buf.as_mut_ptr().add(buf.pos), sep.len) }
        sep.pos -= sep.step + (sep.len as isize - 1);
        buf.pos -= 1;
    }
    unsafe {
        ptr::copy_nonoverlapping(
            TABLE.as_ptr().offset(table_index),
            buf.as_mut_ptr().add(buf.pos),
            1,
        )
    };
}

#[inline(always)]
fn write_two_bytes(buf: &mut Buffer, sep: &mut Sep<'_>, table_index: isize) {
    write_one_byte(buf, sep, table_index + 1);
    write_one_byte(buf, sep, table_index);
}
