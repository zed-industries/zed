// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::platform::{self, OsIpcChannel, OsIpcReceiver, OsIpcReceiverSet, OsIpcSender};
use crate::platform::{
    OsIpcOneShotServer, OsIpcSelectionResult, OsIpcSharedMemory, OsOpaqueIpcChannel,
};

use bincode;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
use std::cmp::min;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Formatter};
use std::io;
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::time::Duration;

thread_local! {
    static OS_IPC_CHANNELS_FOR_DESERIALIZATION: RefCell<Vec<OsOpaqueIpcChannel>> =
        const { RefCell::new(Vec::new()) }
}
thread_local! {
    static OS_IPC_SHARED_MEMORY_REGIONS_FOR_DESERIALIZATION:
        RefCell<Vec<Option<OsIpcSharedMemory>>> = const { RefCell::new(Vec::new()) }
}
thread_local! {
    static OS_IPC_CHANNELS_FOR_SERIALIZATION: RefCell<Vec<OsIpcChannel>> = const { RefCell::new(Vec::new()) }
}
thread_local! {
    static OS_IPC_SHARED_MEMORY_REGIONS_FOR_SERIALIZATION: RefCell<Vec<OsIpcSharedMemory>> =
        const { RefCell::new(Vec::new()) }
}

#[derive(Debug)]
pub enum IpcError {
    Bincode(bincode::Error),
    Io(io::Error),
    Disconnected,
}

impl fmt::Display for IpcError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpcError::Bincode(ref err) => write!(fmt, "bincode error: {}", err),
            IpcError::Io(ref err) => write!(fmt, "io error: {}", err),
            IpcError::Disconnected => write!(fmt, "disconnected"),
        }
    }
}

impl StdError for IpcError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            IpcError::Bincode(ref err) => Some(err),
            IpcError::Io(ref err) => Some(err),
            IpcError::Disconnected => None,
        }
    }
}

#[derive(Debug)]
pub enum TryRecvError {
    IpcError(IpcError),
    Empty,
}

impl fmt::Display for TryRecvError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TryRecvError::IpcError(ref err) => write!(fmt, "ipc error: {}", err),
            TryRecvError::Empty => write!(fmt, "empty"),
        }
    }
}

impl StdError for TryRecvError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            TryRecvError::IpcError(ref err) => Some(err),
            TryRecvError::Empty => None,
        }
    }
}

/// Create a connected [IpcSender] and [IpcReceiver] that
/// transfer messages of a given type provided by type `T`
/// or inferred by the types of messages sent by the sender.
///
/// Messages sent by the sender will be available to the
/// receiver even if the sender or receiver has been moved
/// to a different process. In addition, receivers and senders
/// may be sent over an existing channel.
///
/// # Examples
///
/// ```
/// # use ipc_channel::ipc;
///
/// let payload = "Hello, World!".to_owned();
///
/// // Create a channel
/// let (tx, rx) = ipc::channel().unwrap();
///
/// // Send data
/// tx.send(payload).unwrap();
///
/// // Receive the data
/// let response = rx.recv().unwrap();
///
/// assert_eq!(response, "Hello, World!".to_owned());
/// ```
///
/// [IpcSender]: struct.IpcSender.html
/// [IpcReceiver]: struct.IpcReceiver.html
pub fn channel<T>() -> Result<(IpcSender<T>, IpcReceiver<T>), io::Error>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    let (os_sender, os_receiver) = platform::channel()?;
    let ipc_receiver = IpcReceiver {
        os_receiver,
        phantom: PhantomData,
    };
    let ipc_sender = IpcSender {
        os_sender,
        phantom: PhantomData,
    };
    Ok((ipc_sender, ipc_receiver))
}

