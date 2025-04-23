mod connection_pool;

use crate::api::{CloudflareIpCountryHeader, SystemIdHeader};
use crate::llm::LlmTokenClaims;
use crate::{
    AppState, Config, Error, RateLimit, Result, auth,
    db::{
        self, BufferId, Capability, Channel, ChannelId, ChannelRole, ChannelsForUser,
        CreatedChannelMessage, Database, InviteMemberResult, MembershipUpdated, MessageId,
        NotificationId, Project, ProjectId, RejoinedProject, RemoveChannelMemberResult, ReplicaId,
        RespondToChannelInvite, RoomId, ServerId, UpdatedChannelMessage, User, UserId,
    },
    executor::Executor,
};
use anyhow::{Context as _, anyhow, bail};
use async_tungstenite::tungstenite::{
    Message as TungsteniteMessage, protocol::CloseFrame as TungsteniteCloseFrame,
};
use axum::{
    Extension, Router, TypedHeader,
    body::Body,
    extract::{
        ConnectInfo, WebSocketUpgrade,
        ws::{CloseFrame as AxumCloseFrame, Message as AxumMessage},
    },
    headers::{Header, HeaderName},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::get,
};
use chrono::Utc;
use collections::{HashMap, HashSet};
pub use connection_pool::{ConnectionPool, ZedVersion};
use core::fmt::{self, Debug, Formatter};
use http_client::HttpClient;
use open_ai::{OPEN_AI_API_URL, OpenAiEmbeddingModel};
use reqwest_client::ReqwestClient;
use rpc::proto::split_repository_update;
use sha2::Digest;
use supermaven_api::{CreateExternalUserRequest, SupermavenAdminApi};

use futures::{
    FutureExt, SinkExt, StreamExt, TryStreamExt, channel::oneshot, future::BoxFuture,
    stream::FuturesUnordered,
};
use prometheus::{IntGauge, register_int_gauge};
use rpc::{
    Connection, ConnectionId, ErrorCode, ErrorCodeExt, ErrorExt, Peer, Receipt, TypedEnvelope,
    proto::{
        self, Ack, AnyTypedEnvelope, EntityMessage, EnvelopedMessage, LiveKitConnectionInfo,
        RequestMessage, ShareProject, UpdateChannelBufferCollaborators,
    },
};
use semantic_version::SemanticVersion;
use serde::{Serialize, Serializer};
use std::{
    any::TypeId,
    future::Future,
    marker::PhantomData,
    mem,
    net::SocketAddr,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, Ordering::SeqCst},
    },
    time::{Duration, Instant},
};
use time::OffsetDateTime;
use tokio::sync::{MutexGuard, Semaphore, watch};
use tower::ServiceBuilder;
use tracing::{
    Instrument,
    field::{self},
    info_span, instrument,
};

pub const RECONNECT_TIMEOUT: Duration = Duration::from_secs(30);

// kubernetes gives terminated pods 10s to shutdown gracefully. After they're gone, we can clean up old resources.
pub const CLEANUP_TIMEOUT: Duration = Duration::from_secs(15);

const MESSAGE_COUNT_PER_PAGE: usize = 100;
const MAX_MESSAGE_LEN: usize = 1024;
const NOTIFICATION_COUNT_PER_PAGE: usize = 50;

type MessageHandler =
    Box<dyn Send + Sync + Fn(Box<dyn AnyTypedEnvelope>, Session) -> BoxFuture<'static, ()>>;

struct Response<R> {
    peer: Arc<Peer>,
    receipt: Receipt<R>,
    responded: Arc<AtomicBool>,
}

impl<R: RequestMessage> Response<R> {
    fn send(self, payload: R::Response) -> Result<()> {
        self.responded.store(true, SeqCst);
        self.peer.respond(self.receipt, payload)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Principal {
    User(User),
    Impersonated { user: User, admin: User },
}

impl Principal {
    fn update_span(&self, span: &tracing::Span) {
        match &self {
            Principal::User(user) => {
                span.record("user_id", user.id.0);
                span.record("login", &user.github_login);
            }
            Principal::Impersonated { user, admin } => {
                span.record("user_id", user.id.0);
                span.record("login", &user.github_login);
                span.record("impersonator", &admin.github_login);
            }
        }
    }
}

#[derive(Clone)]
struct Session {
    principal: Principal,
    connection_id: ConnectionId,
    db: Arc<tokio::sync::Mutex<DbHandle>>,
    peer: Arc<Peer>,
    connection_pool: Arc<parking_lot::Mutex<ConnectionPool>>,
    app_state: Arc<AppState>,
    supermaven_client: Option<Arc<SupermavenAdminApi>>,
    http_client: Arc<dyn HttpClient>,
    /// The GeoIP country code for the user.
    #[allow(unused)]
    geoip_country_code: Option<String>,
    system_id: Option<String>,
    _executor: Executor,
}

impl Session {
    async fn db(&self) -> tokio::sync::MutexGuard<DbHandle> {
        #[cfg(test)]
        tokio::task::yield_now().await;
        let guard = self.db.lock().await;
        #[cfg(test)]
        tokio::task::yield_now().await;
        guard
    }

    async fn connection_pool(&self) -> ConnectionPoolGuard<'_> {
        #[cfg(test)]
        tokio::task::yield_now().await;
        let guard = self.connection_pool.lock();
        ConnectionPoolGuard {
            guard,
            _not_send: PhantomData,
        }
    }

    fn is_staff(&self) -> bool {
        match &self.principal {
            Principal::User(user) => user.admin,
            Principal::Impersonated { .. } => true,
        }
    }

    pub async fn has_llm_subscription(
        &self,
        db: &MutexGuard<'_, DbHandle>,
    ) -> anyhow::Result<bool> {
        if self.is_staff() {
            return Ok(true);
        }

        let user_id = self.user_id();

        Ok(db.has_active_billing_subscription(user_id).await?)
    }

    pub async fn current_plan(
        &self,
        _db: &MutexGuard<'_, DbHandle>,
    ) -> anyhow::Result<proto::Plan> {
        if self.is_staff() {
            Ok(proto::Plan::ZedPro)
        } else {
            Ok(proto::Plan::Free)
        }
    }

    fn user_id(&self) -> UserId {
        match &self.principal {
            Principal::User(user) => user.id,
            Principal::Impersonated { user, .. } => user.id,
        }
    }

    pub fn email(&self) -> Option<String> {
        match &self.principal {
            Principal::User(user) => user.email_address.clone(),
            Principal::Impersonated { user, .. } => user.email_address.clone(),
        }
    }
}

impl Debug for Session {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = f.debug_struct("Session");
        match &self.principal {
            Principal::User(user) => {
                result.field("user", &user.github_login);
            }
            Principal::Impersonated { user, admin } => {
                result.field("user", &user.github_login);
                result.field("impersonator", &admin.github_login);
            }
        }
        result.field("connection_id", &self.connection_id).finish()
    }
}

struct DbHandle(Arc<Database>);

impl Deref for DbHandle {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

pub struct Server {
    id: parking_lot::Mutex<ServerId>,
    peer: Arc<Peer>,
    pub(crate) connection_pool: Arc<parking_lot::Mutex<ConnectionPool>>,
    app_state: Arc<AppState>,
    handlers: HashMap<TypeId, MessageHandler>,
    teardown: watch::Sender<bool>,
}

pub(crate) struct ConnectionPoolGuard<'a> {
    guard: parking_lot::MutexGuard<'a, ConnectionPool>,
    _not_send: PhantomData<Rc<()>>,
}

#[derive(Serialize)]
pub struct ServerSnapshot<'a> {
    peer: &'a Peer,
    #[serde(serialize_with = "serialize_deref")]
    connection_pool: ConnectionPoolGuard<'a>,
}

pub fn serialize_deref<S, T, U>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Deref<Target = U>,
    U: Serialize,
{
    Serialize::serialize(value.deref(), serializer)
}

impl Server {
    pub fn new(id: ServerId, app_state: Arc<AppState>) -> Arc<Self> {
        let mut server = Self {
            id: parking_lot::Mutex::new(id),
            peer: Peer::new(id.0 as u32),
            app_state: app_state.clone(),
            connection_pool: Default::default(),
            handlers: Default::default(),
            teardown: watch::channel(false).0,
        };

        server
            .add_request_handler(ping)
            .add_request_handler(create_room)
            .add_request_handler(join_room)
            .add_request_handler(rejoin_room)
            .add_request_handler(leave_room)
            .add_request_handler(set_room_participant_role)
            .add_request_handler(call)
            .add_request_handler(cancel_call)
            .add_message_handler(decline_call)
            .add_request_handler(update_participant_location)
            .add_request_handler(share_project)
            .add_message_handler(unshare_project)
            .add_request_handler(join_project)
            .add_message_handler(leave_project)
            .add_request_handler(update_project)
            .add_request_handler(update_worktree)
            .add_request_handler(update_repository)
            .add_request_handler(remove_repository)
            .add_message_handler(start_language_server)
            .add_message_handler(update_language_server)
            .add_message_handler(update_diagnostic_summary)
            .add_message_handler(update_worktree_settings)
            .add_request_handler(forward_read_only_project_request::<proto::GetHover>)
            .add_request_handler(forward_read_only_project_request::<proto::GetDefinition>)
            .add_request_handler(forward_read_only_project_request::<proto::GetTypeDefinition>)
            .add_request_handler(forward_read_only_project_request::<proto::GetReferences>)
            .add_request_handler(forward_find_search_candidates_request)
            .add_request_handler(forward_read_only_project_request::<proto::GetDocumentHighlights>)
            .add_request_handler(forward_read_only_project_request::<proto::GetDocumentSymbols>)
            .add_request_handler(forward_read_only_project_request::<proto::GetProjectSymbols>)
            .add_request_handler(forward_read_only_project_request::<proto::OpenBufferForSymbol>)
            .add_request_handler(forward_read_only_project_request::<proto::OpenBufferById>)
            .add_request_handler(forward_read_only_project_request::<proto::SynchronizeBuffers>)
            .add_request_handler(forward_read_only_project_request::<proto::InlayHints>)
            .add_request_handler(forward_read_only_project_request::<proto::ResolveInlayHint>)
            .add_request_handler(forward_mutating_project_request::<proto::GetCodeLens>)
            .add_request_handler(forward_read_only_project_request::<proto::OpenBufferByPath>)
            .add_request_handler(forward_read_only_project_request::<proto::GitGetBranches>)
            .add_request_handler(forward_read_only_project_request::<proto::OpenUnstagedDiff>)
            .add_request_handler(forward_read_only_project_request::<proto::OpenUncommittedDiff>)
            .add_request_handler(forward_read_only_project_request::<proto::LspExtExpandMacro>)
            .add_request_handler(forward_read_only_project_request::<proto::LspExtOpenDocs>)
            .add_request_handler(
                forward_read_only_project_request::<proto::LspExtSwitchSourceHeader>,
            )
            .add_request_handler(
                forward_read_only_project_request::<proto::LanguageServerIdForName>,
            )
            .add_request_handler(
                forward_mutating_project_request::<proto::RegisterBufferWithLanguageServers>,
            )
            .add_request_handler(forward_mutating_project_request::<proto::UpdateGitBranch>)
            .add_request_handler(forward_mutating_project_request::<proto::GetCompletions>)
            .add_request_handler(
                forward_mutating_project_request::<proto::ApplyCompletionAdditionalEdits>,
            )
            .add_request_handler(forward_mutating_project_request::<proto::OpenNewBuffer>)
            .add_request_handler(
                forward_mutating_project_request::<proto::ResolveCompletionDocumentation>,
            )
            .add_request_handler(forward_mutating_project_request::<proto::GetCodeActions>)
            .add_request_handler(forward_mutating_project_request::<proto::ApplyCodeAction>)
            .add_request_handler(forward_mutating_project_request::<proto::PrepareRename>)
            .add_request_handler(forward_mutating_project_request::<proto::PerformRename>)
            .add_request_handler(forward_mutating_project_request::<proto::ReloadBuffers>)
            .add_request_handler(forward_mutating_project_request::<proto::ApplyCodeActionKind>)
            .add_request_handler(forward_mutating_project_request::<proto::FormatBuffers>)
            .add_request_handler(forward_mutating_project_request::<proto::CreateProjectEntry>)
            .add_request_handler(forward_mutating_project_request::<proto::RenameProjectEntry>)
            .add_request_handler(forward_mutating_project_request::<proto::CopyProjectEntry>)
            .add_request_handler(forward_mutating_project_request::<proto::DeleteProjectEntry>)
            .add_request_handler(forward_mutating_project_request::<proto::ExpandProjectEntry>)
            .add_request_handler(
                forward_mutating_project_request::<proto::ExpandAllForProjectEntry>,
            )
            .add_request_handler(forward_mutating_project_request::<proto::OnTypeFormatting>)
            .add_request_handler(forward_mutating_project_request::<proto::SaveBuffer>)
            .add_request_handler(forward_mutating_project_request::<proto::BlameBuffer>)
            .add_request_handler(forward_mutating_project_request::<proto::MultiLspQuery>)
            .add_request_handler(forward_mutating_project_request::<proto::RestartLanguageServers>)
            .add_request_handler(forward_mutating_project_request::<proto::StopLanguageServers>)
            .add_request_handler(forward_mutating_project_request::<proto::LinkedEditingRange>)
            .add_message_handler(create_buffer_for_peer)
            .add_request_handler(update_buffer)
            .add_message_handler(broadcast_project_message_from_host::<proto::RefreshInlayHints>)
            .add_message_handler(broadcast_project_message_from_host::<proto::RefreshCodeLens>)
            .add_message_handler(broadcast_project_message_from_host::<proto::UpdateBufferFile>)
            .add_message_handler(broadcast_project_message_from_host::<proto::BufferReloaded>)
            .add_message_handler(broadcast_project_message_from_host::<proto::BufferSaved>)
            .add_message_handler(broadcast_project_message_from_host::<proto::UpdateDiffBases>)
            .add_request_handler(get_users)
            .add_request_handler(fuzzy_search_users)
            .add_request_handler(request_contact)
            .add_request_handler(remove_contact)
            .add_request_handler(respond_to_contact_request)
            .add_message_handler(subscribe_to_channels)
            .add_request_handler(create_channel)
            .add_request_handler(delete_channel)
            .add_request_handler(invite_channel_member)
            .add_request_handler(remove_channel_member)
            .add_request_handler(set_channel_member_role)
            .add_request_handler(set_channel_visibility)
            .add_request_handler(rename_channel)
            .add_request_handler(join_channel_buffer)
            .add_request_handler(leave_channel_buffer)
            .add_message_handler(update_channel_buffer)
            .add_request_handler(rejoin_channel_buffers)
            .add_request_handler(get_channel_members)
            .add_request_handler(respond_to_channel_invite)
            .add_request_handler(join_channel)
            .add_request_handler(join_channel_chat)
            .add_message_handler(leave_channel_chat)
            .add_request_handler(send_channel_message)
            .add_request_handler(remove_channel_message)
            .add_request_handler(update_channel_message)
            .add_request_handler(get_channel_messages)
            .add_request_handler(get_channel_messages_by_id)
            .add_request_handler(get_notifications)
            .add_request_handler(mark_notification_as_read)
            .add_request_handler(move_channel)
            .add_request_handler(follow)
            .add_message_handler(unfollow)
            .add_message_handler(update_followers)
            .add_request_handler(get_private_user_info)
            .add_request_handler(get_llm_api_token)
            .add_request_handler(accept_terms_of_service)
            .add_message_handler(acknowledge_channel_message)
            .add_message_handler(acknowledge_buffer_version)
            .add_request_handler(get_supermaven_api_key)
            .add_request_handler(forward_mutating_project_request::<proto::OpenContext>)
            .add_request_handler(forward_mutating_project_request::<proto::CreateContext>)
            .add_request_handler(forward_mutating_project_request::<proto::SynchronizeContexts>)
            .add_request_handler(forward_mutating_project_request::<proto::Stage>)
            .add_request_handler(forward_mutating_project_request::<proto::Unstage>)
            .add_request_handler(forward_mutating_project_request::<proto::Commit>)
            .add_request_handler(forward_mutating_project_request::<proto::GitInit>)
            .add_request_handler(forward_read_only_project_request::<proto::GetRemotes>)
            .add_request_handler(forward_read_only_project_request::<proto::GitShow>)
            .add_request_handler(forward_read_only_project_request::<proto::LoadCommitDiff>)
            .add_request_handler(forward_read_only_project_request::<proto::GitReset>)
            .add_request_handler(forward_read_only_project_request::<proto::GitCheckoutFiles>)
            .add_request_handler(forward_mutating_project_request::<proto::SetIndexText>)
            .add_request_handler(forward_mutating_project_request::<proto::ToggleBreakpoint>)
            .add_message_handler(broadcast_project_message_from_host::<proto::BreakpointsForFile>)
            .add_request_handler(forward_mutating_project_request::<proto::OpenCommitMessageBuffer>)
            .add_request_handler(forward_mutating_project_request::<proto::GitDiff>)
            .add_request_handler(forward_mutating_project_request::<proto::GitCreateBranch>)
            .add_request_handler(forward_mutating_project_request::<proto::GitChangeBranch>)
            .add_request_handler(forward_mutating_project_request::<proto::CheckForPushedCommits>)
            .add_message_handler(broadcast_project_message_from_host::<proto::AdvertiseContexts>)
            .add_message_handler(update_context)
            .add_request_handler({
                let app_state = app_state.clone();
                move |request, response, session| {
                    let app_state = app_state.clone();
                    async move {
                        count_language_model_tokens(request, response, session, &app_state.config)
                            .await
                    }
                }
            })
            .add_request_handler(get_cached_embeddings)
            .add_request_handler({
                let app_state = app_state.clone();
                move |request, response, session| {
                    compute_embeddings(
                        request,
                        response,
                        session,
                        app_state.config.openai_api_key.clone(),
                    )
                }
            });

        Arc::new(server)
    }

