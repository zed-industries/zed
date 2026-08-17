#![allow(clippy::excessive_precision)]

#[macro_use]
mod support;

macro_rules! impl_vec2_tests {
    ($t:ident, $new:ident, $vec2:ident, $vec3:ident, $mask:ident, $masknew:ident) => {
        glam_test!(test_const, {
            const V0: $vec2 = $vec2::splat(1 as $t);
            const V1: $vec2 = $vec2::new(1 as $t, 2 as $t);
            const V2: $vec2 = $vec2::from_array([1 as $t, 2 as $t]);
            assert_eq!([1 as $t, 1 as $t], *V0.as_ref());
            assert_eq!([1 as $t, 2 as $t], *V1.as_ref());
            assert_eq!([1 as $t, 2 as $t], *V2.as_ref());
        });

        glam_test!(test_vec2_consts, {
            assert_eq!($vec2::ZERO, $new(0 as $t, 0 as $t));
            assert_eq!($vec2::ONE, $new(1 as $t, 1 as $t));
            assert_eq!($vec2::X, $new(1 as $t, 0 as $t));
            assert_eq!($vec2::Y, $new(0 as $t, 1 as $t));
            assert_eq!($vec2::MIN, $new($t::MIN, $t::MIN));
            assert_eq!($vec2::MAX, $new($t::MAX, $t::MAX));
        });

        glam_test!(test_new, {
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

            assert_eq!(a, v.to_array());
            assert_eq!(a, *v.as_ref());

            let mut v2 = $vec2::default();
            *v2.as_mut() = a;
            assert_eq!(a, v2.to_array());

            let v = $vec2::new(t.0, t.1);
            assert_eq!(t, v.into());

            assert_eq!($vec2::new(1 as $t, 0 as $t), BVec2::new(true, false).into());
            assert_eq!($vec2::new(0 as $t, 1 as $t), BVec2::new(false, true).into());

            assert_eq!($vec2::new(1 as $t, 0 as $t), $vec2::X);
            assert_eq!($vec2::new(0 as $t, 1 as $t), $vec2::Y);
        });

        glam_test!(test_fmt, {
            let a = $vec2::new(1 as $t, 2 as $t);
            assert_eq!(
                format!("{:?}", a),
                format!("{}({:?}, {:?})", stringify!($vec2), a.x, a.y)
            );
            assert_eq!(
                format!("{:#?}", a),
                format!(
                    "{}(\n    {:#?},\n    {:#?},\n)",
                    stringify!($vec2),
                    a.x,
                    a.y
                )
            );
            assert_eq!(format!("{}", a), "[1, 2]");
        });

        glam_test!(test_zero, {
            let v = $vec2::ZERO;
            assert_eq!($new(0 as $t, 0 as $t), v);
            assert_eq!(v, $vec2::default());
        });

        glam_test!(test_splat, {
            let v = $vec2::splat(1 as $t);
            assert_eq!($vec2::ONE, v);
        });

        glam_test!(test_map, {
            let v = $vec2::new(1 as $t, 2 as $t);
            assert_eq!(v.map(|n| n + 3 as $t), v + $vec2::splat(3 as $t));
            assert_eq!(v.map(|_| 0 as $t), $vec2::ZERO);
        });

        glam_test!(test_with, {
            assert_eq!($vec2::X, $vec2::ZERO.with_x(1 as $t));
            assert_eq!($vec2::Y, $vec2::ZERO.with_y(1 as $t));
        });

        glam_test!(test_accessors, {
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
        });

        glam_test!(test_dot_unsigned, {
            let x = $new(1 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t);
            assert_eq!(1 as $t, x.dot(x));
            assert_eq!(0 as $t, x.dot(y));

            assert_eq!(
                $new(8 as $t, 8 as $t),
                $new(1 as $t, 2 as $t).dot_into_vec($new(4 as $t, 2 as $t))
            );
        });

        glam_test!(test_length_squared_unsigned, {
            let x = $new(1 as $t, 0 as $t);
            assert_eq!(4 as $t, (2 as $t * x).length_squared());
            assert_eq!(
                2 as $t * 2 as $t + 3 as $t * 3 as $t,
                $new(2 as $t, 3 as $t).length_squared()
            );
        });

        glam_test!(test_ops, {
            let a = $new(2 as $t, 4 as $t);
            assert_eq!($new(4 as $t, 8 as $t), (a + a));
            assert_eq!($new(2 as $t, 4 as $t), 0 as $t + a);
            assert_eq!($new(0 as $t, 0 as $t), (a - a));
            assert_eq!($new(14 as $t, 12 as $t), 16 as $t - a);
            assert_eq!($new(4 as $t, 16 as $t), (a * a));
            assert_eq!($new(4 as $t, 8 as $t), (a * 2 as $t));
            assert_eq!($new(4 as $t, 8 as $t), (2 as $t * a));
            assert_eq!($new(1 as $t, 1 as $t), (a / a));
            assert_eq!($new(1 as $t, 2 as $t), (a / 2 as $t));
            assert_eq!($new(2 as $t, 1 as $t), (4 as $t / a));
            assert_eq!($new(0 as $t, 0 as $t), a % a);
            assert_eq!($new(0 as $t, 1 as $t), a % (a - 1 as $t));
            assert_eq!($new(0 as $t, 0 as $t), a % 1 as $t);
            assert_eq!($new(2 as $t, 1 as $t), a % 3 as $t);
            assert_eq!($new(1 as $t, 1 as $t), 17 as $t % a);
            assert_eq!($new(2 as $t, 4 as $t), a % 8 as $t);
        });

        glam_test!(test_ops_propagated, {
            let vec = $new(2 as $t, 4 as $t);
            let scalar = 2 as $t;
            let g_scalar = 16 as $t;

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
            let a = $new(1 as $t, 2 as $t);
            let mut b = a;

            b += 2 as $t;
            assert_eq!($new(3 as $t, 4 as $t), b);
            b -= 2 as $t;
            assert_eq!($new(1 as $t, 2 as $t), b);
            b *= 2 as $t;
            assert_eq!($new(2 as $t, 4 as $t), b);
            b /= 2 as $t;
            assert_eq!($new(1 as $t, 2 as $t), b);
            b %= 2 as $t;
            assert_eq!($new(1 as $t, 0 as $t), b);

            b = a;
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
            b %= (b + 1 as $t);
            assert_eq!($new(1 as $t, 2 as $t), b);
            b %= b;
            assert_eq!($new(0 as $t, 0 as $t), b);
        });

        glam_test!(test_assign_ops_propagation, {
            let vec = $new(1 as $t, 2 as $t);
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
            let a = $new(0 as $t, 2 as $t);
            let b = $new(1 as $t, 1 as $t);
            assert_eq!($new(0 as $t, 1 as $t), a.min(b));
            assert_eq!($new(0 as $t, 1 as $t), b.min(a));
            assert_eq!($new(1 as $t, 2 as $t), a.max(b));
            assert_eq!($new(1 as $t, 2 as $t), b.max(a));
        });

        glam_test!(test_sum_product, {
            let a = $new(2 as $t, 3 as $t);
            assert_eq!(a.element_sum(), 5 as $t);
            assert_eq!(a.element_product(), 6 as $t);
        });

        glam_test!(test_clamp, {
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

            should_glam_assert!({ $vec2::clamp($vec2::ZERO, $vec2::ONE, $vec2::ZERO) });
        });

        glam_test!(test_hmin_hmax, {
            assert_eq!(1 as $t, $new(1 as $t, 2 as $t).min_element());
            assert_eq!(1 as $t, $new(2 as $t, 1 as $t).min_element());
            assert_eq!(2 as $t, $new(1 as $t, 2 as $t).max_element());
            assert_eq!(2 as $t, $new(2 as $t, 1 as $t).max_element());
        });

        glam_test!(test_eq, {
            let a = $new(1 as $t, 1 as $t);
            let b = $new(1 as $t, 2 as $t);
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
        });

        glam_test!(test_extend_truncate, {
            let a = $new(1 as $t, 2 as $t);
            let b = a.extend(3 as $t);
            assert_eq!($vec3::new(1 as $t, 2 as $t, 3 as $t), b);
        });

        glam_test!(test_vec2mask, {
            // make sure the unused 'z' value doesn't break $vec2 behaviour
            let a = $vec3::ZERO;
            let mut b = a.truncate();
            b.x = 1 as $t;
            b.y = 1 as $t;
            assert!(!b.cmpeq($vec2::ZERO).any());
            assert!(b.cmpeq($vec2::splat(1 as $t)).all());
        });

        glam_test!(test_mask_new, {
            assert_eq!($mask::new(false, false), $masknew(false, false));
            assert_eq!($mask::new(true, false), $masknew(true, false));
            assert_eq!($mask::new(false, true), $masknew(false, true));
            assert_eq!($mask::new(true, true), $masknew(true, true));
        });

        glam_test!(test_mask_from_array_bool, {
            assert_eq!($mask::new(false, false), $mask::from([false, false]));
            assert_eq!($mask::new(true, false), $mask::from([true, false]));
            assert_eq!($mask::new(false, true), $mask::from([false, true]));
            assert_eq!($mask::new(true, true), $mask::from([true, true]));
        });

        glam_test!(test_mask_into_array_u32, {
            assert_eq!(Into::<[u32; 2]>::into($mask::new(false, false)), [0, 0]);
            assert_eq!(Into::<[u32; 2]>::into($mask::new(true, false)), [!0, 0]);
            assert_eq!(Into::<[u32; 2]>::into($mask::new(false, true)), [0, !0]);
            assert_eq!(Into::<[u32; 2]>::into($mask::new(true, true)), [!0, !0]);
        });

        glam_test!(test_mask_into_array_bool, {
            assert_eq!(
                Into::<[bool; 2]>::into($mask::new(false, false)),
                [false, false]
            );
            assert_eq!(
                Into::<[bool; 2]>::into($mask::new(true, false)),
                [true, false]
            );
            assert_eq!(
                Into::<[bool; 2]>::into($mask::new(false, true)),
                [false, true]
            );
            assert_eq!(
                Into::<[bool; 2]>::into($mask::new(true, true)),
                [true, true]
            );
        });

        glam_test!(test_mask_splat, {
            assert_eq!($mask::splat(false), $mask::new(false, false));
            assert_eq!($mask::splat(true), $mask::new(true, true));
        });

        glam_test!(test_mask_bitmask, {
            assert_eq!($mask::new(false, false).bitmask(), 0b00);
            assert_eq!($mask::new(true, false).bitmask(), 0b01);
            assert_eq!($mask::new(false, true).bitmask(), 0b10);
            assert_eq!($mask::new(true, true).bitmask(), 0b11);
        });

        glam_test!(test_mask_any, {
            assert_eq!($mask::new(false, false).any(), false);
            assert_eq!($mask::new(true, false).any(), true);
            assert_eq!($mask::new(false, true).any(), true);
            assert_eq!($mask::new(true, true).any(), true);
        });

        glam_test!(test_mask_all, {
            assert_eq!($mask::new(false, false).all(), false);
            assert_eq!($mask::new(true, false).all(), false);
            assert_eq!($mask::new(false, true).all(), false);
            assert_eq!($mask::new(true, true).all(), true);
        });

        glam_test!(test_mask_select, {
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
        });

        glam_test!(test_mask_and, {
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
        });

        glam_test!(test_mask_or, {
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
        });

        glam_test!(test_mask_xor, {
            assert_eq!(
                ($mask::new(false, false) ^ $mask::new(false, false)).bitmask(),
                0b00,
            );
            assert_eq!(
                ($mask::new(false, false) ^ $mask::new(false, true)).bitmask(),
                0b10,
            );
            assert_eq!(
                ($mask::new(true, false) ^ $mask::new(false, true)).bitmask(),
                0b11,
            );
            assert_eq!(
                ($mask::new(true, true) ^ $mask::new(true, true)).bitmask(),
                0b00,
            );

            let mut mask = $mask::new(false, true);
            mask ^= $mask::new(true, false);
            assert_eq!(mask.bitmask(), 0b11);
        });

        glam_test!(test_mask_not, {
            assert_eq!((!$mask::new(false, false)).bitmask(), 0b11);
            assert_eq!((!$mask::new(true, false)).bitmask(), 0b10);
            assert_eq!((!$mask::new(false, true)).bitmask(), 0b01);
            assert_eq!((!$mask::new(true, true)).bitmask(), 0b00);
        });

        glam_test!(test_mask_fmt, {
            let a = $mask::new(true, false);

            assert_eq!(
                format!("{:?}", a),
                format!("{}(0xffffffff, 0x0)", stringify!($mask))
            );
            assert_eq!(format!("{}", a), "[true, false]");
        });

        glam_test!(test_mask_eq, {
            let a = $mask::new(true, false);
            let b = $mask::new(true, false);
            let c = $mask::new(false, true);

            assert_eq!(a, b);
            assert_eq!(b, a);
            assert_ne!(a, c);
            assert_ne!(b, c);
        });

        glam_test!(test_mask_test, {
            let a = $mask::new(true, false);
            assert_eq!(a.test(0), true);
            assert_eq!(a.test(1), false);

            let b = $mask::new(false, true);
            assert_eq!(b.test(0), false);
            assert_eq!(b.test(1), true);
        });

        glam_test!(test_mask_set, {
            let mut a = $mask::new(false, true);
            a.set(0, true);
            assert_eq!(a.test(0), true);
            a.set(1, false);
            assert_eq!(a.test(1), false);

            let mut b = $mask::new(true, false);
            b.set(0, false);
            assert_eq!(b.test(0), false);
            b.set(1, true);
            assert_eq!(b.test(1), true);
        });

        glam_test!(test_mask_hash, {
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
        });

        glam_test!(test_to_from_slice, {
            let v = $vec2::new(1 as $t, 2 as $t);
            let mut a = [0 as $t, 0 as $t];
            v.write_to_slice(&mut a);
            assert_eq!(v, $vec2::from_slice(&a));

            let mut a = [0 as $t; 17];
            v.write_to_slice(&mut a);
            assert_eq!(v, $vec2::from_slice(&a[..2]));
            assert_eq!([0 as $t; 15], a[2..]);

            should_panic!({ $vec2::ONE.write_to_slice(&mut [0 as $t]) });
            should_panic!({ $vec2::from_slice(&[0 as $t]) });
        });

        glam_test!(test_sum, {
            let one = $vec2::ONE;
            assert_eq!([one, one].iter().sum::<$vec2>(), one + one);
            assert_eq!([one, one].into_iter().sum::<$vec2>(), one + one);
        });

        glam_test!(test_product, {
            let two = $vec2::new(2 as $t, 2 as $t);
            assert_eq!([two, two].iter().product::<$vec2>(), two * two);
            assert_eq!([two, two].into_iter().product::<$vec2>(), two * two);
        });
    };
}

