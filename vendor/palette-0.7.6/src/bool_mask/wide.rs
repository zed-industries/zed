use wide::{f32x4, f32x8, f64x2, f64x4};

use super::{BoolMask, HasBoolMask, LazySelect, Select};

macro_rules! impl_wide_bool_mask {
    ($($ty: ident: ($scalar: ident, $uint: ident)),+) => {
        $(
            impl BoolMask for $ty {
                #[inline]
                fn from_bool(value: bool) -> Self {
                    $ty::splat(if value { $scalar::from_bits($uint::MAX) } else { 0.0 })
                }

                #[inline]
                fn is_true(&self) -> bool {
                    self.all()
                }

                #[inline]
                fn is_false(&self) -> bool {
                    self.none()
                }
            }

            impl HasBoolMask for $ty {
                type Mask = Self;
            }

            impl Select<Self> for $ty {
                #[inline]
                fn select(self, a: Self, b: Self) -> Self {
                    self.blend(a, b)
                }
            }

            impl LazySelect<Self> for $ty {
                #[inline]
                fn lazy_select<A, B>(self, a: A, b: B) -> Self
                where
                    A: FnOnce() -> Self,
                    B: FnOnce() -> Self,
                {
                    let a = a();
                    let b = b();

                    self.select(a, b)
                }
            }
        )+
    };
}

impl_wide_bool_mask!(
    f32x4: (f32, u32),
    f32x8: (f32, u32),
    f64x2: (f64, u64),
    f64x4: (f64, u64)
);

#[cfg(test)]
mod test {
    use wide::{f32x4, f32x8, f64x2, f64x4};

    use crate::bool_mask::BoolMask;

    #[test]
    fn from_true() {
        assert!(f32x4::from_bool(true).is_true());
        assert!(!f32x4::from_bool(true).is_false());

        assert!(f32x8::from_bool(true).is_true());
        assert!(!f32x8::from_bool(true).is_false());

        assert!(f64x2::from_bool(true).is_true());
        assert!(!f64x2::from_bool(true).is_false());

        assert!(f64x4::from_bool(true).is_true());
        assert!(!f64x4::from_bool(true).is_false());
    }

    #[test]
    fn from_false() {
        assert!(f32x4::from_bool(false).is_false());
        assert!(!f32x4::from_bool(false).is_true());

        assert!(f32x8::from_bool(false).is_false());
        assert!(!f32x8::from_bool(false).is_true());

        assert!(f64x2::from_bool(false).is_false());
        assert!(!f64x2::from_bool(false).is_true());

        assert!(f64x4::from_bool(false).is_false());
        assert!(!f64x4::from_bool(false).is_true());
    }
}
