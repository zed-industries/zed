#![recursion_limit = "1024"]

mod async_rt;
mod backend;
mod codec;
mod dealer;
mod endpoint;
mod error;
mod fair_queue;
mod message;
mod r#pub;
mod pull;
mod push;
mod rep;
mod req;
mod router;
mod sub;
mod task_handle;
mod transport;
pub mod util;
mod xpub;

#[doc(hidden)]
pub mod __async_rt {
    //! DO NOT USE! PRIVATE IMPLEMENTATION, EXPOSED ONLY FOR INTEGRATION TESTS.
    pub use super::async_rt::*;
}

pub use crate::dealer::*;
pub use crate::endpoint::{Endpoint, Host, Transport, TryIntoEndpoint};
pub use crate::error::{ZmqError, ZmqResult};
pub use crate::message::*;
pub use crate::pull::*;
pub use crate::push::*;
pub use crate::r#pub::*;
pub use crate::rep::*;
pub use crate::req::*;
pub use crate::router::*;
pub use crate::sub::*;
pub use crate::xpub::*;

use crate::codec::*;
use crate::transport::AcceptStopHandle;
use util::PeerIdentity;

use async_trait::async_trait;
use asynchronous_codec::FramedWrite;
use futures::channel::mpsc;
use futures::{select, FutureExt};
use parking_lot::Mutex;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::sync::Arc;

const COMPATIBILITY_MATRIX: [u8; 121] = [
    // PAIR, PUB, SUB, REQ, REP, DEALER, ROUTER, PULL, PUSH, XPUB, XSUB
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // PAIR
    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, // PUB
    0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, // SUB
    0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, // REQ
    0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, // REP
    0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, // DEALER
    0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, // ROUTER
    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, // PULL
    0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, // PUSH
    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, // XPUB
    0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, // XSUB
];

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(usize)]
pub enum SocketType {
    PAIR = 0,
    PUB = 1,
    SUB = 2,
    REQ = 3,
    REP = 4,
    DEALER = 5,
    ROUTER = 6,
    PULL = 7,
    PUSH = 8,
    XPUB = 9,
    XSUB = 10,
    STREAM = 11,
}

impl SocketType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            SocketType::PAIR => "PAIR",
            SocketType::PUB => "PUB",
            SocketType::SUB => "SUB",
            SocketType::REQ => "REQ",
            SocketType::REP => "REP",
            SocketType::DEALER => "DEALER",
            SocketType::ROUTER => "ROUTER",
            SocketType::PULL => "PULL",
            SocketType::PUSH => "PUSH",
            SocketType::XPUB => "XPUB",
            SocketType::XSUB => "XSUB",
            SocketType::STREAM => "STREAM",
        }
    }

    /// Checks if two sockets are compatible with each other
    /// ```
    /// use zeromq::SocketType;
    /// assert!(SocketType::PUB.compatible(SocketType::SUB));
    /// assert!(SocketType::REQ.compatible(SocketType::REP));
    /// assert!(SocketType::DEALER.compatible(SocketType::ROUTER));
    /// assert!(!SocketType::PUB.compatible(SocketType::REP));
    /// ```
    pub fn compatible(&self, other: SocketType) -> bool {
        let row_index = *self as usize;
        let col_index = other as usize;
        COMPATIBILITY_MATRIX[row_index * 11 + col_index] != 0
    }
}

impl FromStr for SocketType {
    type Err = ZmqError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, ZmqError> {
        Self::try_from(s.as_bytes())
    }
}

impl TryFrom<&[u8]> for SocketType {
    type Error = ZmqError;

