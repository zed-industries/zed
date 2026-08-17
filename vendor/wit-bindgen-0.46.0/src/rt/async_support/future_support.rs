//! Runtime support for `future<T>` in the component model.
//!
//! There are a number of tricky concerns to all balance when implementing
//! bindings to `future<T>`, specifically with how it interacts with Rust. This
//! will attempt to go over some of the high-level details of the implementation
//! here.
//!
//! ## Leak safety
//!
//! It's safe to leak any value at any time currently in Rust. In other words
//! Rust doesn't have linear types (yet). Typically this isn't really a problem
//! but the component model intrinsics we're working with here operate by being
//! given a pointer and then at some point in the future the pointer may be
//! read. This means that it's our responsibility to keep this pointer alive and
//! valid for the entire duration of an asynchronous operation.
//!
//! Chiefly this means that borrowed values are a no-no in this module. For
//! example if you were to send a `&[u8]` as an implementation of
//! `future<list<u8>>` that would not be sound. For example:
//!
//! * The future send operation is started, recording an address of `&[u8]`.
//! * The future is then leaked.
//! * According to rustc, later in code the original `&[u8]` is then no longer
//!   borrowed.
//! * The original source of `&[u8]` could then be deallocated.
//! * Then the component model actually reads the pointer that it was given.
//!
//! This constraint effectively means that all types flowing in-and-out of
//! futures, streams, and async APIs are all "owned values", notably no
//! lifetimes. This requires, for example, that `future<list<u8>>` operates on
//! `Vec<u8>`.
//!
//! This is in stark contrast to bindings generated for `list<u8>` otherwise,
//! however, where for example a synchronous import with a `list<u8>` argument
//! would be bound with a `&[u8]` argument. Until Rust has some form of linear
//! types, however, it's not possible to loosen this restriction soundly because
//! it's generally not safe to leak an active I/O operation. This restriction is
//! similar to why it's so difficult to bind `io_uring` in safe Rust, which
//! operates similarly to the component model where pointers are submitted and
//! read in the future after the original call for submission returns.
//!
//! ## Lowering Owned Values
//!
//! According to the above everything with futures/streams operates on owned
//! values already, but this also affects precisely how lifting and lowering is
//! performed. In general any active asynchronous operation could be cancelled
//! at any time, meaning we have to deal with situations such as:
//!
//! * A `write` hasn't even started yet.
//! * A `write` was started and then cancelled.
//! * A `write` was started and then the other end dropped the channel.
//! * A `write` was started and then the other end received the value.
//!
//! In all of these situations regardless of the structure of `T` we can't leak
//! memory. The `future.write` intrinsic, however, takes no ownership of the
//! memory involved which means that we're still responsible for cleaning up
//! lists. It does take ownership, however, of `own<T>` handles and other
//! resources.
//!
//! The way that this is solved for futures/streams is to lean further into
//! processing owned values. Namely lowering a `T` takes `T`-by-value, not `&T`.
//! This means that lowering operates similarly to return values of exported
//! functions, not parameters to imported functions. By lowering an owned value
//! of `T` this preserves a nice property where the lowered value has exclusive
//! ownership of all of its pointers/resources/etc. Lowering `&T` may require a
//! "cleanup list" for example which we avoid here entirely.
//!
//! This then makes the second and third cases above, getting a value back after
//! lowering, much easier. Namely re-acquisition of a value is simple `lift`
//! operation as if we received a value on the channel.
//!
//! ## Inefficiencies
//!
//! The above requirements generally mean that this is not a hyper-efficient
//! implementation. All writes and reads, for example, start out with allocation
//! memory on the heap to be owned by the asynchronous operation. Writing a
//! `list<u8>` to a future passes ownership of `Vec<u8>` but in theory doesn't
//! not actually require relinquishing ownership of the vector. Furthermore
//! there's no way to re-acquire a `T` after it has been sent, but all of `T` is
//! still valid except for `own<U>` resources.
//!
//! That's all to say that this implementation can probably still be improved
//! upon, but doing so is thought to be pretty nontrivial at this time. It
//! should be noted though that there are other high-level inefficiencies with
//! WIT unrelated to this module. For example `list<T>` is not always
//! represented the same in Rust as it is in the canonical ABI. That means that
//! sending `list<T>` into a future might require copying the entire list and
//! changing its layout. Currently this is par-for-the-course with bindings.
//!
//! ## Linear (exactly once) Writes
//!
//! The component model requires that a writable end of a future must be written
//! to before closing, otherwise the drop operation traps. Ideally usage of
//! this API shouldn't result in traps so this is modeled in the Rust-level API
//! to prevent this trap from occurring. Rust does not support linear types
//! (types that must be used exactly once), instead it only has affine types
//! (types which must be used at most once), meaning that this requires some
//! runtime support.
//!
//! Specifically the `FutureWriter` structure stores two auxiliary Rust-specific
//! pieces of information:
//!
//! * A `should_write_default_value` boolean - if `true` on destruction then a
//!   value has not yet been written and something must be written.
//! * A `default: fn() -> T` constructor to lazily create the default value to
//!   be sent in this situation.
//!
//! This `default` field is provided by the user when the future is initially
//! created. Additionally during `Drop` a new Rust-level task will be spawned to
//! perform the write in the background. That'll keep the component-level task
//! alive until that write completes but otherwise shouldn't hinder anything
//! else.