/// Create a connected [IpcBytesSender] and [IpcBytesReceiver].
///
/// Note: The [IpcBytesSender] transfers messages of the type `[u8]`
/// and the [IpcBytesReceiver] receives a `Vec<u8>`. This sender/receiver
/// type does not serialize/deserialize messages through `serde`, making
/// it more efficient where applicable.
///
/// # Examples
///
/// ```
/// # use ipc_channel::ipc;
///
/// let payload = b"'Tis but a scratch!!";
///
/// // Create a channel
/// let (tx, rx) = ipc::bytes_channel().unwrap();
///
/// // Send data
/// tx.send(payload).unwrap();
///
/// // Receive the data
/// let response = rx.recv().unwrap();
///
/// assert_eq!(response, payload);
/// ```
///
/// [IpcBytesReceiver]: struct.IpcBytesReceiver.html
/// [IpcBytesSender]: struct.IpcBytesSender.html
pub fn bytes_channel() -> Result<(IpcBytesSender, IpcBytesReceiver), io::Error> {
    let (os_sender, os_receiver) = platform::channel()?;
    let ipc_bytes_receiver = IpcBytesReceiver { os_receiver };
    let ipc_bytes_sender = IpcBytesSender { os_sender };
    Ok((ipc_bytes_sender, ipc_bytes_receiver))
}

/// Receiving end of a channel using serialized messages.
///
/// # Examples
///
/// ## Blocking IO
///
/// ```
/// # use ipc_channel::ipc;
/// #
/// # let (tx, rx) = ipc::channel().unwrap();
/// #
/// # let q = "Answer to the ultimate question of life, the universe, and everything";
/// #
/// # tx.send(q.to_owned()).unwrap();
/// let response = rx.recv().unwrap();
/// println!("Received data...");
/// # assert_eq!(response, q);
/// ```
///
/// ## Non-blocking IO
///
/// ```
/// # use ipc_channel::ipc;
/// #
/// # let (tx, rx) = ipc::channel().unwrap();
/// #
/// # let answer = "42";
/// #
/// # tx.send(answer.to_owned()).unwrap();
/// loop {
///     match rx.try_recv() {
///         Ok(res) => {
///             // Do something interesting with your result
///             println!("Received data...");
///             break;
///         },
///         Err(_) => {
///             // Do something else useful while we wait
///             println!("Still waiting...");
///         }
///     }
/// }
/// ```
///
/// ## Embedding Receivers
///
/// ```
/// # use ipc_channel::ipc;
/// #
/// let (tx, rx) = ipc::channel().unwrap();
/// let (embedded_tx, embedded_rx) = ipc::channel().unwrap();
/// # let data = [0x45, 0x6d, 0x62, 0x65, 0x64, 0x64, 0x65, 0x64, 0x00];
/// // Send the IpcReceiver
/// tx.send(embedded_rx).unwrap();
/// # embedded_tx.send(data.to_owned()).unwrap();
/// // Receive the sent IpcReceiver
/// let received_rx = rx.recv().unwrap();
/// // Receive any data sent to the received IpcReceiver
/// let rx_data = received_rx.recv().unwrap();
/// # assert_eq!(rx_data, data);
/// ```
///
/// # Implementation details
///
/// Each [IpcReceiver] is backed by the OS specific implementations of `OsIpcReceiver`.
///
/// [IpcReceiver]: struct.IpcReceiver.html
#[derive(Debug)]
pub struct IpcReceiver<T> {
    os_receiver: OsIpcReceiver,
    phantom: PhantomData<T>,
}

