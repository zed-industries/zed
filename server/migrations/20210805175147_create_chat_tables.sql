CREATE TABLE IF NOT EXISTS "orgs" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "slug" VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS "org_memberships" (
    "id" SERIAL PRIMARY KEY,
    "org_id" INTEGER REFERENCES orgs (id) NOT NULL,
    "user_id" INTEGER REFERENCES users (id) NOT NULL,
    "admin" BOOLEAN NOT NULL
);

CREATE UNIQUE INDEX "index_org_memberships_user_id" ON "org_memberships" ("user_id");
CREATE UNIQUE INDEX "index_org_memberships_org_id" ON "org_memberships" ("org_id");

CREATE TABLE IF NOT EXISTS "channels" (
    "id" SERIAL PRIMARY KEY,
    "owner_id" INTEGER NOT NULL,
    "owner_is_user" BOOLEAN NOT NULL,
    "name" VARCHAR NOT NULL
);

CREATE UNIQUE INDEX "index_channels_owner" ON "channels" ("owner_is_user", "owner_id");

CREATE TABLE IF NOT EXISTS "channel_memberships" (
    "id" SERIAL PRIMARY KEY,
    "channel_id" INTEGER REFERENCES channels (id) NOT NULL,
    "user_id" INTEGER REFERENCES users (id) NOT NULL,
    "admin" BOOLEAN NOT NULL
);

CREATE UNIQUE INDEX "index_channel_memberships_user_id" ON "channel_memberships" ("user_id");
CREATE UNIQUE INDEX "index_channel_memberships_channel_id" ON "channel_memberships" ("channel_id");

CREATE TABLE IF NOT EXISTS "channel_messages" (
    "id" SERIAL PRIMARY KEY,
    "channel_id" INTEGER REFERENCES channels (id) NOT NULL,
    "sender_id" INTEGER REFERENCES users (id) NOT NULL,
    "content" TEXT NOT NULL,
    "sent_at" TIMESTAMP
);

CREATE UNIQUE INDEX "index_channel_messages_channel_id" ON "channel_messages" ("channel_id");

INSERT INTO users (github_login, admin) VALUES ('iamnbutler', true);

DO $$ 
DECLARE
    zed_org_id INTEGER;
    max_id INTEGER;
    nathan_id INTEGER;
    antonio_id INTEGER;
    nate_id INTEGER;
BEGIN 
    INSERT INTO "orgs" (name, slug) VALUES ('Zed', 'zed') RETURNING id into zed_org_id;
END $$;

