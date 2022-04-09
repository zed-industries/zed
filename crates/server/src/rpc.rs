mod store;

use super::{
    auth::process_auth_header,
    db::{ChannelId, MessageId, UserId},
    AppState,
};
use anyhow::anyhow;
use async_io::Timer;
use async_std::task;
use async_tungstenite::{tungstenite::protocol::Role, WebSocketStream};
use collections::{HashMap, HashSet};
use futures::{channel::mpsc, future::BoxFuture, FutureExt, SinkExt, StreamExt};
use log::{as_debug, as_display};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use rpc::{
    proto::{self, AnyTypedEnvelope, EntityMessage, EnvelopedMessage, RequestMessage},
    Connection, ConnectionId, Peer, TypedEnvelope,
};
use sha1::{Digest as _, Sha1};
use std::{
    any::TypeId,
    future::Future,
    sync::Arc,
    time::{Duration, Instant},
};
use store::{Store, Worktree};
use surf::StatusCode;
use tide::{
    http::headers::{HeaderName, CONNECTION, UPGRADE},
    Request, Response,
};
use time::OffsetDateTime;

type MessageHandler = Box<
    dyn Send
        + Sync
        + Fn(Arc<Server>, Box<dyn AnyTypedEnvelope>) -> BoxFuture<'static, tide::Result<()>>,
>;

pub struct Server {
    peer: Arc<Peer>,
    store: RwLock<Store>,
    app_state: Arc<AppState>,
    handlers: HashMap<TypeId, MessageHandler>,
    notifications: Option<mpsc::UnboundedSender<()>>,
}

pub trait Executor: Send + Clone {
    type Timer: Send + Future;
    fn spawn_detached<F: 'static + Send + Future<Output = ()>>(&self, future: F);
    fn timer(&self, duration: Duration) -> Self::Timer;
}

#[derive(Clone)]
pub struct RealExecutor;

const MESSAGE_COUNT_PER_PAGE: usize = 100;
const MAX_MESSAGE_LEN: usize = 1024;

impl Server {
    pub fn new(
        app_state: Arc<AppState>,
        peer: Arc<Peer>,
        notifications: Option<mpsc::UnboundedSender<()>>,
    ) -> Arc<Self> {
        let mut server = Self {
            peer,
            app_state,
            store: Default::default(),
            handlers: Default::default(),
            notifications,
        };

        server
            .add_request_handler(Server::ping)
            .add_request_handler(Server::register_project)
            .add_message_handler(Server::unregister_project)
            .add_request_handler(Server::share_project)
            .add_message_handler(Server::unshare_project)
            .add_request_handler(Server::join_project)
            .add_message_handler(Server::leave_project)
            .add_request_handler(Server::register_worktree)
            .add_message_handler(Server::unregister_worktree)
            .add_request_handler(Server::update_worktree)
            .add_message_handler(Server::start_language_server)
            .add_message_handler(Server::update_language_server)
            .add_message_handler(Server::update_diagnostic_summary)
            .add_request_handler(Server::forward_project_request::<proto::GetDefinition>)
            .add_request_handler(Server::forward_project_request::<proto::GetReferences>)
            .add_request_handler(Server::forward_project_request::<proto::SearchProject>)
            .add_request_handler(Server::forward_project_request::<proto::GetDocumentHighlights>)
            .add_request_handler(Server::forward_project_request::<proto::GetProjectSymbols>)
            .add_request_handler(Server::forward_project_request::<proto::OpenBufferForSymbol>)
            .add_request_handler(Server::forward_project_request::<proto::OpenBufferById>)
            .add_request_handler(Server::forward_project_request::<proto::OpenBufferByPath>)
            .add_request_handler(Server::forward_project_request::<proto::GetCompletions>)
            .add_request_handler(
                Server::forward_project_request::<proto::ApplyCompletionAdditionalEdits>,
            )
            .add_request_handler(Server::forward_project_request::<proto::GetCodeActions>)
            .add_request_handler(Server::forward_project_request::<proto::ApplyCodeAction>)
            .add_request_handler(Server::forward_project_request::<proto::PrepareRename>)
            .add_request_handler(Server::forward_project_request::<proto::PerformRename>)
            .add_request_handler(Server::forward_project_request::<proto::ReloadBuffers>)
            .add_request_handler(Server::forward_project_request::<proto::FormatBuffers>)
            .add_request_handler(Server::update_buffer)
            .add_message_handler(Server::update_buffer_file)
            .add_message_handler(Server::buffer_reloaded)
            .add_message_handler(Server::buffer_saved)
            .add_request_handler(Server::save_buffer)
            .add_request_handler(Server::get_channels)
            .add_request_handler(Server::get_users)
            .add_request_handler(Server::join_channel)
            .add_message_handler(Server::leave_channel)
            .add_request_handler(Server::send_channel_message)
            .add_request_handler(Server::follow)
            .add_message_handler(Server::unfollow)
            .add_message_handler(Server::update_followers)
            .add_request_handler(Server::get_channel_messages);

        Arc::new(server)
    }

    fn add_message_handler<F, Fut, M>(&mut self, handler: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(Arc<Self>, TypedEnvelope<M>) -> Fut,
        Fut: 'static + Send + Future<Output = tide::Result<()>>,
        M: EnvelopedMessage,
    {
        let prev_handler = self.handlers.insert(
            TypeId::of::<M>(),
            Box::new(move |server, envelope| {
                let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                (handler)(server, *envelope).boxed()
            }),
        );
        if prev_handler.is_some() {
            panic!("registered a handler for the same message twice");
        }
        self
    }

    fn add_request_handler<F, Fut, M>(&mut self, handler: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(Arc<Self>, TypedEnvelope<M>) -> Fut,
        Fut: 'static + Send + Future<Output = tide::Result<M::Response>>,
        M: RequestMessage,
    {
        self.add_message_handler(move |server, envelope| {
            let receipt = envelope.receipt();
            let response = (handler)(server.clone(), envelope);
            async move {
                match response.await {
                    Ok(response) => {
                        server.peer.respond(receipt, response)?;
                        Ok(())
                    }
                    Err(error) => {
                        server.peer.respond_with_error(
                            receipt,
                            proto::Error {
                                message: error.to_string(),
                            },
                        )?;
                        Err(error)
                    }
                }
            }
        })
    }

    pub fn handle_connection<E: Executor>(
        self: &Arc<Self>,
        connection: Connection,
        addr: String,
        user_id: UserId,
        mut send_connection_id: Option<mpsc::Sender<ConnectionId>>,
        executor: E,
    ) -> impl Future<Output = ()> {
        let mut this = self.clone();
        async move {
            let (connection_id, handle_io, mut incoming_rx) = this
                .peer
                .add_connection(connection, {
                    let executor = executor.clone();
                    move |duration| {
                        let timer = executor.timer(duration);
                        async move {
                            timer.await;
                        }
                    }
                })
                .await;

            if let Some(send_connection_id) = send_connection_id.as_mut() {
                let _ = send_connection_id.send(connection_id).await;
            }

            this.state_mut().add_connection(connection_id, user_id);
            if let Err(err) = this.update_contacts_for_users(&[user_id]) {
                log::error!("error updating contacts for {:?}: {}", user_id, err);
            }

            let handle_io = handle_io.fuse();
            futures::pin_mut!(handle_io);
            loop {
                let next_message = incoming_rx.next().fuse();
                futures::pin_mut!(next_message);
                futures::select_biased! {
                    result = handle_io => {
                        if let Err(err) = result {
                            log::error!("error handling rpc connection {:?} - {:?}", addr, err);
                        }
                        break;
                    }
                    message = next_message => {
                        if let Some(message) = message {
                            let start_time = Instant::now();
                            let type_name = message.payload_type_name();
                            log::info!(connection_id = connection_id.0, type_name = type_name; "rpc message received");
                            if let Some(handler) = this.handlers.get(&message.payload_type_id()) {
                                let notifications = this.notifications.clone();
                                let is_background = message.is_background();
                                let handle_message = (handler)(this.clone(), message);
                                let handle_message = async move {
                                    if let Err(err) = handle_message.await {
                                        log::error!(connection_id = connection_id.0, type = type_name, error = as_display!(err); "rpc message error");
                                    } else {
                                        log::info!(connection_id = connection_id.0, type = type_name, duration = as_debug!(start_time.elapsed()); "rpc message handled");
                                    }
                                    if let Some(mut notifications) = notifications {
                                        let _ = notifications.send(()).await;
                                    }
                                };
                                if is_background {
                                    executor.spawn_detached(handle_message);
                                } else {
                                    handle_message.await;
                                }
                            } else {
                                log::warn!("unhandled message: {}", type_name);
                            }
                        } else {
                            log::info!(address = as_debug!(addr); "rpc connection closed");
                            break;
                        }
                    }
                }
            }

            if let Err(err) = this.sign_out(connection_id).await {
                log::error!("error signing out connection {:?} - {:?}", addr, err);
            }
        }
    }

    async fn sign_out(self: &mut Arc<Self>, connection_id: ConnectionId) -> tide::Result<()> {
        self.peer.disconnect(connection_id);
        let removed_connection = self.state_mut().remove_connection(connection_id)?;

        for (project_id, project) in removed_connection.hosted_projects {
            if let Some(share) = project.share {
                broadcast(
                    connection_id,
                    share.guests.keys().copied().collect(),
                    |conn_id| {
                        self.peer
                            .send(conn_id, proto::UnshareProject { project_id })
                    },
                )?;
            }
        }

        for (project_id, peer_ids) in removed_connection.guest_project_ids {
            broadcast(connection_id, peer_ids, |conn_id| {
                self.peer.send(
                    conn_id,
                    proto::RemoveProjectCollaborator {
                        project_id,
                        peer_id: connection_id.0,
                    },
                )
            })?;
        }

        self.update_contacts_for_users(removed_connection.contact_ids.iter())?;
        Ok(())
    }

    async fn ping(self: Arc<Server>, _: TypedEnvelope<proto::Ping>) -> tide::Result<proto::Ack> {
        Ok(proto::Ack {})
    }

    async fn register_project(
        mut self: Arc<Server>,
        request: TypedEnvelope<proto::RegisterProject>,
    ) -> tide::Result<proto::RegisterProjectResponse> {
        let project_id = {
            let mut state = self.state_mut();
            let user_id = state.user_id_for_connection(request.sender_id)?;
            state.register_project(request.sender_id, user_id)
        };
        Ok(proto::RegisterProjectResponse { project_id })
    }

    async fn unregister_project(
        mut self: Arc<Server>,
        request: TypedEnvelope<proto::UnregisterProject>,
    ) -> tide::Result<()> {
        let project = self
            .state_mut()
            .unregister_project(request.payload.project_id, request.sender_id)?;
        self.update_contacts_for_users(project.authorized_user_ids().iter())?;
        Ok(())
    }

    async fn share_project(
        mut self: Arc<Server>,
        request: TypedEnvelope<proto::ShareProject>,
    ) -> tide::Result<proto::Ack> {
        self.state_mut()
            .share_project(request.payload.project_id, request.sender_id);
        Ok(proto::Ack {})
    }

    async fn unshare_project(
        mut self: Arc<Server>,
        request: TypedEnvelope<proto::UnshareProject>,
    ) -> tide::Result<()> {
        let project_id = request.payload.project_id;
        let project = self
            .state_mut()
            .unshare_project(project_id, request.sender_id)?;

        broadcast(request.sender_id, project.connection_ids, |conn_id| {
            self.peer
                .send(conn_id, proto::UnshareProject { project_id })
        })?;
        self.update_contacts_for_users(&project.authorized_user_ids)?;
        Ok(())
    }

    async fn join_project(
        mut self: Arc<Server>,
        request: TypedEnvelope<proto::JoinProject>,
    ) -> tide::Result<proto::JoinProjectResponse> {
        let project_id = request.payload.project_id;

        let user_id = self.state().user_id_for_connection(request.sender_id)?;
        let (response, connection_ids, contact_user_ids) = self
            .state_mut()
            .join_project(request.sender_id, user_id, project_id)
            .and_then(|joined| {
                let share = joined.project.share()?;
                let peer_count = share.guests.len();
                let mut collaborators = Vec::with_capacity(peer_count);
                collaborators.push(proto::Collaborator {
                    peer_id: joined.project.host_connection_id.0,
                    replica_id: 0,
                    user_id: joined.project.host_user_id.to_proto(),
                });
                let worktrees = share
                    .worktrees
                    .iter()
                    .filter_map(|(id, shared_worktree)| {
                        let worktree = joined.project.worktrees.get(&id)?;
                        Some(proto::Worktree {
                            id: *id,
                            root_name: worktree.root_name.clone(),
                            entries: shared_worktree.entries.values().cloned().collect(),
                            diagnostic_summaries: shared_worktree
                                .diagnostic_summaries
                                .values()
                                .cloned()
                                .collect(),
                            visible: worktree.visible,
                        })
                    })
                    .collect();
                for (peer_conn_id, (peer_replica_id, peer_user_id)) in &share.guests {
                    if *peer_conn_id != request.sender_id {
                        collaborators.push(proto::Collaborator {
                            peer_id: peer_conn_id.0,
                            replica_id: *peer_replica_id as u32,
                            user_id: peer_user_id.to_proto(),
                        });
                    }
                }
                let response = proto::JoinProjectResponse {
                    worktrees,
                    replica_id: joined.replica_id as u32,
                    collaborators,
                    language_servers: joined.project.language_servers.clone(),
                };
                let connection_ids = joined.project.connection_ids();
                let contact_user_ids = joined.project.authorized_user_ids();
                Ok((response, connection_ids, contact_user_ids))
            })?;

        broadcast(request.sender_id, connection_ids, |conn_id| {
            self.peer.send(
                conn_id,
                proto::AddProjectCollaborator {
                    project_id,
                    collaborator: Some(proto::Collaborator {
                        peer_id: request.sender_id.0,
                        replica_id: response.replica_id,
                        user_id: user_id.to_proto(),
                    }),
                },
            )
        })?;
        self.update_contacts_for_users(&contact_user_ids)?;
        Ok(response)
    }

    async fn leave_project(
        mut self: Arc<Server>,
        request: TypedEnvelope<proto::LeaveProject>,
    ) -> tide::Result<()> {
        let sender_id = request.sender_id;
        let project_id = request.payload.project_id;
        let worktree = self.state_mut().leave_project(sender_id, project_id)?;

        broadcast(sender_id, worktree.connection_ids, |conn_id| {
            self.peer.send(
                conn_id,
                proto::RemoveProjectCollaborator {
                    project_id,
                    peer_id: sender_id.0,
                },
            )
        })?;
        self.update_contacts_for_users(&worktree.authorized_user_ids)?;

        Ok(())
    }

    async fn register_worktree(
        mut self: Arc<Server>,
        request: TypedEnvelope<proto::RegisterWorktree>,
    ) -> tide::Result<proto::Ack> {
        let host_user_id = self.state().user_id_for_connection(request.sender_id)?;

        let mut contact_user_ids = HashSet::default();
        contact_user_ids.insert(host_user_id);
        for github_login in &request.payload.authorized_logins {
            let contact_user_id = self.app_state.db.create_user(github_login, false).await?;
            contact_user_ids.insert(contact_user_id);
        }

        let contact_user_ids = contact_user_ids.into_iter().collect::<Vec<_>>();
        let guest_connection_ids;
        {
            let mut state = self.state_mut();
            guest_connection_ids = state
                .read_project(request.payload.project_id, request.sender_id)?
                .guest_connection_ids();
            state.register_worktree(
                request.payload.project_id,
                request.payload.worktree_id,
                request.sender_id,
                Worktree {
                    authorized_user_ids: contact_user_ids.clone(),
                    root_name: request.payload.root_name.clone(),
                    visible: request.payload.visible,
                },
            )?;
        }
        broadcast(request.sender_id, guest_connection_ids, |connection_id| {
            self.peer
                .forward_send(request.sender_id, connection_id, request.payload.clone())
        })?;
        self.update_contacts_for_users(&contact_user_ids)?;
        Ok(proto::Ack {})
    }

    async fn unregister_worktree(
        mut self: Arc<Server>,
        request: TypedEnvelope<proto::UnregisterWorktree>,
    ) -> tide::Result<()> {
        let project_id = request.payload.project_id;
        let worktree_id = request.payload.worktree_id;
        let (worktree, guest_connection_ids) =
            self.state_mut()
                .unregister_worktree(project_id, worktree_id, request.sender_id)?;
        broadcast(request.sender_id, guest_connection_ids, |conn_id| {
            self.peer.send(
                conn_id,
                proto::UnregisterWorktree {
                    project_id,
                    worktree_id,
                },
            )
        })?;
        self.update_contacts_for_users(&worktree.authorized_user_ids)?;
        Ok(())
    }

    async fn update_worktree(
        mut self: Arc<Server>,
        request: TypedEnvelope<proto::UpdateWorktree>,
    ) -> tide::Result<proto::Ack> {
        let connection_ids = self.state_mut().update_worktree(
            request.sender_id,
            request.payload.project_id,
            request.payload.worktree_id,
            &request.payload.removed_entries,
            &request.payload.updated_entries,
        )?;

        broadcast(request.sender_id, connection_ids, |connection_id| {
            self.peer
                .forward_send(request.sender_id, connection_id, request.payload.clone())
        })?;

        Ok(proto::Ack {})
    }

    async fn update_diagnostic_summary(
        mut self: Arc<Server>,
        request: TypedEnvelope<proto::UpdateDiagnosticSummary>,
    ) -> tide::Result<()> {
        let summary = request
            .payload
            .summary
            .clone()
            .ok_or_else(|| anyhow!("invalid summary"))?;
        let receiver_ids = self.state_mut().update_diagnostic_summary(
            request.payload.project_id,
            request.payload.worktree_id,
            request.sender_id,
            summary,
        )?;

        broadcast(request.sender_id, receiver_ids, |connection_id| {
            self.peer
                .forward_send(request.sender_id, connection_id, request.payload.clone())
        })?;
        Ok(())
    }

    async fn start_language_server(
        mut self: Arc<Server>,
        request: TypedEnvelope<proto::StartLanguageServer>,
    ) -> tide::Result<()> {
        let receiver_ids = self.state_mut().start_language_server(
            request.payload.project_id,
            request.sender_id,
            request
                .payload
                .server
                .clone()
                .ok_or_else(|| anyhow!("invalid language server"))?,
        )?;
        broadcast(request.sender_id, receiver_ids, |connection_id| {
            self.peer
                .forward_send(request.sender_id, connection_id, request.payload.clone())
        })?;
        Ok(())
    }

    async fn update_language_server(
        self: Arc<Server>,
        request: TypedEnvelope<proto::UpdateLanguageServer>,
    ) -> tide::Result<()> {
        let receiver_ids = self
            .state()
            .project_connection_ids(request.payload.project_id, request.sender_id)?;
        broadcast(request.sender_id, receiver_ids, |connection_id| {
            self.peer
                .forward_send(request.sender_id, connection_id, request.payload.clone())
        })?;
        Ok(())
    }

    async fn forward_project_request<T>(
        self: Arc<Server>,
        request: TypedEnvelope<T>,
    ) -> tide::Result<T::Response>
    where
        T: EntityMessage + RequestMessage,
    {
        let host_connection_id = self
            .state()
            .read_project(request.payload.remote_entity_id(), request.sender_id)?
            .host_connection_id;
        Ok(self
            .peer
            .forward_request(request.sender_id, host_connection_id, request.payload)
            .await?)
    }

    async fn save_buffer(
        self: Arc<Server>,
        request: TypedEnvelope<proto::SaveBuffer>,
    ) -> tide::Result<proto::BufferSaved> {
        let host;
        let mut guests;
        {
            let state = self.state();
            let project = state.read_project(request.payload.project_id, request.sender_id)?;
            host = project.host_connection_id;
            guests = project.guest_connection_ids()
        }

        let response = self
            .peer
            .forward_request(request.sender_id, host, request.payload.clone())
            .await?;

        guests.retain(|guest_connection_id| *guest_connection_id != request.sender_id);
        broadcast(host, guests, |conn_id| {
            self.peer.forward_send(host, conn_id, response.clone())
        })?;

        Ok(response)
    }

    async fn update_buffer(
        self: Arc<Server>,
        request: TypedEnvelope<proto::UpdateBuffer>,
    ) -> tide::Result<proto::Ack> {
        let receiver_ids = self
            .state()
            .project_connection_ids(request.payload.project_id, request.sender_id)?;
        broadcast(request.sender_id, receiver_ids, |connection_id| {
            self.peer
                .forward_send(request.sender_id, connection_id, request.payload.clone())
        })?;
        Ok(proto::Ack {})
    }

    async fn update_buffer_file(
        self: Arc<Server>,
        request: TypedEnvelope<proto::UpdateBufferFile>,
    ) -> tide::Result<()> {
        let receiver_ids = self
            .state()
            .project_connection_ids(request.payload.project_id, request.sender_id)?;
        broadcast(request.sender_id, receiver_ids, |connection_id| {
            self.peer
                .forward_send(request.sender_id, connection_id, request.payload.clone())
        })?;
        Ok(())
    }

    async fn buffer_reloaded(
        self: Arc<Server>,
        request: TypedEnvelope<proto::BufferReloaded>,
    ) -> tide::Result<()> {
        let receiver_ids = self
            .state()
            .project_connection_ids(request.payload.project_id, request.sender_id)?;
        broadcast(request.sender_id, receiver_ids, |connection_id| {
            self.peer
                .forward_send(request.sender_id, connection_id, request.payload.clone())
        })?;
        Ok(())
    }

    async fn buffer_saved(
        self: Arc<Server>,
        request: TypedEnvelope<proto::BufferSaved>,
    ) -> tide::Result<()> {
        let receiver_ids = self
            .state()
            .project_connection_ids(request.payload.project_id, request.sender_id)?;
        broadcast(request.sender_id, receiver_ids, |connection_id| {
            self.peer
                .forward_send(request.sender_id, connection_id, request.payload.clone())
        })?;
        Ok(())
    }

    async fn follow(
        self: Arc<Self>,
        request: TypedEnvelope<proto::Follow>,
    ) -> tide::Result<proto::FollowResponse> {
        let leader_id = ConnectionId(request.payload.leader_id);
        let follower_id = request.sender_id;
        if !self
            .state()
            .project_connection_ids(request.payload.project_id, follower_id)?
            .contains(&leader_id)
        {
            Err(anyhow!("no such peer"))?;
        }
        let mut response = self
            .peer
            .forward_request(request.sender_id, leader_id, request.payload)
            .await?;
        response
            .views
            .retain(|view| view.leader_id != Some(follower_id.0));
        Ok(response)
    }

    async fn unfollow(
        self: Arc<Self>,
        request: TypedEnvelope<proto::Unfollow>,
    ) -> tide::Result<()> {
        let leader_id = ConnectionId(request.payload.leader_id);
        if !self
            .state()
            .project_connection_ids(request.payload.project_id, request.sender_id)?
            .contains(&leader_id)
        {
            Err(anyhow!("no such peer"))?;
        }
        self.peer
            .forward_send(request.sender_id, leader_id, request.payload)?;
        Ok(())
    }

    async fn update_followers(
        self: Arc<Self>,
        request: TypedEnvelope<proto::UpdateFollowers>,
    ) -> tide::Result<()> {
        let connection_ids = self
            .state()
            .project_connection_ids(request.payload.project_id, request.sender_id)?;
        let leader_id = request
            .payload
            .variant
            .as_ref()
            .and_then(|variant| match variant {
                proto::update_followers::Variant::CreateView(payload) => payload.leader_id,
                proto::update_followers::Variant::UpdateView(payload) => payload.leader_id,
                proto::update_followers::Variant::UpdateActiveView(payload) => payload.leader_id,
            });
        for follower_id in &request.payload.follower_ids {
            let follower_id = ConnectionId(*follower_id);
            if connection_ids.contains(&follower_id) && Some(follower_id.0) != leader_id {
                self.peer
                    .forward_send(request.sender_id, follower_id, request.payload.clone())?;
            }
        }
        Ok(())
    }

    async fn get_channels(
        self: Arc<Server>,
        request: TypedEnvelope<proto::GetChannels>,
    ) -> tide::Result<proto::GetChannelsResponse> {
        let user_id = self.state().user_id_for_connection(request.sender_id)?;
        let channels = self.app_state.db.get_accessible_channels(user_id).await?;
        Ok(proto::GetChannelsResponse {
            channels: channels
                .into_iter()
                .map(|chan| proto::Channel {
                    id: chan.id.to_proto(),
                    name: chan.name,
                })
                .collect(),
        })
    }

    async fn get_users(
        self: Arc<Server>,
        request: TypedEnvelope<proto::GetUsers>,
    ) -> tide::Result<proto::GetUsersResponse> {
        let user_ids = request
            .payload
            .user_ids
            .into_iter()
            .map(UserId::from_proto)
            .collect();
        let users = self
            .app_state
            .db
            .get_users_by_ids(user_ids)
            .await?
            .into_iter()
            .map(|user| proto::User {
                id: user.id.to_proto(),
                avatar_url: format!("https://github.com/{}.png?size=128", user.github_login),
                github_login: user.github_login,
            })
            .collect();
        Ok(proto::GetUsersResponse { users })
    }

    fn update_contacts_for_users<'a>(
        self: &Arc<Server>,
        user_ids: impl IntoIterator<Item = &'a UserId>,
    ) -> anyhow::Result<()> {
        let mut result = Ok(());
        let state = self.state();
        for user_id in user_ids {
            let contacts = state.contacts_for_user(*user_id);
            for connection_id in state.connection_ids_for_user(*user_id) {
                if let Err(error) = self.peer.send(
                    connection_id,
                    proto::UpdateContacts {
                        contacts: contacts.clone(),
                    },
                ) {
                    result = Err(error);
                }
            }
        }
        result
    }

    async fn join_channel(
        mut self: Arc<Self>,
        request: TypedEnvelope<proto::JoinChannel>,
    ) -> tide::Result<proto::JoinChannelResponse> {
        let user_id = self.state().user_id_for_connection(request.sender_id)?;
        let channel_id = ChannelId::from_proto(request.payload.channel_id);
        if !self
            .app_state
            .db
            .can_user_access_channel(user_id, channel_id)
            .await?
        {
            Err(anyhow!("access denied"))?;
        }

        self.state_mut().join_channel(request.sender_id, channel_id);
        let messages = self
            .app_state
            .db
            .get_channel_messages(channel_id, MESSAGE_COUNT_PER_PAGE, None)
            .await?
            .into_iter()
            .map(|msg| proto::ChannelMessage {
                id: msg.id.to_proto(),
                body: msg.body,
                timestamp: msg.sent_at.unix_timestamp() as u64,
                sender_id: msg.sender_id.to_proto(),
                nonce: Some(msg.nonce.as_u128().into()),
            })
            .collect::<Vec<_>>();
        Ok(proto::JoinChannelResponse {
            done: messages.len() < MESSAGE_COUNT_PER_PAGE,
            messages,
        })
    }

    async fn leave_channel(
        mut self: Arc<Self>,
        request: TypedEnvelope<proto::LeaveChannel>,
    ) -> tide::Result<()> {
        let user_id = self.state().user_id_for_connection(request.sender_id)?;
        let channel_id = ChannelId::from_proto(request.payload.channel_id);
        if !self
            .app_state
            .db
            .can_user_access_channel(user_id, channel_id)
            .await?
        {
            Err(anyhow!("access denied"))?;
        }

        self.state_mut()
            .leave_channel(request.sender_id, channel_id);

        Ok(())
    }

    async fn send_channel_message(
        self: Arc<Self>,
        request: TypedEnvelope<proto::SendChannelMessage>,
    ) -> tide::Result<proto::SendChannelMessageResponse> {
        let channel_id = ChannelId::from_proto(request.payload.channel_id);
        let user_id;
        let connection_ids;
        {
            let state = self.state();
            user_id = state.user_id_for_connection(request.sender_id)?;
            connection_ids = state.channel_connection_ids(channel_id)?;
        }

        // Validate the message body.
        let body = request.payload.body.trim().to_string();
        if body.len() > MAX_MESSAGE_LEN {
            return Err(anyhow!("message is too long"))?;
        }
        if body.is_empty() {
            return Err(anyhow!("message can't be blank"))?;
        }

        let timestamp = OffsetDateTime::now_utc();
        let nonce = request
            .payload
            .nonce
            .ok_or_else(|| anyhow!("nonce can't be blank"))?;

        let message_id = self
            .app_state
            .db
            .create_channel_message(channel_id, user_id, &body, timestamp, nonce.clone().into())
            .await?
            .to_proto();
        let message = proto::ChannelMessage {
            sender_id: user_id.to_proto(),
            id: message_id,
            body,
            timestamp: timestamp.unix_timestamp() as u64,
            nonce: Some(nonce),
        };
        broadcast(request.sender_id, connection_ids, |conn_id| {
            self.peer.send(
                conn_id,
                proto::ChannelMessageSent {
                    channel_id: channel_id.to_proto(),
                    message: Some(message.clone()),
                },
            )
        })?;
        Ok(proto::SendChannelMessageResponse {
            message: Some(message),
        })
    }

    async fn get_channel_messages(
        self: Arc<Self>,
        request: TypedEnvelope<proto::GetChannelMessages>,
    ) -> tide::Result<proto::GetChannelMessagesResponse> {
        let user_id = self.state().user_id_for_connection(request.sender_id)?;
        let channel_id = ChannelId::from_proto(request.payload.channel_id);
        if !self
            .app_state
            .db
            .can_user_access_channel(user_id, channel_id)
            .await?
        {
            Err(anyhow!("access denied"))?;
        }

        let messages = self
            .app_state
            .db
            .get_channel_messages(
                channel_id,
                MESSAGE_COUNT_PER_PAGE,
                Some(MessageId::from_proto(request.payload.before_message_id)),
            )
            .await?
            .into_iter()
            .map(|msg| proto::ChannelMessage {
                id: msg.id.to_proto(),
                body: msg.body,
                timestamp: msg.sent_at.unix_timestamp() as u64,
                sender_id: msg.sender_id.to_proto(),
                nonce: Some(msg.nonce.as_u128().into()),
            })
            .collect::<Vec<_>>();

        Ok(proto::GetChannelMessagesResponse {
            done: messages.len() < MESSAGE_COUNT_PER_PAGE,
            messages,
        })
    }

    fn state<'a>(self: &'a Arc<Self>) -> RwLockReadGuard<'a, Store> {
        self.store.read()
    }

    fn state_mut<'a>(self: &'a mut Arc<Self>) -> RwLockWriteGuard<'a, Store> {
        self.store.write()
    }
}

