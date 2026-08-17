// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::ipc::{self, IpcMessage};
use bincode;
use crossbeam_channel::{self, Receiver, RecvTimeoutError, Select, Sender, TryRecvError};
use lazy_static::lazy_static;
use std::cell::{Ref, RefCell};
use std::cmp::PartialEq;
use std::collections::hash_map::HashMap;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Formatter};
use std::io;
use std::ops::{Deref, RangeFrom};
use std::slice;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::usize;
use uuid::Uuid;

#[derive(Clone)]
struct ServerRecord {
    sender: OsIpcSender,
    conn_sender: Sender<bool>,
    conn_receiver: Receiver<bool>,
}

impl ServerRecord {
    fn new(sender: OsIpcSender) -> ServerRecord {
        let (tx, rx) = crossbeam_channel::unbounded::<bool>();
        ServerRecord {
            sender: sender,
            conn_sender: tx,
            conn_receiver: rx,
        }
    }

    fn accept(&self) {
        self.conn_receiver.recv().unwrap();
    }

    fn connect(&self) {
        self.conn_sender.send(true).unwrap();
    }
}

lazy_static! {
    static ref ONE_SHOT_SERVERS: Mutex<HashMap<String, ServerRecord>> = Mutex::new(HashMap::new());
}

struct ChannelMessage(IpcMessage);

pub fn channel() -> Result<(OsIpcSender, OsIpcReceiver), ChannelError> {
    let (base_sender, base_receiver) = crossbeam_channel::unbounded::<ChannelMessage>();
    Ok((
        OsIpcSender::new(base_sender),
        OsIpcReceiver::new(base_receiver),
    ))
}

#[derive(Debug)]
pub struct OsIpcReceiver {
    receiver: RefCell<Option<crossbeam_channel::Receiver<ChannelMessage>>>,
}

impl PartialEq for OsIpcReceiver {
    fn eq(&self, other: &OsIpcReceiver) -> bool {
        self.receiver.borrow().as_ref().map(|rx| rx as *const _)
            == other.receiver.borrow().as_ref().map(|rx| rx as *const _)
    }
}

impl OsIpcReceiver {
    fn new(receiver: Receiver<ChannelMessage>) -> OsIpcReceiver {
        OsIpcReceiver {
            receiver: RefCell::new(Some(receiver)),
        }
    }

    pub fn consume(&self) -> OsIpcReceiver {
        OsIpcReceiver {
            receiver: RefCell::new(self.receiver.borrow_mut().take()),
        }
    }

    pub fn recv(&self) -> Result<IpcMessage, ChannelError> {
        let r = self.receiver.borrow();
        let r = r.as_ref().unwrap();
        match r.recv() {
            Ok(ChannelMessage(ipc_message)) => Ok(ipc_message),
            Err(_) => Err(ChannelError::ChannelClosedError),
        }
    }

    pub fn try_recv(&self) -> Result<IpcMessage, ChannelError> {
        let r = self.receiver.borrow();
        let r = r.as_ref().unwrap();
        match r.try_recv() {
            Ok(ChannelMessage(ipc_message)) => Ok(ipc_message),
            Err(e) => match e {
                TryRecvError::Empty => Err(ChannelError::ChannelEmpty),
                TryRecvError::Disconnected => Err(ChannelError::ChannelClosedError),
            },
        }
    }

