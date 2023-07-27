CREATE TABLE "channels" (
    "id" SERIAL PRIMARY KEY,
    "id_path" TEXT NOT NULL,
    "name" VARCHAR NOT NULL,
    "room_id" INTEGER REFERENCES rooms (id) ON DELETE SET NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT now
)

CREATE UNIQUE INDEX "index_channels_on_id_path" ON "channels" ("id_path");

CREATE TABLE "channel_members" (
    "id" SERIAL PRIMARY KEY,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "admin" BOOLEAN NOT NULL DEFAULT false,
    "updated_at" TIMESTAMP NOT NULL DEFAULT now
)

CREATE UNIQUE INDEX "index_channel_members_on_channel_id_and_user_id" ON "channel_members" ("channel_id", "user_id");
