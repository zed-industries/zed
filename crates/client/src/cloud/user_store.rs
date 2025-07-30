use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use cloud_api_client::{AuthenticatedUser, CloudApiClient};
use gpui::{Context, Task};
use util::{ResultExt as _, maybe};

pub struct CloudUserStore {
    authenticated_user: Option<AuthenticatedUser>,
    _fetch_authenticated_user_task: Task<()>,
}

impl CloudUserStore {
    pub fn new(cloud_client: Arc<CloudApiClient>, cx: &mut Context<Self>) -> Self {
        Self {
            authenticated_user: None,
            _fetch_authenticated_user_task: cx.spawn(async move |this, cx| {
                maybe!(async move {
                    loop {
                        if cloud_client.has_credentials() {
                            break;
                        }

                        cx.background_executor()
                            .timer(Duration::from_millis(100))
                            .await;
                    }

                    let response = cloud_client.get_authenticated_user().await?;
                    this.update(cx, |this, _cx| {
                        this.authenticated_user = Some(response.user);
                    })
                })
                .await
                .context("failed to fetch authenticated user")
                .log_err();
            }),
        }
    }
}
