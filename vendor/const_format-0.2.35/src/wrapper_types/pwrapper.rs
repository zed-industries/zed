#![allow(unexpected_cfgs)]

use crate::{
    formatting::{FormattingFlags, NumberFormatting, StartAndArray, FOR_ESCAPING},
    pargument::Integer,
};

use core::ops::Range;

#[cfg(test)]
mod tests;

/// Wrapper for many std types,
/// which implements the `const_debug_fmt` and/or `const_display_fmt` methods for them.
///
/// The macros from this crate automatically wraps std types in this type,
/// so you only need to use it if you're manually calling the `const_*_fmt` methods.
///
/// ### Constructors
///
/// Most std types can be wrapped in this type simply by doing `PWrapper(value)`.
///
/// To wrap arrays, there is the [`PWrapper::slice`](#method.slice) constructor
/// for convenience.
///
/// ### Excluded std types
///
/// Note that this type does not implement the formatting methods
/// for std types which wrap non-std types,
/// only for a selection of wrapped std types.
///
/// You can use the [`call_debug_fmt`] macro to format arrays/slices/Options of
/// any type that can be const debug formatted.
///
/// # Example
///
/// This example demonstrates how you can implement debug formatting for a type
/// using PWrapper to write std types.
///
#[cfg_attr(feature = "fmt", doc = "```rust")]
#[cfg_attr(not(feature = "fmt"), doc = "```ignore")]
///
/// use const_format::{Error, Formatter, PWrapper};
/// use const_format::{impl_fmt, formatc, try_};
///
/// use core::num::NonZeroU32;
///
/// pub struct Divide(pub u32, pub u32);
///
/// impl_fmt!{
///     impl Divide;
///     
///     pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
///         let Self(left, right) = *self;
///         let divided = self.0 / self.1;
///
///         let mut f = f.debug_struct("Divide");
///         try_!(PWrapper(self.0).const_debug_fmt(f.field("numerator")));
///         try_!(PWrapper(self.1).const_debug_fmt(f.field("denominator")));
///         try_!(PWrapper(divided).const_debug_fmt(f.field("divided")));
///         f.finish()
///     }
/// }
///
/// const TEXT: &str = formatc!("{:?}", Divide(34, 11));
/// const T_HEX: &str = formatc!("{:X?}", Divide(34, 11));
/// const T_BIN: &str = formatc!("{:b?}", Divide(34, 11));
///
/// assert_eq!(TEXT, "Divide { numerator: 34, denominator: 11, divided: 3 }");
/// assert_eq!(T_HEX, "Divide { numerator: 22, denominator: B, divided: 3 }");
/// assert_eq!(T_BIN, "Divide { numerator: 100010, denominator: 1011, divided: 11 }");
/// ```
///
/// [`call_debug_fmt`]: ./macro.call_debug_fmt.html
/// [`writec`]: ./macro.writec.html
///
#[cfg_attr(feature = "__docsrs", doc(cfg(feature = "fmt")))]
#[derive(Copy, Clone)]
pub struct PWrapper<T>(pub T);

impl<'a, T> PWrapper<&'a [T]> {
    /// For constructing from a reference to an array.
    ///
    /// With slices you can do `PWrapper(slice)` as well.
    #[inline(always)]
    pub const fn slice(x: &'a [T]) -> Self {
        Self { 0: x }
    }
}

macro_rules! compute_hex_count {
    ($bits:expr, $int:expr, $with_0x:expr) => {{
        let with_0x = ($with_0x as usize) << 1;
        let i = ($bits - $int.leading_zeros()) as usize;
        (if i == 0 {
            1
        } else {
            (i >> 2) + ((i & 3) != 0) as usize
        }) + with_0x
    }};
}
macro_rules! compute_binary_count {
    ($bits:expr, $int:expr, $with_0b:expr) => {{
        let with_0b = ($with_0b as usize) << 1;
        let i = ($bits - $int.leading_zeros()) as usize;
        (if i == 0 { 1 } else { i }) + with_0b
    }};
}

