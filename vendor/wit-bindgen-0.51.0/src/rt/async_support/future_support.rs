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

use crate::rt::Cleanup;
use crate::rt::async_support::ReturnCode;
use crate::rt::async_support::waitable::{WaitableOp, WaitableOperation};
use std::alloc::Layout;
use std::fmt;
use std::future::{Future, IntoFuture};
use std::marker;
use std::mem::{self, ManuallyDrop};
use std::pin::Pin;
use std::ptr;
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

/// Helper trait which encapsulates the various operations which can happen
/// with a future.
pub trait FutureOps {
    /// The Rust type that's sent or received on this future.
    type Payload;

    /// The `future.new` intrinsic.
    fn new(&mut self) -> u64;
    /// The canonical ABI layout of the type that this future is
    /// sending/receiving.
    fn elem_layout(&mut self) -> Layout;
    /// Converts a Rust type to its canonical ABI representation.
    unsafe fn lower(&mut self, payload: Self::Payload, dst: *mut u8);
    /// Used to deallocate any Rust-owned lists in the canonical ABI
    /// representation for when a value is successfully sent but needs to be
    /// cleaned up.
    unsafe fn dealloc_lists(&mut self, dst: *mut u8);
    /// Converts from the canonical ABI representation to a Rust value.
    unsafe fn lift(&mut self, dst: *mut u8) -> Self::Payload;
    /// The `future.write` intrinsic
    unsafe fn start_write(&mut self, future: u32, val: *const u8) -> u32;
    /// The `future.read` intrinsic
    unsafe fn start_read(&mut self, future: u32, val: *mut u8) -> u32;
    /// The `future.cancel-read` intrinsic
    unsafe fn cancel_read(&mut self, future: u32) -> u32;
    /// The `future.cancel-write` intrinsic
    unsafe fn cancel_write(&mut self, future: u32) -> u32;
    /// The `future.drop-readable` intrinsic
    unsafe fn drop_readable(&mut self, future: u32);
    /// The `future.drop-writable` intrinsic
    unsafe fn drop_writable(&mut self, future: u32);
}

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

impl<T> FutureOps for &FutureVtable<T> {
    type Payload = T;

    fn new(&mut self) -> u64 {
        unsafe { (self.new)() }
    }
    fn elem_layout(&mut self) -> Layout {
        self.layout
    }
    unsafe fn lower(&mut self, payload: Self::Payload, dst: *mut u8) {
        unsafe { (self.lower)(payload, dst) }
    }
    unsafe fn dealloc_lists(&mut self, dst: *mut u8) {
        unsafe { (self.dealloc_lists)(dst) }
    }
    unsafe fn lift(&mut self, dst: *mut u8) -> Self::Payload {
        unsafe { (self.lift)(dst) }
    }
    unsafe fn start_write(&mut self, future: u32, val: *const u8) -> u32 {
        unsafe { (self.start_write)(future, val) }
    }
    unsafe fn start_read(&mut self, future: u32, val: *mut u8) -> u32 {
        unsafe { (self.start_read)(future, val) }
    }
    unsafe fn cancel_read(&mut self, future: u32) -> u32 {
        unsafe { (self.cancel_read)(future) }
    }
    unsafe fn cancel_write(&mut self, future: u32) -> u32 {
        unsafe { (self.cancel_write)(future) }
    }
    unsafe fn drop_readable(&mut self, future: u32) {
        unsafe { (self.drop_readable)(future) }
    }
    unsafe fn drop_writable(&mut self, future: u32) {
        unsafe { (self.drop_writable)(future) }
    }
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
    let (tx, rx) = unsafe { raw_future_new(vtable) };
    (unsafe { FutureWriter::new(tx, default) }, rx)
}

/// Helper function to create a new read/write pair for a component model
/// future.
///
/// # Unsafety
///
/// This function is unsafe as it requires the functions within `vtable` to
/// correctly uphold the contracts of the component model.
pub unsafe fn raw_future_new<O>(mut ops: O) -> (RawFutureWriter<O>, RawFutureReader<O>)
where
    O: FutureOps + Clone,
{
    unsafe {
        let handles = ops.new();
        let reader = handles as u32;
        let writer = (handles >> 32) as u32;
        rtdebug!("future.new() = [{writer}, {reader}]");
        (
            RawFutureWriter::new(writer, ops.clone()),
            RawFutureReader::new(reader, ops),
        )
    }
}

