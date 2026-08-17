#![allow(clippy::excessive_precision)]

#[macro_use]
mod support;

macro_rules! impl_vec3_tests {
    ($t:ident, $new:ident, $vec3:ident, $mask:ident) => {
        glam_test!(test_const, {
            const V0: $vec3 = $vec3::splat(1 as $t);
            const V1: $vec3 = $vec3::new(1 as $t, 2 as $t, 3 as $t);
            const V2: $vec3 = $vec3::from_array([1 as $t, 2 as $t, 3 as $t]);
            assert_eq!([1 as $t, 1 as $t, 1 as $t], *V0.as_ref());
            assert_eq!([1 as $t, 2 as $t, 3 as $t], *V1.as_ref());
            assert_eq!([1 as $t, 2 as $t, 3 as $t], *V2.as_ref());
        });

        glam_test!(test_vec3_consts, {
            assert_eq!($vec3::ZERO, $new(0 as $t, 0 as $t, 0 as $t));
            assert_eq!($vec3::ONE, $new(1 as $t, 1 as $t, 1 as $t));
            assert_eq!($vec3::X, $new(1 as $t, 0 as $t, 0 as $t));
            assert_eq!($vec3::Y, $new(0 as $t, 1 as $t, 0 as $t));
            assert_eq!($vec3::Z, $new(0 as $t, 0 as $t, 1 as $t));
        });

        glam_test!(test_new, {
            let v = $new(1 as $t, 2 as $t, 3 as $t);

            assert_eq!(v.x, 1 as $t);
            assert_eq!(v.y, 2 as $t);
            assert_eq!(v.z, 3 as $t);

            let t = (1 as $t, 2 as $t, 3 as $t);
            let v = $vec3::from(t);
            assert_eq!(t, v.into());

            let a = [1 as $t, 2 as $t, 3 as $t];
            let v = $vec3::from(a);
            let a1: [$t; 3] = v.into();
            assert_eq!(a, a1);

            assert_eq!(a, v.to_array());
            assert_eq!(a, *v.as_ref());

            let mut v2 = $vec3::default();
            *v2.as_mut() = a;
            assert_eq!(a, v2.to_array());

            let v = $vec3::new(t.0, t.1, t.2);
            assert_eq!(t, v.into());

            assert_eq!($vec3::new(1 as $t, 0 as $t, 0 as $t), $vec3::X);
            assert_eq!($vec3::new(0 as $t, 1 as $t, 0 as $t), $vec3::Y);
            assert_eq!($vec3::new(0 as $t, 0 as $t, 1 as $t), $vec3::Z);
        });

        glam_test!(test_fmt, {
            let a = $vec3::new(1 as $t, 2 as $t, 3 as $t);
            assert_eq!(
                format!("{:?}", a),
                format!("{}({:?}, {:?}, {:?})", stringify!($vec3), a.x, a.y, a.z)
            );
            assert_eq!(
                format!("{:#?}", a),
                format!(
                    "{}(\n    {:#?},\n    {:#?},\n    {:#?},\n)",
                    stringify!($vec3),
                    a.x,
                    a.y,
                    a.z
                )
            );
            assert_eq!(format!("{}", a), "[1, 2, 3]");
        });

        glam_test!(test_zero, {
            let v = $vec3::ZERO;
            assert_eq!((0 as $t, 0 as $t, 0 as $t), v.into());
            assert_eq!(v, $vec3::default());
        });

        glam_test!(test_splat, {
            let v = $vec3::splat(1 as $t);
            assert_eq!($vec3::ONE, v);
        });

        glam_test!(test_accessors, {
            let mut a = $vec3::ZERO;
            a.x = 1 as $t;
            a.y = 2 as $t;
            a.z = 3 as $t;
            assert_eq!(1 as $t, a.x);
            assert_eq!(2 as $t, a.y);
            assert_eq!(3 as $t, a.z);
            assert_eq!((1 as $t, 2 as $t, 3 as $t), a.into());

            let mut a = $vec3::ZERO;
            a[0] = 1 as $t;
            a[1] = 2 as $t;
            a[2] = 3 as $t;
            assert_eq!(1 as $t, a[0]);
            assert_eq!(2 as $t, a[1]);
            assert_eq!(3 as $t, a[2]);
            assert_eq!((1 as $t, 2 as $t, 3 as $t), a.into());
        });

        glam_test!(test_dot_unsigned, {
            let x = $new(1 as $t, 0 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t, 0 as $t);
            assert_eq!(1 as $t, x.dot(x));
            assert_eq!(0 as $t, x.dot(y));
        });

        glam_test!(test_cross, {
            let x = $new(1 as $t, 0 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t, 0 as $t);
            let z = $new(0 as $t, 0 as $t, 1 as $t);
            assert_eq!(y, z.cross(x));
            assert_eq!(z, x.cross(y));
        });

        glam_test!(test_ops, {
            let a = $new(2 as $t, 4 as $t, 8 as $t);
            assert_eq!($new(4 as $t, 8 as $t, 16 as $t), a + a);
            assert_eq!($new(2 as $t, 4 as $t, 8 as $t), 0 as $t + a);
            assert_eq!($new(0 as $t, 0 as $t, 0 as $t), a - a);
            assert_eq!($new(14 as $t, 12 as $t, 8 as $t), 16 as $t - a);
            assert_eq!($new(4 as $t, 16 as $t, 64 as $t), a * a);
            assert_eq!($new(4 as $t, 8 as $t, 16 as $t), a * 2 as $t);
            assert_eq!($new(4 as $t, 8 as $t, 16 as $t), 2 as $t * a);
            assert_eq!($new(1 as $t, 1 as $t, 1 as $t), a / a);
            assert_eq!($new(1 as $t, 2 as $t, 4 as $t), a / 2 as $t);
            assert_eq!($new(4 as $t, 2 as $t, 1 as $t), 8 as $t / a);
            assert_eq!($new(0 as $t, 0 as $t, 0 as $t), a % a);
            assert_eq!($new(0 as $t, 1 as $t, 1 as $t), a % (a - 1 as $t));
            assert_eq!($new(0 as $t, 0 as $t, 0 as $t), a % 1 as $t);
            assert_eq!($new(2 as $t, 1 as $t, 2 as $t), a % 3 as $t);
            assert_eq!($new(1 as $t, 1 as $t, 1 as $t), 17 as $t % a);
            assert_eq!($new(2 as $t, 4 as $t, 0 as $t), a % 8 as $t);
        });

        glam_test!(test_assign_ops, {
            let a = $new(1 as $t, 2 as $t, 3 as $t);
            let mut b = a;

            b += 2 as $t;
            assert_eq!($new(3 as $t, 4 as $t, 5 as $t), b);
            b -= 2 as $t;
            assert_eq!($new(1 as $t, 2 as $t, 3 as $t), b);
            b *= 2 as $t;
            assert_eq!($new(2 as $t, 4 as $t, 6 as $t), b);
            b /= 2 as $t;
            assert_eq!($new(1 as $t, 2 as $t, 3 as $t), b);
            b %= 2 as $t;
            assert_eq!($new(1 as $t, 0 as $t, 1 as $t), b);

            b = a;
            b += a;
            assert_eq!($new(2 as $t, 4 as $t, 6 as $t), b);
            b -= a;
            assert_eq!($new(1 as $t, 2 as $t, 3 as $t), b);
            b *= a;
            assert_eq!($new(1 as $t, 4 as $t, 9 as $t), b);
            b /= a;
            assert_eq!($new(1 as $t, 2 as $t, 3 as $t), b);
            b *= 2 as $t;
            assert_eq!($new(2 as $t, 4 as $t, 6 as $t), b);
            b /= 2 as $t;
            assert_eq!($new(1 as $t, 2 as $t, 3 as $t), b);
            b %= (b + 1 as $t);
            assert_eq!($new(1 as $t, 2 as $t, 3 as $t), b);
            b %= b;
            assert_eq!($new(0 as $t, 0 as $t, 0 as $t), b);
        });

        glam_test!(test_min_max, {
            let a = $new(3 as $t, 5 as $t, 1 as $t);
            let b = $new(4 as $t, 2 as $t, 6 as $t);
            assert_eq!((3 as $t, 2 as $t, 1 as $t), a.min(b).into());
            assert_eq!((3 as $t, 2 as $t, 1 as $t), b.min(a).into());
            assert_eq!((4 as $t, 5 as $t, 6 as $t), a.max(b).into());
            assert_eq!((4 as $t, 5 as $t, 6 as $t), b.max(a).into());
        });

        glam_test!(test_clamp, {
            fn vec(x: i32, y: i32, z: i32) -> $vec3 {
                $vec3::new(x as $t, y as $t, z as $t)
            }
            let min = vec(1, 3, 3);
            let max = vec(6, 8, 8);
            assert_eq!(vec(0, 0, 0).clamp(min, max), vec(1, 3, 3));
            assert_eq!(vec(2, 2, 2).clamp(min, max), vec(2, 3, 3));
            assert_eq!(vec(4, 5, 5).clamp(min, max), vec(4, 5, 5));
            assert_eq!(vec(6, 6, 6).clamp(min, max), vec(6, 6, 6));
            assert_eq!(vec(7, 7, 7).clamp(min, max), vec(6, 7, 7));
            assert_eq!(vec(9, 9, 9).clamp(min, max), vec(6, 8, 8));

            should_glam_assert!({ $vec3::clamp($vec3::ZERO, $vec3::ONE, $vec3::ZERO) });
        });

        glam_test!(test_hmin_hmax, {
            assert_eq!(1 as $t, $new(1 as $t, 2 as $t, 3 as $t).min_element());
            assert_eq!(1 as $t, $new(3 as $t, 1 as $t, 2 as $t).min_element());
            assert_eq!(1 as $t, $new(2 as $t, 3 as $t, 1 as $t).min_element());
            assert_eq!(3 as $t, $new(1 as $t, 2 as $t, 3 as $t).max_element());
            assert_eq!(3 as $t, $new(3 as $t, 1 as $t, 2 as $t).max_element());
            assert_eq!(3 as $t, $new(2 as $t, 3 as $t, 1 as $t).max_element());
        });

        glam_test!(test_eq, {
            let a = $new(1 as $t, 1 as $t, 1 as $t);
            let b = $new(1 as $t, 2 as $t, 3 as $t);
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
            let a = $new(1 as $t, 1 as $t, 1 as $t);
            let b = $new(2 as $t, 2 as $t, 2 as $t);
            let c = $new(1 as $t, 1 as $t, 2 as $t);
            let d = $new(2 as $t, 1 as $t, 1 as $t);
            assert_eq!(a.cmplt(a).bitmask(), 0x0);
            assert_eq!(a.cmplt(b).bitmask(), 0x7);
            assert_eq!(a.cmplt(c).bitmask(), 0x4);
            assert_eq!(c.cmple(a).bitmask(), 0x3);
            assert_eq!(a.cmplt(d).bitmask(), 0x1);
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
            assert!(a == a);
        });

        glam_test!(test_extend_truncate, {
            let a = $new(1 as $t, 2 as $t, 3 as $t);
            let b = a.extend(4 as $t);
            assert_eq!((1 as $t, 2 as $t, 3 as $t, 4 as $t), b.into());
            let c = $vec3::from(b.truncate());
            assert_eq!(a, c);
        });

        glam_test!(test_mask, {
            let mut a = $vec3::ZERO;
            a.x = 1 as $t;
            a.y = 1 as $t;
            a.z = 1 as $t;
            assert!(!a.cmpeq($vec3::ZERO).any());
            assert!(a.cmpeq($vec3::ONE).all());
        });

        glam_test!(test_mask_into_array_u32, {
            assert_eq!(
                Into::<[u32; 3]>::into($mask::new(false, false, false)),
                [0, 0, 0]
            );
            assert_eq!(
                Into::<[u32; 3]>::into($mask::new(true, false, false)),
                [!0, 0, 0]
            );
            assert_eq!(
                Into::<[u32; 3]>::into($mask::new(false, true, true)),
                [0, !0, !0]
            );
            assert_eq!(
                Into::<[u32; 3]>::into($mask::new(false, true, false)),
                [0, !0, 0]
            );
            assert_eq!(
                Into::<[u32; 3]>::into($mask::new(true, false, true)),
                [!0, 0, !0]
            );
            assert_eq!(
                Into::<[u32; 3]>::into($mask::new(true, true, true)),
                [!0, !0, !0]
            );
        });

        glam_test!(test_mask_into_array_bool, {
            assert_eq!(
                Into::<[bool; 3]>::into($mask::new(false, false, false)),
                [false, false, false]
            );
            assert_eq!(
                Into::<[bool; 3]>::into($mask::new(true, false, false)),
                [true, false, false]
            );
            assert_eq!(
                Into::<[bool; 3]>::into($mask::new(false, true, true)),
                [false, true, true]
            );
            assert_eq!(
                Into::<[bool; 3]>::into($mask::new(false, true, false)),
                [false, true, false]
            );
            assert_eq!(
                Into::<[bool; 3]>::into($mask::new(true, false, true)),
                [true, false, true]
            );
            assert_eq!(
                Into::<[u32; 3]>::into($mask::new(true, true, true)),
                [!0, !0, !0]
            );
        });

        glam_test!(test_mask_splat, {
            assert_eq!($mask::splat(false), $mask::new(false, false, false));
            assert_eq!($mask::splat(true), $mask::new(true, true, true));
        });

        glam_test!(test_mask_bitmask, {
            assert_eq!($mask::new(false, false, false).bitmask(), 0b000);
            assert_eq!($mask::new(true, false, false).bitmask(), 0b001);
            assert_eq!($mask::new(false, true, true).bitmask(), 0b110);
            assert_eq!($mask::new(false, true, false).bitmask(), 0b010);
            assert_eq!($mask::new(true, false, true).bitmask(), 0b101);
            assert_eq!($mask::new(true, true, true).bitmask(), 0b111);
        });

        glam_test!(test_mask_any, {
            assert_eq!($mask::new(false, false, false).any(), false);
            assert_eq!($mask::new(true, false, false).any(), true);
            assert_eq!($mask::new(false, true, false).any(), true);
            assert_eq!($mask::new(false, false, true).any(), true);
        });

        glam_test!(test_mask_all, {
            assert_eq!($mask::new(true, true, true).all(), true);
            assert_eq!($mask::new(false, true, true).all(), false);
            assert_eq!($mask::new(true, false, true).all(), false);
            assert_eq!($mask::new(true, true, false).all(), false);
        });

        glam_test!(test_mask_select, {
            let a = $vec3::new(1 as $t, 2 as $t, 3 as $t);
            let b = $vec3::new(4 as $t, 5 as $t, 6 as $t);
            assert_eq!(
                $vec3::select($mask::new(true, true, true), a, b),
                $vec3::new(1 as $t, 2 as $t, 3 as $t),
            );
            assert_eq!(
                $vec3::select($mask::new(true, false, true), a, b),
                $vec3::new(1 as $t, 5 as $t, 3 as $t),
            );
            assert_eq!(
                $vec3::select($mask::new(false, true, false), a, b),
                $vec3::new(4 as $t, 2 as $t, 6 as $t),
            );
            assert_eq!(
                $vec3::select($mask::new(false, false, false), a, b),
                $vec3::new(4 as $t, 5 as $t, 6 as $t),
            );
        });

        glam_test!(test_mask_and, {
            assert_eq!(
                ($mask::new(false, false, false) & $mask::new(false, false, false)).bitmask(),
                0b000,
            );
            assert_eq!(
                ($mask::new(true, true, true) & $mask::new(true, true, true)).bitmask(),
                0b111,
            );
            assert_eq!(
                ($mask::new(true, false, true) & $mask::new(false, true, false)).bitmask(),
                0b000,
            );
            assert_eq!(
                ($mask::new(true, false, true) & $mask::new(true, true, true)).bitmask(),
                0b101,
            );

            let mut mask = $mask::new(true, true, false);
            mask &= $mask::new(true, false, false);
            assert_eq!(mask.bitmask(), 0b001);
        });

        glam_test!(test_mask_or, {
            assert_eq!(
                ($mask::new(false, false, false) | $mask::new(false, false, false)).bitmask(),
                0b000,
            );
            assert_eq!(
                ($mask::new(true, true, true) | $mask::new(true, true, true)).bitmask(),
                0b111,
            );
            assert_eq!(
                ($mask::new(true, false, true) | $mask::new(false, true, false)).bitmask(),
                0b111,
            );
            assert_eq!(
                ($mask::new(true, false, true) | $mask::new(true, false, true)).bitmask(),
                0b101,
            );

            let mut mask = $mask::new(true, true, false);
            mask |= $mask::new(true, false, false);
            assert_eq!(mask.bitmask(), 0b011);
        });

        glam_test!(test_mask_xor, {
            assert_eq!(
                ($mask::new(false, false, false) ^ $mask::new(false, false, false)).bitmask(),
                0b000,
            );
            assert_eq!(
                ($mask::new(true, true, true) ^ $mask::new(true, true, true)).bitmask(),
                0b000,
            );
            assert_eq!(
                ($mask::new(true, false, true) ^ $mask::new(false, true, false)).bitmask(),
                0b111,
            );
            assert_eq!(
                ($mask::new(true, false, true) ^ $mask::new(true, false, true)).bitmask(),
                0b000,
            );

            let mut mask = $mask::new(true, true, false);
            mask ^= $mask::new(true, false, false);
            assert_eq!(mask.bitmask(), 0b010);
        });

        glam_test!(test_mask_not, {
            assert_eq!((!$mask::new(false, false, false)).bitmask(), 0b111);
            assert_eq!((!$mask::new(true, true, true)).bitmask(), 0b000);
            assert_eq!((!$mask::new(true, false, true)).bitmask(), 0b010);
            assert_eq!((!$mask::new(false, true, false)).bitmask(), 0b101);
        });

        glam_test!(test_mask_fmt, {
            let a = $mask::new(true, false, false);

            // debug fmt
            assert_eq!(
                format!("{:?}", a),
                format!("{}(0xffffffff, 0x0, 0x0)", stringify!($mask))
            );

            // display fmt
            assert_eq!(format!("{}", a), "[true, false, false]");
        });

        glam_test!(test_mask_eq, {
            let a = $mask::new(true, false, true);
            let b = $mask::new(true, false, true);
            let c = $mask::new(false, true, true);

            assert_eq!(a, b);
            assert_eq!(b, a);
            assert_ne!(a, c);
            assert_ne!(b, c);
        });

        glam_test!(test_mask_hash, {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let a = $mask::new(true, false, true);
            let b = $mask::new(true, false, true);
            let c = $mask::new(false, true, true);

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
            let v = $vec3::new(1 as $t, 2 as $t, 3 as $t);
            let mut a = [0 as $t, 0 as $t, 0 as $t];
            v.write_to_slice(&mut a);
            assert_eq!(v, $vec3::from_slice(&a));

            should_panic!({ $vec3::ONE.write_to_slice(&mut [0 as $t; 2]) });
            should_panic!({ $vec3::from_slice(&[0 as $t; 2]) });
        });

        glam_test!(test_sum, {
            let one = $vec3::ONE;
            assert_eq!(vec![one, one].iter().sum::<$vec3>(), one + one);
            assert_eq!(vec![one, one].into_iter().sum::<$vec3>(), one + one);
        });

        glam_test!(test_product, {
            let two = $vec3::new(2 as $t, 2 as $t, 2 as $t);
            assert_eq!(vec![two, two].iter().product::<$vec3>(), two * two);
            assert_eq!(vec![two, two].into_iter().product::<$vec3>(), two * two);
        });
    };
}

