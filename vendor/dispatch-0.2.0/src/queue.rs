use std::ffi::{CStr, CString};
use std::os::raw::c_long;
use std::ptr;
use std::str;
use std::time::Duration;

use crate::ffi::*;
use crate::{
    context_and_function, context_and_sync_function, context_and_apply_function,
    time_after_delay,
};

/// The type of a dispatch queue.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum QueueAttribute {
    /// The queue executes blocks serially in FIFO order.
    Serial,
    /// The queue executes blocks concurrently.
    Concurrent,
}

impl QueueAttribute {
    #[cfg(not(all(test, target_os = "linux")))]
    fn as_raw(&self) -> dispatch_queue_attr_t {
        match *self {
            QueueAttribute::Serial => DISPATCH_QUEUE_SERIAL,
            QueueAttribute::Concurrent => DISPATCH_QUEUE_CONCURRENT,
        }
    }

    #[cfg(all(test, target_os = "linux"))]
    fn as_raw(&self) -> dispatch_queue_attr_t {
        // The Linux tests use Ubuntu's libdispatch-dev package, which is
        // apparently really old from before OSX 10.7.
        // Back then, the attr for dispatch_queue_create must be NULL.
        ptr::null()
    }
}

/// The priority of a global concurrent queue.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum QueuePriority {
    /// The queue is scheduled for execution before any default priority or low
    /// priority queue.
    High,
    /// The queue is scheduled for execution after all high priority queues,
    /// but before any low priority queues.
    Default,
    /// The queue is scheduled for execution after all default priority and
    /// high priority queues.
    Low,
    /// The queue is scheduled for execution after all high priority queues
    /// have been scheduled. The system runs items on a thread whose
    /// priority is set for background status and any disk I/O is throttled to
    /// minimize the impact on the system.
    Background,
}

impl QueuePriority {
    fn as_raw(&self) -> c_long {
        match *self {
            QueuePriority::High       => DISPATCH_QUEUE_PRIORITY_HIGH,
            QueuePriority::Default    => DISPATCH_QUEUE_PRIORITY_DEFAULT,
            QueuePriority::Low        => DISPATCH_QUEUE_PRIORITY_LOW,
            QueuePriority::Background => DISPATCH_QUEUE_PRIORITY_BACKGROUND,
        }
    }
}

/// A Grand Central Dispatch queue.
///
/// For more information, see Apple's [Grand Central Dispatch reference](
/// https://developer.apple.com/library/mac/documentation/Performance/Reference/GCD_libdispatch_Ref/index.html).
#[derive(Debug)]
pub struct Queue {
    pub(crate) ptr: dispatch_queue_t,
}

impl Queue {
    /// Returns the serial dispatch `Queue` associated with the application's
    /// main thread.
    pub fn main() -> Self {
        let queue = dispatch_get_main_queue();
        unsafe {
            dispatch_retain(queue);
        }
        Queue { ptr: queue }
    }

    /// Returns a system-defined global concurrent `Queue` with the specified
    /// priority.
    pub fn global(priority: QueuePriority) -> Self {
        unsafe {
            let queue = dispatch_get_global_queue(priority.as_raw(), 0);
            dispatch_retain(queue);
            Queue { ptr: queue }
        }
    }

    /// Creates a new dispatch `Queue`.
    pub fn create(label: &str, attr: QueueAttribute) -> Self {
        let label = CString::new(label).unwrap();
        let queue = unsafe {
            dispatch_queue_create(label.as_ptr(), attr.as_raw())
        };
        Queue { ptr: queue }
    }

    /// Creates a new dispatch `Queue` with the given target queue.
    ///
    /// A dispatch queue's priority is inherited from its target queue.
    /// Additionally, if both the queue and its target are serial queues,
    /// their blocks will not be invoked concurrently.
    pub fn with_target_queue(label: &str, attr: QueueAttribute, target: &Queue)
            -> Self {
        let queue = Queue::create(label, attr);
        unsafe {
            dispatch_set_target_queue(queue.ptr, target.ptr);
        }
        queue
    }

    /// Returns the label that was specified for self.
    pub fn label(&self) -> &str {
        let label = unsafe {
            let label_ptr = dispatch_queue_get_label(self.ptr);
            if label_ptr.is_null() {
                return "";
            }
            CStr::from_ptr(label_ptr)
        };
        str::from_utf8(label.to_bytes()).unwrap()
    }

    /// Submits a closure for execution on self and waits until it completes.
    pub fn exec_sync<T, F>(&self, work: F) -> T
            where F: Send + FnOnce() -> T, T: Send {
        let mut result = None;
        {
            let result_ref = &mut result;
            let work = move || {
                *result_ref = Some(work());
            };

            let mut work = Some(work);
            let (context, work) = context_and_sync_function(&mut work);
            unsafe {
                dispatch_sync_f(self.ptr, context, work);
            }
        }
        // This was set so it's safe to unwrap
        result.unwrap()
    }

