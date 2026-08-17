/// The datetime coordinates
use chrono::{Date, DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Timelike};
use std::ops::{Add, Range, Sub};

use crate::coord::ranged1d::{
    AsRangedCoord, DefaultFormatting, DiscreteRanged, KeyPointHint, NoDefaultFormatting, Ranged,
    ReversibleRanged, ValueFormatter,
};

/// The trait that describe some time value. This is the uniformed abstraction that works
/// for both Date, DateTime and Duration, etc.
pub trait TimeValue: Eq + Sized {
    type DateType: Datelike + PartialOrd;

    /// Returns the date that is no later than the time
    fn date_floor(&self) -> Self::DateType;
    /// Returns the date that is no earlier than the time
    fn date_ceil(&self) -> Self::DateType;
    /// Returns the maximum value that is earlier than the given date
    fn earliest_after_date(date: Self::DateType) -> Self;
    /// Returns the duration between two time value
    fn subtract(&self, other: &Self) -> Duration;
    /// Add duration to time value
    fn add(&self, duration: &Duration) -> Self;
    /// Instantiate a date type for current time value;
    fn ymd(&self, year: i32, month: u32, date: u32) -> Self::DateType;
    /// Cast current date type into this type
    fn from_date(date: Self::DateType) -> Self;

    /// Map the coord spec
    fn map_coord(value: &Self, begin: &Self, end: &Self, limit: (i32, i32)) -> i32 {
        let total_span = end.subtract(begin);
        let value_span = value.subtract(begin);

        // First, lets try the nanoseconds precision
        if let Some(total_ns) = total_span.num_nanoseconds() {
            if let Some(value_ns) = value_span.num_nanoseconds() {
                return (f64::from(limit.1 - limit.0) * value_ns as f64 / total_ns as f64) as i32
                    + limit.0;
            }
        }

        // Yes, converting them to floating point may lose precision, but this is Ok.
        // If it overflows, it means we have a time span nearly 300 years, we are safe to ignore the
        // portion less than 1 day.
        let total_days = total_span.num_days() as f64;
        let value_days = value_span.num_days() as f64;

        (f64::from(limit.1 - limit.0) * value_days / total_days) as i32 + limit.0
    }

    /// Map pixel to coord spec
    fn unmap_coord(point: i32, begin: &Self, end: &Self, limit: (i32, i32)) -> Self {
        let total_span = end.subtract(begin);
        let offset = (point - limit.0) as i64;

        // Check if nanoseconds fit in i64
        if let Some(total_ns) = total_span.num_nanoseconds() {
            let pixel_span = (limit.1 - limit.0) as i64;
            let factor = total_ns / pixel_span;
            let remainder = total_ns % pixel_span;
            if factor == 0
                || i64::MAX / factor > offset.abs()
                || (remainder == 0 && i64::MAX / factor >= offset.abs())
            {
                let nano_seconds = offset * factor + (remainder * offset) / pixel_span;
                return begin.add(&Duration::nanoseconds(nano_seconds));
            }
        }

        // Otherwise, use days
        let total_days = total_span.num_days() as f64;
        let days = (((offset as f64) * total_days) / ((limit.1 - limit.0) as f64)) as i64;
        begin.add(&Duration::days(days))
    }
}

impl TimeValue for NaiveDate {
    type DateType = NaiveDate;
    fn date_floor(&self) -> NaiveDate {
        *self
    }
    fn date_ceil(&self) -> NaiveDate {
        *self
    }
    fn earliest_after_date(date: NaiveDate) -> Self {
        date
    }
    fn subtract(&self, other: &NaiveDate) -> Duration {
        *self - *other
    }
    fn add(&self, other: &Duration) -> NaiveDate {
        *self + *other
    }

    fn ymd(&self, year: i32, month: u32, date: u32) -> Self::DateType {
        NaiveDate::from_ymd(year, month, date)
    }

    fn from_date(date: Self::DateType) -> Self {
        date
    }
}

impl<Z: TimeZone> TimeValue for Date<Z> {
    type DateType = Date<Z>;
    fn date_floor(&self) -> Date<Z> {
        self.clone()
    }
    fn date_ceil(&self) -> Date<Z> {
        self.clone()
    }
    fn earliest_after_date(date: Date<Z>) -> Self {
        date
    }
    fn subtract(&self, other: &Date<Z>) -> Duration {
        self.clone() - other.clone()
    }
    fn add(&self, other: &Duration) -> Date<Z> {
        self.clone() + *other
    }

    fn ymd(&self, year: i32, month: u32, date: u32) -> Self::DateType {
        self.timezone().ymd(year, month, date)
    }

    fn from_date(date: Self::DateType) -> Self {
        date
    }
}

