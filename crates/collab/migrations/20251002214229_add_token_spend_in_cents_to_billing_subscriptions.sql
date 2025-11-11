alter table billing_subscriptions
    add column token_spend_in_cents integer,
    add column token_spend_in_cents_updated_at timestamp without time zone;
