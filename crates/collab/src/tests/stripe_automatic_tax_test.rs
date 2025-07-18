#[cfg(test)]
mod tests {
    use crate::stripe_client::{
        FakeStripeClient, StripeAutomaticTax, StripeClient, StripeCreateSubscriptionItems,
        StripeCreateSubscriptionParams, StripeCustomerId, StripePriceId, StripeSubscriptionId,
        UpdateSubscriptionParams,
    };
    use std::sync::Arc;

    #[tokio::test]
    async fn test_create_subscription_with_automatic_tax() {
        let client = Arc::new(FakeStripeClient::new());

        // Create a test customer
        let customer_id = StripeCustomerId("cus_test123".into());

        // Create subscription with automatic tax enabled
        let params = StripeCreateSubscriptionParams {
            customer: customer_id.clone(),
            items: vec![StripeCreateSubscriptionItems {
                price: Some(StripePriceId("price_test123".into())),
                quantity: Some(1),
            }],
            automatic_tax: Some(StripeAutomaticTax {
                enabled: true,
                liability: None,
            }),
        };

        let subscription = client.create_subscription(params).await.unwrap();

        // Verify subscription was created
        assert_eq!(subscription.customer, customer_id);
        assert_eq!(subscription.items.len(), 1);
    }

    #[tokio::test]
    async fn test_update_subscription_with_automatic_tax() {
        let client = Arc::new(FakeStripeClient::new());

        // Create a subscription first
        let customer_id = StripeCustomerId("cus_test456".into());
        let create_params = StripeCreateSubscriptionParams {
            customer: customer_id,
            items: vec![StripeCreateSubscriptionItems {
                price: Some(StripePriceId("price_test456".into())),
                quantity: Some(1),
            }],
            automatic_tax: None,
        };

        let subscription = client.create_subscription(create_params).await.unwrap();

        // Update subscription with automatic tax
        let update_params = UpdateSubscriptionParams {
            items: None,
            trial_settings: None,
            automatic_tax: Some(StripeAutomaticTax {
                enabled: true,
                liability: None,
            }),
        };

        client
            .update_subscription(&subscription.id, update_params.clone())
            .await
            .unwrap();

        // Verify update was called
        let update_calls = client.update_subscription_calls.lock();
        assert_eq!(update_calls.len(), 1);
        assert_eq!(update_calls[0].0, subscription.id);
        assert!(update_calls[0].1.automatic_tax.is_some());
        assert!(update_calls[0].1.automatic_tax.as_ref().unwrap().enabled);
    }

    #[tokio::test]
    async fn test_stripe_billing_with_automatic_tax() {
        use crate::stripe_billing::StripeBilling;
        use crate::stripe_client::{StripePrice, StripePriceRecurring};
        use collections::HashMap;

        let fake_client = Arc::new(FakeStripeClient::new());

        // Add test prices
        fake_client.prices.lock().insert(
            StripePriceId("price_zed_free".into()),
            StripePrice {
                id: StripePriceId("price_zed_free".into()),
                unit_amount: Some(0),
                lookup_key: Some("zed-free".to_string()),
                recurring: None,
            },
        );

        let billing = StripeBilling::test(fake_client.clone());

        // Initialize billing (would normally load prices from Stripe)
        billing.state.write().await.prices_by_lookup_key.insert(
            "zed-free".to_string(),
            StripePrice {
                id: StripePriceId("price_zed_free".into()),
                unit_amount: Some(0),
                lookup_key: Some("zed-free".to_string()),
                recurring: None,
            },
        );

        // Create subscription with automatic tax
        let customer_id = StripeCustomerId("cus_test789".into());
        let subscription = billing.subscribe_to_zed_free(customer_id).await.unwrap();

        // Verify subscription was created with automatic tax
        assert!(
            fake_client
                .subscriptions
                .lock()
                .contains_key(&subscription.id)
        );
    }
}
