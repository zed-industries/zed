use crate::{
    http::{HttpClient, Method, Request, Url},
    rpc::{Client, Status},
    util::TryFutureExt,
};
use anyhow::{anyhow, Context, Result};
use futures::future;
use gpui::{executor, ImageData, Task};
use parking_lot::Mutex;
use postage::{oneshot, prelude::Stream, sink::Sink, watch};
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
use zrpc::proto;

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub github_login: String,
    pub avatar: Option<Arc<ImageData>>,
}

pub struct UserStore {
    users: Mutex<HashMap<u64, Arc<User>>>,
    current_user: watch::Receiver<Option<Arc<User>>>,
    rpc: Arc<Client>,
    http: Arc<dyn HttpClient>,
    _maintain_current_user: Task<()>,
}

impl UserStore {
    pub fn new(
        rpc: Arc<Client>,
        http: Arc<dyn HttpClient>,
        executor: &executor::Background,
    ) -> Arc<Self> {
        let (mut current_user_tx, current_user_rx) = watch::channel();
        let (mut this_tx, mut this_rx) = oneshot::channel::<Weak<Self>>();
        let this = Arc::new(Self {
            users: Default::default(),
            current_user: current_user_rx,
            rpc: rpc.clone(),
            http,
            _maintain_current_user: executor.spawn(async move {
                let this = if let Some(this) = this_rx.recv().await {
                    this
                } else {
                    return;
                };
                let mut status = rpc.status();
                while let Some(status) = status.recv().await {
                    match status {
                        Status::Connected { .. } => {
                            if let Some((this, user_id)) = this.upgrade().zip(rpc.user_id()) {
                                current_user_tx
                                    .send(this.fetch_user(user_id).log_err().await)
                                    .await
                                    .ok();
                            }
                        }
                        Status::SignedOut => {
                            current_user_tx.send(None).await.ok();
                        }
                        _ => {}
                    }
                }
            }),
        });
        let weak = Arc::downgrade(&this);
        executor
            .spawn(async move { this_tx.send(weak).await })
            .detach();
        this
    }

    pub async fn load_users(&self, mut user_ids: Vec<u64>) -> Result<()> {
        {
            let users = self.users.lock();
            user_ids.retain(|id| !users.contains_key(id));
        }

        if !user_ids.is_empty() {
            let response = self.rpc.request(proto::GetUsers { user_ids }).await?;
            let new_users = future::join_all(
                response
                    .users
                    .into_iter()
                    .map(|user| User::new(user, self.http.as_ref())),
            )
            .await;
            let mut users = self.users.lock();
            for user in new_users {
                users.insert(user.id, Arc::new(user));
            }
        }

        Ok(())
    }

    pub async fn fetch_user(&self, user_id: u64) -> Result<Arc<User>> {
        if let Some(user) = self.users.lock().get(&user_id).cloned() {
            return Ok(user);
        }

        self.load_users(vec![user_id]).await?;
        self.users
            .lock()
            .get(&user_id)
            .cloned()
            .ok_or_else(|| anyhow!("server responded with no users"))
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
