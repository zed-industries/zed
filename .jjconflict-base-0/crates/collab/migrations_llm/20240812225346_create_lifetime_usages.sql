create table lifetime_usages (
    id serial primary key,
    user_id integer not null,
    model_id integer not null references models (id) on delete cascade,
    input_tokens bigint not null default 0,
    output_tokens bigint not null default 0
);

create unique index uix_lifetime_usages_on_user_id_model_id on lifetime_usages (user_id, model_id);