/// Represents the writable end of a Component Model `future`.
///
/// A [`FutureWriter`] can be used to send a single value of `T` to the other
/// end of a `future`. In a sense this is similar to a oneshot channel in Rust.
pub struct FutureWriter<T: 'static> {
    raw: ManuallyDrop<RawFutureWriter<&'static FutureVtable<T>>>,

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
    unsafe fn new(raw: RawFutureWriter<&'static FutureVtable<T>>, default: fn() -> T) -> Self {
        Self {
            raw: ManuallyDrop::new(raw),
            default,
            should_write_default_value: true,
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
    pub fn write(mut self, value: T) -> FutureWrite<T> {
        let raw = unsafe { ManuallyDrop::take(&mut self.raw).write(value) };
        let default = self.default;
        mem::forget(self);
        FutureWrite { raw, default }
    }
}

impl<T> fmt::Debug for FutureWriter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FutureWriter")
            .field("handle", &self.raw.handle)
            .finish()
    }
}

impl<T> Drop for FutureWriter<T> {
    fn drop(&mut self) {
        // If a value has not yet been written into this writer than that must
        // be done so now. Take the `raw` writer and perform the write via a
        // waker that drives the future.
        //
        // If `should_write_default_value` is `false` then a write has already
        // happened and we can go ahead and just synchronously drop this writer
        // as we would any other handle.
        if self.should_write_default_value {
            let raw = unsafe { ManuallyDrop::take(&mut self.raw) };
            let value = (self.default)();
            raw.write_and_forget(value);
        } else {
            unsafe { ManuallyDrop::drop(&mut self.raw) }
        }
    }
}

/// Represents a write operation which may be cancelled prior to completion.
///
/// This is returned by [`FutureWriter::write`].
pub struct FutureWrite<T: 'static> {
    raw: RawFutureWrite<&'static FutureVtable<T>>,
    default: fn() -> T,
}

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

impl<T: 'static> Future for FutureWrite<T> {
    type Output = Result<(), FutureWriteError<T>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.pin_project().poll(cx)
    }
}

impl<T: 'static> FutureWrite<T> {
    fn pin_project(self: Pin<&mut Self>) -> Pin<&mut RawFutureWrite<&'static FutureVtable<T>>> {
        // SAFETY: we've chosen that when `Self` is pinned that it translates to
        // always pinning the inner field, so that's codified here.
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().raw) }
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
        let default = self.default;
        match self.pin_project().cancel() {
            RawFutureWriteCancel::AlreadySent => FutureWriteCancel::AlreadySent,
            RawFutureWriteCancel::Dropped(val) => FutureWriteCancel::Dropped(val),
            RawFutureWriteCancel::Cancelled(val, raw) => FutureWriteCancel::Cancelled(
                val,
                FutureWriter {
                    raw: ManuallyDrop::new(raw),
                    default,
                    should_write_default_value: true,
                },
            ),
        }
    }
}

impl<T: 'static> Drop for FutureWrite<T> {
    fn drop(&mut self) {
        if self.raw.op.is_done() {
            return;
        }

        // Although the underlying `WaitableOperation` will already
        // auto-cancel-on-drop we need to specially handle that here because if
        // the cancellation goes through then it means that no value will have
        // been written to this future which will cause a trap. By using
        // `Self::cancel` it's ensured that if cancellation succeeds a
        // `FutureWriter` is created. In `Drop for FutureWriter` that'll handle
        // the last-ditch write-default logic.
        //
        // SAFETY: we're in the destructor here so the value `self` is about
        // to go away and we can guarantee we're not moving out of it.
        let pin = unsafe { Pin::new_unchecked(self) };
        pin.cancel();
    }
}

/// Raw version of [`FutureWriter`].
pub struct RawFutureWriter<O: FutureOps> {
    handle: u32,
    ops: O,
}

impl<O: FutureOps> RawFutureWriter<O> {
    unsafe fn new(handle: u32, ops: O) -> Self {
        Self { handle, ops }
    }

