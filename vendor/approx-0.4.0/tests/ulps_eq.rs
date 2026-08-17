// Copyright 2015 Brendan Zabarauskas
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Test cases derived from:
// https://github.com/Pybonacci/puntoflotante.org/blob/master/content/errors/NearlyEqualsTest.java

#[macro_use]
extern crate approx;

mod test_f32 {
    use std::f32;

    #[test]
    fn test_basic() {
        assert_ulps_eq!(1.0f32, 1.0f32);
        assert_ulps_ne!(1.0f32, 2.0f32);
    }

    #[test]
    #[should_panic]
    fn test_basic_panic_eq() {
        assert_ulps_eq!(1.0f32, 2.0f32);
    }

    #[test]
    #[should_panic]
    fn test_basic_panic_ne() {
        assert_ulps_ne!(1.0f32, 1.0f32);
    }

    #[test]
    fn test_big() {
        assert_ulps_eq!(100000000.0f32, 100000001.0f32);
        assert_ulps_eq!(100000001.0f32, 100000000.0f32);
        assert_ulps_ne!(10000.0f32, 10001.0f32);
        assert_ulps_ne!(10001.0f32, 10000.0f32);
    }

    #[test]
    fn test_big_neg() {
        assert_ulps_eq!(-100000000.0f32, -100000001.0f32);
        assert_ulps_eq!(-100000001.0f32, -100000000.0f32);
        assert_ulps_ne!(-10000.0f32, -10001.0f32);
        assert_ulps_ne!(-10001.0f32, -10000.0f32);
    }

    #[test]
    fn test_mid() {
        assert_ulps_eq!(1.0000001f32, 1.0000002f32);
        assert_ulps_eq!(1.0000002f32, 1.0000001f32);
        assert_ulps_ne!(1.000001f32, 1.000002f32);
        assert_ulps_ne!(1.000002f32, 1.000001f32);
    }

    #[test]
    fn test_mid_neg() {
        assert_ulps_eq!(-1.0000001f32, -1.0000002f32);
        assert_ulps_eq!(-1.0000002f32, -1.0000001f32);
        assert_ulps_ne!(-1.000001f32, -1.000002f32);
        assert_ulps_ne!(-1.000002f32, -1.000001f32);
    }

    #[test]
    fn test_small() {
        assert_ulps_eq!(0.000010001f32, 0.000010002f32);
        assert_ulps_eq!(0.000010002f32, 0.000010001f32);
        assert_ulps_ne!(0.000001002f32, 0.0000001001f32);
        assert_ulps_ne!(0.000001001f32, 0.0000001002f32);
    }

    #[test]
    fn test_small_neg() {
        assert_ulps_eq!(-0.000010001f32, -0.000010002f32);
        assert_ulps_eq!(-0.000010002f32, -0.000010001f32);
        assert_ulps_ne!(-0.000001002f32, -0.0000001001f32);
        assert_ulps_ne!(-0.000001001f32, -0.0000001002f32);
    }

    #[test]
    fn test_zero() {
        assert_ulps_eq!(0.0f32, 0.0f32);
        assert_ulps_eq!(0.0f32, -0.0f32);
        assert_ulps_eq!(-0.0f32, -0.0f32);

        assert_ulps_ne!(0.000001f32, 0.0f32);
        assert_ulps_ne!(0.0f32, 0.000001f32);
        assert_ulps_ne!(-0.000001f32, 0.0f32);
        assert_ulps_ne!(0.0f32, -0.000001f32);
    }

    #[test]
    fn test_epsilon() {
        assert_ulps_eq!(0.0f32, 1e-40f32, epsilon = 1e-40f32);
        assert_ulps_eq!(1e-40f32, 0.0f32, epsilon = 1e-40f32);
        assert_ulps_eq!(0.0f32, -1e-40f32, epsilon = 1e-40f32);
        assert_ulps_eq!(-1e-40f32, 0.0f32, epsilon = 1e-40f32);

        assert_ulps_ne!(1e-40f32, 0.0f32, epsilon = 1e-41f32);
        assert_ulps_ne!(0.0f32, 1e-40f32, epsilon = 1e-41f32);
        assert_ulps_ne!(-1e-40f32, 0.0f32, epsilon = 1e-41f32);
        assert_ulps_ne!(0.0f32, -1e-40f32, epsilon = 1e-41f32);
    }

    #[test]
    fn test_max() {
        assert_ulps_eq!(f32::MAX, f32::MAX);
        assert_ulps_ne!(f32::MAX, -f32::MAX);
        assert_ulps_ne!(-f32::MAX, f32::MAX);
        assert_ulps_ne!(f32::MAX, f32::MAX / 2.0);
        assert_ulps_ne!(f32::MAX, -f32::MAX / 2.0);
        assert_ulps_ne!(-f32::MAX, f32::MAX / 2.0);
    }

