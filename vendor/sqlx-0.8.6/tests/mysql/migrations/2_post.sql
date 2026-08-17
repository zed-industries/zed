create table post
(
    post_id    integer primary key auto_increment,
    user_id    integer not null references user (user_id),
    content    text    not null,
    -- Defaults have to be wrapped in parenthesis
    created_at datetime default current_timestamp
);

create index post_created_at on post (created_at desc);
