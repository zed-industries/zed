CREATE TABLE dev_server_projects (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY (START WITH 100),
    dev_server_id INT NOT NULL REFERENCES dev_servers(id) ON DELETE CASCADE,
    path TEXT NOT NULL
);
INSERT INTO dev_server_projects OVERRIDING SYSTEM VALUE SELECT * FROM remote_projects;

ALTER TABLE dev_server_projects ADD CONSTRAINT uix_dev_server_projects_dev_server_id_path UNIQUE(dev_server_id, path);

ALTER TABLE projects ADD COLUMN dev_server_project_id INTEGER REFERENCES dev_server_projects(id);
UPDATE projects SET dev_server_project_id = remote_project_id;
