create table subscription_usage_meters (
    id serial primary key,
    subscription_usage_id integer not null references subscription_usages (id) on delete cascade,
    model_id integer not null references models (id) on delete cascade,
    requests integer not null default 0
);

create unique index uix_subscription_usage_meters_on_subscription_usage_model on subscription_usage_meters (subscription_usage_id, model_id);