    fn try_from(s: &[u8]) -> Result<Self, ZmqError> {
        Ok(match s {
            b"PAIR" => SocketType::PAIR,
            b"PUB" => SocketType::PUB,
            b"SUB" => SocketType::SUB,
            b"REQ" => SocketType::REQ,
            b"REP" => SocketType::REP,
            b"DEALER" => SocketType::DEALER,
            b"ROUTER" => SocketType::ROUTER,
            b"PULL" => SocketType::PULL,
            b"PUSH" => SocketType::PUSH,
            b"XPUB" => SocketType::XPUB,
            b"XSUB" => SocketType::XSUB,
            b"STREAM" => SocketType::STREAM,
            _ => return Err(ZmqError::Other("Unknown socket type")),
        })
    }
}

impl Display for SocketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug)]
pub enum SocketEvent {
    Connected(Endpoint, PeerIdentity),
    ConnectDelayed,
    ConnectRetried,
    Listening(Endpoint),
    Accepted(Endpoint, PeerIdentity),
    AcceptFailed(ZmqError),
    Closed,
    CloseFailed,
    Disconnected(PeerIdentity),
}

#[derive(Default)]
pub struct SocketOptions {
    pub(crate) peer_id: Option<PeerIdentity>,
}

impl SocketOptions {
    pub fn peer_identity(&mut self, peer_id: PeerIdentity) -> &mut Self {
        self.peer_id = Some(peer_id);
        self
    }
}

#[async_trait]
pub trait MultiPeerBackend: SocketBackend {
    /// This should not be public..
    /// Find a better way of doing this
    async fn peer_connected(self: Arc<Self>, peer_id: &PeerIdentity, io: FramedIo);

    fn peer_disconnected(&self, peer_id: &PeerIdentity);
}

pub trait SocketBackend: Send + Sync {
    fn socket_type(&self) -> SocketType;
    fn socket_options(&self) -> &SocketOptions;
    fn shutdown(&self);
    fn monitor(&self) -> &Mutex<Option<mpsc::Sender<SocketEvent>>>;
}

#[async_trait]
pub trait SocketRecv {
    async fn recv(&mut self) -> ZmqResult<ZmqMessage>;
}

#[async_trait]
pub trait SocketSend {
    async fn send(&mut self, message: ZmqMessage) -> ZmqResult<()>;
}

/// Marker trait that express the fact that only certain types of sockets might be used
/// in [proxy] function as a capture parameter
pub trait CaptureSocket: SocketSend {}

#[allow(clippy::empty_line_after_outer_attr)]
#[async_trait]
pub trait Socket: Sized + Send {
    fn new() -> Self {
        Self::with_options(SocketOptions::default())
    }

    fn with_options(options: SocketOptions) -> Self;

    fn backend(&self) -> Arc<dyn MultiPeerBackend>;

    /// Binds to the endpoint and starts a coroutine to accept new connections
    /// on it.
    ///
    /// Returns the endpoint resolved to the exact bound location if applicable
    /// (port # resolved, for example).
    async fn bind(&mut self, endpoint: &str) -> ZmqResult<Endpoint> {
        let endpoint = TryIntoEndpoint::try_into(endpoint)?;

        let cloned_backend = self.backend();
        let cback = move |result: ZmqResult<(FramedIo, Endpoint)>| {
            let cloned_backend = cloned_backend.clone();
            async move {
                let result = match result {
                    Ok((socket, endpoint)) => util::peer_connected(socket, cloned_backend.clone())
                        .await
                        .map(|peer_id| (endpoint, peer_id)),
                    Err(e) => Err(e),
                };

                match result {
                    Ok((endpoint, peer_id)) => {
                        if let Some(monitor) = cloned_backend.monitor().lock().as_mut() {
                            let _ = monitor.try_send(SocketEvent::Accepted(endpoint, peer_id));
                        }
                    }
                    Err(e) => {
                        if let Some(monitor) = cloned_backend.monitor().lock().as_mut() {
                            let _ = monitor.try_send(SocketEvent::AcceptFailed(e));
                        }
                    }
                }
            }
        };

        let (endpoint, stop_handle) = transport::begin_accept(endpoint, cback).await?;

        if let Some(monitor) = self.backend().monitor().lock().as_mut() {
            let _ = monitor.try_send(SocketEvent::Listening(endpoint.clone()));
        }

        self.binds().insert(endpoint.clone(), stop_handle);
        Ok(endpoint)
    }

