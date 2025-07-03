create table subscription_usages_v2 (
    id uuid primary key,
    user_id integer not null,
    period_start_at timestamp without time zone not null,
    period_end_at timestamp without time zone not null,
    plan text not null,
    model_requests int not null default 0,
    edit_predictions int not null default 0
);

create unique index uix_subscription_usages_v2_on_user_id_start_at_end_at on subscription_usages_v2 (user_id, period_start_at, period_end_at);

create index ix_subscription_usages_v2_on_plan on subscription_usages_v2 (plan);

create table subscription_usage_meters_v2 (
    id uuid primary key,
    subscription_usage_id uuid not null references subscription_usages_v2 (id) on delete cascade,
    model_id integer not null references models (id) on delete cascade,
    mode text not null,
    requests integer not null default 0
);

create unique index uix_subscription_usage_meters_v2_on_usage_model_mode on subscription_usage_meters_v2 (subscription_usage_id, model_id, mode);