impl<Z: TimeZone> TimeValue for DateTime<Z> {
    type DateType = Date<Z>;
    fn date_floor(&self) -> Date<Z> {
        self.date()
    }
    fn date_ceil(&self) -> Date<Z> {
        if self.time().num_seconds_from_midnight() > 0 {
            self.date() + Duration::days(1)
        } else {
            self.date()
        }
    }
    fn earliest_after_date(date: Date<Z>) -> DateTime<Z> {
        date.and_hms(0, 0, 0)
    }

    fn subtract(&self, other: &DateTime<Z>) -> Duration {
        self.clone() - other.clone()
    }
    fn add(&self, other: &Duration) -> DateTime<Z> {
        self.clone() + *other
    }

    fn ymd(&self, year: i32, month: u32, date: u32) -> Self::DateType {
        self.timezone().ymd(year, month, date)
    }

    fn from_date(date: Self::DateType) -> Self {
        date.and_hms(0, 0, 0)
    }
}

impl TimeValue for NaiveDateTime {
    type DateType = NaiveDate;

    fn date_floor(&self) -> NaiveDate {
        self.date()
    }

    fn date_ceil(&self) -> NaiveDate {
        if self.time().num_seconds_from_midnight() > 0 {
            self.date() + Duration::days(1)
        } else {
            self.date()
        }
    }

    fn earliest_after_date(date: NaiveDate) -> NaiveDateTime {
        date.and_hms(0, 0, 0)
    }

    fn subtract(&self, other: &NaiveDateTime) -> Duration {
        *self - *other
    }
    fn add(&self, other: &Duration) -> NaiveDateTime {
        *self + *other
    }

    fn ymd(&self, year: i32, month: u32, date: u32) -> Self::DateType {
        NaiveDate::from_ymd(year, month, date)
    }

    fn from_date(date: Self::DateType) -> Self {
        date.and_hms(0, 0, 0)
    }
}

/// The ranged coordinate for date
#[derive(Clone)]
pub struct RangedDate<D: Datelike>(D, D);

impl<D: Datelike> From<Range<D>> for RangedDate<D> {
    fn from(range: Range<D>) -> Self {
        Self(range.start, range.end)
    }
}

impl<D> Ranged for RangedDate<D>
where
    D: Datelike + TimeValue + Sub<D, Output = Duration> + Add<Duration, Output = D> + Clone,
{
    type FormatOption = DefaultFormatting;
    type ValueType = D;

    fn range(&self) -> Range<D> {
        self.0.clone()..self.1.clone()
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        TimeValue::map_coord(value, &self.0, &self.1, limit)
    }

    fn key_points<HintType: KeyPointHint>(&self, hint: HintType) -> Vec<Self::ValueType> {
        let max_points = hint.max_num_points();
        let mut ret = vec![];

        let total_days = (self.1.clone() - self.0.clone()).num_days();
        let total_weeks = (self.1.clone() - self.0.clone()).num_weeks();

        if total_days > 0 && total_days as usize <= max_points {
            for day_idx in 0..=total_days {
                ret.push(self.0.clone() + Duration::days(day_idx));
            }
            return ret;
        }

        if total_weeks > 0 && total_weeks as usize <= max_points {
            for day_idx in 0..=total_weeks {
                ret.push(self.0.clone() + Duration::weeks(day_idx));
            }
            return ret;
        }

        // When all data is in the same week, just plot properly.
        if total_weeks == 0 {
            ret.push(self.0.clone());
            return ret;
        }

        let week_per_point = ((total_weeks as f64) / (max_points as f64)).ceil() as usize;

        for idx in 0..=(total_weeks as usize / week_per_point) {
            ret.push(self.0.clone() + Duration::weeks((idx * week_per_point) as i64));
        }

        ret
    }
}

impl<D> DiscreteRanged for RangedDate<D>
where
    D: Datelike + TimeValue + Sub<D, Output = Duration> + Add<Duration, Output = D> + Clone,
{
    fn size(&self) -> usize {
        ((self.1.clone() - self.0.clone()).num_days().max(-1) + 1) as usize
    }

    fn index_of(&self, value: &D) -> Option<usize> {
        let ret = (value.clone() - self.0.clone()).num_days();
        if ret < 0 {
            return None;
        }
        Some(ret as usize)
    }

    fn from_index(&self, index: usize) -> Option<D> {
        Some(self.0.clone() + Duration::days(index as i64))
    }
}

impl<Z: TimeZone> AsRangedCoord for Range<Date<Z>> {
    type CoordDescType = RangedDate<Date<Z>>;
    type Value = Date<Z>;
}

impl AsRangedCoord for Range<NaiveDate> {
    type CoordDescType = RangedDate<NaiveDate>;
    type Value = NaiveDate;
}

/// Indicates the coord has a monthly resolution
///
/// Note: since month doesn't have a constant duration.
/// We can't use a simple granularity to describe it. Thus we have
/// this axis decorator to make it yield monthly key-points.
#[derive(Clone)]
pub struct Monthly<T: TimeValue>(Range<T>);

impl<T: TimeValue + Datelike + Clone> ValueFormatter<T> for Monthly<T> {
    fn format(value: &T) -> String {
        format!("{}-{}", value.year(), value.month())
    }
}

