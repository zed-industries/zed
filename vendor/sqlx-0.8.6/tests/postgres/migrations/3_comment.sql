create table comment (
    comment_id uuid primary key default uuid_generate_v1mc(),
    post_id uuid not null references post(post_id),
    user_id uuid not null references "user"(user_id),
    content text not null,
    created_at timestamptz not null default now()
);

create index on comment(created_at desc);
