create table billing_events (
    id serial primary key,
    user_id integer not null,
    model_id integer not null references models (id) on delete cascade,
    input_tokens bigint not null default 0,
    cache_creation_input_tokens bigint not null default 0,
    cache_read_input_tokens bigint not null default 0,
    output_tokens bigint not null default 0
);

create index uix_billing_events_on_user_id_model_id on billing_events (user_id, model_id);
