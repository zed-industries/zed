use crate::Decimal;
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, UniformInt, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

impl Distribution<Decimal> for Standard {
    fn sample<R>(&self, rng: &mut R) -> Decimal
    where
        R: Rng + ?Sized,
    {
        Decimal::from_parts(
            rng.next_u32(),
            rng.next_u32(),
            rng.next_u32(),
            rng.gen(),
            rng.gen_range(0..=Decimal::MAX_SCALE),
        )
    }
}

impl SampleUniform for Decimal {
    type Sampler = DecimalSampler;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecimalSampler {
    mantissa_sampler: UniformInt<i128>,
    scale: u32,
}

impl UniformSampler for DecimalSampler {
    type X = Decimal;

    /// Creates a new sampler that will yield random decimal objects between `low` and `high`.
    ///
    /// The sampler will always provide decimals at the same scale as the inputs; if the inputs
    /// have different scales, the higher scale is used.
    ///
    /// # Example
    ///
    /// ```
    /// # use rand::Rng;
    /// # use rust_decimal::dec;
    /// let mut rng = rand::rngs::OsRng;
    /// let random = rng.gen_range(dec!(1.00)..dec!(2.00));
    /// assert!(random >= dec!(1.00));
    /// assert!(random < dec!(2.00));
    /// assert_eq!(random.scale(), 2);
    /// ```
    #[inline]
    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let (low, high) = sync_scales(*low.borrow(), *high.borrow());
        let high = Decimal::from_i128_with_scale(high.mantissa() - 1, high.scale());
        UniformSampler::new_inclusive(low, high)
    }

    /// Creates a new sampler that will yield random decimal objects between `low` and `high`.
    ///
    /// The sampler will always provide decimals at the same scale as the inputs; if the inputs
    /// have different scales, the higher scale is used.
    ///
    /// # Example
    ///
    /// ```
    /// # use rand::Rng;
    /// # use rust_decimal::dec;
    /// let mut rng = rand::rngs::OsRng;
    /// let random = rng.gen_range(dec!(1.00)..=dec!(2.00));
    /// assert!(random >= dec!(1.00));
    /// assert!(random <= dec!(2.00));
    /// assert_eq!(random.scale(), 2);
    /// ```
    #[inline]
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let (low, high) = sync_scales(*low.borrow(), *high.borrow());

        // Return our sampler, which contains an underlying i128 sampler so we
        // outsource the actual randomness implementation.
        Self {
            mantissa_sampler: UniformInt::new_inclusive(low.mantissa(), high.mantissa()),
            scale: low.scale(),
        }
    }

    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        let mantissa = self.mantissa_sampler.sample(rng);
        Decimal::from_i128_with_scale(mantissa, self.scale)
    }
}

/// Return equivalent Decimal objects with the same scale as one another.
#[inline]
fn sync_scales(mut a: Decimal, mut b: Decimal) -> (Decimal, Decimal) {
    if a.scale() == b.scale() {
        return (a, b);
    }

    // Set scales to match one another, because we are relying on mantissas'
    // being comparable in order outsource the actual sampling implementation.
    a.rescale(a.scale().max(b.scale()));
    b.rescale(a.scale().max(b.scale()));

    // Edge case: If the values have _wildly_ different scales, the values may not have rescaled far enough to match one another.
    //
    // In this case, we accept some precision loss because the randomization approach we are using assumes that the scales will necessarily match.
    if a.scale() != b.scale() {
        a.rescale(a.scale().min(b.scale()));
        b.rescale(a.scale().min(b.scale()));
    }

    (a, b)
}

#[cfg(test)]
mod rand_tests {
    use std::collections::HashSet;

    use super::*;

    macro_rules! dec {
        ($e:expr) => {
            Decimal::from_str_exact(stringify!($e)).unwrap()
        };
    }

    #[test]
    fn has_random_decimal_instances() {
        let mut rng = rand::rngs::OsRng;
        let random: [Decimal; 32] = rng.gen();
        assert!(random.windows(2).any(|slice| { slice[0] != slice[1] }));
    }

    #[test]
    fn generates_within_range() {
        let mut rng = rand::rngs::OsRng;
        for _ in 0..128 {
            let random = rng.gen_range(dec!(1.00)..dec!(1.05));
            assert!(random < dec!(1.05));
            assert!(random >= dec!(1.00));
        }
    }

    #[test]
    fn generates_within_inclusive_range() {
        let mut rng = rand::rngs::OsRng;
        let mut values: HashSet<Decimal> = HashSet::new();
        for _ in 0..256 {
            let random = rng.gen_range(dec!(1.00)..=dec!(1.01));
            // The scale is 2, so 1.00 and 1.01 are the only two valid choices.
            assert!(random == dec!(1.00) || random == dec!(1.01));
            values.insert(random);
        }
        // Somewhat flaky, will fail 1 out of every 2^255 times this is run.
        // Probably acceptable in the real world.
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn test_edge_case_scales_match() {
        let (low, high) = sync_scales(dec!(1.000_000_000_000_000_000_01), dec!(100_000_000_000_000_000_001));
        assert_eq!(low.scale(), high.scale());
    }
}
