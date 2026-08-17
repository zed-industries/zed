create table user
(
    -- integer primary keys are the most efficient in SQLite
    user_id  integer primary key,
    username text unique not null
);
