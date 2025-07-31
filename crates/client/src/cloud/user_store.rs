use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use chrono::{DateTime, Utc};
use cloud_api_client::{AuthenticatedUser, CloudApiClient, PlanInfo};
use cloud_llm_client::Plan;
use gpui::{Context, Task};
use util::{ResultExt as _, maybe};

pub struct CloudUserStore {
    authenticated_user: Option<Arc<AuthenticatedUser>>,
    plan_info: Option<Arc<PlanInfo>>,
    _maintain_authenticated_user_task: Task<()>,
}

impl CloudUserStore {
    pub fn new(cloud_client: Arc<CloudApiClient>, cx: &mut Context<Self>) -> Self {
        Self {
            authenticated_user: None,
            plan_info: None,
            _maintain_authenticated_user_task: cx.spawn(async move |this, cx| {
                maybe!(async move {
                    loop {
                        let Some(this) = this.upgrade() else {
                            return anyhow::Ok(());
                        };

                        if cloud_client.has_credentials() {
                            let already_fetched_authenticated_user = this
                                .read_with(cx, |this, _cx| this.authenticated_user().is_some())
                                .unwrap_or(false);

                            if already_fetched_authenticated_user {
                                // We already fetched the authenticated user; nothing to do.
                            } else {
                                let authenticated_user_result = cloud_client
                                    .get_authenticated_user()
                                    .await
                                    .context("failed to fetch authenticated user");
                                if let Some(response) = authenticated_user_result.log_err() {
                                    this.update(cx, |this, _cx| {
                                        this.authenticated_user = Some(Arc::new(response.user));
                                        this.plan_info = Some(Arc::new(response.plan));
                                    })
                                    .ok();
                                }
                            }
                        } else {
                            this.update(cx, |this, _cx| {
                                this.authenticated_user.take();
                                this.plan_info.take();
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

    pub fn plan(&self) -> Option<Plan> {
        self.plan_info.as_ref().map(|plan| plan.plan)
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
}
