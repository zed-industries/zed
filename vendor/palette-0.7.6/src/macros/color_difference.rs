macro_rules! impl_euclidean_distance {
    (
        $ty: ident
        {$($component: ident),+}
        $(where $($where: tt)+)?
    ) => {
        // add empty generics brackets
        impl_euclidean_distance!($ty<> {$($component),+} $(where $($where)+)?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        {$($component: ident),+}
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::color_difference::EuclideanDistance for $ty<$($ty_param,)* T>
        where
            T: crate::num::Real + core::ops::Sub<T, Output=T> + core::ops::Add<T, Output=T> + core::ops::Mul<T, Output=T> + Clone,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn distance_squared(self, other: Self) -> Self::Scalar {
                let difference = self - other;
                let difference_squared = difference.clone() * difference;

                strip_plus!($(+ difference_squared.$component)+)
            }
        }
    };
}

macro_rules! impl_hyab {
    (
        $ty: ident
        {$($components: tt)+}
        $(where $($where: tt)+)?
    ) => {
        // add empty generics brackets
        impl_hyab!($ty<> {$($components)+} $(where $($where)+)?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        {lightness: $lightness:ident, chroma1: $chroma1:ident, chroma2: $chroma2:ident $(,)? }
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::color_difference::HyAb for $ty<$($ty_param,)* T>
        where
            T: crate::num::Real + crate::num::Abs + crate::num::Sqrt + core::ops::Sub<T, Output=T> + core::ops::Add<T, Output=T> + core::ops::Mul<T, Output=T> + Clone,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn hybrid_distance(self, other: Self) -> Self::Scalar {
                let lightness = self.$lightness - other.$lightness;
                let chroma1 = self.$chroma1 - other.$chroma1;
                let chroma2 = self.$chroma2 - other.$chroma2;

                lightness.abs() + (chroma1.clone() * chroma1 + chroma2.clone() * chroma2).sqrt()
            }
        }
    };
}