    #[test]
    fn test_infinity() {
        assert_ulps_eq!(f32::INFINITY, f32::INFINITY);
        assert_ulps_eq!(f32::NEG_INFINITY, f32::NEG_INFINITY);
        assert_ulps_ne!(f32::NEG_INFINITY, f32::INFINITY);
        assert_ulps_eq!(f32::INFINITY, f32::MAX);
        assert_ulps_eq!(f32::NEG_INFINITY, -f32::MAX);
    }

    #[test]
    fn test_nan() {
        assert_ulps_ne!(f32::NAN, f32::NAN);

        assert_ulps_ne!(f32::NAN, 0.0);
        assert_ulps_ne!(-0.0, f32::NAN);
        assert_ulps_ne!(f32::NAN, -0.0);
        assert_ulps_ne!(0.0, f32::NAN);

        assert_ulps_ne!(f32::NAN, f32::INFINITY);
        assert_ulps_ne!(f32::INFINITY, f32::NAN);
        assert_ulps_ne!(f32::NAN, f32::NEG_INFINITY);
        assert_ulps_ne!(f32::NEG_INFINITY, f32::NAN);

        assert_ulps_ne!(f32::NAN, f32::MAX);
        assert_ulps_ne!(f32::MAX, f32::NAN);
        assert_ulps_ne!(f32::NAN, -f32::MAX);
        assert_ulps_ne!(-f32::MAX, f32::NAN);

        assert_ulps_ne!(f32::NAN, f32::MIN_POSITIVE);
        assert_ulps_ne!(f32::MIN_POSITIVE, f32::NAN);
        assert_ulps_ne!(f32::NAN, -f32::MIN_POSITIVE);
        assert_ulps_ne!(-f32::MIN_POSITIVE, f32::NAN);
    }

    #[test]
    fn test_opposite_signs() {
        assert_ulps_ne!(1.000000001f32, -1.0f32);
        assert_ulps_ne!(-1.0f32, 1.000000001f32);
        assert_ulps_ne!(-1.000000001f32, 1.0f32);
        assert_ulps_ne!(1.0f32, -1.000000001f32);

        assert_ulps_eq!(10.0 * f32::MIN_POSITIVE, 10.0 * -f32::MIN_POSITIVE);
    }

    #[test]
    fn test_close_to_zero() {
        assert_ulps_eq!(f32::MIN_POSITIVE, f32::MIN_POSITIVE);
        assert_ulps_eq!(f32::MIN_POSITIVE, -f32::MIN_POSITIVE);
        assert_ulps_eq!(-f32::MIN_POSITIVE, f32::MIN_POSITIVE);

        assert_ulps_eq!(f32::MIN_POSITIVE, 0.0f32);
        assert_ulps_eq!(0.0f32, f32::MIN_POSITIVE);
        assert_ulps_eq!(-f32::MIN_POSITIVE, 0.0f32);
        assert_ulps_eq!(0.0f32, -f32::MIN_POSITIVE);

        assert_ulps_ne!(0.000001f32, -f32::MIN_POSITIVE);
        assert_ulps_ne!(0.000001f32, f32::MIN_POSITIVE);
        assert_ulps_ne!(f32::MIN_POSITIVE, 0.000001f32);
        assert_ulps_ne!(-f32::MIN_POSITIVE, 0.000001f32);
    }
}

#[cfg(test)]
mod test_f64 {
    use std::f64;

    #[test]
    fn test_basic() {
        assert_ulps_eq!(1.0f64, 1.0f64);
        assert_ulps_ne!(1.0f64, 2.0f64);
    }

    #[test]
    #[should_panic]
    fn test_basic_panic_eq() {
        assert_ulps_eq!(1.0f64, 2.0f64);
    }

    #[test]
    #[should_panic]
    fn test_basic_panic_ne() {
        assert_ulps_ne!(1.0f64, 1.0f64);
    }

    #[test]
    fn test_big() {
        assert_ulps_eq!(10000000000000000.0f64, 10000000000000001.0f64);
        assert_ulps_eq!(10000000000000001.0f64, 10000000000000000.0f64);
        assert_ulps_ne!(1000000000000000.0f64, 1000000000000001.0f64);
        assert_ulps_ne!(1000000000000001.0f64, 1000000000000000.0f64);
    }

    #[test]
    fn test_big_neg() {
        assert_ulps_eq!(-10000000000000000.0f64, -10000000000000001.0f64);
        assert_ulps_eq!(-10000000000000001.0f64, -10000000000000000.0f64);
        assert_ulps_ne!(-1000000000000000.0f64, -1000000000000001.0f64);
        assert_ulps_ne!(-1000000000000001.0f64, -1000000000000000.0f64);
    }

