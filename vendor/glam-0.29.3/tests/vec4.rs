#![allow(clippy::excessive_precision)]

#[macro_use]
mod support;

macro_rules! impl_vec4_tests {
    ($t:ident, $new:ident, $vec4:ident, $vec3:ident, $vec2:ident, $mask:ident, $masknew:ident) => {
        glam_test!(test_const, {
            const V0: $vec4 = $vec4::splat(1 as $t);
            const V1: $vec4 = $vec4::new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            const V2: $vec4 = $vec4::from_array([1 as $t, 2 as $t, 3 as $t, 4 as $t]);
            assert_eq!([1 as $t, 1 as $t, 1 as $t, 1 as $t], *V0.as_ref());
            assert_eq!([1 as $t, 2 as $t, 3 as $t, 4 as $t], *V1.as_ref());
            assert_eq!([1 as $t, 2 as $t, 3 as $t, 4 as $t], *V2.as_ref());
        });

        glam_test!(test_vec4_consts, {
            assert_eq!($vec4::ZERO, $new(0 as $t, 0 as $t, 0 as $t, 0 as $t));
            assert_eq!($vec4::ONE, $new(1 as $t, 1 as $t, 1 as $t, 1 as $t));
            assert_eq!($vec4::X, $new(1 as $t, 0 as $t, 0 as $t, 0 as $t));
            assert_eq!($vec4::Y, $new(0 as $t, 1 as $t, 0 as $t, 0 as $t));
            assert_eq!($vec4::Z, $new(0 as $t, 0 as $t, 1 as $t, 0 as $t));
            assert_eq!($vec4::W, $new(0 as $t, 0 as $t, 0 as $t, 1 as $t));
            assert_eq!($vec4::MIN, $new($t::MIN, $t::MIN, $t::MIN, $t::MIN));
            assert_eq!($vec4::MAX, $new($t::MAX, $t::MAX, $t::MAX, $t::MAX));
        });

        glam_test!(test_new, {
            let v = $new(1 as $t, 2 as $t, 3 as $t, 4 as $t);

            assert_eq!(v.x, 1 as $t);
            assert_eq!(v.y, 2 as $t);
            assert_eq!(v.z, 3 as $t);
            assert_eq!(v.w, 4 as $t);

            let t = (1 as $t, 2 as $t, 3 as $t, 4 as $t);
            let v = $vec4::from(t);
            assert_eq!(t, v.into());

            let a = [1 as $t, 2 as $t, 3 as $t, 4 as $t];
            let v = $vec4::from(a);
            let a1: [$t; 4] = v.into();
            assert_eq!(a, a1);

            assert_eq!(a, v.to_array());
            assert_eq!(a, *v.as_ref());

            let mut v2 = $vec4::default();
            *v2.as_mut() = a;
            assert_eq!(a, v2.to_array());

            let v = $vec4::new(t.0, t.1, t.2, t.3);
            assert_eq!(t, v.into());

            assert_eq!(
                $vec4::new(1 as $t, 0 as $t, 0 as $t, 0 as $t),
                glam::BVec4::new(true, false, false, false).into()
            );
            assert_eq!(
                $vec4::new(0 as $t, 1 as $t, 0 as $t, 0 as $t),
                glam::BVec4::new(false, true, false, false).into()
            );
            assert_eq!(
                $vec4::new(0 as $t, 0 as $t, 1 as $t, 0 as $t),
                glam::BVec4::new(false, false, true, false).into()
            );

            #[cfg(not(feature = "scalar-math"))]
            {
                assert_eq!(
                    $vec4::new(0 as $t, 0 as $t, 0 as $t, 1 as $t),
                    glam::BVec4::new(false, false, false, true).into()
                );
                assert_eq!(
                    $vec4::new(1 as $t, 0 as $t, 0 as $t, 0 as $t),
                    glam::BVec4A::new(true, false, false, false).into()
                );
                assert_eq!(
                    $vec4::new(0 as $t, 1 as $t, 0 as $t, 0 as $t),
                    glam::BVec4A::new(false, true, false, false).into()
                );
                assert_eq!(
                    $vec4::new(0 as $t, 0 as $t, 1 as $t, 0 as $t),
                    glam::BVec4A::new(false, false, true, false).into()
                );
                assert_eq!(
                    $vec4::new(0 as $t, 0 as $t, 0 as $t, 1 as $t),
                    glam::BVec4A::new(false, false, false, true).into()
                );
            }

            assert_eq!($vec4::new(1 as $t, 0 as $t, 0 as $t, 0 as $t), $vec4::X);
            assert_eq!($vec4::new(0 as $t, 1 as $t, 0 as $t, 0 as $t), $vec4::Y);
            assert_eq!($vec4::new(0 as $t, 0 as $t, 1 as $t, 0 as $t), $vec4::Z);
            assert_eq!($vec4::new(0 as $t, 0 as $t, 0 as $t, 1 as $t), $vec4::W);

            assert_eq!(
                v,
                $vec4::from(($vec3::new(1 as $t, 2 as $t, 3 as $t), 4 as $t))
            );

            assert_eq!(
                v,
                $vec4::from((1 as $t, $vec3::new(2 as $t, 3 as $t, 4 as $t)))
            );

            assert_eq!(
                v,
                $vec4::from(($vec2::new(1 as $t, 2 as $t), 3 as $t, 4 as $t))
            );
            assert_eq!(
                v,
                $vec4::from(($vec2::new(1 as $t, 2 as $t), $vec2::new(3 as $t, 4 as $t)))
            );
        });

        glam_test!(test_fmt, {
            let a = $vec4::new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            assert_eq!(
                format!("{:?}", a),
                format!(
                    "{}({:?}, {:?}, {:?}, {:?})",
                    stringify!($vec4),
                    a.x,
                    a.y,
                    a.z,
                    a.w
                )
            );
            assert_eq!(
                format!("{:#?}", a),
                format!(
                    "{}(\n    {:#?},\n    {:#?},\n    {:#?},\n    {:#?},\n)",
                    stringify!($vec4),
                    a.x,
                    a.y,
                    a.z,
                    a.w
                )
            );
            assert_eq!(format!("{}", a), "[1, 2, 3, 4]");
        });

        glam_test!(test_zero, {
            let v = $vec4::ZERO;
            assert_eq!((0 as $t, 0 as $t, 0 as $t, 0 as $t), v.into());
            assert_eq!(v, $vec4::default());
        });

        glam_test!(test_splat, {
            let v = $vec4::splat(1 as $t);
            assert_eq!($vec4::ONE, v);
        });

        glam_test!(test_map, {
            let v = $vec4::new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            assert_eq!(v.map(|n| n + 3 as $t), v + $vec4::splat(3 as $t));
            assert_eq!(v.map(|_| 0 as $t), $vec4::ZERO);
        });

        glam_test!(test_with, {
            assert_eq!($vec4::X, $vec4::ZERO.with_x(1 as $t));
            assert_eq!($vec4::Y, $vec4::ZERO.with_y(1 as $t));
            assert_eq!($vec4::Z, $vec4::ZERO.with_z(1 as $t));
            assert_eq!($vec4::W, $vec4::ZERO.with_w(1 as $t));
        });

        glam_test!(test_accessors, {
            let mut a = $vec4::ZERO;
            a.x = 1 as $t;
            a.y = 2 as $t;
            a.z = 3 as $t;
            a.w = 4 as $t;
            assert_eq!(1 as $t, a.x);
            assert_eq!(2 as $t, a.y);
            assert_eq!(3 as $t, a.z);
            assert_eq!(4 as $t, a.w);
            assert_eq!((1 as $t, 2 as $t, 3 as $t, 4 as $t), a.into());

            let mut a = $vec4::ZERO;
            a[0] = 1 as $t;
            a[1] = 2 as $t;
            a[2] = 3 as $t;
            a[3] = 4 as $t;
            assert_eq!(1 as $t, a[0]);
            assert_eq!(2 as $t, a[1]);
            assert_eq!(3 as $t, a[2]);
            assert_eq!(4 as $t, a[3]);
            assert_eq!((1 as $t, 2 as $t, 3 as $t, 4 as $t), a.into());
        });

        glam_test!(test_dot_unsigned, {
            let x = $new(1 as $t, 0 as $t, 0 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t, 0 as $t, 0 as $t);
            let z = $new(0 as $t, 0 as $t, 1 as $t, 0 as $t);
            let w = $new(0 as $t, 0 as $t, 0 as $t, 1 as $t);
            assert_eq!(1 as $t, x.dot(x));
            assert_eq!(0 as $t, x.dot(y));
            assert_eq!(0 as $t, y.dot(z));
            assert_eq!(0 as $t, z.dot(w));

            assert_eq!(
                $new(28 as $t, 28 as $t, 28 as $t, 28 as $t),
                $new(0 as $t, 5 as $t, 3 as $t, 6 as $t)
                    .dot_into_vec($new(7 as $t, 2 as $t, 4 as $t, 1 as $t))
            );
        });

        glam_test!(test_length_squared_unsigned, {
            let x = $new(1 as $t, 0 as $t, 0 as $t, 0 as $t);
            let z = $new(0 as $t, 0 as $t, 1 as $t, 0 as $t);
            let w = $new(0 as $t, 0 as $t, 0 as $t, 1 as $t);
            assert_eq!(4 as $t, (2 as $t * x).length_squared());
            assert_eq!(16 as $t, (4 as $t * z).length_squared());
            assert_eq!(64 as $t, (8 as $t * w).length_squared());
            assert_eq!(
                2 as $t * 2 as $t + 3 as $t * 3 as $t + 4 as $t * 4 as $t + 5 as $t * 5 as $t,
                $new(2 as $t, 3 as $t, 4 as $t, 5 as $t).length_squared()
            );
        });

        glam_test!(test_ops, {
            let a = $new(2 as $t, 4 as $t, 8 as $t, 10 as $t);
            assert_eq!($new(4 as $t, 8 as $t, 16 as $t, 20 as $t), a + a);
            assert_eq!($new(2 as $t, 4 as $t, 8 as $t, 10 as $t), 0 as $t + a);
            assert_eq!($new(0 as $t, 0 as $t, 0 as $t, 0 as $t), a - a);
            assert_eq!($new(8 as $t, 6 as $t, 2 as $t, 0 as $t), 10 as $t - a);
            assert_eq!($new(4 as $t, 16 as $t, 64 as $t, 100 as $t), a * a);
            assert_eq!($new(4 as $t, 8 as $t, 16 as $t, 20 as $t), a * 2 as $t);
            assert_eq!($new(4 as $t, 8 as $t, 16 as $t, 20 as $t), 2 as $t * a);
            assert_eq!($new(1 as $t, 1 as $t, 1 as $t, 1 as $t), a / a);
            assert_eq!($new(1 as $t, 2 as $t, 4 as $t, 5 as $t), a / 2 as $t);
            assert_eq!($new(8 as $t, 4 as $t, 2 as $t, 1.6 as $t), 16 as $t / a);
            assert_eq!($new(0 as $t, 0 as $t, 0 as $t, 0 as $t), a % a);
            assert_eq!($new(0 as $t, 1 as $t, 1 as $t, 1 as $t), a % (a - 1 as $t));
            assert_eq!($new(0 as $t, 0 as $t, 0 as $t, 0 as $t), a % 1 as $t);
            assert_eq!($new(2 as $t, 1 as $t, 2 as $t, 1 as $t), a % 3 as $t);
            assert_eq!($new(1 as $t, 1 as $t, 1 as $t, 7 as $t), 17 as $t % a);
            assert_eq!($new(2 as $t, 4 as $t, 0 as $t, 2 as $t), a % 8 as $t);
        });

        glam_test!(test_ops_propagated, {
            let vec = $new(2 as $t, 4 as $t, 8 as $t, 10 as $t);
            let scalar = 2 as $t;
            let g_scalar = 10 as $t;

            assert_eq!((vec + vec), (vec + &vec));
            assert_eq!((vec + vec), (&vec + vec));
            assert_eq!((vec + vec), (&vec + &vec));
            assert_eq!((vec + scalar), (vec + &scalar));
            assert_eq!((vec + scalar), (&vec + &scalar));
            assert_eq!((vec + scalar), (&vec + scalar));
            assert_eq!((scalar + vec), (&scalar + vec));
            assert_eq!((scalar + vec), (&scalar + &vec));
            assert_eq!((scalar + vec), (scalar + &vec));

            assert_eq!((vec - vec), (vec - &vec));
            assert_eq!((vec - vec), (&vec - vec));
            assert_eq!((vec - vec), (&vec - &vec));
            assert_eq!((vec - scalar), (vec - &scalar));
            assert_eq!((vec - scalar), (&vec - &scalar));
            assert_eq!((vec - scalar), (&vec - scalar));
            assert_eq!((g_scalar - vec), (&g_scalar - vec));
            assert_eq!((g_scalar - vec), (&g_scalar - &vec));
            assert_eq!((g_scalar - vec), (g_scalar - &vec));

            assert_eq!((vec * vec), (vec * &vec));
            assert_eq!((vec * vec), (&vec * vec));
            assert_eq!((vec * vec), (&vec * &vec));
            assert_eq!((vec * scalar), (vec * &scalar));
            assert_eq!((vec * scalar), (&vec * &scalar));
            assert_eq!((vec * scalar), (&vec * scalar));
            assert_eq!((scalar * vec), (&scalar * vec));
            assert_eq!((scalar * vec), (&scalar * &vec));
            assert_eq!((scalar * vec), (scalar * &vec));

            assert_eq!((vec / vec), (vec / &vec));
            assert_eq!((vec / vec), (&vec / vec));
            assert_eq!((vec / vec), (&vec / &vec));
            assert_eq!((vec / scalar), (vec / &scalar));
            assert_eq!((vec / scalar), (&vec / &scalar));
            assert_eq!((vec / scalar), (&vec / scalar));
            assert_eq!((scalar / vec), (&scalar / vec));
            assert_eq!((scalar / vec), (&scalar / &vec));
            assert_eq!((scalar / vec), (scalar / &vec));

            assert_eq!((vec % vec), (vec % &vec));
            assert_eq!((vec % vec), (&vec % vec));
            assert_eq!((vec % vec), (&vec % &vec));
            assert_eq!((vec % scalar), (vec % &scalar));
            assert_eq!((vec % scalar), (&vec % &scalar));
            assert_eq!((vec % scalar), (&vec % scalar));
            assert_eq!((scalar % vec), (&scalar % vec));
            assert_eq!((scalar % vec), (&scalar % &vec));
            assert_eq!((scalar % vec), (scalar % &vec));
        });

        glam_test!(test_assign_ops, {
            let a = $new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            let mut b = a;

            b += 2 as $t;
            assert_eq!($new(3 as $t, 4 as $t, 5 as $t, 6 as $t), b);
            b -= 2 as $t;
            assert_eq!($new(1 as $t, 2 as $t, 3 as $t, 4 as $t), b);
            b *= 2 as $t;
            assert_eq!($new(2 as $t, 4 as $t, 6 as $t, 8 as $t), b);
            b /= 2 as $t;
            assert_eq!($new(1 as $t, 2 as $t, 3 as $t, 4 as $t), b);
            b %= 2 as $t;
            assert_eq!($new(1 as $t, 0 as $t, 1 as $t, 0 as $t), b);

            b = a;
            b += a;
            assert_eq!($new(2 as $t, 4 as $t, 6 as $t, 8 as $t), b);
            b -= a;
            assert_eq!($new(1 as $t, 2 as $t, 3 as $t, 4 as $t), b);
            b *= a;
            assert_eq!($new(1 as $t, 4 as $t, 9 as $t, 16 as $t), b);
            b /= a;
            assert_eq!($new(1 as $t, 2 as $t, 3 as $t, 4 as $t), b);
            b *= 2 as $t;
            assert_eq!($new(2 as $t, 4 as $t, 6 as $t, 8 as $t), b);
            b /= 2 as $t;
            assert_eq!($new(1 as $t, 2 as $t, 3 as $t, 4 as $t), b);
            b %= (b + 1 as $t);
            assert_eq!($new(1 as $t, 2 as $t, 3 as $t, 4 as $t), b);
            b %= b;
            assert_eq!($new(0 as $t, 0 as $t, 0 as $t, 0 as $t), b);
        });

        glam_test!(test_assign_ops_propagation, {
            let vec = $new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            let mut a = vec;
            let mut b = vec;
            let scalar = 2 as $t;

            a += &scalar;
            b += scalar;
            assert_eq!(b, a, "AddAssign<Scalar>");
            a -= &scalar;
            b -= scalar;
            assert_eq!(b, a, "SubAssign<Scalar>");
            a *= &scalar;
            b *= scalar;
            assert_eq!(b, a, "MulAssign<Scalar>");
            a /= &scalar;
            b /= scalar;
            assert_eq!(b, a, "DivAssign<Scalar>");
            a %= &scalar;
            b %= scalar;
            assert_eq!(b, a, "MulAssign<Scalar>");

            a = vec;
            b = vec;
            a += &vec;
            b += vec;
            assert_eq!(b, a, "AddAssign<Vec>");
            a -= &vec;
            b -= vec;
            assert_eq!(b, a, "SubAssign<Vec>");
            a *= &vec;
            b *= vec;
            assert_eq!(b, a, "MulAssign<Vec>");
            a /= &vec;
            b /= vec;
            assert_eq!(b, a, "DivAssign<Vec>");
            a %= &vec;
            b %= vec;
            assert_eq!(b, a, "RemAssign<Vec>");
        });

        glam_test!(test_min_max, {
            let a = $new(4 as $t, 6 as $t, 2 as $t, 8 as $t);
            let b = $new(5 as $t, 3 as $t, 7 as $t, 1 as $t);
            assert_eq!((4 as $t, 3 as $t, 2 as $t, 1 as $t), a.min(b).into());
            assert_eq!((4 as $t, 3 as $t, 2 as $t, 1 as $t), b.min(a).into());
            assert_eq!((5 as $t, 6 as $t, 7 as $t, 8 as $t), a.max(b).into());
            assert_eq!((5 as $t, 6 as $t, 7 as $t, 8 as $t), b.max(a).into());
        });

        glam_test!(test_clamp, {
            fn vec(x: i32, y: i32, z: i32, w: i32) -> $vec4 {
                $vec4::new(x as $t, y as $t, z as $t, w as $t)
            }
            let min = vec(1, 1, 3, 3);
            let max = vec(6, 6, 8, 8);
            assert_eq!(vec(0, 0, 0, 0).clamp(min, max), vec(1, 1, 3, 3));
            assert_eq!(vec(2, 2, 2, 2).clamp(min, max), vec(2, 2, 3, 3));
            assert_eq!(vec(4, 4, 5, 5).clamp(min, max), vec(4, 4, 5, 5));
            assert_eq!(vec(6, 6, 6, 6).clamp(min, max), vec(6, 6, 6, 6));
            assert_eq!(vec(7, 7, 7, 7).clamp(min, max), vec(6, 6, 7, 7));
            assert_eq!(vec(9, 9, 9, 9).clamp(min, max), vec(6, 6, 8, 8));

            should_glam_assert!({ $vec4::clamp($vec4::ZERO, $vec4::ONE, $vec4::ZERO) });
        });

        glam_test!(test_hmin_hmax, {
            assert_eq!(
                1 as $t,
                $new(1 as $t, 2 as $t, 3 as $t, 4 as $t).min_element()
            );
            assert_eq!(
                1 as $t,
                $new(4 as $t, 1 as $t, 2 as $t, 3 as $t).min_element()
            );
            assert_eq!(
                1 as $t,
                $new(3 as $t, 4 as $t, 1 as $t, 2 as $t).min_element()
            );
            assert_eq!(
                1 as $t,
                $new(2 as $t, 3 as $t, 4 as $t, 1 as $t).min_element()
            );
            assert_eq!(
                4 as $t,
                $new(1 as $t, 2 as $t, 3 as $t, 4 as $t).max_element()
            );
            assert_eq!(
                4 as $t,
                $new(4 as $t, 1 as $t, 2 as $t, 3 as $t).max_element()
            );
            assert_eq!(
                4 as $t,
                $new(3 as $t, 4 as $t, 1 as $t, 2 as $t).max_element()
            );
            assert_eq!(
                4 as $t,
                $new(2 as $t, 3 as $t, 4 as $t, 1 as $t).max_element()
            );
            assert_eq!(
                3 as $t,
                $new(1 as $t, 2 as $t, 3 as $t, 4 as $t)
                    .truncate()
                    .max_element()
            );
            assert_eq!(
                2 as $t,
                $new(4 as $t, 3 as $t, 2 as $t, 1 as $t)
                    .truncate()
                    .min_element()
            );
        });

        glam_test!(test_sum_product, {
            let a = $new(2 as $t, 3 as $t, 4 as $t, 5 as $t);
            assert_eq!(a.element_sum(), 14 as $t);
            assert_eq!(a.element_product(), 120 as $t);
        });

        glam_test!(test_eq, {
            let a = $new(1 as $t, 1 as $t, 1 as $t, 1 as $t);
            let b = $new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            assert!(a.cmpeq(a).all());
            assert!(b.cmpeq(b).all());
            assert!(a.cmpne(b).any());
            assert!(b.cmpne(a).any());
            assert!(b.cmpeq(a).any());
        });

        glam_test!(test_cmp, {
            assert!(!$mask::default().any());
            assert!(!$mask::default().all());
            assert_eq!($mask::default().bitmask(), 0x0);
            let a = $new(1 as $t, 1 as $t, 1 as $t, 1 as $t);
            let b = $new(2 as $t, 2 as $t, 2 as $t, 2 as $t);
            let c = $new(1 as $t, 1 as $t, 2 as $t, 2 as $t);
            let d = $new(2 as $t, 1 as $t, 1 as $t, 2 as $t);
            assert_eq!(a.cmplt(a).bitmask(), 0x0);
            assert_eq!(a.cmplt(b).bitmask(), 0xf);
            assert_eq!(a.cmplt(c).bitmask(), 0xc);
            assert_eq!(c.cmple(a).bitmask(), 0x3);
            assert_eq!(a.cmplt(d).bitmask(), 0x9);
            assert!(a.cmplt(b).all());
            assert!(a.cmplt(c).any());
            assert!(a.cmple(b).all());
            assert!(a.cmple(a).all());
            assert!(b.cmpgt(a).all());
            assert!(b.cmpge(a).all());
            assert!(b.cmpge(b).all());
            assert!(!(a.cmpge(c).all()));
            assert!(c.cmple(c).all());
            assert!(c.cmpge(c).all());
            assert!(a.cmpeq(a).all());
            assert!(!a.cmpeq(b).all());
            assert!(a.cmpeq(c).any());
            assert!(!a.cmpne(a).all());
            assert!(a.cmpne(b).all());
            assert!(a.cmpne(c).any());
            assert!(a == a);
        });

        glam_test!(test_slice, {
            let a = [1 as $t, 2 as $t, 3 as $t, 4 as $t];
            let b = $vec4::from_slice(&a);
            let c: [$t; 4] = b.into();
            assert_eq!(a, c);
            let mut d = [0 as $t, 0 as $t, 0 as $t, 0 as $t];
            b.write_to_slice(&mut d[..]);
            assert_eq!(a, d);

            should_panic!({ $vec4::ONE.write_to_slice(&mut [0 as $t; 3]) });
            should_panic!({ $vec4::from_slice(&[0 as $t; 3]) });
        });

        glam_test!(test_mask_new, {
            assert_eq!(
                $mask::new(false, false, false, false),
                $masknew(false, false, false, false)
            );
            assert_eq!(
                $mask::new(false, false, true, true),
                $masknew(false, false, true, true)
            );
            assert_eq!(
                $mask::new(true, true, false, false),
                $masknew(true, true, false, false)
            );
            assert_eq!(
                $mask::new(false, true, false, true),
                $masknew(false, true, false, true)
            );
            assert_eq!(
                $mask::new(true, false, true, false),
                $masknew(true, false, true, false)
            );
            assert_eq!(
                $mask::new(true, true, true, true),
                $masknew(true, true, true, true)
            );
        });

        glam_test!(test_mask_from_array_bool, {
            assert_eq!(
                $mask::new(false, false, false, false),
                $mask::from([false, false, false, false])
            );
            assert_eq!(
                $mask::new(false, false, true, true),
                $mask::from([false, false, true, true])
            );
            assert_eq!(
                $mask::new(true, true, false, false),
                $mask::from([true, true, false, false])
            );
            assert_eq!(
                $mask::new(false, true, false, true),
                $mask::from([false, true, false, true])
            );
            assert_eq!(
                $mask::new(true, false, true, false),
                $mask::from([true, false, true, false])
            );
            assert_eq!(
                $mask::new(true, true, true, true),
                $mask::from([true, true, true, true])
            );
        });

        glam_test!(test_mask_into_array_u32, {
            assert_eq!(
                Into::<[u32; 4]>::into($mask::new(false, false, false, false)),
                [0, 0, 0, 0]
            );
            assert_eq!(
                Into::<[u32; 4]>::into($mask::new(false, false, true, true)),
                [0, 0, !0, !0]
            );
            assert_eq!(
                Into::<[u32; 4]>::into($mask::new(true, true, false, false)),
                [!0, !0, 0, 0]
            );
            assert_eq!(
                Into::<[u32; 4]>::into($mask::new(false, true, false, true)),
                [0, !0, 0, !0]
            );
            assert_eq!(
                Into::<[u32; 4]>::into($mask::new(true, false, true, false)),
                [!0, 0, !0, 0]
            );
            assert_eq!(
                Into::<[u32; 4]>::into($mask::new(true, true, true, true)),
                [!0, !0, !0, !0]
            );
        });

        glam_test!(test_mask_into_array_bool, {
            assert_eq!(
                Into::<[bool; 4]>::into($mask::new(false, false, false, false)),
                [false, false, false, false]
            );
            assert_eq!(
                Into::<[bool; 4]>::into($mask::new(false, false, true, true)),
                [false, false, true, true]
            );
            assert_eq!(
                Into::<[bool; 4]>::into($mask::new(true, true, false, false)),
                [true, true, false, false]
            );
            assert_eq!(
                Into::<[bool; 4]>::into($mask::new(false, true, false, true)),
                [false, true, false, true]
            );
            assert_eq!(
                Into::<[bool; 4]>::into($mask::new(true, false, true, false)),
                [true, false, true, false]
            );
            assert_eq!(
                Into::<[bool; 4]>::into($mask::new(true, true, true, true)),
                [true, true, true, true]
            );
        });

        glam_test!(test_mask_splat, {
            assert_eq!($mask::splat(false), $mask::new(false, false, false, false));
            assert_eq!($mask::splat(true), $mask::new(true, true, true, true));
        });

        glam_test!(test_mask_bitmask, {
            assert_eq!($mask::new(false, false, false, false).bitmask(), 0b0000);
            assert_eq!($mask::new(false, false, true, true).bitmask(), 0b1100);
            assert_eq!($mask::new(true, true, false, false).bitmask(), 0b0011);
            assert_eq!($mask::new(false, true, false, true).bitmask(), 0b1010);
            assert_eq!($mask::new(true, false, true, false).bitmask(), 0b0101);
            assert_eq!($mask::new(true, true, true, true).bitmask(), 0b1111);
        });

        glam_test!(test_mask_any, {
            assert_eq!($mask::new(false, false, false, false).any(), false);
            assert_eq!($mask::new(true, false, false, false).any(), true);
            assert_eq!($mask::new(false, true, false, false).any(), true);
            assert_eq!($mask::new(false, false, true, false).any(), true);
            assert_eq!($mask::new(false, false, false, true).any(), true);
        });

        glam_test!(test_mask_all, {
            assert_eq!($mask::new(true, true, true, true).all(), true);
            assert_eq!($mask::new(false, true, true, true).all(), false);
            assert_eq!($mask::new(true, false, true, true).all(), false);
            assert_eq!($mask::new(true, true, false, true).all(), false);
            assert_eq!($mask::new(true, true, true, false).all(), false);
        });

        glam_test!(test_mask_select, {
            let a = $vec4::new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            let b = $vec4::new(5 as $t, 6 as $t, 7 as $t, 8 as $t);
            assert_eq!(
                $vec4::select($mask::new(true, true, true, true), a, b),
                $vec4::new(1 as $t, 2 as $t, 3 as $t, 4 as $t),
            );
            assert_eq!(
                $vec4::select($mask::new(true, false, true, false), a, b),
                $vec4::new(1 as $t, 6 as $t, 3 as $t, 8 as $t),
            );
            assert_eq!(
                $vec4::select($mask::new(false, true, false, true), a, b),
                $vec4::new(5 as $t, 2 as $t, 7 as $t, 4 as $t),
            );
            assert_eq!(
                $vec4::select($mask::new(false, false, false, false), a, b),
                $vec4::new(5 as $t, 6 as $t, 7 as $t, 8 as $t),
            );
        });

        glam_test!(test_mask_and, {
            assert_eq!(
                ($mask::new(false, false, false, false) & $mask::new(false, false, false, false))
                    .bitmask(),
                0b0000,
            );
            assert_eq!(
                ($mask::new(true, true, true, true) & $mask::new(true, true, true, true)).bitmask(),
                0b1111,
            );
            assert_eq!(
                ($mask::new(true, false, true, false) & $mask::new(false, true, false, true))
                    .bitmask(),
                0b0000,
            );
            assert_eq!(
                ($mask::new(true, false, true, true) & $mask::new(true, true, true, false))
                    .bitmask(),
                0b0101,
            );

            let mut mask = $mask::new(true, true, false, false);
            mask &= $mask::new(true, false, true, false);
            assert_eq!(mask.bitmask(), 0b0001);
        });

        glam_test!(test_mask_or, {
            assert_eq!(
                ($mask::new(false, false, false, false) | $mask::new(false, false, false, false))
                    .bitmask(),
                0b0000,
            );
            assert_eq!(
                ($mask::new(true, true, true, true) | $mask::new(true, true, true, true)).bitmask(),
                0b1111,
            );
            assert_eq!(
                ($mask::new(true, false, true, false) | $mask::new(false, true, false, true))
                    .bitmask(),
                0b1111,
            );
            assert_eq!(
                ($mask::new(true, false, true, false) | $mask::new(true, false, true, false))
                    .bitmask(),
                0b0101,
            );

            let mut mask = $mask::new(true, true, false, false);
            mask |= $mask::new(true, false, true, false);
            assert_eq!(mask.bitmask(), 0b0111);
        });

        glam_test!(test_mask_xor, {
            assert_eq!(
                ($mask::new(false, false, false, false) ^ $mask::new(false, false, false, false))
                    .bitmask(),
                0b0000,
            );
            assert_eq!(
                ($mask::new(true, true, true, true) ^ $mask::new(true, true, true, true)).bitmask(),
                0b0000,
            );
            assert_eq!(
                ($mask::new(true, false, true, false) ^ $mask::new(false, true, false, true))
                    .bitmask(),
                0b1111,
            );
            assert_eq!(
                ($mask::new(true, false, true, false) ^ $mask::new(true, false, true, false))
                    .bitmask(),
                0b0000,
            );

            let mut mask = $mask::new(true, true, false, false);
            mask ^= $mask::new(true, false, true, false);
            assert_eq!(mask.bitmask(), 0b0110);
        });

        glam_test!(test_mask_not, {
            assert_eq!((!$mask::new(false, false, false, false)).bitmask(), 0b1111);
            assert_eq!((!$mask::new(true, true, true, true)).bitmask(), 0b0000);
            assert_eq!((!$mask::new(true, false, true, false)).bitmask(), 0b1010);
            assert_eq!((!$mask::new(false, true, false, true)).bitmask(), 0b0101);
        });

        glam_test!(test_mask_fmt, {
            let a = $mask::new(true, false, true, false);

            assert_eq!(format!("{}", a), "[true, false, true, false]");
            assert_eq!(
                format!("{:?}", a),
                format!("{}(0xffffffff, 0x0, 0xffffffff, 0x0)", stringify!($mask))
            );
        });

        glam_test!(test_mask_eq, {
            let a = $mask::new(true, false, true, false);
            let b = $mask::new(true, false, true, false);
            let c = $mask::new(false, true, true, false);

            assert_eq!(a, b);
            assert_eq!(b, a);
            assert_ne!(a, c);
            assert_ne!(b, c);
        });

        glam_test!(test_mask_test, {
            let a = $mask::new(true, false, true, false);
            assert_eq!(a.test(0), true);
            assert_eq!(a.test(1), false);
            assert_eq!(a.test(2), true);
            assert_eq!(a.test(3), false);

            let b = $mask::new(false, true, false, true);
            assert_eq!(b.test(0), false);
            assert_eq!(b.test(1), true);
            assert_eq!(b.test(2), false);
            assert_eq!(b.test(3), true);
        });

        glam_test!(test_mask_set, {
            let mut a = $mask::new(false, true, false, true);
            a.set(0, true);
            assert_eq!(a.test(0), true);
            a.set(1, false);
            assert_eq!(a.test(1), false);
            a.set(2, true);
            assert_eq!(a.test(2), true);
            a.set(3, false);
            assert_eq!(a.test(3), false);

            let mut b = $mask::new(true, false, true, false);
            b.set(0, false);
            assert_eq!(b.test(0), false);
            b.set(1, true);
            assert_eq!(b.test(1), true);
            b.set(2, false);
            assert_eq!(b.test(2), false);
            b.set(3, true);
            assert_eq!(b.test(3), true);
        });

        glam_test!(test_mask_hash, {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let a = $mask::new(true, false, true, false);
            let b = $mask::new(true, false, true, false);
            let c = $mask::new(false, true, true, false);

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
        });

        glam_test!(test_to_from_slice, {
            let v = $vec4::new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            let mut a = [0 as $t, 0 as $t, 0 as $t, 0 as $t];
            v.write_to_slice(&mut a);
            assert_eq!(v, $vec4::from_slice(&a));

            let mut a = [0 as $t; 17];
            v.write_to_slice(&mut a);
            assert_eq!(v, $vec4::from_slice(&a[..4]));
            assert_eq!([0 as $t; 13], a[4..]);
        });

        glam_test!(test_sum, {
            let one = $vec4::ONE;
            assert_eq!([one, one].iter().sum::<$vec4>(), one + one);
            assert_eq!([one, one].into_iter().sum::<$vec4>(), one + one);
        });

        glam_test!(test_product, {
            let two = $vec4::new(2 as $t, 2 as $t, 2 as $t, 2 as $t);
            assert_eq!([two, two].iter().product::<$vec4>(), two * two);
            assert_eq!([two, two].into_iter().product::<$vec4>(), two * two);
        });
    };
}

