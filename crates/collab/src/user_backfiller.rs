use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use util::ResultExt;

use crate::db::Database;
use crate::executor::Executor;
use crate::{AppState, Config};

pub fn spawn_user_backfiller(app_state: Arc<AppState>) {
    let Some(user_backfiller_github_access_token) =
        app_state.config.user_backfiller_github_access_token.clone()
    else {
        log::info!("no USER_BACKFILLER_GITHUB_ACCESS_TOKEN set; not spawning user backfiller");
        return;
    };

    let executor = app_state.executor.clone();
    executor.spawn_detached({
        let executor = executor.clone();
        async move {
            let user_backfiller = UserBackfiller::new(
                app_state.config.clone(),
                user_backfiller_github_access_token,
                app_state.db.clone(),
                executor,
            );

            log::info!("backfilling users");

            user_backfiller
                .backfill_github_user_created_at()
                .await
                .log_err();
        }
    });
}

struct UserBackfiller {
    config: Config,
    github_access_token: Arc<str>,
    db: Arc<Database>,
    http_client: reqwest::Client,
    executor: Executor,
}

impl UserBackfiller {
    fn new(
        config: Config,
        github_access_token: Arc<str>,
        db: Arc<Database>,
        executor: Executor,
    ) -> Self {
        Self {
            config,
            github_access_token,
            db,
            http_client: reqwest::Client::new(),
            executor,
        }
    }

    async fn backfill_github_user_created_at(&self) -> Result<()> {
        let initial_channel_id = self.config.auto_join_channel_id;

        let users_missing_github_user_created_at =
            self.db.get_users_missing_github_user_created_at().await?;

        for user in users_missing_github_user_created_at {
            match self
                .fetch_github_user(&format!(
                    "https://api.github.com/users/{}",
                    user.github_login
                ))
                .await
            {
                Ok(github_user) => {
                    self.db
                        .get_or_create_user_by_github_account(
                            &user.github_login,
                            Some(github_user.id),
                            user.email_address.as_deref(),
                            Some(github_user.created_at),
                            initial_channel_id,
                        )
                        .await?;

                    log::info!("backfilled user: {}", user.github_login);
                }
                Err(err) => {
                    log::error!("failed to fetch GitHub user {}: {err}", user.github_login);
                }
            }

            self.executor
                .sleep(std::time::Duration::from_millis(200))
                .await;
        }

        Ok(())
    }

    async fn fetch_github_user(&self, url: &str) -> Result<GithubUser> {
        let response = self
            .http_client
            .get(url)
            .header(
                "authorization",
                format!("Bearer {}", self.github_access_token),
            )
            .header("user-agent", "zed")
            .send()
            .await
            .with_context(|| format!("failed to fetch '{url}'"))?;

        let response = match response.error_for_status() {
            Ok(response) => response,
            Err(err) => return Err(anyhow!("failed to fetch GitHub user: {err}")),
        };

        response
            .json()
            .await
            .with_context(|| format!("failed to deserialize GitHub user from '{url}'"))
    }
}

#[derive(serde::Deserialize)]
struct GithubUser {
    id: i32,
    created_at: chrono::DateTime<chrono::Utc>,
}
