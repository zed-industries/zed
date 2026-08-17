#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        #[allow(unused_imports)]
        use crate::support::FloatCompare;
        let eps = core::f32::EPSILON;
        let (a, b) = (&$a, &$b);
        assert!(
            a.approx_eq(b, eps),
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            a.abs_diff(b)
        );
    }};
    ($a:expr, $b:expr, $eps:expr) => {{
        use crate::support::FloatCompare;
        let (a, b) = (&$a, &$b);
        let eps = $eps;
        assert!(
            a.approx_eq(b, $eps),
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            a.abs_diff(b)
        );
    }};
}

/// Test vector normalization for float vector
#[macro_export]
macro_rules! impl_vec_float_normalize_tests {
    ($t:ident, $vec:ident) => {
        use core::$t::MAX;
        use core::$t::MIN_POSITIVE;

        /// Works for vec2, vec3, vec4
        fn from_x_y(x: $t, y: $t) -> $vec {
            let mut v = $vec::ZERO;
            v.x = x;
            v.y = y;
            v
        }

        #[test]
        fn test_normalize() {
            assert_eq!(from_x_y(-42.0, 0.0).normalize(), from_x_y(-1.0, 0.0));
            assert_eq!(from_x_y(MAX.sqrt(), 0.0).normalize(), from_x_y(1.0, 0.0));
            // assert_eq!(from_x_y(MAX, 0.0).normalize(), from_x_y(1.0, 0.0)); // normalize fails for huge vectors and returns zero

            // We expect not to be able to normalize small numbers:
            assert!(!from_x_y(0.0, 0.0).normalize().is_finite());
            assert!(!from_x_y(MIN_POSITIVE, 0.0).normalize().is_finite());

            // We expect not to be able to normalize non-finite vectors:
            assert!(!from_x_y(INFINITY, 0.0).normalize().is_finite());
            assert!(!from_x_y(NAN, 0.0).normalize().is_finite());
        }

        #[test]
        fn test_try_normalize() {
            assert_eq!(
                from_x_y(-42.0, 0.0).try_normalize(),
                Some(from_x_y(-1.0, 0.0))
            );
            assert_eq!(
                from_x_y(MAX.sqrt(), 0.0).try_normalize(),
                Some(from_x_y(1.0, 0.0))
            );

            // We expect `try_normalize` to return None when inputs are very small:
            assert_eq!(from_x_y(0.0, 0.0).try_normalize(), None);
            assert_eq!(from_x_y(MIN_POSITIVE, 0.0).try_normalize(), None);

            // We expect `try_normalize` to return None when inputs are non-finite:
            assert_eq!(from_x_y(INFINITY, 0.0).try_normalize(), None);
            assert_eq!(from_x_y(NAN, 0.0).try_normalize(), None);

            // We expect `try_normalize` to return None when inputs are very large:
            assert_eq!(from_x_y(MAX, 0.0).try_normalize(), None);
            assert_eq!(from_x_y(MAX, MAX).try_normalize(), None);
        }

        #[test]
        fn test_normalize_or_zero() {
            assert_eq!(
                from_x_y(-42.0, 0.0).normalize_or_zero(),
                from_x_y(-1.0, 0.0)
            );
            assert_eq!(
                from_x_y(MAX.sqrt(), 0.0).normalize_or_zero(),
                from_x_y(1.0, 0.0)
            );

            // We expect `normalize_or_zero` to return zero when inputs are very small:
            assert_eq!(from_x_y(0.0, 0.0).normalize_or_zero(), $vec::ZERO);
            assert_eq!(from_x_y(MIN_POSITIVE, 0.0).normalize_or_zero(), $vec::ZERO);

            // We expect `normalize_or_zero` to return zero when inputs are non-finite:
            assert_eq!(from_x_y(INFINITY, 0.0).normalize_or_zero(), $vec::ZERO);
            assert_eq!(from_x_y(NAN, 0.0).normalize_or_zero(), $vec::ZERO);

            // We expect `normalize_or_zero` to return zero when inputs are very large:
            assert_eq!(from_x_y(MAX, 0.0).normalize_or_zero(), $vec::ZERO);
            assert_eq!(from_x_y(MAX, MAX).normalize_or_zero(), $vec::ZERO);
        }
    };
}

/// Useful test vectors
#[macro_export]
macro_rules! vec3_float_test_vectors {
    ($vec3:ident) => {
        [
            $vec3::X,
            $vec3::Y,
            $vec3::Z,
            -$vec3::X,
            -$vec3::Y,
            -$vec3::Z,
            $vec3::new(1.0, 1e-3, 0.0),
            $vec3::new(1.0, 1e-4, 0.0),
            $vec3::new(1.0, 1e-5, 0.0),
            $vec3::new(1.0, 1e-6, 0.0),
            $vec3::new(1.0, 1e-7, 0.0),
            $vec3::new(1.0, 1e-14, 0.0),
            $vec3::new(1.0, 1e-15, 0.0),
            $vec3::new(1.0, 1e-16, 0.0),
            $vec3::new(0.1, 0.2, 0.3),
            $vec3::new(0.2, 0.3, 0.4),
            $vec3::new(4.0, -5.0, 6.0),
            $vec3::new(-2.0, 0.5, -1.0),
            // Pathalogical cases from <https://graphics.pixar.com/library/OrthonormalB/paper.pdf>:
            $vec3::new(0.00038527316, 0.00038460016, -0.99999988079),
            $vec3::new(-0.00019813581, -0.00008946839, -0.99999988079),
        ]
    };
}
