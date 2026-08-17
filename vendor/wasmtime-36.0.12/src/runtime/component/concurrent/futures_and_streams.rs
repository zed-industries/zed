use super::table::{TableDebug, TableId};
use super::{
    Event, GlobalErrorContextRefCount, LocalErrorContextRefCount, StateTable, Waitable,
    WaitableCommon, WaitableState,
};
use crate::component::concurrent::{ConcurrentState, WorkItem};
use crate::component::func::{self, LiftContext, LowerContext, Options};
use crate::component::matching::InstanceType;
use crate::component::values::{ErrorContextAny, FutureAny, StreamAny};
use crate::component::{AsAccessor, Instance, Lower, Val, WasmList, WasmStr};
use crate::store::{StoreOpaque, StoreToken};
use crate::vm::VMStore;
use crate::{AsContextMut, StoreContextMut, ValRaw};
use anyhow::{Context, Result, anyhow, bail};
use buffers::Extender;
use buffers::UntypedWriteBuffer;
use futures::channel::oneshot;
use std::boxed::Box;
use std::fmt;
use std::future;
use std::iter;
use std::marker::PhantomData;
use std::mem::{self, MaybeUninit};
use std::string::{String, ToString};
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};
use std::vec::Vec;
use wasmtime_environ::component::{
    CanonicalAbiInfo, ComponentTypes, InterfaceType, OptionsIndex, RuntimeComponentInstanceIndex,
    TypeComponentGlobalErrorContextTableIndex, TypeComponentLocalErrorContextTableIndex,
    TypeFutureTableIndex, TypeStreamTableIndex,
};

pub use buffers::{ReadBuffer, VecBuffer, WriteBuffer};

mod buffers;

/// Enum for distinguishing between a stream or future in functions that handle
/// both.
#[derive(Copy, Clone, Debug)]
enum TransmitKind {
    Stream,
    Future,
}

/// Represents `{stream,future}.{read,write}` results.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ReturnCode {
    Blocked,
    Completed(u32),
    Dropped(u32),
    Cancelled(u32),
}

impl ReturnCode {
    /// Pack `self` into a single 32-bit integer that may be returned to the
    /// guest.
    ///
    /// This corresponds to `pack_copy_result` in the Component Model spec.
    pub fn encode(&self) -> u32 {
        const BLOCKED: u32 = 0xffff_ffff;
        const COMPLETED: u32 = 0x0;
        const DROPPED: u32 = 0x1;
        const CANCELLED: u32 = 0x2;
        match self {
            ReturnCode::Blocked => BLOCKED,
            ReturnCode::Completed(n) => {
                debug_assert!(*n < (1 << 28));
                (n << 4) | COMPLETED
            }
            ReturnCode::Dropped(n) => {
                debug_assert!(*n < (1 << 28));
                (n << 4) | DROPPED
            }
            ReturnCode::Cancelled(n) => {
                debug_assert!(*n < (1 << 28));
                (n << 4) | CANCELLED
            }
        }
    }

    /// Returns `Self::Completed` with the specified count (or zero if
    /// `matches!(kind, TransmitKind::Future)`)
    fn completed(kind: TransmitKind, count: u32) -> Self {
        Self::Completed(if let TransmitKind::Future = kind {
            0
        } else {
            count
        })
    }
}

/// Represents a stream or future type index.
///
/// This is useful as a parameter type for functions which operate on either a
/// future or a stream.
#[derive(Copy, Clone, Debug)]
pub(super) enum TableIndex {
    Stream(TypeStreamTableIndex),
    Future(TypeFutureTableIndex),
}

impl TableIndex {
    fn kind(&self) -> TransmitKind {
        match self {
            TableIndex::Stream(_) => TransmitKind::Stream,
            TableIndex::Future(_) => TransmitKind::Future,
        }
    }
}

/// Action to take after writing
enum PostWrite {
    /// Continue performing writes
    Continue,
    /// Drop the channel post-write
    Drop,
}

/// Represents the result of a host-initiated stream or future read or write.
struct HostResult<B> {
    /// The buffer provided when reading or writing.
    buffer: B,
    /// Whether the other end of the stream or future has been dropped.
    dropped: bool,
}

/// Retrieve the payload type of the specified stream or future, or `None` if it
/// has no payload type.
fn payload(ty: TableIndex, types: &Arc<ComponentTypes>) -> Option<InterfaceType> {
    match ty {
        TableIndex::Future(ty) => types[types[ty].ty].payload,
        TableIndex::Stream(ty) => types[types[ty].ty].payload,
    }
}

/// Retrieve the host rep and state for the specified guest-visible waitable
/// handle.
fn get_mut_by_index_from(
    state_table: &mut StateTable<WaitableState>,
    ty: TableIndex,
    index: u32,
) -> Result<(u32, &mut StreamFutureState)> {
    Ok(match ty {
        TableIndex::Stream(ty) => {
            let (rep, WaitableState::Stream(actual_ty, state)) =
                state_table.get_mut_by_index(index)?
            else {
                bail!("invalid stream handle");
            };
            if *actual_ty != ty {
                bail!("invalid stream handle");
            }
            (rep, state)
        }
        TableIndex::Future(ty) => {
            let (rep, WaitableState::Future(actual_ty, state)) =
                state_table.get_mut_by_index(index)?
            else {
                bail!("invalid future handle");
            };
            if *actual_ty != ty {
                bail!("invalid future handle");
            }
            (rep, state)
        }
    })
}

/// Construct a `WaitableState` using the specified type and state.
fn waitable_state(ty: TableIndex, state: StreamFutureState) -> WaitableState {
    match ty {
        TableIndex::Stream(ty) => WaitableState::Stream(ty, state),
        TableIndex::Future(ty) => WaitableState::Future(ty, state),
    }
}

/// Complete a write initiated by a host-owned future or stream by matching it
/// with the specified `Reader`.
fn accept_reader<T: func::Lower + Send + 'static, B: WriteBuffer<T>, U: 'static>(
    mut store: StoreContextMut<U>,
    instance: Instance,
    reader: Reader,
    mut buffer: B,
    kind: TransmitKind,
) -> Result<(HostResult<B>, ReturnCode)> {
    Ok(match reader {
        Reader::Guest {
            options,
            ty,
            address,
            count,
        } => {
            let types = instance.id().get(store.0).component().types().clone();
            let count = buffer.remaining().len().min(count);

            let lower = &mut if T::MAY_REQUIRE_REALLOC {
                LowerContext::new
            } else {
                LowerContext::new_without_realloc
            }(store.as_context_mut(), options, &types, instance);

            if address % usize::try_from(T::ALIGN32)? != 0 {
                bail!("read pointer not aligned");
            }
            lower
                .as_slice_mut()
                .get_mut(address..)
                .and_then(|b| b.get_mut(..T::SIZE32 * count))
                .ok_or_else(|| anyhow::anyhow!("read pointer out of bounds of memory"))?;

            if let Some(ty) = payload(ty, &types) {
                T::linear_store_list_to_memory(lower, ty, address, &buffer.remaining()[..count])?;
            }

            buffer.skip(count);
            (
                HostResult {
                    buffer,
                    dropped: false,
                },
                ReturnCode::completed(kind, count.try_into().unwrap()),
            )
        }
        Reader::Host { accept } => {
            let count = buffer.remaining().len();
            let mut untyped = UntypedWriteBuffer::new(&mut buffer);
            let count = accept(&mut untyped, count);
            (
                HostResult {
                    buffer,
                    dropped: false,
                },
                ReturnCode::completed(kind, count.try_into().unwrap()),
            )
        }
        Reader::End => (
            HostResult {
                buffer,
                dropped: true,
            },
            ReturnCode::Dropped(0),
        ),
    })
}

/// Complete a read initiated by a host-owned future or stream by matching it with the
/// specified `Writer`.
fn accept_writer<T: func::Lift + Send + 'static, B: ReadBuffer<T>, U>(
    writer: Writer,
    mut buffer: B,
    kind: TransmitKind,
) -> Result<(HostResult<B>, ReturnCode)> {
    Ok(match writer {
        Writer::Guest {
            lift,
            ty,
            address,
            count,
        } => {
            let count = count.min(buffer.remaining_capacity());
            if T::IS_RUST_UNIT_TYPE {
                // SAFETY: `T::IS_RUST_UNIT_TYPE` is only true for `()`, a
                // zero-sized type, so `MaybeUninit::uninit().assume_init()`
                // is a valid way to populate the zero-sized buffer.
                buffer.extend(
                    iter::repeat_with(|| unsafe { MaybeUninit::uninit().assume_init() })
                        .take(count),
                )
            } else {
                let ty = ty.unwrap();
                if address % usize::try_from(T::ALIGN32)? != 0 {
                    bail!("write pointer not aligned");
                }
                lift.memory()
                    .get(address..)
                    .and_then(|b| b.get(..T::SIZE32 * count))
                    .ok_or_else(|| anyhow::anyhow!("write pointer out of bounds of memory"))?;

                let list = &WasmList::new(address, count, lift, ty)?;
                T::linear_lift_into_from_memory(lift, list, &mut Extender(&mut buffer))?
            }
            (
                HostResult {
                    buffer,
                    dropped: false,
                },
                ReturnCode::completed(kind, count.try_into().unwrap()),
            )
        }
        Writer::Host {
            buffer: input,
            count,
        } => {
            let count = count.min(buffer.remaining_capacity());
            buffer.move_from(input.get_mut::<T>(), count);
            (
                HostResult {
                    buffer,
                    dropped: false,
                },
                ReturnCode::completed(kind, count.try_into().unwrap()),
            )
        }
        Writer::End => (
            HostResult {
                buffer,
                dropped: true,
            },
            ReturnCode::Dropped(0),
        ),
    })
}

/// Return a `Future` which will resolve once the reader end corresponding to
/// the specified writer end of a future or stream is dropped.
async fn watch_reader(accessor: impl AsAccessor, instance: Instance, id: TableId<TransmitHandle>) {
    future::poll_fn(|cx| {
        accessor
            .as_accessor()
            .with(|mut access| {
                let concurrent_state = instance.concurrent_state_mut(access.as_context_mut().0);
                let state_id = concurrent_state.get(id)?.state;
                let state = concurrent_state.get_mut(state_id)?;
                anyhow::Ok(if matches!(&state.read, ReadState::Dropped) {
                    Poll::Ready(())
                } else {
                    state.reader_watcher = Some(cx.waker().clone());
                    Poll::Pending
                })
            })
            .unwrap_or(Poll::Ready(()))
    })
    .await
}

/// Return a `Future` which will resolve once the writer end corresponding to
/// the specified reader end of a future or stream is dropped.
async fn watch_writer(accessor: impl AsAccessor, instance: Instance, id: TableId<TransmitHandle>) {
    future::poll_fn(|cx| {
        accessor
            .as_accessor()
            .with(|mut access| {
                let concurrent_state = instance.concurrent_state_mut(access.as_context_mut().0);
                let state_id = concurrent_state.get(id)?.state;
                let state = concurrent_state.get_mut(state_id)?;
                anyhow::Ok(
                    if matches!(
                        &state.write,
                        WriteState::Dropped
                            | WriteState::GuestReady {
                                post_write: PostWrite::Drop,
                                ..
                            }
                            | WriteState::HostReady {
                                post_write: PostWrite::Drop,
                                ..
                            }
                    ) {
                        Poll::Ready(())
                    } else {
                        state.writer_watcher = Some(cx.waker().clone());
                        Poll::Pending
                    },
                )
            })
            .unwrap_or(Poll::Ready(()))
    })
    .await
}

/// Represents the state of a stream or future handle from the perspective of a
/// given component instance.
#[derive(Debug, Eq, PartialEq)]
pub(super) enum StreamFutureState {
    /// The write end of the stream or future.
    Write {
        /// Whether the component instance has been notified that the stream or
        /// future is "done" (i.e. the other end has dropped, or, in the case of
        /// a future, a value has been transmitted).
        done: bool,
    },
    /// The read end of the stream or future.
    Read {
        /// Whether the component instance has been notified that the stream or
        /// future is "done" (i.e. the other end has dropped, or, in the case of
        /// a future, a value has been transmitted).
        done: bool,
    },
    /// A read or write is in progress.
    Busy,
}

/// Represents the state associated with an error context
#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub(super) struct ErrorContextState {
    /// Debug message associated with the error context
    pub(crate) debug_msg: String,
}

/// Represents the size and alignment for a "flat" Component Model type,
/// i.e. one containing no pointers or handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FlatAbi {
    pub(super) size: u32,
    pub(super) align: u32,
}

/// Represents the writable end of a Component Model `future`.
///
/// Note that `FutureWriter` instances must be disposed of using either `write`
/// or `close`; otherwise the in-store representation will leak and the reader
/// end will hang indefinitely.  Consider using [`GuardedFutureWriter`] to
/// ensure that disposal happens automatically.
pub struct FutureWriter<T> {
    default: fn() -> T,
    id: TableId<TransmitHandle>,
    instance: Instance,
}

impl<T> FutureWriter<T> {
    fn new(default: fn() -> T, id: TableId<TransmitHandle>, instance: Instance) -> Self {
        Self {
            default,
            id,
            instance,
        }
    }

    /// Write the specified value to this `future`.
    ///
    /// The returned `Future` will yield `true` if the read end accepted the
    /// value; otherwise it will return `false`, meaning the read end was dropped
    /// before the value could be delivered.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future.
    pub async fn write(self, accessor: impl AsAccessor, value: T) -> bool
    where
        T: func::Lower + Send + Sync + 'static,
    {
        self.guard(accessor).write(value).await
    }

