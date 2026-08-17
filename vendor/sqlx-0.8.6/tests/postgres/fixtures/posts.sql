insert into post(post_id, user_id, content, created_at)
values
       (
        '252c1d98-a9b0-4f18-8298-e59058bdfe16',
        '6592b7c0-b531-4613-ace5-94246b7ce0c3',
        'This new computer is lightning-fast!',
        now() + '1 hour ago'::interval
        ),
       (
        '844265f7-2472-4689-9a2e-b21f40dbf401',
        '6592b7c0-b531-4613-ace5-94246b7ce0c3',
        '@alice is a haxxor :(',
        now() + '30 minutes ago'::interval
        );
