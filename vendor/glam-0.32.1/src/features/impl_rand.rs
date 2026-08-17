macro_rules! impl_vec_types {
    (
        $t:ty,
        $vec2:ident,
        $vec3:ident,
        $vec4:ident,
        $uniform:ident,
        $upper_range_multiplier:expr
    ) => {
        use super::{UniformVec2, UniformVec3, UniformVec4};
        use rand::{
            distr::{
                uniform::{Error as UniformError, SampleBorrow, SampleUniform, UniformSampler},
                Distribution, StandardUniform,
            },
            RngExt,
        };

        impl Distribution<$vec2> for StandardUniform {
            #[inline]
            fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> $vec2 {
                rng.random::<[$t; 2]>().into()
            }
        }

        impl SampleUniform for $vec2 {
            type Sampler = UniformVec2<$uniform<$t>>;
        }

        impl UniformSampler for UniformVec2<$uniform<$t>> {
            type X = $vec2;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, UniformError>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();

                if low.x >= high.x || low.y >= high.y {
                    return Err(UniformError::EmptyRange);
                }

                Ok(Self {
                    x_gen: $uniform::new(low.x, high.x)?,
                    y_gen: $uniform::new(low.y, high.y)?,
                })
            }

            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, UniformError>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();

                if !low.cmple(high).all() {
                    return Err(UniformError::EmptyRange);
                }

                Ok(Self {
                    x_gen: $uniform::new_inclusive(low.x, high.x)?,
                    y_gen: $uniform::new_inclusive(low.y, high.y)?,
                })
            }

            fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> Self::X {
                Self::X::from([self.x_gen.sample(rng), self.y_gen.sample(rng)])
            }

            fn sample_single<R: RngExt + ?Sized, B1, B2>(
                low_b: B1,
                high_b: B2,
                rng: &mut R,
            ) -> Result<Self::X, UniformError>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();

                if low.x >= high.x || low.y >= high.y {
                    return Err(UniformError::EmptyRange);
                }

                Ok(Self::X::from([
                    $uniform::<$t>::sample_single(low.x, high.x, rng)?,
                    $uniform::<$t>::sample_single(low.y, high.y, rng)?,
                ]))
            }

            fn sample_single_inclusive<R: RngExt + ?Sized, B1, B2>(
                low_b: B1,
                high_b: B2,
                rng: &mut R,
            ) -> Result<Self::X, UniformError>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();

                if low.x >= high.x || low.y >= high.y {
                    return Err(UniformError::EmptyRange);
                }

                Ok(Self::X::from([
                    $uniform::<$t>::sample_single_inclusive(low.x, high.x, rng)?,
                    $uniform::<$t>::sample_single_inclusive(low.y, high.y, rng)?,
                ]))
            }
        }

        impl Distribution<$vec3> for StandardUniform {
            #[inline]
            fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> $vec3 {
                rng.random::<[$t; 3]>().into()
            }
        }

        impl SampleUniform for $vec3 {
            type Sampler = UniformVec3<$uniform<$t>>;
        }

        impl UniformSampler for UniformVec3<$uniform<$t>> {
            type X = $vec3;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, UniformError>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();

                if low.x >= high.x || low.y >= high.y || low.z >= high.z {
                    return Err(UniformError::EmptyRange);
                }

                Ok(Self {
                    x_gen: $uniform::new(low.x, high.x)?,
                    y_gen: $uniform::new(low.y, high.y)?,
                    z_gen: $uniform::new(low.z, high.z)?,
                })
            }

            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, UniformError>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();

                if !low.cmple(high).all() {
                    return Err(UniformError::EmptyRange);
                }

                Ok(Self {
                    x_gen: $uniform::new_inclusive(low.x, high.x)?,
                    y_gen: $uniform::new_inclusive(low.y, high.y)?,
                    z_gen: $uniform::new_inclusive(low.z, high.z)?,
                })
            }

            fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> Self::X {
                Self::X::from([
                    self.x_gen.sample(rng),
                    self.y_gen.sample(rng),
                    self.z_gen.sample(rng),
                ])
            }

            fn sample_single<R: RngExt + ?Sized, B1, B2>(
                low_b: B1,
                high_b: B2,
                rng: &mut R,
            ) -> Result<Self::X, UniformError>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();

                if low.x >= high.x || low.y >= high.y || low.z >= high.z {
                    return Err(UniformError::EmptyRange);
                }

                Ok(Self::X::from([
                    $uniform::<$t>::sample_single(low.x, high.x, rng)?,
                    $uniform::<$t>::sample_single(low.y, high.y, rng)?,
                    $uniform::<$t>::sample_single(low.z, high.z, rng)?,
                ]))
            }

            fn sample_single_inclusive<R: RngExt + ?Sized, B1, B2>(
                low_b: B1,
                high_b: B2,
                rng: &mut R,
            ) -> Result<Self::X, UniformError>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();

                if low.x >= high.x || low.y >= high.y || low.z >= high.z {
                    return Err(UniformError::EmptyRange);
                }

                Ok(Self::X::from([
                    $uniform::<$t>::sample_single_inclusive(low.x, high.x, rng)?,
                    $uniform::<$t>::sample_single_inclusive(low.y, high.y, rng)?,
                    $uniform::<$t>::sample_single_inclusive(low.z, high.z, rng)?,
                ]))
            }
        }

        impl Distribution<$vec4> for StandardUniform {
            #[inline]
            fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> $vec4 {
                rng.random::<[$t; 4]>().into()
            }
        }

        impl SampleUniform for $vec4 {
            type Sampler = UniformVec4<$uniform<$t>>;
        }

        impl UniformSampler for UniformVec4<$uniform<$t>> {
            type X = $vec4;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, UniformError>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();

                if low.x >= high.x || low.y >= high.y || low.z >= high.z || low.w >= high.w {
                    return Err(UniformError::EmptyRange);
                }

                Ok(Self {
                    x_gen: $uniform::new(low.x, high.x)?,
                    y_gen: $uniform::new(low.y, high.y)?,
                    z_gen: $uniform::new(low.z, high.z)?,
                    w_gen: $uniform::new(low.w, high.w)?,
                })
            }

            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, UniformError>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();

                if !low.cmple(high).all() {
                    return Err(UniformError::EmptyRange);
                }

                Ok(Self {
                    x_gen: $uniform::new_inclusive(low.x, high.x)?,
                    y_gen: $uniform::new_inclusive(low.y, high.y)?,
                    z_gen: $uniform::new_inclusive(low.z, high.z)?,
                    w_gen: $uniform::new_inclusive(low.w, high.w)?,
                })
            }

            fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> Self::X {
                Self::X::from([
                    self.x_gen.sample(rng),
                    self.y_gen.sample(rng),
                    self.z_gen.sample(rng),
                    self.w_gen.sample(rng),
                ])
            }

            fn sample_single<R: RngExt + ?Sized, B1, B2>(
                low_b: B1,
                high_b: B2,
                rng: &mut R,
            ) -> Result<Self::X, UniformError>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();

                if low.x >= high.x || low.y >= high.y || low.z >= high.z || low.w >= high.w {
                    return Err(UniformError::EmptyRange);
                }

                Ok(Self::X::from([
                    $uniform::<$t>::sample_single(low.x, high.x, rng)?,
                    $uniform::<$t>::sample_single(low.y, high.y, rng)?,
                    $uniform::<$t>::sample_single(low.z, high.z, rng)?,
                    $uniform::<$t>::sample_single(low.w, high.w, rng)?,
                ]))
            }

            fn sample_single_inclusive<R: RngExt + ?Sized, B1, B2>(
                low_b: B1,
                high_b: B2,
                rng: &mut R,
            ) -> Result<Self::X, UniformError>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();

                if low.x >= high.x || low.y >= high.y || low.z >= high.z || low.w >= high.w {
                    return Err(UniformError::EmptyRange);
                }

                Ok(Self::X::from([
                    $uniform::<$t>::sample_single_inclusive(low.x, high.x, rng)?,
                    $uniform::<$t>::sample_single_inclusive(low.y, high.y, rng)?,
                    $uniform::<$t>::sample_single_inclusive(low.z, high.z, rng)?,
                    $uniform::<$t>::sample_single_inclusive(low.w, high.w, rng)?,
                ]))
            }
        }

        #[test]
        fn test_vec2_rand_standard() {
            use rand::{RngExt, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: ($t, $t) = rng1.random();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $vec2 = rng2.random();
            assert_eq!(a, b.into());
        }

        #[test]
        fn test_vec3_rand_standard() {
            use rand::{RngExt, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: ($t, $t, $t) = rng1.random();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $vec3 = rng2.random();
            assert_eq!(a, b.into());
        }

        #[test]
        fn test_vec4_rand_standard() {
            use rand::{RngExt, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: ($t, $t, $t, $t) = rng1.random();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $vec4 = rng2.random();
            assert_eq!(a, b.into());
        }

        test_vec_type_uniform!(
            test_vec2_rand_uniform_equality,
            $vec2,
            $t,
            2,
            $upper_range_multiplier
        );

        test_vec_type_uniform!(
            test_vec3_rand_uniform_equality,
            $vec3,
            $t,
            3,
            $upper_range_multiplier
        );

        test_vec_type_uniform!(
            test_vec4_rand_uniform_equality,
            $vec4,
            $t,
            4,
            $upper_range_multiplier
        );
    };
}

macro_rules! test_vec_type_uniform {
    // NOTE: These were intended to be placed in a `macro_rules!` statement in
    // the main rule below, but rustc wants to complain about unused macros if I
    // try to do that... even if I do use the macros.
    (__repeat_code 2, $code:expr) => {
        ($code, $code)
    };
    (__repeat_code 3, $code:expr) => {
        ($code, $code, $code)
    };
    (__repeat_code 4, $code:expr) => {
        ($code, $code, $code, $code)
    };
    (
        $equality_test_name:ident,
        $vec:ident,
        $t:ty,
        $t_count:tt,
        $upper_range_divisor:expr
    ) => {
        /// Tests that we reach the same result, whether we generate the vector
        /// type directly, or generate its internal values $t_count times and
        /// convert the result into the vector type.
        #[test]
        fn $equality_test_name() {
            use rand::{distr::Uniform, RngExt, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;

            let mut int_rng = Xoshiro256Plus::seed_from_u64(0);
            let mut vec_rng = Xoshiro256Plus::seed_from_u64(0);

            macro_rules! test_uniform {
                (
                    __single_test,
                    $uniform_function_name:ident,
                    $t_low:expr,
                    $t_high:expr,
                    $vec_low:expr,
                    $vec_high:expr
                ) => {
                    let int_u = Uniform::$uniform_function_name($t_low, $t_high).unwrap();
                    let vec_u = Uniform::$uniform_function_name($vec_low, $vec_high).unwrap();

                    let v_int = test_vec_type_uniform!(
                        __repeat_code $t_count,
                        int_rng.sample(int_u)
                    );
                    let v_vec: $vec = vec_rng.sample(vec_u);
                    assert_eq!(v_int, v_vec.into());
                };
                (
                    $uniform_function_name:ident,
                    $t_low:expr,
                    $t_high:expr,
                    $vec_low:expr,
                    $vec_high:expr
                ) => {
                    test_uniform!(
                        __single_test,
                        $uniform_function_name,
                        $t_low, $t_high,
                        $vec_low, $vec_high
                    );

                    test_uniform!(
                        __single_test,
                        $uniform_function_name,
                        &$t_low, &$t_high,
                        &$vec_low, &$vec_high
                    );
                };
            }

            test_uniform!(
                new,
                <$t>::default(),
                <$t>::MAX / $upper_range_divisor,
                $vec::default(),
                $vec::MAX / $upper_range_divisor
            );

            test_uniform!(
                new_inclusive,
                <$t>::default(),
                <$t>::MAX / $upper_range_divisor,
                $vec::default(),
                $vec::MAX / $upper_range_divisor
            );

            test_uniform!(
                new_inclusive,
                <$t>::default(),
                <$t>::default(),
                $vec::default(),
                $vec::default()
            );

            macro_rules! test_sample_uniform_sampler {
                ($sampler_function_name:ident) => {
                    let v_int = test_vec_type_uniform!(
                        __repeat_code $t_count,
                        <$t as SampleUniform>::Sampler::$sampler_function_name(
                            <$t>::default(),
                            <$t>::MAX / $upper_range_divisor,
                            &mut int_rng,
                        ).unwrap()
                    );

                    let v_vec: $vec = <$vec as SampleUniform>::Sampler::$sampler_function_name(
                        $vec::default(),
                        $vec::MAX / $upper_range_divisor,
                        &mut vec_rng,
                    ).unwrap();
                    assert_eq!(v_int, v_vec.into());
                };
            }

            test_sample_uniform_sampler!(sample_single);
            test_sample_uniform_sampler!(sample_single_inclusive);
        }
    };
}

macro_rules! impl_int_types {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident) => {
        use rand::distr::uniform::UniformInt;

        impl_vec_types!($t, $vec2, $vec3, $vec4, UniformInt, 1);
    };
}

macro_rules! impl_float_types {
    ($t:ident, $mat2:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
        use rand::distr::uniform::UniformFloat;

        impl_vec_types!($t, $vec2, $vec3, $vec4, UniformFloat, 10.0);

        impl Distribution<$mat2> for StandardUniform {
            #[inline]
            fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> $mat2 {
                $mat2::from_cols_array(&rng.random())
            }
        }

        impl Distribution<$mat3> for StandardUniform {
            #[inline]
            fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> $mat3 {
                $mat3::from_cols_array(&rng.random())
            }
        }

        impl Distribution<$mat4> for StandardUniform {
            #[inline]
            fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> $mat4 {
                $mat4::from_cols_array(&rng.random())
            }
        }

        impl Distribution<$quat> for StandardUniform {
            #[inline]
            fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> $quat {
                let z = rng.random_range::<$t, _>(-1.0..=1.0);
                let (y, x) = math::sin_cos(rng.random_range::<$t, _>(0.0..TAU));
                let r = math::sqrt(1.0 - z * z);
                let axis = $vec3::new(r * x, r * y, z);
                let angle = rng.random_range::<$t, _>(0.0..TAU);
                $quat::from_axis_angle(axis, angle)
            }
        }

        #[test]
        fn test_mat2_rand_standard() {
            use rand::{RngExt, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a = $mat2::from_cols_array(&rng1.random::<[$t; 4]>());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b = rng2.random::<$mat2>();
            assert_eq!(a, b);
        }

        #[test]
        fn test_mat3_rand_standard() {
            use rand::{RngExt, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a = $mat3::from_cols_array(&rng1.random::<[$t; 9]>());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b = rng2.random::<$mat3>();
            assert_eq!(a, b);
        }

        #[test]
        fn test_mat4_rand_standard() {
            use rand::{RngExt, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a = $mat4::from_cols_array(&rng1.random::<[$t; 16]>());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b = rng2.random::<$mat4>();
            assert_eq!(a, b);
        }

        #[test]
        fn test_quat_rand_standard() {
            use rand::{RngExt, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: $quat = rng1.random();
            assert!(a.is_normalized());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $quat = rng2.random();
            assert_eq!(a, b);
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniformVec2<G> {
    x_gen: G,
    y_gen: G,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniformVec3<G> {
    x_gen: G,
    y_gen: G,
    z_gen: G,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniformVec4<G> {
    x_gen: G,
    y_gen: G,
    z_gen: G,
    w_gen: G,
}

mod f32 {
    use crate::f32::math;
    use crate::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    use core::f32::consts::TAU;

    impl_float_types!(f32, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4);

    impl Distribution<Vec3A> for StandardUniform {
        #[inline]
        fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> Vec3A {
            rng.random::<[f32; 3]>().into()
        }
    }

    #[test]
    fn test_vec3a_rand_standard() {
        use rand::{RngExt, SeedableRng};
        use rand_xoshiro::Xoshiro256Plus;
        let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
        let a: (f32, f32, f32) = rng1.random();
        let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
        let b: Vec3A = rng2.random();
        assert_eq!(a, b.into());
    }
}

mod f64 {
    use crate::f64::math;
    use crate::{DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
    use core::f64::consts::TAU;

    impl_float_types!(f64, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4);
}

mod i8 {
    use crate::{I8Vec2, I8Vec3, I8Vec4};

    impl_int_types!(i8, I8Vec2, I8Vec3, I8Vec4);
}

mod i16 {
    use crate::{I16Vec2, I16Vec3, I16Vec4};

    impl_int_types!(i16, I16Vec2, I16Vec3, I16Vec4);
}

mod i32 {
    use crate::{IVec2, IVec3, IVec4};

    impl_int_types!(i32, IVec2, IVec3, IVec4);
}

mod i64 {
    use crate::{I64Vec2, I64Vec3, I64Vec4};

    impl_int_types!(i64, I64Vec2, I64Vec3, I64Vec4);
}

mod u8 {
    use crate::{U8Vec2, U8Vec3, U8Vec4};

    impl_int_types!(u8, U8Vec2, U8Vec3, U8Vec4);
}

mod u16 {
    use crate::{U16Vec2, U16Vec3, U16Vec4};

    impl_int_types!(u16, U16Vec2, U16Vec3, U16Vec4);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};

    impl_int_types!(u32, UVec2, UVec3, UVec4);
}

mod u64 {
    use crate::{U64Vec2, U64Vec3, U64Vec4};

    impl_int_types!(u64, U64Vec2, U64Vec3, U64Vec4);
}
