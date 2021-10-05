use super::*;
use std::sync::atomic::Ordering::SeqCst;

use super::Client;
use gpui::TestAppContext;
use parking_lot::Mutex;
use postage::{mpsc, prelude::Stream};
use rpc::{proto, ConnectionId, Peer, Receipt, TypedEnvelope};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize},
    Arc,
};

pub struct FakeServer {
    peer: Arc<Peer>,
    incoming: Mutex<Option<mpsc::Receiver<Box<dyn proto::AnyTypedEnvelope>>>>,
    connection_id: Mutex<Option<ConnectionId>>,
    forbid_connections: AtomicBool,
    auth_count: AtomicUsize,
    access_token: AtomicUsize,
    user_id: u64,
}

impl FakeServer {
    pub async fn for_client(
        client_user_id: u64,
        client: &mut Arc<Client>,
        cx: &TestAppContext,
    ) -> Arc<Self> {
        let server = Arc::new(Self {
            peer: Peer::new(),
            incoming: Default::default(),
            connection_id: Default::default(),
            forbid_connections: Default::default(),
            auth_count: Default::default(),
            access_token: Default::default(),
            user_id: client_user_id,
        });

        Arc::get_mut(client)
            .unwrap()
            .override_authenticate({
                let server = server.clone();
                move |cx| {
                    server.auth_count.fetch_add(1, SeqCst);
                    let access_token = server.access_token.load(SeqCst).to_string();
                    cx.spawn(move |_| async move {
                        Ok(Credentials {
                            user_id: client_user_id,
                            access_token,
                        })
                    })
                }
            })
            .override_establish_connection({
                let server = server.clone();
                move |credentials, cx| {
                    let credentials = credentials.clone();
                    cx.spawn({
                        let server = server.clone();
                        move |cx| async move { server.establish_connection(&credentials, &cx).await }
                    })
                }
            });

        client
            .authenticate_and_connect(&cx.to_async())
            .await
            .unwrap();
        server
    }

    pub async fn disconnect(&self) {
        self.peer.disconnect(self.connection_id()).await;
        self.connection_id.lock().take();
        self.incoming.lock().take();
    }

    async fn establish_connection(
        &self,
        credentials: &Credentials,
        cx: &AsyncAppContext,
    ) -> Result<Connection, EstablishConnectionError> {
        assert_eq!(credentials.user_id, self.user_id);

        if self.forbid_connections.load(SeqCst) {
            Err(EstablishConnectionError::Other(anyhow!(
                "server is forbidding connections"
            )))?
        }

        if credentials.access_token != self.access_token.load(SeqCst).to_string() {
            Err(EstablishConnectionError::Unauthorized)?
        }

        let (client_conn, server_conn, _) = Connection::in_memory();
        let (connection_id, io, incoming) = self.peer.add_connection(server_conn).await;
        cx.background().spawn(io).detach();
        *self.incoming.lock() = Some(incoming);
        *self.connection_id.lock() = Some(connection_id);
        Ok(client_conn)
    }

    pub fn auth_count(&self) -> usize {
        self.auth_count.load(SeqCst)
    }

    pub fn roll_access_token(&self) {
        self.access_token.fetch_add(1, SeqCst);
    }

    pub fn forbid_connections(&self) {
        self.forbid_connections.store(true, SeqCst);
    }

    pub fn allow_connections(&self) {
        self.forbid_connections.store(false, SeqCst);
    }

    pub async fn send<T: proto::EnvelopedMessage>(&self, message: T) {
        self.peer.send(self.connection_id(), message).await.unwrap();
    }

    pub async fn receive<M: proto::EnvelopedMessage>(&self) -> Result<TypedEnvelope<M>> {
        let message = self
            .incoming
            .lock()
            .as_mut()
            .expect("not connected")
            .recv()
            .await
            .ok_or_else(|| anyhow!("other half hung up"))?;
        let type_name = message.payload_type_name();
        Ok(*message
            .into_any()
            .downcast::<TypedEnvelope<M>>()
            .unwrap_or_else(|_| {
                panic!(
                    "fake server received unexpected message type: {:?}",
                    type_name
                );
            }))
    }

    pub async fn respond<T: proto::RequestMessage>(
        &self,
        receipt: Receipt<T>,
        response: T::Response,
    ) {
        self.peer.respond(receipt, response).await.unwrap()
    }

    fn connection_id(&self) -> ConnectionId {
        self.connection_id.lock().expect("not connected")
    }
}
