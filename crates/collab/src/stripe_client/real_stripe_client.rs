use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use serde::Serialize;
use stripe::{CreateCustomer, Customer, CustomerId, ListCustomers, Price, PriceId, Recurring};

use crate::stripe_client::{
    CreateCustomerParams, StripeClient, StripeCustomer, StripeCustomerId, StripeMeter, StripePrice,
    StripePriceId, StripePriceRecurring,
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
