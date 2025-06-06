use crate::db::UserId;
use crate::llm::db::queries::subscription_usages::convert_chrono_to_time;

use super::*;

impl LlmDatabase {
    /// Returns all current subscription usage meters as of the given timestamp.
    pub async fn get_current_subscription_usage_meters(
        &self,
        now: DateTimeUtc,
    ) -> Result<Vec<(subscription_usage_meter::Model, subscription_usage::Model)>> {
        let now = convert_chrono_to_time(now)?;

        self.transaction(|tx| async move {
            let result = subscription_usage_meter::Entity::find()
                .inner_join(subscription_usage::Entity)
                .filter(
                    subscription_usage::Column::PeriodStartAt
                        .lte(now)
                        .and(subscription_usage::Column::PeriodEndAt.gte(now)),
                )
                .select_also(subscription_usage::Entity)
                .all(&*tx)
                .await?;

            let result = result
                .into_iter()
                .filter_map(|(meter, usage)| {
                    let usage = usage?;
                    Some((meter, usage))
                })
                .collect();

            Ok(result)
        })
        .await
    }

    /// Returns all current subscription usage meters for the given user as of the given timestamp.
    pub async fn get_current_subscription_usage_meters_for_user(
        &self,
        user_id: UserId,
        now: DateTimeUtc,
    ) -> Result<Vec<(subscription_usage_meter::Model, subscription_usage::Model)>> {
        let now = convert_chrono_to_time(now)?;

        self.transaction(|tx| async move {
            let result = subscription_usage_meter::Entity::find()
                .inner_join(subscription_usage::Entity)
                .filter(subscription_usage::Column::UserId.eq(user_id))
                .filter(
                    subscription_usage::Column::PeriodStartAt
                        .lte(now)
                        .and(subscription_usage::Column::PeriodEndAt.gte(now)),
                )
                .select_also(subscription_usage::Entity)
                .all(&*tx)
                .await?;

            let result = result
                .into_iter()
                .filter_map(|(meter, usage)| {
                    let usage = usage?;
                    Some((meter, usage))
                })
                .collect();

            Ok(result)
        })
        .await
    }
}
