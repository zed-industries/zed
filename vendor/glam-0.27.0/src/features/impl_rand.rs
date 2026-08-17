macro_rules! impl_vec_types {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl Distribution<$vec2> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $vec2 {
                rng.gen::<[$t; 2]>().into()
            }
        }

        impl Distribution<$vec3> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $vec3 {
                rng.gen::<[$t; 3]>().into()
            }
        }

        impl Distribution<$vec4> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $vec4 {
                rng.gen::<[$t; 4]>().into()
            }
        }

        #[test]
        fn test_vec2_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: ($t, $t) = rng1.gen();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $vec2 = rng2.gen();
            assert_eq!(a, b.into());
        }

        #[test]
        fn test_vec3_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: ($t, $t, $t) = rng1.gen();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $vec3 = rng2.gen();
            assert_eq!(a, b.into());
        }

        #[test]
        fn test_vec4_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: ($t, $t, $t, $t) = rng1.gen();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $vec4 = rng2.gen();
            assert_eq!(a, b.into());
        }
    };
}

macro_rules! impl_float_types {
    ($t:ident, $mat2:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl_vec_types!($t, $vec2, $vec3, $vec4);

        impl Distribution<$mat2> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $mat2 {
                $mat2::from_cols_array(&rng.gen())
            }
        }

        impl Distribution<$mat3> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $mat3 {
                $mat3::from_cols_array(&rng.gen())
            }
        }

        impl Distribution<$mat4> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $mat4 {
                $mat4::from_cols_array(&rng.gen())
            }
        }

        impl Distribution<$quat> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $quat {
                let yaw = -PI + rng.gen::<$t>() * 2.0 * PI;
                let pitch = -PI + rng.gen::<$t>() * 2.0 * PI;
                let roll = -PI + rng.gen::<$t>() * 2.0 * PI;
                $quat::from_euler(crate::EulerRot::YXZ, yaw, pitch, roll)
            }
        }

        #[test]
        fn test_mat2_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a = $mat2::from_cols_array(&rng1.gen::<[$t; 4]>());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b = rng2.gen::<$mat2>();
            assert_eq!(a, b);
        }

        #[test]
        fn test_mat3_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a = $mat3::from_cols_array(&rng1.gen::<[$t; 9]>());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b = rng2.gen::<$mat3>();
            assert_eq!(a, b);
        }

        #[test]
        fn test_mat4_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a = $mat4::from_cols_array(&rng1.gen::<[$t; 16]>());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b = rng2.gen::<$mat4>();
            assert_eq!(a, b);
        }

        #[test]
        fn test_quat_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: $quat = rng1.gen();
            assert!(a.is_normalized());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $quat = rng2.gen();
            assert_eq!(a, b);
        }
    };
}

mod f32 {
    use crate::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    use core::f32::consts::PI;
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_float_types!(f32, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4);

    impl Distribution<Vec3A> for Standard {
        #[inline]
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3A {
            rng.gen::<[f32; 3]>().into()
        }
    }

    #[test]
    fn test_vec3a_rand() {
        use rand::{Rng, SeedableRng};
        use rand_xoshiro::Xoshiro256Plus;
        let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
        let a: (f32, f32, f32) = rng1.gen();
        let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
        let b: Vec3A = rng2.gen();
        assert_eq!(a, b.into());
    }
}

mod f64 {
    use crate::{DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
    use core::f64::consts::PI;
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_float_types!(f64, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4);
}

mod i16 {
    use crate::{I16Vec2, I16Vec3, I16Vec4};
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_vec_types!(i16, I16Vec2, I16Vec3, I16Vec4);
}

mod i32 {
    use crate::{IVec2, IVec3, IVec4};
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_vec_types!(i32, IVec2, IVec3, IVec4);
}

mod i64 {
    use crate::{I64Vec2, I64Vec3, I64Vec4};
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_vec_types!(i64, I64Vec2, I64Vec3, I64Vec4);
}

mod u16 {
    use crate::{U16Vec2, U16Vec3, U16Vec4};
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_vec_types!(u16, U16Vec2, U16Vec3, U16Vec4);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_vec_types!(u32, UVec2, UVec3, UVec4);
}

mod u64 {
    use crate::{U64Vec2, U64Vec3, U64Vec4};
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_vec_types!(u64, U64Vec2, U64Vec3, U64Vec4);
}
