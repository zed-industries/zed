// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    any::Any, cell::Cell, fmt, marker::PhantomData, mem, num::NonZeroU32, panic, pin::Pin, ptr,
    thread,
};

use futures_channel::oneshot;
use futures_core::{
    future::Future,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};
use futures_task::{FutureObj, LocalFutureObj, LocalSpawn, Spawn, SpawnError};
use futures_util::FutureExt;

use crate::{
    ffi, thread_guard::ThreadGuard, translate::*, MainContext, MainLoop, Priority, Source, SourceId,
};

// Wrapper around Send Futures and non-Send Futures that will panic
// if the non-Send Future is polled/dropped from a different thread
// than where this was created.
enum FutureWrapper {
    Send(FutureObj<'static, Box<dyn Any + Send + 'static>>),
    NonSend(ThreadGuard<LocalFutureObj<'static, Box<dyn Any + 'static>>>),
}

unsafe impl Send for FutureWrapper {}
unsafe impl Sync for FutureWrapper {}

impl Future for FutureWrapper {
    type Output = Box<dyn Any + 'static>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.get_mut() {
            FutureWrapper::Send(fut) => {
                Pin::new(fut).poll(ctx).map(|b| b as Box<dyn Any + 'static>)
            }
            FutureWrapper::NonSend(fut) => Pin::new(fut.get_mut()).poll(ctx),
        }
    }
}

// The TaskSource and WakerSource are split up as the TaskSource
// must only be finalized on the thread that owns the main context,
// but the WakerSource is passed around to arbitrary threads for
// being able to wake up the TaskSource.
//
// The WakerSource is set up as a child source of the TaskSource, i.e.
// whenever it is ready also the TaskSource is ready.
#[repr(C)]
struct TaskSource {
    source: ffi::GSource,
    future: FutureWrapper,
    waker: Waker,
    return_tx: Option<oneshot::Sender<thread::Result<Box<dyn Any + 'static>>>>,
}

#[repr(C)]
struct WakerSource {
    source: ffi::GSource,
}

impl TaskSource {
    unsafe extern "C" fn dispatch(
        source: *mut ffi::GSource,
        callback: ffi::GSourceFunc,
        _user_data: ffi::gpointer,
    ) -> ffi::gboolean {
        let source = &mut *(source as *mut Self);
        debug_assert!(callback.is_none());

        // Poll the TaskSource and ensure we're never called again if the
        // contained Future resolved now.
        if let Poll::Ready(()) = source.poll() {
            ffi::G_SOURCE_REMOVE
        } else {
            ffi::G_SOURCE_CONTINUE
        }
    }

    unsafe extern "C" fn finalize(source: *mut ffi::GSource) {
        let source = source as *mut Self;

        // This will panic if the future was a local future and is dropped from a different thread
        // than where it was created so try to drop it from the main context if we're on another
        // thread and the main context still exists.
        //
        // This can only really happen if the `Source` was manually retrieve from the context, but
        // better safe than sorry.
        match (*source).future {
            FutureWrapper::Send(_) => {
                ptr::drop_in_place(&mut (*source).future);
            }
            FutureWrapper::NonSend(ref mut future) if future.is_owner() => {
                ptr::drop_in_place(&mut (*source).future);
            }
            FutureWrapper::NonSend(ref mut future) => {
                let context = ffi::g_source_get_context(source as *mut ffi::GSource);
                if !context.is_null() {
                    let future = ptr::read(future);
                    let context = MainContext::from_glib_none(context);
                    context.invoke(move || {
                        drop(future);
                    });
                } else {
                    // This will panic
                    ptr::drop_in_place(&mut (*source).future);
                }
            }
        }

        ptr::drop_in_place(&mut (*source).return_tx);

        // Drop the waker to unref the underlying GSource
        ptr::drop_in_place(&mut (*source).waker);
    }
}

impl WakerSource {
    unsafe fn clone_raw(waker: *const ()) -> RawWaker {
        static VTABLE: RawWakerVTable = RawWakerVTable::new(
            WakerSource::clone_raw,
            WakerSource::wake_raw,
            WakerSource::wake_by_ref_raw,
            WakerSource::drop_raw,
        );

        let waker = waker as *const ffi::GSource;
        ffi::g_source_ref(mut_override(waker));
        RawWaker::new(waker as *const (), &VTABLE)
    }

    unsafe fn wake_raw(waker: *const ()) {
        Self::wake_by_ref_raw(waker);
        Self::drop_raw(waker);
    }

