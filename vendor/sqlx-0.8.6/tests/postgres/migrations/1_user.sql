create table "user"
(
    user_id  uuid primary key default uuid_generate_v1mc(),
    username text unique not null
);
