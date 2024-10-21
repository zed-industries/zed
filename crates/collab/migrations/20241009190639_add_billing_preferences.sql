create table if not exists billing_preferences (
    id serial primary key,
    created_at timestamp without time zone not null default now(),
    user_id integer not null references users(id) on delete cascade,
    max_monthly_llm_usage_spending_in_cents integer not null
);

create unique index "uix_billing_preferences_on_user_id" on billing_preferences (user_id);
