use std::ptr::{self, NonNull};
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::Relaxed;

/// [`Collectible`] defines the memory layout for the type in order to be passed to the garbage
/// collector.
pub(super) trait Collectible {
    /// Returns the next [`Collectible`] pointer.
    fn next_ptr(&self) -> Option<NonNull<dyn Collectible>>;

    /// Sets the next [`Collectible`] pointer.
    fn set_next_ptr(&self, next_ptr: Option<NonNull<dyn Collectible>>);
}

/// [`Link`] implements [`Collectible`].
#[derive(Debug, Default)]
pub struct Link {
    data: (AtomicPtr<()>, AtomicPtr<()>),
}

/// [`DeferredClosure`] implements [`Collectible`] for a closure to execute it after all the
/// current readers in the process are gone.
pub(super) struct DeferredClosure<F: 'static + FnOnce()> {
    f: Option<F>,
    link: Link,
}

impl Link {
    #[inline]
    pub(super) const fn new_shared() -> Self {
        Link {
            data: (
                AtomicPtr::new(ptr::without_provenance_mut(1)),
                AtomicPtr::new(ptr::null_mut()),
            ),
        }
    }

    #[inline]
    pub(super) const fn new_unique() -> Self {
        Link {
            data: (
                AtomicPtr::new(ptr::without_provenance_mut(0)),
                AtomicPtr::new(ptr::null_mut()),
            ),
        }
    }

    #[inline]
    pub(super) const fn ref_cnt(&self) -> &AtomicPtr<()> {
        &self.data.0
    }
}

impl Collectible for Link {
    #[inline]
    fn next_ptr(&self) -> Option<NonNull<dyn Collectible>> {
        let fat_ptr: (*mut (), *mut ()) = (self.data.0.load(Relaxed), self.data.1.load(Relaxed));
        unsafe { std::mem::transmute(fat_ptr) }
    }

    #[inline]
    fn set_next_ptr(&self, next_ptr: Option<NonNull<dyn Collectible>>) {
        let data: (*mut (), *mut ()) = next_ptr.map_or_else(
            || (ptr::null_mut(), ptr::null_mut()),
            |p| unsafe { std::mem::transmute(p) },
        );
        self.data.0.store(data.0, Relaxed);
        self.data.1.store(data.1, Relaxed);
    }
}

impl<F: 'static + FnOnce()> DeferredClosure<F> {
    /// Creates a new [`DeferredClosure`].
    #[inline]
    pub fn new(f: F) -> Self {
        DeferredClosure {
            f: Some(f),
            link: Link::default(),
        }
    }
}

impl<F: 'static + FnOnce()> Collectible for DeferredClosure<F> {
    #[inline]
    fn next_ptr(&self) -> Option<NonNull<dyn Collectible>> {
        self.link.next_ptr()
    }

    #[inline]
    fn set_next_ptr(&self, next_ptr: Option<NonNull<dyn Collectible>>) {
        self.link.set_next_ptr(next_ptr);
    }
}

impl<F: 'static + FnOnce()> Drop for DeferredClosure<F> {
    #[inline]
    fn drop(&mut self) {
        if let Some(f) = self.f.take() {
            f();
        }
    }
}
