create table if not exists usages (
    id serial primary key,
    user_id integer not null,
    model_id integer not null references models (id) on delete cascade,
    requests_this_minute integer not null default 0,
    tokens_this_minute bigint not null default 0,
    requests_this_day integer not null default 0,
    tokens_this_day bigint not null default 0,
    requests_this_month integer not null default 0,
    tokens_this_month bigint not null default 0
);

create index ix_usages_on_user_id on usages (user_id);
create index ix_usages_on_model_id on usages (model_id);
create unique index uix_usages_on_user_id_model_id on usages (user_id, model_id);
