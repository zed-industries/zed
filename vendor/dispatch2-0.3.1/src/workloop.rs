use alloc::ffi::CString;
use core::{borrow::Borrow, ops::Deref};

use crate::{DispatchQueue, DispatchRetained};

dispatch_object!(
    /// Dispatch workloop queue.
    #[doc(alias = "dispatch_workloop_t")]
    #[doc(alias = "dispatch_workloop_s")]
    pub struct DispatchWorkloop;
);

dispatch_object_not_data!(unsafe DispatchWorkloop);

impl DispatchWorkloop {
    /// Create a new [`DispatchWorkloop`].
    #[inline]
    pub fn new(label: &str, inactive: bool) -> DispatchRetained<Self> {
        let label = CString::new(label).expect("Invalid label!");

        // Safety: label can only be valid.
        unsafe {
            if inactive {
                DispatchWorkloop::__new_inactive(label.as_ptr())
            } else {
                DispatchWorkloop::__new(label.as_ptr())
            }
        }
    }
}

impl Deref for DispatchWorkloop {
    type Target = DispatchQueue;

    /// Access the workloop as a [`DispatchQueue`].
    #[inline]
    fn deref(&self) -> &Self::Target {
        let ptr: *const DispatchWorkloop = self;
        let ptr: *const DispatchQueue = ptr.cast();
        // SAFETY: Workloop queues are "subclasses" of queues (they can be
        // used in all the same places that normal queues can).
        unsafe { &*ptr }
    }
}

impl AsRef<DispatchQueue> for DispatchWorkloop {
    #[inline]
    fn as_ref(&self) -> &DispatchQueue {
        self
    }
}

// PartialEq, Eq and Hash work the same for workloops and queues.
impl Borrow<DispatchQueue> for DispatchWorkloop {
    #[inline]
    fn borrow(&self) -> &DispatchQueue {
        self
    }
}