macro_rules! impl_vec2_signed_tests {
    ($t:ident, $new:ident, $vec2:ident, $vec3:ident, $mask:ident, $masknew:ident) => {
        impl_vec2_tests!($t, $new, $vec2, $vec3, $mask, $masknew);

        glam_test!(test_is_negative_bitmask, {
            assert_eq!($vec2::ZERO.is_negative_bitmask(), 0b00);
            assert_eq!($vec2::ONE.is_negative_bitmask(), 0b00);
            assert_eq!((-$vec2::ONE).is_negative_bitmask(), 0b11);
            assert_eq!($vec2::new(-1 as $t, 2 as $t).is_negative_bitmask(), 0b01);
            assert_eq!($vec2::new(8 as $t, 3 as $t).is_negative_bitmask(), 0b00);
            assert_eq!($vec2::new(3 as $t, -4 as $t).is_negative_bitmask(), 0b10);
            assert_eq!($vec2::new(-2 as $t, -6 as $t).is_negative_bitmask(), 0b11);
        });

        glam_test!(test_abs, {
            assert_eq!($vec2::ZERO.abs(), $vec2::ZERO);
            assert_eq!($vec2::ONE.abs(), $vec2::ONE);
            assert_eq!((-$vec2::ONE).abs(), $vec2::ONE);
        });

        glam_test!(test_dot_signed, {
            let x = $new(1 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t);
            assert_eq!(1 as $t, x.dot(x));
            assert_eq!(0 as $t, x.dot(y));
            assert_eq!(-1 as $t, x.dot(-x));
        });

        glam_test!(test_length_squared_signed, {
            let x = $new(1 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t);
            assert_eq!(9 as $t, (-3 as $t * y).length_squared());
            assert_eq!(2 as $t, x.distance_squared(y));
            assert_eq!(13 as $t, (2 as $t * x).distance_squared(-3 as $t * y));
        });

        glam_test!(test_neg, {
            let a = $new(1 as $t, 2 as $t);
            assert_eq!($new(-1 as $t, -2 as $t), (-a));
            assert_eq!($new(-0.0 as $t, -0.0 as $t), -$new(0.0 as $t, 0.0 as $t));
            assert_eq!($new(0.0 as $t, -0.0 as $t), -$new(-0.0 as $t, 0.0 as $t));
        });

        glam_test!(test_neg_propagation, {
            let a = $new(1 as $t, 2 as $t);
            assert_eq!(-a, -(&a));
        });

        glam_test!(test_perp, {
            let v1 = $vec2::new(1 as $t, 2 as $t);
            let v2 = $vec2::new(1 as $t, 1 as $t);
            let v1_perp = $vec2::new(-2 as $t, 1 as $t);

            assert_eq!(v1_perp, v1.perp());
            assert_eq!(v1.perp().dot(v1), 0 as $t);
            assert_eq!(v2.perp().dot(v2), 0 as $t);
            assert_eq!(v1.perp().dot(v2), v1.perp_dot(v2));
        });

        glam_test!(test_rotate, {
            assert_eq!(
                $vec2::new(0 as $t, 1 as $t).rotate($vec2::new(1 as $t, 1 as $t)),
                $vec2::new(-1 as $t, 1 as $t)
            );
        });

        glam_test!(test_div_euclid, {
            let one = $vec2::ONE;
            let two = one + one;
            let three = two + one;
            assert_eq!(three.div_euclid(two), one);
            assert_eq!((-three).div_euclid(two), -two);
            assert_eq!(three.div_euclid(-two), -one);
            assert_eq!((-three).div_euclid(-two), two);
        });

        glam_test!(test_rem_euclid, {
            let one = $vec2::ONE;
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

macro_rules! impl_vec2_signed_integer_tests {
    ($t:ident, $new:ident, $vec2:ident, $vec3:ident, $mask:ident, $masknew:ident) => {
        impl_vec2_signed_tests!($t, $new, $vec2, $vec3, $mask, $masknew);

        glam_test!(test_signum, {
            assert_eq!($vec3::ZERO.signum(), $vec3::ZERO);
            assert_eq!($vec3::ONE.signum(), $vec3::ONE);
            assert_eq!((-$vec3::ONE).signum(), -$vec3::ONE);
        });
    };
}

macro_rules! impl_vec2_eq_hash_tests {
    ($t:ident, $new:ident) => {
        glam_test!(test_ve2_hash, {
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
        });
    };
}

macro_rules! impl_vec2_float_tests {
    ($t:ident, $new:ident, $vec2:ident, $vec3:ident, $mask:ident, $masknew:ident) => {
        impl_vec2_signed_tests!($t, $new, $vec2, $vec3, $mask, $masknew);
        impl_vec_float_normalize_tests!($t, $vec2);

        glam_test!(test_vec2_nan, {
            assert!($vec2::NAN.is_nan());
            assert!(!$vec2::NAN.is_finite());
        });

        glam_test!(test_length, {
            let x = $new(1.0, 0.0);
            let y = $new(0.0, 1.0);
            assert_eq!(2.0, (-2.0 * x).length());
            assert_eq!(3.0, (3.0 * y).length());
            assert_eq!((2.0 as $t).sqrt(), x.distance(y));
            assert_eq!(5.0, (3.0 * x).distance(-4.0 * y));
            assert_eq!(13.0, (-5.0 * x).distance(12.0 * y));
            assert_eq!(x, (2.0 * x).normalize());
            assert_eq!(1.0 * 3.0 + 2.0 * 4.0, $new(1.0, 2.0).dot($new(3.0, 4.0)));
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
        });

        glam_test!(test_project_reject, {
            assert_eq!($new(0.0, 1.0), $new(1.0, 1.0).project_onto($new(0.0, 2.0)));
            assert_eq!($new(1.0, 0.0), $new(1.0, 1.0).reject_from($new(0.0, 2.0)));
            assert_eq!(
                $new(0.0, 1.0),
                $new(1.0, 1.0).project_onto_normalized($new(0.0, 1.0))
            );
            assert_eq!(
                $new(1.0, 0.0),
                $new(1.0, 1.0).reject_from_normalized($new(0.0, 1.0))
            );
            should_glam_assert!({ $vec2::ONE.project_onto($vec2::ZERO) });
            should_glam_assert!({ $vec2::ONE.reject_from($vec2::ZERO) });
            should_glam_assert!({ $vec2::ONE.project_onto_normalized($vec2::ONE) });
            should_glam_assert!({ $vec2::ONE.reject_from_normalized($vec2::ONE) });
        });

        glam_test!(test_signum, {
            assert_eq!($vec2::ZERO.signum(), $vec2::ONE);
            assert_eq!((-$vec2::ZERO).signum(), -$vec2::ONE);
            assert_eq!($vec2::ONE.signum(), $vec2::ONE);
            assert_eq!((-$vec2::ONE).signum(), -$vec2::ONE);
            assert_eq!($vec2::INFINITY.signum(), $vec2::ONE);
            assert_eq!($vec2::NEG_INFINITY.signum(), -$vec2::ONE);
            assert!($vec2::NAN.signum().is_nan_mask().all());
        });

        glam_test!(test_copysign, {
            assert_eq!($vec2::ZERO.copysign(-$vec2::ZERO), -$vec2::ZERO);
            assert_eq!((-$vec2::ZERO).copysign(-$vec2::ZERO), -$vec2::ZERO);
            assert_eq!($vec2::ZERO.copysign($vec2::ZERO), $vec2::ZERO);
            assert_eq!((-$vec2::ZERO).copysign($vec2::ZERO), $vec2::ZERO);
            assert_eq!($vec2::ONE.copysign(-$vec2::ZERO), -$vec2::ONE);
            assert_eq!((-$vec2::ONE).copysign(-$vec2::ZERO), -$vec2::ONE);
            assert_eq!($vec2::ONE.copysign($vec2::ZERO), $vec2::ONE);
            assert_eq!((-$vec2::ONE).copysign($vec2::ZERO), $vec2::ONE);
            assert_eq!($vec2::ZERO.copysign(-$vec2::ONE), -$vec2::ZERO);
            assert_eq!((-$vec2::ZERO).copysign(-$vec2::ONE), -$vec2::ZERO);
            assert_eq!($vec2::ZERO.copysign($vec2::ONE), $vec2::ZERO);
            assert_eq!((-$vec2::ZERO).copysign($vec2::ONE), $vec2::ZERO);
            assert_eq!($vec2::ONE.copysign(-$vec2::ONE), -$vec2::ONE);
            assert_eq!((-$vec2::ONE).copysign(-$vec2::ONE), -$vec2::ONE);
            assert_eq!($vec2::ONE.copysign($vec2::ONE), $vec2::ONE);
            assert_eq!((-$vec2::ONE).copysign($vec2::ONE), $vec2::ONE);
            assert_eq!($vec2::INFINITY.copysign($vec2::ONE), $vec2::INFINITY);
            assert_eq!($vec2::INFINITY.copysign(-$vec2::ONE), $vec2::NEG_INFINITY);
            assert_eq!($vec2::NEG_INFINITY.copysign($vec2::ONE), $vec2::INFINITY);
            assert_eq!(
                $vec2::NEG_INFINITY.copysign(-$vec2::ONE),
                $vec2::NEG_INFINITY
            );
            assert!($vec2::NAN.copysign($vec2::ONE).is_nan_mask().all());
            assert!($vec2::NAN.copysign(-$vec2::ONE).is_nan_mask().all());
        });

        glam_test!(test_float_is_negative_bitmask, {
            assert_eq!($vec2::ZERO.is_negative_bitmask(), 0b00);
            assert_eq!((-$vec2::ZERO).is_negative_bitmask(), 0b11);
            assert_eq!($vec2::ONE.is_negative_bitmask(), 0b00);
            assert_eq!((-$vec2::ONE).is_negative_bitmask(), 0b11);
            assert_eq!($vec2::new(-1.0, 2.0).is_negative_bitmask(), 0b01);
            assert_eq!($vec2::new(8.0, 3.0).is_negative_bitmask(), 0b00);
            assert_eq!($vec2::new(3.0, -4.0).is_negative_bitmask(), 0b10);
            assert_eq!($vec2::new(-2.0, -6.0).is_negative_bitmask(), 0b11);
        });

        glam_test!(test_round, {
            assert_eq!($vec2::new(1.35, 0.0).round().x, 1.0);
            assert_eq!($vec2::new(0.0, 1.5).round().y, 2.0);
            assert_eq!($vec2::new(0.0, -15.5).round().y, -16.0);
            assert_eq!($vec2::new(0.0, 0.0).round().y, 0.0);
            assert_eq!($vec2::new(0.0, 21.1).round().y, 21.0);
            assert_eq!($vec2::new(0.0, 11.123).round().y, 11.0);
            assert_eq!($vec2::new(0.0, 11.499).round().y, 11.0);
            assert_eq!(
                $vec2::new($t::NEG_INFINITY, $t::INFINITY).round(),
                $vec2::new($t::NEG_INFINITY, $t::INFINITY)
            );
            assert!($vec2::new($t::NAN, 0.0).round().x.is_nan());
        });

        glam_test!(test_floor, {
            assert_eq!($vec2::new(1.35, -1.5).floor(), $vec2::new(1.0, -2.0));
            assert_eq!(
                $vec2::new($t::INFINITY, $t::NEG_INFINITY).floor(),
                $vec2::new($t::INFINITY, $t::NEG_INFINITY)
            );
            assert!($vec2::new($t::NAN, 0.0).floor().x.is_nan());
            assert_eq!(
                $vec2::new(-2000000.123, 10000000.123).floor(),
                $vec2::new(-2000001.0, 10000000.0)
            );
        });

        glam_test!(test_fract_gl, {
            assert_approx_eq!($vec2::new(1.35, -1.5).fract_gl(), $vec2::new(0.35, 0.5));
            assert_approx_eq!(
                $vec2::new(-2000000.123, 1000000.123).fract_gl(),
                $vec2::new(0.877, 0.123),
                0.002
            );
        });

        glam_test!(test_fract, {
            assert_approx_eq!($vec2::new(1.35, -1.5).fract(), $vec2::new(0.35, -0.5));
            assert_approx_eq!(
                $vec2::new(-2000000.123, 1000000.123).fract(),
                $vec2::new(-0.123, 0.123),
                0.002
            );
        });

        glam_test!(test_ceil, {
            assert_eq!($vec2::new(1.35, -1.5).ceil(), $vec2::new(2.0, -1.0));
            assert_eq!(
                $vec2::new($t::INFINITY, $t::NEG_INFINITY).ceil(),
                $vec2::new($t::INFINITY, $t::NEG_INFINITY)
            );
            assert!($vec2::new($t::NAN, 0.0).ceil().x.is_nan());
            assert_eq!(
                $vec2::new(-2000000.123, 1000000.123).ceil(),
                $vec2::new(-2000000.0, 1000001.0)
            );
        });

        glam_test!(test_trunc, {
            assert_eq!($vec2::new(1.35, -1.5).trunc(), $vec2::new(1.0, -1.0));
            assert_eq!(
                $vec2::new($t::INFINITY, $t::NEG_INFINITY).trunc(),
                $vec2::new($t::INFINITY, $t::NEG_INFINITY)
            );
            assert!($vec2::new(0.0, $t::NAN).trunc().y.is_nan());
            assert_eq!(
                $vec2::new(-0.0, -2000000.123).trunc(),
                $vec2::new(-0.0, -2000000.0)
            );
        });

        glam_test!(test_lerp, {
            let v0 = $vec2::new(-1.0, -1.0);
            let v1 = $vec2::new(1.0, 1.0);
            assert_approx_eq!(v0, v0.lerp(v1, 0.0));
            assert_approx_eq!(v1, v0.lerp(v1, 1.0));
            assert_approx_eq!($vec2::ZERO, v0.lerp(v1, 0.5));
        });

        glam_test!(test_lerp_big_difference, {
            let v0 = $vec2::new(-1e30, -1e30);
            let v1 = $vec2::new(16.0, 16.0);
            assert_approx_eq!(v0, v0.lerp(v1, 0.0));
            assert_approx_eq!(v1, v0.lerp(v1, 1.0));
        });

        glam_test!(test_move_towards, {
            let v0 = $vec2::new(-1.0, -1.0);
            let v1 = $vec2::new(1.0, 1.0);
            assert_approx_eq!(v0, v0.move_towards(v1, 0.0));
            assert_approx_eq!(v1, v0.move_towards(v1, v0.distance(v1)));
            assert_approx_eq!(v1, v0.move_towards(v1, v0.distance(v1) + 1.0));
        });

        glam_test!(test_rotate_towards, {
            use core::$t::consts::{FRAC_PI_2, FRAC_PI_4};
            let eps = 10.0 * $t::EPSILON as f32;

            // Setup such that `v0` is `PI/2` and `-PI/2` radians away from `v1` and `v2` respectively.
            let v0 = $vec2::new(1.0, 0.0);
            let v1 = $vec2::new(0.0, -1.0);
            let v2 = $vec2::new(0.0, 1.0);

            // Positive delta
            assert_approx_eq!(v0, v0.rotate_towards(v1, 0.0), eps);
            assert_approx_eq!(
                $vec2::new($t::sqrt(2.0) / 2.0, -$t::sqrt(2.0) / 2.0),
                v0.rotate_towards(v1, FRAC_PI_4),
                eps
            );
            assert_approx_eq!(v1, v0.rotate_towards(v1, FRAC_PI_2), eps);
            assert_approx_eq!(v1, v0.rotate_towards(v1, FRAC_PI_2 * 1.5), eps);

            // Negative delta
            assert_approx_eq!(
                $vec2::new($t::sqrt(2.0) / 2.0, $t::sqrt(2.0) / 2.0),
                v0.rotate_towards(v1, -FRAC_PI_4),
                eps
            );
            assert_approx_eq!(v2, v0.rotate_towards(v1, -FRAC_PI_2), eps);
            assert_approx_eq!(v2, v0.rotate_towards(v1, -FRAC_PI_2 * 1.5), eps);
        });

        glam_test!(test_midpoint, {
            let v0 = $vec2::new(-1.0, -1.0);
            let v1 = $vec2::new(1.0, 1.0);
            let v2 = $vec2::new(-1.5, 0.0);
            assert_approx_eq!($vec2::ZERO, v0.midpoint(v1));
            assert_approx_eq!($vec2::new(-0.25, 0.5), v1.midpoint(v2));
        });

        glam_test!(test_is_finite, {
            assert!($vec2::new(0.0, 0.0).is_finite());
            assert!($vec2::new(-1e-10, 1e10).is_finite());
            assert!(!$vec2::new($t::INFINITY, 0.0).is_finite());
            assert!(!$vec2::new(0.0, $t::NAN).is_finite());
            assert!(!$vec2::new(0.0, $t::NEG_INFINITY).is_finite());
            assert!(!$vec2::new($t::INFINITY, $t::NEG_INFINITY).is_finite());
            assert!(!$vec2::INFINITY.is_finite());
            assert!(!$vec2::NEG_INFINITY.is_finite());
        });

        glam_test!(test_powf, {
            assert_eq!($vec2::new(2.0, 4.0).powf(2.0), $vec2::new(4.0, 16.0));
        });

        glam_test!(test_exp, {
            assert_approx_eq!(
                $vec2::new(1.0, 2.0).exp(),
                $vec2::new((1.0 as $t).exp(), (2.0 as $t).exp())
            );
        });

        glam_test!(test_angle_to, {
            let angle = $vec2::new(1.0, 0.0).angle_to($vec2::new(0.0, 1.0));
            assert_approx_eq!(core::$t::consts::FRAC_PI_2, angle, 1e-6);

            let angle = $vec2::new(10.0, 0.0).angle_to($vec2::new(0.0, 5.0));
            assert_approx_eq!(core::$t::consts::FRAC_PI_2, angle, 1e-6);

            let angle = $vec2::new(-1.0, 0.0).angle_to($vec2::new(0.0, 1.0));
            assert_approx_eq!(-core::$t::consts::FRAC_PI_2, angle, 1e-6);

            // the angle returned by angle_to should rotate the input vector to the
            // destination vector
            assert_approx_eq!(
                $vec2::from_angle($vec2::X.angle_to($vec2::Y)).rotate($vec2::X),
                $vec2::Y
            );
        });

        glam_test!(test_clamp_length, {
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
            should_glam_assert!({ $vec2::ONE.clamp_length(1.0, 0.0) });
        });

        glam_test!(test_clamp_length_max, {
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
        });

        glam_test!(test_clamp_length_min, {
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
        });

        #[cfg(any(feature = "glam-assert", feature = "debug-glam-assert"))]
        glam_test!(test_float_glam_assert, {
            use std::panic::catch_unwind;

            assert!(catch_unwind(|| $vec2::ZERO.normalize()).is_err());
        });

        glam_test!(test_mul_add, {
            assert_eq!(
                $vec2::new(1.0, 1.0).mul_add($vec2::new(0.5, 2.0), $vec2::new(-1.0, -1.0)),
                $vec2::new(-0.5, 1.0)
            );
        });

        glam_test!(test_fmt_float, {
            let a = $vec2::new(1.0, 2.0);
            assert_eq!(format!("{:.2}", a), "[1.00, 2.00]");
        });

        glam_test!(test_angle_conversion, {
            let angle = 0.;
            let vec = $vec2::from_angle(angle);
            assert_approx_eq!(vec, $vec2::new(1.0, 0.0));
            assert_approx_eq!(vec.to_angle(), angle);

            let angle = core::$t::consts::FRAC_PI_2;
            let vec = $vec2::from_angle(angle);
            assert_approx_eq!(vec, $vec2::new(0.0, 1.0));
            assert_approx_eq!(vec.to_angle(), angle);

            let angle = core::$t::consts::PI;
            let vec = $vec2::from_angle(angle);
            assert_approx_eq!(vec, $vec2::new(-1.0, 0.0));
            // The sign of the angle PI gets flipped and is slightly less precise but correct
            assert_approx_eq!(vec.to_angle().abs(), angle, 1e-6);

            let angle = -core::$t::consts::FRAC_PI_2;
            let vec = $vec2::from_angle(angle);
            assert_approx_eq!(vec, $vec2::new(0.0, -1.0));
            assert_approx_eq!(vec.to_angle(), angle);
        });

        glam_test!(test_reflect, {
            let incident = $vec2::new(1.0, -1.0);
            let normal = $vec2::Y;
            assert_approx_eq!(incident.reflect(normal), $vec2::ONE);
        });

        glam_test!(test_refract, {
            let incident = $vec2::NEG_ONE.normalize();
            let normal = $vec2::ONE.normalize();
            assert_approx_eq!(incident.refract(normal, 0.5), incident);

            let incident = $vec2::new(1.0, -1.0).normalize();
            let normal = $vec2::Y;
            assert_approx_eq!(incident.refract(normal, 1.5), $vec2::ZERO);
        });
    };
}

macro_rules! impl_vec2_scalar_shift_op_test {
    ($vec2:ident, $t_min:literal, $t_max:literal, $rhs_min:literal, $rhs_max:literal) => {
        glam_test!(test_vec2_scalar_shift_ops, {
            for x in $t_min..$t_max {
                for y in $t_min..$t_max {
                    for rhs in $rhs_min..$rhs_max {
                        assert_eq!($vec2::new(x, y) << rhs, $vec2::new(x << rhs, y << rhs));
                        assert_eq!($vec2::new(x, y) >> rhs, $vec2::new(x >> rhs, y >> rhs));
                    }
                }
            }
        });
    };
}

macro_rules! impl_vec2_scalar_shift_op_tests {
    ($vec2:ident, $t_min:literal, $t_max:literal) => {
        mod shift_by_i8 {
            use glam::$vec2;
            impl_vec2_scalar_shift_op_test!($vec2, $t_min, $t_max, 0i8, 2);
        }
        mod shift_by_i16 {
            use glam::$vec2;
            impl_vec2_scalar_shift_op_test!($vec2, $t_min, $t_max, 0i16, 2);
        }
        mod shift_by_i32 {
            use glam::$vec2;
            impl_vec2_scalar_shift_op_test!($vec2, $t_min, $t_max, 0i32, 2);
        }
        mod shift_by_i64 {
            use glam::$vec2;
            impl_vec2_scalar_shift_op_test!($vec2, $t_min, $t_max, 0i64, 2);
        }
        mod shift_by_u8 {
            use glam::$vec2;
            impl_vec2_scalar_shift_op_test!($vec2, $t_min, $t_max, 0u8, 2);
        }
        mod shift_by_u16 {
            use glam::$vec2;
            impl_vec2_scalar_shift_op_test!($vec2, $t_min, $t_max, 0u16, 2);
        }
        mod shift_by_u32 {
            use glam::$vec2;
            impl_vec2_scalar_shift_op_test!($vec2, $t_min, $t_max, 0u32, 2);
        }
        mod shift_by_u64 {
            use glam::$vec2;
            impl_vec2_scalar_shift_op_test!($vec2, $t_min, $t_max, 0u64, 2);
        }
    };
}

macro_rules! impl_vec2_shift_op_test {
    ($vec2:ident, $rhs:ident, $t_min:literal, $t_max:literal) => {
        glam_test!(test_vec2_shift_ops, {
            for x1 in $t_min..$t_max {
                for y1 in $t_min..$t_max {
                    for x2 in $t_min..$t_max {
                        for y2 in $t_min..$t_max {
                            assert_eq!(
                                $vec2::new(x1, y1) << $rhs::new(x2, y2),
                                $vec2::new(x1 << x2, y1 << y2)
                            );
                            assert_eq!(
                                $vec2::new(x1, y1) >> $rhs::new(x2, y2),
                                $vec2::new(x1 >> x2, y1 >> y2)
                            );
                        }
                    }
                }
            }
        });
    };
}

macro_rules! impl_vec2_shift_op_tests {
    ($vec2:ident) => {
        mod shift_ivec2_by_ivec2 {
            use super::*;
            impl_vec2_shift_op_test!($vec2, IVec2, 0, 2);
        }
        mod shift_ivec2_by_uvec2 {
            use super::*;
            impl_vec2_shift_op_test!($vec2, UVec2, 0, 2);
        }
    };
}

macro_rules! impl_vec2_scalar_bit_op_tests {
    ($vec2:ident, $t_min:literal, $t_max:literal) => {
        glam_test!(test_vec2_scalar_bit_ops, {
            for x in $t_min..$t_max {
                for y in $t_min..$t_max {
                    for rhs in $t_min..$t_max {
                        assert_eq!($vec2::new(x, y) & rhs, $vec2::new(x & rhs, y & rhs));
                        assert_eq!($vec2::new(x, y) | rhs, $vec2::new(x | rhs, y | rhs));
                        assert_eq!($vec2::new(x, y) ^ rhs, $vec2::new(x ^ rhs, y ^ rhs));
                    }
                }
            }
        });
    };
}

macro_rules! impl_vec2_bit_op_tests {
    ($vec2:ident, $t_min:literal, $t_max:literal) => {
        glam_test!(test_vec2_bit_ops, {
            for x1 in $t_min..$t_max {
                for y1 in $t_min..$t_max {
                    assert_eq!(!$vec2::new(x1, y1), $vec2::new(!x1, !y1));

                    for x2 in $t_min..$t_max {
                        for y2 in $t_min..$t_max {
                            assert_eq!(
                                $vec2::new(x1, y1) & $vec2::new(x2, y2),
                                $vec2::new(x1 & x2, y1 & y2)
                            );
                            assert_eq!(
                                $vec2::new(x1, y1) | $vec2::new(x2, y2),
                                $vec2::new(x1 | x2, y1 | y2)
                            );
                            assert_eq!(
                                $vec2::new(x1, y1) ^ $vec2::new(x2, y2),
                                $vec2::new(x1 ^ x2, y1 ^ y2)
                            );
                        }
                    }
                }
            }
        });
    };
}

mod vec2 {
    use glam::{bvec2, vec2, BVec2, Vec2, Vec3};

    glam_test!(test_align, {
        use core::mem;
        assert_eq!(8, mem::size_of::<Vec2>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(4, mem::align_of::<Vec2>());
        #[cfg(feature = "cuda")]
        assert_eq!(8, mem::align_of::<Vec2>());
        assert_eq!(2, mem::size_of::<BVec2>());
        assert_eq!(1, mem::align_of::<BVec2>());
    });

    glam_test!(test_as, {
        use glam::{DVec2, I16Vec2, I64Vec2, I8Vec2, IVec2, U16Vec2, U64Vec2, U8Vec2, UVec2};
        assert_eq!(DVec2::new(-1.0, -2.0), Vec2::new(-1.0, -2.0).as_dvec2());
        assert_eq!(I8Vec2::new(-1, -2), Vec2::new(-1.0, -2.0).as_i8vec2());
        assert_eq!(U8Vec2::new(1, 2), Vec2::new(1.0, 2.0).as_u8vec2());
        assert_eq!(I16Vec2::new(-1, -2), Vec2::new(-1.0, -2.0).as_i16vec2());
        assert_eq!(U16Vec2::new(1, 2), Vec2::new(1.0, 2.0).as_u16vec2());
        assert_eq!(IVec2::new(-1, -2), Vec2::new(-1.0, -2.0).as_ivec2());
        assert_eq!(UVec2::new(1, 2), Vec2::new(1.0, 2.0).as_uvec2());
        assert_eq!(I64Vec2::new(-1, -2), Vec2::new(-1.0, -2.0).as_i64vec2());
        assert_eq!(U64Vec2::new(1, 2), Vec2::new(1.0, 2.0).as_u64vec2());

        assert_eq!(Vec2::new(-1.0, -2.0), DVec2::new(-1.0, -2.0).as_vec2());
        assert_eq!(I8Vec2::new(-1, -2), DVec2::new(-1.0, -2.0).as_i8vec2());
        assert_eq!(U8Vec2::new(1, 2), DVec2::new(1.0, 2.0).as_u8vec2());
        assert_eq!(I16Vec2::new(-1, -2), DVec2::new(-1.0, -2.0).as_i16vec2());
        assert_eq!(U16Vec2::new(1, 2), DVec2::new(1.0, 2.0).as_u16vec2());
        assert_eq!(IVec2::new(-1, -2), DVec2::new(-1.0, -2.0).as_ivec2());
        assert_eq!(UVec2::new(1, 2), DVec2::new(1.0, 2.0).as_uvec2());
        assert_eq!(I64Vec2::new(-1, -2), DVec2::new(-1.0, -2.0).as_i64vec2());
        assert_eq!(U64Vec2::new(1, 2), DVec2::new(1.0, 2.0).as_u64vec2());

        assert_eq!(Vec2::new(-1.0, -2.0), I8Vec2::new(-1, -2).as_vec2());
        assert_eq!(DVec2::new(-1.0, -2.0), I8Vec2::new(-1, -2).as_dvec2());
        assert_eq!(U8Vec2::new(1, 2), I8Vec2::new(1, 2).as_u8vec2());
        assert_eq!(I16Vec2::new(-1, -2), I8Vec2::new(-1, -2).as_i16vec2());
        assert_eq!(U16Vec2::new(1, 2), I8Vec2::new(1, 2).as_u16vec2());
        assert_eq!(IVec2::new(-1, -2), I8Vec2::new(-1, -2).as_ivec2());
        assert_eq!(UVec2::new(1, 2), I8Vec2::new(1, 2).as_uvec2());
        assert_eq!(I64Vec2::new(-1, -2), I8Vec2::new(-1, -2).as_i64vec2());
        assert_eq!(U64Vec2::new(1, 2), I8Vec2::new(1, 2).as_u64vec2());

        assert_eq!(Vec2::new(1.0, 2.0), U8Vec2::new(1, 2).as_vec2());
        assert_eq!(DVec2::new(1.0, 2.0), U8Vec2::new(1, 2).as_dvec2());
        assert_eq!(I8Vec2::new(1, 2), U8Vec2::new(1, 2).as_i8vec2());
        assert_eq!(I16Vec2::new(1, 2), U8Vec2::new(1, 2).as_i16vec2());
        assert_eq!(U16Vec2::new(1, 2), U8Vec2::new(1, 2).as_u16vec2());
        assert_eq!(IVec2::new(1, 2), U8Vec2::new(1, 2).as_ivec2());
        assert_eq!(UVec2::new(1, 2), U8Vec2::new(1, 2).as_uvec2());
        assert_eq!(I64Vec2::new(1, 2), U8Vec2::new(1, 2).as_i64vec2());
        assert_eq!(U64Vec2::new(1, 2), U8Vec2::new(1, 2).as_u64vec2());

        assert_eq!(Vec2::new(-1.0, -2.0), I16Vec2::new(-1, -2).as_vec2());
        assert_eq!(DVec2::new(-1.0, -2.0), I16Vec2::new(-1, -2).as_dvec2());
        assert_eq!(I8Vec2::new(-1, -2), I16Vec2::new(-1, -2).as_i8vec2());
        assert_eq!(U8Vec2::new(1, 2), I16Vec2::new(1, 2).as_u8vec2());
        assert_eq!(U16Vec2::new(1, 2), I16Vec2::new(1, 2).as_u16vec2());
        assert_eq!(IVec2::new(-1, -2), I16Vec2::new(-1, -2).as_ivec2());
        assert_eq!(UVec2::new(1, 2), I16Vec2::new(1, 2).as_uvec2());
        assert_eq!(I64Vec2::new(-1, -2), I16Vec2::new(-1, -2).as_i64vec2());
        assert_eq!(U64Vec2::new(1, 2), I16Vec2::new(1, 2).as_u64vec2());

        assert_eq!(Vec2::new(1.0, 2.0), U16Vec2::new(1, 2).as_vec2());
        assert_eq!(DVec2::new(1.0, 2.0), U16Vec2::new(1, 2).as_dvec2());
        assert_eq!(I8Vec2::new(1, 2), U16Vec2::new(1, 2).as_i8vec2());
        assert_eq!(U8Vec2::new(1, 2), U16Vec2::new(1, 2).as_u8vec2());
        assert_eq!(I16Vec2::new(1, 2), U16Vec2::new(1, 2).as_i16vec2());
        assert_eq!(IVec2::new(1, 2), U16Vec2::new(1, 2).as_ivec2());
        assert_eq!(UVec2::new(1, 2), U16Vec2::new(1, 2).as_uvec2());
        assert_eq!(I64Vec2::new(1, 2), U16Vec2::new(1, 2).as_i64vec2());
        assert_eq!(U64Vec2::new(1, 2), U16Vec2::new(1, 2).as_u64vec2());

        assert_eq!(Vec2::new(-1.0, -2.0), IVec2::new(-1, -2).as_vec2());
        assert_eq!(DVec2::new(-1.0, -2.0), IVec2::new(-1, -2).as_dvec2());
        assert_eq!(UVec2::new(1, 2), IVec2::new(1, 2).as_uvec2());
        assert_eq!(I8Vec2::new(-1, -2), IVec2::new(-1, -2).as_i8vec2());
        assert_eq!(U8Vec2::new(1, 2), IVec2::new(1, 2).as_u8vec2());
        assert_eq!(I16Vec2::new(-1, -2), IVec2::new(-1, -2).as_i16vec2());
        assert_eq!(U16Vec2::new(1, 2), IVec2::new(1, 2).as_u16vec2());
        assert_eq!(I64Vec2::new(-1, -2), IVec2::new(-1, -2).as_i64vec2());
        assert_eq!(U64Vec2::new(1, 2), IVec2::new(1, 2).as_u64vec2());

        assert_eq!(Vec2::new(1.0, 2.0), UVec2::new(1, 2).as_vec2());
        assert_eq!(DVec2::new(1.0, 2.0), UVec2::new(1, 2).as_dvec2());
        assert_eq!(I8Vec2::new(1, 2), UVec2::new(1, 2).as_i8vec2());
        assert_eq!(U8Vec2::new(1, 2), UVec2::new(1, 2).as_u8vec2());
        assert_eq!(I16Vec2::new(1, 2), UVec2::new(1, 2).as_i16vec2());
        assert_eq!(U16Vec2::new(1, 2), UVec2::new(1, 2).as_u16vec2());
        assert_eq!(IVec2::new(1, 2), UVec2::new(1, 2).as_ivec2());
        assert_eq!(I64Vec2::new(1, 2), UVec2::new(1, 2).as_i64vec2());
        assert_eq!(U64Vec2::new(1, 2), UVec2::new(1, 2).as_u64vec2());

        assert_eq!(Vec2::new(-1.0, -2.0), I64Vec2::new(-1, -2).as_vec2());
        assert_eq!(DVec2::new(-1.0, -2.0), I64Vec2::new(-1, -2).as_dvec2());
        assert_eq!(U8Vec2::new(1, 2), I64Vec2::new(1, 2).as_u8vec2());
        assert_eq!(I8Vec2::new(-1, -2), I64Vec2::new(-1, -2).as_i8vec2());
        assert_eq!(U16Vec2::new(1, 2), I64Vec2::new(1, 2).as_u16vec2());
        assert_eq!(I16Vec2::new(-1, -2), I64Vec2::new(-1, -2).as_i16vec2());
        assert_eq!(UVec2::new(1, 2), I64Vec2::new(1, 2).as_uvec2());
        assert_eq!(IVec2::new(-1, -2), I64Vec2::new(-1, -2).as_ivec2());
        assert_eq!(U64Vec2::new(1, 2), I64Vec2::new(1, 2).as_u64vec2());

        assert_eq!(Vec2::new(1.0, 2.0), U64Vec2::new(1, 2).as_vec2());
        assert_eq!(DVec2::new(1.0, 2.0), U64Vec2::new(1, 2).as_dvec2());
        assert_eq!(I8Vec2::new(1, 2), U64Vec2::new(1, 2).as_i8vec2());
        assert_eq!(U8Vec2::new(1, 2), U64Vec2::new(1, 2).as_u8vec2());
        assert_eq!(I16Vec2::new(1, 2), U64Vec2::new(1, 2).as_i16vec2());
        assert_eq!(U16Vec2::new(1, 2), U64Vec2::new(1, 2).as_u16vec2());
        assert_eq!(IVec2::new(1, 2), U64Vec2::new(1, 2).as_ivec2());
        assert_eq!(UVec2::new(1, 2), U64Vec2::new(1, 2).as_uvec2());
        assert_eq!(I64Vec2::new(1, 2), U64Vec2::new(1, 2).as_i64vec2());
    });

    impl_vec2_float_tests!(f32, vec2, Vec2, Vec3, BVec2, bvec2);
}

mod dvec2 {
    use glam::{bvec2, dvec2, BVec2, DVec2, DVec3, IVec2, UVec2, Vec2};

    glam_test!(test_align, {
        use core::mem;
        assert_eq!(16, mem::size_of::<DVec2>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(mem::align_of::<f64>(), mem::align_of::<DVec2>());
        #[cfg(feature = "cuda")]
        assert_eq!(16, mem::align_of::<DVec2>());
        assert_eq!(2, mem::size_of::<BVec2>());
        assert_eq!(1, mem::align_of::<BVec2>());
    });

    glam_test!(test_try_from, {
        assert_eq!(DVec2::new(1.0, 2.0), DVec2::from(Vec2::new(1.0, 2.0)));
        assert_eq!(DVec2::new(1.0, 2.0), DVec2::from(IVec2::new(1, 2)));
        assert_eq!(DVec2::new(1.0, 2.0), DVec2::from(UVec2::new(1, 2)));
    });

    impl_vec2_float_tests!(f64, dvec2, DVec2, DVec3, BVec2, bvec2);
}

mod i8vec2 {
    use glam::{
        bvec2, i8vec2, BVec2, I16Vec2, I64Vec2, I8Vec2, I8Vec3, IVec2, U16Vec2, U64Vec2, U8Vec2,
        UVec2,
    };

    glam_test!(test_align, {
        use core::mem;
        assert_eq!(2, mem::size_of::<I8Vec2>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(1, mem::align_of::<I8Vec2>());
        #[cfg(feature = "cuda")]
        assert_eq!(2, mem::align_of::<I8Vec2>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            I8Vec2::new(1, 2),
            I8Vec2::try_from(U8Vec2::new(1, 2)).unwrap()
        );
        assert!(I8Vec2::try_from(U8Vec2::new(u8::MAX, 2)).is_err());
        assert!(I8Vec2::try_from(U8Vec2::new(1, u8::MAX)).is_err());

        assert_eq!(
            I8Vec2::new(1, 2),
            I8Vec2::try_from(I16Vec2::new(1, 2)).unwrap()
        );
        assert!(I8Vec2::try_from(I16Vec2::new(i16::MAX, 2)).is_err());
        assert!(I8Vec2::try_from(I16Vec2::new(1, i16::MAX)).is_err());

        assert_eq!(
            I8Vec2::new(1, 2),
            I8Vec2::try_from(U16Vec2::new(1, 2)).unwrap()
        );
        assert!(I8Vec2::try_from(U16Vec2::new(u16::MAX, 2)).is_err());
        assert!(I8Vec2::try_from(U16Vec2::new(1, u16::MAX)).is_err());

        assert_eq!(
            I8Vec2::new(1, 2),
            I8Vec2::try_from(IVec2::new(1, 2)).unwrap()
        );
        assert!(I8Vec2::try_from(IVec2::new(i32::MAX, 2)).is_err());
        assert!(I8Vec2::try_from(IVec2::new(1, i32::MAX)).is_err());

        assert_eq!(
            I8Vec2::new(1, 2),
            I8Vec2::try_from(UVec2::new(1, 2)).unwrap()
        );
        assert!(I8Vec2::try_from(UVec2::new(u32::MAX, 2)).is_err());
        assert!(I8Vec2::try_from(UVec2::new(1, u32::MAX)).is_err());

        assert_eq!(
            I8Vec2::new(1, 2),
            I8Vec2::try_from(I64Vec2::new(1, 2)).unwrap()
        );
        assert!(I8Vec2::try_from(I64Vec2::new(i64::MAX, 2)).is_err());
        assert!(I8Vec2::try_from(I64Vec2::new(1, i64::MAX)).is_err());

        assert_eq!(
            I8Vec2::new(1, 2),
            I8Vec2::try_from(U64Vec2::new(1, 2)).unwrap()
        );
        assert!(I8Vec2::try_from(U64Vec2::new(u64::MAX, 2)).is_err());
        assert!(I8Vec2::try_from(U64Vec2::new(1, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add, {
        assert_eq!(
            I8Vec2::new(i8::MAX, 5).wrapping_add(I8Vec2::new(1, 3)),
            I8Vec2::new(i8::MIN, 8),
        );
    });

    glam_test!(test_wrapping_sub, {
        assert_eq!(
            I8Vec2::new(i8::MAX, 5).wrapping_sub(I8Vec2::new(1, 3)),
            I8Vec2::new(126, 2)
        );
    });

    glam_test!(test_wrapping_mul, {
        assert_eq!(
            I8Vec2::new(i8::MAX, 5).wrapping_mul(I8Vec2::new(3, 3)),
            I8Vec2::new(125, 15)
        );
    });

    glam_test!(test_wrapping_div, {
        assert_eq!(
            I8Vec2::new(i8::MAX, 5).wrapping_div(I8Vec2::new(3, 3)),
            I8Vec2::new(42, 1)
        );
    });

    glam_test!(test_saturating_add, {
        assert_eq!(
            I8Vec2::new(i8::MAX, i8::MIN,).saturating_add(I8Vec2::new(1, -1)),
            I8Vec2::new(i8::MAX, i8::MIN)
        );
    });

    glam_test!(test_saturating_sub, {
        assert_eq!(
            I8Vec2::new(i8::MIN, i8::MAX).saturating_sub(I8Vec2::new(1, -1)),
            I8Vec2::new(i8::MIN, i8::MAX)
        );
    });

    glam_test!(test_saturating_mul, {
        assert_eq!(
            I8Vec2::new(i8::MAX, i8::MIN).saturating_mul(I8Vec2::new(2, 2)),
            I8Vec2::new(i8::MAX, i8::MIN)
        );
    });

    glam_test!(test_saturating_div, {
        assert_eq!(
            I8Vec2::new(i8::MAX, i8::MIN).saturating_div(I8Vec2::new(2, 2)),
            I8Vec2::new(63, -64)
        );
    });

    glam_test!(test_wrapping_add_unsigned, {
        assert_eq!(
            I8Vec2::new(i8::MAX, i8::MAX).wrapping_add_unsigned(U8Vec2::new(1, 1)),
            I8Vec2::new(i8::MIN, i8::MIN)
        );
    });

    glam_test!(test_wrapping_sub_unsigned, {
        assert_eq!(
            I8Vec2::new(i8::MIN, i8::MIN).wrapping_sub_unsigned(U8Vec2::new(1, 1)),
            I8Vec2::new(i8::MAX, i8::MAX)
        );
    });

    glam_test!(test_saturating_add_unsigned, {
        assert_eq!(
            I8Vec2::new(i8::MAX, i8::MAX).saturating_add_unsigned(U8Vec2::new(1, 1)),
            I8Vec2::new(i8::MAX, i8::MAX)
        );
    });

    glam_test!(test_saturating_sub_unsigned, {
        assert_eq!(
            I8Vec2::new(i8::MIN, i8::MIN).saturating_sub_unsigned(U8Vec2::new(1, 1)),
            I8Vec2::new(i8::MIN, i8::MIN)
        );
    });

    impl_vec2_signed_integer_tests!(i8, i8vec2, I8Vec2, I8Vec3, BVec2, bvec2);
    impl_vec2_eq_hash_tests!(i8, i8vec2);

    impl_vec2_scalar_shift_op_tests!(I8Vec2, -2, 2);
    impl_vec2_shift_op_tests!(I8Vec2);

    impl_vec2_scalar_bit_op_tests!(I8Vec2, -2, 2);
    impl_vec2_bit_op_tests!(I8Vec2, -2, 2);
}

mod u8vec2 {
    use glam::{
        bvec2, u8vec2, BVec2, I16Vec2, I64Vec2, I8Vec2, IVec2, U16Vec2, U64Vec2, U8Vec2, U8Vec3,
        UVec2,
    };

    glam_test!(test_align, {
        use core::mem;
        assert_eq!(2, mem::size_of::<U8Vec2>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(1, mem::align_of::<U8Vec2>());
        #[cfg(feature = "cuda")]
        assert_eq!(2, mem::align_of::<U8Vec2>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            U8Vec2::new(1, 2),
            U8Vec2::try_from(I8Vec2::new(1, 2)).unwrap()
        );
        assert!(U8Vec2::try_from(I8Vec2::new(-1, 2)).is_err());
        assert!(U8Vec2::try_from(I8Vec2::new(1, -2)).is_err());

        assert_eq!(
            U8Vec2::new(1, 2),
            U8Vec2::try_from(I16Vec2::new(1, 2)).unwrap()
        );
        assert!(U8Vec2::try_from(I16Vec2::new(-1, 2)).is_err());
        assert!(U8Vec2::try_from(I16Vec2::new(1, -2)).is_err());
        assert!(U8Vec2::try_from(I16Vec2::new(i16::MAX, 2)).is_err());
        assert!(U8Vec2::try_from(I16Vec2::new(1, i16::MAX)).is_err());

        assert_eq!(
            U8Vec2::new(1, 2),
            U8Vec2::try_from(U16Vec2::new(1, 2)).unwrap()
        );
        assert!(U8Vec2::try_from(U16Vec2::new(u16::MAX, 2)).is_err());
        assert!(U8Vec2::try_from(U16Vec2::new(1, u16::MAX)).is_err());

        assert_eq!(
            U8Vec2::new(1, 2),
            U8Vec2::try_from(IVec2::new(1, 2)).unwrap()
        );
        assert!(U8Vec2::try_from(IVec2::new(-1, 2)).is_err());
        assert!(U8Vec2::try_from(IVec2::new(1, -2)).is_err());

        assert!(U8Vec2::try_from(IVec2::new(i32::MAX, 2)).is_err());
        assert!(U8Vec2::try_from(IVec2::new(1, i32::MAX)).is_err());

        assert_eq!(
            U8Vec2::new(1, 2),
            U8Vec2::try_from(UVec2::new(1, 2)).unwrap()
        );
        assert!(U8Vec2::try_from(UVec2::new(u32::MAX, 2)).is_err());
        assert!(U8Vec2::try_from(UVec2::new(1, u32::MAX)).is_err());

        assert_eq!(
            U8Vec2::new(1, 2),
            U8Vec2::try_from(I64Vec2::new(1, 2)).unwrap()
        );
        assert!(U8Vec2::try_from(I64Vec2::new(-1, 2)).is_err());
        assert!(U8Vec2::try_from(I64Vec2::new(1, -2)).is_err());

        assert!(U8Vec2::try_from(I64Vec2::new(i64::MAX, 2)).is_err());
        assert!(U8Vec2::try_from(I64Vec2::new(1, i64::MAX)).is_err());

        assert_eq!(
            U8Vec2::new(1, 2),
            U8Vec2::try_from(U64Vec2::new(1, 2)).unwrap()
        );
        assert!(U8Vec2::try_from(U64Vec2::new(u64::MAX, 2)).is_err());
        assert!(U8Vec2::try_from(U64Vec2::new(1, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add, {
        assert_eq!(
            U8Vec2::new(u8::MAX, 5).wrapping_add(U8Vec2::new(1, 3)),
            U8Vec2::new(0, 8),
        );
    });

    glam_test!(test_wrapping_sub, {
        assert_eq!(
            U8Vec2::new(u8::MAX, 5).wrapping_sub(U8Vec2::new(1, 3)),
            U8Vec2::new(254, 2)
        );
    });

    glam_test!(test_wrapping_mul, {
        assert_eq!(
            U8Vec2::new(u8::MAX, 5).wrapping_mul(U8Vec2::new(3, 3)),
            U8Vec2::new(253, 15)
        );
    });

    glam_test!(test_wrapping_div, {
        assert_eq!(
            U8Vec2::new(u8::MAX, 5).wrapping_div(U8Vec2::new(3, 3)),
            U8Vec2::new(85, 1)
        );
    });

    glam_test!(test_saturating_add, {
        assert_eq!(
            U8Vec2::new(u8::MAX, u8::MAX).saturating_add(U8Vec2::new(1, u8::MAX)),
            U8Vec2::new(u8::MAX, u8::MAX)
        );
    });

    glam_test!(test_saturating_sub, {
        assert_eq!(
            U8Vec2::new(0, u8::MAX).saturating_sub(U8Vec2::new(1, 1)),
            U8Vec2::new(0, 254)
        );
    });

    glam_test!(test_saturating_mul, {
        assert_eq!(
            U8Vec2::new(u8::MAX, u8::MAX).saturating_mul(U8Vec2::new(2, u8::MAX)),
            U8Vec2::new(u8::MAX, u8::MAX)
        );
    });

    glam_test!(test_saturating_div, {
        assert_eq!(
            U8Vec2::new(u8::MAX, u8::MAX).saturating_div(U8Vec2::new(2, u8::MAX)),
            U8Vec2::new(127, 1)
        );
    });

    glam_test!(test_wrapping_add_signed, {
        assert_eq!(
            U8Vec2::new(u8::MAX, u8::MAX).wrapping_add_signed(I8Vec2::new(1, 1)),
            U8Vec2::new(u8::MIN, u8::MIN)
        );
    });

    glam_test!(test_saturating_add_signed, {
        assert_eq!(
            U8Vec2::new(u8::MAX, u8::MAX).saturating_add_signed(I8Vec2::new(1, 1)),
            U8Vec2::new(u8::MAX, u8::MAX)
        );
    });

    impl_vec2_tests!(u8, u8vec2, U8Vec2, U8Vec3, BVec2, bvec2);
    impl_vec2_eq_hash_tests!(u8, u8vec2);

    impl_vec2_scalar_shift_op_tests!(U8Vec2, 0, 2);
    impl_vec2_shift_op_tests!(U8Vec2);

    impl_vec2_scalar_bit_op_tests!(U8Vec2, 0, 2);
    impl_vec2_bit_op_tests!(U8Vec2, 0, 2);
}

mod i16vec2 {
    use glam::{
        bvec2, i16vec2, BVec2, I16Vec2, I16Vec3, I64Vec2, I8Vec2, IVec2, U16Vec2, U64Vec2, U8Vec2,
        UVec2,
    };

    glam_test!(test_align, {
        use core::mem;
        assert_eq!(4, mem::size_of::<I16Vec2>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(2, mem::align_of::<I16Vec2>());
        #[cfg(feature = "cuda")]
        assert_eq!(4, mem::align_of::<I16Vec2>());
    });

    glam_test!(test_try_from, {
        assert_eq!(I16Vec2::new(1, 2), I16Vec2::from(U8Vec2::new(1, 2)));
        assert_eq!(I16Vec2::new(1, 2), I16Vec2::from(I8Vec2::new(1, 2)));

        assert_eq!(
            I16Vec2::new(1, 2),
            I16Vec2::try_from(U16Vec2::new(1, 2)).unwrap()
        );
        assert!(I16Vec2::try_from(U16Vec2::new(u16::MAX, 2)).is_err());
        assert!(I16Vec2::try_from(U16Vec2::new(1, u16::MAX)).is_err());

        assert_eq!(
            I16Vec2::new(1, 2),
            I16Vec2::try_from(IVec2::new(1, 2)).unwrap()
        );
        assert!(I16Vec2::try_from(IVec2::new(i32::MAX, 2)).is_err());
        assert!(I16Vec2::try_from(IVec2::new(1, i32::MAX)).is_err());

        assert_eq!(
            I16Vec2::new(1, 2),
            I16Vec2::try_from(UVec2::new(1, 2)).unwrap()
        );
        assert!(I16Vec2::try_from(UVec2::new(u32::MAX, 2)).is_err());
        assert!(I16Vec2::try_from(UVec2::new(1, u32::MAX)).is_err());

        assert_eq!(
            I16Vec2::new(1, 2),
            I16Vec2::try_from(I64Vec2::new(1, 2)).unwrap()
        );
        assert!(I16Vec2::try_from(I64Vec2::new(i64::MAX, 2)).is_err());
        assert!(I16Vec2::try_from(I64Vec2::new(1, i64::MAX)).is_err());

        assert_eq!(
            I16Vec2::new(1, 2),
            I16Vec2::try_from(U64Vec2::new(1, 2)).unwrap()
        );
        assert!(I16Vec2::try_from(U64Vec2::new(u64::MAX, 2)).is_err());
        assert!(I16Vec2::try_from(U64Vec2::new(1, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add, {
        assert_eq!(
            I16Vec2::new(i16::MAX, 5).wrapping_add(I16Vec2::new(1, 3)),
            I16Vec2::new(i16::MIN, 8),
        );
    });

    glam_test!(test_wrapping_sub, {
        assert_eq!(
            I16Vec2::new(i16::MAX, 5).wrapping_sub(I16Vec2::new(1, 3)),
            I16Vec2::new(32766, 2)
        );
    });

    glam_test!(test_wrapping_mul, {
        assert_eq!(
            I16Vec2::new(i16::MAX, 5).wrapping_mul(I16Vec2::new(3, 3)),
            I16Vec2::new(32765, 15)
        );
    });

    glam_test!(test_wrapping_div, {
        assert_eq!(
            I16Vec2::new(i16::MAX, 5).wrapping_div(I16Vec2::new(3, 3)),
            I16Vec2::new(10922, 1)
        );
    });

    glam_test!(test_saturating_add, {
        assert_eq!(
            I16Vec2::new(i16::MAX, i16::MIN,).saturating_add(I16Vec2::new(1, -1)),
            I16Vec2::new(i16::MAX, i16::MIN)
        );
    });

    glam_test!(test_saturating_sub, {
        assert_eq!(
            I16Vec2::new(i16::MIN, i16::MAX).saturating_sub(I16Vec2::new(1, -1)),
            I16Vec2::new(i16::MIN, i16::MAX)
        );
    });

    glam_test!(test_saturating_mul, {
        assert_eq!(
            I16Vec2::new(i16::MAX, i16::MIN).saturating_mul(I16Vec2::new(2, 2)),
            I16Vec2::new(i16::MAX, i16::MIN)
        );
    });

    glam_test!(test_saturating_div, {
        assert_eq!(
            I16Vec2::new(i16::MAX, i16::MIN).saturating_div(I16Vec2::new(2, 2)),
            I16Vec2::new(16383, -16384)
        );
    });

    glam_test!(test_wrapping_add_unsigned, {
        assert_eq!(
            I16Vec2::new(i16::MAX, i16::MAX).wrapping_add_unsigned(U16Vec2::new(1, 1)),
            I16Vec2::new(i16::MIN, i16::MIN)
        );
    });

    glam_test!(test_wrapping_sub_unsigned, {
        assert_eq!(
            I16Vec2::new(i16::MIN, i16::MIN).wrapping_sub_unsigned(U16Vec2::new(1, 1)),
            I16Vec2::new(i16::MAX, i16::MAX)
        );
    });

    glam_test!(test_saturating_add_unsigned, {
        assert_eq!(
            I16Vec2::new(i16::MAX, i16::MAX).saturating_add_unsigned(U16Vec2::new(1, 1)),
            I16Vec2::new(i16::MAX, i16::MAX)
        );
    });

    glam_test!(test_saturating_sub_unsigned, {
        assert_eq!(
            I16Vec2::new(i16::MIN, i16::MIN).saturating_sub_unsigned(U16Vec2::new(1, 1)),
            I16Vec2::new(i16::MIN, i16::MIN)
        );
    });

    impl_vec2_signed_integer_tests!(i16, i16vec2, I16Vec2, I16Vec3, BVec2, bvec2);
    impl_vec2_eq_hash_tests!(i16, i16vec2);

    impl_vec2_scalar_shift_op_tests!(I16Vec2, -2, 2);
    impl_vec2_shift_op_tests!(I16Vec2);

    impl_vec2_scalar_bit_op_tests!(I16Vec2, -2, 2);
    impl_vec2_bit_op_tests!(I16Vec2, -2, 2);
}

mod u16vec2 {
    use glam::{
        bvec2, u16vec2, BVec2, I16Vec2, I64Vec2, I8Vec2, IVec2, U16Vec2, U16Vec3, U64Vec2, UVec2,
    };

    glam_test!(test_align, {
        use core::mem;
        assert_eq!(4, mem::size_of::<U16Vec2>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(2, mem::align_of::<U16Vec2>());
        #[cfg(feature = "cuda")]
        assert_eq!(4, mem::align_of::<U16Vec2>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            U16Vec2::new(1, 2),
            U16Vec2::try_from(I8Vec2::new(1, 2)).unwrap()
        );
        assert!(U16Vec2::try_from(I8Vec2::new(-1, 2)).is_err());
        assert!(U16Vec2::try_from(I8Vec2::new(1, -2)).is_err());

        assert_eq!(
            U16Vec2::new(1, 2),
            U16Vec2::try_from(I16Vec2::new(1, 2)).unwrap()
        );
        assert!(U16Vec2::try_from(I16Vec2::new(-1, 2)).is_err());
        assert!(U16Vec2::try_from(I16Vec2::new(1, -2)).is_err());

        assert_eq!(
            U16Vec2::new(1, 2),
            U16Vec2::try_from(IVec2::new(1, 2)).unwrap()
        );
        assert!(U16Vec2::try_from(IVec2::new(-1, 2)).is_err());
        assert!(U16Vec2::try_from(IVec2::new(1, -2)).is_err());

        assert!(U16Vec2::try_from(IVec2::new(i32::MAX, 2)).is_err());
        assert!(U16Vec2::try_from(IVec2::new(1, i32::MAX)).is_err());

        assert_eq!(
            U16Vec2::new(1, 2),
            U16Vec2::try_from(UVec2::new(1, 2)).unwrap()
        );
        assert!(U16Vec2::try_from(UVec2::new(u32::MAX, 2)).is_err());
        assert!(U16Vec2::try_from(UVec2::new(1, u32::MAX)).is_err());

        assert_eq!(
            U16Vec2::new(1, 2),
            U16Vec2::try_from(I64Vec2::new(1, 2)).unwrap()
        );
        assert!(U16Vec2::try_from(I64Vec2::new(-1, 2)).is_err());
        assert!(U16Vec2::try_from(I64Vec2::new(1, -2)).is_err());

        assert!(U16Vec2::try_from(I64Vec2::new(i64::MAX, 2)).is_err());
        assert!(U16Vec2::try_from(I64Vec2::new(1, i64::MAX)).is_err());

        assert_eq!(
            U16Vec2::new(1, 2),
            U16Vec2::try_from(U64Vec2::new(1, 2)).unwrap()
        );
        assert!(U16Vec2::try_from(U64Vec2::new(u64::MAX, 2)).is_err());
        assert!(U16Vec2::try_from(U64Vec2::new(1, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add, {
        assert_eq!(
            U16Vec2::new(u16::MAX, 5).wrapping_add(U16Vec2::new(1, 3)),
            U16Vec2::new(0, 8),
        );
    });

    glam_test!(test_wrapping_sub, {
        assert_eq!(
            U16Vec2::new(u16::MAX, 5).wrapping_sub(U16Vec2::new(1, 3)),
            U16Vec2::new(65534, 2)
        );
    });

    glam_test!(test_wrapping_mul, {
        assert_eq!(
            U16Vec2::new(u16::MAX, 5).wrapping_mul(U16Vec2::new(3, 3)),
            U16Vec2::new(65533, 15)
        );
    });

    glam_test!(test_wrapping_div, {
        assert_eq!(
            U16Vec2::new(u16::MAX, 5).wrapping_div(U16Vec2::new(3, 3)),
            U16Vec2::new(21845, 1)
        );
    });

    glam_test!(test_saturating_add, {
        assert_eq!(
            U16Vec2::new(u16::MAX, u16::MAX).saturating_add(U16Vec2::new(1, u16::MAX)),
            U16Vec2::new(u16::MAX, u16::MAX)
        );
    });

    glam_test!(test_saturating_sub, {
        assert_eq!(
            U16Vec2::new(0, u16::MAX).saturating_sub(U16Vec2::new(1, 1)),
            U16Vec2::new(0, 65534)
        );
    });

    glam_test!(test_saturating_mul, {
        assert_eq!(
            U16Vec2::new(u16::MAX, u16::MAX).saturating_mul(U16Vec2::new(2, u16::MAX)),
            U16Vec2::new(u16::MAX, u16::MAX)
        );
    });

    glam_test!(test_saturating_div, {
        assert_eq!(
            U16Vec2::new(u16::MAX, u16::MAX).saturating_div(U16Vec2::new(2, u16::MAX)),
            U16Vec2::new(32767, 1)
        );
    });

    glam_test!(test_wrapping_add_signed, {
        assert_eq!(
            U16Vec2::new(u16::MAX, u16::MAX).wrapping_add_signed(I16Vec2::new(1, 1)),
            U16Vec2::new(u16::MIN, u16::MIN)
        );
    });

    glam_test!(test_saturating_add_signed, {
        assert_eq!(
            U16Vec2::new(u16::MAX, u16::MAX).saturating_add_signed(I16Vec2::new(1, 1)),
            U16Vec2::new(u16::MAX, u16::MAX)
        );
    });

    impl_vec2_tests!(u16, u16vec2, U16Vec2, U16Vec3, BVec2, bvec2);
    impl_vec2_eq_hash_tests!(u16, u16vec2);

    impl_vec2_scalar_shift_op_tests!(U16Vec2, 0, 2);
    impl_vec2_shift_op_tests!(U16Vec2);

    impl_vec2_scalar_bit_op_tests!(U16Vec2, 0, 2);
    impl_vec2_bit_op_tests!(U16Vec2, 0, 2);
}

mod ivec2 {
    use glam::{
        bvec2, ivec2, BVec2, I16Vec2, I64Vec2, I8Vec2, IVec2, IVec3, U16Vec2, U64Vec2, U8Vec2,
        UVec2,
    };

    glam_test!(test_align, {
        use core::mem;
        assert_eq!(8, mem::size_of::<IVec2>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(4, mem::align_of::<IVec2>());
        #[cfg(feature = "cuda")]
        assert_eq!(8, mem::align_of::<IVec2>());
        assert_eq!(2, mem::size_of::<BVec2>());
        assert_eq!(1, mem::align_of::<BVec2>());
    });

    glam_test!(test_try_from, {
        assert_eq!(IVec2::new(1, 2), IVec2::from(U8Vec2::new(1, 2)));
        assert_eq!(IVec2::new(1, 2), IVec2::from(I8Vec2::new(1, 2)));

        assert_eq!(IVec2::new(1, 2), IVec2::from(U16Vec2::new(1, 2)));
        assert_eq!(IVec2::new(1, 2), IVec2::from(I16Vec2::new(1, 2)));

        assert_eq!(IVec2::new(1, 2), IVec2::try_from(UVec2::new(1, 2)).unwrap());
        assert!(IVec2::try_from(UVec2::new(u32::MAX, 2)).is_err());
        assert!(IVec2::try_from(UVec2::new(1, u32::MAX)).is_err());

        assert_eq!(
            IVec2::new(1, 2),
            IVec2::try_from(I64Vec2::new(1, 2)).unwrap()
        );
        assert!(IVec2::try_from(I64Vec2::new(i64::MAX, 2)).is_err());
        assert!(IVec2::try_from(I64Vec2::new(1, i64::MAX)).is_err());

        assert_eq!(
            IVec2::new(1, 2),
            IVec2::try_from(U64Vec2::new(1, 2)).unwrap()
        );
        assert!(IVec2::try_from(U64Vec2::new(u64::MAX, 2)).is_err());
        assert!(IVec2::try_from(U64Vec2::new(1, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add, {
        assert_eq!(
            IVec2::new(i32::MAX, 5).wrapping_add(IVec2::new(1, 3)),
            IVec2::new(i32::MIN, 8),
        );
    });

    glam_test!(test_wrapping_sub, {
        assert_eq!(
            IVec2::new(i32::MAX, 5).wrapping_sub(IVec2::new(1, 3)),
            IVec2::new(2147483646, 2)
        );
    });

    glam_test!(test_wrapping_mul, {
        assert_eq!(
            IVec2::new(i32::MAX, 5).wrapping_mul(IVec2::new(3, 3)),
            IVec2::new(2147483645, 15)
        );
    });

    glam_test!(test_wrapping_div, {
        assert_eq!(
            IVec2::new(i32::MAX, 5).wrapping_div(IVec2::new(3, 3)),
            IVec2::new(715827882, 1)
        );
    });

    glam_test!(test_saturating_add, {
        assert_eq!(
            IVec2::new(i32::MAX, i32::MIN,).saturating_add(IVec2::new(1, -1)),
            IVec2::new(i32::MAX, i32::MIN)
        );
    });

    glam_test!(test_saturating_sub, {
        assert_eq!(
            IVec2::new(i32::MIN, i32::MAX).saturating_sub(IVec2::new(1, -1)),
            IVec2::new(i32::MIN, i32::MAX)
        );
    });

    glam_test!(test_saturating_mul, {
        assert_eq!(
            IVec2::new(i32::MAX, i32::MIN).saturating_mul(IVec2::new(2, 2)),
            IVec2::new(i32::MAX, i32::MIN)
        );
    });

    glam_test!(test_saturating_div, {
        assert_eq!(
            IVec2::new(i32::MAX, i32::MIN).saturating_div(IVec2::new(2, 2)),
            IVec2::new(1073741823, -1073741824)
        );
    });

    glam_test!(test_wrapping_add_unsigned, {
        assert_eq!(
            IVec2::new(i32::MAX, i32::MAX).wrapping_add_unsigned(UVec2::new(1, 1)),
            IVec2::new(i32::MIN, i32::MIN)
        );
    });

    glam_test!(test_wrapping_sub_unsigned, {
        assert_eq!(
            IVec2::new(i32::MIN, i32::MIN).wrapping_sub_unsigned(UVec2::new(1, 1)),
            IVec2::new(i32::MAX, i32::MAX)
        );
    });

    glam_test!(test_saturating_add_unsigned, {
        assert_eq!(
            IVec2::new(i32::MAX, i32::MAX).saturating_add_unsigned(UVec2::new(1, 1)),
            IVec2::new(i32::MAX, i32::MAX)
        );
    });

    glam_test!(test_saturating_sub_unsigned, {
        assert_eq!(
            IVec2::new(i32::MIN, i32::MIN).saturating_sub_unsigned(UVec2::new(1, 1)),
            IVec2::new(i32::MIN, i32::MIN)
        );
    });

    impl_vec2_signed_integer_tests!(i32, ivec2, IVec2, IVec3, BVec2, bvec2);
    impl_vec2_eq_hash_tests!(i32, ivec2);

    impl_vec2_scalar_shift_op_tests!(IVec2, -2, 2);
    impl_vec2_shift_op_tests!(IVec2);

    impl_vec2_scalar_bit_op_tests!(IVec2, -2, 2);
    impl_vec2_bit_op_tests!(IVec2, -2, 2);
}

mod uvec2 {
    use glam::{
        bvec2, uvec2, BVec2, I16Vec2, I64Vec2, I8Vec2, IVec2, U16Vec2, U64Vec2, U8Vec2, UVec2,
        UVec3,
    };

    glam_test!(test_align, {
        use core::mem;
        assert_eq!(8, mem::size_of::<UVec2>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(4, mem::align_of::<UVec2>());
        #[cfg(feature = "cuda")]
        assert_eq!(8, mem::align_of::<UVec2>());
        assert_eq!(2, mem::size_of::<BVec2>());
        assert_eq!(1, mem::align_of::<BVec2>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            UVec2::new(1, 2),
            UVec2::try_from(I8Vec2::new(1, 2)).unwrap()
        );
        assert!(UVec2::try_from(I8Vec2::new(-1, 2)).is_err());
        assert!(UVec2::try_from(I8Vec2::new(1, -2)).is_err());

        assert_eq!(UVec2::new(1, 2), UVec2::from(U8Vec2::new(1, 2)));

        assert_eq!(
            UVec2::new(1, 2),
            UVec2::try_from(I16Vec2::new(1, 2)).unwrap()
        );
        assert!(UVec2::try_from(I16Vec2::new(-1, 2)).is_err());
        assert!(UVec2::try_from(I16Vec2::new(1, -2)).is_err());

        assert_eq!(UVec2::new(1, 2), UVec2::from(U16Vec2::new(1, 2)));

        assert_eq!(UVec2::new(1, 2), UVec2::try_from(IVec2::new(1, 2)).unwrap());
        assert!(UVec2::try_from(IVec2::new(-1, 2)).is_err());
        assert!(UVec2::try_from(IVec2::new(1, -2)).is_err());

        assert_eq!(
            UVec2::new(1, 2),
            UVec2::try_from(I64Vec2::new(1, 2)).unwrap()
        );
        assert!(UVec2::try_from(I64Vec2::new(-1, 2)).is_err());
        assert!(UVec2::try_from(I64Vec2::new(1, -2)).is_err());

        assert!(UVec2::try_from(I64Vec2::new(i64::MAX, 2)).is_err());
        assert!(UVec2::try_from(I64Vec2::new(1, i64::MAX)).is_err());

        assert_eq!(
            UVec2::new(1, 2),
            UVec2::try_from(U64Vec2::new(1, 2)).unwrap()
        );
        assert!(UVec2::try_from(U64Vec2::new(u64::MAX, 2)).is_err());
        assert!(UVec2::try_from(U64Vec2::new(1, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add, {
        assert_eq!(
            UVec2::new(u32::MAX, 5).wrapping_add(UVec2::new(1, 3)),
            UVec2::new(0, 8),
        );
    });

    glam_test!(test_wrapping_sub, {
        assert_eq!(
            UVec2::new(u32::MAX, 5).wrapping_sub(UVec2::new(1, 3)),
            UVec2::new(4294967294, 2)
        );
    });

    glam_test!(test_wrapping_mul, {
        assert_eq!(
            UVec2::new(u32::MAX, 5).wrapping_mul(UVec2::new(3, 3)),
            UVec2::new(4294967293, 15)
        );
    });

    glam_test!(test_wrapping_div, {
        assert_eq!(
            UVec2::new(u32::MAX, 5).wrapping_div(UVec2::new(3, 3)),
            UVec2::new(1431655765, 1)
        );
    });

    glam_test!(test_saturating_add, {
        assert_eq!(
            UVec2::new(u32::MAX, u32::MAX).saturating_add(UVec2::new(1, u32::MAX)),
            UVec2::new(u32::MAX, u32::MAX)
        );
    });

    glam_test!(test_saturating_sub, {
        assert_eq!(
            UVec2::new(0, u32::MAX).saturating_sub(UVec2::new(1, 1)),
            UVec2::new(0, 4294967294)
        );
    });

    glam_test!(test_saturating_mul, {
        assert_eq!(
            UVec2::new(u32::MAX, u32::MAX).saturating_mul(UVec2::new(2, u32::MAX)),
            UVec2::new(u32::MAX, u32::MAX)
        );
    });

    glam_test!(test_saturating_div, {
        assert_eq!(
            UVec2::new(u32::MAX, u32::MAX).saturating_div(UVec2::new(2, u32::MAX)),
            UVec2::new(2147483647, 1)
        );
    });

    glam_test!(test_wrapping_add_signed, {
        assert_eq!(
            UVec2::new(u32::MAX, u32::MAX).wrapping_add_signed(IVec2::new(1, 1)),
            UVec2::new(u32::MIN, u32::MIN)
        );
    });

    glam_test!(test_saturating_add_signed, {
        assert_eq!(
            UVec2::new(u32::MAX, u32::MAX).saturating_add_signed(IVec2::new(1, 1)),
            UVec2::new(u32::MAX, u32::MAX)
        );
    });

    impl_vec2_tests!(u32, uvec2, UVec2, UVec3, BVec2, bvec2);
    impl_vec2_eq_hash_tests!(u32, uvec2);

    impl_vec2_scalar_shift_op_tests!(UVec2, 0, 2);
    impl_vec2_shift_op_tests!(UVec2);

    impl_vec2_scalar_bit_op_tests!(UVec2, 0, 2);
    impl_vec2_bit_op_tests!(UVec2, 0, 2);
}

mod i64vec2 {
    use glam::{
        bvec2, i64vec2, BVec2, I16Vec2, I64Vec2, I64Vec3, I8Vec2, IVec2, U16Vec2, U64Vec2, U8Vec2,
        UVec2,
    };

    glam_test!(test_align, {
        use core::mem;
        assert_eq!(16, mem::size_of::<I64Vec2>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(8, mem::align_of::<I64Vec2>());
        #[cfg(feature = "cuda")]
        assert_eq!(16, mem::align_of::<I64Vec2>());
        assert_eq!(2, mem::size_of::<BVec2>());
        assert_eq!(1, mem::align_of::<BVec2>());
    });

    glam_test!(test_try_from, {
        assert_eq!(I64Vec2::new(1, 2), I64Vec2::from(I8Vec2::new(1, 2)));
        assert_eq!(I64Vec2::new(1, 2), I64Vec2::from(U8Vec2::new(1, 2)));
        assert_eq!(I64Vec2::new(1, 2), I64Vec2::from(I16Vec2::new(1, 2)));
        assert_eq!(I64Vec2::new(1, 2), I64Vec2::from(U16Vec2::new(1, 2)));
        assert_eq!(I64Vec2::new(1, 2), I64Vec2::from(IVec2::new(1, 2)));
        assert_eq!(I64Vec2::new(1, 2), I64Vec2::from(UVec2::new(1, 2)));

        assert_eq!(
            I64Vec2::new(1, 2),
            I64Vec2::try_from(U64Vec2::new(1, 2)).unwrap()
        );
        assert!(I64Vec2::try_from(U64Vec2::new(u64::MAX, 2)).is_err());
        assert!(I64Vec2::try_from(U64Vec2::new(1, u64::MAX)).is_err());
    });

    glam_test!(test_wrapping_add_unsigned, {
        assert_eq!(
            I64Vec2::new(i64::MAX, i64::MAX).wrapping_add_unsigned(U64Vec2::new(1, 1)),
            I64Vec2::new(i64::MIN, i64::MIN)
        );
    });

    glam_test!(test_wrapping_sub_unsigned, {
        assert_eq!(
            I64Vec2::new(i64::MIN, i64::MIN).wrapping_sub_unsigned(U64Vec2::new(1, 1)),
            I64Vec2::new(i64::MAX, i64::MAX)
        );
    });

    glam_test!(test_saturating_add_unsigned, {
        assert_eq!(
            I64Vec2::new(i64::MAX, i64::MAX).saturating_add_unsigned(U64Vec2::new(1, 1)),
            I64Vec2::new(i64::MAX, i64::MAX)
        );
    });

    glam_test!(test_saturating_sub_unsigned, {
        assert_eq!(
            I64Vec2::new(i64::MIN, i64::MIN).saturating_sub_unsigned(U64Vec2::new(1, 1)),
            I64Vec2::new(i64::MIN, i64::MIN)
        );
    });

    impl_vec2_signed_integer_tests!(i64, i64vec2, I64Vec2, I64Vec3, BVec2, bvec2);
    impl_vec2_eq_hash_tests!(i64, i64vec2);

    impl_vec2_scalar_shift_op_tests!(I64Vec2, -2, 2);
    impl_vec2_shift_op_tests!(I64Vec2);

    impl_vec2_scalar_bit_op_tests!(I64Vec2, -2, 2);
    impl_vec2_bit_op_tests!(I64Vec2, -2, 2);
}

mod u64vec2 {
    use glam::{
        bvec2, u64vec2, BVec2, I16Vec2, I64Vec2, I8Vec2, IVec2, U16Vec2, U64Vec2, U64Vec3, U8Vec2,
        UVec2,
    };

    glam_test!(test_align, {
        use core::mem;
        assert_eq!(16, mem::size_of::<U64Vec2>());
        #[cfg(not(feature = "cuda"))]
        assert_eq!(8, mem::align_of::<U64Vec2>());
        #[cfg(feature = "cuda")]
        assert_eq!(16, mem::align_of::<U64Vec2>());
        assert_eq!(2, mem::size_of::<BVec2>());
        assert_eq!(1, mem::align_of::<BVec2>());
    });

    glam_test!(test_try_from, {
        assert_eq!(
            U64Vec2::new(1, 2),
            U64Vec2::try_from(I8Vec2::new(1, 2)).unwrap()
        );
        assert!(U64Vec2::try_from(I8Vec2::new(-1, 2)).is_err());
        assert!(U64Vec2::try_from(I8Vec2::new(1, -2)).is_err());

        assert_eq!(U64Vec2::new(1, 2), U64Vec2::from(U8Vec2::new(1, 2)));

        assert_eq!(
            U64Vec2::new(1, 2),
            U64Vec2::try_from(I16Vec2::new(1, 2)).unwrap()
        );
        assert!(U64Vec2::try_from(I16Vec2::new(-1, 2)).is_err());
        assert!(U64Vec2::try_from(I16Vec2::new(1, -2)).is_err());

        assert_eq!(U64Vec2::new(1, 2), U64Vec2::from(U16Vec2::new(1, 2)));

        assert_eq!(
            U64Vec2::new(1, 2),
            U64Vec2::try_from(IVec2::new(1, 2)).unwrap()
        );
        assert!(U64Vec2::try_from(IVec2::new(-1, 2)).is_err());
        assert!(U64Vec2::try_from(IVec2::new(1, -2)).is_err());

        assert_eq!(U64Vec2::new(1, 2), U64Vec2::from(UVec2::new(1, 2)));

        assert_eq!(
            U64Vec2::new(1, 2),
            U64Vec2::try_from(I64Vec2::new(1, 2)).unwrap()
        );
        assert!(U64Vec2::try_from(I64Vec2::new(-1, 2)).is_err());
        assert!(U64Vec2::try_from(I64Vec2::new(1, -2)).is_err());
    });

    glam_test!(test_wrapping_add_signed, {
        assert_eq!(
            U64Vec2::new(u64::MAX, u64::MAX).wrapping_add_signed(I64Vec2::new(1, 1)),
            U64Vec2::new(u64::MIN, u64::MIN)
        );
    });

    glam_test!(test_saturating_add_signed, {
        assert_eq!(
            U64Vec2::new(u64::MAX, u64::MAX).saturating_add_signed(I64Vec2::new(1, 1)),
            U64Vec2::new(u64::MAX, u64::MAX)
        );
    });

    impl_vec2_tests!(u64, u64vec2, U64Vec2, U64Vec3, BVec2, bvec2);
    impl_vec2_eq_hash_tests!(u64, u64vec2);

    impl_vec2_scalar_shift_op_tests!(U64Vec2, 0, 2);
    impl_vec2_shift_op_tests!(U64Vec2);

    impl_vec2_scalar_bit_op_tests!(U64Vec2, 0, 2);
    impl_vec2_bit_op_tests!(U64Vec2, 0, 2);
}
