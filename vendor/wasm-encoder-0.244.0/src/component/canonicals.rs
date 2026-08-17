use crate::{ComponentSection, ComponentSectionId, ComponentValType, Encode, encode_section};
use alloc::vec::Vec;

/// Represents options for canonical function definitions.
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
    /// The core function type to lower a component function into.
    CoreType(u32),
    /// Use the GC variant of the canonical ABI.
    Gc,
}

impl Encode for CanonicalOption {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            Self::UTF8 => sink.push(0x00),
            Self::UTF16 => sink.push(0x01),
            Self::CompactUTF16 => sink.push(0x02),
            Self::Memory(idx) => {
                sink.push(0x03);
                idx.encode(sink);
            }
            Self::Realloc(idx) => {
                sink.push(0x04);
                idx.encode(sink);
            }
            Self::PostReturn(idx) => {
                sink.push(0x05);
                idx.encode(sink);
            }
            Self::Async => {
                sink.push(0x06);
            }
            Self::Callback(idx) => {
                sink.push(0x07);
                idx.encode(sink);
            }
            Self::CoreType(idx) => {
                sink.push(0x08);
                idx.encode(sink);
            }
            Self::Gc => {
                sink.push(0x09);
            }
        }
    }
}

/// An encoder for the canonical function section of WebAssembly components.
///
/// # Example
///
/// ```
/// use wasm_encoder::{Component, CanonicalFunctionSection, CanonicalOption};
///
/// let mut functions = CanonicalFunctionSection::new();
/// functions.lift(0, 0, [CanonicalOption::UTF8]);
///
/// let mut component = Component::new();
/// component.section(&functions);
///
/// let bytes = component.finish();
/// ```
#[derive(Clone, Debug, Default)]
pub struct CanonicalFunctionSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl CanonicalFunctionSection {
    /// Construct a new component function section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of functions in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define a function that will lift a core WebAssembly function to the canonical ABI.
    pub fn lift<O>(&mut self, core_func_index: u32, type_index: u32, options: O) -> &mut Self
    where
        O: IntoIterator<Item = CanonicalOption>,
        O::IntoIter: ExactSizeIterator,
    {
        self.bytes.push(0x00);
        self.bytes.push(0x00);
        core_func_index.encode(&mut self.bytes);
        self.encode_options(options);
        type_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Define a function that will lower a canonical ABI function to a core WebAssembly function.
    pub fn lower<O>(&mut self, func_index: u32, options: O) -> &mut Self
    where
        O: IntoIterator<Item = CanonicalOption>,
        O::IntoIter: ExactSizeIterator,
    {
        self.bytes.push(0x01);
        self.bytes.push(0x00);
        func_index.encode(&mut self.bytes);
        self.encode_options(options);
        self.num_added += 1;
        self
    }

    /// Defines a function which will create an owned handle to the resource
    /// specified by `ty_index`.
    pub fn resource_new(&mut self, ty_index: u32) -> &mut Self {
        self.bytes.push(0x02);
        ty_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function which will drop the specified type of handle.
    pub fn resource_drop(&mut self, ty_index: u32) -> &mut Self {
        self.bytes.push(0x03);
        ty_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function which will drop the specified type of handle.
    pub fn resource_drop_async(&mut self, ty_index: u32) -> &mut Self {
        self.bytes.push(0x07);
        ty_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function which will return the representation of the specified
    /// resource type.
    pub fn resource_rep(&mut self, ty_index: u32) -> &mut Self {
        self.bytes.push(0x04);
        ty_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function which will spawn a new thread by invoking a shared
    /// function of type `ty_index`.
    pub fn thread_spawn_ref(&mut self, ty_index: u32) -> &mut Self {
        self.bytes.push(0x40);
        ty_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function which will spawn a new thread by invoking a shared
    /// function indirectly through a `funcref` table.
    pub fn thread_spawn_indirect(&mut self, ty_index: u32, table_index: u32) -> &mut Self {
        self.bytes.push(0x41);
        ty_index.encode(&mut self.bytes);
        table_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function which will return the number of threads that can be
    /// expected to execute concurrently.
    pub fn thread_available_parallelism(&mut self) -> &mut Self {
        self.bytes.push(0x42);
        self.num_added += 1;
        self
    }

    /// Defines a function which tells the host to increment the backpressure
    /// counter.
    pub fn backpressure_inc(&mut self) -> &mut Self {
        self.bytes.push(0x24);
        self.num_added += 1;
        self
    }

    /// Defines a function which tells the host to decrement the backpressure
    /// counter.
    pub fn backpressure_dec(&mut self) -> &mut Self {
        self.bytes.push(0x25);
        self.num_added += 1;
        self
    }

    /// Defines a function which returns a result to the caller of a lifted
    /// export function.  This allows the callee to continue executing after
    /// returning a result.
    pub fn task_return<O>(&mut self, ty: Option<ComponentValType>, options: O) -> &mut Self
    where
        O: IntoIterator<Item = CanonicalOption>,
        O::IntoIter: ExactSizeIterator,
    {
        self.bytes.push(0x09);
        crate::encode_resultlist(&mut self.bytes, ty);
        self.encode_options(options);
        self.num_added += 1;
        self
    }

    /// Defines a function to acknowledge cancellation of the current task.
    pub fn task_cancel(&mut self) -> &mut Self {
        self.bytes.push(0x05);
        self.num_added += 1;
        self
    }

    /// Defines a new `context.get` intrinsic of the ith slot.
    pub fn context_get(&mut self, i: u32) -> &mut Self {
        self.bytes.push(0x0a);
        self.bytes.push(0x7f);
        i.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a new `context.set` intrinsic of the ith slot.
    pub fn context_set(&mut self, i: u32) -> &mut Self {
        self.bytes.push(0x0b);
        self.bytes.push(0x7f);
        i.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function which yields control to the host so that other tasks
    /// are able to make progress, if any.
    ///
    /// If `cancellable` is true, the caller instance may be reentered.
    pub fn thread_yield(&mut self, cancellable: bool) -> &mut Self {
        self.bytes.push(0x0c);
        self.bytes.push(if cancellable { 1 } else { 0 });
        self.num_added += 1;
        self
    }

    /// Defines a function to drop a specified task which has completed.
    pub fn subtask_drop(&mut self) -> &mut Self {
        self.bytes.push(0x0d);
        self.num_added += 1;
        self
    }

    /// Defines a function to cancel an in-progress task.
    pub fn subtask_cancel(&mut self, async_: bool) -> &mut Self {
        self.bytes.push(0x06);
        self.bytes.push(if async_ { 1 } else { 0 });
        self.num_added += 1;
        self
    }

    /// Defines a function to create a new `stream` handle of the specified
    /// type.
    pub fn stream_new(&mut self, ty: u32) -> &mut Self {
        self.bytes.push(0x0e);
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function to read from a `stream` of the specified type.
    pub fn stream_read<O>(&mut self, ty: u32, options: O) -> &mut Self
    where
        O: IntoIterator<Item = CanonicalOption>,
        O::IntoIter: ExactSizeIterator,
    {
        self.bytes.push(0x0f);
        ty.encode(&mut self.bytes);
        self.encode_options(options);
        self.num_added += 1;
        self
    }

    /// Defines a function to write to a `stream` of the specified type.
    pub fn stream_write<O>(&mut self, ty: u32, options: O) -> &mut Self
    where
        O: IntoIterator<Item = CanonicalOption>,
        O::IntoIter: ExactSizeIterator,
    {
        self.bytes.push(0x10);
        ty.encode(&mut self.bytes);
        self.encode_options(options);
        self.num_added += 1;
        self
    }

    /// Defines a function to cancel an in-progress read from a `stream` of the
    /// specified type.
    pub fn stream_cancel_read(&mut self, ty: u32, async_: bool) -> &mut Self {
        self.bytes.push(0x11);
        ty.encode(&mut self.bytes);
        self.bytes.push(if async_ { 1 } else { 0 });
        self.num_added += 1;
        self
    }

    /// Defines a function to cancel an in-progress write to a `stream` of the
    /// specified type.
    pub fn stream_cancel_write(&mut self, ty: u32, async_: bool) -> &mut Self {
        self.bytes.push(0x12);
        ty.encode(&mut self.bytes);
        self.bytes.push(if async_ { 1 } else { 0 });
        self.num_added += 1;
        self
    }

    /// Defines a function to drop the readable end of a `stream` of the
    /// specified type.
    pub fn stream_drop_readable(&mut self, ty: u32) -> &mut Self {
        self.bytes.push(0x13);
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function to drop the writable end of a `stream` of the
    /// specified type.
    pub fn stream_drop_writable(&mut self, ty: u32) -> &mut Self {
        self.bytes.push(0x14);
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function to create a new `future` handle of the specified
    /// type.
    pub fn future_new(&mut self, ty: u32) -> &mut Self {
        self.bytes.push(0x15);
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function to read from a `future` of the specified type.
    pub fn future_read<O>(&mut self, ty: u32, options: O) -> &mut Self
    where
        O: IntoIterator<Item = CanonicalOption>,
        O::IntoIter: ExactSizeIterator,
    {
        self.bytes.push(0x16);
        ty.encode(&mut self.bytes);
        self.encode_options(options);
        self.num_added += 1;
        self
    }

    /// Defines a function to write to a `future` of the specified type.
    pub fn future_write<O>(&mut self, ty: u32, options: O) -> &mut Self
    where
        O: IntoIterator<Item = CanonicalOption>,
        O::IntoIter: ExactSizeIterator,
    {
        self.bytes.push(0x17);
        ty.encode(&mut self.bytes);
        self.encode_options(options);
        self.num_added += 1;
        self
    }

    /// Defines a function to cancel an in-progress read from a `future` of the
    /// specified type.
    pub fn future_cancel_read(&mut self, ty: u32, async_: bool) -> &mut Self {
        self.bytes.push(0x18);
        ty.encode(&mut self.bytes);
        self.bytes.push(if async_ { 1 } else { 0 });
        self.num_added += 1;
        self
    }

    /// Defines a function to cancel an in-progress write to a `future` of the
    /// specified type.
    pub fn future_cancel_write(&mut self, ty: u32, async_: bool) -> &mut Self {
        self.bytes.push(0x19);
        ty.encode(&mut self.bytes);
        self.bytes.push(if async_ { 1 } else { 0 });
        self.num_added += 1;
        self
    }

    /// Defines a function to drop the readable end of a `future` of the
    /// specified type.
    pub fn future_drop_readable(&mut self, ty: u32) -> &mut Self {
        self.bytes.push(0x1a);
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function to drop the writable end of a `future` of the
    /// specified type.
    pub fn future_drop_writable(&mut self, ty: u32) -> &mut Self {
        self.bytes.push(0x1b);
        ty.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Defines a function to create a new `error-context` with a specified
    /// debug message.
    pub fn error_context_new<O>(&mut self, options: O) -> &mut Self
    where
        O: IntoIterator<Item = CanonicalOption>,
        O::IntoIter: ExactSizeIterator,
    {
        self.bytes.push(0x1c);
        self.encode_options(options);
        self.num_added += 1;
        self
    }

    /// Defines a function to get the debug message for a specified
    /// `error-context`.
    ///
    /// Note that the debug message might not necessarily match what was passed
    /// to `error-context.new`.
    pub fn error_context_debug_message<O>(&mut self, options: O) -> &mut Self
    where
        O: IntoIterator<Item = CanonicalOption>,
        O::IntoIter: ExactSizeIterator,
    {
        self.bytes.push(0x1d);
        self.encode_options(options);
        self.num_added += 1;
        self
    }

    /// Defines a function to drop a specified `error-context`.
    pub fn error_context_drop(&mut self) -> &mut Self {
        self.bytes.push(0x1e);
        self.num_added += 1;
        self
    }

    /// Declare a new `waitable-set.new` intrinsic, used to create a
    /// `waitable-set` pseudo-resource.
    pub fn waitable_set_new(&mut self) -> &mut Self {
        self.bytes.push(0x1f);
        self.num_added += 1;
        self
    }

    /// Declare a new `waitable-set.wait` intrinsic, used to block on a
    /// `waitable-set`.
    pub fn waitable_set_wait(&mut self, async_: bool, memory: u32) -> &mut Self {
        self.bytes.push(0x20);
        self.bytes.push(if async_ { 1 } else { 0 });
        memory.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Declare a new `waitable-set.wait` intrinsic, used to check, without
    /// blocking, if anything in a `waitable-set` is ready.
    pub fn waitable_set_poll(&mut self, async_: bool, memory: u32) -> &mut Self {
        self.bytes.push(0x21);
        self.bytes.push(if async_ { 1 } else { 0 });
        memory.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Declare a new `waitable-set.drop` intrinsic, used to dispose a
    /// `waitable-set` pseudo-resource.
    pub fn waitable_set_drop(&mut self) -> &mut Self {
        self.bytes.push(0x22);
        self.num_added += 1;
        self
    }

    /// Declare a new `waitable.join` intrinsic, used to add an item to a
    /// `waitable-set`.
    pub fn waitable_join(&mut self) -> &mut Self {
        self.bytes.push(0x23);
        self.num_added += 1;
        self
    }

    /// Declare a new `thread.index` intrinsic, used to get the index of the
    /// current thread.
    pub fn thread_index(&mut self) -> &mut Self {
        self.bytes.push(0x26);
        self.num_added += 1;
        self
    }

    /// Declare a new `thread.new-indirect` intrinsic, used to create a new
    /// thread by invoking a function indirectly through a `funcref` table.
    pub fn thread_new_indirect(&mut self, ty_index: u32, table_index: u32) -> &mut Self {
        self.bytes.push(0x27);
        ty_index.encode(&mut self.bytes);
        table_index.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Declare a new `thread.switch-to` intrinsic, used to switch execution to
    /// another thread.
    pub fn thread_switch_to(&mut self, cancellable: bool) -> &mut Self {
        self.bytes.push(0x28);
        self.bytes.push(if cancellable { 1 } else { 0 });
        self.num_added += 1;
        self
    }

    /// Declare a new `thread.suspend` intrinsic, used to suspend execution of
    /// the current thread.
    pub fn thread_suspend(&mut self, cancellable: bool) -> &mut Self {
        self.bytes.push(0x29);
        self.bytes.push(if cancellable { 1 } else { 0 });
        self.num_added += 1;
        self
    }

    /// Declare a new `thread.resume-later` intrinsic, used to resume execution
    /// of the given thread.
    pub fn thread_resume_later(&mut self) -> &mut Self {
        self.bytes.push(0x2a);
        self.num_added += 1;
        self
    }

    /// Declare a new `thread.yield-to` intrinsic, used to yield execution to
    /// a given thread.
    pub fn thread_yield_to(&mut self, cancellable: bool) -> &mut Self {
        self.bytes.push(0x2b);
        self.bytes.push(if cancellable { 1 } else { 0 });
        self.num_added += 1;
        self
    }

    fn encode_options<O>(&mut self, options: O) -> &mut Self
    where
        O: IntoIterator<Item = CanonicalOption>,
        O::IntoIter: ExactSizeIterator,
    {
        let options = options.into_iter();
        options.len().encode(&mut self.bytes);
        for option in options {
            option.encode(&mut self.bytes);
        }
        self
    }
}

impl Encode for CanonicalFunctionSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl ComponentSection for CanonicalFunctionSection {
    fn id(&self) -> u8 {
        ComponentSectionId::CanonicalFunction.into()
    }
}