impl<T: TimeValue + Clone> Monthly<T> {
    fn bold_key_points<H: KeyPointHint>(&self, hint: &H) -> Vec<T> {
        let max_points = hint.max_num_points();
        let start_date = self.0.start.date_ceil();
        let end_date = self.0.end.date_floor();

        let mut start_year = start_date.year();
        let mut start_month = start_date.month();
        let start_day = start_date.day();

        let end_year = end_date.year();
        let end_month = end_date.month();

        if start_day != 1 {
            start_month += 1;
            if start_month == 13 {
                start_month = 1;
                start_year += 1;
            }
        }

        let total_month = (end_year - start_year) * 12 + end_month as i32 - start_month as i32;

        fn generate_key_points<T: TimeValue>(
            mut start_year: i32,
            mut start_month: i32,
            end_year: i32,
            end_month: i32,
            step: u32,
            builder: &T,
        ) -> Vec<T> {
            let mut ret = vec![];
            while end_year > start_year || (end_year == start_year && end_month >= start_month) {
                ret.push(T::earliest_after_date(builder.ymd(
                    start_year,
                    start_month as u32,
                    1,
                )));
                start_month += step as i32;

                if start_month >= 13 {
                    start_year += start_month / 12;
                    start_month %= 12;
                }
            }

            ret
        }

        if total_month as usize <= max_points {
            // Monthly
            return generate_key_points(
                start_year,
                start_month as i32,
                end_year,
                end_month as i32,
                1,
                &self.0.start,
            );
        } else if total_month as usize <= max_points * 3 {
            // Quarterly
            return generate_key_points(
                start_year,
                start_month as i32,
                end_year,
                end_month as i32,
                3,
                &self.0.start,
            );
        } else if total_month as usize <= max_points * 6 {
            // Biyearly
            return generate_key_points(
                start_year,
                start_month as i32,
                end_year,
                end_month as i32,
                6,
                &self.0.start,
            );
        }

        // Otherwise we could generate the yearly keypoints
        generate_yearly_keypoints(
            max_points,
            start_year,
            start_month,
            end_year,
            end_month,
            &self.0.start,
        )
    }
}

impl<T: TimeValue + Clone> Ranged for Monthly<T>
where
    Range<T>: AsRangedCoord<Value = T>,
{
    type FormatOption = NoDefaultFormatting;
    type ValueType = T;

    fn range(&self) -> Range<T> {
        self.0.start.clone()..self.0.end.clone()
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        T::map_coord(value, &self.0.start, &self.0.end, limit)
    }

    fn key_points<HintType: KeyPointHint>(&self, hint: HintType) -> Vec<Self::ValueType> {
        if hint.weight().allow_light_points() && self.size() <= hint.bold_points() * 2 {
            let coord: <Range<T> as AsRangedCoord>::CoordDescType = self.0.clone().into();
            let normal = coord.key_points(hint.max_num_points());
            return normal;
        }
        self.bold_key_points(&hint)
    }
}

impl<T: TimeValue + Clone> DiscreteRanged for Monthly<T>
where
    Range<T>: AsRangedCoord<Value = T>,
{
    fn size(&self) -> usize {
        let (start_year, start_month) = {
            let ceil = self.0.start.date_ceil();
            (ceil.year(), ceil.month())
        };
        let (end_year, end_month) = {
            let floor = self.0.end.date_floor();
            (floor.year(), floor.month())
        };
        ((end_year - start_year).max(0) * 12
            + (1 - start_month as i32)
            + (end_month as i32 - 1)
            + 1)
        .max(0) as usize
    }

    fn index_of(&self, value: &T) -> Option<usize> {
        let this_year = value.date_floor().year();
        let this_month = value.date_floor().month();

        let start_year = self.0.start.date_ceil().year();
        let start_month = self.0.start.date_ceil().month();

        let ret = (this_year - start_year).max(0) * 12
            + (1 - start_month as i32)
            + (this_month as i32 - 1);
        if ret >= 0 {
            return Some(ret as usize);
        }
        None
    }

    fn from_index(&self, index: usize) -> Option<T> {
        if index == 0 {
            return Some(T::earliest_after_date(self.0.start.date_ceil()));
        }
        let index_from_start_year = index + (self.0.start.date_ceil().month() - 1) as usize;
        let year = self.0.start.date_ceil().year() + index_from_start_year as i32 / 12;
        let month = index_from_start_year % 12;
        Some(T::earliest_after_date(self.0.start.ymd(
            year,
            month as u32 + 1,
            1,
        )))
    }
}

/// Indicate the coord has a yearly granularity.
#[derive(Clone)]
pub struct Yearly<T: TimeValue>(Range<T>);

