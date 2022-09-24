DROP TABLE signups;

ALTER TABLE users
    DROP COLUMN github_user_id;

DROP INDEX index_users_on_email_address;
DROP INDEX index_users_on_github_user_id;
