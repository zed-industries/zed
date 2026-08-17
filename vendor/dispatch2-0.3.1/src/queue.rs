use alloc::boxed::Box;
use alloc::ffi::CString;
use core::ffi::c_long;
use core::ptr::NonNull;

use super::utils::function_wrapper;
use crate::generated::{
    _dispatch_main_q, _dispatch_queue_attr_concurrent, dispatch_get_global_queue,
    dispatch_queue_set_specific,
};
use crate::{
    DispatchObject, DispatchQoS, DispatchRetained, DispatchTime, QualityOfServiceClassFloorError,
};

/// Error returned by [`DispatchQueue::after`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QueueAfterError {
    /// The given timeout value will result in an overflow when converting to dispatch time.
    TimeOverflow,
}

enum_with_val! {
    /// Queue priority.
    #[doc(alias = "dispatch_queue_priority_t")]
    #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct DispatchQueueGlobalPriority(pub c_long) {
        /// High priority.
        #[doc(alias = "DISPATCH_QUEUE_PRIORITY_HIGH")]
        High = 0x2,
        /// Default priority.
        #[doc(alias = "DISPATCH_QUEUE_PRIORITY_DEFAULT")]
        Default = 0x0,
        /// Low priority.
        #[doc(alias = "DISPATCH_QUEUE_PRIORITY_LOW")]
        Low = -0x2,
        /// Background priority.
        #[doc(alias = "DISPATCH_QUEUE_PRIORITY_BACKGROUND")]
        Background = u16::MIN as c_long,
    }
}

/// Global queue identifier definition for [`DispatchQueue::new`] and [`DispatchQueue::new_with_target`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GlobalQueueIdentifier {
    /// Standard priority based queue.
    Priority(DispatchQueueGlobalPriority),
    /// Quality of service priority based queue.
    QualityOfService(DispatchQoS),
}

impl GlobalQueueIdentifier {
    /// Convert and consume [GlobalQueueIdentifier] into its raw value.
    #[inline]
    pub fn to_identifier(self) -> isize {
        match self {
            GlobalQueueIdentifier::Priority(queue_priority) => queue_priority.0 as isize,
            GlobalQueueIdentifier::QualityOfService(qos_class) => qos_class.0 as isize,
        }
    }
}

dispatch_object!(
    /// Dispatch queue.
    #[doc(alias = "dispatch_queue_t")]
    #[doc(alias = "dispatch_queue_s")]
    pub struct DispatchQueue;
);

dispatch_object_not_data!(unsafe DispatchQueue);

impl DispatchQueue {
    /// Create a new [`DispatchQueue`].
    #[inline]
    pub fn new(label: &str, queue_attribute: Option<&DispatchQueueAttr>) -> DispatchRetained<Self> {
        let label = CString::new(label).expect("Invalid label!");

        // SAFETY: The label is a valid C string.
        unsafe { Self::__new(label.as_ptr(), queue_attribute) }
    }

    /// Create a new [`DispatchQueue`] with a given target [`DispatchQueue`].
    #[inline]
    pub fn new_with_target(
        label: &str,
        queue_attribute: Option<&DispatchQueueAttr>,
        target: Option<&DispatchQueue>,
    ) -> DispatchRetained<Self> {
        let label = CString::new(label).expect("Invalid label!");

        // SAFETY: The label is a valid C string.
        unsafe { Self::__new_with_target(label.as_ptr(), queue_attribute, target) }
    }

    /// Return a system-defined global concurrent [`DispatchQueue`] with the priority derived from [GlobalQueueIdentifier].
    #[inline]
    pub fn global_queue(identifier: GlobalQueueIdentifier) -> DispatchRetained<Self> {
        let raw_identifier = identifier.to_identifier();

        // Safety: raw_identifier cannot be invalid, flags is reserved.
        dispatch_get_global_queue(raw_identifier, 0)
    }

    /// Return the main queue.
    // TODO: Mark this as `const` once in MSRV.
    #[inline]
    #[doc(alias = "dispatch_get_main_queue")]
    pub fn main() -> &'static Self {
        // Inline function in the header

