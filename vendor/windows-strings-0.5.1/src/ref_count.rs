use core::sync::atomic::{fence, AtomicI32, Ordering};

#[repr(transparent)]
#[derive(Default)]
pub struct RefCount(pub(crate) AtomicI32);

impl RefCount {
    pub fn new(count: u32) -> Self {
        Self(AtomicI32::new(count as i32))
    }

    pub fn add_ref(&self) -> u32 {
        (self.0.fetch_add(1, Ordering::Relaxed) + 1) as u32
    }

    pub fn release(&self) -> u32 {
        let remaining = self.0.fetch_sub(1, Ordering::Release) - 1;

        match remaining.cmp(&0) {
            core::cmp::Ordering::Equal => fence(Ordering::Acquire),
            core::cmp::Ordering::Less => panic!("Object has been over-released."),
            core::cmp::Ordering::Greater => {}
        }

        remaining as u32
    }
}
