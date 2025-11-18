alter table billing_subscriptions
    add column orb_subscription_status text,
    add column orb_current_billing_period_start_date timestamp without time zone,
    add column orb_current_billing_period_end_date timestamp without time zone;
