use super::*;
use crate::db::tables::rate_buckets;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

impl Database {
    /// Saves the rate limit for the given user and rate limit name if the last_refill is later
    /// than the currently saved timestamp.
    pub async fn save_rate_buckets(&self, buckets: &[rate_buckets::Model]) -> Result<()> {
        self.transaction(|tx| async move {
            rate_buckets::Entity::insert_many(
                buckets
                    .iter()
                    .map(|bucket| bucket.clone().into_active_model()),
            )
            .on_conflict(
                OnConflict::columns([
                    rate_buckets::Column::UserId,
                    rate_buckets::Column::RateLimitName,
                ])
                .update_columns([
                    rate_buckets::Column::TokenCount,
                    rate_buckets::Column::LastRefill,
                ])
                .to_owned(),
            )
            .exec(&*tx)
            .await?;

            Ok(())
        })
        .await
    }

    /// Retrieves the rate limit for the given user and rate limit name.
    pub async fn get_rate_bucket(
        &self,
        user_id: UserId,
        rate_limit_name: &str,
    ) -> Result<Option<rate_buckets::Model>> {
        self.transaction(|tx| async move {
            let rate_limit = rate_buckets::Entity::find()
                .filter(rate_buckets::Column::UserId.eq(user_id))
                .filter(rate_buckets::Column::RateLimitName.eq(rate_limit_name))
                .one(&*tx)
                .await?;

            Ok(rate_limit)
        })
        .await
    }
}
