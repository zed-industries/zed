#![allow(deprecated)]

#[cfg(feature = "transform-types")]
#[macro_use]
mod support;

#[cfg(feature = "transform-types")]
mod transform {
    use crate::support::FloatCompare;
    use glam::*;

    impl FloatCompare for TransformSRT {
        #[inline]
        fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
            self.abs_diff_eq(*other, max_abs_diff)
        }

        #[inline]
        fn abs_diff(&self, other: &Self) -> Self {
            Self::from_scale_rotation_translation(
                self.scale.abs_diff(&other.scale),
                self.rotation.abs_diff(&other.rotation),
                self.translation.abs_diff(&other.translation),
            )
        }
    }

    impl FloatCompare for TransformRT {
        #[inline]
        fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
            self.abs_diff_eq(*other, max_abs_diff)
        }

        #[inline]
        fn abs_diff(&self, other: &Self) -> Self {
            Self::from_rotation_translation(
                self.rotation.abs_diff(&other.rotation),
                self.translation.abs_diff(&other.translation),
            )
        }
    }

    #[test]
    fn test_identity() {
        let tr = TransformRT::IDENTITY;
        assert_eq!(tr.rotation, Quat::IDENTITY);
        assert_eq!(tr.translation, Vec3::ZERO);

        let srt = TransformSRT::IDENTITY;
        assert_eq!(srt.scale, Vec3::ONE);
        assert_eq!(srt.rotation, Quat::IDENTITY);
        assert_eq!(srt.translation, Vec3::ZERO);

        assert_eq!(srt, tr.into());

        assert_eq!(TransformRT::IDENTITY, TransformRT::default());
        assert_eq!(TransformSRT::IDENTITY, TransformSRT::default());
    }

    #[test]
    fn test_nan() {
        assert!(TransformRT::NAN.is_nan());
        assert!(!TransformRT::NAN.is_finite());

        assert!(TransformSRT::NAN.is_nan());
        assert!(!TransformSRT::NAN.is_finite());
    }

    #[test]
    fn test_new() {
        let t = Vec3::new(1.0, 2.0, 3.0);
        let r = Quat::from_rotation_y(90.0_f32.to_radians());
        let s = Vec3::new(-1.0, -2.0, -3.0);

        let tr = TransformRT::from_rotation_translation(r, t);
        assert_eq!(tr.rotation, r);
        assert_eq!(tr.translation, t);

        let srt = TransformSRT::from_scale_rotation_translation(s, r, t);
        assert_eq!(srt.scale, s);
        assert_eq!(srt.rotation, r);
        assert_eq!(srt.translation, t);

        assert_eq!(tr, tr);
        assert_eq!(srt, srt);
    }

    #[test]
    fn test_mul() {
        let tr = TransformRT::from_rotation_translation(
            Quat::from_rotation_z(-90.0_f32.to_radians()),
            Vec3::X,
        );
        let v0 = Vec3A::Y;
        let v1 = tr.transform_point3a(v0);
        assert_approx_eq!(v1, Vec3A::X * 2.0);
        assert_approx_eq!(v1, tr.transform_point3a(v0));
        let inv_tr = tr.inverse();
        let v2 = inv_tr.transform_point3a(v1);
        assert_approx_eq!(v0, v2);

        assert_eq!(tr * TransformRT::IDENTITY, tr);
        assert_approx_eq!(tr * inv_tr, TransformRT::IDENTITY);

        assert_eq!(tr * TransformSRT::IDENTITY, TransformSRT::from(tr));
        assert_eq!(TransformSRT::IDENTITY * tr, TransformSRT::from(tr));

        let s = Vec3::splat(2.0);
        let r = Quat::from_rotation_y(180.0_f32.to_radians());
        let t = -Vec3::Y;
        let srt = TransformSRT::from_scale_rotation_translation(s, r, t);
        let v0 = Vec3A::X;
        let v1 = srt.transform_point3a(v0);
        assert_approx_eq!(v1, (r * (v0 * Vec3A::from(s))) + Vec3A::from(t));
        assert_approx_eq!(v1, srt.transform_point3a(v0));
        let inv_srt = srt.inverse();
        let v2 = inv_srt.transform_point3a(v1);
        assert_approx_eq!(v0, v2);

        assert_eq!(srt * TransformSRT::IDENTITY, srt);
        assert_eq!(srt * inv_srt, TransformSRT::IDENTITY);

        // negative scale mul test
        let s = Vec3::splat(-2.0);
        let srt = TransformSRT::from_scale_rotation_translation(s, r, t);
        let inv_srt = srt.inverse();
        assert_eq!(srt * inv_srt, TransformSRT::IDENTITY);
    }
}
