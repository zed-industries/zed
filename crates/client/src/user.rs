use super::{Client, Status, TypedEnvelope, proto};
use anyhow::{Context as _, Result, anyhow};
use chrono::{DateTime, Utc};
use collections::{HashMap, HashSet, hash_map::Entry};
use feature_flags::FeatureFlagAppExt;
use futures::{Future, StreamExt, channel::mpsc};
use gpui::{
    App, AsyncApp, Context, Entity, EventEmitter, SharedString, SharedUri, Task, WeakEntity,
};
use postage::{sink::Sink, watch};
use rpc::proto::{RequestMessage, UsersResponse};
use std::sync::{Arc, Weak};
use text::ReplicaId;
use util::TryFutureExt as _;

pub type UserId = u64;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Serialize, serde::Deserialize,
)]
pub struct ChannelId(pub u64);

impl std::fmt::Display for ChannelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ProjectId(pub u64);

impl ProjectId {
    pub fn to_proto(&self) -> u64 {
        self.0
    }
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Serialize, serde::Deserialize,
)]
pub struct DevServerProjectId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParticipantIndex(pub u32);

#[derive(Default, Debug)]
pub struct User {
    pub id: UserId,
    pub github_login: String,
    pub avatar_uri: SharedUri,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Collaborator {
    pub peer_id: proto::PeerId,
    pub replica_id: ReplicaId,
    pub user_id: UserId,
    pub is_host: bool,
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.github_login.cmp(&other.github_login)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.github_login == other.github_login
    }
}

impl Eq for User {}