use {
    crate::rt::async_support::waitable::{WaitableOp, WaitableOperation},
    crate::rt::async_support::ReturnCode,
    crate::rt::Cleanup,
    std::{
        alloc::Layout,
        fmt,
        future::{Future, IntoFuture},
        marker,
        pin::Pin,
        ptr,
        sync::atomic::{AtomicU32, Ordering::Relaxed},
        task::{Context, Poll},
    },
};

/// Function table used for [`FutureWriter`] and [`FutureReader`]
///
/// Instances of this table are generated by `wit_bindgen::generate!`. This is
/// not a trait to enable different `FutureVtable<()>` instances to exist, for
/// example, through different calls to `wit_bindgen::generate!`.
///
/// It's not intended that any user implements this vtable, instead it's
/// intended to only be auto-generated.
#[doc(hidden)]
pub struct FutureVtable<T> {
    /// The Canonical ABI layout of `T` in-memory.
    pub layout: Layout,

    /// A callback to consume a value of `T` and lower it to the canonical ABI
    /// pointed to by `dst`.
    ///
    /// The `dst` pointer should have `self.layout`. This is used to convert
    /// in-memory representations in Rust to their canonical representations in
    /// the component model.
    pub lower: unsafe fn(value: T, dst: *mut u8),

    /// A callback to deallocate any lists within the canonical ABI value `dst`
    /// provided.
    ///
    /// This is used when a value is successfully sent to another component. In
    /// such a situation it may be possible that the canonical lowering of `T`
    /// has lists that are still owned by this component and must be
    /// deallocated. This is akin to a `post-return` callback for returns of
    /// exported functions.
    pub dealloc_lists: unsafe fn(dst: *mut u8),

    /// A callback to lift a value of `T` from the canonical ABI representation
    /// provided.
    pub lift: unsafe fn(dst: *mut u8) -> T,

    /// The raw `future.write` intrinsic.
    pub start_write: unsafe extern "C" fn(future: u32, val: *const u8) -> u32,
    /// The raw `future.read` intrinsic.
    pub start_read: unsafe extern "C" fn(future: u32, val: *mut u8) -> u32,
    /// The raw `future.cancel-write` intrinsic.
    pub cancel_write: unsafe extern "C" fn(future: u32) -> u32,
    /// The raw `future.cancel-read` intrinsic.
    pub cancel_read: unsafe extern "C" fn(future: u32) -> u32,
    /// The raw `future.drop-writable` intrinsic.
    pub drop_writable: unsafe extern "C" fn(future: u32),
    /// The raw `future.drop-readable` intrinsic.
    pub drop_readable: unsafe extern "C" fn(future: u32),
    /// The raw `future.new` intrinsic.
    pub new: unsafe extern "C" fn() -> u64,
}

/// Helper function to create a new read/write pair for a component model
/// future.
///
/// # Unsafety
///
/// This function is unsafe as it requires the functions within `vtable` to
/// correctly uphold the contracts of the component model.
pub unsafe fn future_new<T>(
    default: fn() -> T,
    vtable: &'static FutureVtable<T>,
) -> (FutureWriter<T>, FutureReader<T>) {
    unsafe {
        let handles = (vtable.new)();
        let reader = handles as u32;
        let writer = (handles >> 32) as u32;
        rtdebug!("future.new() = [{writer}, {reader}]");
        (
            FutureWriter::new(writer, default, vtable),
            FutureReader::new(reader, vtable),
        )
    }
}

