use super::*;
use crate::{db::UserId, Result};
use sea_orm::{FromQueryResult, JoinType, QuerySelect as _};

#[derive(FromQueryResult)]
pub struct BillingEventRollup {
    pub max_billing_event_id: BillingEventId,
    pub model_id: ModelId,
    pub model_name: String,
    pub user_id: UserId,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub cache_creation_input_tokens: i64,
    pub cache_read_input_tokens: i64,
    pub price_per_million_input_tokens: i32,
    pub price_per_million_cache_creation_input_tokens: i32,
    pub price_per_million_cache_read_input_tokens: i32,
    pub price_per_million_output_tokens: i32,
}

impl LlmDatabase {
    pub async fn insert_billing_event(
        &self,
        user_id: UserId,
        model_id: ModelId,
        input_tokens: i64,
        cache_creation_input_tokens: i64,
        cache_read_input_tokens: i64,
        output_tokens: i64,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            billing_event::ActiveModel {
                id: ActiveValue::not_set(),
                user_id: ActiveValue::set(user_id),
                model_id: ActiveValue::set(model_id),
                input_tokens: ActiveValue::set(input_tokens),
                cache_creation_input_tokens: ActiveValue::set(cache_creation_input_tokens),
                cache_read_input_tokens: ActiveValue::set(cache_read_input_tokens),
                output_tokens: ActiveValue::set(output_tokens),
            }
            .insert(&*tx)
            .await?;
            Ok(())
        })
        .await
    }

    pub async fn get_billing_event_rollups(&self) -> Result<Vec<BillingEventRollup>> {
        self.transaction(|tx| async move {
            Ok(billing_event::Entity::find()
                .select_only()
                .column(billing_event::Column::UserId)
                .column(billing_event::Column::ModelId)
                .column_as(billing_event::Column::Id.max(), "max_billing_event_id")
                .column_as(billing_event::Column::InputTokens.sum(), "input_tokens")
                .column_as(billing_event::Column::OutputTokens.sum(), "output_tokens")
                .column_as(
                    billing_event::Column::CacheCreationInputTokens.sum(),
                    "cache_creation_input_tokens",
                )
                .column_as(
                    billing_event::Column::CacheReadInputTokens.sum(),
                    "cache_read_input_tokens",
                )
                .column_as(
                    model::Column::PricePerMillionInputTokens.max(),
                    "price_per_million_input_tokens",
                )
                .column_as(
                    model::Column::PricePerMillionCacheCreationInputTokens.max(),
                    "price_per_million_cache_creation_input_tokens",
                )
                .column_as(
                    model::Column::PricePerMillionCacheReadInputTokens.max(),
                    "price_per_million_cache_read_input_tokens",
                )
                .column_as(
                    model::Column::PricePerMillionOutputTokens.max(),
                    "price_per_million_output_tokens",
                )
                .column_as(model::Column::Name.max(), "model_name")
                .join(JoinType::InnerJoin, billing_event::Relation::Model.def())
                .group_by(billing_event::Column::UserId)
                .into_model::<BillingEventRollup>()
                .all(&*tx)
                .await?)
        })
        .await
    }

    pub async fn consume_billing_event_rollup(&self, rollup: BillingEventRollup) -> Result<()> {
        self.transaction(|tx| async move {
            billing_event::Entity::delete_many()
                .filter(
                    billing_event::Column::UserId
                        .eq(rollup.user_id)
                        .and(billing_event::Column::ModelId.eq(rollup.model_id))
                        .and(billing_event::Column::Id.lte(rollup.max_billing_event_id)),
                )
                .exec(&*tx)
                .await?;
            Ok(())
        })
        .await
    }
}
