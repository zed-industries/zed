use super::*;

impl Database {
    /// Creates a new user.
    pub async fn create_user(
        &self,
        email_address: &str,
        admin: bool,
        params: NewUserParams,
    ) -> Result<NewUserResult> {
        self.transaction(|tx| async {
            let tx = tx;
            let user = user::Entity::insert(user::ActiveModel {
                email_address: ActiveValue::set(Some(email_address.into())),
                github_login: ActiveValue::set(params.github_login.clone()),
                github_user_id: ActiveValue::set(Some(params.github_user_id)),
                admin: ActiveValue::set(admin),
                metrics_id: ActiveValue::set(Uuid::new_v4()),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::column(user::Column::GithubLogin)
                    .update_columns([
                        user::Column::Admin,
                        user::Column::EmailAddress,
                        user::Column::GithubUserId,
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
        self.transaction(|tx| async {
            let tx = tx;
            Ok(user::Entity::find()
                .filter(user::Column::Id.is_in(ids.iter().copied()))
                .all(&*tx)
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

    pub async fn get_or_create_user_by_github_account(
        &self,
        github_login: &str,
        github_user_id: Option<i32>,
        github_email: Option<&str>,
    ) -> Result<User> {
        self.transaction(|tx| async move {
            self.get_or_create_user_by_github_account_tx(
                github_login,
                github_user_id,
                github_email,
                &tx,
            )
            .await
        })
        .await
    }

    pub async fn get_or_create_user_by_github_account_tx(
        &self,
        github_login: &str,
        github_user_id: Option<i32>,
        github_email: Option<&str>,
        tx: &DatabaseTransaction,
    ) -> Result<User> {
        if let Some(github_user_id) = github_user_id {
            if let Some(user_by_github_user_id) = user::Entity::find()
                .filter(user::Column::GithubUserId.eq(github_user_id))
                .one(tx)
                .await?
            {
                let mut user_by_github_user_id = user_by_github_user_id.into_active_model();
                user_by_github_user_id.github_login = ActiveValue::set(github_login.into());
                Ok(user_by_github_user_id.update(tx).await?)
            } else if let Some(user_by_github_login) = user::Entity::find()
                .filter(user::Column::GithubLogin.eq(github_login))
                .one(tx)
                .await?
            {
                let mut user_by_github_login = user_by_github_login.into_active_model();
                user_by_github_login.github_user_id = ActiveValue::set(Some(github_user_id));
                Ok(user_by_github_login.update(tx).await?)
            } else {
                let user = user::Entity::insert(user::ActiveModel {
                    email_address: ActiveValue::set(github_email.map(|email| email.into())),
                    github_login: ActiveValue::set(github_login.into()),
                    github_user_id: ActiveValue::set(Some(github_user_id)),
                    admin: ActiveValue::set(false),
                    invite_count: ActiveValue::set(0),
                    invite_code: ActiveValue::set(None),
                    metrics_id: ActiveValue::set(Uuid::new_v4()),
                    ..Default::default()
                })
                .exec_with_returning(&*tx)
                .await?;
                Ok(user)
            }
        } else {
            let user = user::Entity::find()
                .filter(user::Column::GithubLogin.eq(github_login))
                .one(tx)
                .await?
                .ok_or_else(|| anyhow!("no such user {}", github_login))?;
            Ok(user)
        }
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
                .ok_or_else(|| anyhow!("could not find user"))?;
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

    /// Creates a new feature flag.
    pub async fn create_user_flag(&self, flag: &str) -> Result<FlagId> {
        self.transaction(|tx| async move {
            let flag = feature_flag::Entity::insert(feature_flag::ActiveModel {
                flag: ActiveValue::set(flag.to_string()),
                ..Default::default()
            })
            .exec(&*tx)
            .await?
            .last_insert_id;

            Ok(flag)
        })
        .await
    }

    /// Add the given user to the feature flag
    pub async fn add_user_flag(&self, user: UserId, flag: FlagId) -> Result<()> {
        self.transaction(|tx| async move {
            user_feature::Entity::insert(user_feature::ActiveModel {
                user_id: ActiveValue::set(user),
                feature_id: ActiveValue::set(flag),
            })
            .exec(&*tx)
            .await?;

            Ok(())
        })
        .await
    }

    /// Returns the active flags for the user.
    pub async fn get_user_flags(&self, user: UserId) -> Result<Vec<String>> {
        self.transaction(|tx| async move {
            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            enum QueryAs {
                Flag,
            }

            let flags = user::Model {
                id: user,
                ..Default::default()
            }
            .find_linked(user::UserFlags)
            .select_only()
            .column(feature_flag::Column::Flag)
            .into_values::<_, QueryAs>()
            .all(&*tx)
            .await?;

            Ok(flags)
        })
        .await
    }
}