impl<T> IpcReceiver<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    /// Blocking receive.
    pub fn recv(&self) -> Result<T, IpcError> {
        self.os_receiver.recv()?.to().map_err(IpcError::Bincode)
    }

    /// Non-blocking receive
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.os_receiver
            .try_recv()?
            .to()
            .map_err(IpcError::Bincode)
            .map_err(TryRecvError::IpcError)
    }

    /// Blocks for up to the specified duration attempting to receive a message.
    ///
    /// This may block for longer than the specified duration if the channel is busy. If your timeout
    /// exceeds the duration that your operating system can represent in milliseconds, this may
    /// block forever. At the time of writing, the smallest duration that may trigger this behavior
    /// is over 24 days.
    pub fn try_recv_timeout(&self, duration: Duration) -> Result<T, TryRecvError> {
        self.os_receiver
            .try_recv_timeout(duration)?
            .to()
            .map_err(IpcError::Bincode)
            .map_err(TryRecvError::IpcError)
    }

    /// Erase the type of the channel.
    ///
    /// Useful for adding routes to a `RouterProxy`.
    pub fn to_opaque(self) -> OpaqueIpcReceiver {
        OpaqueIpcReceiver {
            os_receiver: self.os_receiver,
        }
    }
}

impl<'de, T> Deserialize<'de> for IpcReceiver<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let os_receiver = deserialize_os_ipc_receiver(deserializer)?;
        Ok(IpcReceiver {
            os_receiver,
            phantom: PhantomData,
        })
    }
}

impl<T> Serialize for IpcReceiver<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_os_ipc_receiver(&self.os_receiver, serializer)
    }
}

/// Sending end of a channel using serialized messages.
///
///
/// ## Embedding Senders
///
/// ```
/// # use ipc_channel::ipc;
/// #
/// # let (tx, rx) = ipc::channel().unwrap();
/// # let (embedded_tx, embedded_rx) = ipc::channel().unwrap();
/// # let data = [0x45, 0x6d, 0x62, 0x65, 0x64, 0x64, 0x65, 0x64, 0x00];
/// // Send the IpcSender
/// tx.send(embedded_tx).unwrap();
/// // Receive the sent IpcSender
/// let received_tx = rx.recv().unwrap();
/// // Send data from the received IpcSender
/// received_tx.send(data.clone()).unwrap();
/// # let rx_data = embedded_rx.recv().unwrap();
/// # assert_eq!(rx_data, data);
/// ```
#[derive(Debug)]
pub struct IpcSender<T> {
    os_sender: OsIpcSender,
    phantom: PhantomData<T>,
}

impl<T> Clone for IpcSender<T>
where
    T: Serialize,
{
    fn clone(&self) -> IpcSender<T> {
        IpcSender {
            os_sender: self.os_sender.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T> IpcSender<T>
where
    T: Serialize,
{
    /// Create an [IpcSender] connected to a previously defined [IpcOneShotServer].
    ///
    /// [IpcSender]: struct.IpcSender.html
    /// [IpcOneShotServer]: struct.IpcOneShotServer.html
    pub fn connect(name: String) -> Result<IpcSender<T>, io::Error> {
        Ok(IpcSender {
            os_sender: OsIpcSender::connect(name)?,
            phantom: PhantomData,
        })
    }

    /// Send data across the channel to the receiver.
    pub fn send(&self, data: T) -> Result<(), bincode::Error> {
        let mut bytes = Vec::with_capacity(4096);
        OS_IPC_CHANNELS_FOR_SERIALIZATION.with(|os_ipc_channels_for_serialization| {
            OS_IPC_SHARED_MEMORY_REGIONS_FOR_SERIALIZATION.with(
                |os_ipc_shared_memory_regions_for_serialization| {
                    let old_os_ipc_channels =
                        mem::take(&mut *os_ipc_channels_for_serialization.borrow_mut());
                    let old_os_ipc_shared_memory_regions = mem::take(
                        &mut *os_ipc_shared_memory_regions_for_serialization.borrow_mut(),
                    );
                    let os_ipc_shared_memory_regions;
                    let os_ipc_channels;
                    {
                        bincode::serialize_into(&mut bytes, &data)?;
                        os_ipc_channels = mem::replace(
                            &mut *os_ipc_channels_for_serialization.borrow_mut(),
                            old_os_ipc_channels,
                        );
                        os_ipc_shared_memory_regions = mem::replace(
                            &mut *os_ipc_shared_memory_regions_for_serialization.borrow_mut(),
                            old_os_ipc_shared_memory_regions,
                        );
                    };
                    Ok(self.os_sender.send(
                        &bytes[..],
                        os_ipc_channels,
                        os_ipc_shared_memory_regions,
                    )?)
                },
            )
        })
    }

    pub fn to_opaque(self) -> OpaqueIpcSender {
        OpaqueIpcSender {
            os_sender: self.os_sender,
        }
    }
}

impl<'de, T> Deserialize<'de> for IpcSender<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let os_sender = deserialize_os_ipc_sender(deserializer)?;
        Ok(IpcSender {
            os_sender,
            phantom: PhantomData,
        })
    }
}

impl<T> Serialize for IpcSender<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_os_ipc_sender(&self.os_sender, serializer)
    }
}

