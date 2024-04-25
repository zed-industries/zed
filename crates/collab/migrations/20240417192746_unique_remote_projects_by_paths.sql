ALTER TABLE remote_projects DROP COLUMN name;
ALTER TABLE remote_projects
ADD CONSTRAINT unique_path_constraint UNIQUE(dev_server_id, path);
