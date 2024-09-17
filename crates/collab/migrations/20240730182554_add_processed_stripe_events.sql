ALTER TABLE billing_customers DROP COLUMN last_stripe_event_id;
ALTER TABLE billing_subscriptions DROP COLUMN last_stripe_event_id;

CREATE TABLE IF NOT EXISTS processed_stripe_events (
    stripe_event_id TEXT PRIMARY KEY,
    stripe_event_type TEXT NOT NULL,
    stripe_event_created_timestamp BIGINT NOT NULL,
    processed_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);

CREATE INDEX "ix_processed_stripe_events_on_stripe_event_created_timestamp" ON processed_stripe_events (stripe_event_created_timestamp);
