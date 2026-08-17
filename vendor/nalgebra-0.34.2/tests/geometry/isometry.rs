#![cfg(feature = "proptest-support")]
#![allow(non_snake_case)]

use na::{Isometry3, Point3, Vector3};

use crate::proptest::*;
use proptest::{prop_assert, prop_assert_eq, proptest};

proptest!(
    #[test]
    fn append_rotation_wrt_point_to_id(r in unit_quaternion(), p in point3()) {
        let mut iso = Isometry3::identity();
        iso.append_rotation_wrt_point_mut(&r, &p);

        prop_assert_eq!(iso, Isometry3::rotation_wrt_point(r, p))
    }

    #[test]
    fn rotation_wrt_point_invariance(r in unit_quaternion(), p in point3()) {
        let iso = Isometry3::rotation_wrt_point(r, p);

        prop_assert!(relative_eq!(iso * p, p, epsilon = 1.0e-7))
    }

    #[test]
    fn look_at_rh_3(eye in point3(), target in point3(), up in vector3()) {
        let viewmatrix = Isometry3::look_at_rh(&eye, &target, &up);
        let origin = Point3::origin();

        prop_assert!(relative_eq!(viewmatrix * eye, origin, epsilon = 1.0e-7)
            && relative_eq!(
                (viewmatrix * (target - eye)).normalize(),
                -Vector3::z(),
                epsilon = 1.0e-7
            ))
    }

    #[test]
    fn observer_frame_3(eye in point3(), target in point3(), up in vector3()) {
        let observer = Isometry3::face_towards(&eye, &target, &up);
        let origin = Point3::origin();

        prop_assert!(relative_eq!(observer * origin, eye, epsilon = 1.0e-7)
            && relative_eq!(
                observer * Vector3::z(),
                (target - eye).normalize(),
                epsilon = 1.0e-7
            ))
    }

    #[test]
    fn inverse_is_identity(i in isometry3(), p in point3(), v in vector3()) {
        let ii = i.inverse();

        prop_assert!(relative_eq!(i * ii, Isometry3::identity(), epsilon = 1.0e-7)
            && relative_eq!(ii * i, Isometry3::identity(), epsilon = 1.0e-7)
            && relative_eq!((i * ii) * p, p, epsilon = 1.0e-7)
            && relative_eq!((ii * i) * p, p, epsilon = 1.0e-7)
            && relative_eq!((i * ii) * v, v, epsilon = 1.0e-7)
            && relative_eq!((ii * i) * v, v, epsilon = 1.0e-7))
    }

    #[test]
    fn inverse_is_parts_inversion(t in translation3(), r in unit_quaternion()) {
        let i = t * r;
        prop_assert!(i.inverse() == r.inverse() * t.inverse())
    }

    #[test]
    fn multiply_equals_alga_transform(i in isometry3(), v in vector3(), p in point3()) {
        prop_assert!(i * v == i.transform_vector(&v)
            && i * p == i.transform_point(&p)
            && relative_eq!(
                i.inverse() * v,
                i.inverse_transform_vector(&v),
                epsilon = 1.0e-7
            )
            && relative_eq!(
                i.inverse() * p,
                i.inverse_transform_point(&p),
                epsilon = 1.0e-7
            ))
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn composition2(
        i in isometry2(),
        uc in unit_complex(),
        r in rotation2(),
        t in translation2(),
        v in vector2(),
        p in point2()
    ) {
        // (rotation × translation) * point = rotation × (translation * point)
        prop_assert!(relative_eq!((uc * t) * v, uc * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!((r  * t) * v, r  * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!((uc * t) * p, uc * (t * p), epsilon = 1.0e-7));
        prop_assert!(relative_eq!((r * t)  * p, r  * (t * p), epsilon = 1.0e-7));

        // (translation × rotation) * point = translation × (rotation * point)
        prop_assert_eq!((t * uc) * v, uc * v);
        prop_assert_eq!((t * r)  * v, r  * v);
        prop_assert_eq!((t * uc) * p, t * (uc * p));
        prop_assert_eq!((t * r)  * p, t * (r  * p));

        // (rotation × isometry) * point = rotation × (isometry * point)
        prop_assert!(relative_eq!((uc * i) * v, uc * (i * v), epsilon = 1.0e-7));
        prop_assert!(relative_eq!((uc * i) * p, uc * (i * p), epsilon = 1.0e-7));

        // (isometry × rotation) * point = isometry × (rotation * point)
        prop_assert!(relative_eq!((i * uc) * v, i * (uc * v), epsilon = 1.0e-7));
        prop_assert!(relative_eq!((i * uc) * p, i * (uc * p), epsilon = 1.0e-7));

        // (translation × isometry) * point = translation × (isometry * point)
        prop_assert!(relative_eq!((t * i) * v,     (i * v), epsilon = 1.0e-7));
        prop_assert!(relative_eq!((t * i) * p, t * (i * p), epsilon = 1.0e-7));

        // (isometry × translation) * point = isometry × (translation * point)
        prop_assert!(relative_eq!((i * t) * v, i * v,       epsilon = 1.0e-7));
        prop_assert!(relative_eq!((i * t) * p, i * (t * p), epsilon = 1.0e-7));
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn composition3(
        i in isometry3(),
        uq in unit_quaternion(),
        r in rotation3(),
        t in translation3(),
        v in vector3(),
        p in point3()
    ) {
        // (rotation × translation) * point = rotation × (translation * point)
        prop_assert!(relative_eq!((uq * t) * v, uq * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!((r  * t) * v, r  * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!((uq * t) * p, uq * (t * p), epsilon = 1.0e-7));
        prop_assert!(relative_eq!((r * t)  * p, r  * (t * p), epsilon = 1.0e-7));

        // (translation × rotation) * point = translation × (rotation * point)
        prop_assert_eq!((t * uq) * v, uq * v);
        prop_assert_eq!((t * r)  * v, r  * v);
        prop_assert_eq!((t * uq) * p, t * (uq * p));
        prop_assert_eq!((t * r)  * p, t * (r  * p));

        // (rotation × isometry) * point = rotation × (isometry * point)
        prop_assert!(relative_eq!((uq * i) * v, uq * (i * v), epsilon = 1.0e-7));
        prop_assert!(relative_eq!((uq * i) * p, uq * (i * p), epsilon = 1.0e-7));

        // (isometry × rotation) * point = isometry × (rotation * point)
        prop_assert!(relative_eq!((i * uq) * v, i * (uq * v), epsilon = 1.0e-7));
        prop_assert!(relative_eq!((i * uq) * p, i * (uq * p), epsilon = 1.0e-7));

        // (translation × isometry) * point = translation × (isometry * point)
        prop_assert!(relative_eq!((t * i) * v,     (i * v), epsilon = 1.0e-7));
        prop_assert!(relative_eq!((t * i) * p, t * (i * p), epsilon = 1.0e-7));

        // (isometry × translation) * point = isometry × (translation * point)
        prop_assert!(relative_eq!((i * t) * v, i * v,       epsilon = 1.0e-7));
        prop_assert!(relative_eq!((i * t) * p, i * (t * p), epsilon = 1.0e-7));
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn all_op_exist(
        i in isometry3(),
        uq in unit_quaternion(),
        t in translation3(),
        v in vector3(),
        p in point3(),
        r in rotation3()
    ) {
        let iMi = i * i;
        let iMuq = i * uq;
        let iDi = i / i;
        let iDuq = i / uq;

        let iMp = i * p;
        let iMv = i * v;

        let iMt = i * t;
        let tMi = t * i;

        let tMr = t * r;
        let tMuq = t * uq;

        let uqMi = uq * i;
        let uqDi = uq / i;

        let rMt = r * t;
        let uqMt = uq * t;

        let mut iMt1 = i;
        let mut iMt2 = i;

        let mut iMi1 = i;
        let mut iMi2 = i;

        let mut iMuq1 = i;
        let mut iMuq2 = i;

        let mut iDi1 = i;
        let mut iDi2 = i;

        let mut iDuq1 = i;
        let mut iDuq2 = i;

        iMt1 *= t;
        iMt2 *= &t;

        iMi1 *= i;
        iMi2 *= &i;

        iMuq1 *= uq;
        iMuq2 *= &uq;

        iDi1 /= i;
        iDi2 /= &i;

        iDuq1 /= uq;
        iDuq2 /= &uq;

        prop_assert!(iMt == iMt1
            && iMt == iMt2
            && iMi == iMi1
            && iMi == iMi2
            && iMuq == iMuq1
            && iMuq == iMuq2
            && iDi == iDi1
            && iDi == iDi2
            && iDuq == iDuq1
            && iDuq == iDuq2
            && iMi == &i * &i
            && iMi == i * &i
            && iMi == &i * i
            && iMuq == &i * &uq
            && iMuq == i * &uq
            && iMuq == &i * uq
            && iDi == &i / &i
            && iDi == i / &i
            && iDi == &i / i
            && iDuq == &i / &uq
            && iDuq == i / &uq
            && iDuq == &i / uq
            && iMp == &i * &p
            && iMp == i * &p
            && iMp == &i * p
            && iMv == &i * &v
            && iMv == i * &v
            && iMv == &i * v
            && iMt == &i * &t
            && iMt == i * &t
            && iMt == &i * t
            && tMi == &t * &i
            && tMi == t * &i
            && tMi == &t * i
            && tMr == &t * &r
            && tMr == t * &r
            && tMr == &t * r
            && tMuq == &t * &uq
            && tMuq == t * &uq
            && tMuq == &t * uq
            && uqMi == &uq * &i
            && uqMi == uq * &i
            && uqMi == &uq * i
            && uqDi == &uq / &i
            && uqDi == uq / &i
            && uqDi == &uq / i
            && rMt == &r * &t
            && rMt == r * &t
            && rMt == &r * t
            && uqMt == &uq * &t
            && uqMt == uq * &t
            && uqMt == &uq * t)
    }
);
