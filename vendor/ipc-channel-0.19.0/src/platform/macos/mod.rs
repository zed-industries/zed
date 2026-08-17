// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use self::mach_sys::mach_port_deallocate;
use self::mach_sys::{kern_return_t, mach_msg_body_t, mach_msg_header_t, mach_msg_return_t};
use self::mach_sys::{mach_msg_ool_descriptor_t, mach_msg_port_descriptor_t, mach_msg_type_name_t};
use self::mach_sys::{mach_msg_timeout_t, mach_port_limits_t, mach_port_msgcount_t};
use self::mach_sys::{mach_port_right_t, mach_port_t, mach_task_self_, vm_inherit_t};
use crate::ipc::{self, IpcMessage};

use bincode;
use lazy_static::lazy_static;
use libc::{self, c_char, c_uint, c_void, size_t};
use rand::{self, Rng};
use std::cell::Cell;
use std::convert::TryInto;
use std::error::Error as StdError;
use std::ffi::CString;
use std::fmt::{self, Debug, Formatter};
use std::io;
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::slice;
use std::sync::RwLock;
use std::time::Duration;

mod mach_sys;

/// The size that we preallocate on the stack to receive messages. If the message is larger than
/// this, we retry and spill to the heap.
const SMALL_MESSAGE_SIZE: usize = 4096;

/// A string to prepend to our bootstrap ports.
static BOOTSTRAP_PREFIX: &str = "org.rust-lang.ipc-channel.";

const BOOTSTRAP_NAME_IN_USE: kern_return_t = 1101;
const BOOTSTRAP_SUCCESS: kern_return_t = 0;
const KERN_NOT_IN_SET: kern_return_t = 12;
const KERN_INVALID_NAME: kern_return_t = 15;
const KERN_INVALID_RIGHT: kern_return_t = 17;
const KERN_INVALID_VALUE: kern_return_t = 18;
const KERN_UREFS_OVERFLOW: kern_return_t = 19;
const KERN_INVALID_CAPABILITY: kern_return_t = 20;
const KERN_SUCCESS: kern_return_t = 0;
const KERN_NO_SPACE: kern_return_t = 3;
const MACH_MSGH_BITS_COMPLEX: u32 = 0x80000000;
const MACH_MSG_IPC_KERNEL: kern_return_t = 0x00000800;
const MACH_MSG_IPC_SPACE: kern_return_t = 0x00002000;
const MACH_MSG_OOL_DESCRIPTOR: u32 = 1;
const MACH_MSG_PORT_DESCRIPTOR: u32 = 0;
const MACH_MSG_SUCCESS: kern_return_t = 0;
const MACH_MSG_TIMEOUT_NONE: mach_msg_timeout_t = 0;
const MACH_MSG_TYPE_COPY_SEND: u8 = 19;
const MACH_MSG_TYPE_MAKE_SEND: u8 = 20;
const MACH_MSG_TYPE_MAKE_SEND_ONCE: u8 = 21;
const MACH_MSG_TYPE_MOVE_RECEIVE: u32 = 16;
const MACH_MSG_TYPE_MOVE_SEND: u32 = 17;
const MACH_MSG_TYPE_PORT_SEND: u32 = MACH_MSG_TYPE_MOVE_SEND;
const MACH_MSG_VIRTUAL_COPY: c_uint = 1;
const MACH_MSG_VM_KERNEL: kern_return_t = 0x00000400;
const MACH_MSG_VM_SPACE: kern_return_t = 0x00001000;
const MACH_NOTIFY_FIRST: i32 = 64;
const MACH_NOTIFY_NO_SENDERS: i32 = MACH_NOTIFY_FIRST + 6;
const MACH_PORT_LIMITS_INFO: i32 = 1;
const MACH_PORT_NULL: mach_port_t = 0;
const MACH_PORT_QLIMIT_LARGE: mach_port_msgcount_t = 1024;
const MACH_PORT_QLIMIT_MAX: mach_port_msgcount_t = MACH_PORT_QLIMIT_LARGE;
const MACH_PORT_RIGHT_PORT_SET: mach_port_right_t = 3;
const MACH_PORT_RIGHT_RECEIVE: mach_port_right_t = 1;
const MACH_PORT_RIGHT_SEND: mach_port_right_t = 0;
const MACH_RCV_BODY_ERROR: kern_return_t = 0x1000400c;
const MACH_RCV_HEADER_ERROR: kern_return_t = 0x1000400b;
const MACH_RCV_INTERRUPTED: kern_return_t = 0x10004005;
const MACH_RCV_INVALID_DATA: kern_return_t = 0x10004008;
const MACH_RCV_INVALID_NAME: kern_return_t = 0x10004002;
const MACH_RCV_INVALID_NOTIFY: kern_return_t = 0x10004007;
const MACH_RCV_INVALID_TRAILER: kern_return_t = 0x1000400f;
const MACH_RCV_INVALID_TYPE: kern_return_t = 0x1000400d;
const MACH_RCV_IN_PROGRESS: kern_return_t = 0x10004001;
const MACH_RCV_IN_PROGRESS_TIMED: kern_return_t = 0x10004011;
const MACH_RCV_IN_SET: kern_return_t = 0x1000400a;
const MACH_RCV_LARGE: i32 = 4;
const MACH_RCV_MSG: i32 = 2;
const MACH_RCV_PORT_CHANGED: kern_return_t = 0x10004006;
const MACH_RCV_PORT_DIED: kern_return_t = 0x10004009;
const MACH_RCV_SCATTER_SMALL: kern_return_t = 0x1000400e;
const MACH_RCV_TIMED_OUT: kern_return_t = 0x10004003;
const MACH_RCV_TIMEOUT: i32 = 0x100;
const MACH_RCV_TOO_LARGE: kern_return_t = 0x10004004;
const MACH_SEND_INTERRUPTED: kern_return_t = 0x10000007;
const MACH_SEND_INVALID_DATA: kern_return_t = 0x10000002;
const MACH_SEND_INVALID_DEST: kern_return_t = 0x10000003;
const MACH_SEND_INVALID_HEADER: kern_return_t = 0x10000010;
const MACH_SEND_INVALID_MEMORY: kern_return_t = 0x1000000c;
const MACH_SEND_INVALID_NOTIFY: kern_return_t = 0x1000000b;
const MACH_SEND_INVALID_REPLY: kern_return_t = 0x10000009;
const MACH_SEND_INVALID_RIGHT: kern_return_t = 0x1000000a;
const MACH_SEND_INVALID_RT_OOL_SIZE: kern_return_t = 0x10000015;
const MACH_SEND_INVALID_TRAILER: kern_return_t = 0x10000011;
const MACH_SEND_INVALID_TYPE: kern_return_t = 0x1000000f;
const MACH_SEND_INVALID_VOUCHER: kern_return_t = 0x10000005;
const MACH_SEND_IN_PROGRESS: kern_return_t = 0x10000001;
const MACH_SEND_MSG: i32 = 1;
const MACH_SEND_MSG_TOO_SMALL: kern_return_t = 0x10000008;
const MACH_SEND_NO_BUFFER: kern_return_t = 0x1000000d;
const MACH_SEND_TIMED_OUT: kern_return_t = 0x10000004;
const MACH_SEND_TOO_LARGE: kern_return_t = 0x1000000e;
const TASK_BOOTSTRAP_PORT: i32 = 4;
const VM_INHERIT_SHARE: vm_inherit_t = 0;

