use na::{
    Matrix3, Quaternion, RealField, Rotation3, UnitQuaternion, UnitVector3, Vector2, Vector3,
};
use std::f64::consts::PI;

#[test]
fn angle_2() {
    let a = Vector2::new(4.0, 0.0);
    let b = Vector2::new(9.0, 0.0);

    assert_eq!(a.angle(&b), 0.0);
}

#[test]
fn angle_3() {
    let a = Vector3::new(4.0, 0.0, 0.5);
    let b = Vector3::new(8.0, 0.0, 1.0);

    assert_eq!(a.angle(&b), 0.0);
}

#[test]
fn from_rotation_matrix() {
    // Test degenerate case when from_matrix gets stuck in Identity rotation
    let identity =
        Rotation3::from_matrix(&Matrix3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0));
    assert_relative_eq!(identity, &Rotation3::identity(), epsilon = 0.001);
    let rotated_z =
        Rotation3::from_matrix(&Matrix3::new(1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0));
    assert_relative_eq!(
        rotated_z,
        &Rotation3::from_axis_angle(&UnitVector3::new_unchecked(Vector3::new(1.0, 0.0, 0.0)), PI),
        epsilon = 0.001
    );
    // Test that issue 627 is fixed
    let m_627 = Matrix3::<f64>::new(-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0);
    assert_relative_ne!(identity, Rotation3::from_matrix(&m_627), epsilon = 0.01);
    assert_relative_eq!(
        Rotation3::from_matrix_unchecked(m_627.clone()),
        Rotation3::from_matrix(&m_627),
        epsilon = 0.001
    );
    // Test that issue 1078 is fixed
    let m_1078 = Matrix3::<f64>::new(0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, 0.0);
    assert_relative_ne!(identity, Rotation3::from_matrix(&m_1078), epsilon = 0.01);
    assert_relative_eq!(
        Rotation3::from_matrix_unchecked(m_1078.clone()),
        Rotation3::from_matrix(&m_1078),
        epsilon = 0.001
    );
    // Additional test cases for eps >= 1.0
    assert_relative_ne!(
        identity,
        Rotation3::from_matrix_eps(&m_627, 1.2, 0, Rotation3::identity()),
        epsilon = 0.6
    );
    assert_relative_eq!(
        Rotation3::from_matrix_unchecked(m_627.clone()),
        Rotation3::from_matrix_eps(&m_627, 1.2, 0, Rotation3::identity()),
        epsilon = 0.6
    );
    assert_relative_ne!(
        identity,
        Rotation3::from_matrix_eps(&m_1078, 1.0, 0, Rotation3::identity()),
        epsilon = 0.1
    );
    assert_relative_eq!(
        Rotation3::from_matrix_unchecked(m_1078.clone()),
        Rotation3::from_matrix_eps(&m_1078, 1.0, 0, Rotation3::identity()),
        epsilon = 0.1
    );
}

#[test]
fn quaternion_euler_angles_issue_494() {
    let quat = UnitQuaternion::from_quaternion(Quaternion::new(
        -0.10405792,
        -0.6993922f32,
        -0.10406871,
        0.69942284,
    ));
    let angs = quat.euler_angles();
    assert_eq!(angs.0, 2.8461843);
    assert_eq!(angs.1, f32::frac_pi_2());
    assert_eq!(angs.2, 0.0);
}

#[cfg(feature = "proptest-support")]
mod proptest_tests {
    use approx::AbsDiffEq;
    use na::{self, Rotation2, Rotation3, Unit};
    use na::{UnitComplex, UnitQuaternion};
    use simba::scalar::RealField;
    use std::f64;

    use crate::proptest::*;
    use proptest::{prop_assert, prop_assert_eq, proptest};

