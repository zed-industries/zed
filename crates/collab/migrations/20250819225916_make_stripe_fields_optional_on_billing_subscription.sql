alter table billing_subscriptions
    alter column stripe_subscription_id drop not null,
    alter column stripe_subscription_status drop not null;