#[allow(non_camel_case_types)]
type name_t = *const c_char;

pub fn channel() -> Result<(OsIpcSender, OsIpcReceiver), MachError> {
    let receiver = OsIpcReceiver::new()?;
    let sender = receiver.sender()?;
    receiver.request_no_senders_notification()?;
    Ok((sender, receiver))
}

#[derive(PartialEq, Debug)]
pub struct OsIpcReceiver {
    port: Cell<mach_port_t>,
}

impl Drop for OsIpcReceiver {
    fn drop(&mut self) {
        let port = self.port.get();
        if port != MACH_PORT_NULL {
            mach_port_mod_release(port, MACH_PORT_RIGHT_RECEIVE).unwrap();
        }
    }
}

fn mach_port_allocate(right: mach_port_right_t) -> Result<mach_port_t, KernelError> {
    let mut port: mach_port_t = 0;
    let os_result = unsafe { mach_sys::mach_port_allocate(mach_task_self(), right, &mut port) };
    if os_result == KERN_SUCCESS {
        return Ok(port);
    }
    Err(os_result.into())
}

fn mach_port_mod_addref(port: mach_port_t, right: mach_port_right_t) -> Result<(), KernelError> {
    let err = unsafe { mach_sys::mach_port_mod_refs(mach_task_self(), port, right, 1) };
    if err == KERN_SUCCESS {
        return Ok(());
    }
    Err(err.into())
}

fn mach_port_mod_release(port: mach_port_t, right: mach_port_right_t) -> Result<(), KernelError> {
    let err = unsafe { mach_sys::mach_port_mod_refs(mach_task_self(), port, right, -1) };
    if err == KERN_SUCCESS {
        return Ok(());
    }
    Err(err.into())
}

fn mach_port_move_member(port: mach_port_t, set: mach_port_t) -> Result<(), KernelError> {
    let error = unsafe { mach_sys::mach_port_move_member(mach_task_self(), port, set) };
    if error == KERN_SUCCESS {
        return Ok(());
    }
    Err(error.into())
}

fn mach_port_extract_right(
    port: mach_port_t,
    message_type: mach_msg_type_name_t,
) -> Result<(mach_port_t, mach_msg_type_name_t), KernelError> {
    let (mut right, mut acquired_right) = (0, 0);
    let error = unsafe {
        mach_sys::mach_port_extract_right(
            mach_task_self(),
            port,
            message_type,
            &mut right,
            &mut acquired_right,
        )
    };
    if error == KERN_SUCCESS {
        return Ok((right, acquired_right));
    }
    Err(error.into())
}

impl OsIpcReceiver {
    fn new() -> Result<OsIpcReceiver, MachError> {
        let port = mach_port_allocate(MACH_PORT_RIGHT_RECEIVE)?;
        let limits = mach_port_limits_t {
            mpl_qlimit: MACH_PORT_QLIMIT_MAX,
        };
        let os_result = unsafe {
            mach_sys::mach_port_set_attributes(
                mach_task_self(),
                port,
                MACH_PORT_LIMITS_INFO,
                &limits as *const mach_port_limits_t as *mut i32,
                1,
            )
        };
        if os_result == KERN_SUCCESS {
            Ok(OsIpcReceiver::from_name(port))
        } else {
            Err(KernelError::from(os_result).into())
        }
    }

    fn from_name(port: mach_port_t) -> OsIpcReceiver {
        OsIpcReceiver {
            port: Cell::new(port),
        }
    }

    fn extract_port(&self) -> mach_port_t {
        let port = self.port.get();
        debug_assert!(port != MACH_PORT_NULL);
        port
    }

    fn consume_port(&self) -> mach_port_t {
        let port = self.extract_port();
        self.port.set(MACH_PORT_NULL);
        port
    }

    pub fn consume(&self) -> OsIpcReceiver {
        OsIpcReceiver::from_name(self.consume_port())
    }

    fn sender(&self) -> Result<OsIpcSender, MachError> {
        let port = self.port.get();
        debug_assert!(port != MACH_PORT_NULL);
        let (right, acquired_right) =
            mach_port_extract_right(port, MACH_MSG_TYPE_MAKE_SEND as u32)?;
        debug_assert!(acquired_right == MACH_MSG_TYPE_PORT_SEND);
        Ok(OsIpcSender::from_name(right))
    }

    fn register_bootstrap_name(&self) -> Result<(u32, String), MachError> {
        let port = self.port.get();
        debug_assert!(port != MACH_PORT_NULL);
        unsafe {
            let mut bootstrap_port = 0;
            let os_result = mach_sys::task_get_special_port(
                mach_task_self(),
                TASK_BOOTSTRAP_PORT,
                &mut bootstrap_port,
            );
            if os_result != KERN_SUCCESS {
                return Err(KernelError::from(os_result).into());
            }

            let (right, acquired_right) =
                mach_port_extract_right(port, MACH_MSG_TYPE_MAKE_SEND as u32)?;
            debug_assert!(acquired_right == MACH_MSG_TYPE_PORT_SEND);

            let mut os_result;
            let mut name;
            loop {
                name = format!("{}{}", BOOTSTRAP_PREFIX, rand::thread_rng().gen::<i64>());
                let c_name = CString::new(name.clone()).unwrap();
                os_result = bootstrap_register2(bootstrap_port, c_name.as_ptr(), right, 0);
                if os_result == BOOTSTRAP_NAME_IN_USE {
                    continue;
                }
                if os_result != BOOTSTRAP_SUCCESS {
                    return Err(MachError::from(os_result));
                }
                break;
            }
            Ok((right, name))
        }
    }