    /// Mut-ref signature instead of by-value signature for
    /// `GuardedFutureWriter` to more easily call.
    async fn write_(&mut self, accessor: impl AsAccessor, value: T) -> bool
    where
        T: func::Lower + Send + Sync + 'static,
    {
        let accessor = accessor.as_accessor();

        let result = self
            .instance
            .host_write_async(accessor, self.id, Some(value), TransmitKind::Future)
            .await;

        match result {
            Ok(HostResult { dropped, .. }) => !dropped,
            Err(_) => todo!("guarantee buffer recovery if `host_write` fails"),
        }
    }

    /// Wait for the read end of this `future` is dropped.
    ///
    /// The [`Accessor`] provided can be acquired from [`Instance::run_concurrent`] or
    /// from within a host function for example.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future.
    pub async fn watch_reader(&mut self, accessor: impl AsAccessor) {
        watch_reader(accessor, self.instance, self.id).await
    }

    /// Close this `FutureWriter`, writing the default value.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future. Usage of this future after calling `close` will also cause
    /// a panic.
    pub fn close(&mut self, mut store: impl AsContextMut)
    where
        T: func::Lower + Send + Sync + 'static,
    {
        let id = mem::replace(&mut self.id, TableId::new(0));
        let default = self.default;
        self.instance
            .host_drop_writer(store.as_context_mut(), id, Some(&move || Ok(default())))
            .unwrap();
    }

    /// Convenience method around [`Self::close`].
    pub fn close_with(&mut self, accessor: impl AsAccessor)
    where
        T: func::Lower + Send + Sync + 'static,
    {
        accessor.as_accessor().with(|access| self.close(access))
    }

    /// Returns a [`GuardedFutureWriter`] which will auto-close this future on
    /// drop and clean it up from the store.
    ///
    /// Note that the `accessor` provided must own this future and is
    /// additionally transferred to the `GuardedFutureWriter` return value.
    pub fn guard<A>(self, accessor: A) -> GuardedFutureWriter<T, A>
    where
        T: func::Lower + Send + Sync + 'static,
        A: AsAccessor,
    {
        GuardedFutureWriter::new(accessor, self)
    }
}

/// A [`FutureWriter`] paired with an [`Accessor`].
///
/// This is an RAII wrapper around [`FutureWriter`] that ensures it is closed
/// when dropped. This can be created through [`GuardedFutureWriter::new`] or
/// [`FutureWriter::guard`].
pub struct GuardedFutureWriter<T, A>
where
    T: func::Lower + Send + Sync + 'static,
    A: AsAccessor,
{
    // This field is `None` to implement the conversion from this guard back to
    // `FutureWriter`. When `None` is seen in the destructor it will cause the
    // destructor to do nothing.
    writer: Option<FutureWriter<T>>,
    accessor: A,
}

impl<T, A> GuardedFutureWriter<T, A>
where
    T: func::Lower + Send + Sync + 'static,
    A: AsAccessor,
{
    /// Create a new `GuardedFutureWriter` with the specified `accessor` and
    /// `writer`.
    pub fn new(accessor: A, writer: FutureWriter<T>) -> Self {
        Self {
            writer: Some(writer),
            accessor,
        }
    }

    /// Wrapper for [`FutureWriter::write`].
    pub async fn write(mut self, value: T) -> bool
    where
        T: func::Lower + Send + Sync + 'static,
    {
        self.writer
            .as_mut()
            .unwrap()
            .write_(&self.accessor, value)
            .await
    }

    /// Wrapper for [`FutureWriter::watch_reader`]
    pub async fn watch_reader(&mut self) {
        self.writer
            .as_mut()
            .unwrap()
            .watch_reader(&self.accessor)
            .await
    }

    /// Extracts the underlying [`FutureWriter`] from this guard, returning it
    /// back.
    pub fn into_future(self) -> FutureWriter<T> {
        self.into()
    }
}

impl<T, A> From<GuardedFutureWriter<T, A>> for FutureWriter<T>
where
    T: func::Lower + Send + Sync + 'static,
    A: AsAccessor,
{
    fn from(mut guard: GuardedFutureWriter<T, A>) -> Self {
        guard.writer.take().unwrap()
    }
}

impl<T, A> Drop for GuardedFutureWriter<T, A>
where
    T: func::Lower + Send + Sync + 'static,
    A: AsAccessor,
{
    fn drop(&mut self) {
        if let Some(writer) = &mut self.writer {
            writer.close_with(&self.accessor)
        }
    }
}

/// Represents the readable end of a Component Model `future`.
///
/// Note that `FutureReader` instances must be disposed of using either `read`
/// or `close`; otherwise the in-store representation will leak and the writer
/// end will hang indefinitely.  Consider using [`GuardedFutureReader`] to
/// ensure that disposal happens automatically.
pub struct FutureReader<T> {
    instance: Instance,
    id: TableId<TransmitHandle>,
    _phantom: PhantomData<T>,
}

impl<T> FutureReader<T> {
    fn new(id: TableId<TransmitHandle>, instance: Instance) -> Self {
        Self {
            instance,
            id,
            _phantom: PhantomData,
        }
    }

    /// Read the value from this `future`.
    ///
    /// The returned `Future` will yield `Err` if the guest has trapped
    /// before it could produce a result.
    ///
    /// The [`Accessor`] provided can be acquired from [`Instance::run_concurrent`] or
    /// from within a host function for example.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future.
    pub async fn read(self, accessor: impl AsAccessor) -> Option<T>
    where
        T: func::Lift + Send + 'static,
    {
        self.guard(accessor).read().await
    }

    async fn read_(&mut self, accessor: impl AsAccessor) -> Option<T>
    where
        T: func::Lift + Send + 'static,
    {
        let accessor = accessor.as_accessor();

        let result = self
            .instance
            .host_read_async(accessor, self.id, None, TransmitKind::Future)
            .await;

        if let Ok(HostResult {
            mut buffer,
            dropped: false,
        }) = result
        {
            buffer.take()
        } else {
            None
        }
    }

    /// Wait for the write end of this `future` to be dropped.
    ///
    /// The [`Accessor`] provided can be acquired from
    /// [`Instance::run_concurrent`] or from within a host function for example.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future.
    pub async fn watch_writer(&mut self, accessor: impl AsAccessor) {
        watch_writer(accessor, self.instance, self.id).await;
    }

    /// Convert this `FutureReader` into a [`Val`].
    // See TODO comment for `FutureAny`; this is prone to handle leakage.
    pub fn into_val(self) -> Val {
        Val::Future(FutureAny(self.id.rep()))
    }

    /// Attempt to convert the specified [`Val`] to a `FutureReader`.
    pub fn from_val(
        mut store: impl AsContextMut<Data: Send>,
        instance: Instance,
        value: &Val,
    ) -> Result<Self> {
        let Val::Future(FutureAny(rep)) = value else {
            bail!("expected `future`; got `{}`", value.desc());
        };
        let store = store.as_context_mut();
        let id = TableId::<TransmitHandle>::new(*rep);
        instance.concurrent_state_mut(store.0).get(id)?; // Just make sure it's present
        Ok(Self::new(id, instance))
    }

    /// Transfer ownership of the read end of a future from a guest to the host.
    fn lift_from_index(cx: &mut LiftContext<'_>, ty: InterfaceType, index: u32) -> Result<Self> {
        match ty {
            InterfaceType::Future(src) => {
                let state_table = cx
                    .instance_mut()
                    .concurrent_state_mut()
                    .state_table(TableIndex::Future(src));
                let (rep, state) =
                    get_mut_by_index_from(state_table, TableIndex::Future(src), index)?;

                match state {
                    StreamFutureState::Read { .. } => {
                        state_table.remove_by_index(index)?;
                    }
                    StreamFutureState::Write { .. } => bail!("cannot transfer write end of future"),
                    StreamFutureState::Busy => bail!("cannot transfer busy future"),
                }

                let id = TableId::<TransmitHandle>::new(rep);
                let concurrent_state = cx.instance_mut().concurrent_state_mut();
                let state = concurrent_state.get(id)?.state;

                if concurrent_state.get(state)?.done {
                    bail!("cannot lift future after previous read succeeded");
                }

                Ok(Self::new(id, cx.instance_handle()))
            }
            _ => func::bad_type_info(),
        }
    }

    /// Close this `FutureReader`, writing the default value.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future. Usage of this future after calling `close` will also cause
    /// a panic.
    pub fn close(&mut self, mut store: impl AsContextMut) {
        // `self` should never be used again, but leave an invalid handle there just in case.
        let id = mem::replace(&mut self.id, TableId::new(0));
        self.instance
            .host_drop_reader(
                store.as_context_mut().0.traitobj_mut(),
                id,
                TransmitKind::Future,
            )
            .unwrap();
    }

    /// Convenience method around [`Self::close`].
    pub fn close_with(&mut self, accessor: impl AsAccessor) {
        accessor.as_accessor().with(|access| self.close(access))
    }

    /// Returns a [`GuardedFutureReader`] which will auto-close this future on
    /// drop and clean it up from the store.
    ///
    /// Note that the `accessor` provided must own this future and is
    /// additionally transferred to the `GuardedFutureReader` return value.
    pub fn guard<A>(self, accessor: A) -> GuardedFutureReader<T, A>
    where
        A: AsAccessor,
    {
        GuardedFutureReader::new(accessor, self)
    }
}

impl<T> fmt::Debug for FutureReader<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FutureReader")
            .field("id", &self.id)
            .field("instance", &self.instance)
            .finish()
    }
}

/// Transfer ownership of the read end of a future from the host to a guest.
pub(crate) fn lower_future_to_index<U>(
    rep: u32,
    cx: &mut LowerContext<'_, U>,
    ty: InterfaceType,
) -> Result<u32> {
    match ty {
        InterfaceType::Future(dst) => {
            let concurrent_state = cx.instance_mut().concurrent_state_mut();
            let state = concurrent_state
                .get(TableId::<TransmitHandle>::new(rep))?
                .state;
            let rep = concurrent_state.get(state)?.read_handle.rep();

            concurrent_state
                .state_table(TableIndex::Future(dst))
                .insert(
                    rep,
                    WaitableState::Future(dst, StreamFutureState::Read { done: false }),
                )
        }
        _ => func::bad_type_info(),
    }
}

// SAFETY: This relies on the `ComponentType` implementation for `u32` being
// safe and correct since we lift and lower future handles as `u32`s.
unsafe impl<T: Send + Sync> func::ComponentType for FutureReader<T> {
    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::SCALAR4;

    type Lower = <u32 as func::ComponentType>::Lower;

    fn typecheck(ty: &InterfaceType, _types: &InstanceType<'_>) -> Result<()> {
        match ty {
            InterfaceType::Future(_) => Ok(()),
            other => bail!("expected `future`, found `{}`", func::desc(other)),
        }
    }
}

// SAFETY: See the comment on the `ComponentType` `impl` for this type.
unsafe impl<T: Send + Sync> func::Lower for FutureReader<T> {
    fn linear_lower_to_flat<U>(
        &self,
        cx: &mut LowerContext<'_, U>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Self::Lower>,
    ) -> Result<()> {
        lower_future_to_index(self.id.rep(), cx, ty)?.linear_lower_to_flat(
            cx,
            InterfaceType::U32,
            dst,
        )
    }

    fn linear_lower_to_memory<U>(
        &self,
        cx: &mut LowerContext<'_, U>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()> {
        lower_future_to_index(self.id.rep(), cx, ty)?.linear_lower_to_memory(
            cx,
            InterfaceType::U32,
            offset,
        )
    }
}

// SAFETY: See the comment on the `ComponentType` `impl` for this type.
unsafe impl<T: Send + Sync> func::Lift for FutureReader<T> {
    fn linear_lift_from_flat(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &Self::Lower,
    ) -> Result<Self> {
        let index = u32::linear_lift_from_flat(cx, InterfaceType::U32, src)?;
        Self::lift_from_index(cx, ty, index)
    }

    fn linear_lift_from_memory(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        bytes: &[u8],
    ) -> Result<Self> {
        let index = u32::linear_lift_from_memory(cx, InterfaceType::U32, bytes)?;
        Self::lift_from_index(cx, ty, index)
    }
}

/// A [`FutureReader`] paired with an [`Accessor`].
///
/// This is an RAII wrapper around [`FutureReader`] that ensures it is closed
/// when dropped. This can be created through [`GuardedFutureReader::new`] or
/// [`FutureReader::guard`].
pub struct GuardedFutureReader<T, A>
where
    A: AsAccessor,
{
    // This field is `None` to implement the conversion from this guard back to
    // `FutureReader`. When `None` is seen in the destructor it will cause the
    // destructor to do nothing.
    reader: Option<FutureReader<T>>,
    accessor: A,
}

impl<T, A> GuardedFutureReader<T, A>
where
    A: AsAccessor,
{
    /// Create a new `GuardedFutureReader` with the specified `accessor` and `reader`.
    pub fn new(accessor: A, reader: FutureReader<T>) -> Self {
        Self {
            reader: Some(reader),
            accessor,
        }
    }

    /// Wrapper for [`FutureReader::read`].
    pub async fn read(mut self) -> Option<T>
    where
        T: func::Lift + Send + 'static,
    {
        self.reader.as_mut().unwrap().read_(&self.accessor).await
    }

    /// Wrapper for [`FutureReader::watch_writer`].
    pub async fn watch_writer(&mut self) {
        self.reader
            .as_mut()
            .unwrap()
            .watch_writer(&self.accessor)
            .await
    }

    /// Extracts the underlying [`FutureReader`] from this guard, returning it
    /// back.
    pub fn into_future(self) -> FutureReader<T> {
        self.into()
    }
}

impl<T, A> From<GuardedFutureReader<T, A>> for FutureReader<T>
where
    A: AsAccessor,
{
    fn from(mut guard: GuardedFutureReader<T, A>) -> Self {
        guard.reader.take().unwrap()
    }
}

