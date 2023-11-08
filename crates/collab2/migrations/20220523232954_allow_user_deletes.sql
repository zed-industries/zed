ALTER TABLE contacts DROP CONSTRAINT contacts_user_id_a_fkey;
ALTER TABLE contacts DROP CONSTRAINT contacts_user_id_b_fkey;
ALTER TABLE contacts ADD CONSTRAINT contacts_user_id_a_fkey FOREIGN KEY (user_id_a) REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE contacts ADD CONSTRAINT contacts_user_id_b_fkey FOREIGN KEY (user_id_b) REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE users DROP CONSTRAINT users_inviter_id_fkey;
ALTER TABLE users ADD CONSTRAINT users_inviter_id_fkey FOREIGN KEY (inviter_id) REFERENCES users(id) ON DELETE SET NULL;
