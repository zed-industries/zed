#[cfg(test)]
macro_rules! raw_pixel_conversion_tests {
    ($name: ident <$($ty_param: path),*> : $($component: ident),+) => {
        #[test]
        fn convert_from_f32_array() {
            raw_pixel_conversion_tests!(@float_array_test f32, $name<$($ty_param),*>: $($component),+);
        }

        #[test]
        fn convert_from_f64_array() {
            raw_pixel_conversion_tests!(@float_array_test f64, $name<$($ty_param),*>: $($component),+);
        }

        #[test]
        fn convert_from_f32_slice() {
            raw_pixel_conversion_tests!(@float_slice_test f32, $name<$($ty_param),*>: $($component),+);
        }

        #[test]
        fn convert_from_f64_slice() {
            raw_pixel_conversion_tests!(@float_slice_test f64, $name<$($ty_param),*>: $($component),+);
        }
    };

    (@float_array_test $float: ty, $name: ident <$($ty_param: path),*> : $($component: ident),+) => {
        use crate::cast::ArrayCast;
        use crate::Alpha;

        let mut counter: $float = 0.0;
        $(
            counter += 0.1;
            let $component = counter;
        )+
        let alpha = counter + 0.1;

        let raw: <$name<$($ty_param,)* $float> as ArrayCast>::Array = [$($component),+];
        let raw_plus_1: <Alpha<$name<$($ty_param,)* $float>, $float> as ArrayCast>::Array = [
            $($component,)+
            alpha
        ];
        let color: $name<$($ty_param,)* $float> = crate::cast::from_array(raw);

        let color_alpha: Alpha<$name<$($ty_param,)* $float>, $float> = crate::cast::from_array(raw_plus_1);

        assert_eq!(color, $name::new($($component),+));

        assert_eq!(color_alpha, Alpha::<$name<$($ty_param,)* $float>, $float>::new($($component,)+ alpha));
    };

    (@float_slice_test $float: ty, $name: ident <$($ty_param: path),*> : $($component: ident),+) => {
        use core::convert::{TryInto, TryFrom};
        use crate::Alpha;

        let mut counter: $float = 0.0;
        $(
            counter += 0.1;
            let $component = counter;
        )+
        let alpha = counter + 0.1;
        let extra = counter + 0.2;
        let raw: &[$float] = &[$($component),+];
        let raw_plus_1: &[$float] = &[
            $($component,)+
            alpha
        ];
        let raw_plus_2: &[$float] = &[
            $($component,)+
            alpha,
            extra
        ];
        let color: &$name<$($ty_param,)* $float> = raw.try_into().unwrap();
        assert!(<&$name<$($ty_param,)* $float>>::try_from(raw_plus_1).is_err());

        let color_alpha: &Alpha<$name<$($ty_param,)* $float>, $float> = raw_plus_1.try_into().unwrap();
        assert!(<&Alpha<$name<$($ty_param,)* $float>, $float>>::try_from(raw_plus_2).is_err());

        assert_eq!(color, &$name::new($($component),+));

        assert_eq!(color_alpha, &Alpha::<$name<$($ty_param,)* $float>, $float>::new($($component,)+ alpha));
    };
}

#[cfg(test)]
macro_rules! raw_pixel_conversion_fail_tests {
    ($name: ident <$($ty_param: path),*> : $($component: ident),+) => {
        #[test]
        #[should_panic(expected = "TryFromSliceError")]
        fn convert_from_short_f32_slice() {
            raw_pixel_conversion_fail_tests!(@float_slice_test f32, $name<$($ty_param),*>);
        }

        #[test]
        #[should_panic(expected = "TryFromSliceError")]
        fn convert_from_short_f64_slice() {
            raw_pixel_conversion_fail_tests!(@float_slice_test f64, $name<$($ty_param),*>);
        }
    };

    (@float_slice_test $float: ty, $name: ident <$($ty_param: path),*>) => {
        use core::convert::TryInto;

        let raw: &[$float] = &[0.1];
        let _: &$name<$($ty_param,)* $float> = raw.try_into().unwrap();
    };
}

