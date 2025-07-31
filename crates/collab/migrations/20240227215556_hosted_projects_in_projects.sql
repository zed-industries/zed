-- Add migration script here
ALTER TABLE projects ALTER COLUMN host_user_id DROP NOT NULL;
ALTER TABLE projects ADD COLUMN hosted_project_id INTEGER REFERENCES hosted_projects(id) UNIQUE NULL;