/// Collection of [IpcReceiver]s moved into the set; thus creating a common
/// (and exclusive) endpoint for receiving messages on any of the added
/// channels.
///
/// # Examples
///
/// ```
/// # use ipc_channel::ipc::{self, IpcReceiverSet, IpcSelectionResult};
/// let data = vec![0x52, 0x75, 0x73, 0x74, 0x00];
/// let (tx, rx) = ipc::channel().unwrap();
/// let mut rx_set = IpcReceiverSet::new().unwrap();
///
/// // Add the receiver to the receiver set and send the data
/// // from the sender
/// let rx_id = rx_set.add(rx).unwrap();
/// tx.send(data.clone()).unwrap();
///
/// // Poll the receiver set for any readable events
/// for event in rx_set.select().unwrap() {
///     match event {
///         IpcSelectionResult::MessageReceived(id, message) => {
///             let rx_data: Vec<u8> = message.to().unwrap();
///             assert_eq!(id, rx_id);
///             assert_eq!(data, rx_data);
///             println!("Received: {:?} from {}...", data, id);
///         },
///         IpcSelectionResult::ChannelClosed(id) => {
///             assert_eq!(id, rx_id);
///             println!("No more data from {}...", id);
///         }
///     }
/// }
/// ```
/// [IpcReceiver]: struct.IpcReceiver.html
pub struct IpcReceiverSet {
    os_receiver_set: OsIpcReceiverSet,
}

impl IpcReceiverSet {
    /// Create a new empty [IpcReceiverSet].
    ///
    /// Receivers may then be added to the set with the [add]
    /// method.
    ///
    /// [add]: #method.add
    /// [IpcReceiverSet]: struct.IpcReceiverSet.html
    pub fn new() -> Result<IpcReceiverSet, io::Error> {
        Ok(IpcReceiverSet {
            os_receiver_set: OsIpcReceiverSet::new()?,
        })
    }

    /// Add and consume the [IpcReceiver] to the set of receivers to be polled.
    /// [IpcReceiver]: struct.IpcReceiver.html
    pub fn add<T>(&mut self, receiver: IpcReceiver<T>) -> Result<u64, io::Error>
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        Ok(self.os_receiver_set.add(receiver.os_receiver)?)
    }

    /// Add an [OpaqueIpcReceiver] to the set of receivers to be polled.
    /// [OpaqueIpcReceiver]: struct.OpaqueIpcReceiver.html
    pub fn add_opaque(&mut self, receiver: OpaqueIpcReceiver) -> Result<u64, io::Error> {
        Ok(self.os_receiver_set.add(receiver.os_receiver)?)
    }

    /// Wait for IPC messages received on any of the receivers in the set. The
    /// method will return multiple events. An event may be either a message
    /// received or a channel closed event.
    ///
    /// [IpcReceiver]: struct.IpcReceiver.html
    pub fn select(&mut self) -> Result<Vec<IpcSelectionResult>, io::Error> {
        let results = self.os_receiver_set.select()?;
        Ok(results
            .into_iter()
            .map(|result| match result {
                OsIpcSelectionResult::DataReceived(os_receiver_id, ipc_message) => {
                    IpcSelectionResult::MessageReceived(os_receiver_id, ipc_message)
                },
                OsIpcSelectionResult::ChannelClosed(os_receiver_id) => {
                    IpcSelectionResult::ChannelClosed(os_receiver_id)
                },
            })
            .collect())
    }
}