macro_rules! impl_vec3_signed_tests {
    ($t:ident, $new:ident, $vec3:ident, $mask:ident) => {
        impl_vec3_tests!($t, $new, $vec3, $mask);

        glam_test!(test_neg, {
            let a = $new(1 as $t, 2 as $t, 3 as $t);
            assert_eq!((-1 as $t, -2 as $t, -3 as $t), (-a).into());
            assert_eq!(
                $new(-0.0 as $t, -0.0 as $t, -0.0 as $t),
                -$new(0.0 as $t, 0.0 as $t, 0.0 as $t)
            );
            assert_eq!(
                $new(0.0 as $t, -0.0 as $t, -0.0 as $t),
                -$new(-0.0 as $t, 0.0 as $t, 0.0 as $t)
            );
        });

        glam_test!(test_dot_signed, {
            let x = $new(1 as $t, 0 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t, 0 as $t);
            let z = $new(0 as $t, 0 as $t, 1 as $t);
            assert_eq!(1 as $t, x.dot(x));
            assert_eq!(0 as $t, x.dot(y));
            assert_eq!(-1 as $t, z.dot(-z));
        });
    };
}

macro_rules! impl_vec3_eq_hash_tests {
    ($t:ident, $new:ident) => {
        glam_test!(test_vec3_hash, {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let a = $new(1 as $t, 2 as $t, 3 as $t);
            let b = $new(1 as $t, 2 as $t, 3 as $t);
            let c = $new(3 as $t, 2 as $t, 1 as $t);

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

macro_rules! impl_vec3_float_tests {
    ($t:ident, $new:ident, $vec3:ident, $mask:ident) => {
        impl_vec3_signed_tests!($t, $new, $vec3, $mask);
        impl_vec_float_normalize_tests!($t, $vec3);

        use core::$t::INFINITY;
        use core::$t::NAN;
        use core::$t::NEG_INFINITY;

        glam_test!(test_nan, {
            assert!($vec3::NAN.is_nan());
            assert!(!$vec3::NAN.is_finite());
        });

        glam_test!(test_funcs, {
            let x = $new(1.0, 0.0, 0.0);
            let y = $new(0.0, 1.0, 0.0);
            let z = $new(0.0, 0.0, 1.0);
            assert_eq!(y, z.cross(x));
            assert_eq!(z, x.cross(y));
            assert_eq!(4.0, (2.0 * x).length_squared());
            assert_eq!(9.0, (-3.0 * y).length_squared());
            assert_eq!(16.0, (4.0 * z).length_squared());
            assert_eq!(2.0, (-2.0 * x).length());
            assert_eq!(3.0, (3.0 * y).length());
            assert_eq!(4.0, (-4.0 * z).length());
            assert_eq!(2.0, x.distance_squared(y));
            assert_eq!(13.0, (2.0 * x).distance_squared(-3.0 * z));
            assert_eq!((2.0 as $t).sqrt(), x.distance(y));
            assert_eq!(5.0, (3.0 * x).distance(-4.0 * y));
            assert_eq!(13.0, (-5.0 * z).distance(12.0 * y));
            assert_eq!(x, (2.0 * x).normalize());
            assert_eq!(
                1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0,
                $new(1.0, 2.0, 3.0).dot($new(4.0, 5.0, 6.0))
            );
            assert_eq!(
                $new(14.0, 14.0, 14.0),
                $new(0.0, 4.0, 6.0).dot_into_vec($new(3.0, 2.0, 1.0))
            );
            assert_eq!(
                2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0,
                $new(2.0, 3.0, 4.0).length_squared()
            );
            assert_eq!(
                (2.0 as $t * 2.0 + 3.0 * 3.0 + 4.0 * 4.0).sqrt(),
                $new(2.0, 3.0, 4.0).length()
            );
            assert_eq!(
                1.0 / (2.0 as $t * 2.0 + 3.0 * 3.0 + 4.0 * 4.0).sqrt(),
                $new(2.0, 3.0, 4.0).length_recip()
            );
            assert!($new(2.0, 3.0, 4.0).normalize().is_normalized());
            assert_approx_eq!(
                $new(2.0, 3.0, 4.0) / (2.0 as $t * 2.0 + 3.0 * 3.0 + 4.0 * 4.0).sqrt(),
                $new(2.0, 3.0, 4.0).normalize()
            );
            assert_eq!($new(0.5, 0.25, 0.125), $new(2.0, 4.0, 8.0).recip());
        });

        glam_test!(test_project_reject, {
            assert_eq!(
                $new(0.0, 0.0, 1.0),
                $new(1.0, 0.0, 1.0).project_onto($new(0.0, 0.0, 2.0))
            );
            assert_eq!(
                $new(1.0, 0.0, 0.0),
                $new(1.0, 0.0, 1.0).reject_from($new(0.0, 0.0, 2.0))
            );
            assert_eq!(
                $new(0.0, 0.0, 1.0),
                $new(1.0, 0.0, 1.0).project_onto_normalized($new(0.0, 0.0, 1.0))
            );
            assert_eq!(
                $new(1.0, 0.0, 0.0),
                $new(1.0, 0.0, 1.0).reject_from_normalized($new(0.0, 0.0, 1.0))
            );
            should_glam_assert!({ $vec3::ONE.project_onto($vec3::ZERO) });
            should_glam_assert!({ $vec3::ONE.reject_from($vec3::ZERO) });
            should_glam_assert!({ $vec3::ONE.project_onto_normalized($vec3::ONE) });
            should_glam_assert!({ $vec3::ONE.reject_from_normalized($vec3::ONE) });
        });

        glam_test!(test_signum, {
            assert_eq!($vec3::ZERO.signum(), $vec3::ONE);
            assert_eq!((-$vec3::ZERO).signum(), -$vec3::ONE);
            assert_eq!($vec3::ONE.signum(), $vec3::ONE);
            assert_eq!((-$vec3::ONE).signum(), -$vec3::ONE);
            assert_eq!($vec3::splat(INFINITY).signum(), $vec3::ONE);
            assert_eq!($vec3::splat(NEG_INFINITY).signum(), -$vec3::ONE);
            assert!($vec3::splat(NAN).signum().is_nan_mask().all());
        });

        glam_test!(test_is_negative_bitmask, {
            assert_eq!($vec3::ZERO.is_negative_bitmask(), 0b000);
            assert_eq!((-$vec3::ZERO).is_negative_bitmask(), 0b111);
            assert_eq!($vec3::ONE.is_negative_bitmask(), 0b000);
            assert_eq!((-$vec3::ONE).is_negative_bitmask(), 0b111);
            assert_eq!($vec3::new(-0.1, 0.2, 0.3).is_negative_bitmask(), 0b001);
            assert_eq!($vec3::new(0.8, 0.3, 0.1).is_negative_bitmask(), 0b000);
            assert_eq!($vec3::new(0.1, 0.5, -0.3).is_negative_bitmask(), 0b100);
            assert_eq!($vec3::new(0.3, -0.4, 0.1).is_negative_bitmask(), 0b010);
            assert_eq!($vec3::new(-0.2, 0.6, -0.5).is_negative_bitmask(), 0b101);
        });

        glam_test!(test_abs, {
            assert_eq!($vec3::ZERO.abs(), $vec3::ZERO);
            assert_eq!($vec3::ONE.abs(), $vec3::ONE);
            assert_eq!((-$vec3::ONE).abs(), $vec3::ONE);
        });

        glam_test!(test_round, {
            assert_eq!($vec3::new(1.35, 0.0, 0.0).round().x, 1.0);
            assert_eq!($vec3::new(0.0, 1.5, 0.0).round().y, 2.0);
            assert_eq!($vec3::new(0.0, 0.0, -15.5).round().z, -16.0);
            assert_eq!($vec3::new(0.0, 0.0, 0.0).round().z, 0.0);
            assert_eq!($vec3::new(0.0, 21.1, 0.0).round().y, 21.0);
            assert_eq!($vec3::new(0.0, 11.123, 0.0).round().y, 11.0);
            assert_eq!($vec3::new(0.0, 11.499, 0.0).round().y, 11.0);
            assert_eq!(
                $vec3::new(NEG_INFINITY, INFINITY, 0.0).round(),
                $vec3::new(NEG_INFINITY, INFINITY, 0.0)
            );
            assert!($vec3::new(NAN, 0.0, 0.0).round().x.is_nan());
        });

        glam_test!(test_floor, {
            assert_eq!(
                $vec3::new(1.35, 1.5, -1.5).floor(),
                $vec3::new(1.0, 1.0, -2.0)
            );
            assert_eq!(
                $vec3::new(INFINITY, NEG_INFINITY, 0.0).floor(),
                $vec3::new(INFINITY, NEG_INFINITY, 0.0)
            );
            assert!($vec3::new(NAN, 0.0, 0.0).floor().x.is_nan());
            assert_eq!(
                $vec3::new(-2000000.123, 10000000.123, 1000.9).floor(),
                $vec3::new(-2000001.0, 10000000.0, 1000.0)
            );
        });

        glam_test!(test_fract, {
            assert_approx_eq!(
                $vec3::new(1.35, 1.5, -1.5).fract(),
                $vec3::new(0.35, 0.5, 0.5)
            );
            assert_approx_eq!(
                $vec3::new(-200000.123, 1000000.123, 1000.9).fract(),
                $vec3::new(0.877, 0.123, 0.9),
                0.002
            );
        });

        glam_test!(test_ceil, {
            assert_eq!(
                $vec3::new(1.35, 1.5, -1.5).ceil(),
                $vec3::new(2.0, 2.0, -1.0)
            );
            assert_eq!(
                $vec3::new(INFINITY, NEG_INFINITY, 0.0).ceil(),
                $vec3::new(INFINITY, NEG_INFINITY, 0.0)
            );
            assert!($vec3::new(NAN, 0.0, 0.0).ceil().x.is_nan());
            assert_eq!(
                $vec3::new(-2000000.123, 1000000.123, 1000.9).ceil(),
                $vec3::new(-2000000.0, 1000001.0, 1001.0)
            );
        });

        glam_test!(test_lerp, {
            let v0 = $vec3::new(-1.0, -1.0, -1.0);
            let v1 = $vec3::new(1.0, 1.0, 1.0);
            assert_approx_eq!(v0, v0.lerp(v1, 0.0));
            assert_approx_eq!(v1, v0.lerp(v1, 1.0));
            assert_approx_eq!($vec3::ZERO, v0.lerp(v1, 0.5));
        });

        glam_test!(test_is_finite, {
            assert!($vec3::new(0.0, 0.0, 0.0).is_finite());
            assert!($vec3::new(-1e-10, 1.0, 1e10).is_finite());
            assert!(!$vec3::new(INFINITY, 0.0, 0.0).is_finite());
            assert!(!$vec3::new(0.0, NAN, 0.0).is_finite());
            assert!(!$vec3::new(0.0, 0.0, NEG_INFINITY).is_finite());
            assert!(!$vec3::splat(NAN).is_finite());
        });

        glam_test!(test_powf, {
            assert_eq!(
                $vec3::new(2.0, 4.0, 8.0).powf(2.0),
                $vec3::new(4.0, 16.0, 64.0)
            );
        });

        glam_test!(test_exp, {
            assert_approx_eq!(
                $vec3::new(1.0, 2.0, 3.0).exp(),
                $vec3::new((1.0 as $t).exp(), (2.0 as $t).exp(), (3.0 as $t).exp())
            );
        });

        glam_test!(test_angle_between, {
            let angle = $vec3::new(1.0, 0.0, 1.0).angle_between($vec3::new(1.0, 1.0, 0.0));
            assert_approx_eq!(core::$t::consts::FRAC_PI_3, angle, 1e-6);

            let angle = $vec3::new(10.0, 0.0, 10.0).angle_between($vec3::new(5.0, 5.0, 0.0));
            assert_approx_eq!(core::$t::consts::FRAC_PI_3, angle, 1e-6);

            let angle = $vec3::new(-1.0, 0.0, -1.0).angle_between($vec3::new(1.0, -1.0, 0.0));
            assert_approx_eq!(2.0 * core::$t::consts::FRAC_PI_3, angle, 1e-6);
        });

        glam_test!(test_clamp_length, {
            // Too long gets shortened
            assert_eq!(
                $vec3::new(12.0, 16.0, 0.0).clamp_length(7.0, 10.0),
                $vec3::new(6.0, 8.0, 0.0) // shortened to length 10.0
            );
            // In the middle is unchanged
            assert_eq!(
                $vec3::new(2.0, 1.0, 0.0).clamp_length(0.5, 5.0),
                $vec3::new(2.0, 1.0, 0.0) // unchanged
            );
            // Too short gets lengthened
            assert_eq!(
                $vec3::new(0.6, 0.8, 0.0).clamp_length(10.0, 20.0),
                $vec3::new(6.0, 8.0, 0.0) // lengthened to length 10.0
            );
            should_glam_assert!({ $vec3::ONE.clamp_length(1.0, 0.0) });
        });

        glam_test!(test_clamp_length_max, {
            // Too long gets shortened
            assert_eq!(
                $vec3::new(12.0, 16.0, 0.0).clamp_length_max(10.0),
                $vec3::new(6.0, 8.0, 0.0) // shortened to length 10.0
            );
            // Not too long is unchanged
            assert_eq!(
                $vec3::new(2.0, 1.0, 0.0).clamp_length_max(5.0),
                $vec3::new(2.0, 1.0, 0.0) // unchanged
            );
        });

        glam_test!(test_clamp_length_min, {
            // Not too short is unchanged
            assert_eq!(
                $vec3::new(2.0, 1.0, 0.0).clamp_length_min(0.5),
                $vec3::new(2.0, 1.0, 0.0) // unchanged
            );
            // Too short gets lengthened
            assert_eq!(
                $vec3::new(0.6, 0.8, 0.0).clamp_length_min(10.0),
                $vec3::new(6.0, 8.0, 0.0) // lengthened to length 10.0
            );
        });

        glam_test!(test_any_ortho, {
            let eps = 2.0 * core::$t::EPSILON;

            for &v in &vec3_float_test_vectors!($vec3) {
                let orthogonal = v.any_orthogonal_vector();
                assert!(orthogonal != $vec3::ZERO && orthogonal.is_finite());
                assert!(v.dot(orthogonal).abs() < eps);

                let n = v.normalize();

                let orthonormal = n.any_orthonormal_vector();
                assert!(orthonormal.is_normalized());
                assert!(n.dot(orthonormal).abs() < eps);

                let (a, b) = n.any_orthonormal_pair();
                assert!(a.is_normalized() && n.dot(a).abs() < eps);
                assert!(b.is_normalized() && n.dot(b).abs() < eps);
            }
        });

        glam_test!(test_mul_add, {
            assert_eq!(
                $vec3::new(1.0, 1.0, 1.0)
                    .mul_add($vec3::new(0.5, 2.0, -4.0), $vec3::new(-1.0, -1.0, -1.0)),
                $vec3::new(-0.5, 1.0, -5.0)
            );
        });
    };
}

macro_rules! impl_vec3_scalar_shift_op_test {
    ($vec3:ident, $t_min:literal, $t_max:literal, $rhs_min:literal, $rhs_max:literal) => {
        glam_test!(test_vec3_scalar_shift_ops, {
            for x in $t_min..$t_max {
                for y in $t_min..$t_max {
                    for z in $t_min..$t_max {
                        for rhs in $rhs_min..$rhs_max {
                            assert_eq!(
                                $vec3::new(x, y, z) << rhs,
                                $vec3::new(x << rhs, y << rhs, z << rhs)
                            );
                            assert_eq!(
                                $vec3::new(x, y, z) >> rhs,
                                $vec3::new(x >> rhs, y >> rhs, z >> rhs)
                            );
                        }
                    }
                }
            }
        });
    };
}

macro_rules! impl_vec3_scalar_shift_op_tests {
    ($vec3:ident, $t_min:literal, $t_max:literal) => {
        mod shift_by_i8 {
            use glam::$vec3;
            impl_vec3_scalar_shift_op_test!($vec3, $t_min, $t_max, 0i8, 2);
        }
        mod shift_by_i16 {
            use glam::$vec3;
            impl_vec3_scalar_shift_op_test!($vec3, $t_min, $t_max, 0i16, 2);
        }
        mod shift_by_i32 {
            use glam::$vec3;
            impl_vec3_scalar_shift_op_test!($vec3, $t_min, $t_max, 0i32, 2);
        }
        mod shift_by_u8 {
            use glam::$vec3;
            impl_vec3_scalar_shift_op_test!($vec3, $t_min, $t_max, 0u8, 2);
        }
        mod shift_by_u16 {
            use glam::$vec3;
            impl_vec3_scalar_shift_op_test!($vec3, $t_min, $t_max, 0u16, 2);
        }
        mod shift_by_u32 {
            use glam::$vec3;
            impl_vec3_scalar_shift_op_test!($vec3, $t_min, $t_max, 0u32, 2);
        }
    };
}

macro_rules! impl_vec3_shift_op_test {
    ($vec3:ident, $rhs:ident, $t_min:literal, $t_max:literal) => {
        glam_test!(test_vec3_shift_ops, {
            for x1 in $t_min..$t_max {
                for y1 in $t_min..$t_max {
                    for z1 in $t_min..$t_max {
                        for x2 in $t_min..$t_max {
                            for y2 in $t_min..$t_max {
                                for z2 in $t_min..$t_max {
                                    assert_eq!(
                                        $vec3::new(x1, y1, z1) << $rhs::new(x2, y2, z2),
                                        $vec3::new(x1 << x2, y1 << y2, z1 << z2)
                                    );
                                    assert_eq!(
                                        $vec3::new(x1, y1, z1) >> $rhs::new(x2, y2, z2),
                                        $vec3::new(x1 >> x2, y1 >> y2, z1 >> z2)
                                    );
                                }
                            }
                        }
                    }
                }
            }
        });
    };
}

macro_rules! impl_vec3_shift_op_tests {
    ($vec3:ident) => {
        mod shift_ivec3_by_ivec3 {
            use super::*;
            impl_vec3_shift_op_test!($vec3, IVec3, 0, 2);
        }
        mod shift_ivec3_by_uvec3 {
            use super::*;
            impl_vec3_shift_op_test!($vec3, UVec3, 0, 2);
        }
    };
}

macro_rules! impl_vec3_scalar_bit_op_tests {
    ($vec3:ident, $t_min:literal, $t_max:literal) => {
        glam_test!(test_vec3_scalar_bit_ops, {
            for x in $t_min..$t_max {
                for y in $t_min..$t_max {
                    for z in $t_min..$t_max {
                        for rhs in $t_min..$t_max {
                            assert_eq!(
                                $vec3::new(x, y, z) & rhs,
                                $vec3::new(x & rhs, y & rhs, z & rhs)
                            );
                            assert_eq!(
                                $vec3::new(x, y, z) | rhs,
                                $vec3::new(x | rhs, y | rhs, z | rhs)
                            );
                            assert_eq!(
                                $vec3::new(x, y, z) ^ rhs,
                                $vec3::new(x ^ rhs, y ^ rhs, z ^ rhs)
                            );
                        }
                    }
                }
            }
        });
    };
}

macro_rules! impl_vec3_bit_op_tests {
    ($vec3:ident, $t_min:literal, $t_max:literal) => {
        glam_test!(test_vec3_bit_ops, {
            for x1 in $t_min..$t_max {
                for y1 in $t_min..$t_max {
                    for z1 in $t_min..$t_max {
                        assert_eq!(!$vec3::new(x1, y1, z1), $vec3::new(!x1, !y1, !z1));

                        for x2 in $t_min..$t_max {
                            for y2 in $t_min..$t_max {
                                for z2 in $t_min..$t_max {
                                    assert_eq!(
                                        $vec3::new(x1, y1, z1) & $vec3::new(x2, y2, z2),
                                        $vec3::new(x1 & x2, y1 & y2, z1 & z2)
                                    );
                                    assert_eq!(
                                        $vec3::new(x1, y1, z1) | $vec3::new(x2, y2, z2),
                                        $vec3::new(x1 | x2, y1 | y2, z1 | z2)
                                    );
                                    assert_eq!(
                                        $vec3::new(x1, y1, z1) ^ $vec3::new(x2, y2, z2),
                                        $vec3::new(x1 ^ x2, y1 ^ y2, z1 ^ z2)
                                    );
                                }
                            }
                        }
                    }
                }
            }
        });
    };
}

mod vec3 {
    use glam::{vec3, BVec3, Vec3};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(12, mem::size_of::<Vec3>());
        assert_eq!(4, mem::align_of::<Vec3>());
        assert_eq!(3, mem::size_of::<BVec3>());
        assert_eq!(1, mem::align_of::<BVec3>());
    });

    glam_test!(test_as, {
        use glam::{DVec3, IVec3, UVec3, Vec3A};
        assert_eq!(
            DVec3::new(-1.0, -2.0, -3.0),
            Vec3::new(-1.0, -2.0, -3.0).as_dvec3()
        );
        assert_eq!(
            IVec3::new(-1, -2, -3),
            Vec3::new(-1.0, -2.0, -3.0).as_ivec3()
        );
        assert_eq!(UVec3::new(1, 2, 3), Vec3::new(1.0, 2.0, 3.0).as_uvec3());

        assert_eq!(
            DVec3::new(-1.0, -2.0, -3.0),
            Vec3A::new(-1.0, -2.0, -3.0).as_dvec3()
        );
        assert_eq!(
            IVec3::new(-1, -2, -3),
            Vec3A::new(-1.0, -2.0, -3.0).as_ivec3()
        );
        assert_eq!(UVec3::new(1, 2, 3), Vec3A::new(1.0, 2.0, 3.0).as_uvec3());

        assert_eq!(
            IVec3::new(-1, -2, -3),
            DVec3::new(-1.0, -2.0, -3.0).as_ivec3()
        );
        assert_eq!(UVec3::new(1, 2, 3), DVec3::new(1.0, 2.0, 3.0).as_uvec3());
        assert_eq!(
            Vec3::new(-1.0, -2.0, -3.0),
            DVec3::new(-1.0, -2.0, -3.0).as_vec3()
        );
        assert_eq!(
            Vec3A::new(-1.0, -2.0, -3.0),
            DVec3::new(-1.0, -2.0, -3.0).as_vec3a()
        );

        assert_eq!(
            DVec3::new(-1.0, -2.0, -3.0),
            IVec3::new(-1, -2, -3).as_dvec3()
        );
        assert_eq!(UVec3::new(1, 2, 3), IVec3::new(1, 2, 3).as_uvec3());
        assert_eq!(
            Vec3::new(-1.0, -2.0, -3.0),
            IVec3::new(-1, -2, -3).as_vec3()
        );
        assert_eq!(
            Vec3A::new(-1.0, -2.0, -3.0),
            IVec3::new(-1, -2, -3).as_vec3a()
        );

        assert_eq!(DVec3::new(1.0, 2.0, 3.0), UVec3::new(1, 2, 3).as_dvec3());
        assert_eq!(IVec3::new(1, 2, 3), UVec3::new(1, 2, 3).as_ivec3());
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), UVec3::new(1, 2, 3).as_vec3());
        assert_eq!(Vec3A::new(1.0, 2.0, 3.0), UVec3::new(1, 2, 3).as_vec3a());
    });

    impl_vec3_float_tests!(f32, vec3, Vec3, BVec3);
}