    pub async fn start(&self) -> Result<()> {
        let server_id = *self.id.lock();
        let app_state = self.app_state.clone();
        let peer = self.peer.clone();
        let timeout = self.app_state.executor.sleep(CLEANUP_TIMEOUT);
        let pool = self.connection_pool.clone();
        let livekit_client = self.app_state.livekit_client.clone();

        let span = info_span!("start server");
        self.app_state.executor.spawn_detached(
            async move {
                tracing::info!("waiting for cleanup timeout");
                timeout.await;
                tracing::info!("cleanup timeout expired, retrieving stale rooms");
                if let Some((room_ids, channel_ids)) = app_state
                    .db
                    .stale_server_resource_ids(&app_state.config.zed_environment, server_id)
                    .await
                    .trace_err()
                {
                    tracing::info!(stale_room_count = room_ids.len(), "retrieved stale rooms");
                    tracing::info!(
                        stale_channel_buffer_count = channel_ids.len(),
                        "retrieved stale channel buffers"
                    );

                    for channel_id in channel_ids {
                        if let Some(refreshed_channel_buffer) = app_state
                            .db
                            .clear_stale_channel_buffer_collaborators(channel_id, server_id)
                            .await
                            .trace_err()
                        {
                            for connection_id in refreshed_channel_buffer.connection_ids {
                                peer.send(
                                    connection_id,
                                    proto::UpdateChannelBufferCollaborators {
                                        channel_id: channel_id.to_proto(),
                                        collaborators: refreshed_channel_buffer
                                            .collaborators
                                            .clone(),
                                    },
                                )
                                .trace_err();
                            }
                        }
                    }

                    for room_id in room_ids {
                        let mut contacts_to_update = HashSet::default();
                        let mut canceled_calls_to_user_ids = Vec::new();
                        let mut livekit_room = String::new();
                        let mut delete_livekit_room = false;

                        if let Some(mut refreshed_room) = app_state
                            .db
                            .clear_stale_room_participants(room_id, server_id)
                            .await
                            .trace_err()
                        {
                            tracing::info!(
                                room_id = room_id.0,
                                new_participant_count = refreshed_room.room.participants.len(),
                                "refreshed room"
                            );
                            room_updated(&refreshed_room.room, &peer);
                            if let Some(channel) = refreshed_room.channel.as_ref() {
                                channel_updated(channel, &refreshed_room.room, &peer, &pool.lock());
                            }
                            contacts_to_update
                                .extend(refreshed_room.stale_participant_user_ids.iter().copied());
                            contacts_to_update
                                .extend(refreshed_room.canceled_calls_to_user_ids.iter().copied());
                            canceled_calls_to_user_ids =
                                mem::take(&mut refreshed_room.canceled_calls_to_user_ids);
                            livekit_room = mem::take(&mut refreshed_room.room.livekit_room);
                            delete_livekit_room = refreshed_room.room.participants.is_empty();
                        }

                        {
                            let pool = pool.lock();
                            for canceled_user_id in canceled_calls_to_user_ids {
                                for connection_id in pool.user_connection_ids(canceled_user_id) {
                                    peer.send(
                                        connection_id,
                                        proto::CallCanceled {
                                            room_id: room_id.to_proto(),
                                        },
                                    )
                                    .trace_err();
                                }
                            }
                        }

                        for user_id in contacts_to_update {
                            let busy = app_state.db.is_user_busy(user_id).await.trace_err();
                            let contacts = app_state.db.get_contacts(user_id).await.trace_err();
                            if let Some((busy, contacts)) = busy.zip(contacts) {
                                let pool = pool.lock();
                                let updated_contact = contact_for_user(user_id, busy, &pool);
                                for contact in contacts {
                                    if let db::Contact::Accepted {
                                        user_id: contact_user_id,
                                        ..
                                    } = contact
                                    {
                                        for contact_conn_id in
                                            pool.user_connection_ids(contact_user_id)
                                        {
                                            peer.send(
                                                contact_conn_id,
                                                proto::UpdateContacts {
                                                    contacts: vec![updated_contact.clone()],
                                                    remove_contacts: Default::default(),
                                                    incoming_requests: Default::default(),
                                                    remove_incoming_requests: Default::default(),
                                                    outgoing_requests: Default::default(),
                                                    remove_outgoing_requests: Default::default(),
                                                },
                                            )
                                            .trace_err();
                                        }
                                    }
                                }
                            }
                        }

                        if let Some(live_kit) = livekit_client.as_ref() {
                            if delete_livekit_room {
                                live_kit.delete_room(livekit_room).await.trace_err();
                            }
                        }
                    }
                }

                app_state
                    .db
                    .delete_stale_servers(&app_state.config.zed_environment, server_id)
                    .await
                    .trace_err();
            }
            .instrument(span),
        );
        Ok(())
    }

    pub fn teardown(&self) {
        self.peer.teardown();
        self.connection_pool.lock().reset();
        let _ = self.teardown.send(true);
    }

    #[cfg(test)]
    pub fn reset(&self, id: ServerId) {
        self.teardown();
        *self.id.lock() = id;
        self.peer.reset(id.0 as u32);
        let _ = self.teardown.send(false);
    }

    #[cfg(test)]
    pub fn id(&self) -> ServerId {
        *self.id.lock()
    }

    fn add_handler<F, Fut, M>(&mut self, handler: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(TypedEnvelope<M>, Session) -> Fut,
        Fut: 'static + Send + Future<Output = Result<()>>,
        M: EnvelopedMessage,
    {
        let prev_handler = self.handlers.insert(
            TypeId::of::<M>(),
            Box::new(move |envelope, session| {
                let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                let received_at = envelope.received_at;
                tracing::info!("message received");
                let start_time = Instant::now();
                let future = (handler)(*envelope, session);
                async move {
                    let result = future.await;
                    let total_duration_ms = received_at.elapsed().as_micros() as f64 / 1000.0;
                    let processing_duration_ms = start_time.elapsed().as_micros() as f64 / 1000.0;
                    let queue_duration_ms = total_duration_ms - processing_duration_ms;
                    let payload_type = M::NAME;

                    match result {
                        Err(error) => {
                            tracing::error!(
                                ?error,
                                total_duration_ms,
                                processing_duration_ms,
                                queue_duration_ms,
                                payload_type,
                                "error handling message"
                            )
                        }
                        Ok(()) => tracing::info!(
                            total_duration_ms,
                            processing_duration_ms,
                            queue_duration_ms,
                            "finished handling message"
                        ),
                    }
                }
                .boxed()
            }),
        );
        if prev_handler.is_some() {
            panic!("registered a handler for the same message twice");
        }
        self
    }

    fn add_message_handler<F, Fut, M>(&mut self, handler: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(M, Session) -> Fut,
        Fut: 'static + Send + Future<Output = Result<()>>,
        M: EnvelopedMessage,
    {
        self.add_handler(move |envelope, session| handler(envelope.payload, session));
        self
    }

    fn add_request_handler<F, Fut, M>(&mut self, handler: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(M, Response<M>, Session) -> Fut,
        Fut: Send + Future<Output = Result<()>>,
        M: RequestMessage,
    {
        let handler = Arc::new(handler);
        self.add_handler(move |envelope, session| {
            let receipt = envelope.receipt();
            let handler = handler.clone();
            async move {
                let peer = session.peer.clone();
                let responded = Arc::new(AtomicBool::default());
                let response = Response {
                    peer: peer.clone(),
                    responded: responded.clone(),
                    receipt,
                };
                match (handler)(envelope.payload, response, session).await {
                    Ok(()) => {
                        if responded.load(std::sync::atomic::Ordering::SeqCst) {
                            Ok(())
                        } else {
                            Err(anyhow!("handler did not send a response"))?
                        }
                    }
                    Err(error) => {
                        let proto_err = match &error {
                            Error::Internal(err) => err.to_proto(),
                            _ => ErrorCode::Internal.message(format!("{}", error)).to_proto(),
                        };
                        peer.respond_with_error(receipt, proto_err)?;
                        Err(error)
                    }
                }
            }
        })
    }

    pub fn handle_connection(
        self: &Arc<Self>,
        connection: Connection,
        address: String,
        principal: Principal,
        zed_version: ZedVersion,
        geoip_country_code: Option<String>,
        system_id: Option<String>,
        send_connection_id: Option<oneshot::Sender<ConnectionId>>,
        executor: Executor,
    ) -> impl Future<Output = ()> + use<> {
        let this = self.clone();
        let span = info_span!("handle connection", %address,
            connection_id=field::Empty,
            user_id=field::Empty,
            login=field::Empty,
            impersonator=field::Empty,
            geoip_country_code=field::Empty
        );
        principal.update_span(&span);
        if let Some(country_code) = geoip_country_code.as_ref() {
            span.record("geoip_country_code", country_code);
        }

        let mut teardown = self.teardown.subscribe();
        async move {
            if *teardown.borrow() {
                tracing::error!("server is tearing down");
                return
            }
            let (connection_id, handle_io, mut incoming_rx) = this
                .peer
                .add_connection(connection, {
                    let executor = executor.clone();
                    move |duration| executor.sleep(duration)
                });
            tracing::Span::current().record("connection_id", format!("{}", connection_id));

            tracing::info!("connection opened");

            let user_agent = format!("Zed Server/{}", env!("CARGO_PKG_VERSION"));
            let http_client = match ReqwestClient::user_agent(&user_agent) {
                Ok(http_client) => Arc::new(http_client),
                Err(error) => {
                    tracing::error!(?error, "failed to create HTTP client");
                    return;
                }
            };

            let supermaven_client = this.app_state.config.supermaven_admin_api_key.clone().map(|supermaven_admin_api_key| Arc::new(SupermavenAdminApi::new(
                    supermaven_admin_api_key.to_string(),
                    http_client.clone(),
                )));

            let session = Session {
                principal: principal.clone(),
                connection_id,
                db: Arc::new(tokio::sync::Mutex::new(DbHandle(this.app_state.db.clone()))),
                peer: this.peer.clone(),
                connection_pool: this.connection_pool.clone(),
                app_state: this.app_state.clone(),
                http_client,
                geoip_country_code,
                system_id,
                _executor: executor.clone(),
                supermaven_client,
            };

            if let Err(error) = this.send_initial_client_update(connection_id, &principal, zed_version, send_connection_id, &session).await {
                tracing::error!(?error, "failed to send initial client update");
                return;
            }

            let handle_io = handle_io.fuse();
            futures::pin_mut!(handle_io);

            // Handlers for foreground messages are pushed into the following `FuturesUnordered`.
            // This prevents deadlocks when e.g., client A performs a request to client B and
            // client B performs a request to client A. If both clients stop processing further
            // messages until their respective request completes, they won't have a chance to
            // respond to the other client's request and cause a deadlock.
            //
            // This arrangement ensures we will attempt to process earlier messages first, but fall
            // back to processing messages arrived later in the spirit of making progress.
            let mut foreground_message_handlers = FuturesUnordered::new();
            let concurrent_handlers = Arc::new(Semaphore::new(256));
            loop {
                let next_message = async {
                    let permit = concurrent_handlers.clone().acquire_owned().await.unwrap();
                    let message = incoming_rx.next().await;
                    (permit, message)
                }.fuse();
                futures::pin_mut!(next_message);
                futures::select_biased! {
                    _ = teardown.changed().fuse() => return,
                    result = handle_io => {
                        if let Err(error) = result {
                            tracing::error!(?error, "error handling I/O");
                        }
                        break;
                    }
                    _ = foreground_message_handlers.next() => {}
                    next_message = next_message => {
                        let (permit, message) = next_message;
                        if let Some(message) = message {
                            let type_name = message.payload_type_name();
                            // note: we copy all the fields from the parent span so we can query them in the logs.
                            // (https://github.com/tokio-rs/tracing/issues/2670).
                            let span = tracing::info_span!("receive message", %connection_id, %address, type_name,
                                user_id=field::Empty,
                                login=field::Empty,
                                impersonator=field::Empty,
                            );
                            principal.update_span(&span);
                            let span_enter = span.enter();
                            if let Some(handler) = this.handlers.get(&message.payload_type_id()) {
                                let is_background = message.is_background();
                                let handle_message = (handler)(message, session.clone());
                                drop(span_enter);

                                let handle_message = async move {
                                    handle_message.await;
                                    drop(permit);
                                }.instrument(span);
                                if is_background {
                                    executor.spawn_detached(handle_message);
                                } else {
                                    foreground_message_handlers.push(handle_message);
                                }
                            } else {
                                tracing::error!("no message handler");
                            }
                        } else {
                            tracing::info!("connection closed");
                            break;
                        }
                    }
                }
            }

            drop(foreground_message_handlers);
            tracing::info!("signing out");
            if let Err(error) = connection_lost(session, teardown, executor).await {
                tracing::error!(?error, "error signing out");
            }

        }.instrument(span)
    }

    async fn send_initial_client_update(
        &self,
        connection_id: ConnectionId,
        principal: &Principal,
        zed_version: ZedVersion,
        mut send_connection_id: Option<oneshot::Sender<ConnectionId>>,
        session: &Session,
    ) -> Result<()> {
        self.peer.send(
            connection_id,
            proto::Hello {
                peer_id: Some(connection_id.into()),
            },
        )?;
        tracing::info!("sent hello message");
        if let Some(send_connection_id) = send_connection_id.take() {
            let _ = send_connection_id.send(connection_id);
        }

        match principal {
            Principal::User(user) | Principal::Impersonated { user, admin: _ } => {
                if !user.connected_once {
                    self.peer.send(connection_id, proto::ShowContacts {})?;
                    self.app_state
                        .db
                        .set_user_connected_once(user.id, true)
                        .await?;
                }

                update_user_plan(user.id, session).await?;

                let contacts = self.app_state.db.get_contacts(user.id).await?;

                {
                    let mut pool = self.connection_pool.lock();
                    pool.add_connection(connection_id, user.id, user.admin, zed_version);
                    self.peer.send(
                        connection_id,
                        build_initial_contacts_update(contacts, &pool),
                    )?;
                }

                if should_auto_subscribe_to_channels(zed_version) {
                    subscribe_user_to_channels(user.id, session).await?;
                }

                if let Some(incoming_call) =
                    self.app_state.db.incoming_call_for_user(user.id).await?
                {
                    self.peer.send(connection_id, incoming_call)?;
                }

                update_user_contacts(user.id, session).await?;
            }
        }

        Ok(())
    }

    pub async fn invite_code_redeemed(
        self: &Arc<Self>,
        inviter_id: UserId,
        invitee_id: UserId,
    ) -> Result<()> {
        if let Some(user) = self.app_state.db.get_user_by_id(inviter_id).await? {
            if let Some(code) = &user.invite_code {
                let pool = self.connection_pool.lock();
                let invitee_contact = contact_for_user(invitee_id, false, &pool);
                for connection_id in pool.user_connection_ids(inviter_id) {
                    self.peer.send(
                        connection_id,
                        proto::UpdateContacts {
                            contacts: vec![invitee_contact.clone()],
                            ..Default::default()
                        },
                    )?;
                    self.peer.send(
                        connection_id,
                        proto::UpdateInviteInfo {
                            url: format!("{}{}", self.app_state.config.invite_link_prefix, &code),
                            count: user.invite_count as u32,
                        },
                    )?;
                }
            }
        }
        Ok(())
    }

    pub async fn invite_count_updated(self: &Arc<Self>, user_id: UserId) -> Result<()> {
        if let Some(user) = self.app_state.db.get_user_by_id(user_id).await? {
            if let Some(invite_code) = &user.invite_code {
                let pool = self.connection_pool.lock();
                for connection_id in pool.user_connection_ids(user_id) {
                    self.peer.send(
                        connection_id,
                        proto::UpdateInviteInfo {
                            url: format!(
                                "{}{}",
                                self.app_state.config.invite_link_prefix, invite_code
                            ),
                            count: user.invite_count as u32,
                        },
                    )?;
                }
            }
        }
        Ok(())
    }

    pub async fn refresh_llm_tokens_for_user(self: &Arc<Self>, user_id: UserId) {
        let pool = self.connection_pool.lock();
        for connection_id in pool.user_connection_ids(user_id) {
            self.peer
                .send(connection_id, proto::RefreshLlmToken {})
                .trace_err();
        }
    }

    pub async fn snapshot(self: &Arc<Self>) -> ServerSnapshot {
        ServerSnapshot {
            connection_pool: ConnectionPoolGuard {
                guard: self.connection_pool.lock(),
                _not_send: PhantomData,
            },
            peer: &self.peer,
        }
    }
}

impl Deref for ConnectionPoolGuard<'_> {
    type Target = ConnectionPool;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl DerefMut for ConnectionPoolGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

impl Drop for ConnectionPoolGuard<'_> {
    fn drop(&mut self) {
        #[cfg(test)]
        self.check_invariants();
    }
}

fn broadcast<F>(
    sender_id: Option<ConnectionId>,
    receiver_ids: impl IntoIterator<Item = ConnectionId>,
    mut f: F,
) where
    F: FnMut(ConnectionId) -> anyhow::Result<()>,
{
    for receiver_id in receiver_ids {
        if Some(receiver_id) != sender_id {
            if let Err(error) = f(receiver_id) {
                tracing::error!("failed to send to {:?} {}", receiver_id, error);
            }
        }
    }
}

pub struct ProtocolVersion(u32);

impl Header for ProtocolVersion {
    fn name() -> &'static HeaderName {
        static ZED_PROTOCOL_VERSION: OnceLock<HeaderName> = OnceLock::new();
        ZED_PROTOCOL_VERSION.get_or_init(|| HeaderName::from_static("x-zed-protocol-version"))
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let version = values
            .next()
            .ok_or_else(axum::headers::Error::invalid)?
            .to_str()
            .map_err(|_| axum::headers::Error::invalid())?
            .parse()
            .map_err(|_| axum::headers::Error::invalid())?;
        Ok(Self(version))
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        values.extend([self.0.to_string().parse().unwrap()]);
    }
}

pub struct AppVersionHeader(SemanticVersion);
impl Header for AppVersionHeader {
    fn name() -> &'static HeaderName {
        static ZED_APP_VERSION: OnceLock<HeaderName> = OnceLock::new();
        ZED_APP_VERSION.get_or_init(|| HeaderName::from_static("x-zed-app-version"))
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let version = values
            .next()
            .ok_or_else(axum::headers::Error::invalid)?
            .to_str()
            .map_err(|_| axum::headers::Error::invalid())?
            .parse()
            .map_err(|_| axum::headers::Error::invalid())?;
        Ok(Self(version))
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        values.extend([self.0.to_string().parse().unwrap()]);
    }
}

