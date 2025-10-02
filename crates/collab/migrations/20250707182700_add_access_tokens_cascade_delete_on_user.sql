ALTER TABLE access_tokens DROP CONSTRAINT access_tokens_user_id_fkey;
ALTER TABLE access_tokens ADD CONSTRAINT access_tokens_user_id_fkey
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
