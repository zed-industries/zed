use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use stripe::{CreateCustomer, Customer, CustomerId, ListCustomers};

use crate::stripe_client::{CreateCustomerParams, StripeClient, StripeCustomer, StripeCustomerId};

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