/// Shared memory descriptor that will be made accessible to the receiver
/// of an IPC message that contains the discriptor.
///
/// # Examples
/// ```
/// # use ipc_channel::ipc::{self, IpcSharedMemory};
/// # let (tx, rx) = ipc::channel().unwrap();
/// # let data = [0x76, 0x69, 0x6d, 0x00];
/// let shmem = IpcSharedMemory::from_bytes(&data);
/// tx.send(shmem.clone()).unwrap();
/// # let rx_shmem = rx.recv().unwrap();
/// # assert_eq!(shmem, rx_shmem);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct IpcSharedMemory {
    /// None represents no data (empty slice)
    os_shared_memory: Option<OsIpcSharedMemory>,
}

impl Deref for IpcSharedMemory {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        if let Some(os_shared_memory) = &self.os_shared_memory {
            os_shared_memory
        } else {
            &[]
        }
    }
}

impl<'de> Deserialize<'de> for IpcSharedMemory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let index: usize = Deserialize::deserialize(deserializer)?;
        if index == usize::MAX {
            return Ok(IpcSharedMemory::empty());
        }

        let os_shared_memory = OS_IPC_SHARED_MEMORY_REGIONS_FOR_DESERIALIZATION.with(
            |os_ipc_shared_memory_regions_for_deserialization| {
                // FIXME(pcwalton): This could panic if the data was corrupt and the index was out
                // of bounds. We should return an `Err` result instead.
                os_ipc_shared_memory_regions_for_deserialization.borrow_mut()[index]
                    .take()
                    .unwrap()
            },
        );
        Ok(IpcSharedMemory {
            os_shared_memory: Some(os_shared_memory),
        })
    }
}

impl Serialize for IpcSharedMemory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(os_shared_memory) = &self.os_shared_memory {
            let index = OS_IPC_SHARED_MEMORY_REGIONS_FOR_SERIALIZATION.with(
                |os_ipc_shared_memory_regions_for_serialization| {
                    let mut os_ipc_shared_memory_regions_for_serialization =
                        os_ipc_shared_memory_regions_for_serialization.borrow_mut();
                    let index = os_ipc_shared_memory_regions_for_serialization.len();
                    os_ipc_shared_memory_regions_for_serialization.push(os_shared_memory.clone());
                    index
                },
            );
            debug_assert!(index < usize::MAX);
            index
        } else {
            usize::MAX
        }
        .serialize(serializer)
    }
}

impl IpcSharedMemory {
    const fn empty() -> Self {
        Self {
            os_shared_memory: None,
        }
    }

    /// Create shared memory initialized with the bytes provided.
    pub fn from_bytes(bytes: &[u8]) -> IpcSharedMemory {
        if bytes.is_empty() {
            IpcSharedMemory::empty()
        } else {
            IpcSharedMemory {
                os_shared_memory: Some(OsIpcSharedMemory::from_bytes(bytes)),
            }
        }
    }

    /// Create a chunk of shared memory that is filled with the byte
    /// provided.
    pub fn from_byte(byte: u8, length: usize) -> IpcSharedMemory {
        if length == 0 {
            IpcSharedMemory::empty()
        } else {
            IpcSharedMemory {
                os_shared_memory: Some(OsIpcSharedMemory::from_byte(byte, length)),
            }
        }
    }
}

/// Result for readable events returned from [IpcReceiverSet::select].
///
/// [IpcReceiverSet::select]: struct.IpcReceiverSet.html#method.select
pub enum IpcSelectionResult {
    /// A message received from the [`IpcReceiver`] in the [`IpcMessage`] form,
    /// identified by the `u64` value.
    MessageReceived(u64, IpcMessage),
    /// The channel has been closed for the [IpcReceiver] identified by the `u64` value.
    /// [IpcReceiver]: struct.IpcReceiver.html
    ChannelClosed(u64),
}

