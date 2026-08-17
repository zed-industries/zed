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

pub mod matrix2 {
    use std::f64;

    use cgmath::*;

    const A: Matrix2<f64> = Matrix2 {
        x: Vector2 {
            x: 1.0f64,
            y: 3.0f64,
        },
        y: Vector2 {
            x: 2.0f64,
            y: 4.0f64,
        },
    };
    const B: Matrix2<f64> = Matrix2 {
        x: Vector2 {
            x: 2.0f64,
            y: 4.0f64,
        },
        y: Vector2 {
            x: 3.0f64,
            y: 5.0f64,
        },
    };
    const C: Matrix2<f64> = Matrix2 {
        x: Vector2 {
            x: 2.0f64,
            y: 1.0f64,
        },
        y: Vector2 {
            x: 1.0f64,
            y: 2.0f64,
        },
    };

    const V: Vector2<f64> = Vector2 {
        x: 1.0f64,
        y: 2.0f64,
    };
    const F: f64 = 0.5;

    #[test]
    fn test_neg() {
        assert_eq!(-A, Matrix2::new(-1.0f64, -3.0f64, -2.0f64, -4.0f64));
    }

    #[test]
    fn test_mul_scalar() {
        let result = Matrix2::new(0.5f64, 1.5f64, 1.0f64, 2.0f64);
        assert_eq!(A * F, result);
        assert_eq!(F * A, result);
    }

    #[test]
    fn test_div_scalar() {
        assert_eq!(A / F, Matrix2::new(2.0f64, 6.0f64, 4.0f64, 8.0f64));
        assert_eq!(4.0f64 / C, Matrix2::new(2.0f64, 4.0f64, 4.0f64, 2.0f64));
    }

    #[test]
    fn test_rem_scalar() {
        assert_eq!(A % 3.0f64, Matrix2::new(1.0f64, 0.0f64, 2.0f64, 1.0f64));
        assert_eq!(3.0f64 % A, Matrix2::new(0.0f64, 0.0f64, 1.0f64, 3.0f64));
    }

    #[test]
    fn test_add_matrix() {
        assert_eq!(A + B, Matrix2::new(3.0f64, 7.0f64, 5.0f64, 9.0f64));
    }

    #[test]
    fn test_sub_matrix() {
        assert_eq!(A - B, Matrix2::new(-1.0f64, -1.0f64, -1.0f64, -1.0f64));
    }

    #[test]
    fn test_mul_vector() {
        assert_eq!(A * V, Vector2::new(5.0f64, 11.0f64));
    }

    #[test]
    fn test_mul_matrix() {
        assert_eq!(A * B, Matrix2::new(10.0f64, 22.0f64, 13.0f64, 29.0f64));

        assert_eq!(A * B, &A * &B);
    }

    #[test]
    fn test_sum_matrix() {
        assert_eq!(A + B + C, [A, B, C].iter().sum());
        assert_eq!(A + B + C, [A, B, C].iter().cloned().sum());
    }

    #[test]
    fn test_product_matrix() {
        assert_eq!(A * B * C, [A, B, C].iter().product());
        assert_eq!(A * B * C, [A, B, C].iter().cloned().product());
    }

    #[test]
    fn test_determinant() {
        assert_eq!(A.determinant(), -2.0f64)
    }

    #[test]
    fn test_trace() {
        assert_eq!(A.trace(), 5.0f64);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(
            A.transpose(),
            Matrix2::<f64>::new(1.0f64, 2.0f64, 3.0f64, 4.0f64)
        );
    }

    #[test]
    fn test_transpose_self() {
        let mut mut_a = A;
        mut_a.transpose_self();
        assert_eq!(mut_a, A.transpose());
    }

    #[test]
    fn test_invert() {
        assert!(Matrix2::<f64>::identity().invert().unwrap().is_identity());

        assert_eq!(
            A.invert().unwrap(),
            Matrix2::new(-2.0f64, 1.5f64, 1.0f64, -0.5f64)
        );
        assert!(Matrix2::new(0.0f64, 2.0f64, 0.0f64, 5.0f64)
            .invert()
            .is_none());
    }

    #[test]
    fn test_predicates() {
        assert!(Matrix2::<f64>::identity().is_identity());
        assert!(Matrix2::<f64>::identity().is_symmetric());
        assert!(Matrix2::<f64>::identity().is_diagonal());
        assert!(Matrix2::<f64>::identity().is_invertible());

        assert!(!A.is_identity());
        assert!(!A.is_symmetric());
        assert!(!A.is_diagonal());
        assert!(A.is_invertible());

        assert!(!C.is_identity());
        assert!(C.is_symmetric());
        assert!(!C.is_diagonal());
        assert!(C.is_invertible());

        assert!(Matrix2::from_value(6.0f64).is_diagonal());
    }

