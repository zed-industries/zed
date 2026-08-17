use crate::codec::*;
use crate::endpoint::Endpoint;
use crate::error::ZmqResult;
use crate::fair_queue::{FairQueue, QueueInner};
use crate::message::*;
use crate::transport::AcceptStopHandle;
use crate::util::PeerIdentity;
use crate::{CaptureSocket, SocketOptions};
use crate::{
    MultiPeerBackend, Socket, SocketBackend, SocketEvent, SocketRecv, SocketSend, SocketType,
    ZmqError,
};

use async_trait::async_trait;
use futures::channel::mpsc;
use futures::StreamExt;
use parking_lot::Mutex;

use std::collections::HashMap;
use std::io::ErrorKind;
use std::pin::Pin;
use std::sync::Arc;

pub(crate) struct XPubSubscriber {
    pub(crate) subscriptions: Vec<Vec<u8>>,
    pub(crate) send_queue: Pin<Box<ZmqFramedWrite>>,
}

pub(crate) struct XPubSocketBackend {
    subscribers: scc::HashMap<PeerIdentity, XPubSubscriber>,
    fair_queue_inner: Arc<Mutex<QueueInner<ZmqFramedRead, PeerIdentity>>>,
    socket_monitor: Mutex<Option<mpsc::Sender<SocketEvent>>>,
    socket_options: SocketOptions,
}

impl XPubSocketBackend {
    fn message_received(&self, peer_id: &PeerIdentity, message: Message) {
        let data = match message {
            Message::Message(m) => {
                if m.len() != 1 {
                    log::warn!("Received message with unexpected length: {}", m.len());
                    return;
                }
                m.into_vec().pop().unwrap_or_default()
            }
            _ => return,
        };

        if data.is_empty() {
            return;
        }

        match data.first() {
            Some(1) => {
                // Subscribe
                if let Some(mut entry) = self.subscribers.get_sync(peer_id) {
                    entry.subscriptions.push(Vec::from(&data[1..]));
                }
            }
            Some(0) => {
                // Unsubscribe
                let sub = Vec::from(&data[1..]);
                if let Some(mut entry) = self.subscribers.get_sync(peer_id) {
                    if let Some(index) = entry.subscriptions.iter().position(|s| s == &sub) {
                        entry.subscriptions.remove(index);
                    }
                }
            }
            _ => log::warn!(
                "Received message with unexpected first byte: {:?}",
                data.first()
            ),
        }
    }
}

impl SocketBackend for XPubSocketBackend {
    fn socket_type(&self) -> SocketType {
        SocketType::XPUB
    }

    fn socket_options(&self) -> &SocketOptions {
        &self.socket_options
    }

    fn shutdown(&self) {
        self.subscribers.clear_sync();
    }

    fn monitor(&self) -> &Mutex<Option<mpsc::Sender<SocketEvent>>> {
        &self.socket_monitor
    }
}

#[async_trait]
impl MultiPeerBackend for XPubSocketBackend {
    async fn peer_connected(self: Arc<Self>, peer_id: &PeerIdentity, io: FramedIo) {
        let (recv_queue, send_queue) = io.into_parts();

        self.subscribers
            .upsert_async(
                peer_id.clone(),
                XPubSubscriber {
                    subscriptions: vec![],
                    send_queue: Box::pin(send_queue),
                },
            )
            .await;

        self.fair_queue_inner
            .lock()
            .insert(peer_id.clone(), recv_queue);
    }

    fn peer_disconnected(&self, peer_id: &PeerIdentity) {
        log::info!("Client disconnected {:?}", peer_id);
        self.subscribers.remove_sync(peer_id);
        self.fair_queue_inner.lock().remove(peer_id);
    }
}

pub struct XPubSocket {
    pub(crate) backend: Arc<XPubSocketBackend>,
    fair_queue: FairQueue<ZmqFramedRead, PeerIdentity>,
    binds: HashMap<Endpoint, AcceptStopHandle>,
}

