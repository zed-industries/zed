// Copyright 2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::ipc::{self, IpcMessage};
use bincode;
use lazy_static::lazy_static;
use serde;

use std::{
    cell::{Cell, RefCell},
    cmp::PartialEq,
    convert::TryInto,
    env,
    ffi::CString,
    fmt, io,
    marker::{PhantomData, Send, Sync},
    mem,
    ops::{Deref, DerefMut, RangeFrom},
    ptr, slice, thread,
    time::Duration,
};
use uuid::Uuid;
use windows::{
    core::{Error as WinError, PCSTR},
    Win32::{
        Foundation::{
            CloseHandle, CompareObjectHandles, DuplicateHandle, GetLastError,
            DUPLICATE_CLOSE_SOURCE, DUPLICATE_HANDLE_OPTIONS, DUPLICATE_SAME_ACCESS,
            ERROR_BROKEN_PIPE, ERROR_IO_INCOMPLETE, ERROR_IO_PENDING, ERROR_NOT_FOUND,
            ERROR_NO_DATA, ERROR_PIPE_CONNECTED, HANDLE, INVALID_HANDLE_VALUE, WAIT_TIMEOUT,
        },
        Storage::FileSystem::{
            CreateFileA, ReadFile, WriteFile, FILE_ATTRIBUTE_NORMAL, FILE_FLAG_OVERLAPPED,
            FILE_GENERIC_WRITE, FILE_SHARE_MODE, OPEN_EXISTING, PIPE_ACCESS_INBOUND,
        },
        System::{
            Memory::{
                CreateFileMappingA, MapViewOfFile, UnmapViewOfFile, FILE_MAP_ALL_ACCESS,
                MEMORY_MAPPED_VIEW_ADDRESS, PAGE_READWRITE, SEC_COMMIT,
            },
            Pipes::{
                ConnectNamedPipe, CreateNamedPipeA, GetNamedPipeServerProcessId,
                PIPE_READMODE_BYTE, PIPE_REJECT_REMOTE_CLIENTS, PIPE_TYPE_BYTE,
            },
            Threading::{
                CreateEventA, GetCurrentProcess, GetCurrentProcessId, OpenProcess, ResetEvent,
                INFINITE, PROCESS_DUP_HANDLE,
            },
            IO::{
                CancelIoEx, CreateIoCompletionPort, GetOverlappedResult, GetOverlappedResultEx,
                GetQueuedCompletionStatus, OVERLAPPED,
            },
        },
    },
};

mod aliased_cell;
use self::aliased_cell::AliasedCell;

lazy_static! {
    static ref CURRENT_PROCESS_ID: u32 = unsafe { GetCurrentProcessId() };
    static ref CURRENT_PROCESS_HANDLE: WinHandle = WinHandle::new(unsafe { GetCurrentProcess() });
}

// Added to overcome build error where Box<OVERLAPPED> was used and
// struct had a trait of #[derive(Debug)].  Adding NoDebug<> overrode the Debug() trait.
// e.g. - NoDebug<Box<OVERLAPPED>>,
struct NoDebug<T>(T);

impl<T> Deref for NoDebug<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for NoDebug<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> fmt::Debug for NoDebug<T> {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

lazy_static! {
    static ref DEBUG_TRACE_ENABLED: bool = env::var_os("IPC_CHANNEL_WIN_DEBUG_TRACE").is_some();
}

/// Debug macro to better track what's going on in case of errors.
macro_rules! win32_trace {
    ($($rest:tt)*) => {
        if cfg!(feature = "win32-trace") {
            if *DEBUG_TRACE_ENABLED { println!($($rest)*); }
        }
    }
}

/// When we create the pipe, how big of a write buffer do we specify?
///
/// This is reserved in the nonpaged pool.  The fragment size is the
/// max we can write to the pipe without fragmentation, and the
/// buffer size is what we tell the pipe it is, so we have room
/// for out of band data etc.
const MAX_FRAGMENT_SIZE: usize = 64 * 1024;

/// Size of the pipe's write buffer, with excess room for the header.
const PIPE_BUFFER_SIZE: usize = MAX_FRAGMENT_SIZE + 4 * 1024;

pub fn channel() -> Result<(OsIpcSender, OsIpcReceiver), WinError> {
    let pipe_id = make_pipe_id();
    let pipe_name = make_pipe_name(&pipe_id);

    let receiver = OsIpcReceiver::new_named(&pipe_name)?;
    let sender = OsIpcSender::connect_named(&pipe_name)?;

    Ok((sender, receiver))
}

struct MessageHeader {
    data_len: u32,
    oob_len: u32,
}

impl MessageHeader {
    fn total_message_bytes_needed(&self) -> usize {
        mem::size_of::<MessageHeader>() + self.data_len as usize + self.oob_len as usize
    }
}

struct Message<'data> {
    data_len: usize,
    oob_len: usize,
    bytes: &'data [u8],
}

impl<'data> Message<'data> {
    fn from_bytes(bytes: &'data [u8]) -> Option<Message> {
        if bytes.len() < mem::size_of::<MessageHeader>() {
            return None;
        }

        unsafe {
            let header = &(*(bytes.as_ptr() as *const MessageHeader));
            if bytes.len() < header.total_message_bytes_needed() {
                return None;
            }

            Some(Message {
                data_len: header.data_len as usize,
                oob_len: header.oob_len as usize,
                bytes: &bytes[0..header.total_message_bytes_needed()],
            })
        }
    }

    fn data(&self) -> &[u8] {
        &self.bytes
            [mem::size_of::<MessageHeader>()..(mem::size_of::<MessageHeader>() + self.data_len)]
    }

    fn oob_bytes(&self) -> &[u8] {
        &self.bytes[(mem::size_of::<MessageHeader>() + self.data_len)..]
    }

    fn oob_data(&self) -> Option<OutOfBandMessage> {
        if self.oob_len > 0 {
            let oob = bincode::deserialize::<OutOfBandMessage>(self.oob_bytes())
                .expect("Failed to deserialize OOB data");
            if oob.target_process_id != *CURRENT_PROCESS_ID {
                panic!("Windows IPC channel received handles intended for pid {}, but this is pid {}. \
                       This likely happened because a receiver was transferred while it had outstanding data \
                       that contained a channel or shared memory in its pipe. \
                       This isn't supported in the Windows implementation.",
                       oob.target_process_id, *CURRENT_PROCESS_ID);
            }
            Some(oob)
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        mem::size_of::<MessageHeader>() + self.data_len + self.oob_len
    }
}

/// If we have any channel handles or shmem segments, then we'll send an
/// OutOfBandMessage after the data message.
///
/// This includes the receiver's process ID, which the receiver checks to
/// make sure that the message was originally sent to it, and was not sitting
/// in another channel's buffer when that channel got transferred to another
/// process.  On Windows, we duplicate handles on the sender side to a specific
/// receiver.  If the wrong receiver gets it, those handles are not valid.
///
/// TODO(vlad): We could attempt to recover from the above situation by
/// duplicating from the intended target process to ourselves (the receiver).
/// That would only work if the intended process a) still exists; b) can be
/// opened by the receiver with handle dup privileges.  Another approach
/// could be to use a separate dedicated process intended purely for handle
/// passing, though that process would need to be global to any processes
/// amongst which you want to share channels or connect one-shot servers to.
/// There may be a system process that we could use for this purpose, but
/// I haven't found one -- and in the system process case, we'd need to ensure
/// that we don't leak the handles (e.g. dup a handle to the system process,
/// and then everything dies -- we don't want those resources to be leaked).
#[derive(Debug)]
struct OutOfBandMessage {
    target_process_id: u32,
    channel_handles: Vec<isize>,
    shmem_handles: Vec<(isize, u64)>, // handle and size
    big_data_receiver_handle: Option<(isize, u64)>, // handle and size
}

impl OutOfBandMessage {
    fn new(target_id: u32) -> OutOfBandMessage {
        OutOfBandMessage {
            target_process_id: target_id,
            channel_handles: vec![],
            shmem_handles: vec![],
            big_data_receiver_handle: None,
        }
    }