    #[test]
    fn test_mid() {
        assert_ulps_eq!(1.0000000000000001f64, 1.0000000000000002f64);
        assert_ulps_eq!(1.0000000000000002f64, 1.0000000000000001f64);
        assert_ulps_ne!(1.000000000000001f64, 1.0000000000000022f64);
        assert_ulps_ne!(1.0000000000000022f64, 1.000000000000001f64);
    }

    #[test]
    fn test_mid_neg() {
        assert_ulps_eq!(-1.0000000000000001f64, -1.0000000000000002f64);
        assert_ulps_eq!(-1.0000000000000002f64, -1.0000000000000001f64);
        assert_ulps_ne!(-1.000000000000001f64, -1.0000000000000022f64);
        assert_ulps_ne!(-1.0000000000000022f64, -1.000000000000001f64);
    }

    #[test]
    fn test_small() {
        assert_ulps_eq!(0.0000000100000001f64, 0.0000000100000002f64);
        assert_ulps_eq!(0.0000000100000002f64, 0.0000000100000001f64);
        assert_ulps_ne!(0.0000000100000001f64, 0.0000000010000002f64);
        assert_ulps_ne!(0.0000000100000002f64, 0.0000000010000001f64);
    }

    #[test]
    fn test_small_neg() {
        assert_ulps_eq!(-0.0000000100000001f64, -0.0000000100000002f64);
        assert_ulps_eq!(-0.0000000100000002f64, -0.0000000100000001f64);
        assert_ulps_ne!(-0.0000000100000001f64, -0.0000000010000002f64);
        assert_ulps_ne!(-0.0000000100000002f64, -0.0000000010000001f64);
    }

    #[test]
    fn test_zero() {
        assert_ulps_eq!(0.0f64, 0.0f64);
        assert_ulps_eq!(0.0f64, -0.0f64);
        assert_ulps_eq!(-0.0f64, -0.0f64);

        assert_ulps_ne!(0.000000000000001f64, 0.0f64);
        assert_ulps_ne!(0.0f64, 0.000000000000001f64);
        assert_ulps_ne!(-0.000000000000001f64, 0.0f64);
        assert_ulps_ne!(0.0f64, -0.000000000000001f64);
    }

    #[test]
    fn test_epsilon() {
        assert_ulps_eq!(0.0f64, 1e-40f64, epsilon = 1e-40f64);
        assert_ulps_eq!(1e-40f64, 0.0f64, epsilon = 1e-40f64);
        assert_ulps_eq!(0.0f64, -1e-40f64, epsilon = 1e-40f64);
        assert_ulps_eq!(-1e-40f64, 0.0f64, epsilon = 1e-40f64);

        assert_ulps_ne!(1e-40f64, 0.0f64, epsilon = 1e-41f64);
        assert_ulps_ne!(0.0f64, 1e-40f64, epsilon = 1e-41f64);
        assert_ulps_ne!(-1e-40f64, 0.0f64, epsilon = 1e-41f64);
        assert_ulps_ne!(0.0f64, -1e-40f64, epsilon = 1e-41f64);
    }

    #[test]
    fn test_max() {
        assert_ulps_eq!(f64::MAX, f64::MAX);
        assert_ulps_ne!(f64::MAX, -f64::MAX);
        assert_ulps_ne!(-f64::MAX, f64::MAX);
        assert_ulps_ne!(f64::MAX, f64::MAX / 2.0);
        assert_ulps_ne!(f64::MAX, -f64::MAX / 2.0);
        assert_ulps_ne!(-f64::MAX, f64::MAX / 2.0);
    }

    #[test]
    fn test_infinity() {
        assert_ulps_eq!(f64::INFINITY, f64::INFINITY);
        assert_ulps_eq!(f64::NEG_INFINITY, f64::NEG_INFINITY);
        assert_ulps_ne!(f64::NEG_INFINITY, f64::INFINITY);
        assert_ulps_eq!(f64::INFINITY, f64::MAX);
        assert_ulps_eq!(f64::NEG_INFINITY, -f64::MAX);
    }

    #[test]
    fn test_nan() {
        assert_ulps_ne!(f64::NAN, f64::NAN);

        assert_ulps_ne!(f64::NAN, 0.0);
        assert_ulps_ne!(-0.0, f64::NAN);
        assert_ulps_ne!(f64::NAN, -0.0);
        assert_ulps_ne!(0.0, f64::NAN);

        assert_ulps_ne!(f64::NAN, f64::INFINITY);
        assert_ulps_ne!(f64::INFINITY, f64::NAN);
        assert_ulps_ne!(f64::NAN, f64::NEG_INFINITY);
        assert_ulps_ne!(f64::NEG_INFINITY, f64::NAN);

        assert_ulps_ne!(f64::NAN, f64::MAX);
        assert_ulps_ne!(f64::MAX, f64::NAN);
        assert_ulps_ne!(f64::NAN, -f64::MAX);
        assert_ulps_ne!(-f64::MAX, f64::NAN);

        assert_ulps_ne!(f64::NAN, f64::MIN_POSITIVE);
        assert_ulps_ne!(f64::MIN_POSITIVE, f64::NAN);
        assert_ulps_ne!(f64::NAN, -f64::MIN_POSITIVE);
        assert_ulps_ne!(-f64::MIN_POSITIVE, f64::NAN);
    }

