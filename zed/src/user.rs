use crate::{
    http::{HttpClient, Method, Request, Url},
    rpc::{Client, Status},
    util::TryFutureExt,
};
use anyhow::{anyhow, Context, Result};
use futures::future;
use gpui::{AsyncAppContext, Entity, ImageData, ModelContext, ModelHandle, Task};
use postage::{prelude::Stream, sink::Sink, watch};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use zrpc::{proto, TypedEnvelope};

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub github_login: String,
    pub avatar: Option<Arc<ImageData>>,
}

#[derive(Debug)]
pub struct Collaborator {
    pub user: Arc<User>,
    pub worktrees: Vec<WorktreeMetadata>,
}

#[derive(Debug)]
pub struct WorktreeMetadata {
    pub root_name: String,
    pub is_shared: bool,
    pub participants: Vec<Arc<User>>,
}

pub struct UserStore {
    users: HashMap<u64, Arc<User>>,
    current_user: watch::Receiver<Option<Arc<User>>>,
    collaborators: Vec<Collaborator>,
    rpc: Arc<Client>,
    http: Arc<dyn HttpClient>,
    _maintain_collaborators: Task<()>,
    _maintain_current_user: Task<()>,
}

pub enum Event {}

impl Entity for UserStore {
    type Event = Event;
}

impl UserStore {
    pub fn new(rpc: Arc<Client>, http: Arc<dyn HttpClient>, cx: &mut ModelContext<Self>) -> Self {
        let (mut current_user_tx, current_user_rx) = watch::channel();
        let (mut update_collaborators_tx, mut update_collaborators_rx) =
            watch::channel::<Option<proto::UpdateCollaborators>>();
        let update_collaborators_subscription = rpc.subscribe(
            cx,
            move |_: &mut Self, msg: TypedEnvelope<proto::UpdateCollaborators>, _, _| {
                let _ = update_collaborators_tx.blocking_send(Some(msg.payload));
                Ok(())
            },
        );
        Self {
            users: Default::default(),
            current_user: current_user_rx,
            collaborators: Default::default(),
            rpc: rpc.clone(),
            http,
            _maintain_collaborators: cx.spawn_weak(|this, mut cx| async move {
                let _subscription = update_collaborators_subscription;
                while let Some(message) = update_collaborators_rx.recv().await {
                    if let Some((message, this)) = message.zip(this.upgrade(&cx)) {
                        this.update(&mut cx, |this, cx| this.update_collaborators(message, cx))
                            .log_err()
                            .await;
                    }
                }
            }),
            _maintain_current_user: cx.spawn_weak(|this, mut cx| async move {
                let mut status = rpc.status();
                while let Some(status) = status.recv().await {
                    match status {
                        Status::Connected { .. } => {
                            if let Some((this, user_id)) = this.upgrade(&cx).zip(rpc.user_id()) {
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

    fn update_collaborators(
        &mut self,
        message: proto::UpdateCollaborators,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let mut user_ids = HashSet::new();
        for collaborator in &message.collaborators {
            user_ids.insert(collaborator.user_id);
            user_ids.extend(
                collaborator
                    .worktrees
                    .iter()
                    .flat_map(|w| &w.participants)
                    .copied(),
            );
        }

        let load_users = self.load_users(user_ids.into_iter().collect(), cx);
        cx.spawn(|this, mut cx| async move {
            load_users.await?;

            let mut collaborators = Vec::new();
            for collaborator in message.collaborators {
                collaborators.push(Collaborator::from_proto(collaborator, &this, &mut cx).await?);
            }

            this.update(&mut cx, |this, cx| {
                this.collaborators = collaborators;
                cx.notify();
            });

            Ok(())
        })
    }

    pub fn collaborators(&self) -> &[Collaborator] {
        &self.collaborators
    }

    pub fn load_users(
        &mut self,
        mut user_ids: Vec<u64>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let rpc = self.rpc.clone();
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
            avatar: fetch_avatar(http, &message.avatar_url).log_err().await,
        }
    }
}

impl Collaborator {
    async fn from_proto(
        collaborator: proto::Collaborator,
        user_store: &ModelHandle<UserStore>,
        cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        let user = user_store
            .update(cx, |user_store, cx| {
                user_store.fetch_user(collaborator.user_id, cx)
            })
            .await?;
        let mut worktrees = Vec::new();
        for worktree in collaborator.worktrees {
            let mut participants = Vec::new();
            for participant_id in worktree.participants {
                participants.push(
                    user_store
                        .update(cx, |user_store, cx| {
                            user_store.fetch_user(participant_id, cx)
                        })
                        .await?,
                );
            }
            worktrees.push(WorktreeMetadata {
                root_name: worktree.root_name,
                is_shared: worktree.is_shared,
                participants,
            });
        }
        Ok(Self { user, worktrees })
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
    let bytes = response
        .body_bytes()
        .await
        .map_err(|e| anyhow!("failed to read user avatar response body: {}", e))?;
    let format = image::guess_format(&bytes)?;
    let image = image::load_from_memory_with_format(&bytes, format)?.into_bgra8();
    Ok(ImageData::new(image))
}
