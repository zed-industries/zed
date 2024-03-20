use super::*;
use crate::db::tables::rate_buckets;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

impl Database {
    /// Saves the rate limit for the given user and rate limit name if the last_refill is later
    /// than the currently saved timestamp.
    pub async fn save_rate_buckets(&self, buckets: &[rate_buckets::Model]) -> Result<()> {
        if buckets.is_empty() {
            return Ok(());
        }

        self.transaction(|tx| async move {
            rate_buckets::Entity::insert_many(buckets.iter().map(|bucket| {
                rate_buckets::ActiveModel {
                    user_id: ActiveValue::Set(bucket.user_id),
                    rate_limit_name: ActiveValue::Set(bucket.rate_limit_name.clone()),
                    token_count: ActiveValue::Set(bucket.token_count),
                    last_refill: ActiveValue::Set(bucket.last_refill),
                }
            }))
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
