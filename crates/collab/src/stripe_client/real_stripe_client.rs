use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use serde::Serialize;
use stripe::{
    CancellationDetails, CancellationDetailsReason, CheckoutSession, CheckoutSessionMode,
    CheckoutSessionPaymentMethodCollection, CreateCheckoutSession, CreateCheckoutSessionLineItems,
    CreateCheckoutSessionSubscriptionData, CreateCheckoutSessionSubscriptionDataTrialSettings,
    CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior,
    CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
    CreateCustomer, Customer, CustomerId, ListCustomers, Price, PriceId, Recurring, Subscription,
    SubscriptionId, SubscriptionItem, SubscriptionItemId, UpdateSubscriptionItems,
    UpdateSubscriptionTrialSettings, UpdateSubscriptionTrialSettingsEndBehavior,
    UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod,
};

use crate::stripe_client::{
    CreateCustomerParams, StripeCancellationDetails, StripeCancellationDetailsReason,
    StripeCheckoutSession, StripeCheckoutSessionMode, StripeCheckoutSessionPaymentMethodCollection,
    StripeClient, StripeCreateCheckoutSessionLineItems, StripeCreateCheckoutSessionParams,
    StripeCreateCheckoutSessionSubscriptionData, StripeCreateMeterEventParams,
    StripeCreateSubscriptionParams, StripeCustomer, StripeCustomerId, StripeMeter, StripePrice,
    StripePriceId, StripePriceRecurring, StripeSubscription, StripeSubscriptionId,
    StripeSubscriptionItem, StripeSubscriptionItemId, StripeSubscriptionTrialSettings,
    StripeSubscriptionTrialSettingsEndBehavior,
    StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod, UpdateSubscriptionParams,
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

    async fn get_customer(&self, customer_id: &StripeCustomerId) -> Result<StripeCustomer> {
        let customer_id = customer_id.try_into()?;

        let customer = Customer::retrieve(&self.client, &customer_id, &[]).await?;

        Ok(StripeCustomer::from(customer))
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

    async fn list_subscriptions_for_customer(
        &self,
        customer_id: &StripeCustomerId,
    ) -> Result<Vec<StripeSubscription>> {
        let customer_id = customer_id.try_into()?;

        let subscriptions = stripe::Subscription::list(
            &self.client,
            &stripe::ListSubscriptions {
                customer: Some(customer_id),
                status: None,
                ..Default::default()
            },
        )
        .await?;

        Ok(subscriptions
            .data
            .into_iter()
            .map(StripeSubscription::from)
            .collect())
    }

    async fn get_subscription(
        &self,
        subscription_id: &StripeSubscriptionId,
    ) -> Result<StripeSubscription> {
        let subscription_id = subscription_id.try_into()?;

        let subscription = Subscription::retrieve(&self.client, &subscription_id, &[]).await?;

        Ok(StripeSubscription::from(subscription))
    }

    async fn create_subscription(
        &self,
        params: StripeCreateSubscriptionParams,
    ) -> Result<StripeSubscription> {
        let customer_id = params.customer.try_into()?;

        let mut create_subscription = stripe::CreateSubscription::new(customer_id);
        create_subscription.items = Some(
            params
                .items
                .into_iter()
                .map(|item| stripe::CreateSubscriptionItems {
                    price: item.price.map(|price| price.to_string()),
                    quantity: item.quantity,
                    ..Default::default()
                })
                .collect(),
        );

        let subscription = Subscription::create(&self.client, create_subscription).await?;

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

    async fn cancel_subscription(&self, subscription_id: &StripeSubscriptionId) -> Result<()> {
        let subscription_id = subscription_id.try_into()?;

        Subscription::cancel(
            &self.client,
            &subscription_id,
            stripe::CancelSubscription {
                invoice_now: None,
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

    async fn create_checkout_session(
        &self,
        params: StripeCreateCheckoutSessionParams<'_>,
    ) -> Result<StripeCheckoutSession> {
        let params = params.try_into()?;
        let session = CheckoutSession::create(&self.client, params).await?;

        Ok(session.into())
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

impl TryFrom<&StripeCustomerId> for CustomerId {
    type Error = anyhow::Error;

    fn try_from(value: &StripeCustomerId) -> Result<Self, Self::Error> {
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
            customer: value.customer.id().into(),
            status: value.status,
            current_period_start: value.current_period_start,
            current_period_end: value.current_period_end,
            items: value.items.data.into_iter().map(Into::into).collect(),
            cancel_at: value.cancel_at,
            cancellation_details: value.cancellation_details.map(Into::into),
        }
    }
}

impl From<CancellationDetails> for StripeCancellationDetails {
    fn from(value: CancellationDetails) -> Self {
        Self {
            reason: value.reason.map(Into::into),
        }
    }
}

impl From<CancellationDetailsReason> for StripeCancellationDetailsReason {
    fn from(value: CancellationDetailsReason) -> Self {
        match value {
            CancellationDetailsReason::CancellationRequested => Self::CancellationRequested,
            CancellationDetailsReason::PaymentDisputed => Self::PaymentDisputed,
            CancellationDetailsReason::PaymentFailed => Self::PaymentFailed,
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

impl From<StripeSubscriptionTrialSettings> for UpdateSubscriptionTrialSettings {
    fn from(value: StripeSubscriptionTrialSettings) -> Self {
        Self {
            end_behavior: value.end_behavior.into(),
        }
    }
}

impl From<StripeSubscriptionTrialSettingsEndBehavior>
    for UpdateSubscriptionTrialSettingsEndBehavior
{
    fn from(value: StripeSubscriptionTrialSettingsEndBehavior) -> Self {
        Self {
            missing_payment_method: value.missing_payment_method.into(),
        }
    }
}

impl From<StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod>
    for UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn from(value: StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod) -> Self {
        match value {
            StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Cancel => Self::Cancel,
            StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice => {
                Self::CreateInvoice
            }
            StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Pause => Self::Pause,
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

impl<'a> TryFrom<StripeCreateCheckoutSessionParams<'a>> for CreateCheckoutSession<'a> {
    type Error = anyhow::Error;

    fn try_from(value: StripeCreateCheckoutSessionParams<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            customer: value
                .customer
                .map(|customer_id| customer_id.try_into())
                .transpose()?,
            client_reference_id: value.client_reference_id,
            mode: value.mode.map(Into::into),
            line_items: value
                .line_items
                .map(|line_items| line_items.into_iter().map(Into::into).collect()),
            payment_method_collection: value.payment_method_collection.map(Into::into),
            subscription_data: value.subscription_data.map(Into::into),
            success_url: value.success_url,
            ..Default::default()
        })
    }
}

impl From<StripeCheckoutSessionMode> for CheckoutSessionMode {
    fn from(value: StripeCheckoutSessionMode) -> Self {
        match value {
            StripeCheckoutSessionMode::Payment => Self::Payment,
            StripeCheckoutSessionMode::Setup => Self::Setup,
            StripeCheckoutSessionMode::Subscription => Self::Subscription,
        }
    }
}

impl From<StripeCreateCheckoutSessionLineItems> for CreateCheckoutSessionLineItems {
    fn from(value: StripeCreateCheckoutSessionLineItems) -> Self {
        Self {
            price: value.price,
            quantity: value.quantity,
            ..Default::default()
        }
    }
}

impl From<StripeCheckoutSessionPaymentMethodCollection> for CheckoutSessionPaymentMethodCollection {
    fn from(value: StripeCheckoutSessionPaymentMethodCollection) -> Self {
        match value {
            StripeCheckoutSessionPaymentMethodCollection::Always => Self::Always,
            StripeCheckoutSessionPaymentMethodCollection::IfRequired => Self::IfRequired,
        }
    }
}

impl From<StripeCreateCheckoutSessionSubscriptionData> for CreateCheckoutSessionSubscriptionData {
    fn from(value: StripeCreateCheckoutSessionSubscriptionData) -> Self {
        Self {
            trial_period_days: value.trial_period_days,
            trial_settings: value.trial_settings.map(Into::into),
            metadata: value.metadata,
            ..Default::default()
        }
    }
}

impl From<StripeSubscriptionTrialSettings> for CreateCheckoutSessionSubscriptionDataTrialSettings {
    fn from(value: StripeSubscriptionTrialSettings) -> Self {
        Self {
            end_behavior: value.end_behavior.into(),
        }
    }
}

impl From<StripeSubscriptionTrialSettingsEndBehavior>
    for CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior
{
    fn from(value: StripeSubscriptionTrialSettingsEndBehavior) -> Self {
        Self {
            missing_payment_method: value.missing_payment_method.into(),
        }
    }
}

impl From<StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod>
    for CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn from(value: StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod) -> Self {
        match value {
            StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Cancel => Self::Cancel,
            StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice => {
                Self::CreateInvoice
            }
            StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Pause => Self::Pause,
        }
    }
}

impl From<CheckoutSession> for StripeCheckoutSession {
    fn from(value: CheckoutSession) -> Self {
        Self { url: value.url }
    }
}