    #[test]
    fn test_from_angle() {
        // Rotate the vector (1, 0) by π/2 radians to the vector (0, 1)
        let rot1 = Matrix2::from_angle(Rad(0.5f64 * f64::consts::PI));
        assert_ulps_eq!(rot1 * Vector2::unit_x(), &Vector2::unit_y());

        // Rotate the vector (-1, 0) by -π/2 radians to the vector (0, 1)
        let rot2 = -rot1;
        assert_ulps_eq!(rot2 * -Vector2::unit_x(), &Vector2::unit_y());

        // Rotate the vector (1, 1) by π radians to the vector (-1, -1)
        let rot3: Matrix2<f64> = Matrix2::from_angle(Rad(f64::consts::PI));
        assert_ulps_eq!(rot3 * Vector2::new(1.0, 1.0), &Vector2::new(-1.0, -1.0));
    }

    #[test]
    fn test_look_at() {
        // rot should rotate unit_x() to look at the input vector
        let rot = Matrix2::look_at(V, Vector2::unit_y());
        assert_eq!(rot * Vector2::unit_x(), V.normalize());
        let new_up = Vector2::new(-V.y, V.x).normalize();
        assert_eq!(rot * Vector2::unit_y(), new_up);

        let rot_down = Matrix2::look_at(V, -1.0 * Vector2::unit_y());
        assert_eq!(rot_down * Vector2::unit_x(), V.normalize());
        assert_eq!(rot_down * Vector2::unit_y(), -1.0 * new_up);

        let rot2 = Matrix2::look_at(-V, Vector2::unit_y());
        assert_eq!(rot2 * Vector2::unit_x(), (-V).normalize());
    }
}

pub mod matrix3 {
    use cgmath::*;

    const A: Matrix3<f64> = Matrix3 {
        x: Vector3 {
            x: 1.0f64,
            y: 4.0f64,
            z: 7.0f64,
        },
        y: Vector3 {
            x: 2.0f64,
            y: 5.0f64,
            z: 8.0f64,
        },
        z: Vector3 {
            x: 3.0f64,
            y: 6.0f64,
            z: 9.0f64,
        },
    };
    const B: Matrix3<f64> = Matrix3 {
        x: Vector3 {
            x: 2.0f64,
            y: 5.0f64,
            z: 8.0f64,
        },
        y: Vector3 {
            x: 3.0f64,
            y: 6.0f64,
            z: 9.0f64,
        },
        z: Vector3 {
            x: 4.0f64,
            y: 7.0f64,
            z: 10.0f64,
        },
    };
    const C: Matrix3<f64> = Matrix3 {
        x: Vector3 {
            x: 2.0f64,
            y: 4.0f64,
            z: 6.0f64,
        },
        y: Vector3 {
            x: 0.0f64,
            y: 2.0f64,
            z: 4.0f64,
        },
        z: Vector3 {
            x: 0.0f64,
            y: 0.0f64,
            z: 1.0f64,
        },
    };
    const D: Matrix3<f64> = Matrix3 {
        x: Vector3 {
            x: 3.0f64,
            y: 2.0f64,
            z: 1.0f64,
        },
        y: Vector3 {
            x: 2.0f64,
            y: 3.0f64,
            z: 2.0f64,
        },
        z: Vector3 {
            x: 1.0f64,
            y: 2.0f64,
            z: 3.0f64,
        },
    };

    const V: Vector3<f64> = Vector3 {
        x: 1.0f64,
        y: 2.0f64,
        z: 3.0f64,
    };
    const F: f64 = 0.5;

    #[test]
    fn test_neg() {
        assert_eq!(
            -A,
            Matrix3::new(
                -1.0f64, -4.0f64, -7.0f64, -2.0f64, -5.0f64, -8.0f64, -3.0f64, -6.0f64, -9.0f64
            )
        );
    }

    #[test]
    fn test_mul_scalar() {
        let result = Matrix3::new(
            0.5f64, 2.0f64, 3.5f64, 1.0f64, 2.5f64, 4.0f64, 1.5f64, 3.0f64, 4.5f64,
        );
        assert_eq!(A * F, result);
        assert_eq!(F * A, result);
    }

    #[test]
    fn test_div_scalar() {
        assert_eq!(
            A / F,
            Matrix3::new(
                2.0f64, 8.0f64, 14.0f64, 4.0f64, 10.0f64, 16.0f64, 6.0f64, 12.0f64, 18.0f64
            )
        );
        assert_eq!(
            6.0f64 / D,
            Matrix3::new(2.0f64, 3.0f64, 6.0f64, 3.0f64, 2.0f64, 3.0f64, 6.0f64, 3.0f64, 2.0f64)
        );
    }

