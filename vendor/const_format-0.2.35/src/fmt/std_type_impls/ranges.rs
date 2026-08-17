use crate::{
    fmt::{Error, Formatter},
    marker_traits::{FormatMarker, IsAFormatMarker, IsStdKind},
    wrapper_types::PWrapper,
};

use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

////////////////////////////////////////////////////////////////////////////////

impl FormatMarker for Range<usize> {
    type Kind = IsStdKind;
    type This = Self;
}

impl<T> IsAFormatMarker<IsStdKind, Range<usize>, T> {
    #[inline(always)]
    pub const fn coerce(self, range: &Range<usize>) -> PWrapper<Range<usize>> {
        PWrapper(Range {
            start: range.start,
            end: range.end,
        })
    }
}

impl PWrapper<Range<usize>> {
    const RANGE: &'static str = "..";

    #[inline(always)]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        try_!(PWrapper(self.0.start).const_debug_fmt(f));
        try_!(PWrapper(Self::RANGE).const_display_fmt(f));
        try_!(PWrapper(self.0.end).const_debug_fmt(f));
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////

impl FormatMarker for RangeFrom<usize> {
    type Kind = IsStdKind;
    type This = Self;
}

impl<T> IsAFormatMarker<IsStdKind, RangeFrom<usize>, T> {
    #[inline(always)]
    pub const fn coerce(self, range: &RangeFrom<usize>) -> PWrapper<RangeFrom<usize>> {
        PWrapper(RangeFrom { start: range.start })
    }
}

impl PWrapper<RangeFrom<usize>> {
    const RANGE: &'static str = "..";

    #[inline(always)]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        try_!(PWrapper(self.0.start).const_debug_fmt(f));
        try_!(PWrapper(Self::RANGE).const_display_fmt(f));
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////

impl FormatMarker for RangeTo<usize> {
    type Kind = IsStdKind;
    type This = Self;
}

impl<T> IsAFormatMarker<IsStdKind, RangeTo<usize>, T> {
    #[inline(always)]
    pub const fn coerce(self, range: &RangeTo<usize>) -> PWrapper<RangeTo<usize>> {
        PWrapper(RangeTo { end: range.end })
    }
}

impl PWrapper<RangeTo<usize>> {
    const RANGE: &'static str = "..";

    #[inline(always)]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        try_!(PWrapper(Self::RANGE).const_display_fmt(f));
        try_!(PWrapper(self.0.end).const_debug_fmt(f));
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////

impl FormatMarker for RangeToInclusive<usize> {
    type Kind = IsStdKind;
    type This = Self;
}

impl<T> IsAFormatMarker<IsStdKind, RangeToInclusive<usize>, T> {
    #[inline(always)]
    pub const fn coerce(
        self,
        range: &RangeToInclusive<usize>,
    ) -> PWrapper<RangeToInclusive<usize>> {
        PWrapper(RangeToInclusive { end: range.end })
    }
}

impl PWrapper<RangeToInclusive<usize>> {
    const RANGE: &'static str = "..=";

    #[inline(always)]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        try_!(PWrapper(Self::RANGE).const_display_fmt(f));
        try_!(PWrapper(self.0.end).const_debug_fmt(f));
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////

impl FormatMarker for RangeInclusive<usize> {
    type Kind = IsStdKind;
    type This = Self;
}

impl<T> IsAFormatMarker<IsStdKind, RangeInclusive<usize>, T> {
    #[inline(always)]
    pub const fn coerce(self, range: &RangeInclusive<usize>) -> PWrapper<RangeInclusive<usize>> {
        PWrapper(RangeInclusive::new(*range.start(), *range.end()))
    }
}

impl PWrapper<RangeInclusive<usize>> {
    const RANGE: &'static str = "..=";

    #[inline(always)]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        try_!(PWrapper(*self.0.start()).const_debug_fmt(f));
        try_!(PWrapper(Self::RANGE).const_display_fmt(f));
        try_!(PWrapper(*self.0.end()).const_debug_fmt(f));
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////

impl FormatMarker for RangeFull {
    type Kind = IsStdKind;
    type This = Self;
}

impl<T> IsAFormatMarker<IsStdKind, RangeFull, T> {
    #[inline(always)]
    pub const fn coerce(self, _: &RangeFull) -> PWrapper<RangeFull> {
        PWrapper(..)
    }
}

impl PWrapper<RangeFull> {
    const RANGE: &'static str = "..";

    #[inline(always)]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        PWrapper(Self::RANGE).const_display_fmt(f)
    }
}
