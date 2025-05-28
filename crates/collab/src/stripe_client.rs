#[cfg(test)]
mod fake_stripe_client;
mod real_stripe_client;

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

#[cfg(test)]
pub use fake_stripe_client::*;
pub use real_stripe_client::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[async_trait]
pub trait StripeClient: Send + Sync {
    async fn list_customers_by_email(&self, email: &str) -> Result<Vec<StripeCustomer>>;

    async fn create_customer(&self, params: CreateCustomerParams<'_>) -> Result<StripeCustomer>;
}
