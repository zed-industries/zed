alter table billing_preferences
    add column model_request_overages_enabled bool not null default false,
    add column model_request_overages_spend_limit_in_cents integer not null default 0;