macro_rules! impl_vec4_signed_tests {
    ($t:ident, $new:ident, $vec4:ident, $vec3:ident, $vec2:ident, $mask:ident, $masknew:ident) => {
        impl_vec4_tests!($t, $new, $vec4, $vec3, $vec2, $mask, $masknew);

        glam_test!(test_neg, {
            let a = $new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            assert_eq!((-1 as $t, -2 as $t, -3 as $t, -4 as $t), (-a).into());
            assert_eq!(
                $new(-0 as $t, -0 as $t, -0 as $t, -0 as $t),
                -$new(0 as $t, 0 as $t, 0 as $t, 0 as $t)
            );
            assert_eq!(
                $new(0 as $t, -0 as $t, -0 as $t, -0 as $t),
                -$new(-0 as $t, 0 as $t, 0 as $t, 0 as $t)
            );
        });

        glam_test!(test_neg_propagation, {
            let a = $new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            assert_eq!(-a, -(&a));
        });

        glam_test!(test_is_negative_bitmask, {
            assert_eq!($vec4::ZERO.is_negative_bitmask(), 0b0000);
            assert_eq!($vec4::ONE.is_negative_bitmask(), 0b0000);
            assert_eq!((-$vec4::ONE).is_negative_bitmask(), 0b1111);
            assert_eq!(
                $vec4::new(-1 as $t, 2 as $t, 3 as $t, -4 as $t).is_negative_bitmask(),
                0b1001
            );
            assert_eq!(
                $vec4::new(1 as $t, 5 as $t, -3 as $t, 7 as $t).is_negative_bitmask(),
                0b0100
            );
            assert_eq!(
                $vec4::new(3 as $t, -4 as $t, 1 as $t, 6 as $t).is_negative_bitmask(),
                0b0010
            );
            assert_eq!(
                $vec4::new(2 as $t, -6 as $t, 5 as $t, -3 as $t).is_negative_bitmask(),
                0b1010
            );
        });

        glam_test!(test_abs, {
            assert_eq!($vec4::ZERO.abs(), $vec4::ZERO);
            assert_eq!($vec4::ONE.abs(), $vec4::ONE);
            assert_eq!((-$vec4::ONE).abs(), $vec4::ONE);
        });

        glam_test!(test_dot_signed, {
            let x = $new(1 as $t, 0 as $t, 0 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t, 0 as $t, 0 as $t);
            let z = $new(0 as $t, 0 as $t, 1 as $t, 0 as $t);
            let w = $new(0 as $t, 0 as $t, 0 as $t, 1 as $t);
            assert_eq!(1 as $t, x.dot(x));
            assert_eq!(0 as $t, x.dot(y));
            assert_eq!(0 as $t, x.dot(-z));
            assert_eq!(-1 as $t, w.dot(-w));
        });

        glam_test!(test_length_squared_signed, {
            let x = $new(1 as $t, 0 as $t, 0 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t, 0 as $t, 0 as $t);
            let z = $new(0 as $t, 0 as $t, 1 as $t, 0 as $t);
            let w = $new(0 as $t, 0 as $t, 0 as $t, 1 as $t);
            assert_eq!(4 as $t, (2 as $t * x).length_squared());
            assert_eq!(9 as $t, (-3 as $t * y).length_squared());
            assert_eq!(16 as $t, (4 as $t * z).length_squared());
            assert_eq!(64 as $t, (8 as $t * w).length_squared());
            assert_eq!(
                2 as $t * 2 as $t + 3 as $t * 3 as $t + 4 as $t * 4 as $t + 5 as $t * 5 as $t,
                $new(2 as $t, 3 as $t, 4 as $t, 5 as $t).length_squared()
            );
            assert_eq!(2 as $t, x.distance_squared(y));
            assert_eq!(13 as $t, (2 as $t * x).distance_squared(-3 as $t * z));
        });

        glam_test!(test_div_euclid, {
            let one = $vec4::ONE;
            let two = one + one;
            let three = two + one;
            assert_eq!(three.div_euclid(two), one);
            assert_eq!((-three).div_euclid(two), -two);
            assert_eq!(three.div_euclid(-two), -one);
            assert_eq!((-three).div_euclid(-two), two);
        });

        glam_test!(test_rem_euclid, {
            let one = $vec4::ONE;
            let two = one + one;
            let three = two + one;
            let four = three + one;
            assert_eq!(four.rem_euclid(three), one);
            assert_eq!((-four).rem_euclid(three), two);
            assert_eq!(four.rem_euclid(-three), one);
            assert_eq!((-four).rem_euclid(-three), two);
        });
    };
}