macro_rules! impl_number_of_digits {
    (num number_of_digits;delegate $n:ident $len:ident)=>{
        $n.number_of_digits()
    };
    (num number_of_digits;128 $n:ident $len:ident)=>{{
        if $n >= 1_0000_0000_0000_0000{$n /= 1_0000_0000_0000_0000; $len += 16;}
        impl_number_of_digits!(num number_of_digits;64 $n $len)
    }};
    (num number_of_digits;64 $n:ident $len:ident)=>{{
        if $n >= 1_0000_0000_0000{$n /= 1_0000_0000_0000; $len += 12;}
        impl_number_of_digits!(num number_of_digits;32 $n $len)
    }};
    (num number_of_digits;32 $n:ident $len:ident)=>{{
        if $n >= 1_0000_0000{$n /= 100_000_000; $len += 8;}
        impl_number_of_digits!(num number_of_digits;16 $n $len)
    }};
    (num number_of_digits;16 $n:ident $len:ident)=>{{
        if $n >= 1_0000{$n /= 1_0000; $len += 4;}
        impl_number_of_digits!(num number_of_digits;8 $n $len)
    }};
    (num number_of_digits;8 $n:ident $len:ident)=>{{
        if $n >= 100{$n /= 100; $len += 2;}
        if $n >= 10{            $len += 1;}
        $len
    }};
    (@shared $This:ty, $bits:tt)=>{
        impl PWrapper<$This> {
            /// Computes how long much space is necessary to write this integer as a literal.
            #[allow(unused_mut,unused_variables)]
            #[doc(hidden)]
            pub const fn compute_debug_len(self, fmt: FormattingFlags)-> usize {
                match fmt.num_fmt() {
                    NumberFormatting::Decimal=>
                        self.compute_display_len(fmt),
                    NumberFormatting::Hexadecimal=>
                        compute_hex_count!($bits, self.0, fmt.is_alternate()),
                    NumberFormatting::Binary=>
                        compute_binary_count!($bits, self.0, fmt.is_alternate()),
                }
            }

            /// Computes how long much space is necessary to
            /// write this integer as a hexadecimal literal.
            pub const fn hexadecimal_len(self, fmt: FormattingFlags)-> usize {
                compute_hex_count!($bits, self.0, fmt.is_alternate())
            }

            /// Computes how long much space is necessary to
            /// write this integer as a binary literal.
            pub const fn binary_len(self, fmt: FormattingFlags)-> usize {
                compute_binary_count!($bits, self.0, fmt.is_alternate())
            }
        }
    };
    (impl_either;
        signed,
        ($This:ty, $Unsigned:ty),
        $bits:tt $(,)?
    )=>{
        impl_number_of_digits!{@shared $This, $bits}

        impl PWrapper<$This> {
            /// Returns the absolute value of this integer, as the equivalent unsigned type.
            pub const fn unsigned_abs(self) -> $Unsigned {
                self.0.wrapping_abs() as $Unsigned
            }

            #[allow(unused_mut,unused_variables)]
            #[doc(hidden)]
            pub const fn compute_display_len(self, _: FormattingFlags)-> usize {
                let mut n = self.0.wrapping_abs() as $Unsigned;
                let mut len = 1 + (self.0 < 0) as usize;
                impl_number_of_digits!(num number_of_digits;$bits n len)
            }


        }
    };
    (impl_either;
        unsigned,
        ($This:ty, $Unsigned:ty),
        $bits:tt $(,)?
    )=>{
        impl_number_of_digits!{@shared $This, $bits}

        impl PWrapper<$This> {
            /// Returns the absolute value of this integer, as the equivalent unsigned type.
            pub const fn unsigned_abs(self) -> $Unsigned {
                self.0
            }

            #[doc(hidden)]
            pub const fn compute_display_len(self, _: FormattingFlags)-> usize {
                let mut n = self.0;
                let mut len = 1usize;
                impl_number_of_digits!(num number_of_digits;$bits n len)
            }
        }
    };
}