    fn needs_to_be_sent(&self) -> bool {
        !self.channel_handles.is_empty()
            || !self.shmem_handles.is_empty()
            || self.big_data_receiver_handle.is_some()
    }
}

impl serde::Serialize for OutOfBandMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (
            self.target_process_id,
            &self.channel_handles,
            &self.shmem_handles,
            &self.big_data_receiver_handle,
        )
            .serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for OutOfBandMessage {
    fn deserialize<D>(deserializer: D) -> Result<OutOfBandMessage, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (target_process_id, channel_handles, shmem_handles, big_data_receiver_handle) =
            serde::Deserialize::deserialize(deserializer)?;
        Ok(OutOfBandMessage {
            target_process_id,
            channel_handles,
            shmem_handles,
            big_data_receiver_handle,
        })
    }
}

fn make_pipe_id() -> Uuid {
    Uuid::new_v4()
}

fn make_pipe_name(pipe_id: &Uuid) -> CString {
    CString::new(format!("\\\\.\\pipe\\rust-ipc-{}", pipe_id)).unwrap()
}

/// Duplicate a given handle from this process to the target one, passing the
/// given flags to DuplicateHandle.
///
/// Unlike win32 DuplicateHandle, this will preserve INVALID_HANDLE_VALUE (which is
/// also the pseudohandle for the current process).
fn dup_handle_to_process_with_flags(
    handle: &WinHandle,
    other_process: &WinHandle,
    flags: DUPLICATE_HANDLE_OPTIONS,
) -> Result<WinHandle, WinError> {
    if !handle.is_valid() {
        return Ok(WinHandle::invalid());
    }

    unsafe {
        let mut new_handle: HANDLE = INVALID_HANDLE_VALUE;
        DuplicateHandle(
            CURRENT_PROCESS_HANDLE.as_raw(),
            handle.as_raw(),
            other_process.as_raw(),
            &mut new_handle,
            0,
            false,
            flags,
        )
        .map(|()| WinHandle::new(new_handle))
    }
}

/// Duplicate a handle in the current process.
fn dup_handle(handle: &WinHandle) -> Result<WinHandle, WinError> {
    dup_handle_to_process(handle, &WinHandle::new(CURRENT_PROCESS_HANDLE.as_raw()))
}

/// Duplicate a handle to the target process.
fn dup_handle_to_process(
    handle: &WinHandle,
    other_process: &WinHandle,
) -> Result<WinHandle, WinError> {
    dup_handle_to_process_with_flags(handle, other_process, DUPLICATE_SAME_ACCESS)
}

/// Duplicate a handle to the target process, closing the source handle.
fn move_handle_to_process(
    handle: WinHandle,
    other_process: &WinHandle,
) -> Result<WinHandle, WinError> {
    let result = dup_handle_to_process_with_flags(
        &handle,
        other_process,
        DUPLICATE_CLOSE_SOURCE | DUPLICATE_SAME_ACCESS,
    );
    // Since the handle was moved to another process, the original is no longer valid;
    // so we probably shouldn't try to close it explicitly?
    mem::forget(handle);
    result
}

#[derive(Debug)]
struct WinHandle {
    handle: HANDLE,
}

unsafe impl Send for WinHandle {}
unsafe impl Sync for WinHandle {}

impl Drop for WinHandle {
    fn drop(&mut self) {
        unsafe {
            if self.is_valid() {
                let result = CloseHandle(self.handle);
                assert!(result.is_ok() || thread::panicking());
            }
        }
    }
}

impl Default for WinHandle {
    fn default() -> WinHandle {
        WinHandle {
            handle: INVALID_HANDLE_VALUE,
        }
    }
}

impl PartialEq for WinHandle {
    fn eq(&self, other: &WinHandle) -> bool {
        unsafe { CompareObjectHandles(self.handle, other.handle).into() }
    }
}

impl WinHandle {
    fn new(handle: HANDLE) -> WinHandle {
        WinHandle { handle }
    }

    fn invalid() -> WinHandle {
        WinHandle {
            handle: INVALID_HANDLE_VALUE,
        }
    }

    fn is_valid(&self) -> bool {
        self.handle != INVALID_HANDLE_VALUE
    }

    fn as_raw(&self) -> HANDLE {
        self.handle
    }

    fn take_raw(&mut self) -> HANDLE {
        mem::replace(&mut self.handle, INVALID_HANDLE_VALUE)
    }

    fn take(&mut self) -> WinHandle {
        WinHandle::new(self.take_raw())
    }
}

/// Helper struct for all data being aliased by the kernel during async reads.
#[derive(Debug)]
struct AsyncData {
    /// File handle of the pipe on which the async operation is performed.
    handle: WinHandle,

    /// Meta-data for this async read operation, filled by the kernel.
    ///
    /// This must be on the heap, in order for its memory location --
    /// which is registered in the kernel during an async read --
    /// to remain stable even when the enclosing structure is passed around.
    ov: NoDebug<Box<Overlapped>>,

    /// Buffer for the kernel to store the results of the async read operation.
    ///
    /// The vector provided here needs to have some allocated yet unused space,
    /// i.e. `capacity()` needs to be larger than `len()`.
    /// If part of the vector is already filled, that is left in place;
    /// the new data will only be written to the unused space.
    buf: Vec<u8>,
}

#[repr(transparent)]
struct Overlapped(OVERLAPPED);

impl Drop for Overlapped {
    fn drop(&mut self) {
        unsafe {
            if !self.0.hEvent.is_invalid() {
                let result = CloseHandle(self.0.hEvent);
                assert!(result.is_ok() || thread::panicking());
            }
        }
    }
}

impl Overlapped {
    fn new(ov: OVERLAPPED) -> Self {
        Self(ov)
    }
}

