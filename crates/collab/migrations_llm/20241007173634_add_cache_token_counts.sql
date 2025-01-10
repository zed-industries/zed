alter table models
    add column price_per_million_cache_creation_input_tokens integer not null default 0,
    add column price_per_million_cache_read_input_tokens integer not null default 0;

alter table usages
    add column cache_creation_input_tokens_this_month bigint not null default 0,
    add column cache_read_input_tokens_this_month bigint not null default 0;

alter table lifetime_usages
    add column cache_creation_input_tokens bigint not null default 0,
    add column cache_read_input_tokens bigint not null default 0;