impl IpcSelectionResult {
    /// Helper method to move the value out of the [IpcSelectionResult] if it
    /// is [MessageReceived].
    ///
    /// # Panics
    ///
    /// If the result is [ChannelClosed] this call will panic.
    ///
    /// [IpcSelectionResult]: enum.IpcSelectionResult.html
    /// [MessageReceived]: enum.IpcSelectionResult.html#variant.MessageReceived
    /// [ChannelClosed]: enum.IpcSelectionResult.html#variant.ChannelClosed
    pub fn unwrap(self) -> (u64, IpcMessage) {
        match self {
            IpcSelectionResult::MessageReceived(id, message) => (id, message),
            IpcSelectionResult::ChannelClosed(id) => {
                panic!("IpcSelectionResult::unwrap(): channel {} closed", id)
            },
        }
    }
}

/// Structure used to represent a raw message from an [`IpcSender`].
///
/// Use the [to] method to deserialize the raw result into the requested type.
///
/// [to]: #method.to
#[derive(PartialEq)]
pub struct IpcMessage {
    pub(crate) data: Vec<u8>,
    pub(crate) os_ipc_channels: Vec<OsOpaqueIpcChannel>,
    pub(crate) os_ipc_shared_memory_regions: Vec<OsIpcSharedMemory>,
}

impl IpcMessage {
    /// Create a new [`IpcMessage`] with data and without any [`OsOpaqueIpcChannel`]s and
    /// [`OsIpcSharedMemory`] regions.
    pub fn from_data(data: Vec<u8>) -> Self {
        Self {
            data,
            os_ipc_channels: vec![],
            os_ipc_shared_memory_regions: vec![],
        }
    }
}

impl Debug for IpcMessage {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match String::from_utf8(self.data.clone()) {
            Ok(string) => string.chars().take(256).collect::<String>().fmt(formatter),
            Err(..) => self.data[0..min(self.data.len(), 256)].fmt(formatter),
        }
    }
}

impl IpcMessage {
    pub(crate) fn new(
        data: Vec<u8>,
        os_ipc_channels: Vec<OsOpaqueIpcChannel>,
        os_ipc_shared_memory_regions: Vec<OsIpcSharedMemory>,
    ) -> IpcMessage {
        IpcMessage {
            data,
            os_ipc_channels,
            os_ipc_shared_memory_regions,
        }
    }

    /// Deserialize the raw data in the contained message into the inferred type.
    pub fn to<T>(mut self) -> Result<T, bincode::Error>
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        OS_IPC_CHANNELS_FOR_DESERIALIZATION.with(|os_ipc_channels_for_deserialization| {
            OS_IPC_SHARED_MEMORY_REGIONS_FOR_DESERIALIZATION.with(
                |os_ipc_shared_memory_regions_for_deserialization| {
                    mem::swap(
                        &mut *os_ipc_channels_for_deserialization.borrow_mut(),
                        &mut self.os_ipc_channels,
                    );
                    let old_ipc_shared_memory_regions_for_deserialization = mem::replace(
                        &mut *os_ipc_shared_memory_regions_for_deserialization.borrow_mut(),
                        self.os_ipc_shared_memory_regions
                            .into_iter()
                            .map(Some)
                            .collect(),
                    );
                    let result = bincode::deserialize(&self.data[..]);
                    *os_ipc_shared_memory_regions_for_deserialization.borrow_mut() =
                        old_ipc_shared_memory_regions_for_deserialization;
                    mem::swap(
                        &mut *os_ipc_channels_for_deserialization.borrow_mut(),
                        &mut self.os_ipc_channels,
                    );
                    /* Error check comes after doing cleanup,
                     * since we need the cleanup both in the success and the error cases. */
                    result
                },
            )
        })
    }
}

