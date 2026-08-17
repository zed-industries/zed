create table post (
    post_id uuid primary key default uuid_generate_v1mc(),
    user_id uuid not null references "user"(user_id),
    content text not null,
    created_at timestamptz default now()
);

create index on post(created_at desc);
