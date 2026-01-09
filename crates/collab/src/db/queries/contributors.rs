use super::*;

impl Database {
    /// Retrieves the GitHub logins of all users who have signed the CLA.
    pub async fn get_contributors(&self) -> Result<Vec<String>> {
        self.transaction(|tx| async move {
            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            enum QueryGithubLogin {
                GithubLogin,
            }

            Ok(contributor::Entity::find()
                .inner_join(user::Entity)
                .order_by_asc(contributor::Column::SignedAt)
                .select_only()
                .column(user::Column::GithubLogin)
                .into_values::<_, QueryGithubLogin>()
                .all(&*tx)
                .await?)
        })
        .await
    }

    /// Records that a given user has signed the CLA.
    pub async fn add_contributor(
        &self,
        github_login: &str,
        github_user_id: i32,
        github_email: Option<&str>,
        github_name: Option<&str>,
        github_user_created_at: DateTimeUtc,
        initial_channel_id: Option<ChannelId>,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            let user = self
                .update_or_create_user_by_github_account_tx(
                    github_login,
                    github_user_id,
                    github_email,
                    github_name,
                    github_user_created_at.naive_utc(),
                    initial_channel_id,
                    &tx,
                )
                .await?;

            contributor::Entity::insert(contributor::ActiveModel {
                user_id: ActiveValue::Set(user.id),
                signed_at: ActiveValue::NotSet,
            })
            .on_conflict(
                OnConflict::column(contributor::Column::UserId)
                    .do_nothing()
                    .to_owned(),
            )
            .exec_without_returning(&*tx)
            .await?;
            Ok(())
        })
        .await
    }
}