/// Represents the writable end of a Component Model `future`.
///
/// A [`FutureWriter`] can be used to send a single value of `T` to the other
/// end of a `future`. In a sense this is similar to a oneshot channel in Rust.
pub struct FutureWriter<T: 'static> {
    handle: u32,
    vtable: &'static FutureVtable<T>,

    /// Whether or not a value should be written during `drop`.
    ///
    /// This is set to `false` when a value is successfully written or when a
    /// value is written but the future is witnessed as being dropped.
    ///
    /// Note that this is set to `true` on construction to ensure that only
    /// location which actually witness a completed write set it to `false`.
    should_write_default_value: bool,

    /// Constructor for the default value to write during `drop`, should one
    /// need to be written.
    default: fn() -> T,
}

impl<T> FutureWriter<T> {
    /// Helper function to wrap a handle/vtable into a `FutureWriter`.
    ///
    /// # Unsafety
    ///
    /// This function is unsafe as it requires the functions within `vtable` to
    /// correctly uphold the contracts of the component model.
    #[doc(hidden)]
    pub unsafe fn new(handle: u32, default: fn() -> T, vtable: &'static FutureVtable<T>) -> Self {
        Self {
            handle,
            default,
            should_write_default_value: true,
            vtable,
        }
    }

    /// Write the specified `value` to this `future`.
    ///
    /// This method is equivalent to an `async fn` which sends the `value` into
    /// this future. The asynchronous operation acts as a rendezvous where the
    /// operation does not complete until the other side has successfully
    /// received the value.
    ///
    /// # Return Value
    ///
    /// The returned [`FutureWrite`] is a future that can be `.await`'d. The
    /// return value of this future is:
    ///
    /// * `Ok(())` - the `value` was sent and received. The `self` value was
    ///   consumed along the way and will no longer be accessible.
    /// * `Err(FutureWriteError { value })` - an attempt was made to send
    ///   `value` but the other half of this [`FutureWriter`] was dropped before
    ///   the value was received. This consumes `self` because the channel is
    ///   now dropped, but `value` is returned in case the caller wants to reuse
    ///   it.
    ///
    /// # Cancellation
    ///
    /// The returned future can be cancelled normally via `drop` which means
    /// that the `value` provided here, along with this `FutureWriter` itself,
    /// will be lost. There is also [`FutureWrite::cancel`] which can be used to
    /// possibly re-acquire `value` and `self` if the operation was cancelled.
    /// In such a situation the operation can be retried at a future date.
    pub fn write(self, value: T) -> FutureWrite<T> {
        FutureWrite {
            op: WaitableOperation::new((self, value)),
        }
    }
}

impl<T> fmt::Debug for FutureWriter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FutureWriter")
            .field("handle", &self.handle)
            .finish()
    }
}

impl<T> Drop for FutureWriter<T> {
    fn drop(&mut self) {
        // If a value has not yet been written into this writer than that must
        // be done so now. Perform a "clone" of `self` by moving our data into a
        // subtask, but ensure that `should_write_default_value` is set to
        // `false` to avoid infinite loops by accident. Once the task is spawned
        // we're done and the subtask's destructor of the closed-over
        // `FutureWriter` will be responsible for performing the
        // `drop-writable` call below.
        //
        // Note, though, that if `should_write_default_value` is `false` then a
        // write has already happened and we can go ahead and just synchronously
        // drop this writer as we would any other handle.
        if self.should_write_default_value {
            let clone = FutureWriter {
                handle: self.handle,
                default: self.default,
                should_write_default_value: false,
                vtable: self.vtable,
            };
            crate::rt::async_support::spawn(async move {
                let value = (clone.default)();
                let _ = clone.write(value).await;
            });
        } else {
            unsafe {
                rtdebug!("future.drop-writable({})", self.handle);
                (self.vtable.drop_writable)(self.handle);
            }
        }
    }
}

/// Represents a write operation which may be cancelled prior to completion.
///
/// This is returned by [`FutureWriter::write`].
pub struct FutureWrite<T: 'static> {
    op: WaitableOperation<FutureWriteOp<T>>,
}

struct FutureWriteOp<T>(marker::PhantomData<T>);

enum WriteComplete<T> {
    Written,
    Dropped(T),
    Cancelled(T),
}

