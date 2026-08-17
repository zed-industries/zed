macro_rules! impl_copy_clone {
    (  $self_ty: ident , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl_copy_clone!($self_ty<>, [$($element),+] $(, $phantom)?);
    };
    (  $self_ty: ident < $($phantom_ty: ident)? > , [$($element: ident),+] $(, $phantom: ident)?) => {
        impl<$($phantom_ty,)? T> Copy for $self_ty<$($phantom_ty,)? T> where T: Copy {}

        impl<$($phantom_ty,)? T> Clone for $self_ty<$($phantom_ty,)? T>
        where
            T: Clone,
        {
            fn clone(&self) -> $self_ty<$($phantom_ty,)? T> {
                $self_ty {
                    $($element: self.$element.clone(),)*
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }
    }
}
