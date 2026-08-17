// Take a look at the license at the top of the repository in the LICENSE file.

use std::{future::Future, panic, ptr};

use futures_channel::oneshot;

use crate::{ffi, translate::*};

#[derive(Debug)]
#[doc(alias = "GThreadPool")]
pub struct ThreadPool(ptr::NonNull<ffi::GThreadPool>);

unsafe impl Send for ThreadPool {}
unsafe impl Sync for ThreadPool {}

// rustdoc-stripper-ignore-next
/// A handle to a thread running on a [`ThreadPool`].
///
/// Like [`std::thread::JoinHandle`] for a GLib thread. The return value from the task can be
/// retrieved by calling [`ThreadHandle::join`]. Dropping the handle "detaches" the thread,
/// allowing it to complete but discarding the return value.
#[derive(Debug)]
pub struct ThreadHandle<T> {
    rx: std::sync::mpsc::Receiver<std::thread::Result<T>>,
}

impl<T> ThreadHandle<T> {
    // rustdoc-stripper-ignore-next
    /// Waits for the associated thread to finish.
    ///
    /// Blocks until the associated thread returns. Returns `Ok` with the value returned from the
    /// thread, or `Err` if the thread panicked. This function will return immediately if the
    /// associated thread has already finished.
    #[inline]
    pub fn join(self) -> std::thread::Result<T> {
        self.rx.recv().unwrap()
    }
}

impl ThreadPool {
    #[doc(alias = "g_thread_pool_new")]
    pub fn shared(max_threads: Option<u32>) -> Result<Self, crate::Error> {
        unsafe {
            let mut err = ptr::null_mut();
            let pool = ffi::g_thread_pool_new(
                Some(spawn_func),
                ptr::null_mut(),
                max_threads.map(|v| v as i32).unwrap_or(-1),
                ffi::GFALSE,
                &mut err,
            );
            if pool.is_null() {
                Err(from_glib_full(err))
            } else {
                Ok(ThreadPool(ptr::NonNull::new_unchecked(pool)))
            }
        }
    }

    #[doc(alias = "g_thread_pool_new")]
    pub fn exclusive(max_threads: u32) -> Result<Self, crate::Error> {
        unsafe {
            let mut err = ptr::null_mut();
            let pool = ffi::g_thread_pool_new(
                Some(spawn_func),
                ptr::null_mut(),
                max_threads as i32,
                ffi::GTRUE,
                &mut err,
            );
            if pool.is_null() {
                Err(from_glib_full(err))
            } else {
                Ok(ThreadPool(ptr::NonNull::new_unchecked(pool)))
            }
        }
    }

    #[doc(alias = "g_thread_pool_push")]
    pub fn push<T: Send + 'static, F: FnOnce() -> T + Send + 'static>(
        &self,
        func: F,
    ) -> Result<ThreadHandle<T>, crate::Error> {
        let (tx, rx) = std::sync::mpsc::sync_channel(1);
        unsafe {
            let func: Box<dyn FnOnce() + Send + 'static> = Box::new(move || {
                let _ = tx.send(panic::catch_unwind(panic::AssertUnwindSafe(func)));
            });
            let func = Box::new(func);
            let mut err = ptr::null_mut();

