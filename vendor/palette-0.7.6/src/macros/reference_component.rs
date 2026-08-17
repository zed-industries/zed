macro_rules! impl_reference_component_methods {
    (  $self_ty: ident , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl_reference_component_methods!($self_ty<>, [$($element),+] $(, $phantom)?);
    };
    (  $self_ty: ident < $($phantom_ty: ident)? > , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($phantom_ty,)? T> $self_ty<$($phantom_ty,)? &T> {
            /// Get an owned, copied version of this color.
            #[inline]
            pub fn copied(&self) -> $self_ty<$($phantom_ty,)? T>
            where
                T: Copy,
            {
                $self_ty {
                    $($element: *self.$element,)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            /// Get an owned, cloned version of this color.
            #[inline]
            pub fn cloned(&self) -> $self_ty<$($phantom_ty,)? T>
            where
                T: Clone,
            {
                $self_ty {
                    $($element: self.$element.clone(),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($phantom_ty,)? T> $self_ty<$($phantom_ty,)? &mut T> {
            /// Update this color with new values.
            #[inline]
            pub fn set(&mut self, value: $self_ty<$($phantom_ty,)? T>) {
                $(*self.$element = value.$element;)+
            }

            /// Borrow this color's components as shared references.
            #[inline]
            pub fn as_refs(&self) -> $self_ty<$($phantom_ty,)? &T> {
                $self_ty {
                    $($element: &*self.$element,)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            /// Get an owned, copied version of this color.
            #[inline]
            pub fn copied(&self) -> $self_ty<$($phantom_ty,)? T>
            where
                T: Copy,
            {
                $self_ty {
                    $($element: *self.$element,)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            /// Get an owned, cloned version of this color.
            #[inline]
            pub fn cloned(&self) -> $self_ty<$($phantom_ty,)? T>
            where
                T: Clone,
            {
                $self_ty {
                    $($element: self.$element.clone(),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($phantom_ty,)? T, A> crate::Alpha<$self_ty<$($phantom_ty,)? &T>, &A> {
            /// Get an owned, copied version of this color.
            #[inline]
            pub fn copied(&self) -> crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>
            where
                T: Copy,
                A: Copy,
            {
                crate::Alpha{color: self.color.copied(), alpha: *self.alpha}
            }

            /// Get an owned, cloned version of this color.
            #[inline]
            pub fn cloned(&self) -> crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>
            where
                T: Clone,
                A: Clone,
            {
                crate::Alpha{color: self.color.cloned(), alpha: self.alpha.clone()}
            }
        }

        impl<$($phantom_ty,)? T, A> crate::Alpha<$self_ty<$($phantom_ty,)? &mut T>, &mut A> {
            /// Update this color with new values.
            #[inline]
            pub fn set(&mut self, value: crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>) {
                self.color.set(value.color);
                *self.alpha = value.alpha;
            }

            /// Borrow this color's components as shared references.
            #[inline]
            pub fn as_refs(&self) -> crate::Alpha<$self_ty<$($phantom_ty,)? &T>, &A>{
                crate::Alpha{color: self.color.as_refs(), alpha: &*self.alpha}
            }

            /// Get an owned, copied version of this color.
            #[inline]
            pub fn copied(&self) -> crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>
            where
                T: Copy,
                A: Copy,
            {
                crate::Alpha{color: self.color.copied(), alpha: *self.alpha}
            }

            /// Get an owned, cloned version of this color.
            #[inline]
            pub fn cloned(&self) -> crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>
            where
                T: Clone,
                A: Clone,
            {
                crate::Alpha{color: self.color.cloned(), alpha: self.alpha.clone()}
            }
        }
    }
}

macro_rules! impl_reference_component_methods_hue {
    (  $self_ty: ident , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl_reference_component_methods_hue!($self_ty<>, [$($element),+] $(, $phantom)?);
    };
    (  $self_ty: ident < $($phantom_ty: ident)? > , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($phantom_ty,)? T> $self_ty<$($phantom_ty,)? &T> {
            /// Get an owned, copied version of this color.
            #[inline]
            pub fn copied(&self) -> $self_ty<$($phantom_ty,)? T>
            where
                T: Copy,
            {
                $self_ty {
                    hue: self.hue.copied(),
                    $($element: *self.$element,)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            /// Get an owned, cloned version of this color.
            #[inline]
            pub fn cloned(&self) -> $self_ty<$($phantom_ty,)? T>
            where
                T: Clone,
            {
                $self_ty {
                    hue: self.hue.cloned(),
                    $($element: self.$element.clone(),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($phantom_ty,)? T> $self_ty<$($phantom_ty,)? &mut T> {
            /// Update this color with new values.
            #[inline]
            pub fn set(&mut self, value: $self_ty<$($phantom_ty,)? T>) {
                self.hue.set(value.hue);
                $(*self.$element = value.$element;)+
            }

            /// Borrow this color's components as shared references.
            #[inline]
            pub fn as_refs(&self) -> $self_ty<$($phantom_ty,)? &T> {
                $self_ty {
                    hue: self.hue.as_ref(),
                    $($element: &*self.$element,)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            /// Get an owned, copied version of this color.
            #[inline]
            pub fn copied(&self) -> $self_ty<$($phantom_ty,)? T>
            where
                T: Copy,
            {
                $self_ty {
                    hue: self.hue.copied(),
                    $($element: *self.$element,)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            /// Get an owned, cloned version of this color.
            #[inline]
            pub fn cloned(&self) -> $self_ty<$($phantom_ty,)? T>
            where
                T: Clone,
            {
                $self_ty {
                    hue: self.hue.cloned(),
                    $($element: self.$element.clone(),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        impl<$($phantom_ty,)? T, A> crate::Alpha<$self_ty<$($phantom_ty,)? &T>, &A> {
            /// Get an owned, copied version of this color.
            #[inline]
            pub fn copied(&self) -> crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>
            where
                T: Copy,
                A: Copy,
            {
                crate::Alpha{color: self.color.copied(), alpha: *self.alpha}
            }

            /// Get an owned, cloned version of this color.
            #[inline]
            pub fn cloned(&self) -> crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>
            where
                T: Clone,
                A: Clone,
            {
                crate::Alpha{color: self.color.cloned(), alpha: self.alpha.clone()}
            }
        }

        impl<$($phantom_ty,)? T, A> crate::Alpha<$self_ty<$($phantom_ty,)? &mut T>, &mut A> {
            /// Update this color with new values.
            #[inline]
            pub fn set(&mut self, value: crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>) {
                self.color.set(value.color);
                *self.alpha = value.alpha;
            }

            /// Borrow this color's components as shared references.
            #[inline]
            pub fn as_refs(&self) -> crate::Alpha<$self_ty<$($phantom_ty,)? &T>, &A>{
                crate::Alpha{color: self.color.as_refs(), alpha: &*self.alpha}
            }

            /// Get an owned, copied version of this color.
            #[inline]
            pub fn copied(&self) -> crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>
            where
                T: Copy,
                A: Copy,
            {
                crate::Alpha{color: self.color.copied(), alpha: *self.alpha}
            }

            /// Get an owned, cloned version of this color.
            #[inline]
            pub fn cloned(&self) -> crate::Alpha<$self_ty<$($phantom_ty,)? T>, A>
            where
                T: Clone,
                A: Clone,
            {
                crate::Alpha{color: self.color.cloned(), alpha: self.alpha.clone()}
            }
        }
    }
}
