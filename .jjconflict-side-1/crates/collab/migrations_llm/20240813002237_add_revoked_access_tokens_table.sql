create table revoked_access_tokens (
    id serial primary key,
    jti text not null,
    revoked_at timestamp without time zone not null default now()
);

create unique index uix_revoked_access_tokens_on_jti on revoked_access_tokens (jti);
