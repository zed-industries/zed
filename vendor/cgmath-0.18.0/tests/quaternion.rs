// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate approx;
extern crate cgmath;

macro_rules! impl_test_mul {
    ($s:expr, $v:expr) => {
        // point * scalar ops
        assert_eq!($v * $s, Quaternion::from_sv($v.s * $s, $v.v * $s));
        assert_eq!($s * $v, Quaternion::from_sv($s * $v.s, $s * $v.v));
        assert_eq!(&$v * $s, $v * $s);
        assert_eq!($s * &$v, $s * $v);
        // commutativity
        assert_eq!($v * $s, $s * $v);
    };
}

macro_rules! impl_test_div {
    ($s:expr, $v:expr) => {
        // point / scalar ops
        assert_eq!($v / $s, Quaternion::from_sv($v.s / $s, $v.v / $s));
        assert_eq!($s / $v, Quaternion::from_sv($s / $v.s, $s / $v.v));
        assert_eq!(&$v / $s, $v / $s);
        assert_eq!($s / &$v, $s / $v);
    };
}

mod operators {
    use cgmath::*;

    #[test]
    fn test_mul() {
        impl_test_mul!(
            2.0f32,
            Quaternion::from(Euler {
                x: Rad(1f32),
                y: Rad(1f32),
                z: Rad(1f32),
            })
        );
    }

    #[test]
    fn test_div() {
        impl_test_div!(
            2.0f32,
            Quaternion::from(Euler {
                x: Rad(1f32),
                y: Rad(1f32),
                z: Rad(1f32),
            })
        );
    }

    #[test]
    fn test_iter_sum() {
        let q1 = Quaternion::from(Euler {
            x: Rad(2f32),
            y: Rad(1f32),
            z: Rad(1f32),
        });
        let q2 = Quaternion::from(Euler {
            x: Rad(1f32),
            y: Rad(2f32),
            z: Rad(1f32),
        });
        let q3 = Quaternion::from(Euler {
            x: Rad(1f32),
            y: Rad(1f32),
            z: Rad(2f32),
        });

        assert_eq!(q1 + q2 + q3, [q1, q2, q3].iter().sum());
        assert_eq!(q1 + q2 + q3, [q1, q2, q3].iter().cloned().sum());
    }

    #[test]
    fn test_iter_product() {
        let q1 = Quaternion::from(Euler {
            x: Rad(2f32),
            y: Rad(1f32),
            z: Rad(1f32),
        });
        let q2 = Quaternion::from(Euler {
            x: Rad(1f32),
            y: Rad(2f32),
            z: Rad(1f32),
        });
        let q3 = Quaternion::from(Euler {
            x: Rad(1f32),
            y: Rad(1f32),
            z: Rad(2f32),
        });

        assert_eq!(q1 * q2 * q3, [q1, q2, q3].iter().product());
        assert_eq!(q1 * q2 * q3, [q1, q2, q3].iter().cloned().product());
    }
}

mod to_from_euler {
    use std::f32;

    use cgmath::*;

    fn check_euler(rotation: Euler<Rad<f32>>) {
        assert_relative_eq!(
            Euler::from(Quaternion::from(rotation)),
            rotation,
            epsilon = 0.001
        );
    }

    const HPI: f32 = f32::consts::FRAC_PI_2;

    #[test]
    fn test_zero() {
        check_euler(Euler {
            x: Rad(0f32),
            y: Rad(0f32),
            z: Rad(0f32),
        });
    }
    #[test]
    fn test_yaw_pos_1() {
        check_euler(Euler {
            x: Rad(0f32),
            y: Rad(1f32),
            z: Rad(0f32),
        });
    }
    #[test]
    fn test_yaw_neg_1() {
        check_euler(Euler {
            x: Rad(0f32),
            y: Rad(-1f32),
            z: Rad(0f32),
        });
    }
    #[test]
    fn test_pitch_pos_1() {
        check_euler(Euler {
            x: Rad(1f32),
            y: Rad(0f32),
            z: Rad(0f32),
        });
    }
    #[test]
    fn test_pitch_neg_1() {
        check_euler(Euler {
            x: Rad(-1f32),
            y: Rad(0f32),
            z: Rad(0f32),
        });
    }
    #[test]
    fn test_roll_pos_1() {
        check_euler(Euler {
            x: Rad(0f32),
            y: Rad(0f32),
            z: Rad(1f32),
        });
    }
    #[test]
    fn test_roll_neg_1() {
        check_euler(Euler {
            x: Rad(0f32),
            y: Rad(0f32),
            z: Rad(-1f32),
        });
    }
    #[test]
    fn test_pitch_yaw_roll_pos_1() {
        check_euler(Euler {
            x: Rad(1f32),
            y: Rad(1f32),
            z: Rad(1f32),
        });
    }
    #[test]
    fn test_pitch_yaw_roll_neg_1() {
        check_euler(Euler {
            x: Rad(-1f32),
            y: Rad(-1f32),
            z: Rad(-1f32),
        });
    }
    #[test]
    fn test_pitch_yaw_roll_pos_hp() {
        check_euler(Euler {
            x: Rad(0f32),
            y: Rad(HPI),
            z: Rad(1f32),
        });
    }
    #[test]
    fn test_pitch_yaw_roll_neg_hp() {
        check_euler(Euler {
            x: Rad(0f32),
            y: Rad(-HPI),
            z: Rad(1f32),
        });
    }
}

