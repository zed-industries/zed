use super::*;
use crate::db::tables::rate_buckets;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

impl Database {
    /// Saves the rate limit for the given user and rate limit name if the last_refill is later
    /// than the currently saved timestamp.
    pub async fn save_rate_bucket(
        &self,
        user_id: UserId,
        rate_limit_name: &str,
        token_count: i32,
        last_refill: DateTime,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            let saved_bucket = rate_buckets::Entity::find()
                .filter(rate_buckets::Column::UserId.eq(user_id))
                .filter(rate_buckets::Column::RateLimitName.eq(rate_limit_name))
                .one(&*tx)
                .await?;

            // Check if the current `last_refill` is older than the one we're trying to save.
            if let Some(saved_bucket) = saved_bucket {
                if last_refill > saved_bucket.last_refill {
                    let bucket = rate_buckets::ActiveModel {
                        token_count: ActiveValue::set(token_count),
                        last_refill: ActiveValue::set(last_refill),
                        ..saved_bucket.into_active_model()
                    };
                    bucket.save(&*tx).await?;
                }
            } else {
                let bucket = rate_buckets::ActiveModel {
                    user_id: ActiveValue::set(user_id),
                    rate_limit_name: ActiveValue::set(rate_limit_name.to_owned()),
                    token_count: ActiveValue::set(token_count),
                    last_refill: ActiveValue::set(last_refill),
                    ..Default::default()
                };
                bucket.save(&*tx).await?;
            }

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
