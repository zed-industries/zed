use crate::limits::MAX_WASM_CANONICAL_OPTIONS;
use crate::prelude::*;
use crate::{BinaryReader, ComponentValType, FromReader, Result, SectionLimited};

/// Represents options for component functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalOption {
    /// The string types in the function signature are UTF-8 encoded.
    UTF8,
    /// The string types in the function signature are UTF-16 encoded.
    UTF16,
    /// The string types in the function signature are compact UTF-16 encoded.
    CompactUTF16,
    /// The memory to use if the lifting or lowering of a function requires memory access.
    ///
    /// The value is an index to a core memory.
    Memory(u32),
    /// The realloc function to use if the lifting or lowering of a function requires memory
    /// allocation.
    ///
    /// The value is an index to a core function of type `(func (param i32 i32 i32 i32) (result i32))`.
    Realloc(u32),
    /// The post-return function to use if the lifting of a function requires
    /// cleanup after the function returns.
    PostReturn(u32),
    /// Indicates that specified function should be lifted or lowered using the `async` ABI.
    Async,
    /// The function to use if the async lifting of a function should receive task/stream/future progress events
    /// using a callback.
    Callback(u32),
    /// The core function type to lower this component function to.
    CoreType(u32),
    /// Use the GC version of the canonical ABI.
    Gc,
}