macro_rules! impl_vec4_signed_integer_tests {
    ($t:ident, $new:ident, $vec4:ident, $vec3:ident, $vec2:ident, $mask:ident, $masknew:ident) => {
        impl_vec4_signed_tests!($t, $new, $vec4, $vec3, $vec2, $mask, $masknew);

        glam_test!(test_signum, {
            assert_eq!($vec4::ZERO.signum(), $vec4::ZERO);
            assert_eq!($vec4::ONE.signum(), $vec4::ONE);
            assert_eq!((-$vec4::ONE).signum(), -$vec4::ONE);
        });
    };
}

macro_rules! impl_vec4_eq_hash_tests {
    ($t:ident, $new:ident) => {
        glam_test!(test_ve2_hash, {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let a = $new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            let b = $new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            let c = $new(3 as $t, 2 as $t, 1 as $t, 4 as $t);

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
        });
    };
}

macro_rules! impl_vec4_float_tests {
    ($t:ident, $new:ident, $vec4:ident, $vec3:ident, $vec2:ident, $mask:ident, $masknew:ident) => {
        impl_vec4_signed_tests!($t, $new, $vec4, $vec3, $vec2, $mask, $masknew);
        impl_vec_float_normalize_tests!($t, $vec4);

        glam_test!(test_vec4_nan, {
            assert!($vec4::NAN.is_nan());
            assert!(!$vec4::NAN.is_finite());
        });

        glam_test!(test_funcs, {
            let x = $new(1.0, 0.0, 0.0, 0.0);
            let y = $new(0.0, 1.0, 0.0, 0.0);
            let z = $new(0.0, 0.0, 1.0, 0.0);
            let w = $new(0.0, 0.0, 0.0, 1.0);
            assert_eq!(2.0, (-2.0 * x).length());
            assert_eq!(3.0, (3.0 * y).length());
            assert_eq!(4.0, (-4.0 * z).length());
            assert_eq!(5.0, (-5.0 * w).length());
            assert_eq!((2.0 as $t).sqrt(), w.distance(y));
            assert_eq!(5.0, (3.0 * x).distance(-4.0 * y));
            assert_eq!(13.0, (-5.0 * w).distance(12.0 * y));
            assert_eq!(x, (2.0 * x).normalize());
            assert_eq!(
                1.0 * 5.0 + 2.0 * 6.0 + 3.0 * 7.0 + 4.0 * 8.0,
                $new(1.0, 2.0, 3.0, 4.0).dot($new(5.0, 6.0, 7.0, 8.0))
            );
            assert_eq!(
                (2.0 as $t * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt(),
                $new(2.0, 3.0, 4.0, 5.0).length()
            );
            assert_eq!(
                1.0 / (2.0 as $t * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt(),
                $new(2.0, 3.0, 4.0, 5.0).length_recip()
            );
            assert!($new(2.0, 3.0, 4.0, 5.0).normalize().is_normalized());
            assert_approx_eq!(
                $new(2.0, 3.0, 4.0, 5.0)
                    / (2.0 as $t * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt(),
                $new(2.0, 3.0, 4.0, 5.0).normalize()
            );
            assert_eq!(
                $new(0.5, 0.25, 0.125, 0.0625),
                $new(2.0, 4.0, 8.0, 16.0).recip()
            );
        });

        glam_test!(test_project_reject, {
            assert_eq!(
                $new(0.0, 0.0, 0.0, 1.0),
                $new(0.0, 1.0, 0.0, 1.0).project_onto($new(0.0, 0.0, 0.0, 2.0))
            );
            assert_eq!(
                $new(0.0, 1.0, 0.0, 0.0),
                $new(0.0, 1.0, 0.0, 1.0).reject_from($new(0.0, 0.0, 0.0, 2.0))
            );
            assert_eq!(
                $new(0.0, 0.0, 0.0, 1.0),
                $new(0.0, 1.0, 0.0, 1.0).project_onto_normalized($new(0.0, 0.0, 0.0, 1.0))
            );
            assert_eq!(
                $new(0.0, 1.0, 0.0, 0.0),
                $new(0.0, 1.0, 0.0, 1.0).reject_from_normalized($new(0.0, 0.0, 0.0, 1.0))
            );
            should_glam_assert!({ $vec4::ONE.project_onto($vec4::ZERO) });
            should_glam_assert!({ $vec4::ONE.reject_from($vec4::ZERO) });
            should_glam_assert!({ $vec4::ONE.project_onto_normalized($vec4::ONE) });
            should_glam_assert!({ $vec4::ONE.reject_from_normalized($vec4::ONE) });
        });

        glam_test!(test_signum, {
            assert_eq!($vec4::ZERO.signum(), $vec4::ONE);
            assert_eq!((-$vec4::ZERO).signum(), -$vec4::ONE);
            assert_eq!($vec4::ONE.signum(), $vec4::ONE);
            assert_eq!((-$vec4::ONE).signum(), -$vec4::ONE);
            assert_eq!($vec4::INFINITY.signum(), $vec4::ONE);
            assert_eq!($vec4::NEG_INFINITY.signum(), -$vec4::ONE);
            assert!($vec4::NAN.signum().is_nan_mask().all());
        });

        glam_test!(test_copysign, {
            assert_eq!($vec4::ZERO.copysign(-$vec4::ZERO), -$vec4::ZERO);
            assert_eq!((-$vec4::ZERO).copysign(-$vec4::ZERO), -$vec4::ZERO);
            assert_eq!($vec4::ZERO.copysign($vec4::ZERO), $vec4::ZERO);
            assert_eq!((-$vec4::ZERO).copysign($vec4::ZERO), $vec4::ZERO);
            assert_eq!($vec4::ONE.copysign(-$vec4::ZERO), -$vec4::ONE);
            assert_eq!((-$vec4::ONE).copysign(-$vec4::ZERO), -$vec4::ONE);
            assert_eq!($vec4::ONE.copysign($vec4::ZERO), $vec4::ONE);
            assert_eq!((-$vec4::ONE).copysign($vec4::ZERO), $vec4::ONE);
            assert_eq!($vec4::ZERO.copysign(-$vec4::ONE), -$vec4::ZERO);
            assert_eq!((-$vec4::ZERO).copysign(-$vec4::ONE), -$vec4::ZERO);
            assert_eq!($vec4::ZERO.copysign($vec4::ONE), $vec4::ZERO);
            assert_eq!((-$vec4::ZERO).copysign($vec4::ONE), $vec4::ZERO);
            assert_eq!($vec4::ONE.copysign(-$vec4::ONE), -$vec4::ONE);
            assert_eq!((-$vec4::ONE).copysign(-$vec4::ONE), -$vec4::ONE);
            assert_eq!($vec4::ONE.copysign($vec4::ONE), $vec4::ONE);
            assert_eq!((-$vec4::ONE).copysign($vec4::ONE), $vec4::ONE);
            assert_eq!($vec4::INFINITY.copysign($vec4::ONE), $vec4::INFINITY);
            assert_eq!($vec4::INFINITY.copysign(-$vec4::ONE), $vec4::NEG_INFINITY);
            assert_eq!($vec4::NEG_INFINITY.copysign($vec4::ONE), $vec4::INFINITY);
            assert_eq!(
                $vec4::NEG_INFINITY.copysign(-$vec4::ONE),
                $vec4::NEG_INFINITY
            );
            assert!($vec4::NAN.copysign($vec4::ONE).is_nan_mask().all());
            assert!($vec4::NAN.copysign(-$vec4::ONE).is_nan_mask().all());
        });

        glam_test!(test_float_is_negative_bitmask, {
            assert_eq!($vec4::ZERO.is_negative_bitmask(), 0b0000);
            assert_eq!((-$vec4::ZERO).is_negative_bitmask(), 0b1111);
            assert_eq!($vec4::ONE.is_negative_bitmask(), 0b0000);
            assert_eq!((-$vec4::ONE).is_negative_bitmask(), 0b1111);
            assert_eq!(
                $vec4::new(-1.0, 2.0, 3.0, -4.0).is_negative_bitmask(),
                0b1001
            );
            assert_eq!(
                $vec4::new(8.0, 3.0, 1.0, -0.0).is_negative_bitmask(),
                0b1000
            );
            assert_eq!(
                $vec4::new(1.0, 5.0, -3.0, 7.0).is_negative_bitmask(),
                0b0100
            );
            assert_eq!(
                $vec4::new(3.0, -4.0, 1.0, 6.0).is_negative_bitmask(),
                0b0010
            );
            assert_eq!(
                $vec4::new(2.0, -6.0, 5.0, -3.0).is_negative_bitmask(),
                0b1010
            );
        });

        glam_test!(test_round, {
            assert_eq!($vec4::new(1.35, 0.0, 0.0, 0.0).round().x, 1.0);
            assert_eq!($vec4::new(0.0, 1.5, 0.0, 0.0).round().y, 2.0);
            assert_eq!($vec4::new(0.0, 0.0, -15.5, 0.0).round().z, -16.0);
            assert_eq!($vec4::new(0.0, 0.0, 0.0, 0.0).round().z, 0.0);
            assert_eq!($vec4::new(0.0, 21.1, 0.0, 0.0).round().y, 21.0);
            assert_eq!($vec4::new(0.0, 0.0, 0.0, 11.123).round().w, 11.0);
            assert_eq!($vec4::new(0.0, 0.0, 11.501, 0.0).round().z, 12.0);
            assert_eq!(
                $vec4::new($t::NEG_INFINITY, $t::INFINITY, 1.0, -1.0).round(),
                $vec4::new($t::NEG_INFINITY, $t::INFINITY, 1.0, -1.0)
            );
            assert!($vec4::new($t::NAN, 0.0, 0.0, 1.0).round().x.is_nan());
        });

        glam_test!(test_floor, {
            assert_eq!(
                $vec4::new(1.35, 1.5, -1.5, 1.999).floor(),
                $vec4::new(1.0, 1.0, -2.0, 1.0)
            );
            assert_eq!(
                $vec4::new($t::INFINITY, $t::NEG_INFINITY, 0.0, 0.0).floor(),
                $vec4::new($t::INFINITY, $t::NEG_INFINITY, 0.0, 0.0)
            );
            assert!($vec4::new(0.0, $t::NAN, 0.0, 0.0).floor().y.is_nan());
            assert_eq!(
                $vec4::new(-0.0, -2000000.123, 10000000.123, 1000.9).floor(),
                $vec4::new(-0.0, -2000001.0, 10000000.0, 1000.0)
            );
        });

        glam_test!(test_fract_gl, {
            assert_approx_eq!(
                $vec4::new(1.35, 1.5, -1.5, 1.999).fract_gl(),
                $vec4::new(0.35, 0.5, 0.5, 0.999)
            );
            assert_approx_eq!(
                $vec4::new(-0.0, -200000.123, 1000000.123, 1000.9).fract_gl(),
                $vec4::new(0.0, 0.877, 0.123, 0.9),
                0.002
            );
        });

        glam_test!(test_fract, {
            assert_approx_eq!(
                $vec4::new(1.35, 1.5, -1.5, 1.999).fract(),
                $vec4::new(0.35, 0.5, -0.5, 0.999)
            );
            assert_approx_eq!(
                $vec4::new(-0.0, -200000.123, 1000000.123, 1000.9).fract(),
                $vec4::new(0.0, -0.123, 0.123, 0.9),
                0.002
            );
        });

        glam_test!(test_ceil, {
            assert_eq!(
                $vec4::new(1.35, 1.5, -1.5, 1234.1234).ceil(),
                $vec4::new(2.0, 2.0, -1.0, 1235.0)
            );
            assert_eq!(
                $vec4::new($t::INFINITY, $t::NEG_INFINITY, 0.0, 0.0).ceil(),
                $vec4::new($t::INFINITY, $t::NEG_INFINITY, 0.0, 0.0)
            );
            assert!($vec4::new(0.0, 0.0, $t::NAN, 0.0).ceil().z.is_nan());
            assert_eq!(
                $vec4::new(-1234.1234, -2000000.123, 1000000.123, 1000.9).ceil(),
                $vec4::new(-1234.0, -2000000.0, 1000001.0, 1001.0)
            );
        });

        glam_test!(test_trunc, {
            assert_eq!(
                $vec4::new(1.35, 1.5, -1.5, 1.999).trunc(),
                $vec4::new(1.0, 1.0, -1.0, 1.0)
            );
            assert_eq!(
                $vec4::new($t::INFINITY, $t::NEG_INFINITY, 0.0, 0.0).trunc(),
                $vec4::new($t::INFINITY, $t::NEG_INFINITY, 0.0, 0.0)
            );
            assert!($vec4::new(0.0, $t::NAN, 0.0, 0.0).trunc().y.is_nan());
            assert_eq!(
                $vec4::new(-0.0, -2000000.123, 10000000.123, 1000.9).trunc(),
                $vec4::new(-0.0, -2000000.0, 10000000.0, 1000.0)
            );
        });

        glam_test!(test_lerp, {
            let v0 = $vec4::new(-1.0, -1.0, -1.0, -1.0);
            let v1 = $vec4::new(1.0, 1.0, 1.0, 1.0);
            assert_approx_eq!(v0, v0.lerp(v1, 0.0));
            assert_approx_eq!(v1, v0.lerp(v1, 1.0));
            assert_approx_eq!($vec4::ZERO, v0.lerp(v1, 0.5));
        });

        glam_test!(test_lerp_big_difference, {
            let v0 = $vec4::new(-1e30, -1e30, -1e30, -1e30);
            let v1 = $vec4::new(16.0, 16.0, 16.0, 16.0);
            assert_approx_eq!(v0, v0.lerp(v1, 0.0));
            assert_approx_eq!(v1, v0.lerp(v1, 1.0));
        });

        glam_test!(test_move_towards, {
            let v0 = $vec4::new(-1.0, -1.0, -1.0, -1.0);
            let v1 = $vec4::new(1.0, 1.0, 1.0, 1.0);
            assert_approx_eq!(v0, v0.move_towards(v1, 0.0));
            assert_approx_eq!(v1, v0.move_towards(v1, v0.distance(v1)));
            assert_approx_eq!(v1, v0.move_towards(v1, v0.distance(v1) + 1.0));
        });

        glam_test!(test_midpoint, {
            let v0 = $vec4::new(-1.0, -1.0, -1.0, -1.0);
            let v1 = $vec4::new(1.0, 1.0, 1.0, 1.0);
            let v2 = $vec4::new(-1.5, 0.0, 1.0, 0.5);
            assert_approx_eq!($vec4::ZERO, v0.midpoint(v1));
            assert_approx_eq!($vec4::new(-0.25, 0.5, 1.0, 0.75), v1.midpoint(v2));
        });

        glam_test!(test_is_finite, {
            assert!($vec4::new(0.0, 0.0, 0.0, 0.0).is_finite());
            assert!($vec4::new(-1e-10, 1.0, 1e10, 42.0).is_finite());
            assert!(!$vec4::new($t::INFINITY, 0.0, 0.0, 0.0).is_finite());
            assert!(!$vec4::new(0.0, $t::NAN, 0.0, 0.0).is_finite());
            assert!(!$vec4::new(0.0, 0.0, $t::NEG_INFINITY, 0.0).is_finite());
            assert!(!$vec4::new(0.0, 0.0, 0.0, $t::NAN).is_finite());
            assert!(!$vec4::INFINITY.is_finite());
            assert!(!$vec4::NEG_INFINITY.is_finite());
        });

        glam_test!(test_powf, {
            assert_eq!(
                $vec4::new(2.0, 4.0, 8.0, 16.0).powf(2.0),
                $vec4::new(4.0, 16.0, 64.0, 256.0)
            );
        });

        glam_test!(test_exp, {
            assert_approx_eq!(
                $vec4::new(1.0, 2.0, 3.0, 4.0).exp(),
                $vec4::new(
                    (1.0 as $t).exp(),
                    (2.0 as $t).exp(),
                    (3.0 as $t).exp(),
                    (4.0 as $t).exp()
                ),
                1e-5
            );
        });

        glam_test!(test_clamp_length, {
            // Too long gets shortened
            assert_eq!(
                $vec4::new(12.0, 16.0, 0.0, 0.0).clamp_length(7.0, 10.0),
                $vec4::new(6.0, 8.0, 0.0, 0.0) // shortened to length 10.0
            );
            // In the middle is unchanged
            assert_eq!(
                $vec4::new(2.0, 1.0, 0.0, 0.0).clamp_length(0.5, 5.0),
                $vec4::new(2.0, 1.0, 0.0, 0.0) // unchanged
            );
            // Too short gets lengthened
            assert_eq!(
                $vec4::new(0.6, 0.8, 0.0, 0.0).clamp_length(10.0, 20.0),
                $vec4::new(6.0, 8.0, 0.0, 0.0) // lengthened to length 10.0
            );
            should_glam_assert!({ $vec4::ONE.clamp_length(1.0, 0.0) });
        });

        glam_test!(test_clamp_length_max, {
            // Too long gets shortened
            assert_eq!(
                $vec4::new(12.0, 16.0, 0.0, 0.0).clamp_length_max(10.0),
                $vec4::new(6.0, 8.0, 0.0, 0.0) // shortened to length 10.0
            );
            // Not too long is unchanged
            assert_eq!(
                $vec4::new(2.0, 1.0, 0.0, 0.0).clamp_length_max(5.0),
                $vec4::new(2.0, 1.0, 0.0, 0.0) // unchanged
            );
        });

        glam_test!(test_clamp_length_min, {
            // Not too short is unchanged
            assert_eq!(
                $vec4::new(2.0, 1.0, 0.0, 0.0).clamp_length_min(0.5),
                $vec4::new(2.0, 1.0, 0.0, 0.0) // unchanged
            );
            // Too short gets lengthened
            assert_eq!(
                $vec4::new(0.6, 0.8, 0.0, 0.0).clamp_length_min(10.0),
                $vec4::new(6.0, 8.0, 0.0, 0.0) // lengthened to length 10.0
            );
        });

        glam_test!(test_mul_add, {
            assert_eq!(
                $vec4::new(1.0, 1.0, 1.0, 1.0).mul_add(
                    $vec4::new(0.5, 2.0, -4.0, 0.0),
                    $vec4::new(-1.0, -1.0, -1.0, -1.0)
                ),
                $vec4::new(-0.5, 1.0, -5.0, -1.0)
            );
        });

        glam_test!(test_fmt_float, {
            let a = $vec4::new(1.0, 2.0, 3.0, 4.0);
            assert_eq!(format!("{:.2}", a), "[1.00, 2.00, 3.00, 4.00]");
        });

        glam_test!(test_reflect, {
            let incident = $vec4::new(1.0, -1.0, 1.0, 1.0);
            let normal = $vec4::Y;
            assert_approx_eq!(incident.reflect(normal), $vec4::ONE);
        });

        glam_test!(test_refract, {
            let incident = $vec4::NEG_ONE.normalize();
            let normal = $vec4::ONE.normalize();
            assert_approx_eq!(incident.refract(normal, 0.5), incident);

            let incident = $vec4::new(1.0, -1.0, 0.0, 0.0).normalize();
            let normal = $vec4::Y;
            assert_approx_eq!(incident.refract(normal, 1.5), $vec4::ZERO);
        });
    };
}

macro_rules! impl_vec4_scalar_shift_op_test {
    ($vec4:ident, $t_min:literal, $t_max:literal, $rhs_min:literal, $rhs_max:literal) => {
        glam_test!(test_vec4_scalar_shift_ops, {
            for x in $t_min..$t_max {
                for y in $t_min..$t_max {
                    for z in $t_min..$t_max {
                        for w in $t_min..$t_max {
                            for rhs in $rhs_min..$rhs_max {
                                assert_eq!(
                                    $vec4::new(x, y, z, w) << rhs,
                                    $vec4::new(x << rhs, y << rhs, z << rhs, w << rhs)
                                );
                                assert_eq!(
                                    $vec4::new(x, y, z, w) >> rhs,
                                    $vec4::new(x >> rhs, y >> rhs, z >> rhs, w >> rhs)
                                );
                            }
                        }
                    }
                }
            }
        });
    };
}

macro_rules! impl_vec4_scalar_shift_op_tests {
    ($vec4:ident, $t_min:literal, $t_max:literal) => {
        mod shift_by_i8 {
            use glam::$vec4;
            impl_vec4_scalar_shift_op_test!($vec4, $t_min, $t_max, 0i8, 2);
        }
        mod shift_by_i16 {
            use glam::$vec4;
            impl_vec4_scalar_shift_op_test!($vec4, $t_min, $t_max, 0i16, 2);
        }
        mod shift_by_i32 {
            use glam::$vec4;
            impl_vec4_scalar_shift_op_test!($vec4, $t_min, $t_max, 0i32, 2);
        }
        mod shift_by_i64 {
            use glam::$vec4;
            impl_vec4_scalar_shift_op_test!($vec4, $t_min, $t_max, 0i64, 2);
        }
        mod shift_by_u8 {
            use glam::$vec4;
            impl_vec4_scalar_shift_op_test!($vec4, $t_min, $t_max, 0u8, 2);
        }
        mod shift_by_u16 {
            use glam::$vec4;
            impl_vec4_scalar_shift_op_test!($vec4, $t_min, $t_max, 0u16, 2);
        }
        mod shift_by_u32 {
            use glam::$vec4;
            impl_vec4_scalar_shift_op_test!($vec4, $t_min, $t_max, 0u32, 2);
        }
        mod shift_by_u64 {
            use glam::$vec4;
            impl_vec4_scalar_shift_op_test!($vec4, $t_min, $t_max, 0u64, 2);
        }
    };
}

macro_rules! impl_vec4_shift_op_test {
    ($vec4:ident, $rhs:ident, $t_min:literal, $t_max:literal) => {
        glam_test!(test_vec4_shift_ops, {
            for x1 in $t_min..$t_max {
                for y1 in $t_min..$t_max {
                    for z1 in $t_min..$t_max {
                        for w1 in $t_min..$t_max {
                            for x2 in $t_min..$t_max {
                                for y2 in $t_min..$t_max {
                                    for z2 in $t_min..$t_max {
                                        for w2 in $t_min..$t_max {
                                            assert_eq!(
                                                $vec4::new(x1, y1, z1, w1)
                                                    << $rhs::new(x2, y2, z2, w2),
                                                $vec4::new(x1 << x2, y1 << y2, z1 << z2, w1 << w2)
                                            );
                                            assert_eq!(
                                                $vec4::new(x1, y1, z1, w1)
                                                    >> $rhs::new(x2, y2, z2, w2),
                                                $vec4::new(x1 >> x2, y1 >> y2, z1 >> z2, w1 >> w2)
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    };
}

macro_rules! impl_vec4_shift_op_tests {
    ($vec4:ident) => {
        mod shift_ivec4_by_ivec4 {
            use super::*;
            impl_vec4_shift_op_test!($vec4, IVec4, 0, 2);
        }
        mod shift_ivec4_by_uvec4 {
            use super::*;
            impl_vec4_shift_op_test!($vec4, UVec4, 0, 2);
        }
    };
}

macro_rules! impl_vec4_scalar_bit_op_tests {
    ($vec4:ident, $t_min:literal, $t_max:literal) => {
        glam_test!(test_vec4_scalar_bit_ops, {
            for x in $t_min..$t_max {
                for y in $t_min..$t_max {
                    for z in $t_min..$t_max {
                        for w in $t_min..$t_max {
                            for rhs in $t_min..$t_max {
                                assert_eq!(
                                    $vec4::new(x, y, z, w) & rhs,
                                    $vec4::new(x & rhs, y & rhs, z & rhs, w & rhs)
                                );
                                assert_eq!(
                                    $vec4::new(x, y, z, w) | rhs,
                                    $vec4::new(x | rhs, y | rhs, z | rhs, w | rhs)
                                );
                                assert_eq!(
                                    $vec4::new(x, y, z, w) ^ rhs,
                                    $vec4::new(x ^ rhs, y ^ rhs, z ^ rhs, w ^ rhs)
                                );
                            }
                        }
                    }
                }
            }
        });
    };
}

macro_rules! impl_vec4_bit_op_tests {
    ($vec4:ident, $t_min:literal, $t_max:literal) => {
        glam_test!(test_vec4_bit_ops, {
            for x1 in $t_min..$t_max {
                for y1 in $t_min..$t_max {
                    for z1 in $t_min..$t_max {
                        for w1 in $t_min..$t_max {
                            assert_eq!(!$vec4::new(x1, y1, z1, w1), $vec4::new(!x1, !y1, !z1, !w1));

                            for x2 in $t_min..$t_max {
                                for y2 in $t_min..$t_max {
                                    for z2 in $t_min..$t_max {
                                        for w2 in $t_min..$t_max {
                                            assert_eq!(
                                                $vec4::new(x1, y1, z1, w1)
                                                    & $vec4::new(x2, y2, z2, w2),
                                                $vec4::new(x1 & x2, y1 & y2, z1 & z2, w1 & w2)
                                            );
                                            assert_eq!(
                                                $vec4::new(x1, y1, z1, w1)
                                                    | $vec4::new(x2, y2, z2, w2),
                                                $vec4::new(x1 | x2, y1 | y2, z1 | z2, w1 | w2)
                                            );
                                            assert_eq!(
                                                $vec4::new(x1, y1, z1, w1)
                                                    ^ $vec4::new(x2, y2, z2, w2),
                                                $vec4::new(x1 ^ x2, y1 ^ y2, z1 ^ z2, w1 ^ w2)
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    };
}

mod vec4 {
    #[cfg(feature = "scalar-math")]
    use glam::{bvec4, BVec4};
    #[cfg(not(feature = "scalar-math"))]
    use glam::{bvec4a, BVec4A};
    use glam::{vec4, Vec2, Vec3, Vec4};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(16, mem::size_of::<Vec4>());
        if cfg!(any(not(feature = "scalar-math"), feature = "cuda")) {
            assert_eq!(16, mem::align_of::<Vec4>());
        } else {
            assert_eq!(4, mem::align_of::<Vec4>());
        }
        #[cfg(not(feature = "scalar-math"))]
        {
            assert_eq!(16, mem::size_of::<BVec4A>());
            assert_eq!(16, mem::align_of::<BVec4A>());
        }
        #[cfg(feature = "scalar-math")]
        {
            assert_eq!(4, mem::size_of::<BVec4>());
            assert_eq!(1, mem::align_of::<BVec4>());
        }
    });

    #[cfg(all(
        target_feature = "sse2",
        not(any(feature = "core-simd", feature = "scalar-math"))
    ))]
    #[test]
    fn test_m128() {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        #[repr(C, align(16))]
        struct F32x4_A16([f32; 4]);

        let v0 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let m0: __m128 = v0.into();
        let mut a0 = F32x4_A16([0.0, 0.0, 0.0, 0.0]);
        unsafe {
            _mm_store_ps(a0.0.as_mut_ptr(), m0);
        }
        assert_eq!([1.0, 2.0, 3.0, 4.0], a0.0);
        let v1 = Vec4::from(m0);
        assert_eq!(v0, v1);

        #[repr(C, align(16))]
        struct U32x4_A16([u32; 4]);

        let v0 = BVec4A::new(true, false, true, false);
        let m0: __m128 = v0.into();
        let mut a0 = U32x4_A16([1, 2, 3, 4]);
        unsafe {
            _mm_store_ps(a0.0.as_mut_ptr() as *mut f32, m0);
        }
        assert_eq!([0xffffffff, 0, 0xffffffff, 0], a0.0);
    }

    glam_test!(test_as, {
        use glam::{DVec4, I16Vec4, I64Vec4, I8Vec4, IVec4, U16Vec4, U64Vec4, U8Vec4, UVec4, Vec4};
        assert_eq!(
            DVec4::new(-1.0, -2.0, -3.0, -4.0),
            Vec4::new(-1.0, -2.0, -3.0, -4.0).as_dvec4()
        );
        assert_eq!(
            I8Vec4::new(-1, -2, -3, -4),
            Vec4::new(-1.0, -2.0, -3.0, -4.0).as_i8vec4()
        );
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            Vec4::new(1.0, 2.0, 3.0, 4.0).as_u8vec4()
        );
        assert_eq!(
            I16Vec4::new(-1, -2, -3, -4),
            Vec4::new(-1.0, -2.0, -3.0, -4.0).as_i16vec4()
        );
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            Vec4::new(1.0, 2.0, 3.0, 4.0).as_u16vec4()
        );
        assert_eq!(
            IVec4::new(-1, -2, -3, -4),
            Vec4::new(-1.0, -2.0, -3.0, -4.0).as_ivec4()
        );
        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            Vec4::new(1.0, 2.0, 3.0, 4.0).as_uvec4()
        );
        assert_eq!(
            I64Vec4::new(-1, -2, -3, -4),
            Vec4::new(-1.0, -2.0, -3.0, -4.0).as_i64vec4()
        );
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            Vec4::new(1.0, 2.0, 3.0, 4.0).as_u64vec4()
        );

        assert_eq!(
            Vec4::new(-1.0, -2.0, -3.0, -4.0),
            DVec4::new(-1.0, -2.0, -3.0, -4.0).as_vec4()
        );
        assert_eq!(
            I8Vec4::new(-1, -2, -3, -4),
            DVec4::new(-1.0, -2.0, -3.0, -4.0).as_i8vec4()
        );
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            DVec4::new(1.0, 2.0, 3.0, 4.0).as_u8vec4()
        );
        assert_eq!(
            I16Vec4::new(-1, -2, -3, -4),
            DVec4::new(-1.0, -2.0, -3.0, -4.0).as_i16vec4()
        );
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            DVec4::new(1.0, 2.0, 3.0, 4.0).as_u16vec4()
        );
        assert_eq!(
            IVec4::new(-1, -2, -3, -4),
            DVec4::new(-1.0, -2.0, -3.0, -4.0).as_ivec4()
        );
        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            DVec4::new(1.0, 2.0, 3.0, 4.0).as_uvec4()
        );
        assert_eq!(
            I64Vec4::new(-1, -2, -3, -4),
            DVec4::new(-1.0, -2.0, -3.0, -4.0).as_i64vec4()
        );
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            DVec4::new(1.0, 2.0, 3.0, 4.0).as_u64vec4()
        );

        assert_eq!(
            Vec4::new(-1.0, -2.0, -3.0, -4.0),
            I8Vec4::new(-1, -2, -3, -4).as_vec4()
        );
        assert_eq!(
            DVec4::new(-1.0, -2.0, -3.0, -4.0),
            I8Vec4::new(-1, -2, -3, -4).as_dvec4()
        );
        assert_eq!(U8Vec4::new(1, 2, 3, 4), I8Vec4::new(1, 2, 3, 4).as_u8vec4());
        assert_eq!(
            I16Vec4::new(-1, -2, -3, -4),
            I8Vec4::new(-1, -2, -3, -4).as_i16vec4()
        );
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            I8Vec4::new(1, 2, 3, 4).as_u16vec4()
        );
        assert_eq!(
            IVec4::new(-1, -2, -3, -4),
            I8Vec4::new(-1, -2, -3, -4).as_ivec4()
        );
        assert_eq!(UVec4::new(1, 2, 3, 4), I8Vec4::new(1, 2, 3, 4).as_uvec4());
        assert_eq!(
            I64Vec4::new(-1, -2, -3, -4),
            I8Vec4::new(-1, -2, -3, -4).as_i64vec4()
        );
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            I8Vec4::new(1, 2, 3, 4).as_u64vec4()
        );

        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            U8Vec4::new(1, 2, 3, 4).as_vec4()
        );
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            U8Vec4::new(1, 2, 3, 4).as_dvec4()
        );
        assert_eq!(I8Vec4::new(1, 2, 3, 4), U8Vec4::new(1, 2, 3, 4).as_i8vec4());
        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            U8Vec4::new(1, 2, 3, 4).as_i16vec4()
        );
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            U8Vec4::new(1, 2, 3, 4).as_u16vec4()
        );
        assert_eq!(IVec4::new(1, 2, 3, 4), U8Vec4::new(1, 2, 3, 4).as_ivec4());
        assert_eq!(UVec4::new(1, 2, 3, 4), U8Vec4::new(1, 2, 3, 4).as_uvec4());
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            U8Vec4::new(1, 2, 3, 4).as_i64vec4()
        );
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            U8Vec4::new(1, 2, 3, 4).as_u64vec4()
        );

        assert_eq!(
            Vec4::new(-1.0, -2.0, -3.0, -4.0),
            I16Vec4::new(-1, -2, -3, -4).as_vec4()
        );
        assert_eq!(
            DVec4::new(-1.0, -2.0, -3.0, -4.0),
            I16Vec4::new(-1, -2, -3, -4).as_dvec4()
        );
        assert_eq!(
            I8Vec4::new(-1, -2, -3, -4),
            I16Vec4::new(-1, -2, -3, -4).as_i8vec4()
        );
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            I16Vec4::new(1, 2, 3, 4).as_u8vec4()
        );
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            I16Vec4::new(1, 2, 3, 4).as_u16vec4()
        );
        assert_eq!(
            IVec4::new(-1, -2, -3, -4),
            I16Vec4::new(-1, -2, -3, -4).as_ivec4()
        );
        assert_eq!(UVec4::new(1, 2, 3, 4), I16Vec4::new(1, 2, 3, 4).as_uvec4());
        assert_eq!(
            I64Vec4::new(-1, -2, -3, -4),
            I16Vec4::new(-1, -2, -3, -4).as_i64vec4()
        );
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            I16Vec4::new(1, 2, 3, 4).as_u64vec4()
        );

        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            U16Vec4::new(1, 2, 3, 4).as_vec4()
        );
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            U16Vec4::new(1, 2, 3, 4).as_dvec4()
        );
        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            U16Vec4::new(1, 2, 3, 4).as_i8vec4()
        );
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            U16Vec4::new(1, 2, 3, 4).as_u8vec4()
        );
        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            U16Vec4::new(1, 2, 3, 4).as_i16vec4()
        );
        assert_eq!(IVec4::new(1, 2, 3, 4), U16Vec4::new(1, 2, 3, 4).as_ivec4());
        assert_eq!(UVec4::new(1, 2, 3, 4), U16Vec4::new(1, 2, 3, 4).as_uvec4());
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            U16Vec4::new(1, 2, 3, 4).as_i64vec4()
        );
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            U16Vec4::new(1, 2, 3, 4).as_u64vec4()
        );

        assert_eq!(
            Vec4::new(-1.0, -2.0, -3.0, -4.0),
            IVec4::new(-1, -2, -3, -4).as_vec4()
        );
        assert_eq!(
            DVec4::new(-1.0, -2.0, -3.0, -4.0),
            IVec4::new(-1, -2, -3, -4).as_dvec4()
        );
        assert_eq!(
            I8Vec4::new(-1, -2, -3, -4),
            IVec4::new(-1, -2, -3, -4).as_i8vec4()
        );
        assert_eq!(U8Vec4::new(1, 2, 3, 4), IVec4::new(1, 2, 3, 4).as_u8vec4());
        assert_eq!(UVec4::new(1, 2, 3, 4), IVec4::new(1, 2, 3, 4).as_uvec4());
        assert_eq!(
            I16Vec4::new(-1, -2, -3, -4),
            IVec4::new(-1, -2, -3, -4).as_i16vec4()
        );
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            IVec4::new(1, 2, 3, 4).as_u16vec4()
        );
        assert_eq!(
            I64Vec4::new(-1, -2, -3, -4),
            IVec4::new(-1, -2, -3, -4).as_i64vec4()
        );
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            IVec4::new(1, 2, 3, 4).as_u64vec4()
        );

        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            UVec4::new(1, 2, 3, 4).as_vec4()
        );
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            UVec4::new(1, 2, 3, 4).as_dvec4()
        );
        assert_eq!(I8Vec4::new(1, 2, 3, 4), UVec4::new(1, 2, 3, 4).as_i8vec4());
        assert_eq!(U8Vec4::new(1, 2, 3, 4), UVec4::new(1, 2, 3, 4).as_u8vec4());
        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            UVec4::new(1, 2, 3, 4).as_i16vec4()
        );
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            UVec4::new(1, 2, 3, 4).as_u16vec4()
        );
        assert_eq!(IVec4::new(1, 2, 3, 4), UVec4::new(1, 2, 3, 4).as_ivec4());
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            UVec4::new(1, 2, 3, 4).as_i64vec4()
        );
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            UVec4::new(1, 2, 3, 4).as_u64vec4()
        );

        assert_eq!(
            Vec4::new(-1.0, -2.0, -3.0, -4.0),
            I64Vec4::new(-1, -2, -3, -4).as_vec4()
        );
        assert_eq!(
            DVec4::new(-1.0, -2.0, -3.0, -4.0),
            I64Vec4::new(-1, -2, -3, -4).as_dvec4()
        );
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            I64Vec4::new(1, 2, 3, 4).as_u8vec4()
        );
        assert_eq!(
            I8Vec4::new(-1, -2, -3, -4),
            I64Vec4::new(-1, -2, -3, -4).as_i8vec4()
        );
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            I64Vec4::new(1, 2, 3, 4).as_u16vec4()
        );
        assert_eq!(
            I16Vec4::new(-1, -2, -3, -4),
            I64Vec4::new(-1, -2, -3, -4).as_i16vec4()
        );
        assert_eq!(UVec4::new(1, 2, 3, 4), I64Vec4::new(1, 2, 3, 4).as_uvec4());
        assert_eq!(
            IVec4::new(-1, -2, -3, -4),
            I64Vec4::new(-1, -2, -3, -4).as_ivec4()
        );
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            I64Vec4::new(1, 2, 3, 4).as_u64vec4()
        );

        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            U64Vec4::new(1, 2, 3, 4).as_vec4()
        );
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            U64Vec4::new(1, 2, 3, 4).as_dvec4()
        );
        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            U64Vec4::new(1, 2, 3, 4).as_i8vec4()
        );
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            U64Vec4::new(1, 2, 3, 4).as_u8vec4()
        );
        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            U64Vec4::new(1, 2, 3, 4).as_i16vec4()
        );
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            U64Vec4::new(1, 2, 3, 4).as_u16vec4()
        );
        assert_eq!(IVec4::new(1, 2, 3, 4), U64Vec4::new(1, 2, 3, 4).as_ivec4());
        assert_eq!(UVec4::new(1, 2, 3, 4), U64Vec4::new(1, 2, 3, 4).as_uvec4());
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            U64Vec4::new(1, 2, 3, 4).as_i64vec4()
        );
    });

    glam_test!(test_vec3a, {
        use glam::Vec3A;
        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::from((Vec3A::new(1.0, 2.0, 3.0), 4.0))
        );
        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::from((1.0, Vec3A::new(2.0, 3.0, 4.0)))
        );
    });

    #[cfg(not(feature = "scalar-math"))]
    impl_vec4_float_tests!(f32, vec4, Vec4, Vec3, Vec2, BVec4A, bvec4a);

    #[cfg(feature = "scalar-math")]
    impl_vec4_float_tests!(f32, vec4, Vec4, Vec3, Vec2, BVec4, bvec4);
}

