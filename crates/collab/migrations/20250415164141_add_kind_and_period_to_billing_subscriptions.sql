alter table billing_subscriptions
    add column kind text,
    add column stripe_current_period_start bigint,
    add column stripe_current_period_end bigint;