    #[test]
    fn test_rem_scalar() {
        assert_eq!(
            A % 3.0f64,
            Matrix3::new(1.0f64, 1.0f64, 1.0f64, 2.0f64, 2.0f64, 2.0f64, 0.0f64, 0.0f64, 0.0f64)
        );
        assert_eq!(
            9.0f64 % A,
            Matrix3::new(0.0f64, 1.0f64, 2.0f64, 1.0f64, 4.0f64, 1.0f64, 0.0f64, 3.0f64, 0.0f64)
        );
    }

    #[test]
    fn test_add_matrix() {
        assert_eq!(
            A + B,
            Matrix3::new(
                3.0f64, 9.0f64, 15.0f64, 5.0f64, 11.0f64, 17.0f64, 7.0f64, 13.0f64, 19.0f64
            )
        );
    }

    #[test]
    fn test_sub_matrix() {
        assert_eq!(
            A - B,
            Matrix3::new(
                -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64
            )
        );
    }

    #[test]
    fn test_mul_vector() {
        assert_eq!(A * V, Vector3::new(14.0f64, 32.0f64, 50.0f64));
    }

    #[test]
    fn test_mul_matrix() {
        assert_eq!(
            A * B,
            Matrix3::new(
                36.0f64, 81.0f64, 126.0f64, 42.0f64, 96.0f64, 150.0f64, 48.0f64, 111.0f64, 174.0f64
            )
        );

        assert_eq!(A * B, &A * &B);
    }

    #[test]
    fn test_sum_matrix() {
        assert_eq!(A + B + C + D, [A, B, C, D].iter().sum());
        assert_eq!(A + B + C + D, [A, B, C, D].iter().cloned().sum());
    }

    #[test]
    fn test_product_matrix() {
        assert_eq!(A * B * C * D, [A, B, C, D].iter().product());
        assert_eq!(A * B * C * D, [A, B, C, D].iter().cloned().product());
    }

    #[test]
    fn test_determinant() {
        assert_eq!(A.determinant(), 0.0f64);
    }

    #[test]
    fn test_trace() {
        assert_eq!(A.trace(), 15.0f64);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(
            A.transpose(),
            Matrix3::<f64>::new(
                1.0f64, 2.0f64, 3.0f64, 4.0f64, 5.0f64, 6.0f64, 7.0f64, 8.0f64, 9.0f64
            )
        );
    }

    #[test]
    fn test_transpose_self() {
        let mut mut_a = A;
        mut_a.transpose_self();
        assert_eq!(mut_a, A.transpose());
    }

    #[test]
    fn test_invert() {
        assert!(Matrix3::<f64>::identity().invert().unwrap().is_identity());

        assert_eq!(A.invert(), None);

        assert_eq!(
            C.invert().unwrap(),
            Matrix3::new(0.5f64, -1.0f64, 1.0f64, 0.0f64, 0.5f64, -2.0f64, 0.0f64, 0.0f64, 1.0f64)
        );
    }

    #[test]
    fn test_predicates() {
        assert!(Matrix3::<f64>::identity().is_identity());
        assert!(Matrix3::<f64>::identity().is_symmetric());
        assert!(Matrix3::<f64>::identity().is_diagonal());
        assert!(Matrix3::<f64>::identity().is_invertible());

        assert!(!A.is_identity());
        assert!(!A.is_symmetric());
        assert!(!A.is_diagonal());
        assert!(!A.is_invertible());

        assert!(!D.is_identity());
        assert!(D.is_symmetric());
        assert!(!D.is_diagonal());
        assert!(D.is_invertible());

        assert!(Matrix3::from_value(6.0f64).is_diagonal());
    }

    #[test]
    fn test_from_translation() {
        let mat = Matrix3::from_translation(Vector2::new(1.0f64, 2.0f64));
        let vertex = Vector3::new(0.0f64, 0.0f64, 1.0f64);
        let res = mat * vertex;
        assert_eq!(res, Vector3::new(1., 2., 1.));
    }

    mod from_axis_x {
        use cgmath::*;

        fn check_from_axis_angle_x(pitch: Rad<f32>) {
            let found = Matrix3::from_angle_x(pitch);
            let expected = Matrix3::from(Euler {
                x: pitch,
                y: Rad(0.0),
                z: Rad(0.0),
            });
            assert_relative_eq!(found, expected, epsilon = 0.001);
        }

        #[test]
        fn test_zero() {
            check_from_axis_angle_x(Rad(0.0));
        }
        #[test]
        fn test_pos_1() {
            check_from_axis_angle_x(Rad(1.0));
        }
        #[test]
        fn test_neg_1() {
            check_from_axis_angle_x(Rad(-1.0));
        }
    }

    mod from_axis_y {
        use cgmath::*;

