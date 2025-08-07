use std::sync::Arc;

use pretty_assertions::assert_eq;

use crate::stripe_billing::StripeBilling;
use crate::stripe_client::{FakeStripeClient, StripePrice, StripePriceId, StripePriceRecurring};

fn make_stripe_billing() -> (StripeBilling, Arc<FakeStripeClient>) {
    let stripe_client = Arc::new(FakeStripeClient::new());
    let stripe_billing = StripeBilling::test(stripe_client.clone());

    (stripe_billing, stripe_client)
}

#[gpui::test]
async fn test_initialize() {
    let (stripe_billing, stripe_client) = make_stripe_billing();

    // Add test prices
    let price1 = StripePrice {
        id: StripePriceId("price_1".into()),
        unit_amount: Some(1_000),
        lookup_key: Some("zed-pro".to_string()),
        recurring: None,
    };
    let price2 = StripePrice {
        id: StripePriceId("price_2".into()),
        unit_amount: Some(0),
        lookup_key: Some("zed-free".to_string()),
        recurring: None,
    };
    let price3 = StripePrice {
        id: StripePriceId("price_3".into()),
        unit_amount: Some(500),
        lookup_key: None,
        recurring: Some(StripePriceRecurring {
            meter: Some("meter_1".to_string()),
        }),
    };
    stripe_client
        .prices
        .lock()
        .insert(price1.id.clone(), price1);
    stripe_client
        .prices
        .lock()
        .insert(price2.id.clone(), price2);
    stripe_client
        .prices
        .lock()
        .insert(price3.id.clone(), price3);

    // Initialize the billing system
    stripe_billing.initialize().await.unwrap();

    // Verify that prices can be found by lookup key
    let zed_pro_price_id = stripe_billing.zed_pro_price_id().await.unwrap();
    assert_eq!(zed_pro_price_id.to_string(), "price_1");

    let zed_free_price_id = stripe_billing.zed_free_price_id().await.unwrap();
    assert_eq!(zed_free_price_id.to_string(), "price_2");

    // Verify that a price can be found by lookup key
    let zed_pro_price = stripe_billing
        .find_price_by_lookup_key("zed-pro")
        .await
        .unwrap();
    assert_eq!(zed_pro_price.id.to_string(), "price_1");
    assert_eq!(zed_pro_price.unit_amount, Some(1_000));

    // Verify that finding a non-existent lookup key returns an error
    let result = stripe_billing
        .find_price_by_lookup_key("non-existent")
        .await;
    assert!(result.is_err());
}

#[gpui::test]
async fn test_find_or_create_customer_by_email() {
    let (stripe_billing, stripe_client) = make_stripe_billing();

    // Create a customer with an email that doesn't yet correspond to a customer.
    {
        let email = "user@example.com";

        let customer_id = stripe_billing
            .find_or_create_customer_by_email(Some(email))
            .await
            .unwrap();

        let customer = stripe_client
            .customers
            .lock()
            .get(&customer_id)
            .unwrap()
            .clone();
        assert_eq!(customer.email.as_deref(), Some(email));
    }

    // Create a customer with an email that corresponds to an existing customer.
    {
        let email = "user2@example.com";

        let existing_customer_id = stripe_billing
            .find_or_create_customer_by_email(Some(email))
            .await
            .unwrap();

        let customer_id = stripe_billing
            .find_or_create_customer_by_email(Some(email))
            .await
            .unwrap();
        assert_eq!(customer_id, existing_customer_id);

        let customer = stripe_client
            .customers
            .lock()
            .get(&customer_id)
            .unwrap()
            .clone();
        assert_eq!(customer.email.as_deref(), Some(email));
    }
}
