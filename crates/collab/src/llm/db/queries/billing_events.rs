use super::*;
use crate::{db::UserId, Result};
use anyhow::Context as _;

impl LlmDatabase {
    pub async fn insert_billing_event(
        &self,
        user_id: UserId,
        model_id: ModelId,
        input_tokens: i64,
        input_cache_creation_tokens: i64,
        input_cache_read_tokens: i64,
        output_tokens: i64,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            billing_event::ActiveModel {
                id: ActiveValue::not_set(),
                idempotency_key: ActiveValue::not_set(),
                user_id: ActiveValue::set(user_id),
                model_id: ActiveValue::set(model_id),
                input_tokens: ActiveValue::set(input_tokens),
                input_cache_creation_tokens: ActiveValue::set(input_cache_creation_tokens),
                input_cache_read_tokens: ActiveValue::set(input_cache_read_tokens),
                output_tokens: ActiveValue::set(output_tokens),
            }
            .insert(&*tx)
            .await?;
            Ok(())
        })
        .await
    }

    pub async fn get_billing_events(&self) -> Result<Vec<(billing_event::Model, model::Model)>> {
        self.transaction(|tx| async move {
            let events_with_models = billing_event::Entity::find()
                .find_also_related(model::Entity)
                .all(&*tx)
                .await?;
            events_with_models
                .into_iter()
                .map(|(event, model)| {
                    let model =
                        model.context("could not find model associated with billing event")?;
                    Ok((event, model))
                })
                .collect()
        })
        .await
    }

    pub async fn consume_billing_event(&self, id: BillingEventId) -> Result<()> {
        self.transaction(|tx| async move {
            billing_event::Entity::delete_by_id(id).exec(&*tx).await?;
            Ok(())
        })
        .await
    }
}
