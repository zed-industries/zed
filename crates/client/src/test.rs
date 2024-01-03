use crate::{Client, Connection, Credentials, EstablishConnectionError, UserStore};
use anyhow::{anyhow, Result};
use futures::{stream::BoxStream, StreamExt};
use gpui::{BackgroundExecutor, Context, Model, TestAppContext};
use parking_lot::Mutex;
use rpc::{
    proto::{self, GetPrivateUserInfo, GetPrivateUserInfoResponse},
    ConnectionId, Peer, Receipt, TypedEnvelope,
};
use std::sync::Arc;

pub struct FakeServer {
    peer: Arc<Peer>,
    state: Arc<Mutex<FakeServerState>>,
    user_id: u64,
    executor: BackgroundExecutor,
}

#[derive(Default)]
struct FakeServerState {
    incoming: Option<BoxStream<'static, Box<dyn proto::AnyTypedEnvelope>>>,
    connection_id: Option<ConnectionId>,
    forbid_connections: bool,
    auth_count: usize,
    access_token: usize,
}

impl FakeServer {
    pub async fn for_client(
        client_user_id: u64,
        client: &Arc<Client>,
        cx: &TestAppContext,
    ) -> Self {
        let server = Self {
            peer: Peer::new(0),
            state: Default::default(),
            user_id: client_user_id,
            executor: cx.executor(),
        };

        client
            .override_authenticate({
                let state = Arc::downgrade(&server.state);
                move |cx| {
                    let state = state.clone();
                    cx.spawn(move |_| async move {
                        let state = state.upgrade().ok_or_else(|| anyhow!("server dropped"))?;
                        let mut state = state.lock();
                        state.auth_count += 1;
                        let access_token = state.access_token.to_string();
                        Ok(Credentials {
                            user_id: client_user_id,
                            access_token,
                        })
                    })
                }
            })
            .override_establish_connection({
                let peer = Arc::downgrade(&server.peer);
                let state = Arc::downgrade(&server.state);
                move |credentials, cx| {
                    let peer = peer.clone();
                    let state = state.clone();
                    let credentials = credentials.clone();
                    cx.spawn(move |cx| async move {
                        let state = state.upgrade().ok_or_else(|| anyhow!("server dropped"))?;
                        let peer = peer.upgrade().ok_or_else(|| anyhow!("server dropped"))?;
                        if state.lock().forbid_connections {
                            Err(EstablishConnectionError::Other(anyhow!(
                                "server is forbidding connections"
                            )))?
                        }

                        assert_eq!(credentials.user_id, client_user_id);

                        if credentials.access_token != state.lock().access_token.to_string() {
                            Err(EstablishConnectionError::Unauthorized)?
                        }

                        let (client_conn, server_conn, _) =
                            Connection::in_memory(cx.background_executor().clone());
                        let (connection_id, io, incoming) =
                            peer.add_test_connection(server_conn, cx.background_executor().clone());
                        cx.background_executor().spawn(io).detach();
                        {
                            let mut state = state.lock();
                            state.connection_id = Some(connection_id);
                            state.incoming = Some(incoming);
                        }
                        peer.send(
                            connection_id,
                            proto::Hello {
                                peer_id: Some(connection_id.into()),
                            },
                        )
                        .unwrap();

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
        if self.state.lock().connection_id.is_some() {
            self.peer.disconnect(self.connection_id());
            let mut state = self.state.lock();
            state.connection_id.take();
            state.incoming.take();
        }
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

    #[allow(clippy::await_holding_lock)]
    pub async fn receive<M: proto::EnvelopedMessage>(&self) -> Result<TypedEnvelope<M>> {
        self.executor.start_waiting();

        loop {
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
            let message = message.into_any();

            if message.is::<TypedEnvelope<M>>() {
                return Ok(*message.downcast().unwrap());
            }

            if message.is::<TypedEnvelope<GetPrivateUserInfo>>() {
                self.respond(
                    message
                        .downcast::<TypedEnvelope<GetPrivateUserInfo>>()
                        .unwrap()
                        .receipt(),
                    GetPrivateUserInfoResponse {
                        metrics_id: "the-metrics-id".into(),
                        staff: false,
                        flags: Default::default(),
                    },
                );
                continue;
            }

            panic!(
                "fake server received unexpected message type: {:?}",
                type_name
            );
        }
    }

    pub fn respond<T: proto::RequestMessage>(&self, receipt: Receipt<T>, response: T::Response) {
        self.peer.respond(receipt, response).unwrap()
    }

    fn connection_id(&self) -> ConnectionId {
        self.state.lock().connection_id.expect("not connected")
    }

    pub async fn build_user_store(
        &self,
        client: Arc<Client>,
        cx: &mut TestAppContext,
    ) -> Model<UserStore> {
        let user_store = cx.new_model(|cx| UserStore::new(client, cx));
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

impl Drop for FakeServer {
    fn drop(&mut self) {
        self.disconnect();
    }
}
