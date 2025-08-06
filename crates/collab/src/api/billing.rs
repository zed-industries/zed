use std::sync::Arc;
use stripe::SubscriptionStatus;

use crate::AppState;
use crate::db::billing_subscription::StripeSubscriptionStatus;
use crate::db::{CreateBillingCustomerParams, billing_customer};
use crate::stripe_client::{StripeClient, StripeCustomerId};

impl From<SubscriptionStatus> for StripeSubscriptionStatus {
    fn from(value: SubscriptionStatus) -> Self {
        match value {
            SubscriptionStatus::Incomplete => Self::Incomplete,
            SubscriptionStatus::IncompleteExpired => Self::IncompleteExpired,
            SubscriptionStatus::Trialing => Self::Trialing,
            SubscriptionStatus::Active => Self::Active,
            SubscriptionStatus::PastDue => Self::PastDue,
            SubscriptionStatus::Canceled => Self::Canceled,
            SubscriptionStatus::Unpaid => Self::Unpaid,
            SubscriptionStatus::Paused => Self::Paused,
        }
    }
}

/// Finds or creates a billing customer using the provided customer.
pub async fn find_or_create_billing_customer(
    app: &Arc<AppState>,
    stripe_client: &dyn StripeClient,
    customer_id: &StripeCustomerId,
) -> anyhow::Result<Option<billing_customer::Model>> {
    // If we already have a billing customer record associated with the Stripe customer,
    // there's nothing more we need to do.
    if let Some(billing_customer) = app
        .db
        .get_billing_customer_by_stripe_customer_id(customer_id.0.as_ref())
        .await?
    {
        return Ok(Some(billing_customer));
    }

    let customer = stripe_client.get_customer(customer_id).await?;

    let Some(email) = customer.email else {
        return Ok(None);
    };

    let Some(user) = app.db.get_user_by_email(&email).await? else {
        return Ok(None);
    };

    let billing_customer = app
        .db
        .create_billing_customer(&CreateBillingCustomerParams {
            user_id: user.id,
            stripe_customer_id: customer.id.to_string(),
        })
        .await?;

    Ok(Some(billing_customer))
}
