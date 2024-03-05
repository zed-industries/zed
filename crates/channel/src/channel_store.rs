mod channel_index;

use crate::{channel_buffer::ChannelBuffer, channel_chat::ChannelChat, ChannelMessage};
use anyhow::{anyhow, Result};
use channel_index::ChannelIndex;
use client::{
    ChannelId, Client, ClientSettings, HostedProjectId, Subscription, User, UserId, UserStore,
};
use collections::{hash_map, HashMap, HashSet};
use futures::{channel::mpsc, future::Shared, Future, FutureExt, StreamExt};
use gpui::{
    AppContext, AsyncAppContext, Context, EventEmitter, Global, Model, ModelContext, SharedString,
    Task, WeakModel,
};
use language::Capability;
use rpc::{
    proto::{self, ChannelRole, ChannelVisibility},
    TypedEnvelope,
};
use settings::Settings;
use std::{mem, sync::Arc, time::Duration};
use util::{async_maybe, maybe, ResultExt};

pub const RECONNECT_TIMEOUT: Duration = Duration::from_secs(30);

pub fn init(client: &Arc<Client>, user_store: Model<UserStore>, cx: &mut AppContext) {
    let channel_store =
        cx.new_model(|cx| ChannelStore::new(client.clone(), user_store.clone(), cx));
    cx.set_global(GlobalChannelStore(channel_store));
}

#[derive(Debug, Clone, Default)]
struct NotesVersion {
    epoch: u64,
    version: clock::Global,
}

#[derive(Debug, Clone)]
pub struct HostedProject {
    id: HostedProjectId,
    channel_id: ChannelId,
    name: SharedString,
    _visibility: proto::ChannelVisibility,
}

impl From<proto::HostedProject> for HostedProject {
    fn from(project: proto::HostedProject) -> Self {
        Self {
            id: HostedProjectId(project.id),
            channel_id: ChannelId(project.channel_id),
            _visibility: project.visibility(),
            name: project.name.into(),
        }
    }
}

pub struct ChannelStore {
    pub channel_index: ChannelIndex,
    channel_invitations: Vec<Arc<Channel>>,
    channel_participants: HashMap<ChannelId, Vec<Arc<User>>>,
    channel_states: HashMap<ChannelId, ChannelState>,
    hosted_projects: HashMap<HostedProjectId, HostedProject>,

    outgoing_invites: HashSet<(ChannelId, UserId)>,
    update_channels_tx: mpsc::UnboundedSender<proto::UpdateChannels>,
    opened_buffers: HashMap<ChannelId, OpenedModelHandle<ChannelBuffer>>,
    opened_chats: HashMap<ChannelId, OpenedModelHandle<ChannelChat>>,
    client: Arc<Client>,
    user_store: Model<UserStore>,
    _rpc_subscriptions: [Subscription; 2],
    _watch_connection_status: Task<Option<()>>,
    disconnect_channel_buffers_task: Option<Task<()>>,
    _update_channels: Task<()>,
}

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: ChannelId,
    pub name: SharedString,
    pub visibility: proto::ChannelVisibility,
    pub parent_path: Vec<ChannelId>,
}

#[derive(Default)]
pub struct ChannelState {
    latest_chat_message: Option<u64>,
    latest_notes_versions: Option<NotesVersion>,
    observed_chat_message: Option<u64>,
    observed_notes_versions: Option<NotesVersion>,
    role: Option<ChannelRole>,
    projects: HashSet<HostedProjectId>,
}

impl Channel {
    pub fn link(&self, cx: &AppContext) -> String {
        format!(
            "{}/channel/{}-{}",
            ClientSettings::get_global(cx).server_url,
            Self::slug(&self.name),
            self.id
        )
    }

    pub fn notes_link(&self, heading: Option<String>, cx: &AppContext) -> String {
        self.link(cx)
            + "/notes"
            + &heading
                .map(|h| format!("#{}", Self::slug(&h)))
                .unwrap_or_default()
    }

    pub fn is_root_channel(&self) -> bool {
        self.parent_path.is_empty()
    }

    pub fn root_id(&self) -> ChannelId {
        self.parent_path.first().copied().unwrap_or(self.id)
    }

    pub fn slug(str: &str) -> String {
        let slug: String = str
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect();

        slug.trim_matches(|c| c == '-').to_string()
    }
}