impl<T, A> Drop for GuardedFutureReader<T, A>
where
    A: AsAccessor,
{
    fn drop(&mut self) {
        if let Some(reader) = &mut self.reader {
            reader.close_with(&self.accessor)
        }
    }
}

/// Represents the writable end of a Component Model `stream`.
///
/// Note that `StreamWriter` instances must be disposed of using `close`;
/// otherwise the in-store representation will leak and the reader end will hang
/// indefinitely.  Consider using [`GuardedStreamWriter`] to ensure that
/// disposal happens automatically.
pub struct StreamWriter<T> {
    instance: Instance,
    id: TableId<TransmitHandle>,
    closed: bool,
    _phantom: PhantomData<T>,
}

impl<T> StreamWriter<T> {
    fn new(id: TableId<TransmitHandle>, instance: Instance) -> Self {
        Self {
            instance,
            id,
            closed: false,
            _phantom: PhantomData,
        }
    }

    /// Returns whether this stream is "closed" meaning that the other end of
    /// the stream has been dropped.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Write the specified items to the `stream`.
    ///
    /// Note that this will only write as many items as the reader accepts
    /// during its current or next read.  Use `write_all` to loop until the
    /// buffer is drained or the read end is dropped.
    ///
    /// The returned `Future` will yield the input buffer back,
    /// possibly consuming a subset of the items or nothing depending on the
    /// number of items the reader accepted.
    ///
    /// The [`is_closed`](Self::is_closed) method can be used to determine
    /// whether the stream was learned to be closed after this operation completes.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future.
    pub async fn write<B>(&mut self, accessor: impl AsAccessor, buffer: B) -> B
    where
        T: func::Lower + 'static,
        B: WriteBuffer<T>,
    {
        let result = self
            .instance
            .host_write_async(
                accessor.as_accessor(),
                self.id,
                buffer,
                TransmitKind::Stream,
            )
            .await;

        match result {
            Ok(HostResult { buffer, dropped }) => {
                if self.closed {
                    debug_assert!(dropped);
                }
                self.closed = dropped;
                buffer
            }
            Err(_) => todo!("guarantee buffer recovery if `host_write` fails"),
        }
    }

    /// Write the specified values until either the buffer is drained or the
    /// read end is dropped.
    ///
    /// The buffer is returned back to the caller and may still contain items
    /// within it if the other end of this stream was dropped. Use the
    /// [`is_closed`](Self::is_closed) method to determine if the other end is
    /// dropped.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future.
    pub async fn write_all<B>(&mut self, accessor: impl AsAccessor, mut buffer: B) -> B
    where
        T: func::Lower + 'static,
        B: WriteBuffer<T>,
    {
        let accessor = accessor.as_accessor();
        while !self.is_closed() && buffer.remaining().len() > 0 {
            buffer = self.write(accessor, buffer).await;
        }
        buffer
    }

    /// Wait for the read end of this `stream` to be dropped.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future.
    pub async fn watch_reader(&mut self, accessor: impl AsAccessor) {
        watch_reader(accessor, self.instance, self.id).await
    }

    /// Close this `StreamWriter`, writing the default value.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future. Usage of this future after calling `close` will also cause
    /// a panic.
    pub fn close(&mut self, mut store: impl AsContextMut) {
        // `self` should never be used again, but leave an invalid handle there just in case.
        let id = mem::replace(&mut self.id, TableId::new(0));
        self.instance
            .host_drop_writer(store.as_context_mut(), id, None::<&dyn Fn() -> Result<()>>)
            .unwrap()
    }

    /// Convenience method around [`Self::close`].
    pub fn close_with(&mut self, accessor: impl AsAccessor) {
        accessor.as_accessor().with(|access| self.close(access))
    }

    /// Returns a [`GuardedStreamWriter`] which will auto-close this stream on
    /// drop and clean it up from the store.
    ///
    /// Note that the `accessor` provided must own this future and is
    /// additionally transferred to the `GuardedStreamWriter` return value.
    pub fn guard<A>(self, accessor: A) -> GuardedStreamWriter<T, A>
    where
        A: AsAccessor,
    {
        GuardedStreamWriter::new(accessor, self)
    }
}

/// A [`StreamWriter`] paired with an [`Accessor`].
///
/// This is an RAII wrapper around [`StreamWriter`] that ensures it is closed
/// when dropped. This can be created through [`GuardedStreamWriter::new`] or
/// [`StreamWriter::guard`].
pub struct GuardedStreamWriter<T, A>
where
    A: AsAccessor,
{
    // This field is `None` to implement the conversion from this guard back to
    // `StreamWriter`. When `None` is seen in the destructor it will cause the
    // destructor to do nothing.
    writer: Option<StreamWriter<T>>,
    accessor: A,
}

impl<T, A> GuardedStreamWriter<T, A>
where
    A: AsAccessor,
{
    /// Create a new `GuardedStreamWriter` with the specified `accessor` and `writer`.
    pub fn new(accessor: A, writer: StreamWriter<T>) -> Self {
        Self {
            writer: Some(writer),
            accessor,
        }
    }

    /// Wrapper for [`StreamWriter::is_closed`].
    pub fn is_closed(&self) -> bool {
        self.writer.as_ref().unwrap().is_closed()
    }

    /// Wrapper for [`StreamWriter::write`].
    pub async fn write<B>(&mut self, buffer: B) -> B
    where
        T: func::Lower + 'static,
        B: WriteBuffer<T>,
    {
        self.writer
            .as_mut()
            .unwrap()
            .write(&self.accessor, buffer)
            .await
    }

    /// Wrapper for [`StreamWriter::write_all`].
    pub async fn write_all<B>(&mut self, buffer: B) -> B
    where
        T: func::Lower + 'static,
        B: WriteBuffer<T>,
    {
        self.writer
            .as_mut()
            .unwrap()
            .write_all(&self.accessor, buffer)
            .await
    }

    /// Wrapper for [`StreamWriter::watch_reader`].
    pub async fn watch_reader(&mut self) {
        self.writer
            .as_mut()
            .unwrap()
            .watch_reader(&self.accessor)
            .await
    }

    /// Extracts the underlying [`StreamWriter`] from this guard, returning it
    /// back.
    pub fn into_stream(self) -> StreamWriter<T> {
        self.into()
    }
}

impl<T, A> From<GuardedStreamWriter<T, A>> for StreamWriter<T>
where
    A: AsAccessor,
{
    fn from(mut guard: GuardedStreamWriter<T, A>) -> Self {
        guard.writer.take().unwrap()
    }
}

impl<T, A> Drop for GuardedStreamWriter<T, A>
where
    A: AsAccessor,
{
    fn drop(&mut self) {
        if let Some(writer) = &mut self.writer {
            writer.close_with(&self.accessor)
        }
    }
}

/// Represents the readable end of a Component Model `stream`.
///
/// Note that `StreamReader` instances must be disposed of using `close`;
/// otherwise the in-store representation will leak and the writer end will hang
/// indefinitely.  Consider using [`GuardedStreamReader`] to ensure that
/// disposal happens automatically.
pub struct StreamReader<T> {
    instance: Instance,
    id: TableId<TransmitHandle>,
    closed: bool,
    _phantom: PhantomData<T>,
}

impl<T> StreamReader<T> {
    fn new(id: TableId<TransmitHandle>, instance: Instance) -> Self {
        Self {
            instance,
            id,
            closed: false,
            _phantom: PhantomData,
        }
    }

    /// Returns whether this stream is "closed" meaning that the other end of
    /// the stream has been dropped.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Read values from this `stream`.
    ///
    /// The returned `Future` will yield a `(Some(_), _)` if the read completed
    /// (possibly with zero items if the write was empty).  It will return
    /// `(None, _)` if the read failed due to the closure of the write end. In
    /// either case, the returned buffer will be the same one passed as a
    /// parameter, with zero or more items added.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future.
    pub async fn read<B>(&mut self, accessor: impl AsAccessor, buffer: B) -> B
    where
        T: func::Lift + 'static,
        B: ReadBuffer<T> + Send + 'static,
    {
        let result = self
            .instance
            .host_read_async(
                accessor.as_accessor(),
                self.id,
                buffer,
                TransmitKind::Stream,
            )
            .await;

        match result {
            Ok(HostResult { buffer, dropped }) => {
                if self.closed {
                    debug_assert!(dropped);
                }
                self.closed = dropped;
                buffer
            }
            Err(_) => {
                todo!("guarantee buffer recovery if `host_read` fails")
            }
        }
    }

    /// Wait until the write end of this `stream` is dropped.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future.
    pub async fn watch_writer(&mut self, accessor: impl AsAccessor) {
        watch_writer(accessor, self.instance, self.id).await
    }

    /// Convert this `StreamReader` into a [`Val`].
    // See TODO comment for `StreamAny`; this is prone to handle leakage.
    pub fn into_val(self) -> Val {
        Val::Stream(StreamAny(self.id.rep()))
    }

    /// Attempt to convert the specified [`Val`] to a `StreamReader`.
    pub fn from_val(
        mut store: impl AsContextMut<Data: Send>,
        instance: Instance,
        value: &Val,
    ) -> Result<Self> {
        let Val::Stream(StreamAny(rep)) = value else {
            bail!("expected `stream`; got `{}`", value.desc());
        };
        let store = store.as_context_mut();
        let id = TableId::<TransmitHandle>::new(*rep);
        instance.concurrent_state_mut(store.0).get(id)?; // Just make sure it's present
        Ok(Self::new(id, instance))
    }

    /// Transfer ownership of the read end of a stream from a guest to the host.
    fn lift_from_index(cx: &mut LiftContext<'_>, ty: InterfaceType, index: u32) -> Result<Self> {
        match ty {
            InterfaceType::Stream(src) => {
                let state_table = cx
                    .instance_mut()
                    .concurrent_state_mut()
                    .state_table(TableIndex::Stream(src));
                let (rep, state) =
                    get_mut_by_index_from(state_table, TableIndex::Stream(src), index)?;

                match state {
                    StreamFutureState::Read { done: true } => bail!(
                        "cannot lift stream after being notified that the writable end dropped"
                    ),
                    StreamFutureState::Read { done: false } => {
                        state_table.remove_by_index(index)?;
                    }
                    StreamFutureState::Write { .. } => bail!("cannot transfer write end of stream"),
                    StreamFutureState::Busy => bail!("cannot transfer busy stream"),
                }

                let id = TableId::<TransmitHandle>::new(rep);

                Ok(Self::new(id, cx.instance_handle()))
            }
            _ => func::bad_type_info(),
        }
    }

    /// Close this `StreamReader`, writing the default value.
    ///
    /// # Panics
    ///
    /// Panics if the store that the [`Accessor`] is derived from does not own
    /// this future. Usage of this future after calling `close` will also cause
    /// a panic.
    pub fn close(&mut self, mut store: impl AsContextMut) {
        // `self` should never be used again, but leave an invalid handle there just in case.
        let id = mem::replace(&mut self.id, TableId::new(0));
        self.instance
            .host_drop_reader(
                store.as_context_mut().0.traitobj_mut(),
                id,
                TransmitKind::Stream,
            )
            .unwrap()
    }

    /// Convenience method around [`Self::close`].
    pub fn close_with(&mut self, accessor: impl AsAccessor) {
        accessor.as_accessor().with(|access| self.close(access))
    }

    /// Returns a [`GuardedStreamReader`] which will auto-close this stream on
    /// drop and clean it up from the store.
    ///
    /// Note that the `accessor` provided must own this future and is
    /// additionally transferred to the `GuardedStreamReader` return value.
    pub fn guard<A>(self, accessor: A) -> GuardedStreamReader<T, A>
    where
        A: AsAccessor,
    {
        GuardedStreamReader::new(accessor, self)
    }
}

impl<T> fmt::Debug for StreamReader<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamReader")
            .field("id", &self.id)
            .field("instance", &self.instance)
            .finish()
    }
}

/// Transfer ownership of the read end of a stream from the host to a guest.
pub(crate) fn lower_stream_to_index<U>(
    rep: u32,
    cx: &mut LowerContext<'_, U>,
    ty: InterfaceType,
) -> Result<u32> {
    match ty {
        InterfaceType::Stream(dst) => {
            let concurrent_state = cx.instance_mut().concurrent_state_mut();
            let state = concurrent_state
                .get(TableId::<TransmitHandle>::new(rep))?
                .state;
            let rep = concurrent_state.get(state)?.read_handle.rep();

            concurrent_state
                .state_table(TableIndex::Stream(dst))
                .insert(
                    rep,
                    WaitableState::Stream(dst, StreamFutureState::Read { done: false }),
                )
        }
        _ => func::bad_type_info(),
    }
}

// SAFETY: This relies on the `ComponentType` implementation for `u32` being
// safe and correct since we lift and lower stream handles as `u32`s.
unsafe impl<T: Send + Sync> func::ComponentType for StreamReader<T> {
    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::SCALAR4;

    type Lower = <u32 as func::ComponentType>::Lower;

    fn typecheck(ty: &InterfaceType, _types: &InstanceType<'_>) -> Result<()> {
        match ty {
            InterfaceType::Stream(_) => Ok(()),
            other => bail!("expected `stream`, found `{}`", func::desc(other)),
        }
    }
}

// SAFETY: See the comment on the `ComponentType` `impl` for this type.
unsafe impl<T: Send + Sync> func::Lower for StreamReader<T> {
    fn linear_lower_to_flat<U>(
        &self,
        cx: &mut LowerContext<'_, U>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Self::Lower>,
    ) -> Result<()> {
        lower_stream_to_index(self.id.rep(), cx, ty)?.linear_lower_to_flat(
            cx,
            InterfaceType::U32,
            dst,
        )
    }

