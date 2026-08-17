use core::ops::{Add, Mul, Sub};

use crate::{num::One, Oklab};

impl_rand_traits_cartesian!(
    UniformOklab,
    Oklab {
        l,
        a => [|x| x  * (T::one() + T::one()) - T::one()],
        b => [|x| x  * (T::one() + T::one()) - T::one()]
    }
    where T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + One
);
