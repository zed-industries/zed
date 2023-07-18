use crate::{ClientNetwork, Message, RoomName, RoomToken, ServerNetwork};
use anyhow::Result;
use futures::{future::BoxFuture, FutureExt};
use parking_lot::Mutex;
use std::{
    any::{type_name, Any, TypeId},
    collections::BTreeMap,
    sync::Arc,
};

#[derive(Default)]
pub struct TestNetwork(Arc<Mutex<NetworkState>>);

impl TestNetwork {
    pub fn server(&self) -> TestServer {
        TestServer(self.0.clone())
    }

    pub fn client(&self) -> TestClient {
        TestClient(self.0.clone())
    }
}

#[derive(Default)]
struct NetworkState {
    request_handlers: BTreeMap<
        TypeId,
        Box<dyn Send + Fn(Box<dyn Any>) -> BoxFuture<'static, Result<Box<dyn Any>>>>,
    >,
    rooms: BTreeMap<RoomName, Room>,
}

pub struct Room {
    inboxes: BTreeMap<RoomToken, Vec<Box<dyn Message>>>,
}

pub struct TestServer(Arc<Mutex<NetworkState>>);

impl ServerNetwork for TestServer {
    fn on_request<H, F, R>(&self, handle_request: H)
    where
        H: 'static + Send + Sync + Fn(R) -> F,
        F: 'static + Send + Sync + futures::Future<Output = Result<R::Response>>,
        R: crate::Request,
    {
        self.0.lock().request_handlers.insert(
            TypeId::of::<R>(),
            Box::new(move |request| {
                let request = request.downcast::<R>().unwrap();
                let response = handle_request(*request);
                async move {
                    response
                        .await
                        .map(|response| Box::new(response) as Box<dyn Any>)
                }
                .boxed()
            }),
        );
    }
}

pub struct TestClient(Arc<Mutex<NetworkState>>);

impl ClientNetwork for TestClient {
    fn request<R: crate::Request>(
        &self,
        request: R,
    ) -> futures::future::BoxFuture<anyhow::Result<R::Response>> {
        let request = self
            .0
            .lock()
            .request_handlers
            .get(&TypeId::of::<R>())
            .expect(&format!(
                "handler for request {} not found",
                type_name::<R>()
            ))(Box::new(request));
        async move {
            request
                .await
                .map(|response| *response.downcast::<R::Response>().unwrap())
        }
        .boxed()
    }

    fn broadcast<M: Message>(&self, room: RoomName, token: RoomToken, message: M) {
        todo!()
    }
}
