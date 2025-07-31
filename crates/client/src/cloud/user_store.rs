use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use chrono::{DateTime, Utc};
use cloud_api_client::{AuthenticatedUser, CloudApiClient, GetAuthenticatedUserResponse, PlanInfo};
use cloud_llm_client::Plan;
use gpui::{Context, Entity, Subscription, Task};
use util::{ResultExt as _, maybe};

use crate::UserStore;
use crate::user::Event as RpcUserStoreEvent;

pub struct CloudUserStore {
    cloud_client: Arc<CloudApiClient>,
    authenticated_user: Option<Arc<AuthenticatedUser>>,
    plan_info: Option<Arc<PlanInfo>>,
    _maintain_authenticated_user_task: Task<()>,
    _rpc_plan_updated_subscription: Subscription,
}

impl CloudUserStore {
    pub fn new(
        cloud_client: Arc<CloudApiClient>,
        rpc_user_store: Entity<UserStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let rpc_plan_updated_subscription =
            cx.subscribe(&rpc_user_store, Self::handle_rpc_user_store_event);

        Self {
            cloud_client: cloud_client.clone(),
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
                                        this.update_authenticated_user(response);
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
            _rpc_plan_updated_subscription: rpc_plan_updated_subscription,
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

    fn update_authenticated_user(&mut self, response: GetAuthenticatedUserResponse) {
        self.authenticated_user = Some(Arc::new(response.user));
        self.plan_info = Some(Arc::new(response.plan));
    }

    fn handle_rpc_user_store_event(
        &mut self,
        _: Entity<UserStore>,
        event: &RpcUserStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            RpcUserStoreEvent::PlanUpdated => {
                cx.spawn(async move |this, cx| {
                    let cloud_client =
                        cx.update(|cx| this.read_with(cx, |this, _cx| this.cloud_client.clone()))??;

                    let response = cloud_client
                        .get_authenticated_user()
                        .await
                        .context("failed to fetch authenticated user")?;

                    cx.update(|cx| {
                        this.update(cx, |this, _cx| {
                            this.update_authenticated_user(response);
                        })
                    })??;

                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            }
            _ => {}
        }
    }
}
