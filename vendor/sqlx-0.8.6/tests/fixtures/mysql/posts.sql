insert into post(post_id, user_id, content, created_at)
values (1,
        1,
        'This new computer is lightning-fast!',
        timestamp(now(), '-1:00:00')),
       (2,
        2,
        '@alice is a haxxor :(',
        timestamp(now(), '-0:30:00'));
