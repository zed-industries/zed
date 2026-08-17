macro_rules! impl_color_add {
    ($self_ty: ident , [$($element: ident),+]) => {
        impl_color_add!($self_ty<>, [$($element),+]);
    };
    ($self_ty: ident < $($ty_param: ident),* > , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($ty_param,)* T> core::ops::Add<Self> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::Add<Output=T>
        {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                $self_ty {
                    $($element: self.$element + other.$element,)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> core::ops::Add<T> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::Add<Output=T> + Clone
        {
            type Output = Self;

            fn add(self, c: T) -> Self::Output {
                $self_ty {
                    $($element: self.$element + c.clone(),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> core::ops::AddAssign<Self> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::AddAssign,
        {
            fn add_assign(&mut self, other: Self) {
                $( self.$element += other.$element; )+
            }
        }

        impl<$($ty_param,)* T> core::ops::AddAssign<T> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::AddAssign + Clone
        {
            fn add_assign(&mut self, c: T) {
                $( self.$element += c.clone(); )+
            }
        }

        impl<$($ty_param,)* T> $crate::num::SaturatingAdd<Self> for $self_ty<$($ty_param,)* T>
        where
            T: $crate::num::SaturatingAdd<Output=T>
        {
            type Output = Self;

            fn saturating_add(self, other: Self) -> Self::Output {
                $self_ty {
                    $($element: self.$element.saturating_add(other.$element),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> $crate::num::SaturatingAdd<T> for $self_ty<$($ty_param,)* T>
        where
            T: $crate::num::SaturatingAdd<Output=T> + Clone
        {
            type Output = Self;

            fn saturating_add(self, c: T) -> Self::Output {
                $self_ty {
                    $($element: self.$element.saturating_add(c.clone()),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }
    };
}

/// Implement `Sub` and `SubAssign` traits for a color space.
///
/// Both scalars and color arithmetic are implemented.
macro_rules! impl_color_sub {
    ($self_ty: ident , [$($element: ident),+]) => {
        impl_color_sub!($self_ty<>, [$($element),+]);
    };
    ($self_ty: ident < $($ty_param: ident),* > , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($ty_param,)* T> core::ops::Sub<Self> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::Sub<Output=T>
        {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                $self_ty {
                    $($element: self.$element - other.$element,)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> core::ops::Sub<T> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::Sub<Output=T> + Clone
        {
            type Output = Self;

            fn sub(self, c: T) -> Self::Output {
                $self_ty {
                    $($element: self.$element - c.clone(),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> core::ops::SubAssign<Self> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::SubAssign,
        {
            fn sub_assign(&mut self, other: Self) {
                $( self.$element -= other.$element; )+
            }
        }

        impl<$($ty_param,)* T> core::ops::SubAssign<T> for $self_ty<$($ty_param,)* T>
        where
            T:  core::ops::SubAssign + Clone
        {
            fn sub_assign(&mut self, c: T) {
                $( self.$element -= c.clone(); )+
            }
        }

        impl<$($ty_param,)* T> $crate::num::SaturatingSub<Self> for $self_ty<$($ty_param,)* T>
        where
            T: $crate::num::SaturatingSub<Output=T>
        {
            type Output = Self;

            fn saturating_sub(self, other: Self) -> Self::Output {
                $self_ty {
                    $($element: self.$element.saturating_sub(other.$element),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> $crate::num::SaturatingSub<T> for $self_ty<$($ty_param,)* T>
        where
            T: $crate::num::SaturatingSub<Output=T> + Clone
        {
            type Output = Self;

            fn saturating_sub(self, c: T) -> Self::Output {
                $self_ty {
                    $($element: self.$element.saturating_sub(c.clone()),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }
    };
}

/// Implement `Mul` and `MulAssign` traits for a color space.
///
/// Both scalars and color arithmetic are implemented.
macro_rules! impl_color_mul {
    ($self_ty: ident , [$($element: ident),+]) => {
        impl_color_mul!($self_ty<>, [$($element),+]);
    };
    ($self_ty: ident < $($ty_param: ident),* > , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($ty_param,)* T> core::ops::Mul<Self> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::Mul<Output=T>
        {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                $self_ty {
                    $($element: self.$element * other.$element,)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> core::ops::Mul<T> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::Mul<Output=T> + Clone
        {
            type Output = Self;

            fn mul(self, c: T) -> Self::Output {
                $self_ty {
                    $($element: self.$element * c.clone(),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> core::ops::MulAssign<Self> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::MulAssign,
        {
            fn mul_assign(&mut self, other: Self) {
                $( self.$element *= other.$element; )+
            }
        }

        impl<$($ty_param,)* T> core::ops::MulAssign<T> for $self_ty<$($ty_param,)* T>
        where
            T:  core::ops::MulAssign + Clone
        {
            fn mul_assign(&mut self, c: T) {
                $( self.$element *= c.clone(); )+
            }
        }
    };
}

/// Implement `Div` and `DivAssign` traits for a color space.
///
/// Both scalars and color arithmetic are implemented.
macro_rules! impl_color_div {
    ($self_ty: ident , [$($element: ident),+]) => {
        impl_color_div!($self_ty<>, [$($element),+]);
    };
    ($self_ty: ident < $($ty_param: ident),* > , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($ty_param,)* T> core::ops::Div<Self> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::Div<Output=T>
        {
            type Output = Self;

            fn div(self, other: Self) -> Self::Output {
                $self_ty {
                    $($element: self.$element / other.$element,)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> core::ops::Div<T> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::Div<Output=T> + Clone
        {
            type Output = Self;

            fn div(self, c: T) -> Self::Output {
                $self_ty {
                    $($element: self.$element / c.clone(),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> core::ops::DivAssign<Self> for $self_ty<$($ty_param,)* T>
        where
            T: core::ops::DivAssign,
        {
            fn div_assign(&mut self, other: Self) {
                $( self.$element /= other.$element; )+
            }
        }

        impl<$($ty_param,)* T> core::ops::DivAssign<T> for $self_ty<$($ty_param,)* T>
        where
            T:  core::ops::DivAssign + Clone
        {
            fn div_assign(&mut self, c: T) {
                $( self.$element /= c.clone(); )+
            }
        }
    };
}
