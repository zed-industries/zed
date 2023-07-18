use crate::{
    ClientNetwork, ClientRoom, Message, RoomCredentials, RoomName, RoomToken, ServerNetwork, User,
};
use anyhow::Result;
use collections::HashMap;
use futures::{channel::mpsc, future::BoxFuture, FutureExt, StreamExt};
use gpui::executor::Background;
use parking_lot::Mutex;
use std::{
    any::{type_name, Any, TypeId},
    collections::BTreeMap,
    sync::Arc,
};

pub struct TestNetwork(Arc<Mutex<NetworkState>>);

impl TestNetwork {
    pub fn new(executor: Arc<Background>) -> Self {
        Self(Arc::new(Mutex::new(NetworkState {
            executor,
            request_handlers: Default::default(),
            rooms: Default::default(),
        })))
    }

    pub fn server(&self) -> TestServer {
        TestServer(self.0.clone())
    }

    pub fn client(&self, login: impl Into<Arc<str>>) -> TestClient {
        TestClient {
            user: User {
                login: login.into(),
            },
            network: self.0.clone(),
        }
    }
}

struct NetworkState {
    executor: Arc<Background>,
    request_handlers: BTreeMap<
        TypeId,
        Box<dyn Send + Fn(User, Box<dyn Any>) -> BoxFuture<'static, Result<Box<dyn Any>>>>,
    >,
    rooms: BTreeMap<RoomName, Room>,
}

pub struct Room {
    inboxes: BTreeMap<RoomToken, mpsc::UnboundedSender<Vec<u8>>>,
}

pub struct TestServer(Arc<Mutex<NetworkState>>);

impl ServerNetwork for TestServer {
    fn on_request<H, F, R>(&self, handle_request: H)
    where
        H: 'static + Send + Fn(User, R) -> F,
        F: 'static + Send + futures::Future<Output = Result<R::Response>>,
        R: crate::Request,
    {
        self.0.lock().request_handlers.insert(
            TypeId::of::<R>(),
            Box::new(move |user, request| {
                let request = request.downcast::<R>().unwrap();
                let response = handle_request(user, *request);
                async move {
                    response
                        .await
                        .map(|response| Box::new(response) as Box<dyn Any>)
                }
                .boxed()
            }),
        );
    }

    fn create_room(&self, room: &RoomName) -> BoxFuture<Result<()>> {
        todo!()
    }

    fn grant_room_access(&self, room: &RoomName, user: &str) -> RoomToken {
        todo!()
    }
}

pub struct TestClient {
    user: User,
    network: Arc<Mutex<NetworkState>>,
}

impl ClientNetwork for TestClient {
    type Room = TestClientRoom;

    fn request<R: crate::Request>(
        &self,
        request: R,
    ) -> futures::future::BoxFuture<anyhow::Result<R::Response>> {
        let network = self.network.lock();
        let executor = network.executor.clone();
        let request = network
            .request_handlers
            .get(&TypeId::of::<R>())
            .expect(&format!(
                "handler for request {} not found",
                type_name::<R>()
            ))(self.user.clone(), Box::new(request));
        async move {
            executor.simulate_random_delay().await;
            let response = request
                .await
                .map(|response| *response.downcast::<R::Response>().unwrap());
            executor.simulate_random_delay().await;
            response
        }
        .boxed()
    }

    fn room(&self, credentials: RoomCredentials) -> Self::Room {
        TestClientRoom {
            outbox: Default::default(),
            credentials,
            message_handlers: Default::default(),
            network: self.network.clone(),
        }
    }
}

pub struct TestClientRoom {
    outbox: Option<mpsc::UnboundedSender<Vec<u8>>>,
    credentials: RoomCredentials,
    message_handlers:
        Arc<Mutex<HashMap<TypeId, Box<dyn Send + Sync + Fn(Vec<u8>) -> Result<(), Vec<u8>>>>>>,
    network: Arc<Mutex<NetworkState>>,
}

impl ClientRoom for TestClientRoom {
    fn connect(&mut self) -> BoxFuture<Result<()>> {
        assert!(
            self.outbox.is_none(),
            "client should not connect more than once"
        );

        let (inbox_tx, mut inbox_rx) = mpsc::unbounded();
        let existing_inbox = self
            .network
            .lock()
            .rooms
            .get_mut(&self.credentials.name)
            .expect("room should exist")
            .inboxes
            .insert(self.credentials.token.clone(), inbox_tx);
        assert!(
            existing_inbox.is_none(),
            "client should not connect twice with the same token"
        );
        let message_handlers = self.message_handlers.clone();
        self.network
            .lock()
            .executor
            .spawn(async move {
                while let Some(mut message) = inbox_rx.next().await {
                    for handler in message_handlers.lock().values() {
                        match handler(message) {
                            Ok(()) => break,
                            Err(unhandled_message) => message = unhandled_message,
                        }
                    }
                }
            })
            .detach();

        // Send outbound messages to other clients in the room.
        let (outbox_tx, mut outbox_rx) = mpsc::unbounded();
        self.outbox = Some(outbox_tx);
        let executor = self.network.lock().executor.clone();
        let network = self.network.clone();
        let credentials = self.credentials.clone();
        self.network
            .lock()
            .executor
            .spawn(async move {
                while let Some(message) = outbox_rx.next().await {
                    let inboxes = network
                        .lock()
                        .rooms
                        .get(&credentials.name)
                        .map(|room| room.inboxes.clone());
                    if let Some(inboxes) = inboxes {
                        for (inbox_token, inbox) in inboxes {
                            executor.simulate_random_delay().await;
                            if inbox_token != credentials.token {
                                let _ = inbox.unbounded_send(message.clone());
                            }
                        }
                    }
                }
            })
            .detach();

        async { Ok(()) }.boxed()
    }

    fn broadcast<M: Message>(&self, message: M) {
        let tx = self.outbox.as_ref().expect("must be connected");
        tx.unbounded_send(message.to_bytes())
            .expect("channel must be open");
    }

    fn on_message<M, F>(&self, handle_message: F)
    where
        M: Message,
        F: 'static + Send + Sync + Fn(M),
    {
        self.message_handlers.lock().insert(
            TypeId::of::<M>(),
            Box::new(move |bytes| {
                handle_message(M::from_bytes(bytes)?);
                Ok(())
            }),
        );
    }
}
