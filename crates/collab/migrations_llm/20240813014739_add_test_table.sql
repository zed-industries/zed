create table squawk_test (
    id serial primary key,
    created_at timestamp without time zone not null default now()
);
