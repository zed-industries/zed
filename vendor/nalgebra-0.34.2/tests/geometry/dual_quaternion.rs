#![cfg(feature = "proptest-support")]
#![allow(non_snake_case)]

use na::{DualQuaternion, Point3, Unit, UnitDualQuaternion, UnitQuaternion, Vector3};

use crate::proptest::*;
use proptest::{prop_assert, proptest};

proptest!(
    #[test]
    fn isometry_equivalence(iso in isometry3(), p in point3(), v in vector3()) {
        let dq = UnitDualQuaternion::from_isometry(&iso);

        prop_assert!(relative_eq!(iso * p, dq * p, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(iso * v, dq * v, epsilon = 1.0e-7));
    }

    #[test]
    fn inverse_is_identity(i in unit_dual_quaternion(), p in point3(), v in vector3()) {
        let ii = i.inverse();

        prop_assert!(relative_eq!(i * ii, UnitDualQuaternion::identity(), epsilon = 1.0e-7)
            && relative_eq!(ii * i, UnitDualQuaternion::identity(), epsilon = 1.0e-7)
            && relative_eq!((i * ii) * p, p, epsilon = 1.0e-7)
            && relative_eq!((ii * i) * p, p, epsilon = 1.0e-7)
            && relative_eq!((i * ii) * v, v, epsilon = 1.0e-7)
            && relative_eq!((ii * i) * v, v, epsilon = 1.0e-7));
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn multiply_equals_alga_transform(
        dq in unit_dual_quaternion(),
        v in vector3(),
        p in point3()
    ) {
        prop_assert!(dq * v == dq.transform_vector(&v)
            && dq * p == dq.transform_point(&p)
            && relative_eq!(
                dq.inverse() * v,
                dq.inverse_transform_vector(&v),
                epsilon = 1.0e-7
            )
            && relative_eq!(
                dq.inverse() * p,
                dq.inverse_transform_point(&p),
                epsilon = 1.0e-7
            ));
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn composition(
        dq in unit_dual_quaternion(),
        uq in unit_quaternion(),
        t in translation3(),
        v in vector3(),
        p in point3()
    ) {
        // (rotation × dual quaternion) * point = rotation × (dual quaternion * point)
        prop_assert!(relative_eq!((uq * dq) * v, uq * (dq * v), epsilon = 1.0e-7));
        prop_assert!(relative_eq!((uq * dq) * p, uq * (dq * p), epsilon = 1.0e-7));

        // (dual quaternion × rotation) * point = dual quaternion × (rotation * point)
        prop_assert!(relative_eq!((dq * uq) * v, dq * (uq * v), epsilon = 1.0e-7));
        prop_assert!(relative_eq!((dq * uq) * p, dq * (uq * p), epsilon = 1.0e-7));

        // (translation × dual quaternion) * point = translation × (dual quaternion * point)
        prop_assert!(relative_eq!((t * dq) * v,     (dq * v), epsilon = 1.0e-7));
        prop_assert!(relative_eq!((t * dq) * p, t * (dq * p), epsilon = 1.0e-7));

        // (dual quaternion × translation) * point = dual quaternion × (translation * point)
        prop_assert!(relative_eq!((dq * t) * v, dq * v,       epsilon = 1.0e-7));
        prop_assert!(relative_eq!((dq * t) * p, dq * (t * p), epsilon = 1.0e-7));
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn sclerp_is_defined_for_identical_orientations(
        dq in unit_dual_quaternion(),
        s in -1.0f64..2.0f64,
        t in translation3(),
    ) {
        // Should not panic.
        prop_assert!(relative_eq!(dq.sclerp(&dq, 0.0), dq, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(dq.sclerp(&dq, 0.5), dq, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(dq.sclerp(&dq, 1.0), dq, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(dq.sclerp(&dq, s), dq, epsilon = 1.0e-7));

        let unit = UnitDualQuaternion::identity();
        prop_assert!(relative_eq!(unit.sclerp(&unit, 0.0), unit, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(unit.sclerp(&unit, 0.5), unit, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(unit.sclerp(&unit, 1.0), unit, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(unit.sclerp(&unit, s), unit, epsilon = 1.0e-7));

        // ScLERPing two unit dual quaternions with nearly equal rotation
        // components should result in a unit dual quaternion with a rotation
        // component nearly equal to either input.
        let dq2 = t * dq;
        prop_assert!(relative_eq!(dq.sclerp(&dq2, 0.0).real, dq.real, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(dq.sclerp(&dq2, 0.5).real, dq.real, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(dq.sclerp(&dq2, 1.0).real, dq.real, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(dq.sclerp(&dq2, s).real, dq.real, epsilon = 1.0e-7));

        // ScLERPing two unit dual quaternions with nearly equal rotation
        // components should result in a unit dual quaternion with a translation
        // component which is nearly equal to linearly interpolating the
        // translation components of the inputs.
        prop_assert!(relative_eq!(
            dq.sclerp(&dq2, s).translation().vector,
            dq.translation().vector.lerp(&dq2.translation().vector, s),
            epsilon = 1.0e-7
        ));

        let unit2 = t * unit;
        prop_assert!(relative_eq!(unit.sclerp(&unit2, 0.0).real, unit.real, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(unit.sclerp(&unit2, 0.5).real, unit.real, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(unit.sclerp(&unit2, 1.0).real, unit.real, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(unit.sclerp(&unit2, s).real, unit.real, epsilon = 1.0e-7));

        prop_assert!(relative_eq!(
            unit.sclerp(&unit2, s).translation().vector,
            unit.translation().vector.lerp(&unit2.translation().vector, s),
            epsilon = 1.0e-7
        ));
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn sclerp_is_not_defined_for_opposite_orientations(
        dq in unit_dual_quaternion(),
        s in 0.1f64..0.9f64,
        t in translation3(),
        t2 in translation3(),
        v in vector3(),
    ) {
        let iso = dq.to_isometry();
        let rot = iso.rotation;
        if let Some((axis, angle)) = rot.axis_angle() {
            let flipped = UnitQuaternion::from_axis_angle(&axis, angle + std::f64::consts::PI);
            let dqf = flipped * rot.inverse() * dq.clone();
            prop_assert!(dq.try_sclerp(&dqf, 0.5, 1.0e-7).is_none());
            prop_assert!(dq.try_sclerp(&dqf, s, 1.0e-7).is_none());
        }

        let dq2 = t * dq;
        let iso2 = dq2.to_isometry();
        let rot2 = iso2.rotation;
        if let Some((axis, angle)) = rot2.axis_angle() {
            let flipped = UnitQuaternion::from_axis_angle(&axis, angle + std::f64::consts::PI);
            let dq3f = t2 * flipped * rot.inverse() * dq.clone();
            prop_assert!(dq2.try_sclerp(&dq3f, 0.5, 1.0e-7).is_none());
            prop_assert!(dq2.try_sclerp(&dq3f, s, 1.0e-7).is_none());
        }

        if let Some(axis) = Unit::try_new(v, 1.0e-7) {
            let unit = UnitDualQuaternion::identity();
            let flip = UnitQuaternion::from_axis_angle(&axis, std::f64::consts::PI);
            let unitf = flip * unit;
            prop_assert!(unit.try_sclerp(&unitf, 0.5, 1.0e-7).is_none());
            prop_assert!(unit.try_sclerp(&unitf, s, 1.0e-7).is_none());

            let unit2f = t * unit * flip;
            prop_assert!(unit.try_sclerp(&unit2f, 0.5, 1.0e-7).is_none());
            prop_assert!(unit.try_sclerp(&unit2f, s, 1.0e-7).is_none());
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn all_op_exist(
        dq in dual_quaternion(),
        udq in unit_dual_quaternion(),
        uq in unit_quaternion(),
        s in PROPTEST_F64,
        t in translation3(),
        v in vector3(),
        p in point3()
    ) {
        let dqMs: DualQuaternion<_> = dq * s;

        let dqMdq: DualQuaternion<_> = dq * dq;
        let dqMudq: DualQuaternion<_> = dq * udq;
        let udqMdq: DualQuaternion<_> = udq * dq;

        let iMi: UnitDualQuaternion<_> = udq * udq;
        let iMuq: UnitDualQuaternion<_> = udq * uq;
        let iDi: UnitDualQuaternion<_> = udq / udq;
        let iDuq: UnitDualQuaternion<_> = udq / uq;

        let iMp: Point3<_> = udq * p;
        let iMv: Vector3<_> = udq * v;

        let iMt: UnitDualQuaternion<_> = udq * t;
        let tMi: UnitDualQuaternion<_> = t * udq;

        let uqMi: UnitDualQuaternion<_> = uq * udq;
        let uqDi: UnitDualQuaternion<_> = uq / udq;

        let mut dqMs1 = dq;

        let mut dqMdq1 = dq;
        let mut dqMdq2 = dq;

        let mut dqMudq1 = dq;
        let mut dqMudq2 = dq;

        let mut iMt1 = udq;
        let mut iMt2 = udq;

        let mut iMi1 = udq;
        let mut iMi2 = udq;

        let mut iMuq1 = udq;
        let mut iMuq2 = udq;

        let mut iDi1 = udq;
        let mut iDi2 = udq;

        let mut iDuq1 = udq;
        let mut iDuq2 = udq;

        dqMs1 *= s;

        dqMdq1 *= dq;
        dqMdq2 *= &dq;

        dqMudq1 *= udq;
        dqMudq2 *= &udq;

        iMt1 *= t;
        iMt2 *= &t;

        iMi1 *= udq;
        iMi2 *= &udq;

        iMuq1 *= uq;
        iMuq2 *= &uq;

        iDi1 /= udq;
        iDi2 /= &udq;

        iDuq1 /= uq;
        iDuq2 /= &uq;

        prop_assert!(dqMs == dqMs1
            && dqMdq == dqMdq1
            && dqMdq == dqMdq2
            && dqMudq == dqMudq1
            && dqMudq == dqMudq2
            && iMt == iMt1
            && iMt == iMt2
            && iMi == iMi1
            && iMi == iMi2
            && iMuq == iMuq1
            && iMuq == iMuq2
            && iDi == iDi1
            && iDi == iDi2
            && iDuq == iDuq1
            && iDuq == iDuq2
            && dqMs == &dq * s
            && dqMdq == &dq * &dq
            && dqMdq == dq * &dq
            && dqMdq == &dq * dq
            && dqMudq == &dq * &udq
            && dqMudq == dq * &udq
            && dqMudq == &dq * udq
            && udqMdq == &udq * &dq
            && udqMdq == udq * &dq
            && udqMdq == &udq * dq
            && iMi == &udq * &udq
            && iMi == udq * &udq
            && iMi == &udq * udq
            && iMuq == &udq * &uq
            && iMuq == udq * &uq
            && iMuq == &udq * uq
            && iDi == &udq / &udq
            && iDi == udq / &udq
            && iDi == &udq / udq
            && iDuq == &udq / &uq
            && iDuq == udq / &uq
            && iDuq == &udq / uq
            && iMp == &udq * &p
            && iMp == udq * &p
            && iMp == &udq * p
            && iMv == &udq * &v
            && iMv == udq * &v
            && iMv == &udq * v
            && iMt == &udq * &t
            && iMt == udq * &t
            && iMt == &udq * t
            && tMi == &t * &udq
            && tMi == t * &udq
            && tMi == &t * udq
            && uqMi == &uq * &udq
            && uqMi == uq * &udq
            && uqMi == &uq * udq
            && uqDi == &uq / &udq
            && uqDi == uq / &udq
            && uqDi == &uq / udq)
    }
);
