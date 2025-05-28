#[cfg(test)]
mod fake_stripe_client;
mod real_stripe_client;

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

#[cfg(test)]
pub use fake_stripe_client::*;
pub use real_stripe_client::*;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, derive_more::Display)]
pub struct StripeCustomerId(pub Arc<str>);

#[derive(Debug, Clone)]
pub struct StripeCustomer {
    pub id: StripeCustomerId,
    pub email: Option<String>,
}

#[derive(Debug)]
pub struct CreateCustomerParams<'a> {
    pub email: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, derive_more::Display)]
pub struct StripeSubscriptionId(pub Arc<str>);

#[derive(Debug, Clone)]
pub struct StripeSubscription {
    pub id: StripeSubscriptionId,
    pub items: Vec<StripeSubscriptionItem>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, derive_more::Display)]
pub struct StripeSubscriptionItemId(pub Arc<str>);

#[derive(Debug, Clone)]
pub struct StripeSubscriptionItem {
    pub id: StripeSubscriptionItemId,
    pub price: Option<StripePrice>,
}

#[derive(Debug, Clone)]
pub struct UpdateSubscriptionParams {
    pub items: Option<Vec<UpdateSubscriptionItems>>,
    pub trial_settings: Option<UpdateSubscriptionTrialSettings>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UpdateSubscriptionItems {
    pub price: Option<StripePriceId>,
}

#[derive(Debug, Clone)]
pub struct UpdateSubscriptionTrialSettings {
    pub end_behavior: UpdateSubscriptionTrialSettingsEndBehavior,
}

#[derive(Debug, Clone)]
pub struct UpdateSubscriptionTrialSettingsEndBehavior {
    pub missing_payment_method: UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, derive_more::Display)]
pub struct StripePriceId(pub Arc<str>);

#[derive(Debug, Clone)]
pub struct StripePrice {
    pub id: StripePriceId,
    pub unit_amount: Option<i64>,
    pub lookup_key: Option<String>,
    pub recurring: Option<StripePriceRecurring>,
}

#[derive(Debug, Clone)]
pub struct StripePriceRecurring {
    pub meter: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, derive_more::Display, Deserialize)]
pub struct StripeMeterId(pub Arc<str>);

#[derive(Debug, Clone, Deserialize)]
pub struct StripeMeter {
    pub id: StripeMeterId,
    pub event_name: String,
}

#[async_trait]
pub trait StripeClient: Send + Sync {
    async fn list_customers_by_email(&self, email: &str) -> Result<Vec<StripeCustomer>>;

    async fn create_customer(&self, params: CreateCustomerParams<'_>) -> Result<StripeCustomer>;

    async fn get_subscription(
        &self,
        subscription_id: &StripeSubscriptionId,
    ) -> Result<StripeSubscription>;

    async fn update_subscription(
        &self,
        subscription_id: &StripeSubscriptionId,
        params: UpdateSubscriptionParams,
    ) -> Result<()>;

    async fn list_prices(&self) -> Result<Vec<StripePrice>>;

    async fn list_meters(&self) -> Result<Vec<StripeMeter>>;
}
