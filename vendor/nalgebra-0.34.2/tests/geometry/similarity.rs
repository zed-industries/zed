#![cfg(feature = "proptest-support")]
#![allow(non_snake_case)]

use na::Similarity3;

use crate::proptest::*;
use proptest::{prop_assert, prop_assert_eq, proptest};

proptest!(
    #[test]
    fn inverse_is_identity(i in similarity3(), p in point3(), v in vector3()) {
        let ii = i.inverse();

        prop_assert!(relative_eq!(i * ii, Similarity3::identity(), epsilon = 1.0e-7)
            && relative_eq!(ii * i, Similarity3::identity(), epsilon = 1.0e-7)
            && relative_eq!((i * ii) * p, p, epsilon = 1.0e-7)
            && relative_eq!((ii * i) * p, p, epsilon = 1.0e-7)
            && relative_eq!((i * ii) * v, v, epsilon = 1.0e-7)
            && relative_eq!((ii * i) * v, v, epsilon = 1.0e-7))
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn inverse_is_parts_inversion(
        t in translation3(),
        r in unit_quaternion(),
        scaling in PROPTEST_F64
    ) {
        if !relative_eq!(scaling, 0.0) {
            let s = Similarity3::from_isometry(t * r, scaling);
            prop_assert_eq!(s.inverse(), Similarity3::from_scaling(1.0 / scaling) * r.inverse() * t.inverse())
        }
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn multiply_equals_alga_transform(
        s in similarity3(),
        v in vector3(),
        p in point3()
    ) {
        prop_assert!(s * v == s.transform_vector(&v)
            && s * p == s.transform_point(&p)
            && relative_eq!(
                s.inverse() * v,
                s.inverse_transform_vector(&v),
                epsilon = 1.0e-7
            )
            && relative_eq!(
                s.inverse() * p,
                s.inverse_transform_point(&p),
                epsilon = 1.0e-7
            ))
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn composition(
        i in isometry3(),
        uq in unit_quaternion(),
        t in translation3(),
        v in vector3(),
        p in point3(),
        scaling in PROPTEST_F64
    ) {
        if !relative_eq!(scaling, 0.0) {
            let s = Similarity3::from_scaling(scaling);

            // (rotation × translation × scaling) × point = rotation × (translation × (scaling × point))
            prop_assert!(relative_eq!((uq * t * s) * v, uq * (scaling * v), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((uq * t * s) * p, uq * (t * (scaling * p)), epsilon = 1.0e-7));

            // (translation × rotation × scaling) × point = translation × (rotation × (scaling × point))
            prop_assert!(relative_eq!((t * uq * s) * v, uq * (scaling * v), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((t * uq * s) * p, t * (uq * (scaling * p)), epsilon = 1.0e-7));

            // (rotation × isometry × scaling) × point = rotation × (isometry × (scaling × point))
            prop_assert!(relative_eq!((uq * i * s) * v, uq * (i * (scaling * v)), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((uq * i * s) * p, uq * (i * (scaling * p)), epsilon = 1.0e-7));

            // (isometry × rotation × scaling) × point = isometry × (rotation × (scaling × point))
            prop_assert!(relative_eq!((i * uq * s) * v, i * (uq * (scaling * v)), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((i * uq * s) * p, i * (uq * (scaling * p)), epsilon = 1.0e-7));

            // (translation × isometry × scaling) × point = translation × (isometry × (scaling × point))
            prop_assert!(relative_eq!((t * i * s) * v,     (i * (scaling * v)), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((t * i * s) * p, t * (i * (scaling * p)), epsilon = 1.0e-7));

            // (isometry × translation × scaling) × point = isometry × (translation × (scaling × point))
            prop_assert!(relative_eq!((i * t * s) * v, i * (scaling * v),       epsilon = 1.0e-7));
            prop_assert!(relative_eq!((i * t * s) * p, i * (t * (scaling * p)), epsilon = 1.0e-7));


            /*
             * Same as before but with scaling on the middle.
             */
            // (rotation × scaling × translation) × point = rotation × (scaling × (translation × point))
            prop_assert!(relative_eq!((uq * s * t) * v, uq * (scaling * v), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((uq * s * t) * p, uq * (scaling * (t * p)), epsilon = 1.0e-7));

            // (translation × scaling × rotation) × point = translation × (scaling × (rotation × point))
            prop_assert!(relative_eq!((t * s * uq) * v, scaling * (uq * v), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((t * s * uq) * p, t * (scaling * (uq * p)), epsilon = 1.0e-7));

            // (rotation × scaling × isometry) × point = rotation × (scaling × (isometry × point))
            prop_assert!(relative_eq!((uq * s * i) * v, uq * (scaling * (i * v)), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((uq * s * i) * p, uq * (scaling * (i * p)), epsilon = 1.0e-7));

            // (isometry × scaling × rotation) × point = isometry × (scaling × (rotation × point))
            prop_assert!(relative_eq!((i * s * uq) * v, i * (scaling * (uq * v)), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((i * s * uq) * p, i * (scaling * (uq * p)), epsilon = 1.0e-7));

            // (translation × scaling × isometry) × point = translation × (scaling × (isometry × point))
            prop_assert!(relative_eq!((t * s * i) * v,     (scaling * (i * v)), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((t * s * i) * p, t * (scaling * (i * p)), epsilon = 1.0e-7));

            // (isometry × scaling × translation) × point = isometry × (scaling × (translation × point))
            prop_assert!(relative_eq!((i * s * t) * v, i * (scaling * v),       epsilon = 1.0e-7));
            prop_assert!(relative_eq!((i * s * t) * p, i * (scaling * (t * p)), epsilon = 1.0e-7));


            /*
             * Same as before but with scaling on the left.
             */
            // (scaling × rotation × translation) × point = scaling × (rotation × (translation × point))
            prop_assert!(relative_eq!((s * uq * t) * v, scaling * (uq * v), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((s * uq * t) * p, scaling * (uq * (t * p)), epsilon = 1.0e-7));

            // (scaling × translation × rotation) × point = scaling × (translation × (rotation × point))
            prop_assert!(relative_eq!((s * t * uq) * v, scaling * (uq * v), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((s * t * uq) * p, scaling * (t * (uq * p)), epsilon = 1.0e-7));

            // (scaling × rotation × isometry) × point = scaling × (rotation × (isometry × point))
            prop_assert!(relative_eq!((s * uq * i) * v, scaling * (uq * (i * v)), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((s * uq * i) * p, scaling * (uq * (i * p)), epsilon = 1.0e-7));

            // (scaling × isometry × rotation) × point = scaling × (isometry × (rotation × point))
            prop_assert!(relative_eq!((s * i * uq) * v, scaling * (i * (uq * v)), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((s * i * uq) * p, scaling * (i * (uq * p)), epsilon = 1.0e-7));

            // (scaling × translation × isometry) × point = scaling × (translation × (isometry × point))
            prop_assert!(relative_eq!((s * t * i) * v,     (scaling * (i * v)), epsilon = 1.0e-7));
            prop_assert!(relative_eq!((s * t * i) * p, scaling * (t * (i * p)), epsilon = 1.0e-7));

            // (scaling × isometry × translation) × point = scaling × (isometry × (translation × point))
            prop_assert!(relative_eq!((s * i * t) * v, scaling * (i * v),       epsilon = 1.0e-7));
            prop_assert!(relative_eq!((s * i * t) * p, scaling * (i * (t * p)), epsilon = 1.0e-7));
        }
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn all_op_exist(
        s in similarity3(),
        i in isometry3(),
        uq in unit_quaternion(),
        t in translation3(),
        v in vector3(),
        p in point3()
    ) {
        let sMs = s * s;
        let sMuq = s * uq;
        let sDs = s / s;
        let sDuq = s / uq;

        let sMp = s * p;
        let sMv = s * v;

        let sMt = s * t;
        let tMs = t * s;

        let uqMs = uq * s;
        let uqDs = uq / s;

        let sMi = s * i;
        let sDi = s / i;

        let iMs = i * s;
        let iDs = i / s;

        let mut sMt1 = s;
        let mut sMt2 = s;

        let mut sMs1 = s;
        let mut sMs2 = s;

        let mut sMuq1 = s;
        let mut sMuq2 = s;

        let mut sMi1 = s;
        let mut sMi2 = s;

        let mut sDs1 = s;
        let mut sDs2 = s;

        let mut sDuq1 = s;
        let mut sDuq2 = s;

        let mut sDi1 = s;
        let mut sDi2 = s;

        sMt1 *= t;
        sMt2 *= &t;

        sMs1 *= s;
        sMs2 *= &s;

        sMuq1 *= uq;
        sMuq2 *= &uq;

        sMi1 *= i;
        sMi2 *= &i;

        sDs1 /= s;
        sDs2 /= &s;

        sDuq1 /= uq;
        sDuq2 /= &uq;

        sDi1 /= i;
        sDi2 /= &i;

        prop_assert!(sMt == sMt1
            && sMt == sMt2
            && sMs == sMs1
            && sMs == sMs2
            && sMuq == sMuq1
            && sMuq == sMuq2
            && sMi == sMi1
            && sMi == sMi2
            && sDs == sDs1
            && sDs == sDs2
            && sDuq == sDuq1
            && sDuq == sDuq2
            && sDi == sDi1
            && sDi == sDi2
            && sMs == &s * &s
            && sMs == s * &s
            && sMs == &s * s
            && sMuq == &s * &uq
            && sMuq == s * &uq
            && sMuq == &s * uq
            && sDs == &s / &s
            && sDs == s / &s
            && sDs == &s / s
            && sDuq == &s / &uq
            && sDuq == s / &uq
            && sDuq == &s / uq
            && sMp == &s * &p
            && sMp == s * &p
            && sMp == &s * p
            && sMv == &s * &v
            && sMv == s * &v
            && sMv == &s * v
            && sMt == &s * &t
            && sMt == s * &t
            && sMt == &s * t
            && tMs == &t * &s
            && tMs == t * &s
            && tMs == &t * s
            && uqMs == &uq * &s
            && uqMs == uq * &s
            && uqMs == &uq * s
            && uqDs == &uq / &s
            && uqDs == uq / &s
            && uqDs == &uq / s
            && sMi == &s * &i
            && sMi == s * &i
            && sMi == &s * i
            && sDi == &s / &i
            && sDi == s / &i
            && sDi == &s / i
            && iMs == &i * &s
            && iMs == i * &s
            && iMs == &i * s
            && iDs == &i / &s
            && iDs == i / &s
            && iDs == &i / s)
    }
);
