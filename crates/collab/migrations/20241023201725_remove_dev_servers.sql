ALTER TABLE projects DROP COLUMN dev_server_project_id;
ALTER TABLE projects DROP COLUMN hosted_project_id;

DROP TABLE hosted_projects;
DROP TABLE dev_server_projects;
DROP TABLE dev_servers;
