use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use serde::Serialize;
use stripe::{
    CreateCustomer, Customer, CustomerId, ListCustomers, Price, PriceId, Recurring, Subscription,
    SubscriptionId, SubscriptionItem, SubscriptionItemId, UpdateSubscriptionItems,
    UpdateSubscriptionTrialSettings, UpdateSubscriptionTrialSettingsEndBehavior,
    UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod,
};

use crate::stripe_client::{
    CreateCustomerParams, StripeClient, StripeCreateMeterEventParams, StripeCustomer,
    StripeCustomerId, StripeMeter, StripePrice, StripePriceId, StripePriceRecurring,
    StripeSubscription, StripeSubscriptionId, StripeSubscriptionItem, StripeSubscriptionItemId,
    UpdateSubscriptionParams,
};

pub struct RealStripeClient {
    client: Arc<stripe::Client>,
}

impl RealStripeClient {
    pub fn new(client: Arc<stripe::Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl StripeClient for RealStripeClient {
    async fn list_customers_by_email(&self, email: &str) -> Result<Vec<StripeCustomer>> {
        let response = Customer::list(
            &self.client,
            &ListCustomers {
                email: Some(email),
                ..Default::default()
            },
        )
        .await?;

        Ok(response
            .data
            .into_iter()
            .map(StripeCustomer::from)
            .collect())
    }

    async fn create_customer(&self, params: CreateCustomerParams<'_>) -> Result<StripeCustomer> {
        let customer = Customer::create(
            &self.client,
            CreateCustomer {
                email: params.email,
                ..Default::default()
            },
        )
        .await?;

        Ok(StripeCustomer::from(customer))
    }

    async fn get_subscription(
        &self,
        subscription_id: &StripeSubscriptionId,
    ) -> Result<StripeSubscription> {
        let subscription_id = subscription_id.try_into()?;

        let subscription = Subscription::retrieve(&self.client, &subscription_id, &[]).await?;

        Ok(StripeSubscription::from(subscription))
    }

    async fn update_subscription(
        &self,
        subscription_id: &StripeSubscriptionId,
        params: UpdateSubscriptionParams,
    ) -> Result<()> {
        let subscription_id = subscription_id.try_into()?;

        stripe::Subscription::update(
            &self.client,
            &subscription_id,
            stripe::UpdateSubscription {
                items: params.items.map(|items| {
                    items
                        .into_iter()
                        .map(|item| UpdateSubscriptionItems {
                            price: item.price.map(|price| price.to_string()),
                            ..Default::default()
                        })
                        .collect()
                }),
                trial_settings: params.trial_settings.map(Into::into),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    async fn list_prices(&self) -> Result<Vec<StripePrice>> {
        let response = stripe::Price::list(
            &self.client,
            &stripe::ListPrices {
                limit: Some(100),
                ..Default::default()
            },
        )
        .await?;

        Ok(response.data.into_iter().map(StripePrice::from).collect())
    }

    async fn list_meters(&self) -> Result<Vec<StripeMeter>> {
        #[derive(Serialize)]
        struct Params {
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u64>,
        }

        let response = self
            .client
            .get_query::<stripe::List<StripeMeter>, _>(
                "/billing/meters",
                Params { limit: Some(100) },
            )
            .await?;

        Ok(response.data)
    }

    async fn create_meter_event(&self, params: StripeCreateMeterEventParams<'_>) -> Result<()> {
        let identifier = params.identifier;
        match self.client.post_form("/billing/meter_events", params).await {
            Ok(event) => Ok(event),
            Err(stripe::StripeError::Stripe(error)) => {
                if error.http_status == 400
                    && error
                        .message
                        .as_ref()
                        .map_or(false, |message| message.contains(identifier))
                {
                    Ok(())
                } else {
                    Err(anyhow!(stripe::StripeError::Stripe(error)))
                }
            }
            Err(error) => Err(anyhow!(error)),
        }
    }
}

impl From<CustomerId> for StripeCustomerId {
    fn from(value: CustomerId) -> Self {
        Self(value.as_str().into())
    }
}

impl TryFrom<StripeCustomerId> for CustomerId {
    type Error = anyhow::Error;

    fn try_from(value: StripeCustomerId) -> Result<Self, Self::Error> {
        Self::from_str(value.0.as_ref()).context("failed to parse Stripe customer ID")
    }
}

impl From<Customer> for StripeCustomer {
    fn from(value: Customer) -> Self {
        StripeCustomer {
            id: value.id.into(),
            email: value.email,
        }
    }
}

impl From<SubscriptionId> for StripeSubscriptionId {
    fn from(value: SubscriptionId) -> Self {
        Self(value.as_str().into())
    }
}

impl TryFrom<&StripeSubscriptionId> for SubscriptionId {
    type Error = anyhow::Error;

    fn try_from(value: &StripeSubscriptionId) -> Result<Self, Self::Error> {
        Self::from_str(value.0.as_ref()).context("failed to parse Stripe subscription ID")
    }
}

impl From<Subscription> for StripeSubscription {
    fn from(value: Subscription) -> Self {
        Self {
            id: value.id.into(),
            items: value.items.data.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<SubscriptionItemId> for StripeSubscriptionItemId {
    fn from(value: SubscriptionItemId) -> Self {
        Self(value.as_str().into())
    }
}

impl From<SubscriptionItem> for StripeSubscriptionItem {
    fn from(value: SubscriptionItem) -> Self {
        Self {
            id: value.id.into(),
            price: value.price.map(Into::into),
        }
    }
}

impl From<crate::stripe_client::UpdateSubscriptionTrialSettings>
    for UpdateSubscriptionTrialSettings
{
    fn from(value: crate::stripe_client::UpdateSubscriptionTrialSettings) -> Self {
        Self {
            end_behavior: value.end_behavior.into(),
        }
    }
}

impl From<crate::stripe_client::UpdateSubscriptionTrialSettingsEndBehavior>
    for UpdateSubscriptionTrialSettingsEndBehavior
{
    fn from(value: crate::stripe_client::UpdateSubscriptionTrialSettingsEndBehavior) -> Self {
        Self {
            missing_payment_method: value.missing_payment_method.into(),
        }
    }
}

impl From<crate::stripe_client::UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod>
    for UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn from(
        value: crate::stripe_client::UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod,
    ) -> Self {
        match value {
            crate::stripe_client::UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Cancel => Self::Cancel,
            crate::stripe_client::UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice => {
                Self::CreateInvoice
            }
            crate::stripe_client::UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Pause => Self::Pause,
        }
    }
}

impl From<PriceId> for StripePriceId {
    fn from(value: PriceId) -> Self {
        Self(value.as_str().into())
    }
}

impl TryFrom<StripePriceId> for PriceId {
    type Error = anyhow::Error;

    fn try_from(value: StripePriceId) -> Result<Self, Self::Error> {
        Self::from_str(value.0.as_ref()).context("failed to parse Stripe price ID")
    }
}

impl From<Price> for StripePrice {
    fn from(value: Price) -> Self {
        Self {
            id: value.id.into(),
            unit_amount: value.unit_amount,
            lookup_key: value.lookup_key,
            recurring: value.recurring.map(StripePriceRecurring::from),
        }
    }
}

impl From<Recurring> for StripePriceRecurring {
    fn from(value: Recurring) -> Self {
        Self { meter: value.meter }
    }
}
