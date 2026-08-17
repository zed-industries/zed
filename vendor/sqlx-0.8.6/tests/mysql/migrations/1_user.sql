create table user
(
    -- integer primary keys are the most efficient in SQLite
    user_id  integer primary key auto_increment,
    -- indexed text values have to have a max length
    username varchar(16) unique not null
);
