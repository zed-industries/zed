macro_rules! first {
    (($($first: tt)+) $(, ($($rest: tt)+))*) => {
        $($first)+
    };
}

macro_rules! skip_first {
    (($($first: tt)+) $(, ($($rest: tt)+))*) => {
        $($($rest)+)*
    };
}

macro_rules! impl_struct_of_array_traits {
    (  $self_ty: ident , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl_struct_of_array_traits!($self_ty<>, [$($element),+] $(, $phantom)?);
    };
    (  $self_ty: ident < $($phantom_ty: ident)? > , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($phantom_ty,)? T, C> Extend<$self_ty<$($phantom_ty,)? T>> for $self_ty<$($phantom_ty,)? C>
        where
            C: Extend<T>,
        {
            #[inline(always)]
            fn extend<I: IntoIterator<Item = $self_ty<$($phantom_ty,)? T>>>(&mut self, iter: I) {
                let iter = iter.into_iter();

                for color in iter {
                    $(self.$element.extend(core::iter::once(color.$element));)+
                }
            }
        }

        impl<$($phantom_ty,)? T, C> core::iter::FromIterator<$self_ty<$($phantom_ty,)? T>> for $self_ty<$($phantom_ty,)? C>
        where
            Self: Extend<$self_ty<$($phantom_ty,)? T>>,
            C: Default,
        {
            #[inline(always)]
            fn from_iter<I: IntoIterator<Item = $self_ty<$($phantom_ty,)? T>>>(iter: I) -> Self {
                let mut result = Self {
                    $($element: C::default(),)+
                    $($phantom: core::marker::PhantomData)?
                };
                result.extend(iter);

                result
            }
        }

        impl<$($phantom_ty,)? T, const N: usize> IntoIterator for $self_ty<$($phantom_ty,)? [T; N]>
        {
            type Item = $self_ty<$($phantom_ty,)? T>;

            type IntoIter = Iter<core::array::IntoIter<T, N> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: IntoIterator::into_iter(self.$element),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<$($phantom_ty,)? T, const N: usize> IntoIterator for crate::alpha::Alpha<$self_ty<$($phantom_ty,)? [T; N]>, [T; N]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? T>, T>;

            type IntoIter = crate::alpha::Iter<Iter<core::array::IntoIter<T, N> $(,$phantom_ty)?>, core::array::IntoIter<T, N>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: self.color.into_iter(),
                    alpha: IntoIterator::into_iter(self.alpha)
                }
            }
        }

        impl<'a, $($phantom_ty,)? T> IntoIterator for $self_ty<$($phantom_ty,)? &'a [T]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a T>;

            type IntoIter = Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: self.$element.into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, $($phantom_ty,)? T> IntoIterator for crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a [T]>, &'a [T]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a T>, &'a T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>, core::slice::Iter<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: self.color.into_iter(),
                    alpha: self.alpha.into_iter(),
                }
            }
        }

        impl<'a, $($phantom_ty,)? T> IntoIterator for $self_ty<$($phantom_ty,)? &'a mut [T]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a mut T>;

            type IntoIter = Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: self.$element.into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, $($phantom_ty,)? T> IntoIterator for crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a mut [T]>, &'a mut [T]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a mut T>, &'a mut T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>, core::slice::IterMut<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: self.color.into_iter(),
                    alpha: self.alpha.into_iter(),
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<$($phantom_ty,)? T> IntoIterator for $self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>
        {
            type Item = $self_ty<$($phantom_ty,)? T>;

            type IntoIter = Iter<alloc::vec::IntoIter<T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: self.$element.into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, $($phantom_ty,)? T> IntoIterator for crate::alpha::Alpha<$self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>, alloc::vec::Vec<T>>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? T>, T>;

            type IntoIter = crate::alpha::Iter<Iter<alloc::vec::IntoIter<T> $(,$phantom_ty)?>, alloc::vec::IntoIter<T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: self.color.into_iter(),
                    alpha: self.alpha.into_iter(),
                }
            }
        }

        impl<'a, $($phantom_ty,)? T, const N: usize> IntoIterator for &'a $self_ty<$($phantom_ty,)? [T; N]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a T>;

            type IntoIter = Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: (&self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, $($phantom_ty,)? T, const N: usize> IntoIterator for &'a crate::alpha::Alpha<$self_ty<$($phantom_ty,)? [T; N]>, [T; N]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a T>, &'a T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>, core::slice::Iter<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&self.color).into_iter(),
                    alpha: (&self.alpha).into_iter(),
                }
            }
        }

        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a $self_ty<$($phantom_ty,)? &'b [T]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a T>;

            type IntoIter = Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: (&self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'b [T]>, &'b [T]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a T>, &'a T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>, core::slice::Iter<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: self.color.into_iter(),
                    alpha: self.alpha.into_iter(),
                }
            }
        }

        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a $self_ty<$($phantom_ty,)? &'b mut [T]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a T>;

            type IntoIter = Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: (&*self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'b mut [T]>, &'b mut [T]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a T>, &'a T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>, core::slice::Iter<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&self.color).into_iter(),
                    alpha: (&*self.alpha).into_iter(),
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, $($phantom_ty,)? T> IntoIterator for &'a $self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a T>;

            type IntoIter = Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: (&self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a crate::alpha::Alpha<$self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>, alloc::vec::Vec<T>>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a T>, &'a T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>, core::slice::Iter<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&self.color).into_iter(),
                    alpha: (&self.alpha).into_iter(),
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, $($phantom_ty,)? T> IntoIterator for &'a $self_ty<$($phantom_ty,)? alloc::boxed::Box<[T]>>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a T>;

            type IntoIter = Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: (&self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a crate::alpha::Alpha<$self_ty<$($phantom_ty,)? alloc::boxed::Box<[T]>>, alloc::boxed::Box<[T]>>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a T>, &'a T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>, core::slice::Iter<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&self.color).into_iter(),
                    alpha: (&self.alpha).into_iter(),
                }
            }
        }

        impl<'a, $($phantom_ty,)? T, const N: usize> IntoIterator for &'a mut $self_ty<$($phantom_ty,)? [T; N]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a mut T>;

            type IntoIter = Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: (&mut self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, $($phantom_ty,)? T, const N: usize> IntoIterator for &'a mut crate::alpha::Alpha<$self_ty<$($phantom_ty,)? [T; N]>, [T; N]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a mut T>, &'a mut T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>, core::slice::IterMut<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&mut self.color).into_iter(),
                    alpha: (&mut self.alpha).into_iter(),
                }
            }
        }

        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a mut $self_ty<$($phantom_ty,)? &'b mut [T]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a mut T>;

            type IntoIter = Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: self.$element.into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a mut crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'b mut [T]>, &'b mut [T]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a mut T>, &'a mut T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>, core::slice::IterMut<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&mut self.color).into_iter(),
                    alpha: (self.alpha).into_iter(),
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, $($phantom_ty,)? T> IntoIterator for &'a mut $self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a mut T>;

            type IntoIter = Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: (&mut self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a mut crate::alpha::Alpha<$self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>, alloc::vec::Vec<T>>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a mut T>, &'a mut T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>, core::slice::IterMut<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&mut self.color).into_iter(),
                    alpha: (&mut self.alpha).into_iter(),
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, $($phantom_ty,)? T> IntoIterator for &'a mut $self_ty<$($phantom_ty,)? alloc::boxed::Box<[T]>>
        where
            T: 'a
        {
            type Item = $self_ty<$($phantom_ty,)? &'a mut T>;

            type IntoIter = Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    $($element: (&mut *self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a mut crate::alpha::Alpha<$self_ty<$($phantom_ty,)? alloc::boxed::Box<[T]>>, alloc::boxed::Box<[T]>>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a mut T>, &'a mut T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>, core::slice::IterMut<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&mut self.color).into_iter(),
                    alpha: (&mut *self.alpha).into_iter(),
                }
            }
        }

        #[doc = concat!("An iterator for [`", stringify!($self_ty), "`] values.")]
        pub struct Iter<I $(,$phantom_ty)?> {
            $(pub(crate) $element: I,)+
            $(pub(crate) $phantom: core::marker::PhantomData<$phantom_ty>)?
        }

        impl<I $(,$phantom_ty)?> Iterator for Iter<I $(,$phantom_ty)?>
        where
            I: Iterator,
        {
            type Item = $self_ty<$($phantom_ty,)? I::Item>;

            #[inline(always)]
            fn next(&mut self) -> Option<Self::Item> {
                $(let $element = self.$element.next();)+

                if let ($(Some($element),)+) = ($($element,)+) {
                    Some($self_ty {
                        $($element,)+
                        $($phantom: core::marker::PhantomData,)?
                    })
                } else {
                    None
                }
            }

            #[inline(always)]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let hint = first!($((self.$element)),+).size_hint();
                skip_first!($((debug_assert_eq!(self.$element.size_hint(), hint, "the component iterators have different size hints");)),+);

                hint
            }

            #[inline(always)]
            fn count(self) -> usize {
                let count = first!($((self.$element)),+).count();
                skip_first!($((debug_assert_eq!(self.$element.count(), count, "the component iterators have different counts");)),+);

                count
            }
        }

        impl<I $(,$phantom_ty)?> DoubleEndedIterator for Iter<I $(,$phantom_ty)?>
        where
            I: DoubleEndedIterator,
        {
            #[inline(always)]
            fn next_back(&mut self) -> Option<Self::Item> {
                $(let $element = self.$element.next_back();)+

                if let ($(Some($element),)+) = ($($element,)+) {
                    Some($self_ty {
                        $($element,)+
                        $($phantom: core::marker::PhantomData,)?
                    })
                } else {
                    None
                }
            }
        }

        impl<I $(,$phantom_ty)?> ExactSizeIterator for Iter<I $(,$phantom_ty)?>
        where
            I: ExactSizeIterator,
        {
            #[inline(always)]
            fn len(&self) -> usize {
                let len = first!($((self.$element)),+).len();
                skip_first!($((debug_assert_eq!(self.$element.len(), len, "the component iterators have different lengths");)),+);

                len
            }
        }
    }
}