pub fn routes(server: Arc<Server>) -> Router<(), Body> {
    Router::new()
        .route("/rpc", get(handle_websocket_request))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(server.app_state.clone()))
                .layer(middleware::from_fn(auth::validate_header)),
        )
        .route("/metrics", get(handle_metrics))
        .layer(Extension(server))
}

pub async fn handle_websocket_request(
    TypedHeader(ProtocolVersion(protocol_version)): TypedHeader<ProtocolVersion>,
    app_version_header: Option<TypedHeader<AppVersionHeader>>,
    ConnectInfo(socket_address): ConnectInfo<SocketAddr>,
    Extension(server): Extension<Arc<Server>>,
    Extension(principal): Extension<Principal>,
    country_code_header: Option<TypedHeader<CloudflareIpCountryHeader>>,
    system_id_header: Option<TypedHeader<SystemIdHeader>>,
    ws: WebSocketUpgrade,
) -> axum::response::Response {
    if protocol_version != rpc::PROTOCOL_VERSION {
        return (
            StatusCode::UPGRADE_REQUIRED,
            "client must be upgraded".to_string(),
        )
            .into_response();
    }

    let Some(version) = app_version_header.map(|header| ZedVersion(header.0.0)) else {
        return (
            StatusCode::UPGRADE_REQUIRED,
            "no version header found".to_string(),
        )
            .into_response();
    };

    if !version.can_collaborate() {
        return (
            StatusCode::UPGRADE_REQUIRED,
            "client must be upgraded".to_string(),
        )
            .into_response();
    }

    let socket_address = socket_address.to_string();
    ws.on_upgrade(move |socket| {
        let socket = socket
            .map_ok(to_tungstenite_message)
            .err_into()
            .with(|message| async move { to_axum_message(message) });
        let connection = Connection::new(Box::pin(socket));
        async move {
            server
                .handle_connection(
                    connection,
                    socket_address,
                    principal,
                    version,
                    country_code_header.map(|header| header.to_string()),
                    system_id_header.map(|header| header.to_string()),
                    None,
                    Executor::Production,
                )
                .await;
        }
    })
}

pub async fn handle_metrics(Extension(server): Extension<Arc<Server>>) -> Result<String> {
    static CONNECTIONS_METRIC: OnceLock<IntGauge> = OnceLock::new();
    let connections_metric = CONNECTIONS_METRIC
        .get_or_init(|| register_int_gauge!("connections", "number of connections").unwrap());

    let connections = server
        .connection_pool
        .lock()
        .connections()
        .filter(|connection| !connection.admin)
        .count();
    connections_metric.set(connections as _);

    static SHARED_PROJECTS_METRIC: OnceLock<IntGauge> = OnceLock::new();
    let shared_projects_metric = SHARED_PROJECTS_METRIC.get_or_init(|| {
        register_int_gauge!(
            "shared_projects",
            "number of open projects with one or more guests"
        )
        .unwrap()
    });

    let shared_projects = server.app_state.db.project_count_excluding_admins().await?;
    shared_projects_metric.set(shared_projects as _);

    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    let encoded_metrics = encoder
        .encode_to_string(&metric_families)
        .map_err(|err| anyhow!("{}", err))?;
    Ok(encoded_metrics)
}

#[instrument(err, skip(executor))]
async fn connection_lost(
    session: Session,
    mut teardown: watch::Receiver<bool>,
    executor: Executor,
) -> Result<()> {
    session.peer.disconnect(session.connection_id);
    session
        .connection_pool()
        .await
        .remove_connection(session.connection_id)?;

    session
        .db()
        .await
        .connection_lost(session.connection_id)
        .await
        .trace_err();

    futures::select_biased! {
        _ = executor.sleep(RECONNECT_TIMEOUT).fuse() => {

            log::info!("connection lost, removing all resources for user:{}, connection:{:?}", session.user_id(), session.connection_id);
            leave_room_for_session(&session, session.connection_id).await.trace_err();
            leave_channel_buffers_for_session(&session)
                .await
                .trace_err();

            if !session
                .connection_pool()
                .await
                .is_user_online(session.user_id())
            {
                let db = session.db().await;
                if let Some(room) = db.decline_call(None, session.user_id()).await.trace_err().flatten() {
                    room_updated(&room, &session.peer);
                }
            }

            update_user_contacts(session.user_id(), &session).await?;
        },
        _ = teardown.changed().fuse() => {}
    }

    Ok(())
}

/// Acknowledges a ping from a client, used to keep the connection alive.
async fn ping(_: proto::Ping, response: Response<proto::Ping>, _session: Session) -> Result<()> {
    response.send(proto::Ack {})?;
    Ok(())
}

/// Creates a new room for calling (outside of channels)
async fn create_room(
    _request: proto::CreateRoom,
    response: Response<proto::CreateRoom>,
    session: Session,
) -> Result<()> {
    let livekit_room = nanoid::nanoid!(30);

    let live_kit_connection_info = util::maybe!(async {
        let live_kit = session.app_state.livekit_client.as_ref();
        let live_kit = live_kit?;
        let user_id = session.user_id().to_string();

        let token = live_kit
            .room_token(&livekit_room, &user_id.to_string())
            .trace_err()?;

        Some(proto::LiveKitConnectionInfo {
            server_url: live_kit.url().into(),
            token,
            can_publish: true,
        })
    })
    .await;

    let room = session
        .db()
        .await
        .create_room(session.user_id(), session.connection_id, &livekit_room)
        .await?;

    response.send(proto::CreateRoomResponse {
        room: Some(room.clone()),
        live_kit_connection_info,
    })?;

    update_user_contacts(session.user_id(), &session).await?;
    Ok(())
}

/// Join a room from an invitation. Equivalent to joining a channel if there is one.
async fn join_room(
    request: proto::JoinRoom,
    response: Response<proto::JoinRoom>,
    session: Session,
) -> Result<()> {
    let room_id = RoomId::from_proto(request.id);

    let channel_id = session.db().await.channel_id_for_room(room_id).await?;

    if let Some(channel_id) = channel_id {
        return join_channel_internal(channel_id, Box::new(response), session).await;
    }

    let joined_room = {
        let room = session
            .db()
            .await
            .join_room(room_id, session.user_id(), session.connection_id)
            .await?;
        room_updated(&room.room, &session.peer);
        room.into_inner()
    };

    for connection_id in session
        .connection_pool()
        .await
        .user_connection_ids(session.user_id())
    {
        session
            .peer
            .send(
                connection_id,
                proto::CallCanceled {
                    room_id: room_id.to_proto(),
                },
            )
            .trace_err();
    }

    let live_kit_connection_info = if let Some(live_kit) = session.app_state.livekit_client.as_ref()
    {
        live_kit
            .room_token(
                &joined_room.room.livekit_room,
                &session.user_id().to_string(),
            )
            .trace_err()
            .map(|token| proto::LiveKitConnectionInfo {
                server_url: live_kit.url().into(),
                token,
                can_publish: true,
            })
    } else {
        None
    };

    response.send(proto::JoinRoomResponse {
        room: Some(joined_room.room),
        channel_id: None,
        live_kit_connection_info,
    })?;

    update_user_contacts(session.user_id(), &session).await?;
    Ok(())
}

/// Rejoin room is used to reconnect to a room after connection errors.
async fn rejoin_room(
    request: proto::RejoinRoom,
    response: Response<proto::RejoinRoom>,
    session: Session,
) -> Result<()> {
    let room;
    let channel;
    {
        let mut rejoined_room = session
            .db()
            .await
            .rejoin_room(request, session.user_id(), session.connection_id)
            .await?;

        response.send(proto::RejoinRoomResponse {
            room: Some(rejoined_room.room.clone()),
            reshared_projects: rejoined_room
                .reshared_projects
                .iter()
                .map(|project| proto::ResharedProject {
                    id: project.id.to_proto(),
                    collaborators: project
                        .collaborators
                        .iter()
                        .map(|collaborator| collaborator.to_proto())
                        .collect(),
                })
                .collect(),
            rejoined_projects: rejoined_room
                .rejoined_projects
                .iter()
                .map(|rejoined_project| rejoined_project.to_proto())
                .collect(),
        })?;
        room_updated(&rejoined_room.room, &session.peer);

        for project in &rejoined_room.reshared_projects {
            for collaborator in &project.collaborators {
                session
                    .peer
                    .send(
                        collaborator.connection_id,
                        proto::UpdateProjectCollaborator {
                            project_id: project.id.to_proto(),
                            old_peer_id: Some(project.old_connection_id.into()),
                            new_peer_id: Some(session.connection_id.into()),
                        },
                    )
                    .trace_err();
            }

            broadcast(
                Some(session.connection_id),
                project
                    .collaborators
                    .iter()
                    .map(|collaborator| collaborator.connection_id),
                |connection_id| {
                    session.peer.forward_send(
                        session.connection_id,
                        connection_id,
                        proto::UpdateProject {
                            project_id: project.id.to_proto(),
                            worktrees: project.worktrees.clone(),
                        },
                    )
                },
            );
        }

        notify_rejoined_projects(&mut rejoined_room.rejoined_projects, &session)?;

        let rejoined_room = rejoined_room.into_inner();

        room = rejoined_room.room;
        channel = rejoined_room.channel;
    }

    if let Some(channel) = channel {
        channel_updated(
            &channel,
            &room,
            &session.peer,
            &*session.connection_pool().await,
        );
    }

    update_user_contacts(session.user_id(), &session).await?;
    Ok(())
}

