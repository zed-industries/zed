// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Trait for testing approximate equality
pub trait ApproxEq<Eps> {
    /// Default epsilon value
    fn approx_epsilon() -> Eps;

    /// Returns `true` if this object is approximately equal to the other one, using
    /// a provided epsilon value.
    fn approx_eq_eps(&self, other: &Self, approx_epsilon: &Eps) -> bool;

    /// Returns `true` if this object is approximately equal to the other one, using
    /// the [`approx_epsilon`](ApproxEq::approx_epsilon) epsilon value.
    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_eps(other, &Self::approx_epsilon())
    }
}

macro_rules! approx_eq {
    ($ty:ty, $eps:expr) => {
        impl ApproxEq<$ty> for $ty {
            #[inline]
            fn approx_epsilon() -> $ty {
                $eps
            }
            #[inline]
            fn approx_eq_eps(&self, other: &$ty, approx_epsilon: &$ty) -> bool {
                num_traits::Float::abs(*self - *other) < *approx_epsilon
            }
        }
    };
}

approx_eq!(f32, 1.0e-6);
approx_eq!(f64, 1.0e-6);
