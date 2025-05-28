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

    async fn list_prices(&self) -> Result<Vec<StripePrice>>;

    async fn list_meters(&self) -> Result<Vec<StripeMeter>>;
}
