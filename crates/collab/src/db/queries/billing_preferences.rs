use super::*;

#[derive(Debug)]
pub struct CreateBillingPreferencesParams {
    pub max_monthly_llm_usage_spending_in_cents: i32,
    pub model_request_overages_enabled: bool,
    pub model_request_overages_spend_limit_in_cents: i32,
}

#[derive(Debug, Default)]
pub struct UpdateBillingPreferencesParams {
    pub max_monthly_llm_usage_spending_in_cents: ActiveValue<i32>,
    pub model_request_overages_enabled: ActiveValue<bool>,
    pub model_request_overages_spend_limit_in_cents: ActiveValue<i32>,
}

impl Database {
    /// Returns the billing preferences for the given user, if they exist.
    pub async fn get_billing_preferences(
        &self,
        user_id: UserId,
    ) -> Result<Option<billing_preference::Model>> {
        self.transaction(|tx| async move {
            Ok(billing_preference::Entity::find()
                .filter(billing_preference::Column::UserId.eq(user_id))
                .one(&*tx)
                .await?)
        })
        .await
    }

    /// Creates new billing preferences for the given user.
    pub async fn create_billing_preferences(
        &self,
        user_id: UserId,
        params: &CreateBillingPreferencesParams,
    ) -> Result<billing_preference::Model> {
        self.transaction(|tx| async move {
            let preferences = billing_preference::Entity::insert(billing_preference::ActiveModel {
                user_id: ActiveValue::set(user_id),
                max_monthly_llm_usage_spending_in_cents: ActiveValue::set(
                    params.max_monthly_llm_usage_spending_in_cents,
                ),
                model_request_overages_enabled: ActiveValue::set(
                    params.model_request_overages_enabled,
                ),
                model_request_overages_spend_limit_in_cents: ActiveValue::set(
                    params.model_request_overages_spend_limit_in_cents,
                ),
                ..Default::default()
            })
            .exec_with_returning(&*tx)
            .await?;

            Ok(preferences)
        })
        .await
    }

    /// Updates the billing preferences for the given user.
    pub async fn update_billing_preferences(
        &self,
        user_id: UserId,
        params: &UpdateBillingPreferencesParams,
    ) -> Result<billing_preference::Model> {
        self.transaction(|tx| async move {
            let preferences = billing_preference::Entity::update_many()
                .set(billing_preference::ActiveModel {
                    max_monthly_llm_usage_spending_in_cents: params
                        .max_monthly_llm_usage_spending_in_cents
                        .clone(),
                    model_request_overages_enabled: params.model_request_overages_enabled.clone(),
                    model_request_overages_spend_limit_in_cents: params
                        .model_request_overages_spend_limit_in_cents
                        .clone(),
                    ..Default::default()
                })
                .filter(billing_preference::Column::UserId.eq(user_id))
                .exec_with_returning(&*tx)
                .await?;

            Ok(preferences
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!("billing preferences not found"))?)
        })
        .await
    }
}
