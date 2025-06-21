#[cfg(test)]
mod fake_stripe_client;
mod real_stripe_client;

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

#[cfg(test)]
pub use fake_stripe_client::*;
pub use real_stripe_client::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, derive_more::Display, Serialize)]
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

#[derive(Debug)]
pub struct UpdateCustomerParams<'a> {
    pub email: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, derive_more::Display)]
pub struct StripeSubscriptionId(pub Arc<str>);

#[derive(Debug, PartialEq, Clone)]
pub struct StripeSubscription {
    pub id: StripeSubscriptionId,
    pub customer: StripeCustomerId,
    // TODO: Create our own version of this enum.
    pub status: stripe::SubscriptionStatus,
    pub current_period_end: i64,
    pub current_period_start: i64,
    pub items: Vec<StripeSubscriptionItem>,
    pub cancel_at: Option<i64>,
    pub cancellation_details: Option<StripeCancellationDetails>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, derive_more::Display)]
pub struct StripeSubscriptionItemId(pub Arc<str>);

#[derive(Debug, PartialEq, Clone)]
pub struct StripeSubscriptionItem {
    pub id: StripeSubscriptionItemId,
    pub price: Option<StripePrice>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StripeCancellationDetails {
    pub reason: Option<StripeCancellationDetailsReason>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StripeCancellationDetailsReason {
    CancellationRequested,
    PaymentDisputed,
    PaymentFailed,
}

#[derive(Debug)]
pub struct StripeCreateSubscriptionParams {
    pub customer: StripeCustomerId,
    pub items: Vec<StripeCreateSubscriptionItems>,
}

#[derive(Debug)]
pub struct StripeCreateSubscriptionItems {
    pub price: Option<StripePriceId>,
    pub quantity: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct UpdateSubscriptionParams {
    pub items: Option<Vec<UpdateSubscriptionItems>>,
    pub trial_settings: Option<StripeSubscriptionTrialSettings>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UpdateSubscriptionItems {
    pub price: Option<StripePriceId>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StripeSubscriptionTrialSettings {
    pub end_behavior: StripeSubscriptionTrialSettingsEndBehavior,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StripeSubscriptionTrialSettingsEndBehavior {
    pub missing_payment_method: StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, derive_more::Display)]
pub struct StripePriceId(pub Arc<str>);

#[derive(Debug, PartialEq, Clone)]
pub struct StripePrice {
    pub id: StripePriceId,
    pub unit_amount: Option<i64>,
    pub lookup_key: Option<String>,
    pub recurring: Option<StripePriceRecurring>,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Serialize)]
pub struct StripeCreateMeterEventParams<'a> {
    pub identifier: &'a str,
    pub event_name: &'a str,
    pub payload: StripeCreateMeterEventPayload<'a>,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct StripeCreateMeterEventPayload<'a> {
    pub value: u64,
    pub stripe_customer_id: &'a StripeCustomerId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StripeBillingAddressCollection {
    Auto,
    Required,
}

#[derive(Debug, Default)]
pub struct StripeCreateCheckoutSessionParams<'a> {
    pub customer: Option<&'a StripeCustomerId>,
    pub client_reference_id: Option<&'a str>,
    pub mode: Option<StripeCheckoutSessionMode>,
    pub line_items: Option<Vec<StripeCreateCheckoutSessionLineItems>>,
    pub payment_method_collection: Option<StripeCheckoutSessionPaymentMethodCollection>,
    pub subscription_data: Option<StripeCreateCheckoutSessionSubscriptionData>,
    pub success_url: Option<&'a str>,
    pub billing_address_collection: Option<StripeBillingAddressCollection>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StripeCheckoutSessionMode {
    Payment,
    Setup,
    Subscription,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StripeCreateCheckoutSessionLineItems {
    pub price: Option<String>,
    pub quantity: Option<u64>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StripeCheckoutSessionPaymentMethodCollection {
    Always,
    IfRequired,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StripeCreateCheckoutSessionSubscriptionData {
    pub metadata: Option<HashMap<String, String>>,
    pub trial_period_days: Option<u32>,
    pub trial_settings: Option<StripeSubscriptionTrialSettings>,
}

#[derive(Debug)]
pub struct StripeCheckoutSession {
    pub url: Option<String>,
}

#[async_trait]
pub trait StripeClient: Send + Sync {
    async fn list_customers_by_email(&self, email: &str) -> Result<Vec<StripeCustomer>>;

    async fn get_customer(&self, customer_id: &StripeCustomerId) -> Result<StripeCustomer>;

    async fn create_customer(&self, params: CreateCustomerParams<'_>) -> Result<StripeCustomer>;

    async fn update_customer(
        &self,
        customer_id: &StripeCustomerId,
        params: UpdateCustomerParams<'_>,
    ) -> Result<StripeCustomer>;

    async fn list_subscriptions_for_customer(
        &self,
        customer_id: &StripeCustomerId,
    ) -> Result<Vec<StripeSubscription>>;

    async fn get_subscription(
        &self,
        subscription_id: &StripeSubscriptionId,
    ) -> Result<StripeSubscription>;

    async fn create_subscription(
        &self,
        params: StripeCreateSubscriptionParams,
    ) -> Result<StripeSubscription>;

    async fn update_subscription(
        &self,
        subscription_id: &StripeSubscriptionId,
        params: UpdateSubscriptionParams,
    ) -> Result<()>;

    async fn cancel_subscription(&self, subscription_id: &StripeSubscriptionId) -> Result<()>;

    async fn list_prices(&self) -> Result<Vec<StripePrice>>;

    async fn list_meters(&self) -> Result<Vec<StripeMeter>>;

    async fn create_meter_event(&self, params: StripeCreateMeterEventParams<'_>) -> Result<()>;

    async fn create_checkout_session(
        &self,
        params: StripeCreateCheckoutSessionParams<'_>,
    ) -> Result<StripeCheckoutSession>;
}
