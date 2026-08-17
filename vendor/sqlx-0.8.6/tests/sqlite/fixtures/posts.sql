insert into post(post_id, user_id, content, created_at)
values (1,
        1,
        'This new computer is lightning-fast!',
        datetime('now', '-1 hour')),
       (2,
        2,
        '@alice is a haxxor :(',
        datetime('now', '-30 minutes'));
