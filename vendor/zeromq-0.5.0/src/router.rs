use crate::backend::GenericSocketBackend;
use crate::codec::*;
use crate::endpoint::Endpoint;
use crate::error::{ZmqError, ZmqResult};
use crate::fair_queue::FairQueue;
use crate::message::*;
use crate::transport::AcceptStopHandle;
use crate::util::PeerIdentity;
use crate::{MultiPeerBackend, SocketEvent, SocketOptions, SocketRecv, SocketSend, SocketType};
use crate::{Socket, SocketBackend};

use async_trait::async_trait;
use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};

use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::Arc;

pub struct RouterSocket {
    backend: Arc<GenericSocketBackend>,
    binds: HashMap<Endpoint, AcceptStopHandle>,
    fair_queue: FairQueue<ZmqFramedRead, PeerIdentity>,
}

impl Drop for RouterSocket {
    fn drop(&mut self) {
        self.backend.shutdown();
    }
}

#[async_trait]
impl Socket for RouterSocket {
    fn with_options(options: SocketOptions) -> Self {
        let fair_queue = FairQueue::new(true);
        Self {
            backend: Arc::new(GenericSocketBackend::with_options(
                Some(fair_queue.inner()),
                SocketType::ROUTER,
                options,
            )),
            binds: HashMap::new(),
            fair_queue,
        }
    }

    fn backend(&self) -> Arc<dyn MultiPeerBackend> {
        self.backend.clone()
    }

    fn binds(&mut self) -> &mut HashMap<Endpoint, AcceptStopHandle> {
        &mut self.binds
    }

    fn monitor(&mut self) -> mpsc::Receiver<SocketEvent> {
        let (sender, receiver) = mpsc::channel(1024);
        self.backend.socket_monitor.lock().replace(sender);
        receiver
    }
}

#[async_trait]
impl SocketRecv for RouterSocket {
    async fn recv(&mut self) -> ZmqResult<ZmqMessage> {
        loop {
            match self.fair_queue.next().await {
                Some((peer_id, Ok(Message::Message(mut message)))) => {
                    message.push_front(peer_id.into());
                    return Ok(message);
                }
                Some((_peer_id, Ok(_msg))) => {
                    // todo: Log or handle other message types if needed
                    // We could take an approach of using `tracing` and have that be an optional feature
                    // tracing::warn!("Received unimplemented message type: {:?}", msg);
                }
                Some((peer_id, Err(_e))) => {
                    self.backend.peer_disconnected(&peer_id);
                    // We could take an approach of using `tracing` and have that be an optional feature
                    // tracing::error!("Error receiving message from peer {}: {:?}", peer_id, e);
                }
                None => {
                    // The fair queue is empty, which shouldn't happen in normal operation
                    return Err(ZmqError::NoMessage);
                }
            };
        }
    }
}

#[async_trait]
impl SocketSend for RouterSocket {
    async fn send(&mut self, mut message: ZmqMessage) -> ZmqResult<()> {
        assert!(message.len() > 1);
        let peer_id: PeerIdentity = message.pop_front().unwrap().try_into()?;
        match self.backend.peers.get_async(&peer_id).await {
            Some(mut peer) => {
                peer.send_queue.send(Message::Message(message)).await?;
                Ok(())
            }
            None => Err(ZmqError::Other("Destination client not found by identity")),
        }
    }
}

impl RouterSocket {
    /// Splits the socket into separate send and recv halves, allowing concurrent
    /// sending and receiving from independent async tasks.
    ///
    /// The underlying socket stays alive until both halves are dropped.
    pub fn split(mut self) -> (RouterSendHalf, RouterRecvHalf) {
        let backend = std::mem::replace(
            &mut self.backend,
            Arc::new(GenericSocketBackend::with_options(
                None,
                SocketType::ROUTER,
                SocketOptions::default(),
            )),
        );
        let fair_queue = std::mem::replace(&mut self.fair_queue, FairQueue::new(true));
        let binds = std::mem::take(&mut self.binds);

        let inner = Arc::new(RouterSocketInner {
            backend,
            _binds: binds,
        });

        (
            RouterSendHalf {
                inner: inner.clone(),
            },
            RouterRecvHalf { inner, fair_queue },
        )
    }
}

struct RouterSocketInner {
    backend: Arc<GenericSocketBackend>,
    _binds: HashMap<Endpoint, AcceptStopHandle>,
}

impl Drop for RouterSocketInner {
    fn drop(&mut self) {
        self.backend.shutdown();
    }
}

/// The send half of a [`RouterSocket`] produced by [`RouterSocket::split`].
pub struct RouterSendHalf {
    inner: Arc<RouterSocketInner>,
}

/// The recv half of a [`RouterSocket`] produced by [`RouterSocket::split`].
pub struct RouterRecvHalf {
    inner: Arc<RouterSocketInner>,
    fair_queue: FairQueue<ZmqFramedRead, PeerIdentity>,
}

#[async_trait]
impl SocketSend for RouterSendHalf {
    async fn send(&mut self, mut message: ZmqMessage) -> ZmqResult<()> {
        assert!(message.len() > 1);
        let peer_id: PeerIdentity = message.pop_front().unwrap().try_into()?;
        match self.inner.backend.peers.get_async(&peer_id).await {
            Some(mut peer) => {
                peer.send_queue.send(Message::Message(message)).await?;
                Ok(())
            }
            None => Err(ZmqError::Other("Destination client not found by identity")),
        }
    }
}

#[async_trait]
impl SocketRecv for RouterRecvHalf {
    async fn recv(&mut self) -> ZmqResult<ZmqMessage> {
        loop {
            match self.fair_queue.next().await {
                Some((peer_id, Ok(Message::Message(mut message)))) => {
                    message.push_front(peer_id.into());
                    return Ok(message);
                }
                Some((_peer_id, Ok(_))) => {}
                Some((peer_id, Err(_e))) => {
                    self.inner.backend.peer_disconnected(&peer_id);
                }
                None => {
                    return Err(ZmqError::NoMessage);
                }
            };
        }
    }
}
