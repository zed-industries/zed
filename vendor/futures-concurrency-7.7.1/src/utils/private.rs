/// We hide the `Try` trait in a private module, as it's only meant to be a
/// stable clone of the standard library's `Try` trait, as yet unstable.
// NOTE: copied from `rayon`
use core::convert::Infallible;
use core::ops::ControlFlow::{self, Break, Continue};
use core::task::Poll;

use crate::{private_decl, private_impl};

/// Clone of `std::ops::Try`.
///
/// Implementing this trait is not permitted outside of `futures_concurrency`.
pub trait Try {
    private_decl! {}

    type Output;
    type Residual;

    fn from_output(output: Self::Output) -> Self;

    fn from_residual(residual: Self::Residual) -> Self;

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
}

impl<B, C> Try for ControlFlow<B, C> {
    private_impl! {}

    type Output = C;
    type Residual = ControlFlow<B, Infallible>;

    fn from_output(output: Self::Output) -> Self {
        Continue(output)
    }

    fn from_residual(residual: Self::Residual) -> Self {
        match residual {
            Break(b) => Break(b),
            Continue(_) => unreachable!(),
        }
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Continue(c) => Continue(c),
            Break(b) => Break(Break(b)),
        }
    }
}

impl<T> Try for Option<T> {
    private_impl! {}

    type Output = T;
    type Residual = Option<Infallible>;

    fn from_output(output: Self::Output) -> Self {
        Some(output)
    }

    fn from_residual(residual: Self::Residual) -> Self {
        match residual {
            None => None,
            Some(_) => unreachable!(),
        }
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Some(c) => Continue(c),
            None => Break(None),
        }
    }
}

impl<T, E> Try for Result<T, E> {
    private_impl! {}

    type Output = T;
    type Residual = Result<Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        Ok(output)
    }

    fn from_residual(residual: Self::Residual) -> Self {
        match residual {
            Err(e) => Err(e),
            Ok(_) => unreachable!(),
        }
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Ok(c) => Continue(c),
            Err(e) => Break(Err(e)),
        }
    }
}

impl<T, E> Try for Poll<Result<T, E>> {
    private_impl! {}

    type Output = Poll<T>;
    type Residual = Result<Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        output.map(Ok)
    }

    fn from_residual(residual: Self::Residual) -> Self {
        match residual {
            Err(e) => Poll::Ready(Err(e)),
            Ok(_) => unreachable!(),
        }
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Poll::Pending => Continue(Poll::Pending),
            Poll::Ready(Ok(c)) => Continue(Poll::Ready(c)),
            Poll::Ready(Err(e)) => Break(Err(e)),
        }
    }
}

impl<T, E> Try for Poll<Option<Result<T, E>>> {
    private_impl! {}

    type Output = Poll<Option<T>>;
    type Residual = Result<Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        match output {
            Poll::Ready(o) => Poll::Ready(o.map(Ok)),
            Poll::Pending => Poll::Pending,
        }
    }

    fn from_residual(residual: Self::Residual) -> Self {
        match residual {
            Err(e) => Poll::Ready(Some(Err(e))),
            Ok(_) => unreachable!(),
        }
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Poll::Pending => Continue(Poll::Pending),
            Poll::Ready(None) => Continue(Poll::Ready(None)),
            Poll::Ready(Some(Ok(c))) => Continue(Poll::Ready(Some(c))),
            Poll::Ready(Some(Err(e))) => Break(Err(e)),
        }
    }
}

#[allow(missing_debug_implementations)]
pub struct PrivateMarker;

#[doc(hidden)]
#[macro_export]
macro_rules! private_impl {
    () => {
        fn __futures_concurrency_private__(&self) -> $crate::private::PrivateMarker {
            $crate::private::PrivateMarker
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! private_decl {
    () => {
        /// This trait is private; this method exists to make it
        /// impossible to implement outside the crate.
        #[doc(hidden)]
        fn __futures_concurrency_private__(&self) -> $crate::private::PrivateMarker;
    };
}
