use chrono::{Duration, Utc};
use pretty_assertions::assert_eq;

use crate::db::billing_subscription::{StripeSubscriptionStatus, SubscriptionKind};
use crate::db::{UserId, billing_subscription};
use crate::llm::db::LlmDatabase;
use crate::test_llm_db;

test_llm_db!(
    test_transfer_existing_subscription_usage,
    test_transfer_existing_subscription_usage_postgres
);

async fn test_transfer_existing_subscription_usage(db: &mut LlmDatabase) {
    // Test when an existing Zed Pro trial subscription is upgraded to Zed Pro.
    {
        let user_id = UserId(1);

        let now = Utc::now();

        let trial_period_start_at = now - Duration::days(14);
        let trial_period_end_at = now;

        let new_period_start_at = now;
        let new_period_end_at = now + Duration::days(30);

        let existing_subscription = billing_subscription::Model {
            kind: Some(SubscriptionKind::ZedProTrial),
            stripe_current_period_start: Some(trial_period_start_at.timestamp()),
            stripe_current_period_end: Some(trial_period_end_at.timestamp()),
            ..Default::default()
        };

        let existing_usage = db
            .create_subscription_usage(
                user_id,
                trial_period_start_at,
                trial_period_end_at,
                SubscriptionKind::ZedProTrial,
                25,
                1_000,
            )
            .await
            .unwrap();

        let transferred_usage = db
            .transfer_existing_subscription_usage(
                user_id,
                &existing_subscription,
                Some(SubscriptionKind::ZedPro),
                StripeSubscriptionStatus::Active,
                new_period_start_at,
                new_period_end_at,
            )
            .await
            .unwrap();

        assert!(
            transferred_usage.is_some(),
            "subscription usage not transferred successfully"
        );
        let transferred_usage = transferred_usage.unwrap();

        assert_eq!(
            transferred_usage.model_requests,
            existing_usage.model_requests
        );
        assert_eq!(
            transferred_usage.edit_predictions,
            existing_usage.edit_predictions
        );
    }

    // Test when an existing Zed Pro trial subscription is canceled.
    {
        let user_id = UserId(2);

        let now = Utc::now();

        let trial_period_start_at = now - Duration::days(14);
        let trial_period_end_at = now;

        let existing_subscription = billing_subscription::Model {
            kind: Some(SubscriptionKind::ZedProTrial),
            stripe_current_period_start: Some(trial_period_start_at.timestamp()),
            stripe_current_period_end: Some(trial_period_end_at.timestamp()),
            ..Default::default()
        };

        let _existing_usage = db
            .create_subscription_usage(
                user_id,
                trial_period_start_at,
                trial_period_end_at,
                SubscriptionKind::ZedProTrial,
                25,
                1_000,
            )
            .await
            .unwrap();

        let transferred_usage = db
            .transfer_existing_subscription_usage(
                user_id,
                &existing_subscription,
                Some(SubscriptionKind::ZedPro),
                StripeSubscriptionStatus::Canceled,
                trial_period_start_at,
                trial_period_end_at,
            )
            .await
            .unwrap();

        assert!(
            transferred_usage.is_none(),
            "subscription usage was transferred when it should not have been"
        );
    }
}
