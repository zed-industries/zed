use std::convert::TryFrom;
use std::ops::Range;

use crate::coord::ranged1d::{
    AsRangedCoord, DefaultFormatting, DiscreteRanged, KeyPointHint, NoDefaultFormatting, Ranged,
    ReversibleRanged, ValueFormatter,
};

macro_rules! impl_discrete_trait {
    ($name:ident) => {
        impl DiscreteRanged for $name {
            fn size(&self) -> usize {
                if &self.1 < &self.0 {
                    return 0;
                }
                let values = self.1 - self.0;
                (values + 1) as usize
            }

            fn index_of(&self, value: &Self::ValueType) -> Option<usize> {
                if value < &self.0 {
                    return None;
                }
                let ret = value - self.0;
                Some(ret as usize)
            }

            fn from_index(&self, index: usize) -> Option<Self::ValueType> {
                if let Ok(index) = Self::ValueType::try_from(index) {
                    return Some(self.0 + index);
                }
                None
            }
        }
    };
}

macro_rules! impl_ranged_type_trait {
    ($value:ty, $coord:ident) => {
        impl AsRangedCoord for Range<$value> {
            type CoordDescType = $coord;
            type Value = $value;
        }
    };
}
macro_rules! impl_reverse_mapping_trait {
    ($type:ty, $name: ident) => {
        impl ReversibleRanged for $name {
            fn unmap(&self, p: i32, (min, max): (i32, i32)) -> Option<$type> {
                if p < min.min(max) || p > max.max(min) || min == max {
                    return None;
                }

                let logical_offset = f64::from(p - min) / f64::from(max - min);

                return Some(((self.1 - self.0) as f64 * logical_offset + self.0 as f64) as $type);
            }
        }
    };
}
macro_rules! make_numeric_coord {
    ($type:ty, $name:ident, $key_points:ident, $doc: expr, $fmt: ident) => {
        #[doc = $doc]
        #[derive(Clone)]
        pub struct $name($type, $type);
        impl From<Range<$type>> for $name {
            fn from(range: Range<$type>) -> Self {
                return $name(range.start, range.end);
            }
        }
        impl Ranged for $name {
            type FormatOption = $fmt;
            type ValueType = $type;
            #[allow(clippy::float_cmp)]
            fn map(&self, v: &$type, limit: (i32, i32)) -> i32 {
                // Corner case: If we have a range that have only one value,
                // then we just assign everything to the only point
                if self.1 == self.0 {
                    return (limit.1 - limit.0) / 2;
                }

                let logic_length = (*v as f64 - self.0 as f64) / (self.1 as f64 - self.0 as f64);

                let actual_length = limit.1 - limit.0;

                if actual_length == 0 {
                    return limit.1;
                }

                if logic_length.is_infinite() {
                    if logic_length.is_sign_positive() {
                        return limit.1;
                    } else {
                        return limit.0;
                    }
                }

                if actual_length > 0 {
                    return limit.0 + (actual_length as f64 * logic_length + 1e-3).floor() as i32;
                } else {
                    return limit.0 + (actual_length as f64 * logic_length - 1e-3).ceil() as i32;
                }
            }
            fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<$type> {
                $key_points((self.0, self.1), hint.max_num_points())
            }
            fn range(&self) -> Range<$type> {
                return self.0..self.1;
            }
        }
    };
    ($type:ty, $name:ident, $key_points:ident, $doc: expr) => {
        make_numeric_coord!($type, $name, $key_points, $doc, DefaultFormatting);
    };
}

