ALTER TABLE "room_participants"
    ADD "answering_connection_lost" BOOLEAN NOT NULL DEFAULT FALSE;

CREATE INDEX "index_project_collaborators_on_connection_id" ON "project_collaborators" ("connection_id");
CREATE UNIQUE INDEX "index_project_collaborators_on_project_id_connection_id_and_epoch" ON "project_collaborators" ("project_id", "connection_id", "connection_epoch");
CREATE INDEX "index_room_participants_on_answering_connection_id" ON "room_participants" ("answering_connection_id");
CREATE UNIQUE INDEX "index_room_participants_on_answering_connection_id_and_answering_connection_epoch" ON "room_participants" ("answering_connection_id", "answering_connection_epoch");
