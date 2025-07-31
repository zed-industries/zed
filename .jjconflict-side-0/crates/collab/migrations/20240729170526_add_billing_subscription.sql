CREATE TABLE IF NOT EXISTS billing_subscriptions (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    stripe_customer_id TEXT NOT NULL,
    stripe_subscription_id TEXT NOT NULL,
    stripe_subscription_status TEXT NOT NULL
);

CREATE INDEX "ix_billing_subscriptions_on_user_id" ON billing_subscriptions (user_id);
CREATE INDEX "ix_billing_subscriptions_on_stripe_customer_id" ON billing_subscriptions (stripe_customer_id);
CREATE UNIQUE INDEX "uix_billing_subscriptions_on_stripe_subscription_id" ON billing_subscriptions (stripe_subscription_id);