mod vec3a {
    #[cfg(any(
        not(any(target_feature = "sse2", target_feature = "simd128")),
        feature = "scalar-math"
    ))]
    use glam::BVec3;
    #[cfg(not(feature = "scalar-math"))]
    use glam::BVec3A;
    use glam::{vec3a, Vec3A, Vec4};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(16, mem::size_of::<Vec3A>());
        assert_eq!(16, mem::align_of::<Vec3A>());
        #[cfg(not(feature = "scalar-math"))]
        {
            assert_eq!(16, mem::size_of::<BVec3A>());
            assert_eq!(16, mem::align_of::<BVec3A>());
        }
        #[cfg(feature = "scalar-math")]
        {
            assert_eq!(3, mem::size_of::<BVec3>());
            assert_eq!(1, mem::align_of::<BVec3>());
        }
    });

    glam_test!(test_mask_align16, {
        // make sure the unused 'w' value doesn't break Vec3Ab behaviour
        let a = Vec4::ZERO;
        let mut b = Vec3A::from(a);
        b.x = 1.0;
        b.y = 1.0;
        b.z = 1.0;
        assert!(!b.cmpeq(Vec3A::ZERO).any());
        assert!(b.cmpeq(Vec3A::splat(1.0)).all());
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
        struct F32x3_A16([f32; 3]);

        let v0 = Vec3A::new(1.0, 2.0, 3.0);
        let m0: __m128 = v0.into();
        let mut a0 = F32x3_A16([0.0, 0.0, 0.0]);
        unsafe {
            _mm_store_ps(a0.0.as_mut_ptr(), m0);
        }
        assert_eq!([1.0, 2.0, 3.0], a0.0);
        let v1 = Vec3A::from(m0);
        assert_eq!(v0, v1);

        #[repr(C, align(16))]
        struct U32x3_A16([u32; 3]);

        let v0 = BVec3A::new(true, false, true);
        let m0: __m128 = v0.into();
        let mut a0 = U32x3_A16([1, 2, 3]);
        unsafe {
            _mm_store_ps(a0.0.as_mut_ptr() as *mut f32, m0);
        }
        assert_eq!([0xffffffff, 0, 0xffffffff], a0.0);
    }

    glam_test!(test_min_max_from_vec4, {
        // checks that the 4th element is unused.
        let v1 = Vec3A::from(Vec4::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(v1.max_element(), 3.0);
        let v2 = Vec3A::from(Vec4::new(4.0, 3.0, 2.0, 1.0));
        assert_eq!(v2.min_element(), 2.0);
    });

    #[cfg(all(
        any(target_feature = "sse2", target_feature = "simd128"),
        not(feature = "scalar-math")
    ))]
    impl_vec3_float_tests!(f32, vec3a, Vec3A, BVec3A);

    #[cfg(any(
        not(any(target_feature = "sse2", target_feature = "simd128")),
        feature = "scalar-math"
    ))]
    impl_vec3_float_tests!(f32, vec3a, Vec3A, BVec3);
}