    /// Submits a closure for asynchronous execution on self and returns
    /// immediately.
    pub fn exec_async<F>(&self, work: F) where F: 'static + Send + FnOnce() {
        let (context, work) = context_and_function(work);
        unsafe {
            dispatch_async_f(self.ptr, context, work);
        }
    }

    /// After the specified delay, submits a closure for asynchronous execution
    /// on self.
    pub fn exec_after<F>(&self, delay: Duration, work: F)
            where F: 'static + Send + FnOnce() {
        let when = time_after_delay(delay);
        let (context, work) = context_and_function(work);
        unsafe {
            dispatch_after_f(when, self.ptr, context, work);
        }
    }

    /// Submits a closure to be executed on self the given number of iterations
    /// and waits until it completes.
    pub fn apply<F>(&self, iterations: usize, work: F)
            where F: Sync + Fn(usize) {
        let (context, work) = context_and_apply_function(&work);
        unsafe {
            dispatch_apply_f(iterations, self.ptr, context, work);
        }
    }

    /// Submits a closure to be executed on self for each element of the
    /// provided slice and waits until it completes.
    pub fn for_each<T, F>(&self, slice: &mut [T], work: F)
            where F: Sync + Fn(&mut T), T: Send {
        let slice_ptr = slice.as_mut_ptr();
        let work = move |i| unsafe {
            work(&mut *slice_ptr.offset(i as isize));
        };
        let (context, work) = context_and_apply_function(&work);
        unsafe {
            dispatch_apply_f(slice.len(), self.ptr, context, work);
        }
    }

    /// Submits a closure to be executed on self for each element of the
    /// provided vector and returns a `Vec` of the mapped elements.
    pub fn map<T, U, F>(&self, vec: Vec<T>, work: F) -> Vec<U>
            where F: Sync + Fn(T) -> U, T: Send, U: Send {
        let mut src = vec;
        let len = src.len();
        let src_ptr = src.as_ptr();

        let mut dest: Vec<U> = Vec::with_capacity(len);
        let dest_ptr = dest.as_mut_ptr();

        let work = move |i| unsafe {
            let result = work(ptr::read(src_ptr.offset(i as isize)));
            ptr::write(dest_ptr.offset(i as isize), result);
        };
        let (context, work) = context_and_apply_function(&work);
        unsafe {
            src.set_len(0);
            dispatch_apply_f(len, self.ptr, context, work);
            dest.set_len(len);
        }

        dest
    }

    /// Submits a closure to be executed on self as a barrier and waits until
    /// it completes.
    ///
    /// Barriers create synchronization points within a concurrent queue.
    /// If self is concurrent, when it encounters a barrier it delays execution
    /// of the closure (and any further ones) until all closures submitted
    /// before the barrier finish executing.
    /// At that point, the barrier closure executes by itself.
    /// Upon completion, self resumes its normal execution behavior.
    ///
    /// If self is a serial queue or one of the global concurrent queues,
    /// this method behaves like the normal `sync` method.
    pub fn barrier_sync<T, F>(&self, work: F) -> T
            where F: Send + FnOnce() -> T, T: Send {
        let mut result = None;
        {
            let result_ref = &mut result;
            let work = move || {
                *result_ref = Some(work());
            };

            let mut work = Some(work);
            let (context, work) = context_and_sync_function(&mut work);
            unsafe {
                dispatch_barrier_sync_f(self.ptr, context, work);
            }
        }
        // This was set so it's safe to unwrap
        result.unwrap()
    }

    /// Submits a closure to be executed on self as a barrier and returns
    /// immediately.
    ///
    /// Barriers create synchronization points within a concurrent queue.
    /// If self is concurrent, when it encounters a barrier it delays execution
    /// of the closure (and any further ones) until all closures submitted
    /// before the barrier finish executing.
    /// At that point, the barrier closure executes by itself.
    /// Upon completion, self resumes its normal execution behavior.
    ///
    /// If self is a serial queue or one of the global concurrent queues,
    /// this method behaves like the normal `async` method.
    pub fn barrier_async<F>(&self, work: F)
            where F: 'static + Send + FnOnce() {
        let (context, work) = context_and_function(work);
        unsafe {
            dispatch_barrier_async_f(self.ptr, context, work);
        }
    }

    /// Suspends the invocation of blocks on self and returns a `SuspendGuard`
    /// that can be dropped to resume.
    ///
    /// The suspension occurs after completion of any blocks running at the
    /// time of the call.
    /// Invocation does not resume until all `SuspendGuard`s have been dropped.
    pub fn suspend(&self) -> SuspendGuard {
        SuspendGuard::new(self)
    }
}

unsafe impl Sync for Queue { }
unsafe impl Send for Queue { }

impl Clone for Queue {
    fn clone(&self) -> Self {
        unsafe {
            dispatch_retain(self.ptr);
        }
        Queue { ptr: self.ptr }
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            dispatch_release(self.ptr);
        }
    }
}

/// An RAII guard which will resume a suspended `Queue` when dropped.
#[derive(Debug)]
pub struct SuspendGuard {
    queue: Queue,
}