fn generate_yearly_keypoints<T: TimeValue>(
    max_points: usize,
    mut start_year: i32,
    start_month: u32,
    mut end_year: i32,
    end_month: u32,
    builder: &T,
) -> Vec<T> {
    if start_month > end_month {
        end_year -= 1;
    }

    let mut exp10 = 1;

    while (end_year - start_year + 1) as usize / (exp10 * 10) > max_points {
        exp10 *= 10;
    }

    let mut freq = exp10;

    for try_freq in &[1, 2, 5, 10] {
        freq = *try_freq * exp10;
        if (end_year - start_year + 1) as usize / (exp10 * *try_freq) <= max_points {
            break;
        }
    }

    let mut ret = vec![];

    while start_year <= end_year {
        ret.push(T::earliest_after_date(builder.ymd(
            start_year,
            start_month,
            1,
        )));
        start_year += freq as i32;
    }

    ret
}

impl<T: TimeValue + Datelike + Clone> ValueFormatter<T> for Yearly<T> {
    fn format(value: &T) -> String {
        format!("{}-{}", value.year(), value.month())
    }
}

impl<T: TimeValue + Clone> Ranged for Yearly<T>
where
    Range<T>: AsRangedCoord<Value = T>,
{
    type FormatOption = NoDefaultFormatting;
    type ValueType = T;

    fn range(&self) -> Range<T> {
        self.0.start.clone()..self.0.end.clone()
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        T::map_coord(value, &self.0.start, &self.0.end, limit)
    }

    fn key_points<HintType: KeyPointHint>(&self, hint: HintType) -> Vec<Self::ValueType> {
        if hint.weight().allow_light_points() && self.size() <= hint.bold_points() * 2 {
            return Monthly(self.0.clone()).key_points(hint);
        }
        let max_points = hint.max_num_points();
        let start_date = self.0.start.date_ceil();
        let end_date = self.0.end.date_floor();

        let mut start_year = start_date.year();
        let mut start_month = start_date.month();
        let start_day = start_date.day();

        let end_year = end_date.year();
        let end_month = end_date.month();

        if start_day != 1 {
            start_month += 1;
            if start_month == 13 {
                start_month = 1;
                start_year += 1;
            }
        }

        generate_yearly_keypoints(
            max_points,
            start_year,
            start_month,
            end_year,
            end_month,
            &self.0.start,
        )
    }
}

impl<T: TimeValue + Clone> DiscreteRanged for Yearly<T>
where
    Range<T>: AsRangedCoord<Value = T>,
{
    fn size(&self) -> usize {
        let year_start = self.0.start.date_ceil().year();
        let year_end = self.0.end.date_floor().year();
        ((year_end - year_start).max(-1) + 1) as usize
    }

    fn index_of(&self, value: &T) -> Option<usize> {
        let year_start = self.0.start.date_ceil().year();
        let year_value = value.date_floor().year();
        let ret = year_value - year_start;
        if ret < 0 {
            return None;
        }
        Some(ret as usize)
    }

    fn from_index(&self, index: usize) -> Option<T> {
        let year = self.0.start.date_ceil().year() + index as i32;
        let ret = T::earliest_after_date(self.0.start.ymd(year, 1, 1));
        if ret.date_ceil() <= self.0.start.date_floor() {
            return Some(self.0.start.clone());
        }
        Some(ret)
    }
}

/// The trait that converts a normal date coord into a monthly one
pub trait IntoMonthly<T: TimeValue> {
    /// Converts a normal date coord into a monthly one
    fn monthly(self) -> Monthly<T>;
}

/// The trait that converts a normal date coord into a yearly one
pub trait IntoYearly<T: TimeValue> {
    /// Converts a normal date coord into a yearly one
    fn yearly(self) -> Yearly<T>;
}

impl<T: TimeValue> IntoMonthly<T> for Range<T> {
    fn monthly(self) -> Monthly<T> {
        Monthly(self)
    }
}

impl<T: TimeValue> IntoYearly<T> for Range<T> {
    fn yearly(self) -> Yearly<T> {
        Yearly(self)
    }
}

/// The ranged coordinate for the date and time
#[derive(Clone)]
pub struct RangedDateTime<DT: Datelike + Timelike + TimeValue>(DT, DT);

impl<Z: TimeZone> AsRangedCoord for Range<DateTime<Z>> {
    type CoordDescType = RangedDateTime<DateTime<Z>>;
    type Value = DateTime<Z>;
}

impl<Z: TimeZone> From<Range<DateTime<Z>>> for RangedDateTime<DateTime<Z>> {
    fn from(range: Range<DateTime<Z>>) -> Self {
        Self(range.start, range.end)
    }
}

impl From<Range<NaiveDateTime>> for RangedDateTime<NaiveDateTime> {
    fn from(range: Range<NaiveDateTime>) -> Self {
        Self(range.start, range.end)
    }
}

