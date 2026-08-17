/// Check that traits for converting to and from XYZ have been implemented.
#[cfg(test)]
macro_rules! test_convert_into_from_xyz {
    ($ty:ty) => {
        #[test]
        fn convert_from_xyz() {
            use crate::FromColor;

            let _: $ty = <$ty>::from_color(crate::Xyz::<crate::white_point::D65, f32>::default());
        }

        #[test]
        fn convert_into_xyz() {
            use crate::FromColor;

            let _: crate::Xyz = crate::Xyz::from_color(<$ty>::default());
        }
    };
}

macro_rules! impl_tuple_conversion {
    ($ty: ident as ($($component_ty: ident),+)) => {
        impl_tuple_conversion!($ty<> as ($($component_ty),+));
    };
    ($ty: ident <$($ty_param: ident),*> as ($($component_ty: ident),+)) => {
        impl<$($ty_param,)* T> From<($($component_ty,)+)> for $ty<$($ty_param,)* T> {
            fn from(components: ($($component_ty,)+)) -> Self {
                Self::from_components(components)
            }
        }

        impl<$($ty_param,)* T> From<$ty<$($ty_param,)* T>> for ($($component_ty,)+) {
            fn from(color: $ty<$($ty_param,)* T>) -> ($($component_ty,)+) {
                color.into_components()
            }
        }

        impl<$($ty_param,)* T, A> From<($($component_ty,)+ A)> for crate::Alpha<$ty<$($ty_param,)* T>, A> {
            fn from(components: ($($component_ty,)+ A)) -> Self {
                Self::from_components(components)
            }
        }

        impl<$($ty_param,)* T, A> From<crate::Alpha<$ty<$($ty_param,)* T>, A>> for ($($component_ty,)+ A) {
            fn from(color: crate::Alpha<$ty<$($ty_param,)* T>, A>) -> ($($component_ty,)+ A) {
                color.into_components()
            }
        }
    };
}

macro_rules! __replace_generic_hue {
    (H, $hue_ty: ident) => {$hue_ty<T>};
    ($other: ident, $hue_ty: ident) => {$other};
}

macro_rules! impl_tuple_conversion_hue {
    ($ty: ident as ($($component_ty: ident),+), $hue_ty: ident) => {
        impl_tuple_conversion_hue!($ty<> as ($($component_ty),+), $hue_ty);
    };
    ($ty: ident <$($ty_param: ident),*> as ($($component_ty: ident),+), $hue_ty: ident) => {
        impl<$($ty_param,)* T, H: Into<$hue_ty<T>>> From<($($component_ty,)+)> for $ty<$($ty_param,)* T> {
            fn from(components: ($($component_ty,)+)) -> Self {
                Self::from_components(components)
            }
        }

        impl<$($ty_param,)* T> From<$ty<$($ty_param,)* T>> for ($(__replace_generic_hue!($component_ty, $hue_ty),)+) {
            fn from(color: $ty<$($ty_param,)* T>) -> ($(__replace_generic_hue!($component_ty, $hue_ty),)+) {
                color.into_components()
            }
        }

        impl<$($ty_param,)* T, H: Into<$hue_ty<T>>, A> From<($($component_ty,)+ A)> for crate::Alpha<$ty<$($ty_param,)* T>, A> {
            fn from(components: ($($component_ty,)+ A)) -> Self {
                Self::from_components(components)
            }
        }

        impl<$($ty_param,)* T, A> From<crate::Alpha<$ty<$($ty_param,)* T>, A>> for ($(__replace_generic_hue!($component_ty, $hue_ty),)+ A) {
            fn from(color: crate::Alpha<$ty<$($ty_param,)* T>, A>) -> ($(__replace_generic_hue!($component_ty, $hue_ty),)+ A) {
                color.into_components()
            }
        }
    };
}
