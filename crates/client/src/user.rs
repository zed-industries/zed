use super::{Client, Status, TypedEnvelope, proto};
use anyhow::{Context as _, Result};
use chrono::{DateTime, Utc};
use cloud_api_client::websocket_protocol::MessageToClient;
use cloud_api_client::{GetAuthenticatedUserResponse, PlanInfo};
use cloud_llm_client::{
    EDIT_PREDICTIONS_USAGE_AMOUNT_HEADER_NAME, EDIT_PREDICTIONS_USAGE_LIMIT_HEADER_NAME,
    MODEL_REQUESTS_USAGE_AMOUNT_HEADER_NAME, MODEL_REQUESTS_USAGE_LIMIT_HEADER_NAME, Plan,
    UsageLimit,
};
use collections::{HashMap, HashSet, hash_map::Entry};
use derive_more::Deref;
use feature_flags::FeatureFlagAppExt;
use futures::{Future, StreamExt, channel::mpsc};
use gpui::{
    App, AsyncApp, Context, Entity, EventEmitter, SharedString, SharedUri, Task, WeakEntity,
};
use http_client::http::{HeaderMap, HeaderValue};
use postage::{sink::Sink, watch};
use rpc::proto::{RequestMessage, UsersResponse};
use std::{
    str::FromStr as _,
    sync::{Arc, Weak},
};
use text::ReplicaId;
use util::{ResultExt, TryFutureExt as _};

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
    pub fn to_proto(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParticipantIndex(pub u32);

#[derive(Default, Debug)]
pub struct User {
    pub id: UserId,
    pub github_login: SharedString,
    pub avatar_uri: SharedUri,
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Collaborator {
    pub peer_id: proto::PeerId,
    pub replica_id: ReplicaId,
    pub user_id: UserId,
    pub is_host: bool,
    pub committer_name: Option<String>,
    pub committer_email: Option<String>,
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
    by_github_login: HashMap<SharedString, u64>,
    participant_indices: HashMap<u64, ParticipantIndex>,
    update_contacts_tx: mpsc::UnboundedSender<UpdateContacts>,
    model_request_usage: Option<ModelRequestUsage>,
    edit_prediction_usage: Option<EditPredictionUsage>,
    plan_info: Option<PlanInfo>,
    current_user: watch::Receiver<Option<Arc<User>>>,
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
    PlanUpdated,
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

#[derive(Debug, Clone, Copy, Deref)]
pub struct ModelRequestUsage(pub RequestUsage);

#[derive(Debug, Clone, Copy, Deref)]
pub struct EditPredictionUsage(pub RequestUsage);

#[derive(Debug, Clone, Copy)]
pub struct RequestUsage {
    pub limit: UsageLimit,
    pub amount: i32,
}

impl UserStore {
    pub fn new(client: Arc<Client>, cx: &Context<Self>) -> Self {
        let (mut current_user_tx, current_user_rx) = watch::channel();
        let (update_contacts_tx, mut update_contacts_rx) = mpsc::unbounded();
        let rpc_subscriptions = vec![
            client.add_message_handler(cx.weak_entity(), Self::handle_update_contacts),
            client.add_message_handler(cx.weak_entity(), Self::handle_update_invite_info),
            client.add_message_handler(cx.weak_entity(), Self::handle_show_contacts),
        ];

        client.add_message_to_client_handler({
            let this = cx.weak_entity();
            move |message, cx| Self::handle_message_to_client(this.clone(), message, cx)
        });

        Self {
            users: Default::default(),
            by_github_login: Default::default(),
            current_user: current_user_rx,
            plan_info: None,
            model_request_usage: None,
            edit_prediction_usage: None,
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
                        Status::Authenticated
                        | Status::Reauthenticated
                        | Status::Connected { .. } => {
                            if let Some(user_id) = client.user_id() {
                                let response = client
                                    .cloud_client()
                                    .get_authenticated_user()
                                    .await
                                    .log_err();

                                let current_user_and_response = if let Some(response) = response {
                                    let user = Arc::new(User {
                                        id: user_id,
                                        github_login: response.user.github_login.clone().into(),
                                        avatar_uri: response.user.avatar_url.clone().into(),
                                        name: response.user.name.clone(),
                                    });

                                    Some((user, response))
                                } else {
                                    None
                                };
                                current_user_tx
                                    .send(
                                        current_user_and_response
                                            .as_ref()
                                            .map(|(user, _)| user.clone()),
                                    )
                                    .await
                                    .ok();

                                cx.update(|cx| {
                                    if let Some((user, response)) = current_user_and_response {
                                        this.update(cx, |this, cx| {
                                            this.by_github_login
                                                .insert(user.github_login.clone(), user_id);
                                            this.users.insert(user_id, user);
                                            this.update_authenticated_user(response, cx)
                                        })
                                    } else {
                                        anyhow::Ok(())
                                    }
                                })??;

                                this.update(cx, |_, cx| cx.notify())?;
                            }
                        }
                        Status::SignedOut => {
                            current_user_tx.send(None).await.ok();
                            this.update(cx, |this, cx| {
                                this.clear_plan_and_usage();
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
        cx: AsyncApp,
    ) -> Result<()> {
        this.read_with(&cx, |this, _| {
            this.update_contacts_tx
                .unbounded_send(UpdateContacts::Update(message.payload))
                .unwrap();
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
                    let this = this.upgrade().context("can't upgrade user store handle")?;
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
                .context("can't upgrade client reference")?
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
                .context("can't upgrade client reference")?
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

            this.read_with(cx, |this, _| {
                user_ids
                    .iter()
                    .map(|user_id| {
                        this.users
                            .get(user_id)
                            .cloned()
                            .with_context(|| format!("user {user_id} not found"))
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
            this.read_with(cx, |this, _| {
                this.users
                    .get(&user_id)
                    .cloned()
                    .context("server responded with no users")
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

    pub fn plan(&self) -> Option<Plan> {
        #[cfg(debug_assertions)]
        if let Ok(plan) = std::env::var("ZED_SIMULATE_PLAN").as_ref() {
            use cloud_llm_client::PlanV1;

            return match plan.as_str() {
                "free" => Some(Plan::V1(PlanV1::ZedFree)),
                "trial" => Some(Plan::V1(PlanV1::ZedProTrial)),
                "pro" => Some(Plan::V1(PlanV1::ZedPro)),
                _ => {
                    panic!("ZED_SIMULATE_PLAN must be one of 'free', 'trial', or 'pro'");
                }
            };
        }

        self.plan_info.as_ref().map(|info| info.plan())
    }

    pub fn subscription_period(&self) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        self.plan_info
            .as_ref()
            .and_then(|plan| plan.subscription_period)
            .map(|subscription_period| {
                (
                    subscription_period.started_at.0,
                    subscription_period.ended_at.0,
                )
            })
    }

    pub fn trial_started_at(&self) -> Option<DateTime<Utc>> {
        self.plan_info
            .as_ref()
            .and_then(|plan| plan.trial_started_at)
            .map(|trial_started_at| trial_started_at.0)
    }

    /// Returns whether the user's account is too new to use the service.
    pub fn account_too_young(&self) -> bool {
        self.plan_info
            .as_ref()
            .map(|plan| plan.is_account_too_young)
            .unwrap_or_default()
    }

    /// Returns whether the current user has overdue invoices and usage should be blocked.
    pub fn has_overdue_invoices(&self) -> bool {
        self.plan_info
            .as_ref()
            .map(|plan| plan.has_overdue_invoices)
            .unwrap_or_default()
    }

    pub fn is_usage_based_billing_enabled(&self) -> bool {
        self.plan_info
            .as_ref()
            .map(|plan| plan.is_usage_based_billing_enabled)
            .unwrap_or_default()
    }

    pub fn model_request_usage(&self) -> Option<ModelRequestUsage> {
        if self.plan().is_some_and(|plan| plan.is_v2()) {
            return None;
        }

        self.model_request_usage
    }

    pub fn update_model_request_usage(&mut self, usage: ModelRequestUsage, cx: &mut Context<Self>) {
        self.model_request_usage = Some(usage);
        cx.notify();
    }

    pub fn edit_prediction_usage(&self) -> Option<EditPredictionUsage> {
        self.edit_prediction_usage
    }

    pub fn update_edit_prediction_usage(
        &mut self,
        usage: EditPredictionUsage,
        cx: &mut Context<Self>,
    ) {
        self.edit_prediction_usage = Some(usage);
        cx.notify();
    }

    pub fn clear_plan_and_usage(&mut self) {
        self.plan_info = None;
        self.model_request_usage = None;
        self.edit_prediction_usage = None;
    }

    fn update_authenticated_user(
        &mut self,
        response: GetAuthenticatedUserResponse,
        cx: &mut Context<Self>,
    ) {
        let staff = response.user.is_staff && !*feature_flags::ZED_DISABLE_STAFF;
        cx.update_flags(staff, response.feature_flags);
        if let Some(client) = self.client.upgrade() {
            client
                .telemetry
                .set_authenticated_user_info(Some(response.user.metrics_id.clone()), staff);
        }

        self.model_request_usage = Some(ModelRequestUsage(RequestUsage {
            limit: response.plan.usage.model_requests.limit,
            amount: response.plan.usage.model_requests.used as i32,
        }));
        self.edit_prediction_usage = Some(EditPredictionUsage(RequestUsage {
            limit: response.plan.usage.edit_predictions.limit,
            amount: response.plan.usage.edit_predictions.used as i32,
        }));
        self.plan_info = Some(response.plan);
        cx.emit(Event::PrivateUserInfoUpdated);
    }

    fn handle_message_to_client(this: WeakEntity<Self>, message: &MessageToClient, cx: &App) {
        cx.spawn(async move |cx| {
            match message {
                MessageToClient::UserUpdated => {
                    let cloud_client = cx
                        .update(|cx| {
                            this.read_with(cx, |this, _cx| {
                                this.client.upgrade().map(|client| client.cloud_client())
                            })
                        })??
                        .ok_or(anyhow::anyhow!("Failed to get Cloud client"))?;

                    let response = cloud_client.get_authenticated_user().await?;
                    cx.update(|cx| {
                        this.update(cx, |this, cx| {
                            this.update_authenticated_user(response, cx);
                        })
                    })??;
                }
            }

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn watch_current_user(&self) -> watch::Receiver<Option<Arc<User>>> {
        self.current_user.clone()
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
            if let Some(old) = self.users.insert(user.id, user.clone())
                && old.github_login != user.github_login
            {
                self.by_github_login.remove(&old.github_login);
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
                ret.insert(id, github_login);
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
            github_login: message.github_login.into(),
            avatar_uri: message.avatar_url.into(),
            name: message.name,
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
            peer_id: message.peer_id.context("invalid peer id")?,
            replica_id: ReplicaId::new(message.replica_id as u16),
            user_id: message.user_id as UserId,
            is_host: message.is_host,
            committer_name: message.committer_name,
            committer_email: message.committer_email,
        })
    }
}

impl RequestUsage {
    pub fn over_limit(&self) -> bool {
        match self.limit {
            UsageLimit::Limited(limit) => self.amount >= limit,
            UsageLimit::Unlimited => false,
        }
    }

    fn from_headers(
        limit_name: &str,
        amount_name: &str,
        headers: &HeaderMap<HeaderValue>,
    ) -> Result<Self> {
        let limit = headers
            .get(limit_name)
            .with_context(|| format!("missing {limit_name:?} header"))?;
        let limit = UsageLimit::from_str(limit.to_str()?)?;

        let amount = headers
            .get(amount_name)
            .with_context(|| format!("missing {amount_name:?} header"))?;
        let amount = amount.to_str()?.parse::<i32>()?;

        Ok(Self { limit, amount })
    }
}

impl ModelRequestUsage {
    pub fn from_headers(headers: &HeaderMap<HeaderValue>) -> Result<Self> {
        Ok(Self(RequestUsage::from_headers(
            MODEL_REQUESTS_USAGE_LIMIT_HEADER_NAME,
            MODEL_REQUESTS_USAGE_AMOUNT_HEADER_NAME,
            headers,
        )?))
    }
}

impl EditPredictionUsage {
    pub fn from_headers(headers: &HeaderMap<HeaderValue>) -> Result<Self> {
        Ok(Self(RequestUsage::from_headers(
            EDIT_PREDICTIONS_USAGE_LIMIT_HEADER_NAME,
            EDIT_PREDICTIONS_USAGE_AMOUNT_HEADER_NAME,
            headers,
        )?))
    }
}