fn notify_rejoined_projects(
    rejoined_projects: &mut Vec<RejoinedProject>,
    session: &Session,
) -> Result<()> {
    for project in rejoined_projects.iter() {
        for collaborator in &project.collaborators {
            session
                .peer
                .send(
                    collaborator.connection_id,
                    proto::UpdateProjectCollaborator {
                        project_id: project.id.to_proto(),
                        old_peer_id: Some(project.old_connection_id.into()),
                        new_peer_id: Some(session.connection_id.into()),
                    },
                )
                .trace_err();
        }
    }

    for project in rejoined_projects {
        for worktree in mem::take(&mut project.worktrees) {
            // Stream this worktree's entries.
            let message = proto::UpdateWorktree {
                project_id: project.id.to_proto(),
                worktree_id: worktree.id,
                abs_path: worktree.abs_path.clone(),
                root_name: worktree.root_name,
                updated_entries: worktree.updated_entries,
                removed_entries: worktree.removed_entries,
                scan_id: worktree.scan_id,
                is_last_update: worktree.completed_scan_id == worktree.scan_id,
                updated_repositories: worktree.updated_repositories,
                removed_repositories: worktree.removed_repositories,
            };
            for update in proto::split_worktree_update(message) {
                session.peer.send(session.connection_id, update)?;
            }

            // Stream this worktree's diagnostics.
            for summary in worktree.diagnostic_summaries {
                session.peer.send(
                    session.connection_id,
                    proto::UpdateDiagnosticSummary {
                        project_id: project.id.to_proto(),
                        worktree_id: worktree.id,
                        summary: Some(summary),
                    },
                )?;
            }

            for settings_file in worktree.settings_files {
                session.peer.send(
                    session.connection_id,
                    proto::UpdateWorktreeSettings {
                        project_id: project.id.to_proto(),
                        worktree_id: worktree.id,
                        path: settings_file.path,
                        content: Some(settings_file.content),
                        kind: Some(settings_file.kind.to_proto().into()),
                    },
                )?;
            }
        }

        for repository in mem::take(&mut project.updated_repositories) {
            for update in split_repository_update(repository) {
                session.peer.send(session.connection_id, update)?;
            }
        }

        for id in mem::take(&mut project.removed_repositories) {
            session.peer.send(
                session.connection_id,
                proto::RemoveRepository {
                    project_id: project.id.to_proto(),
                    id,
                },
            )?;
        }
    }

    Ok(())
}

/// leave room disconnects from the room.
async fn leave_room(
    _: proto::LeaveRoom,
    response: Response<proto::LeaveRoom>,
    session: Session,
) -> Result<()> {
    leave_room_for_session(&session, session.connection_id).await?;
    response.send(proto::Ack {})?;
    Ok(())
}

/// Updates the permissions of someone else in the room.
async fn set_room_participant_role(
    request: proto::SetRoomParticipantRole,
    response: Response<proto::SetRoomParticipantRole>,
    session: Session,
) -> Result<()> {
    let user_id = UserId::from_proto(request.user_id);
    let role = ChannelRole::from(request.role());

    let (livekit_room, can_publish) = {
        let room = session
            .db()
            .await
            .set_room_participant_role(
                session.user_id(),
                RoomId::from_proto(request.room_id),
                user_id,
                role,
            )
            .await?;

        let livekit_room = room.livekit_room.clone();
        let can_publish = ChannelRole::from(request.role()).can_use_microphone();
        room_updated(&room, &session.peer);
        (livekit_room, can_publish)
    };

    if let Some(live_kit) = session.app_state.livekit_client.as_ref() {
        live_kit
            .update_participant(
                livekit_room.clone(),
                request.user_id.to_string(),
                livekit_api::proto::ParticipantPermission {
                    can_subscribe: true,
                    can_publish,
                    can_publish_data: can_publish,
                    hidden: false,
                    recorder: false,
                },
            )
            .await
            .trace_err();
    }

    response.send(proto::Ack {})?;
    Ok(())
}

/// Call someone else into the current room
async fn call(
    request: proto::Call,
    response: Response<proto::Call>,
    session: Session,
) -> Result<()> {
    let room_id = RoomId::from_proto(request.room_id);
    let calling_user_id = session.user_id();
    let calling_connection_id = session.connection_id;
    let called_user_id = UserId::from_proto(request.called_user_id);
    let initial_project_id = request.initial_project_id.map(ProjectId::from_proto);
    if !session
        .db()
        .await
        .has_contact(calling_user_id, called_user_id)
        .await?
    {
        return Err(anyhow!("cannot call a user who isn't a contact"))?;
    }

    let incoming_call = {
        let (room, incoming_call) = &mut *session
            .db()
            .await
            .call(
                room_id,
                calling_user_id,
                calling_connection_id,
                called_user_id,
                initial_project_id,
            )
            .await?;
        room_updated(room, &session.peer);
        mem::take(incoming_call)
    };
    update_user_contacts(called_user_id, &session).await?;

    let mut calls = session
        .connection_pool()
        .await
        .user_connection_ids(called_user_id)
        .map(|connection_id| session.peer.request(connection_id, incoming_call.clone()))
        .collect::<FuturesUnordered<_>>();

    while let Some(call_response) = calls.next().await {
        match call_response.as_ref() {
            Ok(_) => {
                response.send(proto::Ack {})?;
                return Ok(());
            }
            Err(_) => {
                call_response.trace_err();
            }
        }
    }

    {
        let room = session
            .db()
            .await
            .call_failed(room_id, called_user_id)
            .await?;
        room_updated(&room, &session.peer);
    }
    update_user_contacts(called_user_id, &session).await?;

    Err(anyhow!("failed to ring user"))?
}

/// Cancel an outgoing call.
async fn cancel_call(
    request: proto::CancelCall,
    response: Response<proto::CancelCall>,
    session: Session,
) -> Result<()> {
    let called_user_id = UserId::from_proto(request.called_user_id);
    let room_id = RoomId::from_proto(request.room_id);
    {
        let room = session
            .db()
            .await
            .cancel_call(room_id, session.connection_id, called_user_id)
            .await?;
        room_updated(&room, &session.peer);
    }

    for connection_id in session
        .connection_pool()
        .await
        .user_connection_ids(called_user_id)
    {
        session
            .peer
            .send(
                connection_id,
                proto::CallCanceled {
                    room_id: room_id.to_proto(),
                },
            )
            .trace_err();
    }
    response.send(proto::Ack {})?;

    update_user_contacts(called_user_id, &session).await?;
    Ok(())
}

/// Decline an incoming call.
async fn decline_call(message: proto::DeclineCall, session: Session) -> Result<()> {
    let room_id = RoomId::from_proto(message.room_id);
    {
        let room = session
            .db()
            .await
            .decline_call(Some(room_id), session.user_id())
            .await?
            .ok_or_else(|| anyhow!("failed to decline call"))?;
        room_updated(&room, &session.peer);
    }

    for connection_id in session
        .connection_pool()
        .await
        .user_connection_ids(session.user_id())
    {
        session
            .peer
            .send(
                connection_id,
                proto::CallCanceled {
                    room_id: room_id.to_proto(),
                },
            )
            .trace_err();
    }
    update_user_contacts(session.user_id(), &session).await?;
    Ok(())
}

/// Updates other participants in the room with your current location.
async fn update_participant_location(
    request: proto::UpdateParticipantLocation,
    response: Response<proto::UpdateParticipantLocation>,
    session: Session,
) -> Result<()> {
    let room_id = RoomId::from_proto(request.room_id);
    let location = request
        .location
        .ok_or_else(|| anyhow!("invalid location"))?;

    let db = session.db().await;
    let room = db
        .update_room_participant_location(room_id, session.connection_id, location)
        .await?;

    room_updated(&room, &session.peer);
    response.send(proto::Ack {})?;
    Ok(())
}

/// Share a project into the room.
async fn share_project(
    request: proto::ShareProject,
    response: Response<proto::ShareProject>,
    session: Session,
) -> Result<()> {
    let (project_id, room) = &*session
        .db()
        .await
        .share_project(
            RoomId::from_proto(request.room_id),
            session.connection_id,
            &request.worktrees,
            request.is_ssh_project,
        )
        .await?;
    response.send(proto::ShareProjectResponse {
        project_id: project_id.to_proto(),
    })?;
    room_updated(room, &session.peer);

    Ok(())
}

/// Unshare a project from the room.
async fn unshare_project(message: proto::UnshareProject, session: Session) -> Result<()> {
    let project_id = ProjectId::from_proto(message.project_id);
    unshare_project_internal(project_id, session.connection_id, &session).await
}

async fn unshare_project_internal(
    project_id: ProjectId,
    connection_id: ConnectionId,
    session: &Session,
) -> Result<()> {
    let delete = {
        let room_guard = session
            .db()
            .await
            .unshare_project(project_id, connection_id)
            .await?;

        let (delete, room, guest_connection_ids) = &*room_guard;

        let message = proto::UnshareProject {
            project_id: project_id.to_proto(),
        };

        broadcast(
            Some(connection_id),
            guest_connection_ids.iter().copied(),
            |conn_id| session.peer.send(conn_id, message.clone()),
        );
        if let Some(room) = room {
            room_updated(room, &session.peer);
        }

        *delete
    };

    if delete {
        let db = session.db().await;
        db.delete_project(project_id).await?;
    }

    Ok(())
}

/// Join someone elses shared project.
async fn join_project(
    request: proto::JoinProject,
    response: Response<proto::JoinProject>,
    session: Session,
) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);

    tracing::info!(%project_id, "join project");

    let db = session.db().await;
    let (project, replica_id) = &mut *db
        .join_project(project_id, session.connection_id, session.user_id())
        .await?;
    drop(db);
    tracing::info!(%project_id, "join remote project");
    join_project_internal(response, session, project, replica_id)
}

trait JoinProjectInternalResponse {
    fn send(self, result: proto::JoinProjectResponse) -> Result<()>;
}
impl JoinProjectInternalResponse for Response<proto::JoinProject> {
    fn send(self, result: proto::JoinProjectResponse) -> Result<()> {
        Response::<proto::JoinProject>::send(self, result)
    }
}

fn join_project_internal(
    response: impl JoinProjectInternalResponse,
    session: Session,
    project: &mut Project,
    replica_id: &ReplicaId,
) -> Result<()> {
    let collaborators = project
        .collaborators
        .iter()
        .filter(|collaborator| collaborator.connection_id != session.connection_id)
        .map(|collaborator| collaborator.to_proto())
        .collect::<Vec<_>>();
    let project_id = project.id;
    let guest_user_id = session.user_id();

    let worktrees = project
        .worktrees
        .iter()
        .map(|(id, worktree)| proto::WorktreeMetadata {
            id: *id,
            root_name: worktree.root_name.clone(),
            visible: worktree.visible,
            abs_path: worktree.abs_path.clone(),
        })
        .collect::<Vec<_>>();

    let add_project_collaborator = proto::AddProjectCollaborator {
        project_id: project_id.to_proto(),
        collaborator: Some(proto::Collaborator {
            peer_id: Some(session.connection_id.into()),
            replica_id: replica_id.0 as u32,
            user_id: guest_user_id.to_proto(),
            is_host: false,
        }),
    };

    for collaborator in &collaborators {
        session
            .peer
            .send(
                collaborator.peer_id.unwrap().into(),
                add_project_collaborator.clone(),
            )
            .trace_err();
    }

    // First, we send the metadata associated with each worktree.
    response.send(proto::JoinProjectResponse {
        project_id: project.id.0 as u64,
        worktrees: worktrees.clone(),
        replica_id: replica_id.0 as u32,
        collaborators: collaborators.clone(),
        language_servers: project.language_servers.clone(),
        role: project.role.into(),
    })?;

    for (worktree_id, worktree) in mem::take(&mut project.worktrees) {
        // Stream this worktree's entries.
        let message = proto::UpdateWorktree {
            project_id: project_id.to_proto(),
            worktree_id,
            abs_path: worktree.abs_path.clone(),
            root_name: worktree.root_name,
            updated_entries: worktree.entries,
            removed_entries: Default::default(),
            scan_id: worktree.scan_id,
            is_last_update: worktree.scan_id == worktree.completed_scan_id,
            updated_repositories: worktree.legacy_repository_entries.into_values().collect(),
            removed_repositories: Default::default(),
        };
        for update in proto::split_worktree_update(message) {
            session.peer.send(session.connection_id, update.clone())?;
        }

        // Stream this worktree's diagnostics.
        for summary in worktree.diagnostic_summaries {
            session.peer.send(
                session.connection_id,
                proto::UpdateDiagnosticSummary {
                    project_id: project_id.to_proto(),
                    worktree_id: worktree.id,
                    summary: Some(summary),
                },
            )?;
        }

        for settings_file in worktree.settings_files {
            session.peer.send(
                session.connection_id,
                proto::UpdateWorktreeSettings {
                    project_id: project_id.to_proto(),
                    worktree_id: worktree.id,
                    path: settings_file.path,
                    content: Some(settings_file.content),
                    kind: Some(settings_file.kind.to_proto() as i32),
                },
            )?;
        }
    }

    for repository in mem::take(&mut project.repositories) {
        for update in split_repository_update(repository) {
            session.peer.send(session.connection_id, update)?;
        }
    }

    for language_server in &project.language_servers {
        session.peer.send(
            session.connection_id,
            proto::UpdateLanguageServer {
                project_id: project_id.to_proto(),
                language_server_id: language_server.id,
                variant: Some(
                    proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(
                        proto::LspDiskBasedDiagnosticsUpdated {},
                    ),
                ),
            },
        )?;
    }

    Ok(())
}

/// Leave someone elses shared project.
async fn leave_project(request: proto::LeaveProject, session: Session) -> Result<()> {
    let sender_id = session.connection_id;
    let project_id = ProjectId::from_proto(request.project_id);
    let db = session.db().await;

    let (room, project) = &*db.leave_project(project_id, sender_id).await?;
    tracing::info!(
        %project_id,
        "leave project"
    );

    project_left(project, &session);
    if let Some(room) = room {
        room_updated(room, &session.peer);
    }

    Ok(())
}

/// Updates other participants with changes to the project
async fn update_project(
    request: proto::UpdateProject,
    response: Response<proto::UpdateProject>,
    session: Session,
) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let (room, guest_connection_ids) = &*session
        .db()
        .await
        .update_project(project_id, session.connection_id, &request.worktrees)
        .await?;
    broadcast(
        Some(session.connection_id),
        guest_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    if let Some(room) = room {
        room_updated(room, &session.peer);
    }
    response.send(proto::Ack {})?;

    Ok(())
}

/// Updates other participants with changes to the worktree
async fn update_worktree(
    request: proto::UpdateWorktree,
    response: Response<proto::UpdateWorktree>,
    session: Session,
) -> Result<()> {
    let guest_connection_ids = session
        .db()
        .await
        .update_worktree(&request, session.connection_id)
        .await?;

    broadcast(
        Some(session.connection_id),
        guest_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    response.send(proto::Ack {})?;
    Ok(())
}

async fn update_repository(
    request: proto::UpdateRepository,
    response: Response<proto::UpdateRepository>,
    session: Session,
) -> Result<()> {
    let guest_connection_ids = session
        .db()
        .await
        .update_repository(&request, session.connection_id)
        .await?;

    broadcast(
        Some(session.connection_id),
        guest_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    response.send(proto::Ack {})?;
    Ok(())
}

async fn remove_repository(
    request: proto::RemoveRepository,
    response: Response<proto::RemoveRepository>,
    session: Session,
) -> Result<()> {
    let guest_connection_ids = session
        .db()
        .await
        .remove_repository(&request, session.connection_id)
        .await?;

    broadcast(
        Some(session.connection_id),
        guest_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    response.send(proto::Ack {})?;
    Ok(())
}

/// Updates other participants with changes to the diagnostics
async fn update_diagnostic_summary(
    message: proto::UpdateDiagnosticSummary,
    session: Session,
) -> Result<()> {
    let guest_connection_ids = session
        .db()
        .await
        .update_diagnostic_summary(&message, session.connection_id)
        .await?;

    broadcast(
        Some(session.connection_id),
        guest_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, message.clone())
        },
    );

    Ok(())
}

/// Updates other participants with changes to the worktree settings
async fn update_worktree_settings(
    message: proto::UpdateWorktreeSettings,
    session: Session,
) -> Result<()> {
    let guest_connection_ids = session
        .db()
        .await
        .update_worktree_settings(&message, session.connection_id)
        .await?;

    broadcast(
        Some(session.connection_id),
        guest_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, message.clone())
        },
    );

    Ok(())
}

