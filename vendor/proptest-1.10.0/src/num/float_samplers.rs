//-
// Copyright 2022 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Alternative uniform float samplers.
//! These samplers are used over the ones from `rand` because the ones provided by the
//! rand crate are prone to overflow. In addition, these are 'high precision' samplers
//! that are more appropriate for test data.
//! The samplers work by splitting the range into equally sized intervals and selecting
//! an iterval at random. That interval is then itself split and a new interval is
//! selected at random. The process repeats until the interval only contains two
//! floating point values at the bounds. At that stage, one is selected at random and
//! returned.

pub(crate) use self::f32::F32U;
pub(crate) use self::f64::F64U;

macro_rules! float_sampler {
    ($typ: ident, $int_typ: ident, $wrapper: ident) => {
        pub mod $typ {
            use rand::prelude::*;
            use rand::distr::uniform::{
                SampleBorrow, SampleUniform, UniformSampler,
            };
            #[cfg(not(feature = "std"))]
            use num_traits::float::Float;

            #[must_use]
            // Returns the previous float value. In other words the greatest value representable
            // as a float such that `next_down(a) < a`. `-0.` is treated as `0.`.
            fn next_down(a: $typ) -> $typ {
                debug_assert!(a.is_finite() && a > $typ::MIN, "`next_down` invalid input: {}", a);
                if a == (0.) {
                    -$typ::from_bits(1)
                } else if a < 0. {
                    $typ::from_bits(a.to_bits() + 1)
                } else {
                    $typ::from_bits(a.to_bits() - 1)
                }
            }

            #[must_use]
            // Returns the unit in last place using the definition by John Harrison.
            // This is the distance between `a` and the next closest float. Note that
            // `ulp(1) = $typ::EPSILON/2`.
            fn ulp(a: $typ) -> $typ {
                debug_assert!(a.is_finite() && a > $typ::MIN, "`ulp` invalid input: {}", a);
                a.abs() - next_down(a.abs())
            }

            #[derive(Copy, Clone, Debug)]
            pub(crate) struct $wrapper($typ);

            impl From<$typ> for $wrapper {
                fn from(x: $typ) -> Self {
                    $wrapper(x)
                }
            }
            impl From<$wrapper> for $typ {
                fn from(x: $wrapper) -> Self {
                    x.0
                }
            }

            #[derive(Clone, Copy, Debug)]
            pub(crate) struct FloatUniform {
                low: $typ,
                high: $typ,
                intervals: IntervalCollection,
                inclusive: bool,
            }

            impl UniformSampler for FloatUniform {

                type X = $wrapper;

                fn new<B1, B2>(low: B1, high: B2) -> Result<Self, rand::distr::uniform::Error>
                where
                    B1: SampleBorrow<Self::X> + Sized,
                    B2: SampleBorrow<Self::X> + Sized,
                {
                    let low = low.borrow().0;
                    let high = high.borrow().0;
                    Ok(FloatUniform {
                        low,
                        high,
                        intervals: split_interval([low, high]),
                        inclusive: false,
                    })
                }

                fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Self, rand::distr::uniform::Error>
                where
                    B1: SampleBorrow<Self::X> + Sized,
                    B2: SampleBorrow<Self::X> + Sized,
                {
                    let low = low.borrow().0;
                    let high = high.borrow().0;

                    Ok(FloatUniform {
                        low,
                        high,
                        intervals: split_interval([low, high]),
                        inclusive: true,
                    })
                }

                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                    let mut intervals = self.intervals;
                    while intervals.count > 1 {
                        let new_interval = intervals.get(rng.random_range(0..intervals.count));
                        intervals = split_interval(new_interval);
                    }
                    let last = intervals.get(0);
                    let result = *last.choose(rng).expect("Slice is not empty");

                    // These results could happen because the first split might
                    // overshoot one of the bounds. We could resample in this
                    // case but for testing data this is not a problem.
                    let clamped_result = if result < self.low {
                        debug_assert!(self.low - result < self.intervals.step);
                        self.low
                    } else if result > self.high{
                        debug_assert!(result - self.high < self.intervals.step);
                        self.high
                    } else {
                        result
                    };

                    if !self.inclusive && clamped_result == self.high  {
                        return $wrapper(next_down(self.high));
                    };

                    $wrapper(clamped_result)
                }
            }

            impl SampleUniform for $wrapper {
                type Sampler = FloatUniform;
            }

            // Divides the range [low, high] into intervals of size epsilon * max(abs(low, high));
            // Note that the one interval may extend out of the range.
            #[derive(Clone, Copy, Debug)]
            struct IntervalCollection {
                start: $typ,
                step: $typ,
                count: $int_typ,
            }

            fn split_interval([low, high]: [$typ; 2]) -> IntervalCollection {
                    assert!(low.is_finite(), "low finite");
                    assert!(high.is_finite(), "high finite");
                    assert!(high - low > 0., "invalid range");

                    let min_abs = $typ::min(low.abs(), high.abs());
                    let max_abs = $typ::max(low.abs(), high.abs());

                    let gap = ulp(max_abs);

                    let (start, step) = if low.abs() < high.abs() {
                        (high, -gap)
                    } else {
                        (low, gap)
                    };

                    let min_gaps = min_abs / gap;
                    let max_gaps = max_abs / gap;
                    debug_assert!(
                        max_gaps.floor() == max_gaps,
                        "max_gaps is an integer"
                    );

                    let count = if low.signum() == high.signum() {
                        max_gaps as $int_typ - min_gaps.floor() as $int_typ
                    } else {
                        // `step` is a power of two so `min_gaps` won't be rounded
                        // except possibly to 0.
                        if min_gaps == 0. && min_abs > 0. {
                            max_gaps as $int_typ + 1
                        } else {
                            max_gaps as $int_typ + min_gaps.ceil() as $int_typ
                        }
                    };

                    debug_assert!(count - 1 <= 2 * MAX_PRECISE_INT);

                    IntervalCollection {
                        start,
                        step,
                        count,
                    }
            }


            impl IntervalCollection {
                fn get(&self, index: $int_typ) -> [$typ; 2] {
                    assert!(index < self.count, "index out of bounds");

                    // `index` might be greater that `MAX_PERCISE_INT`
                    // which means `MAX_PRECIST_INT as $typ` would round
                    // to a different number. Fortunately, `index` will
                    // never be larger than `2 * MAX_PRECISE_INT` (as
                    // asserted above).
                    let x = ((index / 2) as $typ).mul_add(
                        2. * self.step,
                        (index % 2) as $typ * self.step + self.start,
                    );

                    let y = x + self.step;

                    if self.step > 0. {
                        [x, y]
                    } else {
                        [y, x]
                    }
                }
            }


            // Values greater than MAX_PRECISE_INT may be rounded when converted to float.
            const MAX_PRECISE_INT: $int_typ =
                (2 as $int_typ).pow($typ::MANTISSA_DIGITS);

            #[cfg(test)]
            mod test {

                use super::*;
                use crate::prelude::*;

                fn sort((left, right): ($typ, $typ)) -> ($typ, $typ) {
                    if left < right {
                        (left, right)
                    } else {
                        (right, left)
                    }
                }

                fn finite() -> impl Strategy<Value = $typ> {
                    prop::num::$typ::NEGATIVE
                    | prop::num::$typ::POSITIVE
                    | prop::num::$typ::NORMAL
                    | prop::num::$typ::SUBNORMAL
                    | prop::num::$typ::ZERO
                }

                fn bounds() -> impl Strategy<Value = ($typ, $typ)> {
                    (finite(), finite())
                        .prop_filter("Bounds can't be equal", |(a, b)| a != b)
                        .prop_map(sort)
                }

                #[test]
                fn range_test() {
                    use crate::test_runner::{RngAlgorithm, TestRng};

                    let mut test_rng = TestRng::deterministic_rng(RngAlgorithm::default());
                    let (low, high) = (-1., 10.);
                    let uniform = FloatUniform::new($wrapper(low), $wrapper(high)).expect("not uniform");

                    let samples = (0..100)
                        .map(|_| $typ::from(uniform.sample(&mut test_rng)));
                    for s in samples {
                        assert!(low <= s && s < high);
                    }
                }

                #[test]
                fn range_end_bound_test() {
                    use crate::test_runner::{RngAlgorithm, TestRng};

                    let mut test_rng = TestRng::deterministic_rng(RngAlgorithm::default());
                    let (low, high) = (1., 1. + $typ::EPSILON);
                    let uniform = FloatUniform::new($wrapper(low), $wrapper(high)).expect("not uniform");

                    let mut samples = (0..100)
                        .map(|_| $typ::from(uniform.sample(&mut test_rng)));
                    assert!(samples.all(|x| x == 1.));
                }

                #[test]
                fn inclusive_range_test() {
                    use crate::test_runner::{RngAlgorithm, TestRng};

                    let mut test_rng = TestRng::deterministic_rng(RngAlgorithm::default());
                    let (low, high) = (-1., 10.);
                    let uniform = FloatUniform::new_inclusive($wrapper(low), $wrapper(high)).expect("not uniform");

                    let samples = (0..100)
                        .map(|_| $typ::from(uniform.sample(&mut test_rng)));
                    for s in samples {
                        assert!(low <= s && s <= high);
                    }
                }

                #[test]
                fn inclusive_range_end_bound_test() {
                    use crate::test_runner::{RngAlgorithm, TestRng};

                    let mut test_rng = TestRng::deterministic_rng(RngAlgorithm::default());
                    let (low, high) = (1., 1. + $typ::EPSILON);
                    let uniform = FloatUniform::new_inclusive($wrapper(low), $wrapper(high)).expect("not uniform");

                    let mut samples = (0..100)
                        .map(|_| $typ::from(uniform.sample(&mut test_rng)));
                    assert!(samples.any(|x| x == 1. + $typ::EPSILON));
                }

                #[test]
                fn all_floats_in_range_are_possible_1() {
                    use crate::test_runner::{RngAlgorithm, TestRng};

                    let mut test_rng = TestRng::deterministic_rng(RngAlgorithm::default());
                    let (low, high) = (1. - $typ::EPSILON, 1. + $typ::EPSILON);
                    let uniform = FloatUniform::new_inclusive($wrapper(low), $wrapper(high)).expect("not uniform");

                    let mut samples = (0..100)
                        .map(|_| $typ::from(uniform.sample(&mut test_rng)));
                    assert!(samples.any(|x| x == 1. - $typ::EPSILON / 2.));
                }

                #[test]
                fn all_floats_in_range_are_possible_2() {
                    use crate::test_runner::{RngAlgorithm, TestRng};

                    let mut test_rng = TestRng::deterministic_rng(RngAlgorithm::default());
                    let (low, high) = (0., MAX_PRECISE_INT as $typ);
                    let uniform = FloatUniform::new_inclusive($wrapper(low), $wrapper(high)).expect("not uniform");

                    let mut samples = (0..100)
                        .map(|_| $typ::from(uniform.sample(&mut test_rng)))
                        .map(|x| x.fract());

                    assert!(samples.any(|x| x != 0.));
                }

                #[test]
                fn max_precise_int_plus_one_is_rounded_down() {
                    assert_eq!(((MAX_PRECISE_INT + 1) as $typ) as $int_typ, MAX_PRECISE_INT);
                }

                proptest! {
                    #[test]
                    fn next_down_less_than_float(val in finite()) {
                        prop_assume!(val > $typ::MIN);
                        prop_assert!(next_down(val) <  val);
                    }

                    #[test]
                    fn no_value_between_float_and_next_down(val in finite()) {
                        prop_assume!(val > $typ::MIN);
                        let prev = next_down(val);
                        let avg = prev / 2. + val / 2.;
                        prop_assert!(avg == prev || avg == val);
                    }

                    #[test]
                    fn values_less_than_or_equal_to_max_precise_int_are_not_rounded(i in 0..=MAX_PRECISE_INT) {
                        prop_assert_eq!((i as $typ) as $int_typ, i);
                    }

                    #[test]
                    fn indivisible_intervals_are_split_to_self(val in finite()) {
                        prop_assume!(val > $typ::MIN);
                        let prev = next_down(val);
                        let intervals = split_interval([prev, val]);
                        prop_assert_eq!(intervals.count, 1);
                    }

                    #[test]
                    fn split_intervals_are_the_same_size(
                            (low, high) in bounds(),
                            indices: [prop::sample::Index; 32]) {

                        let intervals = split_interval([low, high]);

                        let size = (intervals.count - 1) as usize;
                        prop_assume!(size > 0);

                        let mut it = indices.iter()
                            .map(|i| i.index(size) as $int_typ)
                            .map(|i| intervals.get(i))
                            .map(|[low, high]| high - low);

                        let interval_size = it.next().unwrap();
                        let all_equal = it.all(|g| g == interval_size);
                        prop_assert!(all_equal);
                    }

                    #[test]
                    fn split_intervals_are_consecutive(
                        (low, high) in bounds(),
                        indices: [prop::sample::Index; 32]) {

                        let intervals = split_interval([low, high]);

                        let size = (intervals.count - 1) as usize;
                        prop_assume!(size > 1);

                        let mut it = indices.iter()
                            .map(|i| i.index(size - 1) as $int_typ)
                            .map(|i| (intervals.get(i), intervals.get(i + 1)));

                        let ascending = it.all(|([_, h1], [l2, _])| h1 == l2);
                        let descending = it.all(|([l1, _], [_, h2])| l1 == h2);

                        prop_assert!(ascending || descending);
                    }

                    #[test]
                    fn first_split_might_slightly_overshoot_one_bound((low, high) in bounds()) {
                        let intervals = split_interval([low, high]);
                        let start = intervals.get(0);
                        let end = intervals.get(intervals.count - 1);
                        let (low_interval, high_interval) = if  start[0] < end[0] {
                            (start, end)
                        } else {
                            (end, start)
                        };

                        prop_assert!(
                            low == low_interval[0] && high_interval[0] < high && high <= high_interval[1] ||
                            low_interval[0] <= low && low < low_interval[1] && high == high_interval[1]);
                    }

                    #[test]
                    fn subsequent_splits_always_match_bounds(
                        (low, high) in bounds(),
                        index: prop::sample::Index) {
                        // This property is true because the distances of split intervals of
                        // are powers of two so the smaller one always divides the larger.

                        let intervals = split_interval([low, high]);
                        let size = (intervals.count - 1) as usize;

                        let interval = intervals.get(index.index(size) as $int_typ);
                        let small_intervals = split_interval(interval);

                        let start = small_intervals.get(0);
                        let end = small_intervals.get(small_intervals.count - 1);
                        let (low_interval, high_interval) = if  start[0] < end[0] {
                            (start, end)
                        } else {
                            (end, start)
                        };

                        prop_assert!(
                            interval[0] == low_interval[0] &&
                            interval[1] == high_interval[1]);
                    }
                }
            }
        }
    };
}

float_sampler!(f32, u32, F32U);
float_sampler!(f64, u64, F64U);