#[derive(Debug, PartialEq)]
pub struct Contact {
    pub user: Arc<User>,
    pub online: bool,
    pub busy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactRequestStatus {
    None,
    RequestSent,
    RequestReceived,
    RequestAccepted,
}

pub struct UserStore {
    users: HashMap<u64, Arc<User>>,
    by_github_login: HashMap<String, u64>,
    participant_indices: HashMap<u64, ParticipantIndex>,
    update_contacts_tx: mpsc::UnboundedSender<UpdateContacts>,
    current_plan: Option<proto::Plan>,
    trial_started_at: Option<DateTime<Utc>>,
    is_usage_based_billing_enabled: Option<bool>,
    current_user: watch::Receiver<Option<Arc<User>>>,
    accepted_tos_at: Option<Option<DateTime<Utc>>>,
    contacts: Vec<Arc<Contact>>,
    incoming_contact_requests: Vec<Arc<User>>,
    outgoing_contact_requests: Vec<Arc<User>>,
    pending_contact_requests: HashMap<u64, usize>,
    invite_info: Option<InviteInfo>,
    client: Weak<Client>,
    _maintain_contacts: Task<()>,
    _maintain_current_user: Task<Result<()>>,
    weak_self: WeakEntity<Self>,
}

#[derive(Clone)]
pub struct InviteInfo {
    pub count: u32,
    pub url: Arc<str>,
}

pub enum Event {
    Contact {
        user: Arc<User>,
        kind: ContactEventKind,
    },
    ShowContacts,
    ParticipantIndicesChanged,
    PrivateUserInfoUpdated,
}

#[derive(Clone, Copy)]
pub enum ContactEventKind {
    Requested,
    Accepted,
    Cancelled,
}

impl EventEmitter<Event> for UserStore {}

enum UpdateContacts {
    Update(proto::UpdateContacts),
    Wait(postage::barrier::Sender),
    Clear(postage::barrier::Sender),
}

impl UserStore {
    pub fn new(client: Arc<Client>, cx: &Context<Self>) -> Self {
        let (mut current_user_tx, current_user_rx) = watch::channel();
        let (update_contacts_tx, mut update_contacts_rx) = mpsc::unbounded();
        let rpc_subscriptions = vec![
            client.add_message_handler(cx.weak_entity(), Self::handle_update_plan),
            client.add_message_handler(cx.weak_entity(), Self::handle_update_contacts),
            client.add_message_handler(cx.weak_entity(), Self::handle_update_invite_info),
            client.add_message_handler(cx.weak_entity(), Self::handle_show_contacts),
        ];
        Self {
            users: Default::default(),
            by_github_login: Default::default(),
            current_user: current_user_rx,
            current_plan: None,
            trial_started_at: None,
            is_usage_based_billing_enabled: None,
            accepted_tos_at: None,
            contacts: Default::default(),
            incoming_contact_requests: Default::default(),
            participant_indices: Default::default(),
            outgoing_contact_requests: Default::default(),
            invite_info: None,
            client: Arc::downgrade(&client),
            update_contacts_tx,
            _maintain_contacts: cx.spawn(async move |this, cx| {
                let _subscriptions = rpc_subscriptions;
                while let Some(message) = update_contacts_rx.next().await {
                    if let Ok(task) = this.update(cx, |this, cx| this.update_contacts(message, cx))
                    {
                        task.log_err().await;
                    } else {
                        break;
                    }
                }
            }),
            _maintain_current_user: cx.spawn(async move |this, cx| {
                let mut status = client.status();
                let weak = Arc::downgrade(&client);
                drop(client);
                while let Some(status) = status.next().await {
                    // if the client is dropped, the app is shutting down.
                    let Some(client) = weak.upgrade() else {
                        return Ok(());
                    };
                    match status {
                        Status::Connected { .. } => {
                            if let Some(user_id) = client.user_id() {
                                let fetch_user = if let Ok(fetch_user) =
                                    this.update(cx, |this, cx| this.get_user(user_id, cx).log_err())
                                {
                                    fetch_user
                                } else {
                                    break;
                                };
                                let fetch_private_user_info =
                                    client.request(proto::GetPrivateUserInfo {}).log_err();
                                let (user, info) =
                                    futures::join!(fetch_user, fetch_private_user_info);

                                cx.update(|cx| {
                                    if let Some(info) = info {
                                        let staff =
                                            info.staff && !*feature_flags::ZED_DISABLE_STAFF;
                                        cx.update_flags(staff, info.flags);
                                        client.telemetry.set_authenticated_user_info(
                                            Some(info.metrics_id.clone()),
                                            staff,
                                        );

                                        this.update(cx, |this, cx| {
                                            let accepted_tos_at = {
                                                #[cfg(debug_assertions)]
                                                if std::env::var("ZED_IGNORE_ACCEPTED_TOS").is_ok()
                                                {
                                                    None
                                                } else {
                                                    info.accepted_tos_at
                                                }

                                                #[cfg(not(debug_assertions))]
                                                info.accepted_tos_at
                                            };

                                            this.set_current_user_accepted_tos_at(accepted_tos_at);
                                            cx.emit(Event::PrivateUserInfoUpdated);
                                        })
                                    } else {
                                        anyhow::Ok(())
                                    }
                                })??;

                                current_user_tx.send(user).await.ok();

                                this.update(cx, |_, cx| cx.notify())?;
                            }
                        }
                        Status::SignedOut => {
                            current_user_tx.send(None).await.ok();
                            this.update(cx, |this, cx| {
                                this.accepted_tos_at = None;
                                cx.emit(Event::PrivateUserInfoUpdated);
                                cx.notify();
                                this.clear_contacts()
                            })?
                            .await;
                        }
                        Status::ConnectionLost => {
                            this.update(cx, |this, cx| {
                                cx.notify();
                                this.clear_contacts()
                            })?
                            .await;
                        }
                        _ => {}
                    }
                }
                Ok(())
            }),
            pending_contact_requests: Default::default(),
            weak_self: cx.weak_entity(),
        }
    }

    #[cfg(feature = "test-support")]
    pub fn clear_cache(&mut self) {
        self.users.clear();
        self.by_github_login.clear();
    }