    fn linear_lower_to_memory<U>(
        &self,
        cx: &mut LowerContext<'_, U>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()> {
        lower_stream_to_index(self.id.rep(), cx, ty)?.linear_lower_to_memory(
            cx,
            InterfaceType::U32,
            offset,
        )
    }
}

// SAFETY: See the comment on the `ComponentType` `impl` for this type.
unsafe impl<T: Send + Sync> func::Lift for StreamReader<T> {
    fn linear_lift_from_flat(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &Self::Lower,
    ) -> Result<Self> {
        let index = u32::linear_lift_from_flat(cx, InterfaceType::U32, src)?;
        Self::lift_from_index(cx, ty, index)
    }

    fn linear_lift_from_memory(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        bytes: &[u8],
    ) -> Result<Self> {
        let index = u32::linear_lift_from_memory(cx, InterfaceType::U32, bytes)?;
        Self::lift_from_index(cx, ty, index)
    }
}

/// A [`StreamReader`] paired with an [`Accessor`].
///
/// This is an RAII wrapper around [`StreamReader`] that ensures it is closed
/// when dropped. This can be created through [`GuardedStreamReader::new`] or
/// [`StreamReader::guard`].
pub struct GuardedStreamReader<T, A>
where
    A: AsAccessor,
{
    // This field is `None` to implement the conversion from this guard back to
    // `StreamReader`. When `None` is seen in the destructor it will cause the
    // destructor to do nothing.
    reader: Option<StreamReader<T>>,
    accessor: A,
}

impl<T, A> GuardedStreamReader<T, A>
where
    A: AsAccessor,
{
    /// Create a new `GuardedStreamReader` with the specified `accessor` and
    /// `reader`.
    pub fn new(accessor: A, reader: StreamReader<T>) -> Self {
        Self {
            reader: Some(reader),
            accessor,
        }
    }

    /// Wrapper for `StreamReader::is_closed`
    pub fn is_closed(&self) -> bool {
        self.reader.as_ref().unwrap().is_closed()
    }

    /// Wrapper for `StreamReader::read`.
    pub async fn read<B>(&mut self, buffer: B) -> B
    where
        T: func::Lift + 'static,
        B: ReadBuffer<T> + Send + 'static,
    {
        self.reader
            .as_mut()
            .unwrap()
            .read(&self.accessor, buffer)
            .await
    }

    /// Wrapper for `StreamReader::watch_writer`.
    pub async fn watch_writer(&mut self) {
        self.reader
            .as_mut()
            .unwrap()
            .watch_writer(&self.accessor)
            .await
    }

    /// Extracts the underlying [`StreamReader`] from this guard, returning it
    /// back.
    pub fn into_stream(self) -> StreamReader<T> {
        self.into()
    }
}

impl<T, A> From<GuardedStreamReader<T, A>> for StreamReader<T>
where
    A: AsAccessor,
{
    fn from(mut guard: GuardedStreamReader<T, A>) -> Self {
        guard.reader.take().unwrap()
    }
}

impl<T, A> Drop for GuardedStreamReader<T, A>
where
    A: AsAccessor,
{
    fn drop(&mut self) {
        if let Some(reader) = &mut self.reader {
            reader.close_with(&self.accessor)
        }
    }
}

/// Represents a Component Model `error-context`.
pub struct ErrorContext {
    rep: u32,
}

impl ErrorContext {
    pub(crate) fn new(rep: u32) -> Self {
        Self { rep }
    }

    /// Convert this `ErrorContext` into a [`Val`].
    pub fn into_val(self) -> Val {
        Val::ErrorContext(ErrorContextAny(self.rep))
    }

    /// Attempt to convert the specified [`Val`] to a `ErrorContext`.
    pub fn from_val(_: impl AsContextMut, value: &Val) -> Result<Self> {
        let Val::ErrorContext(ErrorContextAny(rep)) = value else {
            bail!("expected `error-context`; got `{}`", value.desc());
        };
        Ok(Self::new(*rep))
    }

    fn lift_from_index(cx: &mut LiftContext<'_>, ty: InterfaceType, index: u32) -> Result<Self> {
        match ty {
            InterfaceType::ErrorContext(src) => {
                let (rep, _) = cx
                    .instance_mut()
                    .concurrent_state_mut()
                    .error_context_tables
                    .get_mut(src)
                    .expect("error context table index present in (sub)component table during lift")
                    .get_mut_by_index(index)?;

                Ok(Self { rep })
            }
            _ => func::bad_type_info(),
        }
    }
}

pub(crate) fn lower_error_context_to_index<U>(
    rep: u32,
    cx: &mut LowerContext<'_, U>,
    ty: InterfaceType,
) -> Result<u32> {
    match ty {
        InterfaceType::ErrorContext(dst) => {
            let tbl = cx
                .instance_mut()
                .concurrent_state_mut()
                .error_context_tables
                .get_mut(dst)
                .expect("error context table index present in (sub)component table during lower");

            if let Some((dst_idx, dst_state)) = tbl.get_mut_by_rep(rep) {
                dst_state.0 += 1;
                Ok(dst_idx)
            } else {
                tbl.insert(rep, LocalErrorContextRefCount(1))
            }
        }
        _ => func::bad_type_info(),
    }
}

// SAFETY: This relies on the `ComponentType` implementation for `u32` being
// safe and correct since we lift and lower future handles as `u32`s.
unsafe impl func::ComponentType for ErrorContext {
    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::SCALAR4;

    type Lower = <u32 as func::ComponentType>::Lower;

    fn typecheck(ty: &InterfaceType, _types: &InstanceType<'_>) -> Result<()> {
        match ty {
            InterfaceType::ErrorContext(_) => Ok(()),
            other => bail!("expected `error`, found `{}`", func::desc(other)),
        }
    }
}

// SAFETY: See the comment on the `ComponentType` `impl` for this type.
unsafe impl func::Lower for ErrorContext {
    fn linear_lower_to_flat<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Self::Lower>,
    ) -> Result<()> {
        lower_error_context_to_index(self.rep, cx, ty)?.linear_lower_to_flat(
            cx,
            InterfaceType::U32,
            dst,
        )
    }

    fn linear_lower_to_memory<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()> {
        lower_error_context_to_index(self.rep, cx, ty)?.linear_lower_to_memory(
            cx,
            InterfaceType::U32,
            offset,
        )
    }
}

// SAFETY: See the comment on the `ComponentType` `impl` for this type.
unsafe impl func::Lift for ErrorContext {
    fn linear_lift_from_flat(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &Self::Lower,
    ) -> Result<Self> {
        let index = u32::linear_lift_from_flat(cx, InterfaceType::U32, src)?;
        Self::lift_from_index(cx, ty, index)
    }

    fn linear_lift_from_memory(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        bytes: &[u8],
    ) -> Result<Self> {
        let index = u32::linear_lift_from_memory(cx, InterfaceType::U32, bytes)?;
        Self::lift_from_index(cx, ty, index)
    }
}

/// Represents the read or write end of a stream or future.
pub(super) struct TransmitHandle {
    pub(super) common: WaitableCommon,
    /// See `TransmitState`
    state: TableId<TransmitState>,
}

impl TransmitHandle {
    fn new(state: TableId<TransmitState>) -> Self {
        Self {
            common: WaitableCommon::default(),
            state,
        }
    }
}

impl TableDebug for TransmitHandle {
    fn type_name() -> &'static str {
        "TransmitHandle"
    }
}

/// Represents the state of a stream or future.
struct TransmitState {
    /// The write end of the stream or future.
    write_handle: TableId<TransmitHandle>,
    /// The read end of the stream or future.
    read_handle: TableId<TransmitHandle>,
    /// See `WriteState`
    write: WriteState,
    /// See `ReadState`
    read: ReadState,
    /// The `Waker`, if any, to be woken when the write end of the stream or
    /// future is dropped.
    ///
    /// This will signal to the host-owned read end that the write end has been
    /// dropped.
    writer_watcher: Option<Waker>,
    /// Like `writer_watcher`, but for the reverse direction.
    reader_watcher: Option<Waker>,
    /// Whether futher values may be transmitted via this stream or future.
    done: bool,
}

impl Default for TransmitState {
    fn default() -> Self {
        Self {
            write_handle: TableId::new(0),
            read_handle: TableId::new(0),
            read: ReadState::Open,
            write: WriteState::Open,
            reader_watcher: None,
            writer_watcher: None,
            done: false,
        }
    }
}

impl TableDebug for TransmitState {
    fn type_name() -> &'static str {
        "TransmitState"
    }
}

/// Represents the state of the write end of a stream or future.
enum WriteState {
    /// The write end is open, but no write is pending.
    Open,
    /// The write end is owned by a guest task and a write is pending.
    GuestReady {
        ty: TableIndex,
        flat_abi: Option<FlatAbi>,
        options: Options,
        address: usize,
        count: usize,
        handle: u32,
        post_write: PostWrite,
    },
    /// The write end is owned by a host task and a write is pending.
    HostReady {
        accept:
            Box<dyn FnOnce(&mut dyn VMStore, Instance, Reader) -> Result<ReturnCode> + Send + Sync>,
        post_write: PostWrite,
    },
    /// The write end has been dropped.
    Dropped,
}

impl fmt::Debug for WriteState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Open => f.debug_tuple("Open").finish(),
            Self::GuestReady { .. } => f.debug_tuple("GuestReady").finish(),
            Self::HostReady { .. } => f.debug_tuple("HostReady").finish(),
            Self::Dropped => f.debug_tuple("Dropped").finish(),
        }
    }
}

/// Represents the state of the read end of a stream or future.
enum ReadState {
    /// The read end is open, but no read is pending.
    Open,
    /// The read end is owned by a guest task and a read is pending.
    GuestReady {
        ty: TableIndex,
        flat_abi: Option<FlatAbi>,
        options: Options,
        address: usize,
        count: usize,
        handle: u32,
    },
    /// The read end is owned by a host task and a read is pending.
    HostReady {
        accept: Box<dyn FnOnce(Writer) -> Result<ReturnCode> + Send + Sync>,
    },
    /// The read end has been dropped.
    Dropped,
}

impl fmt::Debug for ReadState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Open => f.debug_tuple("Open").finish(),
            Self::GuestReady { .. } => f.debug_tuple("GuestReady").finish(),
            Self::HostReady { .. } => f.debug_tuple("HostReady").finish(),
            Self::Dropped => f.debug_tuple("Dropped").finish(),
        }
    }
}

/// Parameter type to pass to a `ReadState::HostReady` closure.
///
/// See also `accept_writer`.
enum Writer<'a> {
    /// The write end is owned by a guest task.
    Guest {
        lift: &'a mut LiftContext<'a>,
        ty: Option<InterfaceType>,
        address: usize,
        count: usize,
    },
    /// The write end is owned by the host.
    Host {
        buffer: &'a mut UntypedWriteBuffer<'a>,
        count: usize,
    },
    /// The write end has been dropped.
    End,
}

/// Parameter type to pass to a `WriteState::HostReady` closure.
///
/// See also `accept_reader`.
enum Reader<'a> {
    /// The read end is owned by a guest task.
    Guest {
        options: &'a Options,
        ty: TableIndex,
        address: usize,
        count: usize,
    },
    /// The read end is owned by the host.
    Host {
        accept: Box<dyn FnOnce(&mut UntypedWriteBuffer, usize) -> usize + 'a>,
    },
    /// The read end has been dropped.
    End,
}

