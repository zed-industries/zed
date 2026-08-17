//! This module exercises the `ff_derive` procedural macros, to ensure that changes to the
//! `ff` crate are reflected in `ff_derive`. It also uses the resulting field to test some
//! of the APIs provided by `ff`, such as batch inversion.

#[macro_use]
extern crate ff;

/// The BLS12-381 scalar field.
#[derive(PrimeField)]
#[PrimeFieldModulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
struct Bls381K12Scalar([u64; 4]);

mod fermat {
    /// The largest known Fermat prime, used to test the case `t = 1`.
    #[derive(PrimeField)]
    #[PrimeFieldModulus = "65537"]
    #[PrimeFieldGenerator = "3"]
    #[PrimeFieldReprEndianness = "little"]
    struct Fermat65537Field([u64; 1]);
}

mod full_limbs {
    #[derive(PrimeField)]
    #[PrimeFieldModulus = "39402006196394479212279040100143613805079739270465446667948293404245721771496870329047266088258938001861606973112319"]
    #[PrimeFieldGenerator = "19"]
    #[PrimeFieldReprEndianness = "little"]
    struct F384p([u64; 7]);

    #[test]
    fn random_masking_does_not_overflow() {
        use ff::Field;
        use rand::rngs::OsRng;

        let _ = F384p::random(OsRng);
    }
}

#[test]
fn batch_inversion() {
    use ff::{BatchInverter, Field};

    let one = Bls381K12Scalar::one();

    // [1, 2, 3, 4]
    let values: Vec<_> = (0..4)
        .scan(one, |acc, _| {
            let ret = *acc;
            *acc += &one;
            Some(ret)
        })
        .collect();

    // Test BatchInverter::invert_with_external_scratch
    {
        let mut elements = values.clone();
        let mut scratch_space = vec![Bls381K12Scalar::zero(); elements.len()];
        BatchInverter::invert_with_external_scratch(&mut elements, &mut scratch_space);
        for (a, a_inv) in values.iter().zip(elements.into_iter()) {
            assert_eq!(*a * a_inv, one);
        }
    }

    // Test BatchInverter::invert_with_internal_scratch
    {
        let mut items: Vec<_> = values.iter().cloned().map(|p| (p, one)).collect();
        BatchInverter::invert_with_internal_scratch(
            &mut items,
            |item| &mut item.0,
            |item| &mut item.1,
        );
        for (a, (a_inv, _)) in values.iter().zip(items.into_iter()) {
            assert_eq!(*a * a_inv, one);
        }
    }

    // Test BatchInvert trait
    #[cfg(feature = "alloc")]
    {
        use ff::BatchInvert;
        let mut elements = values.clone();
        elements.iter_mut().batch_invert();
        for (a, a_inv) in values.iter().zip(elements.into_iter()) {
            assert_eq!(*a * a_inv, one);
        }
    }
}