impl Deref for Overlapped {
    type Target = OVERLAPPED;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Overlapped {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Main object keeping track of a receive handle and its associated state.
///
/// Implements blocking/nonblocking reads of messages from the handle.
#[derive(Debug)]
struct MessageReader {
    /// The pipe read handle.
    ///
    /// Note: this is only set while no async read operation
    /// is currently in progress with the kernel.
    /// When an async read is in progress,
    /// it is moved into the `async` sub-structure (see below)
    /// along with the other fields used for the async operation,
    /// to make sure they all stay in sync,
    /// and nothing else can meddle with the the pipe
    /// until the operation is completed.
    handle: WinHandle,

    /// Buffer for outstanding data, that has been received but not yet processed.
    ///
    /// Note: just like `handle` above,
    /// this is only set while no async read is in progress.
    /// When an async read is in progress,
    /// the receive buffer is aliased by the kernel;
    /// so we need to temporarily move it into an `AliasedCell`,
    /// thus making it inaccessible from safe code --
    /// see `async` below.
    /// We only move it back once the kernel signals completion of the async read.
    read_buf: Vec<u8>,

    /// Data used by the kernel during an async read operation.
    ///
    /// Note: Since this field only has a value
    /// when an async read operation is in progress
    /// (i.e. has been issued to the system, and not completed yet),
    /// this also serves as an indicator of the latter.
    ///
    /// WARNING: As the kernel holds mutable aliases of this data
    /// while an async read is in progress,
    /// it is crucial that it is never accessed in user space
    /// from the moment we issue an async read in `start_read()`,
    /// until the moment we process the event
    /// signalling completion of the async read in `notify_completion()`.
    ///
    /// Since Rust's type system is not aware of the kernel aliases,
    /// the compiler cannot guarantee exclusive access the way it normally would,
    /// i.e. any access to this value is inherently unsafe!
    /// We thus wrap it in an `AliasedCell`,
    /// making sure the data is only accessible from code marked `unsafe`;
    /// and only move it out when the kernel signals that the async read is done.
    r#async: Option<AliasedCell<AsyncData>>,

    /// Token identifying the reader/receiver within an `OsIpcReceiverSet`.
    ///
    /// This is returned to callers of `OsIpcReceiverSet.add()` and `OsIpcReceiverSet.select()`.
    ///
    /// `None` if this `MessageReader` is not part of any set.
    entry_id: Option<u64>,
}

// We need to explicitly declare this, because of the raw pointer
// contained in the `OVERLAPPED` structure.
//
// Note: the `Send` claim is only really fulfilled
// as long as nothing can ever alias the aforementioned raw pointer.
// As explained in the documentation of the `async` field,
// this is a tricky condition (because of kernel aliasing),
// which we however need to uphold regardless of the `Send` property --
// so claiming `Send` should not introduce any additional issues.
unsafe impl Send for OsIpcReceiver {}

impl Drop for MessageReader {
    fn drop(&mut self) {
        // Before dropping the `ov` structure and read buffer,
        // make sure the kernel won't do any more async updates to them!
        self.cancel_io();
    }
}

impl MessageReader {
    fn new(handle: WinHandle) -> MessageReader {
        MessageReader {
            handle,
            read_buf: Vec::new(),
            r#async: None,
            entry_id: None,
        }
    }

    fn take(&mut self) -> MessageReader {
        // This is currently somewhat inefficient,
        // because of the initialisation of things that won't be used.
        // Moving the data items of `MessageReader` into an enum will fix this,
        // as that way we will be able to just define a data-less `Invalid` case.
        mem::replace(self, MessageReader::new(WinHandle::invalid()))
    }

    /// Request the kernel to cancel a pending async I/O operation on this reader.
    ///
    /// Note that this only schedules the cancel request;
    /// but doesn't guarantee that the operation is done
    /// (and the buffers are no longer used by the kernel)
    /// before this method returns.
    ///
    /// A caller that wants to ensure the operation is really done,
    /// will need to wait using `fetch_async_result()`.
    /// (Or `fetch_iocp_result()` for readers in a set.)
    ///
    /// The only exception is if the kernel indicates
    /// that no operation was actually outstanding at this point.
    /// In that case, the `async` data is released immediately;
    /// and the caller should not attempt waiting for completion.
    fn issue_async_cancel(&mut self) {
        unsafe {
            let result = CancelIoEx(
                self.r#async.as_ref().unwrap().alias().handle.as_raw(),
                self.r#async
                    .as_ref()
                    .map(|a| std::ptr::addr_of!(a.alias().ov.0.deref().0)),
            );

            if let Err(error) = result {
                // A cancel operation is not expected to fail.
                // If it does, callers are not prepared for that -- so we have to bail.
                //
                // Note that we should never ignore a failed cancel,
                // since that would affect further operations;
                // and the caller definitely must not free the aliased data in that case!
                //
                // Sometimes `CancelIoEx()` fails with `ERROR_NOT_FOUND` though,
                // meaning there is actually no async operation outstanding at this point.
                // (Specifically, this is triggered by the `receiver_set_big_data()` test.)
                // Not sure why that happens -- but I *think* it should be benign...
                //
                // In that case, we can safely free the async data right now;
                // and the caller should not attempt to wait for completion.
                assert!(error.code() == ERROR_NOT_FOUND.to_hresult());

                let async_data = self.r#async.take().unwrap().into_inner();
                self.handle = async_data.handle;
                self.read_buf = async_data.buf;
            }
        }
    }

    fn cancel_io(&mut self) {
        if self.r#async.is_some() {
            // This doesn't work for readers in a receiver set.
            // (`fetch_async_result()` would hang indefinitely.)
            // Receiver sets have to handle cancellation specially,
            // and make sure they always do that *before* dropping readers.
            assert!(self.entry_id.is_none());

            self.issue_async_cancel();

            // If there is an operation still in flight, wait for it to complete.
            //
            // This will usually fail with `ERROR_OPERATION_ABORTED`;
            // but it could also return success, or some other error,
            // if the operation actually completed in the mean time.
            // We don't really care either way --
            // we just want to be certain there is no operation in flight any more.
            if self.r#async.is_some() {
                let _ = self.fetch_async_result(BlockingMode::Blocking);
            }
        }
    }

    /// Kick off an asynchronous read.
    ///
    /// When an async read is started successfully,
    /// the receive buffer is moved out of `read_buf`
    /// into the `AliasedCell<>` in `async`,
    /// thus making it inaccessible from safe code;
    /// it will only be moved back in `notify_completion()`.
    /// (See documentation of the `read_buf` and `async` fields.)
    fn start_read(&mut self) -> Result<(), WinIpcError> {
        // Nothing needs to be done if an async read operation is already in progress.
        if self.r#async.is_some() {
            return Ok(());
        }

        win32_trace!("[$ {:?}] start_read", self.handle);

        if self.read_buf.len() == self.read_buf.capacity() {
            self.read_buf.reserve(PIPE_BUFFER_SIZE);
        }