impl Instance {
    /// Create a new Component Model `future` as pair of writable and readable ends,
    /// the latter of which may be passed to guest code.
    ///
    /// `default` is a callback to be used if the writable end of the future is
    /// closed without having written a value.  You may supply e.g. `||
    /// unreachable!()` if you're sure that won't happen.
    pub fn future<T: func::Lower + func::Lift + Send + Sync + 'static>(
        self,
        mut store: impl AsContextMut,
        default: fn() -> T,
    ) -> Result<(FutureWriter<T>, FutureReader<T>)> {
        let (write, read) = self
            .concurrent_state_mut(store.as_context_mut().0)
            .new_transmit()?;

        Ok((
            FutureWriter::new(default, write, self),
            FutureReader::new(read, self),
        ))
    }

    /// Create a new Component Model `stream` as pair of writable and readable ends,
    /// the latter of which may be passed to guest code.
    pub fn stream<T: func::Lower + func::Lift + Send + 'static>(
        self,
        mut store: impl AsContextMut,
    ) -> Result<(StreamWriter<T>, StreamReader<T>)> {
        let (write, read) = self
            .concurrent_state_mut(store.as_context_mut().0)
            .new_transmit()?;

        Ok((
            StreamWriter::new(write, self),
            StreamReader::new(read, self),
        ))
    }

    /// Write to the specified stream or future from the host.
    fn host_write<T: func::Lower + Send + 'static, B: WriteBuffer<T>, U>(
        self,
        mut store: StoreContextMut<U>,
        id: TableId<TransmitHandle>,
        mut buffer: B,
        kind: TransmitKind,
        post_write: PostWrite,
    ) -> Result<Result<HostResult<B>, oneshot::Receiver<HostResult<B>>>> {
        let transmit_id = self.concurrent_state_mut(store.0).get(id)?.state;
        let transmit = self
            .concurrent_state_mut(store.0)
            .get_mut(transmit_id)
            .with_context(|| format!("retrieving state for transmit [{transmit_id:?}]"))?;
        log::trace!("host_write state {transmit_id:?}; {:?}", transmit.read);

        let new_state = if let ReadState::Dropped = &transmit.read {
            ReadState::Dropped
        } else {
            ReadState::Open
        };

        if matches!(post_write, PostWrite::Drop) && !matches!(transmit.read, ReadState::Open) {
            transmit.write = WriteState::Dropped;
        }

        Ok(match mem::replace(&mut transmit.read, new_state) {
            ReadState::Open => {
                assert!(matches!(&transmit.write, WriteState::Open));

                let token = StoreToken::new(store.as_context_mut());
                let (tx, rx) = oneshot::channel();
                let state = WriteState::HostReady {
                    accept: Box::new(move |store, instance, reader| {
                        let (result, code) = accept_reader::<T, B, U>(
                            token.as_context_mut(store),
                            instance,
                            reader,
                            buffer,
                            kind,
                        )?;
                        _ = tx.send(result);
                        Ok(code)
                    }),
                    post_write,
                };
                self.concurrent_state_mut(store.0)
                    .get_mut(transmit_id)?
                    .write = state;

                Err(rx)
            }

            ReadState::GuestReady {
                ty,
                flat_abi: _,
                options,
                address,
                count,
                handle,
                ..
            } => {
                if let TransmitKind::Future = kind {
                    transmit.done = true;
                }

                let read_handle = transmit.read_handle;
                let accept = move |mut store: StoreContextMut<U>| {
                    let (result, code) = accept_reader::<T, B, U>(
                        store.as_context_mut(),
                        self,
                        Reader::Guest {
                            options: &options,
                            ty,
                            address,
                            count,
                        },
                        buffer,
                        kind,
                    )?;

                    self.concurrent_state_mut(store.0).set_event(
                        read_handle.rep(),
                        match ty {
                            TableIndex::Future(ty) => Event::FutureRead {
                                code,
                                pending: Some((ty, handle)),
                            },
                            TableIndex::Stream(ty) => Event::StreamRead {
                                code,
                                pending: Some((ty, handle)),
                            },
                        },
                    )?;

                    anyhow::Ok(result)
                };

                if T::MAY_REQUIRE_REALLOC {
                    // For payloads which may require a realloc call, use a
                    // oneshot::channel and background task.  This is necessary
                    // because calling the guest while there are host embedder
                    // frames on the stack is unsound.
                    let (tx, rx) = oneshot::channel();
                    let token = StoreToken::new(store.as_context_mut());
                    self.concurrent_state_mut(store.0).push_high_priority(
                        WorkItem::WorkerFunction(Mutex::new(Box::new(move |store, _| {
                            _ = tx.send(accept(token.as_context_mut(store))?);
                            Ok(())
                        }))),
                    );
                    Err(rx)
                } else {
                    // Optimize flat payloads (i.e. those which do not require
                    // calling the guest's realloc function) by lowering
                    // directly instead of using a oneshot::channel and
                    // background task.
                    Ok(accept(store)?)
                }
            }

            ReadState::HostReady { accept } => {
                let count = buffer.remaining().len();
                let mut untyped = UntypedWriteBuffer::new(&mut buffer);
                let code = accept(Writer::Host {
                    buffer: &mut untyped,
                    count,
                })?;
                let (ReturnCode::Completed(_) | ReturnCode::Dropped(_)) = code else {
                    unreachable!()
                };

                Ok(HostResult {
                    buffer,
                    dropped: false,
                })
            }

            ReadState::Dropped => Ok(HostResult {
                buffer,
                dropped: true,
            }),
        })
    }

    /// Async wrapper around `Self::host_write`.
    async fn host_write_async<T: func::Lower + Send + 'static, B: WriteBuffer<T>>(
        self,
        accessor: impl AsAccessor,
        id: TableId<TransmitHandle>,
        buffer: B,
        kind: TransmitKind,
    ) -> Result<HostResult<B>> {
        match accessor.as_accessor().with(move |mut access| {
            self.host_write(
                access.as_context_mut(),
                id,
                buffer,
                kind,
                PostWrite::Continue,
            )
        })? {
            Ok(result) => Ok(result),
            Err(rx) => Ok(rx.await?),
        }
    }

    /// Read from the specified stream or future from the host.
    fn host_read<T: func::Lift + Send + 'static, B: ReadBuffer<T>, U>(
        self,
        store: StoreContextMut<U>,
        id: TableId<TransmitHandle>,
        mut buffer: B,
        kind: TransmitKind,
    ) -> Result<Result<HostResult<B>, oneshot::Receiver<HostResult<B>>>> {
        let transmit_id = self.concurrent_state_mut(store.0).get(id)?.state;
        let transmit = self
            .concurrent_state_mut(store.0)
            .get_mut(transmit_id)
            .with_context(|| format!("retrieving state for transmit [{transmit_id:?}]"))?;
        log::trace!("host_read state {transmit_id:?}; {:?}", transmit.write);

        let new_state = if let WriteState::Dropped = &transmit.write {
            WriteState::Dropped
        } else {
            WriteState::Open
        };

        Ok(match mem::replace(&mut transmit.write, new_state) {
            WriteState::Open => {
                assert!(matches!(&transmit.read, ReadState::Open));

                let (tx, rx) = oneshot::channel();
                transmit.read = ReadState::HostReady {
                    accept: Box::new(move |writer| {
                        let (result, code) = accept_writer::<T, B, U>(writer, buffer, kind)?;
                        _ = tx.send(result);
                        Ok(code)
                    }),
                };

                Err(rx)
            }

            WriteState::GuestReady {
                ty,
                flat_abi: _,
                options,
                address,
                count,
                handle,
                post_write,
                ..
            } => {
                if let TableIndex::Future(_) = ty {
                    transmit.done = true;
                }

                let write_handle = transmit.write_handle;
                let lift = &mut LiftContext::new(store.0.store_opaque_mut(), &options, self);
                let (result, code) = accept_writer::<T, B, U>(
                    Writer::Guest {
                        ty: payload(ty, lift.types),
                        lift,
                        address,
                        count,
                    },
                    buffer,
                    kind,
                )?;

                let state = self.concurrent_state_mut(store.0);
                let pending = if let PostWrite::Drop = post_write {
                    state.get_mut(transmit_id)?.write = WriteState::Dropped;
                    false
                } else {
                    true
                };

                state.set_event(
                    write_handle.rep(),
                    match ty {
                        TableIndex::Future(ty) => Event::FutureWrite {
                            code,
                            pending: pending.then_some((ty, handle)),
                        },
                        TableIndex::Stream(ty) => Event::StreamWrite {
                            code,
                            pending: pending.then_some((ty, handle)),
                        },
                    },
                )?;

                Ok(result)
            }

            WriteState::HostReady { accept, post_write } => {
                accept(
                    store.0.traitobj_mut(),
                    self,
                    Reader::Host {
                        accept: Box::new(|input, count| {
                            let count = count.min(buffer.remaining_capacity());
                            buffer.move_from(input.get_mut::<T>(), count);
                            count
                        }),
                    },
                )?;

                if let PostWrite::Drop = post_write {
                    self.concurrent_state_mut(store.0)
                        .get_mut(transmit_id)?
                        .write = WriteState::Dropped;
                }

                Ok(HostResult {
                    buffer,
                    dropped: false,
                })
            }

            WriteState::Dropped => Ok(HostResult {
                buffer,
                dropped: true,
            }),
        })
    }

    /// Async wrapper around `Self::host_read`.
    async fn host_read_async<T: func::Lift + Send + 'static, B: ReadBuffer<T>>(
        self,
        accessor: impl AsAccessor,
        id: TableId<TransmitHandle>,
        buffer: B,
        kind: TransmitKind,
    ) -> Result<HostResult<B>> {
        match accessor
            .as_accessor()
            .with(move |mut access| self.host_read(access.as_context_mut(), id, buffer, kind))?
        {
            Ok(result) => Ok(result),
            Err(rx) => Ok(rx.await?),
        }
    }

    /// Drop the read end of a stream or future read from the host.
    fn host_drop_reader(
        self,
        store: &mut dyn VMStore,
        id: TableId<TransmitHandle>,
        kind: TransmitKind,
    ) -> Result<()> {
        let transmit_id = self.concurrent_state_mut(store).get(id)?.state;
        let state = self.concurrent_state_mut(store);
        let transmit = state
            .get_mut(transmit_id)
            .with_context(|| format!("error closing reader {transmit_id:?}"))?;
        log::trace!(
            "host_drop_reader state {transmit_id:?}; read state {:?} write state {:?}",
            transmit.read,
            transmit.write
        );

        transmit.read = ReadState::Dropped;
        if let Some(waker) = transmit.reader_watcher.take() {
            waker.wake();
        }

        // If the write end is already dropped, it should stay dropped,
        // otherwise, it should be opened.
        let new_state = if let WriteState::Dropped = &transmit.write {
            WriteState::Dropped
        } else {
            WriteState::Open
        };

        let write_handle = transmit.write_handle;

        match mem::replace(&mut transmit.write, new_state) {
            // If a guest is waiting to write, notify it that the read end has
            // been dropped.
            WriteState::GuestReady {
                ty,
                handle,
                post_write,
                ..
            } => {
                if let PostWrite::Drop = post_write {
                    state.delete_transmit(transmit_id)?;
                } else {
                    state.update_event(
                        write_handle.rep(),
                        match ty {
                            TableIndex::Future(ty) => Event::FutureWrite {
                                code: ReturnCode::Dropped(0),
                                pending: Some((ty, handle)),
                            },
                            TableIndex::Stream(ty) => Event::StreamWrite {
                                code: ReturnCode::Dropped(0),
                                pending: Some((ty, handle)),
                            },
                        },
                    )?;
                };
            }

            WriteState::HostReady { accept, .. } => {
                accept(store, self, Reader::End)?;
            }

            WriteState::Open => {
                state.update_event(
                    write_handle.rep(),
                    match kind {
                        TransmitKind::Future => Event::FutureWrite {
                            code: ReturnCode::Dropped(0),
                            pending: None,
                        },
                        TransmitKind::Stream => Event::StreamWrite {
                            code: ReturnCode::Dropped(0),
                            pending: None,
                        },
                    },
                )?;
            }

            WriteState::Dropped => {
                log::trace!("host_drop_reader delete {transmit_id:?}");
                state.delete_transmit(transmit_id)?;
            }
        }
        Ok(())
    }

    /// Drop the write end of a stream or future read from the host.
    fn host_drop_writer<T: func::Lower + Send + 'static, U>(
        self,
        mut store: StoreContextMut<U>,
        id: TableId<TransmitHandle>,
        default: Option<&dyn Fn() -> Result<T>>,
    ) -> Result<()> {
        let transmit_id = self.concurrent_state_mut(store.0).get(id)?.state;
        let transmit = self
            .concurrent_state_mut(store.0)
            .get_mut(transmit_id)
            .with_context(|| format!("error closing writer {transmit_id:?}"))?;
        log::trace!(
            "host_drop_writer state {transmit_id:?}; write state {:?} read state {:?}",
            transmit.read,
            transmit.write
        );

        if let Some(waker) = transmit.writer_watcher.take() {
            waker.wake();
        }

        // Existing queued transmits must be updated with information for the impending writer closure
        match &mut transmit.write {
            WriteState::GuestReady { .. } => {
                unreachable!("can't call `host_drop_writer` on a guest-owned writer");
            }
            WriteState::HostReady { post_write, .. } => {
                *post_write = PostWrite::Drop;
            }
            v @ WriteState::Open => {
                if let (Some(default), false) = (
                    default,
                    transmit.done || matches!(transmit.read, ReadState::Dropped),
                ) {
                    // This is a future, and we haven't written a value yet --
                    // write the default value.
                    _ = self.host_write(
                        store.as_context_mut(),
                        id,
                        Some(default()?),
                        TransmitKind::Future,
                        PostWrite::Drop,
                    )?;
                } else {
                    *v = WriteState::Dropped;
                }
            }
            WriteState::Dropped => unreachable!("write state is already dropped"),
        }

        let transmit = self.concurrent_state_mut(store.0).get_mut(transmit_id)?;

        // If the existing read state is dropped, then there's nothing to read
        // and we can keep it that way.
        //
        // If the read state was any other state, then we must set the new state to open
        // to indicate that there *is* data to be read
        let new_state = if let ReadState::Dropped = &transmit.read {
            ReadState::Dropped
        } else {
            ReadState::Open
        };

        let read_handle = transmit.read_handle;

        // Swap in the new read state
        match mem::replace(&mut transmit.read, new_state) {
            // If the guest was ready to read, then we cannot drop the reader (or writer);
            // we must deliver the event, and update the state associated with the handle to
            // represent that a read must be performed
            ReadState::GuestReady { ty, handle, .. } => {
                // Ensure the final read of the guest is queued, with appropriate closure indicator
                self.concurrent_state_mut(store.0).update_event(
                    read_handle.rep(),
                    match ty {
                        TableIndex::Future(ty) => Event::FutureRead {
                            code: ReturnCode::Dropped(0),
                            pending: Some((ty, handle)),
                        },
                        TableIndex::Stream(ty) => Event::StreamRead {
                            code: ReturnCode::Dropped(0),
                            pending: Some((ty, handle)),
                        },
                    },
                )?;
            }

            // If the host was ready to read, and the writer end is being dropped (host->host write?)
            // signal to the reader that we've reached the end of the stream
            ReadState::HostReady { accept } => {
                accept(Writer::End)?;
            }

            // If the read state is open, then there are no registered readers of the stream/future
            ReadState::Open => {
                self.concurrent_state_mut(store.0).update_event(
                    read_handle.rep(),
                    match default {
                        Some(_) => Event::FutureRead {
                            code: ReturnCode::Dropped(0),
                            pending: None,
                        },
                        None => Event::StreamRead {
                            code: ReturnCode::Dropped(0),
                            pending: None,
                        },
                    },
                )?;
            }

            // If the read state was already dropped, then we can remove the transmit state completely
            // (both writer and reader have been dropped)
            ReadState::Dropped => {
                log::trace!("host_drop_writer delete {transmit_id:?}");
                self.concurrent_state_mut(store.0)
                    .delete_transmit(transmit_id)?;
            }
        }
        Ok(())
    }

    /// Drop the writable end of the specified stream or future from the guest.
    pub(super) fn guest_drop_writable<T>(
        self,
        store: StoreContextMut<T>,
        ty: TableIndex,
        writer: u32,
    ) -> Result<()> {
        let (transmit_rep, state) = self
            .concurrent_state_mut(store.0)
            .state_table(ty)
            .remove_by_index(writer)
            .context("failed to find writer")?;
        let (state, kind) = match state {
            WaitableState::Stream(_, state) => (state, TransmitKind::Stream),
            WaitableState::Future(_, state) => (state, TransmitKind::Future),
            _ => {
                bail!("invalid stream or future handle");
            }
        };
        match state {
            StreamFutureState::Write { .. } => {}
            StreamFutureState::Read { .. } => {
                bail!("passed read end to `{{stream|future}}.drop-writable`")
            }
            StreamFutureState::Busy => bail!("cannot drop busy stream or future"),
        }

        let id = TableId::<TransmitHandle>::new(transmit_rep);
        log::trace!("guest_drop_writable: drop writer {id:?}");
        match kind {
            TransmitKind::Stream => {
                self.host_drop_writer(store, id, None::<&dyn Fn() -> Result<()>>)
            }
            TransmitKind::Future => self.host_drop_writer(
                store,
                id,
                Some(&|| {
                    Err::<(), _>(anyhow!(
                        "cannot drop future write end without first writing a value"
                    ))
                }),
            ),
        }
    }

    /// Copy `count` items from `read_address` to `write_address` for the
    /// specified stream or future.
    fn copy<T: 'static>(
        self,
        mut store: StoreContextMut<T>,
        flat_abi: Option<FlatAbi>,
        write_ty: TableIndex,
        write_options: &Options,
        write_address: usize,
        read_ty: TableIndex,
        read_options: &Options,
        read_address: usize,
        count: usize,
        rep: u32,
    ) -> Result<()> {
        let types = self.id().get(store.0).component().types().clone();
        match (write_ty, read_ty) {
            (TableIndex::Future(write_ty), TableIndex::Future(read_ty)) => {
                assert_eq!(count, 1);

                let val = types[types[write_ty].ty]
                    .payload
                    .map(|ty| {
                        let abi = types.canonical_abi(&ty);
                        // FIXME: needs to read an i64 for memory64
                        if write_address % usize::try_from(abi.align32)? != 0 {
                            bail!("write pointer not aligned");
                        }

                        let lift =
                            &mut LiftContext::new(store.0.store_opaque_mut(), write_options, self);
                        let bytes = lift
                            .memory()
                            .get(write_address..)
                            .and_then(|b| b.get(..usize::try_from(abi.size32).unwrap()))
                            .ok_or_else(|| {
                                anyhow::anyhow!("write pointer out of bounds of memory")
                            })?;

                        Val::load(lift, ty, bytes)
                    })
                    .transpose()?;

                if let Some(val) = val {
                    let lower =
                        &mut LowerContext::new(store.as_context_mut(), read_options, &types, self);
                    let ty = types[types[read_ty].ty].payload.unwrap();
                    let ptr = func::validate_inbounds_dynamic(
                        types.canonical_abi(&ty),
                        lower.as_slice_mut(),
                        &ValRaw::u32(read_address.try_into().unwrap()),
                    )?;
                    val.store(lower, ty, ptr)?;
                }
            }
            (TableIndex::Stream(write_ty), TableIndex::Stream(read_ty)) => {
                if let Some(flat_abi) = flat_abi {
                    // Fast path memcpy for "flat" (i.e. no pointers or handles) payloads:
                    let length_in_bytes = usize::try_from(flat_abi.size).unwrap() * count;
                    if length_in_bytes > 0 {
                        if write_address % usize::try_from(flat_abi.align)? != 0 {
                            bail!("write pointer not aligned");
                        }
                        if read_address % usize::try_from(flat_abi.align)? != 0 {
                            bail!("read pointer not aligned");
                        }

                        let store_opaque = store.0.store_opaque_mut();

                        {
                            let src = write_options
                                .memory(store_opaque)
                                .get(write_address..)
                                .and_then(|b| b.get(..length_in_bytes))
                                .ok_or_else(|| {
                                    anyhow::anyhow!("write pointer out of bounds of memory")
                                })?
                                .as_ptr();
                            let dst = read_options
                                .memory_mut(store_opaque)
                                .get_mut(read_address..)
                                .and_then(|b| b.get_mut(..length_in_bytes))
                                .ok_or_else(|| {
                                    anyhow::anyhow!("read pointer out of bounds of memory")
                                })?
                                .as_mut_ptr();
                            // SAFETY: Both `src` and `dst` have been validated
                            // above.
                            unsafe { src.copy_to(dst, length_in_bytes) };
                        }
                    }
                } else {
                    let store_opaque = store.0.store_opaque_mut();
                    let lift = &mut LiftContext::new(store_opaque, write_options, self);
                    let ty = types[types[write_ty].ty].payload.unwrap();
                    let abi = lift.types.canonical_abi(&ty);
                    let size = usize::try_from(abi.size32).unwrap();
                    if write_address % usize::try_from(abi.align32)? != 0 {
                        bail!("write pointer not aligned");
                    }
                    let bytes = lift
                        .memory()
                        .get(write_address..)
                        .and_then(|b| b.get(..size * count))
                        .ok_or_else(|| anyhow::anyhow!("write pointer out of bounds of memory"))?;

                    let values = (0..count)
                        .map(|index| Val::load(lift, ty, &bytes[(index * size)..][..size]))
                        .collect::<Result<Vec<_>>>()?;

                    let id = TableId::<TransmitHandle>::new(rep);
                    log::trace!("copy values {values:?} for {id:?}");

                    let lower =
                        &mut LowerContext::new(store.as_context_mut(), read_options, &types, self);
                    let ty = types[types[read_ty].ty].payload.unwrap();
                    let abi = lower.types.canonical_abi(&ty);
                    if read_address % usize::try_from(abi.align32)? != 0 {
                        bail!("read pointer not aligned");
                    }
                    let size = usize::try_from(abi.size32).unwrap();
                    lower
                        .as_slice_mut()
                        .get_mut(read_address..)
                        .and_then(|b| b.get_mut(..size * count))
                        .ok_or_else(|| anyhow::anyhow!("read pointer out of bounds of memory"))?;
                    let mut ptr = read_address;
                    for value in values {
                        value.store(lower, ty, ptr)?;
                        ptr += size
                    }
                }
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    /// Write to the specified stream or future from the guest.
    pub(super) fn guest_write<T: 'static>(
        self,
        mut store: StoreContextMut<T>,
        ty: TableIndex,
        options: OptionsIndex,
        flat_abi: Option<FlatAbi>,
        handle: u32,
        address: u32,
        count: u32,
    ) -> Result<ReturnCode> {
        let address = usize::try_from(address).unwrap();
        let count = usize::try_from(count).unwrap();
        let options = Options::new_index(store.0, self, options);
        if !options.async_() {
            bail!("synchronous stream and future writes not yet supported");
        }
        let concurrent_state = self.concurrent_state_mut(store.0);
        let (rep, state) = concurrent_state.get_mut_by_index(ty, handle)?;
        let StreamFutureState::Write { done } = *state else {
            bail!(
                "invalid handle {handle}; expected `Write`; got {:?}",
                *state
            );
        };

        if done {
            bail!("cannot write to stream after being notified that the readable end dropped");
        }

        *state = StreamFutureState::Busy;
        let transmit_handle = TableId::<TransmitHandle>::new(rep);
        let transmit_id = concurrent_state.get(transmit_handle)?.state;
        let transmit = concurrent_state.get_mut(transmit_id)?;
        log::trace!(
            "guest_write {transmit_handle:?} (handle {handle}; state {transmit_id:?}); {:?}",
            transmit.read
        );

        if transmit.done {
            bail!("cannot write to future after previous write succeeded or readable end dropped");
        }

        let new_state = if let ReadState::Dropped = &transmit.read {
            ReadState::Dropped
        } else {
            ReadState::Open
        };

        let set_guest_ready = |me: &mut ConcurrentState| {
            let transmit = me.get_mut(transmit_id)?;
            assert!(matches!(&transmit.write, WriteState::Open));
            transmit.write = WriteState::GuestReady {
                ty,
                flat_abi,
                options,
                address,
                count,
                handle,
                post_write: PostWrite::Continue,
            };
            Ok::<_, crate::Error>(())
        };

        let result = match mem::replace(&mut transmit.read, new_state) {
            ReadState::GuestReady {
                ty: read_ty,
                flat_abi: read_flat_abi,
                options: read_options,
                address: read_address,
                count: read_count,
                handle: read_handle,
            } => {
                assert_eq!(flat_abi, read_flat_abi);

                if let TableIndex::Future(_) = ty {
                    transmit.done = true;
                }

                // Note that zero-length reads and writes are handling specially
                // by the spec to allow each end to signal readiness to the
                // other.  Quoting the spec:
                //
                // ```
                // The meaning of a read or write when the length is 0 is that
                // the caller is querying the "readiness" of the other
                // side. When a 0-length read/write rendezvous with a
                // non-0-length read/write, only the 0-length read/write
                // completes; the non-0-length read/write is kept pending (and
                // ready for a subsequent rendezvous).
                //
                // In the corner case where a 0-length read and write
                // rendezvous, only the writer is notified of readiness. To
                // avoid livelock, the Canonical ABI requires that a writer must
                // (eventually) follow a completed 0-length write with a
                // non-0-length write that is allowed to block (allowing the
                // reader end to run and rendezvous with its own non-0-length
                // read).
                // ```

                let write_complete = count == 0 || read_count > 0;
                let read_complete = count > 0;
                let read_buffer_remaining = count < read_count;

                let read_handle_rep = transmit.read_handle.rep();

                let count = count.min(read_count);

                self.copy(
                    store.as_context_mut(),
                    flat_abi,
                    ty,
                    &options,
                    address,
                    read_ty,
                    &read_options,
                    read_address,
                    count,
                    rep,
                )?;

                let instance = self.id().get_mut(store.0);
                let types = instance.component().types();
                let item_size = payload(ty, types)
                    .map(|ty| usize::try_from(types.canonical_abi(&ty).size32).unwrap())
                    .unwrap_or(0);
                let concurrent_state = instance.concurrent_state_mut();
                if read_complete {
                    let count = u32::try_from(count).unwrap();
                    let total = if let Some(Event::StreamRead {
                        code: ReturnCode::Completed(old_total),
                        ..
                    }) = concurrent_state.take_event(read_handle_rep)?
                    {
                        count + old_total
                    } else {
                        count
                    };

                    let code = ReturnCode::completed(ty.kind(), total);

                    concurrent_state.set_event(
                        read_handle_rep,
                        match read_ty {
                            TableIndex::Future(ty) => Event::FutureRead {
                                code,
                                pending: Some((ty, read_handle)),
                            },
                            TableIndex::Stream(ty) => Event::StreamRead {
                                code,
                                pending: Some((ty, read_handle)),
                            },
                        },
                    )?;
                }

                if read_buffer_remaining {
                    let transmit = concurrent_state.get_mut(transmit_id)?;
                    transmit.read = ReadState::GuestReady {
                        ty: read_ty,
                        flat_abi: read_flat_abi,
                        options: read_options,
                        address: read_address + (count * item_size),
                        count: read_count - count,
                        handle: read_handle,
                    };
                }

                if write_complete {
                    ReturnCode::completed(ty.kind(), count.try_into().unwrap())
                } else {
                    set_guest_ready(concurrent_state)?;
                    ReturnCode::Blocked
                }
            }

            ReadState::HostReady { accept } => {
                if let TableIndex::Future(_) = ty {
                    transmit.done = true;
                }

                let lift = &mut LiftContext::new(store.0.store_opaque_mut(), &options, self);
                accept(Writer::Guest {
                    ty: payload(ty, lift.types),
                    lift,
                    address,
                    count,
                })?
            }

            ReadState::Open => {
                set_guest_ready(concurrent_state)?;
                ReturnCode::Blocked
            }

            ReadState::Dropped => {
                if let TableIndex::Future(_) = ty {
                    transmit.done = true;
                }

                ReturnCode::Dropped(0)
            }
        };

        if result != ReturnCode::Blocked {
            let state = self.concurrent_state_mut(store.0);
            *state.get_mut_by_index(ty, handle)?.1 = StreamFutureState::Write {
                done: matches!(
                    (result, ty),
                    (ReturnCode::Dropped(_), TableIndex::Stream(_))
                ),
            };
        }

        Ok(result)
    }

    /// Read from the specified stream or future from the guest.
    pub(super) fn guest_read<T: 'static>(
        self,
        mut store: StoreContextMut<T>,
        ty: TableIndex,
        options: OptionsIndex,
        flat_abi: Option<FlatAbi>,
        handle: u32,
        address: u32,
        count: u32,
    ) -> Result<ReturnCode> {
        let address = usize::try_from(address).unwrap();
        let options = Options::new_index(store.0, self, options);
        if !options.async_() {
            bail!("synchronous stream and future reads not yet supported");
        }
        let concurrent_state = self.concurrent_state_mut(store.0);
        let (rep, state) = concurrent_state.get_mut_by_index(ty, handle)?;
        let StreamFutureState::Read { done } = *state else {
            bail!("invalid handle {handle}; expected `Read`; got {:?}", *state);
        };

        if done {
            bail!("cannot read from stream after being notified that the writable end dropped");
        }

        *state = StreamFutureState::Busy;
        let transmit_handle = TableId::<TransmitHandle>::new(rep);
        let transmit_id = concurrent_state.get(transmit_handle)?.state;
        let transmit = concurrent_state.get_mut(transmit_id)?;
        log::trace!(
            "guest_read {transmit_handle:?} (handle {handle}; state {transmit_id:?}); {:?}",
            transmit.write
        );

        if transmit.done {
            bail!("cannot read from future after previous read succeeded");
        }

        let new_state = if let WriteState::Dropped = &transmit.write {
            WriteState::Dropped
        } else {
            WriteState::Open
        };

        let set_guest_ready = |me: &mut ConcurrentState| {
            let transmit = me.get_mut(transmit_id)?;
            assert!(matches!(&transmit.read, ReadState::Open));
            transmit.read = ReadState::GuestReady {
                ty,
                flat_abi,
                options,
                address,
                count: usize::try_from(count).unwrap(),
                handle,
            };
            Ok::<_, crate::Error>(())
        };

        let result = match mem::replace(&mut transmit.write, new_state) {
            WriteState::GuestReady {
                ty: write_ty,
                flat_abi: write_flat_abi,
                options: write_options,
                address: write_address,
                count: write_count,
                handle: write_handle,
                post_write,
            } => {
                assert_eq!(flat_abi, write_flat_abi);

                if let TableIndex::Future(_) = ty {
                    transmit.done = true;
                }

                let write_handle_rep = transmit.write_handle.rep();

                // See the comment in `guest_write` for the
                // `ReadState::GuestReady` case concerning zero-length reads and
                // writes.

                let count = usize::try_from(count).unwrap();

                let write_complete = write_count == 0 || count > 0;
                let read_complete = write_count > 0;
                let write_buffer_remaining = count < write_count;

                let count = count.min(write_count);

                self.copy(
                    store.as_context_mut(),
                    flat_abi,
                    write_ty,
                    &write_options,
                    write_address,
                    ty,
                    &options,
                    address,
                    count,
                    rep,
                )?;

                let instance = self.id().get_mut(store.0);
                let types = instance.component().types();
                let item_size = payload(ty, types)
                    .map(|ty| usize::try_from(types.canonical_abi(&ty).size32).unwrap())
                    .unwrap_or(0);
                let concurrent_state = instance.concurrent_state_mut();
                let pending = if let PostWrite::Drop = post_write {
                    concurrent_state.get_mut(transmit_id)?.write = WriteState::Dropped;
                    false
                } else {
                    true
                };

                if write_complete {
                    let count = u32::try_from(count).unwrap();
                    let total = if let Some(Event::StreamWrite {
                        code: ReturnCode::Completed(old_total),
                        ..
                    }) = concurrent_state.take_event(write_handle_rep)?
                    {
                        count + old_total
                    } else {
                        count
                    };

                    let code = ReturnCode::completed(ty.kind(), total);

                    concurrent_state.set_event(
                        write_handle_rep,
                        match write_ty {
                            TableIndex::Future(ty) => Event::FutureWrite {
                                code,
                                pending: pending.then_some((ty, write_handle)),
                            },
                            TableIndex::Stream(ty) => Event::StreamWrite {
                                code,
                                pending: pending.then_some((ty, write_handle)),
                            },
                        },
                    )?;
                }

                if write_buffer_remaining {
                    let transmit = concurrent_state.get_mut(transmit_id)?;
                    transmit.write = WriteState::GuestReady {
                        ty: write_ty,
                        flat_abi: write_flat_abi,
                        options: write_options,
                        address: write_address + (count * item_size),
                        count: write_count - count,
                        handle: write_handle,
                        post_write,
                    };
                }

                if read_complete {
                    ReturnCode::completed(ty.kind(), count.try_into().unwrap())
                } else {
                    set_guest_ready(concurrent_state)?;
                    ReturnCode::Blocked
                }
            }

            WriteState::HostReady { accept, post_write } => {
                if let TableIndex::Future(_) = ty {
                    transmit.done = true;
                }

                let code = accept(
                    store.0.traitobj_mut(),
                    self,
                    Reader::Guest {
                        options: &options,
                        ty,
                        address,
                        count: count.try_into().unwrap(),
                    },
                )?;

                if let PostWrite::Drop = post_write {
                    self.concurrent_state_mut(store.0)
                        .get_mut(transmit_id)?
                        .write = WriteState::Dropped;
                }

                code
            }

            WriteState::Open => {
                set_guest_ready(concurrent_state)?;
                ReturnCode::Blocked
            }

            WriteState::Dropped => ReturnCode::Dropped(0),
        };

        if result != ReturnCode::Blocked {
            let state = self.concurrent_state_mut(store.0);
            *state.get_mut_by_index(ty, handle)?.1 = StreamFutureState::Read {
                done: matches!(
                    (result, ty),
                    (ReturnCode::Dropped(_), TableIndex::Stream(_))
                ),
            };
        }

        Ok(result)
    }

    /// Drop the readable end of the specified stream or future from the guest.
    fn guest_drop_readable(
        self,
        store: &mut dyn VMStore,
        ty: TableIndex,
        reader: u32,
    ) -> Result<()> {
        let concurrent_state = self.concurrent_state_mut(store);
        let (rep, state) = concurrent_state.state_table(ty).remove_by_index(reader)?;
        let (state, kind) = match state {
            WaitableState::Stream(_, state) => (state, TransmitKind::Stream),
            WaitableState::Future(_, state) => (state, TransmitKind::Future),
            _ => {
                bail!("invalid stream or future handle");
            }
        };
        match state {
            StreamFutureState::Read { .. } => {}
            StreamFutureState::Write { .. } => {
                bail!("passed write end to `{{stream|future}}.drop-readable`")
            }
            StreamFutureState::Busy => bail!("cannot drop busy stream or future"),
        }
        let id = TableId::<TransmitHandle>::new(rep);
        log::trace!("guest_drop_readable: drop reader {id:?}");
        self.host_drop_reader(store, id, kind)
    }

    /// Create a new error context for the given component.
    pub(crate) fn error_context_new(
        self,
        store: &mut StoreOpaque,
        ty: TypeComponentLocalErrorContextTableIndex,
        options: OptionsIndex,
        debug_msg_address: u32,
        debug_msg_len: u32,
    ) -> Result<u32> {
        let options = Options::new_index(store, self, options);
        let lift_ctx = &mut LiftContext::new(store, &options, self);
        //  Read string from guest memory
        let address = usize::try_from(debug_msg_address)?;
        let len = usize::try_from(debug_msg_len)?;
        lift_ctx
            .memory()
            .get(address..)
            .and_then(|b| b.get(..len))
            .ok_or_else(|| anyhow::anyhow!("invalid debug message pointer: out of bounds"))?;
        let message = WasmStr::new(address, len, lift_ctx)?;

        // Create a new ErrorContext that is tracked along with other concurrent state
        let err_ctx = ErrorContextState {
            debug_msg: message
                .to_str_from_memory(options.memory(store))?
                .to_string(),
        };
        let state = self.concurrent_state_mut(store);
        let table_id = state.push(err_ctx)?;
        let global_ref_count_idx =
            TypeComponentGlobalErrorContextTableIndex::from_u32(table_id.rep());

        // Add to the global error context ref counts
        let _ = state
            .global_error_context_ref_counts
            .insert(global_ref_count_idx, GlobalErrorContextRefCount(1));

        // Error context are tracked both locally (to a single component instance) and globally
        // the counts for both must stay in sync.
        //
        // Here we reflect the newly created global concurrent error context state into the
        // component instance's locally tracked count, along with the appropriate key into the global
        // ref tracking data structures to enable later lookup
        let local_tbl = &mut state.error_context_tables[ty];

        assert!(
            !local_tbl.has_handle(table_id.rep()),
            "newly created error context state already tracked by component"
        );
        let local_idx = local_tbl.insert(table_id.rep(), LocalErrorContextRefCount(1))?;

        Ok(local_idx)
    }

    /// Retrieve the debug message from the specified error context.
    pub(super) fn error_context_debug_message<T>(
        self,
        store: StoreContextMut<T>,
        ty: TypeComponentLocalErrorContextTableIndex,
        options: OptionsIndex,
        err_ctx_handle: u32,
        debug_msg_address: u32,
    ) -> Result<()> {
        // Retrieve the error context and internal debug message
        let state = self.concurrent_state_mut(store.0);
        let (state_table_id_rep, _) = state
            .error_context_tables
            .get_mut(ty)
            .context("error context table index present in (sub)component lookup during debug_msg")?
            .get_mut_by_index(err_ctx_handle)?;

        // Get the state associated with the error context
        let ErrorContextState { debug_msg } =
            state.get_mut(TableId::<ErrorContextState>::new(state_table_id_rep))?;
        let debug_msg = debug_msg.clone();

        let options = Options::new_index(store.0, self, options);
        let types = self.id().get(store.0).component().types().clone();
        let lower_cx = &mut LowerContext::new(store, &options, &types, self);
        let debug_msg_address = usize::try_from(debug_msg_address)?;
        // Lower the string into the component's memory
        let offset = lower_cx
            .as_slice_mut()
            .get(debug_msg_address..)
            .and_then(|b| b.get(..debug_msg.bytes().len()))
            .map(|_| debug_msg_address)
            .ok_or_else(|| anyhow::anyhow!("invalid debug message pointer: out of bounds"))?;
        debug_msg
            .as_str()
            .linear_lower_to_memory(lower_cx, InterfaceType::String, offset)?;

        Ok(())
    }

    /// Implements the `future.drop-readable` intrinsic.
    pub(crate) fn future_drop_readable(
        self,
        store: &mut dyn VMStore,
        ty: TypeFutureTableIndex,
        reader: u32,
    ) -> Result<()> {
        self.guest_drop_readable(store, TableIndex::Future(ty), reader)
    }

    /// Implements the `stream.drop-readable` intrinsic.
    pub(crate) fn stream_drop_readable(
        self,
        store: &mut dyn VMStore,
        ty: TypeStreamTableIndex,
        reader: u32,
    ) -> Result<()> {
        self.guest_drop_readable(store, TableIndex::Stream(ty), reader)
    }
}

