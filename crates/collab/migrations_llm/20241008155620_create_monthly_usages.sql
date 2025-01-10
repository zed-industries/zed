create table monthly_usages (
    id serial primary key,
    user_id integer not null,
    model_id integer not null references models (id) on delete cascade,
    month integer not null,
    year integer not null,
    input_tokens bigint not null default 0,
    cache_creation_input_tokens bigint not null default 0,
    cache_read_input_tokens bigint not null default 0,
    output_tokens bigint not null default 0
);

create unique index uix_monthly_usages_on_user_id_model_id_month_year on monthly_usages (user_id, model_id, month, year);