mod dvec3 {
    use glam::{dvec3, BVec3, DVec3};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(24, mem::size_of::<DVec3>());
        assert_eq!(mem::align_of::<f64>(), mem::align_of::<DVec3>());
        assert_eq!(3, mem::size_of::<BVec3>());
        assert_eq!(1, mem::align_of::<BVec3>());
    });

    impl_vec3_float_tests!(f64, dvec3, DVec3, BVec3);
}

mod ivec3 {
    use glam::{ivec3, BVec3, IVec3, UVec3};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(12, mem::size_of::<IVec3>());
        assert_eq!(4, mem::align_of::<IVec3>());
        assert_eq!(3, mem::size_of::<BVec3>());
        assert_eq!(1, mem::align_of::<BVec3>());
    });

    impl_vec3_signed_tests!(i32, ivec3, IVec3, BVec3);
    impl_vec3_eq_hash_tests!(i32, ivec3);

    impl_vec3_scalar_shift_op_tests!(IVec3, -2, 2);
    impl_vec3_shift_op_tests!(IVec3);

    impl_vec3_scalar_bit_op_tests!(IVec3, -2, 2);
    impl_vec3_bit_op_tests!(IVec3, -2, 2);
}

mod uvec3 {
    use glam::{uvec3, BVec3, IVec3, UVec3};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(12, mem::size_of::<UVec3>());
        assert_eq!(4, mem::align_of::<UVec3>());
        assert_eq!(3, mem::size_of::<BVec3>());
        assert_eq!(1, mem::align_of::<BVec3>());
    });

    impl_vec3_tests!(u32, uvec3, UVec3, BVec3);
    impl_vec3_eq_hash_tests!(u32, uvec3);

    impl_vec3_scalar_shift_op_tests!(UVec3, 0, 2);
    impl_vec3_shift_op_tests!(UVec3);

    impl_vec3_scalar_bit_op_tests!(UVec3, 0, 2);
    impl_vec3_bit_op_tests!(UVec3, 0, 2);
}
