use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use parking_lot::Mutex;
use uuid::Uuid;

use crate::stripe_client::{
    CreateCustomerParams, StripeClient, StripeCreateMeterEventParams, StripeCustomer,
    StripeCustomerId, StripeMeter, StripeMeterId, StripePrice, StripePriceId, StripeSubscription,
    StripeSubscriptionId, UpdateSubscriptionParams,
};

#[derive(Debug, Clone)]
pub struct StripeCreateMeterEventCall {
    pub identifier: Arc<str>,
    pub event_name: Arc<str>,
    pub value: u64,
    pub stripe_customer_id: StripeCustomerId,
    pub timestamp: Option<i64>,
}

pub struct FakeStripeClient {
    pub customers: Arc<Mutex<HashMap<StripeCustomerId, StripeCustomer>>>,
    pub subscriptions: Arc<Mutex<HashMap<StripeSubscriptionId, StripeSubscription>>>,
    pub update_subscription_calls:
        Arc<Mutex<Vec<(StripeSubscriptionId, UpdateSubscriptionParams)>>>,
    pub prices: Arc<Mutex<HashMap<StripePriceId, StripePrice>>>,
    pub meters: Arc<Mutex<HashMap<StripeMeterId, StripeMeter>>>,
    pub create_meter_event_calls: Arc<Mutex<Vec<StripeCreateMeterEventCall>>>,
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
}