macro_rules! gen_key_points_comp {
    (float, $name:ident, $type:ty) => {
        fn $name(range: ($type, $type), max_points: usize) -> Vec<$type> {
            if max_points == 0 {
                return vec![];
            }

            let range = (range.0.min(range.1) as f64, range.1.max(range.0) as f64);

            assert!(!(range.0.is_nan() || range.1.is_nan()));

            if (range.0 - range.1).abs() < f64::EPSILON {
                return vec![range.0 as $type];
            }

            let mut scale = (10f64).powf((range.1 - range.0).log(10.0).floor());
            // The value granularity controls how we round the values.
            // To avoid generating key points like 1.00000000001, we round to the nearest multiple of the
            // value granularity.
            // By default, we make the granularity as the 1/10 of the scale.
            let mut value_granularity = scale / 10.0;
            fn rem_euclid(a: f64, b: f64) -> f64 {
                let ret = if b > 0.0 {
                    a - (a / b).floor() * b
                } else {
                    a - (a / b).ceil() * b
                };
                if (ret - b).abs() < f64::EPSILON {
                    0.0
                } else {
                    ret
                }
            }

            // At this point we need to make sure that the loop invariant:
            // The scale must yield number of points than requested
            if 1 + ((range.1 - range.0) / scale).floor() as usize > max_points {
                scale *= 10.0;
                value_granularity *= 10.0;
            }

            'outer: loop {
                let old_scale = scale;
                for nxt in [2.0, 5.0, 10.0].iter() {
                    let mut new_left = range.0 - rem_euclid(range.0, old_scale / nxt);
                    if new_left < range.0 {
                        new_left += old_scale / nxt;
                    }
                    let new_right = range.1 - rem_euclid(range.1, old_scale / nxt);

                    let npoints = 1.0 + ((new_right - new_left) / old_scale * nxt);

                    if npoints.round() as usize > max_points {
                        break 'outer;
                    }

                    scale = old_scale / nxt;
                }
                scale = old_scale / 10.0;
                value_granularity /= 10.0;
            }

            let mut ret = vec![];
            // In some extreme cases, left might be too big, so that (left + scale) - left == 0 due to
            // floating point error.
            // In this case, we may loop forever. To avoid this, we need to use two variables to store
            // the current left value. So we need keep a left_base and a left_relative.
            let left = {
                let mut value = range.0 - rem_euclid(range.0, scale);
                if value < range.0 {
                    value += scale;
                }
                value
            };
            let left_base = (left / value_granularity).floor() * value_granularity;
            let mut left_relative = left - left_base;
            let right = range.1 - rem_euclid(range.1, scale);
            while (right - left_relative - left_base) >= -f64::EPSILON {
                let new_left_relative =
                    (left_relative / value_granularity).round() * value_granularity;
                if new_left_relative < 0.0 {
                    left_relative += value_granularity;
                }
                ret.push((left_relative + left_base) as $type);
                left_relative += scale;
            }
            return ret;
        }
    };
    (integer, $name:ident, $type:ty) => {
        fn $name(range: ($type, $type), max_points: usize) -> Vec<$type> {
            let mut scale: $type = 1;
            let range = (range.0.min(range.1), range.0.max(range.1));
            let range_size = range.1 as f64 - range.0 as f64;
            'outer: while (range_size / scale as f64).ceil() > max_points as f64 {
                let next_scale = scale * 10;
                for new_scale in [scale * 2, scale * 5, scale * 10].iter() {
                    scale = *new_scale;
                    if (range_size / *new_scale as f64).ceil() < max_points as f64 {
                        break 'outer;
                    }
                }
                scale = next_scale;
            }

            let (mut left, right) = (
                range.0 + (scale - range.0 % scale) % scale,
                range.1 - range.1 % scale,
            );

            let mut ret = vec![];
            while left <= right {
                ret.push(left as $type);
                if left < right {
                    left += scale;
                } else {
                    break;
                }
            }

            return ret;
        }
    };
}

gen_key_points_comp!(float, compute_f32_key_points, f32);
gen_key_points_comp!(float, compute_f64_key_points, f64);
gen_key_points_comp!(integer, compute_i32_key_points, i32);
gen_key_points_comp!(integer, compute_u32_key_points, u32);
gen_key_points_comp!(integer, compute_i64_key_points, i64);
gen_key_points_comp!(integer, compute_u64_key_points, u64);
gen_key_points_comp!(integer, compute_i128_key_points, i128);
gen_key_points_comp!(integer, compute_u128_key_points, u128);
gen_key_points_comp!(integer, compute_isize_key_points, isize);
gen_key_points_comp!(integer, compute_usize_key_points, usize);

make_numeric_coord!(
    f32,
    RangedCoordf32,
    compute_f32_key_points,
    "The ranged coordinate for type f32",
    NoDefaultFormatting
);
impl_reverse_mapping_trait!(f32, RangedCoordf32);
impl ValueFormatter<f32> for RangedCoordf32 {
    fn format(value: &f32) -> String {
        crate::data::float::FloatPrettyPrinter {
            allow_scientific: false,
            min_decimal: 1,
            max_decimal: 5,
        }
        .print(*value as f64)
    }
}
make_numeric_coord!(
    f64,
    RangedCoordf64,
    compute_f64_key_points,
    "The ranged coordinate for type f64",
    NoDefaultFormatting
);
impl_reverse_mapping_trait!(f64, RangedCoordf64);
impl ValueFormatter<f64> for RangedCoordf64 {
    fn format(value: &f64) -> String {
        crate::data::float::FloatPrettyPrinter {
            allow_scientific: false,
            min_decimal: 1,
            max_decimal: 5,
        }
        .print(*value)
    }
}
make_numeric_coord!(
    u32,
    RangedCoordu32,
    compute_u32_key_points,
    "The ranged coordinate for type u32"
);
make_numeric_coord!(
    i32,
    RangedCoordi32,
    compute_i32_key_points,
    "The ranged coordinate for type i32"
);
make_numeric_coord!(
    u64,
    RangedCoordu64,
    compute_u64_key_points,
    "The ranged coordinate for type u64"
);
make_numeric_coord!(
    i64,
    RangedCoordi64,
    compute_i64_key_points,
    "The ranged coordinate for type i64"
);
make_numeric_coord!(
    u128,
    RangedCoordu128,
    compute_u128_key_points,
    "The ranged coordinate for type u128"
);
make_numeric_coord!(
    i128,
    RangedCoordi128,
    compute_i128_key_points,
    "The ranged coordinate for type i128"
);
make_numeric_coord!(
    usize,
    RangedCoordusize,
    compute_usize_key_points,
    "The ranged coordinate for type usize"
);
make_numeric_coord!(
    isize,
    RangedCoordisize,
    compute_isize_key_points,
    "The ranged coordinate for type isize"
);

