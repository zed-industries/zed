use std::sync::Arc;

use chrono::{Duration, Utc};
use pretty_assertions::assert_eq;

use crate::llm::AGENT_EXTENDED_TRIAL_FEATURE_FLAG;
use crate::stripe_billing::StripeBilling;
use crate::stripe_client::{
    FakeStripeClient, StripeBillingAddressCollection, StripeCheckoutSessionMode,
    StripeCheckoutSessionPaymentMethodCollection, StripeCreateCheckoutSessionLineItems,
    StripeCreateCheckoutSessionSubscriptionData, StripeCustomerId, StripeMeter, StripeMeterId,
    StripePrice, StripePriceId, StripePriceRecurring, StripeSubscription, StripeSubscriptionId,
    StripeSubscriptionItem, StripeSubscriptionItemId, StripeSubscriptionTrialSettings,
    StripeSubscriptionTrialSettingsEndBehavior,
    StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod, UpdateSubscriptionItems,
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

    let now = Utc::now();
    let subscription = StripeSubscription {
        id: StripeSubscriptionId("sub_test".into()),
        customer: StripeCustomerId("cus_test".into()),
        status: stripe::SubscriptionStatus::Active,
        current_period_start: now.timestamp(),
        current_period_end: (now + Duration::days(30)).timestamp(),
        items: vec![],
        cancel_at: None,
        cancellation_details: None,
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
        let now = Utc::now();
        let subscription = StripeSubscription {
            id: StripeSubscriptionId("sub_test".into()),
            customer: StripeCustomerId("cus_test".into()),
            status: stripe::SubscriptionStatus::Active,
            current_period_start: now.timestamp(),
            current_period_end: (now + Duration::days(30)).timestamp(),
            items: vec![StripeSubscriptionItem {
                id: StripeSubscriptionItemId("si_test".into()),
                price: Some(price.clone()),
            }],
            cancel_at: None,
            cancellation_details: None,
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

#[gpui::test]
async fn test_subscribe_to_zed_free() {
    let (stripe_billing, stripe_client) = make_stripe_billing();

    let zed_pro_price = StripePrice {
        id: StripePriceId("price_1".into()),
        unit_amount: Some(0),
        lookup_key: Some("zed-pro".to_string()),
        recurring: None,
    };
    stripe_client
        .prices
        .lock()
        .insert(zed_pro_price.id.clone(), zed_pro_price.clone());
    let zed_free_price = StripePrice {
        id: StripePriceId("price_2".into()),
        unit_amount: Some(0),
        lookup_key: Some("zed-free".to_string()),
        recurring: None,
    };
    stripe_client
        .prices
        .lock()
        .insert(zed_free_price.id.clone(), zed_free_price.clone());

    stripe_billing.initialize().await.unwrap();

    // Customer is subscribed to Zed Free when not already subscribed to a plan.
    {
        let customer_id = StripeCustomerId("cus_no_plan".into());

        let subscription = stripe_billing
            .subscribe_to_zed_free(customer_id)
            .await
            .unwrap();

        assert_eq!(subscription.items[0].price.as_ref(), Some(&zed_free_price));
    }

    // Customer is not subscribed to Zed Free when they already have an active subscription.
    {
        let customer_id = StripeCustomerId("cus_active_subscription".into());

        let now = Utc::now();
        let existing_subscription = StripeSubscription {
            id: StripeSubscriptionId("sub_existing_active".into()),
            customer: customer_id.clone(),
            status: stripe::SubscriptionStatus::Active,
            current_period_start: now.timestamp(),
            current_period_end: (now + Duration::days(30)).timestamp(),
            items: vec![StripeSubscriptionItem {
                id: StripeSubscriptionItemId("si_test".into()),
                price: Some(zed_pro_price.clone()),
            }],
            cancel_at: None,
            cancellation_details: None,
        };
        stripe_client.subscriptions.lock().insert(
            existing_subscription.id.clone(),
            existing_subscription.clone(),
        );

        let subscription = stripe_billing
            .subscribe_to_zed_free(customer_id)
            .await
            .unwrap();

        assert_eq!(subscription, existing_subscription);
    }

    // Customer is not subscribed to Zed Free when they already have a trial subscription.
    {
        let customer_id = StripeCustomerId("cus_trial_subscription".into());

        let now = Utc::now();
        let existing_subscription = StripeSubscription {
            id: StripeSubscriptionId("sub_existing_trial".into()),
            customer: customer_id.clone(),
            status: stripe::SubscriptionStatus::Trialing,
            current_period_start: now.timestamp(),
            current_period_end: (now + Duration::days(14)).timestamp(),
            items: vec![StripeSubscriptionItem {
                id: StripeSubscriptionItemId("si_test".into()),
                price: Some(zed_pro_price.clone()),
            }],
            cancel_at: None,
            cancellation_details: None,
        };
        stripe_client.subscriptions.lock().insert(
            existing_subscription.id.clone(),
            existing_subscription.clone(),
        );

        let subscription = stripe_billing
            .subscribe_to_zed_free(customer_id)
            .await
            .unwrap();

        assert_eq!(subscription, existing_subscription);
    }
}

#[gpui::test]
async fn test_bill_model_request_usage() {
    let (stripe_billing, stripe_client) = make_stripe_billing();

    let customer_id = StripeCustomerId("cus_test".into());

    stripe_billing
        .bill_model_request_usage(&customer_id, "some_model/requests", 73)
        .await
        .unwrap();

    let create_meter_event_calls = stripe_client
        .create_meter_event_calls
        .lock()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(create_meter_event_calls.len(), 1);
    assert!(
        create_meter_event_calls[0]
            .identifier
            .starts_with("model_requests/")
    );
    assert_eq!(create_meter_event_calls[0].stripe_customer_id, customer_id);
    assert_eq!(
        create_meter_event_calls[0].event_name.as_ref(),
        "some_model/requests"
    );
    assert_eq!(create_meter_event_calls[0].value, 73);
}

#[gpui::test]
async fn test_checkout_with_zed_pro() {
    let (stripe_billing, stripe_client) = make_stripe_billing();

    let customer_id = StripeCustomerId("cus_test".into());
    let github_login = "zeduser1";
    let success_url = "https://example.com/success";

    // It returns an error when the Zed Pro price doesn't exist.
    {
        let result = stripe_billing
            .checkout_with_zed_pro(&customer_id, github_login, success_url)
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            r#"no price ID found for "zed-pro""#
        );
    }

    // Successful checkout.
    {
        let price = StripePrice {
            id: StripePriceId("price_1".into()),
            unit_amount: Some(2000),
            lookup_key: Some("zed-pro".to_string()),
            recurring: None,
        };
        stripe_client
            .prices
            .lock()
            .insert(price.id.clone(), price.clone());

        stripe_billing.initialize().await.unwrap();

        let checkout_url = stripe_billing
            .checkout_with_zed_pro(&customer_id, github_login, success_url)
            .await
            .unwrap();

        assert!(checkout_url.starts_with("https://checkout.stripe.com/c/pay"));

        let create_checkout_session_calls = stripe_client
            .create_checkout_session_calls
            .lock()
            .drain(..)
            .collect::<Vec<_>>();
        assert_eq!(create_checkout_session_calls.len(), 1);
        let call = create_checkout_session_calls.into_iter().next().unwrap();
        assert_eq!(call.customer, Some(customer_id));
        assert_eq!(call.client_reference_id.as_deref(), Some(github_login));
        assert_eq!(call.mode, Some(StripeCheckoutSessionMode::Subscription));
        assert_eq!(
            call.line_items,
            Some(vec![StripeCreateCheckoutSessionLineItems {
                price: Some(price.id.to_string()),
                quantity: Some(1)
            }])
        );
        assert_eq!(call.payment_method_collection, None);
        assert_eq!(call.subscription_data, None);
        assert_eq!(call.success_url.as_deref(), Some(success_url));
        assert_eq!(
            call.billing_address_collection,
            Some(StripeBillingAddressCollection::Required)
        );
    }
}

#[gpui::test]
async fn test_checkout_with_zed_pro_trial() {
    let (stripe_billing, stripe_client) = make_stripe_billing();

    let customer_id = StripeCustomerId("cus_test".into());
    let github_login = "zeduser1";
    let success_url = "https://example.com/success";

    // It returns an error when the Zed Pro price doesn't exist.
    {
        let result = stripe_billing
            .checkout_with_zed_pro_trial(&customer_id, github_login, Vec::new(), success_url)
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            r#"no price ID found for "zed-pro""#
        );
    }

    let price = StripePrice {
        id: StripePriceId("price_1".into()),
        unit_amount: Some(2000),
        lookup_key: Some("zed-pro".to_string()),
        recurring: None,
    };
    stripe_client
        .prices
        .lock()
        .insert(price.id.clone(), price.clone());

    stripe_billing.initialize().await.unwrap();

    // Successful checkout.
    {
        let checkout_url = stripe_billing
            .checkout_with_zed_pro_trial(&customer_id, github_login, Vec::new(), success_url)
            .await
            .unwrap();

        assert!(checkout_url.starts_with("https://checkout.stripe.com/c/pay"));

        let create_checkout_session_calls = stripe_client
            .create_checkout_session_calls
            .lock()
            .drain(..)
            .collect::<Vec<_>>();
        assert_eq!(create_checkout_session_calls.len(), 1);
        let call = create_checkout_session_calls.into_iter().next().unwrap();
        assert_eq!(call.customer.as_ref(), Some(&customer_id));
        assert_eq!(call.client_reference_id.as_deref(), Some(github_login));
        assert_eq!(call.mode, Some(StripeCheckoutSessionMode::Subscription));
        assert_eq!(
            call.line_items,
            Some(vec![StripeCreateCheckoutSessionLineItems {
                price: Some(price.id.to_string()),
                quantity: Some(1)
            }])
        );
        assert_eq!(
            call.payment_method_collection,
            Some(StripeCheckoutSessionPaymentMethodCollection::IfRequired)
        );
        assert_eq!(
            call.subscription_data,
            Some(StripeCreateCheckoutSessionSubscriptionData {
                trial_period_days: Some(14),
                trial_settings: Some(StripeSubscriptionTrialSettings {
                    end_behavior: StripeSubscriptionTrialSettingsEndBehavior {
                        missing_payment_method:
                            StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Cancel,
                    },
                }),
                metadata: None,
            })
        );
        assert_eq!(call.success_url.as_deref(), Some(success_url));
        assert_eq!(
            call.billing_address_collection,
            Some(StripeBillingAddressCollection::Required)
        );
    }

    // Successful checkout with extended trial.
    {
        let checkout_url = stripe_billing
            .checkout_with_zed_pro_trial(
                &customer_id,
                github_login,
                vec![AGENT_EXTENDED_TRIAL_FEATURE_FLAG.to_string()],
                success_url,
            )
            .await
            .unwrap();

        assert!(checkout_url.starts_with("https://checkout.stripe.com/c/pay"));

        let create_checkout_session_calls = stripe_client
            .create_checkout_session_calls
            .lock()
            .drain(..)
            .collect::<Vec<_>>();
        assert_eq!(create_checkout_session_calls.len(), 1);
        let call = create_checkout_session_calls.into_iter().next().unwrap();
        assert_eq!(call.customer, Some(customer_id));
        assert_eq!(call.client_reference_id.as_deref(), Some(github_login));
        assert_eq!(call.mode, Some(StripeCheckoutSessionMode::Subscription));
        assert_eq!(
            call.line_items,
            Some(vec![StripeCreateCheckoutSessionLineItems {
                price: Some(price.id.to_string()),
                quantity: Some(1)
            }])
        );
        assert_eq!(
            call.payment_method_collection,
            Some(StripeCheckoutSessionPaymentMethodCollection::IfRequired)
        );
        assert_eq!(
            call.subscription_data,
            Some(StripeCreateCheckoutSessionSubscriptionData {
                trial_period_days: Some(60),
                trial_settings: Some(StripeSubscriptionTrialSettings {
                    end_behavior: StripeSubscriptionTrialSettingsEndBehavior {
                        missing_payment_method:
                            StripeSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Cancel,
                    },
                }),
                metadata: Some(std::collections::HashMap::from_iter([(
                    "promo_feature_flag".into(),
                    AGENT_EXTENDED_TRIAL_FEATURE_FLAG.into()
                )])),
            })
        );
        assert_eq!(call.success_url.as_deref(), Some(success_url));
        assert_eq!(
            call.billing_address_collection,
            Some(StripeBillingAddressCollection::Required)
        );
    }
}