    /// Same as [`FutureWriter::write`], but the raw version.
    pub fn write(self, value: O::Payload) -> RawFutureWrite<O> {
        RawFutureWrite {
            op: WaitableOperation::new(FutureWriteOp(marker::PhantomData), (self, value)),
        }
    }

    /// Writes `value` in the background.
    ///
    /// This does not block and is not cancellable.
    pub fn write_and_forget(self, value: O::Payload)
    where
        O: 'static,
    {
        return Arc::new(DeferredWrite {
            write: Mutex::new(self.write(value)),
        })
        .wake();

        /// Helper structure which behaves both as a future of sorts and an
        /// executor of sorts.
        ///
        /// This type is constructed in `Drop for FutureWriter<T>` to send out a
        /// default value when no other has been written. This manages the
        /// `FutureWrite` operation happening internally through a `Wake`
        /// implementation. That means that this is a sort of cyclical future
        /// which, when woken, will complete the write operation.
        ///
        /// The purpose of this is to be a "lightweight" way of "spawn"-ing a
        /// future write to happen in the background. Crucially, however, this
        /// doesn't require the `async-spawn` feature and instead works with the
        /// `wasip3_task` C ABI structures (which spawn doesn't support).
        struct DeferredWrite<O: FutureOps> {
            write: Mutex<RawFutureWrite<O>>,
        }

        // SAFETY: Needed to satisfy `Waker::from` but otherwise should be ok
        // because wasm doesn't have threads anyway right now.
        unsafe impl<O: FutureOps> Send for DeferredWrite<O> {}
        unsafe impl<O: FutureOps> Sync for DeferredWrite<O> {}

        impl<O: FutureOps + 'static> Wake for DeferredWrite<O> {
            fn wake(self: Arc<Self>) {
                // When a `wake` signal comes in that should happen in two
                // locations:
                //
                // 1. When `DeferredWrite` is initially constructed.
                // 2. When an event comes in indicating that the internal write
                //    has completed.
                //
                // The implementation here is the same in both cases. A clone of
                // `self` is converted to a `Waker`, and this `Waker` notably
                // owns the internal future itself. The internal write operation
                // is then pushed forward (e.g. it's issued in (1) or checked up
                // on in (2)).
                //
                // If `Pending` is returned then `waker` should have been stored
                // away within the `wasip3_task` C ABI structure. Otherwise it
                // should not have been stored away and `self` should be the
                // sole reference which means everything will get cleaned up
                // when this function returns.
                let poll = {
                    let waker = Waker::from(self.clone());
                    let mut cx = Context::from_waker(&waker);
                    let mut write = self.write.lock().unwrap();
                    unsafe { Pin::new_unchecked(&mut *write).poll(&mut cx) }
                };
                if poll.is_ready() {
                    assert_eq!(Arc::strong_count(&self), 1);
                } else {
                    assert!(Arc::strong_count(&self) > 1);
                }
                assert_eq!(Arc::weak_count(&self), 0);
            }
        }
    }
}

impl<O: FutureOps> fmt::Debug for RawFutureWriter<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawFutureWriter")
            .field("handle", &self.handle)
            .finish()
    }
}

impl<O: FutureOps> Drop for RawFutureWriter<O> {
    fn drop(&mut self) {
        unsafe {
            rtdebug!("future.drop-writable({})", self.handle);
            self.ops.drop_writable(self.handle);
        }
    }
}

/// Represents a write operation which may be cancelled prior to completion.
///
/// This is returned by [`FutureWriter::write`].
pub struct RawFutureWrite<O: FutureOps> {
    op: WaitableOperation<FutureWriteOp<O>>,
}

struct FutureWriteOp<O>(marker::PhantomData<O>);

enum WriteComplete<T> {
    Written,
    Dropped(T),
    Cancelled(T),
}

unsafe impl<O: FutureOps> WaitableOp for FutureWriteOp<O> {
    type Start = (RawFutureWriter<O>, O::Payload);
    type InProgress = (RawFutureWriter<O>, Option<Cleanup>);
    type Result = (WriteComplete<O::Payload>, RawFutureWriter<O>);
    type Cancel = RawFutureWriteCancel<O>;

