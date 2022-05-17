use super::{http::HttpClient, proto, Client, Status, TypedEnvelope};
use anyhow::{anyhow, Context, Result};
use futures::{channel::mpsc, future, AsyncReadExt, Future, StreamExt};
use gpui::{AsyncAppContext, Entity, ImageData, ModelContext, ModelHandle, Task};
use postage::{prelude::Stream, sink::Sink, watch};
use rpc::proto::{RequestMessage, UsersResponse};
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    sync::{Arc, Weak},
};
use util::TryFutureExt as _;

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub github_login: String,
    pub avatar: Option<Arc<ImageData>>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.github_login == other.github_login
    }
}

impl Eq for User {}

#[derive(Debug)]
pub struct Contact {
    pub user: Arc<User>,
    pub online: bool,
    pub projects: Vec<ProjectMetadata>,
}

#[derive(Debug)]
pub struct ProjectMetadata {
    pub id: u64,
    pub worktree_root_names: Vec<String>,
    pub guests: Vec<Arc<User>>,
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
    update_contacts_tx: mpsc::UnboundedSender<UpdateContacts>,
    current_user: watch::Receiver<Option<Arc<User>>>,
    contacts: Vec<Arc<Contact>>,
    incoming_contact_requests: Vec<Arc<User>>,
    outgoing_contact_requests: Vec<Arc<User>>,
    pending_contact_requests: HashMap<u64, usize>,
    client: Weak<Client>,
    http: Arc<dyn HttpClient>,
    _maintain_contacts: Task<()>,
    _maintain_current_user: Task<()>,
}

#[derive(Clone)]
pub struct ContactEvent {
    pub user: Arc<User>,
    pub kind: ContactEventKind,
}

#[derive(Clone, Copy)]
pub enum ContactEventKind {
    Requested,
    Accepted,
    Cancelled,
}

impl Entity for UserStore {
    type Event = ContactEvent;
}

enum UpdateContacts {
    Update(proto::UpdateContacts),
    Clear(postage::barrier::Sender),
}

impl UserStore {
    pub fn new(
        client: Arc<Client>,
        http: Arc<dyn HttpClient>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let (mut current_user_tx, current_user_rx) = watch::channel();
        let (update_contacts_tx, mut update_contacts_rx) = mpsc::unbounded();
        let rpc_subscription =
            client.add_message_handler(cx.handle(), Self::handle_update_contacts);
        Self {
            users: Default::default(),
            current_user: current_user_rx,
            contacts: Default::default(),
            incoming_contact_requests: Default::default(),
            outgoing_contact_requests: Default::default(),
            client: Arc::downgrade(&client),
            update_contacts_tx,
            http,
            _maintain_contacts: cx.spawn_weak(|this, mut cx| async move {
                let _subscription = rpc_subscription;
                while let Some(message) = update_contacts_rx.next().await {
                    if let Some(this) = this.upgrade(&cx) {
                        this.update(&mut cx, |this, cx| this.update_contacts(message, cx))
                            .log_err()
                            .await;
                    }
                }
            }),
            _maintain_current_user: cx.spawn_weak(|this, mut cx| async move {
                let mut status = client.status();
                while let Some(status) = status.recv().await {
                    match status {
                        Status::Connected { .. } => {
                            if let Some((this, user_id)) = this.upgrade(&cx).zip(client.user_id()) {
                                let user = this
                                    .update(&mut cx, |this, cx| this.fetch_user(user_id, cx))
                                    .log_err()
                                    .await;
                                current_user_tx.send(user).await.ok();
                            }
                        }
                        Status::SignedOut => {
                            current_user_tx.send(None).await.ok();
                            if let Some(this) = this.upgrade(&cx) {
                                this.update(&mut cx, |this, _| this.clear_contacts()).await;
                            }
                        }
                        Status::ConnectionLost => {
                            if let Some(this) = this.upgrade(&cx) {
                                this.update(&mut cx, |this, _| this.clear_contacts()).await;
                            }
                        }
                        _ => {}
                    }
                }
            }),
            pending_contact_requests: Default::default(),
        }
    }