impl Executor for RealExecutor {
    type Timer = Timer;

    fn spawn_detached<F: 'static + Send + Future<Output = ()>>(&self, future: F) {
        task::spawn(future);
    }

    fn timer(&self, duration: Duration) -> Self::Timer {
        Timer::after(duration)
    }
}

fn broadcast<F>(
    sender_id: ConnectionId,
    receiver_ids: Vec<ConnectionId>,
    mut f: F,
) -> anyhow::Result<()>
where
    F: FnMut(ConnectionId) -> anyhow::Result<()>,
{
    let mut result = Ok(());
    for receiver_id in receiver_ids {
        if receiver_id != sender_id {
            if let Err(error) = f(receiver_id) {
                if result.is_ok() {
                    result = Err(error);
                }
            }
        }
    }
    result
}

pub fn add_routes(app: &mut tide::Server<Arc<AppState>>, rpc: &Arc<Peer>) {
    let server = Server::new(app.state().clone(), rpc.clone(), None);
    app.at("/rpc").get(move |request: Request<Arc<AppState>>| {
        let server = server.clone();
        async move {
            const WEBSOCKET_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

            let connection_upgrade = header_contains_ignore_case(&request, CONNECTION, "upgrade");
            let upgrade_to_websocket = header_contains_ignore_case(&request, UPGRADE, "websocket");
            let upgrade_requested = connection_upgrade && upgrade_to_websocket;
            let client_protocol_version: Option<u32> = request
                .header("X-Zed-Protocol-Version")
                .and_then(|v| v.as_str().parse().ok());

            if !upgrade_requested || client_protocol_version != Some(rpc::PROTOCOL_VERSION) {
                return Ok(Response::new(StatusCode::UpgradeRequired));
            }

            let header = match request.header("Sec-Websocket-Key") {
                Some(h) => h.as_str(),
                None => return Err(anyhow!("expected sec-websocket-key"))?,
            };

            let user_id = process_auth_header(&request).await?;

            let mut response = Response::new(StatusCode::SwitchingProtocols);
            response.insert_header(UPGRADE, "websocket");
            response.insert_header(CONNECTION, "Upgrade");
            let hash = Sha1::new().chain(header).chain(WEBSOCKET_GUID).finalize();
            response.insert_header("Sec-Websocket-Accept", base64::encode(&hash[..]));
            response.insert_header("Sec-Websocket-Version", "13");

            let http_res: &mut tide::http::Response = response.as_mut();
            let upgrade_receiver = http_res.recv_upgrade().await;
            let addr = request.remote().unwrap_or("unknown").to_string();
            task::spawn(async move {
                if let Some(stream) = upgrade_receiver.await {
                    server
                        .handle_connection(
                            Connection::new(
                                WebSocketStream::from_raw_socket(stream, Role::Server, None).await,
                            ),
                            addr,
                            user_id,
                            None,
                            RealExecutor,
                        )
                        .await;
                }
            });

            Ok(response)
        }
    });
}

