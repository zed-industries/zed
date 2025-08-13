use std::sync::Arc;

use anyhow::anyhow;
use collections::HashMap;
use stripe::SubscriptionStatus;
use tokio::sync::RwLock;

use crate::Result;
use crate::stripe_client::{
    RealStripeClient, StripeAutomaticTax, StripeClient, StripeCreateSubscriptionItems,
    StripeCreateSubscriptionParams, StripeCustomerId, StripePrice, StripePriceId,
    StripeSubscription,
};

pub struct StripeBilling {
    state: RwLock<StripeBillingState>,
    client: Arc<dyn StripeClient>,
}

#[derive(Default)]
struct StripeBillingState {
    prices_by_lookup_key: HashMap<String, StripePrice>,
}

impl StripeBilling {
    pub fn new(client: Arc<stripe::Client>) -> Self {
        Self {
            client: Arc::new(RealStripeClient::new(client.clone())),
            state: RwLock::default(),
        }
    }

    #[cfg(test)]
    pub fn test(client: Arc<crate::stripe_client::FakeStripeClient>) -> Self {
        Self {
            client,
            state: RwLock::default(),
        }
    }

    pub fn client(&self) -> &Arc<dyn StripeClient> {
        &self.client
    }

    pub async fn initialize(&self) -> Result<()> {
        log::info!("StripeBilling: initializing");

        let mut state = self.state.write().await;

        let prices = self.client.list_prices().await?;

        for price in prices {
            if let Some(lookup_key) = price.lookup_key.clone() {
                state.prices_by_lookup_key.insert(lookup_key, price);
            }
        }

        log::info!("StripeBilling: initialized");

        Ok(())
    }

    pub async fn zed_pro_price_id(&self) -> Result<StripePriceId> {
        self.find_price_id_by_lookup_key("zed-pro").await
    }

    pub async fn zed_free_price_id(&self) -> Result<StripePriceId> {
        self.find_price_id_by_lookup_key("zed-free").await
    }

    pub async fn find_price_id_by_lookup_key(&self, lookup_key: &str) -> Result<StripePriceId> {
        self.state
            .read()
            .await
            .prices_by_lookup_key
            .get(lookup_key)
            .map(|price| price.id.clone())
            .ok_or_else(|| crate::Error::Internal(anyhow!("no price ID found for {lookup_key:?}")))
    }

    pub async fn find_price_by_lookup_key(&self, lookup_key: &str) -> Result<StripePrice> {
        self.state
            .read()
            .await
            .prices_by_lookup_key
            .get(lookup_key)
            .cloned()
            .ok_or_else(|| crate::Error::Internal(anyhow!("no price found for {lookup_key:?}")))
    }

    /// Returns the Stripe customer associated with the provided email address, or creates a new customer, if one does
    /// not already exist.
    ///
    /// Always returns a new Stripe customer if the email address is `None`.
    pub async fn find_or_create_customer_by_email(
        &self,
        email_address: Option<&str>,
    ) -> Result<StripeCustomerId> {
        let existing_customer = if let Some(email) = email_address {
            let customers = self.client.list_customers_by_email(email).await?;

            customers.first().cloned()
        } else {
            None
        };

        let customer_id = if let Some(existing_customer) = existing_customer {
            existing_customer.id
        } else {
            let customer = self
                .client
                .create_customer(crate::stripe_client::CreateCustomerParams {
                    email: email_address,
                })
                .await?;

            customer.id
        };

        Ok(customer_id)
    }

    pub async fn subscribe_to_zed_free(
        &self,
        customer_id: StripeCustomerId,
    ) -> Result<StripeSubscription> {
        let zed_free_price_id = self.zed_free_price_id().await?;

        let existing_subscriptions = self
            .client
            .list_subscriptions_for_customer(&customer_id)
            .await?;

        let existing_active_subscription =
            existing_subscriptions.into_iter().find(|subscription| {
                subscription.status == SubscriptionStatus::Active
                    || subscription.status == SubscriptionStatus::Trialing
            });
        if let Some(subscription) = existing_active_subscription {
            return Ok(subscription);
        }

        let params = StripeCreateSubscriptionParams {
            customer: customer_id,
            items: vec![StripeCreateSubscriptionItems {
                price: Some(zed_free_price_id),
                quantity: Some(1),
            }],
            automatic_tax: Some(StripeAutomaticTax { enabled: true }),
        };

        let subscription = self.client.create_subscription(params).await?;

        Ok(subscription)
    }
}