        unsafe {
            // Temporarily extend the vector to span its entire capacity,
            // so we can safely sub-slice it for the actual read.
            let buf_len = self.read_buf.len();
            let buf_cap = self.read_buf.capacity();
            self.read_buf.set_len(buf_cap);

            // issue the read to the buffer, at the current length offset
            self.r#async = Some(AliasedCell::new(AsyncData {
                handle: self.handle.take(),
                ov: NoDebug(Box::new({
                    let mut overlapped: OVERLAPPED = mem::zeroed();
                    // Create a manually reset event. The documentation for GetOverlappedResultEx
                    // states you must do this in the remarks section.
                    overlapped.hEvent = CreateEventA(None, true, false, None)?;
                    Overlapped::new(overlapped)
                })),
                buf: std::mem::take(&mut self.read_buf),
            }));
            let result = {
                let async_data = self.r#async.as_mut().unwrap().alias_mut();
                let remaining_buf = &mut async_data.buf[buf_len..];
                ReadFile(
                    async_data.handle.as_raw(),
                    Some(remaining_buf),
                    None,
                    Some(&mut ***async_data.ov.deref_mut()),
                )
            };

            // Reset the vector to only expose the already filled part.
            //
            // This means that the async read
            // will actually fill memory beyond the exposed part of the vector.
            // While this use of a vector is officially sanctioned for such cases,
            // it still feel rather icky to me...
            //
            // On the other hand, this way we make sure
            // the buffer never appears to have more valid data
            // than what is actually present,
            // which could pose a potential danger in its own right.
            // Also, it avoids the need to keep a separate state variable --
            // which would bear some risk of getting out of sync.
            self.r#async
                .as_mut()
                .unwrap()
                .alias_mut()
                .buf
                .set_len(buf_len);

            match result.as_ref().map_err(|e| e.code()) {
                // Normally, for an async operation, a call like
                // `ReadFile` would return `false`, and the error code
                // would be `ERROR_IO_PENDING`.  But in some situations,
                // `ReadFile` can complete synchronously (returns `true`).
                // Even if it does, a notification that the IO completed
                // is still sent to the IO completion port that this
                // handle is part of, meaning that we don't have to do any
                // special handling for sync-completed operations.
                Ok(_) => Ok(()),
                Err(err) => {
                    if err == ERROR_IO_PENDING.to_hresult() {
                        Ok(())
                    } else if err == ERROR_BROKEN_PIPE.to_hresult() {
                        win32_trace!("[$ {:?}] BROKEN_PIPE straight from ReadFile", self.handle);

                        let async_data = self.r#async.take().unwrap().into_inner();
                        self.handle = async_data.handle;
                        self.read_buf = async_data.buf;

                        Err(WinIpcError::ChannelClosed)
                    } else {
                        let async_data = self.r#async.take().unwrap().into_inner();
                        self.handle = async_data.handle;
                        self.read_buf = async_data.buf;

                        result.map_err(|e| e.into())
                    }
                },
            }
        }
    }

    /// Called when we receive an IO Completion Packet for this handle.
    ///
    /// During its course, this method moves `async.buf` back into `read_buf`,
    /// thus making it accessible from normal code again;
    /// so `get_message()` can extract the received messages from the buffer.
    ///
    /// Invoking this is unsafe, since calling it in error
    /// while an async read is actually still in progress in the kernel
    /// would have catastrophic effects,
    /// as the `async` data is still mutably aliased by the kernel in that case!
    /// (See documentation of the `async` field.)
    ///
    /// Also, this method relies on `async` actually having valid data,
    /// i.e. nothing should modify its constituent fields
    /// between receiving the completion notification from the kernel
    /// and invoking this method.
    unsafe fn notify_completion(
        &mut self,
        io_result: Result<(), WinError>,
    ) -> Result<(), WinIpcError> {
        win32_trace!(
            "[$ {:?}] notify_completion",
            self.r#async.as_ref().unwrap().alias().handle
        );

        // Regardless whether the kernel reported success or error,
        // it doesn't have an async read operation in flight at this point anymore.
        // (And it's safe again to access the `async` data.)
        let async_data = self.r#async.take().unwrap().into_inner();
        self.handle = async_data.handle;
        let ov = async_data.ov;
        self.read_buf = async_data.buf;

        match io_result {
            Ok(()) => {},
            Err(err) => {
                if err.code() == ERROR_BROKEN_PIPE.to_hresult() {
                    // Remote end closed the channel.
                    return Err(WinIpcError::ChannelClosed);
                }
                return Err(err.into());
            },
        }

        let nbytes = ov.InternalHigh as u32;
        let offset = ov.Anonymous.Anonymous.Offset;

        assert!(offset == 0);

        let new_size = self.read_buf.len() + nbytes as usize;
        win32_trace!(
            "nbytes: {}, offset {}, buf len {}->{}, capacity {}",
            nbytes,
            offset,
            self.read_buf.len(),
            new_size,
            self.read_buf.capacity()
        );
        assert!(new_size <= self.read_buf.capacity());
        self.read_buf.set_len(new_size);

        Ok(())
    }

    /// Attempt to conclude an already issued async read operation.
    ///
    /// If successful, the result will be ready for picking up by `get_message()`.
    ///
    /// (`get_message()` might still yield nothing though,
    /// in case only part of the message was received in this read,
    /// and further read operations are necessary to get the rest.)
    ///
    /// In non-blocking mode, this may return with `WinError:NoData`,
    /// while the async operation remains in flight.
    /// The read buffer remains unavailable in that case,
    /// since it's still aliased by the kernel.
    /// (And there is nothing new to pick up anyway.)
    /// It will only become available again
    /// when `fetch_async_result()` returns successfully upon retry.
    /// (Or the async read is aborted with `cancel_io()`.)
    fn fetch_async_result(&mut self, blocking_mode: BlockingMode) -> Result<(), WinIpcError> {
        unsafe {
            // Get the overlapped result, blocking if we need to.
            let mut nbytes: u32 = 0;
            let timeout = match blocking_mode {
                BlockingMode::Blocking => INFINITE,
                BlockingMode::Nonblocking => 0,
                BlockingMode::Timeout(duration) => {
                    duration.as_millis().try_into().unwrap_or(INFINITE)
                },
            };
            let result = GetOverlappedResultEx(
                self.r#async.as_ref().unwrap().alias().handle.as_raw(),
                &***self.r#async.as_mut().unwrap().alias_mut().ov.deref(),
                &mut nbytes,
                timeout,
                false,
            );
            let _ = ResetEvent(
                self.r#async
                    .as_mut()
                    .unwrap()
                    .alias_mut()
                    .ov
                    .deref_mut()
                    .hEvent,
            );
            let io_result = if result.is_err() {
                let err = GetLastError();
                if blocking_mode != BlockingMode::Blocking && err == ERROR_IO_INCOMPLETE {
                    // Async read hasn't completed yet.
                    // Inform the caller, while keeping the read in flight.
                    return Err(WinIpcError::NoData);
                }
                // Timeout has elapsed, so we must cancel the read operation before proceeding
                if err.0 == WAIT_TIMEOUT.0 {
                    self.cancel_io();
                    return Err(WinIpcError::NoData);
                }
                // We pass err through to notify_completion so
                // that it can handle other errors.
                Err(WinError::new(err.to_hresult(), ""))
            } else {
                Ok(())
            };

            // Notify that the read completed, which will update the
            // read pointers
            self.notify_completion(io_result)
        }
    }

    fn get_message(&mut self) -> Result<Option<IpcMessage>, WinIpcError> {
        // Never touch the buffer while it's still mutably aliased by the kernel!
        if self.r#async.is_some() {
            return Ok(None);
        }

        let drain_bytes;
        let result;
        if let Some(message) = Message::from_bytes(&self.read_buf) {
            let mut channels: Vec<OsOpaqueIpcChannel> = vec![];
            let mut shmems: Vec<OsIpcSharedMemory> = vec![];
            let mut big_data = None;

            if let Some(oob) = message.oob_data() {
                win32_trace!("[$ {:?}] msg with total {} bytes, {} channels, {} shmems, big data handle {:?}",
                     self.handle, message.data_len, oob.channel_handles.len(), oob.shmem_handles.len(),
                     oob.big_data_receiver_handle);

                for handle in oob.channel_handles {
                    channels.push(OsOpaqueIpcChannel::new(WinHandle::new(HANDLE(handle as _))));
                }

                for (handle, size) in oob.shmem_handles {
                    shmems.push(
                        OsIpcSharedMemory::from_handle(
                            WinHandle::new(HANDLE(handle as _)),
                            size as usize,
                        )
                        .unwrap(),
                    );
                }

                if oob.big_data_receiver_handle.is_some() {
                    let (handle, big_data_size) = oob.big_data_receiver_handle.unwrap();
                    let receiver = OsIpcReceiver::from_handle(WinHandle::new(HANDLE(handle as _)));
                    big_data = Some(receiver.recv_raw(big_data_size as usize)?);
                }
            }

            let buf_data = big_data.unwrap_or_else(|| message.data().to_vec());

            win32_trace!(
                "[$ {:?}] get_message success -> {} bytes, {} channels, {} shmems",
                self.handle,
                buf_data.len(),
                channels.len(),
                shmems.len()
            );
            drain_bytes = Some(message.size());
            result = Some(IpcMessage::new(buf_data, channels, shmems));
        } else {
            drain_bytes = None;
            result = None;
        }

        if let Some(size) = drain_bytes {
            // If the only valid bytes in the buffer are what we just
            // consumed, then just set the vector's length to 0.  This
            // avoids reallocations as in the drain() case, and is
            // a significant speedup.
            if self.read_buf.len() == size {
                self.read_buf.clear();
            } else {
                self.read_buf.drain(0..size);
            }
        }

        Ok(result)
    }

    fn add_to_iocp(&mut self, iocp: &WinHandle, entry_id: u64) -> Result<(), WinIpcError> {
        unsafe {
            assert!(self.entry_id.is_none());

            let completion_key = self.handle.as_raw().0;
            CreateIoCompletionPort(
                self.handle.as_raw(),
                iocp.as_raw(),
                completion_key as usize,
                0,
            )?;
        }

        self.entry_id = Some(entry_id);

        // The readers in the IOCP need to have async reads in flight,
        // so they can actually get completion events --
        // otherwise, a subsequent `select()` call would just hang indefinitely.
        self.start_read()
    }

    /// Specialized read for out-of-band data ports.
    ///
    /// Here the buffer size is known in advance,
    /// and the transfer doesn't have our typical message framing.
    ///
    /// It's only valid to call this as the one and only call after creating a MessageReader.
    fn read_raw_sized(mut self, size: usize) -> Result<Vec<u8>, WinIpcError> {
        assert!(self.read_buf.is_empty());

        self.read_buf.reserve(size);
        while self.read_buf.len() < size {
            // Because our handle is asynchronous, we have to do a two-part read --
            // first issue the operation, then wait for its completion.
            match self.start_read() {
                Err(WinIpcError::ChannelClosed) => {
                    // If the helper channel closes unexpectedly
                    // (i.e. before supplying the expected amount of data),
                    // don't report that as a "sender closed" condition on the main channel:
                    // rather, fail with the actual raw error code.
                    return Err(WinError::new(ERROR_BROKEN_PIPE.to_hresult(), "ReadFile").into());
                },
                Err(err) => return Err(err),
                Ok(()) => {},
            };
            match self.fetch_async_result(BlockingMode::Blocking) {
                Err(WinIpcError::ChannelClosed) => {
                    return Err(WinError::new(ERROR_BROKEN_PIPE.to_hresult(), "ReadFile").into());
                },
                Err(err) => return Err(err),
                Ok(()) => {},
            };
        }

        Ok(std::mem::take(&mut self.read_buf))
    }

    /// Get raw handle of the receive port.
    ///
    /// This is only for debug tracing purposes, and must not be used for anything else.
    fn get_raw_handle(&self) -> HANDLE {
        self.handle.as_raw()
    }
}

