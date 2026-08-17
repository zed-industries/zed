use crate::{
    approx::sqrt::RawApprox, fraction::approx::Accuracy, generic::GenericInteger, GenericDecimal,
};
use num::{
    bigint::{ToBigInt, ToBigUint},
    BigUint, Integer,
};
use std::borrow::Borrow;

impl<
        T: Clone + Integer + ToBigUint + ToBigInt + GenericInteger,
        P: Copy + Integer + Into<usize>,
    > GenericDecimal<T, P>
{
    /// See `GenericFraction::sqrt_abs_with_accuracy_raw`.
    pub fn sqrt_abs_with_accuracy_raw(&self, accuracy: impl Borrow<Accuracy>) -> RawApprox {
        self.0.sqrt_abs_with_accuracy_raw(accuracy)
    }

    /// See `GenericFraction::sqrt_abs_with_accuracy`.
    pub fn sqrt_abs_with_accuracy(
        &self,
        accuracy: impl Borrow<Accuracy>,
    ) -> GenericDecimal<BigUint, P> {
        GenericDecimal(self.0.sqrt_abs_with_accuracy(accuracy), self.1)
    }

    /// See `GenericFraction::sqrt_abs_raw`.
    pub fn sqrt_abs_raw(&self, decimal_places: u32) -> RawApprox {
        self.0.sqrt_abs_raw(decimal_places)
    }

    /// See `GenericFraction::sqrt_abs`.
    pub fn sqrt_abs(&self, decimal_places: u32) -> GenericDecimal<BigUint, P> {
        GenericDecimal(self.0.sqrt_abs(decimal_places), self.1)
    }

    /// See `GenericFraction::sqrt_with_accuracy_raw`.
    pub fn sqrt_with_accuracy_raw(&self, accuracy: impl Borrow<Accuracy>) -> RawApprox {
        self.0.sqrt_with_accuracy_raw(accuracy)
    }

    /// See `GenericFraction::sqrt_with_accuracy`.
    pub fn sqrt_with_accuracy(
        &self,
        accuracy: impl Borrow<Accuracy>,
    ) -> GenericDecimal<BigUint, P> {
        GenericDecimal(self.0.sqrt_with_accuracy(accuracy), self.1)
    }

    /// See `GenericFraction::sqrt_raw`.
    pub fn sqrt_raw(&self, decimal_places: u32) -> RawApprox {
        self.0.sqrt_raw(decimal_places)
    }

    /// See `GenericFraction::sqrt`.
    pub fn sqrt(&self, decimal_places: u32) -> GenericDecimal<BigUint, P> {
        GenericDecimal(self.0.sqrt(decimal_places), self.1)
    }
}
