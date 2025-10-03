use anyhow::{Context, Result};
use collections::HashMap;
use futures::{
    Future, FutureExt as _,
    channel::oneshot,
    future::{BoxFuture, LocalBoxFuture},
};
use gpui::{AnyEntity, AnyWeakEntity, AsyncApp, BackgroundExecutor, Entity, FutureExt as _};
use parking_lot::Mutex;
use proto::{
    AnyTypedEnvelope, EntityMessage, Envelope, EnvelopedMessage, LspRequestId, LspRequestMessage,
    RequestMessage, TypedEnvelope, error::ErrorExt as _,
};
use std::{
    any::{Any, TypeId},
    sync::{
        Arc, OnceLock,
        atomic::{self, AtomicU64},
    },
    time::Duration,
};

#[derive(Clone)]
pub struct AnyProtoClient(Arc<State>);

type RequestIds = Arc<
    Mutex<
        HashMap<
            LspRequestId,
            oneshot::Sender<
                Result<
                    Option<TypedEnvelope<Vec<proto::ProtoLspResponse<Box<dyn AnyTypedEnvelope>>>>>,
                >,
            >,
        >,
    >,
>;

static NEXT_LSP_REQUEST_ID: OnceLock<Arc<AtomicU64>> = OnceLock::new();
static REQUEST_IDS: OnceLock<RequestIds> = OnceLock::new();

struct State {
    client: Arc<dyn ProtoClient>,
    next_lsp_request_id: Arc<AtomicU64>,
    request_ids: RequestIds,
}

pub trait ProtoClient: Send + Sync {
    fn request(
        &self,
        envelope: Envelope,
        request_type: &'static str,
    ) -> BoxFuture<'static, Result<Envelope>>;

    fn send(&self, envelope: Envelope, message_type: &'static str) -> Result<()>;

    fn send_response(&self, envelope: Envelope, message_type: &'static str) -> Result<()>;

    fn message_handler_set(&self) -> &parking_lot::Mutex<ProtoMessageHandlerSet>;

    fn is_via_collab(&self) -> bool;
}

#[derive(Default)]
pub struct ProtoMessageHandlerSet {
    pub entity_types_by_message_type: HashMap<TypeId, TypeId>,
    pub entities_by_type_and_remote_id: HashMap<(TypeId, u64), EntityMessageSubscriber>,
    pub entity_id_extractors: HashMap<TypeId, fn(&dyn AnyTypedEnvelope) -> u64>,
    pub entities_by_message_type: HashMap<TypeId, AnyWeakEntity>,
    pub message_handlers: HashMap<TypeId, ProtoMessageHandler>,
}

pub type ProtoMessageHandler = Arc<
    dyn Send
        + Sync
        + Fn(
            AnyEntity,
            Box<dyn AnyTypedEnvelope>,
            AnyProtoClient,
            AsyncApp,
        ) -> LocalBoxFuture<'static, Result<()>>,
>;

impl ProtoMessageHandlerSet {
    pub fn clear(&mut self) {
        self.message_handlers.clear();
        self.entities_by_message_type.clear();
        self.entities_by_type_and_remote_id.clear();
        self.entity_id_extractors.clear();
    }

    fn add_message_handler(
        &mut self,
        message_type_id: TypeId,
        entity: gpui::AnyWeakEntity,
        handler: ProtoMessageHandler,
    ) {
        self.entities_by_message_type
            .insert(message_type_id, entity);
        let prev_handler = self.message_handlers.insert(message_type_id, handler);
        if prev_handler.is_some() {
            panic!("registered handler for the same message twice");
        }
    }

    fn add_entity_message_handler(
        &mut self,
        message_type_id: TypeId,
        entity_type_id: TypeId,
        entity_id_extractor: fn(&dyn AnyTypedEnvelope) -> u64,
        handler: ProtoMessageHandler,
    ) {
        self.entity_id_extractors
            .entry(message_type_id)
            .or_insert(entity_id_extractor);
        self.entity_types_by_message_type
            .insert(message_type_id, entity_type_id);
        let prev_handler = self.message_handlers.insert(message_type_id, handler);
        if prev_handler.is_some() {
            panic!("registered handler for the same message twice");
        }
    }

    pub fn handle_message(
        this: &parking_lot::Mutex<Self>,
        message: Box<dyn AnyTypedEnvelope>,
        client: AnyProtoClient,
        cx: AsyncApp,
    ) -> Option<LocalBoxFuture<'static, Result<()>>> {
        let payload_type_id = message.payload_type_id();
        let mut this = this.lock();
        let handler = this.message_handlers.get(&payload_type_id)?.clone();
        let entity = if let Some(entity) = this.entities_by_message_type.get(&payload_type_id) {
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
    Entity { handle: AnyWeakEntity },
    Pending(Vec<Box<dyn AnyTypedEnvelope>>),
}

impl std::fmt::Debug for EntityMessageSubscriber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityMessageSubscriber::Entity { handle } => f
                .debug_struct("EntityMessageSubscriber::Entity")
                .field("handle", handle)
                .finish(),
            EntityMessageSubscriber::Pending(vec) => f
                .debug_struct("EntityMessageSubscriber::Pending")
                .field(
                    "envelopes",
                    &vec.iter()
                        .map(|envelope| envelope.payload_type_name())
                        .collect::<Vec<_>>(),
                )
                .finish(),
        }
    }
}

