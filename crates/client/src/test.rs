use super::Client;
use super::*;
use crate::http::{HttpClient, Request, Response, ServerResponse};
use futures::{future::BoxFuture, stream::BoxStream, Future, StreamExt};
use gpui::{ModelHandle, TestAppContext};
use parking_lot::Mutex;
use rpc::{proto, ConnectionId, Peer, Receipt, TypedEnvelope};
use std::fmt;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize},
    Arc,
};

pub struct FakeServer {
    peer: Arc<Peer>,
    incoming: Mutex<Option<BoxStream<'static, Box<dyn proto::AnyTypedEnvelope>>>>,
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

    pub fn disconnect(&self) {
        self.peer.disconnect(self.connection_id());
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

        let (client_conn, server_conn, _) = Connection::in_memory(cx.background());
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

    pub fn send<T: proto::EnvelopedMessage>(&self, message: T) {
        self.peer.send(self.connection_id(), message).unwrap();
    }

    pub async fn receive<M: proto::EnvelopedMessage>(&self) -> Result<TypedEnvelope<M>> {
        let message = self
            .incoming
            .lock()
            .as_mut()
            .expect("not connected")
            .next()
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
        self.peer.respond(receipt, response).unwrap()
    }

    fn connection_id(&self) -> ConnectionId {
        self.connection_id.lock().expect("not connected")
    }

    pub async fn build_user_store(
        &self,
        client: Arc<Client>,
        cx: &mut TestAppContext,
    ) -> ModelHandle<UserStore> {
        let http_client = FakeHttpClient::with_404_response();
        let user_store = cx.add_model(|cx| UserStore::new(client, http_client, cx));
        assert_eq!(
            self.receive::<proto::GetUsers>()
                .await
                .unwrap()
                .payload
                .user_ids,
            &[self.user_id]
        );
        user_store
    }
}

pub struct FakeHttpClient {
    handler:
        Box<dyn 'static + Send + Sync + Fn(Request) -> BoxFuture<'static, Result<ServerResponse>>>,
}

impl FakeHttpClient {
    pub fn new<Fut, F>(handler: F) -> Arc<dyn HttpClient>
    where
        Fut: 'static + Send + Future<Output = Result<ServerResponse>>,
        F: 'static + Send + Sync + Fn(Request) -> Fut,
    {
        Arc::new(Self {
            handler: Box::new(move |req| Box::pin(handler(req))),
        })
    }

    pub fn with_404_response() -> Arc<dyn HttpClient> {
        Self::new(|_| async move { Ok(ServerResponse::new(404)) })
    }
}

impl fmt::Debug for FakeHttpClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FakeHttpClient").finish()
    }
}

impl HttpClient for FakeHttpClient {
    fn send<'a>(&'a self, req: Request) -> BoxFuture<'a, Result<Response>> {
        let future = (self.handler)(req);
        Box::pin(async move { future.await.map(Into::into) })
    }
}
