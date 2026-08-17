macro_rules! impl_is_within_bounds {
    (
        $ty: ident
        {$($component: ident => [$get_min: expr, $get_max: expr]),+}
        $(where $($where: tt)+)?
    ) => {
        // add empty generics brackets
        impl_is_within_bounds!($ty<> {$($component => [$get_min, $get_max]),+} $(where $($where)+)?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        {$($component: ident => [$get_min: expr, $get_max: expr]),+}
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::IsWithinBounds for $ty<$($ty_param,)* T>
        where
            T: crate::num::PartialCmp,
            T::Mask: core::ops::BitAnd<Output = T::Mask>,
            $($($where)+)?
        {
            #[inline]
            fn is_within_bounds(&self) -> T::Mask {
                $(
                    self.$component.gt_eq(&$get_min)
                    & Option::from($get_max).map_or(crate::BoolMask::from_bool(true), |max|self.$component.lt_eq(&max))
                )&+
            }
        }
    };
}

macro_rules! impl_is_within_bounds_hwb {
    (
        $ty: ident
        $(where $($where: tt)+)?
    ) => {
        // add empty generics brackets
        impl_is_within_bounds_hwb!($ty<> $(where $($where)+)?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::IsWithinBounds for $ty<$($ty_param,)* T>
        where
            T: crate::num::PartialCmp + core::ops::Add<Output = T> + Clone,
            T::Mask: core::ops::BitAnd<Output = T::Mask>,
            $($($where)+)?
        {
            #[inline]
            fn is_within_bounds(&self) -> T::Mask {
                self.blackness.gt_eq(&Self::min_blackness()) & self.blackness.lt_eq(&Self::max_blackness()) &
                self.whiteness.gt_eq(&Self::min_whiteness()) & self.whiteness.lt_eq(&Self::max_blackness()) &
                (self.whiteness.clone() + self.blackness.clone()).lt_eq(&T::max_intensity())
            }
        }
    };
}

macro_rules! _clamp_value {
    ($value: expr, $min: expr) => {
        crate::clamp_min($value, $min)
    };
    ($value: expr, $min: expr, $max: expr) => {
        crate::clamp($value, $min, $max)
    };
    (@assign $value: expr, $min: expr) => {
        crate::clamp_min_assign($value, $min)
    };
    (@assign $value: expr, $min: expr, $max: expr) => {
        crate::clamp_assign($value, $min, $max)
    };
}

macro_rules! impl_clamp {
    (
        $ty: ident
        {$($component: ident => [$get_min: expr $(, $get_max: expr)?]),+}
        $(other {$($other: ident),+})?
        $(where $($where: tt)+)?
    ) => {
        // add empty generics brackets
        impl_clamp!($ty<> {$($component => [$get_min $(, $get_max)?]),+} $(other {$($other),+})? $(where $($where)+)?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        {$($component: ident => [$get_min: expr $(, $get_max: expr)?]),+}
        $(other {$($other: ident),+})?
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::Clamp for $ty<$($ty_param,)* T>
        where
            T: crate::num::Clamp,
            $($($where)+)?
        {
            #[inline]
            fn clamp(self) -> Self {
                Self {
                    $($component: _clamp_value!(self.$component, $get_min $(, $get_max)?),)+
                    $($($other: self.$other,)+)?
                }
            }
        }

        impl<$($ty_param,)* T> crate::ClampAssign for $ty<$($ty_param,)* T>
        where
            T: crate::num::ClampAssign,
            $($($where)+)?
        {
            #[inline]
            fn clamp_assign(&mut self) {
                $(_clamp_value!(@assign &mut self.$component, $get_min $(, $get_max)?);)+
            }
        }
    };
}

macro_rules! impl_clamp_hwb {
    (
        $ty: ident
        $(phantom: $phantom: ident)?
        $(where $($where: tt)+)?
    ) => {
        // add empty generics brackets
        impl_clamp_hwb!($ty<> $(phantom: $phantom)? $(where $($where)+)?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        $(phantom: $phantom: ident)?
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::Clamp for $ty<$($ty_param,)* T>
        where
            T: crate::num::One
                + crate::num::Clamp
                + crate::num::PartialCmp
                + core::ops::Add<Output = T>
                + core::ops::DivAssign
                + Clone,
            T::Mask: crate::bool_mask::Select<T>,
            $($($where)+)?
        {
            #[inline]
            fn clamp(self) -> Self {
                let mut whiteness = crate::clamp_min(self.whiteness.clone(), Self::min_whiteness());
                let mut blackness = crate::clamp_min(self.blackness.clone(), Self::min_blackness());

                let sum = self.blackness + self.whiteness;
                let divisor = sum.gt(&T::max_intensity()).select(sum, T::one());
                whiteness /= divisor.clone();
                blackness /= divisor;

                Self {hue: self.hue, whiteness, blackness $(, $phantom: self.$phantom)?}
            }
        }

        impl<$($ty_param,)* T> crate::ClampAssign for $ty<$($ty_param,)* T>
        where
            T: crate::num::One
                + crate::num::ClampAssign
                + crate::num::PartialCmp
                + core::ops::Add<Output = T>
                + core::ops::DivAssign
                + Clone,
            T::Mask: crate::bool_mask::Select<T>,
            $($($where)+)?
        {
            #[inline]
            fn clamp_assign(&mut self) {
                crate::clamp_min_assign(&mut self.whiteness, Self::min_whiteness());
                crate::clamp_min_assign(&mut self.blackness, Self::min_blackness());

                let sum = self.blackness.clone() + self.whiteness.clone();
                let divisor = sum.gt(&T::max_intensity()).select(sum, T::one());
                self.whiteness /= divisor.clone();
                self.blackness /= divisor;
            }
        }
    };
}

//Helper macro for checking ranges and clamping.
#[cfg(test)]
macro_rules! assert_ranges {
    (@make_tuple $first:pat, $next:ident,) => (($first, $next));

    (@make_tuple $first:pat, $next:ident, $($rest:ident,)*) => (
        assert_ranges!(@make_tuple ($first, $next), $($rest,)*)
    );

    (
        $ty:ident < $($ty_params:ty),+ >;
        clamped {$($clamped:ident: $clamped_from:expr => $clamped_to:expr),+}
        clamped_min {$($clamped_min:ident: $clamped_min_from:expr => $clamped_min_to:expr),*}
        unclamped {$($unclamped:ident: $unclamped_from:expr => $unclamped_to:expr),*}
    ) => (
        {
            use core::iter::repeat;
            use crate::{Clamp, IsWithinBounds};

            {
                print!("checking below clamp bounds... ");
                $(
                    let from = $clamped_from;
                    let to = $clamped_to;
                    let diff = to - from;
                    let $clamped = (1..11).map(|i| from - (i as f64 / 10.0) * diff);
                )+

                $(
                    let from = $clamped_min_from;
                    let to = $clamped_min_to;
                    let diff = to - from;
                    let $clamped_min = (1..11).map(|i| from - (i as f64 / 10.0) * diff);
                )*

                $(
                    let from = $unclamped_from;
                    let to = $unclamped_to;
                    let diff = to - from;
                    let $unclamped = (1..11).map(|i| from - (i as f64 / 10.0) * diff);
                )*

                #[allow(clippy::needless_update)]
                for assert_ranges!(@make_tuple (), $($clamped,)+ $($clamped_min,)* $($unclamped,)* ) in repeat(()) $(.zip($clamped))+ $(.zip($clamped_min))* $(.zip($unclamped))* {
                    let color: $ty<$($ty_params),+> = $ty {
                        $($clamped: $clamped.into(),)+
                        $($clamped_min: $clamped_min.into(),)*
                        $($unclamped: $unclamped.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };

                    let clamped = color.clamp();

                    let expected: $ty<$($ty_params),+> = $ty {
                        $($clamped: $clamped_from.into(),)+
                        $($clamped_min: $clamped_min_from.into(),)*
                        $($unclamped: $unclamped.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };

                    assert!(!color.is_within_bounds());
                    assert_eq!(clamped, expected);
                }

                println!("ok")
            }

            {
                print!("checking within clamp bounds... ");
                $(
                    let from = $clamped_from;
                    let to = $clamped_to;
                    let diff = to - from;
                    let $clamped = (0..11).map(|i| from + (i as f64 / 10.0) * diff);
                )+

                $(
                    let from = $clamped_min_from;
                    let to = $clamped_min_to;
                    let diff = to - from;
                    let $clamped_min = (0..11).map(|i| from + (i as f64 / 10.0) * diff);
                )*

                $(
                    let from = $unclamped_from;
                    let to = $unclamped_to;
                    let diff = to - from;
                    let $unclamped = (0..11).map(|i| from + (i as f64 / 10.0) * diff);
                )*

                #[allow(clippy::needless_update)]
                for assert_ranges!(@make_tuple (), $($clamped,)+ $($clamped_min,)* $($unclamped,)* ) in repeat(()) $(.zip($clamped))+ $(.zip($clamped_min))* $(.zip($unclamped))* {
                    let color: $ty<$($ty_params),+> = $ty {
                        $($clamped: $clamped.into(),)+
                        $($clamped_min: $clamped_min.into(),)*
                        $($unclamped: $unclamped.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };

                    let clamped = color.clamp();

                    assert!(color.is_within_bounds());
                    assert_eq!(clamped, color);
                }

                println!("ok")
            }

            {
                print!("checking above clamp bounds... ");
                $(
                    let from = $clamped_from;
                    let to = $clamped_to;
                    let diff = to - from;
                    let $clamped = (1..11).map(|i| to + (i as f64 / 10.0) * diff);
                )+

                $(
                    let from = $clamped_min_from;
                    let to = $clamped_min_to;
                    let diff = to - from;
                    let $clamped_min = (1..11).map(|i| to + (i as f64 / 10.0) * diff);
                )*

                $(
                    let from = $unclamped_from;
                    let to = $unclamped_to;
                    let diff = to - from;
                    let $unclamped = (1..11).map(|i| to + (i as f64 / 10.0) * diff);
                )*

                #[allow(clippy::needless_update)]
                for assert_ranges!(@make_tuple (), $($clamped,)+ $($clamped_min,)* $($unclamped,)* ) in repeat(()) $(.zip($clamped))+ $(.zip($clamped_min))* $(.zip($unclamped))* {
                    let color: $ty<$($ty_params),+> = $ty {
                        $($clamped: $clamped.into(),)+
                        $($clamped_min: $clamped_min.into(),)*
                        $($unclamped: $unclamped.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };

                    let clamped = color.clamp();

                    let expected: $ty<$($ty_params),+> = $ty {
                        $($clamped: $clamped_to.into(),)+
                        $($clamped_min: $clamped_min.into(),)*
                        $($unclamped: $unclamped.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };

                    assert!(!color.is_within_bounds());
                    assert_eq!(clamped, expected);
                }

                println!("ok")
            }
        }
    );
}