fn header_contains_ignore_case<T>(
    request: &tide::Request<T>,
    header_name: HeaderName,
    value: &str,
) -> bool {
    request
        .header(header_name)
        .map(|h| {
            h.as_str()
                .split(',')
                .any(|s| s.trim().eq_ignore_ascii_case(value.trim()))
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        auth,
        db::{tests::TestDb, UserId},
        github, AppState, Config,
    };
    use ::rpc::Peer;
    use client::{
        self, test::FakeHttpClient, Channel, ChannelDetails, ChannelList, Client, Credentials,
        EstablishConnectionError, UserStore, RECEIVE_TIMEOUT,
    };
    use collections::BTreeMap;
    use editor::{
        self, ConfirmCodeAction, ConfirmCompletion, ConfirmRename, Editor, Input, Redo, Rename,
        ToOffset, ToggleCodeActions, Undo,
    };
    use gpui::{executor, geometry::vector::vec2f, ModelHandle, TestAppContext, ViewHandle};
    use language::{
        range_to_lsp, tree_sitter_rust, Diagnostic, DiagnosticEntry, FakeLspAdapter, Language,
        LanguageConfig, LanguageRegistry, OffsetRangeExt, Point, Rope,
    };
    use lsp::{self, FakeLanguageServer};
    use parking_lot::Mutex;
    use project::{
        fs::{FakeFs, Fs as _},
        search::SearchQuery,
        worktree::WorktreeHandle,
        DiagnosticSummary, Project, ProjectPath, WorktreeId,
    };
    use rand::prelude::*;
    use rpc::PeerId;
    use serde_json::json;
    use settings::Settings;
    use sqlx::types::time::OffsetDateTime;
    use std::{
        cell::Cell,
        env,
        ops::Deref,
        path::{Path, PathBuf},
        rc::Rc,
        sync::{
            atomic::{AtomicBool, Ordering::SeqCst},
            Arc,
        },
        time::Duration,
    };
    use util::TryFutureExt;
    use workspace::{Item, SplitDirection, ToggleFollow, Workspace, WorkspaceParams};

    #[cfg(test)]
    #[ctor::ctor]
    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }

    #[gpui::test(iterations = 10)]
    async fn test_share_project(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        let (window_b, _) = cx_b.add_window(|_| EmptyView);
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());
        cx_a.foreground().forbid_parking();

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/a", true, cx)
            })
            .await
            .unwrap();
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join that project as client B
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        let replica_id_b = project_b.read_with(cx_b, |project, _| {
            assert_eq!(
                project
                    .collaborators()
                    .get(&client_a.peer_id)
                    .unwrap()
                    .user
                    .github_login,
                "user_a"
            );
            project.replica_id()
        });
        project_a
            .condition(&cx_a, |tree, _| {
                tree.collaborators()
                    .get(&client_b.peer_id)
                    .map_or(false, |collaborator| {
                        collaborator.replica_id == replica_id_b
                            && collaborator.user.github_login == "user_b"
                    })
            })
            .await;

        // Open the same file as client B and client A.
        let buffer_b = project_b
            .update(cx_b, |p, cx| p.open_buffer((worktree_id, "b.txt"), cx))
            .await
            .unwrap();
        buffer_b.read_with(cx_b, |buf, _| assert_eq!(buf.text(), "b-contents"));
        project_a.read_with(cx_a, |project, cx| {
            assert!(project.has_open_buffer((worktree_id, "b.txt"), cx))
        });
        let buffer_a = project_a
            .update(cx_a, |p, cx| p.open_buffer((worktree_id, "b.txt"), cx))
            .await
            .unwrap();

        let editor_b = cx_b.add_view(window_b, |cx| Editor::for_buffer(buffer_b, None, cx));

        // TODO
        // // Create a selection set as client B and see that selection set as client A.
        // buffer_a
        //     .condition(&cx_a, |buffer, _| buffer.selection_sets().count() == 1)
        //     .await;

        // Edit the buffer as client B and see that edit as client A.
        editor_b.update(cx_b, |editor, cx| {
            editor.handle_input(&Input("ok, ".into()), cx)
        });
        buffer_a
            .condition(&cx_a, |buffer, _| buffer.text() == "ok, b-contents")
            .await;

        // TODO
        // // Remove the selection set as client B, see those selections disappear as client A.
        cx_b.update(move |_| drop(editor_b));
        // buffer_a
        //     .condition(&cx_a, |buffer, _| buffer.selection_sets().count() == 0)
        //     .await;

        // Dropping the client B's project removes client B from client A's collaborators.
        cx_b.update(move |_| drop(project_b));
        project_a
            .condition(&cx_a, |project, _| project.collaborators().is_empty())
            .await;
    }

    #[gpui::test(iterations = 10)]
    async fn test_unshare_project(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());
        cx_a.foreground().forbid_parking();

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/a", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();
        assert!(worktree_a.read_with(cx_a, |tree, _| tree.as_local().unwrap().is_shared()));

        // Join that project as client B
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();
        project_b
            .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
            .await
            .unwrap();

        // Unshare the project as client A
        project_a.update(cx_a, |project, cx| project.unshare(cx));
        project_b
            .condition(cx_b, |project, _| project.is_read_only())
            .await;
        assert!(worktree_a.read_with(cx_a, |tree, _| !tree.as_local().unwrap().is_shared()));
        cx_b.update(|_| {
            drop(project_b);
        });

        // Share the project again and ensure guests can still join.
        project_a
            .update(cx_a, |project, cx| project.share(cx))
            .await
            .unwrap();
        assert!(worktree_a.read_with(cx_a, |tree, _| tree.as_local().unwrap().is_shared()));

        let project_b2 = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();
        project_b2
            .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
            .await
            .unwrap();
    }

    #[gpui::test(iterations = 10)]
    async fn test_host_disconnect(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());
        cx_a.foreground().forbid_parking();

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/a", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();
        assert!(worktree_a.read_with(cx_a, |tree, _| tree.as_local().unwrap().is_shared()));

        // Join that project as client B
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();
        project_b
            .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
            .await
            .unwrap();

        // Drop client A's connection. Collaborators should disappear and the project should not be shown as shared.
        server.disconnect_client(client_a.current_user_id(cx_a));
        cx_a.foreground().advance_clock(rpc::RECEIVE_TIMEOUT);
        project_a
            .condition(cx_a, |project, _| project.collaborators().is_empty())
            .await;
        project_a.read_with(cx_a, |project, _| assert!(!project.is_shared()));
        project_b
            .condition(cx_b, |project, _| project.is_read_only())
            .await;
        assert!(worktree_a.read_with(cx_a, |tree, _| !tree.as_local().unwrap().is_shared()));
        cx_b.update(|_| {
            drop(project_b);
        });

        // Await reconnection
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;

        // Share the project again and ensure guests can still join.
        project_a
            .update(cx_a, |project, cx| project.share(cx))
            .await
            .unwrap();
        assert!(worktree_a.read_with(cx_a, |tree, _| tree.as_local().unwrap().is_shared()));

        let project_b2 = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();
        project_b2
            .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
            .await
            .unwrap();
    }

    #[gpui::test(iterations = 10)]
    async fn test_propagate_saves_and_fs_changes(
        cx_a: &mut TestAppContext,
        cx_b: &mut TestAppContext,
        cx_c: &mut TestAppContext,
    ) {
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());
        cx_a.foreground().forbid_parking();

        // Connect to a server as 3 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;
        let client_c = server.create_client(cx_c, "user_c").await;

        // Share a worktree as client A.
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b", "user_c"]"#,
                "file1": "",
                "file2": ""
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/a", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join that worktree as clients B and C.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();
        let project_c = Project::remote(
            project_id,
            client_c.clone(),
            client_c.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_c.to_async(),
        )
        .await
        .unwrap();
        let worktree_b = project_b.read_with(cx_b, |p, cx| p.worktrees(cx).next().unwrap());
        let worktree_c = project_c.read_with(cx_c, |p, cx| p.worktrees(cx).next().unwrap());

        // Open and edit a buffer as both guests B and C.
        let buffer_b = project_b
            .update(cx_b, |p, cx| p.open_buffer((worktree_id, "file1"), cx))
            .await
            .unwrap();
        let buffer_c = project_c
            .update(cx_c, |p, cx| p.open_buffer((worktree_id, "file1"), cx))
            .await
            .unwrap();
        buffer_b.update(cx_b, |buf, cx| buf.edit([0..0], "i-am-b, ", cx));
        buffer_c.update(cx_c, |buf, cx| buf.edit([0..0], "i-am-c, ", cx));

        // Open and edit that buffer as the host.
        let buffer_a = project_a
            .update(cx_a, |p, cx| p.open_buffer((worktree_id, "file1"), cx))
            .await
            .unwrap();

        buffer_a
            .condition(cx_a, |buf, _| buf.text() == "i-am-c, i-am-b, ")
            .await;
        buffer_a.update(cx_a, |buf, cx| {
            buf.edit([buf.len()..buf.len()], "i-am-a", cx)
        });

        // Wait for edits to propagate
        buffer_a
            .condition(cx_a, |buf, _| buf.text() == "i-am-c, i-am-b, i-am-a")
            .await;
        buffer_b
            .condition(cx_b, |buf, _| buf.text() == "i-am-c, i-am-b, i-am-a")
            .await;
        buffer_c
            .condition(cx_c, |buf, _| buf.text() == "i-am-c, i-am-b, i-am-a")
            .await;

        // Edit the buffer as the host and concurrently save as guest B.
        let save_b = buffer_b.update(cx_b, |buf, cx| buf.save(cx));
        buffer_a.update(cx_a, |buf, cx| buf.edit([0..0], "hi-a, ", cx));
        save_b.await.unwrap();
        assert_eq!(
            fs.load("/a/file1".as_ref()).await.unwrap(),
            "hi-a, i-am-c, i-am-b, i-am-a"
        );
        buffer_a.read_with(cx_a, |buf, _| assert!(!buf.is_dirty()));
        buffer_b.read_with(cx_b, |buf, _| assert!(!buf.is_dirty()));
        buffer_c.condition(cx_c, |buf, _| !buf.is_dirty()).await;

        worktree_a.flush_fs_events(cx_a).await;

        // Make changes on host's file system, see those changes on guest worktrees.
        fs.rename(
            "/a/file1".as_ref(),
            "/a/file1-renamed".as_ref(),
            Default::default(),
        )
        .await
        .unwrap();

        fs.rename("/a/file2".as_ref(), "/a/file3".as_ref(), Default::default())
            .await
            .unwrap();
        fs.insert_file(Path::new("/a/file4"), "4".into()).await;

        worktree_a
            .condition(&cx_a, |tree, _| {
                tree.paths()
                    .map(|p| p.to_string_lossy())
                    .collect::<Vec<_>>()
                    == [".zed.toml", "file1-renamed", "file3", "file4"]
            })
            .await;
        worktree_b
            .condition(&cx_b, |tree, _| {
                tree.paths()
                    .map(|p| p.to_string_lossy())
                    .collect::<Vec<_>>()
                    == [".zed.toml", "file1-renamed", "file3", "file4"]
            })
            .await;
        worktree_c
            .condition(&cx_c, |tree, _| {
                tree.paths()
                    .map(|p| p.to_string_lossy())
                    .collect::<Vec<_>>()
                    == [".zed.toml", "file1-renamed", "file3", "file4"]
            })
            .await;

        // Ensure buffer files are updated as well.
        buffer_a
            .condition(&cx_a, |buf, _| {
                buf.file().unwrap().path().to_str() == Some("file1-renamed")
            })
            .await;
        buffer_b
            .condition(&cx_b, |buf, _| {
                buf.file().unwrap().path().to_str() == Some("file1-renamed")
            })
            .await;
        buffer_c
            .condition(&cx_c, |buf, _| {
                buf.file().unwrap().path().to_str() == Some("file1-renamed")
            })
            .await;
    }

    #[gpui::test(iterations = 10)]
    async fn test_buffer_conflict_after_save(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/dir",
            json!({
                ".zed.toml": r#"collaborators = ["user_b", "user_c"]"#,
                "a.txt": "a-contents",
            }),
        )
        .await;

        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/dir", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join that project as client B
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        // Open a buffer as client B
        let buffer_b = project_b
            .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
            .await
            .unwrap();

        buffer_b.update(cx_b, |buf, cx| buf.edit([0..0], "world ", cx));
        buffer_b.read_with(cx_b, |buf, _| {
            assert!(buf.is_dirty());
            assert!(!buf.has_conflict());
        });

        buffer_b.update(cx_b, |buf, cx| buf.save(cx)).await.unwrap();
        buffer_b
            .condition(&cx_b, |buffer_b, _| !buffer_b.is_dirty())
            .await;
        buffer_b.read_with(cx_b, |buf, _| {
            assert!(!buf.has_conflict());
        });

        buffer_b.update(cx_b, |buf, cx| buf.edit([0..0], "hello ", cx));
        buffer_b.read_with(cx_b, |buf, _| {
            assert!(buf.is_dirty());
            assert!(!buf.has_conflict());
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_buffer_reloading(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/dir",
            json!({
                ".zed.toml": r#"collaborators = ["user_b", "user_c"]"#,
                "a.txt": "a-contents",
            }),
        )
        .await;

        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/dir", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join that project as client B
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();
        let _worktree_b = project_b.update(cx_b, |p, cx| p.worktrees(cx).next().unwrap());

        // Open a buffer as client B
        let buffer_b = project_b
            .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
            .await
            .unwrap();
        buffer_b.read_with(cx_b, |buf, _| {
            assert!(!buf.is_dirty());
            assert!(!buf.has_conflict());
        });

        fs.save(Path::new("/dir/a.txt"), &"new contents".into())
            .await
            .unwrap();
        buffer_b
            .condition(&cx_b, |buf, _| {
                buf.text() == "new contents" && !buf.is_dirty()
            })
            .await;
        buffer_b.read_with(cx_b, |buf, _| {
            assert!(!buf.has_conflict());
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_editing_while_guest_opens_buffer(
        cx_a: &mut TestAppContext,
        cx_b: &mut TestAppContext,
    ) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/dir",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "a.txt": "a-contents",
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/dir", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join that project as client B
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        // Open a buffer as client A
        let buffer_a = project_a
            .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
            .await
            .unwrap();

        // Start opening the same buffer as client B
        let buffer_b = cx_b
            .background()
            .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx)));

        // Edit the buffer as client A while client B is still opening it.
        cx_b.background().simulate_random_delay().await;
        buffer_a.update(cx_a, |buf, cx| buf.edit([0..0], "X", cx));
        cx_b.background().simulate_random_delay().await;
        buffer_a.update(cx_a, |buf, cx| buf.edit([1..1], "Y", cx));

        let text = buffer_a.read_with(cx_a, |buf, _| buf.text());
        let buffer_b = buffer_b.await.unwrap();
        buffer_b.condition(&cx_b, |buf, _| buf.text() == text).await;
    }

    #[gpui::test(iterations = 10)]
    async fn test_leaving_worktree_while_opening_buffer(
        cx_a: &mut TestAppContext,
        cx_b: &mut TestAppContext,
    ) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/dir",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "a.txt": "a-contents",
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/dir", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join that project as client B
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        // See that a guest has joined as client A.
        project_a
            .condition(&cx_a, |p, _| p.collaborators().len() == 1)
            .await;

        // Begin opening a buffer as client B, but leave the project before the open completes.
        let buffer_b = cx_b
            .background()
            .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx)));
        cx_b.update(|_| drop(project_b));
        drop(buffer_b);

        // See that the guest has left.
        project_a
            .condition(&cx_a, |p, _| p.collaborators().len() == 0)
            .await;
    }

    #[gpui::test(iterations = 10)]
    async fn test_leaving_project(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/a", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a
            .update(cx_a, |project, _| project.next_remote_id())
            .await;
        project_a
            .update(cx_a, |project, cx| project.share(cx))
            .await
            .unwrap();

        // Join that project as client B
        let _project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        // Client A sees that a guest has joined.
        project_a
            .condition(cx_a, |p, _| p.collaborators().len() == 1)
            .await;

        // Drop client B's connection and ensure client A observes client B leaving the project.
        client_b.disconnect(&cx_b.to_async()).unwrap();
        project_a
            .condition(cx_a, |p, _| p.collaborators().len() == 0)
            .await;

        // Rejoin the project as client B
        let _project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        // Client A sees that a guest has re-joined.
        project_a
            .condition(cx_a, |p, _| p.collaborators().len() == 1)
            .await;

        // Simulate connection loss for client B and ensure client A observes client B leaving the project.
        client_b.wait_for_current_user(cx_b).await;
        server.disconnect_client(client_b.current_user_id(cx_b));
        cx_a.foreground().advance_clock(Duration::from_secs(3));
        project_a
            .condition(cx_a, |p, _| p.collaborators().len() == 0)
            .await;
    }

    #[gpui::test(iterations = 10)]
    async fn test_collaborating_with_diagnostics(
        cx_a: &mut TestAppContext,
        cx_b: &mut TestAppContext,
    ) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());

        // Set up a fake language server.
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());
        lang_registry.add(Arc::new(language));

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "a.rs": "let one = two",
                "other.rs": "",
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/a", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Cause the language server to start.
        let _ = cx_a
            .background()
            .spawn(project_a.update(cx_a, |project, cx| {
                project.open_buffer(
                    ProjectPath {
                        worktree_id,
                        path: Path::new("other.rs").into(),
                    },
                    cx,
                )
            }))
            .await
            .unwrap();

        // Simulate a language server reporting errors for a file.
        let mut fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;
        fake_language_server.notify::<lsp::notification::PublishDiagnostics>(
            lsp::PublishDiagnosticsParams {
                uri: lsp::Url::from_file_path("/a/a.rs").unwrap(),
                version: None,
                diagnostics: vec![lsp::Diagnostic {
                    severity: Some(lsp::DiagnosticSeverity::ERROR),
                    range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 7)),
                    message: "message 1".to_string(),
                    ..Default::default()
                }],
            },
        );

        // Wait for server to see the diagnostics update.
        server
            .condition(|store| {
                let worktree = store
                    .project(project_id)
                    .unwrap()
                    .share
                    .as_ref()
                    .unwrap()
                    .worktrees
                    .get(&worktree_id.to_proto())
                    .unwrap();

                !worktree.diagnostic_summaries.is_empty()
            })
            .await;

        // Join the worktree as client B.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        project_b.read_with(cx_b, |project, cx| {
            assert_eq!(
                project.diagnostic_summaries(cx).collect::<Vec<_>>(),
                &[(
                    ProjectPath {
                        worktree_id,
                        path: Arc::from(Path::new("a.rs")),
                    },
                    DiagnosticSummary {
                        error_count: 1,
                        warning_count: 0,
                        ..Default::default()
                    },
                )]
            )
        });

        // Simulate a language server reporting more errors for a file.
        fake_language_server.notify::<lsp::notification::PublishDiagnostics>(
            lsp::PublishDiagnosticsParams {
                uri: lsp::Url::from_file_path("/a/a.rs").unwrap(),
                version: None,
                diagnostics: vec![
                    lsp::Diagnostic {
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 7)),
                        message: "message 1".to_string(),
                        ..Default::default()
                    },
                    lsp::Diagnostic {
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        range: lsp::Range::new(
                            lsp::Position::new(0, 10),
                            lsp::Position::new(0, 13),
                        ),
                        message: "message 2".to_string(),
                        ..Default::default()
                    },
                ],
            },
        );

        // Client b gets the updated summaries
        project_b
            .condition(&cx_b, |project, cx| {
                project.diagnostic_summaries(cx).collect::<Vec<_>>()
                    == &[(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("a.rs")),
                        },
                        DiagnosticSummary {
                            error_count: 1,
                            warning_count: 1,
                            ..Default::default()
                        },
                    )]
            })
            .await;

        // Open the file with the errors on client B. They should be present.
        let buffer_b = cx_b
            .background()
            .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx)))
            .await
            .unwrap();

        buffer_b.read_with(cx_b, |buffer, _| {
            assert_eq!(
                buffer
                    .snapshot()
                    .diagnostics_in_range::<_, Point>(0..buffer.len(), false)
                    .map(|entry| entry)
                    .collect::<Vec<_>>(),
                &[
                    DiagnosticEntry {
                        range: Point::new(0, 4)..Point::new(0, 7),
                        diagnostic: Diagnostic {
                            group_id: 0,
                            message: "message 1".to_string(),
                            severity: lsp::DiagnosticSeverity::ERROR,
                            is_primary: true,
                            ..Default::default()
                        }
                    },
                    DiagnosticEntry {
                        range: Point::new(0, 10)..Point::new(0, 13),
                        diagnostic: Diagnostic {
                            group_id: 1,
                            severity: lsp::DiagnosticSeverity::WARNING,
                            message: "message 2".to_string(),
                            is_primary: true,
                            ..Default::default()
                        }
                    }
                ]
            );
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_collaborating_with_completion(
        cx_a: &mut TestAppContext,
        cx_b: &mut TestAppContext,
    ) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());

        // Set up a fake language server.
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_language_servers = language.set_fake_lsp_adapter(FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        });
        lang_registry.add(Arc::new(language));

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "main.rs": "fn main() { a }",
                "other.rs": "",
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/a", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join the worktree as client B.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        // Open a file in an editor as the guest.
        let buffer_b = project_b
            .update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
            .await
            .unwrap();
        let (window_b, _) = cx_b.add_window(|_| EmptyView);
        let editor_b = cx_b.add_view(window_b, |cx| {
            Editor::for_buffer(buffer_b.clone(), Some(project_b.clone()), cx)
        });

        let fake_language_server = fake_language_servers.next().await.unwrap();
        buffer_b
            .condition(&cx_b, |buffer, _| !buffer.completion_triggers().is_empty())
            .await;

        // Type a completion trigger character as the guest.
        editor_b.update(cx_b, |editor, cx| {
            editor.select_ranges([13..13], None, cx);
            editor.handle_input(&Input(".".into()), cx);
            cx.focus(&editor_b);
        });

        // Receive a completion request as the host's language server.
        // Return some completions from the host's language server.
        cx_a.foreground().start_waiting();
        fake_language_server
            .handle_request::<lsp::request::Completion, _, _>(|params, _| async move {
                assert_eq!(
                    params.text_document_position.text_document.uri,
                    lsp::Url::from_file_path("/a/main.rs").unwrap(),
                );
                assert_eq!(
                    params.text_document_position.position,
                    lsp::Position::new(0, 14),
                );

                Ok(Some(lsp::CompletionResponse::Array(vec![
                    lsp::CompletionItem {
                        label: "first_method(…)".into(),
                        detail: Some("fn(&mut self, B) -> C".into()),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            new_text: "first_method($1)".to_string(),
                            range: lsp::Range::new(
                                lsp::Position::new(0, 14),
                                lsp::Position::new(0, 14),
                            ),
                        })),
                        insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                        ..Default::default()
                    },
                    lsp::CompletionItem {
                        label: "second_method(…)".into(),
                        detail: Some("fn(&mut self, C) -> D<E>".into()),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            new_text: "second_method()".to_string(),
                            range: lsp::Range::new(
                                lsp::Position::new(0, 14),
                                lsp::Position::new(0, 14),
                            ),
                        })),
                        insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                        ..Default::default()
                    },
                ])))
            })
            .next()
            .await
            .unwrap();
        cx_a.foreground().finish_waiting();

        // Open the buffer on the host.
        let buffer_a = project_a
            .update(cx_a, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
            .await
            .unwrap();
        buffer_a
            .condition(&cx_a, |buffer, _| buffer.text() == "fn main() { a. }")
            .await;

        // Confirm a completion on the guest.
        editor_b
            .condition(&cx_b, |editor, _| editor.context_menu_visible())
            .await;
        editor_b.update(cx_b, |editor, cx| {
            editor.confirm_completion(&ConfirmCompletion(Some(0)), cx);
            assert_eq!(editor.text(cx), "fn main() { a.first_method() }");
        });

        // Return a resolved completion from the host's language server.
        // The resolved completion has an additional text edit.
        fake_language_server.handle_request::<lsp::request::ResolveCompletionItem, _, _>(
            |params, _| async move {
                assert_eq!(params.label, "first_method(…)");
                Ok(lsp::CompletionItem {
                    label: "first_method(…)".into(),
                    detail: Some("fn(&mut self, B) -> C".into()),
                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                        new_text: "first_method($1)".to_string(),
                        range: lsp::Range::new(
                            lsp::Position::new(0, 14),
                            lsp::Position::new(0, 14),
                        ),
                    })),
                    additional_text_edits: Some(vec![lsp::TextEdit {
                        new_text: "use d::SomeTrait;\n".to_string(),
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
                    }]),
                    insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                    ..Default::default()
                })
            },
        );

        // The additional edit is applied.
        buffer_a
            .condition(&cx_a, |buffer, _| {
                buffer.text() == "use d::SomeTrait;\nfn main() { a.first_method() }"
            })
            .await;
        buffer_b
            .condition(&cx_b, |buffer, _| {
                buffer.text() == "use d::SomeTrait;\nfn main() { a.first_method() }"
            })
            .await;
    }

    #[gpui::test(iterations = 10)]
    async fn test_reloading_buffer_manually(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "a.rs": "let one = 1;",
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/a", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();
        let buffer_a = project_a
            .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx))
            .await
            .unwrap();

        // Join the worktree as client B.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        let buffer_b = cx_b
            .background()
            .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx)))
            .await
            .unwrap();
        buffer_b.update(cx_b, |buffer, cx| {
            buffer.edit([4..7], "six", cx);
            buffer.edit([10..11], "6", cx);
            assert_eq!(buffer.text(), "let six = 6;");
            assert!(buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });
        buffer_a
            .condition(cx_a, |buffer, _| buffer.text() == "let six = 6;")
            .await;

        fs.save(Path::new("/a/a.rs"), &Rope::from("let seven = 7;"))
            .await
            .unwrap();
        buffer_a
            .condition(cx_a, |buffer, _| buffer.has_conflict())
            .await;
        buffer_b
            .condition(cx_b, |buffer, _| buffer.has_conflict())
            .await;

        project_b
            .update(cx_b, |project, cx| {
                project.reload_buffers(HashSet::from_iter([buffer_b.clone()]), true, cx)
            })
            .await
            .unwrap();
        buffer_a.read_with(cx_a, |buffer, _| {
            assert_eq!(buffer.text(), "let seven = 7;");
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });
        buffer_b.read_with(cx_b, |buffer, _| {
            assert_eq!(buffer.text(), "let seven = 7;");
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });

        buffer_a.update(cx_a, |buffer, cx| {
            // Undoing on the host is a no-op when the reload was initiated by the guest.
            buffer.undo(cx);
            assert_eq!(buffer.text(), "let seven = 7;");
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });
        buffer_b.update(cx_b, |buffer, cx| {
            // Undoing on the guest rolls back the buffer to before it was reloaded but the conflict gets cleared.
            buffer.undo(cx);
            assert_eq!(buffer.text(), "let six = 6;");
            assert!(buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_formatting_buffer(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());

        // Set up a fake language server.
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());
        lang_registry.add(Arc::new(language));

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "a.rs": "let one = two",
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/a", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join the worktree as client B.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        let buffer_b = cx_b
            .background()
            .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx)))
            .await
            .unwrap();

        let fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server.handle_request::<lsp::request::Formatting, _, _>(|_, _| async move {
            Ok(Some(vec![
                lsp::TextEdit {
                    range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 4)),
                    new_text: "h".to_string(),
                },
                lsp::TextEdit {
                    range: lsp::Range::new(lsp::Position::new(0, 7), lsp::Position::new(0, 7)),
                    new_text: "y".to_string(),
                },
            ]))
        });

        project_b
            .update(cx_b, |project, cx| {
                project.format(HashSet::from_iter([buffer_b.clone()]), true, cx)
            })
            .await
            .unwrap();
        assert_eq!(
            buffer_b.read_with(cx_b, |buffer, _| buffer.text()),
            "let honey = two"
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_definition(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());
        fs.insert_tree(
            "/root-1",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "a.rs": "const ONE: usize = b::TWO + b::THREE;",
            }),
        )
        .await;
        fs.insert_tree(
            "/root-2",
            json!({
                "b.rs": "const TWO: usize = 2;\nconst THREE: usize = 3;",
            }),
        )
        .await;

        // Set up a fake language server.
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());
        lang_registry.add(Arc::new(language));

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/root-1", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join the worktree as client B.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        // Open the file on client B.
        let buffer_b = cx_b
            .background()
            .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx)))
            .await
            .unwrap();

        // Request the definition of a symbol as the guest.
        let fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server.handle_request::<lsp::request::GotoDefinition, _, _>(
            |_, _| async move {
                Ok(Some(lsp::GotoDefinitionResponse::Scalar(
                    lsp::Location::new(
                        lsp::Url::from_file_path("/root-2/b.rs").unwrap(),
                        lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                    ),
                )))
            },
        );

        let definitions_1 = project_b
            .update(cx_b, |p, cx| p.definition(&buffer_b, 23, cx))
            .await
            .unwrap();
        cx_b.read(|cx| {
            assert_eq!(definitions_1.len(), 1);
            assert_eq!(project_b.read(cx).worktrees(cx).count(), 2);
            let target_buffer = definitions_1[0].buffer.read(cx);
            assert_eq!(
                target_buffer.text(),
                "const TWO: usize = 2;\nconst THREE: usize = 3;"
            );
            assert_eq!(
                definitions_1[0].range.to_point(target_buffer),
                Point::new(0, 6)..Point::new(0, 9)
            );
        });

        // Try getting more definitions for the same buffer, ensuring the buffer gets reused from
        // the previous call to `definition`.
        fake_language_server.handle_request::<lsp::request::GotoDefinition, _, _>(
            |_, _| async move {
                Ok(Some(lsp::GotoDefinitionResponse::Scalar(
                    lsp::Location::new(
                        lsp::Url::from_file_path("/root-2/b.rs").unwrap(),
                        lsp::Range::new(lsp::Position::new(1, 6), lsp::Position::new(1, 11)),
                    ),
                )))
            },
        );

        let definitions_2 = project_b
            .update(cx_b, |p, cx| p.definition(&buffer_b, 33, cx))
            .await
            .unwrap();
        cx_b.read(|cx| {
            assert_eq!(definitions_2.len(), 1);
            assert_eq!(project_b.read(cx).worktrees(cx).count(), 2);
            let target_buffer = definitions_2[0].buffer.read(cx);
            assert_eq!(
                target_buffer.text(),
                "const TWO: usize = 2;\nconst THREE: usize = 3;"
            );
            assert_eq!(
                definitions_2[0].range.to_point(target_buffer),
                Point::new(1, 6)..Point::new(1, 11)
            );
        });
        assert_eq!(definitions_1[0].buffer, definitions_2[0].buffer);
    }

    #[gpui::test(iterations = 10)]
    async fn test_references(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());
        fs.insert_tree(
            "/root-1",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
            }),
        )
        .await;
        fs.insert_tree(
            "/root-2",
            json!({
                "three.rs": "const THREE: usize = two::TWO + one::ONE;",
            }),
        )
        .await;

        // Set up a fake language server.
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());
        lang_registry.add(Arc::new(language));

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/root-1", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join the worktree as client B.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        // Open the file on client B.
        let buffer_b = cx_b
            .background()
            .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "one.rs"), cx)))
            .await
            .unwrap();

        // Request references to a symbol as the guest.
        let fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server.handle_request::<lsp::request::References, _, _>(
            |params, _| async move {
                assert_eq!(
                    params.text_document_position.text_document.uri.as_str(),
                    "file:///root-1/one.rs"
                );
                Ok(Some(vec![
                    lsp::Location {
                        uri: lsp::Url::from_file_path("/root-1/two.rs").unwrap(),
                        range: lsp::Range::new(
                            lsp::Position::new(0, 24),
                            lsp::Position::new(0, 27),
                        ),
                    },
                    lsp::Location {
                        uri: lsp::Url::from_file_path("/root-1/two.rs").unwrap(),
                        range: lsp::Range::new(
                            lsp::Position::new(0, 35),
                            lsp::Position::new(0, 38),
                        ),
                    },
                    lsp::Location {
                        uri: lsp::Url::from_file_path("/root-2/three.rs").unwrap(),
                        range: lsp::Range::new(
                            lsp::Position::new(0, 37),
                            lsp::Position::new(0, 40),
                        ),
                    },
                ]))
            },
        );

        let references = project_b
            .update(cx_b, |p, cx| p.references(&buffer_b, 7, cx))
            .await
            .unwrap();
        cx_b.read(|cx| {
            assert_eq!(references.len(), 3);
            assert_eq!(project_b.read(cx).worktrees(cx).count(), 2);

            let two_buffer = references[0].buffer.read(cx);
            let three_buffer = references[2].buffer.read(cx);
            assert_eq!(
                two_buffer.file().unwrap().path().as_ref(),
                Path::new("two.rs")
            );
            assert_eq!(references[1].buffer, references[0].buffer);
            assert_eq!(
                three_buffer.file().unwrap().full_path(cx),
                Path::new("three.rs")
            );

            assert_eq!(references[0].range.to_offset(&two_buffer), 24..27);
            assert_eq!(references[1].range.to_offset(&two_buffer), 35..38);
            assert_eq!(references[2].range.to_offset(&three_buffer), 37..40);
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_project_search(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());
        fs.insert_tree(
            "/root-1",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "a": "hello world",
                "b": "goodnight moon",
                "c": "a world of goo",
                "d": "world champion of clown world",
            }),
        )
        .await;
        fs.insert_tree(
            "/root-2",
            json!({
                "e": "disney world is fun",
            }),
        )
        .await;

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;

        let (worktree_1, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/root-1", true, cx)
            })
            .await
            .unwrap();
        worktree_1
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let (worktree_2, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/root-2", true, cx)
            })
            .await
            .unwrap();
        worktree_2
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;

        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join the worktree as client B.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        let results = project_b
            .update(cx_b, |project, cx| {
                project.search(SearchQuery::text("world", false, false), cx)
            })
            .await
            .unwrap();

        let mut ranges_by_path = results
            .into_iter()
            .map(|(buffer, ranges)| {
                buffer.read_with(cx_b, |buffer, cx| {
                    let path = buffer.file().unwrap().full_path(cx);
                    let offset_ranges = ranges
                        .into_iter()
                        .map(|range| range.to_offset(buffer))
                        .collect::<Vec<_>>();
                    (path, offset_ranges)
                })
            })
            .collect::<Vec<_>>();
        ranges_by_path.sort_by_key(|(path, _)| path.clone());

        assert_eq!(
            ranges_by_path,
            &[
                (PathBuf::from("root-1/a"), vec![6..11]),
                (PathBuf::from("root-1/c"), vec![2..7]),
                (PathBuf::from("root-1/d"), vec![0..5, 24..29]),
                (PathBuf::from("root-2/e"), vec![7..12]),
            ]
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_document_highlights(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());
        fs.insert_tree(
            "/root-1",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "main.rs": "fn double(number: i32) -> i32 { number + number }",
            }),
        )
        .await;

        // Set up a fake language server.
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());
        lang_registry.add(Arc::new(language));

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/root-1", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join the worktree as client B.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        // Open the file on client B.
        let buffer_b = cx_b
            .background()
            .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx)))
            .await
            .unwrap();

        // Request document highlights as the guest.
        let fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server.handle_request::<lsp::request::DocumentHighlightRequest, _, _>(
            |params, _| async move {
                assert_eq!(
                    params
                        .text_document_position_params
                        .text_document
                        .uri
                        .as_str(),
                    "file:///root-1/main.rs"
                );
                assert_eq!(
                    params.text_document_position_params.position,
                    lsp::Position::new(0, 34)
                );
                Ok(Some(vec![
                    lsp::DocumentHighlight {
                        kind: Some(lsp::DocumentHighlightKind::WRITE),
                        range: lsp::Range::new(
                            lsp::Position::new(0, 10),
                            lsp::Position::new(0, 16),
                        ),
                    },
                    lsp::DocumentHighlight {
                        kind: Some(lsp::DocumentHighlightKind::READ),
                        range: lsp::Range::new(
                            lsp::Position::new(0, 32),
                            lsp::Position::new(0, 38),
                        ),
                    },
                    lsp::DocumentHighlight {
                        kind: Some(lsp::DocumentHighlightKind::READ),
                        range: lsp::Range::new(
                            lsp::Position::new(0, 41),
                            lsp::Position::new(0, 47),
                        ),
                    },
                ]))
            },
        );

        let highlights = project_b
            .update(cx_b, |p, cx| p.document_highlights(&buffer_b, 34, cx))
            .await
            .unwrap();
        buffer_b.read_with(cx_b, |buffer, _| {
            let snapshot = buffer.snapshot();

            let highlights = highlights
                .into_iter()
                .map(|highlight| (highlight.kind, highlight.range.to_offset(&snapshot)))
                .collect::<Vec<_>>();
            assert_eq!(
                highlights,
                &[
                    (lsp::DocumentHighlightKind::WRITE, 10..16),
                    (lsp::DocumentHighlightKind::READ, 32..38),
                    (lsp::DocumentHighlightKind::READ, 41..47)
                ]
            )
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_project_symbols(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());
        fs.insert_tree(
            "/code",
            json!({
                "crate-1": {
                    ".zed.toml": r#"collaborators = ["user_b"]"#,
                    "one.rs": "const ONE: usize = 1;",
                },
                "crate-2": {
                    "two.rs": "const TWO: usize = 2; const THREE: usize = 3;",
                },
                "private": {
                    "passwords.txt": "the-password",
                }
            }),
        )
        .await;

        // Set up a fake language server.
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());
        lang_registry.add(Arc::new(language));

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/code/crate-1", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join the worktree as client B.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        // Cause the language server to start.
        let _buffer = cx_b
            .background()
            .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "one.rs"), cx)))
            .await
            .unwrap();

        let fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server.handle_request::<lsp::request::WorkspaceSymbol, _, _>(
            |_, _| async move {
                #[allow(deprecated)]
                Ok(Some(vec![lsp::SymbolInformation {
                    name: "TWO".into(),
                    location: lsp::Location {
                        uri: lsp::Url::from_file_path("/code/crate-2/two.rs").unwrap(),
                        range: lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                    },
                    kind: lsp::SymbolKind::CONSTANT,
                    tags: None,
                    container_name: None,
                    deprecated: None,
                }]))
            },
        );

        // Request the definition of a symbol as the guest.
        let symbols = project_b
            .update(cx_b, |p, cx| p.symbols("two", cx))
            .await
            .unwrap();
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "TWO");

        // Open one of the returned symbols.
        let buffer_b_2 = project_b
            .update(cx_b, |project, cx| {
                project.open_buffer_for_symbol(&symbols[0], cx)
            })
            .await
            .unwrap();
        buffer_b_2.read_with(cx_b, |buffer, _| {
            assert_eq!(
                buffer.file().unwrap().path().as_ref(),
                Path::new("../crate-2/two.rs")
            );
        });

        // Attempt to craft a symbol and violate host's privacy by opening an arbitrary file.
        let mut fake_symbol = symbols[0].clone();
        fake_symbol.path = Path::new("/code/secrets").into();
        let error = project_b
            .update(cx_b, |project, cx| {
                project.open_buffer_for_symbol(&fake_symbol, cx)
            })
            .await
            .unwrap_err();
        assert!(error.to_string().contains("invalid symbol signature"));
    }

    #[gpui::test(iterations = 10)]
    async fn test_open_buffer_while_getting_definition_pointing_to_it(
        cx_a: &mut TestAppContext,
        cx_b: &mut TestAppContext,
        mut rng: StdRng,
    ) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());
        fs.insert_tree(
            "/root",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "a.rs": "const ONE: usize = b::TWO;",
                "b.rs": "const TWO: usize = 2",
            }),
        )
        .await;

        // Set up a fake language server.
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());
        lang_registry.add(Arc::new(language));

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });

        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/root", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join the worktree as client B.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        let buffer_b1 = cx_b
            .background()
            .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx)))
            .await
            .unwrap();

        let fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server.handle_request::<lsp::request::GotoDefinition, _, _>(
            |_, _| async move {
                Ok(Some(lsp::GotoDefinitionResponse::Scalar(
                    lsp::Location::new(
                        lsp::Url::from_file_path("/root/b.rs").unwrap(),
                        lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                    ),
                )))
            },
        );

        let definitions;
        let buffer_b2;
        if rng.gen() {
            definitions = project_b.update(cx_b, |p, cx| p.definition(&buffer_b1, 23, cx));
            buffer_b2 = project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "b.rs"), cx));
        } else {
            buffer_b2 = project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "b.rs"), cx));
            definitions = project_b.update(cx_b, |p, cx| p.definition(&buffer_b1, 23, cx));
        }

        let buffer_b2 = buffer_b2.await.unwrap();
        let definitions = definitions.await.unwrap();
        assert_eq!(definitions.len(), 1);
        assert_eq!(definitions[0].buffer, buffer_b2);
    }

    #[gpui::test(iterations = 10)]
    async fn test_collaborating_with_code_actions(
        cx_a: &mut TestAppContext,
        cx_b: &mut TestAppContext,
    ) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());
        cx_b.update(|cx| editor::init(cx));

        // Set up a fake language server.
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());
        lang_registry.add(Arc::new(language));

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "main.rs": "mod other;\nfn main() { let foo = other::foo(); }",
                "other.rs": "pub fn foo() -> usize { 4 }",
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/a", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join the worktree as client B.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();
        let mut params = cx_b.update(WorkspaceParams::test);
        params.languages = lang_registry.clone();
        params.client = client_b.client.clone();
        params.user_store = client_b.user_store.clone();
        params.project = project_b;

        let (_window_b, workspace_b) = cx_b.add_window(|cx| Workspace::new(&params, cx));
        let editor_b = workspace_b
            .update(cx_b, |workspace, cx| {
                workspace.open_path((worktree_id, "main.rs"), cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        let mut fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server
            .handle_request::<lsp::request::CodeActionRequest, _, _>(|params, _| async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path("/a/main.rs").unwrap(),
                );
                assert_eq!(params.range.start, lsp::Position::new(0, 0));
                assert_eq!(params.range.end, lsp::Position::new(0, 0));
                Ok(None)
            })
            .next()
            .await;

        // Move cursor to a location that contains code actions.
        editor_b.update(cx_b, |editor, cx| {
            editor.select_ranges([Point::new(1, 31)..Point::new(1, 31)], None, cx);
            cx.focus(&editor_b);
        });

        fake_language_server
            .handle_request::<lsp::request::CodeActionRequest, _, _>(|params, _| async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path("/a/main.rs").unwrap(),
                );
                assert_eq!(params.range.start, lsp::Position::new(1, 31));
                assert_eq!(params.range.end, lsp::Position::new(1, 31));

                Ok(Some(vec![lsp::CodeActionOrCommand::CodeAction(
                    lsp::CodeAction {
                        title: "Inline into all callers".to_string(),
                        edit: Some(lsp::WorkspaceEdit {
                            changes: Some(
                                [
                                    (
                                        lsp::Url::from_file_path("/a/main.rs").unwrap(),
                                        vec![lsp::TextEdit::new(
                                            lsp::Range::new(
                                                lsp::Position::new(1, 22),
                                                lsp::Position::new(1, 34),
                                            ),
                                            "4".to_string(),
                                        )],
                                    ),
                                    (
                                        lsp::Url::from_file_path("/a/other.rs").unwrap(),
                                        vec![lsp::TextEdit::new(
                                            lsp::Range::new(
                                                lsp::Position::new(0, 0),
                                                lsp::Position::new(0, 27),
                                            ),
                                            "".to_string(),
                                        )],
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            ..Default::default()
                        }),
                        data: Some(json!({
                            "codeActionParams": {
                                "range": {
                                    "start": {"line": 1, "column": 31},
                                    "end": {"line": 1, "column": 31},
                                }
                            }
                        })),
                        ..Default::default()
                    },
                )]))
            })
            .next()
            .await;

        // Toggle code actions and wait for them to display.
        editor_b.update(cx_b, |editor, cx| {
            editor.toggle_code_actions(&ToggleCodeActions(false), cx);
        });
        editor_b
            .condition(&cx_b, |editor, _| editor.context_menu_visible())
            .await;

        fake_language_server.remove_request_handler::<lsp::request::CodeActionRequest>();

        // Confirming the code action will trigger a resolve request.
        let confirm_action = workspace_b
            .update(cx_b, |workspace, cx| {
                Editor::confirm_code_action(workspace, &ConfirmCodeAction(Some(0)), cx)
            })
            .unwrap();
        fake_language_server.handle_request::<lsp::request::CodeActionResolveRequest, _, _>(
            |_, _| async move {
                Ok(lsp::CodeAction {
                    title: "Inline into all callers".to_string(),
                    edit: Some(lsp::WorkspaceEdit {
                        changes: Some(
                            [
                                (
                                    lsp::Url::from_file_path("/a/main.rs").unwrap(),
                                    vec![lsp::TextEdit::new(
                                        lsp::Range::new(
                                            lsp::Position::new(1, 22),
                                            lsp::Position::new(1, 34),
                                        ),
                                        "4".to_string(),
                                    )],
                                ),
                                (
                                    lsp::Url::from_file_path("/a/other.rs").unwrap(),
                                    vec![lsp::TextEdit::new(
                                        lsp::Range::new(
                                            lsp::Position::new(0, 0),
                                            lsp::Position::new(0, 27),
                                        ),
                                        "".to_string(),
                                    )],
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            },
        );

        // After the action is confirmed, an editor containing both modified files is opened.
        confirm_action.await.unwrap();
        let code_action_editor = workspace_b.read_with(cx_b, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .downcast::<Editor>()
                .unwrap()
        });
        code_action_editor.update(cx_b, |editor, cx| {
            assert_eq!(editor.text(cx), "\nmod other;\nfn main() { let foo = 4; }");
            editor.undo(&Undo, cx);
            assert_eq!(
                editor.text(cx),
                "pub fn foo() -> usize { 4 }\nmod other;\nfn main() { let foo = other::foo(); }"
            );
            editor.redo(&Redo, cx);
            assert_eq!(editor.text(cx), "\nmod other;\nfn main() { let foo = 4; }");
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_collaborating_with_renames(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());
        cx_b.update(|cx| editor::init(cx));

        // Set up a fake language server.
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());
        lang_registry.add(Arc::new(language));

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Share a project as client A
        fs.insert_tree(
            "/dir",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;"
            }),
        )
        .await;
        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/dir", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let project_id = project_a.update(cx_a, |p, _| p.next_remote_id()).await;
        let worktree_id = worktree_a.read_with(cx_a, |tree, _| tree.id());
        project_a.update(cx_a, |p, cx| p.share(cx)).await.unwrap();

        // Join the worktree as client B.
        let project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();
        let mut params = cx_b.update(WorkspaceParams::test);
        params.languages = lang_registry.clone();
        params.client = client_b.client.clone();
        params.user_store = client_b.user_store.clone();
        params.project = project_b;

        let (_window_b, workspace_b) = cx_b.add_window(|cx| Workspace::new(&params, cx));
        let editor_b = workspace_b
            .update(cx_b, |workspace, cx| {
                workspace.open_path((worktree_id, "one.rs"), cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let fake_language_server = fake_language_servers.next().await.unwrap();

        // Move cursor to a location that can be renamed.
        let prepare_rename = editor_b.update(cx_b, |editor, cx| {
            editor.select_ranges([7..7], None, cx);
            editor.rename(&Rename, cx).unwrap()
        });

        fake_language_server
            .handle_request::<lsp::request::PrepareRenameRequest, _, _>(|params, _| async move {
                assert_eq!(params.text_document.uri.as_str(), "file:///dir/one.rs");
                assert_eq!(params.position, lsp::Position::new(0, 7));
                Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
                    lsp::Position::new(0, 6),
                    lsp::Position::new(0, 9),
                ))))
            })
            .next()
            .await
            .unwrap();
        prepare_rename.await.unwrap();
        editor_b.update(cx_b, |editor, cx| {
            let rename = editor.pending_rename().unwrap();
            let buffer = editor.buffer().read(cx).snapshot(cx);
            assert_eq!(
                rename.range.start.to_offset(&buffer)..rename.range.end.to_offset(&buffer),
                6..9
            );
            rename.editor.update(cx, |rename_editor, cx| {
                rename_editor.buffer().update(cx, |rename_buffer, cx| {
                    rename_buffer.edit([0..3], "THREE", cx);
                });
            });
        });

        let confirm_rename = workspace_b.update(cx_b, |workspace, cx| {
            Editor::confirm_rename(workspace, &ConfirmRename, cx).unwrap()
        });
        fake_language_server
            .handle_request::<lsp::request::Rename, _, _>(|params, _| async move {
                assert_eq!(
                    params.text_document_position.text_document.uri.as_str(),
                    "file:///dir/one.rs"
                );
                assert_eq!(
                    params.text_document_position.position,
                    lsp::Position::new(0, 6)
                );
                assert_eq!(params.new_name, "THREE");
                Ok(Some(lsp::WorkspaceEdit {
                    changes: Some(
                        [
                            (
                                lsp::Url::from_file_path("/dir/one.rs").unwrap(),
                                vec![lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(0, 6),
                                        lsp::Position::new(0, 9),
                                    ),
                                    "THREE".to_string(),
                                )],
                            ),
                            (
                                lsp::Url::from_file_path("/dir/two.rs").unwrap(),
                                vec![
                                    lsp::TextEdit::new(
                                        lsp::Range::new(
                                            lsp::Position::new(0, 24),
                                            lsp::Position::new(0, 27),
                                        ),
                                        "THREE".to_string(),
                                    ),
                                    lsp::TextEdit::new(
                                        lsp::Range::new(
                                            lsp::Position::new(0, 35),
                                            lsp::Position::new(0, 38),
                                        ),
                                        "THREE".to_string(),
                                    ),
                                ],
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                    ..Default::default()
                }))
            })
            .next()
            .await
            .unwrap();
        confirm_rename.await.unwrap();

        let rename_editor = workspace_b.read_with(cx_b, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .downcast::<Editor>()
                .unwrap()
        });
        rename_editor.update(cx_b, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "const TWO: usize = one::THREE + one::THREE;\nconst THREE: usize = 1;"
            );
            editor.undo(&Undo, cx);
            assert_eq!(
                editor.text(cx),
                "const TWO: usize = one::ONE + one::ONE;\nconst ONE: usize = 1;"
            );
            editor.redo(&Redo, cx);
            assert_eq!(
                editor.text(cx),
                "const TWO: usize = one::THREE + one::THREE;\nconst THREE: usize = 1;"
            );
        });

        // Ensure temporary rename edits cannot be undone/redone.
        editor_b.update(cx_b, |editor, cx| {
            editor.undo(&Undo, cx);
            assert_eq!(editor.text(cx), "const ONE: usize = 1;");
            editor.undo(&Undo, cx);
            assert_eq!(editor.text(cx), "const ONE: usize = 1;");
            editor.redo(&Redo, cx);
            assert_eq!(editor.text(cx), "const THREE: usize = 1;");
        })
    }

    #[gpui::test(iterations = 10)]
    async fn test_basic_chat(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;

        // Create an org that includes these 2 users.
        let db = &server.app_state.db;
        let org_id = db.create_org("Test Org", "test-org").await.unwrap();
        db.add_org_member(org_id, client_a.current_user_id(&cx_a), false)
            .await
            .unwrap();
        db.add_org_member(org_id, client_b.current_user_id(&cx_b), false)
            .await
            .unwrap();

        // Create a channel that includes all the users.
        let channel_id = db.create_org_channel(org_id, "test-channel").await.unwrap();
        db.add_channel_member(channel_id, client_a.current_user_id(&cx_a), false)
            .await
            .unwrap();
        db.add_channel_member(channel_id, client_b.current_user_id(&cx_b), false)
            .await
            .unwrap();
        db.create_channel_message(
            channel_id,
            client_b.current_user_id(&cx_b),
            "hello A, it's B.",
            OffsetDateTime::now_utc(),
            1,
        )
        .await
        .unwrap();

        let channels_a = cx_a
            .add_model(|cx| ChannelList::new(client_a.user_store.clone(), client_a.clone(), cx));
        channels_a
            .condition(cx_a, |list, _| list.available_channels().is_some())
            .await;
        channels_a.read_with(cx_a, |list, _| {
            assert_eq!(
                list.available_channels().unwrap(),
                &[ChannelDetails {
                    id: channel_id.to_proto(),
                    name: "test-channel".to_string()
                }]
            )
        });
        let channel_a = channels_a.update(cx_a, |this, cx| {
            this.get_channel(channel_id.to_proto(), cx).unwrap()
        });
        channel_a.read_with(cx_a, |channel, _| assert!(channel.messages().is_empty()));
        channel_a
            .condition(&cx_a, |channel, _| {
                channel_messages(channel)
                    == [("user_b".to_string(), "hello A, it's B.".to_string(), false)]
            })
            .await;

        let channels_b = cx_b
            .add_model(|cx| ChannelList::new(client_b.user_store.clone(), client_b.clone(), cx));
        channels_b
            .condition(cx_b, |list, _| list.available_channels().is_some())
            .await;
        channels_b.read_with(cx_b, |list, _| {
            assert_eq!(
                list.available_channels().unwrap(),
                &[ChannelDetails {
                    id: channel_id.to_proto(),
                    name: "test-channel".to_string()
                }]
            )
        });

        let channel_b = channels_b.update(cx_b, |this, cx| {
            this.get_channel(channel_id.to_proto(), cx).unwrap()
        });
        channel_b.read_with(cx_b, |channel, _| assert!(channel.messages().is_empty()));
        channel_b
            .condition(&cx_b, |channel, _| {
                channel_messages(channel)
                    == [("user_b".to_string(), "hello A, it's B.".to_string(), false)]
            })
            .await;

        channel_a
            .update(cx_a, |channel, cx| {
                channel
                    .send_message("oh, hi B.".to_string(), cx)
                    .unwrap()
                    .detach();
                let task = channel.send_message("sup".to_string(), cx).unwrap();
                assert_eq!(
                    channel_messages(channel),
                    &[
                        ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                        ("user_a".to_string(), "oh, hi B.".to_string(), true),
                        ("user_a".to_string(), "sup".to_string(), true)
                    ]
                );
                task
            })
            .await
            .unwrap();

        channel_b
            .condition(&cx_b, |channel, _| {
                channel_messages(channel)
                    == [
                        ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                        ("user_a".to_string(), "oh, hi B.".to_string(), false),
                        ("user_a".to_string(), "sup".to_string(), false),
                    ]
            })
            .await;

        assert_eq!(
            server
                .state()
                .await
                .channel(channel_id)
                .unwrap()
                .connection_ids
                .len(),
            2
        );
        cx_b.update(|_| drop(channel_b));
        server
            .condition(|state| state.channel(channel_id).unwrap().connection_ids.len() == 1)
            .await;

        cx_a.update(|_| drop(channel_a));
        server
            .condition(|state| state.channel(channel_id).is_none())
            .await;
    }

    #[gpui::test(iterations = 10)]
    async fn test_chat_message_validation(cx_a: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();

        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;

        let db = &server.app_state.db;
        let org_id = db.create_org("Test Org", "test-org").await.unwrap();
        let channel_id = db.create_org_channel(org_id, "test-channel").await.unwrap();
        db.add_org_member(org_id, client_a.current_user_id(&cx_a), false)
            .await
            .unwrap();
        db.add_channel_member(channel_id, client_a.current_user_id(&cx_a), false)
            .await
            .unwrap();

        let channels_a = cx_a
            .add_model(|cx| ChannelList::new(client_a.user_store.clone(), client_a.clone(), cx));
        channels_a
            .condition(cx_a, |list, _| list.available_channels().is_some())
            .await;
        let channel_a = channels_a.update(cx_a, |this, cx| {
            this.get_channel(channel_id.to_proto(), cx).unwrap()
        });

        // Messages aren't allowed to be too long.
        channel_a
            .update(cx_a, |channel, cx| {
                let long_body = "this is long.\n".repeat(1024);
                channel.send_message(long_body, cx).unwrap()
            })
            .await
            .unwrap_err();

        // Messages aren't allowed to be blank.
        channel_a.update(cx_a, |channel, cx| {
            channel.send_message(String::new(), cx).unwrap_err()
        });

        // Leading and trailing whitespace are trimmed.
        channel_a
            .update(cx_a, |channel, cx| {
                channel
                    .send_message("\n surrounded by whitespace  \n".to_string(), cx)
                    .unwrap()
            })
            .await
            .unwrap();
        assert_eq!(
            db.get_channel_messages(channel_id, 10, None)
                .await
                .unwrap()
                .iter()
                .map(|m| &m.body)
                .collect::<Vec<_>>(),
            &["surrounded by whitespace"]
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_chat_reconnection(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();

        // Connect to a server as 2 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;
        let mut status_b = client_b.status();

        // Create an org that includes these 2 users.
        let db = &server.app_state.db;
        let org_id = db.create_org("Test Org", "test-org").await.unwrap();
        db.add_org_member(org_id, client_a.current_user_id(&cx_a), false)
            .await
            .unwrap();
        db.add_org_member(org_id, client_b.current_user_id(&cx_b), false)
            .await
            .unwrap();

        // Create a channel that includes all the users.
        let channel_id = db.create_org_channel(org_id, "test-channel").await.unwrap();
        db.add_channel_member(channel_id, client_a.current_user_id(&cx_a), false)
            .await
            .unwrap();
        db.add_channel_member(channel_id, client_b.current_user_id(&cx_b), false)
            .await
            .unwrap();
        db.create_channel_message(
            channel_id,
            client_b.current_user_id(&cx_b),
            "hello A, it's B.",
            OffsetDateTime::now_utc(),
            2,
        )
        .await
        .unwrap();

        let channels_a = cx_a
            .add_model(|cx| ChannelList::new(client_a.user_store.clone(), client_a.clone(), cx));
        channels_a
            .condition(cx_a, |list, _| list.available_channels().is_some())
            .await;

        channels_a.read_with(cx_a, |list, _| {
            assert_eq!(
                list.available_channels().unwrap(),
                &[ChannelDetails {
                    id: channel_id.to_proto(),
                    name: "test-channel".to_string()
                }]
            )
        });
        let channel_a = channels_a.update(cx_a, |this, cx| {
            this.get_channel(channel_id.to_proto(), cx).unwrap()
        });
        channel_a.read_with(cx_a, |channel, _| assert!(channel.messages().is_empty()));
        channel_a
            .condition(&cx_a, |channel, _| {
                channel_messages(channel)
                    == [("user_b".to_string(), "hello A, it's B.".to_string(), false)]
            })
            .await;

        let channels_b = cx_b
            .add_model(|cx| ChannelList::new(client_b.user_store.clone(), client_b.clone(), cx));
        channels_b
            .condition(cx_b, |list, _| list.available_channels().is_some())
            .await;
        channels_b.read_with(cx_b, |list, _| {
            assert_eq!(
                list.available_channels().unwrap(),
                &[ChannelDetails {
                    id: channel_id.to_proto(),
                    name: "test-channel".to_string()
                }]
            )
        });

        let channel_b = channels_b.update(cx_b, |this, cx| {
            this.get_channel(channel_id.to_proto(), cx).unwrap()
        });
        channel_b.read_with(cx_b, |channel, _| assert!(channel.messages().is_empty()));
        channel_b
            .condition(&cx_b, |channel, _| {
                channel_messages(channel)
                    == [("user_b".to_string(), "hello A, it's B.".to_string(), false)]
            })
            .await;

        // Disconnect client B, ensuring we can still access its cached channel data.
        server.forbid_connections();
        server.disconnect_client(client_b.current_user_id(&cx_b));
        cx_b.foreground().advance_clock(Duration::from_secs(3));
        while !matches!(
            status_b.next().await,
            Some(client::Status::ReconnectionError { .. })
        ) {}

        channels_b.read_with(cx_b, |channels, _| {
            assert_eq!(
                channels.available_channels().unwrap(),
                [ChannelDetails {
                    id: channel_id.to_proto(),
                    name: "test-channel".to_string()
                }]
            )
        });
        channel_b.read_with(cx_b, |channel, _| {
            assert_eq!(
                channel_messages(channel),
                [("user_b".to_string(), "hello A, it's B.".to_string(), false)]
            )
        });

        // Send a message from client B while it is disconnected.
        channel_b
            .update(cx_b, |channel, cx| {
                let task = channel
                    .send_message("can you see this?".to_string(), cx)
                    .unwrap();
                assert_eq!(
                    channel_messages(channel),
                    &[
                        ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                        ("user_b".to_string(), "can you see this?".to_string(), true)
                    ]
                );
                task
            })
            .await
            .unwrap_err();

        // Send a message from client A while B is disconnected.
        channel_a
            .update(cx_a, |channel, cx| {
                channel
                    .send_message("oh, hi B.".to_string(), cx)
                    .unwrap()
                    .detach();
                let task = channel.send_message("sup".to_string(), cx).unwrap();
                assert_eq!(
                    channel_messages(channel),
                    &[
                        ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                        ("user_a".to_string(), "oh, hi B.".to_string(), true),
                        ("user_a".to_string(), "sup".to_string(), true)
                    ]
                );
                task
            })
            .await
            .unwrap();

        // Give client B a chance to reconnect.
        server.allow_connections();
        cx_b.foreground().advance_clock(Duration::from_secs(10));

        // Verify that B sees the new messages upon reconnection, as well as the message client B
        // sent while offline.
        channel_b
            .condition(&cx_b, |channel, _| {
                channel_messages(channel)
                    == [
                        ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                        ("user_a".to_string(), "oh, hi B.".to_string(), false),
                        ("user_a".to_string(), "sup".to_string(), false),
                        ("user_b".to_string(), "can you see this?".to_string(), false),
                    ]
            })
            .await;

        // Ensure client A and B can communicate normally after reconnection.
        channel_a
            .update(cx_a, |channel, cx| {
                channel.send_message("you online?".to_string(), cx).unwrap()
            })
            .await
            .unwrap();
        channel_b
            .condition(&cx_b, |channel, _| {
                channel_messages(channel)
                    == [
                        ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                        ("user_a".to_string(), "oh, hi B.".to_string(), false),
                        ("user_a".to_string(), "sup".to_string(), false),
                        ("user_b".to_string(), "can you see this?".to_string(), false),
                        ("user_a".to_string(), "you online?".to_string(), false),
                    ]
            })
            .await;

        channel_b
            .update(cx_b, |channel, cx| {
                channel.send_message("yep".to_string(), cx).unwrap()
            })
            .await
            .unwrap();
        channel_a
            .condition(&cx_a, |channel, _| {
                channel_messages(channel)
                    == [
                        ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                        ("user_a".to_string(), "oh, hi B.".to_string(), false),
                        ("user_a".to_string(), "sup".to_string(), false),
                        ("user_b".to_string(), "can you see this?".to_string(), false),
                        ("user_a".to_string(), "you online?".to_string(), false),
                        ("user_b".to_string(), "yep".to_string(), false),
                    ]
            })
            .await;
    }

    #[gpui::test(iterations = 10)]
    async fn test_contacts(
        cx_a: &mut TestAppContext,
        cx_b: &mut TestAppContext,
        cx_c: &mut TestAppContext,
    ) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::test());
        let fs = FakeFs::new(cx_a.background());

        // Connect to a server as 3 clients.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let client_a = server.create_client(cx_a, "user_a").await;
        let client_b = server.create_client(cx_b, "user_b").await;
        let client_c = server.create_client(cx_c, "user_c").await;

        // Share a worktree as client A.
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b", "user_c"]"#,
            }),
        )
        .await;

        let project_a = cx_a.update(|cx| {
            Project::local(
                client_a.clone(),
                client_a.user_store.clone(),
                lang_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let (worktree_a, _) = project_a
            .update(cx_a, |p, cx| {
                p.find_or_create_local_worktree("/a", true, cx)
            })
            .await
            .unwrap();
        worktree_a
            .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;

        client_a
            .user_store
            .condition(&cx_a, |user_store, _| {
                contacts(user_store) == vec![("user_a", vec![("a", vec![])])]
            })
            .await;
        client_b
            .user_store
            .condition(&cx_b, |user_store, _| {
                contacts(user_store) == vec![("user_a", vec![("a", vec![])])]
            })
            .await;
        client_c
            .user_store
            .condition(&cx_c, |user_store, _| {
                contacts(user_store) == vec![("user_a", vec![("a", vec![])])]
            })
            .await;

        let project_id = project_a
            .update(cx_a, |project, _| project.next_remote_id())
            .await;
        project_a
            .update(cx_a, |project, cx| project.share(cx))
            .await
            .unwrap();

        let _project_b = Project::remote(
            project_id,
            client_b.clone(),
            client_b.user_store.clone(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        client_a
            .user_store
            .condition(&cx_a, |user_store, _| {
                contacts(user_store) == vec![("user_a", vec![("a", vec!["user_b"])])]
            })
            .await;
        client_b
            .user_store
            .condition(&cx_b, |user_store, _| {
                contacts(user_store) == vec![("user_a", vec![("a", vec!["user_b"])])]
            })
            .await;
        client_c
            .user_store
            .condition(&cx_c, |user_store, _| {
                contacts(user_store) == vec![("user_a", vec![("a", vec!["user_b"])])]
            })
            .await;

        project_a
            .condition(&cx_a, |project, _| {
                project.collaborators().contains_key(&client_b.peer_id)
            })
            .await;

        cx_a.update(move |_| drop(project_a));
        client_a
            .user_store
            .condition(&cx_a, |user_store, _| contacts(user_store) == vec![])
            .await;
        client_b
            .user_store
            .condition(&cx_b, |user_store, _| contacts(user_store) == vec![])
            .await;
        client_c
            .user_store
            .condition(&cx_c, |user_store, _| contacts(user_store) == vec![])
            .await;

        fn contacts(user_store: &UserStore) -> Vec<(&str, Vec<(&str, Vec<&str>)>)> {
            user_store
                .contacts()
                .iter()
                .map(|contact| {
                    let worktrees = contact
                        .projects
                        .iter()
                        .map(|p| {
                            (
                                p.worktree_root_names[0].as_str(),
                                p.guests.iter().map(|p| p.github_login.as_str()).collect(),
                            )
                        })
                        .collect();
                    (contact.user.github_login.as_str(), worktrees)
                })
                .collect()
        }
    }

    #[gpui::test(iterations = 10)]
    async fn test_following(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let fs = FakeFs::new(cx_a.background());

        // 2 clients connect to a server.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let mut client_a = server.create_client(cx_a, "user_a").await;
        let mut client_b = server.create_client(cx_b, "user_b").await;
        cx_a.update(editor::init);
        cx_b.update(editor::init);

        // Client A shares a project.
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "1.txt": "one",
                "2.txt": "two",
                "3.txt": "three",
            }),
        )
        .await;
        let (project_a, worktree_id) = client_a.build_local_project(fs.clone(), "/a", cx_a).await;
        project_a
            .update(cx_a, |project, cx| project.share(cx))
            .await
            .unwrap();

        // Client B joins the project.
        let project_b = client_b
            .build_remote_project(
                project_a
                    .read_with(cx_a, |project, _| project.remote_id())
                    .unwrap(),
                cx_b,
            )
            .await;

        // Client A opens some editors.
        let workspace_a = client_a.build_workspace(&project_a, cx_a);
        let pane_a = workspace_a.read_with(cx_a, |workspace, _| workspace.active_pane().clone());
        let editor_a1 = workspace_a
            .update(cx_a, |workspace, cx| {
                workspace.open_path((worktree_id, "1.txt"), cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let editor_a2 = workspace_a
            .update(cx_a, |workspace, cx| {
                workspace.open_path((worktree_id, "2.txt"), cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        // Client B opens an editor.
        let workspace_b = client_b.build_workspace(&project_b, cx_b);
        let editor_b1 = workspace_b
            .update(cx_b, |workspace, cx| {
                workspace.open_path((worktree_id, "1.txt"), cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        let client_a_id = project_b.read_with(cx_b, |project, _| {
            project.collaborators().values().next().unwrap().peer_id
        });
        let client_b_id = project_a.read_with(cx_a, |project, _| {
            project.collaborators().values().next().unwrap().peer_id
        });

        // When client B starts following client A, all visible view states are replicated to client B.
        editor_a1.update(cx_a, |editor, cx| editor.select_ranges([0..1], None, cx));
        editor_a2.update(cx_a, |editor, cx| editor.select_ranges([2..3], None, cx));
        workspace_b
            .update(cx_b, |workspace, cx| {
                workspace
                    .toggle_follow(&ToggleFollow(client_a_id), cx)
                    .unwrap()
            })
            .await
            .unwrap();
        let editor_b2 = workspace_b.read_with(cx_b, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .downcast::<Editor>()
                .unwrap()
        });
        assert!(cx_b.read(|cx| editor_b2.is_focused(cx)));
        assert_eq!(
            editor_b2.read_with(cx_b, |editor, cx| editor.project_path(cx)),
            Some((worktree_id, "2.txt").into())
        );
        assert_eq!(
            editor_b2.read_with(cx_b, |editor, cx| editor.selected_ranges(cx)),
            vec![2..3]
        );
        assert_eq!(
            editor_b1.read_with(cx_b, |editor, cx| editor.selected_ranges(cx)),
            vec![0..1]
        );

        // When client A activates a different editor, client B does so as well.
        workspace_a.update(cx_a, |workspace, cx| {
            workspace.activate_item(&editor_a1, cx)
        });
        workspace_b
            .condition(cx_b, |workspace, cx| {
                workspace.active_item(cx).unwrap().id() == editor_b1.id()
            })
            .await;

        // When client A navigates back and forth, client B does so as well.
        workspace_a
            .update(cx_a, |workspace, cx| {
                workspace::Pane::go_back(workspace, None, cx)
            })
            .await;
        workspace_b
            .condition(cx_b, |workspace, cx| {
                workspace.active_item(cx).unwrap().id() == editor_b2.id()
            })
            .await;

        workspace_a
            .update(cx_a, |workspace, cx| {
                workspace::Pane::go_forward(workspace, None, cx)
            })
            .await;
        workspace_b
            .condition(cx_b, |workspace, cx| {
                workspace.active_item(cx).unwrap().id() == editor_b1.id()
            })
            .await;

        // Changes to client A's editor are reflected on client B.
        editor_a1.update(cx_a, |editor, cx| {
            editor.select_ranges([1..1, 2..2], None, cx);
        });
        editor_b1
            .condition(cx_b, |editor, cx| {
                editor.selected_ranges(cx) == vec![1..1, 2..2]
            })
            .await;

        editor_a1.update(cx_a, |editor, cx| editor.set_text("TWO", cx));
        editor_b1
            .condition(cx_b, |editor, cx| editor.text(cx) == "TWO")
            .await;

        editor_a1.update(cx_a, |editor, cx| {
            editor.select_ranges([3..3], None, cx);
            editor.set_scroll_position(vec2f(0., 100.), cx);
        });
        editor_b1
            .condition(cx_b, |editor, cx| editor.selected_ranges(cx) == vec![3..3])
            .await;

        // After unfollowing, client B stops receiving updates from client A.
        workspace_b.update(cx_b, |workspace, cx| {
            workspace.unfollow(&workspace.active_pane().clone(), cx)
        });
        workspace_a.update(cx_a, |workspace, cx| {
            workspace.activate_item(&editor_a2, cx)
        });
        cx_a.foreground().run_until_parked();
        assert_eq!(
            workspace_b.read_with(cx_b, |workspace, cx| workspace
                .active_item(cx)
                .unwrap()
                .id()),
            editor_b1.id()
        );

        // Client A starts following client B.
        workspace_a
            .update(cx_a, |workspace, cx| {
                workspace
                    .toggle_follow(&ToggleFollow(client_b_id), cx)
                    .unwrap()
            })
            .await
            .unwrap();
        assert_eq!(
            workspace_a.read_with(cx_a, |workspace, _| workspace.leader_for_pane(&pane_a)),
            Some(client_b_id)
        );
        assert_eq!(
            workspace_a.read_with(cx_a, |workspace, cx| workspace
                .active_item(cx)
                .unwrap()
                .id()),
            editor_a1.id()
        );

        // Following interrupts when client B disconnects.
        client_b.disconnect(&cx_b.to_async()).unwrap();
        cx_a.foreground().run_until_parked();
        assert_eq!(
            workspace_a.read_with(cx_a, |workspace, _| workspace.leader_for_pane(&pane_a)),
            None
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_peers_following_each_other(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let fs = FakeFs::new(cx_a.background());

        // 2 clients connect to a server.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let mut client_a = server.create_client(cx_a, "user_a").await;
        let mut client_b = server.create_client(cx_b, "user_b").await;
        cx_a.update(editor::init);
        cx_b.update(editor::init);

        // Client A shares a project.
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "1.txt": "one",
                "2.txt": "two",
                "3.txt": "three",
                "4.txt": "four",
            }),
        )
        .await;
        let (project_a, worktree_id) = client_a.build_local_project(fs.clone(), "/a", cx_a).await;
        project_a
            .update(cx_a, |project, cx| project.share(cx))
            .await
            .unwrap();

        // Client B joins the project.
        let project_b = client_b
            .build_remote_project(
                project_a
                    .read_with(cx_a, |project, _| project.remote_id())
                    .unwrap(),
                cx_b,
            )
            .await;

        // Client A opens some editors.
        let workspace_a = client_a.build_workspace(&project_a, cx_a);
        let pane_a1 = workspace_a.read_with(cx_a, |workspace, _| workspace.active_pane().clone());
        let _editor_a1 = workspace_a
            .update(cx_a, |workspace, cx| {
                workspace.open_path((worktree_id, "1.txt"), cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        // Client B opens an editor.
        let workspace_b = client_b.build_workspace(&project_b, cx_b);
        let pane_b1 = workspace_b.read_with(cx_b, |workspace, _| workspace.active_pane().clone());
        let _editor_b1 = workspace_b
            .update(cx_b, |workspace, cx| {
                workspace.open_path((worktree_id, "2.txt"), cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        // Clients A and B follow each other in split panes
        workspace_a
            .update(cx_a, |workspace, cx| {
                workspace.split_pane(workspace.active_pane().clone(), SplitDirection::Right, cx);
                assert_ne!(*workspace.active_pane(), pane_a1);
                let leader_id = *project_a.read(cx).collaborators().keys().next().unwrap();
                workspace
                    .toggle_follow(&workspace::ToggleFollow(leader_id), cx)
                    .unwrap()
            })
            .await
            .unwrap();
        workspace_b
            .update(cx_b, |workspace, cx| {
                workspace.split_pane(workspace.active_pane().clone(), SplitDirection::Right, cx);
                assert_ne!(*workspace.active_pane(), pane_b1);
                let leader_id = *project_b.read(cx).collaborators().keys().next().unwrap();
                workspace
                    .toggle_follow(&workspace::ToggleFollow(leader_id), cx)
                    .unwrap()
            })
            .await
            .unwrap();

        workspace_a
            .update(cx_a, |workspace, cx| {
                workspace.activate_next_pane(cx);
                assert_eq!(*workspace.active_pane(), pane_a1);
                workspace.open_path((worktree_id, "3.txt"), cx)
            })
            .await
            .unwrap();
        workspace_b
            .update(cx_b, |workspace, cx| {
                workspace.activate_next_pane(cx);
                assert_eq!(*workspace.active_pane(), pane_b1);
                workspace.open_path((worktree_id, "4.txt"), cx)
            })
            .await
            .unwrap();
        cx_a.foreground().run_until_parked();

        // Ensure leader updates don't change the active pane of followers
        workspace_a.read_with(cx_a, |workspace, _| {
            assert_eq!(*workspace.active_pane(), pane_a1);
        });
        workspace_b.read_with(cx_b, |workspace, _| {
            assert_eq!(*workspace.active_pane(), pane_b1);
        });

        // Ensure peers following each other doesn't cause an infinite loop.
        assert_eq!(
            workspace_a.read_with(cx_a, |workspace, cx| workspace
                .active_item(cx)
                .unwrap()
                .project_path(cx)),
            Some((worktree_id, "3.txt").into())
        );
        workspace_a.update(cx_a, |workspace, cx| {
            assert_eq!(
                workspace.active_item(cx).unwrap().project_path(cx),
                Some((worktree_id, "3.txt").into())
            );
            workspace.activate_next_pane(cx);
            assert_eq!(
                workspace.active_item(cx).unwrap().project_path(cx),
                Some((worktree_id, "4.txt").into())
            );
        });
        workspace_b.update(cx_b, |workspace, cx| {
            assert_eq!(
                workspace.active_item(cx).unwrap().project_path(cx),
                Some((worktree_id, "4.txt").into())
            );
            workspace.activate_next_pane(cx);
            assert_eq!(
                workspace.active_item(cx).unwrap().project_path(cx),
                Some((worktree_id, "3.txt").into())
            );
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_auto_unfollowing(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
        cx_a.foreground().forbid_parking();
        let fs = FakeFs::new(cx_a.background());

        // 2 clients connect to a server.
        let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
        let mut client_a = server.create_client(cx_a, "user_a").await;
        let mut client_b = server.create_client(cx_b, "user_b").await;
        cx_a.update(editor::init);
        cx_b.update(editor::init);

        // Client A shares a project.
        fs.insert_tree(
            "/a",
            json!({
                ".zed.toml": r#"collaborators = ["user_b"]"#,
                "1.txt": "one",
                "2.txt": "two",
                "3.txt": "three",
            }),
        )
        .await;
        let (project_a, worktree_id) = client_a.build_local_project(fs.clone(), "/a", cx_a).await;
        project_a
            .update(cx_a, |project, cx| project.share(cx))
            .await
            .unwrap();

        // Client B joins the project.
        let project_b = client_b
            .build_remote_project(
                project_a
                    .read_with(cx_a, |project, _| project.remote_id())
                    .unwrap(),
                cx_b,
            )
            .await;

        // Client A opens some editors.
        let workspace_a = client_a.build_workspace(&project_a, cx_a);
        let _editor_a1 = workspace_a
            .update(cx_a, |workspace, cx| {
                workspace.open_path((worktree_id, "1.txt"), cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        // Client B starts following client A.
        let workspace_b = client_b.build_workspace(&project_b, cx_b);
        let pane_b = workspace_b.read_with(cx_b, |workspace, _| workspace.active_pane().clone());
        let leader_id = project_b.read_with(cx_b, |project, _| {
            project.collaborators().values().next().unwrap().peer_id
        });
        workspace_b
            .update(cx_b, |workspace, cx| {
                workspace
                    .toggle_follow(&ToggleFollow(leader_id), cx)
                    .unwrap()
            })
            .await
            .unwrap();
        assert_eq!(
            workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
            Some(leader_id)
        );
        let editor_b2 = workspace_b.read_with(cx_b, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .downcast::<Editor>()
                .unwrap()
        });

        // When client B moves, it automatically stops following client A.
        editor_b2.update(cx_b, |editor, cx| editor.move_right(&editor::MoveRight, cx));
        assert_eq!(
            workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
            None
        );

        workspace_b
            .update(cx_b, |workspace, cx| {
                workspace
                    .toggle_follow(&ToggleFollow(leader_id), cx)
                    .unwrap()
            })
            .await
            .unwrap();
        assert_eq!(
            workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
            Some(leader_id)
        );

        // When client B edits, it automatically stops following client A.
        editor_b2.update(cx_b, |editor, cx| editor.insert("X", cx));
        assert_eq!(
            workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
            None
        );

        workspace_b
            .update(cx_b, |workspace, cx| {
                workspace
                    .toggle_follow(&ToggleFollow(leader_id), cx)
                    .unwrap()
            })
            .await
            .unwrap();
        assert_eq!(
            workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
            Some(leader_id)
        );

        // When client B scrolls, it automatically stops following client A.
        editor_b2.update(cx_b, |editor, cx| {
            editor.set_scroll_position(vec2f(0., 3.), cx)
        });
        assert_eq!(
            workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
            None
        );

        workspace_b
            .update(cx_b, |workspace, cx| {
                workspace
                    .toggle_follow(&ToggleFollow(leader_id), cx)
                    .unwrap()
            })
            .await
            .unwrap();
        assert_eq!(
            workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
            Some(leader_id)
        );

        // When client B activates a different pane, it continues following client A in the original pane.
        workspace_b.update(cx_b, |workspace, cx| {
            workspace.split_pane(pane_b.clone(), SplitDirection::Right, cx)
        });
        assert_eq!(
            workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
            Some(leader_id)
        );

        workspace_b.update(cx_b, |workspace, cx| workspace.activate_next_pane(cx));
        assert_eq!(
            workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
            Some(leader_id)
        );

        // When client B activates a different item in the original pane, it automatically stops following client A.
        workspace_b
            .update(cx_b, |workspace, cx| {
                workspace.open_path((worktree_id, "2.txt"), cx)
            })
            .await
            .unwrap();
        assert_eq!(
            workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
            None
        );
    }

    #[gpui::test(iterations = 100)]
    async fn test_random_collaboration(cx: &mut TestAppContext, rng: StdRng) {
        cx.foreground().forbid_parking();
        let max_peers = env::var("MAX_PEERS")
            .map(|i| i.parse().expect("invalid `MAX_PEERS` variable"))
            .unwrap_or(5);
        let max_operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let rng = Arc::new(Mutex::new(rng));

        let guest_lang_registry = Arc::new(LanguageRegistry::test());
        let host_language_registry = Arc::new(LanguageRegistry::test());

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/_collab",
            json!({
                ".zed.toml": r#"collaborators = ["guest-1", "guest-2", "guest-3", "guest-4", "guest-5"]"#
            }),
        )
        .await;

        let operations = Rc::new(Cell::new(0));
        let mut server = TestServer::start(cx.foreground(), cx.background()).await;
        let mut clients = Vec::new();
        let mut user_ids = Vec::new();
        let files = Arc::new(Mutex::new(Vec::new()));

        let mut next_entity_id = 100000;
        let mut host_cx = TestAppContext::new(
            cx.foreground_platform(),
            cx.platform(),
            cx.foreground(),
            cx.background(),
            cx.font_cache(),
            cx.leak_detector(),
            next_entity_id,
        );
        let host = server.create_client(&mut host_cx, "host").await;
        let host_project = host_cx.update(|cx| {
            Project::local(
                host.client.clone(),
                host.user_store.clone(),
                host_language_registry.clone(),
                fs.clone(),
                cx,
            )
        });
        let host_project_id = host_project
            .update(&mut host_cx, |p, _| p.next_remote_id())
            .await;

        let (collab_worktree, _) = host_project
            .update(&mut host_cx, |project, cx| {
                project.find_or_create_local_worktree("/_collab", true, cx)
            })
            .await
            .unwrap();
        collab_worktree
            .read_with(&host_cx, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        host_project
            .update(&mut host_cx, |project, cx| project.share(cx))
            .await
            .unwrap();

        // Set up fake language servers.
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            None,
        );
        let _fake_servers = language.set_fake_lsp_adapter(FakeLspAdapter {
            name: "the-fake-language-server",
            capabilities: lsp::LanguageServer::full_capabilities(),
            initializer: Some(Box::new({
                let rng = rng.clone();
                let files = files.clone();
                let project = host_project.downgrade();
                move |fake_server: &mut FakeLanguageServer| {
                    fake_server.handle_request::<lsp::request::Completion, _, _>(
                        |_, _| async move {
                            Ok(Some(lsp::CompletionResponse::Array(vec![
                                lsp::CompletionItem {
                                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                                        range: lsp::Range::new(
                                            lsp::Position::new(0, 0),
                                            lsp::Position::new(0, 0),
                                        ),
                                        new_text: "the-new-text".to_string(),
                                    })),
                                    ..Default::default()
                                },
                            ])))
                        },
                    );

                    fake_server.handle_request::<lsp::request::CodeActionRequest, _, _>(
                        |_, _| async move {
                            Ok(Some(vec![lsp::CodeActionOrCommand::CodeAction(
                                lsp::CodeAction {
                                    title: "the-code-action".to_string(),
                                    ..Default::default()
                                },
                            )]))
                        },
                    );

                    fake_server.handle_request::<lsp::request::PrepareRenameRequest, _, _>(
                        |params, _| async move {
                            Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
                                params.position,
                                params.position,
                            ))))
                        },
                    );

                    fake_server.handle_request::<lsp::request::GotoDefinition, _, _>({
                        let files = files.clone();
                        let rng = rng.clone();
                        move |_, _| {
                            let files = files.clone();
                            let rng = rng.clone();
                            async move {
                                let files = files.lock();
                                let mut rng = rng.lock();
                                let count = rng.gen_range::<usize, _>(1..3);
                                let files = (0..count)
                                    .map(|_| files.choose(&mut *rng).unwrap())
                                    .collect::<Vec<_>>();
                                log::info!("LSP: Returning definitions in files {:?}", &files);
                                Ok(Some(lsp::GotoDefinitionResponse::Array(
                                    files
                                        .into_iter()
                                        .map(|file| lsp::Location {
                                            uri: lsp::Url::from_file_path(file).unwrap(),
                                            range: Default::default(),
                                        })
                                        .collect(),
                                )))
                            }
                        }
                    });

                    fake_server.handle_request::<lsp::request::DocumentHighlightRequest, _, _>({
                        let rng = rng.clone();
                        let project = project.clone();
                        move |params, mut cx| {
                            let highlights = if let Some(project) = project.upgrade(&cx) {
                                project.update(&mut cx, |project, cx| {
                                    let path = params
                                        .text_document_position_params
                                        .text_document
                                        .uri
                                        .to_file_path()
                                        .unwrap();
                                    let (worktree, relative_path) =
                                        project.find_local_worktree(&path, cx)?;
                                    let project_path =
                                        ProjectPath::from((worktree.read(cx).id(), relative_path));
                                    let buffer =
                                        project.get_open_buffer(&project_path, cx)?.read(cx);

                                    let mut highlights = Vec::new();
                                    let highlight_count = rng.lock().gen_range(1..=5);
                                    let mut prev_end = 0;
                                    for _ in 0..highlight_count {
                                        let range =
                                            buffer.random_byte_range(prev_end, &mut *rng.lock());

                                        highlights.push(lsp::DocumentHighlight {
                                            range: range_to_lsp(range.to_point_utf16(buffer)),
                                            kind: Some(lsp::DocumentHighlightKind::READ),
                                        });
                                        prev_end = range.end;
                                    }
                                    Some(highlights)
                                })
                            } else {
                                None
                            };
                            async move { Ok(highlights) }
                        }
                    });
                }
            })),
            ..Default::default()
        });
        host_language_registry.add(Arc::new(language));

        let host_disconnected = Rc::new(AtomicBool::new(false));
        user_ids.push(host.current_user_id(&host_cx));
        clients.push(cx.foreground().spawn(host.simulate_host(
            host_project,
            files,
            operations.clone(),
            max_operations,
            rng.clone(),
            host_cx,
        )));

        while operations.get() < max_operations {
            cx.background().simulate_random_delay().await;
            if clients.len() >= max_peers {
                break;
            } else if rng.lock().gen_bool(0.05) {
                operations.set(operations.get() + 1);

                let guest_id = clients.len();
                log::info!("Adding guest {}", guest_id);
                next_entity_id += 100000;
                let mut guest_cx = TestAppContext::new(
                    cx.foreground_platform(),
                    cx.platform(),
                    cx.foreground(),
                    cx.background(),
                    cx.font_cache(),
                    cx.leak_detector(),
                    next_entity_id,
                );
                let guest = server
                    .create_client(&mut guest_cx, &format!("guest-{}", guest_id))
                    .await;
                let guest_project = Project::remote(
                    host_project_id,
                    guest.client.clone(),
                    guest.user_store.clone(),
                    guest_lang_registry.clone(),
                    FakeFs::new(cx.background()),
                    &mut guest_cx.to_async(),
                )
                .await
                .unwrap();
                user_ids.push(guest.current_user_id(&guest_cx));
                clients.push(cx.foreground().spawn(guest.simulate_guest(
                    guest_id,
                    guest_project,
                    operations.clone(),
                    max_operations,
                    rng.clone(),
                    host_disconnected.clone(),
                    guest_cx,
                )));

                log::info!("Guest {} added", guest_id);
            } else if rng.lock().gen_bool(0.05) {
                host_disconnected.store(true, SeqCst);
                server.disconnect_client(user_ids[0]);
                cx.foreground().advance_clock(RECEIVE_TIMEOUT);
                let mut clients = futures::future::join_all(clients).await;
                cx.foreground().run_until_parked();

                let (host, mut host_cx) = clients.remove(0);
                host.project
                    .as_ref()
                    .unwrap()
                    .read_with(&host_cx, |project, _| assert!(!project.is_shared()));
                for (guest, mut guest_cx) in clients {
                    let contacts = server
                        .store
                        .read()
                        .contacts_for_user(guest.current_user_id(&guest_cx));
                    assert!(!contacts
                        .iter()
                        .flat_map(|contact| &contact.projects)
                        .any(|project| project.id == host_project_id));
                    guest
                        .project
                        .as_ref()
                        .unwrap()
                        .read_with(&guest_cx, |project, _| assert!(project.is_read_only()));
                    guest_cx.update(|_| drop(guest));
                }
                host_cx.update(|_| drop(host));

                return;
            }
        }

        let mut clients = futures::future::join_all(clients).await;
        cx.foreground().run_until_parked();

        let (host_client, mut host_cx) = clients.remove(0);
        let host_project = host_client.project.as_ref().unwrap();
        let host_worktree_snapshots = host_project.read_with(&host_cx, |project, cx| {
            project
                .worktrees(cx)
                .map(|worktree| {
                    let snapshot = worktree.read(cx).snapshot();
                    (snapshot.id(), snapshot)
                })
                .collect::<BTreeMap<_, _>>()
        });

        host_client
            .project
            .as_ref()
            .unwrap()
            .read_with(&host_cx, |project, cx| project.check_invariants(cx));

        for (guest_client, mut guest_cx) in clients.into_iter() {
            let guest_id = guest_client.client.id();
            let worktree_snapshots =
                guest_client
                    .project
                    .as_ref()
                    .unwrap()
                    .read_with(&guest_cx, |project, cx| {
                        project
                            .worktrees(cx)
                            .map(|worktree| {
                                let worktree = worktree.read(cx);
                                (worktree.id(), worktree.snapshot())
                            })
                            .collect::<BTreeMap<_, _>>()
                    });

            assert_eq!(
                worktree_snapshots.keys().collect::<Vec<_>>(),
                host_worktree_snapshots.keys().collect::<Vec<_>>(),
                "guest {} has different worktrees than the host",
                guest_id
            );
            for (id, host_snapshot) in &host_worktree_snapshots {
                let guest_snapshot = &worktree_snapshots[id];
                assert_eq!(
                    guest_snapshot.root_name(),
                    host_snapshot.root_name(),
                    "guest {} has different root name than the host for worktree {}",
                    guest_id,
                    id
                );
                assert_eq!(
                    guest_snapshot.entries(false).collect::<Vec<_>>(),
                    host_snapshot.entries(false).collect::<Vec<_>>(),
                    "guest {} has different snapshot than the host for worktree {}",
                    guest_id,
                    id
                );
            }

            guest_client
                .project
                .as_ref()
                .unwrap()
                .read_with(&guest_cx, |project, cx| project.check_invariants(cx));

            for guest_buffer in &guest_client.buffers {
                let buffer_id = guest_buffer.read_with(&guest_cx, |buffer, _| buffer.remote_id());
                let host_buffer = host_project.read_with(&host_cx, |project, cx| {
                    project.buffer_for_id(buffer_id, cx).expect(&format!(
                        "host does not have buffer for guest:{}, peer:{}, id:{}",
                        guest_id, guest_client.peer_id, buffer_id
                    ))
                });
                let path = host_buffer
                    .read_with(&host_cx, |buffer, cx| buffer.file().unwrap().full_path(cx));

                assert_eq!(
                    guest_buffer.read_with(&guest_cx, |buffer, _| buffer.deferred_ops_len()),
                    0,
                    "guest {}, buffer {}, path {:?} has deferred operations",
                    guest_id,
                    buffer_id,
                    path,
                );
                assert_eq!(
                    guest_buffer.read_with(&guest_cx, |buffer, _| buffer.text()),
                    host_buffer.read_with(&host_cx, |buffer, _| buffer.text()),
                    "guest {}, buffer {}, path {:?}, differs from the host's buffer",
                    guest_id,
                    buffer_id,
                    path
                );
            }

            guest_cx.update(|_| drop(guest_client));
        }

        host_cx.update(|_| drop(host_client));
    }

    struct TestServer {
        peer: Arc<Peer>,
        app_state: Arc<AppState>,
        server: Arc<Server>,
        foreground: Rc<executor::Foreground>,
        notifications: mpsc::UnboundedReceiver<()>,
        connection_killers: Arc<Mutex<HashMap<UserId, Arc<AtomicBool>>>>,
        forbid_connections: Arc<AtomicBool>,
        _test_db: TestDb,
    }

    impl TestServer {
        async fn start(
            foreground: Rc<executor::Foreground>,
            background: Arc<executor::Background>,
        ) -> Self {
            let test_db = TestDb::fake(background);
            let app_state = Self::build_app_state(&test_db).await;
            let peer = Peer::new();
            let notifications = mpsc::unbounded();
            let server = Server::new(app_state.clone(), peer.clone(), Some(notifications.0));
            Self {
                peer,
                app_state,
                server,
                foreground,
                notifications: notifications.1,
                connection_killers: Default::default(),
                forbid_connections: Default::default(),
                _test_db: test_db,
            }
        }

        async fn create_client(&mut self, cx: &mut TestAppContext, name: &str) -> TestClient {
            cx.update(|cx| {
                let settings = Settings::test(cx);
                cx.set_global(settings);
            });

            let http = FakeHttpClient::with_404_response();
            let user_id = self.app_state.db.create_user(name, false).await.unwrap();
            let client_name = name.to_string();
            let mut client = Client::new(http.clone());
            let server = self.server.clone();
            let connection_killers = self.connection_killers.clone();
            let forbid_connections = self.forbid_connections.clone();
            let (connection_id_tx, mut connection_id_rx) = mpsc::channel(16);

            Arc::get_mut(&mut client)
                .unwrap()
                .override_authenticate(move |cx| {
                    cx.spawn(|_| async move {
                        let access_token = "the-token".to_string();
                        Ok(Credentials {
                            user_id: user_id.0 as u64,
                            access_token,
                        })
                    })
                })
                .override_establish_connection(move |credentials, cx| {
                    assert_eq!(credentials.user_id, user_id.0 as u64);
                    assert_eq!(credentials.access_token, "the-token");

                    let server = server.clone();
                    let connection_killers = connection_killers.clone();
                    let forbid_connections = forbid_connections.clone();
                    let client_name = client_name.clone();
                    let connection_id_tx = connection_id_tx.clone();
                    cx.spawn(move |cx| async move {
                        if forbid_connections.load(SeqCst) {
                            Err(EstablishConnectionError::other(anyhow!(
                                "server is forbidding connections"
                            )))
                        } else {
                            let (client_conn, server_conn, killed) =
                                Connection::in_memory(cx.background());
                            connection_killers.lock().insert(user_id, killed);
                            cx.background()
                                .spawn(server.handle_connection(
                                    server_conn,
                                    client_name,
                                    user_id,
                                    Some(connection_id_tx),
                                    cx.background(),
                                ))
                                .detach();
                            Ok(client_conn)
                        }
                    })
                });

            client
                .authenticate_and_connect(false, &cx.to_async())
                .await
                .unwrap();

            Channel::init(&client);
            Project::init(&client);
            cx.update(|cx| {
                workspace::init(&client, cx);
            });

            let peer_id = PeerId(connection_id_rx.next().await.unwrap().0);
            let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http, cx));

            let client = TestClient {
                client,
                peer_id,
                user_store,
                language_registry: Arc::new(LanguageRegistry::test()),
                project: Default::default(),
                buffers: Default::default(),
            };
            client.wait_for_current_user(cx).await;
            client
        }

        fn disconnect_client(&self, user_id: UserId) {
            self.connection_killers
                .lock()
                .remove(&user_id)
                .unwrap()
                .store(true, SeqCst);
        }

        fn forbid_connections(&self) {
            self.forbid_connections.store(true, SeqCst);
        }

        fn allow_connections(&self) {
            self.forbid_connections.store(false, SeqCst);
        }

        async fn build_app_state(test_db: &TestDb) -> Arc<AppState> {
            let mut config = Config::default();
            config.session_secret = "a".repeat(32);
            config.database_url = test_db.url.clone();
            let github_client = github::AppClient::test();
            Arc::new(AppState {
                db: test_db.db().clone(),
                handlebars: Default::default(),
                auth_client: auth::build_client("", ""),
                repo_client: github::RepoClient::test(&github_client),
                github_client,
                config,
            })
        }

        async fn state<'a>(&'a self) -> RwLockReadGuard<'a, Store> {
            self.server.store.read()
        }

        async fn condition<F>(&mut self, mut predicate: F)
        where
            F: FnMut(&Store) -> bool,
        {
            async_std::future::timeout(Duration::from_millis(500), async {
                while !(predicate)(&*self.server.store.read()) {
                    self.foreground.start_waiting();
                    self.notifications.next().await;
                    self.foreground.finish_waiting();
                }
            })
            .await
            .expect("condition timed out");
        }
    }

    impl Deref for TestServer {
        type Target = Server;

        fn deref(&self) -> &Self::Target {
            &self.server
        }
    }

    impl Drop for TestServer {
        fn drop(&mut self) {
            self.peer.reset();
        }
    }

    struct TestClient {
        client: Arc<Client>,
        pub peer_id: PeerId,
        pub user_store: ModelHandle<UserStore>,
        language_registry: Arc<LanguageRegistry>,
        project: Option<ModelHandle<Project>>,
        buffers: HashSet<ModelHandle<language::Buffer>>,
    }

    impl Deref for TestClient {
        type Target = Arc<Client>;

        fn deref(&self) -> &Self::Target {
            &self.client
        }
    }

    impl TestClient {
        pub fn current_user_id(&self, cx: &TestAppContext) -> UserId {
            UserId::from_proto(
                self.user_store
                    .read_with(cx, |user_store, _| user_store.current_user().unwrap().id),
            )
        }

        async fn wait_for_current_user(&self, cx: &TestAppContext) {
            let mut authed_user = self
                .user_store
                .read_with(cx, |user_store, _| user_store.watch_current_user());
            while authed_user.next().await.unwrap().is_none() {}
        }

        async fn build_local_project(
            &mut self,
            fs: Arc<FakeFs>,
            root_path: impl AsRef<Path>,
            cx: &mut TestAppContext,
        ) -> (ModelHandle<Project>, WorktreeId) {
            let project = cx.update(|cx| {
                Project::local(
                    self.client.clone(),
                    self.user_store.clone(),
                    self.language_registry.clone(),
                    fs,
                    cx,
                )
            });
            self.project = Some(project.clone());
            let (worktree, _) = project
                .update(cx, |p, cx| {
                    p.find_or_create_local_worktree(root_path, true, cx)
                })
                .await
                .unwrap();
            worktree
                .read_with(cx, |tree, _| tree.as_local().unwrap().scan_complete())
                .await;
            project
                .update(cx, |project, _| project.next_remote_id())
                .await;
            (project, worktree.read_with(cx, |tree, _| tree.id()))
        }

        async fn build_remote_project(
            &mut self,
            project_id: u64,
            cx: &mut TestAppContext,
        ) -> ModelHandle<Project> {
            let project = Project::remote(
                project_id,
                self.client.clone(),
                self.user_store.clone(),
                self.language_registry.clone(),
                FakeFs::new(cx.background()),
                &mut cx.to_async(),
            )
            .await
            .unwrap();
            self.project = Some(project.clone());
            project
        }

        fn build_workspace(
            &self,
            project: &ModelHandle<Project>,
            cx: &mut TestAppContext,
        ) -> ViewHandle<Workspace> {
            let (window_id, _) = cx.add_window(|_| EmptyView);
            cx.add_view(window_id, |cx| {
                let fs = project.read(cx).fs().clone();
                Workspace::new(
                    &WorkspaceParams {
                        fs,
                        project: project.clone(),
                        user_store: self.user_store.clone(),
                        languages: self.language_registry.clone(),
                        channel_list: cx.add_model(|cx| {
                            ChannelList::new(self.user_store.clone(), self.client.clone(), cx)
                        }),
                        client: self.client.clone(),
                    },
                    cx,
                )
            })
        }

        async fn simulate_host(
            mut self,
            project: ModelHandle<Project>,
            files: Arc<Mutex<Vec<PathBuf>>>,
            operations: Rc<Cell<usize>>,
            max_operations: usize,
            rng: Arc<Mutex<StdRng>>,
            mut cx: TestAppContext,
        ) -> (Self, TestAppContext) {
            async fn simulate_host_internal(
                client: &mut TestClient,
                project: ModelHandle<Project>,
                files: Arc<Mutex<Vec<PathBuf>>>,
                operations: Rc<Cell<usize>>,
                max_operations: usize,
                rng: Arc<Mutex<StdRng>>,
                cx: &mut TestAppContext,
            ) -> anyhow::Result<()> {
                let fs = project.read_with(cx, |project, _| project.fs().clone());
                while operations.get() < max_operations {
                    operations.set(operations.get() + 1);

                    let distribution = rng.lock().gen_range::<usize, _>(0..100);
                    match distribution {
                        0..=20 if !files.lock().is_empty() => {
                            let path = files.lock().choose(&mut *rng.lock()).unwrap().clone();
                            let mut path = path.as_path();
                            while let Some(parent_path) = path.parent() {
                                path = parent_path;
                                if rng.lock().gen() {
                                    break;
                                }
                            }

                            log::info!("Host: find/create local worktree {:?}", path);
                            let find_or_create_worktree = project.update(cx, |project, cx| {
                                project.find_or_create_local_worktree(path, true, cx)
                            });
                            if rng.lock().gen() {
                                cx.background().spawn(find_or_create_worktree).detach();
                            } else {
                                find_or_create_worktree.await?;
                            }
                        }
                        10..=80 if !files.lock().is_empty() => {
                            let buffer = if client.buffers.is_empty() || rng.lock().gen() {
                                let file = files.lock().choose(&mut *rng.lock()).unwrap().clone();
                                let (worktree, path) = project
                                    .update(cx, |project, cx| {
                                        project.find_or_create_local_worktree(
                                            file.clone(),
                                            true,
                                            cx,
                                        )
                                    })
                                    .await?;
                                let project_path =
                                    worktree.read_with(cx, |worktree, _| (worktree.id(), path));
                                log::info!(
                                    "Host: opening path {:?}, worktree {}, relative_path {:?}",
                                    file,
                                    project_path.0,
                                    project_path.1
                                );
                                let buffer = project
                                    .update(cx, |project, cx| project.open_buffer(project_path, cx))
                                    .await
                                    .unwrap();
                                client.buffers.insert(buffer.clone());
                                buffer
                            } else {
                                client
                                    .buffers
                                    .iter()
                                    .choose(&mut *rng.lock())
                                    .unwrap()
                                    .clone()
                            };

                            if rng.lock().gen_bool(0.1) {
                                cx.update(|cx| {
                                    log::info!(
                                        "Host: dropping buffer {:?}",
                                        buffer.read(cx).file().unwrap().full_path(cx)
                                    );
                                    client.buffers.remove(&buffer);
                                    drop(buffer);
                                });
                            } else {
                                buffer.update(cx, |buffer, cx| {
                                    log::info!(
                                        "Host: updating buffer {:?} ({})",
                                        buffer.file().unwrap().full_path(cx),
                                        buffer.remote_id()
                                    );
                                    buffer.randomly_edit(&mut *rng.lock(), 5, cx)
                                });
                            }
                        }
                        _ => loop {
                            let path_component_count = rng.lock().gen_range::<usize, _>(1..=5);
                            let mut path = PathBuf::new();
                            path.push("/");
                            for _ in 0..path_component_count {
                                let letter = rng.lock().gen_range(b'a'..=b'z');
                                path.push(std::str::from_utf8(&[letter]).unwrap());
                            }
                            path.set_extension("rs");
                            let parent_path = path.parent().unwrap();

                            log::info!("Host: creating file {:?}", path,);

                            if fs.create_dir(&parent_path).await.is_ok()
                                && fs.create_file(&path, Default::default()).await.is_ok()
                            {
                                files.lock().push(path);
                                break;
                            } else {
                                log::info!("Host: cannot create file");
                            }
                        },
                    }

                    cx.background().simulate_random_delay().await;
                }

                Ok(())
            }

            simulate_host_internal(
                &mut self,
                project.clone(),
                files,
                operations,
                max_operations,
                rng,
                &mut cx,
            )
            .log_err()
            .await;
            log::info!("Host done");
            self.project = Some(project);
            (self, cx)
        }

        pub async fn simulate_guest(
            mut self,
            guest_id: usize,
            project: ModelHandle<Project>,
            operations: Rc<Cell<usize>>,
            max_operations: usize,
            rng: Arc<Mutex<StdRng>>,
            host_disconnected: Rc<AtomicBool>,
            mut cx: TestAppContext,
        ) -> (Self, TestAppContext) {
            async fn simulate_guest_internal(
                client: &mut TestClient,
                guest_id: usize,
                project: ModelHandle<Project>,
                operations: Rc<Cell<usize>>,
                max_operations: usize,
                rng: Arc<Mutex<StdRng>>,
                cx: &mut TestAppContext,
            ) -> anyhow::Result<()> {
                while operations.get() < max_operations {
                    let buffer = if client.buffers.is_empty() || rng.lock().gen() {
                        let worktree = if let Some(worktree) =
                            project.read_with(cx, |project, cx| {
                                project
                                    .worktrees(&cx)
                                    .filter(|worktree| {
                                        let worktree = worktree.read(cx);
                                        worktree.is_visible()
                                            && worktree.entries(false).any(|e| e.is_file())
                                    })
                                    .choose(&mut *rng.lock())
                            }) {
                            worktree
                        } else {
                            cx.background().simulate_random_delay().await;
                            continue;
                        };

                        operations.set(operations.get() + 1);
                        let (worktree_root_name, project_path) =
                            worktree.read_with(cx, |worktree, _| {
                                let entry = worktree
                                    .entries(false)
                                    .filter(|e| e.is_file())
                                    .choose(&mut *rng.lock())
                                    .unwrap();
                                (
                                    worktree.root_name().to_string(),
                                    (worktree.id(), entry.path.clone()),
                                )
                            });
                        log::info!(
                            "Guest {}: opening path {:?} in worktree {} ({})",
                            guest_id,
                            project_path.1,
                            project_path.0,
                            worktree_root_name,
                        );
                        let buffer = project
                            .update(cx, |project, cx| {
                                project.open_buffer(project_path.clone(), cx)
                            })
                            .await?;
                        log::info!(
                            "Guest {}: opened path {:?} in worktree {} ({}) with buffer id {}",
                            guest_id,
                            project_path.1,
                            project_path.0,
                            worktree_root_name,
                            buffer.read_with(cx, |buffer, _| buffer.remote_id())
                        );
                        client.buffers.insert(buffer.clone());
                        buffer
                    } else {
                        operations.set(operations.get() + 1);

                        client
                            .buffers
                            .iter()
                            .choose(&mut *rng.lock())
                            .unwrap()
                            .clone()
                    };

                    let choice = rng.lock().gen_range(0..100);
                    match choice {
                        0..=9 => {
                            cx.update(|cx| {
                                log::info!(
                                    "Guest {}: dropping buffer {:?}",
                                    guest_id,
                                    buffer.read(cx).file().unwrap().full_path(cx)
                                );
                                client.buffers.remove(&buffer);
                                drop(buffer);
                            });
                        }
                        10..=19 => {
                            let completions = project.update(cx, |project, cx| {
                                log::info!(
                                    "Guest {}: requesting completions for buffer {} ({:?})",
                                    guest_id,
                                    buffer.read(cx).remote_id(),
                                    buffer.read(cx).file().unwrap().full_path(cx)
                                );
                                let offset = rng.lock().gen_range(0..=buffer.read(cx).len());
                                project.completions(&buffer, offset, cx)
                            });
                            let completions = cx.background().spawn(async move {
                                completions
                                    .await
                                    .map_err(|err| anyhow!("completions request failed: {:?}", err))
                            });
                            if rng.lock().gen_bool(0.3) {
                                log::info!("Guest {}: detaching completions request", guest_id);
                                cx.update(|cx| completions.detach_and_log_err(cx));
                            } else {
                                completions.await?;
                            }
                        }
                        20..=29 => {
                            let code_actions = project.update(cx, |project, cx| {
                                log::info!(
                                    "Guest {}: requesting code actions for buffer {} ({:?})",
                                    guest_id,
                                    buffer.read(cx).remote_id(),
                                    buffer.read(cx).file().unwrap().full_path(cx)
                                );
                                let range = buffer.read(cx).random_byte_range(0, &mut *rng.lock());
                                project.code_actions(&buffer, range, cx)
                            });
                            let code_actions = cx.background().spawn(async move {
                                code_actions.await.map_err(|err| {
                                    anyhow!("code actions request failed: {:?}", err)
                                })
                            });
                            if rng.lock().gen_bool(0.3) {
                                log::info!("Guest {}: detaching code actions request", guest_id);
                                cx.update(|cx| code_actions.detach_and_log_err(cx));
                            } else {
                                code_actions.await?;
                            }
                        }
                        30..=39 if buffer.read_with(cx, |buffer, _| buffer.is_dirty()) => {
                            let (requested_version, save) = buffer.update(cx, |buffer, cx| {
                                log::info!(
                                    "Guest {}: saving buffer {} ({:?})",
                                    guest_id,
                                    buffer.remote_id(),
                                    buffer.file().unwrap().full_path(cx)
                                );
                                (buffer.version(), buffer.save(cx))
                            });
                            let save = cx.background().spawn(async move {
                                let (saved_version, _) = save
                                    .await
                                    .map_err(|err| anyhow!("save request failed: {:?}", err))?;
                                assert!(saved_version.observed_all(&requested_version));
                                Ok::<_, anyhow::Error>(())
                            });
                            if rng.lock().gen_bool(0.3) {
                                log::info!("Guest {}: detaching save request", guest_id);
                                cx.update(|cx| save.detach_and_log_err(cx));
                            } else {
                                save.await?;
                            }
                        }
                        40..=44 => {
                            let prepare_rename = project.update(cx, |project, cx| {
                                log::info!(
                                    "Guest {}: preparing rename for buffer {} ({:?})",
                                    guest_id,
                                    buffer.read(cx).remote_id(),
                                    buffer.read(cx).file().unwrap().full_path(cx)
                                );
                                let offset = rng.lock().gen_range(0..=buffer.read(cx).len());
                                project.prepare_rename(buffer, offset, cx)
                            });
                            let prepare_rename = cx.background().spawn(async move {
                                prepare_rename.await.map_err(|err| {
                                    anyhow!("prepare rename request failed: {:?}", err)
                                })
                            });
                            if rng.lock().gen_bool(0.3) {
                                log::info!("Guest {}: detaching prepare rename request", guest_id);
                                cx.update(|cx| prepare_rename.detach_and_log_err(cx));
                            } else {
                                prepare_rename.await?;
                            }
                        }
                        45..=49 => {
                            let definitions = project.update(cx, |project, cx| {
                                log::info!(
                                    "Guest {}: requesting definitions for buffer {} ({:?})",
                                    guest_id,
                                    buffer.read(cx).remote_id(),
                                    buffer.read(cx).file().unwrap().full_path(cx)
                                );
                                let offset = rng.lock().gen_range(0..=buffer.read(cx).len());
                                project.definition(&buffer, offset, cx)
                            });
                            let definitions = cx.background().spawn(async move {
                                definitions
                                    .await
                                    .map_err(|err| anyhow!("definitions request failed: {:?}", err))
                            });
                            if rng.lock().gen_bool(0.3) {
                                log::info!("Guest {}: detaching definitions request", guest_id);
                                cx.update(|cx| definitions.detach_and_log_err(cx));
                            } else {
                                client
                                    .buffers
                                    .extend(definitions.await?.into_iter().map(|loc| loc.buffer));
                            }
                        }
                        50..=54 => {
                            let highlights = project.update(cx, |project, cx| {
                                log::info!(
                                    "Guest {}: requesting highlights for buffer {} ({:?})",
                                    guest_id,
                                    buffer.read(cx).remote_id(),
                                    buffer.read(cx).file().unwrap().full_path(cx)
                                );
                                let offset = rng.lock().gen_range(0..=buffer.read(cx).len());
                                project.document_highlights(&buffer, offset, cx)
                            });
                            let highlights = cx.background().spawn(async move {
                                highlights
                                    .await
                                    .map_err(|err| anyhow!("highlights request failed: {:?}", err))
                            });
                            if rng.lock().gen_bool(0.3) {
                                log::info!("Guest {}: detaching highlights request", guest_id);
                                cx.update(|cx| highlights.detach_and_log_err(cx));
                            } else {
                                highlights.await?;
                            }
                        }
                        55..=59 => {
                            let search = project.update(cx, |project, cx| {
                                let query = rng.lock().gen_range('a'..='z');
                                log::info!("Guest {}: project-wide search {:?}", guest_id, query);
                                project.search(SearchQuery::text(query, false, false), cx)
                            });
                            let search = cx.background().spawn(async move {
                                search
                                    .await
                                    .map_err(|err| anyhow!("search request failed: {:?}", err))
                            });
                            if rng.lock().gen_bool(0.3) {
                                log::info!("Guest {}: detaching search request", guest_id);
                                cx.update(|cx| search.detach_and_log_err(cx));
                            } else {
                                client.buffers.extend(search.await?.into_keys());
                            }
                        }
                        _ => {
                            buffer.update(cx, |buffer, cx| {
                                log::info!(
                                    "Guest {}: updating buffer {} ({:?})",
                                    guest_id,
                                    buffer.remote_id(),
                                    buffer.file().unwrap().full_path(cx)
                                );
                                buffer.randomly_edit(&mut *rng.lock(), 5, cx)
                            });
                        }
                    }
                    cx.background().simulate_random_delay().await;
                }
                Ok(())
            }

            match simulate_guest_internal(
                &mut self,
                guest_id,
                project.clone(),
                operations,
                max_operations,
                rng,
                &mut cx,
            )
            .await
            {
                Ok(()) => log::info!("guest {} done", guest_id),
                Err(err) => {
                    if host_disconnected.load(SeqCst) {
                        log::error!("guest {} simulation error - {:?}", guest_id, err);
                    } else {
                        panic!("guest {} simulation error - {:?}", guest_id, err);
                    }
                }
            }

            self.project = Some(project);
            (self, cx)
        }
    }

    impl Drop for TestClient {
        fn drop(&mut self) {
            self.client.tear_down();
        }
    }

    impl Executor for Arc<gpui::executor::Background> {
        type Timer = gpui::executor::Timer;

        fn spawn_detached<F: 'static + Send + Future<Output = ()>>(&self, future: F) {
            self.spawn(future).detach();
        }

        fn timer(&self, duration: Duration) -> Self::Timer {
            self.as_ref().timer(duration)
        }
    }

    fn channel_messages(channel: &Channel) -> Vec<(String, String, bool)> {
        channel
            .messages()
            .cursor::<()>()
            .map(|m| {
                (
                    m.sender.github_login.clone(),
                    m.body.clone(),
                    m.is_pending(),
                )
            })
            .collect()
    }

    struct EmptyView;

    impl gpui::Entity for EmptyView {
        type Event = ();
    }

    impl gpui::View for EmptyView {
        fn ui_name() -> &'static str {
            "empty view"
        }

        fn render(&mut self, _: &mut gpui::RenderContext<Self>) -> gpui::ElementBox {
            gpui::Element::boxed(gpui::elements::Empty)
        }
    }
}