/// Notify other participants that a language server has started.
async fn start_language_server(
    request: proto::StartLanguageServer,
    session: Session,
) -> Result<()> {
    let guest_connection_ids = session
        .db()
        .await
        .start_language_server(&request, session.connection_id)
        .await?;

    broadcast(
        Some(session.connection_id),
        guest_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    Ok(())
}

/// Notify other participants that a language server has changed.
async fn update_language_server(
    request: proto::UpdateLanguageServer,
    session: Session,
) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let project_connection_ids = session
        .db()
        .await
        .project_connection_ids(project_id, session.connection_id, true)
        .await?;
    broadcast(
        Some(session.connection_id),
        project_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    Ok(())
}

/// forward a project request to the host. These requests should be read only
/// as guests are allowed to send them.
async fn forward_read_only_project_request<T>(
    request: T,
    response: Response<T>,
    session: Session,
) -> Result<()>
where
    T: EntityMessage + RequestMessage,
{
    let project_id = ProjectId::from_proto(request.remote_entity_id());
    let host_connection_id = session
        .db()
        .await
        .host_for_read_only_project_request(project_id, session.connection_id)
        .await?;
    let payload = session
        .peer
        .forward_request(session.connection_id, host_connection_id, request)
        .await?;
    response.send(payload)?;
    Ok(())
}

async fn forward_find_search_candidates_request(
    request: proto::FindSearchCandidates,
    response: Response<proto::FindSearchCandidates>,
    session: Session,
) -> Result<()> {
    let project_id = ProjectId::from_proto(request.remote_entity_id());
    let host_connection_id = session
        .db()
        .await
        .host_for_read_only_project_request(project_id, session.connection_id)
        .await?;
    let payload = session
        .peer
        .forward_request(session.connection_id, host_connection_id, request)
        .await?;
    response.send(payload)?;
    Ok(())
}

/// forward a project request to the host. These requests are disallowed
/// for guests.
async fn forward_mutating_project_request<T>(
    request: T,
    response: Response<T>,
    session: Session,
) -> Result<()>
where
    T: EntityMessage + RequestMessage,
{
    let project_id = ProjectId::from_proto(request.remote_entity_id());

    let host_connection_id = session
        .db()
        .await
        .host_for_mutating_project_request(project_id, session.connection_id)
        .await?;
    let payload = session
        .peer
        .forward_request(session.connection_id, host_connection_id, request)
        .await?;
    response.send(payload)?;
    Ok(())
}

/// Notify other participants that a new buffer has been created
async fn create_buffer_for_peer(
    request: proto::CreateBufferForPeer,
    session: Session,
) -> Result<()> {
    session
        .db()
        .await
        .check_user_is_project_host(
            ProjectId::from_proto(request.project_id),
            session.connection_id,
        )
        .await?;
    let peer_id = request.peer_id.ok_or_else(|| anyhow!("invalid peer id"))?;
    session
        .peer
        .forward_send(session.connection_id, peer_id.into(), request)?;
    Ok(())
}

/// Notify other participants that a buffer has been updated. This is
/// allowed for guests as long as the update is limited to selections.
async fn update_buffer(
    request: proto::UpdateBuffer,
    response: Response<proto::UpdateBuffer>,
    session: Session,
) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let mut capability = Capability::ReadOnly;

    for op in request.operations.iter() {
        match op.variant {
            None | Some(proto::operation::Variant::UpdateSelections(_)) => {}
            Some(_) => capability = Capability::ReadWrite,
        }
    }

    let host = {
        let guard = session
            .db()
            .await
            .connections_for_buffer_update(project_id, session.connection_id, capability)
            .await?;

        let (host, guests) = &*guard;

        broadcast(
            Some(session.connection_id),
            guests.clone(),
            |connection_id| {
                session
                    .peer
                    .forward_send(session.connection_id, connection_id, request.clone())
            },
        );

        *host
    };

    if host != session.connection_id {
        session
            .peer
            .forward_request(session.connection_id, host, request.clone())
            .await?;
    }

    response.send(proto::Ack {})?;
    Ok(())
}

async fn update_context(message: proto::UpdateContext, session: Session) -> Result<()> {
    let project_id = ProjectId::from_proto(message.project_id);

    let operation = message.operation.as_ref().context("invalid operation")?;
    let capability = match operation.variant.as_ref() {
        Some(proto::context_operation::Variant::BufferOperation(buffer_op)) => {
            if let Some(buffer_op) = buffer_op.operation.as_ref() {
                match buffer_op.variant {
                    None | Some(proto::operation::Variant::UpdateSelections(_)) => {
                        Capability::ReadOnly
                    }
                    _ => Capability::ReadWrite,
                }
            } else {
                Capability::ReadWrite
            }
        }
        Some(_) => Capability::ReadWrite,
        None => Capability::ReadOnly,
    };

    let guard = session
        .db()
        .await
        .connections_for_buffer_update(project_id, session.connection_id, capability)
        .await?;

    let (host, guests) = &*guard;

    broadcast(
        Some(session.connection_id),
        guests.iter().chain([host]).copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, message.clone())
        },
    );

    Ok(())
}

/// Notify other participants that a project has been updated.
async fn broadcast_project_message_from_host<T: EntityMessage<Entity = ShareProject>>(
    request: T,
    session: Session,
) -> Result<()> {
    let project_id = ProjectId::from_proto(request.remote_entity_id());
    let project_connection_ids = session
        .db()
        .await
        .project_connection_ids(project_id, session.connection_id, false)
        .await?;

    broadcast(
        Some(session.connection_id),
        project_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    Ok(())
}

/// Start following another user in a call.
async fn follow(
    request: proto::Follow,
    response: Response<proto::Follow>,
    session: Session,
) -> Result<()> {
    let room_id = RoomId::from_proto(request.room_id);
    let project_id = request.project_id.map(ProjectId::from_proto);
    let leader_id = request
        .leader_id
        .ok_or_else(|| anyhow!("invalid leader id"))?
        .into();
    let follower_id = session.connection_id;

    session
        .db()
        .await
        .check_room_participants(room_id, leader_id, session.connection_id)
        .await?;

    let response_payload = session
        .peer
        .forward_request(session.connection_id, leader_id, request)
        .await?;
    response.send(response_payload)?;

    if let Some(project_id) = project_id {
        let room = session
            .db()
            .await
            .follow(room_id, project_id, leader_id, follower_id)
            .await?;
        room_updated(&room, &session.peer);
    }

    Ok(())
}

/// Stop following another user in a call.
async fn unfollow(request: proto::Unfollow, session: Session) -> Result<()> {
    let room_id = RoomId::from_proto(request.room_id);
    let project_id = request.project_id.map(ProjectId::from_proto);
    let leader_id = request
        .leader_id
        .ok_or_else(|| anyhow!("invalid leader id"))?
        .into();
    let follower_id = session.connection_id;

    session
        .db()
        .await
        .check_room_participants(room_id, leader_id, session.connection_id)
        .await?;

    session
        .peer
        .forward_send(session.connection_id, leader_id, request)?;

    if let Some(project_id) = project_id {
        let room = session
            .db()
            .await
            .unfollow(room_id, project_id, leader_id, follower_id)
            .await?;
        room_updated(&room, &session.peer);
    }

    Ok(())
}

/// Notify everyone following you of your current location.
async fn update_followers(request: proto::UpdateFollowers, session: Session) -> Result<()> {
    let room_id = RoomId::from_proto(request.room_id);
    let database = session.db.lock().await;

    let connection_ids = if let Some(project_id) = request.project_id {
        let project_id = ProjectId::from_proto(project_id);
        database
            .project_connection_ids(project_id, session.connection_id, true)
            .await?
    } else {
        database
            .room_connection_ids(room_id, session.connection_id)
            .await?
    };

    // For now, don't send view update messages back to that view's current leader.
    let peer_id_to_omit = request.variant.as_ref().and_then(|variant| match variant {
        proto::update_followers::Variant::UpdateView(payload) => payload.leader_id,
        _ => None,
    });

    for connection_id in connection_ids.iter().cloned() {
        if Some(connection_id.into()) != peer_id_to_omit && connection_id != session.connection_id {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())?;
        }
    }
    Ok(())
}

/// Get public data about users.
async fn get_users(
    request: proto::GetUsers,
    response: Response<proto::GetUsers>,
    session: Session,
) -> Result<()> {
    let user_ids = request
        .user_ids
        .into_iter()
        .map(UserId::from_proto)
        .collect();
    let users = session
        .db()
        .await
        .get_users_by_ids(user_ids)
        .await?
        .into_iter()
        .map(|user| proto::User {
            id: user.id.to_proto(),
            avatar_url: format!("https://github.com/{}.png?size=128", user.github_login),
            github_login: user.github_login,
            email: user.email_address,
            name: user.name,
        })
        .collect();
    response.send(proto::UsersResponse { users })?;
    Ok(())
}

/// Search for users (to invite) buy Github login
async fn fuzzy_search_users(
    request: proto::FuzzySearchUsers,
    response: Response<proto::FuzzySearchUsers>,
    session: Session,
) -> Result<()> {
    let query = request.query;
    let users = match query.len() {
        0 => vec![],
        1 | 2 => session
            .db()
            .await
            .get_user_by_github_login(&query)
            .await?
            .into_iter()
            .collect(),
        _ => session.db().await.fuzzy_search_users(&query, 10).await?,
    };
    let users = users
        .into_iter()
        .filter(|user| user.id != session.user_id())
        .map(|user| proto::User {
            id: user.id.to_proto(),
            avatar_url: format!("https://github.com/{}.png?size=128", user.github_login),
            github_login: user.github_login,
            name: user.name,
            email: user.email_address,
        })
        .collect();
    response.send(proto::UsersResponse { users })?;
    Ok(())
}

/// Send a contact request to another user.
async fn request_contact(
    request: proto::RequestContact,
    response: Response<proto::RequestContact>,
    session: Session,
) -> Result<()> {
    let requester_id = session.user_id();
    let responder_id = UserId::from_proto(request.responder_id);
    if requester_id == responder_id {
        return Err(anyhow!("cannot add yourself as a contact"))?;
    }

    let notifications = session
        .db()
        .await
        .send_contact_request(requester_id, responder_id)
        .await?;

    // Update outgoing contact requests of requester
    let mut update = proto::UpdateContacts::default();
    update.outgoing_requests.push(responder_id.to_proto());
    for connection_id in session
        .connection_pool()
        .await
        .user_connection_ids(requester_id)
    {
        session.peer.send(connection_id, update.clone())?;
    }

    // Update incoming contact requests of responder
    let mut update = proto::UpdateContacts::default();
    update
        .incoming_requests
        .push(proto::IncomingContactRequest {
            requester_id: requester_id.to_proto(),
        });
    let connection_pool = session.connection_pool().await;
    for connection_id in connection_pool.user_connection_ids(responder_id) {
        session.peer.send(connection_id, update.clone())?;
    }

    send_notifications(&connection_pool, &session.peer, notifications);

    response.send(proto::Ack {})?;
    Ok(())
}

/// Accept or decline a contact request
async fn respond_to_contact_request(
    request: proto::RespondToContactRequest,
    response: Response<proto::RespondToContactRequest>,
    session: Session,
) -> Result<()> {
    let responder_id = session.user_id();
    let requester_id = UserId::from_proto(request.requester_id);
    let db = session.db().await;
    if request.response == proto::ContactRequestResponse::Dismiss as i32 {
        db.dismiss_contact_notification(responder_id, requester_id)
            .await?;
    } else {
        let accept = request.response == proto::ContactRequestResponse::Accept as i32;

        let notifications = db
            .respond_to_contact_request(responder_id, requester_id, accept)
            .await?;
        let requester_busy = db.is_user_busy(requester_id).await?;
        let responder_busy = db.is_user_busy(responder_id).await?;

        let pool = session.connection_pool().await;
        // Update responder with new contact
        let mut update = proto::UpdateContacts::default();
        if accept {
            update
                .contacts
                .push(contact_for_user(requester_id, requester_busy, &pool));
        }
        update
            .remove_incoming_requests
            .push(requester_id.to_proto());
        for connection_id in pool.user_connection_ids(responder_id) {
            session.peer.send(connection_id, update.clone())?;
        }

        // Update requester with new contact
        let mut update = proto::UpdateContacts::default();
        if accept {
            update
                .contacts
                .push(contact_for_user(responder_id, responder_busy, &pool));
        }
        update
            .remove_outgoing_requests
            .push(responder_id.to_proto());

        for connection_id in pool.user_connection_ids(requester_id) {
            session.peer.send(connection_id, update.clone())?;
        }

        send_notifications(&pool, &session.peer, notifications);
    }

    response.send(proto::Ack {})?;
    Ok(())
}

/// Remove a contact.
async fn remove_contact(
    request: proto::RemoveContact,
    response: Response<proto::RemoveContact>,
    session: Session,
) -> Result<()> {
    let requester_id = session.user_id();
    let responder_id = UserId::from_proto(request.user_id);
    let db = session.db().await;
    let (contact_accepted, deleted_notification_id) =
        db.remove_contact(requester_id, responder_id).await?;

    let pool = session.connection_pool().await;
    // Update outgoing contact requests of requester
    let mut update = proto::UpdateContacts::default();
    if contact_accepted {
        update.remove_contacts.push(responder_id.to_proto());
    } else {
        update
            .remove_outgoing_requests
            .push(responder_id.to_proto());
    }
    for connection_id in pool.user_connection_ids(requester_id) {
        session.peer.send(connection_id, update.clone())?;
    }

    // Update incoming contact requests of responder
    let mut update = proto::UpdateContacts::default();
    if contact_accepted {
        update.remove_contacts.push(requester_id.to_proto());
    } else {
        update
            .remove_incoming_requests
            .push(requester_id.to_proto());
    }
    for connection_id in pool.user_connection_ids(responder_id) {
        session.peer.send(connection_id, update.clone())?;
        if let Some(notification_id) = deleted_notification_id {
            session.peer.send(
                connection_id,
                proto::DeleteNotification {
                    notification_id: notification_id.to_proto(),
                },
            )?;
        }
    }

    response.send(proto::Ack {})?;
    Ok(())
}

fn should_auto_subscribe_to_channels(version: ZedVersion) -> bool {
    version.0.minor() < 139
}

async fn update_user_plan(_user_id: UserId, session: &Session) -> Result<()> {
    let plan = session.current_plan(&session.db().await).await?;

    session
        .peer
        .send(
            session.connection_id,
            proto::UpdateUserPlan { plan: plan.into() },
        )
        .trace_err();

    Ok(())
}

async fn subscribe_to_channels(_: proto::SubscribeToChannels, session: Session) -> Result<()> {
    subscribe_user_to_channels(session.user_id(), &session).await?;
    Ok(())
}

async fn subscribe_user_to_channels(user_id: UserId, session: &Session) -> Result<(), Error> {
    let channels_for_user = session.db().await.get_channels_for_user(user_id).await?;
    let mut pool = session.connection_pool().await;
    for membership in &channels_for_user.channel_memberships {
        pool.subscribe_to_channel(user_id, membership.channel_id, membership.role)
    }
    session.peer.send(
        session.connection_id,
        build_update_user_channels(&channels_for_user),
    )?;
    session.peer.send(
        session.connection_id,
        build_channels_update(channels_for_user),
    )?;
    Ok(())
}

