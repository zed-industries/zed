DROP TABLE signups;

ALTER TABLE users
    DROP COLUMN github_user_id,
    DROP COLUMN metrics_id;

DROP SEQUENCE metrics_id_seq;

DROP INDEX index_users_on_email_address;
DROP INDEX index_users_on_github_user_id;