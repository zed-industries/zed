use super::{
    http::{HttpClient, Method, Request, Url},
    proto, Client, Status, TypedEnvelope,
};
use anyhow::{anyhow, Context, Result};
use futures::future;
use gpui::{AsyncAppContext, Entity, ImageData, ModelContext, ModelHandle, Task};
use postage::{prelude::Stream, sink::Sink, watch};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use util::TryFutureExt as _;

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub github_login: String,
    pub avatar: Option<Arc<ImageData>>,
}

#[derive(Debug)]
pub struct Contact {
    pub user: Arc<User>,
    pub projects: Vec<ProjectMetadata>,
}

#[derive(Debug)]
pub struct ProjectMetadata {
    pub id: u64,
    pub is_shared: bool,
    pub worktree_root_names: Vec<String>,
    pub guests: Vec<Arc<User>>,
}

pub struct UserStore {
    users: HashMap<u64, Arc<User>>,
    update_contacts_tx: watch::Sender<Option<proto::UpdateContacts>>,
    current_user: watch::Receiver<Option<Arc<User>>>,
    contacts: Arc<[Contact]>,
    client: Arc<Client>,
    http: Arc<dyn HttpClient>,
    _maintain_contacts: Task<()>,
    _maintain_current_user: Task<()>,
}

pub enum Event {}

impl Entity for UserStore {
    type Event = Event;
}

impl UserStore {
    pub fn new(
        client: Arc<Client>,
        http: Arc<dyn HttpClient>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let (mut current_user_tx, current_user_rx) = watch::channel();
        let (update_contacts_tx, mut update_contacts_rx) =
            watch::channel::<Option<proto::UpdateContacts>>();
        let rpc_subscription =
            client.add_message_handler(cx.handle(), Self::handle_update_contacts);
        Self {
            users: Default::default(),
            current_user: current_user_rx,
            contacts: Arc::from([]),
            client: client.clone(),
            update_contacts_tx,
            http,
            _maintain_contacts: cx.spawn_weak(|this, mut cx| async move {
                let _subscription = rpc_subscription;
                while let Some(message) = update_contacts_rx.recv().await {
                    if let Some((message, this)) = message.zip(this.upgrade(&cx)) {
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
                        }
                        _ => {}
                    }
                }
            }),
        }
    }

    async fn handle_update_contacts(
        this: ModelHandle<Self>,
        msg: TypedEnvelope<proto::UpdateContacts>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            *this.update_contacts_tx.borrow_mut() = Some(msg.payload);
        });
        Ok(())
    }

    fn update_contacts(
        &mut self,
        message: proto::UpdateContacts,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let mut user_ids = HashSet::new();
        for contact in &message.contacts {
            user_ids.insert(contact.user_id);
            user_ids.extend(contact.projects.iter().flat_map(|w| &w.guests).copied());
        }

        let load_users = self.load_users(user_ids.into_iter().collect(), cx);
        cx.spawn(|this, mut cx| async move {
            load_users.await?;

            let mut contacts = Vec::new();
            for contact in message.contacts {
                contacts.push(Contact::from_proto(contact, &this, &mut cx).await?);
            }

            this.update(&mut cx, |this, cx| {
                contacts.sort_by(|a, b| a.user.github_login.cmp(&b.user.github_login));
                this.contacts = contacts.into();
                cx.notify();
            });

            Ok(())
        })
    }

    pub fn contacts(&self) -> &Arc<[Contact]> {
        &self.contacts
    }

    pub fn load_users(
        &mut self,
        mut user_ids: Vec<u64>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let rpc = self.client.clone();
        let http = self.http.clone();
        user_ids.retain(|id| !self.users.contains_key(id));
        cx.spawn_weak(|this, mut cx| async move {
            if !user_ids.is_empty() {
                let response = rpc.request(proto::GetUsers { user_ids }).await?;
                let new_users = future::join_all(
                    response
                        .users
                        .into_iter()
                        .map(|user| User::new(user, http.as_ref())),
                )
                .await;

                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, _| {
                        for user in new_users {
                            this.users.insert(user.id, Arc::new(user));
                        }
                    });
                }
            }

            Ok(())
        })
    }

    pub fn fetch_user(
        &mut self,
        user_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Arc<User>>> {
        if let Some(user) = self.users.get(&user_id).cloned() {
            return cx.spawn_weak(|_, _| async move { Ok(user) });
        }

        let load_users = self.load_users(vec![user_id], cx);
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
}

impl User {
    async fn new(message: proto::User, http: &dyn HttpClient) -> Self {
        User {
            id: message.id,
            github_login: message.github_login,
            avatar: fetch_avatar(http, &message.avatar_url).warn_on_err().await,
        }
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
                is_shared: project.is_shared,
                guests,
            });
        }
        Ok(Self { user, projects })
    }
}

async fn fetch_avatar(http: &dyn HttpClient, url: &str) -> Result<Arc<ImageData>> {
    let url = Url::parse(url).with_context(|| format!("failed to parse avatar url {:?}", url))?;
    let mut request = Request::new(Method::Get, url);
    request.middleware(surf::middleware::Redirect::default());

    let mut response = http
        .send(request)
        .await
        .map_err(|e| anyhow!("failed to send user avatar request: {}", e))?;
    if !response.status().is_success() {
        return Err(anyhow!("avatar request failed {:?}", response.status()));
    }
    let bytes = response
        .body_bytes()
        .await
        .map_err(|e| anyhow!("failed to read user avatar response body: {}", e))?;
    let format = image::guess_format(&bytes)?;
    let image = image::load_from_memory_with_format(&bytes, format)?.into_bgra8();
    Ok(ImageData::new(image))
}