impl<T> From<Arc<T>> for AnyProtoClient
where
    T: ProtoClient + 'static,
{
    fn from(client: Arc<T>) -> Self {
        Self::new(client)
    }
}

impl AnyProtoClient {
    pub fn new<T: ProtoClient + 'static>(client: Arc<T>) -> Self {
        Self(Arc::new(State {
            client,
            next_lsp_request_id: NEXT_LSP_REQUEST_ID
                .get_or_init(|| Arc::new(AtomicU64::new(0)))
                .clone(),
            request_ids: REQUEST_IDS.get_or_init(RequestIds::default).clone(),
        }))
    }

    pub fn is_via_collab(&self) -> bool {
        self.0.client.is_via_collab()
    }

    pub fn request<T: RequestMessage>(
        &self,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> + use<T> {
        let envelope = request.into_envelope(0, None, None);
        let response = self.0.client.request(envelope, T::NAME);
        async move {
            T::Response::from_envelope(response.await?)
                .context("received response of the wrong type")
        }
    }

    pub fn send<T: EnvelopedMessage>(&self, request: T) -> Result<()> {
        let envelope = request.into_envelope(0, None, None);
        self.0.client.send(envelope, T::NAME)
    }

    pub fn send_response<T: EnvelopedMessage>(&self, request_id: u32, request: T) -> Result<()> {
        let envelope = request.into_envelope(0, Some(request_id), None);
        self.0.client.send(envelope, T::NAME)
    }

    pub fn request_lsp<T>(
        &self,
        project_id: u64,
        server_id: Option<u64>,
        timeout: Duration,
        executor: BackgroundExecutor,
        request: T,
    ) -> impl Future<
        Output = Result<Option<TypedEnvelope<Vec<proto::ProtoLspResponse<T::Response>>>>>,
    > + use<T>
    where
        T: LspRequestMessage,
    {
        let new_id = LspRequestId(
            self.0
                .next_lsp_request_id
                .fetch_add(1, atomic::Ordering::Acquire),
        );
        let (tx, rx) = oneshot::channel();
        {
            self.0.request_ids.lock().insert(new_id, tx);
        }

        let query = proto::LspQuery {
            project_id,
            server_id,
            lsp_request_id: new_id.0,
            request: Some(request.to_proto_query()),
        };
        let request = self.request(query);
        let request_ids = self.0.request_ids.clone();
        async move {
            match request.await {
                Ok(_request_enqueued) => {}
                Err(e) => {
                    request_ids.lock().remove(&new_id);
                    return Err(e).context("sending LSP proto request");
                }
            }

            let response = rx.with_timeout(timeout, &executor).await;
            {
                request_ids.lock().remove(&new_id);
            }
            match response {
                Ok(Ok(response)) => {
                    let response = response
                        .context("waiting for LSP proto response")?
                        .map(|response| {
                            anyhow::Ok(TypedEnvelope {
                                payload: response
                                    .payload
                                    .into_iter()
                                    .map(|lsp_response| lsp_response.into_response::<T>())
                                    .collect::<Result<Vec<_>>>()?,
                                sender_id: response.sender_id,
                                original_sender_id: response.original_sender_id,
                                message_id: response.message_id,
                                received_at: response.received_at,
                            })
                        })
                        .transpose()
                        .context("converting LSP proto response")?;
                    Ok(response)
                }
                Err(_cancelled_due_timeout) => Ok(None),
                Ok(Err(_channel_dropped)) => Ok(None),
            }
        }
    }

    pub fn send_lsp_response<T: LspRequestMessage>(
        &self,
        project_id: u64,
        lsp_request_id: LspRequestId,
        server_responses: HashMap<u64, T::Response>,
    ) -> Result<()> {
        self.send(proto::LspQueryResponse {
            project_id,
            lsp_request_id: lsp_request_id.0,
            responses: server_responses
                .into_iter()
                .map(|(server_id, response)| proto::LspResponse {
                    server_id,
                    response: Some(T::response_to_proto_query(response)),
                })
                .collect(),
        })
    }

    pub fn handle_lsp_response(&self, mut envelope: TypedEnvelope<proto::LspQueryResponse>) {
        let request_id = LspRequestId(envelope.payload.lsp_request_id);
        let mut response_senders = self.0.request_ids.lock();
        if let Some(tx) = response_senders.remove(&request_id) {
            let responses = envelope.payload.responses.drain(..).collect::<Vec<_>>();
            tx.send(Ok(Some(proto::TypedEnvelope {
                sender_id: envelope.sender_id,
                original_sender_id: envelope.original_sender_id,
                message_id: envelope.message_id,
                received_at: envelope.received_at,
                payload: responses
                    .into_iter()
                    .filter_map(|response| {
                        use proto::lsp_response::Response;

                        let server_id = response.server_id;
                        let response = match response.response? {
                            Response::GetReferencesResponse(response) => {
                                to_any_envelope(&envelope, response)
                            }
                            Response::GetDocumentColorResponse(response) => {
                                to_any_envelope(&envelope, response)
                            }
                            Response::GetHoverResponse(response) => {
                                to_any_envelope(&envelope, response)
                            }
                            Response::GetCodeActionsResponse(response) => {
                                to_any_envelope(&envelope, response)
                            }
                            Response::GetSignatureHelpResponse(response) => {
                                to_any_envelope(&envelope, response)
                            }
                            Response::GetCodeLensResponse(response) => {
                                to_any_envelope(&envelope, response)
                            }
                            Response::GetDocumentDiagnosticsResponse(response) => {
                                to_any_envelope(&envelope, response)
                            }
                            Response::GetDefinitionResponse(response) => {
                                to_any_envelope(&envelope, response)
                            }
                            Response::GetDeclarationResponse(response) => {
                                to_any_envelope(&envelope, response)
                            }
                            Response::GetTypeDefinitionResponse(response) => {
                                to_any_envelope(&envelope, response)
                            }
                            Response::GetImplementationResponse(response) => {
                                to_any_envelope(&envelope, response)
                            }
                            Response::InlayHintsResponse(response) => {
                                to_any_envelope(&envelope, response)
                            }
                        };
                        Some(proto::ProtoLspResponse {
                            server_id,
                            response,
                        })
                    })
                    .collect(),
            })))
            .ok();
        }
    }

    pub fn add_request_handler<M, E, H, F>(&self, entity: gpui::WeakEntity<E>, handler: H)
    where
        M: RequestMessage,
        E: 'static,
        H: 'static + Sync + Fn(Entity<E>, TypedEnvelope<M>, AsyncApp) -> F + Send + Sync,
        F: 'static + Future<Output = Result<M::Response>>,
    {
        self.0
            .client
            .message_handler_set()
            .lock()
            .add_message_handler(
                TypeId::of::<M>(),
                entity.into(),
                Arc::new(move |entity, envelope, client, cx| {
                    let entity = entity.downcast::<E>().unwrap();
                    let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                    let request_id = envelope.message_id();
                    handler(entity, *envelope, cx)
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

    pub fn add_entity_request_handler<M, E, H, F>(&self, handler: H)
    where
        M: EnvelopedMessage + RequestMessage + EntityMessage,
        E: 'static,
        H: 'static + Sync + Send + Fn(gpui::Entity<E>, TypedEnvelope<M>, AsyncApp) -> F,
        F: 'static + Future<Output = Result<M::Response>>,
    {
        let message_type_id = TypeId::of::<M>();
        let entity_type_id = TypeId::of::<E>();
        let entity_id_extractor = |envelope: &dyn AnyTypedEnvelope| {
            (envelope as &dyn Any)
                .downcast_ref::<TypedEnvelope<M>>()
                .unwrap()
                .payload
                .remote_entity_id()
        };
        self.0
            .client
            .message_handler_set()
            .lock()
            .add_entity_message_handler(
                message_type_id,
                entity_type_id,
                entity_id_extractor,
                Arc::new(move |entity, envelope, client, cx| {
                    let entity = entity.downcast::<E>().unwrap();
                    let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                    let request_id = envelope.message_id();
                    handler(entity, *envelope, cx)
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

    pub fn add_entity_message_handler<M, E, H, F>(&self, handler: H)
    where
        M: EnvelopedMessage + EntityMessage,
        E: 'static,
        H: 'static + Sync + Send + Fn(gpui::Entity<E>, TypedEnvelope<M>, AsyncApp) -> F,
        F: 'static + Future<Output = Result<()>>,
    {
        let message_type_id = TypeId::of::<M>();
        let entity_type_id = TypeId::of::<E>();
        let entity_id_extractor = |envelope: &dyn AnyTypedEnvelope| {
            (envelope as &dyn Any)
                .downcast_ref::<TypedEnvelope<M>>()
                .unwrap()
                .payload
                .remote_entity_id()
        };
        self.0
            .client
            .message_handler_set()
            .lock()
            .add_entity_message_handler(
                message_type_id,
                entity_type_id,
                entity_id_extractor,
                Arc::new(move |entity, envelope, _, cx| {
                    let entity = entity.downcast::<E>().unwrap();
                    let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                    handler(entity, *envelope, cx).boxed_local()
                }),
            );
    }

    pub fn subscribe_to_entity<E: 'static>(&self, remote_id: u64, entity: &Entity<E>) {
        let id = (TypeId::of::<E>(), remote_id);

        let mut message_handlers = self.0.client.message_handler_set().lock();
        if message_handlers
            .entities_by_type_and_remote_id
            .contains_key(&id)
        {
            panic!("already subscribed to entity");
        }

        message_handlers.entities_by_type_and_remote_id.insert(
            id,
            EntityMessageSubscriber::Entity {
                handle: entity.downgrade().into(),
            },
        );
    }
}

fn to_any_envelope<T: EnvelopedMessage>(
    envelope: &TypedEnvelope<proto::LspQueryResponse>,
    response: T,
) -> Box<dyn AnyTypedEnvelope> {
    Box::new(proto::TypedEnvelope {
        sender_id: envelope.sender_id,
        original_sender_id: envelope.original_sender_id,
        message_id: envelope.message_id,
        received_at: envelope.received_at,
        payload: response,
    }) as Box<_>
}
