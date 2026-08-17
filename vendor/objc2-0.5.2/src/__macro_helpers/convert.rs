use crate::encode::{EncodeArgument, EncodeArguments, EncodeReturn};
use crate::rc::Retained;
use crate::runtime::Bool;
use crate::Message;

mod argument_private {
    pub trait Sealed {}
}

/// Represents types that can be converted to/from an [`EncodeArgument`] type.
///
/// This is implemented specially for [`bool`] to allow using that as
/// Objective-C `BOOL`, where it would otherwise not be allowed (since they
/// are not ABI compatible).
///
/// This is also done specially for `&mut Retained<_>`-like arguments, to allow
/// using those as "out" parameters.
pub trait ConvertArgument: argument_private::Sealed {
    /// The inner type that this can be converted to and from.
    #[doc(hidden)]
    type __Inner: EncodeArgument;

    /// A helper type for out parameters.
    #[doc(hidden)]
    type __StoredBeforeMessage: Sized;

    #[doc(hidden)]
    fn __from_declared_param(inner: Self::__Inner) -> Self;

    #[doc(hidden)]
    fn __into_argument(self) -> (Self::__Inner, Self::__StoredBeforeMessage);

    #[doc(hidden)]
    #[inline]
    unsafe fn __process_after_message_send(_stored: Self::__StoredBeforeMessage) {}
}

// Implemented in writeback.rs
impl<T: Message> argument_private::Sealed for &mut Retained<T> {}
impl<T: Message> argument_private::Sealed for Option<&mut Retained<T>> {}
impl<T: Message> argument_private::Sealed for &mut Option<Retained<T>> {}
impl<T: Message> argument_private::Sealed for Option<&mut Option<Retained<T>>> {}

impl<T: EncodeArgument> argument_private::Sealed for T {}
impl<T: EncodeArgument> ConvertArgument for T {
    type __Inner = Self;

    type __StoredBeforeMessage = ();

    #[inline]
    fn __from_declared_param(inner: Self::__Inner) -> Self {
        inner
    }

    #[inline]
    fn __into_argument(self) -> (Self::__Inner, Self::__StoredBeforeMessage) {
        (self, ())
    }
}

impl argument_private::Sealed for bool {}
impl ConvertArgument for bool {
    type __Inner = Bool;

    type __StoredBeforeMessage = ();

    #[inline]
    fn __from_declared_param(inner: Self::__Inner) -> Self {
        inner.as_bool()
    }

    #[inline]
    fn __into_argument(self) -> (Self::__Inner, Self::__StoredBeforeMessage) {
        (Bool::new(self), ())
    }
}

mod return_private {
    pub trait Sealed {}
}

/// Same as [`ConvertArgument`], but for return types.
pub trait ConvertReturn: return_private::Sealed {
    /// The inner type that this can be converted to and from.
    #[doc(hidden)]
    type __Inner: EncodeReturn;

    #[doc(hidden)]
    fn __into_declared_return(self) -> Self::__Inner;

    #[doc(hidden)]
    fn __from_return(inner: Self::__Inner) -> Self;
}

impl<T: EncodeReturn> return_private::Sealed for T {}
impl<T: EncodeReturn> ConvertReturn for T {
    type __Inner = Self;

    #[inline]
    fn __into_declared_return(self) -> Self::__Inner {
        self
    }

    #[inline]
    fn __from_return(inner: Self::__Inner) -> Self {
        inner
    }
}

impl return_private::Sealed for bool {}
impl ConvertReturn for bool {
    type __Inner = Bool;

    #[inline]
    fn __into_declared_return(self) -> Self::__Inner {
        Bool::new(self)
    }

    #[inline]
    fn __from_return(inner: Self::__Inner) -> Self {
        inner.as_bool()
    }
}

pub trait ConvertArguments {
    #[doc(hidden)]
    type __Inner: EncodeArguments;

    #[doc(hidden)]
    type __StoredBeforeMessage: Sized;

