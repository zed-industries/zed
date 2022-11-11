CREATE TABLE IF NOT EXISTS "rooms" (
    "id" SERIAL PRIMARY KEY,
    "version" INTEGER NOT NULL,
    "live_kit_room" VARCHAR NOT NULL
);

ALTER TABLE "projects"
    ADD "room_id" INTEGER REFERENCES rooms (id),
    DROP COLUMN "unregistered";

CREATE TABLE "project_collaborators" (
    "id" SERIAL PRIMARY KEY,
    "project_id" INTEGER NOT NULL REFERENCES projects (id),
    "connection_id" INTEGER NOT NULL,
    "user_id" INTEGER NOT NULL,
    "replica_id" INTEGER NOT NULL,
    "is_host" BOOLEAN NOT NULL
);
CREATE INDEX "index_project_collaborators_on_project_id" ON "project_collaborators" ("project_id");

CREATE TABLE IF NOT EXISTS "worktrees" (
    "id" INTEGER NOT NULL,
    "project_id" INTEGER NOT NULL REFERENCES projects (id),
    "root_name" VARCHAR NOT NULL,
    PRIMARY KEY(project_id, id)
);
CREATE INDEX "index_worktrees_on_project_id" ON "worktrees" ("project_id");

CREATE TABLE IF NOT EXISTS "room_participants" (
    "id" SERIAL PRIMARY KEY,
    "room_id" INTEGER NOT NULL REFERENCES rooms (id),
    "user_id" INTEGER NOT NULL REFERENCES users (id),
    "connection_id" INTEGER,
    "location_kind" INTEGER,
    "location_project_id" INTEGER REFERENCES projects (id)
);
CREATE UNIQUE INDEX "index_room_participants_on_user_id_and_room_id" ON "room_participants" ("user_id", "room_id");

CREATE TABLE IF NOT EXISTS "calls" (
    "id" SERIAL PRIMARY KEY,
    "room_id" INTEGER NOT NULL REFERENCES rooms (id),
    "calling_user_id" INTEGER NOT NULL REFERENCES users (id),
    "called_user_id" INTEGER NOT NULL REFERENCES users (id),
    "answering_connection_id" INTEGER,
    "initial_project_id" INTEGER REFERENCES projects (id)
);
CREATE UNIQUE INDEX "index_calls_on_called_user_id" ON "calls" ("called_user_id");
