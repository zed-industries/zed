macro_rules! _impl_increase_value_trait {
    (
        $trait: ident :: {$method: ident, $method_fixed: ident},
        $assign_trait: ident :: {$assign_method: ident, $assign_method_fixed: ident},
        $ty: ident <$($ty_param: ident),*>
        increase {$($component: ident => [$get_min: expr, $get_max: expr]),+}
        other {$($other_component: ident),*}
        $(phantom: $phantom: ident)?
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::$trait for $ty<$($ty_param,)* T>
        where
            T: crate::num::Real
                + crate::num::Zero
                + crate::num::MinMax
                + crate::num::Clamp
                + crate::num::Arithmetics
                + crate::num::PartialCmp
                + Clone,
            T::Mask: crate::bool_mask::LazySelect<T>,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn $method(self, factor: T) -> Self {
                $(
                    let difference = lazy_select!{
                        if factor.gt_eq(&T::zero()) => $get_max - &self.$component,
                        else => self.$component.clone(),
                    };

                    let $component = difference.max(T::zero()) * &factor;
                )+

                $ty {
                    $($other_component: self.$other_component,)*
                    $($component: crate::clamp(self.$component + $component, $get_min, $get_max),)+
                    $($phantom: PhantomData,)?
                }
            }

            #[inline]
            fn $method_fixed(self, amount: T) -> Self {
                $ty {
                    $($other_component: self.$other_component,)*
                    $($component: crate::clamp(self.$component + $get_max * &amount, $get_min, $get_max),)+
                    $($phantom: PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> crate::$assign_trait for $ty<$($ty_param,)* T>
        where
            T: crate::num::Real
                + crate::num::Zero
                + crate::num::MinMax
                + crate::num::ClampAssign
                + core::ops::AddAssign
                + crate::num::Arithmetics
                + crate::num::PartialCmp
                + Clone,
            T::Mask: crate::bool_mask::LazySelect<T>,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn $assign_method(&mut self, factor: T) {
                $(
                    let difference = lazy_select!{
                        if factor.gt_eq(&T::zero()) => $get_max - &self.$component,
                        else => self.$component.clone(),
                    };

                    self.$component += difference.max(T::zero()) * &factor;
                    crate::clamp_assign(&mut self.$component, $get_min, $get_max);
                )+
            }

            #[inline]
            fn $assign_method_fixed(&mut self, amount: T) {
                $(
                    self.$component += $get_max * &amount;
                    crate::clamp_assign(&mut self.$component, $get_min, $get_max);
                )+
            }
        }
    };
}

macro_rules! impl_lighten {
    ($ty: ident increase $($input: tt)+) => {
        impl_lighten!($ty<> increase $($input)+);
    };
    ($($input: tt)+) => {
        _impl_increase_value_trait!(
            Lighten::{lighten, lighten_fixed},
            LightenAssign::{lighten_assign, lighten_fixed_assign},
            $($input)+
        );
    };
}

macro_rules! impl_saturate {
    ($ty: ident increase $($input: tt)+) => {
        // add empty generics brackets
        impl_saturate!($ty<> increase $($input)+);
    };
    ($($input: tt)+) => {
        _impl_increase_value_trait!(
            Saturate::{saturate, saturate_fixed},
            SaturateAssign::{saturate_assign, saturate_fixed_assign},
            $($input)+
        );
    };
}

macro_rules! impl_lighten_hwb {
    (
        $ty: ident
        $(phantom: $phantom: ident)?
        $(where $($where: tt)+)?
    ) => {
        impl_lighten_hwb!($ty<> $(phantom: $phantom)? $(where $($where)+ )?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        $(phantom: $phantom: ident)?
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::Lighten for $ty<$($ty_param,)* T>
        where
            T: crate::num::Real
                + crate::num::Zero
                + crate::num::MinMax
                + crate::num::Arithmetics
                + crate::num::PartialCmp
                + Clone,
            T::Mask: LazySelect<T>,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn lighten(self, factor: T) -> Self {
                let difference_whiteness = lazy_select! {
                    if factor.gt_eq(&T::zero()) => Self::max_whiteness() - &self.whiteness,
                    else => self.whiteness.clone(),
                };
                let delta_whiteness = difference_whiteness.max(T::zero()) * &factor;

                let difference_blackness = lazy_select! {
                    if factor.gt_eq(&T::zero()) => self.blackness.clone(),
                    else => Self::max_blackness() - &self.blackness,
                };
                let delta_blackness = difference_blackness.max(T::zero()) * factor;

                Self {
                    hue: self.hue,
                    whiteness: (self.whiteness + delta_whiteness).max(Self::min_whiteness()),
                    blackness: (self.blackness - delta_blackness).max(Self::min_blackness()),
                    $($phantom: PhantomData,)?
                }
            }

            #[inline]
            fn lighten_fixed(self, amount: T) -> Self {
                Self {
                    hue: self.hue,
                    whiteness: (self.whiteness + Self::max_whiteness() * &amount)
                        .max(Self::min_whiteness()),
                    blackness: (self.blackness - Self::max_blackness() * amount).max(Self::min_blackness()),
                    $($phantom: PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> crate::LightenAssign for $ty<$($ty_param,)* T>
        where
            T: crate::num::Real
                + crate::num::Zero
                + crate::num::MinMax
                + crate::num::ClampAssign
                + core::ops::AddAssign
                + core::ops::SubAssign
                + crate::num::Arithmetics
                + crate::num::PartialCmp
                + Clone,
            T::Mask: LazySelect<T>,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn lighten_assign(&mut self, factor: T) {
                let difference_whiteness = lazy_select! {
                    if factor.gt_eq(&T::zero()) => Self::max_whiteness() - &self.whiteness,
                    else => self.whiteness.clone(),
                };
                self.whiteness += difference_whiteness.max(T::zero()) * &factor;
                crate::clamp_min_assign(&mut self.whiteness, Self::min_whiteness());

                let difference_blackness = lazy_select! {
                    if factor.gt_eq(&T::zero()) => self.blackness.clone(),
                    else => Self::max_blackness() - &self.blackness,
                };
                self.blackness -= difference_blackness.max(T::zero()) * factor;
                crate::clamp_min_assign(&mut self.blackness, Self::min_blackness());
            }

            #[inline]
            fn lighten_fixed_assign(&mut self, amount: T) {
                self.whiteness += Self::max_whiteness() * &amount;
                crate::clamp_min_assign(&mut self.whiteness, Self::min_whiteness());

                self.blackness -= Self::max_blackness() * amount;
                crate::clamp_min_assign(&mut self.blackness, Self::min_blackness());
            }
        }
    };
}