impl_number_of_digits! {impl_either; signed  , (i8, u8), 8}
impl_number_of_digits! {impl_either; signed  , (i16, u16), 16}
impl_number_of_digits! {impl_either; signed  , (i32, u32), 32}
impl_number_of_digits! {impl_either; signed  , (i64, u64), 64}
impl_number_of_digits! {impl_either; signed  , (i128, u128), 128}
impl_number_of_digits! {impl_either; unsigned, (u8, u8), 8}
impl_number_of_digits! {impl_either; unsigned, (u16, u16), 16}
impl_number_of_digits! {impl_either; unsigned, (u32, u32), 32}
impl_number_of_digits! {impl_either; unsigned, (u64, u64), 64}
impl_number_of_digits! {impl_either; unsigned, (u128, u128), 128}

#[cfg(target_pointer_width = "16")]
type UWord = u16;
#[cfg(target_pointer_width = "32")]
type UWord = u32;
#[cfg(target_pointer_width = "64")]
type UWord = u64;
#[cfg(target_pointer_width = "128")]
type UWord = u128;

#[cfg(target_pointer_width = "16")]
type IWord = i16;
#[cfg(target_pointer_width = "32")]
type IWord = i32;
#[cfg(target_pointer_width = "64")]
type IWord = i64;
#[cfg(target_pointer_width = "128")]
type IWord = i128;

macro_rules! impl_for_xsize {
    ($XSize:ident, $XWord:ident) => {
        impl PWrapper<$XSize> {
            /// Computes how long much space is necessary to write this integer as a literal.
            #[inline(always)]
            pub const fn compute_display_len(self, fmt: FormattingFlags) -> usize {
                PWrapper(self.0 as $XWord).compute_display_len(fmt)
            }

            /// Computes how long much space is necessary to write this integer as a literal.
            #[inline(always)]
            pub const fn compute_debug_len(self, fmt: FormattingFlags) -> usize {
                PWrapper(self.0 as $XWord).compute_debug_len(fmt)
            }

            /// Computes how long much space is necessary to
            /// write this integer as a hexadecimal literal.
            #[inline(always)]
            pub const fn hexadecimal_len(self, fmt: FormattingFlags) -> usize {
                PWrapper(self.0 as $XWord).hexadecimal_len(fmt)
            }

            /// Computes how long much space is necessary to
            /// write this integer as a binary literal.
            #[inline(always)]
            pub const fn binary_len(self, fmt: FormattingFlags) -> usize {
                PWrapper(self.0 as $XWord).binary_len(fmt)
            }
        }
    };
}

impl_for_xsize! {usize, UWord}
impl_for_xsize! {isize, IWord}

impl PWrapper<usize> {
    /// Returns the absolute value of this integer.
    pub const fn unsigned_abs(self) -> usize {
        self.0
    }
}

impl PWrapper<isize> {
    /// Returns the absolute value of this integer, as the equivalent unsigned type.
    pub const fn unsigned_abs(self) -> usize {
        self.0.wrapping_abs() as usize
    }
}

impl Integer {
    #[inline]
    const fn as_negative(self) -> i128 {
        (self.unsigned as i128).wrapping_neg()
    }
}

#[doc(hidden)]
impl PWrapper<Integer> {
    pub const fn to_start_array_binary(self, flags: FormattingFlags) -> StartAndArray<[u8; 130]> {
        let mut n = if self.0.is_negative {
            self.0.as_negative() as u128
        } else {
            self.0.unsigned
        };

        n &= *self.0.mask;

        let mut out = StartAndArray {
            start: 130,
            array: [0u8; 130],
        };

        loop {
            out.start -= 1;
            let digit = (n & 1) as u8;
            out.array[out.start] = b'0' + digit;
            n >>= 1;
            if n == 0 {
                break;
            }
        }

        if flags.is_alternate() {
            out.start -= 1;
            out.array[out.start] = b'b';
            out.start -= 1;
            out.array[out.start] = b'0';
        }

        out
    }

