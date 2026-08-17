macro_rules! impl_lab_color_schemes {
    ($color_ty: ident $(<$phantom_ty: ident>)? [$a: ident, $b: ident] [$($other_component: ident),+]) => {
        impl<$($phantom_ty,)? T> crate::color_theory::Complementary for $color_ty<$($phantom_ty,)? T>
        where
            T: core::ops::Neg<Output = T>,
        {
            fn complementary(self) -> Self {
                Self {
                    $a: -self.$a,
                    $b: -self.$b,
                    $($other_component: self.$other_component,)+
                }
            }
        }

        impl<$($phantom_ty,)? T, A> crate::color_theory::Complementary for crate::Alpha<$color_ty<$($phantom_ty,)? T>, A>
        where
            $color_ty<$($phantom_ty,)? T>: crate::color_theory::Complementary,
        {
            fn complementary(self) -> Self {
                crate::Alpha {
                    color: self.color.complementary(),
                    alpha: self.alpha
                }
            }
        }

        impl<$($phantom_ty,)? T> crate::color_theory::Tetradic for $color_ty<$($phantom_ty,)? T>
        where
            T: core::ops::Neg<Output = T> + Clone,
        {
            fn tetradic(self) -> (Self, Self, Self) {
                use crate::color_theory::Complementary;

                let first = Self {
                    $a: -self.$b.clone(),
                    $b: self.$a.clone(),
                    $($other_component: self.$other_component.clone(),)+
                };

                let second = self.clone().complementary();
                let third = first.clone().complementary();

                (first, second, third)
            }
        }

        impl<$($phantom_ty,)? T, A> crate::color_theory::Tetradic for crate::Alpha<$color_ty<$($phantom_ty,)? T>, A>
        where
            $color_ty<$($phantom_ty,)? T>: crate::color_theory::Tetradic,
            A: Clone,
        {
            fn tetradic(self) -> (Self, Self, Self) {
                let (color1, color2, color3) = self.color.tetradic();

                (
                    crate::Alpha{
                        color: color1,
                        alpha: self.alpha.clone(),
                    },
                    crate::Alpha{
                        color: color2,
                        alpha: self.alpha.clone(),
                    },
                    crate::Alpha{
                        color: color3,
                        alpha: self.alpha,
                    },
                )
            }
        }
    };
    ($color_ty: ident $(<$phantom_ty:ident>)? [$($other_component: ident),+]) => {
        impl_lab_color_schemes!($color_ty $(<$phantom_ty>)? [a, b] [$($other_component),+]);
    };
}

#[cfg(test)]
macro_rules! test_lab_color_schemes {
    ($color_ty: ident / $radial_ty: ident [$a: ident, $b: ident] [$($other_component: ident),+]) => {
        #[cfg(feature = "approx")]
        #[test]
        fn complementary() {
            use crate::{color_theory::Complementary, convert::FromColorUnclamped};

            let quadrant1: $color_ty = $color_ty::new(0.5f32, 0.1, 0.3);
            let quadrant2: $color_ty = $color_ty::new(0.5f32, 0.1, -0.3);
            let quadrant3: $color_ty = $color_ty::new(0.5f32, -0.1, -0.3);
            let quadrant4: $color_ty = $color_ty::new(0.5f32, -0.1, 0.3);

            let quadrant1_radial = $radial_ty::from_color_unclamped(quadrant1);
            let quadrant2_radial = $radial_ty::from_color_unclamped(quadrant2);
            let quadrant3_radial = $radial_ty::from_color_unclamped(quadrant3);
            let quadrant4_radial = $radial_ty::from_color_unclamped(quadrant4);

            assert_relative_eq!(quadrant1.complementary(), $color_ty::from_color_unclamped(quadrant1_radial.complementary()), epsilon = 0.000001);
            assert_relative_eq!(quadrant2.complementary(), $color_ty::from_color_unclamped(quadrant2_radial.complementary()), epsilon = 0.000001);
            assert_relative_eq!(quadrant3.complementary(), $color_ty::from_color_unclamped(quadrant3_radial.complementary()), epsilon = 0.000001);
            assert_relative_eq!(quadrant4.complementary(), $color_ty::from_color_unclamped(quadrant4_radial.complementary()), epsilon = 0.000001);
        }

        #[cfg(feature = "approx")]
        #[test]
        fn tetradic() {
            use crate::{color_theory::Tetradic, convert::FromColorUnclamped};
            fn convert_tuple<T, U>(input: (T, T, T)) -> (U, U, U)
            where
                U: FromColorUnclamped<T>,
            {
                (
                    U::from_color_unclamped(input.0),
                    U::from_color_unclamped(input.1),
                    U::from_color_unclamped(input.2),
                )
            }

            fn check_tuples(a: ($color_ty, $color_ty, $color_ty), b: ($color_ty, $color_ty, $color_ty)) {
                let (a1, a2, a3) = a;
                let (b1, b2, b3) = b;

                assert_relative_eq!(a1, b1, epsilon = 0.000001);
                assert_relative_eq!(a2, b2, epsilon = 0.000001);
                assert_relative_eq!(a3, b3, epsilon = 0.000001);
            }

            let quadrant1 = $color_ty::new(0.5f32, 0.1, 0.3);
            let quadrant2 = $color_ty::new(0.5f32, 0.1, -0.3);
            let quadrant3 = $color_ty::new(0.5f32, -0.1, -0.3);
            let quadrant4 = $color_ty::new(0.5f32, -0.1, 0.3);

            let quadrant1_radial = $radial_ty::from_color_unclamped(quadrant1);
            let quadrant2_radial = $radial_ty::from_color_unclamped(quadrant2);
            let quadrant3_radial = $radial_ty::from_color_unclamped(quadrant3);
            let quadrant4_radial = $radial_ty::from_color_unclamped(quadrant4);

            check_tuples(quadrant1.tetradic(), convert_tuple::<_, $color_ty>(quadrant1_radial.tetradic()));
            check_tuples(quadrant2.tetradic(), convert_tuple::<_, $color_ty>(quadrant2_radial.tetradic()));
            check_tuples(quadrant3.tetradic(), convert_tuple::<_, $color_ty>(quadrant3_radial.tetradic()));
            check_tuples(quadrant4.tetradic(), convert_tuple::<_, $color_ty>(quadrant4_radial.tetradic()));
        }
    };
    ($color_ty: ident / $radial_ty: ident [$($other_component: ident),+]) => {
        test_lab_color_schemes!($color_ty / $radial_ty [a, b] [$($other_component),+]);
    };
}
