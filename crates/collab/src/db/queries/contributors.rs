use super::*;

impl Database {
    /// Records that a given user has signed the CLA.
    #[cfg(feature = "test-support")]
    pub async fn add_contributor(&self, user_id: UserId) -> Result<()> {
        self.transaction(|tx| async move {
            contributor::Entity::insert(contributor::ActiveModel {
                user_id: ActiveValue::Set(user_id),
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
