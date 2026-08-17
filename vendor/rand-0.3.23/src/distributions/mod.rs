// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Sampling from random distributions.
//!
//! This is a generalization of `Rand` to allow parameters to control the
//! exact properties of the generated values, e.g. the mean and standard
//! deviation of a normal distribution. The `Sample` trait is the most
//! general, and allows for generating values that change some state
//! internally. The `IndependentSample` trait is for generating values
//! that do not need to record state.

pub use rand4::distributions::Range;
pub use rand4::distributions::{Gamma, ChiSquared, FisherF, StudentT};
pub use rand4::distributions::{Normal, LogNormal};
pub use rand4::distributions::Exp;

pub use rand4::distributions::{range, gamma, normal, exponential};

pub use rand4::distributions::{Sample, IndependentSample, RandSample};
pub use rand4::distributions::{Weighted, WeightedChoice};

#[cfg(test)]
mod tests {

    use {Rng, Rand};
    use super::{RandSample, WeightedChoice, Weighted, Sample, IndependentSample};

    #[derive(PartialEq, Debug)]
    struct ConstRand(usize);
    impl Rand for ConstRand {
        fn rand<R: Rng>(_: &mut R) -> ConstRand {
            ConstRand(0)
        }
    }

    // 0, 1, 2, 3, ...
    struct CountingRng { i: u32 }
    impl Rng for CountingRng {
        fn next_u32(&mut self) -> u32 {
            self.i += 1;
            self.i - 1
        }
        fn next_u64(&mut self) -> u64 {
            self.next_u32() as u64
        }
    }

    #[test]
    fn test_rand_sample() {
        let mut rand_sample = RandSample::<ConstRand>::new();

        assert_eq!(rand_sample.sample(&mut ::test::rng()), ConstRand(0));
        assert_eq!(rand_sample.ind_sample(&mut ::test::rng()), ConstRand(0));
    }
    #[test]
    fn test_weighted_choice() {
        // this makes assumptions about the internal implementation of
        // WeightedChoice, specifically: it doesn't reorder the items,
        // it doesn't do weird things to the RNG (so 0 maps to 0, 1 to
        // 1, internally; modulo a modulo operation).

        macro_rules! t {
            ($items:expr, $expected:expr) => {{
                let mut items = $items;
                let wc = WeightedChoice::new(&mut items);
                let expected = $expected;

                let mut rng = CountingRng { i: 0 };

                for &val in expected.iter() {
                    assert_eq!(wc.ind_sample(&mut rng), val)
                }
            }}
        }

        t!(vec!(Weighted { weight: 1, item: 10}), [10]);

        // skip some
        t!(vec!(Weighted { weight: 0, item: 20},
                Weighted { weight: 2, item: 21},
                Weighted { weight: 0, item: 22},
                Weighted { weight: 1, item: 23}),
           [21,21, 23]);

        // different weights
        t!(vec!(Weighted { weight: 4, item: 30},
                Weighted { weight: 3, item: 31}),
           [30,30,30,30, 31,31,31]);

        // check that we're binary searching
        // correctly with some vectors of odd
        // length.
        t!(vec!(Weighted { weight: 1, item: 40},
                Weighted { weight: 1, item: 41},
                Weighted { weight: 1, item: 42},
                Weighted { weight: 1, item: 43},
                Weighted { weight: 1, item: 44}),
           [40, 41, 42, 43, 44]);
        t!(vec!(Weighted { weight: 1, item: 50},
                Weighted { weight: 1, item: 51},
                Weighted { weight: 1, item: 52},
                Weighted { weight: 1, item: 53},
                Weighted { weight: 1, item: 54},
                Weighted { weight: 1, item: 55},
                Weighted { weight: 1, item: 56}),
           [50, 51, 52, 53, 54, 55, 56]);
    }

    #[test]
    fn test_weighted_clone_initialization() {
        let initial : Weighted<u32> = Weighted {weight: 1, item: 1};
        let clone = initial.clone();
        assert_eq!(initial.weight, clone.weight);
        assert_eq!(initial.item, clone.item);
    }

    #[test] #[should_panic]
    fn test_weighted_clone_change_weight() {
        let initial : Weighted<u32> = Weighted {weight: 1, item: 1};
        let mut clone = initial.clone();
        clone.weight = 5;
        assert_eq!(initial.weight, clone.weight);
    }

    #[test] #[should_panic]
    fn test_weighted_clone_change_item() {
        let initial : Weighted<u32> = Weighted {weight: 1, item: 1};
        let mut clone = initial.clone();
        clone.item = 5;
        assert_eq!(initial.item, clone.item);

    }

    #[test] #[should_panic]
    fn test_weighted_choice_no_items() {
        WeightedChoice::<isize>::new(&mut []);
    }
    #[test] #[should_panic]
    fn test_weighted_choice_zero_weight() {
        WeightedChoice::new(&mut [Weighted { weight: 0, item: 0},
                                  Weighted { weight: 0, item: 1}]);
    }
    #[test] #[should_panic]
    fn test_weighted_choice_weight_overflows() {
        let x = ::std::u32::MAX / 2; // x + x + 2 is the overflow
        WeightedChoice::new(&mut [Weighted { weight: x, item: 0 },
                                  Weighted { weight: 1, item: 1 },
                                  Weighted { weight: x, item: 2 },
                                  Weighted { weight: 1, item: 3 }]);
    }
}
