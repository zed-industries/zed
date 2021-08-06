CREATE TABLE IF NOT EXISTS "orgs" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "slug" VARCHAR NOT NULL
);

CREATE UNIQUE INDEX "index_orgs_slug" ON "orgs" ("slug");

CREATE TABLE IF NOT EXISTS "org_memberships" (
    "id" SERIAL PRIMARY KEY,
    "org_id" INTEGER REFERENCES orgs (id) NOT NULL,
    "user_id" INTEGER REFERENCES users (id) NOT NULL,
    "admin" BOOLEAN NOT NULL
);

CREATE INDEX "index_org_memberships_user_id" ON "org_memberships" ("user_id");
CREATE UNIQUE INDEX "index_org_memberships_org_id_and_user_id" ON "org_memberships" ("org_id", "user_id");

CREATE TABLE IF NOT EXISTS "channels" (
    "id" SERIAL PRIMARY KEY,
    "owner_id" INTEGER NOT NULL,
    "owner_is_user" BOOLEAN NOT NULL,
    "name" VARCHAR NOT NULL
);

CREATE UNIQUE INDEX "index_channels_owner_and_name" ON "channels" ("owner_is_user", "owner_id", "name");

CREATE TABLE IF NOT EXISTS "channel_memberships" (
    "id" SERIAL PRIMARY KEY,
    "channel_id" INTEGER REFERENCES channels (id) NOT NULL,
    "user_id" INTEGER REFERENCES users (id) NOT NULL,
    "admin" BOOLEAN NOT NULL
);

CREATE INDEX "index_channel_memberships_user_id" ON "channel_memberships" ("user_id");
CREATE UNIQUE INDEX "index_channel_memberships_channel_id_and_user_id" ON "channel_memberships" ("channel_id", "user_id");

CREATE TABLE IF NOT EXISTS "channel_messages" (
    "id" SERIAL PRIMARY KEY,
    "channel_id" INTEGER REFERENCES channels (id) NOT NULL,
    "sender_id" INTEGER REFERENCES users (id) NOT NULL,
    "body" TEXT NOT NULL,
    "sent_at" TIMESTAMP
);

CREATE INDEX "index_channel_messages_channel_id" ON "channel_messages" ("channel_id");