        fn check_from_axis_angle_y(yaw: Rad<f32>) {
            let found = Matrix3::from_angle_y(yaw);
            let expected = Matrix3::from(Euler {
                x: Rad(0.0),
                y: yaw,
                z: Rad(0.0),
            });
            assert_relative_eq!(found, expected, epsilon = 0.001);
        }

        #[test]
        fn test_zero() {
            check_from_axis_angle_y(Rad(0.0));
        }
        #[test]
        fn test_pos_1() {
            check_from_axis_angle_y(Rad(1.0));
        }
        #[test]
        fn test_neg_1() {
            check_from_axis_angle_y(Rad(-1.0));
        }
    }

    mod from_axis_z {
        use cgmath::*;

        fn check_from_axis_angle_z(roll: Rad<f32>) {
            let found = Matrix3::from_angle_z(roll);
            let expected = Matrix3::from(Euler {
                x: Rad(0.0),
                y: Rad(0.0),
                z: roll,
            });
            assert_relative_eq!(found, expected, epsilon = 0.001);
        }

        #[test]
        fn test_zero() {
            check_from_axis_angle_z(Rad(0.0));
        }
        #[test]
        fn test_pos_1() {
            check_from_axis_angle_z(Rad(1.0));
        }
        #[test]
        fn test_neg_1() {
            check_from_axis_angle_z(Rad(-1.0));
        }
    }

    mod from_axis_angle {
        mod axis_x {
            use cgmath::*;

            fn check_from_axis_angle_x(pitch: Rad<f32>) {
                let found = Matrix3::from_axis_angle(Vector3::unit_x(), pitch);
                let expected = Matrix3::from(Euler {
                    x: pitch,
                    y: Rad(0.0),
                    z: Rad(0.0),
                });
                assert_relative_eq!(found, expected, epsilon = 0.001);
            }

            #[test]
            fn test_zero() {
                check_from_axis_angle_x(Rad(0.0));
            }
            #[test]
            fn test_pos_1() {
                check_from_axis_angle_x(Rad(1.0));
            }
            #[test]
            fn test_neg_1() {
                check_from_axis_angle_x(Rad(-1.0));
            }
        }

        mod axis_y {
            use cgmath::*;

            fn check_from_axis_angle_y(yaw: Rad<f32>) {
                let found = Matrix3::from_axis_angle(Vector3::unit_y(), yaw);
                let expected = Matrix3::from(Euler {
                    x: Rad(0.0),
                    y: yaw,
                    z: Rad(0.0),
                });
                assert_relative_eq!(found, expected, epsilon = 0.001);
            }

            #[test]
            fn test_zero() {
                check_from_axis_angle_y(Rad(0.0));
            }
            #[test]
            fn test_pos_1() {
                check_from_axis_angle_y(Rad(1.0));
            }
            #[test]
            fn test_neg_1() {
                check_from_axis_angle_y(Rad(-1.0));
            }
        }

        mod axis_z {
            use cgmath::*;

            fn check_from_axis_angle_z(roll: Rad<f32>) {
                let found = Matrix3::from_axis_angle(Vector3::unit_z(), roll);
                let expected = Matrix3::from(Euler {
                    x: Rad(0.0),
                    y: Rad(0.0),
                    z: roll,
                });
                assert_relative_eq!(found, expected, epsilon = 0.001);
            }

            #[test]
            fn test_zero() {
                check_from_axis_angle_z(Rad(0.0));
            }
            #[test]
            fn test_pos_1() {
                check_from_axis_angle_z(Rad(1.0));
            }
            #[test]
            fn test_neg_1() {
                check_from_axis_angle_z(Rad(-1.0));
            }
        }
    }

    mod rotate_from_euler {
        use cgmath::*;

        #[test]
        fn test_x() {
            let vec = vec3(0.0, 0.0, 1.0);

            let rot = Matrix3::from(Euler::new(Deg(90.0), Deg(0.0), Deg(0.0)));
            assert_ulps_eq!(vec3(0.0, -1.0, 0.0), rot * vec);

            let rot = Matrix3::from(Euler::new(Deg(-90.0), Deg(0.0), Deg(0.0)));
            assert_ulps_eq!(vec3(0.0, 1.0, 0.0), rot * vec);
        }

        #[test]
        fn test_y() {
            let vec = vec3(0.0, 0.0, 1.0);

            let rot = Matrix3::from(Euler::new(Deg(0.0), Deg(90.0), Deg(0.0)));
            assert_ulps_eq!(vec3(1.0, 0.0, 0.0), rot * vec);

            let rot = Matrix3::from(Euler::new(Deg(0.0), Deg(-90.0), Deg(0.0)));
            assert_ulps_eq!(vec3(-1.0, 0.0, 0.0), rot * vec);
        }

