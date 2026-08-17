mod statement_cache;

pub use statement_cache::StatementCache;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

/// A wrapper for `Fn`s that provides a debug impl that just says "Function"
pub struct DebugFn<F: ?Sized>(pub F);

impl<F: ?Sized> Deref for DebugFn<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: ?Sized> DerefMut for DebugFn<F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<F: ?Sized> Debug for DebugFn<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Function").finish()
    }
}
