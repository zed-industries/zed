use std::sync::Arc;

use crate::test_both_dbs;

use super::{CreateProcessedStripeEventParams, Database};

test_both_dbs!(
    test_already_processed_stripe_event,
    test_already_processed_stripe_event_postgres,
    test_already_processed_stripe_event_sqlite
);

async fn test_already_processed_stripe_event(db: &Arc<Database>) {
    let unprocessed_event_id = "evt_1PiJOuRxOf7d5PNaw2zzWiyO".to_string();
    let processed_event_id = "evt_1PiIfMRxOf7d5PNakHrAUe8P".to_string();

    db.create_processed_stripe_event(&CreateProcessedStripeEventParams {
        stripe_event_id: processed_event_id.clone(),
        stripe_event_type: "customer.created".into(),
        stripe_event_created_timestamp: 1722355968,
    })
    .await
    .unwrap();

    assert_eq!(
        db.already_processed_stripe_event(&processed_event_id)
            .await
            .unwrap(),
        true,
        "Expected {processed_event_id} to already be processed"
    );

    assert_eq!(
        db.already_processed_stripe_event(&unprocessed_event_id)
            .await
            .unwrap(),
        false,
        "Expected {unprocessed_event_id} to be unprocessed"
    );
}
