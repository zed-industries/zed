create table usage_measures (
    id serial primary key,
    name text not null
);

create unique index uix_usage_measures_on_name on usage_measures (name);

create table if not exists usages (
    id serial primary key,
    user_id integer not null,
    model_id integer not null references models (id) on delete cascade,
    measure_id integer not null references usage_measures (id) on delete cascade,
    timestamp timestamp without time zone not null,
    buckets bigint[] not null
);

create index ix_usages_on_user_id on usages (user_id);
create index ix_usages_on_model_id on usages (model_id);
create unique index uix_usages_on_user_id_model_id_measure_id on usages (user_id, model_id, measure_id);
