use std::sync::Arc;

use pretty_assertions::assert_eq;

use crate::stripe_billing::StripeBilling;
use crate::stripe_client::FakeStripeClient;

fn make_stripe_billing() -> (StripeBilling, Arc<FakeStripeClient>) {
    let stripe_client = Arc::new(FakeStripeClient::new());
    let stripe_billing = StripeBilling::test(stripe_client.clone());

    (stripe_billing, stripe_client)
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