impl ConcurrentState {
    fn take_event(&mut self, waitable: u32) -> Result<Option<Event>> {
        Waitable::Transmit(TableId::<TransmitHandle>::new(waitable)).take_event(self)
    }

    fn set_event(&mut self, waitable: u32, event: Event) -> Result<()> {
        Waitable::Transmit(TableId::<TransmitHandle>::new(waitable)).set_event(self, Some(event))
    }

    /// Set or update the event for the specified waitable.
    ///
    /// If there is already an event set for this waitable, we assert that it is
    /// of the same variant as the new one and reuse the `ReturnCode` count and
    /// the `pending` field if applicable.
    // TODO: This is a bit awkward due to how
    // `Event::{Stream,Future}{Write,Read}` and
    // `ReturnCode::{Completed,Dropped,Cancelled}` are currently represented.
    // Consider updating those representations in a way that allows this
    // function to be simplified.
    fn update_event(&mut self, waitable: u32, event: Event) -> Result<()> {
        let waitable = Waitable::Transmit(TableId::<TransmitHandle>::new(waitable));

        fn update_code(old: ReturnCode, new: ReturnCode) -> ReturnCode {
            let (ReturnCode::Completed(count)
            | ReturnCode::Dropped(count)
            | ReturnCode::Cancelled(count)) = old
            else {
                unreachable!()
            };

            match new {
                ReturnCode::Dropped(0) => ReturnCode::Dropped(count),
                ReturnCode::Cancelled(0) => ReturnCode::Cancelled(count),
                _ => unreachable!(),
            }
        }

        let event = match (waitable.take_event(self)?, event) {
            (None, _) => event,
            (Some(old @ Event::FutureWrite { .. }), Event::FutureWrite { .. }) => old,
            (Some(old @ Event::FutureRead { .. }), Event::FutureRead { .. }) => old,
            (
                Some(Event::StreamWrite {
                    code: old_code,
                    pending: old_pending,
                }),
                Event::StreamWrite { code, pending },
            ) => Event::StreamWrite {
                code: update_code(old_code, code),
                pending: old_pending.or(pending),
            },
            (
                Some(Event::StreamRead {
                    code: old_code,
                    pending: old_pending,
                }),
                Event::StreamRead { code, pending },
            ) => Event::StreamRead {
                code: update_code(old_code, code),
                pending: old_pending.or(pending),
            },
            _ => unreachable!(),
        };

        waitable.set_event(self, Some(event))
    }

