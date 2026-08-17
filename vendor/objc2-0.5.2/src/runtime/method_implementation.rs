use core::mem;

use crate::__macro_helpers::IdReturnValue;
use crate::encode::{EncodeArgument, EncodeArguments, EncodeReturn, RefEncode};
use crate::rc::Allocated;
use crate::runtime::{Imp, MessageReceiver, Sel};
use crate::Message;

mod private {
    pub trait Sealed {}
}

/// Types that can be used as the implementation of an Objective-C method.
///
/// This is a sealed trait that is implemented for a lot of `extern "C"`
/// function pointer types.
//
// Note: `Sized` is intentionally added to make the trait not object safe.
pub trait MethodImplementation: private::Sealed + Sized {
    /// The callee type of the method.
    type Callee: ?Sized + RefEncode;

    /// The argument types of the method.
    type Arguments: EncodeArguments;

    /// The return type of the method.
    type Return: EncodeReturn;

    #[doc(hidden)]
    fn __imp(self) -> Imp;
}

macro_rules! method_impl_inner {
    ($(($unsafe:ident))? $abi:literal; $($t:ident),*) => {
        impl<T, R, $($t),*> private::Sealed for $($unsafe)? extern $abi fn(T, Sel $(, $t)*) -> R
        where
            T: ?Sized + MessageReceiver,
            R: EncodeReturn,
            $($t: EncodeArgument,)*
        {}

        impl<T, R, $($t),*> MethodImplementation for $($unsafe)? extern $abi fn(T, Sel $(, $t)*) -> R
        where
            T: ?Sized + MessageReceiver,
            R: EncodeReturn,
            $($t: EncodeArgument,)*
        {
            type Callee = T::__Inner;
            type Arguments = ($($t,)*);
            type Return = R;

            fn __imp(self) -> Imp {
                // SAFETY: Transmuting to an `unsafe` function pointer
                unsafe { mem::transmute(self) }
            }
        }

        impl<T, $($t),*> private::Sealed for $($unsafe)? extern $abi fn(Allocated<T>, Sel $(, $t)*) -> IdReturnValue
        where
            T: ?Sized + Message,
            $($t: EncodeArgument,)*
        {}

        #[doc(hidden)]
        impl<T, $($t),*> MethodImplementation for $($unsafe)? extern $abi fn(Allocated<T>, Sel $(, $t)*) -> IdReturnValue
        where
            T: ?Sized + Message,
            $($t: EncodeArgument,)*
        {
            type Callee = T;
            type Arguments = ($($t,)*);
            type Return = IdReturnValue;

            fn __imp(self) -> Imp {
                // SAFETY: `Allocated<T>` is the same as `NonNull<T>`, except
                // with the assumption of a +1 calling convention.
                //
                // The calling convention is ensured to be upheld by having
                // `IdReturnValue` in the type, since that type is private
                // and hence only internal macros like `#[method_id]` will be
                // able to produce it (and that, in turn, only allows it if
                // the selector is `init` as checked by `MessageRecieveId`).
                unsafe { mem::transmute(self) }
            }
        }
    };
}

macro_rules! method_impl {
    ($($t:ident),*) => {
        method_impl_inner!((unsafe) "C"; $($t),*);
        method_impl_inner!("C"; $($t),*);
        #[cfg(feature = "unstable-c-unwind")]
        method_impl_inner!((unsafe) "C-unwind"; $($t),*);
        #[cfg(feature = "unstable-c-unwind")]
        method_impl_inner!("C-unwind"; $($t),*);
    };
}

method_impl!();
method_impl!(A);
method_impl!(A, B);
method_impl!(A, B, C);
method_impl!(A, B, C, D);
method_impl!(A, B, C, D, E);
method_impl!(A, B, C, D, E, F);
method_impl!(A, B, C, D, E, F, G);
method_impl!(A, B, C, D, E, F, G, H);
method_impl!(A, B, C, D, E, F, G, H, I);
method_impl!(A, B, C, D, E, F, G, H, I, J);
method_impl!(A, B, C, D, E, F, G, H, I, J, K);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
