ALTER TABLE "projects"
    ALTER COLUMN "host_connection_epoch" TYPE INTEGER USING 0;

ALTER TABLE "project_collaborators"
    ALTER COLUMN "connection_epoch" TYPE INTEGER USING 0;

ALTER TABLE "room_participants"
    ALTER COLUMN "answering_connection_epoch" TYPE INTEGER USING 0,
    ALTER COLUMN "calling_connection_epoch" TYPE INTEGER USING 0;
