use crate::rpc::Client;
use anyhow::{anyhow, Result};
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};
use zrpc::proto;

pub use proto::User;

pub struct UserStore {
    users: Mutex<HashMap<u64, Arc<User>>>,
    rpc: Arc<Client>,
}

impl UserStore {
    pub fn new(rpc: Arc<Client>) -> Self {
        Self {
            users: Default::default(),
            rpc,
        }
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
}
