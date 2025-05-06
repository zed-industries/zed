use super::*;

#[derive(Debug)]
pub struct CreateBillingCustomerParams {
    pub user_id: UserId,
    pub stripe_customer_id: String,
}

#[derive(Debug, Default)]
pub struct UpdateBillingCustomerParams {
    pub user_id: ActiveValue<UserId>,
    pub stripe_customer_id: ActiveValue<String>,
    pub has_overdue_invoices: ActiveValue<bool>,
    pub trial_started_at: ActiveValue<Option<DateTime>>,
}

impl Database {
    /// Creates a new billing customer.
    pub async fn create_billing_customer(
        &self,
        params: &CreateBillingCustomerParams,
    ) -> Result<billing_customer::Model> {
        self.transaction(|tx| async move {
            let customer = billing_customer::Entity::insert(billing_customer::ActiveModel {
                user_id: ActiveValue::set(params.user_id),
                stripe_customer_id: ActiveValue::set(params.stripe_customer_id.clone()),
                ..Default::default()
            })
            .exec_with_returning(&*tx)
            .await?;

            Ok(customer)
        })
        .await
    }

    /// Updates the specified billing customer.
    pub async fn update_billing_customer(
        &self,
        id: BillingCustomerId,
        params: &UpdateBillingCustomerParams,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            billing_customer::Entity::update(billing_customer::ActiveModel {
                id: ActiveValue::set(id),
                user_id: params.user_id.clone(),
                stripe_customer_id: params.stripe_customer_id.clone(),
                has_overdue_invoices: params.has_overdue_invoices.clone(),
                trial_started_at: params.trial_started_at.clone(),
                created_at: ActiveValue::not_set(),
            })
            .exec(&*tx)
            .await?;

            Ok(())
        })
        .await
    }

    /// Returns the billing customer for the user with the specified ID.
    pub async fn get_billing_customer_by_user_id(
        &self,
        user_id: UserId,
    ) -> Result<Option<billing_customer::Model>> {
        self.transaction(|tx| async move {
            Ok(billing_customer::Entity::find()
                .filter(billing_customer::Column::UserId.eq(user_id))
                .one(&*tx)
                .await?)
        })
        .await
    }

    /// Returns the billing customer for the user with the specified Stripe customer ID.
    pub async fn get_billing_customer_by_stripe_customer_id(
        &self,
        stripe_customer_id: &str,
    ) -> Result<Option<billing_customer::Model>> {
        self.transaction(|tx| async move {
            Ok(billing_customer::Entity::find()
                .filter(billing_customer::Column::StripeCustomerId.eq(stripe_customer_id))
                .one(&*tx)
                .await?)
        })
        .await
    }
}
