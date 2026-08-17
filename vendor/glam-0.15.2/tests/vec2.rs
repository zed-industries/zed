#[macro_use]
mod support;

macro_rules! impl_vec2_tests {
    ($t:ty, $const_new:ident, $new:ident, $vec2:ident, $vec3:ident, $mask:ident) => {
        #[test]
        fn test_const() {
            const V: $vec2 = $const_new!([1 as $t, 2 as $t]);
            assert_eq!($vec2::new(1 as $t, 2 as $t), V);
        }

        #[test]
        fn test_new() {
            let v = $new(1 as $t, 2 as $t);

            assert_eq!(v.x, 1 as $t);
            assert_eq!(v.y, 2 as $t);

            let t = (1 as $t, 2 as $t);
            let v = $vec2::from(t);
            assert_eq!(t, v.into());

            let a = [1 as $t, 2 as $t];
            let v = $vec2::from(a);
            let a1: [$t; 2] = v.into();
            assert_eq!(a, a1);

            let v = $vec2::new(t.0, t.1);
            assert_eq!(t, v.into());

            assert_eq!($vec2::new(1 as $t, 0 as $t), $vec2::X);
            assert_eq!($vec2::new(0 as $t, 1 as $t), $vec2::Y);
        }

        #[test]
        fn test_fmt() {
            let a = $vec2::new(1 as $t, 2 as $t);
            assert_eq!(
                format!("{:?}", a),
                format!("{}({:?}, {:?})", stringify!($vec2), a.x, a.y)
            );
            // assert_eq!(format!("{:#?}", a), "$vec2(\n    1.0,\n    2.0\n)");
            assert_eq!(format!("{}", a), "[1, 2]");
        }

        #[test]
        fn test_zero() {
            let v = $vec2::ZERO;
            assert_eq!($new(0 as $t, 0 as $t), v);
            assert_eq!(v, $vec2::default());
        }

        #[test]
        fn test_splat() {
            let v = $vec2::splat(1 as $t);
            assert_eq!($vec2::ONE, v);
        }

        #[test]
        fn test_accessors() {
            let mut a = $vec2::ZERO;
            a.x = 1 as $t;
            a.y = 2 as $t;
            assert_eq!(1 as $t, a.x);
            assert_eq!(2 as $t, a.y);
            assert_eq!($vec2::new(1 as $t, 2 as $t), a);

            let mut a = $vec2::ZERO;
            a[0] = 1 as $t;
            a[1] = 2 as $t;
            assert_eq!(1 as $t, a[0]);
            assert_eq!(2 as $t, a[1]);
            assert_eq!($vec2::new(1 as $t, 2 as $t), a);
        }

        #[test]
        fn test_dot_unsigned() {
            let x = $new(1 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t);
            assert_eq!(1 as $t, x.dot(x));
            assert_eq!(0 as $t, x.dot(y));
        }

        #[test]
        fn test_ops() {
            let a = $new(2 as $t, 4 as $t);
            assert_eq!($new(4 as $t, 8 as $t), (a + a));
            assert_eq!($new(0 as $t, 0 as $t), (a - a));
            assert_eq!($new(4 as $t, 16 as $t), (a * a));
            assert_eq!($new(4 as $t, 8 as $t), (a * 2 as $t));
            assert_eq!($new(4 as $t, 8 as $t), (2 as $t * a));
            assert_eq!($new(1 as $t, 1 as $t), (a / a));
            assert_eq!($new(1 as $t, 2 as $t), (a / 2 as $t));
            assert_eq!($new(2 as $t, 1 as $t), (4 as $t / a));
        }

        #[test]
        fn test_assign_ops() {
            let a = $new(1 as $t, 2 as $t);
            let mut b = a;
            b += a;
            assert_eq!($new(2 as $t, 4 as $t), b);
            b -= a;
            assert_eq!($new(1 as $t, 2 as $t), b);
            b *= a;
            assert_eq!($new(1 as $t, 4 as $t), b);
            b /= a;
            assert_eq!($new(1 as $t, 2 as $t), b);
            b *= 2 as $t;
            assert_eq!($new(2 as $t, 4 as $t), b);
            b /= 2 as $t;
            assert_eq!($new(1 as $t, 2 as $t), b);
        }

        #[test]
        fn test_min_max() {
            let a = $new(0 as $t, 2 as $t);
            let b = $new(1 as $t, 1 as $t);
            assert_eq!($new(0 as $t, 1 as $t), a.min(b));
            assert_eq!($new(0 as $t, 1 as $t), b.min(a));
            assert_eq!($new(1 as $t, 2 as $t), a.max(b));
            assert_eq!($new(1 as $t, 2 as $t), b.max(a));
        }

        #[test]
        fn test_clamp() {
            fn vec(x: i32, y: i32) -> $vec2 {
                $vec2::new(x as $t, y as $t)
            }
            let min = vec(1, 3);
            let max = vec(6, 8);
            assert_eq!(vec(0, 0).clamp(min, max), vec(1, 3));
            assert_eq!(vec(2, 2).clamp(min, max), vec(2, 3));
            assert_eq!(vec(4, 5).clamp(min, max), vec(4, 5));
            assert_eq!(vec(6, 6).clamp(min, max), vec(6, 6));
            assert_eq!(vec(7, 7).clamp(min, max), vec(6, 7));
            assert_eq!(vec(9, 9).clamp(min, max), vec(6, 8));
        }

        #[test]
        fn test_hmin_hmax() {
            let a = $new(1 as $t, 2 as $t);
            assert_eq!(1 as $t, a.min_element());
            assert_eq!(2 as $t, a.max_element());
        }

        #[test]
        fn test_eq() {
            let a = $new(1 as $t, 1 as $t);
            let b = $new(1 as $t, 2 as $t);
            assert!(a.cmpeq(a).all());
            assert!(b.cmpeq(b).all());
            assert!(a.cmpne(b).any());
            assert!(b.cmpne(a).any());
            assert!(b.cmpeq(a).any());
        }

        #[test]
        fn test_cmp() {
            assert!(!$mask::default().any());
            assert!(!$mask::default().all());
            assert_eq!($mask::default().bitmask(), 0x0);
            let a = $new(1 as $t, 1 as $t);
            let b = $new(2 as $t, 2 as $t);
            let c = $new(1 as $t, 1 as $t);
            let d = $new(2 as $t, 1 as $t);
            assert_eq!(a.cmplt(a).bitmask(), 0x0);
            assert_eq!(a.cmplt(b).bitmask(), 0x3);
            assert_eq!(a.cmplt(d).bitmask(), 0x1);
            assert_eq!(c.cmple(a).bitmask(), 0x3);
            assert!(a.cmplt(b).all());
            assert!(a.cmplt(d).any());
            assert!(a.cmple(b).all());
            assert!(a.cmple(a).all());
            assert!(b.cmpgt(a).all());
            assert!(b.cmpge(a).all());
            assert!(b.cmpge(b).all());
            assert!(!(a.cmpge(d).all()));
            assert!(c.cmple(c).all());
            assert!(c.cmpge(c).all());
            assert!(a == a);
        }

        #[test]
        fn test_extend_truncate() {
            let a = $new(1 as $t, 2 as $t);
            let b = a.extend(3 as $t);
            assert_eq!($vec3::new(1 as $t, 2 as $t, 3 as $t), b);
        }

        #[test]
        fn test_vec2mask() {
            // make sure the unused 'z' value doesn't break $vec2 behaviour
            let a = $vec3::ZERO;
            let mut b = a.truncate();
            b.x = 1 as $t;
            b.y = 1 as $t;
            assert!(!b.cmpeq($vec2::ZERO).any());
            assert!(b.cmpeq($vec2::splat(1 as $t)).all());
        }

        // #[test]
        // fn test_mask_as_ref() {
        //     assert_eq!($mask::new(false, false).as_ref(), &[0, 0]);
        //     assert_eq!($mask::new(true, false).as_ref(), &[!0, 0]);
        //     assert_eq!($mask::new(false, true).as_ref(), &[0, !0]);
        //     assert_eq!($mask::new(true, true).as_ref(), &[!0, !0]);
        // }

        #[test]
        fn test_mask_from() {
            assert_eq!(Into::<[u32; 2]>::into($mask::new(false, false)), [0, 0]);
            assert_eq!(Into::<[u32; 2]>::into($mask::new(true, false)), [!0, 0]);
            assert_eq!(Into::<[u32; 2]>::into($mask::new(false, true)), [0, !0]);
            assert_eq!(Into::<[u32; 2]>::into($mask::new(true, true)), [!0, !0]);
        }

        #[test]
        fn test_mask_bitmask() {
            assert_eq!($mask::new(false, false).bitmask(), 0b00);
            assert_eq!($mask::new(true, false).bitmask(), 0b01);
            assert_eq!($mask::new(false, true).bitmask(), 0b10);
            assert_eq!($mask::new(true, true).bitmask(), 0b11);
        }

        #[test]
        fn test_mask_any() {
            assert_eq!($mask::new(false, false).any(), false);
            assert_eq!($mask::new(true, false).any(), true);
            assert_eq!($mask::new(false, true).any(), true);
            assert_eq!($mask::new(true, true).any(), true);
        }

        #[test]
        fn test_mask_all() {
            assert_eq!($mask::new(false, false).all(), false);
            assert_eq!($mask::new(true, false).all(), false);
            assert_eq!($mask::new(false, true).all(), false);
            assert_eq!($mask::new(true, true).all(), true);
        }

        #[test]
        fn test_mask_select() {
            let a = $vec2::new(1 as $t, 2 as $t);
            let b = $vec2::new(3 as $t, 4 as $t);
            assert_eq!(
                $vec2::select($mask::new(true, true), a, b),
                $vec2::new(1 as $t, 2 as $t),
            );
            assert_eq!(
                $vec2::select($mask::new(true, false), a, b),
                $vec2::new(1 as $t, 4 as $t),
            );
            assert_eq!(
                $vec2::select($mask::new(false, true), a, b),
                $vec2::new(3 as $t, 2 as $t),
            );
            assert_eq!(
                $vec2::select($mask::new(false, false), a, b),
                $vec2::new(3 as $t, 4 as $t),
            );
        }

        #[test]
        fn test_mask_and() {
            assert_eq!(
                ($mask::new(false, false) & $mask::new(false, false)).bitmask(),
                0b00,
            );
            assert_eq!(
                ($mask::new(true, true) & $mask::new(true, false)).bitmask(),
                0b01,
            );
            assert_eq!(
                ($mask::new(true, false) & $mask::new(false, true)).bitmask(),
                0b00,
            );
            assert_eq!(
                ($mask::new(true, true) & $mask::new(true, true)).bitmask(),
                0b11,
            );

            let mut mask = $mask::new(true, true);
            mask &= $mask::new(true, false);
            assert_eq!(mask.bitmask(), 0b01);
        }

        #[test]
        fn test_mask_or() {
            assert_eq!(
                ($mask::new(false, false) | $mask::new(false, false)).bitmask(),
                0b00,
            );
            assert_eq!(
                ($mask::new(false, false) | $mask::new(false, true)).bitmask(),
                0b10,
            );
            assert_eq!(
                ($mask::new(true, false) | $mask::new(false, true)).bitmask(),
                0b11,
            );
            assert_eq!(
                ($mask::new(true, true) | $mask::new(true, true)).bitmask(),
                0b11,
            );

            let mut mask = $mask::new(true, true);
            mask |= $mask::new(true, false);
            assert_eq!(mask.bitmask(), 0b11);
        }

        #[test]
        fn test_mask_not() {
            assert_eq!((!$mask::new(false, false)).bitmask(), 0b11);
            assert_eq!((!$mask::new(true, false)).bitmask(), 0b10);
            assert_eq!((!$mask::new(false, true)).bitmask(), 0b01);
            assert_eq!((!$mask::new(true, true)).bitmask(), 0b00);
        }

        #[test]
        fn test_mask_fmt() {
            let a = $mask::new(true, false);

            assert_eq!(
                format!("{:?}", a),
                format!("{}(0xffffffff, 0x0)", stringify!($mask))
            );
            assert_eq!(format!("{}", a), "[true, false]");
        }

        #[test]
        fn test_mask_eq() {
            let a = $mask::new(true, false);
            let b = $mask::new(true, false);
            let c = $mask::new(false, true);

            assert_eq!(a, b);
            assert_eq!(b, a);
            assert_ne!(a, c);
            assert_ne!(b, c);
        }

        #[test]
        fn test_mask_hash() {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let a = $mask::new(true, false);
            let b = $mask::new(true, false);
            let c = $mask::new(false, true);

            let mut hasher = DefaultHasher::new();
            a.hash(&mut hasher);
            let a_hashed = hasher.finish();

            let mut hasher = DefaultHasher::new();
            b.hash(&mut hasher);
            let b_hashed = hasher.finish();

            let mut hasher = DefaultHasher::new();
            c.hash(&mut hasher);
            let c_hashed = hasher.finish();

            assert_eq!(a, b);
            assert_eq!(a_hashed, b_hashed);
            assert_ne!(a, c);
            assert_ne!(a_hashed, c_hashed);
        }

        #[test]
        fn test_to_from_slice() {
            let v = $vec2::new(1 as $t, 2 as $t);
            let mut a = [0 as $t, 0 as $t];
            v.write_to_slice(&mut a);
            assert_eq!(v, $vec2::from_slice(&a));
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_sum() {
            let one = $vec2::ONE;
            assert_eq!(vec![one, one].iter().sum::<$vec2>(), one + one);
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_product() {
            let two = $vec2::new(2 as $t, 2 as $t);
            assert_eq!(vec![two, two].iter().product::<$vec2>(), two * two);
        }
    };
}

macro_rules! impl_vec2_signed_tests {
    ($t:ident, $const_new:ident, $new:ident, $vec2:ident, $vec3:ident, $mask:ident) => {
        impl_vec2_tests!($t, $const_new, $new, $vec2, $vec3, $mask);

        #[test]
        fn test_dot_signed() {
            let x = $new(1 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t);
            assert_eq!(1 as $t, x.dot(x));
            assert_eq!(0 as $t, x.dot(y));
            assert_eq!(-1 as $t, x.dot(-x));
        }

        #[test]
        fn test_neg() {
            let a = $new(1 as $t, 2 as $t);
            assert_eq!($new(-1 as $t, -2 as $t), (-a));
        }
    };
}

macro_rules! impl_vec2_eq_hash_tests {
    ($t:ident, $new:ident) => {
        #[test]
        fn test_ve2_hash() {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let a = $new(1 as $t, 2 as $t);
            let b = $new(1 as $t, 2 as $t);
            let c = $new(3 as $t, 2 as $t);

            let mut hasher = DefaultHasher::new();
            a.hash(&mut hasher);
            let a_hashed = hasher.finish();

            let mut hasher = DefaultHasher::new();
            b.hash(&mut hasher);
            let b_hashed = hasher.finish();

            let mut hasher = DefaultHasher::new();
            c.hash(&mut hasher);
            let c_hashed = hasher.finish();

            assert_eq!(a, b);
            assert_eq!(a_hashed, b_hashed);
            assert_ne!(a, c);
            assert_ne!(a_hashed, c_hashed);
        }
    };
}

macro_rules! impl_vec2_float_tests {
    ($t:ident, $const_new:ident, $new:ident, $vec2:ident, $vec3:ident, $mask:ident, $mat2:ident) => {
        impl_vec2_signed_tests!($t, $const_new, $new, $vec2, $vec3, $mask);
        impl_vec_float_normalize_tests!($t, $vec2);

        use core::$t::INFINITY;
        use core::$t::NAN;
        use core::$t::NEG_INFINITY;

        #[test]
        fn test_vec2_consts() {
            assert_eq!($vec2::ZERO, $new(0 as $t, 0 as $t));
            assert_eq!($vec2::ONE, $new(1 as $t, 1 as $t));
            assert_eq!($vec2::X, $new(1 as $t, 0 as $t));
            assert_eq!($vec2::Y, $new(0 as $t, 1 as $t));
        }

        #[test]
        fn test_length() {
            let x = $new(1.0, 0.0);
            let y = $new(0.0, 1.0);
            assert_eq!(4.0, (2.0 * x).length_squared());
            assert_eq!(9.0, (-3.0 * y).length_squared());
            assert_eq!(2.0, (-2.0 * x).length());
            assert_eq!(3.0, (3.0 * y).length());
            assert_eq!(2.0, x.distance_squared(y));
            assert_eq!(13.0, (2.0 * x).distance_squared(-3.0 * y));
            assert_eq!((2.0 as $t).sqrt(), x.distance(y));
            assert_eq!(5.0, (3.0 * x).distance(-4.0 * y));
            assert_eq!(13.0, (-5.0 * x).distance(12.0 * y));
            assert_eq!(x, (2.0 * x).normalize());
            assert_eq!(1.0 * 3.0 + 2.0 * 4.0, $new(1.0, 2.0).dot($new(3.0, 4.0)));
            assert_eq!(2.0 * 2.0 + 3.0 * 3.0, $new(2.0, 3.0).length_squared());
            assert_eq!(
                (2.0 as $t * 2.0 + 3.0 * 3.0).sqrt(),
                $new(2.0, 3.0).length()
            );
            assert_eq!(
                1.0 / (2.0 as $t * 2.0 + 3.0 * 3.0).sqrt(),
                $new(2.0, 3.0).length_recip()
            );
            assert!($new(2.0, 3.0).normalize().is_normalized());
            assert_eq!(
                $new(2.0, 3.0) / (2.0 as $t * 2.0 + 3.0 * 3.0).sqrt(),
                $new(2.0, 3.0).normalize()
            );
            assert_eq!($new(0.5, 0.25), $new(2.0, 4.0).recip());
        }

        #[test]
        fn test_perp() {
            let v1 = $vec2::new(1.0, 2.0);
            let v2 = $vec2::new(1.0, 1.0);
            let v1_perp = $vec2::new(-2.0, 1.0);
            let rot90 = $mat2::from_angle($t::to_radians(90.0));

            assert_eq!(v1_perp, v1.perp());
            assert_eq!(v1.perp().dot(v1), 0.0);
            assert_eq!(v2.perp().dot(v2), 0.0);
            assert_eq!(v1.perp().dot(v2), v1.perp_dot(v2));

            assert_approx_eq!(v1.perp(), rot90 * v1);
        }

        #[test]
        fn test_sign() {
            assert_eq!($vec2::ZERO.signum(), $vec2::ONE);
            assert_eq!(-$vec2::ZERO.signum(), -$vec2::ONE);
            assert_eq!($vec2::ONE.signum(), $vec2::ONE);
            assert_eq!((-$vec2::ONE).signum(), -$vec2::ONE);
            assert_eq!($vec2::splat(INFINITY).signum(), $vec2::ONE);
            assert_eq!($vec2::splat(NEG_INFINITY).signum(), -$vec2::ONE);
            assert!($vec2::splat(NAN).signum().is_nan_mask().all());
        }

        #[test]
        fn test_abs() {
            assert_eq!($vec2::ZERO.abs(), $vec2::ZERO);
            assert_eq!($vec2::ONE.abs(), $vec2::ONE);
            assert_eq!((-$vec2::ONE).abs(), $vec2::ONE);
        }

        #[test]
        fn test_round() {
            assert_eq!($vec2::new(1.35, 0.0).round().x, 1.0);
            assert_eq!($vec2::new(0.0, 1.5).round().y, 2.0);
            assert_eq!($vec2::new(0.0, -15.5).round().y, -16.0);
            assert_eq!($vec2::new(0.0, 0.0).round().y, 0.0);
            assert_eq!($vec2::new(0.0, 21.1).round().y, 21.0);
            assert_eq!($vec2::new(0.0, 11.123).round().y, 11.0);
            assert_eq!($vec2::new(0.0, 11.499).round().y, 11.0);
            assert_eq!(
                $vec2::new(NEG_INFINITY, INFINITY).round(),
                $vec2::new(NEG_INFINITY, INFINITY)
            );
            assert!($vec2::new(NAN, 0.0).round().x.is_nan());
        }

        #[test]
        fn test_floor() {
            assert_eq!($vec2::new(1.35, -1.5).floor(), $vec2::new(1.0, -2.0));
            assert_eq!(
                $vec2::new(INFINITY, NEG_INFINITY).floor(),
                $vec2::new(INFINITY, NEG_INFINITY)
            );
            assert!($vec2::new(NAN, 0.0).floor().x.is_nan());
            assert_eq!(
                $vec2::new(-2000000.123, 10000000.123).floor(),
                $vec2::new(-2000001.0, 10000000.0)
            );
        }

        #[test]
        fn test_ceil() {
            assert_eq!($vec2::new(1.35, -1.5).ceil(), $vec2::new(2.0, -1.0));
            assert_eq!(
                $vec2::new(INFINITY, NEG_INFINITY).ceil(),
                $vec2::new(INFINITY, NEG_INFINITY)
            );
            assert!($vec2::new(NAN, 0.0).ceil().x.is_nan());
            assert_eq!(
                $vec2::new(-2000000.123, 1000000.123).ceil(),
                $vec2::new(-2000000.0, 1000001.0)
            );
        }

        #[test]
        fn test_lerp() {
            let v0 = $vec2::new(-1.0, -1.0);
            let v1 = $vec2::new(1.0, 1.0);
            assert_approx_eq!(v0, v0.lerp(v1, 0.0));
            assert_approx_eq!(v1, v0.lerp(v1, 1.0));
            assert_approx_eq!($vec2::ZERO, v0.lerp(v1, 0.5));
        }

        #[test]
        fn test_is_finite() {
            assert!($vec2::new(0.0, 0.0).is_finite());
            assert!($vec2::new(-1e-10, 1e10).is_finite());
            assert!(!$vec2::new(INFINITY, 0.0).is_finite());
            assert!(!$vec2::new(0.0, NAN).is_finite());
            assert!(!$vec2::new(0.0, NEG_INFINITY).is_finite());
            assert!(!$vec2::new(INFINITY, NEG_INFINITY).is_finite());
        }

        #[test]
        fn test_powf() {
            assert_eq!($vec2::new(2.0, 4.0).powf(2.0), $vec2::new(4.0, 16.0));
        }

        #[test]
        fn test_exp() {
            assert_eq!(
                $vec2::new(1.0, 2.0).exp(),
                $vec2::new((1.0 as $t).exp(), (2.0 as $t).exp())
            );
        }

        #[test]
        fn test_angle_between() {
            let angle = $vec2::new(1.0, 0.0).angle_between($vec2::new(0.0, 1.0));
            assert_approx_eq!(core::$t::consts::FRAC_PI_2, angle, 1e-6);

            let angle = $vec2::new(10.0, 0.0).angle_between($vec2::new(0.0, 5.0));
            assert_approx_eq!(core::$t::consts::FRAC_PI_2, angle, 1e-6);

            let angle = $vec2::new(-1.0, 0.0).angle_between($vec2::new(0.0, 1.0));
            assert_approx_eq!(-core::$t::consts::FRAC_PI_2, angle, 1e-6);
        }

        #[test]
        fn test_clamp_length() {
            // Too long gets shortened
            assert_eq!(
                $vec2::new(12.0, 16.0).clamp_length(7.0, 10.0),
                $vec2::new(6.0, 8.0) // shortened to length 10.0
            );
            // In the middle is unchanged
            assert_eq!(
                $vec2::new(2.0, 1.0).clamp_length(0.5, 5.0),
                $vec2::new(2.0, 1.0) // unchanged
            );
            // Too short gets lengthened
            assert_eq!(
                $vec2::new(0.6, 0.8).clamp_length(10.0, 20.0),
                $vec2::new(6.0, 8.0) // lengthened to length 10.0
            );
        }

        #[test]
        fn test_clamp_length_max() {
            // Too long gets shortened
            assert_eq!(
                $vec2::new(12.0, 16.0).clamp_length_max(10.0),
                $vec2::new(6.0, 8.0) // shortened to length 10.0
            );
            // Not too long is unchanged
            assert_eq!(
                $vec2::new(2.0, 1.0).clamp_length_max(5.0),
                $vec2::new(2.0, 1.0) // unchanged
            );
        }

        #[test]
        fn test_clamp_length_min() {
            // Not too short is unchanged
            assert_eq!(
                $vec2::new(2.0, 1.0).clamp_length_min(0.5),
                $vec2::new(2.0, 1.0) // unchanged
            );
            // Too short gets lengthened
            assert_eq!(
                $vec2::new(0.6, 0.8).clamp_length_min(10.0),
                $vec2::new(6.0, 8.0) // lengthened to length 10.0
            );
        }
    };
}

mod vec2 {
    use glam::{const_vec2, vec2, BVec2, Mat2, Vec2, Vec3};

    #[test]
    fn test_align() {
        use core::mem;
        assert_eq!(8, mem::size_of::<Vec2>());
        assert_eq!(4, mem::align_of::<Vec2>());
        assert_eq!(2, mem::size_of::<BVec2>());
        assert_eq!(1, mem::align_of::<BVec2>());
    }

    #[test]
    fn test_as() {
        use glam::{DVec2, IVec2, UVec2};
        assert_eq!(DVec2::new(-1.0, -2.0), Vec2::new(-1.0, -2.0).as_f64());
        assert_eq!(IVec2::new(-1, -2), Vec2::new(-1.0, -2.0).as_i32());
        assert_eq!(UVec2::new(1, 2), Vec2::new(1.0, 2.0).as_u32());

        assert_eq!(IVec2::new(-1, -2), DVec2::new(-1.0, -2.0).as_i32());
        assert_eq!(UVec2::new(1, 2), DVec2::new(1.0, 2.0).as_u32());
        assert_eq!(Vec2::new(-1.0, -2.0), DVec2::new(-1.0, -2.0).as_f32());

        assert_eq!(DVec2::new(-1.0, -2.0), IVec2::new(-1, -2).as_f64());
        assert_eq!(UVec2::new(1, 2), IVec2::new(1, 2).as_u32());
        assert_eq!(Vec2::new(-1.0, -2.0), IVec2::new(-1, -2).as_f32());

        assert_eq!(DVec2::new(1.0, 2.0), UVec2::new(1, 2).as_f64());
        assert_eq!(IVec2::new(1, 2), UVec2::new(1, 2).as_i32());
        assert_eq!(Vec2::new(1.0, 2.0), UVec2::new(1, 2).as_f32());
    }

    impl_vec2_float_tests!(f32, const_vec2, vec2, Vec2, Vec3, BVec2, Mat2);
}

mod dvec2 {
    use glam::{const_dvec2, dvec2, BVec2, DMat2, DVec2, DVec3};

    #[test]
    fn test_align() {
        use core::mem;
        assert_eq!(16, mem::size_of::<DVec2>());
        assert_eq!(8, mem::align_of::<DVec2>());
        assert_eq!(2, mem::size_of::<BVec2>());
        assert_eq!(1, mem::align_of::<BVec2>());
    }

    impl_vec2_float_tests!(f64, const_dvec2, dvec2, DVec2, DVec3, BVec2, DMat2);
}

mod ivec2 {
    use glam::{const_ivec2, ivec2, BVec2, IVec2, IVec3};

    #[test]
    fn test_align() {
        use core::mem;
        assert_eq!(8, mem::size_of::<IVec2>());
        assert_eq!(4, mem::align_of::<IVec2>());
        assert_eq!(2, mem::size_of::<BVec2>());
        assert_eq!(1, mem::align_of::<BVec2>());
    }

    impl_vec2_signed_tests!(i32, const_ivec2, ivec2, IVec2, IVec3, BVec2);
    impl_vec2_eq_hash_tests!(i32, ivec2);
}

mod uvec2 {
    use glam::{const_uvec2, uvec2, BVec2, UVec2, UVec3};

    #[test]
    fn test_align() {
        use core::mem;
        assert_eq!(8, mem::size_of::<UVec2>());
        assert_eq!(4, mem::align_of::<UVec2>());
        assert_eq!(2, mem::size_of::<BVec2>());
        assert_eq!(1, mem::align_of::<BVec2>());
    }

    impl_vec2_tests!(u32, const_uvec2, uvec2, UVec2, UVec3, BVec2);
    impl_vec2_eq_hash_tests!(u32, uvec2);
}
