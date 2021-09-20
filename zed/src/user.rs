use crate::{
    http::{HttpClient, Method, Request, Url},
    rpc::{Client, Status},
    util::TryFutureExt,
};
use anyhow::{anyhow, Context, Result};
use futures::future;
use gpui::{Entity, ImageData, ModelContext, Task};
use postage::{prelude::Stream, sink::Sink, watch};
use std::{collections::HashMap, sync::Arc};
use zrpc::proto;

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub github_login: String,
    pub avatar: Option<Arc<ImageData>>,
}

pub struct UserStore {
    users: HashMap<u64, Arc<User>>,
    current_user: watch::Receiver<Option<Arc<User>>>,
    rpc: Arc<Client>,
    http: Arc<dyn HttpClient>,
    _maintain_current_user: Task<()>,
}

pub enum Event {}

impl Entity for UserStore {
    type Event = Event;
}

impl UserStore {
    pub fn new(rpc: Arc<Client>, http: Arc<dyn HttpClient>, cx: &mut ModelContext<Self>) -> Self {
        let (mut current_user_tx, current_user_rx) = watch::channel();
        Self {
            users: Default::default(),
            current_user: current_user_rx,
            rpc: rpc.clone(),
            http,
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