impl SuspendGuard {
    fn new(queue: &Queue) -> SuspendGuard {
        unsafe {
            dispatch_suspend(queue.ptr);
        }
        SuspendGuard { queue: queue.clone() }
    }

    /// Drops self, allowing the suspended `Queue` to resume.
    pub fn resume(self) { }
}

impl Clone for SuspendGuard {
    fn clone(&self) -> Self {
        SuspendGuard::new(&self.queue)
    }
}

impl Drop for SuspendGuard {
    fn drop(&mut self) {
        unsafe {
            dispatch_resume(self.queue.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};
    use crate::Group;
    use super::*;

    fn async_increment(queue: &Queue, num: &Arc<Mutex<i32>>) {
        let num = num.clone();
        queue.exec_async(move || {
            let mut num = num.lock().unwrap();
            *num += 1;
        });
    }

    #[test]
    fn test_serial_queue() {
        let q = Queue::create("", QueueAttribute::Serial);
        let mut num = 0;

        q.exec_sync(|| num = 1);
        assert_eq!(num, 1);

        assert_eq!(q.exec_sync(|| num), 1);
    }

    #[test]
    fn test_sync_owned() {
        let q = Queue::create("", QueueAttribute::Serial);

        let s = "Hello, world!".to_string();
        let len = q.exec_sync(move || s.len());
        assert_eq!(len, 13);
    }

    #[test]
    fn test_serial_queue_async() {
        let q = Queue::create("", QueueAttribute::Serial);
        let num = Arc::new(Mutex::new(0));

        async_increment(&q, &num);

        // Sync an empty block to ensure the async one finishes
        q.exec_sync(|| ());
        assert_eq!(*num.lock().unwrap(), 1);
    }

    #[test]
    fn test_after() {
        let q = Queue::create("", QueueAttribute::Serial);
        let group = Group::create();
        let num = Arc::new(Mutex::new(0));

        let delay = Duration::from_millis(5);
        let num2 = num.clone();
        let guard = group.enter();
        let start = Instant::now();
        q.exec_after(delay, move || {
            let mut num = num2.lock().unwrap();
            *num = 1;
            guard.leave();
        });

        // Wait for the previous block to complete
        group.wait_timeout(Duration::from_millis(5000)).unwrap();
        assert!(start.elapsed() >= delay);
        assert_eq!(*num.lock().unwrap(), 1);
    }

    #[test]
    fn test_queue_label() {
        let q = Queue::create("com.example.rust", QueueAttribute::Serial);
        assert_eq!(q.label(), "com.example.rust");
    }

    #[test]
    fn test_apply() {
        let q = Queue::create("", QueueAttribute::Serial);
        let num = Arc::new(Mutex::new(0));

        q.apply(5, |_| *num.lock().unwrap() += 1);
        assert_eq!(*num.lock().unwrap(), 5);
    }

    #[test]
    fn test_for_each() {
        let q = Queue::create("", QueueAttribute::Serial);
        let mut nums = [0, 1];

        q.for_each(&mut nums, |x| *x += 1);
        assert_eq!(nums, [1, 2]);
    }

    #[test]
    fn test_map() {
        let q = Queue::create("", QueueAttribute::Serial);
        let nums = vec![0, 1];

        let result = q.map(nums, |x| x + 1);
        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn test_barrier_sync() {
        let q = Queue::create("", QueueAttribute::Concurrent);
        let num = Arc::new(Mutex::new(0));

        async_increment(&q, &num);
        async_increment(&q, &num);

        let num2 = num.clone();
        let result = q.barrier_sync(move || {
            let mut num = num2.lock().unwrap();
            if *num == 2 {
                *num = 10;
            }
            *num
        });
        assert_eq!(result, 10);

        async_increment(&q, &num);
        async_increment(&q, &num);

        q.barrier_sync(|| ());
        assert_eq!(*num.lock().unwrap(), 12);
    }

    #[test]
    fn test_barrier_async() {
        let q = Queue::create("", QueueAttribute::Concurrent);
        let num = Arc::new(Mutex::new(0));

        async_increment(&q, &num);
        async_increment(&q, &num);

        let num2 = num.clone();
        q.barrier_async(move || {
            let mut num = num2.lock().unwrap();
            if *num == 2 {
                *num = 10;
            }
        });

        async_increment(&q, &num);
        async_increment(&q, &num);

        q.barrier_sync(|| ());
        assert_eq!(*num.lock().unwrap(), 12);
    }

    #[test]
    fn test_suspend() {
        let q = Queue::create("", QueueAttribute::Serial);
        let num = Arc::new(Mutex::new(0));

        // Suspend the queue and then dispatch some work to it
        let guard = q.suspend();
        async_increment(&q, &num);

        // Sleep and ensure the work doesn't occur
        ::std::thread::sleep(Duration::from_millis(5));
        assert_eq!(*num.lock().unwrap(), 0);

        // But ensure the work does complete after we resume
        guard.resume();
        q.exec_sync(|| ());
        assert_eq!(*num.lock().unwrap(), 1);
    }
}
