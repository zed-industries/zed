use crate::{
    rpc::{Client, Status},
    util::TryFutureExt,
};
use anyhow::{anyhow, Result};
use gpui::{elements::Image, executor, ImageData, Task};
use parking_lot::Mutex;
use postage::{prelude::Stream, sink::Sink, watch};
use std::{collections::HashMap, sync::Arc};
use surf::{
    http::{Method, Request},
    HttpClient, Url,
};
use zrpc::proto;

pub struct User {
    id: u64,
    github_login: String,
    avatar: Option<ImageData>,
}

pub struct UserStore {
    users: Mutex<HashMap<u64, Arc<User>>>,
    current_user: watch::Receiver<Option<Arc<User>>>,
    rpc: Arc<Client>,
    http: Arc<dyn HttpClient>,
    _maintain_current_user: Option<Task<()>>,
}

impl UserStore {
    pub fn new(
        rpc: Arc<Client>,
        http: Arc<dyn HttpClient>,
        executor: &executor::Background,
    ) -> Arc<Self> {
        let (mut current_user_tx, current_user_rx) = watch::channel();

        let mut this = Arc::new(Self {
            users: Default::default(),
            current_user: current_user_rx,
            rpc: rpc.clone(),
            http,
            _maintain_current_user: None,
        });

        let task = {
            let this = Arc::downgrade(&this);
            executor.spawn(async move {
                let mut status = rpc.status();
                while let Some(status) = status.recv().await {
                    match status {
                        Status::Connected { user_id, .. } => {
                            if let Some(this) = this.upgrade() {
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
            })
        };
        Arc::get_mut(&mut this).unwrap()._maintain_current_user = Some(task);

        this
    }

    pub async fn load_users(&self, mut user_ids: Vec<u64>) -> Result<()> {
        {
            let users = self.users.lock();
            user_ids.retain(|id| !users.contains_key(id));
        }

        if !user_ids.is_empty() {
            let response = self.rpc.request(proto::GetUsers { user_ids }).await?;
            let mut users = self.users.lock();
            for user in response.users {
                users.insert(user.id, Arc::new(user));
            }
        }

        Ok(())
    }

    pub async fn fetch_user(&self, user_id: u64) -> Result<Arc<User>> {
        if let Some(user) = self.users.lock().get(&user_id).cloned() {
            return Ok(user);
        }

        let response = self
            .rpc
            .request(proto::GetUsers {
                user_ids: vec![user_id],
            })
            .await?;

        if let Some(user) = response.users.into_iter().next() {
            let user = Arc::new(user);
            self.users.lock().insert(user_id, user.clone());
            Ok(user)
        } else {
            Err(anyhow!("server responded with no users"))
        }
    }

    pub fn current_user(&self) -> &watch::Receiver<Option<Arc<User>>> {
        &self.current_user
    }
}

impl User {
    async fn new(message: proto::User, http: &dyn HttpClient) -> Self {
        let avatar = fetch_avatar(http, &message.avatar_url).await.log_err();
        User {
            id: message.id,
            github_login: message.github_login,
            avatar,
        }
    }
}

async fn fetch_avatar(http: &dyn HttpClient, url: &str) -> Result<Arc<ImageData>> {
    let url = Url::parse(url)?;
    let request = Request::new(Method::Get, url);
    let response = http.send(request).await?;
    let bytes = response.body_bytes().await?;
    let format = image::guess_format(&bytes)?;
    let image = image::load_from_memory_with_format(&bytes, format)?.into_bgra8();
    Ok(ImageData::new(image))
}