impl_discrete_trait!(RangedCoordu32);
impl_discrete_trait!(RangedCoordi32);
impl_discrete_trait!(RangedCoordu64);
impl_discrete_trait!(RangedCoordi64);
impl_discrete_trait!(RangedCoordu128);
impl_discrete_trait!(RangedCoordi128);
impl_discrete_trait!(RangedCoordusize);
impl_discrete_trait!(RangedCoordisize);

impl_ranged_type_trait!(f32, RangedCoordf32);
impl_ranged_type_trait!(f64, RangedCoordf64);
impl_ranged_type_trait!(i32, RangedCoordi32);
impl_ranged_type_trait!(u32, RangedCoordu32);
impl_ranged_type_trait!(i64, RangedCoordi64);
impl_ranged_type_trait!(u64, RangedCoordu64);
impl_ranged_type_trait!(i128, RangedCoordi128);
impl_ranged_type_trait!(u128, RangedCoordu128);
impl_ranged_type_trait!(isize, RangedCoordisize);
impl_ranged_type_trait!(usize, RangedCoordusize);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_key_points() {
        let kp = compute_i32_key_points((0, 999), 28);

        assert!(!kp.is_empty());
        assert!(kp.len() <= 28);

        let kp = compute_f64_key_points((-1.2, 1.2), 1);
        assert!(kp.len() == 1);

        let kp = compute_f64_key_points((-1.2, 1.2), 0);
        assert!(kp.is_empty());
    }

    #[test]
    fn test_linear_coord_map() {
        let coord: RangedCoordu32 = (0..20).into();
        assert_eq!(coord.key_points(11).len(), 11);
        assert_eq!(coord.key_points(11)[0], 0);
        assert_eq!(coord.key_points(11)[10], 20);
        assert_eq!(coord.map(&5, (0, 100)), 25);

        let coord: RangedCoordf32 = (0f32..20f32).into();
        assert_eq!(coord.map(&5.0, (0, 100)), 25);
    }

    #[test]
    fn test_linear_coord_system() {
        let _coord =
            crate::coord::ranged2d::cartesian::Cartesian2d::<RangedCoordu32, RangedCoordu32>::new(
                0..10,
                0..10,
                (0..1024, 0..768),
            );
    }

    #[test]
    fn test_coord_unmap() {
        let coord: RangedCoordu32 = (0..20).into();
        let pos = coord.map(&5, (1000, 2000));
        let value = coord.unmap(pos, (1000, 2000));
        assert_eq!(value, Some(5));
    }

    #[test]
    fn regression_test_issue_253_zero_sized_coord_not_hang() {
        let coord: RangedCoordf32 = (0.0..0.0).into();
        let _points = coord.key_points(10);
    }

    #[test]
    fn test_small_coord() {
        let coord: RangedCoordf64 = (0.0..1e-25).into();
        let points = coord.key_points(10);
        assert!(!points.is_empty());
    }

    #[test]
    fn regression_test_issue_255_reverse_f32_coord_no_hang() {
        let coord: RangedCoordf32 = (10.0..0.0).into();
        let _points = coord.key_points(10);
    }

    #[test]
    fn regression_test_issue_358_key_points_no_hang() {
        let coord: RangedCoordf64 = (-200.0..801.0).into();
        let points = coord.key_points(500);
        assert!(points.len() <= 500);
    }

    #[test]
    fn regression_test_issue_358_key_points_no_hang_2() {
        let coord: RangedCoordf64 = (10000000000001f64..10000000000002f64).into();
        let points = coord.key_points(500);
        assert!(points.len() <= 500);
    }

    #[test]
    fn test_coord_follows_hint() {
        let coord: RangedCoordf64 = (1.0..2.0).into();
        let points = coord.key_points(6);
        assert_eq!(points.len(), 6);
        assert_eq!(points[0], 1.0);
        let coord: RangedCoordf64 = (1.0..125.0).into();
        let points = coord.key_points(12);
        assert_eq!(points.len(), 12);
        let coord: RangedCoordf64 = (0.9995..1.0005).into();
        let points = coord.key_points(11);
        assert_eq!(points.len(), 11);
        let coord: RangedCoordf64 = (0.9995..1.0005).into();
        let points = coord.key_points(2);
        assert!(points.len() <= 2);
    }

    #[test]
    fn regression_test_issue_304_intmax_keypoint_no_panic() {
        let coord: RangedCoordu32 = (0..u32::MAX).into();
        let p = coord.key_points(10);
        assert!(!p.is_empty() && p.len() <= 10);
    }
}
