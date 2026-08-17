use std::sync::atomic::{fence, AtomicI32, Ordering};

#[doc(hidden)]
#[repr(transparent)]
#[derive(Default)]
pub struct RefCount(pub(crate) AtomicI32);

impl RefCount {
    /// Creates a new `RefCount` with an initial value of `1`.
    pub fn new(count: u32) -> Self {
        Self(AtomicI32::new(count as i32))
    }

    /// Increments the reference count, returning the new value.
    pub fn add_ref(&self) -> u32 {
        (self.0.fetch_add(1, Ordering::Relaxed) + 1) as u32
    }

    /// Decrements the reference count, returning the new value.
    ///
    /// This operation inserts an `Acquire` fence when the reference count reaches zero.
    /// This prevents reordering before the object is destroyed.
    pub fn release(&self) -> u32 {
        let remaining = self.0.fetch_sub(1, Ordering::Release) - 1;

        match remaining.cmp(&0) {
            std::cmp::Ordering::Equal => fence(Ordering::Acquire),
            std::cmp::Ordering::Less => panic!("Object has been over-released."),
            std::cmp::Ordering::Greater => {}
        }

        remaining as u32
    }
}