unsafe impl<T> WaitableOp for FutureWriteOp<T>
where
    T: 'static,
{
    type Start = (FutureWriter<T>, T);
    type InProgress = (FutureWriter<T>, Option<Cleanup>);
    type Result = (WriteComplete<T>, FutureWriter<T>);
    type Cancel = FutureWriteCancel<T>;

    fn start((writer, value): Self::Start) -> (u32, Self::InProgress) {
        // TODO: it should be safe to store the lower-destination in
        // `WaitableOperation` using `Pin` memory and such, but that would
        // require some type-level trickery to get a correctly-sized value
        // plumbed all the way to here. For now just dynamically allocate it and
        // leave the optimization of leaving out this dynamic allocation to the
        // future.
        //
        // In lieu of that a dedicated location on the heap is created for the
        // lowering, and then `value`, as an owned value, is lowered into this
        // pointer to initialize it.
        let (ptr, cleanup) = Cleanup::new(writer.vtable.layout);
        // SAFETY: `ptr` is allocated with `vtable.layout` and should be
        // safe to use here.
        let code = unsafe {
            (writer.vtable.lower)(value, ptr);
            (writer.vtable.start_write)(writer.handle, ptr)
        };
        rtdebug!("future.write({}, {ptr:?}) = {code:#x}", writer.handle);
        (code, (writer, cleanup))
    }

    fn start_cancelled((writer, value): Self::Start) -> Self::Cancel {
        FutureWriteCancel::Cancelled(value, writer)
    }

    fn in_progress_update(
        (mut writer, cleanup): Self::InProgress,
        code: u32,
    ) -> Result<Self::Result, Self::InProgress> {
        let ptr = cleanup
            .as_ref()
            .map(|c| c.ptr.as_ptr())
            .unwrap_or(ptr::null_mut());
        match code {
            super::BLOCKED => Err((writer, cleanup)),

            // The other end has dropped its end.
            //
            // The value was not received by the other end so `ptr` still has
            // all of its resources intact. Use `lift` to construct a new
            // instance of `T` which takes ownership of pointers and resources
            // and such. The allocation of `ptr` is then cleaned up naturally
            // when `cleanup` goes out of scope.
            super::DROPPED | super::CANCELLED => {
                // SAFETY: we're the ones managing `ptr` so we know it's safe to
                // pass here.
                let value = unsafe { (writer.vtable.lift)(ptr) };
                let status = if code == super::DROPPED {
                    // This writer has been witnessed to be dropped, meaning that
                    // `writer` is going to get destroyed soon as this return
                    // value propagates up the stack. There's no need to write
                    // the default value, so set this to `false`.
                    writer.should_write_default_value = false;
                    WriteComplete::Dropped(value)
                } else {
                    WriteComplete::Cancelled(value)
                };
                Ok((status, writer))
            }

            // This write has completed.
            //
            // Here we need to clean up our allocations. The `ptr` exclusively
            // owns all of the value being sent and we notably need to cleanup
            // the transitive list allocations present in this pointer. Use
            // `dealloc_lists` for that (effectively a post-return lookalike).
            //
            // Afterwards the `cleanup` itself is naturally dropped and cleaned
            // up.
            super::COMPLETED => {
                // A value was written, so no need to write the default value.
                writer.should_write_default_value = false;

                // SAFETY: we're the ones managing `ptr` so we know it's safe to
                // pass here.
                unsafe {
                    (writer.vtable.dealloc_lists)(ptr);
                }
                Ok((WriteComplete::Written, writer))
            }

            other => unreachable!("unexpected code {other:?}"),
        }
    }

    fn in_progress_waitable((writer, _): &Self::InProgress) -> u32 {
        writer.handle
    }

    fn in_progress_cancel((writer, _): &Self::InProgress) -> u32 {
        // SAFETY: we're managing `writer` and all the various operational bits,
        // so this relies on `WaitableOperation` being safe.
        let code = unsafe { (writer.vtable.cancel_write)(writer.handle) };
        rtdebug!("future.cancel-write({}) = {code:#x}", writer.handle);
        code
    }

    fn result_into_cancel((result, writer): Self::Result) -> Self::Cancel {
        match result {
            // The value was actually sent, meaning we can't yield back the
            // future nor the value.
            WriteComplete::Written => FutureWriteCancel::AlreadySent,

            // The value was not sent because the other end either hung up or we
            // successfully cancelled. In both cases return back the value here
            // with the writer.
            WriteComplete::Dropped(val) => FutureWriteCancel::Dropped(val),
            WriteComplete::Cancelled(val) => FutureWriteCancel::Cancelled(val, writer),
        }
    }
}

