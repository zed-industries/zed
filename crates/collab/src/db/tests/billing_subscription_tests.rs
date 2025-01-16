use std::sync::Arc;

use crate::db::billing_subscription::{StripeCancellationReason, StripeSubscriptionStatus};
use crate::db::tests::new_test_user;
use crate::db::{CreateBillingCustomerParams, CreateBillingSubscriptionParams};
use crate::test_both_dbs;

use super::Database;

test_both_dbs!(
    test_get_active_billing_subscriptions,
    test_get_active_billing_subscriptions_postgres,
    test_get_active_billing_subscriptions_sqlite
);

async fn test_get_active_billing_subscriptions(db: &Arc<Database>) {
    // A user with no subscription has no active billing subscriptions.
    {
        let user_id = new_test_user(db, "no-subscription-user@example.com").await;
        let subscription_count = db
            .count_active_billing_subscriptions(user_id)
            .await
            .unwrap();

        assert_eq!(subscription_count, 0);
    }

    // A user with an active subscription has one active billing subscription.
    {
        let user_id = new_test_user(db, "active-user@example.com").await;
        let customer = db
            .create_billing_customer(&CreateBillingCustomerParams {
                user_id,
                stripe_customer_id: "cus_active_user".into(),
            })
            .await
            .unwrap();
        assert_eq!(customer.stripe_customer_id, "cus_active_user".to_string());

        db.create_billing_subscription(&CreateBillingSubscriptionParams {
            billing_customer_id: customer.id,
            stripe_subscription_id: "sub_active_user".into(),
            stripe_subscription_status: StripeSubscriptionStatus::Active,
            stripe_cancellation_reason: None,
        })
        .await
        .unwrap();

        let subscriptions = db.get_billing_subscriptions(user_id).await.unwrap();
        assert_eq!(subscriptions.len(), 1);

        let subscription = &subscriptions[0];
        assert_eq!(
            subscription.stripe_subscription_id,
            "sub_active_user".to_string()
        );
        assert_eq!(
            subscription.stripe_subscription_status,
            StripeSubscriptionStatus::Active
        );
    }

    // A user with a past-due subscription has no active billing subscriptions.
    {
        let user_id = new_test_user(db, "past-due-user@example.com").await;
        let customer = db
            .create_billing_customer(&CreateBillingCustomerParams {
                user_id,
                stripe_customer_id: "cus_past_due_user".into(),
            })
            .await
            .unwrap();
        assert_eq!(customer.stripe_customer_id, "cus_past_due_user".to_string());

        db.create_billing_subscription(&CreateBillingSubscriptionParams {
            billing_customer_id: customer.id,
            stripe_subscription_id: "sub_past_due_user".into(),
            stripe_subscription_status: StripeSubscriptionStatus::PastDue,
            stripe_cancellation_reason: None,
        })
        .await
        .unwrap();

        let subscription_count = db
            .count_active_billing_subscriptions(user_id)
            .await
            .unwrap();
        assert_eq!(subscription_count, 0);
    }
}

test_both_dbs!(
    test_count_overdue_billing_subscriptions,
    test_count_overdue_billing_subscriptions_postgres,
    test_count_overdue_billing_subscriptions_sqlite
);

async fn test_count_overdue_billing_subscriptions(db: &Arc<Database>) {
    // A user with no subscription has no overdue billing subscriptions.
    {
        let user_id = new_test_user(db, "no-subscription-user@example.com").await;
        let subscription_count = db
            .count_overdue_billing_subscriptions(user_id)
            .await
            .unwrap();

        assert_eq!(subscription_count, 0);
    }

    // A user with a past-due subscription has an overdue billing subscription.
    {
        let user_id = new_test_user(db, "past-due-user@example.com").await;
        let customer = db
            .create_billing_customer(&CreateBillingCustomerParams {
                user_id,
                stripe_customer_id: "cus_past_due_user".into(),
            })
            .await
            .unwrap();
        assert_eq!(customer.stripe_customer_id, "cus_past_due_user".to_string());

        db.create_billing_subscription(&CreateBillingSubscriptionParams {
            billing_customer_id: customer.id,
            stripe_subscription_id: "sub_past_due_user".into(),
            stripe_subscription_status: StripeSubscriptionStatus::PastDue,
            stripe_cancellation_reason: None,
        })
        .await
        .unwrap();

        let subscription_count = db
            .count_overdue_billing_subscriptions(user_id)
            .await
            .unwrap();
        assert_eq!(subscription_count, 1);
    }

    // A user with a canceled subscription with a reason of `payment_failed` has an overdue billing subscription.
    {
        let user_id =
            new_test_user(db, "canceled-subscription-payment-failed-user@example.com").await;
        let customer = db
            .create_billing_customer(&CreateBillingCustomerParams {
                user_id,
                stripe_customer_id: "cus_canceled_subscription_payment_failed_user".into(),
            })
            .await
            .unwrap();
        assert_eq!(
            customer.stripe_customer_id,
            "cus_canceled_subscription_payment_failed_user".to_string()
        );

        db.create_billing_subscription(&CreateBillingSubscriptionParams {
            billing_customer_id: customer.id,
            stripe_subscription_id: "sub_canceled_subscription_payment_failed_user".into(),
            stripe_subscription_status: StripeSubscriptionStatus::Canceled,
            stripe_cancellation_reason: Some(StripeCancellationReason::PaymentFailed),
        })
        .await
        .unwrap();

        let subscription_count = db
            .count_overdue_billing_subscriptions(user_id)
            .await
            .unwrap();
        assert_eq!(subscription_count, 1);
    }

    // A user with a canceled subscription with a reason of `cancellation_requested` has no overdue billing subscriptions.
    {
        let user_id = new_test_user(db, "canceled-subscription-user@example.com").await;
        let customer = db
            .create_billing_customer(&CreateBillingCustomerParams {
                user_id,
                stripe_customer_id: "cus_canceled_subscription_user".into(),
            })
            .await
            .unwrap();
        assert_eq!(
            customer.stripe_customer_id,
            "cus_canceled_subscription_user".to_string()
        );

        db.create_billing_subscription(&CreateBillingSubscriptionParams {
            billing_customer_id: customer.id,
            stripe_subscription_id: "sub_canceled_subscription_user".into(),
            stripe_subscription_status: StripeSubscriptionStatus::Canceled,
            stripe_cancellation_reason: Some(StripeCancellationReason::CancellationRequested),
        })
        .await
        .unwrap();

        let subscription_count = db
            .count_overdue_billing_subscriptions(user_id)
            .await
            .unwrap();
        assert_eq!(subscription_count, 0);
    }
}