    fn get_mut_by_index(
        &mut self,
        ty: TableIndex,
        index: u32,
    ) -> Result<(u32, &mut StreamFutureState)> {
        get_mut_by_index_from(self.state_table(ty), ty, index)
    }

    /// Allocate a new future or stream, including the `TransmitState` and the
    /// `TransmitHandle`s corresponding to the read and write ends.
    fn new_transmit(&mut self) -> Result<(TableId<TransmitHandle>, TableId<TransmitHandle>)> {
        let state_id = self.push(TransmitState::default())?;

        let write = self.push(TransmitHandle::new(state_id))?;
        let read = self.push(TransmitHandle::new(state_id))?;

        let state = self.get_mut(state_id)?;
        state.write_handle = write;
        state.read_handle = read;

        log::trace!("new transmit: state {state_id:?}; write {write:?}; read {read:?}",);

        Ok((write, read))
    }

    /// Delete the specified future or stream, including the read and write ends.
    fn delete_transmit(&mut self, state_id: TableId<TransmitState>) -> Result<()> {
        let state = self.delete(state_id)?;
        self.delete(state.write_handle)?;
        self.delete(state.read_handle)?;

        log::trace!(
            "delete transmit: state {state_id:?}; write {:?}; read {:?}",
            state.write_handle,
            state.read_handle,
        );

        Ok(())
    }

