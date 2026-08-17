macro_rules! impl_premultiply {
    ($ty: ident {$($component: ident),+} $(phantom: $phantom: ident)? $(where $($where: tt)+)?) => {
        impl_premultiply!($ty<> {$($component),+} $(phantom: $phantom)? $(where $($where)+)?);
    };
    ($ty: ident <$($ty_param: ident),*> {$($component: ident),+} $(phantom: $phantom: ident)? $(where $($where: tt)+)?) => {
        impl<$($ty_param,)* T> crate::blend::Premultiply for $ty<$($ty_param,)* T>
        where
            T: crate::num::Real
                + crate::stimulus::Stimulus
                + crate::num::Zero
                + crate::num::IsValidDivisor
                + core::ops::Mul<T, Output = T>
                + core::ops::Div<T, Output = T>
                + Clone,
            T::Mask: crate::bool_mask::LazySelect<T> + Clone,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn premultiply(self, alpha: T) -> crate::blend::PreAlpha<Self> {
                crate::blend::PreAlpha {
                    color: self * alpha.clone(),
                    alpha
                }
            }

            #[inline]
            fn unpremultiply(premultiplied: crate::blend::PreAlpha<Self>) -> (Self, T) {
                let crate::blend::PreAlpha {
                    color: $ty { $($component,)+ .. },
                    alpha,
                } = premultiplied;

                let is_valid_divisor = alpha.is_valid_divisor();

                let color = Self {
                    $(
                        $component: lazy_select! {
                            if is_valid_divisor.clone() => $component / alpha.clone(),
                            else => T::zero()
                        },
                    )+
                    $($phantom: core::marker::PhantomData,)?
                };

                (color, alpha)
            }
        }

        impl<$($ty_param,)* T> From<crate::blend::PreAlpha<Self>> for $ty<$($ty_param,)* T>
        where
            Self: crate::blend::Premultiply<Scalar = T>,
        {
            fn from(premultiplied: crate::blend::PreAlpha<Self>) -> Self {
                use crate::blend::Premultiply;

                Self::unpremultiply(premultiplied).0
            }
        }
    };
}
