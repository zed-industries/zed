use super::*;

impl Database {
    /// Returns the billing preferences for the given user, if they exist.
    pub async fn get_billing_preferences(
        &self,
        user_id: UserId,
    ) -> Result<Option<billing_preference::Model>> {
        self.transaction(|tx| async move {
            Ok(billing_preference::Entity::find()
                .filter(billing_preference::Column::UserId.eq(user_id))
                .one(&*tx)
                .await?)
        })
        .await
    }
}