#[derive(Clone, Debug)]
pub struct OpaqueIpcSender {
    os_sender: OsIpcSender,
}

impl OpaqueIpcSender {
    pub fn to<'de, T>(self) -> IpcSender<T>
    where
        T: Deserialize<'de> + Serialize,
    {
        IpcSender {
            os_sender: self.os_sender,
            phantom: PhantomData,
        }
    }
}

impl<'de> Deserialize<'de> for OpaqueIpcSender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let os_sender = deserialize_os_ipc_sender(deserializer)?;
        Ok(OpaqueIpcSender { os_sender })
    }
}

impl Serialize for OpaqueIpcSender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_os_ipc_sender(&self.os_sender, serializer)
    }
}

#[derive(Debug)]
pub struct OpaqueIpcReceiver {
    os_receiver: OsIpcReceiver,
}

impl OpaqueIpcReceiver {
    pub fn to<'de, T>(self) -> IpcReceiver<T>
    where
        T: Deserialize<'de> + Serialize,
    {
        IpcReceiver {
            os_receiver: self.os_receiver,
            phantom: PhantomData,
        }
    }
}

impl<'de> Deserialize<'de> for OpaqueIpcReceiver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let os_receiver = deserialize_os_ipc_receiver(deserializer)?;
        Ok(OpaqueIpcReceiver { os_receiver })
    }
}

impl Serialize for OpaqueIpcReceiver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_os_ipc_receiver(&self.os_receiver, serializer)
    }
}

/// A server associated with a given name.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```
/// use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
///
/// let (server, server_name) = IpcOneShotServer::new().unwrap();
/// let tx: IpcSender<Vec<u8>> = IpcSender::connect(server_name).unwrap();
///
/// tx.send(vec![0x10, 0x11, 0x12, 0x13]).unwrap();
/// let (_, data): (_, Vec<u8>) = server.accept().unwrap();
/// assert_eq!(data, vec![0x10, 0x11, 0x12, 0x13]);
/// ```
///
/// ## Sending an [IpcSender]
/// ```
/// use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
/// let (server, name) = IpcOneShotServer::new().unwrap();
///
/// let (tx1, rx1): (IpcSender<Vec<u8>>, IpcReceiver<Vec<u8>>) = ipc::channel().unwrap();
/// let tx0 = IpcSender::connect(name).unwrap();
/// tx0.send(tx1).unwrap();
///
/// let (_, tx1): (_, IpcSender<Vec<u8>>) = server.accept().unwrap();
/// tx1.send(vec![0x48, 0x65, 0x6b, 0x6b, 0x6f, 0x00]).unwrap();
///
/// let data = rx1.recv().unwrap();
/// assert_eq!(data, vec![0x48, 0x65, 0x6b, 0x6b, 0x6f, 0x00]);
/// ```
/// [IpcSender]: struct.IpcSender.html
pub struct IpcOneShotServer<T> {
    os_server: OsIpcOneShotServer,
    phantom: PhantomData<T>,
}

impl<T> IpcOneShotServer<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    pub fn new() -> Result<(IpcOneShotServer<T>, String), io::Error> {
        let (os_server, name) = OsIpcOneShotServer::new()?;
        Ok((
            IpcOneShotServer {
                os_server,
                phantom: PhantomData,
            },
            name,
        ))
    }

    pub fn accept(self) -> Result<(IpcReceiver<T>, T), bincode::Error> {
        let (os_receiver, ipc_message) = self.os_server.accept()?;
        Ok((
            IpcReceiver {
                os_receiver,
                phantom: PhantomData,
            },
            ipc_message.to()?,
        ))
    }
}

/// Receiving end of a channel that does not used serialized messages.
#[derive(Debug)]
pub struct IpcBytesReceiver {
    os_receiver: OsIpcReceiver,
}

impl IpcBytesReceiver {
    /// Blocking receive.
    #[inline]
    pub fn recv(&self) -> Result<Vec<u8>, IpcError> {
        match self.os_receiver.recv() {
            Ok(ipc_message) => Ok(ipc_message.data),
            Err(err) => Err(err.into()),
        }
    }