    unsafe fn wake_by_ref_raw(waker: *const ()) {
        let waker = waker as *const ffi::GSource;
        ffi::g_source_set_ready_time(mut_override(waker), 0);
    }

    unsafe fn drop_raw(waker: *const ()) {
        let waker = waker as *const ffi::GSource;
        ffi::g_source_unref(mut_override(waker));
    }

    unsafe extern "C" fn dispatch(
        source: *mut ffi::GSource,
        _callback: ffi::GSourceFunc,
        _user_data: ffi::gpointer,
    ) -> ffi::gboolean {
        // Set ready-time to -1 so that we're not called again before
        // being woken up another time.
        ffi::g_source_set_ready_time(mut_override(source), -1);
        ffi::G_SOURCE_CONTINUE
    }
}

unsafe impl Send for TaskSource {}
unsafe impl Sync for TaskSource {}

unsafe impl Send for WakerSource {}
unsafe impl Sync for WakerSource {}

impl TaskSource {
    #[allow(clippy::new_ret_no_self)]
    // checker-ignore-item
    fn new(
        priority: Priority,
        future: FutureWrapper,
        return_tx: Option<oneshot::Sender<thread::Result<Box<dyn Any + 'static>>>>,
    ) -> Source {
        unsafe {
            static TASK_SOURCE_FUNCS: ffi::GSourceFuncs = ffi::GSourceFuncs {
                check: None,
                prepare: None,
                dispatch: Some(TaskSource::dispatch),
                finalize: Some(TaskSource::finalize),
                closure_callback: None,
                closure_marshal: None,
            };
            static WAKER_SOURCE_FUNCS: ffi::GSourceFuncs = ffi::GSourceFuncs {
                check: None,
                prepare: None,
                dispatch: Some(WakerSource::dispatch),
                finalize: None,
                closure_callback: None,
                closure_marshal: None,
            };

            let source = ffi::g_source_new(
                mut_override(&TASK_SOURCE_FUNCS),
                mem::size_of::<Self>() as u32,
            );

            let waker_source = ffi::g_source_new(
                mut_override(&WAKER_SOURCE_FUNCS),
                mem::size_of::<WakerSource>() as u32,
            );

            ffi::g_source_set_priority(source, priority.into_glib());
            ffi::g_source_add_child_source(source, waker_source);

            {
                let source = &mut *(source as *mut Self);
                ptr::write(&mut source.future, future);
                ptr::write(&mut source.return_tx, return_tx);

                // This creates a new reference to the waker source.
                let waker = Waker::from_raw(WakerSource::clone_raw(waker_source as *const ()));
                ptr::write(&mut source.waker, waker);
            }

            // Set ready time to 0 so that the source is immediately dispatched
            // for doing the initial polling. This will then either resolve the
            // future or register the waker wherever necessary.
            ffi::g_source_set_ready_time(waker_source, 0);

            // Unref the waker source, a strong reference to it is stored inside
            // the task source directly and inside the task source as child source.
            ffi::g_source_unref(waker_source);

            from_glib_full(source)
        }
    }

    fn poll(&mut self) -> Poll<()> {
        let source = &self.source as *const _;
        let executor: Borrowed<MainContext> =
            unsafe { from_glib_borrow(ffi::g_source_get_context(mut_override(source))) };

        assert!(
            executor.is_owner(),
            "Polling futures only allowed if the thread is owning the MainContext"
        );

        executor
            .with_thread_default(|| {
                let _enter = futures_executor::enter().unwrap();
                let mut context = Context::from_waker(&self.waker);

                // This will panic if the future was a local future and is called from
                // a different thread than where it was created.
                if let Some(tx) = self.return_tx.take() {
                    let res = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                        Pin::new(&mut self.future).poll(&mut context)
                    }));
                    match res {
                        Ok(Poll::Ready(res)) => {
                            let _ = tx.send(Ok(res));
                            Poll::Ready(())
                        }
                        Ok(Poll::Pending) => {
                            self.return_tx.replace(tx);
                            Poll::Pending
                        }
                        Err(e) => {
                            let _ = tx.send(Err(e));
                            Poll::Ready(())
                        }
                    }
                } else {
                    Pin::new(&mut self.future).poll(&mut context).map(|_| ())
                }
            })
            .expect("current thread is not owner of the main context")
    }
}