    #[test]
    fn test_opposite_signs() {
        assert_ulps_ne!(1.000000001f64, -1.0f64);
        assert_ulps_ne!(-1.0f64, 1.000000001f64);
        assert_ulps_ne!(-1.000000001f64, 1.0f64);
        assert_ulps_ne!(1.0f64, -1.000000001f64);

        assert_ulps_eq!(10.0 * f64::MIN_POSITIVE, 10.0 * -f64::MIN_POSITIVE);
    }

    #[test]
    fn test_close_to_zero() {
        assert_ulps_eq!(f64::MIN_POSITIVE, f64::MIN_POSITIVE);
        assert_ulps_eq!(f64::MIN_POSITIVE, -f64::MIN_POSITIVE);
        assert_ulps_eq!(-f64::MIN_POSITIVE, f64::MIN_POSITIVE);

        assert_ulps_eq!(f64::MIN_POSITIVE, 0.0f64);
        assert_ulps_eq!(0.0f64, f64::MIN_POSITIVE);
        assert_ulps_eq!(-f64::MIN_POSITIVE, 0.0f64);
        assert_ulps_eq!(0.0f64, -f64::MIN_POSITIVE);

        assert_ulps_ne!(0.000000000000001f64, -f64::MIN_POSITIVE);
        assert_ulps_ne!(0.000000000000001f64, f64::MIN_POSITIVE);
        assert_ulps_ne!(f64::MIN_POSITIVE, 0.000000000000001f64);
        assert_ulps_ne!(-f64::MIN_POSITIVE, 0.000000000000001f64);
    }
}

mod test_ref {
    mod test_f32 {
        #[test]
        fn test_basic() {
            assert_ulps_eq!(&1.0f32, &1.0f32);
            assert_ulps_ne!(&1.0f32, &2.0f32);
        }
    }

    mod test_f64 {
        #[test]
        fn test_basic() {
            assert_ulps_eq!(&1.0f64, &1.0f64);
            assert_ulps_ne!(&1.0f64, &2.0f64);
        }
    }
}

mod test_slice {
    mod test_f32 {
        #[test]
        fn test_basic() {
            assert_ulps_eq!([1.0f32, 2.0f32][..], [1.0f32, 2.0f32][..]);
            assert_ulps_ne!([1.0f32, 2.0f32][..], [2.0f32, 1.0f32][..]);
        }
    }

    mod test_f64 {
        #[test]
        fn test_basic() {
            assert_ulps_eq!([1.0f64, 2.0f64][..], [1.0f64, 2.0f64][..]);
            assert_ulps_ne!([1.0f64, 2.0f64][..], [2.0f64, 1.0f64][..]);
        }
    }
}

#[cfg(feature = "num-complex")]
mod test_complex {
    extern crate num_complex;
    pub use self::num_complex::Complex;

    mod test_f32 {
        use super::Complex;

        #[test]
        fn test_basic() {
            assert_ulps_eq!(Complex::new(1.0f32, 2.0f32), Complex::new(1.0f32, 2.0f32));
            assert_ulps_ne!(Complex::new(1.0f32, 2.0f32), Complex::new(2.0f32, 1.0f32));
        }

        #[test]
        #[should_panic]
        fn test_basic_panic_eq() {
            assert_ulps_eq!(Complex::new(1.0f32, 2.0f32), Complex::new(2.0f32, 1.0f32));
        }

        #[test]
        #[should_panic]
        fn test_basic_panic_ne() {
            assert_ulps_ne!(Complex::new(1.0f32, 2.0f32), Complex::new(1.0f32, 2.0f32));
        }
    }

    mod test_f64 {
        use super::Complex;

        #[test]
        fn test_basic() {
            assert_ulps_eq!(Complex::new(1.0f64, 2.0f64), Complex::new(1.0f64, 2.0f64));
            assert_ulps_ne!(Complex::new(1.0f64, 2.0f64), Complex::new(2.0f64, 1.0f64));
        }

        #[test]
        #[should_panic]
        fn test_basic_panic_eq() {
            assert_ulps_eq!(Complex::new(1.0f64, 2.0f64), Complex::new(2.0f64, 1.0f64));
        }

        #[test]
        #[should_panic]
        fn test_basic_panic_ne() {
            assert_ulps_ne!(Complex::new(1.0f64, 2.0f64), Complex::new(1.0f64, 2.0f64));
        }
    }
}