pub struct ChannelMembership {
    pub user: Arc<User>,
    pub kind: proto::channel_member::Kind,
    pub role: proto::ChannelRole,
}
impl ChannelMembership {
    pub fn sort_key(&self) -> MembershipSortKey {
        MembershipSortKey {
            role_order: match self.role {
                proto::ChannelRole::Admin => 0,
                proto::ChannelRole::Member => 1,
                proto::ChannelRole::Banned => 2,
                proto::ChannelRole::Talker => 3,
                proto::ChannelRole::Guest => 4,
            },
            kind_order: match self.kind {
                proto::channel_member::Kind::Member => 0,
                proto::channel_member::Kind::Invitee => 1,
            },
            username_order: self.user.github_login.as_str(),
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub struct MembershipSortKey<'a> {
    role_order: u8,
    kind_order: u8,
    username_order: &'a str,
}

pub enum ChannelEvent {
    ChannelCreated(ChannelId),
    ChannelRenamed(ChannelId),
}

impl EventEmitter<ChannelEvent> for ChannelStore {}

enum OpenedModelHandle<E> {
    Open(WeakModel<E>),
    Loading(Shared<Task<Result<Model<E>, Arc<anyhow::Error>>>>),
}

struct GlobalChannelStore(Model<ChannelStore>);

impl Global for GlobalChannelStore {}

impl ChannelStore {
    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalChannelStore>().0.clone()
    }

    pub fn new(
        client: Arc<Client>,
        user_store: Model<UserStore>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let rpc_subscriptions = [
            client.add_message_handler(cx.weak_model(), Self::handle_update_channels),
            client.add_message_handler(cx.weak_model(), Self::handle_update_user_channels),
        ];

        let mut connection_status = client.status();
        let (update_channels_tx, mut update_channels_rx) = mpsc::unbounded();
        let watch_connection_status = cx.spawn(|this, mut cx| async move {
            while let Some(status) = connection_status.next().await {
                let this = this.upgrade()?;
                match status {
                    client::Status::Connected { .. } => {
                        this.update(&mut cx, |this, cx| this.handle_connect(cx))
                            .ok()?
                            .await
                            .log_err()?;
                    }
                    client::Status::SignedOut | client::Status::UpgradeRequired => {
                        this.update(&mut cx, |this, cx| this.handle_disconnect(false, cx))
                            .ok();
                    }
                    _ => {
                        this.update(&mut cx, |this, cx| this.handle_disconnect(true, cx))
                            .ok();
                    }
                }
            }
            Some(())
        });

        Self {
            channel_invitations: Vec::default(),
            channel_index: ChannelIndex::default(),
            channel_participants: Default::default(),
            hosted_projects: Default::default(),
            outgoing_invites: Default::default(),
            opened_buffers: Default::default(),
            opened_chats: Default::default(),
            update_channels_tx,
            client,
            user_store,
            _rpc_subscriptions: rpc_subscriptions,
            _watch_connection_status: watch_connection_status,
            disconnect_channel_buffers_task: None,
            _update_channels: cx.spawn(|this, mut cx| async move {
                async_maybe!({
                    while let Some(update_channels) = update_channels_rx.next().await {
                        if let Some(this) = this.upgrade() {
                            let update_task = this.update(&mut cx, |this, cx| {
                                this.update_channels(update_channels, cx)
                            })?;
                            if let Some(update_task) = update_task {
                                update_task.await.log_err();
                            }
                        }
                    }
                    anyhow::Ok(())
                })
                .await
                .log_err();
            }),
            channel_states: Default::default(),
        }
    }

    pub fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    /// Returns the number of unique channels in the store
    pub fn channel_count(&self) -> usize {
        self.channel_index.by_id().len()
    }

    /// Returns the index of a channel ID in the list of unique channels
    pub fn index_of_channel(&self, channel_id: ChannelId) -> Option<usize> {
        self.channel_index
            .by_id()
            .keys()
            .position(|id| *id == channel_id)
    }

    /// Returns an iterator over all unique channels
    pub fn channels(&self) -> impl '_ + Iterator<Item = &Arc<Channel>> {
        self.channel_index.by_id().values()
    }