// rustdoc-stripper-ignore-next
/// A handle to a task running on a [`MainContext`].
///
/// Like [`std::thread::JoinHandle`] for a task rather than a thread. The return value from the
/// task can be retrieved by awaiting on this object. Dropping the handle "detaches" the task,
/// allowing it to complete but discarding the return value.
#[derive(Debug)]
pub struct JoinHandle<T> {
    rx: oneshot::Receiver<std::thread::Result<Box<dyn Any + 'static>>>,
    source: Source,
    id: Cell<Option<NonZeroU32>>,
    phantom: PhantomData<oneshot::Receiver<std::thread::Result<T>>>,
}

impl<T> JoinHandle<T> {
    #[inline]
    fn new(
        ctx: &MainContext,
        source: Source,
        rx: oneshot::Receiver<std::thread::Result<Box<dyn Any + 'static>>>,
    ) -> Self {
        let id = source.attach(Some(ctx));
        let id = Cell::new(Some(unsafe { NonZeroU32::new_unchecked(id.as_raw()) }));
        Self {
            rx,
            source,
            id,
            phantom: PhantomData,
        }
    }
    // rustdoc-stripper-ignore-next
    /// Returns the internal source ID.
    ///
    /// Returns `None` if the handle was aborted already.
    #[inline]
    pub fn as_raw_source_id(&self) -> Option<u32> {
        self.id.get().map(|i| i.get())
    }
    // rustdoc-stripper-ignore-next
    /// Aborts the task associated with the handle.
    #[inline]
    pub fn abort(&self) {
        self.source.destroy();
        self.id.replace(None);
    }
    // rustdoc-stripper-ignore-next
    /// Returns the [`Source`] associated with this handle.
    #[inline]
    pub fn source(&self) -> &Source {
        &self.source
    }
    // rustdoc-stripper-ignore-next
    /// Safely converts the handle into a [`SourceId`].
    ///
    /// Can be used to discard the return value while still retaining the ability to abort the
    /// underlying task. Returns `Err(self)` if the handle was aborted already.
    pub fn into_source_id(self) -> Result<SourceId, Self> {
        if let Some(id) = self.id.take() {
            Ok(unsafe { SourceId::from_glib(id.get()) })
        } else {
            Err(self)
        }
    }
}

impl<T: 'static> Future for JoinHandle<T> {
    type Output = Result<T, JoinError>;
    #[inline]
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::pin::Pin::new(&mut self.rx).poll(cx).map(|r| match r {
            Err(_) => Err(JoinErrorInner::Cancelled.into()),
            Ok(Err(e)) => Err(JoinErrorInner::Panic(e).into()),
            Ok(Ok(r)) => Ok(*r.downcast().unwrap()),
        })
    }
}

impl<T: 'static> futures_core::FusedFuture for JoinHandle<T> {
    #[inline]
    fn is_terminated(&self) -> bool {
        self.rx.is_terminated()
    }
}

// Safety: We can't rely on the auto implementation because we are retrieving
// the result as a `Box<dyn Any + 'static>` from the [`Source`]. We need to
// rely on type erasure here, so we have to manually assert the Send bound too.
unsafe impl<T: Send> Send for JoinHandle<T> {}

// rustdoc-stripper-ignore-next
/// Variant of [`JoinHandle`] that is returned from [`MainContext::spawn_from_within`].
#[derive(Debug)]
pub struct SpawnWithinJoinHandle<T> {
    rx: Option<oneshot::Receiver<JoinHandle<T>>>,
    join_handle: Option<JoinHandle<T>>,
}

impl<T> SpawnWithinJoinHandle<T> {
    // rustdoc-stripper-ignore-next
    /// Waits until the task is spawned and returns the [`JoinHandle`].
    pub async fn into_inner(self) -> Result<JoinHandle<T>, JoinError> {
        if let Some(join_handle) = self.join_handle {
            return Ok(join_handle);
        }

        if let Some(rx) = self.rx {
            match rx.await {
                Ok(join_handle) => return Ok(join_handle),
                Err(_) => return Err(JoinErrorInner::Cancelled.into()),
            }
        }

        Err(JoinErrorInner::Cancelled.into())
    }
}