    /// Non-blocking receive
    pub fn try_recv(&self) -> Result<Vec<u8>, TryRecvError> {
        match self.os_receiver.try_recv() {
            Ok(ipc_message) => Ok(ipc_message.data),
            Err(err) => Err(err.into()),
        }
    }
}

impl<'de> Deserialize<'de> for IpcBytesReceiver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let os_receiver = deserialize_os_ipc_receiver(deserializer)?;
        Ok(IpcBytesReceiver { os_receiver })
    }
}

impl Serialize for IpcBytesReceiver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_os_ipc_receiver(&self.os_receiver, serializer)
    }
}

/// Sending end of a channel that does not used serialized messages.
#[derive(Debug)]
pub struct IpcBytesSender {
    os_sender: OsIpcSender,
}

impl Clone for IpcBytesSender {
    fn clone(&self) -> IpcBytesSender {
        IpcBytesSender {
            os_sender: self.os_sender.clone(),
        }
    }
}

impl<'de> Deserialize<'de> for IpcBytesSender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let os_sender = deserialize_os_ipc_sender(deserializer)?;
        Ok(IpcBytesSender { os_sender })
    }
}

impl Serialize for IpcBytesSender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_os_ipc_sender(&self.os_sender, serializer)
    }
}

impl IpcBytesSender {
    #[inline]
    pub fn send(&self, data: &[u8]) -> Result<(), io::Error> {
        self.os_sender
            .send(data, vec![], vec![])
            .map_err(io::Error::from)
    }
}

fn serialize_os_ipc_sender<S>(os_ipc_sender: &OsIpcSender, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let index = OS_IPC_CHANNELS_FOR_SERIALIZATION.with(|os_ipc_channels_for_serialization| {
        let mut os_ipc_channels_for_serialization = os_ipc_channels_for_serialization.borrow_mut();
        let index = os_ipc_channels_for_serialization.len();
        os_ipc_channels_for_serialization.push(OsIpcChannel::Sender(os_ipc_sender.clone()));
        index
    });
    index.serialize(serializer)
}

fn deserialize_os_ipc_sender<'de, D>(deserializer: D) -> Result<OsIpcSender, D::Error>
where
    D: Deserializer<'de>,
{
    let index: usize = Deserialize::deserialize(deserializer)?;
    OS_IPC_CHANNELS_FOR_DESERIALIZATION.with(|os_ipc_channels_for_deserialization| {
        // FIXME(pcwalton): This could panic if the data was corrupt and the index was out of
        // bounds. We should return an `Err` result instead.
        Ok(os_ipc_channels_for_deserialization.borrow_mut()[index].to_sender())
    })
}

fn serialize_os_ipc_receiver<S>(
    os_receiver: &OsIpcReceiver,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let index = OS_IPC_CHANNELS_FOR_SERIALIZATION.with(|os_ipc_channels_for_serialization| {
        let mut os_ipc_channels_for_serialization = os_ipc_channels_for_serialization.borrow_mut();
        let index = os_ipc_channels_for_serialization.len();
        os_ipc_channels_for_serialization.push(OsIpcChannel::Receiver(os_receiver.consume()));
        index
    });
    index.serialize(serializer)
}

fn deserialize_os_ipc_receiver<'de, D>(deserializer: D) -> Result<OsIpcReceiver, D::Error>
where
    D: Deserializer<'de>,
{
    let index: usize = Deserialize::deserialize(deserializer)?;

    OS_IPC_CHANNELS_FOR_DESERIALIZATION.with(|os_ipc_channels_for_deserialization| {
        // FIXME(pcwalton): This could panic if the data was corrupt and the index was out
        // of bounds. We should return an `Err` result instead.
        Ok(os_ipc_channels_for_deserialization.borrow_mut()[index].to_receiver())
    })
}