            let func = Box::into_raw(func);
            let ret: bool = from_glib(ffi::g_thread_pool_push(
                self.0.as_ptr(),
                func as *mut _,
                &mut err,
            ));
            if ret {
                Ok(ThreadHandle { rx })
            } else {
                let _ = Box::from_raw(func);
                Err(from_glib_full(err))
            }
        }
    }

    pub fn push_future<T: Send + 'static, F: FnOnce() -> T + Send + 'static>(
        &self,
        func: F,
    ) -> Result<impl Future<Output = std::thread::Result<T>> + Send + Sync + 'static, crate::Error>
    {
        let (sender, receiver) = oneshot::channel();

        self.push(move || {
            let _ = sender.send(panic::catch_unwind(panic::AssertUnwindSafe(func)));
        })?;

        Ok(async move { receiver.await.expect("Dropped before executing") })
    }

    #[doc(alias = "g_thread_pool_set_max_threads")]
    pub fn set_max_threads(&self, max_threads: Option<u32>) -> Result<(), crate::Error> {
        unsafe {
            let mut err = ptr::null_mut();
            let ret: bool = from_glib(ffi::g_thread_pool_set_max_threads(
                self.0.as_ptr(),
                max_threads.map(|v| v as i32).unwrap_or(-1),
                &mut err,
            ));
            if ret {
                Ok(())
            } else {
                Err(from_glib_full(err))
            }
        }
    }

    #[doc(alias = "g_thread_pool_get_max_threads")]
    #[doc(alias = "get_max_threads")]
    pub fn max_threads(&self) -> Option<u32> {
        unsafe {
            let max_threads = ffi::g_thread_pool_get_max_threads(self.0.as_ptr());
            if max_threads == -1 {
                None
            } else {
                Some(max_threads as u32)
            }
        }
    }

    #[doc(alias = "g_thread_pool_get_num_threads")]
    #[doc(alias = "get_num_threads")]
    pub fn num_threads(&self) -> u32 {
        unsafe { ffi::g_thread_pool_get_num_threads(self.0.as_ptr()) }
    }

    #[doc(alias = "g_thread_pool_unprocessed")]
    #[doc(alias = "get_unprocessed")]
    pub fn unprocessed(&self) -> u32 {
        unsafe { ffi::g_thread_pool_unprocessed(self.0.as_ptr()) }
    }

    #[doc(alias = "g_thread_pool_set_max_unused_threads")]
    pub fn set_max_unused_threads(max_threads: Option<u32>) {
        unsafe {
            ffi::g_thread_pool_set_max_unused_threads(max_threads.map(|v| v as i32).unwrap_or(-1))
        }
    }

    #[doc(alias = "g_thread_pool_get_max_unused_threads")]
    #[doc(alias = "get_max_unused_threads")]
    pub fn max_unused_threads() -> Option<u32> {
        unsafe {
            let max_unused_threads = ffi::g_thread_pool_get_max_unused_threads();
            if max_unused_threads == -1 {
                None
            } else {
                Some(max_unused_threads as u32)
            }
        }
    }

    #[doc(alias = "g_thread_pool_get_num_unused_threads")]
    #[doc(alias = "get_num_unused_threads")]
    pub fn num_unused_threads() -> u32 {
        unsafe { ffi::g_thread_pool_get_num_unused_threads() }
    }

    #[doc(alias = "g_thread_pool_stop_unused_threads")]
    pub fn stop_unused_threads() {
        unsafe {
            ffi::g_thread_pool_stop_unused_threads();
        }
    }

    #[doc(alias = "g_thread_pool_set_max_idle_time")]
    pub fn set_max_idle_time(max_idle_time: u32) {
        unsafe { ffi::g_thread_pool_set_max_idle_time(max_idle_time) }
    }

    #[doc(alias = "g_thread_pool_get_max_idle_time")]
    #[doc(alias = "get_max_idle_time")]
    pub fn max_idle_time() -> u32 {
        unsafe { ffi::g_thread_pool_get_max_idle_time() }
    }
}

impl Drop for ThreadPool {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::g_thread_pool_free(self.0.as_ptr(), ffi::GFALSE, ffi::GTRUE);
        }
    }
}

unsafe extern "C" fn spawn_func(func: ffi::gpointer, _data: ffi::gpointer) {
    let func: Box<Box<dyn FnOnce()>> = Box::from_raw(func as *mut _);
    func()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        use std::sync::mpsc;

        let p = ThreadPool::exclusive(1).unwrap();
        let (sender, receiver) = mpsc::channel();

        let handle = p
            .push(move || {
                sender.send(true).unwrap();
                123
            })
            .unwrap();

        assert_eq!(handle.join().unwrap(), 123);
        assert_eq!(receiver.recv(), Ok(true));
    }

    #[test]
    fn test_push_future() {
        let c = crate::MainContext::new();
        let p = ThreadPool::shared(None).unwrap();

        let fut = p.push_future(|| true).unwrap();

        let res = c.block_on(fut);
        assert!(res.unwrap());
    }
}
