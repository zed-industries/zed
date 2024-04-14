CREATE TABLE remote_projects (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    channel_id INT NOT NULL REFERENCES channels(id),
    dev_server_id INT NOT NULL REFERENCES dev_servers(id),
    name TEXT NOT NULL,
    path TEXT NOT NULL
);

ALTER TABLE projects ADD COLUMN remote_project_id INTEGER REFERENCES remote_projects(id);
