create table comment
(
    comment_id integer primary key,
    post_id    integer not null references post (post_id),
    user_id    integer not null references "user" (user_id),
    content    text    not null,
    created_at datetime default (datetime('now'))
);

create index comment_created_at on comment (created_at desc);