impl<DT> Ranged for RangedDateTime<DT>
where
    DT: Datelike + Timelike + TimeValue + Clone + PartialOrd,
    DT: Add<Duration, Output = DT>,
    DT: Sub<DT, Output = Duration>,
    RangedDate<DT::DateType>: Ranged<ValueType = DT::DateType>,
{
    type FormatOption = DefaultFormatting;
    type ValueType = DT;

    fn range(&self) -> Range<DT> {
        self.0.clone()..self.1.clone()
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        TimeValue::map_coord(value, &self.0, &self.1, limit)
    }

    fn key_points<HintType: KeyPointHint>(&self, hint: HintType) -> Vec<Self::ValueType> {
        let max_points = hint.max_num_points();
        let total_span = self.1.clone() - self.0.clone();

        if let Some(total_ns) = total_span.num_nanoseconds() {
            if let Some(actual_ns_per_point) =
                compute_period_per_point(total_ns as u64, max_points, true)
            {
                let start_time_ns = u64::from(self.0.num_seconds_from_midnight()) * 1_000_000_000
                    + u64::from(self.0.nanosecond());

                let mut start_time = DT::from_date(self.0.date_floor())
                    + Duration::nanoseconds(if start_time_ns % actual_ns_per_point > 0 {
                        start_time_ns + (actual_ns_per_point - start_time_ns % actual_ns_per_point)
                    } else {
                        start_time_ns
                    } as i64);

                let mut ret = vec![];

                while start_time < self.1 {
                    ret.push(start_time.clone());
                    start_time = start_time + Duration::nanoseconds(actual_ns_per_point as i64);
                }

                return ret;
            }
        }

        // Otherwise, it actually behaves like a date
        let date_range = RangedDate(self.0.date_ceil(), self.1.date_floor());

        date_range
            .key_points(max_points)
            .into_iter()
            .map(DT::from_date)
            .collect()
    }
}

impl<DT> ReversibleRanged for RangedDateTime<DT>
where
    DT: Datelike + Timelike + TimeValue + Clone + PartialOrd,
    DT: Add<Duration, Output = DT>,
    DT: Sub<DT, Output = Duration>,
    RangedDate<DT::DateType>: Ranged<ValueType = DT::DateType>,
{
    /// Perform the reverse mapping
    fn unmap(&self, input: i32, limit: (i32, i32)) -> Option<Self::ValueType> {
        Some(TimeValue::unmap_coord(input, &self.0, &self.1, limit))
    }
}

/// The coordinate that for duration of time
#[derive(Clone)]
pub struct RangedDuration(Duration, Duration);

impl AsRangedCoord for Range<Duration> {
    type CoordDescType = RangedDuration;
    type Value = Duration;
}

impl From<Range<Duration>> for RangedDuration {
    fn from(range: Range<Duration>) -> Self {
        Self(range.start, range.end)
    }
}

impl Ranged for RangedDuration {
    type FormatOption = DefaultFormatting;
    type ValueType = Duration;

    fn range(&self) -> Range<Duration> {
        self.0..self.1
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        let total_span = self.1 - self.0;
        let value_span = *value - self.0;

        if let Some(total_ns) = total_span.num_nanoseconds() {
            if let Some(value_ns) = value_span.num_nanoseconds() {
                return limit.0
                    + (f64::from(limit.1 - limit.0) * value_ns as f64 / total_ns as f64 + 1e-10)
                        as i32;
            }
            return limit.1;
        }

        let total_days = total_span.num_days();
        let value_days = value_span.num_days();

        limit.0
            + (f64::from(limit.1 - limit.0) * value_days as f64 / total_days as f64 + 1e-10) as i32
    }

    fn key_points<HintType: KeyPointHint>(&self, hint: HintType) -> Vec<Self::ValueType> {
        let max_points = hint.max_num_points();
        let total_span = self.1 - self.0;

        if let Some(total_ns) = total_span.num_nanoseconds() {
            if let Some(period) = compute_period_per_point(total_ns as u64, max_points, false) {
                let mut start_ns = self.0.num_nanoseconds().unwrap();

                if start_ns as u64 % period > 0 {
                    if start_ns > 0 {
                        start_ns += period as i64 - (start_ns % period as i64);
                    } else {
                        start_ns -= start_ns % period as i64;
                    }
                }

                let mut current = Duration::nanoseconds(start_ns);
                let mut ret = vec![];

                while current < self.1 {
                    ret.push(current);
                    current += Duration::nanoseconds(period as i64);
                }

                return ret;
            }
        }

        let begin_days = self.0.num_days();
        let end_days = self.1.num_days();

        let mut days_per_tick = 1;
        let mut idx = 0;
        const MULTIPLIER: &[i32] = &[1, 2, 5];

        while (end_days - begin_days) / i64::from(days_per_tick * MULTIPLIER[idx])
            > max_points as i64
        {
            idx += 1;
            if idx == MULTIPLIER.len() {
                idx = 0;
                days_per_tick *= 10;
            }
        }

        days_per_tick *= MULTIPLIER[idx];

        let mut ret = vec![];

        let mut current = Duration::days(
            self.0.num_days()
                + if Duration::days(self.0.num_days()) != self.0 {
                    1
                } else {
                    0
                },
        );

        while current < self.1 {
            ret.push(current);
            current += Duration::days(i64::from(days_per_tick));
        }

        ret
    }
}