    fn binds(&mut self) -> &mut HashMap<Endpoint, AcceptStopHandle>;

    /// Unbinds the endpoint, blocking until the associated endpoint is no
    /// longer in use
    ///
    /// # Errors
    /// May give a `ZmqError::NoSuchBind` if `endpoint` isn't bound. May also
    /// give any other zmq errors encountered when attempting to disconnect
    async fn unbind(&mut self, endpoint: Endpoint) -> ZmqResult<()> {
        let stop_handle = self.binds().remove(&endpoint);
        let stop_handle = stop_handle.ok_or(ZmqError::NoSuchBind(endpoint))?;
        stop_handle.0.shutdown().await
    }

    /// Unbinds all bound endpoints, blocking until finished.
    async fn unbind_all(&mut self) -> Vec<ZmqError> {
        let mut errs = Vec::new();
        let endpoints: Vec<_> = self.binds().keys().cloned().collect();
        for endpoint in endpoints {
            if let Err(err) = self.unbind(endpoint).await {
                errs.push(err);
            }
        }
        errs
    }

    /// Connects to the given endpoint.
    async fn connect(&mut self, endpoint: &str) -> ZmqResult<()> {
        let backend = self.backend();
        let endpoint = TryIntoEndpoint::try_into(endpoint)?;

        let (socket, endpoint) = util::connect_forever(endpoint).await?;
        let peer_id = util::peer_connected(socket, backend).await?;

        if let Some(monitor) = self.backend().monitor().lock().as_mut() {
            let _ = monitor.try_send(SocketEvent::Connected(endpoint, peer_id));
        }
        Ok(())
    }

    /// Creates and setups new socket monitor
    ///
    /// Subsequent calls to this method each create a new monitor channel.
    /// Sender side of previous one is dropped.
    fn monitor(&mut self) -> mpsc::Receiver<SocketEvent>;

    // TODO: async fn connections(&self) -> ?

    /// Disconnects from the given endpoint, blocking until finished.
    ///
    /// # Errors
    /// May give a `ZmqError::NoSuchConnection` if `endpoint` isn't connected.
    /// May also give any other zmq errors encountered when attempting to
    /// disconnect
    // TODO: async fn disconnect(&mut self, endpoint: impl TryIntoEndpoint + 'async_trait) ->
    // ZmqResult<()>;

    /// Disconnects all connections, blocking until finished.
    // TODO: async fn disconnect_all(&mut self) -> ZmqResult<()>;

    /// Closes the socket, blocking until all associated binds are closed.
    /// This is equivalent to `drop()`, but with the benefit of blocking until
    /// resources are released, and getting any underlying errors.
    ///
    /// Returns any encountered errors.
    // TODO: Call disconnect_all() when added
    async fn close(mut self) -> Vec<ZmqError> {
        // self.disconnect_all().await?;
        self.unbind_all().await
    }
}

pub async fn proxy<Frontend: SocketSend + SocketRecv, Backend: SocketSend + SocketRecv>(
    mut frontend: Frontend,
    mut backend: Backend,
    mut capture: Option<Box<dyn CaptureSocket>>,
) -> ZmqResult<()> {
    loop {
        select! {
            frontend_mess = frontend.recv().fuse() => {
                match frontend_mess {
                    Ok(message) => {
                        if let Some(capture) = &mut capture {
                            capture.send(message.clone()).await?;
                        }
                        backend.send(message).await?;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            backend_mess = backend.recv().fuse() => {
                match backend_mess {
                    Ok(message) => {
                        if let Some(capture) = &mut capture {
                            capture.send(message.clone()).await?;
                        }
                        frontend.send(message).await?;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        };
    }
}

pub mod prelude {
    //! Re-exports important traits. Consider glob-importing.

    pub use crate::{Socket, SocketRecv, SocketSend, TryIntoEndpoint};
}
