use crate::db::UserId;

use super::*;

impl LlmDatabase {
    pub async fn get_subscription_usage_for_period(
        &self,
        user_id: UserId,
        period_start_at: DateTimeUtc,
        period_end_at: DateTimeUtc,
    ) -> Result<Option<subscription_usage::Model>> {
        self.transaction(|tx| async move {
            Ok(subscription_usage::Entity::find()
                .filter(subscription_usage::Column::UserId.eq(user_id))
                .filter(subscription_usage::Column::PeriodStartAt.eq(period_start_at))
                .filter(subscription_usage::Column::PeriodEndAt.eq(period_end_at))
                .one(&*tx)
                .await?)
        })
        .await
    }
}
