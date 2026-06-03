use super::*;

impl Database {
    /// Creates a new user.
    #[cfg(feature = "test-support")]
    pub async fn create_user(&self, admin: bool, params: NewUserParams) -> Result<NewUserResult> {
        self.transaction(|tx| async {
            let tx = tx;
            let user = user::Entity::insert(user::ActiveModel {
                github_login: ActiveValue::set(params.github_login.clone()),
                github_user_id: ActiveValue::set(params.github_user_id),
                admin: ActiveValue::set(admin),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::column(user::Column::GithubUserId)
                    .update_columns([user::Column::Admin, user::Column::GithubLogin])
                    .to_owned(),
            )
            .exec_with_returning(&*tx)
            .await?;

            Ok(NewUserResult { user_id: user.id })
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
}