    #[doc(hidden)]
    fn __into_arguments(self) -> (Self::__Inner, Self::__StoredBeforeMessage);

    #[doc(hidden)]
    unsafe fn __process_after_message_send(_stored: Self::__StoredBeforeMessage);
}

pub trait TupleExtender<T> {
    #[doc(hidden)]
    type PlusOneArgument;
    #[doc(hidden)]
    fn add_argument(self, arg: T) -> Self::PlusOneArgument;
}

macro_rules! args_impl {
    ($($a:ident: $t:ident),*) => (
        impl<$($t: ConvertArgument),*> ConvertArguments for ($($t,)*) {
            type __Inner = ($($t::__Inner,)*);

            type __StoredBeforeMessage = ($($t::__StoredBeforeMessage,)*);

            #[inline]
            fn __into_arguments(self) -> (Self::__Inner, Self::__StoredBeforeMessage) {
                let ($($a,)*) = self;
                $(let $a = ConvertArgument::__into_argument($a);)*

                (($($a.0,)*), ($($a.1,)*))
            }

            #[inline]
            unsafe fn __process_after_message_send(($($a,)*): Self::__StoredBeforeMessage) {
                $(
                    unsafe { <$t as ConvertArgument>::__process_after_message_send($a) };
                )*
            }
        }

        impl<$($t,)* T> TupleExtender<T> for ($($t,)*) {
            type PlusOneArgument = ($($t,)* T,);

            #[inline]
            fn add_argument(self, arg: T) -> Self::PlusOneArgument {
                let ($($a,)*) = self;
                ($($a,)* arg,)
            }
        }
    );
}

args_impl!();
args_impl!(a: A);
args_impl!(a: A, b: B);
args_impl!(a: A, b: B, c: C);
args_impl!(a: A, b: B, c: C, d: D);
args_impl!(a: A, b: B, c: C, d: D, e: E);
args_impl!(a: A, b: B, c: C, d: D, e: E, f: F);
args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K
);
args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L
);
args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M
);
args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M,
    n: N
);
args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M,
    n: N,
    o: O
);
args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M,
    n: N,
    o: O,
    p: P
);

#[cfg(test)]
mod tests {
    use super::*;

    use core::any::TypeId;

    #[test]
    fn convert_normally_noop() {
        assert_eq!(
            TypeId::of::<<i32 as ConvertArgument>::__Inner>(),
            TypeId::of::<i32>()
        );
        assert_eq!(<i32 as ConvertArgument>::__from_declared_param(42), 42);
        assert_eq!(ConvertArgument::__into_argument(42i32).0, 42);
    }

    #[test]
    fn convert_i8() {
        assert_eq!(
            TypeId::of::<<i8 as ConvertArgument>::__Inner>(),
            TypeId::of::<i8>()
        );
        assert_eq!(<i8 as ConvertArgument>::__from_declared_param(-3), -3);
        assert_eq!(ConvertArgument::__into_argument(-3i32).0, -3);
    }

    #[test]
    fn convert_bool() {
        assert!(!<bool as ConvertArgument>::__from_declared_param(Bool::NO));
        assert!(<bool as ConvertArgument>::__from_declared_param(Bool::YES));
        assert!(!<bool as ConvertReturn>::__from_return(Bool::NO));
        assert!(<bool as ConvertReturn>::__from_return(Bool::YES));

        assert!(!ConvertArgument::__into_argument(false).0.as_bool());
        assert!(ConvertArgument::__into_argument(true).0.as_bool());
        assert!(!ConvertReturn::__into_declared_return(false).as_bool());
        assert!(ConvertReturn::__into_declared_return(true).as_bool());

        #[cfg(all(target_vendor = "apple", target_os = "macos", target_arch = "x86_64"))]
        assert_eq!(
            <bool as ConvertArgument>::__Inner::ENCODING_ARGUMENT,
            crate::encode::Encoding::Char,
        );
    }
}