    fn unregister_global_name(name: String) -> Result<(), MachError> {
        unsafe {
            let mut bootstrap_port = 0;
            let os_result = mach_sys::task_get_special_port(
                mach_task_self(),
                TASK_BOOTSTRAP_PORT,
                &mut bootstrap_port,
            );
            if os_result != KERN_SUCCESS {
                return Err(KernelError::from(os_result).into());
            }

            let c_name = CString::new(name).unwrap();
            let os_result = bootstrap_register2(bootstrap_port, c_name.as_ptr(), MACH_PORT_NULL, 0);
            if os_result == BOOTSTRAP_SUCCESS {
                Ok(())
            } else {
                Err(MachError::from(os_result))
            }
        }
    }

    fn request_no_senders_notification(&self) -> Result<(), MachError> {
        let port = self.port.get();
        debug_assert!(port != MACH_PORT_NULL);
        unsafe {
            let os_result = mach_sys::mach_port_request_notification(
                mach_task_self(),
                port,
                MACH_NOTIFY_NO_SENDERS,
                0,
                port,
                MACH_MSG_TYPE_MAKE_SEND_ONCE as u32,
                &mut 0,
            );
            if os_result != KERN_SUCCESS {
                return Err(KernelError::from(os_result).into());
            }
        }
        Ok(())
    }

    fn recv_with_blocking_mode(
        &self,
        blocking_mode: BlockingMode,
    ) -> Result<IpcMessage, MachError> {
        select(self.port.get(), blocking_mode).and_then(|result| match result {
            OsIpcSelectionResult::DataReceived(_, ipc_message) => Ok(ipc_message),
            OsIpcSelectionResult::ChannelClosed(_) => Err(MachError::from(MACH_NOTIFY_NO_SENDERS)),
        })
    }

    pub fn recv(&self) -> Result<IpcMessage, MachError> {
        self.recv_with_blocking_mode(BlockingMode::Blocking)
    }

    pub fn try_recv(&self) -> Result<IpcMessage, MachError> {
        self.recv_with_blocking_mode(BlockingMode::Nonblocking)
    }

    pub fn try_recv_timeout(&self, duration: Duration) -> Result<IpcMessage, MachError> {
        self.recv_with_blocking_mode(BlockingMode::Timeout(duration))
    }
}

enum SendData<'a> {
    Inline(&'a [u8]),
    OutOfLine(Option<OsIpcSharedMemory>),
}

lazy_static! {
    static ref MAX_INLINE_SIZE: RwLock<usize> = RwLock::new(usize::MAX);
}

impl<'a> From<&'a [u8]> for SendData<'a> {
    fn from(data: &'a [u8]) -> SendData<'a> {
        let max_inline_size = *MAX_INLINE_SIZE.read().unwrap();
        if data.len() >= max_inline_size {
            // Convert the data payload into a shared memory region to avoid exceeding
            // any message size limits.
            SendData::OutOfLine(Some(OsIpcSharedMemory::from_bytes(data)))
        } else {
            SendData::Inline(data)
        }
    }
}

impl<'a> SendData<'a> {
    fn take_shared_memory(&mut self) -> Option<OsIpcSharedMemory> {
        match *self {
            SendData::Inline(_) => None,
            SendData::OutOfLine(ref mut data) => data.take(),
        }
    }

    fn is_inline(&self) -> bool {
        match *self {
            SendData::Inline(_) => true,
            SendData::OutOfLine(_) => false,
        }
    }