    fn start(&mut self, (mut writer, value): Self::Start) -> (u32, Self::InProgress) {
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
        let (ptr, cleanup) = Cleanup::new(writer.ops.elem_layout());
        // SAFETY: `ptr` is allocated with `ops.layout` and should be
        // safe to use here.
        let code = unsafe {
            writer.ops.lower(value, ptr);
            writer.ops.start_write(writer.handle, ptr)
        };
        rtdebug!("future.write({}, {ptr:?}) = {code:#x}", writer.handle);
        (code, (writer, cleanup))
    }

    fn start_cancelled(&mut self, (writer, value): Self::Start) -> Self::Cancel {
        RawFutureWriteCancel::Cancelled(value, writer)
    }

    fn in_progress_update(
        &mut self,
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
                let value = unsafe { writer.ops.lift(ptr) };
                let status = if code == super::DROPPED {
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
                // SAFETY: we're the ones managing `ptr` so we know it's safe to
                // pass here.
                unsafe {
                    writer.ops.dealloc_lists(ptr);
                }
                Ok((WriteComplete::Written, writer))
            }

            other => unreachable!("unexpected code {other:?}"),
        }
    }

    fn in_progress_waitable(&mut self, (writer, _): &Self::InProgress) -> u32 {
        writer.handle
    }

    fn in_progress_cancel(&mut self, (writer, _): &mut Self::InProgress) -> u32 {
        // SAFETY: we're managing `writer` and all the various operational bits,
        // so this relies on `WaitableOperation` being safe.
        let code = unsafe { writer.ops.cancel_write(writer.handle) };
        rtdebug!("future.cancel-write({}) = {code:#x}", writer.handle);
        code
    }

    fn result_into_cancel(&mut self, (result, writer): Self::Result) -> Self::Cancel {
        match result {
            // The value was actually sent, meaning we can't yield back the
            // future nor the value.
            WriteComplete::Written => RawFutureWriteCancel::AlreadySent,

            // The value was not sent because the other end either hung up or we
            // successfully cancelled. In both cases return back the value here
            // with the writer.
            WriteComplete::Dropped(val) => RawFutureWriteCancel::Dropped(val),
            WriteComplete::Cancelled(val) => RawFutureWriteCancel::Cancelled(val, writer),
        }
    }
}

impl<O: FutureOps> Future for RawFutureWrite<O> {
    type Output = Result<(), FutureWriteError<O::Payload>>;

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

impl<O: FutureOps> RawFutureWrite<O> {
    fn pin_project(self: Pin<&mut Self>) -> Pin<&mut WaitableOperation<FutureWriteOp<O>>> {
        // SAFETY: we've chosen that when `Self` is pinned that it translates to
        // always pinning the inner field, so that's codified here.
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().op) }
    }

    /// Same as [`FutureWrite::cancel`], but returns a [`RawFutureWriteCancel`]
    /// instead.
    pub fn cancel(self: Pin<&mut Self>) -> RawFutureWriteCancel<O> {
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
pub enum RawFutureWriteCancel<O: FutureOps> {
    /// The cancel request raced with the receipt of the sent value, and the
    /// value was actually sent. Neither the value nor the writer are made
    /// available here as both are gone.
    AlreadySent,

    /// The other end was dropped before cancellation happened.
    ///
    /// In this case the original value is returned back to the caller but the
    /// writer itself is not longer accessible as it's no longer usable.
    Dropped(O::Payload),

    /// The pending write was successfully cancelled and the value being written
    /// is returned along with the writer to resume again in the future if
    /// necessary.
    Cancelled(O::Payload, RawFutureWriter<O>),
}

/// Represents the readable end of a Component Model `future<T>`.
pub type FutureReader<T> = RawFutureReader<&'static FutureVtable<T>>;

/// Represents the readable end of a Component Model `future<T>`.
pub struct RawFutureReader<O: FutureOps> {
    handle: AtomicU32,
    ops: O,
}

impl<O: FutureOps> fmt::Debug for RawFutureReader<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawFutureReader")
            .field("handle", &self.handle)
            .finish()
    }
}