        #[test]
        fn test_z() {
            let vec = vec3(1.0, 0.0, 0.0);

            let rot = Matrix3::from(Euler::new(Deg(0.0), Deg(0.0), Deg(90.0)));
            assert_ulps_eq!(vec3(0.0, 1.0, 0.0), rot * vec);

            let rot = Matrix3::from(Euler::new(Deg(0.0), Deg(0.0), Deg(-90.0)));
            assert_ulps_eq!(vec3(0.0, -1.0, 0.0), rot * vec);
        }

        // tests that the Y rotation is done after the X
        #[test]
        fn test_x_then_y() {
            let vec = vec3(0.0, 1.0, 0.0);

            let rot = Matrix3::from(Euler::new(Deg(90.0), Deg(90.0), Deg(0.0)));
            assert_ulps_eq!(vec3(0.0, 0.0, 1.0), rot * vec);
        }

        // tests that the Z rotation is done after the Y
        #[test]
        fn test_y_then_z() {
            let vec = vec3(0.0, 0.0, 1.0);

            let rot = Matrix3::from(Euler::new(Deg(0.0), Deg(90.0), Deg(90.0)));
            assert_ulps_eq!(vec3(1.0, 0.0, 0.0), rot * vec);
        }
    }

    mod rotate_from_axis_angle {
        use cgmath::*;

        #[test]
        fn test_x() {
            let vec = vec3(0.0, 0.0, 1.0);

            let rot = Matrix3::from_angle_x(Deg(90.0));
            println!("x mat: {:?}", rot);
            assert_ulps_eq!(vec3(0.0, -1.0, 0.0), rot * vec);
        }

        #[test]
        fn test_y() {
            let vec = vec3(0.0, 0.0, 1.0);

            let rot = Matrix3::from_angle_y(Deg(90.0));
            assert_ulps_eq!(vec3(1.0, 0.0, 0.0), rot * vec);
        }

        #[test]
        fn test_z() {
            let vec = vec3(1.0, 0.0, 0.0);

            let rot = Matrix3::from_angle_z(Deg(90.0));
            assert_ulps_eq!(vec3(0.0, 1.0, 0.0), rot * vec);
        }

        #[test]
        fn test_xy() {
            let vec = vec3(0.0, 0.0, 1.0);

            let rot = Matrix3::from_axis_angle(vec3(1.0, 1.0, 0.0).normalize(), Deg(90.0));
            assert_ulps_eq!(
                vec3(2.0f32.sqrt() / 2.0, -2.0f32.sqrt() / 2.0, 0.0),
                rot * vec
            );
        }

        #[test]
        fn test_yz() {
            let vec = vec3(1.0, 0.0, 0.0);

            let rot = Matrix3::from_axis_angle(vec3(0.0, 1.0, 1.0).normalize(), Deg(-90.0));
            assert_ulps_eq!(
                vec3(0.0, -2.0f32.sqrt() / 2.0, 2.0f32.sqrt() / 2.0),
                rot * vec
            );
        }

        #[test]
        fn test_xz() {
            let vec = vec3(0.0, 1.0, 0.0);

            let rot = Matrix3::from_axis_angle(vec3(1.0, 0.0, 1.0).normalize(), Deg(90.0));
            assert_ulps_eq!(
                vec3(-2.0f32.sqrt() / 2.0, 0.0, 2.0f32.sqrt() / 2.0),
                rot * vec
            );
        }
    }

    #[test]
    fn test_look_to_lh() {
        let dir = Vector3::new(1.0, 2.0, 3.0).normalize();
        let up = Vector3::unit_y();
        let m = Matrix3::look_to_lh(dir, up);

        assert_ulps_eq!(m, Matrix3::from([
            [0.9486833, -0.16903085, 0.26726127],
            [0.0, 0.8451542, 0.53452253],
            [-0.31622776, -0.50709254, 0.8017838_f32]
        ]));

        #[allow(deprecated)]
        {
            assert_ulps_eq!(m, Matrix3::look_at(dir, up));
        }
    }

    #[test]
    fn test_look_to_rh() {
        let dir = Vector3::new(1.0, 2.0, 3.0).normalize();
        let up = Vector3::unit_y();
        let m = Matrix3::look_to_rh(dir, up);

        assert_ulps_eq!(m, Matrix3::from([
            [-0.9486833, -0.16903085, -0.26726127],
            [0.0, 0.8451542, -0.53452253],
            [0.31622776, -0.50709254, -0.8017838_f32]
        ]));
    }
}

pub mod matrix4 {
    use cgmath::*;

