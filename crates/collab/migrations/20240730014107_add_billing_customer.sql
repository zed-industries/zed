CREATE TABLE IF NOT EXISTS billing_customers (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    stripe_customer_id TEXT NOT NULL
);

CREATE UNIQUE INDEX "uix_billing_customers_on_user_id" ON billing_customers (user_id);
CREATE UNIQUE INDEX "uix_billing_customers_on_stripe_customer_id" ON billing_customers (stripe_customer_id);

-- Make `billing_subscriptions` reference `billing_customers` instead of having its
-- own `user_id` and `stripe_customer_id`.
DROP INDEX IF EXISTS "ix_billing_subscriptions_on_user_id";
DROP INDEX IF EXISTS "ix_billing_subscriptions_on_stripe_customer_id";
ALTER TABLE billing_subscriptions DROP COLUMN user_id;
ALTER TABLE billing_subscriptions DROP COLUMN stripe_customer_id;
ALTER TABLE billing_subscriptions ADD COLUMN billing_customer_id INTEGER NOT NULL REFERENCES billing_customers (id) ON DELETE CASCADE;
CREATE INDEX "ix_billing_subscriptions_on_billing_customer_id" ON billing_subscriptions (billing_customer_id);