impl<T: 'static> Future for FutureWrite<T> {
    type Output = Result<(), FutureWriteError<T>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.pin_project()
            .poll_complete(cx)
            .map(|(result, _writer)| match result {
                WriteComplete::Written => Ok(()),
                WriteComplete::Dropped(value) | WriteComplete::Cancelled(value) => {
                    Err(FutureWriteError { value })
                }
            })
    }
}

impl<T: 'static> FutureWrite<T> {
    fn pin_project(self: Pin<&mut Self>) -> Pin<&mut WaitableOperation<FutureWriteOp<T>>> {
        // SAFETY: we've chosen that when `Self` is pinned that it translates to
        // always pinning the inner field, so that's codified here.
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().op) }
    }

    /// Cancel this write if it hasn't already completed.
    ///
    /// This method can be used to cancel a write-in-progress and re-acquire
    /// the writer and the value being sent. Note that the write operation may
    /// succeed racily or the other end may also drop racily, and these
    /// outcomes are reflected in the returned value here.
    ///
    /// # Panics
    ///
    /// Panics if the operation has already been completed via `Future::poll`,
    /// or if this method is called twice.
    pub fn cancel(self: Pin<&mut Self>) -> FutureWriteCancel<T> {
        self.pin_project().cancel()
    }
}

/// Error type in the result of [`FutureWrite`], or the error type that is a result of
/// a failure to write a future.
pub struct FutureWriteError<T> {
    /// The value that could not be sent.
    pub value: T,
}

impl<T> fmt::Debug for FutureWriteError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FutureWriteError").finish_non_exhaustive()
    }
}

impl<T> fmt::Display for FutureWriteError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "read end dropped".fmt(f)
    }
}

impl<T> std::error::Error for FutureWriteError<T> {}

/// Result of [`FutureWrite::cancel`].
#[derive(Debug)]
pub enum FutureWriteCancel<T: 'static> {
    /// The cancel request raced with the receipt of the sent value, and the
    /// value was actually sent. Neither the value nor the writer are made
    /// available here as both are gone.
    AlreadySent,

    /// The other end was dropped before cancellation happened.
    ///
    /// In this case the original value is returned back to the caller but the
    /// writer itself is not longer accessible as it's no longer usable.
    Dropped(T),

    /// The pending write was successfully cancelled and the value being written
    /// is returned along with the writer to resume again in the future if
    /// necessary.
    Cancelled(T, FutureWriter<T>),
}

/// Represents the readable end of a Component Model `future<T>`.
pub struct FutureReader<T: 'static> {
    handle: AtomicU32,
    vtable: &'static FutureVtable<T>,
}

impl<T> fmt::Debug for FutureReader<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FutureReader")
            .field("handle", &self.handle)
            .finish()
    }
}

impl<T> FutureReader<T> {
    #[doc(hidden)]
    pub fn new(handle: u32, vtable: &'static FutureVtable<T>) -> Self {
        Self {
            handle: AtomicU32::new(handle),
            vtable,
        }
    }

    #[doc(hidden)]
    pub fn take_handle(&self) -> u32 {
        let ret = self.opt_handle().unwrap();
        self.handle.store(u32::MAX, Relaxed);
        ret
    }

    fn handle(&self) -> u32 {
        self.opt_handle().unwrap()
    }

    fn opt_handle(&self) -> Option<u32> {
        match self.handle.load(Relaxed) {
            u32::MAX => None,
            other => Some(other),
        }
    }
}

impl<T> IntoFuture for FutureReader<T> {
    type Output = T;
    type IntoFuture = FutureRead<T>;

    /// Convert this object into a `Future` which will resolve when a value is
    /// written to the writable end of this `future`.
    fn into_future(self) -> Self::IntoFuture {
        FutureRead {
            op: WaitableOperation::new(self),
        }
    }
}

impl<T> Drop for FutureReader<T> {
    fn drop(&mut self) {
        let Some(handle) = self.opt_handle() else {
            return;
        };
        unsafe {
            rtdebug!("future.drop-readable({handle})");
            (self.vtable.drop_readable)(handle);
        }
    }
}

/// Represents a read operation which may be cancelled prior to completion.
///
/// This represents a read operation on a [`FutureReader`] and is created via
/// `IntoFuture`.
pub struct FutureRead<T: 'static> {
    op: WaitableOperation<FutureReadOp<T>>,
}

struct FutureReadOp<T>(marker::PhantomData<T>);