macro_rules! impl_array_casts {
    ($self_ty: ident < $($ty_param: ident),+ > $($rest: tt)*) => {
        impl_array_casts!([$($ty_param),+] $self_ty < $($ty_param),+ > $($rest)*);
    };
    ([$($ty_param: tt)+] $self_ty: ident < $($self_ty_param: ty),+ > , [$array_item: ty; $array_len: expr] $(, where $($where: tt)+)?) => {
        impl<$($ty_param)+> AsRef<[$array_item; $array_len]> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn as_ref(&self) -> &[$array_item; $array_len] {
                crate::cast::into_array_ref(self)
            }
        }

        impl<$($ty_param)+> AsRef<$self_ty<$($self_ty_param),+>> for [$array_item; $array_len]
        $(where $($where)+)?
        {
            #[inline]
            fn as_ref(&self) -> &$self_ty<$($self_ty_param),+> {
                crate::cast::from_array_ref(self)
            }
        }

        impl<$($ty_param)+> AsMut<[$array_item; $array_len]> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn as_mut(&mut self) -> &mut [$array_item; $array_len] {
                crate::cast::into_array_mut(self)
            }
        }

        impl<$($ty_param)+> AsMut<$self_ty<$($self_ty_param),+>> for [$array_item; $array_len]
        $(where $($where)+)?
        {
            #[inline]
            fn as_mut(&mut self) -> &mut $self_ty<$($self_ty_param),+> {
                crate::cast::from_array_mut(self)
            }
        }

        impl<$($ty_param)+> AsRef<[$array_item]> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn as_ref(&self) -> &[$array_item] {
                &*AsRef::<[$array_item; $array_len]>::as_ref(self)
            }
        }

        impl<$($ty_param)+> AsMut<[$array_item]> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn as_mut(&mut self) -> &mut [$array_item] {
                &mut *AsMut::<[$array_item; $array_len]>::as_mut(self)
            }
        }

        impl<$($ty_param)+> From<$self_ty<$($self_ty_param),+>> for [$array_item; $array_len]
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: $self_ty<$($self_ty_param),+>) -> Self {
                crate::cast::into_array(color)
            }
        }

        impl<$($ty_param)+> From<[$array_item; $array_len]> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn from(array: [$array_item; $array_len]) -> Self {
                crate::cast::from_array(array)
            }
        }

        impl<'a, $($ty_param)+> From<&'a $self_ty<$($self_ty_param),+>> for &'a [$array_item; $array_len]
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: &'a $self_ty<$($self_ty_param),+>) -> Self {
                color.as_ref()
            }
        }

        impl<'a, $($ty_param)+> From<&'a [$array_item; $array_len]> for &'a $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn from(array: &'a [$array_item; $array_len]) -> Self{
                array.as_ref()
            }
        }

        impl<'a, $($ty_param)+> From<&'a $self_ty<$($self_ty_param),+>> for &'a [$array_item]
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: &'a $self_ty<$($self_ty_param),+>) -> Self {
                color.as_ref()
            }
        }

        impl<'a, $($ty_param)+> core::convert::TryFrom<&'a [$array_item]> for &'a $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            type Error = <&'a [$array_item; $array_len] as core::convert::TryFrom<&'a [$array_item]>>::Error;

            #[inline]
            fn try_from(slice: &'a [$array_item]) -> Result<Self, Self::Error> {
                use core::convert::TryInto;

                slice.try_into().map(crate::cast::from_array_ref)
            }
        }

        impl<'a, $($ty_param)+> From<&'a mut $self_ty<$($self_ty_param),+>> for &'a mut [$array_item; $array_len]
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: &'a mut $self_ty<$($self_ty_param),+>) -> Self {
                color.as_mut()
            }
        }

        impl<'a, $($ty_param)+> From<&'a mut [$array_item; $array_len]> for &'a mut $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn from(array: &'a mut [$array_item; $array_len]) -> Self{
                array.as_mut()
            }
        }

        impl<'a, $($ty_param)+> From<&'a mut $self_ty<$($self_ty_param),+>> for &'a mut [$array_item]
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: &'a mut $self_ty<$($self_ty_param),+>) -> Self {
                color.as_mut()
            }
        }

        impl<'a, $($ty_param)+> core::convert::TryFrom<&'a mut [$array_item]> for &'a mut $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            type Error = <&'a mut [$array_item; $array_len] as core::convert::TryFrom<&'a mut [$array_item]>>::Error;

            #[inline]
            fn try_from(slice: &'a mut [$array_item]) -> Result<Self, Self::Error> {
                use core::convert::TryInto;

                slice.try_into().map(crate::cast::from_array_mut)
            }
        }

        #[cfg(feature = "alloc")]
        impl<$($ty_param)+> From<alloc::boxed::Box<$self_ty<$($self_ty_param),+>>> for alloc::boxed::Box<[$array_item; $array_len]>
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: alloc::boxed::Box<$self_ty<$($self_ty_param),+>>) -> Self {
                crate::cast::into_array_box(color)
            }
        }

        #[cfg(feature = "alloc")]
        impl<$($ty_param)+> From<alloc::boxed::Box<[$array_item; $array_len]>> for alloc::boxed::Box<$self_ty<$($self_ty_param),+>>
        $(where $($where)+)?
        {
            #[inline]
            fn from(array: alloc::boxed::Box<[$array_item; $array_len]>) -> Self{
                crate::cast::from_array_box(array)
            }
        }
    }
}

