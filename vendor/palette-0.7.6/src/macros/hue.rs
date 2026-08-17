macro_rules! impl_hue_ops {
    (  $self_ty: ident , $hue_ty: ident) => {
        impl_hue_ops!($self_ty<>, $hue_ty);
    };
    (  $self_ty: ident < $($ty_param: ident),* > , $hue_ty: ident) => {
        impl<$($ty_param,)* T> crate::GetHue for $self_ty<$($ty_param,)* T>
        where
            T: Clone,
        {
            type Hue = $hue_ty<T>;

            #[inline]
            fn get_hue(&self) -> $hue_ty<T> {
                self.hue.clone()
            }
        }

        impl<$($ty_param,)* T, H> crate::WithHue<H> for $self_ty<$($ty_param,)* T>
        where
            H: Into<$hue_ty<T>>,
        {
            #[inline]
            fn with_hue(mut self, hue: H) -> Self {
                self.hue = hue.into();
                self
            }
        }

        impl<$($ty_param,)* T, H> crate::SetHue<H> for $self_ty<$($ty_param,)* T>
        where
            H: Into<$hue_ty<T>>,
        {
            #[inline]
            fn set_hue(&mut self, hue: H) {
                self.hue = hue.into();
            }
        }

        impl<$($ty_param,)* T> crate::ShiftHue for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::Add<Output = T>,
        {
            type Scalar = T;

            #[inline]
            fn shift_hue(mut self, amount: Self::Scalar) -> Self {
                self.hue = self.hue + amount;
                self
            }
        }

        impl<$($ty_param,)* T> crate::ShiftHueAssign for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::AddAssign,
        {
            type Scalar = T;

            #[inline]
            fn shift_hue_assign(&mut self, amount: Self::Scalar) {
                self.hue += amount;
            }
        }
    }
}