/// Represents a canonical function in a WebAssembly component.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CanonicalFunction {
    /// The function lifts a core WebAssembly function to the canonical ABI.
    Lift {
        /// The index of the core WebAssembly function to lift.
        core_func_index: u32,
        /// The index of the lifted function's type.
        type_index: u32,
        /// The canonical options for the function.
        options: Box<[CanonicalOption]>,
    },
    /// The function lowers a canonical ABI function to a core WebAssembly function.
    Lower {
        /// The index of the function to lower.
        func_index: u32,
        /// The canonical options for the function.
        options: Box<[CanonicalOption]>,
    },
    /// A function which creates a new owned handle to a resource.
    ResourceNew {
        /// The type index of the resource that's being created.
        resource: u32,
    },
    /// A function which is used to drop resource handles of the specified type.
    ResourceDrop {
        /// The type index of the resource that's being dropped.
        resource: u32,
    },
    /// Same as `ResourceDrop`, but implements the `async` ABI.
    ResourceDropAsync {
        /// The type index of the resource that's being dropped.
        resource: u32,
    },
    /// A function which returns the underlying i32-based representation of the
    /// specified resource.
    ResourceRep {
        /// The type index of the resource that's being accessed.
        resource: u32,
    },
    /// A function which spawns a new thread by invoking the shared function.
    ThreadSpawnRef {
        /// The index of the function type to spawn.
        func_ty_index: u32,
    },
    /// A function which spawns a new thread by invoking the shared function
    /// passed as an index into a `funcref` table.
    ThreadSpawnIndirect {
        /// The index of the function type to spawn.
        func_ty_index: u32,
        /// The index of the table to use for the indirect spawn.
        table_index: u32,
    },
    /// A function which returns the number of threads that can be expected to
    /// execute concurrently
    ThreadAvailableParallelism,
    /// A function which tells the host to enable backpressure by incrementing
    /// the component's counter by 1.
    BackpressureInc,
    /// A function which tells the host to disable backpressure by decrementing
    /// the component's counter by 1.
    BackpressureDec,
    /// A function which returns a result to the caller of a lifted export
    /// function.  This allows the callee to continue executing after returning
    /// a result.
    TaskReturn {
        /// The result type, if any.
        result: Option<ComponentValType>,
        /// The canonical options for the function.
        options: Box<[CanonicalOption]>,
    },
    /// A function to acknowledge cancellation of the current task.
    TaskCancel,
    /// A `context.get` intrinsic for the `i`th slot of task-local storage.
    ContextGet(u32),
    /// A `context.set` intrinsic for the `i`th slot of task-local storage.
    ContextSet(u32),
    /// A function which yields control to the host so that other tasks are able
    /// to make progress, if any.
    ThreadYield {
        /// If `true`, indicates the caller instance maybe reentered.
        cancellable: bool,
    },
    /// A function to drop a specified task which has completed.
    SubtaskDrop,
    /// A function to cancel an in-progress task.
    SubtaskCancel {
        /// If `false`, block until cancellation completes rather than return
        /// `BLOCKED`.
        async_: bool,
    },
    /// A function to create a new `stream` handle of the specified type.
    StreamNew {
        /// The `stream` type to instantiate.
        ty: u32,
    },
    /// A function to read from a `stream` of the specified type.
    StreamRead {
        /// The `stream` type to expect.
        ty: u32,
        /// Any options (e.g. string encoding) to use when storing values to
        /// memory.
        options: Box<[CanonicalOption]>,
    },
    /// A function to write to a `stream` of the specified type.
    StreamWrite {
        /// The `stream` type to expect.
        ty: u32,
        /// Any options (e.g. string encoding) to use when loading values from
        /// memory.
        options: Box<[CanonicalOption]>,
    },
    /// A function to cancel an in-progress read from a `stream` of the
    /// specified type.
    StreamCancelRead {
        /// The `stream` type to expect.
        ty: u32,
        /// If `false`, block until cancellation completes rather than return
        /// `BLOCKED`.
        async_: bool,
    },
    /// A function to cancel an in-progress write to a `stream` of the specified
    /// type.
    StreamCancelWrite {
        /// The `stream` type to expect.
        ty: u32,
        /// If `false`, block until cancellation completes rather than return
        /// `BLOCKED`.
        async_: bool,
    },
    /// A function to drop the readable end of a `stream` of the specified
    /// type.
    StreamDropReadable {
        /// The `stream` type to expect.
        ty: u32,
    },
    /// A function to drop the writable end of a `stream` of the specified
    /// type.
    StreamDropWritable {
        /// The `stream` type to expect.
        ty: u32,
    },
    /// A function to create a new `future` handle of the specified type.
    FutureNew {
        /// The `future` type to instantiate.
        ty: u32,
    },
    /// A function to read from a `future` of the specified type.
    FutureRead {
        /// The `future` type to expect.
        ty: u32,
        /// Any options (e.g. string encoding) to use when storing values to
        /// memory.
        options: Box<[CanonicalOption]>,
    },
    /// A function to write to a `future` of the specified type.
    FutureWrite {
        /// The `future` type to expect.
        ty: u32,
        /// Any options (e.g. string encoding) to use when loading values from
        /// memory.
        options: Box<[CanonicalOption]>,
    },
    /// A function to cancel an in-progress read from a `future` of the
    /// specified type.
    FutureCancelRead {
        /// The `future` type to expect.
        ty: u32,
        /// If `false`, block until cancellation completes rather than return
        /// `BLOCKED`.
        async_: bool,
    },
    /// A function to cancel an in-progress write to a `future` of the specified
    /// type.
    FutureCancelWrite {
        /// The `future` type to expect.
        ty: u32,
        /// If `false`, block until cancellation completes rather than return
        /// `BLOCKED`.
        async_: bool,
    },
    /// A function to drop the readable end of a `future` of the specified
    /// type.
    FutureDropReadable {
        /// The `future` type to expect.
        ty: u32,
    },
    /// A function to drop the writable end of a `future` of the specified
    /// type.
    FutureDropWritable {
        /// The `future` type to expect.
        ty: u32,
    },
    /// A function to create a new `error-context` with a specified debug
    /// message.
    ErrorContextNew {
        /// String encoding, memory, etc. to use when loading debug message.
        options: Box<[CanonicalOption]>,
    },
    /// A function to get the debug message for a specified `error-context`.
    ///
    /// Note that the debug message might not necessarily match what was passed
    /// to `error.new`.
    ErrorContextDebugMessage {
        /// String encoding, memory, etc. to use when storing debug message.
        options: Box<[CanonicalOption]>,
    },
    /// A function to drop a specified `error-context`.
    ErrorContextDrop,
    /// A function to create a new `waitable-set`.
    WaitableSetNew,
    /// A function to block on the next item within a `waitable-set`.
    WaitableSetWait {
        /// Whether or not the guest can be reentered while calling this
        /// function.
        cancellable: bool,
        /// Which memory the results of this operation are stored in.
        memory: u32,
    },
    /// A function to check if any items are ready within a `waitable-set`.
    WaitableSetPoll {
        /// Whether or not the guest can be reentered while calling this
        /// function.
        cancellable: bool,
        /// Which memory the results of this operation are stored in.
        memory: u32,
    },
    /// A function to drop a `waitable-set`.
    WaitableSetDrop,
    /// A function to add an item to a `waitable-set`.
    WaitableJoin,
    /// A function to get the index of the current thread.
    ThreadIndex,
    /// A function to create a new thread with the specified start function.
    ThreadNewIndirect {
        /// The index of the function type to use as the start function.
        func_ty_index: u32,
        /// The index of the table to use.
        table_index: u32,
    },
    /// A function to suspend the current thread and switch to the given thread.
    ThreadSwitchTo {
        /// Whether or not the thread can be cancelled while awaiting resumption.
        cancellable: bool,
    },
    /// A function to suspend the current thread, immediately yielding to any transitive async-lowered calling component.
    ThreadSuspend {
        /// Whether or not the thread can be cancelled while suspended.
        cancellable: bool,
    },
    /// A function to schedule the given thread to be resumed later.
    ThreadResumeLater,
    /// A function to suspend the current thread and switch to the given thread.
    ThreadYieldTo {
        /// Whether or not the thread can be cancelled while yielding.
        cancellable: bool,
    },
}