    const A: Matrix4<f64> = Matrix4 {
        x: Vector4 {
            x: 1.0f64,
            y: 5.0f64,
            z: 9.0f64,
            w: 13.0f64,
        },
        y: Vector4 {
            x: 2.0f64,
            y: 6.0f64,
            z: 10.0f64,
            w: 14.0f64,
        },
        z: Vector4 {
            x: 3.0f64,
            y: 7.0f64,
            z: 11.0f64,
            w: 15.0f64,
        },
        w: Vector4 {
            x: 4.0f64,
            y: 8.0f64,
            z: 12.0f64,
            w: 16.0f64,
        },
    };
    const B: Matrix4<f64> = Matrix4 {
        x: Vector4 {
            x: 2.0f64,
            y: 6.0f64,
            z: 10.0f64,
            w: 14.0f64,
        },
        y: Vector4 {
            x: 3.0f64,
            y: 7.0f64,
            z: 11.0f64,
            w: 15.0f64,
        },
        z: Vector4 {
            x: 4.0f64,
            y: 8.0f64,
            z: 12.0f64,
            w: 16.0f64,
        },
        w: Vector4 {
            x: 5.0f64,
            y: 9.0f64,
            z: 13.0f64,
            w: 17.0f64,
        },
    };
    const C: Matrix4<f64> = Matrix4 {
        x: Vector4 {
            x: 3.0f64,
            y: 2.0f64,
            z: 1.0f64,
            w: 1.0f64,
        },
        y: Vector4 {
            x: 2.0f64,
            y: 3.0f64,
            z: 2.0f64,
            w: 2.0f64,
        },
        z: Vector4 {
            x: 1.0f64,
            y: 2.0f64,
            z: 3.0f64,
            w: 3.0f64,
        },
        w: Vector4 {
            x: 0.0f64,
            y: 1.0f64,
            z: 1.0f64,
            w: 0.0f64,
        },
    };
    const D: Matrix4<f64> = Matrix4 {
        x: Vector4 {
            x: 4.0f64,
            y: 3.0f64,
            z: 2.0f64,
            w: 1.0f64,
        },
        y: Vector4 {
            x: 3.0f64,
            y: 4.0f64,
            z: 3.0f64,
            w: 2.0f64,
        },
        z: Vector4 {
            x: 2.0f64,
            y: 3.0f64,
            z: 4.0f64,
            w: 3.0f64,
        },
        w: Vector4 {
            x: 1.0f64,
            y: 2.0f64,
            z: 3.0f64,
            w: 4.0f64,
        },
    };

    const V: Vector4<f64> = Vector4 {
        x: 1.0f64,
        y: 2.0f64,
        z: 3.0f64,
        w: 4.0f64,
    };
    const F: f64 = 0.5;

    #[test]
    fn test_neg() {
        assert_eq!(
            -A,
            Matrix4::new(
                -1.0f64, -5.0f64, -9.0f64, -13.0f64, -2.0f64, -6.0f64, -10.0f64, -14.0f64, -3.0f64,
                -7.0f64, -11.0f64, -15.0f64, -4.0f64, -8.0f64, -12.0f64, -16.0f64
            )
        );
    }

    #[test]
    fn test_mul_scalar() {
        let result = Matrix4::new(
            0.5f64, 2.5f64, 4.5f64, 6.5f64, 1.0f64, 3.0f64, 5.0f64, 7.0f64, 1.5f64, 3.5f64, 5.5f64,
            7.5f64, 2.0f64, 4.0f64, 6.0f64, 8.0f64,
        );
        assert_eq!(A * F, result);
        assert_eq!(F * A, result);
    }

    #[test]
    fn test_div_scalar() {
        assert_eq!(
            A / F,
            Matrix4::new(
                2.0f64, 10.0f64, 18.0f64, 26.0f64, 4.0f64, 12.0f64, 20.0f64, 28.0f64, 6.0f64,
                14.0f64, 22.0f64, 30.0f64, 8.0f64, 16.0f64, 24.0f64, 32.0f64
            )
        );
        assert_eq!(
            12.0f64 / D,
            Matrix4::new(
                3.0f64, 4.0f64, 6.0f64, 12.0f64, 4.0f64, 3.0f64, 4.0f64, 6.0f64, 6.0f64, 4.0f64,
                3.0f64, 4.0f64, 12.0f64, 6.0f64, 4.0f64, 3.0f64
            )
        );
    }

    #[test]
    fn test_rem_scalar() {
        assert_eq!(
            A % 4.0f64,
            Matrix4::new(
                1.0f64, 1.0f64, 1.0f64, 1.0f64, 2.0f64, 2.0f64, 2.0f64, 2.0f64, 3.0f64, 3.0f64,
                3.0f64, 3.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64
            )
        );
        assert_eq!(
            16.0f64 % A,
            Matrix4::new(
                0.0f64, 1.0f64, 7.0f64, 3.0f64, 0.0f64, 4.0f64, 6.0f64, 2.0f64, 1.0f64, 2.0f64,
                5.0f64, 1.0f64, 0.0f64, 0.0f64, 4.0f64, 0.0f64
            )
        );
    }

