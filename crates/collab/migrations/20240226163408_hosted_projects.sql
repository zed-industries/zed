-- Add migration script here

CREATE TABLE hosted_projects (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    channel_id INT NOT NULL REFERENCES channels(id),
    name TEXT NOT NULL,
    visibility TEXT NOT NULL,
    deleted_at TIMESTAMP NULL
);
CREATE INDEX idx_hosted_projects_on_channel_id ON hosted_projects (channel_id);
CREATE UNIQUE INDEX uix_hosted_projects_on_channel_id_and_name ON hosted_projects (channel_id, name) WHERE (deleted_at IS NULL);
