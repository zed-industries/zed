use crate::{ClientNetwork, ClientRoom, RoomCredentials, RoomName, RoomToken, ServerNetwork, User};
use anyhow::{anyhow, Result};
use futures::{channel::mpsc, future::BoxFuture, FutureExt, StreamExt};
use gpui::executor::Background;
use parking_lot::Mutex;
use std::{collections::BTreeMap, sync::Arc};

pub struct TestNetwork(Arc<Mutex<NetworkState>>);

impl TestNetwork {
    pub fn new(executor: Arc<Background>) -> Self {
        Self(Arc::new(Mutex::new(NetworkState {
            executor,
            request_handler: None,
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
    request_handler:
        Option<Box<dyn Send + Fn(User, Vec<u8>) -> Result<BoxFuture<'static, Result<Vec<u8>>>>>>,
    rooms: BTreeMap<RoomName, Room>,
}

#[derive(Default)]
pub struct Room {
    inboxes: BTreeMap<RoomToken, mpsc::UnboundedSender<Vec<u8>>>,
    authorized_users: BTreeMap<RoomToken, Arc<str>>,
    next_token_id: usize,
}

pub struct TestServer(Arc<Mutex<NetworkState>>);

impl ServerNetwork for TestServer {
    fn create_room(&self, name: &RoomName) -> BoxFuture<Result<()>> {
        let network = self.0.clone();
        let room = name.clone();
        async move {
            let executor = network.lock().executor.clone();
            executor.simulate_random_delay().await;
            network.lock().rooms.insert(room, Default::default());
            Ok(())
        }
        .boxed()
    }

    fn grant_room_access(&self, room: &RoomName, user: &str) -> RoomToken {
        let mut network = self.0.lock();
        let room = network.rooms.get_mut(&room).expect("room must exist");
        let token_id = room.next_token_id;
        room.next_token_id += 1;
        let token = RoomToken(format!("{}/{}", token_id, user).into());
        room.authorized_users.insert(token.clone(), user.into());
        token
    }

    fn handle_requests<H, F>(&self, handle_request: H)
    where
        H: 'static + Send + Fn(User, Vec<u8>) -> Result<F>,
        F: 'static + Send + futures::Future<Output = Result<Vec<u8>>>,
    {
        self.0.lock().request_handler = Some(Box::new(move |user, request| {
            handle_request(user, request.clone()).map(FutureExt::boxed)
        }));
    }
}

pub struct TestClient {
    user: User,
    network: Arc<Mutex<NetworkState>>,
}

impl ClientNetwork for TestClient {
    type Room = TestClientRoom;

    fn request(&self, request: Vec<u8>) -> BoxFuture<Result<Vec<u8>>> {
        let response =
            self.network.lock().request_handler.as_ref().unwrap()(self.user.clone(), request);
        async move { response?.await }.boxed()
    }

    fn room(&self, credentials: RoomCredentials) -> Self::Room {
        TestClientRoom {
            outbox: Default::default(),
            credentials,
            message_handler: Default::default(),
            network: self.network.clone(),
        }
    }
}

pub struct TestClientRoom {
    outbox: Option<mpsc::UnboundedSender<Vec<u8>>>,
    credentials: RoomCredentials,
    message_handler: Arc<Mutex<Option<Box<dyn Send + Fn(Vec<u8>)>>>>,
    network: Arc<Mutex<NetworkState>>,
}

impl ClientRoom for TestClientRoom {
    fn connect(&mut self) -> BoxFuture<Result<()>> {
        assert!(
            self.outbox.is_none(),
            "client should not connect more than once"
        );

        let (inbox_tx, mut inbox_rx) = mpsc::unbounded();
        {
            let mut network = self.network.lock();
            let room = network
                .rooms
                .get_mut(&self.credentials.name)
                .expect("room should exist");

            if !room.authorized_users.contains_key(&self.credentials.token) {
                return std::future::ready(Err(anyhow!(
                    "token {:?} is not authorized to enter room {:?}",
                    self.credentials.token,
                    self.credentials.name
                )))
                .boxed();
            }

            let existing_inbox = room
                .inboxes
                .insert(self.credentials.token.clone(), inbox_tx);
            assert!(
                existing_inbox.is_none(),
                "client should not connect twice with the same token"
            );
        }
        let message_handler = self.message_handler.clone();
        self.network
            .lock()
            .executor
            .spawn(async move {
                while let Some(message) = inbox_rx.next().await {
                    if let Some(handler) = message_handler.lock().as_ref() {
                        handler(message);
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

    fn broadcast(&self, message: Vec<u8>) {
        let tx = self.outbox.as_ref().expect("must be connected");
        tx.unbounded_send(message).expect("channel must be open");
    }

    fn handle_messages(&self, handle_message: impl 'static + Send + Fn(Vec<u8>)) {
        self.message_handler
            .lock()
            .replace(Box::new(handle_message));
    }
}