    #[test]
    fn test_add_matrix() {
        assert_eq!(
            A + B,
            Matrix4::new(
                3.0f64, 11.0f64, 19.0f64, 27.0f64, 5.0f64, 13.0f64, 21.0f64, 29.0f64, 7.0f64,
                15.0f64, 23.0f64, 31.0f64, 9.0f64, 17.0f64, 25.0f64, 33.0f64
            )
        );
    }

    #[test]
    fn test_sub_matrix() {
        assert_eq!(
            A - B,
            Matrix4::new(
                -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64,
                -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64, -1.0f64
            )
        );
    }

    #[test]
    fn test_mul_vector() {
        assert_eq!(A * V, Vector4::new(30.0f64, 70.0f64, 110.0f64, 150.0f64));
    }

    #[test]
    fn test_mul_matrix() {
        assert_eq!(
            A * B,
            Matrix4::new(
                100.0f64, 228.0f64, 356.0f64, 484.0f64, 110.0f64, 254.0f64, 398.0f64, 542.0f64,
                120.0f64, 280.0f64, 440.0f64, 600.0f64, 130.0f64, 306.0f64, 482.0f64, 658.0f64
            )
        );

        assert_eq!(A * B, &A * &B);
    }

    #[test]
    fn test_sum_matrix() {
        assert_eq!(A + B + C + D, [A, B, C, D].iter().sum());
        assert_eq!(A + B + C + D, [A, B, C, D].iter().cloned().sum());
    }

    #[test]
    fn test_product_matrix() {
        assert_eq!(A * B * C * D, [A, B, C, D].iter().product());
        assert_eq!(A * B * C * D, [A, B, C, D].iter().cloned().product());
    }

    #[test]
    fn test_determinant() {
        assert_eq!(A.determinant(), 0.0f64);
    }

    #[test]
    fn test_trace() {
        assert_eq!(A.trace(), 34.0f64);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(
            A.transpose(),
            Matrix4::<f64>::new(
                1.0f64, 2.0f64, 3.0f64, 4.0f64, 5.0f64, 6.0f64, 7.0f64, 8.0f64, 9.0f64, 10.0f64,
                11.0f64, 12.0f64, 13.0f64, 14.0f64, 15.0f64, 16.0f64
            )
        );
    }

    #[test]
    fn test_transpose_self() {
        let mut mut_a = A;
        mut_a.transpose_self();
        assert_eq!(mut_a, A.transpose());
    }

    #[test]
    fn test_invert() {
        assert!(Matrix4::<f64>::identity().invert().unwrap().is_identity());

        assert_ulps_eq!(
            &C.invert().unwrap(),
            &(Matrix4::new(
                5.0f64, -4.0f64, 1.0f64, 0.0f64, -4.0f64, 8.0f64, -4.0f64, 0.0f64, 4.0f64, -8.0f64,
                4.0f64, 8.0f64, -3.0f64, 4.0f64, 1.0f64, -8.0f64
            ) * 0.125f64)
        );

        let mat_c = Matrix4::new(
            -0.131917f64,
            -0.76871f64,
            0.625846f64,
            0.0f64,
            -0.,
            0.631364f64,
            0.775487f64,
            0.0f64,
            -0.991261f64,
            0.1023f64,
            -0.083287f64,
            0.0f64,
            0.,
            -1.262728f64,
            -1.550973f64,
            1.0f64,
        );
        assert!((mat_c.invert().unwrap() * mat_c).is_identity());

        let mat_d = Matrix4::new(
            0.065455f64,
            -0.720002f64,
            0.690879f64,
            0.0f64,
            -0.,
            0.692364f64,
            0.721549f64,
            0.0f64,
            -0.997856f64,
            -0.047229f64,
            0.045318f64,
            0.0f64,
            0.,
            -1.384727f64,
            -1.443098f64,
            1.0f64,
        );
        assert!((mat_d.invert().unwrap() * mat_d).is_identity());

        let mat_e = Matrix4::new(
            0.409936f64,
            0.683812f64,
            -0.603617f64,
            0.0f64,
            0.,
            0.661778f64,
            0.7497f64,
            0.0f64,
            0.912114f64,
            -0.307329f64,
            0.271286f64,
            0.0f64,
            -0.,
            -1.323555f64,
            -1.499401f64,
            1.0f64,
        );
        assert!((mat_e.invert().unwrap() * mat_e).is_identity());

        let mat_f = Matrix4::new(
            -0.160691f64,
            -0.772608f64,
            0.614211f64,
            0.0f64,
            -0.,
            0.622298f64,
            0.78278f64,
            0.0f64,
            -0.987005f64,
            0.125786f64,
            -0.099998f64,
            0.0f64,
            0.,
            -1.244597f64,
            -1.565561f64,
            1.0f64,
        );
        assert!((mat_f.invert().unwrap() * mat_f).is_identity());
    }

