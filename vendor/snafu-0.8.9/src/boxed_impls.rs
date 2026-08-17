use alloc::{boxed::Box, rc::Rc};

// `target_has_atomic` wasn't stabilized until 1.60. Thankfully, we
// don't support no_std + alloc without Rust 1.81. After the MSRV is
// bumped to >= 1.60, this can be simplified to `target_has_atomic`.
#[cfg(any(feature = "std", all(feature = "rust_1_61", target_has_atomic = "ptr")))]
use alloc::sync::Arc;

use super::{AsBacktrace, Backtrace, ErrorCompat, GenerateImplicitData};

impl<E> ErrorCompat for Box<E>
where
    E: ErrorCompat,
{
    fn backtrace(&self) -> Option<&Backtrace> {
        (**self).backtrace()
    }
}

impl<T> GenerateImplicitData for Box<T>
where
    T: GenerateImplicitData,
{
    #[track_caller]
    fn generate() -> Self {
        Box::new(T::generate())
    }

    #[track_caller]
    fn generate_with_source(source: &dyn crate::Error) -> Self
    where
        Self: Sized,
    {
        Box::new(T::generate_with_source(source))
    }
}

impl<T> GenerateImplicitData for Rc<T>
where
    T: GenerateImplicitData,
{
    #[track_caller]
    fn generate() -> Self {
        Rc::new(T::generate())
    }

    #[track_caller]
    fn generate_with_source(source: &dyn crate::Error) -> Self
    where
        Self: Sized,
    {
        Rc::new(T::generate_with_source(source))
    }
}

#[cfg(any(feature = "std", all(feature = "rust_1_61", target_has_atomic = "ptr")))]
impl<T> GenerateImplicitData for Arc<T>
where
    T: GenerateImplicitData,
{
    #[track_caller]
    fn generate() -> Self {
        Arc::new(T::generate())
    }

    #[track_caller]
    fn generate_with_source(source: &dyn crate::Error) -> Self
    where
        Self: Sized,
    {
        Arc::new(T::generate_with_source(source))
    }
}

impl<T> AsBacktrace for Box<T>
where
    T: AsBacktrace,
{
    fn as_backtrace(&self) -> Option<&Backtrace> {
        T::as_backtrace(self)
    }
}

impl<T> AsBacktrace for Rc<T>
where
    T: AsBacktrace,
{
    fn as_backtrace(&self) -> Option<&Backtrace> {
        T::as_backtrace(self)
    }
}

#[cfg(any(feature = "std", all(feature = "rust_1_61", target_has_atomic = "ptr")))]
impl<T> AsBacktrace for Arc<T>
where
    T: AsBacktrace,
{
    fn as_backtrace(&self) -> Option<&Backtrace> {
        T::as_backtrace(self)
    }
}
