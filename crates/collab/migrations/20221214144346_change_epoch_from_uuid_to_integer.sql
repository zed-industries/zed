ALTER TABLE "projects"
    ALTER COLUMN "host_connection_epoch" TYPE INTEGER USING -1;

ALTER TABLE "project_collaborators"
    ALTER COLUMN "connection_epoch" TYPE INTEGER USING -1;

ALTER TABLE "room_participants"
    ALTER COLUMN "answering_connection_epoch" TYPE INTEGER USING -1,
    ALTER COLUMN "calling_connection_epoch" TYPE INTEGER USING -1;

CREATE TABLE "servers" (
    "epoch" SERIAL PRIMARY KEY,
    "environment" VARCHAR NOT NULL
);
