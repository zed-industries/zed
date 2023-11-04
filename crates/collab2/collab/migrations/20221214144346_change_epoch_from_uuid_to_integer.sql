CREATE TABLE servers (
    id SERIAL PRIMARY KEY,
    environment VARCHAR NOT NULL
);

DROP TABLE worktree_extensions;
DROP TABLE project_activity_periods;
DELETE from projects;
ALTER TABLE projects
    DROP COLUMN host_connection_epoch,
    ADD COLUMN host_connection_server_id INTEGER REFERENCES servers (id) ON DELETE CASCADE;
CREATE INDEX "index_projects_on_host_connection_server_id" ON "projects" ("host_connection_server_id");
CREATE INDEX "index_projects_on_host_connection_id_and_host_connection_server_id" ON "projects" ("host_connection_id", "host_connection_server_id");

DELETE FROM project_collaborators;
ALTER TABLE project_collaborators
    DROP COLUMN connection_epoch,
    ADD COLUMN connection_server_id INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE;
CREATE INDEX "index_project_collaborators_on_connection_server_id" ON "project_collaborators" ("connection_server_id");
CREATE UNIQUE INDEX "index_project_collaborators_on_project_id_connection_id_and_server_id" ON "project_collaborators" ("project_id", "connection_id", "connection_server_id");

DELETE FROM room_participants;
ALTER TABLE room_participants
    DROP COLUMN answering_connection_epoch,
    DROP COLUMN calling_connection_epoch,
    ADD COLUMN answering_connection_server_id INTEGER REFERENCES servers (id) ON DELETE CASCADE,
    ADD COLUMN calling_connection_server_id INTEGER REFERENCES servers (id) ON DELETE SET NULL;
CREATE INDEX "index_room_participants_on_answering_connection_server_id" ON "room_participants" ("answering_connection_server_id");
CREATE INDEX "index_room_participants_on_calling_connection_server_id" ON "room_participants" ("calling_connection_server_id");
CREATE UNIQUE INDEX "index_room_participants_on_answering_connection_id_and_answering_connection_server_id" ON "room_participants" ("answering_connection_id", "answering_connection_server_id");
