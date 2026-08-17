#![allow(clippy::wrong_self_convention)]

use crate::{
    char_encoding::FmtChar,
    formatting::{Formatting, FormattingFlags},
    wrapper_types::PWrapper,
};

#[doc(hidden)]
/// The uniform representation for every argument of the concatcp macro.
pub struct PArgument {
    pub elem: PVariant,
    pub fmt_len: usize,
    pub fmt: Formatting,
    pub fmt_flags: FormattingFlags,
}

impl PArgument {
    /// Calculates the length of the string after adding up all the PArguments
    pub const fn calc_len(mut args: &[PArgument]) -> usize {
        let mut sum = 0;

        while let [curr, rem @ ..] = args {
            args = rem;
            sum += curr.fmt_len;
        }

        sum
    }
}

#[doc(hidden)]
pub enum PVariant {
    Str(&'static str),
    Int(Integer),
    Char(FmtChar),
}

#[derive(Debug, Copy, Clone)]
pub struct Integer {
    pub is_negative: bool,
    pub unsigned: u128,
    pub mask: &'static u128, // A mask which disables the bits that weren't in the original number
}

#[doc(hidden)]
pub struct PConvWrapper<T>(pub T);

macro_rules! pconvwrapper_impls {
    ( $( ($Signed:ty, $Unsigned:ty) )* ) => (
        pconvwrapper_impls!{
            @inner to_pargument_display, compute_display_len, Formatting::Display;
            $(($Signed, $Unsigned))*
        }
        pconvwrapper_impls!{
            @inner to_pargument_debug, compute_debug_len, Formatting::Debug;
            $(($Signed, $Unsigned))*
        }

        $(
            #[doc(hidden)]
            impl PConvWrapper<$Signed>{
                pub const fn to_integer(self)->Integer{
                    Integer{
                        is_negative: self.0 < 0,
                        unsigned: PWrapper(self.0).unsigned_abs() as u128,
                        mask: &(((!(0 as $Signed)) as $Unsigned) as u128),
                    }
                }
            }

            #[doc(hidden)]
            impl PConvWrapper<$Unsigned>{
                pub const fn to_integer(self)->Integer{
                    Integer{
                        is_negative: false,
                        unsigned: self.0 as u128,
                        mask: &((!(0 as $Unsigned)) as u128),
                    }
                }
            }
        )*
    );
    (@inner
        $method:ident,
        $called:ident,
        $formatting:expr;
        $( ($Signed:ty, $Unsigned:ty) )*
    ) => (
        $(
            #[doc(hidden)]
            impl PConvWrapper<$Signed> {
                pub const fn $method(self, fmt_flags: FormattingFlags)->PArgument{
                    PArgument {
                        fmt_len: $crate::pmr::PWrapper(self.0).$called(fmt_flags),
                        fmt: $formatting,
                        fmt_flags,
                        elem: PVariant::Int(self.to_integer()),
                    }
                }
            }

            #[doc(hidden)]
            impl PConvWrapper<$Unsigned> {
                pub const fn $method(self, fmt_flags: FormattingFlags)->PArgument{
                    PArgument {
                        fmt_len: $crate::pmr::PWrapper(self.0).$called(fmt_flags),
                        fmt: $formatting,
                        fmt_flags,
                        elem: PVariant::Int(self.to_integer()),
                    }
                }
            }
        )*
    );
}

pconvwrapper_impls! {
    (i8, u8)
    (i16, u16)
    (i32, u32)
    (i64, u64)
    (i128, u128)
    (isize, usize)
}

#[doc(hidden)]
impl PConvWrapper<PArgument> {
    #[inline]
    pub const fn to_pargument_display(self, _: FormattingFlags) -> PArgument {
        self.0
    }
    #[inline]
    pub const fn to_pargument_debug(self, _: FormattingFlags) -> PArgument {
        self.0
    }
}

#[doc(hidden)]
impl PConvWrapper<bool> {
    #[inline]
    pub const fn to_pargument_display(self, _: FormattingFlags) -> PArgument {
        PConvWrapper(if self.0 { "true" } else { "false" })
            .to_pargument_display(FormattingFlags::DEFAULT)
    }
    #[inline]
    pub const fn to_pargument_debug(self, fmt_flags: FormattingFlags) -> PArgument {
        self.to_pargument_display(fmt_flags)
    }
}

#[doc(hidden)]
impl PConvWrapper<char> {
    #[inline]
    pub const fn to_pargument_display(self, fmt_flags: FormattingFlags) -> PArgument {
        let elem = crate::char_encoding::char_to_display(self.0);
        PArgument {
            fmt_len: elem.len(),
            fmt_flags,
            fmt: Formatting::Display,
            elem: PVariant::Char(elem),
        }
    }
    #[inline]
    pub const fn to_pargument_debug(self, fmt_flags: FormattingFlags) -> PArgument {
        let elem = crate::char_encoding::char_to_debug(self.0);
        PArgument {
            fmt_len: elem.len(),
            fmt_flags,
            fmt: Formatting::Debug,
            elem: PVariant::Char(elem),
        }
    }
}

#[doc(hidden)]
impl PConvWrapper<&'static str> {
    #[inline]
    pub const fn to_pargument_display(self, fmt_flags: FormattingFlags) -> PArgument {
        PArgument {
            fmt_len: PWrapper(self.0).compute_display_len(fmt_flags),
            fmt_flags,
            fmt: Formatting::Display,
            elem: PVariant::Str(self.0),
        }
    }
    #[inline]
    pub const fn to_pargument_debug(self, fmt_flags: FormattingFlags) -> PArgument {
        PArgument {
            fmt_len: PWrapper(self.0).compute_debug_len(fmt_flags),
            fmt_flags,
            fmt: Formatting::Debug,
            elem: PVariant::Str(self.0),
        }
    }
}