#[allow(clippy::inconsistent_digit_grouping)]
fn compute_period_per_point(total_ns: u64, max_points: usize, sub_daily: bool) -> Option<u64> {
    let min_ns_per_point = total_ns as f64 / max_points as f64;
    let actual_ns_per_point: u64 = (10u64).pow(min_ns_per_point.log10().floor() as u32);

    fn determine_actual_ns_per_point(
        total_ns: u64,
        mut actual_ns_per_point: u64,
        units: &[u64],
        base: u64,
        max_points: usize,
    ) -> u64 {
        let mut unit_per_point_idx = 0;
        while total_ns / actual_ns_per_point > max_points as u64 * units[unit_per_point_idx] {
            unit_per_point_idx += 1;
            if unit_per_point_idx == units.len() {
                unit_per_point_idx = 0;
                actual_ns_per_point *= base;
            }
        }
        units[unit_per_point_idx] * actual_ns_per_point
    }

    if actual_ns_per_point < 1_000_000_000 {
        Some(determine_actual_ns_per_point(
            total_ns,
            actual_ns_per_point,
            &[1, 2, 5],
            10,
            max_points,
        ))
    } else if actual_ns_per_point < 3600_000_000_000 {
        Some(determine_actual_ns_per_point(
            total_ns,
            1_000_000_000,
            &[1, 2, 5, 10, 15, 20, 30],
            60,
            max_points,
        ))
    } else if actual_ns_per_point < 3600_000_000_000 * 24 {
        Some(determine_actual_ns_per_point(
            total_ns,
            3600_000_000_000,
            &[1, 2, 4, 8, 12],
            24,
            max_points,
        ))
    } else if !sub_daily {
        if actual_ns_per_point < 3600_000_000_000 * 24 * 10 {
            Some(determine_actual_ns_per_point(
                total_ns,
                3600_000_000_000 * 24,
                &[1, 2, 5, 7],
                10,
                max_points,
            ))
        } else {
            Some(determine_actual_ns_per_point(
                total_ns,
                3600_000_000_000 * 24 * 10,
                &[1, 2, 5],
                10,
                max_points,
            ))
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_date_range_long() {
        let range = Utc.ymd(1000, 1, 1)..Utc.ymd(2999, 1, 1);

        let ranged_coord = Into::<RangedDate<_>>::into(range);

        assert_eq!(ranged_coord.map(&Utc.ymd(1000, 8, 10), (0, 100)), 0);
        assert_eq!(ranged_coord.map(&Utc.ymd(2999, 8, 10), (0, 100)), 100);

        let kps = ranged_coord.key_points(23);

        assert!(kps.len() <= 23);
        let max = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_days())
            .max()
            .unwrap();
        let min = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_days())
            .min()
            .unwrap();
        assert_eq!(max, min);
        assert_eq!(max % 7, 0);
    }

    #[test]
    fn test_date_range_short() {
        let range = Utc.ymd(2019, 1, 1)..Utc.ymd(2019, 1, 21);
        let ranged_coord = Into::<RangedDate<_>>::into(range);

        let kps = ranged_coord.key_points(4);

        assert_eq!(kps.len(), 3);

        let max = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_days())
            .max()
            .unwrap();
        let min = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_days())
            .min()
            .unwrap();
        assert_eq!(max, min);
        assert_eq!(max, 7);

        let kps = ranged_coord.key_points(30);
        assert_eq!(kps.len(), 21);
        let max = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_days())
            .max()
            .unwrap();
        let min = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_days())
            .min()
            .unwrap();
        assert_eq!(max, min);
        assert_eq!(max, 1);
    }

    #[test]
    fn test_yearly_date_range() {
        use crate::coord::ranged1d::BoldPoints;
        let range = Utc.ymd(1000, 8, 5)..Utc.ymd(2999, 1, 1);
        let ranged_coord = range.yearly();

        assert_eq!(ranged_coord.map(&Utc.ymd(1000, 8, 10), (0, 100)), 0);
        assert_eq!(ranged_coord.map(&Utc.ymd(2999, 8, 10), (0, 100)), 100);

        let kps = ranged_coord.key_points(23);

        assert!(kps.len() <= 23);
        let max = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_days())
            .max()
            .unwrap();
        let min = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_days())
            .min()
            .unwrap();
        assert!(max != min);

        assert!(kps.into_iter().all(|x| x.month() == 9 && x.day() == 1));

        let range = Utc.ymd(2019, 8, 5)..Utc.ymd(2020, 1, 1);
        let ranged_coord = range.yearly();
        let kps = ranged_coord.key_points(BoldPoints(23));
        assert!(kps.len() == 1);
    }

    #[test]
    fn test_monthly_date_range() {
        let range = Utc.ymd(2019, 8, 5)..Utc.ymd(2020, 9, 1);
        let ranged_coord = range.monthly();

        use crate::coord::ranged1d::BoldPoints;

        let kps = ranged_coord.key_points(BoldPoints(15));

        assert!(kps.len() <= 15);
        assert!(kps.iter().all(|x| x.day() == 1));
        assert!(kps.into_iter().any(|x| x.month() != 9));

        let kps = ranged_coord.key_points(BoldPoints(5));
        assert!(kps.len() <= 5);
        assert!(kps.iter().all(|x| x.day() == 1));
        let kps: Vec<_> = kps.into_iter().map(|x| x.month()).collect();
        assert_eq!(kps, vec![9, 12, 3, 6, 9]);

        // TODO: Investigate why max_point = 1 breaks the contract
        let kps = ranged_coord.key_points(3);
        assert!(kps.len() == 3);
        assert!(kps.iter().all(|x| x.day() == 1));
        let kps: Vec<_> = kps.into_iter().map(|x| x.month()).collect();
        assert_eq!(kps, vec![9, 3, 9]);
    }

    #[test]
    fn test_datetime_long_range() {
        let coord: RangedDateTime<_> =
            (Utc.ymd(1000, 1, 1).and_hms(0, 0, 0)..Utc.ymd(3000, 1, 1).and_hms(0, 0, 0)).into();

        assert_eq!(
            coord.map(&Utc.ymd(1000, 1, 1).and_hms(0, 0, 0), (0, 100)),
            0
        );
        assert_eq!(
            coord.map(&Utc.ymd(3000, 1, 1).and_hms(0, 0, 0), (0, 100)),
            100
        );

        let kps = coord.key_points(23);

        assert!(kps.len() <= 23);
        let max = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_seconds())
            .max()
            .unwrap();
        let min = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_seconds())
            .min()
            .unwrap();
        assert!(max == min);
        assert!(max % (24 * 3600 * 7) == 0);
    }

    #[test]
    fn test_datetime_medium_range() {
        let coord: RangedDateTime<_> =
            (Utc.ymd(2019, 1, 1).and_hms(0, 0, 0)..Utc.ymd(2019, 1, 11).and_hms(0, 0, 0)).into();

        let kps = coord.key_points(23);

        assert!(kps.len() <= 23);
        let max = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_seconds())
            .max()
            .unwrap();
        let min = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_seconds())
            .min()
            .unwrap();
        assert!(max == min);
        assert_eq!(max, 12 * 3600);
    }

    #[test]
    fn test_datetime_short_range() {
        let coord: RangedDateTime<_> =
            (Utc.ymd(2019, 1, 1).and_hms(0, 0, 0)..Utc.ymd(2019, 1, 2).and_hms(0, 0, 0)).into();

        let kps = coord.key_points(50);

        assert!(kps.len() <= 50);
        let max = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_seconds())
            .max()
            .unwrap();
        let min = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_seconds())
            .min()
            .unwrap();
        assert!(max == min);
        assert_eq!(max, 1800);
    }

    #[test]
    fn test_datetime_nano_range() {
        let start = Utc.ymd(2019, 1, 1).and_hms(0, 0, 0);
        let end = start + Duration::nanoseconds(100);
        let coord: RangedDateTime<_> = (start..end).into();

        let kps = coord.key_points(50);

        assert!(kps.len() <= 50);
        let max = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_nanoseconds().unwrap())
            .max()
            .unwrap();
        let min = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_nanoseconds().unwrap())
            .min()
            .unwrap();
        assert!(max == min);
        assert_eq!(max, 2);
    }

    #[test]
    fn test_duration_long_range() {
        let coord: RangedDuration = (Duration::days(-1000000)..Duration::days(1000000)).into();

        assert_eq!(coord.map(&Duration::days(-1000000), (0, 100)), 0);
        assert_eq!(coord.map(&Duration::days(1000000), (0, 100)), 100);

        let kps = coord.key_points(23);

        assert!(kps.len() <= 23);
        let max = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_seconds())
            .max()
            .unwrap();
        let min = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_seconds())
            .min()
            .unwrap();
        assert!(max == min);
        assert!(max % (24 * 3600 * 10000) == 0);
    }

    #[test]
    fn test_duration_daily_range() {
        let coord: RangedDuration = (Duration::days(0)..Duration::hours(25)).into();

        let kps = coord.key_points(23);

        assert!(kps.len() <= 23);
        let max = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_seconds())
            .max()
            .unwrap();
        let min = kps
            .iter()
            .zip(kps.iter().skip(1))
            .map(|(p, n)| (*n - *p).num_seconds())
            .min()
            .unwrap();
        assert!(max == min);
        assert_eq!(max, 3600 * 2);
    }

    #[test]
    fn test_date_discrete() {
        let coord: RangedDate<Date<_>> = (Utc.ymd(2019, 1, 1)..Utc.ymd(2019, 12, 31)).into();
        assert_eq!(coord.size(), 365);
        assert_eq!(coord.index_of(&Utc.ymd(2019, 2, 28)), Some(31 + 28 - 1));
        assert_eq!(coord.from_index(364), Some(Utc.ymd(2019, 12, 31)));
    }

    #[test]
    fn test_monthly_discrete() {
        let coord1 = (Utc.ymd(2019, 1, 10)..Utc.ymd(2019, 12, 31)).monthly();
        let coord2 = (Utc.ymd(2019, 1, 10)..Utc.ymd(2020, 1, 1)).monthly();
        assert_eq!(coord1.size(), 12);
        assert_eq!(coord2.size(), 13);

        for i in 1..=12 {
            assert_eq!(coord1.from_index(i - 1).unwrap().month(), i as u32);
            assert_eq!(
                coord1.index_of(&coord1.from_index(i - 1).unwrap()).unwrap(),
                i - 1
            );
        }
    }

    #[test]
    fn test_yearly_discrete() {
        let coord1 = (Utc.ymd(2000, 1, 10)..Utc.ymd(2019, 12, 31)).yearly();
        assert_eq!(coord1.size(), 20);

        for i in 0..20 {
            assert_eq!(coord1.from_index(i).unwrap().year(), 2000 + i as i32);
            assert_eq!(coord1.index_of(&coord1.from_index(i).unwrap()).unwrap(), i);
        }
    }

    #[test]
    fn test_datetime_with_unmap() {
        let start_time = Utc.ymd(2021, 1, 1).and_hms(8, 0, 0);
        let end_time = Utc.ymd(2023, 1, 1).and_hms(8, 0, 0);
        let mid = Utc.ymd(2022, 1, 1).and_hms(8, 0, 0);
        let coord: RangedDateTime<_> = (start_time..end_time).into();
        let pos = coord.map(&mid, (1000, 2000));
        assert_eq!(pos, 1500);
        let value = coord.unmap(pos, (1000, 2000));
        assert_eq!(value, Some(mid));
    }

    #[test]
    fn test_naivedatetime_with_unmap() {
        let start_time = NaiveDate::from_ymd(2021, 1, 1).and_hms_milli(8, 0, 0, 0);
        let end_time = NaiveDate::from_ymd(2023, 1, 1).and_hms_milli(8, 0, 0, 0);
        let mid = NaiveDate::from_ymd(2022, 1, 1).and_hms_milli(8, 0, 0, 0);
        let coord: RangedDateTime<_> = (start_time..end_time).into();
        let pos = coord.map(&mid, (1000, 2000));
        assert_eq!(pos, 1500);
        let value = coord.unmap(pos, (1000, 2000));
        assert_eq!(value, Some(mid));
    }

    #[test]
    fn test_date_with_unmap() {
        let start_date = Utc.ymd(2021, 1, 1);
        let end_date = Utc.ymd(2023, 1, 1);
        let mid = Utc.ymd(2022, 1, 1);
        let coord: RangedDate<Date<_>> = (start_date..end_date).into();
        let pos = coord.map(&mid, (1000, 2000));
        assert_eq!(pos, 1500);
        let value = coord.unmap(pos, (1000, 2000));
        assert_eq!(value, Some(mid));
    }

    #[test]
    fn test_naivedate_with_unmap() {
        let start_date = NaiveDate::from_ymd(2021, 1, 1);
        let end_date = NaiveDate::from_ymd(2023, 1, 1);
        let mid = NaiveDate::from_ymd(2022, 1, 1);
        let coord: RangedDate<NaiveDate> = (start_date..end_date).into();
        let pos = coord.map(&mid, (1000, 2000));
        assert_eq!(pos, 1500);
        let value = coord.unmap(pos, (1000, 2000));
        assert_eq!(value, Some(mid));
    }

    #[test]
    fn test_datetime_unmap_for_nanoseconds() {
        let start_time = Utc.ymd(2021, 1, 1).and_hms(8, 0, 0);
        let end_time = start_time + Duration::nanoseconds(1900);
        let mid = start_time + Duration::nanoseconds(950);
        let coord: RangedDateTime<_> = (start_time..end_time).into();
        let pos = coord.map(&mid, (1000, 2000));
        assert_eq!(pos, 1500);
        let value = coord.unmap(pos, (1000, 2000));
        assert_eq!(value, Some(mid));
    }

    #[test]
    fn test_datetime_unmap_for_nanoseconds_small_period() {
        let start_time = Utc.ymd(2021, 1, 1).and_hms(8, 0, 0);
        let end_time = start_time + Duration::nanoseconds(400);
        let coord: RangedDateTime<_> = (start_time..end_time).into();
        let value = coord.unmap(2000, (1000, 2000));
        assert_eq!(value, Some(end_time));
        let mid = start_time + Duration::nanoseconds(200);
        let value = coord.unmap(500, (0, 1000));
        assert_eq!(value, Some(mid));
    }
}
