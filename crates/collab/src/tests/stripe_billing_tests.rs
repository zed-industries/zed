use std::sync::Arc;

use pretty_assertions::assert_eq;

use crate::stripe_billing::StripeBilling;
use crate::stripe_client::{
    FakeStripeClient, StripeMeter, StripeMeterId, StripePrice, StripePriceId, StripePriceRecurring,
    StripeSubscription, StripeSubscriptionId, StripeSubscriptionItem, StripeSubscriptionItemId,
    UpdateSubscriptionItems,
};

fn make_stripe_billing() -> (StripeBilling, Arc<FakeStripeClient>) {
    let stripe_client = Arc::new(FakeStripeClient::new());
    let stripe_billing = StripeBilling::test(stripe_client.clone());

    (stripe_billing, stripe_client)
}

#[gpui::test]
async fn test_initialize() {
    let (stripe_billing, stripe_client) = make_stripe_billing();

    // Add test meters
    let meter1 = StripeMeter {
        id: StripeMeterId("meter_1".into()),
        event_name: "event_1".to_string(),
    };
    let meter2 = StripeMeter {
        id: StripeMeterId("meter_2".into()),
        event_name: "event_2".to_string(),
    };
    stripe_client
        .meters
        .lock()
        .insert(meter1.id.clone(), meter1);
    stripe_client
        .meters
        .lock()
        .insert(meter2.id.clone(), meter2);

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

#[gpui::test]
async fn test_subscribe_to_price() {
    let (stripe_billing, stripe_client) = make_stripe_billing();

    let price = StripePrice {
        id: StripePriceId("price_test".into()),
        unit_amount: Some(2000),
        lookup_key: Some("test-price".to_string()),
        recurring: None,
    };
    stripe_client
        .prices
        .lock()
        .insert(price.id.clone(), price.clone());

    let subscription = StripeSubscription {
        id: StripeSubscriptionId("sub_test".into()),
        items: vec![],
    };
    stripe_client
        .subscriptions
        .lock()
        .insert(subscription.id.clone(), subscription.clone());

    stripe_billing
        .subscribe_to_price(&subscription.id, &price)
        .await
        .unwrap();

    let update_subscription_calls = stripe_client
        .update_subscription_calls
        .lock()
        .iter()
        .map(|(id, params)| (id.clone(), params.clone()))
        .collect::<Vec<_>>();
    assert_eq!(update_subscription_calls.len(), 1);
    assert_eq!(update_subscription_calls[0].0, subscription.id);
    assert_eq!(
        update_subscription_calls[0].1.items,
        Some(vec![UpdateSubscriptionItems {
            price: Some(price.id.clone())
        }])
    );

    // Subscribing to a price that is already on the subscription is a no-op.
    {
        let subscription = StripeSubscription {
            id: StripeSubscriptionId("sub_test".into()),
            items: vec![StripeSubscriptionItem {
                id: StripeSubscriptionItemId("si_test".into()),
                price: Some(price.clone()),
            }],
        };
        stripe_client
            .subscriptions
            .lock()
            .insert(subscription.id.clone(), subscription.clone());

        stripe_billing
            .subscribe_to_price(&subscription.id, &price)
            .await
            .unwrap();

        assert_eq!(stripe_client.update_subscription_calls.lock().len(), 1);
    }
}
