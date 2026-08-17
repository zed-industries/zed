use alloc::vec::Vec;
use core::ops::{Mul, Neg};
use ff::{Field, PrimeField};
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

use crate::{
    prime::{PrimeCurve, PrimeCurveAffine},
    wnaf::WnafGroup,
    GroupEncoding, UncompressedEncoding,
};

pub fn curve_tests<G: PrimeCurve>() {
    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    // Negation edge case with identity.
    {
        let z = G::identity().neg();
        assert!(bool::from(z.is_identity()));
    }

    // Doubling edge case with identity.
    {
        let z = G::identity().double();
        assert!(bool::from(z.is_identity()));
    }

    // Addition edge cases with identity
    {
        let mut r = G::random(&mut rng);
        let rcopy = r;
        r.add_assign(&G::identity());
        assert_eq!(r, rcopy);
        r.add_assign(&G::Affine::identity());
        assert_eq!(r, rcopy);

        let mut z = G::identity();
        z.add_assign(&G::identity());
        assert!(bool::from(z.is_identity()));
        z.add_assign(&G::Affine::identity());
        assert!(bool::from(z.is_identity()));

        let mut z2 = z;
        z2.add_assign(&r);

        z.add_assign(&r.to_affine());

        assert_eq!(z, z2);
        assert_eq!(z, r);
    }

    // Transformations
    {
        let a = G::random(&mut rng);
        let b = a.to_affine().to_curve();
        let c = a.to_affine().to_curve().to_affine().to_curve();
        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    random_addition_tests::<G>();
    random_multiplication_tests::<G>();
    random_doubling_tests::<G>();
    random_negation_tests::<G>();
    random_transformation_tests::<G>();
    random_compressed_encoding_tests::<G>();
}

pub fn random_wnaf_tests<G: WnafGroup>() {
    use crate::wnaf::*;

    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    {
        let mut table = vec![];
        let mut wnaf = vec![];

        for w in 2..14 {
            for _ in 0..100 {
                let g = G::random(&mut rng);
                let s = G::Scalar::random(&mut rng);
                let mut g1 = g;
                g1.mul_assign(s);

                wnaf_table(&mut table, g, w);
                wnaf_form(&mut wnaf, s.to_repr(), w);
                let g2 = wnaf_exp(&table, &wnaf);

                assert_eq!(g1, g2);
            }
        }
    }

    {
        fn only_compiles_if_send<S: Send>(_: &S) {}

        for _ in 0..100 {
            let g = G::random(&mut rng);
            let s = G::Scalar::random(&mut rng);
            let mut g1 = g;
            g1.mul_assign(s);

            let g2 = {
                let mut wnaf = Wnaf::new();
                wnaf.base(g, 1).scalar(&s)
            };
            let g3 = {
                let mut wnaf = Wnaf::new();
                wnaf.scalar(&s).base(g)
            };
            let g4 = {
                let mut wnaf = Wnaf::new();
                let mut shared = wnaf.base(g, 1).shared();

                only_compiles_if_send(&shared);

                shared.scalar(&s)
            };
            let g5 = {
                let mut wnaf = Wnaf::new();
                let mut shared = wnaf.scalar(&s).shared();

                only_compiles_if_send(&shared);

                shared.base(g)
            };

            let g6 = {
                let mut wnaf = Wnaf::new();
                {
                    // Populate the vectors.
                    wnaf.base(G::random(&mut rng), 1)
                        .scalar(&G::Scalar::random(&mut rng));
                }
                wnaf.base(g, 1).scalar(&s)
            };
            let g7 = {
                let mut wnaf = Wnaf::new();
                {
                    // Populate the vectors.
                    wnaf.base(G::random(&mut rng), 1)
                        .scalar(&G::Scalar::random(&mut rng));
                }
                wnaf.scalar(&s).base(g)
            };
            let g8 = {
                let mut wnaf = Wnaf::new();
                {
                    // Populate the vectors.
                    wnaf.base(G::random(&mut rng), 1)
                        .scalar(&G::Scalar::random(&mut rng));
                }
                let mut shared = wnaf.base(g, 1).shared();

                only_compiles_if_send(&shared);

                shared.scalar(&s)
            };
            let g9 = {
                let mut wnaf = Wnaf::new();
                {
                    // Populate the vectors.
                    wnaf.base(G::random(&mut rng), 1)
                        .scalar(&G::Scalar::random(&mut rng));
                }
                let mut shared = wnaf.scalar(&s).shared();

                only_compiles_if_send(&shared);

                shared.base(g)
            };

            assert_eq!(g1, g2);
            assert_eq!(g1, g3);
            assert_eq!(g1, g4);
            assert_eq!(g1, g5);
            assert_eq!(g1, g6);
            assert_eq!(g1, g7);
            assert_eq!(g1, g8);
            assert_eq!(g1, g9);
        }
    }
}

fn random_negation_tests<G: PrimeCurve>() {
    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    for _ in 0..1000 {
        let r = G::random(&mut rng);

        let s = G::Scalar::random(&mut rng);
        let sneg = s.neg();

        let mut t1 = r;
        t1.mul_assign(s);

        let mut t2 = r;
        t2.mul_assign(sneg);

        let mut t3 = t1;
        t3.add_assign(&t2);
        assert!(bool::from(t3.is_identity()));

        let mut t4 = t1;
        t4.add_assign(&t2.to_affine());
        assert!(bool::from(t4.is_identity()));

        assert_eq!(t1.neg(), t2);
    }
}

fn random_doubling_tests<G: PrimeCurve>() {
    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    for _ in 0..1000 {
        let mut a = G::random(&mut rng);
        let mut b = G::random(&mut rng);

        // 2(a + b)
        let tmp1 = (a + b).double();

        // 2a + 2b
        a = a.double();
        b = b.double();

        let mut tmp2 = a;
        tmp2.add_assign(&b);

        let mut tmp3 = a;
        tmp3.add_assign(&b.to_affine());

        assert_eq!(tmp1, tmp2);
        assert_eq!(tmp1, tmp3);
    }
}

fn random_multiplication_tests<G: PrimeCurve>() {
    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    for _ in 0..1000 {
        let mut a = G::random(&mut rng);
        let mut b = G::random(&mut rng);
        let a_affine = a.to_affine();
        let b_affine = b.to_affine();

        let s = G::Scalar::random(&mut rng);

        // s ( a + b )
        let mut tmp1 = a;
        tmp1.add_assign(&b);
        tmp1.mul_assign(s);

        // sa + sb
        a.mul_assign(s);
        b.mul_assign(s);

        let mut tmp2 = a;
        tmp2.add_assign(&b);

        // Affine multiplication
        let mut tmp3 = Mul::<G::Scalar>::mul(a_affine, s);
        tmp3.add_assign(Mul::<G::Scalar>::mul(b_affine, s));

        assert_eq!(tmp1, tmp2);
        assert_eq!(tmp1, tmp3);
    }
}

fn random_addition_tests<G: PrimeCurve>() {
    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    for _ in 0..1000 {
        let a = G::random(&mut rng);
        let b = G::random(&mut rng);
        let c = G::random(&mut rng);
        let a_affine = a.to_affine();
        let b_affine = b.to_affine();
        let c_affine = c.to_affine();

        // a + a should equal the doubling
        {
            let mut aplusa = a;
            aplusa.add_assign(&a);

            let mut aplusamixed = a;
            aplusamixed.add_assign(&a.to_affine());

            let adouble = a.double();

            assert_eq!(aplusa, adouble);
            assert_eq!(aplusa, aplusamixed);
        }

        let mut tmp = vec![G::identity(); 6];

        // (a + b) + c
        tmp[0] = a;
        tmp[0].add_assign(&b);
        tmp[0].add_assign(&c);

        // a + (b + c)
        tmp[1] = b;
        tmp[1].add_assign(&c);
        tmp[1].add_assign(&a);

        // (a + c) + b
        tmp[2] = a;
        tmp[2].add_assign(&c);
        tmp[2].add_assign(&b);

        // Mixed addition

        // (a + b) + c
        tmp[3] = a_affine.to_curve();
        tmp[3].add_assign(&b_affine);
        tmp[3].add_assign(&c_affine);

        // a + (b + c)
        tmp[4] = b_affine.to_curve();
        tmp[4].add_assign(&c_affine);
        tmp[4].add_assign(&a_affine);

        // (a + c) + b
        tmp[5] = a_affine.to_curve();
        tmp[5].add_assign(&c_affine);
        tmp[5].add_assign(&b_affine);

        // Comparisons
        for i in 0..6 {
            for j in 0..6 {
                assert_eq!(tmp[i], tmp[j]);
                assert_eq!(tmp[i].to_affine(), tmp[j].to_affine());
            }

            assert!(tmp[i] != a);
            assert!(tmp[i] != b);
            assert!(tmp[i] != c);

            assert!(a != tmp[i]);
            assert!(b != tmp[i]);
            assert!(c != tmp[i]);
        }
    }
}

fn random_transformation_tests<G: PrimeCurve>() {
    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    for _ in 0..1000 {
        let g = G::random(&mut rng);
        let g_affine = g.to_affine();
        let g_projective = g_affine.to_curve();
        assert_eq!(g, g_projective);
    }

    // Batch normalization
    for _ in 0..10 {
        let mut v = (0..1000).map(|_| G::random(&mut rng)).collect::<Vec<_>>();

        use rand::distributions::{Distribution, Uniform};
        let between = Uniform::new(0, 1000);
        // Sprinkle in some normalized points
        for _ in 0..5 {
            v[between.sample(&mut rng)] = G::identity();
        }
        for _ in 0..5 {
            let s = between.sample(&mut rng);
            v[s] = v[s].to_affine().to_curve();
        }

        let expected_v = v.iter().map(|v| v.to_affine()).collect::<Vec<_>>();

        let mut normalized = vec![G::Affine::identity(); v.len()];
        G::batch_normalize(&v, &mut normalized);

        assert_eq!(normalized, expected_v);
    }
}

fn random_compressed_encoding_tests<G: PrimeCurve>() {
    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    assert_eq!(
        G::Affine::from_bytes(&G::Affine::identity().to_bytes()).unwrap(),
        G::Affine::identity()
    );

    for _ in 0..1000 {
        let mut r = G::random(&mut rng).to_affine();

        let compressed = r.to_bytes();
        let de_compressed = G::Affine::from_bytes(&compressed).unwrap();
        assert_eq!(de_compressed, r);

        r = r.neg();

        let compressed = r.to_bytes();
        let de_compressed = G::Affine::from_bytes(&compressed).unwrap();
        assert_eq!(de_compressed, r);
    }
}

pub fn random_uncompressed_encoding_tests<G: PrimeCurve>()
where
    <G as PrimeCurve>::Affine: UncompressedEncoding,
{
    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    assert_eq!(
        G::Affine::from_uncompressed(&G::Affine::identity().to_uncompressed()).unwrap(),
        G::Affine::identity()
    );

    for _ in 0..1000 {
        let r = G::random(&mut rng).to_affine();

        let uncompressed = r.to_uncompressed();
        let de_uncompressed = G::Affine::from_uncompressed(&uncompressed).unwrap();
        assert_eq!(de_uncompressed, r);
    }
}