    pub const fn to_start_array_hexadecimal(
        self,
        flags: FormattingFlags,
    ) -> StartAndArray<[u8; 34]> {
        let mut n = if self.0.is_negative {
            self.0.as_negative() as u128
        } else {
            self.0.unsigned
        };

        n &= *self.0.mask;

        let mut out = StartAndArray {
            start: 34,
            array: [0u8; 34],
        };

        loop {
            out.start -= 1;
            let digit = (n & 0xF) as u8;
            out.array[out.start] = match digit {
                0..=9 => b'0' + digit,
                _ => digit + flags.hex_fmt() as u8,
            };
            n >>= 4;
            if n == 0 {
                break;
            }
        }

        if flags.is_alternate() {
            out.start -= 1;
            out.array[out.start] = b'x';
            out.start -= 1;
            out.array[out.start] = b'0';
        }

        out
    }

    pub const fn to_start_array_display(self) -> StartAndArray<[u8; 40]> {
        let mut out = StartAndArray {
            start: 40,
            array: [0u8; 40],
        };

        let mut n = self.0.unsigned;

        loop {
            out.start -= 1;
            let digit = (n % 10) as u8;
            out.array[out.start] = b'0' + digit;
            n /= 10;
            if n == 0 {
                break;
            }
        }

        if self.0.is_negative {
            out.start -= 1;
            out.array[out.start] = b'-';
        }

        out
    }

    #[inline(always)]
    pub const fn to_start_array_debug(self) -> StartAndArray<[u8; 40]> {
        self.to_start_array_display()
    }
}

impl PWrapper<&[u8]> {
    /// Computes how much space is necessary to write the wrapped `&[u8]` as a utf8 string,
    /// with debug formatting
    pub const fn compute_utf8_debug_len(self) -> usize {
        self.compute_utf8_debug_len_in_range(0..self.0.len())
    }

    /// Computes how much space is necessary to write `&self.0[range]` as a utf8 string,
    /// with debug formatting
    pub const fn compute_utf8_debug_len_in_range(self, mut range: Range<usize>) -> usize {
        let mut sum = range.end - range.start;
        while range.start < range.end {
            let c = self.0[range.start];
            if c < 128 {
                let shifted = 1 << c;
                if (FOR_ESCAPING.is_escaped & shifted) != 0 {
                    sum += if (FOR_ESCAPING.is_backslash_escaped & shifted) == 0 {
                        3 // `\x01` only add 3 characters
                    } else {
                        1 // Escaped with a backslash
                    };
                }
            }
            range.start += 1;
        }
        sum + 2 // The quote characters
    }
}

impl PWrapper<&str> {
    /// Computes how much space is necessary to write a `&str` with debug formatting
    #[inline(always)]
    #[doc(hidden)]
    pub const fn compute_debug_len(self, _: FormattingFlags) -> usize {
        PWrapper(self.0.as_bytes()).compute_utf8_debug_len()
    }

    /// Computes how much space is necessary to write a `&str` with display formatting
    #[inline(always)]
    #[doc(hidden)]
    pub const fn compute_display_len(self, _: FormattingFlags) -> usize {
        self.0.len()
    }
}

#[cfg(feature = "fmt")]
const _: () = {
    use crate::marker_traits::{FormatMarker, IsNotStdKind};

    impl<P> FormatMarker for PWrapper<P> {
        type Kind = IsNotStdKind;
        type This = Self;
    }
};

///////////////////////////////////////////////////////////////////////////

#[cfg(feature = "assertcp")]
macro_rules! impl_eq_for_primitives {
    (
        (l=$l:ident, r=$r:ident)

        $(
            impl[$($impl_:tt)*] $type:ty = $comparison:expr;
        )*

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
    )
}

#[cfg(feature = "assertcp")]
impl_eq_for_primitives! {
    (l = l, r = r)

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