macro_rules! impl_uint_casts_self {
    ($self_ty: ident < $($ty_param: ident),+ > $($rest: tt)*) => {
        impl_uint_casts_self!([$($ty_param),+] $self_ty < $($ty_param),+ > $($rest)*);
    };
    ([$($ty_param: tt)+] $self_ty: ident < $($self_ty_param: ty),+ >, $uint: ty $(, where $($where: tt)+)?) => {
        impl<$($ty_param)+> AsRef<$uint> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn as_ref(&self) -> &$uint {
                crate::cast::into_uint_ref(self)
            }
        }

        impl<$($ty_param)+> AsMut<$uint> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn as_mut(&mut self) -> &mut $uint {
                crate::cast::into_uint_mut(self)
            }
        }

        impl<$($ty_param)+> From<$uint> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn from(uint: $uint) -> Self {
                crate::cast::from_uint(uint)
            }
        }

        impl<'a, $($ty_param)+> From<&'a $uint> for &'a $self_ty<$($self_ty_param),+>
        where
            $uint: AsRef<$self_ty<$($self_ty_param),+>> $(, $($where)+)?
        {
            #[inline]
            fn from(uint: &'a $uint) -> Self{
                uint.as_ref()
            }
        }

        impl<'a, $($ty_param)+> From<&'a mut $uint> for &'a mut $self_ty<$($self_ty_param),+>
        where
            $uint: AsMut<$self_ty<$($self_ty_param),+>> $(, $($where)+)?
        {
            #[inline]
            fn from(uint: &'a mut $uint) -> Self{
                uint.as_mut()
            }
        }
    }
}

macro_rules! impl_uint_casts_other {
    /*
    ($self_ty: ident < $($ty_param: ident),+ > $($rest: tt)*) => {
        impl_uint_casts_other!([$($ty_param),+] $self_ty < $($ty_param),+ > $($rest)*);
    };
    */
    ([$($ty_param: ident),+] $self_ty: ident < $($self_ty_param: ty),+ >, $uint: ty $(, where $($where: tt)+)?) => {
        impl<$($ty_param)+> AsRef<$self_ty<$($self_ty_param),+>> for $uint
        $(where $($where)+)?
        {
            #[inline]
            fn as_ref(&self) -> &$self_ty<$($self_ty_param),+> {
                crate::cast::from_uint_ref(self)
            }
        }

        impl<$($ty_param)+> AsMut<$self_ty<$($self_ty_param),+>> for $uint
        $(where $($where)+)?
        {
            #[inline]
            fn as_mut(&mut self) -> &mut $self_ty<$($self_ty_param),+> {
                crate::cast::from_uint_mut(self)
            }
        }

        impl<$($ty_param)+> From<$self_ty<$($self_ty_param),+>> for $uint
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: $self_ty<$($self_ty_param),+>) -> Self {
                crate::cast::into_uint(color)
            }
        }

        impl<'a, $($ty_param)+> From<&'a $self_ty<$($self_ty_param),+>> for &'a $uint
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: &'a $self_ty<$($self_ty_param),+>) -> Self {
                color.as_ref()
            }
        }


        impl<'a, $($ty_param)+> From<&'a mut $self_ty<$($self_ty_param),+>> for &'a mut $uint
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: &'a mut $self_ty<$($self_ty_param),+>) -> Self {
                color.as_mut()
            }
        }
    }
}
