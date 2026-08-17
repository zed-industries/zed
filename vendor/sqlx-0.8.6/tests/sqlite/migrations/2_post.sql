create table post
(
    post_id    integer primary key,
    user_id    integer not null references user (user_id),
    content    text    not null,
    -- Defaults have to be wrapped in parenthesis
    created_at datetime default (datetime('now'))
);

create index post_created_at on post (created_at desc);