/// Creates a new channel.
async fn create_channel(
    request: proto::CreateChannel,
    response: Response<proto::CreateChannel>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;

    let parent_id = request.parent_id.map(ChannelId::from_proto);
    let (channel, membership) = db
        .create_channel(&request.name, parent_id, session.user_id())
        .await?;

    let root_id = channel.root_id();
    let channel = Channel::from_model(channel);

    response.send(proto::CreateChannelResponse {
        channel: Some(channel.to_proto()),
        parent_id: request.parent_id,
    })?;

    let mut connection_pool = session.connection_pool().await;
    if let Some(membership) = membership {
        connection_pool.subscribe_to_channel(
            membership.user_id,
            membership.channel_id,
            membership.role,
        );
        let update = proto::UpdateUserChannels {
            channel_memberships: vec![proto::ChannelMembership {
                channel_id: membership.channel_id.to_proto(),
                role: membership.role.into(),
            }],
            ..Default::default()
        };
        for connection_id in connection_pool.user_connection_ids(membership.user_id) {
            session.peer.send(connection_id, update.clone())?;
        }
    }

    for (connection_id, role) in connection_pool.channel_connection_ids(root_id) {
        if !role.can_see_channel(channel.visibility) {
            continue;
        }

        let update = proto::UpdateChannels {
            channels: vec![channel.to_proto()],
            ..Default::default()
        };
        session.peer.send(connection_id, update.clone())?;
    }

    Ok(())
}

/// Delete a channel
async fn delete_channel(
    request: proto::DeleteChannel,
    response: Response<proto::DeleteChannel>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;

    let channel_id = request.channel_id;
    let (root_channel, removed_channels) = db
        .delete_channel(ChannelId::from_proto(channel_id), session.user_id())
        .await?;
    response.send(proto::Ack {})?;

    // Notify members of removed channels
    let mut update = proto::UpdateChannels::default();
    update
        .delete_channels
        .extend(removed_channels.into_iter().map(|id| id.to_proto()));

    let connection_pool = session.connection_pool().await;
    for (connection_id, _) in connection_pool.channel_connection_ids(root_channel) {
        session.peer.send(connection_id, update.clone())?;
    }

    Ok(())
}

/// Invite someone to join a channel.
async fn invite_channel_member(
    request: proto::InviteChannelMember,
    response: Response<proto::InviteChannelMember>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;
    let channel_id = ChannelId::from_proto(request.channel_id);
    let invitee_id = UserId::from_proto(request.user_id);
    let InviteMemberResult {
        channel,
        notifications,
    } = db
        .invite_channel_member(
            channel_id,
            invitee_id,
            session.user_id(),
            request.role().into(),
        )
        .await?;

    let update = proto::UpdateChannels {
        channel_invitations: vec![channel.to_proto()],
        ..Default::default()
    };

    let connection_pool = session.connection_pool().await;
    for connection_id in connection_pool.user_connection_ids(invitee_id) {
        session.peer.send(connection_id, update.clone())?;
    }

    send_notifications(&connection_pool, &session.peer, notifications);

    response.send(proto::Ack {})?;
    Ok(())
}

/// remove someone from a channel
async fn remove_channel_member(
    request: proto::RemoveChannelMember,
    response: Response<proto::RemoveChannelMember>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;
    let channel_id = ChannelId::from_proto(request.channel_id);
    let member_id = UserId::from_proto(request.user_id);

    let RemoveChannelMemberResult {
        membership_update,
        notification_id,
    } = db
        .remove_channel_member(channel_id, member_id, session.user_id())
        .await?;

    let mut connection_pool = session.connection_pool().await;
    notify_membership_updated(
        &mut connection_pool,
        membership_update,
        member_id,
        &session.peer,
    );
    for connection_id in connection_pool.user_connection_ids(member_id) {
        if let Some(notification_id) = notification_id {
            session
                .peer
                .send(
                    connection_id,
                    proto::DeleteNotification {
                        notification_id: notification_id.to_proto(),
                    },
                )
                .trace_err();
        }
    }

    response.send(proto::Ack {})?;
    Ok(())
}

/// Toggle the channel between public and private.
/// Care is taken to maintain the invariant that public channels only descend from public channels,
/// (though members-only channels can appear at any point in the hierarchy).
async fn set_channel_visibility(
    request: proto::SetChannelVisibility,
    response: Response<proto::SetChannelVisibility>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;
    let channel_id = ChannelId::from_proto(request.channel_id);
    let visibility = request.visibility().into();

    let channel_model = db
        .set_channel_visibility(channel_id, visibility, session.user_id())
        .await?;
    let root_id = channel_model.root_id();
    let channel = Channel::from_model(channel_model);

    let mut connection_pool = session.connection_pool().await;
    for (user_id, role) in connection_pool
        .channel_user_ids(root_id)
        .collect::<Vec<_>>()
        .into_iter()
    {
        let update = if role.can_see_channel(channel.visibility) {
            connection_pool.subscribe_to_channel(user_id, channel_id, role);
            proto::UpdateChannels {
                channels: vec![channel.to_proto()],
                ..Default::default()
            }
        } else {
            connection_pool.unsubscribe_from_channel(&user_id, &channel_id);
            proto::UpdateChannels {
                delete_channels: vec![channel.id.to_proto()],
                ..Default::default()
            }
        };

        for connection_id in connection_pool.user_connection_ids(user_id) {
            session.peer.send(connection_id, update.clone())?;
        }
    }

    response.send(proto::Ack {})?;
    Ok(())
}

/// Alter the role for a user in the channel.
async fn set_channel_member_role(
    request: proto::SetChannelMemberRole,
    response: Response<proto::SetChannelMemberRole>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;
    let channel_id = ChannelId::from_proto(request.channel_id);
    let member_id = UserId::from_proto(request.user_id);
    let result = db
        .set_channel_member_role(
            channel_id,
            session.user_id(),
            member_id,
            request.role().into(),
        )
        .await?;

    match result {
        db::SetMemberRoleResult::MembershipUpdated(membership_update) => {
            let mut connection_pool = session.connection_pool().await;
            notify_membership_updated(
                &mut connection_pool,
                membership_update,
                member_id,
                &session.peer,
            )
        }
        db::SetMemberRoleResult::InviteUpdated(channel) => {
            let update = proto::UpdateChannels {
                channel_invitations: vec![channel.to_proto()],
                ..Default::default()
            };

            for connection_id in session
                .connection_pool()
                .await
                .user_connection_ids(member_id)
            {
                session.peer.send(connection_id, update.clone())?;
            }
        }
    }

    response.send(proto::Ack {})?;
    Ok(())
}

/// Change the name of a channel
async fn rename_channel(
    request: proto::RenameChannel,
    response: Response<proto::RenameChannel>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;
    let channel_id = ChannelId::from_proto(request.channel_id);
    let channel_model = db
        .rename_channel(channel_id, session.user_id(), &request.name)
        .await?;
    let root_id = channel_model.root_id();
    let channel = Channel::from_model(channel_model);

    response.send(proto::RenameChannelResponse {
        channel: Some(channel.to_proto()),
    })?;

    let connection_pool = session.connection_pool().await;
    let update = proto::UpdateChannels {
        channels: vec![channel.to_proto()],
        ..Default::default()
    };
    for (connection_id, role) in connection_pool.channel_connection_ids(root_id) {
        if role.can_see_channel(channel.visibility) {
            session.peer.send(connection_id, update.clone())?;
        }
    }

    Ok(())
}