impl<T: 'static> Future for SpawnWithinJoinHandle<T> {
    type Output = Result<T, JoinError>;
    #[inline]
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if let Some(ref mut rx) = self.rx {
            match std::pin::Pin::new(rx).poll(cx) {
                std::task::Poll::Pending => return std::task::Poll::Pending,
                std::task::Poll::Ready(Err(_)) => {
                    self.rx = None;
                    return std::task::Poll::Ready(Err(JoinErrorInner::Cancelled.into()));
                }
                std::task::Poll::Ready(Ok(join_handle)) => {
                    self.rx = None;
                    self.join_handle = Some(join_handle);
                }
            }
        }

        if let Some(ref mut join_handle) = self.join_handle {
            match std::pin::Pin::new(join_handle).poll(cx) {
                std::task::Poll::Pending => return std::task::Poll::Pending,
                std::task::Poll::Ready(Err(e)) => {
                    self.join_handle = None;
                    return std::task::Poll::Ready(Err(e));
                }
                std::task::Poll::Ready(Ok(r)) => {
                    self.join_handle = None;
                    return std::task::Poll::Ready(Ok(r));
                }
            }
        }

        std::task::Poll::Ready(Err(JoinErrorInner::Cancelled.into()))
    }
}

impl<T: 'static> futures_core::FusedFuture for SpawnWithinJoinHandle<T> {
    #[inline]
    fn is_terminated(&self) -> bool {
        if let Some(ref rx) = self.rx {
            rx.is_terminated()
        } else if let Some(ref join_handle) = self.join_handle {
            join_handle.is_terminated()
        } else {
            true
        }
    }
}

// rustdoc-stripper-ignore-next
/// Task failure from awaiting a [`JoinHandle`].
#[derive(Debug)]
pub struct JoinError(JoinErrorInner);

impl JoinError {
    // rustdoc-stripper-ignore-next
    /// Returns `true` if the handle was cancelled.
    #[inline]
    pub fn is_cancelled(&self) -> bool {
        matches!(self.0, JoinErrorInner::Cancelled)
    }
    // rustdoc-stripper-ignore-next
    /// Returns `true` if the task terminated with a panic.
    #[inline]
    pub fn is_panic(&self) -> bool {
        matches!(self.0, JoinErrorInner::Panic(_))
    }
    // rustdoc-stripper-ignore-next
    /// Converts the error into a panic result.
    ///
    /// # Panics
    ///
    /// Panics if the error is not a panic error. Use [`is_panic`](Self::is_panic) to check first
    /// if the error represents a panic.
    #[inline]
    pub fn into_panic(self) -> Box<dyn Any + Send + 'static> {
        self.try_into_panic()
            .expect("`JoinError` is not a panic error")
    }
    // rustdoc-stripper-ignore-next
    /// Attempts to convert the error into a panic result.
    ///
    /// Returns `Err(self)` if the error is not a panic result.
    #[inline]
    pub fn try_into_panic(self) -> Result<Box<dyn Any + Send + 'static>, Self> {
        match self.0 {
            JoinErrorInner::Panic(e) => Ok(e),
            e => Err(Self(e)),
        }
    }
}

impl std::error::Error for JoinError {}

impl std::fmt::Display for JoinError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
enum JoinErrorInner {
    Cancelled,
    Panic(Box<dyn Any + Send + 'static>),
}

impl From<JoinErrorInner> for JoinError {
    #[inline]
    fn from(e: JoinErrorInner) -> Self {
        Self(e)
    }
}

impl std::error::Error for JoinErrorInner {}

impl fmt::Display for JoinErrorInner {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Cancelled => fmt.write_str("task cancelled"),
            Self::Panic(_) => fmt.write_str("task panicked"),
        }
    }
}

impl MainContext {
    // rustdoc-stripper-ignore-next
    /// Spawn a new infallible `Future` on the main context.
    ///
    /// This can be called from any thread and will execute the future from the thread
    /// where main context is running, e.g. via a `MainLoop`.
    pub fn spawn<R: Send + 'static, F: Future<Output = R> + Send + 'static>(
        &self,
        f: F,
    ) -> JoinHandle<R> {
        self.spawn_with_priority(crate::Priority::default(), f)
    }

    // rustdoc-stripper-ignore-next
    /// Spawn a new infallible `Future` on the main context.
    ///
    /// The given `Future` does not have to be `Send`.
    ///
    /// This can be called only from the thread where the main context is running, e.g.
    /// from any other `Future` that is executed on this main context, or after calling
    /// `with_thread_default` or `acquire` on the main context.
    pub fn spawn_local<R: 'static, F: Future<Output = R> + 'static>(&self, f: F) -> JoinHandle<R> {
        self.spawn_local_with_priority(crate::Priority::default(), f)
    }

