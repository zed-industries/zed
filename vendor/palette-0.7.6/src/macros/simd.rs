macro_rules! make_recursive_tuples {
    ($first:tt $(,$rest:tt)+) => {
        make_recursive_tuples!(@ $first [$($rest),+])
    };
    (@ $tuple:tt [$first:tt $(,$rest:tt)*]) => {
        make_recursive_tuples!(@ ($tuple, $first) [$($rest),*])
    };
    (@ $tuple:tt []) => {
        $tuple
    }
}

macro_rules! impl_simd_array_conversion {
    (  $self_ty: ident , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl_simd_array_conversion!($self_ty<>, [$($element),+] $(, $phantom)?);
    };
    (  $self_ty: ident < $($ty_param: ident),* > , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($ty_param,)* T, V, const N: usize> From<[$self_ty<$($ty_param,)* T>; N]> for $self_ty<$($ty_param,)* V>
        where
            [T; N]: Default,
            V: crate::num::FromScalarArray<N, Scalar = T>,
        {
            fn from(colors: [$self_ty<$($ty_param,)* T>; N]) -> Self {
                $(let mut $element: [T; N] = Default::default();)*

                for (index, color) in IntoIterator::into_iter(colors).enumerate() {
                    $($element[index] = color.$element;)*
                }

                $self_ty {
                    $($element: V::from_array($element),)*
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T, V, const N: usize> From<[crate::Alpha<$self_ty<$($ty_param,)* T>, T>; N]> for crate::Alpha<$self_ty<$($ty_param,)* V>, V>
        where
            [T; N]: Default,
            V: crate::num::FromScalarArray<N, Scalar = T>,
        {
            fn from(colors: [crate::Alpha<$self_ty<$($ty_param,)* T>, T>; N]) -> Self {
                $(let mut $element: [T; N] = Default::default();)*
                let mut alpha: [T; N] = Default::default();

                for (index, color) in IntoIterator::into_iter(colors).enumerate() {
                    $($element[index] = color.color.$element;)*
                    alpha[index] = color.alpha
                }

                crate::Alpha {
                    color: $self_ty {
                        $($element: V::from_array($element),)*
                        $($phantom: core::marker::PhantomData,)?
                    },
                    alpha: V::from_array(alpha),
                }
            }
        }

        impl<$($ty_param,)* T, V, const N: usize> From<[crate::blend::PreAlpha<$self_ty<$($ty_param,)* T>>; N]> for crate::blend::PreAlpha<$self_ty<$($ty_param,)* V>>
        where
            [T; N]: Default,
            V: crate::num::FromScalarArray<N, Scalar = T>,
            $self_ty<$($ty_param,)* T>: crate::blend::Premultiply<Scalar = T>,
            $self_ty<$($ty_param,)* V>: crate::blend::Premultiply<Scalar = V>,
        {
            fn from(colors: [crate::blend::PreAlpha<$self_ty<$($ty_param,)* T>>; N]) -> Self {
                $(let mut $element: [T; N] = Default::default();)*
                let mut alpha: [T; N] = Default::default();

                for (index, color) in IntoIterator::into_iter(colors).enumerate() {
                    $($element[index] = color.color.$element;)*
                    alpha[index] = color.alpha
                }

                crate::blend::PreAlpha {
                    color: $self_ty {
                        $($element: V::from_array($element),)*
                        $($phantom: core::marker::PhantomData,)?
                    },
                    alpha: V::from_array(alpha),
                }
            }
        }

        impl<$($ty_param,)* T, V, const N: usize> From<$self_ty<$($ty_param,)* V>> for [$self_ty<$($ty_param,)* T>; N]
        where
            Self: Default,
            V: crate::num::IntoScalarArray<N, Scalar = T>,
        {
            fn from(color: $self_ty<$($ty_param,)* V>) -> Self {
                let mut colors = Self::default();
                $(let $element = color.$element.into_array();)*

                for make_recursive_tuples!(index $(,$element)*) in
                    (0..)$(.zip($element))*
                {
                    colors[index] = $self_ty {
                        $($element,)*
                        $($phantom: core::marker::PhantomData,)?
                    };
                }

                colors
            }
        }

        impl<$($ty_param,)* T, V, const N: usize> From<crate::Alpha<$self_ty<$($ty_param,)* V>, V>> for [crate::Alpha<$self_ty<$($ty_param,)* T>, T>; N]
        where
            Self: Default,
            V: crate::num::IntoScalarArray<N, Scalar = T>,
        {
            fn from(color: crate::Alpha<$self_ty<$($ty_param,)* V>, V>) -> Self {
                let mut colors = Self::default();
                $(let $element = color.color.$element.into_array();)*
                let alpha = color.alpha.into_array();

                for make_recursive_tuples!(index $(,$element)*, alpha) in
                    (0..)$(.zip($element))*.zip(alpha)
                {
                    colors[index] = crate::Alpha {
                        color: $self_ty {
                            $($element,)*
                            $($phantom: core::marker::PhantomData,)?
                        },
                        alpha,
                    };
                }

                colors
            }
        }

        impl<$($ty_param,)* T, V, const N: usize> From<crate::blend::PreAlpha<$self_ty<$($ty_param,)* V>>> for [crate::blend::PreAlpha<$self_ty<$($ty_param,)* T>>; N]
        where
            Self: Default,
            V: crate::num::IntoScalarArray<N, Scalar = T>,
            $self_ty<$($ty_param,)* T>: crate::blend::Premultiply<Scalar = T>,
            $self_ty<$($ty_param,)* V>: crate::blend::Premultiply<Scalar = V>,
        {
            fn from(color: crate::blend::PreAlpha<$self_ty<$($ty_param,)* V>>) -> Self {
                let mut colors = Self::default();
                $(let $element = color.color.$element.into_array();)*
                let alpha = color.alpha.into_array();

                for make_recursive_tuples!(index $(,$element)*, alpha) in
                    (0..)$(.zip($element))*.zip(alpha)
                {
                    colors[index] = crate::blend::PreAlpha {
                        color: $self_ty {
                            $($element,)*
                            $($phantom: core::marker::PhantomData,)?
                        },
                        alpha,
                    };
                }

                colors
            }
        }
    };
}

macro_rules! impl_simd_array_conversion_hue {
    (  $self_ty: ident , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl_simd_array_conversion_hue!($self_ty<>, [$($element),+] $(, $phantom)?);
    };
    (  $self_ty: ident < $($ty_param: ident),* > , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($ty_param,)* T, V, const N: usize> From<[$self_ty<$($ty_param,)* T>; N]> for $self_ty<$($ty_param,)* V>
        where
            [T; N]: Default,
            V: crate::num::FromScalarArray<N, Scalar = T>,
        {
            fn from(colors: [$self_ty<$($ty_param,)* T>; N]) -> Self {
                let mut hue: [T; N] = Default::default();
                $(let mut $element: [T; N] = Default::default();)*

                for (index, color) in IntoIterator::into_iter(colors).enumerate() {
                    hue[index] = color.hue.into_inner();
                    $($element[index] = color.$element;)*
                }

                $self_ty {
                    hue: V::from_array(hue).into(),
                    $($element: V::from_array($element),)*
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T, V, const N: usize> From<[crate::Alpha<$self_ty<$($ty_param,)* T>, T>; N]> for crate::Alpha<$self_ty<$($ty_param,)* V>, V>
        where
            [T; N]: Default,
            V: crate::num::FromScalarArray<N, Scalar = T>,
        {
            fn from(colors: [crate::Alpha<$self_ty<$($ty_param,)* T>, T>; N]) -> Self {
                let mut hue: [T; N] = Default::default();
                $(let mut $element: [T; N] = Default::default();)*
                let mut alpha: [T; N] = Default::default();

                for (index, color) in IntoIterator::into_iter(colors).enumerate() {
                    hue[index] = color.color.hue.into_inner();
                    $($element[index] = color.color.$element;)*
                    alpha[index] = color.alpha
                }

                crate::Alpha {
                    color: $self_ty {
                        hue: V::from_array(hue).into(),
                        $($element: V::from_array($element),)*
                        $($phantom: core::marker::PhantomData,)?
                    },
                    alpha: V::from_array(alpha),
                }
            }
        }

        impl<$($ty_param,)* T, V, const N: usize> From<$self_ty<$($ty_param,)* V>> for [$self_ty<$($ty_param,)* T>; N]
        where
            Self: Default,
            V: crate::num::IntoScalarArray<N, Scalar = T>,
        {
            fn from(color: $self_ty<$($ty_param,)* V>) -> Self {
                let mut colors = Self::default();
                let hue = color.hue.into_inner().into_array();
                $(let $element = color.$element.into_array();)*

                for make_recursive_tuples!(index, hue $(,$element)*) in
                    (0..).zip(hue)$(.zip($element))*
                {
                    colors[index] = $self_ty {
                        hue: hue.into(),
                        $($element,)*
                        $($phantom: core::marker::PhantomData,)?
                    };
                }

                colors
            }
        }

        impl<$($ty_param,)* T, V, const N: usize> From<crate::Alpha<$self_ty<$($ty_param,)* V>, V>> for [crate::Alpha<$self_ty<$($ty_param,)* T>, T>; N]
        where
            Self: Default,
            V: crate::num::IntoScalarArray<N, Scalar = T>,
        {
            fn from(color: crate::Alpha<$self_ty<$($ty_param,)* V>, V>) -> Self {
                let mut colors = Self::default();
                let hue = color.color.hue.into_inner().into_array();
                $(let $element = color.color.$element.into_array();)*
                let alpha = color.alpha.into_array();

                for make_recursive_tuples!(index, hue $(,$element)*, alpha) in
                    (0..).zip(hue)$(.zip($element))*.zip(alpha)
                {
                    colors[index] = crate::Alpha {
                        color: $self_ty {
                            hue: hue.into(),
                            $($element,)*
                            $($phantom: core::marker::PhantomData,)?
                        },
                        alpha,
                    };
                }

                colors
            }
        }
    };
}