    async fn handle_update_invite_info(
        this: Entity<Self>,
        message: TypedEnvelope<proto::UpdateInviteInfo>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.invite_info = Some(InviteInfo {
                url: Arc::from(message.payload.url),
                count: message.payload.count,
            });
            cx.notify();
        })?;
        Ok(())
    }

    async fn handle_show_contacts(
        this: Entity<Self>,
        _: TypedEnvelope<proto::ShowContacts>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |_, cx| cx.emit(Event::ShowContacts))?;
        Ok(())
    }

    pub fn invite_info(&self) -> Option<&InviteInfo> {
        self.invite_info.as_ref()
    }

    async fn handle_update_contacts(
        this: Entity<Self>,
        message: TypedEnvelope<proto::UpdateContacts>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            this.update_contacts_tx
                .unbounded_send(UpdateContacts::Update(message.payload))
                .unwrap();
        })?;
        Ok(())
    }

    async fn handle_update_plan(
        this: Entity<Self>,
        message: TypedEnvelope<proto::UpdateUserPlan>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.current_plan = Some(message.payload.plan());
            this.trial_started_at = message
                .payload
                .trial_started_at
                .and_then(|trial_started_at| DateTime::from_timestamp(trial_started_at as i64, 0));
            this.is_usage_based_billing_enabled = message.payload.is_usage_based_billing_enabled;
            cx.notify();
        })?;
        Ok(())
    }

    fn update_contacts(&mut self, message: UpdateContacts, cx: &Context<Self>) -> Task<Result<()>> {
        match message {
            UpdateContacts::Wait(barrier) => {
                drop(barrier);
                Task::ready(Ok(()))
            }
            UpdateContacts::Clear(barrier) => {
                self.contacts.clear();
                self.incoming_contact_requests.clear();
                self.outgoing_contact_requests.clear();
                drop(barrier);
                Task::ready(Ok(()))
            }
            UpdateContacts::Update(message) => {
                let mut user_ids = HashSet::default();
                for contact in &message.contacts {
                    user_ids.insert(contact.user_id);
                }
                user_ids.extend(message.incoming_requests.iter().map(|req| req.requester_id));
                user_ids.extend(message.outgoing_requests.iter());

                let load_users = self.get_users(user_ids.into_iter().collect(), cx);
                cx.spawn(async move |this, cx| {
                    load_users.await?;

                    // Users are fetched in parallel above and cached in call to get_users
                    // No need to parallelize here
                    let mut updated_contacts = Vec::new();
                    let this = this
                        .upgrade()
                        .ok_or_else(|| anyhow!("can't upgrade user store handle"))?;
                    for contact in message.contacts {
                        updated_contacts
                            .push(Arc::new(Contact::from_proto(contact, &this, cx).await?));
                    }

                    let mut incoming_requests = Vec::new();
                    for request in message.incoming_requests {
                        incoming_requests.push({
                            this.update(cx, |this, cx| this.get_user(request.requester_id, cx))?
                                .await?
                        });
                    }

                    let mut outgoing_requests = Vec::new();
                    for requested_user_id in message.outgoing_requests {
                        outgoing_requests.push(
                            this.update(cx, |this, cx| this.get_user(requested_user_id, cx))?
                                .await?,
                        );
                    }

                    let removed_contacts =
                        HashSet::<u64>::from_iter(message.remove_contacts.iter().copied());
                    let removed_incoming_requests =
                        HashSet::<u64>::from_iter(message.remove_incoming_requests.iter().copied());
                    let removed_outgoing_requests =
                        HashSet::<u64>::from_iter(message.remove_outgoing_requests.iter().copied());

                    this.update(cx, |this, cx| {
                        // Remove contacts
                        this.contacts
                            .retain(|contact| !removed_contacts.contains(&contact.user.id));
                        // Update existing contacts and insert new ones
                        for updated_contact in updated_contacts {
                            match this.contacts.binary_search_by_key(
                                &&updated_contact.user.github_login,
                                |contact| &contact.user.github_login,
                            ) {
                                Ok(ix) => this.contacts[ix] = updated_contact,
                                Err(ix) => this.contacts.insert(ix, updated_contact),
                            }
                        }

                        // Remove incoming contact requests
                        this.incoming_contact_requests.retain(|user| {
                            if removed_incoming_requests.contains(&user.id) {
                                cx.emit(Event::Contact {
                                    user: user.clone(),
                                    kind: ContactEventKind::Cancelled,
                                });
                                false
                            } else {
                                true
                            }
                        });
                        // Update existing incoming requests and insert new ones
                        for user in incoming_requests {
                            match this
                                .incoming_contact_requests
                                .binary_search_by_key(&&user.github_login, |contact| {
                                    &contact.github_login
                                }) {
                                Ok(ix) => this.incoming_contact_requests[ix] = user,
                                Err(ix) => this.incoming_contact_requests.insert(ix, user),
                            }
                        }

                        // Remove outgoing contact requests
                        this.outgoing_contact_requests
                            .retain(|user| !removed_outgoing_requests.contains(&user.id));
                        // Update existing incoming requests and insert new ones
                        for request in outgoing_requests {
                            match this
                                .outgoing_contact_requests
                                .binary_search_by_key(&&request.github_login, |contact| {
                                    &contact.github_login
                                }) {
                                Ok(ix) => this.outgoing_contact_requests[ix] = request,
                                Err(ix) => this.outgoing_contact_requests.insert(ix, request),
                            }
                        }

                        cx.notify();
                    })?;

                    Ok(())
                })
            }
        }
    }

    pub fn contacts(&self) -> &[Arc<Contact>] {
        &self.contacts
    }

    pub fn has_contact(&self, user: &Arc<User>) -> bool {
        self.contacts
            .binary_search_by_key(&&user.github_login, |contact| &contact.user.github_login)
            .is_ok()
    }

    pub fn incoming_contact_requests(&self) -> &[Arc<User>] {
        &self.incoming_contact_requests
    }

    pub fn outgoing_contact_requests(&self) -> &[Arc<User>] {
        &self.outgoing_contact_requests
    }

    pub fn is_contact_request_pending(&self, user: &User) -> bool {
        self.pending_contact_requests.contains_key(&user.id)
    }

    pub fn contact_request_status(&self, user: &User) -> ContactRequestStatus {
        if self
            .contacts
            .binary_search_by_key(&&user.github_login, |contact| &contact.user.github_login)
            .is_ok()
        {
            ContactRequestStatus::RequestAccepted
        } else if self
            .outgoing_contact_requests
            .binary_search_by_key(&&user.github_login, |user| &user.github_login)
            .is_ok()
        {
            ContactRequestStatus::RequestSent
        } else if self
            .incoming_contact_requests
            .binary_search_by_key(&&user.github_login, |user| &user.github_login)
            .is_ok()
        {
            ContactRequestStatus::RequestReceived
        } else {
            ContactRequestStatus::None
        }
    }

    pub fn request_contact(
        &mut self,
        responder_id: u64,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.perform_contact_request(responder_id, proto::RequestContact { responder_id }, cx)
    }

    pub fn remove_contact(&mut self, user_id: u64, cx: &mut Context<Self>) -> Task<Result<()>> {
        self.perform_contact_request(user_id, proto::RemoveContact { user_id }, cx)
    }

    pub fn has_incoming_contact_request(&self, user_id: u64) -> bool {
        self.incoming_contact_requests
            .iter()
            .any(|user| user.id == user_id)
    }

    pub fn respond_to_contact_request(
        &mut self,
        requester_id: u64,
        accept: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.perform_contact_request(
            requester_id,
            proto::RespondToContactRequest {
                requester_id,
                response: if accept {
                    proto::ContactRequestResponse::Accept
                } else {
                    proto::ContactRequestResponse::Decline
                } as i32,
            },
            cx,
        )
    }

    pub fn dismiss_contact_request(
        &self,
        requester_id: u64,
        cx: &Context<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.upgrade();
        cx.spawn(async move |_, _| {
            client
                .ok_or_else(|| anyhow!("can't upgrade client reference"))?
                .request(proto::RespondToContactRequest {
                    requester_id,
                    response: proto::ContactRequestResponse::Dismiss as i32,
                })
                .await?;
            Ok(())
        })
    }

    fn perform_contact_request<T: RequestMessage>(
        &mut self,
        user_id: u64,
        request: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.upgrade();
        *self.pending_contact_requests.entry(user_id).or_insert(0) += 1;
        cx.notify();

        cx.spawn(async move |this, cx| {
            let response = client
                .ok_or_else(|| anyhow!("can't upgrade client reference"))?
                .request(request)
                .await;
            this.update(cx, |this, cx| {
                if let Entry::Occupied(mut request_count) =
                    this.pending_contact_requests.entry(user_id)
                {
                    *request_count.get_mut() -= 1;
                    if *request_count.get() == 0 {
                        request_count.remove();
                    }
                }
                cx.notify();
            })?;
            response?;
            Ok(())
        })
    }

    pub fn clear_contacts(&self) -> impl Future<Output = ()> + use<> {
        let (tx, mut rx) = postage::barrier::channel();
        self.update_contacts_tx
            .unbounded_send(UpdateContacts::Clear(tx))
            .unwrap();
        async move {
            rx.next().await;
        }
    }

    pub fn contact_updates_done(&self) -> impl Future<Output = ()> {
        let (tx, mut rx) = postage::barrier::channel();
        self.update_contacts_tx
            .unbounded_send(UpdateContacts::Wait(tx))
            .unwrap();
        async move {
            rx.next().await;
        }
    }

    pub fn get_users(
        &self,
        user_ids: Vec<u64>,
        cx: &Context<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        let mut user_ids_to_fetch = user_ids.clone();
        user_ids_to_fetch.retain(|id| !self.users.contains_key(id));

        cx.spawn(async move |this, cx| {
            if !user_ids_to_fetch.is_empty() {
                this.update(cx, |this, cx| {
                    this.load_users(
                        proto::GetUsers {
                            user_ids: user_ids_to_fetch,
                        },
                        cx,
                    )
                })?
                .await?;
            }

            this.update(cx, |this, _| {
                user_ids
                    .iter()
                    .map(|user_id| {
                        this.users
                            .get(user_id)
                            .cloned()
                            .ok_or_else(|| anyhow!("user {} not found", user_id))
                    })
                    .collect()
            })?
        })
    }

    pub fn fuzzy_search_users(
        &self,
        query: String,
        cx: &Context<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        self.load_users(proto::FuzzySearchUsers { query }, cx)
    }

    pub fn get_cached_user(&self, user_id: u64) -> Option<Arc<User>> {
        self.users.get(&user_id).cloned()
    }

    pub fn get_user_optimistic(&self, user_id: u64, cx: &Context<Self>) -> Option<Arc<User>> {
        if let Some(user) = self.users.get(&user_id).cloned() {
            return Some(user);
        }

        self.get_user(user_id, cx).detach_and_log_err(cx);
        None
    }

    pub fn get_user(&self, user_id: u64, cx: &Context<Self>) -> Task<Result<Arc<User>>> {
        if let Some(user) = self.users.get(&user_id).cloned() {
            return Task::ready(Ok(user));
        }

        let load_users = self.get_users(vec![user_id], cx);
        cx.spawn(async move |this, cx| {
            load_users.await?;
            this.update(cx, |this, _| {
                this.users
                    .get(&user_id)
                    .cloned()
                    .ok_or_else(|| anyhow!("server responded with no users"))
            })?
        })
    }

    pub fn cached_user_by_github_login(&self, github_login: &str) -> Option<Arc<User>> {
        self.by_github_login
            .get(github_login)
            .and_then(|id| self.users.get(id).cloned())
    }

    pub fn current_user(&self) -> Option<Arc<User>> {
        self.current_user.borrow().clone()
    }

    pub fn current_plan(&self) -> Option<proto::Plan> {
        self.current_plan
    }

    pub fn watch_current_user(&self) -> watch::Receiver<Option<Arc<User>>> {
        self.current_user.clone()
    }

    pub fn current_user_has_accepted_terms(&self) -> Option<bool> {
        self.accepted_tos_at
            .map(|accepted_tos_at| accepted_tos_at.is_some())
    }

    pub fn accept_terms_of_service(&self, cx: &Context<Self>) -> Task<Result<()>> {
        if self.current_user().is_none() {
            return Task::ready(Err(anyhow!("no current user")));
        };

        let client = self.client.clone();
        cx.spawn(async move |this, cx| {
            if let Some(client) = client.upgrade() {
                let response = client
                    .request(proto::AcceptTermsOfService {})
                    .await
                    .context("error accepting tos")?;

                this.update(cx, |this, cx| {
                    this.set_current_user_accepted_tos_at(Some(response.accepted_tos_at));
                    cx.emit(Event::PrivateUserInfoUpdated);
                })
            } else {
                Err(anyhow!("client not found"))
            }
        })
    }

    fn set_current_user_accepted_tos_at(&mut self, accepted_tos_at: Option<u64>) {
        self.accepted_tos_at = Some(
            accepted_tos_at.and_then(|timestamp| DateTime::from_timestamp(timestamp as i64, 0)),
        );
    }

    fn load_users(
        &self,
        request: impl RequestMessage<Response = UsersResponse>,
        cx: &Context<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        let client = self.client.clone();
        cx.spawn(async move |this, cx| {
            if let Some(rpc) = client.upgrade() {
                let response = rpc.request(request).await.context("error loading users")?;
                let users = response.users;

                this.update(cx, |this, _| this.insert(users))
            } else {
                Ok(Vec::new())
            }
        })
    }

    pub fn insert(&mut self, users: Vec<proto::User>) -> Vec<Arc<User>> {
        let mut ret = Vec::with_capacity(users.len());
        for user in users {
            let user = User::new(user);
            if let Some(old) = self.users.insert(user.id, user.clone()) {
                if old.github_login != user.github_login {
                    self.by_github_login.remove(&old.github_login);
                }
            }
            self.by_github_login
                .insert(user.github_login.clone(), user.id);
            ret.push(user)
        }
        ret
    }

    pub fn set_participant_indices(
        &mut self,
        participant_indices: HashMap<u64, ParticipantIndex>,
        cx: &mut Context<Self>,
    ) {
        if participant_indices != self.participant_indices {
            self.participant_indices = participant_indices;
            cx.emit(Event::ParticipantIndicesChanged);
        }
    }

    pub fn participant_indices(&self) -> &HashMap<u64, ParticipantIndex> {
        &self.participant_indices
    }

    pub fn participant_names(
        &self,
        user_ids: impl Iterator<Item = u64>,
        cx: &App,
    ) -> HashMap<u64, SharedString> {
        let mut ret = HashMap::default();
        let mut missing_user_ids = Vec::new();
        for id in user_ids {
            if let Some(github_login) = self.get_cached_user(id).map(|u| u.github_login.clone()) {
                ret.insert(id, github_login.into());
            } else {
                missing_user_ids.push(id)
            }
        }
        if !missing_user_ids.is_empty() {
            let this = self.weak_self.clone();
            cx.spawn(async move |cx| {
                this.update(cx, |this, cx| this.get_users(missing_user_ids, cx))?
                    .await
            })
            .detach_and_log_err(cx);
        }
        ret
    }
}

impl User {
    fn new(message: proto::User) -> Arc<Self> {
        Arc::new(User {
            id: message.id,
            github_login: message.github_login,
            avatar_uri: message.avatar_url.into(),
            name: message.name,
            email: message.email,
        })
    }
}

impl Contact {
    async fn from_proto(
        contact: proto::Contact,
        user_store: &Entity<UserStore>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let user = user_store
            .update(cx, |user_store, cx| {
                user_store.get_user(contact.user_id, cx)
            })?
            .await?;
        Ok(Self {
            user,
            online: contact.online,
            busy: contact.busy,
        })
    }
}

impl Collaborator {
    pub fn from_proto(message: proto::Collaborator) -> Result<Self> {
        Ok(Self {
            peer_id: message.peer_id.ok_or_else(|| anyhow!("invalid peer id"))?,
            replica_id: message.replica_id as ReplicaId,
            user_id: message.user_id as UserId,
            is_host: message.is_host,
        })
    }
}
