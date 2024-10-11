use super::*;
use crate::Result;
use anyhow::Context as _;

impl LlmDatabase {
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