    proptest! {
        /*
         *
         * Euler angles.
         *
         */
        #[test]
        fn from_euler_angles(r in PROPTEST_F64, p in PROPTEST_F64, y in PROPTEST_F64) {
            let roll  = Rotation3::from_euler_angles(r, 0.0, 0.0);
            let pitch = Rotation3::from_euler_angles(0.0, p, 0.0);
            let yaw   = Rotation3::from_euler_angles(0.0, 0.0, y);

            let rpy = Rotation3::from_euler_angles(r, p, y);

            prop_assert_eq!(roll[(0, 0)], 1.0); // rotation wrt. x axis.
            prop_assert_eq!(pitch[(1, 1)], 1.0); // rotation wrt. y axis.
            prop_assert_eq!(yaw[(2, 2)], 1.0); // rotation wrt. z axis.
            prop_assert_eq!(yaw * pitch * roll, rpy);
        }

        #[test]
        fn euler_angles(r in PROPTEST_F64, p in PROPTEST_F64, y in PROPTEST_F64) {
            let rpy = Rotation3::from_euler_angles(r, p, y);
            let (roll, pitch, yaw) = rpy.euler_angles();
            prop_assert!(relative_eq!(Rotation3::from_euler_angles(roll, pitch, yaw), rpy, epsilon = 1.0e-7));
        }

        #[test]
        fn euler_angles_gimble_lock(r in PROPTEST_F64, y in PROPTEST_F64) {
            let pos = Rotation3::from_euler_angles(r,  f64::frac_pi_2(), y);
            let neg = Rotation3::from_euler_angles(r, -f64::frac_pi_2(), y);
            let (pos_r, pos_p, pos_y) = pos.euler_angles();
            let (neg_r, neg_p, neg_y) = neg.euler_angles();
            prop_assert!(relative_eq!(Rotation3::from_euler_angles(pos_r, pos_p, pos_y), pos, epsilon = 1.0e-7));
            prop_assert!(relative_eq!(Rotation3::from_euler_angles(neg_r, neg_p, neg_y), neg, epsilon = 1.0e-7));
        }

        /*
         *
         * Inversion is transposition.
         *
         */
        #[test]
        fn rotation_inv_3(a in rotation3()) {
            let ta = a.transpose();
            let ia = a.inverse();

            prop_assert_eq!(ta, ia);
            prop_assert!(relative_eq!(&ta * &a,  Rotation3::identity(), epsilon = 1.0e-7));
            prop_assert!(relative_eq!(&ia *  a,  Rotation3::identity(), epsilon = 1.0e-7));
            prop_assert!(relative_eq!(  a * &ta, Rotation3::identity(), epsilon = 1.0e-7));
            prop_assert!(relative_eq!(  a *  ia, Rotation3::identity(), epsilon = 1.0e-7));
        }

        #[test]
        fn rotation_inv_2(a in rotation2()) {
            let ta = a.transpose();
            let ia = a.inverse();

            prop_assert_eq!(ta, ia);
            prop_assert!(relative_eq!(&ta * &a,  Rotation2::identity(), epsilon = 1.0e-7));
            prop_assert!(relative_eq!(&ia *  a,  Rotation2::identity(), epsilon = 1.0e-7));
            prop_assert!(relative_eq!(  a * &ta, Rotation2::identity(), epsilon = 1.0e-7));
            prop_assert!(relative_eq!(  a *  ia, Rotation2::identity(), epsilon = 1.0e-7));
        }

        /*
         *
         * Angle between vectors.
         *
         */
        #[test]
        fn angle_is_commutative_2(a in vector2(), b in vector2()) {
            prop_assert_eq!(a.angle(&b), b.angle(&a))
        }

        #[test]
        fn angle_is_commutative_3(a in vector3(), b in vector3()) {
            prop_assert_eq!(a.angle(&b), b.angle(&a))
        }

        /*
         *
         * Rotation matrix between vectors.
         *
         */
        #[test]
        fn rotation_between_is_anticommutative_2(a in vector2(), b in vector2()) {
            let rab = Rotation2::rotation_between(&a, &b);
            let rba = Rotation2::rotation_between(&b, &a);

            prop_assert!(relative_eq!(rab * rba, Rotation2::identity()));
        }

        #[test]
        fn rotation_between_is_anticommutative_3(a in vector3(), b in vector3()) {
            let rots = (Rotation3::rotation_between(&a, &b), Rotation3::rotation_between(&b, &a));
            if let (Some(rab), Some(rba)) = rots {
                prop_assert!(relative_eq!(rab * rba, Rotation3::identity(), epsilon = 1.0e-7));
            }
        }

        #[test]
        fn rotation_between_is_identity(v2 in vector2(), v3 in vector3()) {
            let vv2 = 3.42 * v2;
            let vv3 = 4.23 * v3;

            prop_assert!(relative_eq!(v2.angle(&vv2), 0.0, epsilon = 1.0e-7));
            prop_assert!(relative_eq!(v3.angle(&vv3), 0.0, epsilon = 1.0e-7));
            prop_assert!(relative_eq!(Rotation2::rotation_between(&v2, &vv2), Rotation2::identity(), epsilon = 1.0e-7));
            prop_assert!(relative_eq!(Rotation3::rotation_between(&v3, &vv3).unwrap(), Rotation3::identity(), epsilon = 1.0e-7));
        }

        #[test]
        fn rotation_between_2(a in vector2(), b in vector2()) {
            if !relative_eq!(a.angle(&b), 0.0, epsilon = 1.0e-7) {
                let r = Rotation2::rotation_between(&a, &b);
                prop_assert!(relative_eq!((r * a).angle(&b), 0.0, epsilon = 1.0e-7))
            }
        }

        #[test]
        fn rotation_between_3(a in vector3(), b in vector3()) {
            if !relative_eq!(a.angle(&b), 0.0, epsilon = 1.0e-7) {
                let r = Rotation3::rotation_between(&a, &b).unwrap();
                prop_assert!(relative_eq!((r * a).angle(&b), 0.0, epsilon = 1.0e-7))
            }
        }


        /*
         *
         * Rotation construction.
         *
         */
        #[test]
        fn new_rotation_2(angle in PROPTEST_F64) {
            let r = Rotation2::new(angle);

            let angle = na::wrap(angle, -f64::pi(), f64::pi());
            prop_assert!(relative_eq!(r.angle(), angle, epsilon = 1.0e-7))
        }

        #[test]
        fn new_rotation_3(axisangle in vector3()) {
            let r = Rotation3::new(axisangle);

            if let Some((axis, angle)) = Unit::try_new_and_get(axisangle, 0.0) {
                let angle = na::wrap(angle, -f64::pi(), f64::pi());
                prop_assert!((relative_eq!(r.angle(), angle, epsilon = 1.0e-7) &&
                 relative_eq!(r.axis().unwrap(), axis, epsilon = 1.0e-7)) ||
                (relative_eq!(r.angle(), -angle, epsilon = 1.0e-7) &&
                 relative_eq!(r.axis().unwrap(), -axis, epsilon = 1.0e-7)))
            }
            else {
                prop_assert_eq!(r, Rotation3::identity())
            }
        }

        /*
         *
         * Rotation pow.
         *
         */
        #[test]
        fn powf_rotation_2(angle in PROPTEST_F64, pow in PROPTEST_F64) {
            let r = Rotation2::new(angle).powf(pow);

            let angle  = na::wrap(angle, -f64::pi(), f64::pi());
            let pangle = na::wrap(angle * pow, -f64::pi(), f64::pi());
            prop_assert!(relative_eq!(r.angle(), pangle, epsilon = 1.0e-7));
        }

        #[test]
        fn powf_rotation_3(axisangle in vector3(), pow in PROPTEST_F64) {
            let r = Rotation3::new(axisangle).powf(pow);

            if let Some((axis, angle)) = Unit::try_new_and_get(axisangle, 0.0) {
                let angle = na::wrap(angle, -f64::pi(), f64::pi());
                let pangle = na::wrap(angle * pow, -f64::pi(), f64::pi());

                prop_assert!((relative_eq!(r.angle(), pangle, epsilon = 1.0e-7) &&
                 relative_eq!(r.axis().unwrap(), axis, epsilon = 1.0e-7)) ||
                (relative_eq!(r.angle(), -pangle, epsilon = 1.0e-7) &&
                 relative_eq!(r.axis().unwrap(), -axis, epsilon = 1.0e-7)));
            }
            else {
                prop_assert_eq!(r, Rotation3::identity())
            }
        }

        //
        //In general, `slerp(a,b,t)` should equal `(b/a)^t * a` even though in practice,
        //we may not use that formula directly for complex numbers or quaternions
        //

        #[test]
        fn slerp_powf_agree_2(a in unit_complex(), b in unit_complex(), t in PROPTEST_F64) {
            let z1 = a.slerp(&b, t);
            let z2 = (b/a).powf(t) * a;
            prop_assert!(relative_eq!(z1,z2,epsilon=1e-10));
        }

        #[test]
        fn slerp_powf_agree_3(a in unit_quaternion(), b in unit_quaternion(), t in PROPTEST_F64) {
            if let Some(z1) = a.try_slerp(&b, t, f64::default_epsilon()) {
                let z2 = (b/a).powf(t) * a;
                prop_assert!(relative_eq!(z1,z2,epsilon=1e-10));
            }
        }

        //
        //when not antipodal, slerp should always take the shortest path between two orientations
        //

        #[test]
        fn slerp_takes_shortest_path_2(
            z in unit_complex(), dtheta in -f64::pi()..f64::pi(), t in 0.0..1.0f64
        ) {

            //ambiguous when at ends of angle range, so we don't really care here
            if dtheta.abs() != f64::pi() {

                //make two complex numbers separated by an angle between -pi and pi
                let (z1, z2) = (z, z * UnitComplex::new(dtheta));
                let z3 = z1.slerp(&z2, t);

                //since the angle is no larger than a half-turn, and t is between 0 and 1,
                //the shortest path just corresponds to adding the scaled angle
                let a1 = z3.angle();
                let a2 = na::wrap(z1.angle() + dtheta*t, -f64::pi(), f64::pi());

                prop_assert!(relative_eq!(a1, a2, epsilon=1e-10));
            }

        }

        #[test]
        fn slerp_takes_shortest_path_3(
            q in unit_quaternion(), dtheta in -f64::pi()..f64::pi(), t in 0.0..1.0f64
        ) {

            //ambiguous when at ends of angle range, so we don't really care here
            if let Some(axis) = q.axis() {

                //make two quaternions separated by an angle between -pi and pi
                let (q1, q2) = (q, q * UnitQuaternion::from_axis_angle(&axis, dtheta));
                let q3 = q1.slerp(&q2, t);

                //since the angle is no larger than a half-turn, and t is between 0 and 1,
                //the shortest path just corresponds to adding the scaled angle
                let q4 = q1 * UnitQuaternion::from_axis_angle(&axis, dtheta*t);
                prop_assert!(relative_eq!(q3, q4, epsilon=1e-10));

            }

        }


    }
}
