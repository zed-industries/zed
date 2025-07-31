DROP TABLE "channel_messages";
DROP TABLE "channel_memberships";
DROP TABLE "org_memberships";
DROP TABLE "orgs";
DROP TABLE "channels";

CREATE TABLE "channels" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE "channel_paths" (
    "id_path" VARCHAR NOT NULL PRIMARY KEY,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE
);
CREATE INDEX "index_channel_paths_on_channel_id" ON "channel_paths" ("channel_id");

CREATE TABLE "channel_members" (
    "id" SERIAL PRIMARY KEY,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "admin" BOOLEAN NOT NULL DEFAULT false,
    "accepted" BOOLEAN NOT NULL DEFAULT false,
    "updated_at" TIMESTAMP NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX "index_channel_members_on_channel_id_and_user_id" ON "channel_members" ("channel_id", "user_id");

ALTER TABLE rooms ADD COLUMN "channel_id" INTEGER REFERENCES channels (id) ON DELETE CASCADE;
