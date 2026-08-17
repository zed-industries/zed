use crate::codec::{FramedIo, Message, ZmqFramedRead, ZmqFramedWrite};
use crate::fair_queue::QueueInner;
use crate::util::PeerIdentity;
use crate::{
    MultiPeerBackend, SocketBackend, SocketEvent, SocketOptions, SocketType, ZmqError, ZmqResult,
};

use async_trait::async_trait;
use crossbeam_queue::SegQueue;
use futures::channel::mpsc;
use futures::SinkExt;
use parking_lot::Mutex;

use std::sync::Arc;

pub(crate) struct Peer {
    pub(crate) send_queue: ZmqFramedWrite,
}

pub(crate) struct GenericSocketBackend {
    pub(crate) peers: scc::HashMap<PeerIdentity, Peer>,
    fair_queue_inner: Option<Arc<Mutex<QueueInner<ZmqFramedRead, PeerIdentity>>>>,
    pub(crate) round_robin: SegQueue<PeerIdentity>,
    socket_type: SocketType,
    socket_options: SocketOptions,
    pub(crate) socket_monitor: Mutex<Option<mpsc::Sender<SocketEvent>>>,
}

impl GenericSocketBackend {
    pub(crate) fn with_options(
        fair_queue_inner: Option<Arc<Mutex<QueueInner<ZmqFramedRead, PeerIdentity>>>>,
        socket_type: SocketType,
        options: SocketOptions,
    ) -> Self {
        Self {
            peers: scc::HashMap::new(),
            fair_queue_inner,
            round_robin: SegQueue::new(),
            socket_type,
            socket_options: options,
            socket_monitor: Mutex::new(None),
        }
    }

    pub(crate) async fn send_round_robin(&self, message: Message) -> ZmqResult<PeerIdentity> {
        // In normal scenario this will always be only 1 iteration
        // There can be special case when peer has disconnected and his id is still in
        // RR queue This happens because SegQueue don't have an api to delete
        // items from queue. So in such case we'll just pop item and skip it if
        // we don't have a matching peer in peers map
        loop {
            let next_peer_id = match self.round_robin.pop() {
                Some(peer) => peer,
                None => match message {
                    Message::Greeting(_) => panic!("Sending greeting is not supported"),
                    Message::Command(_) => panic!("Sending commands is not supported"),
                    Message::Message(m) => {
                        return Err(ZmqError::ReturnToSender {
                            reason: "Not connected to peers. Unable to send messages",
                            message: m,
                        })
                    }
                },
            };
            let send_result = match self.peers.get_async(&next_peer_id).await {
                Some(mut peer) => peer.send_queue.send(message).await,
                None => continue,
            };
            return match send_result {
                Ok(()) => {
                    self.round_robin.push(next_peer_id.clone());
                    Ok(next_peer_id)
                }
                Err(e) => {
                    self.peer_disconnected(&next_peer_id);
                    Err(e.into())
                }
            };
        }
    }
}

impl SocketBackend for GenericSocketBackend {
    fn socket_type(&self) -> SocketType {
        self.socket_type
    }

    fn socket_options(&self) -> &SocketOptions {
        &self.socket_options
    }

    fn shutdown(&self) {
        self.peers.clear_sync();
    }

    fn monitor(&self) -> &Mutex<Option<mpsc::Sender<SocketEvent>>> {
        &self.socket_monitor
    }
}

#[async_trait]
impl MultiPeerBackend for GenericSocketBackend {
    async fn peer_connected(self: Arc<Self>, peer_id: &PeerIdentity, io: FramedIo) {
        let (recv_queue, send_queue) = io.into_parts();
        self.peers
            .upsert_async(peer_id.clone(), Peer { send_queue })
            .await;
        self.round_robin.push(peer_id.clone());
        match &self.fair_queue_inner {
            None => {}
            Some(inner) => {
                inner.lock().insert(peer_id.clone(), recv_queue);
            }
        };
    }

    fn peer_disconnected(&self, peer_id: &PeerIdentity) {
        self.peers.remove_sync(peer_id);
        match &self.fair_queue_inner {
            None => {}
            Some(inner) => {
                inner.lock().remove(peer_id);
            }
        };
    }
}
