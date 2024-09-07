use crate::{
    error::ErrorExt as _, AnyTypedEnvelope, EntityMessage, Envelope, EnvelopedMessage,
    RequestMessage, TypedEnvelope,
};
use anyhow::anyhow;
use collections::HashMap;
use futures::{
    future::{BoxFuture, LocalBoxFuture},
    Future, FutureExt as _,
};
use gpui::{AnyModel, AnyWeakModel, AsyncAppContext, Model};
pub use prost::Message;
use std::{any::TypeId, sync::Arc};

#[derive(Clone)]
pub struct AnyProtoClient(Arc<dyn ProtoClient>);

pub trait ProtoClient: Send + Sync {
    fn request(
        &self,
        envelope: Envelope,
        request_type: &'static str,
    ) -> BoxFuture<'static, anyhow::Result<Envelope>>;

    fn send(&self, envelope: Envelope, message_type: &'static str) -> anyhow::Result<()>;

    fn send_response(&self, envelope: Envelope, message_type: &'static str) -> anyhow::Result<()>;

    fn message_handler_set(&self) -> &parking_lot::Mutex<ProtoMessageHandlerSet>;
}

#[derive(Default)]
pub struct ProtoMessageHandlerSet {
    pub entity_types_by_message_type: HashMap<TypeId, TypeId>,
    pub entities_by_type_and_remote_id: HashMap<(TypeId, u64), EntityMessageSubscriber>,
    pub entity_id_extractors: HashMap<TypeId, fn(&dyn AnyTypedEnvelope) -> u64>,
    pub models_by_message_type: HashMap<TypeId, AnyWeakModel>,
    pub message_handlers: HashMap<TypeId, ProtoMessageHandler>,
}

pub type ProtoMessageHandler = Arc<
    dyn Send
        + Sync
        + Fn(
            AnyModel,
            Box<dyn AnyTypedEnvelope>,
            AnyProtoClient,
            AsyncAppContext,
        ) -> LocalBoxFuture<'static, anyhow::Result<()>>,
>;

impl ProtoMessageHandlerSet {
    pub fn clear(&mut self) {
        self.message_handlers.clear();
        self.models_by_message_type.clear();
        self.entities_by_type_and_remote_id.clear();
        self.entity_id_extractors.clear();
    }

    fn add_message_handler(
        &mut self,
        message_type_id: TypeId,
        model: gpui::AnyWeakModel,
        handler: ProtoMessageHandler,
    ) {
        self.models_by_message_type.insert(message_type_id, model);
        let prev_handler = self.message_handlers.insert(message_type_id, handler);
        if prev_handler.is_some() {
            panic!("registered handler for the same message twice");
        }
    }

    fn add_entity_message_handler(
        &mut self,
        message_type_id: TypeId,
        model_type_id: TypeId,
        entity_id_extractor: fn(&dyn AnyTypedEnvelope) -> u64,
        handler: ProtoMessageHandler,
    ) {
        self.entity_id_extractors
            .entry(message_type_id)
            .or_insert(entity_id_extractor);
        self.entity_types_by_message_type
            .insert(message_type_id, model_type_id);
        let prev_handler = self.message_handlers.insert(message_type_id, handler);
        if prev_handler.is_some() {
            panic!("registered handler for the same message twice");
        }
    }

    pub fn handle_message(
        this: &parking_lot::Mutex<Self>,
        message: Box<dyn AnyTypedEnvelope>,
        client: AnyProtoClient,
        cx: AsyncAppContext,
    ) -> Option<LocalBoxFuture<'static, anyhow::Result<()>>> {
        let payload_type_id = message.payload_type_id();
        let mut this = this.lock();
        let handler = this.message_handlers.get(&payload_type_id)?.clone();
        let entity = if let Some(entity) = this.models_by_message_type.get(&payload_type_id) {
            entity.upgrade()?
        } else {
            let extract_entity_id = *this.entity_id_extractors.get(&payload_type_id)?;
            let entity_type_id = *this.entity_types_by_message_type.get(&payload_type_id)?;
            let entity_id = (extract_entity_id)(message.as_ref());

            match this
                .entities_by_type_and_remote_id
                .get_mut(&(entity_type_id, entity_id))?
            {
                EntityMessageSubscriber::Pending(pending) => {
                    pending.push(message);
                    return None;
                }
                EntityMessageSubscriber::Entity { handle } => handle.upgrade()?,
            }
        };
        drop(this);
        Some(handler(entity, message, client, cx))
    }
}

pub enum EntityMessageSubscriber {
    Entity { handle: AnyWeakModel },
    Pending(Vec<Box<dyn AnyTypedEnvelope>>),
}

impl<T> From<Arc<T>> for AnyProtoClient
where
    T: ProtoClient + 'static,
{
    fn from(client: Arc<T>) -> Self {
        Self(client)
    }
}

impl AnyProtoClient {
    pub fn new<T: ProtoClient + 'static>(client: Arc<T>) -> Self {
        Self(client)
    }