impl Drop for XPubSocket {
    fn drop(&mut self) {
        self.backend.shutdown();
    }
}

#[async_trait]
impl SocketSend for XPubSocket {
    async fn send(&mut self, message: ZmqMessage) -> ZmqResult<()> {
        let mut dead_peers = Vec::new();
        let mut iter = self.backend.subscribers.begin_async().await;
        while let Some(mut subscriber) = iter {
            for sub_filter in &subscriber.subscriptions {
                if sub_filter.len() <= message.get(0).unwrap().len()
                    && sub_filter.as_slice() == &message.get(0).unwrap()[0..sub_filter.len()]
                {
                    let res = subscriber
                        .send_queue
                        .as_mut()
                        .try_send(Message::Message(message.clone()));
                    match res {
                        Ok(()) => {}
                        Err(ZmqError::Codec(CodecError::Io(e))) => {
                            if e.kind() == ErrorKind::BrokenPipe {
                                dead_peers.push(subscriber.key().clone());
                            } else {
                                log::error!("Error sending message: {:?}", e);
                            }
                        }
                        Err(ZmqError::BufferFull(_)) => {
                            // Silently drop the message if the queue for a subscriber is full.
                            // https://rfc.zeromq.org/spec/29/
                            log::debug!("Queue for subscriber is full");
                        }
                        Err(e) => {
                            log::error!("Error sending message: {:?}", e);
                            return Err(e);
                        }
                    }
                    break;
                }
            }
            iter = subscriber.next_async().await;
        }
        for peer in dead_peers {
            self.backend.peer_disconnected(&peer);
        }
        Ok(())
    }
}

#[async_trait]
impl SocketRecv for XPubSocket {
    async fn recv(&mut self) -> ZmqResult<ZmqMessage> {
        loop {
            match self.fair_queue.next().await {
                Some((peer_id, Ok(Message::Message(message)))) => {
                    // Process the subscription message internally to update tracking
                    self.backend
                        .message_received(&peer_id, Message::Message(message.clone()));
                    // Also expose it to the application
                    return Ok(message);
                }
                Some((_peer_id, Ok(_msg))) => {
                    // Ignore non-message frames
                }
                Some((peer_id, Err(e))) => {
                    self.backend.peer_disconnected(&peer_id);
                    return Err(e.into());
                }
                None => {
                    return Err(ZmqError::NoMessage);
                }
            }
        }
    }
}

impl CaptureSocket for XPubSocket {}

#[async_trait]
impl Socket for XPubSocket {
    fn with_options(options: SocketOptions) -> Self {
        let fair_queue = FairQueue::new(true);
        Self {
            backend: Arc::new(XPubSocketBackend {
                subscribers: scc::HashMap::new(),
                fair_queue_inner: fair_queue.inner(),
                socket_monitor: Mutex::new(None),
                socket_options: options,
            }),
            fair_queue,
            binds: HashMap::new(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::async_rt;
    use crate::util::tests::{
        test_bind_to_any_port_helper, test_bind_to_unspecified_interface_helper,
    };
    use crate::ZmqResult;
    use std::net::IpAddr;

    #[async_rt::test]
    async fn test_bind_to_any_port() -> ZmqResult<()> {
        let s = XPubSocket::new();
        test_bind_to_any_port_helper(s).await
    }

    #[async_rt::test]
    async fn test_bind_to_any_ipv4_interface() -> ZmqResult<()> {
        let any_ipv4: IpAddr = "0.0.0.0".parse().unwrap();
        let s = XPubSocket::new();
        test_bind_to_unspecified_interface_helper(any_ipv4, s, 4020).await
    }

    #[async_rt::test]
    async fn test_bind_to_any_ipv6_interface() -> ZmqResult<()> {
        let any_ipv6: IpAddr = "::".parse().unwrap();
        let s = XPubSocket::new();
        test_bind_to_unspecified_interface_helper(any_ipv6, s, 4030).await
    }
}
