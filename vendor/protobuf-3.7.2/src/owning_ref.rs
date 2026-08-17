//! Utility similar to provided by `owning_ref` crate.

use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;

enum Owner<A: 'static> {
    Arc(Arc<A>),
    Static(&'static A),
}

impl<A: 'static> Deref for Owner<A> {
    type Target = A;

    fn deref(&self) -> &A {
        match self {
            Owner::Arc(a) => &*a,
            Owner::Static(a) => a,
        }
    }
}

pub(crate) struct OwningRef<A: 'static, B: 'static> {
    owner: Owner<A>,
    ptr: *const B,
}

unsafe impl<A: Send + Sync + 'static, B: Send + Sync + 'static> Sync for OwningRef<A, B> {}
unsafe impl<A: Send + Sync + 'static, B: Send + Sync + 'static> Send for OwningRef<A, B> {}

impl<A: 'static, B: 'static> Deref for OwningRef<A, B> {
    type Target = B;

    fn deref(&self) -> &B {
        // SAFETY: `self.owner` owns the data and it is not movable.
        unsafe { &*self.ptr }
    }
}

impl<A: 'static> Clone for Owner<A> {
    fn clone(&self) -> Owner<A> {
        match self {
            Owner::Arc(arc) => Owner::Arc(arc.clone()),
            Owner::Static(ptr) => Owner::Static(ptr),
        }
    }
}

impl<A: 'static, B: 'static> Clone for OwningRef<A, B> {
    fn clone(&self) -> OwningRef<A, B> {
        OwningRef {
            ptr: self.ptr,
            owner: self.owner.clone(),
        }
    }
}

impl<A: 'static, B: fmt::Debug + 'static> Debug for OwningRef<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&**self, f)
    }
}

impl<A: 'static> OwningRef<A, A> {
    pub(crate) fn new_arc(arc: Arc<A>) -> OwningRef<A, A> {
        OwningRef {
            ptr: Arc::as_ptr(&arc),
            owner: Owner::Arc(arc),
        }
    }

    pub(crate) fn new_static(ptr: &'static A) -> OwningRef<A, A> {
        OwningRef {
            ptr,
            owner: Owner::Static(ptr),
        }
    }

    pub(crate) fn owner(&self) -> &A {
        &self.owner
    }
}

impl<A: 'static, B: 'static> OwningRef<A, B> {
    pub(crate) fn _map<C>(self, f: impl FnOnce(&B) -> &C) -> OwningRef<A, C> {
        let ptr = f(&*self);
        OwningRef {
            ptr,
            owner: self.owner,
        }
    }

    pub(crate) fn flat_map_slice<'x, C, T: FnOnce(&B) -> &[C]>(
        &self,
        f: T,
    ) -> impl Iterator<Item = OwningRef<A, C>> + '_
    where
        C: 'static,
    {
        f(&self).into_iter().map(|ptr| OwningRef {
            ptr,
            owner: self.owner.clone(),
        })
    }
}