    fn inline_data(&self) -> &[u8] {
        match *self {
            SendData::Inline(data) => data,
            SendData::OutOfLine(_) => &[],
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct OsIpcSender {
    port: mach_port_t,
    // Make sure this is `!Sync`, to match `crossbeam_channel::Sender`; and to discourage sharing
    // references.
    //
    // (Rather, senders should just be cloned, as they are shared internally anyway --
    // another layer of sharing only adds unnecessary overhead...)
    nosync_marker: PhantomData<Cell<()>>,
}

impl Drop for OsIpcSender {
    fn drop(&mut self) {
        if self.port == MACH_PORT_NULL {
            return;
        }
        deallocate_mach_port(self.port);
    }
}

impl Clone for OsIpcSender {
    fn clone(&self) -> OsIpcSender {
        let mut cloned_port = self.port;
        if cloned_port != MACH_PORT_NULL {
            match mach_port_mod_addref(cloned_port, MACH_PORT_RIGHT_SEND) {
                Ok(()) => (),
                Err(KernelError::InvalidRight) => cloned_port = MACH_PORT_NULL,
                Err(error) => panic!("mach_port_mod_refs(1, {}) failed: {:?}", cloned_port, error),
            }
        }
        OsIpcSender {
            port: cloned_port,
            nosync_marker: PhantomData,
        }
    }
}

impl OsIpcSender {
    fn from_name(port: mach_port_t) -> OsIpcSender {
        OsIpcSender {
            port,
            nosync_marker: PhantomData,
        }
    }

    pub fn connect(name: String) -> Result<OsIpcSender, MachError> {
        unsafe {
            let mut bootstrap_port = 0;
            let os_result = mach_sys::task_get_special_port(
                mach_task_self(),
                TASK_BOOTSTRAP_PORT,
                &mut bootstrap_port,
            );
            if os_result != KERN_SUCCESS {
                return Err(KernelError::from(os_result).into());
            }

            let mut port = 0;
            let c_name = CString::new(name).unwrap();
            let os_result = bootstrap_look_up(bootstrap_port, c_name.as_ptr(), &mut port);
            if os_result == BOOTSTRAP_SUCCESS {
                Ok(OsIpcSender::from_name(port))
            } else {
                Err(MachError::from(os_result))
            }
        }
    }

    pub fn get_max_fragment_size() -> usize {
        usize::MAX
    }

    pub fn send(
        &self,
        data: &[u8],
        ports: Vec<OsIpcChannel>,
        mut shared_memory_regions: Vec<OsIpcSharedMemory>,
    ) -> Result<(), MachError> {
        let mut data = SendData::from(data);
        if let Some(data) = data.take_shared_memory() {
            shared_memory_regions.push(data);
        }

        unsafe {
            let size = Message::size_of(&data, ports.len(), shared_memory_regions.len());
            let message = libc::malloc(size as size_t) as *mut Message;
            (*message).header.msgh_bits = (MACH_MSG_TYPE_COPY_SEND as u32) | MACH_MSGH_BITS_COMPLEX;
            (*message).header.msgh_size = size as u32;
            (*message).header.msgh_local_port = MACH_PORT_NULL;
            (*message).header.msgh_remote_port = self.port;
            (*message).header.msgh_id = 0;
            (*message).body.msgh_descriptor_count =
                (ports.len() + shared_memory_regions.len()) as u32;

            let mut port_descriptor_dest = message.offset(1) as *mut mach_msg_port_descriptor_t;
            for outgoing_port in &ports {
                (*port_descriptor_dest).name = outgoing_port.port();
                (*port_descriptor_dest).pad1 = 0;

                (*port_descriptor_dest).set_disposition(match *outgoing_port {
                    OsIpcChannel::Sender(_) => MACH_MSG_TYPE_MOVE_SEND,
                    OsIpcChannel::Receiver(_) => MACH_MSG_TYPE_MOVE_RECEIVE,
                });

                (*port_descriptor_dest).set_type(MACH_MSG_PORT_DESCRIPTOR);
                port_descriptor_dest = port_descriptor_dest.offset(1);
            }

            let mut shared_memory_descriptor_dest =
                port_descriptor_dest as *mut mach_msg_ool_descriptor_t;
            for shared_memory_region in &shared_memory_regions {
                (*shared_memory_descriptor_dest).address =
                    shared_memory_region.as_ptr() as *const c_void as *mut c_void;
                (*shared_memory_descriptor_dest).size = shared_memory_region.len() as u32;
                (*shared_memory_descriptor_dest).set_deallocate(1);
                (*shared_memory_descriptor_dest).set_copy(MACH_MSG_VIRTUAL_COPY);
                (*shared_memory_descriptor_dest).set_type(MACH_MSG_OOL_DESCRIPTOR);
                shared_memory_descriptor_dest = shared_memory_descriptor_dest.offset(1);
            }

            let is_inline_dest = shared_memory_descriptor_dest as *mut bool;
            *is_inline_dest = data.is_inline();
            if data.is_inline() {
                // Zero out the last word for paranoia's sake.
                *((message as *mut u8).offset(size as isize - 4) as *mut u32) = 0;

                let data = data.inline_data();
                let data_size = data.len();
                let padding_start = is_inline_dest.offset(1) as *mut u8;
                let padding_count = Message::payload_padding(padding_start as usize);
                // Zero out padding
                padding_start.write_bytes(0, padding_count);
                let data_size_dest = padding_start.add(padding_count) as *mut usize;
                *data_size_dest = data_size;

                let data_dest = data_size_dest.offset(1) as *mut u8;
                ptr::copy_nonoverlapping(data.as_ptr(), data_dest, data_size);
            }

            let os_result = mach_sys::mach_msg(
                message as *mut _,
                MACH_SEND_MSG,
                (*message).header.msgh_size,
                0,
                MACH_PORT_NULL,
                MACH_MSG_TIMEOUT_NONE,
                MACH_PORT_NULL,
            );
            libc::free(message as *mut _);
            if os_result == MACH_SEND_TOO_LARGE && data.is_inline() {
                let inline_data = data.inline_data();
                {
                    let mut max_inline_size = MAX_INLINE_SIZE.write().unwrap();
                    let inline_len = inline_data.len();
                    if inline_len < *max_inline_size {
                        *max_inline_size = inline_len;
                    }
                }
                return self.send(inline_data, ports, shared_memory_regions);
            }
            if os_result != MACH_MSG_SUCCESS {
                return Err(MachError::from(os_result));
            }
            for outgoing_port in ports {
                mem::forget(outgoing_port);
            }
            for shared_memory_region in shared_memory_regions {
                mem::forget(shared_memory_region);
            }
            Ok(())
        }
    }
}

pub enum OsIpcChannel {
    Sender(OsIpcSender),
    Receiver(OsIpcReceiver),
}

impl OsIpcChannel {
    fn port(&self) -> mach_port_t {
        match *self {
            OsIpcChannel::Sender(ref sender) => sender.port,
            OsIpcChannel::Receiver(ref receiver) => receiver.port.get(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct OsOpaqueIpcChannel {
    port: mach_port_t,
}

impl Drop for OsOpaqueIpcChannel {
    fn drop(&mut self) {
        // Make sure we don't leak!
        debug_assert!(self.port == MACH_PORT_NULL);
    }
}

impl OsOpaqueIpcChannel {
    fn from_name(name: mach_port_t) -> OsOpaqueIpcChannel {
        OsOpaqueIpcChannel { port: name }
    }

    pub fn to_sender(&mut self) -> OsIpcSender {
        OsIpcSender {
            port: mem::replace(&mut self.port, MACH_PORT_NULL),
            nosync_marker: PhantomData,
        }
    }

    pub fn to_receiver(&mut self) -> OsIpcReceiver {
        OsIpcReceiver::from_name(mem::replace(&mut self.port, MACH_PORT_NULL))
    }
}

pub struct OsIpcReceiverSet {
    port: mach_port_t,
    ports: Vec<mach_port_t>,
}

impl OsIpcReceiverSet {
    pub fn new() -> Result<OsIpcReceiverSet, MachError> {
        let port = mach_port_allocate(MACH_PORT_RIGHT_PORT_SET)?;
        Ok(OsIpcReceiverSet {
            port,
            ports: vec![],
        })
    }

    pub fn add(&mut self, receiver: OsIpcReceiver) -> Result<u64, MachError> {
        mach_port_move_member(receiver.extract_port(), self.port)?;
        let receiver_port = receiver.consume_port();
        self.ports.push(receiver_port);
        Ok(receiver_port as u64)
    }

    pub fn select(&mut self) -> Result<Vec<OsIpcSelectionResult>, MachError> {
        select(self.port, BlockingMode::Blocking).map(|result| vec![result])
    }
}

impl Drop for OsIpcReceiverSet {
    fn drop(&mut self) {
        for port in &self.ports {
            mach_port_mod_release(*port, MACH_PORT_RIGHT_RECEIVE).unwrap();
        }
        mach_port_mod_release(self.port, MACH_PORT_RIGHT_PORT_SET).unwrap();
    }
}

pub enum OsIpcSelectionResult {
    DataReceived(u64, IpcMessage),
    ChannelClosed(u64),
}

impl OsIpcSelectionResult {
    pub fn unwrap(self) -> (u64, IpcMessage) {
        match self {
            OsIpcSelectionResult::DataReceived(id, ipc_message) => (id, ipc_message),
            OsIpcSelectionResult::ChannelClosed(id) => {
                panic!("OsIpcSelectionResult::unwrap(): receiver ID {id} was closed!")
            },
        }
    }
}

#[derive(Copy, Clone)]
enum BlockingMode {
    Blocking,
    Nonblocking,
    Timeout(Duration),
}

fn select(
    port: mach_port_t,
    blocking_mode: BlockingMode,
) -> Result<OsIpcSelectionResult, MachError> {
    debug_assert!(port != MACH_PORT_NULL);
    unsafe {
        let mut buffer = [0; SMALL_MESSAGE_SIZE];
        let mut allocated_buffer = None;
        setup_receive_buffer(&mut buffer, port);
        let mut message = &mut buffer[0] as *mut _ as *mut Message;
        let (flags, timeout) = match blocking_mode {
            BlockingMode::Blocking => (MACH_RCV_MSG | MACH_RCV_LARGE, MACH_MSG_TIMEOUT_NONE),
            BlockingMode::Nonblocking => (MACH_RCV_MSG | MACH_RCV_LARGE | MACH_RCV_TIMEOUT, 0),
            BlockingMode::Timeout(duration) => duration
                .as_millis()
                .try_into()
                .map(|ms| (MACH_RCV_MSG | MACH_RCV_LARGE | MACH_RCV_TIMEOUT, ms))
                .unwrap_or((MACH_RCV_MSG | MACH_RCV_LARGE, MACH_MSG_TIMEOUT_NONE)),
        };
        match mach_sys::mach_msg(
            message as *mut _,
            flags,
            0,
            (*message).header.msgh_size,
            port,
            timeout,
            MACH_PORT_NULL,
        ) {
            MACH_RCV_TOO_LARGE => {
                let max_trailer_size =
                    mem::size_of::<mach_sys::mach_msg_max_trailer_t>() as mach_sys::mach_msg_size_t;
                // the actual size gets written into msgh_size by the kernel
                let mut actual_size = (*message).header.msgh_size + max_trailer_size;
                loop {
                    allocated_buffer = Some(libc::malloc(actual_size as size_t));
                    setup_receive_buffer(
                        slice::from_raw_parts_mut(
                            allocated_buffer.unwrap() as *mut u8,
                            actual_size as usize,
                        ),
                        port,
                    );
                    message = allocated_buffer.unwrap() as *mut Message;
                    match mach_sys::mach_msg(
                        message as *mut _,
                        flags,
                        0,
                        actual_size,
                        port,
                        timeout,
                        MACH_PORT_NULL,
                    ) {
                        MACH_MSG_SUCCESS => break,
                        MACH_RCV_TOO_LARGE => {
                            actual_size = (*message).header.msgh_size + max_trailer_size;
                            libc::free(allocated_buffer.unwrap() as *mut _);
                            continue;
                        },
                        os_result => {
                            libc::free(allocated_buffer.unwrap() as *mut _);
                            return Err(MachError::from(os_result));
                        },
                    }
                }
            },
            MACH_MSG_SUCCESS => {},
            os_result => return Err(MachError::from(os_result)),
        }

        let local_port = (*message).header.msgh_local_port;
        if (*message).header.msgh_id == MACH_NOTIFY_NO_SENDERS {
            return Ok(OsIpcSelectionResult::ChannelClosed(local_port as u64));
        }

        let (mut ports, mut shared_memory_regions) = (Vec::new(), Vec::new());
        let mut port_descriptor = message.offset(1) as *mut mach_msg_port_descriptor_t;
        let mut descriptors_remaining = (*message).body.msgh_descriptor_count;
        while descriptors_remaining > 0 {
            if (*port_descriptor).type_() != MACH_MSG_PORT_DESCRIPTOR {
                break;
            }
            ports.push(OsOpaqueIpcChannel::from_name((*port_descriptor).name));
            port_descriptor = port_descriptor.offset(1);
            descriptors_remaining -= 1;
        }

        let mut shared_memory_descriptor = port_descriptor as *mut mach_msg_ool_descriptor_t;
        while descriptors_remaining > 0 {
            debug_assert!((*shared_memory_descriptor).type_() == MACH_MSG_OOL_DESCRIPTOR);
            shared_memory_regions.push(OsIpcSharedMemory::from_raw_parts(
                (*shared_memory_descriptor).address as *mut u8,
                (*shared_memory_descriptor).size as usize,
            ));
            shared_memory_descriptor = shared_memory_descriptor.offset(1);
            descriptors_remaining -= 1;
        }

        let has_inline_data_ptr = shared_memory_descriptor as *mut bool;
        let has_inline_data = *has_inline_data_ptr;
        let payload = if has_inline_data {
            let padding_start = has_inline_data_ptr.offset(1) as *mut u8;
            let padding_count = Message::payload_padding(padding_start as usize);
            let payload_size_ptr = padding_start.add(padding_count) as *mut usize;
            let payload_size = *payload_size_ptr;
            let max_payload_size = message as usize + ((*message).header.msgh_size as usize)
                - (shared_memory_descriptor as usize);
            assert!(payload_size <= max_payload_size);
            let payload_ptr = payload_size_ptr.offset(1) as *mut u8;
            slice::from_raw_parts(payload_ptr, payload_size).to_vec()
        } else {
            let ool_payload = shared_memory_regions
                .pop()
                .expect("Missing OOL shared memory region");
            ool_payload.to_vec()
        };

        if let Some(allocated_buffer) = allocated_buffer {
            libc::free(allocated_buffer)
        }

        Ok(OsIpcSelectionResult::DataReceived(
            local_port as u64,
            IpcMessage::new(payload, ports, shared_memory_regions),
        ))
    }
}

pub struct OsIpcOneShotServer {
    receiver: OsIpcReceiver,
    name: String,
    registration_port: u32,
}

impl Drop for OsIpcOneShotServer {
    fn drop(&mut self) {
        let _ = OsIpcReceiver::unregister_global_name(std::mem::take(&mut self.name));
        deallocate_mach_port(self.registration_port);
    }
}

impl OsIpcOneShotServer {
    pub fn new() -> Result<(OsIpcOneShotServer, String), MachError> {
        let receiver = OsIpcReceiver::new()?;
        let (registration_port, name) = receiver.register_bootstrap_name()?;
        Ok((
            OsIpcOneShotServer {
                receiver,
                name: name.clone(),
                registration_port,
            },
            name,
        ))
    }

    pub fn accept(self) -> Result<(OsIpcReceiver, IpcMessage), MachError> {
        let ipc_message = self.receiver.recv()?;
        Ok((self.receiver.consume(), ipc_message))
    }
}

pub struct OsIpcSharedMemory {
    ptr: *mut u8,
    length: usize,
}

unsafe impl Send for OsIpcSharedMemory {}
unsafe impl Sync for OsIpcSharedMemory {}

impl Drop for OsIpcSharedMemory {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                assert!(
                    mach_sys::vm_deallocate(mach_task_self(), self.ptr as usize, self.length)
                        == KERN_SUCCESS
                );
            }
        }
    }
}

impl Clone for OsIpcSharedMemory {
    fn clone(&self) -> OsIpcSharedMemory {
        let mut address = 0;
        unsafe {
            if !self.ptr.is_null() {
                let err = mach_sys::vm_remap(
                    mach_task_self(),
                    &mut address,
                    self.length,
                    0,
                    1,
                    mach_task_self(),
                    self.ptr as usize,
                    0,
                    &mut 0,
                    &mut 0,
                    VM_INHERIT_SHARE,
                );
                assert!(err == KERN_SUCCESS);
            }
            OsIpcSharedMemory::from_raw_parts(address as *mut u8, self.length)
        }
    }
}

impl PartialEq for OsIpcSharedMemory {
    fn eq(&self, other: &OsIpcSharedMemory) -> bool {
        **self == **other
    }
}

impl Debug for OsIpcSharedMemory {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        (**self).fmt(formatter)
    }
}

impl Deref for OsIpcSharedMemory {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        if self.ptr.is_null() && self.length > 0 {
            panic!("attempted to access a consumed `OsIpcSharedMemory`")
        }
        unsafe { slice::from_raw_parts(self.ptr, self.length) }
    }
}

impl OsIpcSharedMemory {
    unsafe fn from_raw_parts(ptr: *mut u8, length: usize) -> OsIpcSharedMemory {
        OsIpcSharedMemory { ptr, length }
    }

    pub fn from_byte(byte: u8, length: usize) -> OsIpcSharedMemory {
        unsafe {
            let address = allocate_vm_pages(length);
            for element in slice::from_raw_parts_mut(address, length) {
                *element = byte;
            }
            OsIpcSharedMemory::from_raw_parts(address, length)
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> OsIpcSharedMemory {
        unsafe {
            let address = allocate_vm_pages(bytes.len());
            ptr::copy_nonoverlapping(bytes.as_ptr(), address, bytes.len());
            OsIpcSharedMemory::from_raw_parts(address, bytes.len())
        }
    }
}

unsafe fn allocate_vm_pages(length: usize) -> *mut u8 {
    let mut address = 0;
    let result = mach_sys::vm_allocate(mach_task_self(), &mut address, length, 1);
    if result != KERN_SUCCESS {
        panic!("`vm_allocate()` failed: {}", result);
    }
    address as *mut u8
}

unsafe fn setup_receive_buffer(buffer: &mut [u8], port_name: mach_port_t) {
    let message = buffer.as_mut_ptr() as *mut mach_msg_header_t;
    (*message).msgh_local_port = port_name;
    (*message).msgh_size = buffer.len() as u32
}

unsafe fn mach_task_self() -> mach_port_t {
    mach_task_self_
}

fn deallocate_mach_port(port: mach_port_t) {
    // mach_port_deallocate and mach_port_mod_refs are very similar, except that
    // mach_port_mod_refs returns an error when there are no receivers for the port,
    // causing the sender port to never be deallocated. mach_port_deallocate handles
    // this case correctly and is therefore important to avoid dangling port leaks.
    let err = unsafe { mach_port_deallocate(mach_task_self(), port) };
    if err != KERN_SUCCESS {
        panic!("mach_port_deallocate({}) failed: {:?}", port, err);
    }
}

#[repr(C)]
struct Message {
    header: mach_msg_header_t,
    body: mach_msg_body_t,
}

impl Message {
    fn payload_padding(unaligned: usize) -> usize {
        ((unaligned + 7) & !7) - unaligned // 8 byte alignment
    }

    fn size_of(data: &SendData, port_length: usize, shared_memory_length: usize) -> usize {
        let mut size = mem::size_of::<Message>()
            + mem::size_of::<mach_msg_port_descriptor_t>() * port_length
            + mem::size_of::<mach_msg_ool_descriptor_t>() * shared_memory_length
            + mem::size_of::<bool>();

        if data.is_inline() {
            // rustc panics in debug mode for unaligned accesses.
            // so include padding to start payload at 8-byte aligned address
            size += Self::payload_padding(size);
            size += mem::size_of::<usize>() + data.inline_data().len();
        }

        // Round up to the next 4 bytes; mach_msg_send returns an error for unaligned sizes.
        if (size & 0x3) != 0 {
            size = (size & !0x3) + 4;
        }

        size
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KernelError {
    Success,
    NoSpace,
    InvalidName,
    InvalidRight,
    InvalidValue,
    InvalidCapability,
    UrefsOverflow,
    NotInSet,
    Unknown(kern_return_t),
}

impl fmt::Display for KernelError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KernelError::Success => write!(fmt, "Success."),
            KernelError::NoSpace => write!(fmt, "No room in IPC name space for another right."),
            KernelError::InvalidName => write!(fmt, "Name doesn't denote a right in the task."),
            KernelError::InvalidRight => {
                write!(fmt, "Name denotes a right, but not an appropriate right.")
            },
            KernelError::InvalidValue => write!(fmt, "Blatant range error."),
            KernelError::InvalidCapability => {
                write!(fmt, "The supplied (port) capability is improper.")
            },
            KernelError::UrefsOverflow => {
                write!(fmt, "Operation would overflow limit on user-references.")
            },
            KernelError::NotInSet => write!(fmt, "Receive right is not a member of a port set."),
            KernelError::Unknown(code) => write!(fmt, "Unknown kernel error: {:x}", code),
        }
    }
}

impl StdError for KernelError {}

impl From<kern_return_t> for KernelError {
    fn from(code: kern_return_t) -> KernelError {
        match code {
            KERN_SUCCESS => KernelError::Success,
            KERN_NO_SPACE => KernelError::NoSpace,
            KERN_INVALID_NAME => KernelError::InvalidName,
            KERN_INVALID_RIGHT => KernelError::InvalidRight,
            KERN_INVALID_VALUE => KernelError::InvalidValue,
            KERN_INVALID_CAPABILITY => KernelError::InvalidCapability,
            KERN_UREFS_OVERFLOW => KernelError::UrefsOverflow,
            KERN_NOT_IN_SET => KernelError::NotInSet,
            code => KernelError::Unknown(code),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MachError {
    Success,
    Kernel(KernelError),
    IpcSpace,
    VmSpace,
    IpcKernel,
    VmKernel,
    RcvInProgress,
    RcvInvalidName,
    RcvTimedOut,
    RcvTooLarge,
    RcvInterrupted,
    RcvPortChanged,
    RcvInvalidNotify,
    RcvInvalidData,
    RcvPortDied,
    RcvInSet,
    RcvHeaderError,
    RcvBodyError,
    RcvInvalidType,
    RcvScatterSmall,
    RcvInvalidTrailer,
    RcvInProgressTimed,
    NotifyNoSenders,
    SendInterrupted,
    SendInvalidData,
    SendInvalidDest,
    SendInvalidHeader,
    SendInvalidMemory,
    SendInvalidNotify,
    SendInvalidReply,
    SendInvalidRight,
    SendInvalidRtOolSize,
    SendInvalidTrailer,
    SendInvalidType,
    SendInvalidVoucher,
    SendInProgress,
    SendMsgTooSmall,
    SendNoBuffer,
    SendTimedOut,
    SendTooLarge,
    Unknown(mach_msg_return_t),
}

impl MachError {
    #[allow(dead_code)]
    pub fn channel_is_closed(&self) -> bool {
        *self == MachError::NotifyNoSenders
    }
}

impl fmt::Display for MachError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MachError::Success => write!(fmt, "Success"),
            MachError::Kernel(kernel_error) => fmt::Display::fmt(&kernel_error, fmt),
            MachError::IpcSpace => write!(
                fmt,
                "No room in IPC name space for another capability name."
            ),
            MachError::VmSpace => {
                write!(fmt, "No room in VM address space for out-of-line memory.")
            },
            MachError::IpcKernel => {
                write!(fmt, "Kernel resource shortage handling an IPC capability.")
            },
            MachError::VmKernel => {
                write!(fmt, "Kernel resource shortage handling out-of-line memory.")
            },
            MachError::SendInProgress => {
                write!(fmt, "Thread is waiting to send.  (Internal use only.)")
            },
            MachError::SendInvalidData => write!(fmt, "Bogus in-line data."),
            MachError::SendInvalidDest => write!(fmt, "Bogus destination port."),
            MachError::SendTimedOut => write!(fmt, "Message not sent before timeout expired."),
            MachError::SendInvalidVoucher => write!(fmt, "Bogus voucher port."),
            MachError::SendInterrupted => write!(fmt, "Software interrupt."),
            MachError::SendMsgTooSmall => write!(fmt, "Data doesn't contain a complete message."),
            MachError::SendInvalidReply => write!(fmt, "Bogus reply port."),
            MachError::SendInvalidRight => write!(fmt, "Bogus port rights in the message body."),
            MachError::SendInvalidNotify => write!(fmt, "Bogus notify port argument."),
            MachError::SendInvalidMemory => write!(fmt, "Invalid out-of-line memory pointer."),
            MachError::SendNoBuffer => write!(fmt, "No message buffer is available."),
            MachError::SendTooLarge => write!(fmt, "Send is too large for port"),
            MachError::SendInvalidType => write!(fmt, "Invalid msg-type specification."),
            MachError::SendInvalidHeader => write!(fmt, "A field in the header had a bad value."),
            MachError::SendInvalidTrailer => {
                write!(fmt, "The trailer to be sent does not match kernel format.")
            },
            MachError::SendInvalidRtOolSize => {
                write!(fmt, "compatibility: no longer a returned error")
            },
            MachError::RcvInProgress => {
                write!(fmt, "Thread is waiting for receive.  (Internal use only.)")
            },
            MachError::RcvInvalidName => write!(fmt, "Bogus name for receive port/port-set."),
            MachError::RcvTimedOut => write!(fmt, "Didn't get a message within the timeout value."),
            MachError::RcvTooLarge => {
                write!(fmt, "Message buffer is not large enough for inline data.")
            },
            MachError::RcvInterrupted => write!(fmt, "Software interrupt."),
            MachError::RcvPortChanged => write!(fmt, "compatibility: no longer a returned error"),
            MachError::RcvInvalidNotify => write!(fmt, "Bogus notify port argument."),
            MachError::RcvInvalidData => write!(fmt, "Bogus message buffer for inline data."),
            MachError::RcvPortDied => write!(fmt, "Port/set was sent away/died during receive."),
            MachError::RcvInSet => write!(fmt, "compatibility: no longer a returned error"),
            MachError::RcvHeaderError => {
                write!(fmt, "Error receiving message header.  See special bits.")
            },
            MachError::RcvBodyError => {
                write!(fmt, "Error receiving message body.  See special bits.")
            },
            MachError::RcvInvalidType => {
                write!(fmt, "Invalid msg-type specification in scatter list.")
            },
            MachError::RcvScatterSmall => {
                write!(fmt, "Out-of-line overwrite region is not large enough")
            },
            MachError::RcvInvalidTrailer => write!(
                fmt,
                "trailer type or number of trailer elements not supported"
            ),
            MachError::RcvInProgressTimed => write!(
                fmt,
                "Waiting for receive with timeout. (Internal use only.)"
            ),
            MachError::NotifyNoSenders => write!(fmt, "No senders exist for this port."),
            MachError::Unknown(mach_error_number) => {
                write!(fmt, "Unknown Mach error: {:x}", mach_error_number)
            },
        }
    }
}

impl StdError for MachError {}

impl From<MachError> for bincode::Error {
    fn from(mach_error: MachError) -> Self {
        io::Error::from(mach_error).into()
    }
}

impl From<mach_msg_return_t> for MachError {
    fn from(code: mach_msg_return_t) -> MachError {
        match code {
            MACH_MSG_SUCCESS => MachError::Success,
            MACH_MSG_IPC_KERNEL => MachError::IpcKernel,
            MACH_MSG_IPC_SPACE => MachError::IpcSpace,
            MACH_MSG_VM_KERNEL => MachError::VmKernel,
            MACH_MSG_VM_SPACE => MachError::VmSpace,
            MACH_RCV_BODY_ERROR => MachError::RcvBodyError,
            MACH_RCV_HEADER_ERROR => MachError::RcvHeaderError,
            MACH_RCV_INTERRUPTED => MachError::RcvInterrupted,
            MACH_RCV_INVALID_DATA => MachError::RcvInvalidData,
            MACH_RCV_INVALID_NAME => MachError::RcvInvalidName,
            MACH_RCV_INVALID_NOTIFY => MachError::RcvInvalidNotify,
            MACH_RCV_INVALID_TRAILER => MachError::RcvInvalidTrailer,
            MACH_RCV_INVALID_TYPE => MachError::RcvInvalidType,
            MACH_RCV_IN_PROGRESS => MachError::RcvInProgress,
            MACH_RCV_IN_PROGRESS_TIMED => MachError::RcvInProgressTimed,
            MACH_RCV_IN_SET => MachError::RcvInSet,
            MACH_RCV_PORT_CHANGED => MachError::RcvPortChanged,
            MACH_RCV_PORT_DIED => MachError::RcvPortDied,
            MACH_RCV_SCATTER_SMALL => MachError::RcvScatterSmall,
            MACH_RCV_TIMED_OUT => MachError::RcvTimedOut,
            MACH_RCV_TOO_LARGE => MachError::RcvTooLarge,
            MACH_NOTIFY_NO_SENDERS => MachError::NotifyNoSenders,
            MACH_SEND_INTERRUPTED => MachError::SendInterrupted,
            MACH_SEND_INVALID_DATA => MachError::SendInvalidData,
            MACH_SEND_INVALID_DEST => MachError::SendInvalidDest,
            MACH_SEND_INVALID_HEADER => MachError::SendInvalidHeader,
            MACH_SEND_INVALID_MEMORY => MachError::SendInvalidMemory,
            MACH_SEND_INVALID_NOTIFY => MachError::SendInvalidNotify,
            MACH_SEND_INVALID_REPLY => MachError::SendInvalidReply,
            MACH_SEND_INVALID_RIGHT => MachError::SendInvalidRight,
            MACH_SEND_INVALID_RT_OOL_SIZE => MachError::SendInvalidRtOolSize,
            MACH_SEND_INVALID_TRAILER => MachError::SendInvalidTrailer,
            MACH_SEND_INVALID_TYPE => MachError::SendInvalidType,
            MACH_SEND_INVALID_VOUCHER => MachError::SendInvalidVoucher,
            MACH_SEND_IN_PROGRESS => MachError::SendInProgress,
            MACH_SEND_MSG_TOO_SMALL => MachError::SendMsgTooSmall,
            MACH_SEND_NO_BUFFER => MachError::SendNoBuffer,
            MACH_SEND_TIMED_OUT => MachError::SendTimedOut,
            MACH_SEND_TOO_LARGE => MachError::SendTooLarge,
            code => MachError::Unknown(code),
        }
    }
}

impl From<KernelError> for MachError {
    fn from(kernel_error: KernelError) -> MachError {
        MachError::Kernel(kernel_error)
    }
}

impl From<MachError> for ipc::TryRecvError {
    fn from(error: MachError) -> Self {
        match error {
            MachError::NotifyNoSenders => ipc::TryRecvError::IpcError(ipc::IpcError::Disconnected),
            MachError::RcvTimedOut => ipc::TryRecvError::Empty,
            e => ipc::TryRecvError::IpcError(ipc::IpcError::Io(io::Error::from(e))),
        }
    }
}

impl From<MachError> for ipc::IpcError {
    fn from(error: MachError) -> Self {
        match error {
            MachError::NotifyNoSenders => ipc::IpcError::Disconnected,
            e => ipc::IpcError::Io(io::Error::from(e)),
        }
    }
}

impl From<MachError> for io::Error {
    /// These error descriptions are from `mach/message.h` and `mach/kern_return.h`.
    fn from(mach_error: MachError) -> io::Error {
        let kind = match mach_error {
            MachError::Success => io::ErrorKind::Other,
            MachError::Kernel(KernelError::Success) => io::ErrorKind::Other,
            MachError::Kernel(KernelError::NoSpace) => io::ErrorKind::Other,
            MachError::Kernel(KernelError::InvalidName) => io::ErrorKind::Other,
            MachError::Kernel(KernelError::InvalidRight) => io::ErrorKind::Other,
            MachError::Kernel(KernelError::InvalidValue) => io::ErrorKind::Other,
            MachError::Kernel(KernelError::InvalidCapability) => io::ErrorKind::Other,
            MachError::Kernel(KernelError::UrefsOverflow) => io::ErrorKind::Other,
            MachError::Kernel(KernelError::NotInSet) => io::ErrorKind::Other,
            MachError::Kernel(KernelError::Unknown(_)) => io::ErrorKind::Other,
            MachError::IpcSpace => io::ErrorKind::Other,
            MachError::VmSpace => io::ErrorKind::Other,
            MachError::IpcKernel => io::ErrorKind::Other,
            MachError::VmKernel => io::ErrorKind::Other,
            MachError::SendInProgress => io::ErrorKind::Interrupted,
            MachError::SendInvalidData => io::ErrorKind::InvalidData,
            MachError::SendInvalidDest => io::ErrorKind::NotFound,
            MachError::SendTimedOut => io::ErrorKind::TimedOut,
            MachError::SendInvalidVoucher => io::ErrorKind::NotFound,
            MachError::SendInterrupted => io::ErrorKind::Interrupted,
            MachError::SendMsgTooSmall => io::ErrorKind::InvalidData,
            MachError::SendInvalidReply => io::ErrorKind::InvalidInput,
            MachError::SendInvalidRight => io::ErrorKind::InvalidInput,
            MachError::SendInvalidNotify => io::ErrorKind::InvalidInput,
            MachError::SendInvalidMemory => io::ErrorKind::InvalidInput,
            MachError::SendNoBuffer => io::ErrorKind::Other,
            MachError::SendTooLarge => io::ErrorKind::InvalidData,
            MachError::SendInvalidType => io::ErrorKind::InvalidInput,
            MachError::SendInvalidHeader => io::ErrorKind::InvalidInput,
            MachError::SendInvalidTrailer => io::ErrorKind::InvalidData,
            MachError::SendInvalidRtOolSize => io::ErrorKind::Other,
            MachError::RcvInProgress => io::ErrorKind::Interrupted,
            MachError::RcvInvalidName => io::ErrorKind::InvalidInput,
            MachError::RcvTimedOut => io::ErrorKind::TimedOut,
            MachError::RcvTooLarge => io::ErrorKind::InvalidInput,
            MachError::RcvInterrupted => io::ErrorKind::Interrupted,
            MachError::RcvPortChanged => io::ErrorKind::Other,
            MachError::RcvInvalidNotify => io::ErrorKind::InvalidInput,
            MachError::RcvInvalidData => io::ErrorKind::InvalidInput,
            MachError::RcvPortDied => io::ErrorKind::BrokenPipe,
            MachError::RcvInSet => io::ErrorKind::Other,
            MachError::RcvHeaderError => io::ErrorKind::Other,
            MachError::RcvBodyError => io::ErrorKind::Other,
            MachError::RcvInvalidType => io::ErrorKind::InvalidInput,
            MachError::RcvScatterSmall => io::ErrorKind::InvalidInput,
            MachError::RcvInvalidTrailer => io::ErrorKind::InvalidInput,
            MachError::RcvInProgressTimed => io::ErrorKind::Interrupted,
            MachError::NotifyNoSenders => io::ErrorKind::ConnectionReset,
            MachError::Unknown(_) => io::ErrorKind::Other,
        };
        io::Error::new(kind, mach_error)
    }
}

extern "C" {
    fn bootstrap_register2(
        bp: mach_port_t,
        service_name: name_t,
        sp: mach_port_t,
        flags: u64,
    ) -> kern_return_t;
    fn bootstrap_look_up(
        bp: mach_port_t,
        service_name: name_t,
        sp: *mut mach_port_t,
    ) -> kern_return_t;
}