macro_rules! impl_struct_of_array_traits_hue {
    (  $self_ty: ident, $hue_iter_ty: ident, [$($element: ident),+] $(, $phantom: ident)?) => {
        impl_struct_of_array_traits_hue!($self_ty<>, $hue_iter_ty, [$($element),+] $(, $phantom)?);
    };
    (  $self_ty: ident < $($phantom_ty: ident)? > , $hue_iter_ty: ident, [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($phantom_ty,)? T, C> Extend<$self_ty<$($phantom_ty,)? T>> for $self_ty<$($phantom_ty,)? C>
        where
            C: Extend<T>,
        {
            #[inline(always)]
            fn extend<I: IntoIterator<Item = $self_ty<$($phantom_ty,)? T>>>(&mut self, iter: I) {
                let iter = iter.into_iter();

                for color in iter {
                    self.hue.extend(core::iter::once(color.hue.into_inner()));
                    $(self.$element.extend(core::iter::once(color.$element));)+
                }
            }
        }

        impl<$($phantom_ty,)? T, C> core::iter::FromIterator<$self_ty<$($phantom_ty,)? T>> for $self_ty<$($phantom_ty,)? C>
        where
            Self: Extend<$self_ty<$($phantom_ty,)? T>>,
            C: Default,
        {
            #[inline(always)]
            fn from_iter<I: IntoIterator<Item = $self_ty<$($phantom_ty,)? T>>>(iter: I) -> Self {
                let mut result = Self {
                    hue: C::default().into(),
                    $($element: C::default(),)+
                    $($phantom: core::marker::PhantomData)?
                };
                result.extend(iter);

                result
            }
        }

        impl<$($phantom_ty,)? T, const N: usize> IntoIterator for $self_ty<$($phantom_ty,)? [T; N]>
        {
            type Item = $self_ty<$($phantom_ty,)? T>;

            type IntoIter = Iter<core::array::IntoIter<T, N> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: self.hue.into_iter(),
                    $($element: IntoIterator::into_iter(self.$element),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<$($phantom_ty,)? T, const N: usize> IntoIterator for crate::alpha::Alpha<$self_ty<$($phantom_ty,)? [T; N]>, [T; N]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? T>, T>;

            type IntoIter = crate::alpha::Iter<Iter<core::array::IntoIter<T, N> $(,$phantom_ty)?>, core::array::IntoIter<T, N>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: self.color.into_iter(),
                    alpha: IntoIterator::into_iter(self.alpha)
                }
            }
        }

        impl<'a, $($phantom_ty,)? T> IntoIterator for $self_ty<$($phantom_ty,)? &'a [T]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a T>;

            type IntoIter = Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: self.hue.into_iter(),
                    $($element: self.$element.into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, $($phantom_ty,)? T> IntoIterator for crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a [T]>, &'a [T]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a T>, &'a T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>, core::slice::Iter<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: self.color.into_iter(),
                    alpha: self.alpha.into_iter(),
                }
            }
        }

        impl<'a, $($phantom_ty,)? T> IntoIterator for $self_ty<$($phantom_ty,)? &'a mut [T]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a mut T>;

            type IntoIter = Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: self.hue.into_iter(),
                    $($element: self.$element.into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, $($phantom_ty,)? T> IntoIterator for crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a mut [T]>, &'a mut [T]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a mut T>, &'a mut T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>, core::slice::IterMut<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: self.color.into_iter(),
                    alpha: self.alpha.into_iter(),
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<$($phantom_ty,)? T> IntoIterator for $self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>
        {
            type Item = $self_ty<$($phantom_ty,)? T>;

            type IntoIter = Iter<alloc::vec::IntoIter<T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: self.hue.into_iter(),
                    $($element: self.$element.into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, $($phantom_ty,)? T> IntoIterator for crate::alpha::Alpha<$self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>, alloc::vec::Vec<T>>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? T>, T>;

            type IntoIter = crate::alpha::Iter<Iter<alloc::vec::IntoIter<T> $(,$phantom_ty)?>, alloc::vec::IntoIter<T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: self.color.into_iter(),
                    alpha: self.alpha.into_iter(),
                }
            }
        }

        impl<'a, $($phantom_ty,)? T, const N: usize> IntoIterator for &'a $self_ty<$($phantom_ty,)? [T; N]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a T>;

            type IntoIter = Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: (&self.hue).into_iter(),
                    $($element: (&self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, $($phantom_ty,)? T, const N: usize> IntoIterator for &'a crate::alpha::Alpha<$self_ty<$($phantom_ty,)? [T; N]>, [T; N]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a T>, &'a T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>, core::slice::Iter<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&self.color).into_iter(),
                    alpha: (&self.alpha).into_iter(),
                }
            }
        }

        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a $self_ty<$($phantom_ty,)? &'b [T]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a T>;

            type IntoIter = Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: self.hue.into_iter(),
                    $($element: (&self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'b [T]>, &'b [T]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a T>, &'a T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>, core::slice::Iter<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: self.color.into_iter(),
                    alpha: self.alpha.into_iter(),
                }
            }
        }

        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a $self_ty<$($phantom_ty,)? &'b mut [T]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a T>;

            type IntoIter = Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: (&self.hue).into_iter(),
                    $($element: (&*self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'b mut [T]>, &'b mut [T]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a T>, &'a T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>, core::slice::Iter<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&self.color).into_iter(),
                    alpha: (&*self.alpha).into_iter(),
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, $($phantom_ty,)? T> IntoIterator for &'a $self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a T>;

            type IntoIter = Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: (&self.hue).into_iter(),
                    $($element: (&self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a crate::alpha::Alpha<$self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>, alloc::vec::Vec<T>>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a T>, &'a T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>, core::slice::Iter<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&self.color).into_iter(),
                    alpha: (&self.alpha).into_iter(),
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, $($phantom_ty,)? T> IntoIterator for &'a $self_ty<$($phantom_ty,)? alloc::boxed::Box<[T]>>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a T>;

            type IntoIter = Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: (&self.hue).into_iter(),
                    $($element: (&self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a crate::alpha::Alpha<$self_ty<$($phantom_ty,)? alloc::boxed::Box<[T]>>, alloc::boxed::Box<[T]>>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a T>, &'a T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::Iter<'a, T> $(,$phantom_ty)?>, core::slice::Iter<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&self.color).into_iter(),
                    alpha: (&self.alpha).into_iter(),
                }
            }
        }

        impl<'a, $($phantom_ty,)? T, const N: usize> IntoIterator for &'a mut $self_ty<$($phantom_ty,)? [T; N]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a mut T>;

            type IntoIter = Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: (&mut self.hue).into_iter(),
                    $($element: (&mut self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, $($phantom_ty,)? T, const N: usize> IntoIterator for &'a mut crate::alpha::Alpha<$self_ty<$($phantom_ty,)? [T; N]>, [T; N]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a mut T>, &'a mut T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>, core::slice::IterMut<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&mut self.color).into_iter(),
                    alpha: (&mut self.alpha).into_iter(),
                }
            }
        }

        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a mut $self_ty<$($phantom_ty,)? &'b mut [T]>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a mut T>;

            type IntoIter = Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: (&mut self.hue).into_iter(),
                    $($element: self.$element.into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a mut crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'b mut [T]>, &'b mut [T]>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a mut T>, &'a mut T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>, core::slice::IterMut<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&mut self.color).into_iter(),
                    alpha: (self.alpha).into_iter(),
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, $($phantom_ty,)? T> IntoIterator for &'a mut $self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a mut T>;

            type IntoIter = Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: (&mut self.hue).into_iter(),
                    $($element: (&mut self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a mut crate::alpha::Alpha<$self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>, alloc::vec::Vec<T>>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a mut T>, &'a mut T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>, core::slice::IterMut<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&mut self.color).into_iter(),
                    alpha: (&mut self.alpha).into_iter(),
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, $($phantom_ty,)? T> IntoIterator for &'a mut $self_ty<$($phantom_ty,)? alloc::boxed::Box<[T]>>
        {
            type Item = $self_ty<$($phantom_ty,)? &'a mut T>;

            type IntoIter = Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                Iter {
                    hue: (&mut self.hue).into_iter(),
                    $($element: (&mut *self.$element).into_iter(),)+
                    $($phantom: core::marker::PhantomData)?
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, 'b, $($phantom_ty,)? T> IntoIterator for &'a mut crate::alpha::Alpha<$self_ty<$($phantom_ty,)? alloc::boxed::Box<[T]>>, alloc::boxed::Box<[T]>>
        {
            type Item = crate::alpha::Alpha<$self_ty<$($phantom_ty,)? &'a mut T>, &'a mut T>;

            type IntoIter = crate::alpha::Iter<Iter<core::slice::IterMut<'a, T> $(,$phantom_ty)?>, core::slice::IterMut<'a, T>>;

            fn into_iter(self) -> Self::IntoIter {
                crate::alpha::Iter {
                    color: (&mut self.color).into_iter(),
                    alpha: (&mut *self.alpha).into_iter(),
                }
            }
        }

        #[doc = concat!("An iterator for [`", stringify!($self_ty), "`] values.")]
        pub struct Iter<I $(,$phantom_ty)?> {
            pub(crate) hue: $hue_iter_ty<I>,
            $(pub(crate) $element: I,)+
            $(pub(crate) $phantom: core::marker::PhantomData<$phantom_ty>)?
        }

        impl<I $(,$phantom_ty)?> Iterator for Iter<I $(,$phantom_ty)?>
        where
            I: Iterator,
        {
            type Item = $self_ty<$($phantom_ty,)? I::Item>;

            #[inline(always)]
            fn next(&mut self) -> Option<Self::Item> {
                let hue = self.hue.next();
                $(let $element = self.$element.next();)+

                if let (Some(hue), $(Some($element),)+) = (hue, $($element,)+) {
                    Some($self_ty {hue $(, $element)+ $(, $phantom: core::marker::PhantomData)?})
                } else {
                    None
                }
            }

            #[inline(always)]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let hint = self.hue.size_hint();
                $(debug_assert_eq!(self.$element.size_hint(), hint, "the component iterators have different size hints");)+

                hint
            }

            #[inline(always)]
            fn count(self) -> usize {
                let count = self.hue.count();
                $(debug_assert_eq!(self.$element.count(), count, "the component iterators have different counts");)+

                count
            }
        }

        impl<I $(,$phantom_ty)?> DoubleEndedIterator for Iter<I $(,$phantom_ty)?>
        where
            I: DoubleEndedIterator,
        {
            #[inline(always)]
            fn next_back(&mut self) -> Option<Self::Item> {
                let hue = self.hue.next_back();
                $(let $element = self.$element.next_back();)+

                if let (Some(hue), $(Some($element),)+) = (hue, $($element,)+) {
                    Some($self_ty {hue $(, $element)+ $(, $phantom: core::marker::PhantomData)?})
                } else {
                    None
                }
            }
        }

        impl<I $(,$phantom_ty)?> ExactSizeIterator for Iter<I $(,$phantom_ty)?>
        where
            I: ExactSizeIterator,
        {
            #[inline(always)]
            fn len(&self) -> usize {
                let len = self.hue.len();
                $(debug_assert_eq!(self.$element.len(), len, "the component iterators have different lengths");)+

                len
            }
        }
    }
}

macro_rules! impl_struct_of_arrays_methods {
    (  $self_ty: ident , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl_struct_of_arrays_methods!($self_ty<>, [$($element),+] $(, $phantom)?);
    };
    (  $self_ty: ident < $($phantom_ty: ident)? > , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($phantom_ty,)? C> $self_ty<$($phantom_ty,)? C> {
            /// Return an iterator over the colors in the wrapped collections.
            #[inline(always)]
            pub fn iter<'a>(&'a self) -> <&'a Self as IntoIterator>::IntoIter where &'a Self: IntoIterator {
                self.into_iter()
            }

            /// Return an iterator that allows modifying the colors in the wrapped collections.
            #[inline(always)]
            pub fn iter_mut<'a>(&'a mut self) -> <&'a mut Self as IntoIterator>::IntoIter where &'a mut Self: IntoIterator {
                self.into_iter()
            }

            /// Get a color, or slice of colors, with references to the components at `index`. See [`slice::get`] for details.
            #[inline(always)]
            pub fn get<'a, I, T>(&'a self, index: I) -> Option<$self_ty<$($phantom_ty,)? &<I as core::slice::SliceIndex<[T]>>::Output>>
            where
                T: 'a,
                C: AsRef<[T]>,
                I: core::slice::SliceIndex<[T]> + Clone,
            {
                $(let $element = self.$element.as_ref().get(index.clone());)+

                if let ($(Some($element),)+) = ($($element,)+) {
                    Some($self_ty {
                        $($element,)+
                        $($phantom: core::marker::PhantomData,)?
                    })
                } else {
                    None
                }
            }

            /// Get a color, or slice of colors, that allows modifying the components at `index`. See [`slice::get_mut`] for details.
            #[inline(always)]
            pub fn get_mut<'a, I, T>(&'a mut self, index: I) -> Option<$self_ty<$($phantom_ty,)? &mut <I as core::slice::SliceIndex<[T]>>::Output>>
            where
                T: 'a,
                C: AsMut<[T]>,
                I: core::slice::SliceIndex<[T]> + Clone,
            {
                $(let $element = self.$element.as_mut().get_mut(index.clone());)+

                if let ($(Some($element),)+) = ($($element,)+) {
                    Some($self_ty {
                        $($element,)+
                        $($phantom: core::marker::PhantomData,)?
                    })
                } else {
                    None
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<$($phantom_ty,)? T> $self_ty<$($phantom_ty,)? alloc::vec::Vec<T>> {
            /// Create a struct of vectors with a minimum capacity. See [`Vec::with_capacity`] for details.
            #[inline(always)]
            pub fn with_capacity(capacity: usize) -> Self {
                $(let $element = alloc::vec::Vec::with_capacity(capacity);)+

                Self {
                    $($element,)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            /// Push an additional color's components onto the component vectors. See [`Vec::push`] for details.
            #[inline(always)]
            pub fn push(&mut self, value: $self_ty<$($phantom_ty,)? T>) {
                $(self.$element.push(value.$element);)+
            }

            /// Pop a color's components from the component vectors. See [`Vec::pop`] for details.
            #[inline(always)]
            pub fn pop(&mut self) -> Option<$self_ty<$($phantom_ty,)? T>> {
                $(let $element = self.$element.pop();)+

                Some($self_ty {
                    $($element: $element?,)+
                    $($phantom: core::marker::PhantomData,)?
                })
            }

            /// Clear the component vectors. See [`Vec::clear`] for details.
            #[inline(always)]
            pub fn clear(&mut self) {
                $(self.$element.clear();)+
            }

            /// Return an iterator that moves colors out of the specified range.
            #[inline(always)]
            pub fn drain<R>(&mut self, range: R) -> Iter<alloc::vec::Drain<T> $(, $phantom_ty)?>
            where
                R: core::ops::RangeBounds<usize> + Clone,
            {
                Iter {
                    $($element: self.$element.drain(range.clone()),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($phantom_ty,)? Ct, Ca> crate::Alpha<$self_ty<$($phantom_ty,)? Ct>, Ca> {
            /// Get a color, or slice of colors, with references to the components at `index`. See [`slice::get`] for details.
            #[inline(always)]
            pub fn get<'a, I, T, A>(&'a self, index: I) -> Option<crate::Alpha<
                $self_ty<$($phantom_ty,)? &<I as core::slice::SliceIndex<[T]>>::Output>,
                &<I as core::slice::SliceIndex<[A]>>::Output
            >>
            where
                T: 'a,
                A: 'a,
                Ct: AsRef<[T]>,
                Ca: AsRef<[A]>,
                I: core::slice::SliceIndex<[T]> + core::slice::SliceIndex<[A]> + Clone
            {
                let color = self.color.get(index.clone());
                let alpha = self.alpha.as_ref().get(index);

                if let (Some(color), Some(alpha)) = (color, alpha) {
                    Some(crate::Alpha{color, alpha})
                } else {
                    None
                }
            }

            /// Get a color, or slice of colors, that allows modifying the components at `index`. See [`slice::get_mut`] for details.
            #[inline(always)]
            pub fn get_mut<'a, I, T, A>(&'a mut self, index: I) -> Option<crate::Alpha<
                $self_ty<$($phantom_ty,)? &mut <I as core::slice::SliceIndex<[T]>>::Output>,
                &mut <I as core::slice::SliceIndex<[A]>>::Output
            >>
            where
                T: 'a,
                A: 'a,
                Ct: AsMut<[T]>,
                Ca: AsMut<[A]>,
                I: core::slice::SliceIndex<[T]> + core::slice::SliceIndex<[A]> + Clone
            {
                let color = self.color.get_mut(index.clone());
                let alpha = self.alpha.as_mut().get_mut(index);

                if let (Some(color), Some(alpha)) = (color, alpha) {
                    Some(crate::Alpha{color, alpha})
                } else {
                    None
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<$($phantom_ty,)? T, A> crate::Alpha<$self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>, alloc::vec::Vec<A>> {
            /// Create a struct of vectors with a minimum capacity. See [`Vec::with_capacity`] for details.
            #[inline(always)]
            pub fn with_capacity(capacity: usize) -> Self {
                crate::Alpha {
                    color: $self_ty::with_capacity(capacity),
                    alpha: alloc::vec::Vec::with_capacity(capacity),
                }
            }

            /// Push an additional color's components onto the component vectors. See [`Vec::push`] for details.
            #[inline(always)]
            pub fn push(&mut self, value: crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>) {
                self.color.push(value.color);
                self.alpha.push(value.alpha);
            }

            /// Pop a color's components from the component vectors. See [`Vec::pop`] for details.
            #[inline(always)]
            pub fn pop(&mut self) -> Option<crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>> {
                let color = self.color.pop();
                let alpha = self.alpha.pop();

                Some(crate::Alpha {
                    color: color?,
                    alpha: alpha?,
                })
            }

            /// Clear the component vectors. See [`Vec::clear`] for details.
            #[inline(always)]
            pub fn clear(&mut self) {
                self.color.clear();
                self.alpha.clear();
            }

            /// Return an iterator that moves colors out of the specified range.
            #[inline(always)]
            pub fn drain<R>(&mut self, range: R) -> crate::alpha::Iter<Iter<alloc::vec::Drain<T> $(, $phantom_ty)?>, alloc::vec::Drain<A>>
            where
                R: core::ops::RangeBounds<usize> + Clone,
            {
                crate::alpha::Iter {
                    color: self.color.drain(range.clone()),
                    alpha: self.alpha.drain(range),
                }
            }
        }
    };
}

macro_rules! impl_struct_of_arrays_methods_hue {
    (  $self_ty: ident , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl_struct_of_arrays_methods_hue!($self_ty<>, [$($element),+] $(, $phantom)?);
    };
    (  $self_ty: ident < $($phantom_ty: ident)? > , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($phantom_ty,)? C> $self_ty<$($phantom_ty,)? C> {
            /// Return an iterator over the colors in the wrapped collections.
            #[inline(always)]
            pub fn iter<'a>(&'a self) -> <&'a Self as IntoIterator>::IntoIter where &'a Self: IntoIterator {
                self.into_iter()
            }

            /// Return an iterator that allows modifying the colors in the wrapped collections.
            #[inline(always)]
            pub fn iter_mut<'a>(&'a mut self) -> <&'a mut Self as IntoIterator>::IntoIter where &'a mut Self: IntoIterator {
                self.into_iter()
            }

            /// Get a color, or slice of colors, with references to the components at `index`. See [`slice::get`] for details.
            #[inline(always)]
            pub fn get<'a, I, T>(&'a self, index: I) -> Option<$self_ty<$($phantom_ty,)? &<I as core::slice::SliceIndex<[T]>>::Output>>
            where
                T: 'a,
                C: AsRef<[T]>,
                I: core::slice::SliceIndex<[T]> + Clone,
            {
                let hue = self.hue.get(index.clone());
                $(let $element = self.$element.as_ref().get(index.clone());)+

                if let (Some(hue) $(, Some($element))+) = (hue $(,$element)+) {
                    Some($self_ty {hue $(, $element)+ $(, $phantom: core::marker::PhantomData)?})
                } else {
                    None
                }
            }

            /// Get a color, or slice of colors, that allows modifying the components at `index`. See [`slice::get_mut`] for details.
            #[inline(always)]
            pub fn get_mut<'a, I, T>(&'a mut self, index: I) -> Option<$self_ty<$($phantom_ty,)? &mut <I as core::slice::SliceIndex<[T]>>::Output>>
            where
                T: 'a,
                C: AsMut<[T]>,
                I: core::slice::SliceIndex<[T]> + Clone,
            {
                let hue = self.hue.get_mut(index.clone());
                $(let $element = self.$element.as_mut().get_mut(index.clone());)+

                if let (Some(hue) $(, Some($element))+) = (hue $(,$element)+) {
                    Some($self_ty {hue $(, $element)+ $(, $phantom: core::marker::PhantomData)?})
                } else {
                    None
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<$($phantom_ty,)? T> $self_ty<$($phantom_ty,)? alloc::vec::Vec<T>> {
            /// Create a struct of vectors with a minimum capacity. See [`Vec::with_capacity`] for details.
            #[inline(always)]
            pub fn with_capacity(capacity: usize) -> Self {
                let hue = alloc::vec::Vec::with_capacity(capacity);
                $(let $element = alloc::vec::Vec::with_capacity(capacity);)+

                Self {hue: hue.into() $(, $element)+ $(, $phantom: core::marker::PhantomData)?}
            }

            /// Push an additional color's components onto the component vectors. See [`Vec::push`] for details.
            #[inline(always)]
            pub fn push(&mut self, value: $self_ty<$($phantom_ty,)? T>) {
                self.hue.push(value.hue);
                $(self.$element.push(value.$element);)+
            }

            /// Pop a color's components from the component vectors. See [`Vec::pop`] for details.
            #[inline(always)]
            pub fn pop(&mut self) -> Option<$self_ty<$($phantom_ty,)? T>> {
                let hue = self.hue.pop();
                $(let $element = self.$element.pop();)+

                Some($self_ty {
                    hue: hue?,
                    $($element: $element?,)+
                    $($phantom: core::marker::PhantomData,)?
                })
            }

            /// Clear the component vectors. See [`Vec::clear`] for details.
            #[inline(always)]
            pub fn clear(&mut self) {
                self.hue.clear();
                $(self.$element.clear();)+
            }

            /// Return an iterator that moves colors out of the specified range.
            #[inline(always)]
            pub fn drain<R>(&mut self, range: R) -> Iter<alloc::vec::Drain<T> $(, $phantom_ty)?>
            where
                R: core::ops::RangeBounds<usize> + Clone,
            {
                Iter {
                    hue: self.hue.drain(range.clone()),
                    $($element: self.$element.drain(range.clone()),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($phantom_ty,)? Ct, Ca> crate::Alpha<$self_ty<$($phantom_ty,)? Ct>, Ca> {
            /// Get a color, or slice of colors, with references to the components at `index`. See [`slice::get`] for details.
            #[inline(always)]
            pub fn get<'a, I, T, A>(&'a self, index: I) -> Option<crate::Alpha<
                $self_ty<$($phantom_ty,)? &<I as core::slice::SliceIndex<[T]>>::Output>,
                &<I as core::slice::SliceIndex<[A]>>::Output
            >>
            where
                T: 'a,
                A: 'a,
                Ct: AsRef<[T]>,
                Ca: AsRef<[A]>,
                I: core::slice::SliceIndex<[T]> + core::slice::SliceIndex<[A]> + Clone
            {
                let color = self.color.get(index.clone());
                let alpha = self.alpha.as_ref().get(index);

                if let (Some(color), Some(alpha)) = (color, alpha) {
                    Some(crate::Alpha{color, alpha})
                } else {
                    None
                }
            }

            /// Get a color, or slice of colors, that allows modifying the components at `index`. See [`slice::get_mut`] for details.
            #[inline(always)]
            pub fn get_mut<'a, I, T, A>(&'a mut self, index: I) -> Option<crate::Alpha<
                $self_ty<$($phantom_ty,)? &mut <I as core::slice::SliceIndex<[T]>>::Output>,
                &mut <I as core::slice::SliceIndex<[A]>>::Output
            >>
            where
                T: 'a,
                A: 'a,
                Ct: AsMut<[T]>,
                Ca: AsMut<[A]>,
                I: core::slice::SliceIndex<[T]> + core::slice::SliceIndex<[A]> + Clone
            {
                let color = self.color.get_mut(index.clone());
                let alpha = self.alpha.as_mut().get_mut(index);

                if let (Some(color), Some(alpha)) = (color, alpha) {
                    Some(crate::Alpha{color, alpha})
                } else {
                    None
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<$($phantom_ty,)? T, A> crate::Alpha<$self_ty<$($phantom_ty,)? alloc::vec::Vec<T>>, alloc::vec::Vec<A>> {
            /// Create a struct of vectors with a minimum capacity. See [`Vec::with_capacity`] for details.
            #[inline(always)]
            pub fn with_capacity(capacity: usize) -> Self {
                crate::Alpha {
                    color: $self_ty::with_capacity(capacity),
                    alpha: alloc::vec::Vec::with_capacity(capacity),
                }
            }

            /// Push an additional color's components onto the component vectors. See [`Vec::push`] for details.
            #[inline(always)]
            pub fn push(&mut self, value: crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>) {
                self.color.push(value.color);
                self.alpha.push(value.alpha);
            }

            /// Pop a color's components from the component vectors. See [`Vec::pop`] for details.
            #[inline(always)]
            pub fn pop(&mut self) -> Option<crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>> {
                let color = self.color.pop();
                let alpha = self.alpha.pop();

                Some(crate::Alpha {
                    color: color?,
                    alpha: alpha?,
                })
            }

            /// Clear the component vectors. See [`Vec::clear`] for details.
            #[inline(always)]
            pub fn clear(&mut self) {
                self.color.clear();
                self.alpha.clear();
            }

            /// Return an iterator that moves colors out of the specified range.
            #[inline(always)]
            pub fn drain<R>(&mut self, range: R) -> crate::alpha::Iter<Iter<alloc::vec::Drain<T> $(, $phantom_ty)?>, alloc::vec::Drain<A>>
            where
                R: core::ops::RangeBounds<usize> + Clone,
            {
                crate::alpha::Iter {
                    color: self.color.drain(range.clone()),
                    alpha: self.alpha.drain(range),
                }
            }
        }
    };
}

#[cfg(test)]
macro_rules! struct_of_arrays_tests {
    ($color_ty: ident $(<$phantom_ty:ty>)? [$($element: ident),+] $(phantom: $phantom: ident)?, $($values:expr),+) => {
        #[cfg(feature = "alloc")]
        #[test]
        fn collect() {
            let vec_of_colors = vec![$($values.color),+];
            let color_of_vecs: $color_ty<$($phantom_ty,)? Vec<_>> = vec_of_colors.into_iter().collect();
            let vec_of_colors: Vec<_> = color_of_vecs.into_iter().collect();

            assert_eq!(vec_of_colors, vec![$($values.color),+]);
        }

        #[cfg(feature = "alloc")]
        #[test]
        fn collect_alpha() {
            let vec_of_colors = vec![$($values),+];
            let color_of_vecs: crate::alpha::Alpha<$color_ty<$($phantom_ty,)? Vec<_>>, Vec<_>> = vec_of_colors.into_iter().collect();
            let vec_of_colors: Vec<_> = color_of_vecs.into_iter().collect();

            assert_eq!(vec_of_colors, vec![$($values),+]);
        }


        #[cfg(feature = "alloc")]
        #[test]
        fn extend() {
            let vec_of_colors = vec![$($values.color),+];

            let mut color_of_vecs = $color_ty::<$($phantom_ty,)? Vec<_>>::with_capacity(vec_of_colors.len());
            color_of_vecs.extend(vec_of_colors);

            let vec_of_colors: Vec<_> = color_of_vecs.into_iter().collect();

            assert_eq!(vec_of_colors, vec![$($values.color),+]);
        }


        #[cfg(feature = "alloc")]
        #[test]
        fn extend_alpha() {
            let vec_of_colors = vec![$($values),+];

            let mut color_of_vecs = crate::alpha::Alpha::<$color_ty<$($phantom_ty,)? Vec<_>>, Vec<_>>::with_capacity(vec_of_colors.len());
            color_of_vecs.extend(vec_of_colors);

            let vec_of_colors: Vec<_> = color_of_vecs.into_iter().collect();

            assert_eq!(vec_of_colors, vec![$($values),+]);
        }


        #[cfg(feature = "alloc")]
        #[test]
        fn pop_push() {
            let vec_of_colors = vec![$($values.color),+];

            let mut color_of_vecs: $color_ty<$($phantom_ty,)? Vec<_>> = vec_of_colors.into_iter().collect();
            let last = color_of_vecs.pop().unwrap();
            color_of_vecs.push(last);

            let vec_of_colors: Vec<_> = color_of_vecs.into_iter().collect();

            assert_eq!(vec_of_colors, vec![$($values.color),+]);
        }


        #[cfg(feature = "alloc")]
        #[test]
        fn pop_push_alpha() {
            let vec_of_colors = vec![$($values),+];

            let mut color_of_vecs: crate::alpha::Alpha<$color_ty<$($phantom_ty,)? Vec<_>>, Vec<_>> = vec_of_colors.into_iter().collect();
            let last = color_of_vecs.pop().unwrap();
            color_of_vecs.push(last);

            let vec_of_colors: Vec<_> = color_of_vecs.into_iter().collect();

            assert_eq!(vec_of_colors, vec![$($values),+]);
        }

        #[cfg(feature = "alloc")]
        #[test]
        fn clear() {
            let vec_of_colors = vec![$($values.color),+];

            let mut color_of_vecs: $color_ty<$($phantom_ty,)? Vec<_>> = vec_of_colors.into_iter().collect();
            color_of_vecs.clear();

            let vec_of_colors: Vec<_> = color_of_vecs.into_iter().collect();

            assert_eq!(vec_of_colors, vec![]);
        }

        #[cfg(feature = "alloc")]
        #[test]
        fn clear_alpha() {
            let vec_of_colors = vec![$($values),+];

            let mut color_of_vecs: crate::alpha::Alpha<$color_ty<$($phantom_ty,)? Vec<_>>, Vec<_>> = vec_of_colors.into_iter().collect();
            color_of_vecs.clear();

            let vec_of_colors: Vec<_> = color_of_vecs.into_iter().collect();

            assert_eq!(vec_of_colors, vec![]);
        }


        #[cfg(feature = "alloc")]
        #[test]
        fn drain() {
            let vec_of_colors = vec![$($values.color),+];

            let mut color_of_vecs: $color_ty<$($phantom_ty,)? Vec<_>> = vec_of_colors.into_iter().collect();

            let vec_of_colors1: Vec<_> = color_of_vecs.drain(..).collect();
            let vec_of_colors2: Vec<_> = color_of_vecs.into_iter().collect();

            assert_eq!(vec_of_colors1, vec![$($values.color),+]);
            assert_eq!(vec_of_colors2, vec![]);
        }


        #[cfg(feature = "alloc")]
        #[test]
        fn drain_alpha() {
            let vec_of_colors = vec![$($values),+];

            let mut color_of_vecs: crate::alpha::Alpha<$color_ty<$($phantom_ty,)? Vec<_>>, Vec<_>> = vec_of_colors.into_iter().collect();

            let vec_of_colors1: Vec<_> = color_of_vecs.drain(..).collect();
            let vec_of_colors2: Vec<_> = color_of_vecs.into_iter().collect();

            assert_eq!(vec_of_colors1, vec![$($values),+]);
            assert_eq!(vec_of_colors2, vec![]);
        }

        #[cfg(feature = "alloc")]
        #[test]
        fn modify() {
            let vec_of_colors = vec![$($values.color),+];

            let mut color_of_vecs: $color_ty<$($phantom_ty,)? Vec<_>> = vec_of_colors.into_iter().collect();

            for mut color in &mut color_of_vecs {
                color.set(color.copied() + 2.0);
            }

            let vec_of_colors: Vec<_> = color_of_vecs.into_iter().collect();

            assert_eq!(vec_of_colors, vec![$($values.color + 2.0),+]);
        }

        #[cfg(feature = "alloc")]
        #[test]
        fn modify_alpha() {
            let vec_of_colors = vec![$($values),+];

            let mut color_of_vecs: crate::alpha::Alpha<$color_ty<$($phantom_ty,)? Vec<_>>, Vec<_>> = vec_of_colors.into_iter().collect();

            for mut color in &mut color_of_vecs {
                color.set(color.copied() + 2.0);
            }

            let vec_of_colors: Vec<_> = color_of_vecs.into_iter().collect();

            assert_eq!(vec_of_colors, vec![$($values + 2.0),+]);
        }

        #[test]
        fn into_iterator() {
            fn expect_move(_: impl Iterator<Item = $color_ty::<$($phantom_ty,)? f32>>){}
            fn expect_ref<'a>(_: impl Iterator<Item = $color_ty::<$($phantom_ty,)? &'a f32>>){}
            fn expect_ref_mut<'a>(_: impl Iterator<Item = $color_ty::<$($phantom_ty,)? &'a mut f32>>){}

            let arrays = $color_ty::<$($phantom_ty,)? [f32; 0]>{
                $($element: Default::default(),)+
                $($phantom: core::marker::PhantomData,)?
            };
            let slices = $color_ty::<$($phantom_ty,)? &[f32]>{
                $($element: Default::default(),)+
                $($phantom: core::marker::PhantomData,)?
            };
            let mut_slices = $color_ty::<$($phantom_ty,)? &mut [f32]>{
                $($element: Default::default(),)+
                $($phantom: core::marker::PhantomData,)?
            };

            expect_move(arrays.into_iter());
            expect_ref(slices.into_iter());
            expect_ref_mut(mut_slices.into_iter());
        }

        #[test]
        fn into_iterator_alpha() {
            use crate::alpha::Alpha;

            fn expect_move(_: impl Iterator<Item = Alpha<$color_ty::<$($phantom_ty,)? f32>, f32>>){}
            fn expect_ref<'a>(_: impl Iterator<Item = Alpha<$color_ty::<$($phantom_ty,)? &'a f32>, &'a f32>>){}
            fn expect_ref_mut<'a>(_: impl Iterator<Item = Alpha<$color_ty::<$($phantom_ty,)? &'a mut f32>, &'a mut f32>>){}

            let arrays = Alpha::<_, [f32; 0]>{
                color: $color_ty::<$($phantom_ty,)? [f32; 0]>{
                    $($element: Default::default(),)+
                    $($phantom: core::marker::PhantomData,)?
                },
                alpha: Default::default(),
            };
            let slices = Alpha::<_, &[f32]>{
                color: $color_ty::<$($phantom_ty,)? &[f32]>{
                    $($element: Default::default(),)+
                    $($phantom: core::marker::PhantomData,)?
                },
                alpha: Default::default(),
            };
            let mut_slices = Alpha::<_, &mut [f32]>{
                color: $color_ty::<$($phantom_ty,)? &mut [f32]>{
                    $($element: Default::default(),)+
                    $($phantom: core::marker::PhantomData,)?
                },
                alpha: Default::default(),
            };

            expect_move(arrays.into_iter());
            expect_ref(slices.into_iter());
            expect_ref_mut(mut_slices.into_iter());
        }

        #[test]
        fn into_iterator_ref() {
            fn expect_ref<'a>(_: impl Iterator<Item = $color_ty::<$($phantom_ty,)? &'a f32>>){}
            fn expect_ref_mut<'a>(_: impl Iterator<Item = $color_ty::<$($phantom_ty,)? &'a mut f32>>){}

            let mut arrays = $color_ty::<$($phantom_ty,)? [f32; 0]>{
                $($element: Default::default(),)+
                $($phantom: core::marker::PhantomData,)?
            };
            let mut slices = $color_ty::<$($phantom_ty,)? &[f32]>{
                $($element: Default::default(),)+
                $($phantom: core::marker::PhantomData,)?
            };
            let mut mut_slices = $color_ty::<$($phantom_ty,)? &mut [f32]>{
                $($element: Default::default(),)+
                $($phantom: core::marker::PhantomData,)?
            };

            expect_ref((&arrays).into_iter());
            expect_ref((&slices).into_iter());
            expect_ref((&mut_slices).into_iter());

            expect_ref_mut((&mut arrays).into_iter());
            expect_ref((&mut slices).into_iter());
            expect_ref_mut((&mut mut_slices).into_iter());
        }

        #[test]
        fn into_iterator_ref_alpha() {
            use crate::alpha::Alpha;

            fn expect_ref<'a>(_: impl Iterator<Item = Alpha<$color_ty::<$($phantom_ty,)? &'a f32>, &'a f32>>){}
            fn expect_ref_mut<'a>(_: impl Iterator<Item = Alpha<$color_ty::<$($phantom_ty,)? &'a mut f32>, &'a mut f32>>){}

            let mut arrays = Alpha::<_, [f32; 0]>{
                color: $color_ty::<$($phantom_ty,)? [f32; 0]>{
                    $($element: Default::default(),)+
                    $($phantom: core::marker::PhantomData,)?
                },
                alpha: Default::default(),
            };
            let mut slices = Alpha::<_, &[f32]>{
                color: $color_ty::<$($phantom_ty,)? &[f32]>{
                    $($element: Default::default(),)+
                    $($phantom: core::marker::PhantomData,)?
                },
                alpha: Default::default(),
            };
            let mut mut_slices = Alpha::<_, &mut [f32]>{
                color: $color_ty::<$($phantom_ty,)? &mut [f32]>{
                    $($element: Default::default(),)+
                    $($phantom: core::marker::PhantomData,)?
                },
                alpha: Default::default(),
            };

            expect_ref((&arrays).into_iter());
            expect_ref((&slices).into_iter());
            expect_ref((&mut_slices).into_iter());

            expect_ref_mut((&mut arrays).into_iter());
            expect_ref((&mut slices).into_iter());
            expect_ref_mut((&mut mut_slices).into_iter());
        }

        #[cfg(feature = "alloc")]
        #[test]
        fn into_iterator_alloc() {
            fn expect_move(_: impl Iterator<Item = $color_ty::<$($phantom_ty,)? f32>>){}
            fn expect_ref<'a>(_: impl Iterator<Item = $color_ty::<$($phantom_ty,)? &'a f32>>){}

            let vecs = $color_ty::<$($phantom_ty,)? Vec<f32>>{
                $($element: Default::default(),)+
                $($phantom: core::marker::PhantomData,)?
            };
            let boxed_slices = $color_ty::<$($phantom_ty,)? Box<[f32]>>{
                $($element: Default::default(),)+
                $($phantom: core::marker::PhantomData,)?
            };

            expect_move(vecs.into_iter());
            expect_ref(boxed_slices.into_iter());
        }

        #[cfg(feature = "alloc")]
        #[test]
        fn into_iterator_alloc_alpha() {
            use crate::alpha::Alpha;

            fn expect_move(_: impl Iterator<Item = Alpha<$color_ty::<$($phantom_ty,)? f32>, f32>>){}
            fn expect_ref<'a>(_: impl Iterator<Item = Alpha<$color_ty::<$($phantom_ty,)? &'a f32>, &'a f32>>){}

            let vecs = Alpha::<_, Vec<f32>>{
                color: $color_ty::<$($phantom_ty,)? Vec<f32>>{
                    $($element: Default::default(),)+
                    $($phantom: core::marker::PhantomData,)?
                },
                alpha: Default::default(),
            };
            let boxed_slices = Alpha::<_, Box<[f32]>>{
                color: $color_ty::<$($phantom_ty,)? Box<[f32]>>{
                    $($element: Default::default(),)+
                    $($phantom: core::marker::PhantomData,)?
                },
                alpha: Default::default(),
            };

            expect_move(vecs.into_iter());
            expect_ref(boxed_slices.into_iter());
        }

        #[cfg(feature = "alloc")]
        #[test]
        fn into_iterator_alloc_ref() {
            fn expect_ref<'a>(_: impl Iterator<Item = $color_ty::<$($phantom_ty,)? &'a f32>>){}
            fn expect_ref_mut<'a>(_: impl Iterator<Item = $color_ty::<$($phantom_ty,)? &'a mut f32>>){}

            let mut vecs = $color_ty::<$($phantom_ty,)? Vec<f32>>{
                $($element: Default::default(),)+
                $($phantom: core::marker::PhantomData,)?
            };
            let mut boxed_slices = $color_ty::<$($phantom_ty,)? Box<[f32]>>{
                $($element: Default::default(),)+
                $($phantom: core::marker::PhantomData,)?
            };

            expect_ref((&vecs).into_iter());
            expect_ref((&boxed_slices).into_iter());

            expect_ref_mut((&mut vecs).into_iter());
            expect_ref_mut((&mut boxed_slices).into_iter());
        }

        #[cfg(feature = "alloc")]
        #[test]
        fn into_iterator_alloc_ref_alpha() {
            use crate::alpha::Alpha;

            fn expect_ref<'a>(_: impl Iterator<Item = Alpha<$color_ty::<$($phantom_ty,)? &'a f32>, &'a f32>>){}
            fn expect_ref_mut<'a>(_: impl Iterator<Item = Alpha<$color_ty::<$($phantom_ty,)? &'a mut f32>, &'a mut f32>>){}

            let mut vecs = Alpha::<_, Vec<f32>>{
                color: $color_ty::<$($phantom_ty,)? Vec<f32>>{
                    $($element: Default::default(),)+
                    $($phantom: core::marker::PhantomData,)?
                },
                alpha: Default::default(),
            };
            let mut boxed_slices = Alpha::<_, Box<[f32]>>{
                color: $color_ty::<$($phantom_ty,)? Box<[f32]>>{
                    $($element: Default::default(),)+
                    $($phantom: core::marker::PhantomData,)?
                },
                alpha: Default::default(),
            };

            expect_ref((&vecs).into_iter());
            expect_ref((&boxed_slices).into_iter());

            expect_ref_mut((&mut vecs).into_iter());
            expect_ref_mut((&mut boxed_slices).into_iter());
        }
    }
}
