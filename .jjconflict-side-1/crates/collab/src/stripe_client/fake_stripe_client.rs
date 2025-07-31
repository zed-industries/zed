use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use collections::HashMap;
use parking_lot::Mutex;
use uuid::Uuid;

use crate::stripe_client::{
    CreateCustomerParams, StripeBillingAddressCollection, StripeCheckoutSession,
    StripeCheckoutSessionMode, StripeCheckoutSessionPaymentMethodCollection, StripeClient,
    StripeCreateCheckoutSessionLineItems, StripeCreateCheckoutSessionParams,
    StripeCreateCheckoutSessionSubscriptionData, StripeCreateMeterEventParams,
    StripeCreateSubscriptionParams, StripeCustomer, StripeCustomerId, StripeCustomerUpdate,
    StripeMeter, StripeMeterId, StripePrice, StripePriceId, StripeSubscription,
    StripeSubscriptionId, StripeSubscriptionItem, StripeSubscriptionItemId, StripeTaxIdCollection,
    UpdateCustomerParams, UpdateSubscriptionParams,
};

#[derive(Debug, Clone)]
pub struct StripeCreateMeterEventCall {
    pub identifier: Arc<str>,
    pub event_name: Arc<str>,
    pub value: u64,
    pub stripe_customer_id: StripeCustomerId,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct StripeCreateCheckoutSessionCall {
    pub customer: Option<StripeCustomerId>,
    pub client_reference_id: Option<String>,
    pub mode: Option<StripeCheckoutSessionMode>,
    pub line_items: Option<Vec<StripeCreateCheckoutSessionLineItems>>,
    pub payment_method_collection: Option<StripeCheckoutSessionPaymentMethodCollection>,
    pub subscription_data: Option<StripeCreateCheckoutSessionSubscriptionData>,
    pub success_url: Option<String>,
    pub billing_address_collection: Option<StripeBillingAddressCollection>,
    pub customer_update: Option<StripeCustomerUpdate>,
    pub tax_id_collection: Option<StripeTaxIdCollection>,
}

pub struct FakeStripeClient {
    pub customers: Arc<Mutex<HashMap<StripeCustomerId, StripeCustomer>>>,
    pub subscriptions: Arc<Mutex<HashMap<StripeSubscriptionId, StripeSubscription>>>,
    pub update_subscription_calls:
        Arc<Mutex<Vec<(StripeSubscriptionId, UpdateSubscriptionParams)>>>,
    pub prices: Arc<Mutex<HashMap<StripePriceId, StripePrice>>>,
    pub meters: Arc<Mutex<HashMap<StripeMeterId, StripeMeter>>>,
    pub create_meter_event_calls: Arc<Mutex<Vec<StripeCreateMeterEventCall>>>,
    pub create_checkout_session_calls: Arc<Mutex<Vec<StripeCreateCheckoutSessionCall>>>,
}

impl FakeStripeClient {
    pub fn new() -> Self {
        Self {
            customers: Arc::new(Mutex::new(HashMap::default())),
            subscriptions: Arc::new(Mutex::new(HashMap::default())),
            update_subscription_calls: Arc::new(Mutex::new(Vec::new())),
            prices: Arc::new(Mutex::new(HashMap::default())),
            meters: Arc::new(Mutex::new(HashMap::default())),
            create_meter_event_calls: Arc::new(Mutex::new(Vec::new())),
            create_checkout_session_calls: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl StripeClient for FakeStripeClient {
    async fn list_customers_by_email(&self, email: &str) -> Result<Vec<StripeCustomer>> {
        Ok(self
            .customers
            .lock()
            .values()
            .filter(|customer| customer.email.as_deref() == Some(email))
            .cloned()
            .collect())
    }

    async fn get_customer(&self, customer_id: &StripeCustomerId) -> Result<StripeCustomer> {
        self.customers
            .lock()
            .get(customer_id)
            .cloned()
            .ok_or_else(|| anyhow!("no customer found for {customer_id:?}"))
    }

    async fn create_customer(&self, params: CreateCustomerParams<'_>) -> Result<StripeCustomer> {
        let customer = StripeCustomer {
            id: StripeCustomerId(format!("cus_{}", Uuid::new_v4()).into()),
            email: params.email.map(|email| email.to_string()),
        };

        self.customers
            .lock()
            .insert(customer.id.clone(), customer.clone());

        Ok(customer)
    }

    async fn update_customer(
        &self,
        customer_id: &StripeCustomerId,
        params: UpdateCustomerParams<'_>,
    ) -> Result<StripeCustomer> {
        let mut customers = self.customers.lock();
        if let Some(customer) = customers.get_mut(customer_id) {
            if let Some(email) = params.email {
                customer.email = Some(email.to_string());
            }
            Ok(customer.clone())
        } else {
            Err(anyhow!("no customer found for {customer_id:?}"))
        }
    }

    async fn list_subscriptions_for_customer(
        &self,
        customer_id: &StripeCustomerId,
    ) -> Result<Vec<StripeSubscription>> {
        let subscriptions = self
            .subscriptions
            .lock()
            .values()
            .filter(|subscription| subscription.customer == *customer_id)
            .cloned()
            .collect();

        Ok(subscriptions)
    }

    async fn get_subscription(
        &self,
        subscription_id: &StripeSubscriptionId,
    ) -> Result<StripeSubscription> {
        self.subscriptions
            .lock()
            .get(subscription_id)
            .cloned()
            .ok_or_else(|| anyhow!("no subscription found for {subscription_id:?}"))
    }

    async fn create_subscription(
        &self,
        params: StripeCreateSubscriptionParams,
    ) -> Result<StripeSubscription> {
        let now = Utc::now();

        let subscription = StripeSubscription {
            id: StripeSubscriptionId(format!("sub_{}", Uuid::new_v4()).into()),
            customer: params.customer,
            status: stripe::SubscriptionStatus::Active,
            current_period_start: now.timestamp(),
            current_period_end: (now + Duration::days(30)).timestamp(),
            items: params
                .items
                .into_iter()
                .map(|item| StripeSubscriptionItem {
                    id: StripeSubscriptionItemId(format!("si_{}", Uuid::new_v4()).into()),
                    price: item
                        .price
                        .and_then(|price_id| self.prices.lock().get(&price_id).cloned()),
                })
                .collect(),
            cancel_at: None,
            cancellation_details: None,
        };

        self.subscriptions
            .lock()
            .insert(subscription.id.clone(), subscription.clone());

        Ok(subscription)
    }

    async fn update_subscription(
        &self,
        subscription_id: &StripeSubscriptionId,
        params: UpdateSubscriptionParams,
    ) -> Result<()> {
        let subscription = self.get_subscription(subscription_id).await?;

        self.update_subscription_calls
            .lock()
            .push((subscription.id, params));

        Ok(())
    }

    async fn cancel_subscription(&self, subscription_id: &StripeSubscriptionId) -> Result<()> {
        // TODO: Implement fake subscription cancellation.
        let _ = subscription_id;

        Ok(())
    }

    async fn list_prices(&self) -> Result<Vec<StripePrice>> {
        let prices = self.prices.lock().values().cloned().collect();

        Ok(prices)
    }

    async fn list_meters(&self) -> Result<Vec<StripeMeter>> {
        let meters = self.meters.lock().values().cloned().collect();

        Ok(meters)
    }

    async fn create_meter_event(&self, params: StripeCreateMeterEventParams<'_>) -> Result<()> {
        self.create_meter_event_calls
            .lock()
            .push(StripeCreateMeterEventCall {
                identifier: params.identifier.into(),
                event_name: params.event_name.into(),
                value: params.payload.value,
                stripe_customer_id: params.payload.stripe_customer_id.clone(),
                timestamp: params.timestamp,
            });

        Ok(())
    }

    async fn create_checkout_session(
        &self,
        params: StripeCreateCheckoutSessionParams<'_>,
    ) -> Result<StripeCheckoutSession> {
        self.create_checkout_session_calls
            .lock()
            .push(StripeCreateCheckoutSessionCall {
                customer: params.customer.cloned(),
                client_reference_id: params.client_reference_id.map(|id| id.to_string()),
                mode: params.mode,
                line_items: params.line_items,
                payment_method_collection: params.payment_method_collection,
                subscription_data: params.subscription_data,
                success_url: params.success_url.map(|url| url.to_string()),
                billing_address_collection: params.billing_address_collection,
                customer_update: params.customer_update,
                tax_id_collection: params.tax_id_collection,
            });

        Ok(StripeCheckoutSession {
            url: Some("https://checkout.stripe.com/c/pay/cs_test_1".to_string()),
        })
    }
}
