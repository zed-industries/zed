use std::{
    borrow::{Borrow, BorrowMut},
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crossbeam_epoch::Owned;

#[derive(Clone, Debug)]
pub(crate) struct NoisyDropper<T: ?Sized> {
    parent: Arc<DropNotifier>,
    pub elem: T,
}

impl<T> NoisyDropper<T> {
    pub(crate) fn new(parent: Arc<DropNotifier>, elem: T) -> Self {
        Self { parent, elem }
    }
}

impl<T: ?Sized> Drop for NoisyDropper<T> {
    fn drop(&mut self) {
        assert!(!self.parent.dropped.swap(true, Ordering::Relaxed));
    }
}

impl<T: ?Sized + PartialEq> PartialEq for NoisyDropper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.elem == other.elem
    }
}

impl<T: ?Sized + PartialEq> PartialEq<T> for NoisyDropper<T> {
    fn eq(&self, other: &T) -> bool {
        &self.elem == other
    }
}

impl<T: ?Sized + Eq> Eq for NoisyDropper<T> {}

impl<T: ?Sized + Hash> Hash for NoisyDropper<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.elem.hash(hasher);
    }
}

impl<T: ?Sized> AsRef<T> for NoisyDropper<T> {
    fn as_ref(&self) -> &T {
        &self.elem
    }
}

impl<T: ?Sized> AsMut<T> for NoisyDropper<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.elem
    }
}

impl<T: ?Sized> Borrow<T> for NoisyDropper<T> {
    fn borrow(&self) -> &T {
        &self.elem
    }
}

impl<T: ?Sized> BorrowMut<T> for NoisyDropper<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.elem
    }
}

impl<T: ?Sized> Deref for NoisyDropper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.elem
    }
}

impl<T: ?Sized> DerefMut for NoisyDropper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elem
    }
}

#[derive(Debug)]
pub(crate) struct DropNotifier {
    dropped: AtomicBool,
}

impl DropNotifier {
    pub(crate) fn new() -> Self {
        Self {
            dropped: AtomicBool::new(false),
        }
    }

    pub(crate) fn was_dropped(&self) -> bool {
        self.dropped.load(Ordering::Relaxed)
    }
}

pub(crate) fn run_deferred() {
    for _ in 0..65536 {
        let guard = crossbeam_epoch::pin();

        unsafe { guard.defer_destroy(Owned::new(0).into_shared(&guard)) };

        guard.flush();
    }
}