enum ReadComplete<T> {
    Value(T),
    Cancelled,
}

unsafe impl<T> WaitableOp for FutureReadOp<T>
where
    T: 'static,
{
    type Start = FutureReader<T>;
    type InProgress = (FutureReader<T>, Option<Cleanup>);
    type Result = (ReadComplete<T>, FutureReader<T>);
    type Cancel = Result<T, FutureReader<T>>;

    fn start(reader: Self::Start) -> (u32, Self::InProgress) {
        let (ptr, cleanup) = Cleanup::new(reader.vtable.layout);
        // SAFETY: `ptr` is allocated with `vtable.layout` and should be
        // safe to use here. Its lifetime for the async operation is hinged on
        // `WaitableOperation` being safe.
        let code = unsafe { (reader.vtable.start_read)(reader.handle(), ptr) };
        rtdebug!("future.read({}, {ptr:?}) = {code:#x}", reader.handle());
        (code, (reader, cleanup))
    }

    fn start_cancelled(state: Self::Start) -> Self::Cancel {
        Err(state)
    }

    fn in_progress_update(
        (reader, cleanup): Self::InProgress,
        code: u32,
    ) -> Result<Self::Result, Self::InProgress> {
        match ReturnCode::decode(code) {
            ReturnCode::Blocked => Err((reader, cleanup)),

            // Let `cleanup` fall out of scope to clean up its allocation here,
            // and otherwise tahe reader is plumbed through to possibly restart
            // the read in the future.
            ReturnCode::Cancelled(0) => Ok((ReadComplete::Cancelled, reader)),

            // The read has completed, so lift the value from the stored memory and
            // `cleanup` naturally falls out of scope after transferring ownership of
            // everything to the returned `value`.
            ReturnCode::Completed(0) => {
                let ptr = cleanup
                    .as_ref()
                    .map(|c| c.ptr.as_ptr())
                    .unwrap_or(ptr::null_mut());

                // SAFETY: we're the ones managing `ptr` so we know it's safe to
                // pass here.
                let value = unsafe { (reader.vtable.lift)(ptr) };
                Ok((ReadComplete::Value(value), reader))
            }

            other => panic!("unexpected code {other:?}"),
        }
    }

    fn in_progress_waitable((reader, _): &Self::InProgress) -> u32 {
        reader.handle()
    }

    fn in_progress_cancel((reader, _): &Self::InProgress) -> u32 {
        // SAFETY: we're managing `reader` and all the various operational bits,
        // so this relies on `WaitableOperation` being safe.
        let code = unsafe { (reader.vtable.cancel_read)(reader.handle()) };
        rtdebug!("future.cancel-read({}) = {code:#x}", reader.handle());
        code
    }

    fn result_into_cancel((value, reader): Self::Result) -> Self::Cancel {
        match value {
            // The value was actually read, so thread that through here.
            ReadComplete::Value(value) => Ok(value),

            // The read was successfully cancelled, so thread through the
            // `reader` to possibly restart later on.
            ReadComplete::Cancelled => Err(reader),
        }
    }
}

impl<T: 'static> Future for FutureRead<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.pin_project()
            .poll_complete(cx)
            .map(|(result, _reader)| match result {
                ReadComplete::Value(val) => val,
                // This is only possible if, after calling `FutureRead::cancel`,
                // the future is polled again. The `cancel` method is documented
                // as "don't do that" so this is left to panic.
                ReadComplete::Cancelled => panic!("cannot poll after cancelling"),
            })
    }
}

impl<T> FutureRead<T> {
    fn pin_project(self: Pin<&mut Self>) -> Pin<&mut WaitableOperation<FutureReadOp<T>>> {
        // SAFETY: we've chosen that when `Self` is pinned that it translates to
        // always pinning the inner field, so that's codified here.
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().op) }
    }

    /// Cancel this read if it hasn't already completed.
    ///
    /// Return values include:
    ///
    /// * `Ok(value)` - future completed before this cancellation request
    ///   was received.
    /// * `Err(reader)` - read operation was cancelled and it can be retried in
    ///   the future if desired.
    ///
    /// # Panics
    ///
    /// Panics if the operation has already been completed via `Future::poll`,
    /// or if this method is called twice. Additionally if this method completes
    /// then calling `poll` again on `self` will panic.
    pub fn cancel(self: Pin<&mut Self>) -> Result<T, FutureReader<T>> {
        self.pin_project().cancel()
    }
}