    #[test]
    fn test_predicates() {
        assert!(Matrix4::<f64>::identity().is_identity());
        assert!(Matrix4::<f64>::identity().is_symmetric());
        assert!(Matrix4::<f64>::identity().is_diagonal());
        assert!(Matrix4::<f64>::identity().is_invertible());

        assert!(!A.is_identity());
        assert!(!A.is_symmetric());
        assert!(!A.is_diagonal());
        assert!(!A.is_invertible());

        assert!(!D.is_identity());
        assert!(D.is_symmetric());
        assert!(!D.is_diagonal());
        assert!(D.is_invertible());

        assert!(Matrix4::from_value(6.0f64).is_diagonal());
    }

    #[test]
    fn test_from_translation() {
        let mat = Matrix4::from_translation(Vector3::new(1.0f64, 2.0f64, 3.0f64));
        let vertex = Vector4::new(0.0f64, 0.0f64, 0.0f64, 1.0f64);
        let res = mat * vertex;
        assert_eq!(res, Vector4::new(1., 2., 3., 1.));
    }

    #[test]
    fn test_cast() {
        assert_ulps_eq!(
            Matrix2::new(0.2f64, 1.5, 4.7, 2.3).cast().unwrap(),
            Matrix2::new(0.2f32, 1.5, 4.7, 2.3)
        );
        assert_ulps_eq!(
            Matrix3::new(0.2f64, 1.5, 4.7, 2.3, 5.7, 2.1, 4.6, 5.2, 6.6,)
                .cast()
                .unwrap(),
            Matrix3::new(0.2f32, 1.5, 4.7, 2.3, 5.7, 2.1, 4.6, 5.2, 6.6,)
        );

        assert_ulps_eq!(
            Matrix4::new(
                0.2f64, 1.5, 4.7, 2.5, 2.3, 5.7, 2.1, 1.1, 4.6, 5.2, 6.6, 0.2, 3.2, 1.8, 0.4, 2.9,
            )
            .cast()
            .unwrap(),
            Matrix4::new(
                0.2f32, 1.5, 4.7, 2.5, 2.3, 5.7, 2.1, 1.1, 4.6, 5.2, 6.6, 0.2, 3.2, 1.8, 0.4, 2.9,
            )
        );
    }

    #[test]
    fn test_look_to_rh() {
        let eye = Point3::new(10.0, 15.0, 20.0);
        let dir = Vector3::new(1.0, 2.0, 3.0).normalize();
        let up = Vector3::unit_y();

        let m = Matrix4::look_to_rh(eye, dir, up);
        #[allow(deprecated)]
        {
            assert_ulps_eq!(m, Matrix4::look_at_dir(eye, dir, up));
        }

        let expected = Matrix4::from([
            [-0.9486833, -0.16903086, -0.26726127, 0.0],
            [0.0, 0.84515435, -0.53452253, 0.0],
            [0.31622776, -0.5070926, -0.8017838, 0.0],
            [3.1622782, -0.84515476, 26.726126, 1.0_f32]
        ]);
        assert_ulps_eq!(expected, m);

        let m = Matrix4::look_at_rh(eye, eye + dir, up);
        assert_abs_diff_eq!(expected, m, epsilon = 1.0e-4);
    }

    #[test]
    fn test_look_to_lh() {
        let eye = Point3::new(10.0, 15.0, 20.0);
        let dir = Vector3::new(1.0, 2.0, 3.0).normalize();
        let up = Vector3::unit_y();

        let m = Matrix4::look_to_lh(eye, dir, up);

        let expected = Matrix4::from([
            [0.9486833, -0.16903086, 0.26726127, 0.0],
            [0.0, 0.84515435, 0.53452253, 0.0],
            [-0.31622776, -0.5070926, 0.8017838, 0.0],
            [-3.1622782, -0.84515476, -26.726126, 1.0_f32]
        ]);
        assert_ulps_eq!(expected, m);

        let m = Matrix4::look_at_lh(eye, eye + dir, up);
        assert_abs_diff_eq!(expected, m, epsilon = 1.0e-4);
    }

    mod from {
        use cgmath::*;

        #[test]
        fn test_quaternion() {
            let quaternion = Quaternion::new(2f32, 3f32, 4f32, 5f32);

            let matrix_short = Matrix4::from(quaternion);

            let matrix_long = Matrix3::from(quaternion);
            let matrix_long = Matrix4::from(matrix_long);

            assert_ulps_eq!(matrix_short, matrix_long);
        }
    }
}