    async fn handle_update_contacts(
        this: ModelHandle<Self>,
        msg: TypedEnvelope<proto::UpdateContacts>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            this.update_contacts_tx
                .unbounded_send(UpdateContacts::Update(msg.payload))
                .unwrap();
        });
        Ok(())
    }

    fn update_contacts(
        &mut self,
        message: UpdateContacts,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        match message {
            UpdateContacts::Clear(barrier) => {
                self.contacts.clear();
                self.incoming_contact_requests.clear();
                self.outgoing_contact_requests.clear();
                drop(barrier);
                Task::ready(Ok(()))
            }
            UpdateContacts::Update(message) => {
                log::info!(
                    "update contacts on client {}: {:?}",
                    self.client.upgrade().unwrap().id,
                    message
                );
                let mut user_ids = HashSet::new();
                for contact in &message.contacts {
                    user_ids.insert(contact.user_id);
                    user_ids.extend(contact.projects.iter().flat_map(|w| &w.guests).copied());
                }
                user_ids.extend(message.incoming_requests.iter().map(|req| req.requester_id));
                user_ids.extend(message.outgoing_requests.iter());

                let load_users = self.get_users(user_ids.into_iter().collect(), cx);
                cx.spawn(|this, mut cx| async move {
                    load_users.await?;

                    // Users are fetched in parallel above and cached in call to get_users
                    // No need to paralellize here
                    let mut updated_contacts = Vec::new();
                    for contact in message.contacts {
                        let should_notify = contact.should_notify;
                        updated_contacts.push((
                            Arc::new(Contact::from_proto(contact, &this, &mut cx).await?),
                            should_notify,
                        ));
                    }

                    let mut incoming_requests = Vec::new();
                    for request in message.incoming_requests {
                        incoming_requests.push({
                            let user = this
                                .update(&mut cx, |this, cx| {
                                    this.fetch_user(request.requester_id, cx)
                                })
                                .await?;
                            (user, request.should_notify)
                        });
                    }

                    let mut outgoing_requests = Vec::new();
                    for requested_user_id in message.outgoing_requests {
                        outgoing_requests.push(
                            this.update(&mut cx, |this, cx| this.fetch_user(requested_user_id, cx))
                                .await?,
                        );
                    }

                    let removed_contacts =
                        HashSet::<u64>::from_iter(message.remove_contacts.iter().copied());
                    let removed_incoming_requests =
                        HashSet::<u64>::from_iter(message.remove_incoming_requests.iter().copied());
                    let removed_outgoing_requests =
                        HashSet::<u64>::from_iter(message.remove_outgoing_requests.iter().copied());

                    this.update(&mut cx, |this, cx| {
                        // Remove contacts
                        this.contacts
                            .retain(|contact| !removed_contacts.contains(&contact.user.id));
                        // Update existing contacts and insert new ones
                        for (updated_contact, should_notify) in updated_contacts {
                            if should_notify {
                                cx.emit(ContactEvent {
                                    user: updated_contact.user.clone(),
                                    kind: ContactEventKind::Accepted,
                                });
                            }
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
                                cx.emit(ContactEvent {
                                    user: user.clone(),
                                    kind: ContactEventKind::Cancelled,
                                });
                                false
                            } else {
                                true
                            }
                        });
                        // Update existing incoming requests and insert new ones
                        for (user, should_notify) in incoming_requests {
                            if should_notify {
                                cx.emit(ContactEvent {
                                    user: user.clone(),
                                    kind: ContactEventKind::Requested,
                                });
                            }

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
                    });

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
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        self.perform_contact_request(responder_id, proto::RequestContact { responder_id }, cx)
    }

    pub fn remove_contact(
        &mut self,
        user_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        self.perform_contact_request(user_id, proto::RemoveContact { user_id }, cx)
    }

    pub fn respond_to_contact_request(
        &mut self,
        requester_id: u64,
        accept: bool,
        cx: &mut ModelContext<Self>,
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
        &mut self,
        requester_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.upgrade();
        cx.spawn_weak(|_, _| async move {
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
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.upgrade();
        *self.pending_contact_requests.entry(user_id).or_insert(0) += 1;
        cx.notify();

        cx.spawn(|this, mut cx| async move {
            let response = client
                .ok_or_else(|| anyhow!("can't upgrade client reference"))?
                .request(request)
                .await;
            this.update(&mut cx, |this, cx| {
                if let Entry::Occupied(mut request_count) =
                    this.pending_contact_requests.entry(user_id)
                {
                    *request_count.get_mut() -= 1;
                    if *request_count.get() == 0 {
                        request_count.remove();
                    }
                }
                cx.notify();
            });
            response?;
            Ok(())
        })
    }

    pub fn clear_contacts(&mut self) -> impl Future<Output = ()> {
        let (tx, mut rx) = postage::barrier::channel();
        self.update_contacts_tx
            .unbounded_send(UpdateContacts::Clear(tx))
            .unwrap();
        async move {
            rx.recv().await;
        }
    }

    pub fn get_users(
        &mut self,
        mut user_ids: Vec<u64>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        user_ids.retain(|id| !self.users.contains_key(id));
        if user_ids.is_empty() {
            Task::ready(Ok(()))
        } else {
            let load = self.load_users(proto::GetUsers { user_ids }, cx);
            cx.foreground().spawn(async move {
                load.await?;
                Ok(())
            })
        }
    }

    pub fn fuzzy_search_users(
        &mut self,
        query: String,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        self.load_users(proto::FuzzySearchUsers { query }, cx)
    }

    pub fn fetch_user(
        &mut self,
        user_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Arc<User>>> {
        if let Some(user) = self.users.get(&user_id).cloned() {
            return cx.foreground().spawn(async move { Ok(user) });
        }

        let load_users = self.get_users(vec![user_id], cx);
        cx.spawn(|this, mut cx| async move {
            load_users.await?;
            this.update(&mut cx, |this, _| {
                this.users
                    .get(&user_id)
                    .cloned()
                    .ok_or_else(|| anyhow!("server responded with no users"))
            })
        })
    }

    pub fn current_user(&self) -> Option<Arc<User>> {
        self.current_user.borrow().clone()
    }

    pub fn watch_current_user(&self) -> watch::Receiver<Option<Arc<User>>> {
        self.current_user.clone()
    }

    fn load_users(
        &mut self,
        request: impl RequestMessage<Response = UsersResponse>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        let client = self.client.clone();
        let http = self.http.clone();
        cx.spawn_weak(|this, mut cx| async move {
            if let Some(rpc) = client.upgrade() {
                let response = rpc.request(request).await.context("error loading users")?;
                let users = future::join_all(
                    response
                        .users
                        .into_iter()
                        .map(|user| User::new(user, http.as_ref())),
                )
                .await;

                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, _| {
                        for user in &users {
                            this.users.insert(user.id, user.clone());
                        }
                    });
                }
                Ok(users)
            } else {
                Ok(Vec::new())
            }
        })
    }
}

impl User {
    async fn new(message: proto::User, http: &dyn HttpClient) -> Arc<Self> {
        Arc::new(User {
            id: message.id,
            github_login: message.github_login,
            avatar: fetch_avatar(http, &message.avatar_url).warn_on_err().await,
        })
    }
}

impl Contact {
    async fn from_proto(
        contact: proto::Contact,
        user_store: &ModelHandle<UserStore>,
        cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        let user = user_store
            .update(cx, |user_store, cx| {
                user_store.fetch_user(contact.user_id, cx)
            })
            .await?;
        let mut projects = Vec::new();
        for project in contact.projects {
            let mut guests = Vec::new();
            for participant_id in project.guests {
                guests.push(
                    user_store
                        .update(cx, |user_store, cx| {
                            user_store.fetch_user(participant_id, cx)
                        })
                        .await?,
                );
            }
            projects.push(ProjectMetadata {
                id: project.id,
                worktree_root_names: project.worktree_root_names.clone(),
                guests,
            });
        }
        Ok(Self {
            user,
            online: contact.online,
            projects,
        })
    }

    pub fn non_empty_projects(&self) -> impl Iterator<Item = &ProjectMetadata> {
        self.projects
            .iter()
            .filter(|project| !project.worktree_root_names.is_empty())
    }
}

async fn fetch_avatar(http: &dyn HttpClient, url: &str) -> Result<Arc<ImageData>> {
    let mut response = http
        .get(url, Default::default(), true)
        .await
        .map_err(|e| anyhow!("failed to send user avatar request: {}", e))?;

    if !response.status().is_success() {
        return Err(anyhow!("avatar request failed {:?}", response.status()));
    }

    let mut body = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .map_err(|e| anyhow!("failed to read user avatar response body: {}", e))?;
    let format = image::guess_format(&body)?;
    let image = image::load_from_memory_with_format(&body, format)?.into_bgra8();
    Ok(ImageData::new(image))
}
