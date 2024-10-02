use super::*;

#[derive(Debug)]
pub struct CreateProcessedStripeEventParams {
    pub stripe_event_id: String,
    pub stripe_event_type: String,
    pub stripe_event_created_timestamp: i64,
}

impl Database {
    /// Creates a new processed Stripe event.
    pub async fn create_processed_stripe_event(
        &self,
        params: &CreateProcessedStripeEventParams,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            processed_stripe_event::Entity::insert(processed_stripe_event::ActiveModel {
                stripe_event_id: ActiveValue::set(params.stripe_event_id.clone()),
                stripe_event_type: ActiveValue::set(params.stripe_event_type.clone()),
                stripe_event_created_timestamp: ActiveValue::set(
                    params.stripe_event_created_timestamp,
                ),
                ..Default::default()
            })
            .exec_without_returning(&*tx)
            .await?;

            Ok(())
        })
        .await
    }

    /// Returns the processed Stripe event with the specified event ID.
    pub async fn get_processed_stripe_event_by_event_id(
        &self,
        event_id: &str,
    ) -> Result<Option<processed_stripe_event::Model>> {
        self.transaction(|tx| async move {
            Ok(processed_stripe_event::Entity::find_by_id(event_id)
                .one(&*tx)
                .await?)
        })
        .await
    }

    /// Returns the processed Stripe events with the specified event IDs.
    pub async fn get_processed_stripe_events_by_event_ids(
        &self,
        event_ids: &[&str],
    ) -> Result<Vec<processed_stripe_event::Model>> {
        self.transaction(|tx| async move {
            Ok(processed_stripe_event::Entity::find()
                .filter(
                    processed_stripe_event::Column::StripeEventId.is_in(event_ids.iter().copied()),
                )
                .all(&*tx)
                .await?)
        })
        .await
    }

    /// Returns whether the Stripe event with the specified ID has already been processed.
    pub async fn already_processed_stripe_event(&self, event_id: &str) -> Result<bool> {
        Ok(self
            .get_processed_stripe_event_by_event_id(event_id)
            .await?
            .is_some())
    }
}
