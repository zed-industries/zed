create table billing_events (
    id serial primary key,
    idempotency_key uuid not null default gen_random_uuid(),
    user_id integer not null,
    model_id integer not null references models (id) on delete cascade,
    input_tokens bigint not null default 0,
    input_cache_creation_tokens bigint not null default 0,
    input_cache_read_tokens bigint not null default 0,
    output_tokens bigint not null default 0
);

create index uix_billing_events_on_user_id_model_id on billing_events (user_id, model_id);