        // SAFETY: The main queue is safe to access from anywhere, and is
        // valid forever.
        unsafe { &_dispatch_main_q }
    }

    /// Submit a function for synchronous execution on the [`DispatchQueue`].
    #[inline]
    pub fn exec_sync<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // NOTE: `dispatch_sync*` functions are discouraged on workloops for
        // performance reasons, but they should still work, so we won't forbid
        // it here.
        //
        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { Self::exec_sync_f(self, work_boxed, function_wrapper::<F>) }
    }

    /// Submit a function for asynchronous execution on the [`DispatchQueue`].
    #[inline]
    pub fn exec_async<F>(&self, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { Self::exec_async_f(self, work_boxed, function_wrapper::<F>) }
    }

    /// Enqueue a function for execution at the specified time on the [`DispatchQueue`].
    #[inline]
    pub fn after<F>(&self, when: DispatchTime, work: F) -> Result<(), QueueAfterError>
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { Self::exec_after_f(when, self, work_boxed, function_wrapper::<F>) };

        Ok(())
    }

    /// Enqueue a barrier function for asynchronous execution on the [`DispatchQueue`] and return immediately.
    #[inline]
    pub fn barrier_async<F>(&self, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { Self::barrier_async_f(self, work_boxed, function_wrapper::<F>) }
    }

    /// Enqueue a barrier function for synchronous execution on the [`DispatchQueue`] and wait until that function completes.
    #[inline]
    pub fn barrier_sync<F>(&self, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { Self::barrier_sync_f(self, work_boxed, function_wrapper::<F>) }
    }

    /// Submit a function for synchronous execution and mark the function as a barrier for subsequent concurrent tasks.
    #[inline]
    pub fn barrier_async_and_wait<F>(&self, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast();

        // Safety: object cannot be null and work is wrapped to avoid ABI incompatibility.
        unsafe { Self::barrier_async_and_wait_f(self, work_boxed, function_wrapper::<F>) }
    }

    /// Sets a function at the given key that will be executed at [`DispatchQueue`] destruction.
    #[inline]
    pub fn set_specific<F>(&self, key: NonNull<()>, destructor: F)
    where
        F: Send + FnOnce(),
    {
        let destructor_boxed = Box::into_raw(Box::new(destructor)).cast();

        // SAFETY: object cannot be null and destructor is wrapped to avoid
        // ABI incompatibility.
        //
        // The key is never dereferenced, so passing _any_ pointer here is
        // safe and allowed.
        unsafe {
            dispatch_queue_set_specific(self, key.cast(), destructor_boxed, function_wrapper::<F>)
        }
    }

    /// Set the QOS class floor of the [`DispatchQueue`].
    #[inline]
    pub fn set_qos_class_floor(
        &self,
        qos_class: DispatchQoS,
        relative_priority: i32,
    ) -> Result<(), QualityOfServiceClassFloorError> {
        // SAFETY: We are a queue.
        unsafe { DispatchObject::set_qos_class_floor(self, qos_class, relative_priority) }
    }

    #[allow(missing_docs)]
    #[doc(alias = "DISPATCH_APPLY_AUTO")]
    pub const APPLY_AUTO: Option<&DispatchQueue> = None;

    #[allow(missing_docs)]
    #[doc(alias = "DISPATCH_TARGET_QUEUE_DEFAULT")]
    pub const TARGET_QUEUE_DEFAULT: Option<&DispatchQueue> = None;

    #[allow(missing_docs)]
    #[doc(alias = "DISPATCH_CURRENT_QUEUE_LABEL")]
    pub const CURRENT_QUEUE_LABEL: Option<&DispatchQueue> = None;
}

dispatch_object!(
    /// Dispatch queue attribute.
    #[doc(alias = "dispatch_queue_attr_t")]
    #[doc(alias = "dispatch_queue_attr_s")]
    pub struct DispatchQueueAttr;
);

dispatch_object_not_data!(unsafe DispatchQueueAttr);

impl DispatchQueueAttr {
    /// A dispatch queue that executes blocks serially in FIFO order.
    #[doc(alias = "DISPATCH_QUEUE_SERIAL")]
    pub const SERIAL: Option<&Self> = None;

    // TODO(msrv): Expose this once
    // #[doc(alias = "DISPATCH_QUEUE_CONCURRENT")]
    // pub static CONCURRENT: Option<&Self> = {
    //     // Safety: immutable external definition
    //     unsafe { Some(&_dispatch_queue_attr_concurrent) }
    // };

    /// A dispatch queue that executes blocks concurrently.
    #[inline]
    pub fn concurrent() -> Option<&'static Self> {
        // SAFETY: Queues are
        unsafe { Some(&_dispatch_queue_attr_concurrent) }
    }
}

/// Executes blocks submitted to the main queue.
#[inline]
pub fn dispatch_main() -> ! {
    extern "C" {
        // `dispatch_main` is marked DISPATCH_NOTHROW.
        fn dispatch_main() -> !;
    }

    // SAFETY: TODO: Must this be run on the main thread? Do we need to take
    // `MainThreadMarker`?
    unsafe { dispatch_main() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_main_queue() {
        let _ = DispatchQueue::main();
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_serial_queue() {
        let queue = DispatchQueue::new("com.github.madsmtm.objc2", DispatchQueueAttr::SERIAL);
        let (tx, rx) = std::sync::mpsc::channel();
        queue.exec_async(move || {
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_concurrent_queue() {
        let queue = DispatchQueue::new("com.github.madsmtm.objc2", DispatchQueueAttr::concurrent());
        let (tx, rx) = std::sync::mpsc::channel();
        let cloned_tx = tx.clone();
        queue.exec_async(move || {
            tx.send(()).unwrap();
        });
        queue.exec_async(move || {
            cloned_tx.send(()).unwrap();
        });
        for _ in 0..2 {
            rx.recv().unwrap();
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_global_default_queue() {
        let queue = DispatchQueue::global_queue(GlobalQueueIdentifier::QualityOfService(
            DispatchQoS::Default,
        ));
        let (tx, rx) = std::sync::mpsc::channel();
        queue.exec_async(move || {
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_share_queue_across_threads() {
        let queue = DispatchQueue::new("com.github.madsmtm.objc2", DispatchQueueAttr::SERIAL);
        let (tx, rx) = std::sync::mpsc::channel();
        let cloned_tx = tx.clone();
        let cloned_queue = queue.clone();
        queue.exec_async(move || {
            cloned_queue.exec_async(move || {
                cloned_tx.send(()).unwrap();
            });
        });
        queue.exec_async(move || {
            tx.send(()).unwrap();
        });
        for _ in 0..2 {
            rx.recv().unwrap();
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_move_queue_between_threads() {
        let queue = DispatchQueue::new("com.github.madsmtm.objc2", DispatchQueueAttr::SERIAL);
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            queue.exec_async(move || {
                tx.send(()).unwrap();
            });
        });
        rx.recv().unwrap();
    }
}
