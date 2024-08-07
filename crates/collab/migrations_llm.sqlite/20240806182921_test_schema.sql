create table providers (
    id integer primary key autoincrement,
    name text not null
);

create unique index uix_providers_on_name on providers (name);

create table models (
    id integer primary key autoincrement,
    provider_id integer not null references providers (id) on delete cascade,
    name text not null
);

create unique index uix_models_on_provider_id_name on models (provider_id, name);
create index ix_models_on_provider_id on models (provider_id);
create index ix_models_on_name on models (name);

create table if not exists usages (
    id integer primary key autoincrement,
    user_id integer not null,
    model_id integer not null references models (id) on delete cascade,
    requests_this_minute integer not null default 0,
    tokens_this_minute integer not null default 0,
    requests_this_day integer not null default 0,
    tokens_this_day integer not null default 0,
    requests_this_month integer not null default 0,
    tokens_this_month integer not null default 0
);

create index ix_usages_on_user_id on usages (user_id);
create index ix_usages_on_model_id on usages (model_id);
create unique index uix_usages_on_user_id_model_id on usages (user_id, model_id);
