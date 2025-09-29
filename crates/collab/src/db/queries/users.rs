use anyhow::Context as _;
use chrono::NaiveDateTime;

use super::*;

impl Database {
    /// Creates a new user.
    pub async fn create_user(
        &self,
        email_address: &str,
        name: Option<&str>,
        admin: bool,
        params: NewUserParams,
    ) -> Result<NewUserResult> {
        self.transaction(|tx| async {
            let tx = tx;
            let user = user::Entity::insert(user::ActiveModel {
                email_address: ActiveValue::set(Some(email_address.into())),
                name: ActiveValue::set(name.map(|s| s.into())),
                github_login: ActiveValue::set(params.github_login.clone()),
                github_user_id: ActiveValue::set(params.github_user_id),
                admin: ActiveValue::set(admin),
                metrics_id: ActiveValue::set(Uuid::new_v4()),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::column(user::Column::GithubUserId)
                    .update_columns([
                        user::Column::Admin,
                        user::Column::EmailAddress,
                        user::Column::GithubLogin,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&*tx)
            .await?;

            Ok(NewUserResult {
                user_id: user.id,
                metrics_id: user.metrics_id.to_string(),
                signup_device_id: None,
                inviting_user_id: None,
            })
        })
        .await
    }

    /// Returns a user by ID. There are no access checks here, so this should only be used internally.
    pub async fn get_user_by_id(&self, id: UserId) -> Result<Option<user::Model>> {
        self.transaction(|tx| async move { Ok(user::Entity::find_by_id(id).one(&*tx).await?) })
            .await
    }

    /// Returns all users by ID. There are no access checks here, so this should only be used internally.
    pub async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<user::Model>> {
        if ids.len() >= 10000_usize {
            return Err(anyhow!("too many users"))?;
        }
        self.transaction(|tx| async {
            let tx = tx;
            Ok(user::Entity::find()
                .filter(user::Column::Id.is_in(ids.iter().copied()))
                .all(&*tx)
                .await?)
        })
        .await
    }

    /// Returns all users flagged as staff.
    pub async fn get_staff_users(&self) -> Result<Vec<user::Model>> {
        self.transaction(|tx| async {
            let tx = tx;
            Ok(user::Entity::find()
                .filter(user::Column::Admin.eq(true))
                .all(&*tx)
                .await?)
        })
        .await
    }

    /// Returns a user by email address. There are no access checks here, so this should only be used internally.
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        self.transaction(|tx| async move {
            Ok(user::Entity::find()
                .filter(user::Column::EmailAddress.eq(email))
                .one(&*tx)
                .await?)
        })
        .await
    }

    /// Returns a user by GitHub user ID. There are no access checks here, so this should only be used internally.
    pub async fn get_user_by_github_user_id(&self, github_user_id: i32) -> Result<Option<User>> {
        self.transaction(|tx| async move {
            Ok(user::Entity::find()
                .filter(user::Column::GithubUserId.eq(github_user_id))
                .one(&*tx)
                .await?)
        })
        .await
    }

    /// Returns a user by GitHub login. There are no access checks here, so this should only be used internally.
    pub async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>> {
        self.transaction(|tx| async move {
            Ok(user::Entity::find()
                .filter(user::Column::GithubLogin.eq(github_login))
                .one(&*tx)
                .await?)
        })
        .await
    }

    pub async fn update_or_create_user_by_github_account(
        &self,
        github_login: &str,
        github_user_id: i32,
        github_email: Option<&str>,
        github_name: Option<&str>,
        github_user_created_at: DateTimeUtc,
        initial_channel_id: Option<ChannelId>,
    ) -> Result<User> {
        self.transaction(|tx| async move {
            self.update_or_create_user_by_github_account_tx(
                github_login,
                github_user_id,
                github_email,
                github_name,
                github_user_created_at.naive_utc(),
                initial_channel_id,
                &tx,
            )
            .await
        })
        .await
    }

    pub async fn update_or_create_user_by_github_account_tx(
        &self,
        github_login: &str,
        github_user_id: i32,
        github_email: Option<&str>,
        github_name: Option<&str>,
        github_user_created_at: NaiveDateTime,
        initial_channel_id: Option<ChannelId>,
        tx: &DatabaseTransaction,
    ) -> Result<User> {
        if let Some(existing_user) = self
            .get_user_by_github_user_id_or_github_login(github_user_id, github_login, tx)
            .await?
        {
            let mut existing_user = existing_user.into_active_model();
            existing_user.github_login = ActiveValue::set(github_login.into());
            existing_user.github_user_created_at = ActiveValue::set(Some(github_user_created_at));

            if let Some(github_email) = github_email {
                existing_user.email_address = ActiveValue::set(Some(github_email.into()));
            }

            if let Some(github_name) = github_name {
                existing_user.name = ActiveValue::set(Some(github_name.into()));
            }

            Ok(existing_user.update(tx).await?)
        } else {
            let user = user::Entity::insert(user::ActiveModel {
                email_address: ActiveValue::set(github_email.map(|email| email.into())),
                name: ActiveValue::set(github_name.map(|name| name.into())),
                github_login: ActiveValue::set(github_login.into()),
                github_user_id: ActiveValue::set(github_user_id),
                github_user_created_at: ActiveValue::set(Some(github_user_created_at)),
                admin: ActiveValue::set(false),
                invite_count: ActiveValue::set(0),
                invite_code: ActiveValue::set(None),
                metrics_id: ActiveValue::set(Uuid::new_v4()),
                ..Default::default()
            })
            .exec_with_returning(tx)
            .await?;
            if let Some(channel_id) = initial_channel_id {
                channel_member::Entity::insert(channel_member::ActiveModel {
                    id: ActiveValue::NotSet,
                    channel_id: ActiveValue::Set(channel_id),
                    user_id: ActiveValue::Set(user.id),
                    accepted: ActiveValue::Set(true),
                    role: ActiveValue::Set(ChannelRole::Guest),
                })
                .exec(tx)
                .await?;
            }
            Ok(user)
        }
    }

    /// Tries to retrieve a user, first by their GitHub user ID, and then by their GitHub login.
    ///
    /// Returns `None` if a user is not found with this GitHub user ID or GitHub login.
    pub async fn get_user_by_github_user_id_or_github_login(
        &self,
        github_user_id: i32,
        github_login: &str,
        tx: &DatabaseTransaction,
    ) -> Result<Option<User>> {
        if let Some(user_by_github_user_id) = user::Entity::find()
            .filter(user::Column::GithubUserId.eq(github_user_id))
            .one(tx)
            .await?
        {
            return Ok(Some(user_by_github_user_id));
        }

        if let Some(user_by_github_login) = user::Entity::find()
            .filter(user::Column::GithubLogin.eq(github_login))
            .one(tx)
            .await?
        {
            return Ok(Some(user_by_github_login));
        }

        Ok(None)
    }

    /// get_all_users returns the next page of users. To get more call again with
    /// the same limit and the page incremented by 1.
    pub async fn get_all_users(&self, page: u32, limit: u32) -> Result<Vec<User>> {
        self.transaction(|tx| async move {
            Ok(user::Entity::find()
                .order_by_asc(user::Column::GithubLogin)
                .limit(limit as u64)
                .offset(page as u64 * limit as u64)
                .all(&*tx)
                .await?)
        })
        .await
    }

    /// Returns the metrics id for the user.
    pub async fn get_user_metrics_id(&self, id: UserId) -> Result<String> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryAs {
            MetricsId,
        }

        self.transaction(|tx| async move {
            let metrics_id: Uuid = user::Entity::find_by_id(id)
                .select_only()
                .column(user::Column::MetricsId)
                .into_values::<_, QueryAs>()
                .one(&*tx)
                .await?
                .context("could not find user")?;
            Ok(metrics_id.to_string())
        })
        .await
    }

    /// Sets "connected_once" on the user for analytics.
    pub async fn set_user_connected_once(&self, id: UserId, connected_once: bool) -> Result<()> {
        self.transaction(|tx| async move {
            user::Entity::update_many()
                .filter(user::Column::Id.eq(id))
                .set(user::ActiveModel {
                    connected_once: ActiveValue::set(connected_once),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            Ok(())
        })
        .await
    }

    /// Sets "accepted_tos_at" on the user to the given timestamp.
    pub async fn set_user_accepted_tos_at(
        &self,
        id: UserId,
        accepted_tos_at: Option<DateTime>,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            user::Entity::update_many()
                .filter(user::Column::Id.eq(id))
                .set(user::ActiveModel {
                    accepted_tos_at: ActiveValue::set(accepted_tos_at),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            Ok(())
        })
        .await
    }

    /// hard delete the user.
    pub async fn destroy_user(&self, id: UserId) -> Result<()> {
        self.transaction(|tx| async move {
            access_token::Entity::delete_many()
                .filter(access_token::Column::UserId.eq(id))
                .exec(&*tx)
                .await?;
            user::Entity::delete_by_id(id).exec(&*tx).await?;
            Ok(())
        })
        .await
    }

    /// Find users where github_login ILIKE name_query.
    pub async fn fuzzy_search_users(&self, name_query: &str, limit: u32) -> Result<Vec<User>> {
        self.transaction(|tx| async {
            let tx = tx;
            let like_string = Self::fuzzy_like_string(name_query);
            let query = "
                SELECT users.*
                FROM users
                WHERE github_login ILIKE $1
                ORDER BY github_login <-> $2
                LIMIT $3
            ";

            Ok(user::Entity::find()
                .from_raw_sql(Statement::from_sql_and_values(
                    self.pool.get_database_backend(),
                    query,
                    vec![like_string.into(), name_query.into(), limit.into()],
                ))
                .all(&*tx)
                .await?)
        })
        .await
    }

    /// fuzzy_like_string creates a string for matching in-order using fuzzy_search_users.
    /// e.g. "cir" would become "%c%i%r%"
    pub fn fuzzy_like_string(string: &str) -> String {
        let mut result = String::with_capacity(string.len() * 2 + 1);
        for c in string.chars() {
            if c.is_alphanumeric() {
                result.push('%');
                result.push(c);
            }
        }
        result.push('%');
        result
    }

    pub async fn get_users_missing_github_user_created_at(&self) -> Result<Vec<user::Model>> {
        self.transaction(|tx| async move {
            Ok(user::Entity::find()
                .filter(user::Column::GithubUserCreatedAt.is_null())
                .all(&*tx)
                .await?)
        })
        .await
    }
}
