use super::*;

impl Database {
    /// Records that a given user has signed the CLA.
    #[cfg(feature = "test-support")]
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
