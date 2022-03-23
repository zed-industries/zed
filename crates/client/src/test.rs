use crate::{
    http::{HttpClient, Request, Response, ServerResponse},
    Client, Connection, Credentials, EstablishConnectionError, UserStore,
};
use anyhow::{anyhow, Result};
use futures::{future::BoxFuture, stream::BoxStream, Future, StreamExt};
use gpui::{executor, ModelHandle, TestAppContext};
use parking_lot::Mutex;
use postage::barrier;
use rpc::{proto, ConnectionId, Peer, Receipt, TypedEnvelope};
use std::{fmt, rc::Rc, sync::Arc};

pub struct FakeServer {
    peer: Arc<Peer>,
    state: Arc<Mutex<FakeServerState>>,
    user_id: u64,
    executor: Rc<executor::Foreground>,
}

#[derive(Default)]
struct FakeServerState {
    incoming: Option<BoxStream<'static, Box<dyn proto::AnyTypedEnvelope>>>,
    connection_id: Option<ConnectionId>,
    forbid_connections: bool,
    auth_count: usize,
    connection_killer: Option<barrier::Sender>,
    access_token: usize,
}

impl FakeServer {
    pub async fn for_client(
        client_user_id: u64,
        client: &mut Arc<Client>,
        cx: &TestAppContext,
    ) -> Self {
        let server = Self {
            peer: Peer::new(),
            state: Default::default(),
            user_id: client_user_id,
            executor: cx.foreground(),
        };

        Arc::get_mut(client)
            .unwrap()
            .override_authenticate({
                let state = server.state.clone();
                move |cx| {
                    let mut state = state.lock();
                    state.auth_count += 1;
                    let access_token = state.access_token.to_string();
                    cx.spawn(move |_| async move {
                        Ok(Credentials {
                            user_id: client_user_id,
                            access_token,
                        })
                    })
                }
            })
            .override_establish_connection({
                let peer = server.peer.clone();
                let state = server.state.clone();
                move |credentials, cx| {
                    let peer = peer.clone();
                    let state = state.clone();
                    let credentials = credentials.clone();
                    cx.spawn(move |cx| async move {
                        assert_eq!(credentials.user_id, client_user_id);

                        if state.lock().forbid_connections {
                            Err(EstablishConnectionError::Other(anyhow!(
                                "server is forbidding connections"
                            )))?
                        }

                        if credentials.access_token != state.lock().access_token.to_string() {
                            Err(EstablishConnectionError::Unauthorized)?
                        }

                        let (client_conn, server_conn, kill) =
                            Connection::in_memory(cx.background());
                        let (connection_id, io, incoming) =
                            peer.add_test_connection(server_conn, cx.background()).await;
                        cx.background().spawn(io).detach();
                        let mut state = state.lock();
                        state.connection_id = Some(connection_id);
                        state.incoming = Some(incoming);
                        state.connection_killer = Some(kill);
                        Ok(client_conn)
                    })
                }
            });

        client
            .authenticate_and_connect(false, &cx.to_async())
            .await
            .unwrap();
        server
    }

    pub fn disconnect(&self) {
        self.peer.disconnect(self.connection_id());
        let mut state = self.state.lock();
        state.connection_id.take();
        state.incoming.take();
    }

    pub fn auth_count(&self) -> usize {
        self.state.lock().auth_count
    }

    pub fn roll_access_token(&self) {
        self.state.lock().access_token += 1;
    }

    pub fn forbid_connections(&self) {
        self.state.lock().forbid_connections = true;
    }

    pub fn allow_connections(&self) {
        self.state.lock().forbid_connections = false;
    }

    pub fn send<T: proto::EnvelopedMessage>(&self, message: T) {
        self.peer.send(self.connection_id(), message).unwrap();
    }

    pub async fn receive<M: proto::EnvelopedMessage>(&self) -> Result<TypedEnvelope<M>> {
        self.executor.start_waiting();
        let message = self
            .state
            .lock()
            .incoming
            .as_mut()
            .expect("not connected")
            .next()
            .await
            .ok_or_else(|| anyhow!("other half hung up"))?;
        self.executor.finish_waiting();
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
        self.state.lock().connection_id.expect("not connected")
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
