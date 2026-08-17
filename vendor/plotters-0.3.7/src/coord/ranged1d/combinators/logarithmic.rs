use crate::coord::ranged1d::types::RangedCoordf64;
use crate::coord::ranged1d::{AsRangedCoord, DefaultFormatting, KeyPointHint, Ranged};
use std::marker::PhantomData;
use std::ops::Range;

/// The trait for the type that is able to be presented in the log scale.
/// This trait is primarily used by [LogRangeExt](struct.LogRangeExt.html).
pub trait LogScalable: Clone {
    /// Make the conversion from the type to the floating point number
    fn as_f64(&self) -> f64;
    /// Convert a floating point number to the scale
    fn from_f64(f: f64) -> Self;
}

macro_rules! impl_log_scalable {
    (i, $t:ty) => {
        impl LogScalable for $t {
            fn as_f64(&self) -> f64 {
                if *self != 0 {
                    return *self as f64;
                }
                // If this is an integer, we should allow zero point to be shown
                // on the chart, thus we can't map the zero point to inf.
                // So we just assigning a value smaller than 1 as the alternative
                // of the zero point.
                return 0.5;
            }
            fn from_f64(f: f64) -> $t {
                f.round() as $t
            }
        }
    };
    (f, $t:ty) => {
        impl LogScalable for $t {
            fn as_f64(&self) -> f64 {
                *self as f64
            }
            fn from_f64(f: f64) -> $t {
                f as $t
            }
        }
    };
}

impl_log_scalable!(i, u8);
impl_log_scalable!(i, u16);
impl_log_scalable!(i, u32);
impl_log_scalable!(i, u64);
impl_log_scalable!(i, usize);

impl_log_scalable!(i, i8);
impl_log_scalable!(i, i16);
impl_log_scalable!(i, i32);
impl_log_scalable!(i, i64);
impl_log_scalable!(i, i128);
impl_log_scalable!(i, isize);

impl_log_scalable!(f, f32);
impl_log_scalable!(f, f64);

/// Convert a range to a log scale coordinate spec
pub trait IntoLogRange {
    /// The type of the value
    type ValueType: LogScalable;

    /// Make the log scale coordinate
    fn log_scale(self) -> LogRangeExt<Self::ValueType>;
}

impl<T: LogScalable> IntoLogRange for Range<T> {
    type ValueType = T;
    fn log_scale(self) -> LogRangeExt<T> {
        LogRangeExt {
            range: self,
            zero: 0.0,
            base: 10.0,
        }
    }
}

/// The logarithmic coordinate decorator.
/// This decorator is used to make the axis rendered as logarithmically.
#[derive(Clone)]
pub struct LogRangeExt<V: LogScalable> {
    range: Range<V>,
    zero: f64,
    base: f64,
}

impl<V: LogScalable> LogRangeExt<V> {
    /// Set the zero point of the log scale coordinate. Zero point is the point where we map -inf
    /// of the axis to the coordinate
    pub fn zero_point(mut self, value: V) -> Self
    where
        V: PartialEq,
    {
        self.zero = if V::from_f64(0.0) == value {
            0.0
        } else {
            value.as_f64()
        };

        self
    }

    /// Set the base multiplier
    pub fn base(mut self, base: f64) -> Self {
        if self.base > 1.0 {
            self.base = base;
        }
        self
    }
}

impl<V: LogScalable> From<LogRangeExt<V>> for LogCoord<V> {
    fn from(spec: LogRangeExt<V>) -> LogCoord<V> {
        let zero_point = spec.zero;
        let mut start = spec.range.start.as_f64() - zero_point;
        let mut end = spec.range.end.as_f64() - zero_point;
        let negative = if start < 0.0 || end < 0.0 {
            start = -start;
            end = -end;
            true
        } else {
            false
        };

        if start < end {
            if start == 0.0 {
                start = start.max(end * 1e-5);
            }
        } else if end == 0.0 {
            end = end.max(start * 1e-5);
        }

        LogCoord {
            linear: (start.ln()..end.ln()).into(),
            logic: spec.range,
            normalized: start..end,
            base: spec.base,
            zero_point,
            negative,
            marker: PhantomData,
        }
    }
}

