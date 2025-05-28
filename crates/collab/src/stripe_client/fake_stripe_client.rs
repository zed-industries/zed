use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use collections::HashMap;
use parking_lot::Mutex;
use uuid::Uuid;

use crate::stripe_client::{CreateCustomerParams, StripeClient, StripeCustomer, StripeCustomerId};

pub struct FakeStripeClient {
    pub customers: Arc<Mutex<HashMap<StripeCustomerId, StripeCustomer>>>,
}

impl FakeStripeClient {
    pub fn new() -> Self {
        Self {
            customers: Arc::new(Mutex::new(HashMap::default())),
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
}
