ALTER TABLE billing_customers ADD COLUMN last_stripe_event_id TEXT;
ALTER TABLE billing_subscriptions ADD COLUMN last_stripe_event_id TEXT;
