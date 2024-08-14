create table if not exists providers (
    id serial primary key,
    name text not null
);

create unique index uix_providers_on_name on providers (name);

create table if not exists models (
    id serial primary key,
    provider_id integer not null references providers (id) on delete cascade,
    name text not null,
    max_requests_per_minute integer not null,
    max_tokens_per_minute integer not null,
    max_tokens_per_day integer not null
);

create unique index uix_models_on_provider_id_name on models (provider_id, name);
create index ix_models_on_provider_id on models (provider_id);
create index ix_models_on_name on models (name);
