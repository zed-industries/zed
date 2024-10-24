DROP TABLE dev_servers;
DROP TABLE dev_server_projects;

ALTER TABLE projects DROP COLUMN dev_server_project_id;