    pub fn request<T: RequestMessage>(
        &self,
        request: T,
    ) -> impl Future<Output = anyhow::Result<T::Response>> {
        let envelope = request.into_envelope(0, None, None);
        let response = self.0.request(envelope, T::NAME);
        async move {
            T::Response::from_envelope(response.await?)
                .ok_or_else(|| anyhow!("received response of the wrong type"))
        }
    }

    pub fn send<T: EnvelopedMessage>(&self, request: T) -> anyhow::Result<()> {
        let envelope = request.into_envelope(0, None, None);
        self.0.send(envelope, T::NAME)
    }

    pub fn send_response<T: EnvelopedMessage>(
        &self,
        request_id: u32,
        request: T,
    ) -> anyhow::Result<()> {
        let envelope = request.into_envelope(0, Some(request_id), None);
        self.0.send(envelope, T::NAME)
    }

    pub fn add_request_handler<M, E, H, F>(&self, model: gpui::WeakModel<E>, handler: H)
    where
        M: RequestMessage,
        E: 'static,
        H: 'static + Sync + Fn(Model<E>, TypedEnvelope<M>, AsyncAppContext) -> F + Send + Sync,
        F: 'static + Future<Output = anyhow::Result<M::Response>>,
    {
        self.0.message_handler_set().lock().add_message_handler(
            TypeId::of::<M>(),
            model.into(),
            Arc::new(move |model, envelope, client, cx| {
                let model = model.downcast::<E>().unwrap();
                let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                let request_id = envelope.message_id();
                handler(model, *envelope, cx)
                    .then(move |result| async move {
                        match result {
                            Ok(response) => {
                                client.send_response(request_id, response)?;
                                Ok(())
                            }
                            Err(error) => {
                                client.send_response(request_id, error.to_proto())?;
                                Err(error)
                            }
                        }
                    })
                    .boxed_local()
            }),
        )
    }

    pub fn add_model_request_handler<M, E, H, F>(&self, handler: H)
    where
        M: EnvelopedMessage + RequestMessage + EntityMessage,
        E: 'static,
        H: 'static + Sync + Send + Fn(gpui::Model<E>, TypedEnvelope<M>, AsyncAppContext) -> F,
        F: 'static + Future<Output = anyhow::Result<M::Response>>,
    {
        let message_type_id = TypeId::of::<M>();
        let model_type_id = TypeId::of::<E>();
        let entity_id_extractor = |envelope: &dyn AnyTypedEnvelope| {
            envelope
                .as_any()
                .downcast_ref::<TypedEnvelope<M>>()
                .unwrap()
                .payload
                .remote_entity_id()
        };
        self.0
            .message_handler_set()
            .lock()
            .add_entity_message_handler(
                message_type_id,
                model_type_id,
                entity_id_extractor,
                Arc::new(move |model, envelope, client, cx| {
                    let model = model.downcast::<E>().unwrap();
                    let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                    let request_id = envelope.message_id();
                    handler(model, *envelope, cx)
                        .then(move |result| async move {
                            match result {
                                Ok(response) => {
                                    client.send_response(request_id, response)?;
                                    Ok(())
                                }
                                Err(error) => {
                                    client.send_response(request_id, error.to_proto())?;
                                    Err(error)
                                }
                            }
                        })
                        .boxed_local()
                }),
            );
    }

    pub fn add_model_message_handler<M, E, H, F>(&self, handler: H)
    where
        M: EnvelopedMessage + EntityMessage,
        E: 'static,
        H: 'static + Sync + Send + Fn(gpui::Model<E>, TypedEnvelope<M>, AsyncAppContext) -> F,
        F: 'static + Future<Output = anyhow::Result<()>>,
    {
        let message_type_id = TypeId::of::<M>();
        let model_type_id = TypeId::of::<E>();
        let entity_id_extractor = |envelope: &dyn AnyTypedEnvelope| {
            envelope
                .as_any()
                .downcast_ref::<TypedEnvelope<M>>()
                .unwrap()
                .payload
                .remote_entity_id()
        };
        self.0
            .message_handler_set()
            .lock()
            .add_entity_message_handler(
                message_type_id,
                model_type_id,
                entity_id_extractor,
                Arc::new(move |model, envelope, _, cx| {
                    let model = model.downcast::<E>().unwrap();
                    let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                    handler(model, *envelope, cx).boxed_local()
                }),
            );
    }
}