mod dvec4 {
    use glam::{bvec4, dvec4, BVec4, DVec2, DVec3, DVec4, IVec4, UVec4, Vec4};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(32, mem::size_of::<DVec4>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(mem::align_of::<f64>(), mem::align_of::<DVec4>());
        #[cfg(feature = "cuda")]
        assert_eq!(16, mem::align_of::<DVec4>());
        assert_eq!(4, mem::size_of::<BVec4>());
        assert_eq!(1, mem::align_of::<BVec4>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            DVec4::from(Vec4::new(1.0, 2.0, 3.0, 4.0))
        );
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            DVec4::from(IVec4::new(1, 2, 3, 4))
        );
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            DVec4::from(UVec4::new(1, 2, 3, 4))
        );
    });

    impl_vec4_float_tests!(f64, dvec4, DVec4, DVec3, DVec2, BVec4, bvec4);
}

mod i8vec4 {
    use glam::{
        bvec4, i8vec4, BVec4, I16Vec4, I64Vec4, I8Vec2, I8Vec3, I8Vec4, IVec4, U16Vec4, U64Vec4,
        U8Vec4, UVec4,
    };

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(4, mem::size_of::<I8Vec4>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(1, mem::align_of::<I8Vec4>());
        #[cfg(feature = "cuda")]
        assert_eq!(4, mem::align_of::<I8Vec4>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            I8Vec4::try_from(U8Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I8Vec4::try_from(U8Vec4::new(u8::MAX, 2, 3, 4)).is_err());
        assert!(I8Vec4::try_from(U8Vec4::new(1, u8::MAX, 3, 4)).is_err());
        assert!(I8Vec4::try_from(U8Vec4::new(1, 2, u8::MAX, 4)).is_err());
        assert!(I8Vec4::try_from(U8Vec4::new(1, 2, 3, u8::MAX)).is_err());

        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            I8Vec4::try_from(I16Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I8Vec4::try_from(I16Vec4::new(i16::MAX, 2, 3, 4)).is_err());
        assert!(I8Vec4::try_from(I16Vec4::new(1, i16::MAX, 3, 4)).is_err());
        assert!(I8Vec4::try_from(I16Vec4::new(1, 2, i16::MAX, 4)).is_err());
        assert!(I8Vec4::try_from(I16Vec4::new(1, 2, 3, i16::MAX)).is_err());

        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            I8Vec4::try_from(U16Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I8Vec4::try_from(U16Vec4::new(u16::MAX, 2, 3, 4)).is_err());
        assert!(I8Vec4::try_from(U16Vec4::new(1, u16::MAX, 3, 4)).is_err());
        assert!(I8Vec4::try_from(U16Vec4::new(1, 2, u16::MAX, 4)).is_err());
        assert!(I8Vec4::try_from(U16Vec4::new(1, 2, 3, u16::MAX)).is_err());

        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            I8Vec4::try_from(IVec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I8Vec4::try_from(IVec4::new(i32::MAX, 2, 3, 4)).is_err());
        assert!(I8Vec4::try_from(IVec4::new(1, i32::MAX, 3, 4)).is_err());
        assert!(I8Vec4::try_from(IVec4::new(1, 2, i32::MAX, 4)).is_err());
        assert!(I8Vec4::try_from(IVec4::new(1, 2, 3, i32::MAX)).is_err());

        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            I8Vec4::try_from(UVec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I8Vec4::try_from(UVec4::new(u32::MAX, 2, 3, 4)).is_err());
        assert!(I8Vec4::try_from(UVec4::new(1, u32::MAX, 3, 4)).is_err());
        assert!(I8Vec4::try_from(UVec4::new(1, 2, u32::MAX, 4)).is_err());
        assert!(I8Vec4::try_from(UVec4::new(1, 2, 3, u32::MAX)).is_err());

        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            I8Vec4::try_from(I64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I8Vec4::try_from(I64Vec4::new(i64::MAX, 2, 3, 4)).is_err());
        assert!(I8Vec4::try_from(I64Vec4::new(1, i64::MAX, 3, 4)).is_err());
        assert!(I8Vec4::try_from(I64Vec4::new(1, 2, i64::MAX, 4)).is_err());
        assert!(I8Vec4::try_from(I64Vec4::new(1, 2, 3, i64::MAX)).is_err());

        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            I8Vec4::try_from(U64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I8Vec4::try_from(U64Vec4::new(u64::MAX, 2, 3, 4)).is_err());
        assert!(I8Vec4::try_from(U64Vec4::new(1, u64::MAX, 3, 4)).is_err());
        assert!(I8Vec4::try_from(U64Vec4::new(1, 2, u64::MAX, 4)).is_err());
        assert!(I8Vec4::try_from(U64Vec4::new(1, 2, 3, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add, {
        assert_eq!(
            I8Vec4::new(i8::MAX, 5, i8::MIN, 0).wrapping_add(I8Vec4::new(1, 3, i8::MAX, 0)),
            I8Vec4::new(i8::MIN, 8, -1, 0),
        );
    });

    glam_test!(test_wrapping_sub, {
        assert_eq!(
            I8Vec4::new(i8::MAX, 5, i8::MIN, 0).wrapping_sub(I8Vec4::new(1, 3, i8::MAX, 0)),
            I8Vec4::new(126, 2, 1, 0)
        );
    });

    glam_test!(test_wrapping_mul, {
        assert_eq!(
            I8Vec4::new(i8::MAX, 5, i8::MIN, 0).wrapping_mul(I8Vec4::new(3, 3, 5, 1)),
            I8Vec4::new(125, 15, -128, 0)
        );
    });

    glam_test!(test_wrapping_div, {
        assert_eq!(
            I8Vec4::new(i8::MAX, 5, i8::MIN, 0).wrapping_div(I8Vec4::new(3, 3, 5, 1)),
            I8Vec4::new(42, 1, -25, 0)
        );
    });

    glam_test!(test_saturating_add, {
        assert_eq!(
            I8Vec4::new(i8::MAX, i8::MIN, 0, 0).saturating_add(I8Vec4::new(1, -1, 2, 3)),
            I8Vec4::new(i8::MAX, i8::MIN, 2, 3)
        );
    });

    glam_test!(test_saturating_sub, {
        assert_eq!(
            I8Vec4::new(i8::MIN, i8::MAX, 0, 0).saturating_sub(I8Vec4::new(1, -1, 2, 3)),
            I8Vec4::new(i8::MIN, i8::MAX, -2, -3)
        );
    });

    glam_test!(test_saturating_mul, {
        assert_eq!(
            I8Vec4::new(i8::MAX, i8::MIN, 0, 0).saturating_mul(I8Vec4::new(2, 2, 0, 0)),
            I8Vec4::new(i8::MAX, i8::MIN, 0, 0)
        );
    });

    glam_test!(test_saturating_div, {
        assert_eq!(
            I8Vec4::new(i8::MAX, i8::MIN, 0, 0).saturating_div(I8Vec4::new(2, 2, 3, 4)),
            I8Vec4::new(63, -64, 0, 0)
        );
    });

    glam_test!(test_wrapping_add_unsigned, {
        assert_eq!(
            I8Vec4::new(i8::MAX, i8::MAX, i8::MAX, i8::MAX)
                .wrapping_add_unsigned(U8Vec4::new(1, 1, 1, 1)),
            I8Vec4::new(i8::MIN, i8::MIN, i8::MIN, i8::MIN)
        );
    });

    glam_test!(test_wrapping_sub_unsigned, {
        assert_eq!(
            I8Vec4::new(i8::MIN, i8::MIN, i8::MIN, i8::MIN)
                .wrapping_sub_unsigned(U8Vec4::new(1, 1, 1, 1)),
            I8Vec4::new(i8::MAX, i8::MAX, i8::MAX, i8::MAX)
        );
    });

    glam_test!(test_saturating_add_unsigned, {
        assert_eq!(
            I8Vec4::new(i8::MAX, i8::MAX, i8::MAX, i8::MAX)
                .saturating_add_unsigned(U8Vec4::new(1, 1, 1, 1)),
            I8Vec4::new(i8::MAX, i8::MAX, i8::MAX, i8::MAX)
        );
    });

    glam_test!(test_saturating_sub_unsigned, {
        assert_eq!(
            I8Vec4::new(i8::MIN, i8::MIN, i8::MIN, i8::MIN)
                .saturating_sub_unsigned(U8Vec4::new(1, 1, 1, 1)),
            I8Vec4::new(i8::MIN, i8::MIN, i8::MIN, i8::MIN)
        );
    });

    impl_vec4_signed_integer_tests!(i8, i8vec4, I8Vec4, I8Vec3, I8Vec2, BVec4, bvec4);
    impl_vec4_eq_hash_tests!(i8, i8vec4);

    impl_vec4_scalar_shift_op_tests!(I8Vec4, -2, 2);
    impl_vec4_shift_op_tests!(I8Vec4);

    impl_vec4_scalar_bit_op_tests!(I8Vec4, -2, 2);
    impl_vec4_bit_op_tests!(I8Vec4, -2, 2);
}

mod u8vec4 {
    use glam::{
        bvec4, u8vec4, BVec4, I16Vec4, I64Vec4, I8Vec4, IVec4, U16Vec4, U64Vec4, U8Vec2, U8Vec3,
        U8Vec4, UVec4,
    };

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(4, mem::size_of::<U8Vec4>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(1, mem::align_of::<U8Vec4>());
        #[cfg(feature = "cuda")]
        assert_eq!(4, mem::align_of::<U8Vec4>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            U8Vec4::try_from(I8Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U8Vec4::try_from(I8Vec4::new(-1, 2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(I8Vec4::new(1, -2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(I8Vec4::new(1, 2, -3, 4)).is_err());
        assert!(U8Vec4::try_from(I8Vec4::new(1, 2, 3, -4)).is_err());

        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            U8Vec4::try_from(I16Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U8Vec4::try_from(I16Vec4::new(-1, 2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(I16Vec4::new(1, -2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(I16Vec4::new(1, 2, -3, 4)).is_err());
        assert!(U8Vec4::try_from(I16Vec4::new(1, 2, 3, -4)).is_err());

        assert!(U8Vec4::try_from(I16Vec4::new(i16::MAX, 2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(I16Vec4::new(1, i16::MAX, 3, 4)).is_err());
        assert!(U8Vec4::try_from(I16Vec4::new(1, 2, i16::MAX, 4)).is_err());
        assert!(U8Vec4::try_from(I16Vec4::new(1, 2, 3, i16::MAX)).is_err());

        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            U8Vec4::try_from(U16Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U8Vec4::try_from(U16Vec4::new(u16::MAX, 2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(U16Vec4::new(1, u16::MAX, 3, 4)).is_err());
        assert!(U8Vec4::try_from(U16Vec4::new(1, 2, u16::MAX, 4)).is_err());
        assert!(U8Vec4::try_from(U16Vec4::new(1, 2, 3, u16::MAX)).is_err());

        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            U8Vec4::try_from(IVec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U8Vec4::try_from(IVec4::new(-1, 2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(IVec4::new(1, -2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(IVec4::new(1, 2, -3, 4)).is_err());
        assert!(U8Vec4::try_from(IVec4::new(1, 2, 3, -4)).is_err());

        assert!(U8Vec4::try_from(IVec4::new(i32::MAX, 2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(IVec4::new(1, i32::MAX, 3, 4)).is_err());
        assert!(U8Vec4::try_from(IVec4::new(1, 2, i32::MAX, 4)).is_err());
        assert!(U8Vec4::try_from(IVec4::new(1, 2, 3, i32::MAX)).is_err());

        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            U8Vec4::try_from(UVec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U8Vec4::try_from(UVec4::new(u32::MAX, 2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(UVec4::new(1, u32::MAX, 3, 4)).is_err());
        assert!(U8Vec4::try_from(UVec4::new(1, 2, u32::MAX, 4)).is_err());
        assert!(U8Vec4::try_from(UVec4::new(1, 2, 3, u32::MAX)).is_err());

        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            U8Vec4::try_from(I64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U8Vec4::try_from(I64Vec4::new(-1, 2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(I64Vec4::new(1, -2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(I64Vec4::new(1, 2, -3, 4)).is_err());
        assert!(U8Vec4::try_from(I64Vec4::new(1, 2, 3, -4)).is_err());

        assert!(U8Vec4::try_from(I64Vec4::new(i64::MAX, 2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(I64Vec4::new(1, i64::MAX, 3, 4)).is_err());
        assert!(U8Vec4::try_from(I64Vec4::new(1, 2, i64::MAX, 4)).is_err());
        assert!(U8Vec4::try_from(I64Vec4::new(1, 2, 3, i64::MAX)).is_err());

        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            U8Vec4::try_from(U64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U8Vec4::try_from(U64Vec4::new(u64::MAX, 2, 3, 4)).is_err());
        assert!(U8Vec4::try_from(U64Vec4::new(1, u64::MAX, 3, 4)).is_err());
        assert!(U8Vec4::try_from(U64Vec4::new(1, 2, u64::MAX, 4)).is_err());
        assert!(U8Vec4::try_from(U64Vec4::new(1, 2, 3, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add, {
        assert_eq!(
            U8Vec4::new(u8::MAX, 5, u8::MAX, 0).wrapping_add(U8Vec4::new(1, 3, u8::MAX, 0)),
            U8Vec4::new(0, 8, 254, 0),
        );
    });

    glam_test!(test_wrapping_sub, {
        assert_eq!(
            U8Vec4::new(u8::MAX, 5, u8::MAX - 1, 0).wrapping_sub(U8Vec4::new(1, 3, u8::MAX, 0)),
            U8Vec4::new(254, 2, 255, 0)
        );
    });

    glam_test!(test_wrapping_mul, {
        assert_eq!(
            U8Vec4::new(u8::MAX, 5, u8::MAX, 0).wrapping_mul(U8Vec4::new(3, 3, 5, 1)),
            U8Vec4::new(253, 15, 251, 0)
        );
    });

    glam_test!(test_wrapping_div, {
        assert_eq!(
            U8Vec4::new(u8::MAX, 5, u8::MAX, 0).wrapping_div(U8Vec4::new(3, 3, 5, 1)),
            U8Vec4::new(85, 1, 51, 0)
        );
    });

    glam_test!(test_saturating_add, {
        assert_eq!(
            U8Vec4::new(u8::MAX, u8::MAX, 0, 0).saturating_add(U8Vec4::new(1, u8::MAX, 2, 3)),
            U8Vec4::new(u8::MAX, u8::MAX, 2, 3)
        );
    });

    glam_test!(test_saturating_sub, {
        assert_eq!(
            U8Vec4::new(0, u8::MAX, 0, 0).saturating_sub(U8Vec4::new(1, 1, 2, 3)),
            U8Vec4::new(0, 254, 0, 0)
        );
    });

    glam_test!(test_saturating_mul, {
        assert_eq!(
            U8Vec4::new(u8::MAX, u8::MAX, 0, 0).saturating_mul(U8Vec4::new(2, u8::MAX, 0, 0)),
            U8Vec4::new(u8::MAX, u8::MAX, 0, 0)
        );
    });

    glam_test!(test_saturating_div, {
        assert_eq!(
            U8Vec4::new(u8::MAX, u8::MAX, 0, 0).saturating_div(U8Vec4::new(2, u8::MAX, 3, 4)),
            U8Vec4::new(127, 1, 0, 0)
        );
    });

    glam_test!(test_wrapping_add_signed, {
        assert_eq!(
            U8Vec4::new(u8::MAX, u8::MAX, u8::MAX, u8::MAX)
                .wrapping_add_signed(I8Vec4::new(1, 1, 1, 1)),
            U8Vec4::new(u8::MIN, u8::MIN, u8::MIN, u8::MIN)
        );
    });

    glam_test!(test_saturating_add_signed, {
        assert_eq!(
            U8Vec4::new(u8::MAX, u8::MAX, u8::MAX, u8::MAX)
                .saturating_add_signed(I8Vec4::new(1, 1, 1, 1)),
            U8Vec4::new(u8::MAX, u8::MAX, u8::MAX, u8::MAX)
        );
    });

    impl_vec4_tests!(u8, u8vec4, U8Vec4, U8Vec3, U8Vec2, BVec4, bvec4);
    impl_vec4_eq_hash_tests!(u8, u8vec4);

    impl_vec4_scalar_shift_op_tests!(U8Vec4, 0, 2);
    impl_vec4_shift_op_tests!(U8Vec4);

    impl_vec4_scalar_bit_op_tests!(U8Vec4, 0, 2);
    impl_vec4_bit_op_tests!(U8Vec4, 0, 2);
}

mod i16vec4 {
    use glam::{
        bvec4, i16vec4, BVec4, I16Vec2, I16Vec3, I16Vec4, I64Vec4, I8Vec4, IVec4, U16Vec4, U64Vec4,
        U8Vec4, UVec4,
    };

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(8, mem::size_of::<I16Vec4>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(2, mem::align_of::<I16Vec4>());
        #[cfg(feature = "cuda")]
        assert_eq!(8, mem::align_of::<I16Vec4>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            I16Vec4::from(U8Vec4::new(1, 2, 3, 4))
        );
        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            I16Vec4::from(I8Vec4::new(1, 2, 3, 4))
        );

        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            I16Vec4::try_from(U16Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I16Vec4::try_from(U16Vec4::new(u16::MAX, 2, 3, 4)).is_err());
        assert!(I16Vec4::try_from(U16Vec4::new(1, u16::MAX, 3, 4)).is_err());
        assert!(I16Vec4::try_from(U16Vec4::new(1, 2, u16::MAX, 4)).is_err());
        assert!(I16Vec4::try_from(U16Vec4::new(1, 2, 3, u16::MAX)).is_err());

        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            I16Vec4::try_from(IVec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I16Vec4::try_from(IVec4::new(i32::MAX, 2, 3, 4)).is_err());
        assert!(I16Vec4::try_from(IVec4::new(1, i32::MAX, 3, 4)).is_err());
        assert!(I16Vec4::try_from(IVec4::new(1, 2, i32::MAX, 4)).is_err());
        assert!(I16Vec4::try_from(IVec4::new(1, 2, 3, i32::MAX)).is_err());

        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            I16Vec4::try_from(UVec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I16Vec4::try_from(UVec4::new(u32::MAX, 2, 3, 4)).is_err());
        assert!(I16Vec4::try_from(UVec4::new(1, u32::MAX, 3, 4)).is_err());
        assert!(I16Vec4::try_from(UVec4::new(1, 2, u32::MAX, 4)).is_err());
        assert!(I16Vec4::try_from(UVec4::new(1, 2, 3, u32::MAX)).is_err());

        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            I16Vec4::try_from(I64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I16Vec4::try_from(I64Vec4::new(i64::MAX, 2, 3, 4)).is_err());
        assert!(I16Vec4::try_from(I64Vec4::new(1, i64::MAX, 3, 4)).is_err());
        assert!(I16Vec4::try_from(I64Vec4::new(1, 2, i64::MAX, 4)).is_err());
        assert!(I16Vec4::try_from(I64Vec4::new(1, 2, 3, i64::MAX)).is_err());

        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            I16Vec4::try_from(U64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I16Vec4::try_from(U64Vec4::new(u64::MAX, 2, 3, 4)).is_err());
        assert!(I16Vec4::try_from(U64Vec4::new(1, u64::MAX, 3, 4)).is_err());
        assert!(I16Vec4::try_from(U64Vec4::new(1, 2, u64::MAX, 4)).is_err());
        assert!(I16Vec4::try_from(U64Vec4::new(1, 2, 3, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add, {
        assert_eq!(
            I16Vec4::new(i16::MAX, 5, i16::MIN, 0).wrapping_add(I16Vec4::new(1, 3, i16::MAX, 0)),
            I16Vec4::new(i16::MIN, 8, -1, 0),
        );
    });

    glam_test!(test_wrapping_sub, {
        assert_eq!(
            I16Vec4::new(i16::MAX, 5, i16::MIN, 0).wrapping_sub(I16Vec4::new(1, 3, i16::MAX, 0)),
            I16Vec4::new(32766, 2, 1, 0)
        );
    });

    glam_test!(test_wrapping_mul, {
        assert_eq!(
            I16Vec4::new(i16::MAX, 5, i16::MIN, 0).wrapping_mul(I16Vec4::new(3, 3, 5, 1)),
            I16Vec4::new(32765, 15, -32768, 0)
        );
    });

    glam_test!(test_wrapping_div, {
        assert_eq!(
            I16Vec4::new(i16::MAX, 5, i16::MIN, 0).wrapping_div(I16Vec4::new(3, 3, 5, 1)),
            I16Vec4::new(10922, 1, -6553, 0)
        );
    });

    glam_test!(test_saturating_add, {
        assert_eq!(
            I16Vec4::new(i16::MAX, i16::MIN, 0, 0).saturating_add(I16Vec4::new(1, -1, 2, 3)),
            I16Vec4::new(i16::MAX, i16::MIN, 2, 3)
        );
    });

    glam_test!(test_saturating_sub, {
        assert_eq!(
            I16Vec4::new(i16::MIN, i16::MAX, 0, 0).saturating_sub(I16Vec4::new(1, -1, 2, 3)),
            I16Vec4::new(i16::MIN, i16::MAX, -2, -3)
        );
    });

    glam_test!(test_saturating_mul, {
        assert_eq!(
            I16Vec4::new(i16::MAX, i16::MIN, 0, 0).saturating_mul(I16Vec4::new(2, 2, 0, 0)),
            I16Vec4::new(i16::MAX, i16::MIN, 0, 0)
        );
    });

    glam_test!(test_saturating_div, {
        assert_eq!(
            I16Vec4::new(i16::MAX, i16::MIN, 0, 0).saturating_div(I16Vec4::new(2, 2, 3, 4)),
            I16Vec4::new(16383, -16384, 0, 0)
        );
    });

    glam_test!(test_wrapping_add_unsigned, {
        assert_eq!(
            I16Vec4::new(i16::MAX, i16::MAX, i16::MAX, i16::MAX)
                .wrapping_add_unsigned(U16Vec4::new(1, 1, 1, 1)),
            I16Vec4::new(i16::MIN, i16::MIN, i16::MIN, i16::MIN)
        );
    });

    glam_test!(test_wrapping_sub_unsigned, {
        assert_eq!(
            I16Vec4::new(i16::MIN, i16::MIN, i16::MIN, i16::MIN)
                .wrapping_sub_unsigned(U16Vec4::new(1, 1, 1, 1)),
            I16Vec4::new(i16::MAX, i16::MAX, i16::MAX, i16::MAX)
        );
    });

    glam_test!(test_saturating_add_unsigned, {
        assert_eq!(
            I16Vec4::new(i16::MAX, i16::MAX, i16::MAX, i16::MAX)
                .saturating_add_unsigned(U16Vec4::new(1, 1, 1, 1)),
            I16Vec4::new(i16::MAX, i16::MAX, i16::MAX, i16::MAX)
        );
    });

    glam_test!(test_saturating_sub_unsigned, {
        assert_eq!(
            I16Vec4::new(i16::MIN, i16::MIN, i16::MIN, i16::MIN)
                .saturating_sub_unsigned(U16Vec4::new(1, 1, 1, 1)),
            I16Vec4::new(i16::MIN, i16::MIN, i16::MIN, i16::MIN)
        );
    });

    impl_vec4_signed_integer_tests!(i16, i16vec4, I16Vec4, I16Vec3, I16Vec2, BVec4, bvec4);
    impl_vec4_eq_hash_tests!(i16, i16vec4);

    impl_vec4_scalar_shift_op_tests!(I16Vec4, -2, 2);
    impl_vec4_shift_op_tests!(I16Vec4);

    impl_vec4_scalar_bit_op_tests!(I16Vec4, -2, 2);
    impl_vec4_bit_op_tests!(I16Vec4, -2, 2);
}

mod u16vec4 {
    use glam::{
        bvec4, u16vec4, BVec4, I16Vec4, I64Vec4, I8Vec4, IVec4, U16Vec2, U16Vec3, U16Vec4, U64Vec4,
        U8Vec4, UVec4,
    };

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(8, mem::size_of::<U16Vec4>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(2, mem::align_of::<U16Vec4>());
        #[cfg(feature = "cuda")]
        assert_eq!(8, mem::align_of::<U16Vec4>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            U16Vec4::try_from(I8Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U16Vec4::try_from(I8Vec4::new(-1, 2, 3, 4)).is_err());
        assert!(U16Vec4::try_from(I8Vec4::new(1, -2, 3, 4)).is_err());
        assert!(U16Vec4::try_from(I8Vec4::new(1, 2, -3, 4)).is_err());
        assert!(U16Vec4::try_from(I8Vec4::new(1, 2, 3, -4)).is_err());

        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            U16Vec4::from(U8Vec4::new(1, 2, 3, 4))
        );

        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            U16Vec4::try_from(I16Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U16Vec4::try_from(I16Vec4::new(-1, 2, 3, 4)).is_err());
        assert!(U16Vec4::try_from(I16Vec4::new(1, -2, 3, 4)).is_err());
        assert!(U16Vec4::try_from(I16Vec4::new(1, 2, -3, 4)).is_err());
        assert!(U16Vec4::try_from(I16Vec4::new(1, 2, 3, -4)).is_err());

        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            U16Vec4::try_from(IVec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U16Vec4::try_from(IVec4::new(-1, 2, 3, 4)).is_err());
        assert!(U16Vec4::try_from(IVec4::new(1, -2, 3, 4)).is_err());
        assert!(U16Vec4::try_from(IVec4::new(1, 2, -3, 4)).is_err());
        assert!(U16Vec4::try_from(IVec4::new(1, 2, 3, -4)).is_err());

        assert!(U16Vec4::try_from(IVec4::new(i32::MAX, 2, 3, 4)).is_err());
        assert!(U16Vec4::try_from(IVec4::new(1, i32::MAX, 3, 4)).is_err());
        assert!(U16Vec4::try_from(IVec4::new(1, 2, i32::MAX, 4)).is_err());
        assert!(U16Vec4::try_from(IVec4::new(1, 2, 3, i32::MAX)).is_err());

        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            U16Vec4::try_from(UVec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U16Vec4::try_from(UVec4::new(u32::MAX, 2, 3, 4)).is_err());
        assert!(U16Vec4::try_from(UVec4::new(1, u32::MAX, 3, 4)).is_err());
        assert!(U16Vec4::try_from(UVec4::new(1, 2, u32::MAX, 4)).is_err());
        assert!(U16Vec4::try_from(UVec4::new(1, 2, 3, u32::MAX)).is_err());

        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            U16Vec4::try_from(I64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U16Vec4::try_from(I64Vec4::new(-1, 2, 3, 4)).is_err());
        assert!(U16Vec4::try_from(I64Vec4::new(1, -2, 3, 4)).is_err());
        assert!(U16Vec4::try_from(I64Vec4::new(1, 2, -3, 4)).is_err());
        assert!(U16Vec4::try_from(I64Vec4::new(1, 2, 3, -4)).is_err());

        assert!(U16Vec4::try_from(I64Vec4::new(i64::MAX, 2, 3, 4)).is_err());
        assert!(U16Vec4::try_from(I64Vec4::new(1, i64::MAX, 3, 4)).is_err());
        assert!(U16Vec4::try_from(I64Vec4::new(1, 2, i64::MAX, 4)).is_err());
        assert!(U16Vec4::try_from(I64Vec4::new(1, 2, 3, i64::MAX)).is_err());

        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            U16Vec4::try_from(U64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U16Vec4::try_from(U64Vec4::new(u64::MAX, 2, 3, 4)).is_err());
        assert!(U16Vec4::try_from(U64Vec4::new(1, u64::MAX, 3, 4)).is_err());
        assert!(U16Vec4::try_from(U64Vec4::new(1, 2, u64::MAX, 4)).is_err());
        assert!(U16Vec4::try_from(U64Vec4::new(1, 2, 3, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add, {
        assert_eq!(
            U16Vec4::new(u16::MAX, 5, u16::MAX, 0).wrapping_add(U16Vec4::new(1, 3, u16::MAX, 0)),
            U16Vec4::new(0, 8, 65534, 0),
        );
    });

    glam_test!(test_wrapping_sub, {
        assert_eq!(
            U16Vec4::new(u16::MAX, 5, u16::MAX - 1, 0).wrapping_sub(U16Vec4::new(
                1,
                3,
                u16::MAX,
                0
            )),
            U16Vec4::new(65534, 2, 65535, 0)
        );
    });

    glam_test!(test_wrapping_mul, {
        assert_eq!(
            U16Vec4::new(u16::MAX, 5, u16::MAX, 0).wrapping_mul(U16Vec4::new(3, 3, 5, 1)),
            U16Vec4::new(65533, 15, 65531, 0)
        );
    });

    glam_test!(test_wrapping_div, {
        assert_eq!(
            U16Vec4::new(u16::MAX, 5, u16::MAX, 0).wrapping_div(U16Vec4::new(3, 3, 5, 1)),
            U16Vec4::new(21845, 1, 13107, 0)
        );
    });

    glam_test!(test_saturating_add, {
        assert_eq!(
            U16Vec4::new(u16::MAX, u16::MAX, 0, 0).saturating_add(U16Vec4::new(1, u16::MAX, 2, 3)),
            U16Vec4::new(u16::MAX, u16::MAX, 2, 3)
        );
    });

    glam_test!(test_saturating_sub, {
        assert_eq!(
            U16Vec4::new(0, u16::MAX, 0, 0).saturating_sub(U16Vec4::new(1, 1, 2, 3)),
            U16Vec4::new(0, 65534, 0, 0)
        );
    });

    glam_test!(test_saturating_mul, {
        assert_eq!(
            U16Vec4::new(u16::MAX, u16::MAX, 0, 0).saturating_mul(U16Vec4::new(2, u16::MAX, 0, 0)),
            U16Vec4::new(u16::MAX, u16::MAX, 0, 0)
        );
    });

    glam_test!(test_saturating_div, {
        assert_eq!(
            U16Vec4::new(u16::MAX, u16::MAX, 0, 0).saturating_div(U16Vec4::new(2, u16::MAX, 3, 4)),
            U16Vec4::new(32767, 1, 0, 0)
        );
    });

    glam_test!(test_wrapping_add_signed, {
        assert_eq!(
            U16Vec4::new(u16::MAX, u16::MAX, u16::MAX, u16::MAX)
                .wrapping_add_signed(I16Vec4::new(1, 1, 1, 1)),
            U16Vec4::new(u16::MIN, u16::MIN, u16::MIN, u16::MIN)
        );
    });

    glam_test!(test_saturating_add_signed, {
        assert_eq!(
            U16Vec4::new(u16::MAX, u16::MAX, u16::MAX, u16::MAX)
                .saturating_add_signed(I16Vec4::new(1, 1, 1, 1)),
            U16Vec4::new(u16::MAX, u16::MAX, u16::MAX, u16::MAX)
        );
    });

    impl_vec4_tests!(u16, u16vec4, U16Vec4, U16Vec3, U16Vec2, BVec4, bvec4);
    impl_vec4_eq_hash_tests!(u16, u16vec4);

    impl_vec4_scalar_shift_op_tests!(U16Vec4, 0, 2);
    impl_vec4_shift_op_tests!(U16Vec4);

    impl_vec4_scalar_bit_op_tests!(U16Vec4, 0, 2);
    impl_vec4_bit_op_tests!(U16Vec4, 0, 2);
}

mod ivec4 {
    use glam::{
        bvec4, ivec4, BVec4, I16Vec4, I64Vec4, I8Vec4, IVec2, IVec3, IVec4, U16Vec4, U64Vec4,
        U8Vec4, UVec4,
    };

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(16, mem::size_of::<IVec4>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(4, mem::align_of::<IVec4>());
        #[cfg(feature = "cuda")]
        assert_eq!(16, mem::align_of::<IVec4>());
        assert_eq!(4, mem::size_of::<BVec4>());
        assert_eq!(1, mem::align_of::<BVec4>());
    });

    glam_test!(test_try_from, {
        assert_eq!(IVec4::new(1, 2, 3, 4), IVec4::from(U8Vec4::new(1, 2, 3, 4)));
        assert_eq!(IVec4::new(1, 2, 3, 4), IVec4::from(I8Vec4::new(1, 2, 3, 4)));

        assert_eq!(
            IVec4::new(1, 2, 3, 4),
            IVec4::from(U16Vec4::new(1, 2, 3, 4))
        );
        assert_eq!(
            IVec4::new(1, 2, 3, 4),
            IVec4::from(I16Vec4::new(1, 2, 3, 4))
        );

        assert_eq!(
            IVec4::new(1, 2, 3, 4),
            IVec4::try_from(UVec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(IVec4::try_from(UVec4::new(u32::MAX, 2, 3, 4)).is_err());
        assert!(IVec4::try_from(UVec4::new(1, u32::MAX, 3, 4)).is_err());
        assert!(IVec4::try_from(UVec4::new(1, 2, u32::MAX, 4)).is_err());
        assert!(IVec4::try_from(UVec4::new(1, 2, 3, u32::MAX)).is_err());

        assert_eq!(
            IVec4::new(1, 2, 3, 4),
            IVec4::try_from(I64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(IVec4::try_from(I64Vec4::new(i64::MAX, 2, 3, 4)).is_err());
        assert!(IVec4::try_from(I64Vec4::new(1, i64::MAX, 3, 4)).is_err());
        assert!(IVec4::try_from(I64Vec4::new(1, 2, i64::MAX, 4)).is_err());
        assert!(IVec4::try_from(I64Vec4::new(1, 2, 3, i64::MAX)).is_err());

        assert_eq!(
            IVec4::new(1, 2, 3, 4),
            IVec4::try_from(U64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(IVec4::try_from(U64Vec4::new(u64::MAX, 2, 3, 4)).is_err());
        assert!(IVec4::try_from(U64Vec4::new(1, u64::MAX, 3, 4)).is_err());
        assert!(IVec4::try_from(U64Vec4::new(1, 2, u64::MAX, 4)).is_err());
        assert!(IVec4::try_from(U64Vec4::new(1, 2, 3, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add, {
        assert_eq!(
            IVec4::new(i32::MAX, 5, i32::MIN, 0).wrapping_add(IVec4::new(1, 3, i32::MAX, 0)),
            IVec4::new(i32::MIN, 8, -1, 0),
        );
    });

    glam_test!(test_wrapping_sub, {
        assert_eq!(
            IVec4::new(i32::MAX, 5, i32::MIN, 0).wrapping_sub(IVec4::new(1, 3, i32::MAX, 0)),
            IVec4::new(2147483646, 2, 1, 0)
        );
    });

    glam_test!(test_wrapping_mul, {
        assert_eq!(
            IVec4::new(i32::MAX, 5, i32::MIN, 0).wrapping_mul(IVec4::new(3, 3, 5, 1)),
            IVec4::new(2147483645, 15, -2147483648, 0)
        );
    });

    glam_test!(test_wrapping_div, {
        assert_eq!(
            IVec4::new(i32::MAX, 5, i32::MIN, 0).wrapping_div(IVec4::new(3, 3, 5, 1)),
            IVec4::new(715827882, 1, -429496729, 0)
        );
    });

    glam_test!(test_saturating_add, {
        assert_eq!(
            IVec4::new(i32::MAX, i32::MIN, 0, 0).saturating_add(IVec4::new(1, -1, 2, 3)),
            IVec4::new(i32::MAX, i32::MIN, 2, 3)
        );
    });

    glam_test!(test_saturating_sub, {
        assert_eq!(
            IVec4::new(i32::MIN, i32::MAX, 0, 0).saturating_sub(IVec4::new(1, -1, 2, 3)),
            IVec4::new(i32::MIN, i32::MAX, -2, -3)
        );
    });

    glam_test!(test_saturating_mul, {
        assert_eq!(
            IVec4::new(i32::MAX, i32::MIN, 0, 0).saturating_mul(IVec4::new(2, 2, 0, 0)),
            IVec4::new(i32::MAX, i32::MIN, 0, 0)
        );
    });

    glam_test!(test_saturating_div, {
        assert_eq!(
            IVec4::new(i32::MAX, i32::MIN, 0, 0).saturating_div(IVec4::new(2, 2, 3, 4)),
            IVec4::new(1073741823, -1073741824, 0, 0)
        );
    });

    glam_test!(test_wrapping_add_unsigned, {
        assert_eq!(
            IVec4::new(i32::MAX, i32::MAX, i32::MAX, i32::MAX)
                .wrapping_add_unsigned(UVec4::new(1, 1, 1, 1)),
            IVec4::new(i32::MIN, i32::MIN, i32::MIN, i32::MIN)
        );
    });

    glam_test!(test_wrapping_sub_unsigned, {
        assert_eq!(
            IVec4::new(i32::MIN, i32::MIN, i32::MIN, i32::MIN)
                .wrapping_sub_unsigned(UVec4::new(1, 1, 1, 1)),
            IVec4::new(i32::MAX, i32::MAX, i32::MAX, i32::MAX)
        );
    });

    glam_test!(test_saturating_add_unsigned, {
        assert_eq!(
            IVec4::new(i32::MAX, i32::MAX, i32::MAX, i32::MAX)
                .saturating_add_unsigned(UVec4::new(1, 1, 1, 1)),
            IVec4::new(i32::MAX, i32::MAX, i32::MAX, i32::MAX)
        );
    });

    glam_test!(test_saturating_sub_unsigned, {
        assert_eq!(
            IVec4::new(i32::MIN, i32::MIN, i32::MIN, i32::MIN)
                .saturating_sub_unsigned(UVec4::new(1, 1, 1, 1)),
            IVec4::new(i32::MIN, i32::MIN, i32::MIN, i32::MIN)
        );
    });

    impl_vec4_signed_integer_tests!(i32, ivec4, IVec4, IVec3, IVec2, BVec4, bvec4);
    impl_vec4_eq_hash_tests!(i32, ivec4);

    impl_vec4_scalar_shift_op_tests!(IVec4, -2, 2);
    impl_vec4_shift_op_tests!(IVec4);

    impl_vec4_scalar_bit_op_tests!(IVec4, -2, 2);
    impl_vec4_bit_op_tests!(IVec4, -2, 2);
}

mod uvec4 {
    use glam::{
        bvec4, uvec4, BVec4, I16Vec4, I64Vec4, I8Vec4, IVec4, U16Vec4, U64Vec4, U8Vec4, UVec2,
        UVec3, UVec4,
    };

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(16, mem::size_of::<UVec4>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(4, mem::align_of::<UVec4>());
        #[cfg(feature = "cuda")]
        assert_eq!(16, mem::align_of::<UVec4>());
        assert_eq!(4, mem::size_of::<BVec4>());
        assert_eq!(1, mem::align_of::<BVec4>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            UVec4::try_from(I8Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(UVec4::try_from(I8Vec4::new(-1, 2, 3, 4)).is_err());
        assert!(UVec4::try_from(I8Vec4::new(1, -2, 3, 4)).is_err());
        assert!(UVec4::try_from(I8Vec4::new(1, 2, -3, 4)).is_err());
        assert!(UVec4::try_from(I8Vec4::new(1, 2, 3, -4)).is_err());

        assert_eq!(UVec4::new(1, 2, 3, 4), UVec4::from(U8Vec4::new(1, 2, 3, 4)));

        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            UVec4::try_from(I16Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(UVec4::try_from(I16Vec4::new(-1, 2, 3, 4)).is_err());
        assert!(UVec4::try_from(I16Vec4::new(1, -2, 3, 4)).is_err());
        assert!(UVec4::try_from(I16Vec4::new(1, 2, -3, 4)).is_err());
        assert!(UVec4::try_from(I16Vec4::new(1, 2, 3, -4)).is_err());

        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            UVec4::from(U16Vec4::new(1, 2, 3, 4))
        );

        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            UVec4::try_from(IVec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(UVec4::try_from(IVec4::new(-1, 2, 3, 4)).is_err());
        assert!(UVec4::try_from(IVec4::new(1, -2, 3, 4)).is_err());
        assert!(UVec4::try_from(IVec4::new(1, 2, -3, 4)).is_err());
        assert!(UVec4::try_from(IVec4::new(1, 2, 3, -4)).is_err());

        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            UVec4::try_from(I64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(UVec4::try_from(I64Vec4::new(-1, 2, 3, 4)).is_err());
        assert!(UVec4::try_from(I64Vec4::new(1, -2, 3, 4)).is_err());
        assert!(UVec4::try_from(I64Vec4::new(1, 2, -3, 4)).is_err());
        assert!(UVec4::try_from(I64Vec4::new(1, 2, 3, -4)).is_err());

        assert!(UVec4::try_from(I64Vec4::new(i64::MAX, 2, 3, 4)).is_err());
        assert!(UVec4::try_from(I64Vec4::new(1, i64::MAX, 3, 4)).is_err());
        assert!(UVec4::try_from(I64Vec4::new(1, 2, i64::MAX, 4)).is_err());
        assert!(UVec4::try_from(I64Vec4::new(1, 2, 3, i64::MAX)).is_err());

        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            UVec4::try_from(U64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(UVec4::try_from(U64Vec4::new(u64::MAX, 2, 3, 4)).is_err());
        assert!(UVec4::try_from(U64Vec4::new(1, u64::MAX, 3, 4)).is_err());
        assert!(UVec4::try_from(U64Vec4::new(1, 2, u64::MAX, 4)).is_err());
        assert!(UVec4::try_from(U64Vec4::new(1, 2, 3, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add, {
        assert_eq!(
            UVec4::new(u32::MAX, 5, u32::MAX, 0).wrapping_add(UVec4::new(1, 3, u32::MAX, 0)),
            UVec4::new(0, 8, 4294967294, 0),
        );
    });

    glam_test!(test_wrapping_sub, {
        assert_eq!(
            UVec4::new(u32::MAX, 5, u32::MAX - 1, 0).wrapping_sub(UVec4::new(1, 3, u32::MAX, 0)),
            UVec4::new(4294967294, 2, 4294967295, 0)
        );
    });

    glam_test!(test_wrapping_mul, {
        assert_eq!(
            UVec4::new(u32::MAX, 5, u32::MAX, 0).wrapping_mul(UVec4::new(3, 3, 5, 1)),
            UVec4::new(4294967293, 15, 4294967291, 0)
        );
    });

    glam_test!(test_wrapping_div, {
        assert_eq!(
            UVec4::new(u32::MAX, 5, u32::MAX, 0).wrapping_div(UVec4::new(3, 3, 5, 1)),
            UVec4::new(1431655765, 1, 858993459, 0)
        );
    });

    glam_test!(test_saturating_add, {
        assert_eq!(
            UVec4::new(u32::MAX, u32::MAX, 0, 0).saturating_add(UVec4::new(1, u32::MAX, 2, 3)),
            UVec4::new(u32::MAX, u32::MAX, 2, 3)
        );
    });

    glam_test!(test_saturating_sub, {
        assert_eq!(
            UVec4::new(0, u32::MAX, 0, 0).saturating_sub(UVec4::new(1, 1, 2, 3)),
            UVec4::new(0, 4294967294, 0, 0)
        );
    });

    glam_test!(test_saturating_mul, {
        assert_eq!(
            UVec4::new(u32::MAX, u32::MAX, 0, 0).saturating_mul(UVec4::new(2, u32::MAX, 0, 0)),
            UVec4::new(u32::MAX, u32::MAX, 0, 0)
        );
    });

    glam_test!(test_saturating_div, {
        assert_eq!(
            UVec4::new(u32::MAX, u32::MAX, 0, 0).saturating_div(UVec4::new(2, u32::MAX, 3, 4)),
            UVec4::new(2147483647, 1, 0, 0)
        );
    });

    glam_test!(test_wrapping_add_signed, {
        assert_eq!(
            UVec4::new(u32::MAX, u32::MAX, u32::MAX, u32::MAX)
                .wrapping_add_signed(IVec4::new(1, 1, 1, 1)),
            UVec4::new(u32::MIN, u32::MIN, u32::MIN, u32::MIN)
        );
    });

    glam_test!(test_saturating_add_signed, {
        assert_eq!(
            UVec4::new(u32::MAX, u32::MAX, u32::MAX, u32::MAX)
                .saturating_add_signed(IVec4::new(1, 1, 1, 1)),
            UVec4::new(u32::MAX, u32::MAX, u32::MAX, u32::MAX)
        );
    });

    impl_vec4_tests!(u32, uvec4, UVec4, UVec3, UVec2, BVec4, bvec4);
    impl_vec4_eq_hash_tests!(u32, uvec4);

    impl_vec4_scalar_shift_op_tests!(UVec4, 0, 2);
    impl_vec4_shift_op_tests!(UVec4);

    impl_vec4_scalar_bit_op_tests!(UVec4, 0, 2);
    impl_vec4_bit_op_tests!(UVec4, 0, 2);
}

mod i64vec4 {
    use glam::{
        bvec4, i64vec4, BVec4, I16Vec4, I64Vec2, I64Vec3, I64Vec4, I8Vec4, IVec4, U16Vec4, U64Vec4,
        U8Vec4, UVec4,
    };

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(32, mem::size_of::<I64Vec4>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(8, mem::align_of::<I64Vec4>());
        #[cfg(feature = "cuda")]
        assert_eq!(16, mem::align_of::<I64Vec4>());
        assert_eq!(4, mem::size_of::<BVec4>());
        assert_eq!(1, mem::align_of::<BVec4>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            I64Vec4::from(I8Vec4::new(1, 2, 3, 4))
        );
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            I64Vec4::from(U8Vec4::new(1, 2, 3, 4))
        );
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            I64Vec4::from(I16Vec4::new(1, 2, 3, 4))
        );
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            I64Vec4::from(U16Vec4::new(1, 2, 3, 4))
        );
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            I64Vec4::from(IVec4::new(1, 2, 3, 4))
        );
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            I64Vec4::from(UVec4::new(1, 2, 3, 4))
        );

        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            I64Vec4::try_from(U64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(I64Vec4::try_from(U64Vec4::new(u64::MAX, 2, 3, 4)).is_err());
        assert!(I64Vec4::try_from(U64Vec4::new(1, u64::MAX, 3, 4)).is_err());
        assert!(I64Vec4::try_from(U64Vec4::new(1, 2, u64::MAX, 4)).is_err());
        assert!(I64Vec4::try_from(U64Vec4::new(1, 2, 3, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add_unsigned, {
        assert_eq!(
            I64Vec4::new(i64::MAX, i64::MAX, i64::MAX, i64::MAX)
                .wrapping_add_unsigned(U64Vec4::new(1, 1, 1, 1)),
            I64Vec4::new(i64::MIN, i64::MIN, i64::MIN, i64::MIN)
        );
    });

    glam_test!(test_wrapping_sub_unsigned, {
        assert_eq!(
            I64Vec4::new(i64::MIN, i64::MIN, i64::MIN, i64::MIN)
                .wrapping_sub_unsigned(U64Vec4::new(1, 1, 1, 1)),
            I64Vec4::new(i64::MAX, i64::MAX, i64::MAX, i64::MAX)
        );
    });

    glam_test!(test_saturating_add_unsigned, {
        assert_eq!(
            I64Vec4::new(i64::MAX, i64::MAX, i64::MAX, i64::MAX)
                .saturating_add_unsigned(U64Vec4::new(1, 1, 1, 1)),
            I64Vec4::new(i64::MAX, i64::MAX, i64::MAX, i64::MAX)
        );
    });

    glam_test!(test_saturating_sub_unsigned, {
        assert_eq!(
            I64Vec4::new(i64::MIN, i64::MIN, i64::MIN, i64::MIN)
                .saturating_sub_unsigned(U64Vec4::new(1, 1, 1, 1)),
            I64Vec4::new(i64::MIN, i64::MIN, i64::MIN, i64::MIN)
        );
    });

    impl_vec4_signed_integer_tests!(i64, i64vec4, I64Vec4, I64Vec3, I64Vec2, BVec4, bvec4);
    impl_vec4_eq_hash_tests!(i64, i64vec4);

    impl_vec4_scalar_shift_op_tests!(I64Vec4, -2, 2);
    impl_vec4_shift_op_tests!(I64Vec4);

    impl_vec4_scalar_bit_op_tests!(I64Vec4, -2, 2);
    impl_vec4_bit_op_tests!(I64Vec4, -2, 2);
}

mod u64vec4 {
    use glam::{
        bvec4, u64vec4, BVec4, I16Vec4, I64Vec4, I8Vec4, IVec4, U16Vec4, U64Vec2, U64Vec3, U64Vec4,
        U8Vec4, UVec4,
    };

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(32, mem::size_of::<U64Vec4>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(8, mem::align_of::<U64Vec4>());
        #[cfg(feature = "cuda")]
        assert_eq!(16, mem::align_of::<U64Vec4>());
        assert_eq!(4, mem::size_of::<BVec4>());
        assert_eq!(1, mem::align_of::<BVec4>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            U64Vec4::try_from(I8Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U64Vec4::try_from(I8Vec4::new(-1, 2, 3, 4)).is_err());
        assert!(U64Vec4::try_from(I8Vec4::new(1, -2, 3, 4)).is_err());
        assert!(U64Vec4::try_from(I8Vec4::new(1, 2, -3, 4)).is_err());
        assert!(U64Vec4::try_from(I8Vec4::new(1, 2, 3, -4)).is_err());

        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            U64Vec4::from(U8Vec4::new(1, 2, 3, 4))
        );

        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            U64Vec4::try_from(I16Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U64Vec4::try_from(I16Vec4::new(-1, 2, 3, 4)).is_err());
        assert!(U64Vec4::try_from(I16Vec4::new(1, -2, 3, 4)).is_err());
        assert!(U64Vec4::try_from(I16Vec4::new(1, 2, -3, 4)).is_err());
        assert!(U64Vec4::try_from(I16Vec4::new(1, 2, 3, -4)).is_err());

        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            U64Vec4::from(U16Vec4::new(1, 2, 3, 4))
        );

        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            U64Vec4::try_from(IVec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U64Vec4::try_from(IVec4::new(-1, 2, 3, 4)).is_err());
        assert!(U64Vec4::try_from(IVec4::new(1, -2, 3, 4)).is_err());
        assert!(U64Vec4::try_from(IVec4::new(1, 2, -3, 4)).is_err());
        assert!(U64Vec4::try_from(IVec4::new(1, 2, 3, -4)).is_err());

        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            U64Vec4::from(UVec4::new(1, 2, 3, 4))
        );

        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            U64Vec4::try_from(I64Vec4::new(1, 2, 3, 4)).unwrap()
        );
        assert!(U64Vec4::try_from(I64Vec4::new(-1, 2, 3, 4)).is_err());
        assert!(U64Vec4::try_from(I64Vec4::new(1, -2, 3, 4)).is_err());
        assert!(U64Vec4::try_from(I64Vec4::new(1, 2, -3, 4)).is_err());
        assert!(U64Vec4::try_from(I64Vec4::new(1, 2, 3, -4)).is_err());
    });

    glam_test!(test_wrapping_add_signed, {
        assert_eq!(
            U64Vec4::new(u64::MAX, u64::MAX, u64::MAX, u64::MAX)
                .wrapping_add_signed(I64Vec4::new(1, 1, 1, 1)),
            U64Vec4::new(u64::MIN, u64::MIN, u64::MIN, u64::MIN)
        );
    });

    glam_test!(test_saturating_add_signed, {
        assert_eq!(
            U64Vec4::new(u64::MAX, u64::MAX, u64::MAX, u64::MAX)
                .saturating_add_signed(I64Vec4::new(1, 1, 1, 1)),
            U64Vec4::new(u64::MAX, u64::MAX, u64::MAX, u64::MAX)
        );
    });

    impl_vec4_tests!(u64, u64vec4, U64Vec4, U64Vec3, U64Vec2, BVec4, bvec4);
    impl_vec4_eq_hash_tests!(u64, u64vec4);

    impl_vec4_scalar_shift_op_tests!(U64Vec4, 0, 2);
    impl_vec4_shift_op_tests!(U64Vec4);

    impl_vec4_scalar_bit_op_tests!(U64Vec4, 0, 2);
    impl_vec4_bit_op_tests!(U64Vec4, 0, 2);
}
