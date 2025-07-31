create table subscription_usages (
    id serial primary key,
    user_id integer not null,
    period_start_at timestamp without time zone not null,
    period_end_at timestamp without time zone not null,
    model_requests int not null default 0,
    edit_predictions int not null default 0
);

create unique index uix_subscription_usages_on_user_id_start_at_end_at on subscription_usages (user_id, period_start_at, period_end_at);
