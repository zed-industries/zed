use super::*;

impl LlmDatabase {
    /// Returns whether the access token with the given `jti` has been revoked.
    pub async fn is_access_token_revoked(&self, jti: &str) -> Result<bool> {
        self.transaction(|tx| async move {
            Ok(revoked_access_token::Entity::find()
                .filter(revoked_access_token::Column::Jti.eq(jti))
                .one(&*tx)
                .await?
                .is_some())
        })
        .await
    }
}