/// Move a channel to a new parent.
async fn move_channel(
    request: proto::MoveChannel,
    response: Response<proto::MoveChannel>,
    session: Session,
) -> Result<()> {
    let channel_id = ChannelId::from_proto(request.channel_id);
    let to = ChannelId::from_proto(request.to);

    let (root_id, channels) = session
        .db()
        .await
        .move_channel(channel_id, to, session.user_id())
        .await?;

    let connection_pool = session.connection_pool().await;
    for (connection_id, role) in connection_pool.channel_connection_ids(root_id) {
        let channels = channels
            .iter()
            .filter_map(|channel| {
                if role.can_see_channel(channel.visibility) {
                    Some(channel.to_proto())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if channels.is_empty() {
            continue;
        }

        let update = proto::UpdateChannels {
            channels,
            ..Default::default()
        };

        session.peer.send(connection_id, update.clone())?;
    }

    response.send(Ack {})?;
    Ok(())
}

/// Get the list of channel members
async fn get_channel_members(
    request: proto::GetChannelMembers,
    response: Response<proto::GetChannelMembers>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;
    let channel_id = ChannelId::from_proto(request.channel_id);
    let limit = if request.limit == 0 {
        u16::MAX as u64
    } else {
        request.limit
    };
    let (members, users) = db
        .get_channel_participant_details(channel_id, &request.query, limit, session.user_id())
        .await?;
    response.send(proto::GetChannelMembersResponse { members, users })?;
    Ok(())
}

/// Accept or decline a channel invitation.
async fn respond_to_channel_invite(
    request: proto::RespondToChannelInvite,
    response: Response<proto::RespondToChannelInvite>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;
    let channel_id = ChannelId::from_proto(request.channel_id);
    let RespondToChannelInvite {
        membership_update,
        notifications,
    } = db
        .respond_to_channel_invite(channel_id, session.user_id(), request.accept)
        .await?;

    let mut connection_pool = session.connection_pool().await;
    if let Some(membership_update) = membership_update {
        notify_membership_updated(
            &mut connection_pool,
            membership_update,
            session.user_id(),
            &session.peer,
        );
    } else {
        let update = proto::UpdateChannels {
            remove_channel_invitations: vec![channel_id.to_proto()],
            ..Default::default()
        };

        for connection_id in connection_pool.user_connection_ids(session.user_id()) {
            session.peer.send(connection_id, update.clone())?;
        }
    };

    send_notifications(&connection_pool, &session.peer, notifications);

    response.send(proto::Ack {})?;

    Ok(())
}

/// Join the channels' room
async fn join_channel(
    request: proto::JoinChannel,
    response: Response<proto::JoinChannel>,
    session: Session,
) -> Result<()> {
    let channel_id = ChannelId::from_proto(request.channel_id);
    join_channel_internal(channel_id, Box::new(response), session).await
}

trait JoinChannelInternalResponse {
    fn send(self, result: proto::JoinRoomResponse) -> Result<()>;
}
impl JoinChannelInternalResponse for Response<proto::JoinChannel> {
    fn send(self, result: proto::JoinRoomResponse) -> Result<()> {
        Response::<proto::JoinChannel>::send(self, result)
    }
}
impl JoinChannelInternalResponse for Response<proto::JoinRoom> {
    fn send(self, result: proto::JoinRoomResponse) -> Result<()> {
        Response::<proto::JoinRoom>::send(self, result)
    }
}

async fn join_channel_internal(
    channel_id: ChannelId,
    response: Box<impl JoinChannelInternalResponse>,
    session: Session,
) -> Result<()> {
    let joined_room = {
        let mut db = session.db().await;
        // If zed quits without leaving the room, and the user re-opens zed before the
        // RECONNECT_TIMEOUT, we need to make sure that we kick the user out of the previous
        // room they were in.
        if let Some(connection) = db.stale_room_connection(session.user_id()).await? {
            tracing::info!(
                stale_connection_id = %connection,
                "cleaning up stale connection",
            );
            drop(db);
            leave_room_for_session(&session, connection).await?;
            db = session.db().await;
        }

        let (joined_room, membership_updated, role) = db
            .join_channel(channel_id, session.user_id(), session.connection_id)
            .await?;

        let live_kit_connection_info =
            session
                .app_state
                .livekit_client
                .as_ref()
                .and_then(|live_kit| {
                    let (can_publish, token) = if role == ChannelRole::Guest {
                        (
                            false,
                            live_kit
                                .guest_token(
                                    &joined_room.room.livekit_room,
                                    &session.user_id().to_string(),
                                )
                                .trace_err()?,
                        )
                    } else {
                        (
                            true,
                            live_kit
                                .room_token(
                                    &joined_room.room.livekit_room,
                                    &session.user_id().to_string(),
                                )
                                .trace_err()?,
                        )
                    };

                    Some(LiveKitConnectionInfo {
                        server_url: live_kit.url().into(),
                        token,
                        can_publish,
                    })
                });

        response.send(proto::JoinRoomResponse {
            room: Some(joined_room.room.clone()),
            channel_id: joined_room
                .channel
                .as_ref()
                .map(|channel| channel.id.to_proto()),
            live_kit_connection_info,
        })?;

        let mut connection_pool = session.connection_pool().await;
        if let Some(membership_updated) = membership_updated {
            notify_membership_updated(
                &mut connection_pool,
                membership_updated,
                session.user_id(),
                &session.peer,
            );
        }

        room_updated(&joined_room.room, &session.peer);

        joined_room
    };

    channel_updated(
        &joined_room
            .channel
            .ok_or_else(|| anyhow!("channel not returned"))?,
        &joined_room.room,
        &session.peer,
        &*session.connection_pool().await,
    );

    update_user_contacts(session.user_id(), &session).await?;
    Ok(())
}

/// Start editing the channel notes
async fn join_channel_buffer(
    request: proto::JoinChannelBuffer,
    response: Response<proto::JoinChannelBuffer>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;
    let channel_id = ChannelId::from_proto(request.channel_id);

    let open_response = db
        .join_channel_buffer(channel_id, session.user_id(), session.connection_id)
        .await?;

    let collaborators = open_response.collaborators.clone();
    response.send(open_response)?;

    let update = UpdateChannelBufferCollaborators {
        channel_id: channel_id.to_proto(),
        collaborators: collaborators.clone(),
    };
    channel_buffer_updated(
        session.connection_id,
        collaborators
            .iter()
            .filter_map(|collaborator| Some(collaborator.peer_id?.into())),
        &update,
        &session.peer,
    );

    Ok(())
}

/// Edit the channel notes
async fn update_channel_buffer(
    request: proto::UpdateChannelBuffer,
    session: Session,
) -> Result<()> {
    let db = session.db().await;
    let channel_id = ChannelId::from_proto(request.channel_id);

    let (collaborators, epoch, version) = db
        .update_channel_buffer(channel_id, session.user_id(), &request.operations)
        .await?;

    channel_buffer_updated(
        session.connection_id,
        collaborators.clone(),
        &proto::UpdateChannelBuffer {
            channel_id: channel_id.to_proto(),
            operations: request.operations,
        },
        &session.peer,
    );

    let pool = &*session.connection_pool().await;

    let non_collaborators =
        pool.channel_connection_ids(channel_id)
            .filter_map(|(connection_id, _)| {
                if collaborators.contains(&connection_id) {
                    None
                } else {
                    Some(connection_id)
                }
            });

    broadcast(None, non_collaborators, |peer_id| {
        session.peer.send(
            peer_id,
            proto::UpdateChannels {
                latest_channel_buffer_versions: vec![proto::ChannelBufferVersion {
                    channel_id: channel_id.to_proto(),
                    epoch: epoch as u64,
                    version: version.clone(),
                }],
                ..Default::default()
            },
        )
    });

    Ok(())
}

/// Rejoin the channel notes after a connection blip
async fn rejoin_channel_buffers(
    request: proto::RejoinChannelBuffers,
    response: Response<proto::RejoinChannelBuffers>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;
    let buffers = db
        .rejoin_channel_buffers(&request.buffers, session.user_id(), session.connection_id)
        .await?;

    for rejoined_buffer in &buffers {
        let collaborators_to_notify = rejoined_buffer
            .buffer
            .collaborators
            .iter()
            .filter_map(|c| Some(c.peer_id?.into()));
        channel_buffer_updated(
            session.connection_id,
            collaborators_to_notify,
            &proto::UpdateChannelBufferCollaborators {
                channel_id: rejoined_buffer.buffer.channel_id,
                collaborators: rejoined_buffer.buffer.collaborators.clone(),
            },
            &session.peer,
        );
    }

    response.send(proto::RejoinChannelBuffersResponse {
        buffers: buffers.into_iter().map(|b| b.buffer).collect(),
    })?;

    Ok(())
}

/// Stop editing the channel notes
async fn leave_channel_buffer(
    request: proto::LeaveChannelBuffer,
    response: Response<proto::LeaveChannelBuffer>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;
    let channel_id = ChannelId::from_proto(request.channel_id);

    let left_buffer = db
        .leave_channel_buffer(channel_id, session.connection_id)
        .await?;

    response.send(Ack {})?;

    channel_buffer_updated(
        session.connection_id,
        left_buffer.connections,
        &proto::UpdateChannelBufferCollaborators {
            channel_id: channel_id.to_proto(),
            collaborators: left_buffer.collaborators,
        },
        &session.peer,
    );

    Ok(())
}

fn channel_buffer_updated<T: EnvelopedMessage>(
    sender_id: ConnectionId,
    collaborators: impl IntoIterator<Item = ConnectionId>,
    message: &T,
    peer: &Peer,
) {
    broadcast(Some(sender_id), collaborators, |peer_id| {
        peer.send(peer_id, message.clone())
    });
}

fn send_notifications(
    connection_pool: &ConnectionPool,
    peer: &Peer,
    notifications: db::NotificationBatch,
) {
    for (user_id, notification) in notifications {
        for connection_id in connection_pool.user_connection_ids(user_id) {
            if let Err(error) = peer.send(
                connection_id,
                proto::AddNotification {
                    notification: Some(notification.clone()),
                },
            ) {
                tracing::error!(
                    "failed to send notification to {:?} {}",
                    connection_id,
                    error
                );
            }
        }
    }
}

/// Send a message to the channel
async fn send_channel_message(
    request: proto::SendChannelMessage,
    response: Response<proto::SendChannelMessage>,
    session: Session,
) -> Result<()> {
    // Validate the message body.
    let body = request.body.trim().to_string();
    if body.len() > MAX_MESSAGE_LEN {
        return Err(anyhow!("message is too long"))?;
    }
    if body.is_empty() {
        return Err(anyhow!("message can't be blank"))?;
    }

    // TODO: adjust mentions if body is trimmed

    let timestamp = OffsetDateTime::now_utc();
    let nonce = request
        .nonce
        .ok_or_else(|| anyhow!("nonce can't be blank"))?;

    let channel_id = ChannelId::from_proto(request.channel_id);
    let CreatedChannelMessage {
        message_id,
        participant_connection_ids,
        notifications,
    } = session
        .db()
        .await
        .create_channel_message(
            channel_id,
            session.user_id(),
            &body,
            &request.mentions,
            timestamp,
            nonce.clone().into(),
            request.reply_to_message_id.map(MessageId::from_proto),
        )
        .await?;

    let message = proto::ChannelMessage {
        sender_id: session.user_id().to_proto(),
        id: message_id.to_proto(),
        body,
        mentions: request.mentions,
        timestamp: timestamp.unix_timestamp() as u64,
        nonce: Some(nonce),
        reply_to_message_id: request.reply_to_message_id,
        edited_at: None,
    };
    broadcast(
        Some(session.connection_id),
        participant_connection_ids.clone(),
        |connection| {
            session.peer.send(
                connection,
                proto::ChannelMessageSent {
                    channel_id: channel_id.to_proto(),
                    message: Some(message.clone()),
                },
            )
        },
    );
    response.send(proto::SendChannelMessageResponse {
        message: Some(message),
    })?;

    let pool = &*session.connection_pool().await;
    let non_participants =
        pool.channel_connection_ids(channel_id)
            .filter_map(|(connection_id, _)| {
                if participant_connection_ids.contains(&connection_id) {
                    None
                } else {
                    Some(connection_id)
                }
            });
    broadcast(None, non_participants, |peer_id| {
        session.peer.send(
            peer_id,
            proto::UpdateChannels {
                latest_channel_message_ids: vec![proto::ChannelMessageId {
                    channel_id: channel_id.to_proto(),
                    message_id: message_id.to_proto(),
                }],
                ..Default::default()
            },
        )
    });
    send_notifications(pool, &session.peer, notifications);

    Ok(())
}

/// Delete a channel message
async fn remove_channel_message(
    request: proto::RemoveChannelMessage,
    response: Response<proto::RemoveChannelMessage>,
    session: Session,
) -> Result<()> {
    let channel_id = ChannelId::from_proto(request.channel_id);
    let message_id = MessageId::from_proto(request.message_id);
    let (connection_ids, existing_notification_ids) = session
        .db()
        .await
        .remove_channel_message(channel_id, message_id, session.user_id())
        .await?;

    broadcast(
        Some(session.connection_id),
        connection_ids,
        move |connection| {
            session.peer.send(connection, request.clone())?;

            for notification_id in &existing_notification_ids {
                session.peer.send(
                    connection,
                    proto::DeleteNotification {
                        notification_id: (*notification_id).to_proto(),
                    },
                )?;
            }

            Ok(())
        },
    );
    response.send(proto::Ack {})?;
    Ok(())
}

async fn update_channel_message(
    request: proto::UpdateChannelMessage,
    response: Response<proto::UpdateChannelMessage>,
    session: Session,
) -> Result<()> {
    let channel_id = ChannelId::from_proto(request.channel_id);
    let message_id = MessageId::from_proto(request.message_id);
    let updated_at = OffsetDateTime::now_utc();
    let UpdatedChannelMessage {
        message_id,
        participant_connection_ids,
        notifications,
        reply_to_message_id,
        timestamp,
        deleted_mention_notification_ids,
        updated_mention_notifications,
    } = session
        .db()
        .await
        .update_channel_message(
            channel_id,
            message_id,
            session.user_id(),
            request.body.as_str(),
            &request.mentions,
            updated_at,
        )
        .await?;

    let nonce = request
        .nonce
        .clone()
        .ok_or_else(|| anyhow!("nonce can't be blank"))?;

    let message = proto::ChannelMessage {
        sender_id: session.user_id().to_proto(),
        id: message_id.to_proto(),
        body: request.body.clone(),
        mentions: request.mentions.clone(),
        timestamp: timestamp.assume_utc().unix_timestamp() as u64,
        nonce: Some(nonce),
        reply_to_message_id: reply_to_message_id.map(|id| id.to_proto()),
        edited_at: Some(updated_at.unix_timestamp() as u64),
    };

    response.send(proto::Ack {})?;

    let pool = &*session.connection_pool().await;
    broadcast(
        Some(session.connection_id),
        participant_connection_ids,
        |connection| {
            session.peer.send(
                connection,
                proto::ChannelMessageUpdate {
                    channel_id: channel_id.to_proto(),
                    message: Some(message.clone()),
                },
            )?;

            for notification_id in &deleted_mention_notification_ids {
                session.peer.send(
                    connection,
                    proto::DeleteNotification {
                        notification_id: (*notification_id).to_proto(),
                    },
                )?;
            }

            for notification in &updated_mention_notifications {
                session.peer.send(
                    connection,
                    proto::UpdateNotification {
                        notification: Some(notification.clone()),
                    },
                )?;
            }

            Ok(())
        },
    );

    send_notifications(pool, &session.peer, notifications);

    Ok(())
}

/// Mark a channel message as read
async fn acknowledge_channel_message(
    request: proto::AckChannelMessage,
    session: Session,
) -> Result<()> {
    let channel_id = ChannelId::from_proto(request.channel_id);
    let message_id = MessageId::from_proto(request.message_id);
    let notifications = session
        .db()
        .await
        .observe_channel_message(channel_id, session.user_id(), message_id)
        .await?;
    send_notifications(
        &*session.connection_pool().await,
        &session.peer,
        notifications,
    );
    Ok(())
}

/// Mark a buffer version as synced
async fn acknowledge_buffer_version(
    request: proto::AckBufferOperation,
    session: Session,
) -> Result<()> {
    let buffer_id = BufferId::from_proto(request.buffer_id);
    session
        .db()
        .await
        .observe_buffer_version(
            buffer_id,
            session.user_id(),
            request.epoch as i32,
            &request.version,
        )
        .await?;
    Ok(())
}

async fn count_language_model_tokens(
    request: proto::CountLanguageModelTokens,
    response: Response<proto::CountLanguageModelTokens>,
    session: Session,
    config: &Config,
) -> Result<()> {
    authorize_access_to_legacy_llm_endpoints(&session).await?;

    let rate_limit: Box<dyn RateLimit> = match session.current_plan(&session.db().await).await? {
        proto::Plan::ZedPro => Box::new(ZedProCountLanguageModelTokensRateLimit),
        proto::Plan::Free | proto::Plan::ZedProTrial => {
            Box::new(FreeCountLanguageModelTokensRateLimit)
        }
    };

    session
        .app_state
        .rate_limiter
        .check(&*rate_limit, session.user_id())
        .await?;

    let result = match proto::LanguageModelProvider::from_i32(request.provider) {
        Some(proto::LanguageModelProvider::Google) => {
            let api_key = config
                .google_ai_api_key
                .as_ref()
                .context("no Google AI API key configured on the server")?;
            google_ai::count_tokens(
                session.http_client.as_ref(),
                google_ai::API_URL,
                api_key,
                serde_json::from_str(&request.request)?,
            )
            .await?
        }
        _ => return Err(anyhow!("unsupported provider"))?,
    };

    response.send(proto::CountLanguageModelTokensResponse {
        token_count: result.total_tokens as u32,
    })?;

    Ok(())
}

struct ZedProCountLanguageModelTokensRateLimit;

impl RateLimit for ZedProCountLanguageModelTokensRateLimit {
    fn capacity(&self) -> usize {
        std::env::var("COUNT_LANGUAGE_MODEL_TOKENS_RATE_LIMIT_PER_HOUR")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(600) // Picked arbitrarily
    }

    fn refill_duration(&self) -> chrono::Duration {
        chrono::Duration::hours(1)
    }

    fn db_name(&self) -> &'static str {
        "zed-pro:count-language-model-tokens"
    }
}

struct FreeCountLanguageModelTokensRateLimit;

impl RateLimit for FreeCountLanguageModelTokensRateLimit {
    fn capacity(&self) -> usize {
        std::env::var("COUNT_LANGUAGE_MODEL_TOKENS_RATE_LIMIT_PER_HOUR_FREE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(600 / 10) // Picked arbitrarily
    }

    fn refill_duration(&self) -> chrono::Duration {
        chrono::Duration::hours(1)
    }

    fn db_name(&self) -> &'static str {
        "free:count-language-model-tokens"
    }
}

struct ZedProComputeEmbeddingsRateLimit;

impl RateLimit for ZedProComputeEmbeddingsRateLimit {
    fn capacity(&self) -> usize {
        std::env::var("EMBED_TEXTS_RATE_LIMIT_PER_HOUR")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5000) // Picked arbitrarily
    }

    fn refill_duration(&self) -> chrono::Duration {
        chrono::Duration::hours(1)
    }

    fn db_name(&self) -> &'static str {
        "zed-pro:compute-embeddings"
    }
}

struct FreeComputeEmbeddingsRateLimit;

impl RateLimit for FreeComputeEmbeddingsRateLimit {
    fn capacity(&self) -> usize {
        std::env::var("EMBED_TEXTS_RATE_LIMIT_PER_HOUR_FREE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5000 / 10) // Picked arbitrarily
    }

    fn refill_duration(&self) -> chrono::Duration {
        chrono::Duration::hours(1)
    }

    fn db_name(&self) -> &'static str {
        "free:compute-embeddings"
    }
}

async fn compute_embeddings(
    request: proto::ComputeEmbeddings,
    response: Response<proto::ComputeEmbeddings>,
    session: Session,
    api_key: Option<Arc<str>>,
) -> Result<()> {
    let api_key = api_key.context("no OpenAI API key configured on the server")?;
    authorize_access_to_legacy_llm_endpoints(&session).await?;

    let rate_limit: Box<dyn RateLimit> = match session.current_plan(&session.db().await).await? {
        proto::Plan::ZedPro => Box::new(ZedProComputeEmbeddingsRateLimit),
        proto::Plan::Free | proto::Plan::ZedProTrial => Box::new(FreeComputeEmbeddingsRateLimit),
    };

    session
        .app_state
        .rate_limiter
        .check(&*rate_limit, session.user_id())
        .await?;

    let embeddings = match request.model.as_str() {
        "openai/text-embedding-3-small" => {
            open_ai::embed(
                session.http_client.as_ref(),
                OPEN_AI_API_URL,
                &api_key,
                OpenAiEmbeddingModel::TextEmbedding3Small,
                request.texts.iter().map(|text| text.as_str()),
            )
            .await?
        }
        provider => return Err(anyhow!("unsupported embedding provider {:?}", provider))?,
    };

    let embeddings = request
        .texts
        .iter()
        .map(|text| {
            let mut hasher = sha2::Sha256::new();
            hasher.update(text.as_bytes());
            let result = hasher.finalize();
            result.to_vec()
        })
        .zip(
            embeddings
                .data
                .into_iter()
                .map(|embedding| embedding.embedding),
        )
        .collect::<HashMap<_, _>>();

    let db = session.db().await;
    db.save_embeddings(&request.model, &embeddings)
        .await
        .context("failed to save embeddings")
        .trace_err();

    response.send(proto::ComputeEmbeddingsResponse {
        embeddings: embeddings
            .into_iter()
            .map(|(digest, dimensions)| proto::Embedding { digest, dimensions })
            .collect(),
    })?;
    Ok(())
}

async fn get_cached_embeddings(
    request: proto::GetCachedEmbeddings,
    response: Response<proto::GetCachedEmbeddings>,
    session: Session,
) -> Result<()> {
    authorize_access_to_legacy_llm_endpoints(&session).await?;

    let db = session.db().await;
    let embeddings = db.get_embeddings(&request.model, &request.digests).await?;

    response.send(proto::GetCachedEmbeddingsResponse {
        embeddings: embeddings
            .into_iter()
            .map(|(digest, dimensions)| proto::Embedding { digest, dimensions })
            .collect(),
    })?;
    Ok(())
}

/// This is leftover from before the LLM service.
///
/// The endpoints protected by this check will be moved there eventually.
async fn authorize_access_to_legacy_llm_endpoints(session: &Session) -> Result<(), Error> {
    if session.is_staff() {
        Ok(())
    } else {
        Err(anyhow!("permission denied"))?
    }
}

/// Get a Supermaven API key for the user
async fn get_supermaven_api_key(
    _request: proto::GetSupermavenApiKey,
    response: Response<proto::GetSupermavenApiKey>,
    session: Session,
) -> Result<()> {
    let user_id: String = session.user_id().to_string();
    if !session.is_staff() {
        return Err(anyhow!("supermaven not enabled for this account"))?;
    }

    let email = session
        .email()
        .ok_or_else(|| anyhow!("user must have an email"))?;

    let supermaven_admin_api = session
        .supermaven_client
        .as_ref()
        .ok_or_else(|| anyhow!("supermaven not configured"))?;

    let result = supermaven_admin_api
        .try_get_or_create_user(CreateExternalUserRequest { id: user_id, email })
        .await?;

    response.send(proto::GetSupermavenApiKeyResponse {
        api_key: result.api_key,
    })?;

    Ok(())
}

/// Start receiving chat updates for a channel
async fn join_channel_chat(
    request: proto::JoinChannelChat,
    response: Response<proto::JoinChannelChat>,
    session: Session,
) -> Result<()> {
    let channel_id = ChannelId::from_proto(request.channel_id);

    let db = session.db().await;
    db.join_channel_chat(channel_id, session.connection_id, session.user_id())
        .await?;
    let messages = db
        .get_channel_messages(channel_id, session.user_id(), MESSAGE_COUNT_PER_PAGE, None)
        .await?;
    response.send(proto::JoinChannelChatResponse {
        done: messages.len() < MESSAGE_COUNT_PER_PAGE,
        messages,
    })?;
    Ok(())
}

/// Stop receiving chat updates for a channel
async fn leave_channel_chat(request: proto::LeaveChannelChat, session: Session) -> Result<()> {
    let channel_id = ChannelId::from_proto(request.channel_id);
    session
        .db()
        .await
        .leave_channel_chat(channel_id, session.connection_id, session.user_id())
        .await?;
    Ok(())
}