    fn state_table(&mut self, ty: TableIndex) -> &mut StateTable<WaitableState> {
        let runtime_instance = match ty {
            TableIndex::Stream(ty) => self.component.types()[ty].instance,
            TableIndex::Future(ty) => self.component.types()[ty].instance,
        };
        &mut self.waitable_tables[runtime_instance]
    }

    /// Allocate a new future or stream and grant ownership of both the read and
    /// write ends to the (sub-)component instance to which the specified
    /// `TableIndex` belongs.
    fn guest_new(&mut self, ty: TableIndex) -> Result<ResourcePair> {
        let (write, read) = self.new_transmit()?;
        let read = self.state_table(ty).insert(
            read.rep(),
            waitable_state(ty, StreamFutureState::Read { done: false }),
        )?;
        let write = self.state_table(ty).insert(
            write.rep(),
            waitable_state(ty, StreamFutureState::Write { done: false }),
        )?;
        Ok(ResourcePair { write, read })
    }

    /// Cancel a pending stream or future write from the host.
    ///
    /// # Arguments
    ///
    /// * `rep` - The `TransmitState` rep for the stream or future.
    fn host_cancel_write(&mut self, rep: u32) -> Result<ReturnCode> {
        let transmit_id = TableId::<TransmitState>::new(rep);
        let transmit = self.get_mut(transmit_id)?;
        log::trace!(
            "host_cancel_write state {transmit_id:?}; write state {:?} read state {:?}",
            transmit.read,
            transmit.write
        );

        let code = if let Some(event) =
            Waitable::Transmit(transmit.write_handle).take_event(self)?
        {
            let (Event::FutureWrite { code, .. } | Event::StreamWrite { code, .. }) = event else {
                unreachable!();
            };
            match (code, event) {
                (ReturnCode::Completed(count), Event::StreamWrite { .. }) => {
                    ReturnCode::Cancelled(count)
                }
                (ReturnCode::Dropped(_) | ReturnCode::Completed(_), _) => code,
                _ => unreachable!(),
            }
        } else {
            ReturnCode::Cancelled(0)
        };

        let transmit = self.get_mut(transmit_id)?;

        match &transmit.write {
            WriteState::GuestReady { .. } | WriteState::HostReady { .. } => {
                transmit.write = WriteState::Open;
            }

            WriteState::Open | WriteState::Dropped => {}
        }

        log::trace!("cancelled write {transmit_id:?}");

        Ok(code)
    }

    /// Cancel a pending stream or future read from the host.
    ///
    /// # Arguments
    ///
    /// * `rep` - The `TransmitState` rep for the stream or future.
    fn host_cancel_read(&mut self, rep: u32) -> Result<ReturnCode> {
        let transmit_id = TableId::<TransmitState>::new(rep);
        let transmit = self.get_mut(transmit_id)?;
        log::trace!(
            "host_cancel_read state {transmit_id:?}; read state {:?} write state {:?}",
            transmit.read,
            transmit.write
        );

        let code = if let Some(event) = Waitable::Transmit(transmit.read_handle).take_event(self)? {
            let (Event::FutureRead { code, .. } | Event::StreamRead { code, .. }) = event else {
                unreachable!();
            };
            match (code, event) {
                (ReturnCode::Completed(count), Event::StreamRead { .. }) => {
                    ReturnCode::Cancelled(count)
                }
                (ReturnCode::Dropped(_) | ReturnCode::Completed(_), _) => code,
                _ => unreachable!(),
            }
        } else {
            ReturnCode::Cancelled(0)
        };

        let transmit = self.get_mut(transmit_id)?;

        match &transmit.read {
            ReadState::GuestReady { .. } | ReadState::HostReady { .. } => {
                transmit.read = ReadState::Open;
            }

            ReadState::Open | ReadState::Dropped => {}
        }

        log::trace!("cancelled read {transmit_id:?}");

        Ok(code)
    }

    /// Cancel a pending write for the specified stream or future from the guest.
    fn guest_cancel_write(
        &mut self,
        ty: TableIndex,
        writer: u32,
        _async_: bool,
    ) -> Result<ReturnCode> {
        let (rep, WaitableState::Stream(_, state) | WaitableState::Future(_, state)) =
            self.state_table(ty).get_mut_by_index(writer)?
        else {
            bail!("invalid stream or future handle");
        };
        let id = TableId::<TransmitHandle>::new(rep);
        log::trace!("guest cancel write {id:?} (handle {writer})");
        match state {
            StreamFutureState::Write { .. } => {
                bail!("stream or future write cancelled when no write is pending")
            }
            StreamFutureState::Read { .. } => {
                bail!("passed read end to `{{stream|future}}.cancel-write`")
            }
            StreamFutureState::Busy => {
                *state = StreamFutureState::Write { done: false };
            }
        }
        let rep = self.get(id)?.state.rep();
        self.host_cancel_write(rep)
    }

    /// Cancel a pending read for the specified stream or future from the guest.
    fn guest_cancel_read(
        &mut self,
        ty: TableIndex,
        reader: u32,
        _async_: bool,
    ) -> Result<ReturnCode> {
        let (rep, WaitableState::Stream(_, state) | WaitableState::Future(_, state)) =
            self.state_table(ty).get_mut_by_index(reader)?
        else {
            bail!("invalid stream or future handle");
        };
        let id = TableId::<TransmitHandle>::new(rep);
        log::trace!("guest cancel read {id:?} (handle {reader})");
        match state {
            StreamFutureState::Read { .. } => {
                bail!("stream or future read cancelled when no read is pending")
            }
            StreamFutureState::Write { .. } => {
                bail!("passed write end to `{{stream|future}}.cancel-read`")
            }
            StreamFutureState::Busy => {
                *state = StreamFutureState::Read { done: false };
            }
        }
        let rep = self.get(id)?.state.rep();
        self.host_cancel_read(rep)
    }

    /// Drop the specified error context.
    pub(crate) fn error_context_drop(
        &mut self,
        ty: TypeComponentLocalErrorContextTableIndex,
        error_context: u32,
    ) -> Result<()> {
        let local_state_table = self
            .error_context_tables
            .get_mut(ty)
            .context("error context table index present in (sub)component table during drop")?;

        // Reduce the local (sub)component ref count, removing tracking if necessary
        let (rep, local_ref_removed) = {
            let (rep, LocalErrorContextRefCount(local_ref_count)) =
                local_state_table.get_mut_by_index(error_context)?;
            assert!(*local_ref_count > 0);
            *local_ref_count -= 1;
            let mut local_ref_removed = false;
            if *local_ref_count == 0 {
                local_ref_removed = true;
                local_state_table
                    .remove_by_index(error_context)
                    .context("removing error context from component-local tracking")?;
            }
            (rep, local_ref_removed)
        };

        let global_ref_count_idx = TypeComponentGlobalErrorContextTableIndex::from_u32(rep);

        let GlobalErrorContextRefCount(global_ref_count) = self
            .global_error_context_ref_counts
            .get_mut(&global_ref_count_idx)
            .expect("retrieve concurrent state for error context during drop");

        // Reduce the component-global ref count, removing tracking if necessary
        assert!(*global_ref_count >= 1);
        *global_ref_count -= 1;
        if *global_ref_count == 0 {
            assert!(local_ref_removed);

            self.global_error_context_ref_counts
                .remove(&global_ref_count_idx);

            self.delete(TableId::<ErrorContextState>::new(rep))
                .context("deleting component-global error context data")?;
        }

        Ok(())
    }

    /// Transfer ownership of the specified stream or future read end from one
    /// guest to another.
    fn guest_transfer<U: PartialEq + Eq + std::fmt::Debug>(
        &mut self,
        src_idx: u32,
        src: U,
        src_instance: RuntimeComponentInstanceIndex,
        dst: U,
        dst_instance: RuntimeComponentInstanceIndex,
        match_state: impl Fn(&mut WaitableState) -> Result<(U, &mut StreamFutureState)>,
        make_state: impl Fn(U, StreamFutureState) -> WaitableState,
    ) -> Result<u32> {
        let src_table = &mut self.waitable_tables[src_instance];
        let (_rep, src_state) = src_table.get_mut_by_index(src_idx)?;
        let (src_ty, _) = match_state(src_state)?;
        if src_ty != src {
            bail!("invalid future handle");
        }

        let src_table = &mut self.waitable_tables[src_instance];
        let (rep, src_state) = src_table.get_mut_by_index(src_idx)?;
        let (_, src_state) = match_state(src_state)?;

        match src_state {
            StreamFutureState::Read { done: true } => {
                bail!("cannot lift stream after being notified that the writable end dropped")
            }
            StreamFutureState::Read { done: false } => {
                src_table.remove_by_index(src_idx)?;

                let dst_table = &mut self.waitable_tables[dst_instance];
                dst_table.insert(
                    rep,
                    make_state(dst, StreamFutureState::Read { done: false }),
                )
            }
            StreamFutureState::Write { .. } => {
                bail!("cannot transfer write end of stream or future")
            }
            StreamFutureState::Busy => bail!("cannot transfer busy stream or future"),
        }
    }

    /// Implements the `future.new` intrinsic.
    pub(crate) fn future_new(&mut self, ty: TypeFutureTableIndex) -> Result<ResourcePair> {
        self.guest_new(TableIndex::Future(ty))
    }

    /// Implements the `future.cancel-write` intrinsic.
    pub(crate) fn future_cancel_write(
        &mut self,
        ty: TypeFutureTableIndex,
        async_: bool,
        writer: u32,
    ) -> Result<u32> {
        self.guest_cancel_write(TableIndex::Future(ty), writer, async_)
            .map(|result| result.encode())
    }

    /// Implements the `future.cancel-read` intrinsic.
    pub(crate) fn future_cancel_read(
        &mut self,
        ty: TypeFutureTableIndex,
        async_: bool,
        reader: u32,
    ) -> Result<u32> {
        self.guest_cancel_read(TableIndex::Future(ty), reader, async_)
            .map(|result| result.encode())
    }

    /// Implements the `stream.new` intrinsic.
    pub(crate) fn stream_new(&mut self, ty: TypeStreamTableIndex) -> Result<ResourcePair> {
        self.guest_new(TableIndex::Stream(ty))
    }

    /// Implements the `stream.cancel-write` intrinsic.
    pub(crate) fn stream_cancel_write(
        &mut self,
        ty: TypeStreamTableIndex,
        async_: bool,
        writer: u32,
    ) -> Result<u32> {
        self.guest_cancel_write(TableIndex::Stream(ty), writer, async_)
            .map(|result| result.encode())
    }

    /// Implements the `stream.cancel-read` intrinsic.
    pub(crate) fn stream_cancel_read(
        &mut self,
        ty: TypeStreamTableIndex,
        async_: bool,
        reader: u32,
    ) -> Result<u32> {
        self.guest_cancel_read(TableIndex::Stream(ty), reader, async_)
            .map(|result| result.encode())
    }

    /// Transfer ownership of the specified future read end from one guest to
    /// another.
    pub(crate) fn future_transfer(
        &mut self,
        src_idx: u32,
        src: TypeFutureTableIndex,
        dst: TypeFutureTableIndex,
    ) -> Result<u32> {
        self.guest_transfer(
            src_idx,
            src,
            self.component.types()[src].instance,
            dst,
            self.component.types()[dst].instance,
            |state| {
                if let WaitableState::Future(ty, state) = state {
                    Ok((*ty, state))
                } else {
                    Err(anyhow!("invalid future handle"))
                }
            },
            WaitableState::Future,
        )
    }

    /// Transfer ownership of the specified stream read end from one guest to
    /// another.
    pub(crate) fn stream_transfer(
        &mut self,
        src_idx: u32,
        src: TypeStreamTableIndex,
        dst: TypeStreamTableIndex,
    ) -> Result<u32> {
        self.guest_transfer(
            src_idx,
            src,
            self.component.types()[src].instance,
            dst,
            self.component.types()[dst].instance,
            |state| {
                if let WaitableState::Stream(ty, state) = state {
                    Ok((*ty, state))
                } else {
                    Err(anyhow!("invalid stream handle"))
                }
            },
            WaitableState::Stream,
        )
    }

    /// Copy the specified error context from one component to another.
    pub(crate) fn error_context_transfer(
        &mut self,
        src_idx: u32,
        src: TypeComponentLocalErrorContextTableIndex,
        dst: TypeComponentLocalErrorContextTableIndex,
    ) -> Result<u32> {
        let (rep, _) = {
            let rep = self
                .error_context_tables
                .get_mut(src)
                .context("error context table index present in (sub)component lookup")?
                .get_mut_by_index(src_idx)?;
            rep
        };
        let dst = self
            .error_context_tables
            .get_mut(dst)
            .context("error context table index present in (sub)component lookup")?;

        // Update the component local for the destination
        let updated_count = if let Some((dst_idx, count)) = dst.get_mut_by_rep(rep) {
            (*count).0 += 1;
            dst_idx
        } else {
            dst.insert(rep, LocalErrorContextRefCount(1))?
        };

        // Update the global (cross-subcomponent) count for error contexts
        // as the new component has essentially created a new reference that will
        // be dropped/handled independently
        let global_ref_count = self
            .global_error_context_ref_counts
            .get_mut(&TypeComponentGlobalErrorContextTableIndex::from_u32(rep))
            .context("global ref count present for existing (sub)component error context")?;
        global_ref_count.0 += 1;

        Ok(updated_count)
    }
}

pub(crate) struct ResourcePair {
    pub(crate) write: u32,
    pub(crate) read: u32,
}