impl<O: FutureOps> RawFutureReader<O> {
    /// Raw constructor for a future reader.
    ///
    /// Takes ownership of the `handle` provided.
    ///
    /// # Safety
    ///
    /// The `ops` specified must be both valid and well-typed for `handle`.
    pub unsafe fn new(handle: u32, ops: O) -> Self {
        Self {
            handle: AtomicU32::new(handle),
            ops,
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

impl<O: FutureOps> IntoFuture for RawFutureReader<O> {
    type Output = O::Payload;
    type IntoFuture = RawFutureRead<O>;

    /// Convert this object into a `Future` which will resolve when a value is
    /// written to the writable end of this `future`.
    fn into_future(self) -> Self::IntoFuture {
        RawFutureRead {
            op: WaitableOperation::new(FutureReadOp(marker::PhantomData), self),
        }
    }
}

impl<O: FutureOps> Drop for RawFutureReader<O> {
    fn drop(&mut self) {
        let Some(handle) = self.opt_handle() else {
            return;
        };
        unsafe {
            rtdebug!("future.drop-readable({handle})");
            self.ops.drop_readable(handle);
        }
    }
}

/// Represents a read operation which may be cancelled prior to completion.
///
/// This represents a read operation on a [`FutureReader`] and is created via
/// `IntoFuture`.
pub type FutureRead<T> = RawFutureRead<&'static FutureVtable<T>>;

/// Represents a read operation which may be cancelled prior to completion.
///
/// This represents a read operation on a [`FutureReader`] and is created via
/// `IntoFuture`.
pub struct RawFutureRead<O: FutureOps> {
    op: WaitableOperation<FutureReadOp<O>>,
}

struct FutureReadOp<O>(marker::PhantomData<O>);

enum ReadComplete<T> {
    Value(T),
    Cancelled,
}

unsafe impl<O: FutureOps> WaitableOp for FutureReadOp<O> {
    type Start = RawFutureReader<O>;
    type InProgress = (RawFutureReader<O>, Option<Cleanup>);
    type Result = (ReadComplete<O::Payload>, RawFutureReader<O>);
    type Cancel = Result<O::Payload, RawFutureReader<O>>;

    fn start(&mut self, mut reader: Self::Start) -> (u32, Self::InProgress) {
        let (ptr, cleanup) = Cleanup::new(reader.ops.elem_layout());
        // SAFETY: `ptr` is allocated with `vtable.layout` and should be
        // safe to use here. Its lifetime for the async operation is hinged on
        // `WaitableOperation` being safe.
        let code = unsafe { reader.ops.start_read(reader.handle(), ptr) };
        rtdebug!("future.read({}, {ptr:?}) = {code:#x}", reader.handle());
        (code, (reader, cleanup))
    }

    fn start_cancelled(&mut self, state: Self::Start) -> Self::Cancel {
        Err(state)
    }

    fn in_progress_update(
        &mut self,
        (mut reader, cleanup): Self::InProgress,
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
                let value = unsafe { reader.ops.lift(ptr) };
                Ok((ReadComplete::Value(value), reader))
            }

            other => panic!("unexpected code {other:?}"),
        }
    }

    fn in_progress_waitable(&mut self, (reader, _): &Self::InProgress) -> u32 {
        reader.handle()
    }

    fn in_progress_cancel(&mut self, (reader, _): &mut Self::InProgress) -> u32 {
        // SAFETY: we're managing `reader` and all the various operational bits,
        // so this relies on `WaitableOperation` being safe.
        let code = unsafe { reader.ops.cancel_read(reader.handle()) };
        rtdebug!("future.cancel-read({}) = {code:#x}", reader.handle());
        code
    }

    fn result_into_cancel(&mut self, (value, reader): Self::Result) -> Self::Cancel {
        match value {
            // The value was actually read, so thread that through here.
            ReadComplete::Value(value) => Ok(value),

            // The read was successfully cancelled, so thread through the
            // `reader` to possibly restart later on.
            ReadComplete::Cancelled => Err(reader),
        }
    }
}

impl<O: FutureOps> Future for RawFutureRead<O> {
    type Output = O::Payload;

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

impl<O: FutureOps> RawFutureRead<O> {
    fn pin_project(self: Pin<&mut Self>) -> Pin<&mut WaitableOperation<FutureReadOp<O>>> {
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
    pub fn cancel(self: Pin<&mut Self>) -> Result<O::Payload, RawFutureReader<O>> {
        self.pin_project().cancel()
    }
}