/// Retrieve the chat history for a channel
async fn get_channel_messages(
    request: proto::GetChannelMessages,
    response: Response<proto::GetChannelMessages>,
    session: Session,
) -> Result<()> {
    let channel_id = ChannelId::from_proto(request.channel_id);
    let messages = session
        .db()
        .await
        .get_channel_messages(
            channel_id,
            session.user_id(),
            MESSAGE_COUNT_PER_PAGE,
            Some(MessageId::from_proto(request.before_message_id)),
        )
        .await?;
    response.send(proto::GetChannelMessagesResponse {
        done: messages.len() < MESSAGE_COUNT_PER_PAGE,
        messages,
    })?;
    Ok(())
}

/// Retrieve specific chat messages
async fn get_channel_messages_by_id(
    request: proto::GetChannelMessagesById,
    response: Response<proto::GetChannelMessagesById>,
    session: Session,
) -> Result<()> {
    let message_ids = request
        .message_ids
        .iter()
        .map(|id| MessageId::from_proto(*id))
        .collect::<Vec<_>>();
    let messages = session
        .db()
        .await
        .get_channel_messages_by_id(session.user_id(), &message_ids)
        .await?;
    response.send(proto::GetChannelMessagesResponse {
        done: messages.len() < MESSAGE_COUNT_PER_PAGE,
        messages,
    })?;
    Ok(())
}

/// Retrieve the current users notifications
async fn get_notifications(
    request: proto::GetNotifications,
    response: Response<proto::GetNotifications>,
    session: Session,
) -> Result<()> {
    let notifications = session
        .db()
        .await
        .get_notifications(
            session.user_id(),
            NOTIFICATION_COUNT_PER_PAGE,
            request.before_id.map(db::NotificationId::from_proto),
        )
        .await?;
    response.send(proto::GetNotificationsResponse {
        done: notifications.len() < NOTIFICATION_COUNT_PER_PAGE,
        notifications,
    })?;
    Ok(())
}

/// Mark notifications as read
async fn mark_notification_as_read(
    request: proto::MarkNotificationRead,
    response: Response<proto::MarkNotificationRead>,
    session: Session,
) -> Result<()> {
    let database = &session.db().await;
    let notifications = database
        .mark_notification_as_read_by_id(
            session.user_id(),
            NotificationId::from_proto(request.notification_id),
        )
        .await?;
    send_notifications(
        &*session.connection_pool().await,
        &session.peer,
        notifications,
    );
    response.send(proto::Ack {})?;
    Ok(())
}

/// Get the current users information
async fn get_private_user_info(
    _request: proto::GetPrivateUserInfo,
    response: Response<proto::GetPrivateUserInfo>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;

    let metrics_id = db.get_user_metrics_id(session.user_id()).await?;
    let user = db
        .get_user_by_id(session.user_id())
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;
    let flags = db.get_user_flags(session.user_id()).await?;

    response.send(proto::GetPrivateUserInfoResponse {
        metrics_id,
        staff: user.admin,
        flags,
        accepted_tos_at: user.accepted_tos_at.map(|t| t.and_utc().timestamp() as u64),
    })?;
    Ok(())
}

/// Accept the terms of service (tos) on behalf of the current user
async fn accept_terms_of_service(
    _request: proto::AcceptTermsOfService,
    response: Response<proto::AcceptTermsOfService>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;

    let accepted_tos_at = Utc::now();
    db.set_user_accepted_tos_at(session.user_id(), Some(accepted_tos_at.naive_utc()))
        .await?;

    response.send(proto::AcceptTermsOfServiceResponse {
        accepted_tos_at: accepted_tos_at.timestamp() as u64,
    })?;
    Ok(())
}

/// The minimum account age an account must have in order to use the LLM service.
pub const MIN_ACCOUNT_AGE_FOR_LLM_USE: chrono::Duration = chrono::Duration::days(30);

async fn get_llm_api_token(
    _request: proto::GetLlmToken,
    response: Response<proto::GetLlmToken>,
    session: Session,
) -> Result<()> {
    let db = session.db().await;

    let flags = db.get_user_flags(session.user_id()).await?;
    let has_language_models_feature_flag = flags.iter().any(|flag| flag == "language-models");

    if !session.is_staff() && !has_language_models_feature_flag {
        Err(anyhow!("permission denied"))?
    }

    let user_id = session.user_id();
    let user = db
        .get_user_by_id(user_id)
        .await?
        .ok_or_else(|| anyhow!("user {} not found", user_id))?;

    if user.accepted_tos_at.is_none() {
        Err(anyhow!("terms of service not accepted"))?
    }

    let has_legacy_llm_subscription = session.has_llm_subscription(&db).await?;
    let billing_subscription = db.get_active_billing_subscription(user.id).await?;
    let billing_preferences = db.get_billing_preferences(user.id).await?;

    let token = LlmTokenClaims::create(
        &user,
        session.is_staff(),
        billing_preferences,
        &flags,
        has_legacy_llm_subscription,
        session.current_plan(&db).await?,
        billing_subscription,
        session.system_id.clone(),
        &session.app_state.config,
    )?;
    response.send(proto::GetLlmTokenResponse { token })?;
    Ok(())
}

fn to_axum_message(message: TungsteniteMessage) -> anyhow::Result<AxumMessage> {
    let message = match message {
        TungsteniteMessage::Text(payload) => AxumMessage::Text(payload.as_str().to_string()),
        TungsteniteMessage::Binary(payload) => AxumMessage::Binary(payload.into()),
        TungsteniteMessage::Ping(payload) => AxumMessage::Ping(payload.into()),
        TungsteniteMessage::Pong(payload) => AxumMessage::Pong(payload.into()),
        TungsteniteMessage::Close(frame) => AxumMessage::Close(frame.map(|frame| AxumCloseFrame {
            code: frame.code.into(),
            reason: frame.reason.as_str().to_owned().into(),
        })),
        // We should never receive a frame while reading the message, according
        // to the `tungstenite` maintainers:
        //
        // > It cannot occur when you read messages from the WebSocket, but it
        // > can be used when you want to send the raw frames (e.g. you want to
        // > send the frames to the WebSocket without composing the full message first).
        // >
        // > — https://github.com/snapview/tungstenite-rs/issues/268
        TungsteniteMessage::Frame(_) => {
            bail!("received an unexpected frame while reading the message")
        }
    };

    Ok(message)
}

fn to_tungstenite_message(message: AxumMessage) -> TungsteniteMessage {
    match message {
        AxumMessage::Text(payload) => TungsteniteMessage::Text(payload.into()),
        AxumMessage::Binary(payload) => TungsteniteMessage::Binary(payload.into()),
        AxumMessage::Ping(payload) => TungsteniteMessage::Ping(payload.into()),
        AxumMessage::Pong(payload) => TungsteniteMessage::Pong(payload.into()),
        AxumMessage::Close(frame) => {
            TungsteniteMessage::Close(frame.map(|frame| TungsteniteCloseFrame {
                code: frame.code.into(),
                reason: frame.reason.as_ref().into(),
            }))
        }
    }
}

fn notify_membership_updated(
    connection_pool: &mut ConnectionPool,
    result: MembershipUpdated,
    user_id: UserId,
    peer: &Peer,
) {
    for membership in &result.new_channels.channel_memberships {
        connection_pool.subscribe_to_channel(user_id, membership.channel_id, membership.role)
    }
    for channel_id in &result.removed_channels {
        connection_pool.unsubscribe_from_channel(&user_id, channel_id)
    }

    let user_channels_update = proto::UpdateUserChannels {
        channel_memberships: result
            .new_channels
            .channel_memberships
            .iter()
            .map(|cm| proto::ChannelMembership {
                channel_id: cm.channel_id.to_proto(),
                role: cm.role.into(),
            })
            .collect(),
        ..Default::default()
    };

    let mut update = build_channels_update(result.new_channels);
    update.delete_channels = result
        .removed_channels
        .into_iter()
        .map(|id| id.to_proto())
        .collect();
    update.remove_channel_invitations = vec![result.channel_id.to_proto()];

    for connection_id in connection_pool.user_connection_ids(user_id) {
        peer.send(connection_id, user_channels_update.clone())
            .trace_err();
        peer.send(connection_id, update.clone()).trace_err();
    }
}

fn build_update_user_channels(channels: &ChannelsForUser) -> proto::UpdateUserChannels {
    proto::UpdateUserChannels {
        channel_memberships: channels
            .channel_memberships
            .iter()
            .map(|m| proto::ChannelMembership {
                channel_id: m.channel_id.to_proto(),
                role: m.role.into(),
            })
            .collect(),
        observed_channel_buffer_version: channels.observed_buffer_versions.clone(),
        observed_channel_message_id: channels.observed_channel_messages.clone(),
    }
}

fn build_channels_update(channels: ChannelsForUser) -> proto::UpdateChannels {
    let mut update = proto::UpdateChannels::default();

    for channel in channels.channels {
        update.channels.push(channel.to_proto());
    }

    update.latest_channel_buffer_versions = channels.latest_buffer_versions;
    update.latest_channel_message_ids = channels.latest_channel_messages;

    for (channel_id, participants) in channels.channel_participants {
        update
            .channel_participants
            .push(proto::ChannelParticipants {
                channel_id: channel_id.to_proto(),
                participant_user_ids: participants.into_iter().map(|id| id.to_proto()).collect(),
            });
    }

    for channel in channels.invited_channels {
        update.channel_invitations.push(channel.to_proto());
    }

    update
}

fn build_initial_contacts_update(
    contacts: Vec<db::Contact>,
    pool: &ConnectionPool,
) -> proto::UpdateContacts {
    let mut update = proto::UpdateContacts::default();

    for contact in contacts {
        match contact {
            db::Contact::Accepted { user_id, busy } => {
                update.contacts.push(contact_for_user(user_id, busy, pool));
            }
            db::Contact::Outgoing { user_id } => update.outgoing_requests.push(user_id.to_proto()),
            db::Contact::Incoming { user_id } => {
                update
                    .incoming_requests
                    .push(proto::IncomingContactRequest {
                        requester_id: user_id.to_proto(),
                    })
            }
        }
    }

    update
}

fn contact_for_user(user_id: UserId, busy: bool, pool: &ConnectionPool) -> proto::Contact {
    proto::Contact {
        user_id: user_id.to_proto(),
        online: pool.is_user_online(user_id),
        busy,
    }
}

fn room_updated(room: &proto::Room, peer: &Peer) {
    broadcast(
        None,
        room.participants
            .iter()
            .filter_map(|participant| Some(participant.peer_id?.into())),
        |peer_id| {
            peer.send(
                peer_id,
                proto::RoomUpdated {
                    room: Some(room.clone()),
                },
            )
        },
    );
}

fn channel_updated(
    channel: &db::channel::Model,
    room: &proto::Room,
    peer: &Peer,
    pool: &ConnectionPool,
) {
    let participants = room
        .participants
        .iter()
        .map(|p| p.user_id)
        .collect::<Vec<_>>();

    broadcast(
        None,
        pool.channel_connection_ids(channel.root_id())
            .filter_map(|(channel_id, role)| {
                role.can_see_channel(channel.visibility)
                    .then_some(channel_id)
            }),
        |peer_id| {
            peer.send(
                peer_id,
                proto::UpdateChannels {
                    channel_participants: vec![proto::ChannelParticipants {
                        channel_id: channel.id.to_proto(),
                        participant_user_ids: participants.clone(),
                    }],
                    ..Default::default()
                },
            )
        },
    );
}

async fn update_user_contacts(user_id: UserId, session: &Session) -> Result<()> {
    let db = session.db().await;

    let contacts = db.get_contacts(user_id).await?;
    let busy = db.is_user_busy(user_id).await?;

    let pool = session.connection_pool().await;
    let updated_contact = contact_for_user(user_id, busy, &pool);
    for contact in contacts {
        if let db::Contact::Accepted {
            user_id: contact_user_id,
            ..
        } = contact
        {
            for contact_conn_id in pool.user_connection_ids(contact_user_id) {
                session
                    .peer
                    .send(
                        contact_conn_id,
                        proto::UpdateContacts {
                            contacts: vec![updated_contact.clone()],
                            remove_contacts: Default::default(),
                            incoming_requests: Default::default(),
                            remove_incoming_requests: Default::default(),
                            outgoing_requests: Default::default(),
                            remove_outgoing_requests: Default::default(),
                        },
                    )
                    .trace_err();
            }
        }
    }
    Ok(())
}

async fn leave_room_for_session(session: &Session, connection_id: ConnectionId) -> Result<()> {
    let mut contacts_to_update = HashSet::default();

    let room_id;
    let canceled_calls_to_user_ids;
    let livekit_room;
    let delete_livekit_room;
    let room;
    let channel;

    if let Some(mut left_room) = session.db().await.leave_room(connection_id).await? {
        contacts_to_update.insert(session.user_id());

        for project in left_room.left_projects.values() {
            project_left(project, session);
        }

        room_id = RoomId::from_proto(left_room.room.id);
        canceled_calls_to_user_ids = mem::take(&mut left_room.canceled_calls_to_user_ids);
        livekit_room = mem::take(&mut left_room.room.livekit_room);
        delete_livekit_room = left_room.deleted;
        room = mem::take(&mut left_room.room);
        channel = mem::take(&mut left_room.channel);

        room_updated(&room, &session.peer);
    } else {
        return Ok(());
    }

    if let Some(channel) = channel {
        channel_updated(
            &channel,
            &room,
            &session.peer,
            &*session.connection_pool().await,
        );
    }

    {
        let pool = session.connection_pool().await;
        for canceled_user_id in canceled_calls_to_user_ids {
            for connection_id in pool.user_connection_ids(canceled_user_id) {
                session
                    .peer
                    .send(
                        connection_id,
                        proto::CallCanceled {
                            room_id: room_id.to_proto(),
                        },
                    )
                    .trace_err();
            }
            contacts_to_update.insert(canceled_user_id);
        }
    }

    for contact_user_id in contacts_to_update {
        update_user_contacts(contact_user_id, session).await?;
    }

    if let Some(live_kit) = session.app_state.livekit_client.as_ref() {
        live_kit
            .remove_participant(livekit_room.clone(), session.user_id().to_string())
            .await
            .trace_err();

        if delete_livekit_room {
            live_kit.delete_room(livekit_room).await.trace_err();
        }
    }

    Ok(())
}

async fn leave_channel_buffers_for_session(session: &Session) -> Result<()> {
    let left_channel_buffers = session
        .db()
        .await
        .leave_channel_buffers(session.connection_id)
        .await?;

    for left_buffer in left_channel_buffers {
        channel_buffer_updated(
            session.connection_id,
            left_buffer.connections,
            &proto::UpdateChannelBufferCollaborators {
                channel_id: left_buffer.channel_id.to_proto(),
                collaborators: left_buffer.collaborators,
            },
            &session.peer,
        );
    }

    Ok(())
}

fn project_left(project: &db::LeftProject, session: &Session) {
    for connection_id in &project.connection_ids {
        if project.should_unshare {
            session
                .peer
                .send(
                    *connection_id,
                    proto::UnshareProject {
                        project_id: project.id.to_proto(),
                    },
                )
                .trace_err();
        } else {
            session
                .peer
                .send(
                    *connection_id,
                    proto::RemoveProjectCollaborator {
                        project_id: project.id.to_proto(),
                        peer_id: Some(session.connection_id.into()),
                    },
                )
                .trace_err();
        }
    }
}

pub trait ResultExt {
    type Ok;

    fn trace_err(self) -> Option<Self::Ok>;
}

impl<T, E> ResultExt for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Ok = T;

    #[track_caller]
    fn trace_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                tracing::error!("{:?}", error);
                None
            }
        }
    }
}
