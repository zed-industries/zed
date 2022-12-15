CREATE TABLE servers (
    id SERIAL PRIMARY KEY,
    environment VARCHAR NOT NULL
);

DELETE FROM projects;
ALTER TABLE projects
    DROP COLUMN host_connection_epoch,
    ADD COLUMN host_connection_server_id INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE;

DELETE FROM project_collaborators;
ALTER TABLE project_collaborators
    DROP COLUMN connection_epoch,
    ADD COLUMN connection_server_id INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE;

DELETE FROM room_participants;
ALTER TABLE room_participants
    DROP COLUMN answering_connection_epoch,
    DROP COLUMN calling_connection_epoch,
    ADD COLUMN answering_connection_server_id INTEGER REFERENCES servers (id) ON DELETE CASCADE,
    ADD COLUMN calling_connection_server_id INTEGER REFERENCES servers (id) ON DELETE SET NULL;