impl<V: LogScalable> AsRangedCoord for LogRangeExt<V> {
    type CoordDescType = LogCoord<V>;
    type Value = V;
}

/// A log scaled coordinate axis
pub struct LogCoord<V: LogScalable> {
    linear: RangedCoordf64,
    logic: Range<V>,
    normalized: Range<f64>,
    base: f64,
    zero_point: f64,
    negative: bool,
    marker: PhantomData<V>,
}

impl<V: LogScalable> LogCoord<V> {
    fn value_to_f64(&self, value: &V) -> f64 {
        let fv = value.as_f64() - self.zero_point;
        if self.negative {
            -fv
        } else {
            fv
        }
    }

    fn f64_to_value(&self, fv: f64) -> V {
        let fv = if self.negative { -fv } else { fv };
        V::from_f64(fv + self.zero_point)
    }

    fn is_inf(&self, fv: f64) -> bool {
        let fv = if self.negative { -fv } else { fv };
        let a = V::from_f64(fv + self.zero_point);
        let b = V::from_f64(self.zero_point);

        (V::as_f64(&a) - V::as_f64(&b)).abs() < f64::EPSILON
    }
}

impl<V: LogScalable> Ranged for LogCoord<V> {
    type FormatOption = DefaultFormatting;
    type ValueType = V;

    fn map(&self, value: &V, limit: (i32, i32)) -> i32 {
        let fv = self.value_to_f64(value);
        let value_ln = fv.ln();
        self.linear.map(&value_ln, limit)
    }

    fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<Self::ValueType> {
        let max_points = hint.max_num_points();

        let base = self.base;
        let base_ln = base.ln();

        let Range { mut start, mut end } = self.normalized;

        if start > end {
            std::mem::swap(&mut start, &mut end);
        }

        let bold_count = ((end / start).ln().abs() / base_ln).floor().max(1.0) as usize;

        let light_density = if max_points < bold_count {
            0
        } else {
            let density = 1 + (max_points - bold_count) / bold_count;
            let mut exp = 1;
            while exp * 10 <= density {
                exp *= 10;
            }
            exp - 1
        };

        let mut multiplier = base;
        let mut cnt = 1;
        while max_points < bold_count / cnt {
            multiplier *= base;
            cnt += 1;
        }

        let mut ret = vec![];
        let mut val = (base).powf((start.ln() / base_ln).ceil());

        while val <= end {
            if !self.is_inf(val) {
                ret.push(self.f64_to_value(val));
            }
            for i in 1..=light_density {
                let v = val
                    * (1.0
                        + multiplier / f64::from(light_density as u32 + 1) * f64::from(i as u32));
                if v > end {
                    break;
                }
                if !self.is_inf(val) {
                    ret.push(self.f64_to_value(v));
                }
            }
            val *= multiplier;
        }

        ret
    }

    fn range(&self) -> Range<V> {
        self.logic.clone()
    }
}

/// The logarithmic coordinate decorator.
/// This decorator is used to make the axis rendered as logarithmically.
#[deprecated(note = "LogRange is deprecated, use IntoLogRange trait method instead")]
#[derive(Clone)]
pub struct LogRange<V: LogScalable>(pub Range<V>);

#[allow(deprecated)]
impl<V: LogScalable> AsRangedCoord for LogRange<V> {
    type CoordDescType = LogCoord<V>;
    type Value = V;
}

#[allow(deprecated)]
impl<V: LogScalable> From<LogRange<V>> for LogCoord<V> {
    fn from(range: LogRange<V>) -> LogCoord<V> {
        range.0.log_scale().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn regression_test_issue_143() {
        let range: LogCoord<f64> = (1.0..5.0).log_scale().into();

        range.key_points(100);
    }
}
