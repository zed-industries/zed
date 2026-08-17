#![cfg(feature = "proptest-support")]
#![allow(non_snake_case)]

use na::{Unit, UnitQuaternion};

use crate::proptest::*;
use proptest::{prop_assert, proptest};

proptest!(
    /*
     *
     * Euler angles.
     *
     */
    #[test]
    fn from_euler_angles(r in PROPTEST_F64, p in PROPTEST_F64, y in PROPTEST_F64) {
        let roll = UnitQuaternion::from_euler_angles(r, 0.0, 0.0);
        let pitch = UnitQuaternion::from_euler_angles(0.0, p, 0.0);
        let yaw = UnitQuaternion::from_euler_angles(0.0, 0.0, y);

        let rpy = UnitQuaternion::from_euler_angles(r, p, y);

        let rroll = roll.to_rotation_matrix();
        let rpitch = pitch.to_rotation_matrix();
        let ryaw = yaw.to_rotation_matrix();

        prop_assert!(relative_eq!(rroll[(0, 0)],  1.0, epsilon = 1.0e-7)); // rotation wrt. x axis.
        prop_assert!(relative_eq!(rpitch[(1, 1)], 1.0, epsilon = 1.0e-7)); // rotation wrt. y axis.
        prop_assert!(relative_eq!(ryaw[(2, 2)],   1.0, epsilon = 1.0e-7)); // rotation wrt. z axis.
        prop_assert!(relative_eq!(yaw * pitch * roll, rpy, epsilon = 1.0e-7));
    }

    #[test]
    fn euler_angles(r in PROPTEST_F64, p in PROPTEST_F64, y in PROPTEST_F64) {
        let rpy = UnitQuaternion::from_euler_angles(r, p, y);
        let (roll, pitch, yaw) = rpy.euler_angles();
        prop_assert!(relative_eq!(
            UnitQuaternion::from_euler_angles(roll, pitch, yaw),
            rpy,
            epsilon = 1.0e-7
        ))
    }

    /*
     *
     * From/to rotation matrix.
     *
     */
    #[test]
    fn unit_quaternion_rotation_conversion(q in unit_quaternion()) {
        let r = q.to_rotation_matrix();
        let qq = UnitQuaternion::from_rotation_matrix(&r);
        let rr = qq.to_rotation_matrix();

        prop_assert!(relative_eq!(q, qq, epsilon = 1.0e-7) && relative_eq!(r, rr, epsilon = 1.0e-7))
    }

    /*
     *
     * Point/Vector transformation.
     *
     */

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn unit_quaternion_transformation(
        q in unit_quaternion(),
        v in vector3(),
        p in point3()
    ) {
        let r = q.to_rotation_matrix();
        let rv = r * v;
        let rp = r * p;

        prop_assert!(relative_eq!(q * v, rv, epsilon = 1.0e-7)
            && relative_eq!(q * &v, rv, epsilon = 1.0e-7)
            && relative_eq!(&q * v, rv, epsilon = 1.0e-7)
            && relative_eq!(&q * &v, rv, epsilon = 1.0e-7)
            && relative_eq!(q * p, rp, epsilon = 1.0e-7)
            && relative_eq!(q * &p, rp, epsilon = 1.0e-7)
            && relative_eq!(&q * p, rp, epsilon = 1.0e-7)
            && relative_eq!(&q * &p, rp, epsilon = 1.0e-7))
    }

    /*
     *
     * Inversion.
     *
     */
    #[test]
    fn unit_quaternion_inv(q in unit_quaternion()) {
        let iq = q.inverse();
        prop_assert!(relative_eq!(&iq * &q, UnitQuaternion::identity(), epsilon = 1.0e-7)
            && relative_eq!(iq * &q, UnitQuaternion::identity(), epsilon = 1.0e-7)
            && relative_eq!(&iq * q, UnitQuaternion::identity(), epsilon = 1.0e-7)
            && relative_eq!(iq * q, UnitQuaternion::identity(), epsilon = 1.0e-7)
            && relative_eq!(&q * &iq, UnitQuaternion::identity(), epsilon = 1.0e-7)
            && relative_eq!(q * &iq, UnitQuaternion::identity(), epsilon = 1.0e-7)
            && relative_eq!(&q * iq, UnitQuaternion::identity(), epsilon = 1.0e-7)
            && relative_eq!(q * iq, UnitQuaternion::identity(), epsilon = 1.0e-7))
    }

    /*
     *
     * Quaternion * Vector == Rotation * Vector
     *
     */
    #[test]
    fn unit_quaternion_mul_vector(q in unit_quaternion(), v in vector3(), p in point3()) {
        let r = q.to_rotation_matrix();

        prop_assert!(relative_eq!(q * v, r * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(q * p, r * p, epsilon = 1.0e-7));
        // Equivalence q = -q
        prop_assert!(relative_eq!(UnitQuaternion::new_unchecked(-q.into_inner()) * v, r * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(UnitQuaternion::new_unchecked(-q.into_inner()) * p, r * p, epsilon = 1.0e-7));
    }

    /*
     *
     * Unit quaternion double-covering.
     *
     */
    #[test]
    fn unit_quaternion_double_covering(q in unit_quaternion()) {
        let mq = UnitQuaternion::new_unchecked(-q.into_inner());
        prop_assert!(mq == q && mq.angle() == q.angle() && mq.axis() == q.axis())
    }

    // Test that all operators (incl. all combinations of references) work.
    // See the top comment on `geometry/quaternion_ops.rs` for details on which operations are
    // supported.
    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn all_op_exist(
        q in quaternion(),
        uq in unit_quaternion(),
        v in vector3(),
        p in point3(),
        r in rotation3(),
        s in PROPTEST_F64
    ) {
        let uv = Unit::new_normalize(v);

        let qpq = q + q;
        let qmq = q - q;
        let qMq = q * q;
        let mq = -q;
        let qMs = q * s;
        let qDs = q / s;
        let sMq = s * q;

        let uqMuq = uq * uq;
        let uqMr = uq * r;
        let rMuq = r * uq;
        let uqDuq = uq / uq;
        let uqDr = uq / r;
        let rDuq = r / uq;

        let uqMp = uq * p;
        let uqMv = uq * v;
        let uqMuv = uq * uv;

        let mut qMs1 = q;

        let mut qMq1 = q;
        let mut qMq2 = q;

        let mut qpq1 = q;
        let mut qpq2 = q;

        let mut qmq1 = q;
        let mut qmq2 = q;

        let mut uqMuq1 = uq;
        let mut uqMuq2 = uq;

        let mut uqMr1 = uq;
        let mut uqMr2 = uq;

        let mut uqDuq1 = uq;
        let mut uqDuq2 = uq;

        let mut uqDr1 = uq;
        let mut uqDr2 = uq;

        qMs1 *= s;

        qMq1 *= q;
        qMq2 *= &q;

        qpq1 += q;
        qpq2 += &q;

        qmq1 -= q;
        qmq2 -= &q;

        uqMuq1 *= uq;
        uqMuq2 *= &uq;

        uqMr1 *= r;
        uqMr2 *= &r;

        uqDuq1 /= uq;
        uqDuq2 /= &uq;

        uqDr1 /= r;
        uqDr2 /= &r;

        prop_assert!(qMs1 == qMs
            && qMq1 == qMq
            && qMq1 == qMq2
            && qpq1 == qpq
            && qpq1 == qpq2
            && qmq1 == qmq
            && qmq1 == qmq2
            && uqMuq1 == uqMuq
            && uqMuq1 == uqMuq2
            && uqMr1 == uqMr
            && uqMr1 == uqMr2
            && uqDuq1 == uqDuq
            && uqDuq1 == uqDuq2
            && uqDr1 == uqDr
            && uqDr1 == uqDr2
            && qpq == &q + &q
            && qpq == q + &q
            && qpq == &q + q
            && qmq == &q - &q
            && qmq == q - &q
            && qmq == &q - q
            && qMq == &q * &q
            && qMq == q * &q
            && qMq == &q * q
            && mq == -&q
            && qMs == &q * s
            && qDs == &q / s
            && sMq == s * &q
            && uqMuq == &uq * &uq
            && uqMuq == uq * &uq
            && uqMuq == &uq * uq
            && uqMr == &uq * &r
            && uqMr == uq * &r
            && uqMr == &uq * r
            && rMuq == &r * &uq
            && rMuq == r * &uq
            && rMuq == &r * uq
            && uqDuq == &uq / &uq
            && uqDuq == uq / &uq
            && uqDuq == &uq / uq
            && uqDr == &uq / &r
            && uqDr == uq / &r
            && uqDr == &uq / r
            && rDuq == &r / &uq
            && rDuq == r / &uq
            && rDuq == &r / uq
            && uqMp == &uq * &p
            && uqMp == uq * &p
            && uqMp == &uq * p
            && uqMv == &uq * &v
            && uqMv == uq * &v
            && uqMv == &uq * v
            && uqMuv == &uq * &uv
            && uqMuv == uq * &uv
            && uqMuv == &uq * uv)
    }
);