    /// Iterate over all entries in the channel DAG
    pub fn ordered_channels(&self) -> impl '_ + Iterator<Item = (usize, &Arc<Channel>)> {
        self.channel_index
            .ordered_channels()
            .iter()
            .filter_map(move |id| {
                let channel = self.channel_index.by_id().get(id)?;
                Some((channel.parent_path.len(), channel))
            })
    }

    pub fn channel_at_index(&self, ix: usize) -> Option<&Arc<Channel>> {
        let channel_id = self.channel_index.ordered_channels().get(ix)?;
        self.channel_index.by_id().get(channel_id)
    }

    pub fn channel_at(&self, ix: usize) -> Option<&Arc<Channel>> {
        self.channel_index.by_id().values().nth(ix)
    }

    pub fn has_channel_invitation(&self, channel_id: ChannelId) -> bool {
        self.channel_invitations
            .iter()
            .any(|channel| channel.id == channel_id)
    }

    pub fn channel_invitations(&self) -> &[Arc<Channel>] {
        &self.channel_invitations
    }

    pub fn channel_for_id(&self, channel_id: ChannelId) -> Option<&Arc<Channel>> {
        self.channel_index.by_id().get(&channel_id)
    }

    pub fn projects_for_id(&self, channel_id: ChannelId) -> Vec<(SharedString, HostedProjectId)> {
        let mut projects: Vec<(SharedString, HostedProjectId)> = self
            .channel_states
            .get(&channel_id)
            .map(|state| state.projects.clone())
            .unwrap_or_default()
            .into_iter()
            .flat_map(|id| Some((self.hosted_projects.get(&id)?.name.clone(), id)))
            .collect();
        projects.sort();
        projects
    }

    pub fn has_open_channel_buffer(&self, channel_id: ChannelId, _cx: &AppContext) -> bool {
        if let Some(buffer) = self.opened_buffers.get(&channel_id) {
            if let OpenedModelHandle::Open(buffer) = buffer {
                return buffer.upgrade().is_some();
            }
        }
        false
    }

    pub fn open_channel_buffer(
        &mut self,
        channel_id: ChannelId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<ChannelBuffer>>> {
        let client = self.client.clone();
        let user_store = self.user_store.clone();
        let channel_store = cx.handle();
        self.open_channel_resource(
            channel_id,
            |this| &mut this.opened_buffers,
            |channel, cx| ChannelBuffer::new(channel, client, user_store, channel_store, cx),
            cx,
        )
    }

    pub fn fetch_channel_messages(
        &self,
        message_ids: Vec<u64>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<ChannelMessage>>> {
        let request = if message_ids.is_empty() {
            None
        } else {
            Some(
                self.client
                    .request(proto::GetChannelMessagesById { message_ids }),
            )
        };
        cx.spawn(|this, mut cx| async move {
            if let Some(request) = request {
                let response = request.await?;
                let this = this
                    .upgrade()
                    .ok_or_else(|| anyhow!("channel store dropped"))?;
                let user_store = this.update(&mut cx, |this, _| this.user_store.clone())?;
                ChannelMessage::from_proto_vec(response.messages, &user_store, &mut cx).await
            } else {
                Ok(Vec::new())
            }
        })
    }

    pub fn has_channel_buffer_changed(&self, channel_id: ChannelId) -> bool {
        self.channel_states
            .get(&channel_id)
            .is_some_and(|state| state.has_channel_buffer_changed())
    }

    pub fn has_new_messages(&self, channel_id: ChannelId) -> bool {
        self.channel_states
            .get(&channel_id)
            .is_some_and(|state| state.has_new_messages())
    }

    pub fn last_acknowledge_message_id(&self, channel_id: ChannelId) -> Option<u64> {
        self.channel_states.get(&channel_id).and_then(|state| {
            if let Some(last_message_id) = state.latest_chat_message {
                if state
                    .last_acknowledged_message_id()
                    .is_some_and(|id| id < last_message_id)
                {
                    return state.last_acknowledged_message_id();
                }
            }

            None
        })
    }

    pub fn acknowledge_message_id(
        &mut self,
        channel_id: ChannelId,
        message_id: u64,
        cx: &mut ModelContext<Self>,
    ) {
        self.channel_states
            .entry(channel_id)
            .or_insert_with(|| Default::default())
            .acknowledge_message_id(message_id);
        cx.notify();
    }

    pub fn update_latest_message_id(
        &mut self,
        channel_id: ChannelId,
        message_id: u64,
        cx: &mut ModelContext<Self>,
    ) {
        self.channel_states
            .entry(channel_id)
            .or_insert_with(|| Default::default())
            .update_latest_message_id(message_id);
        cx.notify();
    }

    pub fn acknowledge_notes_version(
        &mut self,
        channel_id: ChannelId,
        epoch: u64,
        version: &clock::Global,
        cx: &mut ModelContext<Self>,
    ) {
        self.channel_states
            .entry(channel_id)
            .or_insert_with(|| Default::default())
            .acknowledge_notes_version(epoch, version);
        cx.notify()
    }

    pub fn update_latest_notes_version(
        &mut self,
        channel_id: ChannelId,
        epoch: u64,
        version: &clock::Global,
        cx: &mut ModelContext<Self>,
    ) {
        self.channel_states
            .entry(channel_id)
            .or_insert_with(|| Default::default())
            .update_latest_notes_version(epoch, version);
        cx.notify()
    }

    pub fn open_channel_chat(
        &mut self,
        channel_id: ChannelId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<ChannelChat>>> {
        let client = self.client.clone();
        let user_store = self.user_store.clone();
        let this = cx.handle();
        self.open_channel_resource(
            channel_id,
            |this| &mut this.opened_chats,
            |channel, cx| ChannelChat::new(channel, this, user_store, client, cx),
            cx,
        )
    }

    /// Asynchronously open a given resource associated with a channel.
    ///
    /// Make sure that the resource is only opened once, even if this method
    /// is called multiple times with the same channel id while the first task
    /// is still running.
    fn open_channel_resource<T, F, Fut>(
        &mut self,
        channel_id: ChannelId,
        get_map: fn(&mut Self) -> &mut HashMap<ChannelId, OpenedModelHandle<T>>,
        load: F,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<T>>>
    where
        F: 'static + FnOnce(Arc<Channel>, AsyncAppContext) -> Fut,
        Fut: Future<Output = Result<Model<T>>>,
        T: 'static,
    {
        let task = loop {
            match get_map(self).entry(channel_id) {
                hash_map::Entry::Occupied(e) => match e.get() {
                    OpenedModelHandle::Open(model) => {
                        if let Some(model) = model.upgrade() {
                            break Task::ready(Ok(model)).shared();
                        } else {
                            get_map(self).remove(&channel_id);
                            continue;
                        }
                    }
                    OpenedModelHandle::Loading(task) => {
                        break task.clone();
                    }
                },
                hash_map::Entry::Vacant(e) => {
                    let task = cx
                        .spawn(move |this, mut cx| async move {
                            let channel = this.update(&mut cx, |this, _| {
                                this.channel_for_id(channel_id).cloned().ok_or_else(|| {
                                    Arc::new(anyhow!("no channel for id: {}", channel_id))
                                })
                            })??;

                            load(channel, cx).await.map_err(Arc::new)
                        })
                        .shared();

                    e.insert(OpenedModelHandle::Loading(task.clone()));
                    cx.spawn({
                        let task = task.clone();
                        move |this, mut cx| async move {
                            let result = task.await;
                            this.update(&mut cx, |this, _| match result {
                                Ok(model) => {
                                    get_map(this).insert(
                                        channel_id,
                                        OpenedModelHandle::Open(model.downgrade()),
                                    );
                                }
                                Err(_) => {
                                    get_map(this).remove(&channel_id);
                                }
                            })
                            .ok();
                        }
                    })
                    .detach();
                    break task;
                }
            }
        };
        cx.background_executor()
            .spawn(async move { task.await.map_err(|error| anyhow!("{}", error)) })
    }

    pub fn is_channel_admin(&self, channel_id: ChannelId) -> bool {
        self.channel_role(channel_id) == proto::ChannelRole::Admin
    }

    pub fn is_root_channel(&self, channel_id: ChannelId) -> bool {
        self.channel_index
            .by_id()
            .get(&channel_id)
            .map_or(false, |channel| channel.is_root_channel())
    }

    pub fn is_public_channel(&self, channel_id: ChannelId) -> bool {
        self.channel_index
            .by_id()
            .get(&channel_id)
            .map_or(false, |channel| {
                channel.visibility == ChannelVisibility::Public
            })
    }

    pub fn channel_capability(&self, channel_id: ChannelId) -> Capability {
        match self.channel_role(channel_id) {
            ChannelRole::Admin | ChannelRole::Member => Capability::ReadWrite,
            _ => Capability::ReadOnly,
        }
    }

    pub fn channel_role(&self, channel_id: ChannelId) -> proto::ChannelRole {
        maybe!({
            let mut channel = self.channel_for_id(channel_id)?;
            if !channel.is_root_channel() {
                channel = self.channel_for_id(channel.root_id())?;
            }
            let root_channel_state = self.channel_states.get(&channel.id);
            root_channel_state?.role
        })
        .unwrap_or(proto::ChannelRole::Guest)
    }

    pub fn channel_participants(&self, channel_id: ChannelId) -> &[Arc<User>] {
        self.channel_participants
            .get(&channel_id)
            .map_or(&[], |v| v.as_slice())
    }

    pub fn create_channel(
        &self,
        name: &str,
        parent_id: Option<ChannelId>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ChannelId>> {
        let client = self.client.clone();
        let name = name.trim_start_matches('#').to_owned();
        cx.spawn(move |this, mut cx| async move {
            let response = client
                .request(proto::CreateChannel {
                    name,
                    parent_id: parent_id.map(|cid| cid.0),
                })
                .await?;

            let channel = response
                .channel
                .ok_or_else(|| anyhow!("missing channel in response"))?;
            let channel_id = ChannelId(channel.id);

            this.update(&mut cx, |this, cx| {
                let task = this.update_channels(
                    proto::UpdateChannels {
                        channels: vec![channel],
                        ..Default::default()
                    },
                    cx,
                );
                assert!(task.is_none());

                // This event is emitted because the collab panel wants to clear the pending edit state
                // before this frame is rendered. But we can't guarantee that the collab panel's future
                // will resolve before this flush_effects finishes. Synchronously emitting this event
                // ensures that the collab panel will observe this creation before the frame completes
                cx.emit(ChannelEvent::ChannelCreated(channel_id));
            })?;

            Ok(channel_id)
        })
    }

    pub fn move_channel(
        &mut self,
        channel_id: ChannelId,
        to: ChannelId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(move |_, _| async move {
            let _ = client
                .request(proto::MoveChannel {
                    channel_id: channel_id.0,
                    to: to.0,
                })
                .await?;

            Ok(())
        })
    }

    pub fn set_channel_visibility(
        &mut self,
        channel_id: ChannelId,
        visibility: ChannelVisibility,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(move |_, _| async move {
            let _ = client
                .request(proto::SetChannelVisibility {
                    channel_id: channel_id.0,
                    visibility: visibility.into(),
                })
                .await?;

            Ok(())
        })
    }

    pub fn invite_member(
        &mut self,
        channel_id: ChannelId,
        user_id: UserId,
        role: proto::ChannelRole,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if !self.outgoing_invites.insert((channel_id, user_id)) {
            return Task::ready(Err(anyhow!("invite request already in progress")));
        }

        cx.notify();
        let client = self.client.clone();
        cx.spawn(move |this, mut cx| async move {
            let result = client
                .request(proto::InviteChannelMember {
                    channel_id: channel_id.0,
                    user_id,
                    role: role.into(),
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.outgoing_invites.remove(&(channel_id, user_id));
                cx.notify();
            })?;

            result?;

            Ok(())
        })
    }

    pub fn remove_member(
        &mut self,
        channel_id: ChannelId,
        user_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if !self.outgoing_invites.insert((channel_id, user_id)) {
            return Task::ready(Err(anyhow!("invite request already in progress")));
        }

        cx.notify();
        let client = self.client.clone();
        cx.spawn(move |this, mut cx| async move {
            let result = client
                .request(proto::RemoveChannelMember {
                    channel_id: channel_id.0,
                    user_id,
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.outgoing_invites.remove(&(channel_id, user_id));
                cx.notify();
            })?;
            result?;
            Ok(())
        })
    }

    pub fn set_member_role(
        &mut self,
        channel_id: ChannelId,
        user_id: UserId,
        role: proto::ChannelRole,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if !self.outgoing_invites.insert((channel_id, user_id)) {
            return Task::ready(Err(anyhow!("member request already in progress")));
        }

        cx.notify();
        let client = self.client.clone();
        cx.spawn(move |this, mut cx| async move {
            let result = client
                .request(proto::SetChannelMemberRole {
                    channel_id: channel_id.0,
                    user_id,
                    role: role.into(),
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.outgoing_invites.remove(&(channel_id, user_id));
                cx.notify();
            })?;

            result?;
            Ok(())
        })
    }

    pub fn rename(
        &mut self,
        channel_id: ChannelId,
        new_name: &str,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.clone();
        let name = new_name.to_string();
        cx.spawn(move |this, mut cx| async move {
            let channel = client
                .request(proto::RenameChannel {
                    channel_id: channel_id.0,
                    name,
                })
                .await?
                .channel
                .ok_or_else(|| anyhow!("missing channel in response"))?;
            this.update(&mut cx, |this, cx| {
                let task = this.update_channels(
                    proto::UpdateChannels {
                        channels: vec![channel],
                        ..Default::default()
                    },
                    cx,
                );
                assert!(task.is_none());

                // This event is emitted because the collab panel wants to clear the pending edit state
                // before this frame is rendered. But we can't guarantee that the collab panel's future
                // will resolve before this flush_effects finishes. Synchronously emitting this event
                // ensures that the collab panel will observe this creation before the frame complete
                cx.emit(ChannelEvent::ChannelRenamed(channel_id))
            })?;
            Ok(())
        })
    }

    pub fn respond_to_channel_invite(
        &mut self,
        channel_id: ChannelId,
        accept: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.background_executor().spawn(async move {
            client
                .request(proto::RespondToChannelInvite {
                    channel_id: channel_id.0,
                    accept,
                })
                .await?;
            Ok(())
        })
    }

    pub fn get_channel_member_details(
        &self,
        channel_id: ChannelId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<ChannelMembership>>> {
        let client = self.client.clone();
        let user_store = self.user_store.downgrade();
        cx.spawn(move |_, mut cx| async move {
            let response = client
                .request(proto::GetChannelMembers {
                    channel_id: channel_id.0,
                })
                .await?;

            let user_ids = response.members.iter().map(|m| m.user_id).collect();
            let user_store = user_store
                .upgrade()
                .ok_or_else(|| anyhow!("user store dropped"))?;
            let users = user_store
                .update(&mut cx, |user_store, cx| user_store.get_users(user_ids, cx))?
                .await?;

            Ok(users
                .into_iter()
                .zip(response.members)
                .map(|(user, member)| ChannelMembership {
                    user,
                    role: member.role(),
                    kind: member.kind(),
                })
                .collect())
        })
    }

    pub fn remove_channel(&self, channel_id: ChannelId) -> impl Future<Output = Result<()>> {
        let client = self.client.clone();
        async move {
            client
                .request(proto::DeleteChannel {
                    channel_id: channel_id.0,
                })
                .await?;
            Ok(())
        }
    }

    pub fn has_pending_channel_invite_response(&self, _: &Arc<Channel>) -> bool {
        false
    }

    pub fn has_pending_channel_invite(&self, channel_id: ChannelId, user_id: UserId) -> bool {
        self.outgoing_invites.contains(&(channel_id, user_id))
    }

    async fn handle_update_channels(
        this: Model<Self>,
        message: TypedEnvelope<proto::UpdateChannels>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            this.update_channels_tx
                .unbounded_send(message.payload)
                .unwrap();
        })?;
        Ok(())
    }

    async fn handle_update_user_channels(
        this: Model<Self>,
        message: TypedEnvelope<proto::UpdateUserChannels>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            for buffer_version in message.payload.observed_channel_buffer_version {
                let version = language::proto::deserialize_version(&buffer_version.version);
                this.acknowledge_notes_version(
                    ChannelId(buffer_version.channel_id),
                    buffer_version.epoch,
                    &version,
                    cx,
                );
            }
            for message_id in message.payload.observed_channel_message_id {
                this.acknowledge_message_id(
                    ChannelId(message_id.channel_id),
                    message_id.message_id,
                    cx,
                );
            }
            for membership in message.payload.channel_memberships {
                if let Some(role) = ChannelRole::from_i32(membership.role) {
                    this.channel_states
                        .entry(ChannelId(membership.channel_id))
                        .or_insert_with(|| ChannelState::default())
                        .set_role(role)
                }
            }
        })
    }

    fn handle_connect(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        self.channel_index.clear();
        self.channel_invitations.clear();
        self.channel_participants.clear();
        self.channel_index.clear();
        self.outgoing_invites.clear();
        self.disconnect_channel_buffers_task.take();

        for chat in self.opened_chats.values() {
            if let OpenedModelHandle::Open(chat) = chat {
                if let Some(chat) = chat.upgrade() {
                    chat.update(cx, |chat, cx| {
                        chat.rejoin(cx);
                    });
                }
            }
        }

        let mut buffer_versions = Vec::new();
        for buffer in self.opened_buffers.values() {
            if let OpenedModelHandle::Open(buffer) = buffer {
                if let Some(buffer) = buffer.upgrade() {
                    let channel_buffer = buffer.read(cx);
                    let buffer = channel_buffer.buffer().read(cx);
                    buffer_versions.push(proto::ChannelBufferVersion {
                        channel_id: channel_buffer.channel_id.0,
                        epoch: channel_buffer.epoch(),
                        version: language::proto::serialize_version(&buffer.version()),
                    });
                }
            }
        }

        if buffer_versions.is_empty() {
            return Task::ready(Ok(()));
        }

        let response = self.client.request(proto::RejoinChannelBuffers {
            buffers: buffer_versions,
        });

        cx.spawn(|this, mut cx| async move {
            let mut response = response.await?;

            this.update(&mut cx, |this, cx| {
                this.opened_buffers.retain(|_, buffer| match buffer {
                    OpenedModelHandle::Open(channel_buffer) => {
                        let Some(channel_buffer) = channel_buffer.upgrade() else {
                            return false;
                        };

                        channel_buffer.update(cx, |channel_buffer, cx| {
                            let channel_id = channel_buffer.channel_id;
                            if let Some(remote_buffer) = response
                                .buffers
                                .iter_mut()
                                .find(|buffer| buffer.channel_id == channel_id.0)
                            {
                                let channel_id = channel_buffer.channel_id;
                                let remote_version =
                                    language::proto::deserialize_version(&remote_buffer.version);

                                channel_buffer.replace_collaborators(
                                    mem::take(&mut remote_buffer.collaborators),
                                    cx,
                                );

                                let operations = channel_buffer
                                    .buffer()
                                    .update(cx, |buffer, cx| {
                                        let outgoing_operations =
                                            buffer.serialize_ops(Some(remote_version), cx);
                                        let incoming_operations =
                                            mem::take(&mut remote_buffer.operations)
                                                .into_iter()
                                                .map(language::proto::deserialize_operation)
                                                .collect::<Result<Vec<_>>>()?;
                                        buffer.apply_ops(incoming_operations, cx)?;
                                        anyhow::Ok(outgoing_operations)
                                    })
                                    .log_err();

                                if let Some(operations) = operations {
                                    let client = this.client.clone();
                                    cx.background_executor()
                                        .spawn(async move {
                                            let operations = operations.await;
                                            for chunk in
                                                language::proto::split_operations(operations)
                                            {
                                                client
                                                    .send(proto::UpdateChannelBuffer {
                                                        channel_id: channel_id.0,
                                                        operations: chunk,
                                                    })
                                                    .ok();
                                            }
                                        })
                                        .detach();
                                    return true;
                                }
                            }

                            channel_buffer.disconnect(cx);
                            false
                        })
                    }
                    OpenedModelHandle::Loading(_) => true,
                });
            })
            .ok();
            anyhow::Ok(())
        })
    }

    fn handle_disconnect(&mut self, wait_for_reconnect: bool, cx: &mut ModelContext<Self>) {
        cx.notify();

        self.disconnect_channel_buffers_task.get_or_insert_with(|| {
            cx.spawn(move |this, mut cx| async move {
                if wait_for_reconnect {
                    cx.background_executor().timer(RECONNECT_TIMEOUT).await;
                }

                if let Some(this) = this.upgrade() {
                    this.update(&mut cx, |this, cx| {
                        for (_, buffer) in this.opened_buffers.drain() {
                            if let OpenedModelHandle::Open(buffer) = buffer {
                                if let Some(buffer) = buffer.upgrade() {
                                    buffer.update(cx, |buffer, cx| buffer.disconnect(cx));
                                }
                            }
                        }
                    })
                    .ok();
                }
            })
        });
    }

    pub(crate) fn update_channels(
        &mut self,
        payload: proto::UpdateChannels,
        cx: &mut ModelContext<ChannelStore>,
    ) -> Option<Task<Result<()>>> {
        if !payload.remove_channel_invitations.is_empty() {
            self.channel_invitations
                .retain(|channel| !payload.remove_channel_invitations.contains(&channel.id.0));
        }
        for channel in payload.channel_invitations {
            match self
                .channel_invitations
                .binary_search_by_key(&channel.id, |c| c.id.0)
            {
                Ok(ix) => {
                    Arc::make_mut(&mut self.channel_invitations[ix]).name = channel.name.into()
                }
                Err(ix) => self.channel_invitations.insert(
                    ix,
                    Arc::new(Channel {
                        id: ChannelId(channel.id),
                        visibility: channel.visibility(),
                        name: channel.name.into(),
                        parent_path: channel
                            .parent_path
                            .into_iter()
                            .map(|cid| ChannelId(cid))
                            .collect(),
                    }),
                ),
            }
        }

        let channels_changed = !payload.channels.is_empty()
            || !payload.delete_channels.is_empty()
            || !payload.latest_channel_message_ids.is_empty()
            || !payload.latest_channel_buffer_versions.is_empty()
            || !payload.hosted_projects.is_empty()
            || !payload.deleted_hosted_projects.is_empty();

        if channels_changed {
            if !payload.delete_channels.is_empty() {
                let delete_channels: Vec<ChannelId> = payload
                    .delete_channels
                    .into_iter()
                    .map(|cid| ChannelId(cid))
                    .collect();
                self.channel_index.delete_channels(&delete_channels);
                self.channel_participants
                    .retain(|channel_id, _| !delete_channels.contains(&channel_id));

                for channel_id in &delete_channels {
                    let channel_id = *channel_id;
                    if payload
                        .channels
                        .iter()
                        .any(|channel| channel.id == channel_id.0)
                    {
                        continue;
                    }
                    if let Some(OpenedModelHandle::Open(buffer)) =
                        self.opened_buffers.remove(&channel_id)
                    {
                        if let Some(buffer) = buffer.upgrade() {
                            buffer.update(cx, ChannelBuffer::disconnect);
                        }
                    }
                }
            }

            let mut index = self.channel_index.bulk_insert();
            for channel in payload.channels {
                let id = ChannelId(channel.id);
                let channel_changed = index.insert(channel);

                if channel_changed {
                    if let Some(OpenedModelHandle::Open(buffer)) = self.opened_buffers.get(&id) {
                        if let Some(buffer) = buffer.upgrade() {
                            buffer.update(cx, ChannelBuffer::channel_changed);
                        }
                    }
                }
            }

            for latest_buffer_version in payload.latest_channel_buffer_versions {
                let version = language::proto::deserialize_version(&latest_buffer_version.version);
                self.channel_states
                    .entry(ChannelId(latest_buffer_version.channel_id))
                    .or_default()
                    .update_latest_notes_version(latest_buffer_version.epoch, &version)
            }

            for latest_channel_message in payload.latest_channel_message_ids {
                self.channel_states
                    .entry(ChannelId(latest_channel_message.channel_id))
                    .or_default()
                    .update_latest_message_id(latest_channel_message.message_id);
            }

            for hosted_project in payload.hosted_projects {
                let hosted_project: HostedProject = hosted_project.into();
                if let Some(old_project) = self
                    .hosted_projects
                    .insert(hosted_project.id, hosted_project.clone())
                {
                    self.channel_states
                        .entry(old_project.channel_id)
                        .or_default()
                        .remove_hosted_project(old_project.id);
                }
                self.channel_states
                    .entry(hosted_project.channel_id)
                    .or_default()
                    .add_hosted_project(hosted_project.id);
            }

            for hosted_project_id in payload.deleted_hosted_projects {
                let hosted_project_id = HostedProjectId(hosted_project_id);

                if let Some(old_project) = self.hosted_projects.remove(&hosted_project_id) {
                    self.channel_states
                        .entry(old_project.channel_id)
                        .or_default()
                        .remove_hosted_project(old_project.id);
                }
            }
        }

        cx.notify();
        if payload.channel_participants.is_empty() {
            return None;
        }

        let mut all_user_ids = Vec::new();
        let channel_participants = payload.channel_participants;
        for entry in &channel_participants {
            for user_id in entry.participant_user_ids.iter() {
                if let Err(ix) = all_user_ids.binary_search(user_id) {
                    all_user_ids.insert(ix, *user_id);
                }
            }
        }

        let users = self
            .user_store
            .update(cx, |user_store, cx| user_store.get_users(all_user_ids, cx));
        Some(cx.spawn(|this, mut cx| async move {
            let users = users.await?;

            this.update(&mut cx, |this, cx| {
                for entry in &channel_participants {
                    let mut participants: Vec<_> = entry
                        .participant_user_ids
                        .iter()
                        .filter_map(|user_id| {
                            users
                                .binary_search_by_key(&user_id, |user| &user.id)
                                .ok()
                                .map(|ix| users[ix].clone())
                        })
                        .collect();

                    participants.sort_by_key(|u| u.id);

                    this.channel_participants
                        .insert(ChannelId(entry.channel_id), participants);
                }

                cx.notify();
            })
        }))
    }
}

impl ChannelState {
    fn set_role(&mut self, role: ChannelRole) {
        self.role = Some(role);
    }

    fn has_channel_buffer_changed(&self) -> bool {
        if let Some(latest_version) = &self.latest_notes_versions {
            if let Some(observed_version) = &self.observed_notes_versions {
                latest_version.epoch > observed_version.epoch
                    || (latest_version.epoch == observed_version.epoch
                        && latest_version
                            .version
                            .changed_since(&observed_version.version))
            } else {
                true
            }
        } else {
            false
        }
    }

    fn has_new_messages(&self) -> bool {
        let latest_message_id = self.latest_chat_message;
        let observed_message_id = self.observed_chat_message;

        latest_message_id.is_some_and(|latest_message_id| {
            latest_message_id > observed_message_id.unwrap_or_default()
        })
    }

    fn last_acknowledged_message_id(&self) -> Option<u64> {
        self.observed_chat_message
    }

    fn acknowledge_message_id(&mut self, message_id: u64) {
        let observed = self.observed_chat_message.get_or_insert(message_id);
        *observed = (*observed).max(message_id);
    }

    fn update_latest_message_id(&mut self, message_id: u64) {
        self.latest_chat_message =
            Some(message_id.max(self.latest_chat_message.unwrap_or_default()));
    }

    fn acknowledge_notes_version(&mut self, epoch: u64, version: &clock::Global) {
        if let Some(existing) = &mut self.observed_notes_versions {
            if existing.epoch == epoch {
                existing.version.join(version);
                return;
            }
        }
        self.observed_notes_versions = Some(NotesVersion {
            epoch,
            version: version.clone(),
        });
    }

    fn update_latest_notes_version(&mut self, epoch: u64, version: &clock::Global) {
        if let Some(existing) = &mut self.latest_notes_versions {
            if existing.epoch == epoch {
                existing.version.join(version);
                return;
            }
        }
        self.latest_notes_versions = Some(NotesVersion {
            epoch,
            version: version.clone(),
        });
    }

    fn add_hosted_project(&mut self, project_id: HostedProjectId) {
        self.projects.insert(project_id);
    }

    fn remove_hosted_project(&mut self, project_id: HostedProjectId) {
        self.projects.remove(&project_id);
    }
}