    pub fn try_recv_timeout(&self, duration: Duration) -> Result<IpcMessage, ChannelError> {
        let r = self.receiver.borrow();
        let r = r.as_ref().unwrap();
        match r.recv_timeout(duration) {
            Ok(ChannelMessage(ipc_message)) => Ok(ipc_message),
            Err(e) => match e {
                RecvTimeoutError::Timeout => Err(ChannelError::ChannelEmpty),
                RecvTimeoutError::Disconnected => Err(ChannelError::ChannelClosedError),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct OsIpcSender {
    sender: RefCell<Sender<ChannelMessage>>,
}

impl PartialEq for OsIpcSender {
    fn eq(&self, other: &OsIpcSender) -> bool {
        &*self.sender.borrow() as *const _ == &*other.sender.borrow() as *const _
    }
}

impl OsIpcSender {
    fn new(sender: Sender<ChannelMessage>) -> OsIpcSender {
        OsIpcSender {
            sender: RefCell::new(sender),
        }
    }

    pub fn connect(name: String) -> Result<OsIpcSender, ChannelError> {
        let record = ONE_SHOT_SERVERS.lock().unwrap().get(&name).unwrap().clone();
        record.connect();
        Ok(record.sender)
    }

    pub fn get_max_fragment_size() -> usize {
        usize::MAX
    }

    pub fn send(
        &self,
        data: &[u8],
        ports: Vec<OsIpcChannel>,
        shared_memory_regions: Vec<OsIpcSharedMemory>,
    ) -> Result<(), ChannelError> {
        let os_ipc_channels = ports.into_iter().map(OsOpaqueIpcChannel::new).collect();
        let ipc_message = IpcMessage::new(data.to_vec(), os_ipc_channels, shared_memory_regions);
        Ok(self
            .sender
            .borrow()
            .send(ChannelMessage(ipc_message))
            .map_err(|_| ChannelError::BrokenPipeError)?)
    }
}

pub struct OsIpcReceiverSet {
    incrementor: RangeFrom<u64>,
    receiver_ids: Vec<u64>,
    receivers: Vec<OsIpcReceiver>,
}

impl OsIpcReceiverSet {
    pub fn new() -> Result<OsIpcReceiverSet, ChannelError> {
        Ok(OsIpcReceiverSet {
            incrementor: 0..,
            receiver_ids: vec![],
            receivers: vec![],
        })
    }

    pub fn add(&mut self, receiver: OsIpcReceiver) -> Result<u64, ChannelError> {
        let last_index = self.incrementor.next().unwrap();
        self.receiver_ids.push(last_index);
        self.receivers.push(receiver.consume());
        Ok(last_index)
    }

    pub fn select(&mut self) -> Result<Vec<OsIpcSelectionResult>, ChannelError> {
        if self.receivers.is_empty() {
            return Err(ChannelError::UnknownError);
        }

        struct Remove(usize, u64);

        // FIXME: Remove early returns and explicitly drop `borrows` when lifetimes are non-lexical
        let Remove(r_index, r_id) = {
            let borrows: Vec<_> = self
                .receivers
                .iter()
                .map(|r| Ref::map(r.receiver.borrow(), |o| o.as_ref().unwrap()))
                .collect();

            let mut select = Select::new();
            for r in &borrows {
                select.recv(&r);
            }
            let res = select.select();
            let receiver_index = res.index();
            let receiver_id = self.receiver_ids[receiver_index];
            if let Ok(ChannelMessage(ipc_message)) = res.recv(&borrows[receiver_index as usize]) {
                return Ok(vec![OsIpcSelectionResult::DataReceived(
                    receiver_id,
                    ipc_message,
                )]);
            } else {
                Remove(receiver_index, receiver_id)
            }
        };
        self.receivers.remove(r_index);
        self.receiver_ids.remove(r_index);
        Ok(vec![OsIpcSelectionResult::ChannelClosed(r_id)])
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
                panic!(
                    "OsIpcSelectionResult::unwrap(): receiver ID {} was closed!",
                    id
                )
            },
        }
    }
}

pub struct OsIpcOneShotServer {
    receiver: OsIpcReceiver,
    name: String,
}

impl OsIpcOneShotServer {
    pub fn new() -> Result<(OsIpcOneShotServer, String), ChannelError> {
        let (sender, receiver) = channel()?;

        let name = Uuid::new_v4().to_string();
        let record = ServerRecord::new(sender);
        ONE_SHOT_SERVERS
            .lock()
            .unwrap()
            .insert(name.clone(), record);
        Ok((
            OsIpcOneShotServer {
                receiver: receiver,
                name: name.clone(),
            },
            name.clone(),
        ))
    }

    pub fn accept(self) -> Result<(OsIpcReceiver, IpcMessage), ChannelError> {
        let record = ONE_SHOT_SERVERS
            .lock()
            .unwrap()
            .get(&self.name)
            .unwrap()
            .clone();
        record.accept();
        ONE_SHOT_SERVERS.lock().unwrap().remove(&self.name).unwrap();
        let ipc_message = self.receiver.recv()?;
        Ok((self.receiver, ipc_message))
    }
}

#[derive(PartialEq, Debug)]
pub enum OsIpcChannel {
    Sender(OsIpcSender),
    Receiver(OsIpcReceiver),
}

#[derive(PartialEq, Debug)]
pub struct OsOpaqueIpcChannel {
    channel: RefCell<Option<OsIpcChannel>>,
}

impl OsOpaqueIpcChannel {
    fn new(channel: OsIpcChannel) -> OsOpaqueIpcChannel {
        OsOpaqueIpcChannel {
            channel: RefCell::new(Some(channel)),
        }
    }

    pub fn to_receiver(&self) -> OsIpcReceiver {
        match self.channel.borrow_mut().take().unwrap() {
            OsIpcChannel::Sender(_) => panic!("Opaque channel is not a receiver!"),
            OsIpcChannel::Receiver(r) => r,
        }
    }

    pub fn to_sender(&mut self) -> OsIpcSender {
        match self.channel.borrow_mut().take().unwrap() {
            OsIpcChannel::Sender(s) => s,
            OsIpcChannel::Receiver(_) => panic!("Opaque channel is not a sender!"),
        }
    }
}

pub struct OsIpcSharedMemory {
    ptr: *mut u8,
    length: usize,
    data: Arc<Vec<u8>>,
}

unsafe impl Send for OsIpcSharedMemory {}
unsafe impl Sync for OsIpcSharedMemory {}

impl Clone for OsIpcSharedMemory {
    fn clone(&self) -> OsIpcSharedMemory {
        OsIpcSharedMemory {
            ptr: self.ptr,
            length: self.length,
            data: self.data.clone(),
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
        if self.ptr.is_null() {
            panic!("attempted to access a consumed `OsIpcSharedMemory`")
        }
        unsafe { slice::from_raw_parts(self.ptr, self.length) }
    }
}

impl OsIpcSharedMemory {
    pub fn from_byte(byte: u8, length: usize) -> OsIpcSharedMemory {
        let mut v = Arc::new(vec![byte; length]);
        OsIpcSharedMemory {
            ptr: Arc::get_mut(&mut v).unwrap().as_mut_ptr(),
            length: length,
            data: v,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> OsIpcSharedMemory {
        let mut v = Arc::new(bytes.to_vec());
        OsIpcSharedMemory {
            ptr: Arc::get_mut(&mut v).unwrap().as_mut_ptr(),
            length: v.len(),
            data: v,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ChannelError {
    ChannelClosedError,
    BrokenPipeError,
    ChannelEmpty,
    UnknownError,
}

impl ChannelError {
    #[allow(dead_code)]
    pub fn channel_is_closed(&self) -> bool {
        *self == ChannelError::ChannelClosedError
    }
}

impl fmt::Display for ChannelError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChannelError::ChannelClosedError => write!(fmt, "channel closed"),
            ChannelError::BrokenPipeError => write!(fmt, "broken pipe"),
            ChannelError::ChannelEmpty => write!(fmt, "channel empty"),
            ChannelError::UnknownError => write!(fmt, "unknown error"),
        }
    }
}

impl StdError for ChannelError {}

impl From<ChannelError> for bincode::Error {
    fn from(crossbeam_error: ChannelError) -> Self {
        io::Error::from(crossbeam_error).into()
    }
}

impl From<ChannelError> for ipc::IpcError {
    fn from(error: ChannelError) -> Self {
        match error {
            ChannelError::ChannelClosedError => ipc::IpcError::Disconnected,
            e => ipc::IpcError::Bincode(io::Error::from(e).into()),
        }
    }
}

impl From<ChannelError> for ipc::TryRecvError {
    fn from(error: ChannelError) -> Self {
        match error {
            ChannelError::ChannelClosedError => {
                ipc::TryRecvError::IpcError(ipc::IpcError::Disconnected)
            },
            ChannelError::ChannelEmpty => ipc::TryRecvError::Empty,
            e => ipc::TryRecvError::IpcError(ipc::IpcError::Bincode(io::Error::from(e).into())),
        }
    }
}

impl From<ChannelError> for io::Error {
    fn from(crossbeam_error: ChannelError) -> io::Error {
        match crossbeam_error {
            ChannelError::ChannelClosedError => io::Error::new(
                io::ErrorKind::ConnectionReset,
                "crossbeam-channel sender closed",
            ),
            ChannelError::ChannelEmpty => io::Error::new(
                io::ErrorKind::ConnectionReset,
                "crossbeam-channel receiver has no received messages",
            ),
            ChannelError::BrokenPipeError => io::Error::new(
                io::ErrorKind::BrokenPipe,
                "crossbeam-channel receiver closed",
            ),
            ChannelError::UnknownError => {
                io::Error::new(io::ErrorKind::Other, "Other crossbeam-channel error")
            },
        }
    }
}