/// A reader for the canonical section of a WebAssembly component.
pub type ComponentCanonicalSectionReader<'a> = SectionLimited<'a, CanonicalFunction>;

impl<'a> FromReader<'a> for CanonicalFunction {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<CanonicalFunction> {
        Ok(match reader.read_u8()? {
            0x00 => match reader.read_u8()? {
                0x00 => CanonicalFunction::Lift {
                    core_func_index: reader.read_var_u32()?,
                    options: read_opts(reader)?,
                    type_index: reader.read_var_u32()?,
                },
                x => return reader.invalid_leading_byte(x, "canonical function lift"),
            },
            0x01 => match reader.read_u8()? {
                0x00 => CanonicalFunction::Lower {
                    func_index: reader.read_var_u32()?,
                    options: read_opts(reader)?,
                },
                x => return reader.invalid_leading_byte(x, "canonical function lower"),
            },
            0x02 => CanonicalFunction::ResourceNew {
                resource: reader.read()?,
            },
            0x03 => CanonicalFunction::ResourceDrop {
                resource: reader.read()?,
            },
            0x07 => CanonicalFunction::ResourceDropAsync {
                resource: reader.read()?,
            },
            0x04 => CanonicalFunction::ResourceRep {
                resource: reader.read()?,
            },
            0x24 => CanonicalFunction::BackpressureInc,
            0x25 => CanonicalFunction::BackpressureDec,
            0x09 => CanonicalFunction::TaskReturn {
                result: crate::read_resultlist(reader)?,
                options: read_opts(reader)?,
            },
            0x0a => match reader.read_u8()? {
                0x7f => CanonicalFunction::ContextGet(reader.read_var_u32()?),
                x => return reader.invalid_leading_byte(x, "context.get intrinsic type"),
            },
            0x0b => match reader.read_u8()? {
                0x7f => CanonicalFunction::ContextSet(reader.read_var_u32()?),
                x => return reader.invalid_leading_byte(x, "context.set intrinsic type"),
            },
            0x0c => CanonicalFunction::ThreadYield {
                cancellable: reader.read()?,
            },
            0x0d => CanonicalFunction::SubtaskDrop,
            0x0e => CanonicalFunction::StreamNew { ty: reader.read()? },
            0x0f => CanonicalFunction::StreamRead {
                ty: reader.read()?,
                options: read_opts(reader)?,
            },
            0x10 => CanonicalFunction::StreamWrite {
                ty: reader.read()?,
                options: read_opts(reader)?,
            },
            0x11 => CanonicalFunction::StreamCancelRead {
                ty: reader.read()?,
                async_: reader.read()?,
            },
            0x12 => CanonicalFunction::StreamCancelWrite {
                ty: reader.read()?,
                async_: reader.read()?,
            },
            0x13 => CanonicalFunction::StreamDropReadable { ty: reader.read()? },
            0x14 => CanonicalFunction::StreamDropWritable { ty: reader.read()? },
            0x15 => CanonicalFunction::FutureNew { ty: reader.read()? },
            0x16 => CanonicalFunction::FutureRead {
                ty: reader.read()?,
                options: read_opts(reader)?,
            },
            0x17 => CanonicalFunction::FutureWrite {
                ty: reader.read()?,
                options: read_opts(reader)?,
            },
            0x18 => CanonicalFunction::FutureCancelRead {
                ty: reader.read()?,
                async_: reader.read()?,
            },
            0x19 => CanonicalFunction::FutureCancelWrite {
                ty: reader.read()?,
                async_: reader.read()?,
            },
            0x1a => CanonicalFunction::FutureDropReadable { ty: reader.read()? },
            0x1b => CanonicalFunction::FutureDropWritable { ty: reader.read()? },
            0x1c => CanonicalFunction::ErrorContextNew {
                options: read_opts(reader)?,
            },
            0x1d => CanonicalFunction::ErrorContextDebugMessage {
                options: read_opts(reader)?,
            },
            0x1e => CanonicalFunction::ErrorContextDrop,

            0x1f => CanonicalFunction::WaitableSetNew,
            0x20 => CanonicalFunction::WaitableSetWait {
                cancellable: reader.read()?,
                memory: reader.read()?,
            },
            0x21 => CanonicalFunction::WaitableSetPoll {
                cancellable: reader.read()?,
                memory: reader.read()?,
            },
            0x22 => CanonicalFunction::WaitableSetDrop,
            0x23 => CanonicalFunction::WaitableJoin,
            0x26 => CanonicalFunction::ThreadIndex,
            0x27 => CanonicalFunction::ThreadNewIndirect {
                func_ty_index: reader.read()?,
                table_index: reader.read()?,
            },
            0x28 => CanonicalFunction::ThreadSwitchTo {
                cancellable: reader.read()?,
            },
            0x29 => CanonicalFunction::ThreadSuspend {
                cancellable: reader.read()?,
            },
            0x2a => CanonicalFunction::ThreadResumeLater,
            0x2b => CanonicalFunction::ThreadYieldTo {
                cancellable: reader.read()?,
            },
            0x06 => CanonicalFunction::SubtaskCancel {
                async_: reader.read()?,
            },
            0x05 => CanonicalFunction::TaskCancel,
            0x40 => CanonicalFunction::ThreadSpawnRef {
                func_ty_index: reader.read()?,
            },
            0x41 => CanonicalFunction::ThreadSpawnIndirect {
                func_ty_index: reader.read()?,
                table_index: reader.read()?,
            },
            0x42 => CanonicalFunction::ThreadAvailableParallelism,
            x => return reader.invalid_leading_byte(x, "canonical function"),
        })
    }
}

fn read_opts(reader: &mut BinaryReader<'_>) -> Result<Box<[CanonicalOption]>> {
    reader
        .read_iter(MAX_WASM_CANONICAL_OPTIONS, "canonical options")?
        .collect::<Result<_>>()
}

impl<'a> FromReader<'a> for CanonicalOption {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(match reader.read_u8()? {
            0x00 => CanonicalOption::UTF8,
            0x01 => CanonicalOption::UTF16,
            0x02 => CanonicalOption::CompactUTF16,
            0x03 => CanonicalOption::Memory(reader.read_var_u32()?),
            0x04 => CanonicalOption::Realloc(reader.read_var_u32()?),
            0x05 => CanonicalOption::PostReturn(reader.read_var_u32()?),
            0x06 => CanonicalOption::Async,
            0x07 => CanonicalOption::Callback(reader.read_var_u32()?),
            0x08 => CanonicalOption::CoreType(reader.read_var_u32()?),
            0x09 => CanonicalOption::Gc,
            x => return reader.invalid_leading_byte(x, "canonical option"),
        })
    }
}