    // rustdoc-stripper-ignore-next
    /// Spawn a new infallible `Future` on the main context, with a non-default priority.
    ///
    /// This can be called from any thread and will execute the future from the thread
    /// where main context is running, e.g. via a `MainLoop`.
    pub fn spawn_with_priority<R: Send + 'static, F: Future<Output = R> + Send + 'static>(
        &self,
        priority: Priority,
        f: F,
    ) -> JoinHandle<R> {
        let f = FutureObj::new(Box::new(async move {
            Box::new(f.await) as Box<dyn Any + Send + 'static>
        }));
        let (tx, rx) = oneshot::channel();
        let source = TaskSource::new(priority, FutureWrapper::Send(f), Some(tx));
        JoinHandle::new(self, source, rx)
    }

    // rustdoc-stripper-ignore-next
    /// Spawn a new infallible `Future` on the main context, with a non-default priority.
    ///
    /// The given `Future` does not have to be `Send`.
    ///
    /// This can be called only from the thread where the main context is running, e.g.
    /// from any other `Future` that is executed on this main context, or after calling
    /// `with_thread_default` or `acquire` on the main context.
    pub fn spawn_local_with_priority<R: 'static, F: Future<Output = R> + 'static>(
        &self,
        priority: Priority,
        f: F,
    ) -> JoinHandle<R> {
        let _acquire = self
            .acquire()
            .expect("Spawning local futures only allowed on the thread owning the MainContext");
        let f = LocalFutureObj::new(Box::new(async move {
            Box::new(f.await) as Box<dyn Any + 'static>
        }));
        let (tx, rx) = oneshot::channel();
        let source = TaskSource::new(
            priority,
            FutureWrapper::NonSend(ThreadGuard::new(f)),
            Some(tx),
        );
        JoinHandle::new(self, source, rx)
    }

    // rustdoc-stripper-ignore-next
    /// Spawn a new infallible `Future` on the main context from inside the main context.
    ///
    /// The given `Future` does not have to be `Send` but the closure to spawn it has to be.
    ///
    /// This can be called only from any thread.
    pub fn spawn_from_within<R: Send + 'static, F: Future<Output = R> + 'static>(
        &self,
        func: impl FnOnce() -> F + Send + 'static,
    ) -> SpawnWithinJoinHandle<R> {
        self.spawn_from_within_with_priority(crate::Priority::default(), func)
    }

