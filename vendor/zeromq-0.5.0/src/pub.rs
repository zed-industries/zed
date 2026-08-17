use crate::codec::*;
use crate::endpoint::Endpoint;
use crate::error::ZmqResult;
use crate::message::*;
use crate::transport::AcceptStopHandle;
use crate::util::PeerIdentity;
use crate::{async_rt, CaptureSocket, SocketOptions};
use crate::{
    MultiPeerBackend, Socket, SocketBackend, SocketEvent, SocketSend, SocketType, ZmqError,
};

use async_trait::async_trait;
use futures::channel::{mpsc, oneshot};
use futures::{select, FutureExt, StreamExt};
use parking_lot::Mutex;

use std::collections::HashMap;
use std::io::ErrorKind;
use std::pin::Pin;
use std::sync::Arc;

pub(crate) struct Subscriber {
    pub(crate) subscriptions: Vec<Vec<u8>>,
    pub(crate) send_queue: Pin<Box<ZmqFramedWrite>>,
    _subscription_coro_stop: oneshot::Sender<()>,
}

pub(crate) struct PubSocketBackend {
    subscribers: scc::HashMap<PeerIdentity, Subscriber>,
    socket_monitor: Mutex<Option<mpsc::Sender<SocketEvent>>>,
    socket_options: SocketOptions,
}

impl PubSocketBackend {
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

impl SocketBackend for PubSocketBackend {
    fn socket_type(&self) -> SocketType {
        SocketType::PUB
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
impl MultiPeerBackend for PubSocketBackend {
    async fn peer_connected(self: Arc<Self>, peer_id: &PeerIdentity, io: FramedIo) {
        let (mut recv_queue, send_queue) = io.into_parts();
        // TODO provide handling for recv_queue
        let (sender, stop_receiver) = oneshot::channel();
        self.subscribers
            .upsert_async(
                peer_id.clone(),
                Subscriber {
                    subscriptions: vec![],
                    send_queue: Box::pin(send_queue),
                    _subscription_coro_stop: sender,
                },
            )
            .await;
        let backend = self;
        let peer_id = peer_id.clone();
        async_rt::task::spawn(async move {
            let mut stop_receiver = stop_receiver.fuse();
            loop {
                select! {
                     _ = stop_receiver => {
                         break;
                     },
                     message = recv_queue.next().fuse() => {
                        match message {
                            Some(Ok(m)) => backend.message_received(&peer_id, m),
                            Some(Err(e)) => {
                                log::debug!("Error receiving message: {:?}", e);
                                backend.peer_disconnected(&peer_id);
                                break;
                            }
                            None => {
                                backend.peer_disconnected(&peer_id);
                                break
                            }
                        }

                     }
                }
            }
        });
    }

    fn peer_disconnected(&self, peer_id: &PeerIdentity) {
        log::info!("Client disconnected {:?}", peer_id);
        if let Some(monitor) = self.monitor().lock().as_mut() {
            let _ = monitor.try_send(SocketEvent::Disconnected(peer_id.clone()));
        }
        self.subscribers.remove_sync(peer_id);
    }
}

pub struct PubSocket {
    pub(crate) backend: Arc<PubSocketBackend>,
    binds: HashMap<Endpoint, AcceptStopHandle>,
}

impl Drop for PubSocket {
    fn drop(&mut self) {
        self.backend.shutdown();
    }
}

#[async_trait]
impl SocketSend for PubSocket {
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
                                log::error!("Error receiving message: {:?}", e);
                            }
                        }
                        Err(ZmqError::BufferFull(_)) => {
                            // ignore silently. https://rfc.zeromq.org/spec/29/ says:
                            // For processing outgoing messages:
                            //   SHALL silently drop the message if the queue for a subscriber is full.
                            log::debug!("Queue for subscriber is full",);
                        }
                        Err(e) => {
                            log::error!("Error receiving message: {:?}", e);
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

impl CaptureSocket for PubSocket {}

#[async_trait]
impl Socket for PubSocket {
    fn with_options(options: SocketOptions) -> Self {
        Self {
            backend: Arc::new(PubSocketBackend {
                subscribers: scc::HashMap::new(),
                socket_monitor: Mutex::new(None),
                socket_options: options,
            }),
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
    use crate::util::tests::{
        test_bind_to_any_port_helper, test_bind_to_unspecified_interface_helper,
    };
    use crate::ZmqResult;
    use std::net::IpAddr;

    #[async_rt::test]
    async fn test_bind_to_any_port() -> ZmqResult<()> {
        let s = PubSocket::new();
        test_bind_to_any_port_helper(s).await
    }

    #[async_rt::test]
    async fn test_bind_to_any_ipv4_interface() -> ZmqResult<()> {
        let any_ipv4: IpAddr = "0.0.0.0".parse().unwrap();
        let s = PubSocket::new();
        test_bind_to_unspecified_interface_helper(any_ipv4, s, 4000).await
    }

    #[async_rt::test]
    async fn test_bind_to_any_ipv6_interface() -> ZmqResult<()> {
        let any_ipv6: IpAddr = "::".parse().unwrap();
        let s = PubSocket::new();
        test_bind_to_unspecified_interface_helper(any_ipv6, s, 4010).await
    }
}