#[derive(Clone, Copy, Debug)]
enum AtomicMode {
    Atomic,
    Nonatomic,
}

/// Write data to a handle.
///
/// In `Atomic` mode, this panics if the data can't be written in a single system call.
fn write_buf(handle: &WinHandle, bytes: &[u8], atomic: AtomicMode) -> Result<(), WinError> {
    let total = bytes.len();
    if total == 0 {
        return Ok(());
    }

    let mut written = 0;
    while written < total {
        let mut sz: u32 = 0;
        let bytes_to_write = &bytes[written..];
        unsafe { WriteFile(handle.as_raw(), Some(bytes_to_write), Some(&mut sz), None)? }
        written += sz as usize;
        match atomic {
            AtomicMode::Atomic => {
                if written != total {
                    panic!("Windows IPC write_buf expected to write full buffer, but only wrote partial (wrote {} out of {} bytes)", written, total);
                }
            },
            AtomicMode::Nonatomic => {
                win32_trace!(
                    "[c {:?}] ... wrote {} bytes, total {}/{} err {}",
                    handle.as_raw(),
                    sz,
                    written,
                    total,
                    WinError::from_win32()
                );
            },
        }
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BlockingMode {
    Blocking,
    Nonblocking,
    Timeout(Duration),
}

#[derive(Debug)]
pub struct OsIpcReceiver {
    /// The receive handle and its associated state.
    ///
    /// We can't just deal with raw handles like in the other platform back-ends,
    /// since this implementation -- using plain pipes with no native packet handling --
    /// requires keeping track of various bits of receiver state,
    /// which must not be separated from the handle itself.
    ///
    /// Note: Inner mutability is necessary,
    /// since the `consume()` method needs to move out the reader
    /// despite only getting a shared reference to `self`.
    reader: RefCell<MessageReader>,
}

impl PartialEq for OsIpcReceiver {
    fn eq(&self, other: &OsIpcReceiver) -> bool {
        self.reader.borrow().handle == other.reader.borrow().handle
    }
}

impl OsIpcReceiver {
    fn from_handle(handle: WinHandle) -> OsIpcReceiver {
        OsIpcReceiver {
            reader: RefCell::new(MessageReader::new(handle)),
        }
    }

    fn new_named(pipe_name: &CString) -> Result<OsIpcReceiver, WinError> {
        unsafe {
            // create the pipe server
            let handle = CreateNamedPipeA(
                PCSTR::from_raw(pipe_name.as_ptr() as *const u8),
                PIPE_ACCESS_INBOUND | FILE_FLAG_OVERLAPPED,
                PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_REJECT_REMOTE_CLIENTS,
                // 1 max instance of this pipe
                1,
                // out/in buffer sizes
                0,
                PIPE_BUFFER_SIZE as u32,
                0, // default timeout for WaitNamedPipe (0 == 50ms as default)
                None,
            )?;

            Ok(OsIpcReceiver {
                reader: RefCell::new(MessageReader::new(WinHandle::new(handle))),
            })
        }
    }

    fn prepare_for_transfer(&self) -> Result<bool, WinError> {
        let mut reader = self.reader.borrow_mut();
        // cancel any outstanding IO request
        reader.cancel_io();
        // this is only okay if we have nothing in the read buf
        Ok(reader.read_buf.is_empty())
    }

    pub fn consume(&self) -> OsIpcReceiver {
        let mut reader = self.reader.borrow_mut();
        assert!(reader.r#async.is_none());
        OsIpcReceiver::from_handle(reader.handle.take())
    }

    // This is only used for recv/try_recv/try_recv_timeout.  When this is added to an IpcReceiverSet, then
    // the implementation in select() is used.  It does much the same thing, but across multiple
    // channels.
    fn receive_message(&self, mut blocking_mode: BlockingMode) -> Result<IpcMessage, WinIpcError> {
        let mut reader = self.reader.borrow_mut();
        assert!(
            reader.entry_id.is_none(),
            "receive_message is only valid before this OsIpcReceiver was added to a Set"
        );
        // This function loops, because in the case of a blocking read, we may need to
        // read multiple sets of bytes from the pipe to receive a complete message.
        loop {
            // First, try to fetch a message, in case we have one pending
            // in the reader's receive buffer
            if let Some(ipc_message) = reader.get_message()? {
                return Ok(ipc_message);
            }

            // Then, issue a read if we don't have one already in flight.
            reader.start_read()?;

            // Attempt to complete the read.
            //
            // May return `WinError::NoData` in non-blocking mode.
            // The async read remains in flight in that case;
            // and another attempt at getting a result
            // can be done the next time we are called.
            reader.fetch_async_result(blocking_mode)?;

            // If we're not blocking, pretend that we are blocking, since we got part of
            // a message already.  Keep reading until we get a complete message.
            blocking_mode = BlockingMode::Blocking;
        }
    }

    pub fn recv(&self) -> Result<IpcMessage, WinIpcError> {
        win32_trace!("recv");
        self.receive_message(BlockingMode::Blocking)
    }

    pub fn try_recv(&self) -> Result<IpcMessage, WinIpcError> {
        win32_trace!("try_recv");
        self.receive_message(BlockingMode::Nonblocking)
    }

    pub fn try_recv_timeout(&self, duration: Duration) -> Result<IpcMessage, WinIpcError> {
        win32_trace!("try_recv_timeout");
        self.receive_message(BlockingMode::Timeout(duration))
    }

    /// Do a pipe connect.
    ///
    /// Only used for one-shot servers.
    fn accept(&self) -> Result<(), WinError> {
        unsafe {
            let reader_borrow = self.reader.borrow();
            let handle = &reader_borrow.handle;
            // Boxing this to get a stable address is not strictly necesssary here,
            // since we are not moving the local variable around -- but better safe than sorry...
            let mut ov = AliasedCell::new(Box::new(mem::zeroed::<OVERLAPPED>()));
            let result = ConnectNamedPipe(handle.as_raw(), Some(ov.alias_mut().deref_mut()));

            // we should always get false with async IO
            assert!(result.is_err());
            let result = match GetLastError() {
                // did we successfully connect? (it's reported as an error [ok==false])
                ERROR_PIPE_CONNECTED => {
                    win32_trace!("[$ {:?}] accept (PIPE_CONNECTED)", handle.as_raw());
                    Ok(())
                },

                // This is a weird one -- if we create a named pipe (like we do
                // in new() ), the client connects, sends data, then drops its handle,
                // a Connect here will get ERROR_NO_DATA -- but there may be data in
                // the pipe that we'll be able to read.  So we need to go do some reads
                // like normal and wait until ReadFile gives us ERROR_NO_DATA.
                ERROR_NO_DATA => {
                    win32_trace!("[$ {:?}] accept (ERROR_NO_DATA)", handle.as_raw());
                    Ok(())
                },

                // the connect is pending; wait for it to complete
                ERROR_IO_PENDING => {
                    let mut nbytes: u32 = 0;
                    GetOverlappedResult(
                        handle.as_raw(),
                        ov.alias_mut().deref_mut(),
                        &mut nbytes,
                        true,
                    )
                },

                // Anything else signifies some actual I/O error.
                err => {
                    win32_trace!("[$ {:?}] accept error -> {:?}", handle.as_raw(), err);
                    Err(WinError::new(err.to_hresult(), "ConnectNamedPipe"))
                },
            };

            ov.into_inner();
            result
        }
    }

    /// Does a single explicitly-sized recv from the handle,
    /// consuming the receiver in the process.
    ///
    /// This is used for receiving data from the out-of-band big data buffer.
    fn recv_raw(self, size: usize) -> Result<Vec<u8>, WinIpcError> {
        self.reader.into_inner().read_raw_sized(size)
    }
}

#[derive(Debug, PartialEq)]
pub struct OsIpcSender {
    handle: WinHandle,
    // Make sure this is `!Sync`, to match `mpsc::Sender`; and to discourage sharing references.
    //
    // (Rather, senders should just be cloned, as they are shared internally anyway --
    // another layer of sharing only adds unnecessary overhead...)
    nosync_marker: PhantomData<Cell<()>>,
}

impl Clone for OsIpcSender {
    fn clone(&self) -> OsIpcSender {
        OsIpcSender::from_handle(dup_handle(&self.handle).unwrap())
    }
}

impl OsIpcSender {
    pub fn connect(name: String) -> Result<OsIpcSender, WinError> {
        let pipe_name = make_pipe_name(&Uuid::parse_str(&name).unwrap());
        OsIpcSender::connect_named(&pipe_name)
    }

    pub fn get_max_fragment_size() -> usize {
        MAX_FRAGMENT_SIZE
    }

    fn from_handle(handle: WinHandle) -> OsIpcSender {
        OsIpcSender {
            handle,
            nosync_marker: PhantomData,
        }
    }

    /// Connect to a pipe server.
    fn connect_named(pipe_name: &CString) -> Result<OsIpcSender, WinError> {
        unsafe {
            let handle = CreateFileA(
                PCSTR::from_raw(pipe_name.as_ptr() as *const u8),
                FILE_GENERIC_WRITE.0,
                FILE_SHARE_MODE(0),
                None, // lpSecurityAttributes
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )?;

            win32_trace!("[c {:?}] connect_to_server success", handle);

            Ok(OsIpcSender::from_handle(WinHandle::new(handle)))
        }
    }

    fn get_pipe_server_process_id(&self) -> Result<u32, WinError> {
        unsafe {
            let mut server_pid = 0;
            GetNamedPipeServerProcessId(self.handle.as_raw(), &mut server_pid).map(|()| server_pid)
        }
    }

    fn get_pipe_server_process_handle_and_pid(&self) -> Result<(WinHandle, u32), WinError> {
        unsafe {
            let server_pid = self.get_pipe_server_process_id()?;
            if server_pid == *CURRENT_PROCESS_ID {
                return Ok((WinHandle::new(CURRENT_PROCESS_HANDLE.as_raw()), server_pid));
            }

            let raw_handle = OpenProcess(PROCESS_DUP_HANDLE, false, server_pid)?;

            Ok((WinHandle::new(raw_handle), server_pid))
        }
    }

    fn needs_fragmentation(data_len: usize, oob: &OutOfBandMessage) -> bool {
        let oob_size = if oob.needs_to_be_sent() {
            bincode::serialized_size(oob).unwrap()
        } else {
            0
        };

        // make sure we don't have too much oob data to begin with
        assert!(
            (oob_size as usize) <= (PIPE_BUFFER_SIZE - mem::size_of::<MessageHeader>()),
            "too much oob data"
        );

        let bytes_left_for_data =
            (PIPE_BUFFER_SIZE - mem::size_of::<MessageHeader>()) - (oob_size as usize);
        data_len >= bytes_left_for_data
    }

    /// An internal-use-only send method that sends just raw data, with no header.
    fn send_raw(&self, data: &[u8]) -> Result<(), WinError> {
        win32_trace!(
            "[c {:?}] writing {} bytes raw to (pid {}->{})",
            self.handle.as_raw(),
            data.len(),
            *CURRENT_PROCESS_ID,
            self.get_pipe_server_process_id()?
        );

        // Write doesn't need to be atomic,
        // since the pipe is exclusive for this message,
        // so we don't have to fear intermixing with parts of other messages.
        write_buf(&self.handle, data, AtomicMode::Nonatomic)
    }

    pub fn send(
        &self,
        data: &[u8],
        ports: Vec<OsIpcChannel>,
        shared_memory_regions: Vec<OsIpcSharedMemory>,
    ) -> Result<(), WinIpcError> {
        // We limit the max size we can send here; we can fix this
        // just by upping the header to be 2x u64 if we really want
        // to.
        assert!(data.len() <= u32::MAX as usize);

        let (server_h, server_pid) = if !shared_memory_regions.is_empty() || !ports.is_empty() {
            self.get_pipe_server_process_handle_and_pid()?
        } else {
            (WinHandle::invalid(), 0)
        };

        let mut oob = OutOfBandMessage::new(server_pid);

        for ref shmem in shared_memory_regions {
            // shmem.handle, shmem.length
            let mut remote_handle = dup_handle_to_process(&shmem.handle, &server_h)?;
            oob.shmem_handles
                .push((remote_handle.take_raw().0 as _, shmem.length as u64));
        }

        for port in ports {
            match port {
                OsIpcChannel::Sender(s) => {
                    let mut raw_remote_handle = move_handle_to_process(s.handle, &server_h)?;
                    oob.channel_handles
                        .push(raw_remote_handle.take_raw().0 as _);
                },
                OsIpcChannel::Receiver(r) => {
                    if !(r.prepare_for_transfer()?) {
                        panic!("Sending receiver with outstanding partial read buffer, noooooo!  What should even happen?");
                    }

                    let handle = r.reader.into_inner().handle.take();
                    let mut raw_remote_handle = move_handle_to_process(handle, &server_h)?;
                    oob.channel_handles
                        .push(raw_remote_handle.take_raw().0 as _);
                },
            }
        }

        // Do we need to fragment?
        let big_data_sender: Option<OsIpcSender> =
            if OsIpcSender::needs_fragmentation(data.len(), &oob) {
                // We need to create a channel for the big data
                let (sender, receiver) = channel()?;

                let (server_h, server_pid) = if server_h.is_valid() {
                    (server_h, server_pid)
                } else {
                    self.get_pipe_server_process_handle_and_pid()?
                };

                // Put the receiver in the OOB data
                let handle = receiver.reader.into_inner().handle.take();
                let mut raw_receiver_handle = move_handle_to_process(handle, &server_h)?;
                oob.big_data_receiver_handle =
                    Some((raw_receiver_handle.take_raw().0 as _, data.len() as u64));
                oob.target_process_id = server_pid;

                Some(sender)
            } else {
                None
            };

        // If we need to send OOB data, serialize it
        let mut oob_data: Vec<u8> = vec![];
        if oob.needs_to_be_sent() {
            oob_data = bincode::serialize(&oob).unwrap();
        }

        let in_band_data_len = if big_data_sender.is_none() {
            data.len()
        } else {
            0
        };
        let header = MessageHeader {
            data_len: in_band_data_len as u32,
            oob_len: oob_data.len() as u32,
        };
        let full_in_band_len = header.total_message_bytes_needed();
        assert!(full_in_band_len <= PIPE_BUFFER_SIZE);
        let mut full_message = Vec::<u8>::with_capacity(full_in_band_len);

        {
            let header_bytes = unsafe {
                slice::from_raw_parts(&header as *const _ as *const u8, mem::size_of_val(&header))
            };
            full_message.extend_from_slice(header_bytes);
        }

        if big_data_sender.is_none() {
            full_message.extend_from_slice(data);
            full_message.extend_from_slice(&oob_data);
            assert!(full_message.len() == full_in_band_len);

            // Write needs to be atomic, since otherwise concurrent sending
            // could result in parts of different messages getting intermixed,
            // and the receiver would not be able to extract the individual messages.
            write_buf(&self.handle, &full_message, AtomicMode::Atomic)?;
        } else {
            full_message.extend_from_slice(&oob_data);
            assert!(full_message.len() == full_in_band_len);

            write_buf(&self.handle, &full_message, AtomicMode::Atomic)?;
            big_data_sender.unwrap().send_raw(data)?;
        }

        Ok(())
    }
}

pub enum OsIpcSelectionResult {
    DataReceived(u64, IpcMessage),
    ChannelClosed(u64),
}

pub struct OsIpcReceiverSet {
    /// Our incrementor, for unique handle IDs.
    incrementor: RangeFrom<u64>,

    /// The IOCP that we select on.
    iocp: WinHandle,

    /// The set of receivers, stored as MessageReaders.
    readers: Vec<MessageReader>,

    /// Readers that got closed before adding them to the set.
    ///
    /// These need to report a "closed" event on the next `select()` call.
    ///
    /// Only the `entry_id` is necessary for that.
    closed_readers: Vec<u64>,
}

impl Drop for OsIpcReceiverSet {
    fn drop(&mut self) {
        // We need to cancel any in-flight read operations before we drop the receivers,
        // since otherwise the receivers' `Drop` implementation would try to cancel them --
        // but the implementation there doesn't work for receivers in a set...
        for reader in &mut self.readers {
            reader.issue_async_cancel();
        }

        // Wait for any reads still in flight to complete,
        // thus freeing the associated async data.
        self.readers.retain(|r| r.r#async.is_some());
        while !self.readers.is_empty() {
            // We unwrap the outer result (can't deal with the IOCP call failing here),
            // but don't care about the actual results of the completed read operations.
            let _ = self.fetch_iocp_result().unwrap();
        }
    }
}

impl OsIpcReceiverSet {
    pub fn new() -> Result<OsIpcReceiverSet, WinError> {
        unsafe {
            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, None, 0, 0)?;

            Ok(OsIpcReceiverSet {
                incrementor: 0..,
                iocp: WinHandle::new(iocp),
                readers: vec![],
                closed_readers: vec![],
            })
        }
    }

    pub fn add(&mut self, receiver: OsIpcReceiver) -> Result<u64, WinIpcError> {
        // consume the receiver, and take the reader out
        let mut reader = receiver.reader.into_inner();

        let entry_id = self.incrementor.next().unwrap();

        match reader.add_to_iocp(&self.iocp, entry_id) {
            Ok(()) => {
                win32_trace!(
                    "[# {:?}] ReceiverSet add {:?}, id {}",
                    self.iocp.as_raw(),
                    reader.get_raw_handle(),
                    entry_id
                );
                self.readers.push(reader);
            },
            Err(WinIpcError::ChannelClosed) => {
                // If the sender has already been closed, we need to stash this information,
                // so we can report the corresponding event in the next `select()` call.
                win32_trace!(
                    "[# {:?}] ReceiverSet add {:?} (closed), id {}",
                    self.iocp.as_raw(),
                    reader.get_raw_handle(),
                    entry_id
                );
                self.closed_readers.push(entry_id);
            },
            Err(err) => return Err(err),
        };

        Ok(entry_id)
    }

    /// Conclude an async read operation on any of the receivers in the set.
    ///
    /// This fetches a completion event from the set's IOCP;
    /// finds the matching `MessageReader`;
    /// removes it from the list of active readers
    /// (since no operation is in flight on this reader at this point);
    /// and notifies the reader of the completion event.
    ///
    /// If the IOCP call is successful, this returns the respective reader,
    /// along with an inner status describing the type of event received.
    /// This can be a success status, indicating data has been received,
    /// and is ready to be picked up with `get_message()` on the reader;
    /// an error status indicating that the sender connected to this receiver
    /// has closed the connection;
    /// or some other I/O error status.
    ///
    /// Unless a "closed" status is returned,
    /// the respective reader remains a member of the set,
    /// and the caller should add it back to the list of active readers
    /// after kicking off a new read operation on it.
    fn fetch_iocp_result(&mut self) -> Result<(MessageReader, Result<(), WinIpcError>), WinError> {
        unsafe {
            let mut nbytes: u32 = 0;
            let mut completion_key = 0;
            let mut ov_ptr: *mut OVERLAPPED = ptr::null_mut();
            // XXX use GetQueuedCompletionStatusEx to dequeue multiple CP at once!
            let result = GetQueuedCompletionStatus(
                self.iocp.as_raw(),
                &mut nbytes,
                &mut completion_key,
                &mut ov_ptr,
                INFINITE,
            );
            win32_trace!(
                "[# {:?}] GetQueuedCS -> ok:{} nbytes:{} key:{:?}",
                self.iocp.as_raw(),
                result.is_ok(),
                nbytes,
                completion_key
            );
            let io_result = if let Err(err) = result {
                // If the OVERLAPPED result is NULL, then the
                // function call itself failed or timed out.
                // Otherwise, the async IO operation failed, and
                // we want to hand the error to notify_completion below.
                if ov_ptr.is_null() {
                    return Err(err);
                }

                Err(err)
            } else {
                Ok(())
            };

            assert!(!ov_ptr.is_null());
            assert!(completion_key != 0);

            // Find the matching receiver
            let (reader_index, _) = self
                .readers
                .iter()
                .enumerate()
                .find(|(_, reader)| {
                    let raw_handle = reader.r#async.as_ref().unwrap().alias().handle.as_raw();
                    raw_handle.0 as usize == completion_key
                })
                .expect(
                    "Windows IPC ReceiverSet got notification for a receiver it doesn't know about",
                );

            // Remove the entry from the set for now -- we will re-add it later,
            // if we can successfully initiate another async read operation.
            let mut reader = self.readers.swap_remove(reader_index);

            win32_trace!(
                "[# {:?}] result for receiver {:?}",
                self.iocp.as_raw(),
                reader.get_raw_handle()
            );

            // tell it about the completed IO op
            let result = reader.notify_completion(io_result);

            Ok((reader, result))
        }
    }

    pub fn select(&mut self) -> Result<Vec<OsIpcSelectionResult>, WinIpcError> {
        assert!(
            self.readers.len() + self.closed_readers.len() > 0,
            "selecting with no objects?"
        );
        win32_trace!(
            "[# {:?}] select() with {} active and {} closed receivers",
            self.iocp.as_raw(),
            self.readers.len(),
            self.closed_readers.len()
        );

        // the ultimate results
        let mut selection_results = vec![];

        // Process any pending "closed" events
        // from channels that got closed before being added to the set,
        // and thus received "closed" notifications while being added.
        self.closed_readers.drain(..).for_each(|entry_id| {
            selection_results.push(OsIpcSelectionResult::ChannelClosed(entry_id))
        });

        // Do this in a loop, because we may need to dequeue multiple packets to
        // read a complete message.
        while selection_results.is_empty() {
            let (mut reader, result) = self.fetch_iocp_result()?;

            let mut closed = match result {
                Ok(()) => false,
                Err(WinIpcError::ChannelClosed) => true,
                Err(err) => return Err(err),
            };

            if !closed {
                // Drain as many messages as we can.
                while let Some(ipc_message) = reader.get_message()? {
                    win32_trace!(
                        "[# {:?}] receiver {:?} ({}) got a message",
                        self.iocp.as_raw(),
                        reader.get_raw_handle(),
                        reader.entry_id.unwrap()
                    );
                    selection_results.push(OsIpcSelectionResult::DataReceived(
                        reader.entry_id.unwrap(),
                        ipc_message,
                    ));
                }
                win32_trace!(
                    "[# {:?}] receiver {:?} ({}) -- no message",
                    self.iocp.as_raw(),
                    reader.get_raw_handle(),
                    reader.entry_id.unwrap()
                );

                // Now that we are done frobbing the buffer,
                // we can safely initiate the next async read operation.
                closed = match reader.start_read() {
                    Ok(()) => {
                        // We just successfully reinstated it as an active reader --
                        // so add it back to the list.
                        //
                        // Note: `take()` is a workaround for the compiler not seeing
                        // that we won't actually be using it anymore after this...
                        self.readers.push(reader.take());
                        false
                    },
                    Err(WinIpcError::ChannelClosed) => true,
                    Err(err) => return Err(err),
                };
            }

            // If we got a "sender closed" notification --
            // either instead of new data,
            // or while trying to re-initiate an async read after receiving data --
            // add an event to this effect to the result list.
            if closed {
                win32_trace!(
                    "[# {:?}] receiver {:?} ({}) -- now closed!",
                    self.iocp.as_raw(),
                    reader.get_raw_handle(),
                    reader.entry_id.unwrap()
                );
                selection_results.push(OsIpcSelectionResult::ChannelClosed(
                    reader.entry_id.unwrap(),
                ));
            }
        }

        win32_trace!("select() -> {} results", selection_results.len());
        Ok(selection_results)
    }
}

impl OsIpcSelectionResult {
    pub fn unwrap(self) -> (u64, IpcMessage) {
        match self {
            OsIpcSelectionResult::DataReceived(id, ipc_message) => (id, ipc_message),
            OsIpcSelectionResult::ChannelClosed(id) => {
                panic!(
                    "OsIpcSelectionResult::unwrap(): receiver ID {} was closed!",
                    id
                )
            },
        }
    }
}

#[derive(Debug)]
pub struct OsIpcSharedMemory {
    handle: WinHandle,
    view_handle: MEMORY_MAPPED_VIEW_ADDRESS,
    length: usize,
}

unsafe impl Send for OsIpcSharedMemory {}
unsafe impl Sync for OsIpcSharedMemory {}

impl Drop for OsIpcSharedMemory {
    fn drop(&mut self) {
        unsafe {
            let result = UnmapViewOfFile(self.view_handle);
            assert!(result.is_ok() || thread::panicking());
        }
    }
}

impl Clone for OsIpcSharedMemory {
    fn clone(&self) -> OsIpcSharedMemory {
        OsIpcSharedMemory::from_handle(dup_handle(&self.handle).unwrap(), self.length).unwrap()
    }
}

impl PartialEq for OsIpcSharedMemory {
    fn eq(&self, other: &OsIpcSharedMemory) -> bool {
        self.handle == other.handle
    }
}

impl Deref for OsIpcSharedMemory {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        assert!(!self.view_handle.Value.is_null() && self.handle.is_valid());
        unsafe { slice::from_raw_parts(self.view_handle.Value as _, self.length) }
    }
}

impl OsIpcSharedMemory {
    fn new(length: usize) -> Result<OsIpcSharedMemory, WinError> {
        unsafe {
            assert!(length < u32::MAX as usize);
            let (lhigh, llow) = (
                length.checked_shr(32).unwrap_or(0) as u32,
                (length & 0xffffffff) as u32,
            );
            let handle = CreateFileMappingA(
                INVALID_HANDLE_VALUE,
                None,
                PAGE_READWRITE | SEC_COMMIT,
                lhigh,
                llow,
                None,
            )?;

            OsIpcSharedMemory::from_handle(WinHandle::new(handle), length)
        }
    }

    // There is no easy way to query the size of the mapping -- you
    // can use NtQuerySection, but that's an undocumented NT kernel
    // API.  Instead we'll just always pass the length along.
    //
    // This function takes ownership of the handle, and will close it
    // when finished.
    fn from_handle(handle: WinHandle, length: usize) -> Result<OsIpcSharedMemory, WinError> {
        unsafe {
            let address = MapViewOfFile(handle.as_raw(), FILE_MAP_ALL_ACCESS, 0, 0, 0);
            if address.Value.is_null() {
                return Err(WinError::from_win32());
            }

            Ok(OsIpcSharedMemory {
                handle,
                view_handle: address,
                length,
            })
        }
    }

    pub fn from_byte(byte: u8, length: usize) -> OsIpcSharedMemory {
        unsafe {
            // panic if we can't create it
            let mem = OsIpcSharedMemory::new(length).unwrap();
            for element in slice::from_raw_parts_mut(mem.view_handle.Value as _, mem.length) {
                *element = byte;
            }
            mem
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> OsIpcSharedMemory {
        unsafe {
            // panic if we can't create it
            let mem = OsIpcSharedMemory::new(bytes.len()).unwrap();
            ptr::copy_nonoverlapping(bytes.as_ptr(), mem.view_handle.Value as _, bytes.len());
            mem
        }
    }
}

pub struct OsIpcOneShotServer {
    receiver: OsIpcReceiver,
}

impl OsIpcOneShotServer {
    pub fn new() -> Result<(OsIpcOneShotServer, String), WinError> {
        let pipe_id = make_pipe_id();
        let pipe_name = make_pipe_name(&pipe_id);
        let receiver = OsIpcReceiver::new_named(&pipe_name)?;
        Ok((OsIpcOneShotServer { receiver }, pipe_id.to_string()))
    }

    pub fn accept(self) -> Result<(OsIpcReceiver, IpcMessage), WinIpcError> {
        let receiver = self.receiver;
        receiver.accept()?;
        let ipc_message = receiver.recv()?;
        Ok((receiver, ipc_message))
    }
}

pub enum OsIpcChannel {
    Sender(OsIpcSender),
    Receiver(OsIpcReceiver),
}

#[derive(Debug, PartialEq)]
pub struct OsOpaqueIpcChannel {
    handle: WinHandle,
}

impl Drop for OsOpaqueIpcChannel {
    fn drop(&mut self) {
        // Make sure we don't leak!
        //
        // The `OsOpaqueIpcChannel` objects should always be used,
        // i.e. converted with `to_sender()` or `to_receiver()` --
        // so the value should already be unset before the object gets dropped.
        debug_assert!(!self.handle.is_valid());
    }
}

impl OsOpaqueIpcChannel {
    fn new(handle: WinHandle) -> OsOpaqueIpcChannel {
        OsOpaqueIpcChannel { handle }
    }

    pub fn to_receiver(&mut self) -> OsIpcReceiver {
        OsIpcReceiver::from_handle(self.handle.take())
    }

    pub fn to_sender(&mut self) -> OsIpcSender {
        OsIpcSender::from_handle(self.handle.take())
    }
}

#[derive(Debug)]
pub enum WinIpcError {
    WinError(WinError),
    ChannelClosed,
    NoData,
}

impl WinIpcError {
    pub fn channel_is_closed(&self) -> bool {
        matches!(self, Self::ChannelClosed)
    }
}

impl From<WinIpcError> for bincode::Error {
    fn from(error: WinIpcError) -> bincode::Error {
        io::Error::from(error).into()
    }
}

impl From<WinError> for WinIpcError {
    fn from(e: WinError) -> Self {
        Self::WinError(e)
    }
}

impl From<WinIpcError> for ipc::IpcError {
    fn from(error: WinIpcError) -> Self {
        match error {
            WinIpcError::ChannelClosed => ipc::IpcError::Disconnected,
            e => ipc::IpcError::Io(io::Error::from(e)),
        }
    }
}

impl From<WinIpcError> for ipc::TryRecvError {
    fn from(error: WinIpcError) -> Self {
        match error {
            WinIpcError::ChannelClosed => ipc::TryRecvError::IpcError(ipc::IpcError::Disconnected),
            WinIpcError::NoData => ipc::TryRecvError::Empty,
            e => ipc::TryRecvError::IpcError(ipc::IpcError::Io(io::Error::from(e))),
        }
    }
}

impl From<WinIpcError> for io::Error {
    fn from(error: WinIpcError) -> io::Error {
        match error {
            WinIpcError::ChannelClosed => {
                // This is the error code we originally got from the Windows API
                // to signal the "channel closed" (no sender) condition --
                // so hand it back to the Windows API to create an appropriate `Error` value.
                io::Error::from_raw_os_error(ERROR_BROKEN_PIPE.0 as i32)
            },
            WinIpcError::NoData => io::Error::new(
                io::ErrorKind::WouldBlock,
                "Win channel has no data available",
            ),
            WinIpcError::WinError(err) => io::Error::from_raw_os_error(err.code().0),
        }
    }
}