mod from {
    mod matrix3 {
        use cgmath::*;

        fn check_with_euler(x: Rad<f32>, y: Rad<f32>, z: Rad<f32>) {
            let matrix3 = Matrix3::from(Euler { x: x, y: y, z: z });
            let quaternion = Quaternion::from(matrix3);
            let quaternion_matrix3 = Matrix3::from(quaternion);
            assert_ulps_eq!(matrix3, quaternion_matrix3);
        }

        // triggers: trace >= S::zero()
        #[test]
        fn test_positive_trace() {
            check_with_euler(Rad(0.0f32), Rad(0.0), Rad(0.0f32));
        }

        // triggers: (mat[0][0] > mat[1][1]) && (mat[0][0] > mat[2][2])
        #[test]
        fn test_xx_maximum() {
            check_with_euler(Rad(2.0f32), Rad(1.0), Rad(-1.2f32));
        }

        // triggers: mat[1][1] > mat[2][2]
        #[test]
        fn test_yy_maximum() {
            check_with_euler(Rad(2.0f32), Rad(1.0), Rad(3.0f32));
        }

        // base case
        #[test]
        fn test_zz_maximum() {
            check_with_euler(Rad(1.0f32), Rad(1.0), Rad(3.0f32));
        }
    }
}

mod arc {
    use cgmath::*;

    #[inline]
    fn test(src: Vector3<f32>, dst: Vector3<f32>) {
        let q = Quaternion::from_arc(src, dst, None);
        let v = q.rotate_vector(src);
        assert_ulps_eq!(v.normalize(), dst.normalize());
    }

