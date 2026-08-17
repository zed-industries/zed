use core::mem::MaybeUninit;
use core::num::NonZero;

use num_conv::prelude::*;

use crate::format_description::Period;
use crate::{
    Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcDateTime, UtcOffset, Weekday,
};

/// State used by date-providing types to cache computed values.
///
/// This is used to avoid redundant computations when multiple date components are almost certainly
/// going to be requested within the same formatting invocation.
#[derive(Debug)]
pub(crate) struct DateState {
    day: Option<NonZero<u8>>,
    month: Option<Month>,
    iso_week: Option<NonZero<u8>>,
    iso_year_is_initialized: bool,
    iso_year: MaybeUninit<i32>,
}

impl Default for DateState {
    fn default() -> Self {
        Self {
            day: Default::default(),
            month: Default::default(),
            iso_week: Default::default(),
            iso_year_is_initialized: Default::default(),
            iso_year: MaybeUninit::uninit(),
        }
    }
}

macro_rules! unimplemented_methods {
    ($(
        $(#[$meta:meta])*
        ($component:literal) $name:ident => $ret:ty;
    )*) => {
        $(
            $(#[$meta])*
            #[track_caller]
            #[expect(unused_variables, reason = "better for auto-generation of method stubs")]
            fn $name(&self, state: &mut Self::State) -> $ret {
                unimplemented!(concat!("type does not supply ", $component, " components"))
            }
        )*
    };
}

macro_rules! delegate_providers {
    (
        $target:ident {
            $($method:ident -> $return:ty)*
        }
    ) => {$(
        #[inline]
        fn $method(&self, state: &mut Self::State) -> $return {
            ComponentProvider::$method(&self.$target(), state)
        }
    )*};
    (
        $target:ident ($state:expr) {
            $($method:ident -> $return:ty)*
        }
    ) => {$(
        #[inline]
        fn $method(&self, _: &mut Self::State) -> $return {
            ComponentProvider::$method(&self.$target(), $state)
        }
    )*};
}

/// A type with the ability to provide date, time, offset, and/or timestamp components on demand.
///
/// Note that while all methods have a default body, implementations are expected to override the
/// body for all components that they provide. The default implementation exists solely for
/// convenience, avoiding the need to specify unprovided components.
pub(crate) trait ComponentProvider {
    /// The state type used by the provider, allowing for caching of computed values.
    type State: Default;

    /// Whether the type can provide date components, indicating that date-related methods can be
    /// called.
    const SUPPLIES_DATE: bool = false;
    /// Whether the type can provide time components, indicating that time-related methods can be
    /// called.
    const SUPPLIES_TIME: bool = false;
    /// Whether the type can provide offset components, indicating that offset-related methods can
    /// be called.
    const SUPPLIES_OFFSET: bool = false;
    /// Whether the type can provide timestamp components, indicating that timestamp-related methods
    /// can be called.
    const SUPPLIES_TIMESTAMP: bool = false;

    unimplemented_methods! {
        /// Obtain the day of the month.
        ("date") day => u8;
        /// Obtain the month of the year.
        ("date") month => Month;
        /// Obtain the ordinal day of the year.
        ("date") ordinal => u16;
        /// Obtain the day of the week.
        ("date") weekday => Weekday;
        /// Obtain the ISO week number.
        ("date") iso_week_number => u8;
        /// Obtain the Monday-based week number.
        ("date") monday_based_week => u8;
        /// Obtain the Sunday-based week number.
        ("date") sunday_based_week => u8;
        /// Obtain the calendar year.
        ("date") calendar_year => i32;
        /// Obtain the ISO week-based year.
        ("date") iso_year => i32;
        /// Obtain the hour within the day.
        ("time") hour => u8;
        /// Obtain the minute within the hour.
        ("time") minute => u8;
        /// Obtain the period of the day (AM/PM).
        ("time") period => Period;
        /// Obtain the second within the minute.
        ("time") second => u8;
        /// Obtain the nanosecond within the second.
        ("time") nanosecond => u32;
        /// Obtain whether the offset is negative.
        ("offset") offset_is_negative => bool;
        /// Obtain whether the offset is UTC.
        ("offset") offset_is_utc => bool;
        /// Obtain the hour component of the UTC offset.
        ("offset") offset_hour => i8;
        /// Obtain the minute component of the UTC offset.
        ("offset") offset_minute => i8;
        /// Obtain the second component of the UTC offset.
        ("offset") offset_second => i8;
        /// Obtain the Unix timestamp in seconds.
        ("timestamp") unix_timestamp_seconds => i64;
        /// Obtain the Unix timestamp in milliseconds.
        ("timestamp") unix_timestamp_milliseconds => i64;
        /// Obtain the Unix timestamp in microseconds.
        ("timestamp") unix_timestamp_microseconds => i128;
        /// Obtain the Unix timestamp in nanoseconds.
        ("timestamp") unix_timestamp_nanoseconds => i128;
    }
}

impl ComponentProvider for Time {
    type State = ();

    const SUPPLIES_TIME: bool = true;

    #[inline]
    fn hour(&self, _: &mut Self::State) -> u8 {
        (*self).hour()
    }

    #[inline]
    fn minute(&self, _: &mut Self::State) -> u8 {
        (*self).minute()
    }

    #[inline]
    fn period(&self, _: &mut Self::State) -> Period {
        if (*self).hour() < 12 {
            Period::Am
        } else {
            Period::Pm
        }
    }

    #[inline]
    fn second(&self, _: &mut Self::State) -> u8 {
        (*self).second()
    }

    #[inline]
    fn nanosecond(&self, _: &mut Self::State) -> u32 {
        (*self).nanosecond()
    }
}

impl ComponentProvider for Date {
    type State = DateState;

    const SUPPLIES_DATE: bool = true;

    #[inline]
    fn day(&self, state: &mut Self::State) -> u8 {
        if let Some(day) = state.day {
            return day.get();
        }

        let (_, month, day) = (*self).to_calendar_date();
        state.month = Some(month);
        // Safety: `day` is guaranteed to be non-zero.
        state.day = Some(unsafe { NonZero::new_unchecked(day) });
        day
    }

    #[inline]
    fn month(&self, state: &mut Self::State) -> Month {
        *state.month.get_or_insert_with(|| (*self).month())
    }

    #[inline]
    fn ordinal(&self, _: &mut Self::State) -> u16 {
        (*self).ordinal()
    }

    #[inline]
    fn weekday(&self, _: &mut Self::State) -> Weekday {
        (*self).weekday()
    }

    #[inline]
    fn iso_week_number(&self, state: &mut Self::State) -> u8 {
        if let Some(week) = state.iso_week {
            return week.get();
        }

        let (iso_year, iso_week) = (*self).iso_year_week();
        state.iso_year = MaybeUninit::new(iso_year);
        state.iso_year_is_initialized = true;
        // Safety: `iso_week` is guaranteed to be non-zero.
        state.iso_week = Some(unsafe { NonZero::new_unchecked(iso_week) });
        iso_week
    }

    #[inline]
    fn monday_based_week(&self, _: &mut Self::State) -> u8 {
        (*self).monday_based_week()
    }

    #[inline]
    fn sunday_based_week(&self, _: &mut Self::State) -> u8 {
        (*self).sunday_based_week()
    }

    #[inline]
    fn calendar_year(&self, _: &mut Self::State) -> i32 {
        (*self).year()
    }

    #[inline]
    fn iso_year(&self, state: &mut Self::State) -> i32 {
        if state.iso_year_is_initialized {
            // Safety: `iso_year` was declared to be initialized.
            return unsafe { state.iso_year.assume_init() };
        }

        let (iso_year, iso_week) = (*self).iso_year_week();
        state.iso_year = MaybeUninit::new(iso_year);
        state.iso_year_is_initialized = true;
        // Safety: `iso_week` is guaranteed to be non-zero.
        state.iso_week = Some(unsafe { NonZero::new_unchecked(iso_week) });
        iso_year
    }
}

impl ComponentProvider for PrimitiveDateTime {
    type State = DateState;

    const SUPPLIES_DATE: bool = true;
    const SUPPLIES_TIME: bool = true;

    delegate_providers!(date {
        day -> u8
        month -> Month
        ordinal -> u16
        weekday -> Weekday
        iso_week_number -> u8
        monday_based_week -> u8
        sunday_based_week -> u8
        calendar_year -> i32
        iso_year -> i32
    });
    delegate_providers!(time (&mut ()) {
        hour -> u8
        minute -> u8
        period -> Period
        second -> u8
        nanosecond -> u32
    });
}

impl ComponentProvider for UtcOffset {
    type State = ();

    const SUPPLIES_OFFSET: bool = true;

    #[inline]
    fn offset_is_negative(&self, _: &mut Self::State) -> bool {
        (*self).is_negative()
    }

    #[inline]
    fn offset_is_utc(&self, _state: &mut Self::State) -> bool {
        (*self).is_utc()
    }

    #[inline]
    fn offset_hour(&self, _: &mut Self::State) -> i8 {
        (*self).whole_hours()
    }

    #[inline]
    fn offset_minute(&self, _: &mut Self::State) -> i8 {
        (*self).minutes_past_hour()
    }

    #[inline]
    fn offset_second(&self, _: &mut Self::State) -> i8 {
        (*self).seconds_past_minute()
    }
}

impl ComponentProvider for UtcDateTime {
    type State = DateState;

    const SUPPLIES_DATE: bool = true;
    const SUPPLIES_TIME: bool = true;
    const SUPPLIES_OFFSET: bool = true;
    const SUPPLIES_TIMESTAMP: bool = true;

    delegate_providers!(date {
        day -> u8
        month -> Month
        ordinal -> u16
        weekday -> Weekday
        iso_week_number -> u8
        monday_based_week -> u8
        sunday_based_week -> u8
        calendar_year -> i32
        iso_year -> i32
    });
    delegate_providers!(time (&mut ()) {
        hour -> u8
        minute -> u8
        period -> Period
        second -> u8
        nanosecond -> u32
    });

    #[inline]
    fn offset_is_negative(&self, _: &mut Self::State) -> bool {
        false
    }

    #[inline]
    fn offset_is_utc(&self, _state: &mut Self::State) -> bool {
        true
    }

    #[inline]
    fn offset_hour(&self, _: &mut Self::State) -> i8 {
        0
    }

    #[inline]
    fn offset_minute(&self, _: &mut Self::State) -> i8 {
        0
    }

    #[inline]
    fn offset_second(&self, _: &mut Self::State) -> i8 {
        0
    }

    #[inline]
    fn unix_timestamp_seconds(&self, _: &mut Self::State) -> i64 {
        (*self).unix_timestamp()
    }

    #[inline]
    fn unix_timestamp_milliseconds(&self, state: &mut Self::State) -> i64 {
        (ComponentProvider::unix_timestamp_nanoseconds(self, state) / 1_000_000).truncate()
    }

    #[inline]
    fn unix_timestamp_microseconds(&self, state: &mut Self::State) -> i128 {
        ComponentProvider::unix_timestamp_nanoseconds(self, state) / 1_000
    }

    #[inline]
    fn unix_timestamp_nanoseconds(&self, _: &mut Self::State) -> i128 {
        (*self).unix_timestamp_nanos()
    }
}

impl ComponentProvider for OffsetDateTime {
    type State = DateState;

    const SUPPLIES_DATE: bool = true;
    const SUPPLIES_TIME: bool = true;
    const SUPPLIES_OFFSET: bool = true;
    const SUPPLIES_TIMESTAMP: bool = true;

    delegate_providers!(date {
        day -> u8
        month -> Month
        ordinal -> u16
        weekday -> Weekday
        iso_week_number -> u8
        monday_based_week -> u8
        sunday_based_week -> u8
        calendar_year -> i32
        iso_year -> i32
    });
    delegate_providers!(time (&mut ()) {
        hour -> u8
        minute -> u8
        period -> Period
        second -> u8
        nanosecond -> u32
    });
    delegate_providers!(offset (&mut ()) {
        offset_is_negative -> bool
        offset_is_utc -> bool
        offset_hour -> i8
        offset_minute -> i8
        offset_second -> i8
    });

    #[inline]
    fn unix_timestamp_seconds(&self, _: &mut Self::State) -> i64 {
        (*self).unix_timestamp()
    }

    #[inline]
    fn unix_timestamp_milliseconds(&self, _: &mut Self::State) -> i64 {
        ((*self).unix_timestamp_nanos() / 1_000_000) as i64
    }

    #[inline]
    fn unix_timestamp_microseconds(&self, _: &mut Self::State) -> i128 {
        (*self).unix_timestamp_nanos() / 1_000
    }

    #[inline]
    fn unix_timestamp_nanoseconds(&self, _: &mut Self::State) -> i128 {
        (*self).unix_timestamp_nanos()
    }
}
