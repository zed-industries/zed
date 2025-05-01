use std::sync::Arc;

use crate::db::billing_subscription::StripeSubscriptionStatus;
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
            kind: None,
            stripe_subscription_id: "sub_active_user".into(),
            stripe_subscription_status: StripeSubscriptionStatus::Active,
            stripe_cancellation_reason: None,
            stripe_current_period_start: None,
            stripe_current_period_end: None,
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
            kind: None,
            stripe_subscription_id: "sub_past_due_user".into(),
            stripe_subscription_status: StripeSubscriptionStatus::PastDue,
            stripe_cancellation_reason: None,
            stripe_current_period_start: None,
            stripe_current_period_end: None,
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
