DELETE FROM remote_projects;
DELETE FROM dev_servers;

ALTER TABLE dev_servers DROP COLUMN channel_id;
ALTER TABLE dev_servers ADD COLUMN user_id INT NOT NULL REFERENCES users(id);

ALTER TABLE remote_projects DROP COLUMN channel_id;