    #[test]
    fn test_same() {
        let v = Vector3::unit_x();
        let q = Quaternion::from_arc(v, v, None);
        assert_eq!(q, Quaternion::new(1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_opposite() {
        let v = Vector3::unit_x();
        test(v, -v);
    }

    #[test]
    fn test_random() {
        test(vec3(1.0, 2.0, 3.0), vec3(-4.0, 5.0, -6.0));
    }

    #[test]
    fn test_ortho() {
        let q: Quaternion<f32> = Quaternion::from_arc(Vector3::unit_x(), Vector3::unit_y(), None);
        let q2 = Quaternion::from_axis_angle(Vector3::unit_z(), Rad::turn_div_4());
        assert_ulps_eq!(q, q2);
    }
}

mod rotate_from_euler {
    use cgmath::*;

    #[test]
    fn test_x() {
        let vec = vec3(0.0, 0.0, 1.0);

        let rot = Quaternion::from(Euler::new(Deg(90.0), Deg(0.0), Deg(0.0)));
        assert_ulps_eq!(vec3(0.0, -1.0, 0.0), rot * vec);

        let rot = Quaternion::from(Euler::new(Deg(-90.0), Deg(0.0), Deg(0.0)));
        assert_ulps_eq!(vec3(0.0, 1.0, 0.0), rot * vec);
    }

    #[test]
    fn test_y() {
        let vec = vec3(0.0, 0.0, 1.0);

        let rot = Quaternion::from(Euler::new(Deg(0.0), Deg(90.0), Deg(0.0)));
        assert_ulps_eq!(vec3(1.0, 0.0, 0.0), rot * vec);

        let rot = Quaternion::from(Euler::new(Deg(0.0), Deg(-90.0), Deg(0.0)));
        assert_ulps_eq!(vec3(-1.0, 0.0, 0.0), rot * vec);
    }

    #[test]
    fn test_z() {
        let vec = vec3(1.0, 0.0, 0.0);

        let rot = Quaternion::from(Euler::new(Deg(0.0), Deg(0.0), Deg(90.0)));
        assert_ulps_eq!(vec3(0.0, 1.0, 0.0), rot * vec);

        let rot = Quaternion::from(Euler::new(Deg(0.0), Deg(0.0), Deg(-90.0)));
        assert_ulps_eq!(vec3(0.0, -1.0, 0.0), rot * vec);
    }

    // tests that the Y rotation is done after the X
    #[test]
    fn test_x_then_y() {
        let vec = vec3(0.0, 1.0, 0.0);

        let rot = Quaternion::from(Euler::new(Deg(90.0), Deg(90.0), Deg(0.0)));
        assert_ulps_eq!(vec3(0.0f32, 0.0f32, 1.0f32), rot * vec);
    }

    // tests that the Z rotation is done after the Y
    #[test]
    fn test_y_then_z() {
        let vec = vec3(0.0f32, 0.0f32, 1.0f32);

        let rot = Quaternion::from(Euler::new(Deg(0.0), Deg(90.0), Deg(90.0)));
        assert_ulps_eq!(vec3(1.0, 0.0, 0.0), rot * vec);
    }
}

mod rotate_from_axis_angle {
    use cgmath::*;

    #[test]
    fn test_x() {
        let vec = vec3(0.0, 0.0, 1.0);

        let rot = Quaternion::from_angle_x(Deg(90.0));
        assert_ulps_eq!(vec3(0.0, -1.0, 0.0), rot * vec);
    }

    #[test]
    fn test_y() {
        let vec = vec3(0.0, 0.0, 1.0);

        let rot = Quaternion::from_angle_y(Deg(90.0));
        assert_ulps_eq!(vec3(1.0, 0.0, 0.0), rot * vec);
    }

    #[test]
    fn test_z() {
        let vec = vec3(1.0, 0.0, 0.0);

        let rot = Quaternion::from_angle_z(Deg(90.0));
        assert_ulps_eq!(vec3(0.0, 1.0, 0.0), rot * vec);
    }

    #[test]
    fn test_xy() {
        let vec = vec3(0.0, 0.0, 1.0);

        let rot = Quaternion::from_axis_angle(vec3(1.0, 1.0, 0.0).normalize(), Deg(90.0));
        assert_ulps_eq!(
            vec3(2.0f32.sqrt() / 2.0, -2.0f32.sqrt() / 2.0, 0.0),
            rot * vec
        );
    }

    #[test]
    fn test_yz() {
        let vec = vec3(1.0, 0.0, 0.0);

        let rot = Quaternion::from_axis_angle(vec3(0.0, 1.0, 1.0).normalize(), Deg(-90.0));
        assert_ulps_eq!(
            vec3(0.0, -2.0f32.sqrt() / 2.0, 2.0f32.sqrt() / 2.0),
            rot * vec
        );
    }

    #[test]
    fn test_xz() {
        let vec = vec3(0.0, 1.0, 0.0);

        let rot = Quaternion::from_axis_angle(vec3(1.0, 0.0, 1.0).normalize(), Deg(90.0));
        assert_ulps_eq!(
            vec3(-2.0f32.sqrt() / 2.0, 0.0, 2.0f32.sqrt() / 2.0),
            rot * vec
        );
    }
}

mod rotate_between_vectors {
    use cgmath::*;

    #[test]
    fn test_around_z_0() {
        let expected = Quaternion::new(1.0, 0.0, 0.0, 0.0);

        let a = vec3(12.0, 0.0, 0.0);
        let b = vec3(1.0, 0.0, 0.0);

        assert_ulps_eq!(Quaternion::between_vectors(a, b), expected);
    }

    #[test]
    fn test_around_z_90_cw() {
        let expected = Quaternion::new(0.5_f32.sqrt(), 0.0, 0.0, 0.5_f32.sqrt());

        let a = vec3(8.0, 0.0, 0.0);
        let b = vec3(0.0, 9.0, 0.0);

        assert_ulps_eq!(Quaternion::between_vectors(a, b), expected);
    }

    #[test]
    fn test_around_z_90_ccw() {
        let expected = Quaternion::new(0.5_f32.sqrt(), 0.0, 0.0, -0.5_f32.sqrt());

        let a = vec3(-26.0, 0.0, 0.0);
        let b = vec3(0.0, 10.0, 0.0);

        assert_ulps_eq!(Quaternion::between_vectors(a, b), expected);
    }

    #[test]
    fn test_around_z_180_cw() {
        let expected = Quaternion::new(0.0, 0.0, 0.0, 1.0);

        let a = vec3(10.0, 0.0, 0.0);
        let b = vec3(-5.0, 0.0, 0.0);

        assert_ulps_eq!(Quaternion::between_vectors(a, b), expected);
    }

    #[test]
    fn test_around_z_180_ccw() {
        let expected = Quaternion::new(0.0, 0.0, 0.0, -1.0);

        let a = vec3(-3.0, 0.0, 0.0);
        let b = vec3(40.0, 0.0, 0.0);

        assert_ulps_eq!(Quaternion::between_vectors(a, b), expected);
    }
}

mod cast {
    use cgmath::*;

    #[test]
    fn test_cast() {
        assert_ulps_eq!(
            Quaternion::new(0.9f64, 1.5, 2.4, 7.6).cast().unwrap(),
            Quaternion::new(0.9f32, 1.5, 2.4, 7.6)
        );
    }
}
