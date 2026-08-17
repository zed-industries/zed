use crate::backend::GenericSocketBackend;
use crate::codec::{Message, ZmqFramedRead};
use crate::fair_queue::FairQueue;
use crate::transport::AcceptStopHandle;
use crate::util::PeerIdentity;
use crate::{
    CaptureSocket, Endpoint, MultiPeerBackend, Socket, SocketBackend, SocketEvent, SocketOptions,
    SocketRecv, SocketSend, SocketType, ZmqError, ZmqMessage, ZmqResult,
};

use async_trait::async_trait;
use futures::channel::mpsc;
use futures::StreamExt;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::Arc;

pub struct DealerSocket {
    backend: Arc<GenericSocketBackend>,
    fair_queue: FairQueue<ZmqFramedRead, PeerIdentity>,
    binds: HashMap<Endpoint, AcceptStopHandle>,
}

impl Drop for DealerSocket {
    fn drop(&mut self) {
        self.backend.shutdown();
    }
}

#[async_trait]
impl Socket for DealerSocket {
    fn with_options(options: SocketOptions) -> Self {
        let fair_queue = FairQueue::new(true);
        Self {
            backend: Arc::new(GenericSocketBackend::with_options(
                Some(fair_queue.inner()),
                SocketType::DEALER,
                options,
            )),
            fair_queue,
            binds: HashMap::new(),
        }
    }

    fn backend(&self) -> Arc<dyn MultiPeerBackend> {
        self.backend.clone()
    }

    fn binds(&mut self) -> &mut HashMap<Endpoint, AcceptStopHandle, RandomState> {
        &mut self.binds
    }

    fn monitor(&mut self) -> mpsc::Receiver<SocketEvent> {
        let (sender, receiver) = mpsc::channel(1024);
        self.backend.socket_monitor.lock().replace(sender);
        receiver
    }
}

#[async_trait]
impl SocketRecv for DealerSocket {
    async fn recv(&mut self) -> ZmqResult<ZmqMessage> {
        loop {
            match self.fair_queue.next().await {
                Some((_peer_id, Ok(Message::Message(message)))) => {
                    return Ok(message);
                }
                Some((_peer_id, Ok(_))) => {
                    // Ignore non-message frames
                }
                Some((_peer_id, Err(e))) => {
                    // Handle potential errors from the fair queue
                    return Err(e.into());
                }
                None => {
                    // The fair queue is empty, which shouldn't happen in normal operation
                    // We could either wait for more messages or return an error
                    return Err(ZmqError::NoMessage);
                }
            };
        }
    }
}

#[async_trait]
impl SocketSend for DealerSocket {
    async fn send(&mut self, message: ZmqMessage) -> ZmqResult<()> {
        self.backend
            .send_round_robin(Message::Message(message))
            .await?;
        Ok(())
    }
}

impl CaptureSocket for DealerSocket {}

impl DealerSocket {
    /// Splits the socket into separate send and recv halves, allowing concurrent
    /// sending and receiving from independent async tasks.
    ///
    /// The underlying socket stays alive until both halves are dropped.
    pub fn split(mut self) -> (DealerSendHalf, DealerRecvHalf) {
        // Swap the real fields out before self drops. The dummy backend's
        // shutdown() is a no-op on an empty peer map.
        let backend = std::mem::replace(
            &mut self.backend,
            Arc::new(GenericSocketBackend::with_options(
                None,
                SocketType::DEALER,
                SocketOptions::default(),
            )),
        );
        let fair_queue = std::mem::replace(&mut self.fair_queue, FairQueue::new(true));
        let binds = std::mem::take(&mut self.binds);

        let inner = Arc::new(DealerSocketInner {
            backend,
            _binds: binds,
        });

        (
            DealerSendHalf {
                inner: inner.clone(),
            },
            DealerRecvHalf {
                _inner: inner,
                fair_queue,
            },
        )
    }
}

struct DealerSocketInner {
    backend: Arc<GenericSocketBackend>,
    _binds: HashMap<Endpoint, AcceptStopHandle>,
}

impl Drop for DealerSocketInner {
    fn drop(&mut self) {
        self.backend.shutdown();
    }
}

/// The send half of a [`DealerSocket`] produced by [`DealerSocket::split`].
pub struct DealerSendHalf {
    inner: Arc<DealerSocketInner>,
}

/// The recv half of a [`DealerSocket`] produced by [`DealerSocket::split`].
pub struct DealerRecvHalf {
    _inner: Arc<DealerSocketInner>,
    fair_queue: FairQueue<ZmqFramedRead, PeerIdentity>,
}

#[async_trait]
impl SocketSend for DealerSendHalf {
    async fn send(&mut self, message: ZmqMessage) -> ZmqResult<()> {
        self.inner
            .backend
            .send_round_robin(Message::Message(message))
            .await?;
        Ok(())
    }
}

impl CaptureSocket for DealerSendHalf {}

#[async_trait]
impl SocketRecv for DealerRecvHalf {
    async fn recv(&mut self) -> ZmqResult<ZmqMessage> {
        loop {
            match self.fair_queue.next().await {
                Some((_peer_id, Ok(Message::Message(message)))) => {
                    return Ok(message);
                }
                Some((_peer_id, Ok(_))) => {}
                Some((_peer_id, Err(e))) => {
                    return Err(e.into());
                }
                None => {
                    return Err(ZmqError::NoMessage);
                }
            };
        }
    }
}
