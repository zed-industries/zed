#[macro_use]
mod support;

macro_rules! impl_float_tests {
    ($t:ident) => {
        glam_test!(test_lerp, {
            let a = 0.;
            let b = 10.;
            assert_eq!($t::lerp(a, b, 0.), a);
            assert_eq!($t::lerp(a, b, 0.5), 5.);
            assert_eq!($t::lerp(a, b, 1.), b);
            assert_eq!($t::lerp(a, a, 0.), a);
            assert_eq!($t::lerp(a, a, 1.), a);
        });

        glam_test!(test_inverse_lerp, {
            let a = 0.;
            let b = 10.;
            assert_eq!($t::inverse_lerp(a, b, 0.), 0.);
            assert_eq!($t::inverse_lerp(a, b, 5.), 0.5);
            assert_eq!($t::inverse_lerp(a, b, 10.), 1.);
            assert_eq!($t::inverse_lerp(a, b, 15.), 1.5);
            assert!($t::inverse_lerp(a, a, 0.).is_nan());
            assert!($t::inverse_lerp(a, a, 1.).is_infinite());
        });

        glam_test!(test_remap, {
            assert_eq!($t::remap(0., 0., 2., 0., 20.), 0.);
            assert_eq!($t::remap(1., 0., 2., 0., 20.), 10.);
            assert_eq!($t::remap(2., 0., 2., 0., 20.), 20.);
            assert_eq!($t::remap(-5., -10., 30., 60., 20.), 55.);
            assert!($t::remap(0., 0., 0., 0., 1.).is_nan());
            assert!($t::remap(1., 0., 0., 0., 1.).is_infinite());
        });
    };
}

mod float32 {
    use glam::FloatExt;

    impl_float_tests!(f32);
}

mod float64 {
    use glam::FloatExt;

    impl_float_tests!(f64);
}