    // rustdoc-stripper-ignore-next
    /// Spawn a new infallible `Future` on the main context from inside the main context.
    ///
    /// The given `Future` does not have to be `Send` but the closure to spawn it has to be.
    ///
    /// This can be called only from any thread.
    pub fn spawn_from_within_with_priority<R: Send + 'static, F: Future<Output = R> + 'static>(
        &self,
        priority: Priority,
        func: impl FnOnce() -> F + Send + 'static,
    ) -> SpawnWithinJoinHandle<R> {
        let ctx = self.clone();
        let (tx, rx) = oneshot::channel();
        self.invoke_with_priority(priority, move || {
            let _ = tx.send(ctx.spawn_local(func()));
        });

        SpawnWithinJoinHandle {
            rx: Some(rx),
            join_handle: None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Runs a new, infallible `Future` on the main context and block until it finished, returning
    /// the result of the `Future`.
    ///
    /// The given `Future` does not have to be `Send` or `'static`.
    ///
    /// This must only be called if no `MainLoop` or anything else is running on this specific main
    /// context.
    #[allow(clippy::transmute_ptr_to_ptr)]
    pub fn block_on<F: Future>(&self, f: F) -> F::Output {
        let mut res = None;
        let l = MainLoop::new(Some(self), false);

        let f = async {
            res = Some(panic::AssertUnwindSafe(f).catch_unwind().await);
            l.quit();
        };

        let f = unsafe {
            // Super-unsafe: We transmute here to get rid of the 'static lifetime
            let f = LocalFutureObj::new(Box::new(async move {
                f.await;
                Box::new(()) as Box<dyn Any + 'static>
            }));
            let f: LocalFutureObj<'static, Box<dyn Any + 'static>> = mem::transmute(f);
            f
        };

        let source = TaskSource::new(
            crate::Priority::default(),
            FutureWrapper::NonSend(ThreadGuard::new(f)),
            None,
        );
        source.attach(Some(self));

        l.run();

        match res.unwrap() {
            Ok(v) => v,
            Err(e) => panic::resume_unwind(e),
        }
    }
}

impl Spawn for MainContext {
    fn spawn_obj(&self, f: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        let (tx, _) = oneshot::channel();
        let source = TaskSource::new(
            crate::Priority::default(),
            FutureWrapper::Send(FutureObj::new(Box::new(async move {
                f.await;
                Box::new(()) as Box<dyn Any + Send + 'static>
            }))),
            Some(tx),
        );
        source.attach(Some(self));
        Ok(())
    }
}

impl LocalSpawn for MainContext {
    fn spawn_local_obj(&self, f: LocalFutureObj<'static, ()>) -> Result<(), SpawnError> {
        let (tx, _) = oneshot::channel();
        let source = TaskSource::new(
            crate::Priority::default(),
            FutureWrapper::NonSend(ThreadGuard::new(LocalFutureObj::new(Box::new(
                async move {
                    f.await;
                    Box::new(()) as Box<dyn Any + 'static>
                },
            )))),
            Some(tx),
        );
        source.attach(Some(self));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread};

    use futures_channel::oneshot;
    use futures_util::future::{FutureExt, TryFutureExt};

    use super::*;

    #[test]
    fn test_spawn() {
        let c = MainContext::new();
        let l = crate::MainLoop::new(Some(&c), false);

        let (sender, receiver) = mpsc::channel();
        let (o_sender, o_receiver) = oneshot::channel();

        let l_clone = l.clone();
        c.spawn(
            o_receiver
                .and_then(move |()| {
                    sender.send(()).unwrap();
                    l_clone.quit();

                    futures_util::future::ok(())
                })
                .then(|res| {
                    assert!(res.is_ok());
                    futures_util::future::ready(())
                }),
        );

        let join_handle = thread::spawn(move || {
            l.run();
        });

        o_sender.send(()).unwrap();

        receiver.recv().unwrap();

        join_handle.join().unwrap();
    }

    #[test]
    fn test_spawn_local() {
        let c = MainContext::new();
        let l = crate::MainLoop::new(Some(&c), false);

        c.with_thread_default(|| {
            let l_clone = l.clone();
            c.spawn_local(futures_util::future::lazy(move |_ctx| {
                l_clone.quit();
            }));

            l.run();
        })
        .unwrap();
    }

    #[test]
    fn test_spawn_from_within() {
        let c = MainContext::new();
        let l = crate::MainLoop::new(Some(&c), false);

        let join_handle = std::thread::spawn({
            let l_clone = l.clone();
            move || {
                c.spawn_from_within(move || async move {
                    let rc = std::rc::Rc::new(123);
                    futures_util::future::ready(()).await;
                    assert_eq!(std::rc::Rc::strong_count(&rc), 1);
                    l_clone.quit();
                });
            }
        });

        l.run();

        join_handle.join().unwrap();
    }

    #[test]
    fn test_block_on() {
        let c = MainContext::new();

        let mut v = None;
        {
            let v = &mut v;

            let future = futures_util::future::lazy(|_ctx| {
                *v = Some(123);
                Ok::<i32, ()>(123)
            });

            let res = c.block_on(future);
            assert_eq!(res, Ok(123));
        }

        assert_eq!(v, Some(123));
    }

    #[test]
    fn test_spawn_return() {
        let c = MainContext::new();
        c.block_on(async {
            let val = 1;
            let ret = c
                .spawn(async move { futures_util::future::ready(2).await + val })
                .await;
            assert_eq!(ret.unwrap(), 3);
        });
    }

    #[test]
    fn test_spawn_panic() {
        let c = MainContext::new();
        c.block_on(async {
            let ret = c
                .spawn(async {
                    panic!("failed");
                })
                .await;
            assert_eq!(
                *ret.unwrap_err().into_panic().downcast::<&str>().unwrap(),
                "failed"
            );
        });
    }

    #[test]
    fn test_spawn_abort() {
        let c = MainContext::new();
        let v = std::sync::Arc::new(1);
        let v_clone = v.clone();
        let c_ref = &c;
        c.block_on(async move {
            let handle = c_ref.spawn(async move {
                let _v = v_clone;
                let test: u128 = std::future::pending().await;
                println!("{test}");
                unreachable!();
            });

            handle.abort();
        });
        drop(c);

        // Make sure the inner future is actually freed.
        assert_eq!(std::sync::Arc::strong_count(&v), 1);
    }
}
