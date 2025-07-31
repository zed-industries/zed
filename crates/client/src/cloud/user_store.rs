use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use cloud_api_client::{AuthenticatedUser, CloudApiClient};
use gpui::{Context, Task};
use util::{ResultExt as _, maybe};

pub struct CloudUserStore {
    authenticated_user: Option<Arc<AuthenticatedUser>>,
    _maintain_authenticated_user_task: Task<()>,
}

impl CloudUserStore {
    pub fn new(cloud_client: Arc<CloudApiClient>, cx: &mut Context<Self>) -> Self {
        Self {
            authenticated_user: None,
            _maintain_authenticated_user_task: cx.spawn(async move |this, cx| {
                maybe!(async move {
                    loop {
                        let Some(this) = this.upgrade() else {
                            return anyhow::Ok(());
                        };

                        if cloud_client.has_credentials() {
                            if let Some(response) = cloud_client
                                .get_authenticated_user()
                                .await
                                .context("failed to fetch authenticated user")
                                .log_err()
                            {
                                this.update(cx, |this, _cx| {
                                    this.authenticated_user = Some(Arc::new(response.user));
                                })
                                .ok();
                            }
                        } else {
                            this.update(cx, |this, _cx| {
                                this.authenticated_user = None;
                            })
                            .ok();
                        }

                        cx.background_executor()
                            .timer(Duration::from_millis(100))
                            .await;
                    }
                })
                .await
                .log_err();
            }),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.authenticated_user.is_some()
    }

    pub fn authenticated_user(&self) -> Option<Arc<AuthenticatedUser>> {
        self.authenticated_user.clone()
    }
}
